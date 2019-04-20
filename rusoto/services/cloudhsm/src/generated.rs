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

use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_slice;
use serde_json::Value as SerdeJsonValue;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AddTagsToResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the AWS CloudHSM resource to tag.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>One or more tags.</p>
    #[serde(rename = "TagList")]
    pub tag_list: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AddTagsToResourceResponse {
    /// <p>The status of the operation.</p>
    #[serde(rename = "Status")]
    pub status: String,
}

/// <p>Contains the inputs for the <a>CreateHapgRequest</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateHapgRequest {
    /// <p>The label of the new high-availability partition group.</p>
    #[serde(rename = "Label")]
    pub label: String,
}

/// <p>Contains the output of the <a>CreateHAPartitionGroup</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateHapgResponse {
    /// <p>The ARN of the high-availability partition group.</p>
    #[serde(rename = "HapgArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hapg_arn: Option<String>,
}

/// <p>Contains the inputs for the <code>CreateHsm</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateHsmRequest {
    /// <p>A user-defined token to ensure idempotence. Subsequent calls to this operation with the same token will be ignored.</p>
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The IP address to assign to the HSM's ENI.</p> <p>If an IP address is not specified, an IP address will be randomly chosen from the CIDR range of the subnet.</p>
    #[serde(rename = "EniIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eni_ip: Option<String>,
    /// <p>The external ID from <code>IamRoleArn</code>, if present.</p>
    #[serde(rename = "ExternalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// <p>The ARN of an IAM role to enable the AWS CloudHSM service to allocate an ENI on your behalf.</p>
    #[serde(rename = "IamRoleArn")]
    pub iam_role_arn: String,
    /// <p>The SSH public key to install on the HSM.</p>
    #[serde(rename = "SshKey")]
    pub ssh_key: String,
    /// <p>The identifier of the subnet in your VPC in which to place the HSM.</p>
    #[serde(rename = "SubnetId")]
    pub subnet_id: String,
    #[serde(rename = "SubscriptionType")]
    pub subscription_type: String,
    /// <p>The IP address for the syslog monitoring server. The AWS CloudHSM service only supports one syslog monitoring server.</p>
    #[serde(rename = "SyslogIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub syslog_ip: Option<String>,
}

/// <p>Contains the output of the <code>CreateHsm</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateHsmResponse {
    /// <p>The ARN of the HSM.</p>
    #[serde(rename = "HsmArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_arn: Option<String>,
}

/// <p>Contains the inputs for the <a>CreateLunaClient</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateLunaClientRequest {
    /// <p>The contents of a Base64-Encoded X.509 v3 certificate to be installed on the HSMs used by this client.</p>
    #[serde(rename = "Certificate")]
    pub certificate: String,
    /// <p>The label for the client.</p>
    #[serde(rename = "Label")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// <p>Contains the output of the <a>CreateLunaClient</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateLunaClientResponse {
    /// <p>The ARN of the client.</p>
    #[serde(rename = "ClientArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_arn: Option<String>,
}

/// <p>Contains the inputs for the <a>DeleteHapg</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteHapgRequest {
    /// <p>The ARN of the high-availability partition group to delete.</p>
    #[serde(rename = "HapgArn")]
    pub hapg_arn: String,
}

/// <p>Contains the output of the <a>DeleteHapg</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteHapgResponse {
    /// <p>The status of the action.</p>
    #[serde(rename = "Status")]
    pub status: String,
}

/// <p>Contains the inputs for the <a>DeleteHsm</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteHsmRequest {
    /// <p>The ARN of the HSM to delete.</p>
    #[serde(rename = "HsmArn")]
    pub hsm_arn: String,
}

/// <p>Contains the output of the <a>DeleteHsm</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteHsmResponse {
    /// <p>The status of the operation.</p>
    #[serde(rename = "Status")]
    pub status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteLunaClientRequest {
    /// <p>The ARN of the client to delete.</p>
    #[serde(rename = "ClientArn")]
    pub client_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteLunaClientResponse {
    /// <p>The status of the action.</p>
    #[serde(rename = "Status")]
    pub status: String,
}

/// <p>Contains the inputs for the <a>DescribeHapg</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeHapgRequest {
    /// <p>The ARN of the high-availability partition group to describe.</p>
    #[serde(rename = "HapgArn")]
    pub hapg_arn: String,
}

/// <p>Contains the output of the <a>DescribeHapg</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeHapgResponse {
    /// <p>The ARN of the high-availability partition group.</p>
    #[serde(rename = "HapgArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hapg_arn: Option<String>,
    /// <p>The serial number of the high-availability partition group.</p>
    #[serde(rename = "HapgSerial")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hapg_serial: Option<String>,
    /// <p><p/></p>
    #[serde(rename = "HsmsLastActionFailed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsms_last_action_failed: Option<Vec<String>>,
    /// <p><p/></p>
    #[serde(rename = "HsmsPendingDeletion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsms_pending_deletion: Option<Vec<String>>,
    /// <p><p/></p>
    #[serde(rename = "HsmsPendingRegistration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsms_pending_registration: Option<Vec<String>>,
    /// <p>The label for the high-availability partition group.</p>
    #[serde(rename = "Label")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// <p>The date and time the high-availability partition group was last modified.</p>
    #[serde(rename = "LastModifiedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_timestamp: Option<String>,
    /// <p>The list of partition serial numbers that belong to the high-availability partition group.</p>
    #[serde(rename = "PartitionSerialList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_serial_list: Option<Vec<String>>,
    /// <p>The state of the high-availability partition group.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// <p>Contains the inputs for the <a>DescribeHsm</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeHsmRequest {
    /// <p>The ARN of the HSM. Either the <code>HsmArn</code> or the <code>SerialNumber</code> parameter must be specified.</p>
    #[serde(rename = "HsmArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_arn: Option<String>,
    /// <p>The serial number of the HSM. Either the <code>HsmArn</code> or the <code>HsmSerialNumber</code> parameter must be specified.</p>
    #[serde(rename = "HsmSerialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_serial_number: Option<String>,
}

/// <p>Contains the output of the <a>DescribeHsm</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeHsmResponse {
    /// <p>The Availability Zone that the HSM is in.</p>
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>The identifier of the elastic network interface (ENI) attached to the HSM.</p>
    #[serde(rename = "EniId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eni_id: Option<String>,
    /// <p>The IP address assigned to the HSM's ENI.</p>
    #[serde(rename = "EniIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eni_ip: Option<String>,
    /// <p>The ARN of the HSM.</p>
    #[serde(rename = "HsmArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_arn: Option<String>,
    /// <p>The HSM model type.</p>
    #[serde(rename = "HsmType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_type: Option<String>,
    /// <p>The ARN of the IAM role assigned to the HSM.</p>
    #[serde(rename = "IamRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    /// <p>The list of partitions on the HSM.</p>
    #[serde(rename = "Partitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partitions: Option<Vec<String>>,
    /// <p>The serial number of the HSM.</p>
    #[serde(rename = "SerialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// <p>The date and time that the server certificate was last updated.</p>
    #[serde(rename = "ServerCertLastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_cert_last_updated: Option<String>,
    /// <p>The URI of the certificate server.</p>
    #[serde(rename = "ServerCertUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_cert_uri: Option<String>,
    /// <p>The HSM software version.</p>
    #[serde(rename = "SoftwareVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub software_version: Option<String>,
    /// <p>The date and time that the SSH key was last updated.</p>
    #[serde(rename = "SshKeyLastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_key_last_updated: Option<String>,
    /// <p>The public SSH key.</p>
    #[serde(rename = "SshPublicKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_public_key: Option<String>,
    /// <p>The status of the HSM.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>Contains additional information about the status of the HSM.</p>
    #[serde(rename = "StatusDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    /// <p>The identifier of the subnet that the HSM is in.</p>
    #[serde(rename = "SubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    /// <p>The subscription end date.</p>
    #[serde(rename = "SubscriptionEndDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_end_date: Option<String>,
    /// <p>The subscription start date.</p>
    #[serde(rename = "SubscriptionStartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_start_date: Option<String>,
    #[serde(rename = "SubscriptionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_type: Option<String>,
    /// <p>The name of the HSM vendor.</p>
    #[serde(rename = "VendorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_name: Option<String>,
    /// <p>The identifier of the VPC that the HSM is in.</p>
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeLunaClientRequest {
    /// <p>The certificate fingerprint.</p>
    #[serde(rename = "CertificateFingerprint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_fingerprint: Option<String>,
    /// <p>The ARN of the client.</p>
    #[serde(rename = "ClientArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeLunaClientResponse {
    /// <p>The certificate installed on the HSMs used by this client.</p>
    #[serde(rename = "Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    /// <p>The certificate fingerprint.</p>
    #[serde(rename = "CertificateFingerprint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_fingerprint: Option<String>,
    /// <p>The ARN of the client.</p>
    #[serde(rename = "ClientArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_arn: Option<String>,
    /// <p>The label of the client.</p>
    #[serde(rename = "Label")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// <p>The date and time the client was last modified.</p>
    #[serde(rename = "LastModifiedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_timestamp: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetConfigRequest {
    /// <p>The ARN of the client.</p>
    #[serde(rename = "ClientArn")]
    pub client_arn: String,
    /// <p>The client version.</p>
    #[serde(rename = "ClientVersion")]
    pub client_version: String,
    /// <p>A list of ARNs that identify the high-availability partition groups that are associated with the client.</p>
    #[serde(rename = "HapgList")]
    pub hapg_list: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetConfigResponse {
    /// <p>The certificate file containing the server.pem files of the HSMs.</p>
    #[serde(rename = "ConfigCred")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_cred: Option<String>,
    /// <p>The chrystoki.conf configuration file.</p>
    #[serde(rename = "ConfigFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_file: Option<String>,
    /// <p>The type of credentials.</p>
    #[serde(rename = "ConfigType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_type: Option<String>,
}

/// <p>Contains the inputs for the <a>ListAvailableZones</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListAvailableZonesRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListAvailableZonesResponse {
    /// <p>The list of Availability Zones that have available AWS CloudHSM capacity.</p>
    #[serde(rename = "AZList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub az_list: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListHapgsRequest {
    /// <p>The <code>NextToken</code> value from a previous call to <code>ListHapgs</code>. Pass null if this is the first call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListHapgsResponse {
    /// <p>The list of high-availability partition groups.</p>
    #[serde(rename = "HapgList")]
    pub hapg_list: Vec<String>,
    /// <p>If not null, more results are available. Pass this value to <code>ListHapgs</code> to retrieve the next set of items.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListHsmsRequest {
    /// <p>The <code>NextToken</code> value from a previous call to <code>ListHsms</code>. Pass null if this is the first call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Contains the output of the <code>ListHsms</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListHsmsResponse {
    /// <p>The list of ARNs that identify the HSMs.</p>
    #[serde(rename = "HsmList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_list: Option<Vec<String>>,
    /// <p>If not null, more results are available. Pass this value to <code>ListHsms</code> to retrieve the next set of items.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListLunaClientsRequest {
    /// <p>The <code>NextToken</code> value from a previous call to <code>ListLunaClients</code>. Pass null if this is the first call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListLunaClientsResponse {
    /// <p>The list of clients.</p>
    #[serde(rename = "ClientList")]
    pub client_list: Vec<String>,
    /// <p>If not null, more results are available. Pass this to <code>ListLunaClients</code> to retrieve the next set of items.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTagsForResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the AWS CloudHSM resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>One or more tags.</p>
    #[serde(rename = "TagList")]
    pub tag_list: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ModifyHapgRequest {
    /// <p>The ARN of the high-availability partition group to modify.</p>
    #[serde(rename = "HapgArn")]
    pub hapg_arn: String,
    /// <p>The new label for the high-availability partition group.</p>
    #[serde(rename = "Label")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// <p>The list of partition serial numbers to make members of the high-availability partition group.</p>
    #[serde(rename = "PartitionSerialList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_serial_list: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ModifyHapgResponse {
    /// <p>The ARN of the high-availability partition group.</p>
    #[serde(rename = "HapgArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hapg_arn: Option<String>,
}

/// <p>Contains the inputs for the <a>ModifyHsm</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ModifyHsmRequest {
    /// <p>The new IP address for the elastic network interface (ENI) attached to the HSM.</p> <p>If the HSM is moved to a different subnet, and an IP address is not specified, an IP address will be randomly chosen from the CIDR range of the new subnet.</p>
    #[serde(rename = "EniIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eni_ip: Option<String>,
    /// <p>The new external ID.</p>
    #[serde(rename = "ExternalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// <p>The ARN of the HSM to modify.</p>
    #[serde(rename = "HsmArn")]
    pub hsm_arn: String,
    /// <p>The new IAM role ARN.</p>
    #[serde(rename = "IamRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    /// <p>The new identifier of the subnet that the HSM is in. The new subnet must be in the same Availability Zone as the current subnet.</p>
    #[serde(rename = "SubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    /// <p>The new IP address for the syslog monitoring server. The AWS CloudHSM service only supports one syslog monitoring server.</p>
    #[serde(rename = "SyslogIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub syslog_ip: Option<String>,
}

/// <p>Contains the output of the <a>ModifyHsm</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ModifyHsmResponse {
    /// <p>The ARN of the HSM.</p>
    #[serde(rename = "HsmArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ModifyLunaClientRequest {
    /// <p>The new certificate for the client.</p>
    #[serde(rename = "Certificate")]
    pub certificate: String,
    /// <p>The ARN of the client.</p>
    #[serde(rename = "ClientArn")]
    pub client_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ModifyLunaClientResponse {
    /// <p>The ARN of the client.</p>
    #[serde(rename = "ClientArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RemoveTagsFromResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the AWS CloudHSM resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The tag key or keys to remove.</p> <p>Specify only the tag key to remove (not the value). To overwrite the value for an existing tag, use <a>AddTagsToResource</a>.</p>
    #[serde(rename = "TagKeyList")]
    pub tag_key_list: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RemoveTagsFromResourceResponse {
    /// <p>The status of the operation.</p>
    #[serde(rename = "Status")]
    pub status: String,
}

/// <p>A key-value pair that identifies or specifies metadata about an AWS CloudHSM resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>The key of the tag.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The value of the tag.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

/// Errors returned by AddTagsToResource
#[derive(Debug, PartialEq)]
pub enum AddTagsToResourceError {
    /// <p>Indicates that an internal error occurred.</p>
    CloudHsmInternal(String),
    /// <p>Indicates that an exception occurred in the AWS CloudHSM service.</p>
    CloudHsmService(String),
    /// <p>Indicates that one or more of the request parameters are not valid.</p>
    InvalidRequest(String),
}

impl AddTagsToResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddTagsToResourceError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "CloudHsmInternalException" => {
                    return RusotoError::Service(AddTagsToResourceError::CloudHsmInternal(
                        String::from(error_message),
                    ))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(AddTagsToResourceError::CloudHsmService(
                        String::from(error_message),
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(AddTagsToResourceError::InvalidRequest(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            AddTagsToResourceError::CloudHsmInternal(ref cause) => cause,
            AddTagsToResourceError::CloudHsmService(ref cause) => cause,
            AddTagsToResourceError::InvalidRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateHapg
#[derive(Debug, PartialEq)]
pub enum CreateHapgError {
    /// <p>Indicates that an internal error occurred.</p>
    CloudHsmInternal(String),
    /// <p>Indicates that an exception occurred in the AWS CloudHSM service.</p>
    CloudHsmService(String),
    /// <p>Indicates that one or more of the request parameters are not valid.</p>
    InvalidRequest(String),
}

impl CreateHapgError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateHapgError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "CloudHsmInternalException" => {
                    return RusotoError::Service(CreateHapgError::CloudHsmInternal(String::from(
                        error_message,
                    )))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(CreateHapgError::CloudHsmService(String::from(
                        error_message,
                    )))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateHapgError::InvalidRequest(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateHapgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateHapgError {
    fn description(&self) -> &str {
        match *self {
            CreateHapgError::CloudHsmInternal(ref cause) => cause,
            CreateHapgError::CloudHsmService(ref cause) => cause,
            CreateHapgError::InvalidRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateHsm
#[derive(Debug, PartialEq)]
pub enum CreateHsmError {
    /// <p>Indicates that an internal error occurred.</p>
    CloudHsmInternal(String),
    /// <p>Indicates that an exception occurred in the AWS CloudHSM service.</p>
    CloudHsmService(String),
    /// <p>Indicates that one or more of the request parameters are not valid.</p>
    InvalidRequest(String),
}

impl CreateHsmError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateHsmError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "CloudHsmInternalException" => {
                    return RusotoError::Service(CreateHsmError::CloudHsmInternal(String::from(
                        error_message,
                    )))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(CreateHsmError::CloudHsmService(String::from(
                        error_message,
                    )))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateHsmError::InvalidRequest(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateHsmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateHsmError {
    fn description(&self) -> &str {
        match *self {
            CreateHsmError::CloudHsmInternal(ref cause) => cause,
            CreateHsmError::CloudHsmService(ref cause) => cause,
            CreateHsmError::InvalidRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateLunaClient
#[derive(Debug, PartialEq)]
pub enum CreateLunaClientError {
    /// <p>Indicates that an internal error occurred.</p>
    CloudHsmInternal(String),
    /// <p>Indicates that an exception occurred in the AWS CloudHSM service.</p>
    CloudHsmService(String),
    /// <p>Indicates that one or more of the request parameters are not valid.</p>
    InvalidRequest(String),
}

impl CreateLunaClientError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateLunaClientError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "CloudHsmInternalException" => {
                    return RusotoError::Service(CreateLunaClientError::CloudHsmInternal(
                        String::from(error_message),
                    ))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(CreateLunaClientError::CloudHsmService(
                        String::from(error_message),
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateLunaClientError::InvalidRequest(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateLunaClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateLunaClientError {
    fn description(&self) -> &str {
        match *self {
            CreateLunaClientError::CloudHsmInternal(ref cause) => cause,
            CreateLunaClientError::CloudHsmService(ref cause) => cause,
            CreateLunaClientError::InvalidRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteHapg
#[derive(Debug, PartialEq)]
pub enum DeleteHapgError {
    /// <p>Indicates that an internal error occurred.</p>
    CloudHsmInternal(String),
    /// <p>Indicates that an exception occurred in the AWS CloudHSM service.</p>
    CloudHsmService(String),
    /// <p>Indicates that one or more of the request parameters are not valid.</p>
    InvalidRequest(String),
}

impl DeleteHapgError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteHapgError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "CloudHsmInternalException" => {
                    return RusotoError::Service(DeleteHapgError::CloudHsmInternal(String::from(
                        error_message,
                    )))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(DeleteHapgError::CloudHsmService(String::from(
                        error_message,
                    )))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteHapgError::InvalidRequest(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteHapgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteHapgError {
    fn description(&self) -> &str {
        match *self {
            DeleteHapgError::CloudHsmInternal(ref cause) => cause,
            DeleteHapgError::CloudHsmService(ref cause) => cause,
            DeleteHapgError::InvalidRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteHsm
#[derive(Debug, PartialEq)]
pub enum DeleteHsmError {
    /// <p>Indicates that an internal error occurred.</p>
    CloudHsmInternal(String),
    /// <p>Indicates that an exception occurred in the AWS CloudHSM service.</p>
    CloudHsmService(String),
    /// <p>Indicates that one or more of the request parameters are not valid.</p>
    InvalidRequest(String),
}

impl DeleteHsmError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteHsmError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "CloudHsmInternalException" => {
                    return RusotoError::Service(DeleteHsmError::CloudHsmInternal(String::from(
                        error_message,
                    )))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(DeleteHsmError::CloudHsmService(String::from(
                        error_message,
                    )))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteHsmError::InvalidRequest(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteHsmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteHsmError {
    fn description(&self) -> &str {
        match *self {
            DeleteHsmError::CloudHsmInternal(ref cause) => cause,
            DeleteHsmError::CloudHsmService(ref cause) => cause,
            DeleteHsmError::InvalidRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteLunaClient
#[derive(Debug, PartialEq)]
pub enum DeleteLunaClientError {
    /// <p>Indicates that an internal error occurred.</p>
    CloudHsmInternal(String),
    /// <p>Indicates that an exception occurred in the AWS CloudHSM service.</p>
    CloudHsmService(String),
    /// <p>Indicates that one or more of the request parameters are not valid.</p>
    InvalidRequest(String),
}

impl DeleteLunaClientError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteLunaClientError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "CloudHsmInternalException" => {
                    return RusotoError::Service(DeleteLunaClientError::CloudHsmInternal(
                        String::from(error_message),
                    ))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(DeleteLunaClientError::CloudHsmService(
                        String::from(error_message),
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteLunaClientError::InvalidRequest(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteLunaClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteLunaClientError {
    fn description(&self) -> &str {
        match *self {
            DeleteLunaClientError::CloudHsmInternal(ref cause) => cause,
            DeleteLunaClientError::CloudHsmService(ref cause) => cause,
            DeleteLunaClientError::InvalidRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeHapg
#[derive(Debug, PartialEq)]
pub enum DescribeHapgError {
    /// <p>Indicates that an internal error occurred.</p>
    CloudHsmInternal(String),
    /// <p>Indicates that an exception occurred in the AWS CloudHSM service.</p>
    CloudHsmService(String),
    /// <p>Indicates that one or more of the request parameters are not valid.</p>
    InvalidRequest(String),
}

impl DescribeHapgError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeHapgError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "CloudHsmInternalException" => {
                    return RusotoError::Service(DescribeHapgError::CloudHsmInternal(String::from(
                        error_message,
                    )))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(DescribeHapgError::CloudHsmService(String::from(
                        error_message,
                    )))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeHapgError::InvalidRequest(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeHapgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeHapgError {
    fn description(&self) -> &str {
        match *self {
            DescribeHapgError::CloudHsmInternal(ref cause) => cause,
            DescribeHapgError::CloudHsmService(ref cause) => cause,
            DescribeHapgError::InvalidRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeHsm
#[derive(Debug, PartialEq)]
pub enum DescribeHsmError {
    /// <p>Indicates that an internal error occurred.</p>
    CloudHsmInternal(String),
    /// <p>Indicates that an exception occurred in the AWS CloudHSM service.</p>
    CloudHsmService(String),
    /// <p>Indicates that one or more of the request parameters are not valid.</p>
    InvalidRequest(String),
}

impl DescribeHsmError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeHsmError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "CloudHsmInternalException" => {
                    return RusotoError::Service(DescribeHsmError::CloudHsmInternal(String::from(
                        error_message,
                    )))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(DescribeHsmError::CloudHsmService(String::from(
                        error_message,
                    )))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeHsmError::InvalidRequest(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeHsmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeHsmError {
    fn description(&self) -> &str {
        match *self {
            DescribeHsmError::CloudHsmInternal(ref cause) => cause,
            DescribeHsmError::CloudHsmService(ref cause) => cause,
            DescribeHsmError::InvalidRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeLunaClient
#[derive(Debug, PartialEq)]
pub enum DescribeLunaClientError {
    /// <p>Indicates that an internal error occurred.</p>
    CloudHsmInternal(String),
    /// <p>Indicates that an exception occurred in the AWS CloudHSM service.</p>
    CloudHsmService(String),
    /// <p>Indicates that one or more of the request parameters are not valid.</p>
    InvalidRequest(String),
}

impl DescribeLunaClientError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeLunaClientError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "CloudHsmInternalException" => {
                    return RusotoError::Service(DescribeLunaClientError::CloudHsmInternal(
                        String::from(error_message),
                    ))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(DescribeLunaClientError::CloudHsmService(
                        String::from(error_message),
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeLunaClientError::InvalidRequest(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeLunaClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeLunaClientError {
    fn description(&self) -> &str {
        match *self {
            DescribeLunaClientError::CloudHsmInternal(ref cause) => cause,
            DescribeLunaClientError::CloudHsmService(ref cause) => cause,
            DescribeLunaClientError::InvalidRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by GetConfig
#[derive(Debug, PartialEq)]
pub enum GetConfigError {
    /// <p>Indicates that an internal error occurred.</p>
    CloudHsmInternal(String),
    /// <p>Indicates that an exception occurred in the AWS CloudHSM service.</p>
    CloudHsmService(String),
    /// <p>Indicates that one or more of the request parameters are not valid.</p>
    InvalidRequest(String),
}

impl GetConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetConfigError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "CloudHsmInternalException" => {
                    return RusotoError::Service(GetConfigError::CloudHsmInternal(String::from(
                        error_message,
                    )))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(GetConfigError::CloudHsmService(String::from(
                        error_message,
                    )))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetConfigError::InvalidRequest(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetConfigError {
    fn description(&self) -> &str {
        match *self {
            GetConfigError::CloudHsmInternal(ref cause) => cause,
            GetConfigError::CloudHsmService(ref cause) => cause,
            GetConfigError::InvalidRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by ListAvailableZones
#[derive(Debug, PartialEq)]
pub enum ListAvailableZonesError {
    /// <p>Indicates that an internal error occurred.</p>
    CloudHsmInternal(String),
    /// <p>Indicates that an exception occurred in the AWS CloudHSM service.</p>
    CloudHsmService(String),
    /// <p>Indicates that one or more of the request parameters are not valid.</p>
    InvalidRequest(String),
}

impl ListAvailableZonesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAvailableZonesError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "CloudHsmInternalException" => {
                    return RusotoError::Service(ListAvailableZonesError::CloudHsmInternal(
                        String::from(error_message),
                    ))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(ListAvailableZonesError::CloudHsmService(
                        String::from(error_message),
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListAvailableZonesError::InvalidRequest(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListAvailableZonesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAvailableZonesError {
    fn description(&self) -> &str {
        match *self {
            ListAvailableZonesError::CloudHsmInternal(ref cause) => cause,
            ListAvailableZonesError::CloudHsmService(ref cause) => cause,
            ListAvailableZonesError::InvalidRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by ListHapgs
#[derive(Debug, PartialEq)]
pub enum ListHapgsError {
    /// <p>Indicates that an internal error occurred.</p>
    CloudHsmInternal(String),
    /// <p>Indicates that an exception occurred in the AWS CloudHSM service.</p>
    CloudHsmService(String),
    /// <p>Indicates that one or more of the request parameters are not valid.</p>
    InvalidRequest(String),
}

impl ListHapgsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListHapgsError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "CloudHsmInternalException" => {
                    return RusotoError::Service(ListHapgsError::CloudHsmInternal(String::from(
                        error_message,
                    )))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(ListHapgsError::CloudHsmService(String::from(
                        error_message,
                    )))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListHapgsError::InvalidRequest(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListHapgsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListHapgsError {
    fn description(&self) -> &str {
        match *self {
            ListHapgsError::CloudHsmInternal(ref cause) => cause,
            ListHapgsError::CloudHsmService(ref cause) => cause,
            ListHapgsError::InvalidRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by ListHsms
#[derive(Debug, PartialEq)]
pub enum ListHsmsError {
    /// <p>Indicates that an internal error occurred.</p>
    CloudHsmInternal(String),
    /// <p>Indicates that an exception occurred in the AWS CloudHSM service.</p>
    CloudHsmService(String),
    /// <p>Indicates that one or more of the request parameters are not valid.</p>
    InvalidRequest(String),
}

impl ListHsmsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListHsmsError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "CloudHsmInternalException" => {
                    return RusotoError::Service(ListHsmsError::CloudHsmInternal(String::from(
                        error_message,
                    )))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(ListHsmsError::CloudHsmService(String::from(
                        error_message,
                    )))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListHsmsError::InvalidRequest(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListHsmsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListHsmsError {
    fn description(&self) -> &str {
        match *self {
            ListHsmsError::CloudHsmInternal(ref cause) => cause,
            ListHsmsError::CloudHsmService(ref cause) => cause,
            ListHsmsError::InvalidRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by ListLunaClients
#[derive(Debug, PartialEq)]
pub enum ListLunaClientsError {
    /// <p>Indicates that an internal error occurred.</p>
    CloudHsmInternal(String),
    /// <p>Indicates that an exception occurred in the AWS CloudHSM service.</p>
    CloudHsmService(String),
    /// <p>Indicates that one or more of the request parameters are not valid.</p>
    InvalidRequest(String),
}

impl ListLunaClientsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListLunaClientsError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "CloudHsmInternalException" => {
                    return RusotoError::Service(ListLunaClientsError::CloudHsmInternal(
                        String::from(error_message),
                    ))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(ListLunaClientsError::CloudHsmService(
                        String::from(error_message),
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListLunaClientsError::InvalidRequest(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListLunaClientsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListLunaClientsError {
    fn description(&self) -> &str {
        match *self {
            ListLunaClientsError::CloudHsmInternal(ref cause) => cause,
            ListLunaClientsError::CloudHsmService(ref cause) => cause,
            ListLunaClientsError::InvalidRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>Indicates that an internal error occurred.</p>
    CloudHsmInternal(String),
    /// <p>Indicates that an exception occurred in the AWS CloudHSM service.</p>
    CloudHsmService(String),
    /// <p>Indicates that one or more of the request parameters are not valid.</p>
    InvalidRequest(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "CloudHsmInternalException" => {
                    return RusotoError::Service(ListTagsForResourceError::CloudHsmInternal(
                        String::from(error_message),
                    ))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(ListTagsForResourceError::CloudHsmService(
                        String::from(error_message),
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidRequest(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            ListTagsForResourceError::CloudHsmInternal(ref cause) => cause,
            ListTagsForResourceError::CloudHsmService(ref cause) => cause,
            ListTagsForResourceError::InvalidRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by ModifyHapg
#[derive(Debug, PartialEq)]
pub enum ModifyHapgError {
    /// <p>Indicates that an internal error occurred.</p>
    CloudHsmInternal(String),
    /// <p>Indicates that an exception occurred in the AWS CloudHSM service.</p>
    CloudHsmService(String),
    /// <p>Indicates that one or more of the request parameters are not valid.</p>
    InvalidRequest(String),
}

impl ModifyHapgError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ModifyHapgError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "CloudHsmInternalException" => {
                    return RusotoError::Service(ModifyHapgError::CloudHsmInternal(String::from(
                        error_message,
                    )))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(ModifyHapgError::CloudHsmService(String::from(
                        error_message,
                    )))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ModifyHapgError::InvalidRequest(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ModifyHapgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifyHapgError {
    fn description(&self) -> &str {
        match *self {
            ModifyHapgError::CloudHsmInternal(ref cause) => cause,
            ModifyHapgError::CloudHsmService(ref cause) => cause,
            ModifyHapgError::InvalidRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by ModifyHsm
#[derive(Debug, PartialEq)]
pub enum ModifyHsmError {
    /// <p>Indicates that an internal error occurred.</p>
    CloudHsmInternal(String),
    /// <p>Indicates that an exception occurred in the AWS CloudHSM service.</p>
    CloudHsmService(String),
    /// <p>Indicates that one or more of the request parameters are not valid.</p>
    InvalidRequest(String),
}

impl ModifyHsmError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ModifyHsmError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "CloudHsmInternalException" => {
                    return RusotoError::Service(ModifyHsmError::CloudHsmInternal(String::from(
                        error_message,
                    )))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(ModifyHsmError::CloudHsmService(String::from(
                        error_message,
                    )))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ModifyHsmError::InvalidRequest(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ModifyHsmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifyHsmError {
    fn description(&self) -> &str {
        match *self {
            ModifyHsmError::CloudHsmInternal(ref cause) => cause,
            ModifyHsmError::CloudHsmService(ref cause) => cause,
            ModifyHsmError::InvalidRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by ModifyLunaClient
#[derive(Debug, PartialEq)]
pub enum ModifyLunaClientError {
    /// <p>Indicates that an exception occurred in the AWS CloudHSM service.</p>
    CloudHsmService(String),
}

impl ModifyLunaClientError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ModifyLunaClientError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "CloudHsmServiceException" => {
                    return RusotoError::Service(ModifyLunaClientError::CloudHsmService(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ModifyLunaClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifyLunaClientError {
    fn description(&self) -> &str {
        match *self {
            ModifyLunaClientError::CloudHsmService(ref cause) => cause,
        }
    }
}
/// Errors returned by RemoveTagsFromResource
#[derive(Debug, PartialEq)]
pub enum RemoveTagsFromResourceError {
    /// <p>Indicates that an internal error occurred.</p>
    CloudHsmInternal(String),
    /// <p>Indicates that an exception occurred in the AWS CloudHSM service.</p>
    CloudHsmService(String),
    /// <p>Indicates that one or more of the request parameters are not valid.</p>
    InvalidRequest(String),
}

impl RemoveTagsFromResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RemoveTagsFromResourceError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "CloudHsmInternalException" => {
                    return RusotoError::Service(RemoveTagsFromResourceError::CloudHsmInternal(
                        String::from(error_message),
                    ))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(RemoveTagsFromResourceError::CloudHsmService(
                        String::from(error_message),
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(RemoveTagsFromResourceError::InvalidRequest(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            RemoveTagsFromResourceError::CloudHsmInternal(ref cause) => cause,
            RemoveTagsFromResourceError::CloudHsmService(ref cause) => cause,
            RemoveTagsFromResourceError::InvalidRequest(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the CloudHSM API. CloudHSM clients implement this trait.
pub trait CloudHsm {
    /// <p>This is documentation for <b>AWS CloudHSM Classic</b>. For more information, see <a href="http://aws.amazon.com/cloudhsm/faqs-classic/">AWS CloudHSM Classic FAQs</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/classic/userguide/">AWS CloudHSM Classic User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/classic/APIReference/">AWS CloudHSM Classic API Reference</a>.</p> <p> <b>For information about the current version of AWS CloudHSM</b>, see <a href="http://aws.amazon.com/cloudhsm/">AWS CloudHSM</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/latest/userguide/">AWS CloudHSM User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/latest/APIReference/">AWS CloudHSM API Reference</a>.</p> <p>Adds or overwrites one or more tags for the specified AWS CloudHSM resource.</p> <p>Each tag consists of a key and a value. Tag keys must be unique to each resource.</p>
    fn add_tags_to_resource(
        &self,
        input: AddTagsToResourceRequest,
    ) -> RusotoFuture<AddTagsToResourceResponse, AddTagsToResourceError>;

    /// <p>This is documentation for <b>AWS CloudHSM Classic</b>. For more information, see <a href="http://aws.amazon.com/cloudhsm/faqs-classic/">AWS CloudHSM Classic FAQs</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/classic/userguide/">AWS CloudHSM Classic User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/classic/APIReference/">AWS CloudHSM Classic API Reference</a>.</p> <p> <b>For information about the current version of AWS CloudHSM</b>, see <a href="http://aws.amazon.com/cloudhsm/">AWS CloudHSM</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/latest/userguide/">AWS CloudHSM User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/latest/APIReference/">AWS CloudHSM API Reference</a>.</p> <p>Creates a high-availability partition group. A high-availability partition group is a group of partitions that spans multiple physical HSMs.</p>
    fn create_hapg(
        &self,
        input: CreateHapgRequest,
    ) -> RusotoFuture<CreateHapgResponse, CreateHapgError>;

    /// <p><p>This is documentation for <b>AWS CloudHSM Classic</b>. For more information, see <a href="http://aws.amazon.com/cloudhsm/faqs-classic/">AWS CloudHSM Classic FAQs</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/classic/userguide/">AWS CloudHSM Classic User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/classic/APIReference/">AWS CloudHSM Classic API Reference</a>.</p> <p> <b>For information about the current version of AWS CloudHSM</b>, see <a href="http://aws.amazon.com/cloudhsm/">AWS CloudHSM</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/latest/userguide/">AWS CloudHSM User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/latest/APIReference/">AWS CloudHSM API Reference</a>.</p> <p>Creates an uninitialized HSM instance.</p> <p>There is an upfront fee charged for each HSM instance that you create with the <code>CreateHsm</code> operation. If you accidentally provision an HSM and want to request a refund, delete the instance using the <a>DeleteHsm</a> operation, go to the <a href="https://console.aws.amazon.com/support/home">AWS Support Center</a>, create a new case, and select <b>Account and Billing Support</b>.</p> <important> <p>It can take up to 20 minutes to create and provision an HSM. You can monitor the status of the HSM with the <a>DescribeHsm</a> operation. The HSM is ready to be initialized when the status changes to <code>RUNNING</code>.</p> </important></p>
    fn create_hsm(
        &self,
        input: CreateHsmRequest,
    ) -> RusotoFuture<CreateHsmResponse, CreateHsmError>;

    /// <p>This is documentation for <b>AWS CloudHSM Classic</b>. For more information, see <a href="http://aws.amazon.com/cloudhsm/faqs-classic/">AWS CloudHSM Classic FAQs</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/classic/userguide/">AWS CloudHSM Classic User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/classic/APIReference/">AWS CloudHSM Classic API Reference</a>.</p> <p> <b>For information about the current version of AWS CloudHSM</b>, see <a href="http://aws.amazon.com/cloudhsm/">AWS CloudHSM</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/latest/userguide/">AWS CloudHSM User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/latest/APIReference/">AWS CloudHSM API Reference</a>.</p> <p>Creates an HSM client.</p>
    fn create_luna_client(
        &self,
        input: CreateLunaClientRequest,
    ) -> RusotoFuture<CreateLunaClientResponse, CreateLunaClientError>;

    /// <p>This is documentation for <b>AWS CloudHSM Classic</b>. For more information, see <a href="http://aws.amazon.com/cloudhsm/faqs-classic/">AWS CloudHSM Classic FAQs</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/classic/userguide/">AWS CloudHSM Classic User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/classic/APIReference/">AWS CloudHSM Classic API Reference</a>.</p> <p> <b>For information about the current version of AWS CloudHSM</b>, see <a href="http://aws.amazon.com/cloudhsm/">AWS CloudHSM</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/latest/userguide/">AWS CloudHSM User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/latest/APIReference/">AWS CloudHSM API Reference</a>.</p> <p>Deletes a high-availability partition group.</p>
    fn delete_hapg(
        &self,
        input: DeleteHapgRequest,
    ) -> RusotoFuture<DeleteHapgResponse, DeleteHapgError>;

    /// <p>This is documentation for <b>AWS CloudHSM Classic</b>. For more information, see <a href="http://aws.amazon.com/cloudhsm/faqs-classic/">AWS CloudHSM Classic FAQs</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/classic/userguide/">AWS CloudHSM Classic User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/classic/APIReference/">AWS CloudHSM Classic API Reference</a>.</p> <p> <b>For information about the current version of AWS CloudHSM</b>, see <a href="http://aws.amazon.com/cloudhsm/">AWS CloudHSM</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/latest/userguide/">AWS CloudHSM User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/latest/APIReference/">AWS CloudHSM API Reference</a>.</p> <p>Deletes an HSM. After completion, this operation cannot be undone and your key material cannot be recovered.</p>
    fn delete_hsm(
        &self,
        input: DeleteHsmRequest,
    ) -> RusotoFuture<DeleteHsmResponse, DeleteHsmError>;

    /// <p>This is documentation for <b>AWS CloudHSM Classic</b>. For more information, see <a href="http://aws.amazon.com/cloudhsm/faqs-classic/">AWS CloudHSM Classic FAQs</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/classic/userguide/">AWS CloudHSM Classic User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/classic/APIReference/">AWS CloudHSM Classic API Reference</a>.</p> <p> <b>For information about the current version of AWS CloudHSM</b>, see <a href="http://aws.amazon.com/cloudhsm/">AWS CloudHSM</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/latest/userguide/">AWS CloudHSM User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/latest/APIReference/">AWS CloudHSM API Reference</a>.</p> <p>Deletes a client.</p>
    fn delete_luna_client(
        &self,
        input: DeleteLunaClientRequest,
    ) -> RusotoFuture<DeleteLunaClientResponse, DeleteLunaClientError>;

    /// <p>This is documentation for <b>AWS CloudHSM Classic</b>. For more information, see <a href="http://aws.amazon.com/cloudhsm/faqs-classic/">AWS CloudHSM Classic FAQs</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/classic/userguide/">AWS CloudHSM Classic User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/classic/APIReference/">AWS CloudHSM Classic API Reference</a>.</p> <p> <b>For information about the current version of AWS CloudHSM</b>, see <a href="http://aws.amazon.com/cloudhsm/">AWS CloudHSM</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/latest/userguide/">AWS CloudHSM User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/latest/APIReference/">AWS CloudHSM API Reference</a>.</p> <p>Retrieves information about a high-availability partition group.</p>
    fn describe_hapg(
        &self,
        input: DescribeHapgRequest,
    ) -> RusotoFuture<DescribeHapgResponse, DescribeHapgError>;

    /// <p>This is documentation for <b>AWS CloudHSM Classic</b>. For more information, see <a href="http://aws.amazon.com/cloudhsm/faqs-classic/">AWS CloudHSM Classic FAQs</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/classic/userguide/">AWS CloudHSM Classic User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/classic/APIReference/">AWS CloudHSM Classic API Reference</a>.</p> <p> <b>For information about the current version of AWS CloudHSM</b>, see <a href="http://aws.amazon.com/cloudhsm/">AWS CloudHSM</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/latest/userguide/">AWS CloudHSM User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/latest/APIReference/">AWS CloudHSM API Reference</a>.</p> <p>Retrieves information about an HSM. You can identify the HSM by its ARN or its serial number.</p>
    fn describe_hsm(
        &self,
        input: DescribeHsmRequest,
    ) -> RusotoFuture<DescribeHsmResponse, DescribeHsmError>;

    /// <p>This is documentation for <b>AWS CloudHSM Classic</b>. For more information, see <a href="http://aws.amazon.com/cloudhsm/faqs-classic/">AWS CloudHSM Classic FAQs</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/classic/userguide/">AWS CloudHSM Classic User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/classic/APIReference/">AWS CloudHSM Classic API Reference</a>.</p> <p> <b>For information about the current version of AWS CloudHSM</b>, see <a href="http://aws.amazon.com/cloudhsm/">AWS CloudHSM</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/latest/userguide/">AWS CloudHSM User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/latest/APIReference/">AWS CloudHSM API Reference</a>.</p> <p>Retrieves information about an HSM client.</p>
    fn describe_luna_client(
        &self,
        input: DescribeLunaClientRequest,
    ) -> RusotoFuture<DescribeLunaClientResponse, DescribeLunaClientError>;

    /// <p>This is documentation for <b>AWS CloudHSM Classic</b>. For more information, see <a href="http://aws.amazon.com/cloudhsm/faqs-classic/">AWS CloudHSM Classic FAQs</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/classic/userguide/">AWS CloudHSM Classic User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/classic/APIReference/">AWS CloudHSM Classic API Reference</a>.</p> <p> <b>For information about the current version of AWS CloudHSM</b>, see <a href="http://aws.amazon.com/cloudhsm/">AWS CloudHSM</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/latest/userguide/">AWS CloudHSM User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/latest/APIReference/">AWS CloudHSM API Reference</a>.</p> <p>Gets the configuration files necessary to connect to all high availability partition groups the client is associated with.</p>
    fn get_config(
        &self,
        input: GetConfigRequest,
    ) -> RusotoFuture<GetConfigResponse, GetConfigError>;

    /// <p>This is documentation for <b>AWS CloudHSM Classic</b>. For more information, see <a href="http://aws.amazon.com/cloudhsm/faqs-classic/">AWS CloudHSM Classic FAQs</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/classic/userguide/">AWS CloudHSM Classic User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/classic/APIReference/">AWS CloudHSM Classic API Reference</a>.</p> <p> <b>For information about the current version of AWS CloudHSM</b>, see <a href="http://aws.amazon.com/cloudhsm/">AWS CloudHSM</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/latest/userguide/">AWS CloudHSM User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/latest/APIReference/">AWS CloudHSM API Reference</a>.</p> <p>Lists the Availability Zones that have available AWS CloudHSM capacity.</p>
    fn list_available_zones(
        &self,
    ) -> RusotoFuture<ListAvailableZonesResponse, ListAvailableZonesError>;

    /// <p>This is documentation for <b>AWS CloudHSM Classic</b>. For more information, see <a href="http://aws.amazon.com/cloudhsm/faqs-classic/">AWS CloudHSM Classic FAQs</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/classic/userguide/">AWS CloudHSM Classic User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/classic/APIReference/">AWS CloudHSM Classic API Reference</a>.</p> <p> <b>For information about the current version of AWS CloudHSM</b>, see <a href="http://aws.amazon.com/cloudhsm/">AWS CloudHSM</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/latest/userguide/">AWS CloudHSM User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/latest/APIReference/">AWS CloudHSM API Reference</a>.</p> <p>Lists the high-availability partition groups for the account.</p> <p>This operation supports pagination with the use of the <code>NextToken</code> member. If more results are available, the <code>NextToken</code> member of the response contains a token that you pass in the next call to <code>ListHapgs</code> to retrieve the next set of items.</p>
    fn list_hapgs(
        &self,
        input: ListHapgsRequest,
    ) -> RusotoFuture<ListHapgsResponse, ListHapgsError>;

    /// <p>This is documentation for <b>AWS CloudHSM Classic</b>. For more information, see <a href="http://aws.amazon.com/cloudhsm/faqs-classic/">AWS CloudHSM Classic FAQs</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/classic/userguide/">AWS CloudHSM Classic User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/classic/APIReference/">AWS CloudHSM Classic API Reference</a>.</p> <p> <b>For information about the current version of AWS CloudHSM</b>, see <a href="http://aws.amazon.com/cloudhsm/">AWS CloudHSM</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/latest/userguide/">AWS CloudHSM User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/latest/APIReference/">AWS CloudHSM API Reference</a>.</p> <p>Retrieves the identifiers of all of the HSMs provisioned for the current customer.</p> <p>This operation supports pagination with the use of the <code>NextToken</code> member. If more results are available, the <code>NextToken</code> member of the response contains a token that you pass in the next call to <code>ListHsms</code> to retrieve the next set of items.</p>
    fn list_hsms(&self, input: ListHsmsRequest) -> RusotoFuture<ListHsmsResponse, ListHsmsError>;

    /// <p>This is documentation for <b>AWS CloudHSM Classic</b>. For more information, see <a href="http://aws.amazon.com/cloudhsm/faqs-classic/">AWS CloudHSM Classic FAQs</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/classic/userguide/">AWS CloudHSM Classic User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/classic/APIReference/">AWS CloudHSM Classic API Reference</a>.</p> <p> <b>For information about the current version of AWS CloudHSM</b>, see <a href="http://aws.amazon.com/cloudhsm/">AWS CloudHSM</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/latest/userguide/">AWS CloudHSM User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/latest/APIReference/">AWS CloudHSM API Reference</a>.</p> <p>Lists all of the clients.</p> <p>This operation supports pagination with the use of the <code>NextToken</code> member. If more results are available, the <code>NextToken</code> member of the response contains a token that you pass in the next call to <code>ListLunaClients</code> to retrieve the next set of items.</p>
    fn list_luna_clients(
        &self,
        input: ListLunaClientsRequest,
    ) -> RusotoFuture<ListLunaClientsResponse, ListLunaClientsError>;

    /// <p>This is documentation for <b>AWS CloudHSM Classic</b>. For more information, see <a href="http://aws.amazon.com/cloudhsm/faqs-classic/">AWS CloudHSM Classic FAQs</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/classic/userguide/">AWS CloudHSM Classic User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/classic/APIReference/">AWS CloudHSM Classic API Reference</a>.</p> <p> <b>For information about the current version of AWS CloudHSM</b>, see <a href="http://aws.amazon.com/cloudhsm/">AWS CloudHSM</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/latest/userguide/">AWS CloudHSM User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/latest/APIReference/">AWS CloudHSM API Reference</a>.</p> <p>Returns a list of all tags for the specified AWS CloudHSM resource.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> RusotoFuture<ListTagsForResourceResponse, ListTagsForResourceError>;

    /// <p>This is documentation for <b>AWS CloudHSM Classic</b>. For more information, see <a href="http://aws.amazon.com/cloudhsm/faqs-classic/">AWS CloudHSM Classic FAQs</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/classic/userguide/">AWS CloudHSM Classic User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/classic/APIReference/">AWS CloudHSM Classic API Reference</a>.</p> <p> <b>For information about the current version of AWS CloudHSM</b>, see <a href="http://aws.amazon.com/cloudhsm/">AWS CloudHSM</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/latest/userguide/">AWS CloudHSM User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/latest/APIReference/">AWS CloudHSM API Reference</a>.</p> <p>Modifies an existing high-availability partition group.</p>
    fn modify_hapg(
        &self,
        input: ModifyHapgRequest,
    ) -> RusotoFuture<ModifyHapgResponse, ModifyHapgError>;

    /// <p><p>This is documentation for <b>AWS CloudHSM Classic</b>. For more information, see <a href="http://aws.amazon.com/cloudhsm/faqs-classic/">AWS CloudHSM Classic FAQs</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/classic/userguide/">AWS CloudHSM Classic User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/classic/APIReference/">AWS CloudHSM Classic API Reference</a>.</p> <p> <b>For information about the current version of AWS CloudHSM</b>, see <a href="http://aws.amazon.com/cloudhsm/">AWS CloudHSM</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/latest/userguide/">AWS CloudHSM User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/latest/APIReference/">AWS CloudHSM API Reference</a>.</p> <p>Modifies an HSM.</p> <important> <p>This operation can result in the HSM being offline for up to 15 minutes while the AWS CloudHSM service is reconfigured. If you are modifying a production HSM, you should ensure that your AWS CloudHSM service is configured for high availability, and consider executing this operation during a maintenance window.</p> </important></p>
    fn modify_hsm(
        &self,
        input: ModifyHsmRequest,
    ) -> RusotoFuture<ModifyHsmResponse, ModifyHsmError>;

    /// <p>This is documentation for <b>AWS CloudHSM Classic</b>. For more information, see <a href="http://aws.amazon.com/cloudhsm/faqs-classic/">AWS CloudHSM Classic FAQs</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/classic/userguide/">AWS CloudHSM Classic User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/classic/APIReference/">AWS CloudHSM Classic API Reference</a>.</p> <p> <b>For information about the current version of AWS CloudHSM</b>, see <a href="http://aws.amazon.com/cloudhsm/">AWS CloudHSM</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/latest/userguide/">AWS CloudHSM User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/latest/APIReference/">AWS CloudHSM API Reference</a>.</p> <p>Modifies the certificate used by the client.</p> <p>This action can potentially start a workflow to install the new certificate on the client's HSMs.</p>
    fn modify_luna_client(
        &self,
        input: ModifyLunaClientRequest,
    ) -> RusotoFuture<ModifyLunaClientResponse, ModifyLunaClientError>;

    /// <p>This is documentation for <b>AWS CloudHSM Classic</b>. For more information, see <a href="http://aws.amazon.com/cloudhsm/faqs-classic/">AWS CloudHSM Classic FAQs</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/classic/userguide/">AWS CloudHSM Classic User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/classic/APIReference/">AWS CloudHSM Classic API Reference</a>.</p> <p> <b>For information about the current version of AWS CloudHSM</b>, see <a href="http://aws.amazon.com/cloudhsm/">AWS CloudHSM</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/latest/userguide/">AWS CloudHSM User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/latest/APIReference/">AWS CloudHSM API Reference</a>.</p> <p>Removes one or more tags from the specified AWS CloudHSM resource.</p> <p>To remove a tag, specify only the tag key to remove (not the value). To overwrite the value for an existing tag, use <a>AddTagsToResource</a>.</p>
    fn remove_tags_from_resource(
        &self,
        input: RemoveTagsFromResourceRequest,
    ) -> RusotoFuture<RemoveTagsFromResourceResponse, RemoveTagsFromResourceError>;
}
/// A client for the CloudHSM API.
#[derive(Clone)]
pub struct CloudHsmClient {
    client: Client,
    region: region::Region,
}

impl CloudHsmClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> CloudHsmClient {
        CloudHsmClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> CloudHsmClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        CloudHsmClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl CloudHsm for CloudHsmClient {
    /// <p>This is documentation for <b>AWS CloudHSM Classic</b>. For more information, see <a href="http://aws.amazon.com/cloudhsm/faqs-classic/">AWS CloudHSM Classic FAQs</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/classic/userguide/">AWS CloudHSM Classic User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/classic/APIReference/">AWS CloudHSM Classic API Reference</a>.</p> <p> <b>For information about the current version of AWS CloudHSM</b>, see <a href="http://aws.amazon.com/cloudhsm/">AWS CloudHSM</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/latest/userguide/">AWS CloudHSM User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/latest/APIReference/">AWS CloudHSM API Reference</a>.</p> <p>Adds or overwrites one or more tags for the specified AWS CloudHSM resource.</p> <p>Each tag consists of a key and a value. Tag keys must be unique to each resource.</p>
    fn add_tags_to_resource(
        &self,
        input: AddTagsToResourceRequest,
    ) -> RusotoFuture<AddTagsToResourceResponse, AddTagsToResourceError> {
        let mut request = SignedRequest::new("POST", "cloudhsm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CloudHsmFrontendService.AddTagsToResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<AddTagsToResourceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(AddTagsToResourceError::from_response(response))),
                )
            }
        })
    }

    /// <p>This is documentation for <b>AWS CloudHSM Classic</b>. For more information, see <a href="http://aws.amazon.com/cloudhsm/faqs-classic/">AWS CloudHSM Classic FAQs</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/classic/userguide/">AWS CloudHSM Classic User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/classic/APIReference/">AWS CloudHSM Classic API Reference</a>.</p> <p> <b>For information about the current version of AWS CloudHSM</b>, see <a href="http://aws.amazon.com/cloudhsm/">AWS CloudHSM</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/latest/userguide/">AWS CloudHSM User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/latest/APIReference/">AWS CloudHSM API Reference</a>.</p> <p>Creates a high-availability partition group. A high-availability partition group is a group of partitions that spans multiple physical HSMs.</p>
    fn create_hapg(
        &self,
        input: CreateHapgRequest,
    ) -> RusotoFuture<CreateHapgResponse, CreateHapgError> {
        let mut request = SignedRequest::new("POST", "cloudhsm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CloudHsmFrontendService.CreateHapg");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<CreateHapgResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateHapgError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>This is documentation for <b>AWS CloudHSM Classic</b>. For more information, see <a href="http://aws.amazon.com/cloudhsm/faqs-classic/">AWS CloudHSM Classic FAQs</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/classic/userguide/">AWS CloudHSM Classic User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/classic/APIReference/">AWS CloudHSM Classic API Reference</a>.</p> <p> <b>For information about the current version of AWS CloudHSM</b>, see <a href="http://aws.amazon.com/cloudhsm/">AWS CloudHSM</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/latest/userguide/">AWS CloudHSM User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/latest/APIReference/">AWS CloudHSM API Reference</a>.</p> <p>Creates an uninitialized HSM instance.</p> <p>There is an upfront fee charged for each HSM instance that you create with the <code>CreateHsm</code> operation. If you accidentally provision an HSM and want to request a refund, delete the instance using the <a>DeleteHsm</a> operation, go to the <a href="https://console.aws.amazon.com/support/home">AWS Support Center</a>, create a new case, and select <b>Account and Billing Support</b>.</p> <important> <p>It can take up to 20 minutes to create and provision an HSM. You can monitor the status of the HSM with the <a>DescribeHsm</a> operation. The HSM is ready to be initialized when the status changes to <code>RUNNING</code>.</p> </important></p>
    fn create_hsm(
        &self,
        input: CreateHsmRequest,
    ) -> RusotoFuture<CreateHsmResponse, CreateHsmError> {
        let mut request = SignedRequest::new("POST", "cloudhsm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CloudHsmFrontendService.CreateHsm");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<CreateHsmResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateHsmError::from_response(response))),
                )
            }
        })
    }

    /// <p>This is documentation for <b>AWS CloudHSM Classic</b>. For more information, see <a href="http://aws.amazon.com/cloudhsm/faqs-classic/">AWS CloudHSM Classic FAQs</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/classic/userguide/">AWS CloudHSM Classic User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/classic/APIReference/">AWS CloudHSM Classic API Reference</a>.</p> <p> <b>For information about the current version of AWS CloudHSM</b>, see <a href="http://aws.amazon.com/cloudhsm/">AWS CloudHSM</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/latest/userguide/">AWS CloudHSM User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/latest/APIReference/">AWS CloudHSM API Reference</a>.</p> <p>Creates an HSM client.</p>
    fn create_luna_client(
        &self,
        input: CreateLunaClientRequest,
    ) -> RusotoFuture<CreateLunaClientResponse, CreateLunaClientError> {
        let mut request = SignedRequest::new("POST", "cloudhsm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CloudHsmFrontendService.CreateLunaClient");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<CreateLunaClientResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateLunaClientError::from_response(response))),
                )
            }
        })
    }

    /// <p>This is documentation for <b>AWS CloudHSM Classic</b>. For more information, see <a href="http://aws.amazon.com/cloudhsm/faqs-classic/">AWS CloudHSM Classic FAQs</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/classic/userguide/">AWS CloudHSM Classic User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/classic/APIReference/">AWS CloudHSM Classic API Reference</a>.</p> <p> <b>For information about the current version of AWS CloudHSM</b>, see <a href="http://aws.amazon.com/cloudhsm/">AWS CloudHSM</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/latest/userguide/">AWS CloudHSM User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/latest/APIReference/">AWS CloudHSM API Reference</a>.</p> <p>Deletes a high-availability partition group.</p>
    fn delete_hapg(
        &self,
        input: DeleteHapgRequest,
    ) -> RusotoFuture<DeleteHapgResponse, DeleteHapgError> {
        let mut request = SignedRequest::new("POST", "cloudhsm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CloudHsmFrontendService.DeleteHapg");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<DeleteHapgResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteHapgError::from_response(response))),
                )
            }
        })
    }

    /// <p>This is documentation for <b>AWS CloudHSM Classic</b>. For more information, see <a href="http://aws.amazon.com/cloudhsm/faqs-classic/">AWS CloudHSM Classic FAQs</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/classic/userguide/">AWS CloudHSM Classic User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/classic/APIReference/">AWS CloudHSM Classic API Reference</a>.</p> <p> <b>For information about the current version of AWS CloudHSM</b>, see <a href="http://aws.amazon.com/cloudhsm/">AWS CloudHSM</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/latest/userguide/">AWS CloudHSM User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/latest/APIReference/">AWS CloudHSM API Reference</a>.</p> <p>Deletes an HSM. After completion, this operation cannot be undone and your key material cannot be recovered.</p>
    fn delete_hsm(
        &self,
        input: DeleteHsmRequest,
    ) -> RusotoFuture<DeleteHsmResponse, DeleteHsmError> {
        let mut request = SignedRequest::new("POST", "cloudhsm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CloudHsmFrontendService.DeleteHsm");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<DeleteHsmResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteHsmError::from_response(response))),
                )
            }
        })
    }

    /// <p>This is documentation for <b>AWS CloudHSM Classic</b>. For more information, see <a href="http://aws.amazon.com/cloudhsm/faqs-classic/">AWS CloudHSM Classic FAQs</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/classic/userguide/">AWS CloudHSM Classic User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/classic/APIReference/">AWS CloudHSM Classic API Reference</a>.</p> <p> <b>For information about the current version of AWS CloudHSM</b>, see <a href="http://aws.amazon.com/cloudhsm/">AWS CloudHSM</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/latest/userguide/">AWS CloudHSM User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/latest/APIReference/">AWS CloudHSM API Reference</a>.</p> <p>Deletes a client.</p>
    fn delete_luna_client(
        &self,
        input: DeleteLunaClientRequest,
    ) -> RusotoFuture<DeleteLunaClientResponse, DeleteLunaClientError> {
        let mut request = SignedRequest::new("POST", "cloudhsm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CloudHsmFrontendService.DeleteLunaClient");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<DeleteLunaClientResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteLunaClientError::from_response(response))),
                )
            }
        })
    }

    /// <p>This is documentation for <b>AWS CloudHSM Classic</b>. For more information, see <a href="http://aws.amazon.com/cloudhsm/faqs-classic/">AWS CloudHSM Classic FAQs</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/classic/userguide/">AWS CloudHSM Classic User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/classic/APIReference/">AWS CloudHSM Classic API Reference</a>.</p> <p> <b>For information about the current version of AWS CloudHSM</b>, see <a href="http://aws.amazon.com/cloudhsm/">AWS CloudHSM</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/latest/userguide/">AWS CloudHSM User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/latest/APIReference/">AWS CloudHSM API Reference</a>.</p> <p>Retrieves information about a high-availability partition group.</p>
    fn describe_hapg(
        &self,
        input: DescribeHapgRequest,
    ) -> RusotoFuture<DescribeHapgResponse, DescribeHapgError> {
        let mut request = SignedRequest::new("POST", "cloudhsm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CloudHsmFrontendService.DescribeHapg");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<DescribeHapgResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeHapgError::from_response(response))),
                )
            }
        })
    }

    /// <p>This is documentation for <b>AWS CloudHSM Classic</b>. For more information, see <a href="http://aws.amazon.com/cloudhsm/faqs-classic/">AWS CloudHSM Classic FAQs</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/classic/userguide/">AWS CloudHSM Classic User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/classic/APIReference/">AWS CloudHSM Classic API Reference</a>.</p> <p> <b>For information about the current version of AWS CloudHSM</b>, see <a href="http://aws.amazon.com/cloudhsm/">AWS CloudHSM</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/latest/userguide/">AWS CloudHSM User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/latest/APIReference/">AWS CloudHSM API Reference</a>.</p> <p>Retrieves information about an HSM. You can identify the HSM by its ARN or its serial number.</p>
    fn describe_hsm(
        &self,
        input: DescribeHsmRequest,
    ) -> RusotoFuture<DescribeHsmResponse, DescribeHsmError> {
        let mut request = SignedRequest::new("POST", "cloudhsm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CloudHsmFrontendService.DescribeHsm");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<DescribeHsmResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeHsmError::from_response(response))),
                )
            }
        })
    }

    /// <p>This is documentation for <b>AWS CloudHSM Classic</b>. For more information, see <a href="http://aws.amazon.com/cloudhsm/faqs-classic/">AWS CloudHSM Classic FAQs</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/classic/userguide/">AWS CloudHSM Classic User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/classic/APIReference/">AWS CloudHSM Classic API Reference</a>.</p> <p> <b>For information about the current version of AWS CloudHSM</b>, see <a href="http://aws.amazon.com/cloudhsm/">AWS CloudHSM</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/latest/userguide/">AWS CloudHSM User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/latest/APIReference/">AWS CloudHSM API Reference</a>.</p> <p>Retrieves information about an HSM client.</p>
    fn describe_luna_client(
        &self,
        input: DescribeLunaClientRequest,
    ) -> RusotoFuture<DescribeLunaClientResponse, DescribeLunaClientError> {
        let mut request = SignedRequest::new("POST", "cloudhsm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CloudHsmFrontendService.DescribeLunaClient");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<DescribeLunaClientResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeLunaClientError::from_response(response))),
                )
            }
        })
    }

    /// <p>This is documentation for <b>AWS CloudHSM Classic</b>. For more information, see <a href="http://aws.amazon.com/cloudhsm/faqs-classic/">AWS CloudHSM Classic FAQs</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/classic/userguide/">AWS CloudHSM Classic User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/classic/APIReference/">AWS CloudHSM Classic API Reference</a>.</p> <p> <b>For information about the current version of AWS CloudHSM</b>, see <a href="http://aws.amazon.com/cloudhsm/">AWS CloudHSM</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/latest/userguide/">AWS CloudHSM User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/latest/APIReference/">AWS CloudHSM API Reference</a>.</p> <p>Gets the configuration files necessary to connect to all high availability partition groups the client is associated with.</p>
    fn get_config(
        &self,
        input: GetConfigRequest,
    ) -> RusotoFuture<GetConfigResponse, GetConfigError> {
        let mut request = SignedRequest::new("POST", "cloudhsm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CloudHsmFrontendService.GetConfig");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<GetConfigResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetConfigError::from_response(response))),
                )
            }
        })
    }

    /// <p>This is documentation for <b>AWS CloudHSM Classic</b>. For more information, see <a href="http://aws.amazon.com/cloudhsm/faqs-classic/">AWS CloudHSM Classic FAQs</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/classic/userguide/">AWS CloudHSM Classic User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/classic/APIReference/">AWS CloudHSM Classic API Reference</a>.</p> <p> <b>For information about the current version of AWS CloudHSM</b>, see <a href="http://aws.amazon.com/cloudhsm/">AWS CloudHSM</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/latest/userguide/">AWS CloudHSM User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/latest/APIReference/">AWS CloudHSM API Reference</a>.</p> <p>Lists the Availability Zones that have available AWS CloudHSM capacity.</p>
    fn list_available_zones(
        &self,
    ) -> RusotoFuture<ListAvailableZonesResponse, ListAvailableZonesError> {
        let mut request = SignedRequest::new("POST", "cloudhsm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CloudHsmFrontendService.ListAvailableZones");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<ListAvailableZonesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListAvailableZonesError::from_response(response))),
                )
            }
        })
    }

    /// <p>This is documentation for <b>AWS CloudHSM Classic</b>. For more information, see <a href="http://aws.amazon.com/cloudhsm/faqs-classic/">AWS CloudHSM Classic FAQs</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/classic/userguide/">AWS CloudHSM Classic User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/classic/APIReference/">AWS CloudHSM Classic API Reference</a>.</p> <p> <b>For information about the current version of AWS CloudHSM</b>, see <a href="http://aws.amazon.com/cloudhsm/">AWS CloudHSM</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/latest/userguide/">AWS CloudHSM User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/latest/APIReference/">AWS CloudHSM API Reference</a>.</p> <p>Lists the high-availability partition groups for the account.</p> <p>This operation supports pagination with the use of the <code>NextToken</code> member. If more results are available, the <code>NextToken</code> member of the response contains a token that you pass in the next call to <code>ListHapgs</code> to retrieve the next set of items.</p>
    fn list_hapgs(
        &self,
        input: ListHapgsRequest,
    ) -> RusotoFuture<ListHapgsResponse, ListHapgsError> {
        let mut request = SignedRequest::new("POST", "cloudhsm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CloudHsmFrontendService.ListHapgs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<ListHapgsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListHapgsError::from_response(response))),
                )
            }
        })
    }

    /// <p>This is documentation for <b>AWS CloudHSM Classic</b>. For more information, see <a href="http://aws.amazon.com/cloudhsm/faqs-classic/">AWS CloudHSM Classic FAQs</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/classic/userguide/">AWS CloudHSM Classic User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/classic/APIReference/">AWS CloudHSM Classic API Reference</a>.</p> <p> <b>For information about the current version of AWS CloudHSM</b>, see <a href="http://aws.amazon.com/cloudhsm/">AWS CloudHSM</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/latest/userguide/">AWS CloudHSM User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/latest/APIReference/">AWS CloudHSM API Reference</a>.</p> <p>Retrieves the identifiers of all of the HSMs provisioned for the current customer.</p> <p>This operation supports pagination with the use of the <code>NextToken</code> member. If more results are available, the <code>NextToken</code> member of the response contains a token that you pass in the next call to <code>ListHsms</code> to retrieve the next set of items.</p>
    fn list_hsms(&self, input: ListHsmsRequest) -> RusotoFuture<ListHsmsResponse, ListHsmsError> {
        let mut request = SignedRequest::new("POST", "cloudhsm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CloudHsmFrontendService.ListHsms");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<ListHsmsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListHsmsError::from_response(response))),
                )
            }
        })
    }

    /// <p>This is documentation for <b>AWS CloudHSM Classic</b>. For more information, see <a href="http://aws.amazon.com/cloudhsm/faqs-classic/">AWS CloudHSM Classic FAQs</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/classic/userguide/">AWS CloudHSM Classic User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/classic/APIReference/">AWS CloudHSM Classic API Reference</a>.</p> <p> <b>For information about the current version of AWS CloudHSM</b>, see <a href="http://aws.amazon.com/cloudhsm/">AWS CloudHSM</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/latest/userguide/">AWS CloudHSM User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/latest/APIReference/">AWS CloudHSM API Reference</a>.</p> <p>Lists all of the clients.</p> <p>This operation supports pagination with the use of the <code>NextToken</code> member. If more results are available, the <code>NextToken</code> member of the response contains a token that you pass in the next call to <code>ListLunaClients</code> to retrieve the next set of items.</p>
    fn list_luna_clients(
        &self,
        input: ListLunaClientsRequest,
    ) -> RusotoFuture<ListLunaClientsResponse, ListLunaClientsError> {
        let mut request = SignedRequest::new("POST", "cloudhsm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CloudHsmFrontendService.ListLunaClients");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<ListLunaClientsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListLunaClientsError::from_response(response))),
                )
            }
        })
    }

    /// <p>This is documentation for <b>AWS CloudHSM Classic</b>. For more information, see <a href="http://aws.amazon.com/cloudhsm/faqs-classic/">AWS CloudHSM Classic FAQs</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/classic/userguide/">AWS CloudHSM Classic User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/classic/APIReference/">AWS CloudHSM Classic API Reference</a>.</p> <p> <b>For information about the current version of AWS CloudHSM</b>, see <a href="http://aws.amazon.com/cloudhsm/">AWS CloudHSM</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/latest/userguide/">AWS CloudHSM User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/latest/APIReference/">AWS CloudHSM API Reference</a>.</p> <p>Returns a list of all tags for the specified AWS CloudHSM resource.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> RusotoFuture<ListTagsForResourceResponse, ListTagsForResourceError> {
        let mut request = SignedRequest::new("POST", "cloudhsm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CloudHsmFrontendService.ListTagsForResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<ListTagsForResourceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListTagsForResourceError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>This is documentation for <b>AWS CloudHSM Classic</b>. For more information, see <a href="http://aws.amazon.com/cloudhsm/faqs-classic/">AWS CloudHSM Classic FAQs</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/classic/userguide/">AWS CloudHSM Classic User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/classic/APIReference/">AWS CloudHSM Classic API Reference</a>.</p> <p> <b>For information about the current version of AWS CloudHSM</b>, see <a href="http://aws.amazon.com/cloudhsm/">AWS CloudHSM</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/latest/userguide/">AWS CloudHSM User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/latest/APIReference/">AWS CloudHSM API Reference</a>.</p> <p>Modifies an existing high-availability partition group.</p>
    fn modify_hapg(
        &self,
        input: ModifyHapgRequest,
    ) -> RusotoFuture<ModifyHapgResponse, ModifyHapgError> {
        let mut request = SignedRequest::new("POST", "cloudhsm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CloudHsmFrontendService.ModifyHapg");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<ModifyHapgResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ModifyHapgError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>This is documentation for <b>AWS CloudHSM Classic</b>. For more information, see <a href="http://aws.amazon.com/cloudhsm/faqs-classic/">AWS CloudHSM Classic FAQs</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/classic/userguide/">AWS CloudHSM Classic User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/classic/APIReference/">AWS CloudHSM Classic API Reference</a>.</p> <p> <b>For information about the current version of AWS CloudHSM</b>, see <a href="http://aws.amazon.com/cloudhsm/">AWS CloudHSM</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/latest/userguide/">AWS CloudHSM User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/latest/APIReference/">AWS CloudHSM API Reference</a>.</p> <p>Modifies an HSM.</p> <important> <p>This operation can result in the HSM being offline for up to 15 minutes while the AWS CloudHSM service is reconfigured. If you are modifying a production HSM, you should ensure that your AWS CloudHSM service is configured for high availability, and consider executing this operation during a maintenance window.</p> </important></p>
    fn modify_hsm(
        &self,
        input: ModifyHsmRequest,
    ) -> RusotoFuture<ModifyHsmResponse, ModifyHsmError> {
        let mut request = SignedRequest::new("POST", "cloudhsm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CloudHsmFrontendService.ModifyHsm");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<ModifyHsmResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ModifyHsmError::from_response(response))),
                )
            }
        })
    }

    /// <p>This is documentation for <b>AWS CloudHSM Classic</b>. For more information, see <a href="http://aws.amazon.com/cloudhsm/faqs-classic/">AWS CloudHSM Classic FAQs</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/classic/userguide/">AWS CloudHSM Classic User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/classic/APIReference/">AWS CloudHSM Classic API Reference</a>.</p> <p> <b>For information about the current version of AWS CloudHSM</b>, see <a href="http://aws.amazon.com/cloudhsm/">AWS CloudHSM</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/latest/userguide/">AWS CloudHSM User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/latest/APIReference/">AWS CloudHSM API Reference</a>.</p> <p>Modifies the certificate used by the client.</p> <p>This action can potentially start a workflow to install the new certificate on the client's HSMs.</p>
    fn modify_luna_client(
        &self,
        input: ModifyLunaClientRequest,
    ) -> RusotoFuture<ModifyLunaClientResponse, ModifyLunaClientError> {
        let mut request = SignedRequest::new("POST", "cloudhsm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CloudHsmFrontendService.ModifyLunaClient");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<ModifyLunaClientResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ModifyLunaClientError::from_response(response))),
                )
            }
        })
    }

    /// <p>This is documentation for <b>AWS CloudHSM Classic</b>. For more information, see <a href="http://aws.amazon.com/cloudhsm/faqs-classic/">AWS CloudHSM Classic FAQs</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/classic/userguide/">AWS CloudHSM Classic User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/classic/APIReference/">AWS CloudHSM Classic API Reference</a>.</p> <p> <b>For information about the current version of AWS CloudHSM</b>, see <a href="http://aws.amazon.com/cloudhsm/">AWS CloudHSM</a>, the <a href="http://docs.aws.amazon.com/cloudhsm/latest/userguide/">AWS CloudHSM User Guide</a>, and the <a href="http://docs.aws.amazon.com/cloudhsm/latest/APIReference/">AWS CloudHSM API Reference</a>.</p> <p>Removes one or more tags from the specified AWS CloudHSM resource.</p> <p>To remove a tag, specify only the tag key to remove (not the value). To overwrite the value for an existing tag, use <a>AddTagsToResource</a>.</p>
    fn remove_tags_from_resource(
        &self,
        input: RemoveTagsFromResourceRequest,
    ) -> RusotoFuture<RemoveTagsFromResourceResponse, RemoveTagsFromResourceError> {
        let mut request = SignedRequest::new("POST", "cloudhsm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CloudHsmFrontendService.RemoveTagsFromResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<RemoveTagsFromResourceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(RemoveTagsFromResourceError::from_response(response))
                    }),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
