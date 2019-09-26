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

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde_json;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AcceptSharedDirectoryRequest {
    /// <p>Identifier of the shared directory in the directory consumer account. This identifier is different for each directory owner account. </p>
    #[serde(rename = "SharedDirectoryId")]
    pub shared_directory_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AcceptSharedDirectoryResult {
    /// <p>The shared directory in the directory consumer account.</p>
    #[serde(rename = "SharedDirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_directory: Option<SharedDirectory>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AddIpRoutesRequest {
    /// <p>Identifier (ID) of the directory to which to add the address block.</p>
    #[serde(rename = "DirectoryId")]
    pub directory_id: String,
    /// <p>IP address blocks, using CIDR format, of the traffic to route. This is often the IP address block of the DNS server used for your on-premises domain.</p>
    #[serde(rename = "IpRoutes")]
    pub ip_routes: Vec<IpRoute>,
    /// <p>If set to true, updates the inbound and outbound rules of the security group that has the description: "AWS created security group for <i>directory ID</i> directory controllers." Following are the new rules: </p> <p>Inbound:</p> <ul> <li> <p>Type: Custom UDP Rule, Protocol: UDP, Range: 88, Source: 0.0.0.0/0</p> </li> <li> <p>Type: Custom UDP Rule, Protocol: UDP, Range: 123, Source: 0.0.0.0/0</p> </li> <li> <p>Type: Custom UDP Rule, Protocol: UDP, Range: 138, Source: 0.0.0.0/0</p> </li> <li> <p>Type: Custom UDP Rule, Protocol: UDP, Range: 389, Source: 0.0.0.0/0</p> </li> <li> <p>Type: Custom UDP Rule, Protocol: UDP, Range: 464, Source: 0.0.0.0/0</p> </li> <li> <p>Type: Custom UDP Rule, Protocol: UDP, Range: 445, Source: 0.0.0.0/0</p> </li> <li> <p>Type: Custom TCP Rule, Protocol: TCP, Range: 88, Source: 0.0.0.0/0</p> </li> <li> <p>Type: Custom TCP Rule, Protocol: TCP, Range: 135, Source: 0.0.0.0/0</p> </li> <li> <p>Type: Custom TCP Rule, Protocol: TCP, Range: 445, Source: 0.0.0.0/0</p> </li> <li> <p>Type: Custom TCP Rule, Protocol: TCP, Range: 464, Source: 0.0.0.0/0</p> </li> <li> <p>Type: Custom TCP Rule, Protocol: TCP, Range: 636, Source: 0.0.0.0/0</p> </li> <li> <p>Type: Custom TCP Rule, Protocol: TCP, Range: 1024-65535, Source: 0.0.0.0/0</p> </li> <li> <p>Type: Custom TCP Rule, Protocol: TCP, Range: 3268-33269, Source: 0.0.0.0/0</p> </li> <li> <p>Type: DNS (UDP), Protocol: UDP, Range: 53, Source: 0.0.0.0/0</p> </li> <li> <p>Type: DNS (TCP), Protocol: TCP, Range: 53, Source: 0.0.0.0/0</p> </li> <li> <p>Type: LDAP, Protocol: TCP, Range: 389, Source: 0.0.0.0/0</p> </li> <li> <p>Type: All ICMP, Protocol: All, Range: N/A, Source: 0.0.0.0/0</p> </li> </ul> <p/> <p>Outbound:</p> <ul> <li> <p>Type: All traffic, Protocol: All, Range: All, Destination: 0.0.0.0/0</p> </li> </ul> <p>These security rules impact an internal network interface that is not exposed publicly.</p>
    #[serde(rename = "UpdateSecurityGroupForDirectoryControllers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_security_group_for_directory_controllers: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AddIpRoutesResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AddTagsToResourceRequest {
    /// <p>Identifier (ID) for the directory to which to add the tag.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>The tags to be assigned to the directory.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AddTagsToResourceResult {}

/// <p>Represents a named directory attribute.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attribute {
    /// <p>The name of the attribute.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The value of the attribute.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CancelSchemaExtensionRequest {
    /// <p>The identifier of the directory whose schema extension will be canceled.</p>
    #[serde(rename = "DirectoryId")]
    pub directory_id: String,
    /// <p>The identifier of the schema extension that will be canceled.</p>
    #[serde(rename = "SchemaExtensionId")]
    pub schema_extension_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CancelSchemaExtensionResult {}

/// <p>Contains information about a computer account in a directory.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Computer {
    /// <p>An array of <a>Attribute</a> objects containing the LDAP attributes that belong to the computer account.</p>
    #[serde(rename = "ComputerAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computer_attributes: Option<Vec<Attribute>>,
    /// <p>The identifier of the computer.</p>
    #[serde(rename = "ComputerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computer_id: Option<String>,
    /// <p>The computer name.</p>
    #[serde(rename = "ComputerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,
}

/// <p>Points to a remote domain with which you are setting up a trust relationship. Conditional forwarders are required in order to set up a trust relationship with another domain.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ConditionalForwarder {
    /// <p>The IP addresses of the remote DNS server associated with RemoteDomainName. This is the IP address of the DNS server that your conditional forwarder points to.</p>
    #[serde(rename = "DnsIpAddrs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_ip_addrs: Option<Vec<String>>,
    /// <p>The fully qualified domain name (FQDN) of the remote domains pointed to by the conditional forwarder.</p>
    #[serde(rename = "RemoteDomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_domain_name: Option<String>,
    /// <p>The replication scope of the conditional forwarder. The only allowed value is <code>Domain</code>, which will replicate the conditional forwarder to all of the domain controllers for your AWS directory.</p>
    #[serde(rename = "ReplicationScope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_scope: Option<String>,
}

/// <p>Contains the inputs for the <a>ConnectDirectory</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ConnectDirectoryRequest {
    /// <p>A <a>DirectoryConnectSettings</a> object that contains additional information for the operation.</p>
    #[serde(rename = "ConnectSettings")]
    pub connect_settings: DirectoryConnectSettings,
    /// <p>A textual description for the directory.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The fully qualified name of the on-premises directory, such as <code>corp.example.com</code>.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The password for the on-premises user account.</p>
    #[serde(rename = "Password")]
    pub password: String,
    /// <p>The NetBIOS name of the on-premises directory, such as <code>CORP</code>.</p>
    #[serde(rename = "ShortName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    /// <p>The size of the directory.</p>
    #[serde(rename = "Size")]
    pub size: String,
    /// <p>The tags to be assigned to AD Connector.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Contains the results of the <a>ConnectDirectory</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ConnectDirectoryResult {
    /// <p>The identifier of the new directory.</p>
    #[serde(rename = "DirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
}

/// <p>Contains the inputs for the <a>CreateAlias</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateAliasRequest {
    /// <p>The requested alias.</p> <p>The alias must be unique amongst all aliases in AWS. This operation throws an <code>EntityAlreadyExistsException</code> error if the alias already exists.</p>
    #[serde(rename = "Alias")]
    pub alias: String,
    /// <p>The identifier of the directory for which to create the alias.</p>
    #[serde(rename = "DirectoryId")]
    pub directory_id: String,
}

/// <p>Contains the results of the <a>CreateAlias</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateAliasResult {
    /// <p>The alias for the directory.</p>
    #[serde(rename = "Alias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    /// <p>The identifier of the directory.</p>
    #[serde(rename = "DirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
}

/// <p>Contains the inputs for the <a>CreateComputer</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateComputerRequest {
    /// <p>An array of <a>Attribute</a> objects that contain any LDAP attributes to apply to the computer account.</p>
    #[serde(rename = "ComputerAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computer_attributes: Option<Vec<Attribute>>,
    /// <p>The name of the computer account.</p>
    #[serde(rename = "ComputerName")]
    pub computer_name: String,
    /// <p>The identifier of the directory in which to create the computer account.</p>
    #[serde(rename = "DirectoryId")]
    pub directory_id: String,
    /// <p>The fully-qualified distinguished name of the organizational unit to place the computer account in.</p>
    #[serde(rename = "OrganizationalUnitDistinguishedName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit_distinguished_name: Option<String>,
    /// <p>A one-time password that is used to join the computer to the directory. You should generate a random, strong password to use for this parameter.</p>
    #[serde(rename = "Password")]
    pub password: String,
}

/// <p>Contains the results for the <a>CreateComputer</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateComputerResult {
    /// <p>A <a>Computer</a> object that represents the computer account.</p>
    #[serde(rename = "Computer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computer: Option<Computer>,
}

/// <p>Initiates the creation of a conditional forwarder for your AWS Directory Service for Microsoft Active Directory. Conditional forwarders are required in order to set up a trust relationship with another domain.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateConditionalForwarderRequest {
    /// <p>The directory ID of the AWS directory for which you are creating the conditional forwarder.</p>
    #[serde(rename = "DirectoryId")]
    pub directory_id: String,
    /// <p>The IP addresses of the remote DNS server associated with RemoteDomainName.</p>
    #[serde(rename = "DnsIpAddrs")]
    pub dns_ip_addrs: Vec<String>,
    /// <p>The fully qualified domain name (FQDN) of the remote domain with which you will set up a trust relationship.</p>
    #[serde(rename = "RemoteDomainName")]
    pub remote_domain_name: String,
}

/// <p>The result of a CreateConditinalForwarder request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateConditionalForwarderResult {}

/// <p>Contains the inputs for the <a>CreateDirectory</a> operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDirectoryRequest {
    /// <p>A textual description for the directory.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The fully qualified name for the directory, such as <code>corp.example.com</code>.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The password for the directory administrator. The directory creation process creates a directory administrator account with the user name <code>Administrator</code> and this password.</p>
    #[serde(rename = "Password")]
    pub password: String,
    /// <p>The short name of the directory, such as <code>CORP</code>.</p>
    #[serde(rename = "ShortName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    /// <p>The size of the directory.</p>
    #[serde(rename = "Size")]
    pub size: String,
    /// <p>The tags to be assigned to the Simple AD directory.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>A <a>DirectoryVpcSettings</a> object that contains additional information for the operation.</p>
    #[serde(rename = "VpcSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_settings: Option<DirectoryVpcSettings>,
}

/// <p>Contains the results of the <a>CreateDirectory</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateDirectoryResult {
    /// <p>The identifier of the directory that was created.</p>
    #[serde(rename = "DirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateLogSubscriptionRequest {
    /// <p>Identifier (ID) of the directory to which you want to subscribe and receive real-time logs to your specified CloudWatch log group.</p>
    #[serde(rename = "DirectoryId")]
    pub directory_id: String,
    /// <p>The name of the CloudWatch log group where the real-time domain controller logs are forwarded.</p>
    #[serde(rename = "LogGroupName")]
    pub log_group_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateLogSubscriptionResult {}

/// <p>Creates an AWS Managed Microsoft AD directory.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateMicrosoftADRequest {
    /// <p>A textual description for the directory. This label will appear on the AWS console <code>Directory Details</code> page after the directory is created.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>AWS Managed Microsoft AD is available in two editions: Standard and Enterprise. Enterprise is the default.</p>
    #[serde(rename = "Edition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    /// <p>The fully qualified domain name for the directory, such as <code>corp.example.com</code>. This name will resolve inside your VPC only. It does not need to be publicly resolvable.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The password for the default administrative user named <code>Admin</code>.</p>
    #[serde(rename = "Password")]
    pub password: String,
    /// <p>The NetBIOS name for your domain. A short identifier for your domain, such as <code>CORP</code>. If you don't specify a NetBIOS name, it will default to the first part of your directory DNS. For example, <code>CORP</code> for the directory DNS <code>corp.example.com</code>. </p>
    #[serde(rename = "ShortName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    /// <p>The tags to be assigned to the AWS Managed Microsoft AD directory.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>Contains VPC information for the <a>CreateDirectory</a> or <a>CreateMicrosoftAD</a> operation.</p>
    #[serde(rename = "VpcSettings")]
    pub vpc_settings: DirectoryVpcSettings,
}

/// <p>Result of a CreateMicrosoftAD request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateMicrosoftADResult {
    /// <p>The identifier of the directory that was created.</p>
    #[serde(rename = "DirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
}

/// <p>Contains the inputs for the <a>CreateSnapshot</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateSnapshotRequest {
    /// <p>The identifier of the directory of which to take a snapshot.</p>
    #[serde(rename = "DirectoryId")]
    pub directory_id: String,
    /// <p>The descriptive name to apply to the snapshot.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Contains the results of the <a>CreateSnapshot</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateSnapshotResult {
    /// <p>The identifier of the snapshot that was created.</p>
    #[serde(rename = "SnapshotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
}

/// <p>AWS Directory Service for Microsoft Active Directory allows you to configure trust relationships. For example, you can establish a trust between your AWS Managed Microsoft AD directory, and your existing on-premises Microsoft Active Directory. This would allow you to provide users and groups access to resources in either domain, with a single set of credentials.</p> <p>This action initiates the creation of the AWS side of a trust relationship between an AWS Managed Microsoft AD directory and an external domain.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateTrustRequest {
    /// <p>The IP addresses of the remote DNS server associated with RemoteDomainName.</p>
    #[serde(rename = "ConditionalForwarderIpAddrs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_forwarder_ip_addrs: Option<Vec<String>>,
    /// <p>The Directory ID of the AWS Managed Microsoft AD directory for which to establish the trust relationship.</p>
    #[serde(rename = "DirectoryId")]
    pub directory_id: String,
    /// <p>The Fully Qualified Domain Name (FQDN) of the external domain for which to create the trust relationship.</p>
    #[serde(rename = "RemoteDomainName")]
    pub remote_domain_name: String,
    /// <p>Optional parameter to enable selective authentication for the trust.</p>
    #[serde(rename = "SelectiveAuth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective_auth: Option<String>,
    /// <p>The direction of the trust relationship.</p>
    #[serde(rename = "TrustDirection")]
    pub trust_direction: String,
    /// <p>The trust password. The must be the same password that was used when creating the trust relationship on the external domain.</p>
    #[serde(rename = "TrustPassword")]
    pub trust_password: String,
    /// <p>The trust relationship type. <code>Forest</code> is the default.</p>
    #[serde(rename = "TrustType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_type: Option<String>,
}

/// <p>The result of a CreateTrust request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateTrustResult {
    /// <p>A unique identifier for the trust relationship that was created.</p>
    #[serde(rename = "TrustId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_id: Option<String>,
}

/// <p>Deletes a conditional forwarder.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteConditionalForwarderRequest {
    /// <p>The directory ID for which you are deleting the conditional forwarder.</p>
    #[serde(rename = "DirectoryId")]
    pub directory_id: String,
    /// <p>The fully qualified domain name (FQDN) of the remote domain with which you are deleting the conditional forwarder.</p>
    #[serde(rename = "RemoteDomainName")]
    pub remote_domain_name: String,
}

/// <p>The result of a DeleteConditionalForwarder request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteConditionalForwarderResult {}

/// <p>Contains the inputs for the <a>DeleteDirectory</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDirectoryRequest {
    /// <p>The identifier of the directory to delete.</p>
    #[serde(rename = "DirectoryId")]
    pub directory_id: String,
}

/// <p>Contains the results of the <a>DeleteDirectory</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteDirectoryResult {
    /// <p>The directory identifier.</p>
    #[serde(rename = "DirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteLogSubscriptionRequest {
    /// <p>Identifier (ID) of the directory whose log subscription you want to delete.</p>
    #[serde(rename = "DirectoryId")]
    pub directory_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteLogSubscriptionResult {}

/// <p>Contains the inputs for the <a>DeleteSnapshot</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteSnapshotRequest {
    /// <p>The identifier of the directory snapshot to be deleted.</p>
    #[serde(rename = "SnapshotId")]
    pub snapshot_id: String,
}

/// <p>Contains the results of the <a>DeleteSnapshot</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteSnapshotResult {
    /// <p>The identifier of the directory snapshot that was deleted.</p>
    #[serde(rename = "SnapshotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
}

/// <p>Deletes the local side of an existing trust relationship between the AWS Managed Microsoft AD directory and the external domain.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteTrustRequest {
    /// <p>Delete a conditional forwarder as part of a DeleteTrustRequest.</p>
    #[serde(rename = "DeleteAssociatedConditionalForwarder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_associated_conditional_forwarder: Option<bool>,
    /// <p>The Trust ID of the trust relationship to be deleted.</p>
    #[serde(rename = "TrustId")]
    pub trust_id: String,
}

/// <p>The result of a DeleteTrust request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteTrustResult {
    /// <p>The Trust ID of the trust relationship that was deleted.</p>
    #[serde(rename = "TrustId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_id: Option<String>,
}

/// <p>Removes the specified directory as a publisher to the specified SNS topic.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeregisterEventTopicRequest {
    /// <p>The Directory ID to remove as a publisher. This directory will no longer send messages to the specified SNS topic.</p>
    #[serde(rename = "DirectoryId")]
    pub directory_id: String,
    /// <p>The name of the SNS topic from which to remove the directory as a publisher.</p>
    #[serde(rename = "TopicName")]
    pub topic_name: String,
}

/// <p>The result of a DeregisterEventTopic request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeregisterEventTopicResult {}

/// <p>Describes a conditional forwarder.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeConditionalForwardersRequest {
    /// <p>The directory ID for which to get the list of associated conditional forwarders.</p>
    #[serde(rename = "DirectoryId")]
    pub directory_id: String,
    /// <p>The fully qualified domain names (FQDN) of the remote domains for which to get the list of associated conditional forwarders. If this member is null, all conditional forwarders are returned.</p>
    #[serde(rename = "RemoteDomainNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_domain_names: Option<Vec<String>>,
}

/// <p>The result of a DescribeConditionalForwarder request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeConditionalForwardersResult {
    /// <p>The list of conditional forwarders that have been created.</p>
    #[serde(rename = "ConditionalForwarders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_forwarders: Option<Vec<ConditionalForwarder>>,
}

/// <p>Contains the inputs for the <a>DescribeDirectories</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeDirectoriesRequest {
    /// <p>A list of identifiers of the directories for which to obtain the information. If this member is null, all directories that belong to the current account are returned.</p> <p>An empty list results in an <code>InvalidParameterException</code> being thrown.</p>
    #[serde(rename = "DirectoryIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_ids: Option<Vec<String>>,
    /// <p>The maximum number of items to return. If this value is zero, the maximum number of items is specified by the limitations of the operation.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <code>DescribeDirectoriesResult.NextToken</code> value from a previous call to <a>DescribeDirectories</a>. Pass null if this is the first call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Contains the results of the <a>DescribeDirectories</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeDirectoriesResult {
    /// <p>The list of <a>DirectoryDescription</a> objects that were retrieved.</p> <p>It is possible that this list contains less than the number of items specified in the <code>Limit</code> member of the request. This occurs if there are less than the requested number of items left to retrieve, or if the limitations of the operation have been exceeded.</p>
    #[serde(rename = "DirectoryDescriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_descriptions: Option<Vec<DirectoryDescription>>,
    /// <p>If not null, more results are available. Pass this value for the <code>NextToken</code> parameter in a subsequent call to <a>DescribeDirectories</a> to retrieve the next set of items.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeDomainControllersRequest {
    /// <p>Identifier of the directory for which to retrieve the domain controller information.</p>
    #[serde(rename = "DirectoryId")]
    pub directory_id: String,
    /// <p>A list of identifiers for the domain controllers whose information will be provided.</p>
    #[serde(rename = "DomainControllerIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_controller_ids: Option<Vec<String>>,
    /// <p>The maximum number of items to return.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <i>DescribeDomainControllers.NextToken</i> value from a previous call to <a>DescribeDomainControllers</a>. Pass null if this is the first call. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeDomainControllersResult {
    /// <p>List of the <a>DomainController</a> objects that were retrieved.</p>
    #[serde(rename = "DomainControllers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_controllers: Option<Vec<DomainController>>,
    /// <p>If not null, more results are available. Pass this value for the <code>NextToken</code> parameter in a subsequent call to <a>DescribeDomainControllers</a> retrieve the next set of items.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Describes event topics.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeEventTopicsRequest {
    /// <p>The Directory ID for which to get the list of associated SNS topics. If this member is null, associations for all Directory IDs are returned.</p>
    #[serde(rename = "DirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    /// <p>A list of SNS topic names for which to obtain the information. If this member is null, all associations for the specified Directory ID are returned.</p> <p>An empty list results in an <code>InvalidParameterException</code> being thrown.</p>
    #[serde(rename = "TopicNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_names: Option<Vec<String>>,
}

/// <p>The result of a DescribeEventTopic request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeEventTopicsResult {
    /// <p>A list of SNS topic names that receive status messages from the specified Directory ID.</p>
    #[serde(rename = "EventTopics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_topics: Option<Vec<EventTopic>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeSharedDirectoriesRequest {
    /// <p>The number of shared directories to return in the response object.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <code>DescribeSharedDirectoriesResult.NextToken</code> value from a previous call to <a>DescribeSharedDirectories</a>. Pass null if this is the first call. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Returns the identifier of the directory in the directory owner account. </p>
    #[serde(rename = "OwnerDirectoryId")]
    pub owner_directory_id: String,
    /// <p>A list of identifiers of all shared directories in your account. </p>
    #[serde(rename = "SharedDirectoryIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_directory_ids: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeSharedDirectoriesResult {
    /// <p>If not null, token that indicates that more results are available. Pass this value for the <code>NextToken</code> parameter in a subsequent call to <a>DescribeSharedDirectories</a> to retrieve the next set of items.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of all shared directories in your account.</p>
    #[serde(rename = "SharedDirectories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_directories: Option<Vec<SharedDirectory>>,
}

/// <p>Contains the inputs for the <a>DescribeSnapshots</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeSnapshotsRequest {
    /// <p>The identifier of the directory for which to retrieve snapshot information.</p>
    #[serde(rename = "DirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    /// <p>The maximum number of objects to return.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <i>DescribeSnapshotsResult.NextToken</i> value from a previous call to <a>DescribeSnapshots</a>. Pass null if this is the first call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of identifiers of the snapshots to obtain the information for. If this member is null or empty, all snapshots are returned using the <i>Limit</i> and <i>NextToken</i> members.</p>
    #[serde(rename = "SnapshotIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_ids: Option<Vec<String>>,
}

/// <p>Contains the results of the <a>DescribeSnapshots</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeSnapshotsResult {
    /// <p>If not null, more results are available. Pass this value in the <i>NextToken</i> member of a subsequent call to <a>DescribeSnapshots</a>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of <a>Snapshot</a> objects that were retrieved.</p> <p>It is possible that this list contains less than the number of items specified in the <i>Limit</i> member of the request. This occurs if there are less than the requested number of items left to retrieve, or if the limitations of the operation have been exceeded.</p>
    #[serde(rename = "Snapshots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshots: Option<Vec<Snapshot>>,
}

/// <p>Describes the trust relationships for a particular AWS Managed Microsoft AD directory. If no input parameters are are provided, such as directory ID or trust ID, this request describes all the trust relationships.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeTrustsRequest {
    /// <p>The Directory ID of the AWS directory that is a part of the requested trust relationship.</p>
    #[serde(rename = "DirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    /// <p>The maximum number of objects to return.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <i>DescribeTrustsResult.NextToken</i> value from a previous call to <a>DescribeTrusts</a>. Pass null if this is the first call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of identifiers of the trust relationships for which to obtain the information. If this member is null, all trust relationships that belong to the current account are returned.</p> <p>An empty list results in an <code>InvalidParameterException</code> being thrown.</p>
    #[serde(rename = "TrustIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_ids: Option<Vec<String>>,
}

/// <p>The result of a DescribeTrust request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeTrustsResult {
    /// <p>If not null, more results are available. Pass this value for the <i>NextToken</i> parameter in a subsequent call to <a>DescribeTrusts</a> to retrieve the next set of items.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of Trust objects that were retrieved.</p> <p>It is possible that this list contains less than the number of items specified in the <i>Limit</i> member of the request. This occurs if there are less than the requested number of items left to retrieve, or if the limitations of the operation have been exceeded.</p>
    #[serde(rename = "Trusts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusts: Option<Vec<Trust>>,
}

/// <p>Contains information for the <a>ConnectDirectory</a> operation when an AD Connector directory is being created.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DirectoryConnectSettings {
    /// <p>A list of one or more IP addresses of DNS servers or domain controllers in the on-premises directory.</p>
    #[serde(rename = "CustomerDnsIps")]
    pub customer_dns_ips: Vec<String>,
    /// <p><p>The user name of an account in the on-premises directory that is used to connect to the directory. This account must have the following permissions:</p> <ul> <li> <p>Read users and groups</p> </li> <li> <p>Create computer objects</p> </li> <li> <p>Join computers to the domain</p> </li> </ul></p>
    #[serde(rename = "CustomerUserName")]
    pub customer_user_name: String,
    /// <p>A list of subnet identifiers in the VPC in which the AD Connector is created.</p>
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,
    /// <p>The identifier of the VPC in which the AD Connector is created.</p>
    #[serde(rename = "VpcId")]
    pub vpc_id: String,
}

/// <p>Contains information about an AD Connector directory.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DirectoryConnectSettingsDescription {
    /// <p>A list of the Availability Zones that the directory is in.</p>
    #[serde(rename = "AvailabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    /// <p>The IP addresses of the AD Connector servers.</p>
    #[serde(rename = "ConnectIps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_ips: Option<Vec<String>>,
    /// <p>The user name of the service account in the on-premises directory.</p>
    #[serde(rename = "CustomerUserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_user_name: Option<String>,
    /// <p>The security group identifier for the AD Connector directory.</p>
    #[serde(rename = "SecurityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_id: Option<String>,
    /// <p>A list of subnet identifiers in the VPC that the AD connector is in.</p>
    #[serde(rename = "SubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    /// <p>The identifier of the VPC that the AD Connector is in.</p>
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

/// <p>Contains information about an AWS Directory Service directory.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DirectoryDescription {
    /// <p>The access URL for the directory, such as <code>http://&lt;alias&gt;.awsapps.com</code>. If no alias has been created for the directory, <code>&lt;alias&gt;</code> is the directory identifier, such as <code>d-XXXXXXXXXX</code>.</p>
    #[serde(rename = "AccessUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_url: Option<String>,
    /// <p>The alias for the directory. If no alias has been created for the directory, the alias is the directory identifier, such as <code>d-XXXXXXXXXX</code>.</p>
    #[serde(rename = "Alias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    /// <p>A <a>DirectoryConnectSettingsDescription</a> object that contains additional information about an AD Connector directory. This member is only present if the directory is an AD Connector directory.</p>
    #[serde(rename = "ConnectSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_settings: Option<DirectoryConnectSettingsDescription>,
    /// <p>The textual description for the directory.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The desired number of domain controllers in the directory if the directory is Microsoft AD.</p>
    #[serde(rename = "DesiredNumberOfDomainControllers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_number_of_domain_controllers: Option<i64>,
    /// <p>The directory identifier.</p>
    #[serde(rename = "DirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    /// <p>The IP addresses of the DNS servers for the directory. For a Simple AD or Microsoft AD directory, these are the IP addresses of the Simple AD or Microsoft AD directory servers. For an AD Connector directory, these are the IP addresses of the DNS servers or domain controllers in the on-premises directory to which the AD Connector is connected.</p>
    #[serde(rename = "DnsIpAddrs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_ip_addrs: Option<Vec<String>>,
    /// <p>The edition associated with this directory.</p>
    #[serde(rename = "Edition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    /// <p>Specifies when the directory was created.</p>
    #[serde(rename = "LaunchTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_time: Option<f64>,
    /// <p>The fully qualified name of the directory.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Describes the AWS Managed Microsoft AD directory in the directory owner account.</p>
    #[serde(rename = "OwnerDirectoryDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_directory_description: Option<OwnerDirectoryDescription>,
    /// <p>A <a>RadiusSettings</a> object that contains information about the RADIUS server configured for this directory.</p>
    #[serde(rename = "RadiusSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_settings: Option<RadiusSettings>,
    /// <p>The status of the RADIUS MFA server connection.</p>
    #[serde(rename = "RadiusStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_status: Option<String>,
    /// <p>The method used when sharing a directory to determine whether the directory should be shared within your AWS organization (<code>ORGANIZATIONS</code>) or with any AWS account by sending a shared directory request (<code>HANDSHAKE</code>).</p>
    #[serde(rename = "ShareMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_method: Option<String>,
    /// <p>A directory share request that is sent by the directory owner to the directory consumer. The request includes a typed message to help the directory consumer administrator determine whether to approve or reject the share invitation.</p>
    #[serde(rename = "ShareNotes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_notes: Option<String>,
    /// <p>Current directory status of the shared AWS Managed Microsoft AD directory.</p>
    #[serde(rename = "ShareStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_status: Option<String>,
    /// <p>The short name of the directory.</p>
    #[serde(rename = "ShortName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    /// <p>The directory size.</p>
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    /// <p>Indicates if single sign-on is enabled for the directory. For more information, see <a>EnableSso</a> and <a>DisableSso</a>.</p>
    #[serde(rename = "SsoEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sso_enabled: Option<bool>,
    /// <p>The current stage of the directory.</p>
    #[serde(rename = "Stage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
    /// <p>The date and time that the stage was last updated.</p>
    #[serde(rename = "StageLastUpdatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_last_updated_date_time: Option<f64>,
    /// <p>Additional information about the directory stage.</p>
    #[serde(rename = "StageReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_reason: Option<String>,
    /// <p>The directory size.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>A <a>DirectoryVpcSettingsDescription</a> object that contains additional information about a directory. This member is only present if the directory is a Simple AD or Managed AD directory.</p>
    #[serde(rename = "VpcSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_settings: Option<DirectoryVpcSettingsDescription>,
}

/// <p>Contains directory limit information for a region.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DirectoryLimits {
    /// <p>The current number of cloud directories in the region.</p>
    #[serde(rename = "CloudOnlyDirectoriesCurrentCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_only_directories_current_count: Option<i64>,
    /// <p>The maximum number of cloud directories allowed in the region.</p>
    #[serde(rename = "CloudOnlyDirectoriesLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_only_directories_limit: Option<i64>,
    /// <p>Indicates if the cloud directory limit has been reached.</p>
    #[serde(rename = "CloudOnlyDirectoriesLimitReached")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_only_directories_limit_reached: Option<bool>,
    /// <p>The current number of AWS Managed Microsoft AD directories in the region.</p>
    #[serde(rename = "CloudOnlyMicrosoftADCurrentCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_only_microsoft_ad_current_count: Option<i64>,
    /// <p>The maximum number of AWS Managed Microsoft AD directories allowed in the region.</p>
    #[serde(rename = "CloudOnlyMicrosoftADLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_only_microsoft_ad_limit: Option<i64>,
    /// <p>Indicates if the AWS Managed Microsoft AD directory limit has been reached.</p>
    #[serde(rename = "CloudOnlyMicrosoftADLimitReached")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_only_microsoft_ad_limit_reached: Option<bool>,
    /// <p>The current number of connected directories in the region.</p>
    #[serde(rename = "ConnectedDirectoriesCurrentCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_directories_current_count: Option<i64>,
    /// <p>The maximum number of connected directories allowed in the region.</p>
    #[serde(rename = "ConnectedDirectoriesLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_directories_limit: Option<i64>,
    /// <p>Indicates if the connected directory limit has been reached.</p>
    #[serde(rename = "ConnectedDirectoriesLimitReached")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_directories_limit_reached: Option<bool>,
}

/// <p>Contains VPC information for the <a>CreateDirectory</a> or <a>CreateMicrosoftAD</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DirectoryVpcSettings {
    /// <p>The identifiers of the subnets for the directory servers. The two subnets must be in different Availability Zones. AWS Directory Service creates a directory server and a DNS server in each of these subnets.</p>
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,
    /// <p>The identifier of the VPC in which to create the directory.</p>
    #[serde(rename = "VpcId")]
    pub vpc_id: String,
}

/// <p>Contains information about the directory.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DirectoryVpcSettingsDescription {
    /// <p>The list of Availability Zones that the directory is in.</p>
    #[serde(rename = "AvailabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    /// <p>The domain controller security group identifier for the directory.</p>
    #[serde(rename = "SecurityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_id: Option<String>,
    /// <p>The identifiers of the subnets for the directory servers.</p>
    #[serde(rename = "SubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    /// <p>The identifier of the VPC that the directory is in.</p>
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

/// <p>Contains the inputs for the <a>DisableRadius</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisableRadiusRequest {
    /// <p>The identifier of the directory for which to disable MFA.</p>
    #[serde(rename = "DirectoryId")]
    pub directory_id: String,
}

/// <p>Contains the results of the <a>DisableRadius</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DisableRadiusResult {}

/// <p>Contains the inputs for the <a>DisableSso</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisableSsoRequest {
    /// <p>The identifier of the directory for which to disable single-sign on.</p>
    #[serde(rename = "DirectoryId")]
    pub directory_id: String,
    /// <p>The password of an alternate account to use to disable single-sign on. This is only used for AD Connector directories. For more information, see the <i>UserName</i> parameter.</p>
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// <p>The username of an alternate account to use to disable single-sign on. This is only used for AD Connector directories. This account must have privileges to remove a service principal name.</p> <p>If the AD Connector service account does not have privileges to remove a service principal name, you can specify an alternate account with the <i>UserName</i> and <i>Password</i> parameters. These credentials are only used to disable single sign-on and are not stored by the service. The AD Connector service account is not changed.</p>
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

/// <p>Contains the results of the <a>DisableSso</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DisableSsoResult {}

/// <p>Contains information about the domain controllers for a specified directory.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DomainController {
    /// <p>The Availability Zone where the domain controller is located.</p>
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>Identifier of the directory where the domain controller resides.</p>
    #[serde(rename = "DirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    /// <p>The IP address of the domain controller.</p>
    #[serde(rename = "DnsIpAddr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_ip_addr: Option<String>,
    /// <p>Identifies a specific domain controller in the directory.</p>
    #[serde(rename = "DomainControllerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_controller_id: Option<String>,
    /// <p>Specifies when the domain controller was created.</p>
    #[serde(rename = "LaunchTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_time: Option<f64>,
    /// <p>The status of the domain controller.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The date and time that the status was last updated.</p>
    #[serde(rename = "StatusLastUpdatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_last_updated_date_time: Option<f64>,
    /// <p>A description of the domain controller state.</p>
    #[serde(rename = "StatusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    /// <p>Identifier of the subnet in the VPC that contains the domain controller.</p>
    #[serde(rename = "SubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    /// <p>The identifier of the VPC that contains the domain controller.</p>
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

/// <p>Contains the inputs for the <a>EnableRadius</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct EnableRadiusRequest {
    /// <p>The identifier of the directory for which to enable MFA.</p>
    #[serde(rename = "DirectoryId")]
    pub directory_id: String,
    /// <p>A <a>RadiusSettings</a> object that contains information about the RADIUS server.</p>
    #[serde(rename = "RadiusSettings")]
    pub radius_settings: RadiusSettings,
}

/// <p>Contains the results of the <a>EnableRadius</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct EnableRadiusResult {}

/// <p>Contains the inputs for the <a>EnableSso</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct EnableSsoRequest {
    /// <p>The identifier of the directory for which to enable single-sign on.</p>
    #[serde(rename = "DirectoryId")]
    pub directory_id: String,
    /// <p>The password of an alternate account to use to enable single-sign on. This is only used for AD Connector directories. For more information, see the <i>UserName</i> parameter.</p>
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// <p>The username of an alternate account to use to enable single-sign on. This is only used for AD Connector directories. This account must have privileges to add a service principal name.</p> <p>If the AD Connector service account does not have privileges to add a service principal name, you can specify an alternate account with the <i>UserName</i> and <i>Password</i> parameters. These credentials are only used to enable single sign-on and are not stored by the service. The AD Connector service account is not changed.</p>
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

/// <p>Contains the results of the <a>EnableSso</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct EnableSsoResult {}

/// <p>Information about SNS topic and AWS Directory Service directory associations.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct EventTopic {
    /// <p>The date and time of when you associated your directory with the SNS topic.</p>
    #[serde(rename = "CreatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<f64>,
    /// <p>The Directory ID of an AWS Directory Service directory that will publish status messages to an SNS topic.</p>
    #[serde(rename = "DirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    /// <p>The topic registration status.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The SNS topic ARN (Amazon Resource Name).</p>
    #[serde(rename = "TopicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_arn: Option<String>,
    /// <p>The name of an AWS SNS topic the receives status messages from the directory.</p>
    #[serde(rename = "TopicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
}

/// <p>Contains the inputs for the <a>GetDirectoryLimits</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDirectoryLimitsRequest {}

/// <p>Contains the results of the <a>GetDirectoryLimits</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetDirectoryLimitsResult {
    /// <p>A <a>DirectoryLimits</a> object that contains the directory limits for the current region.</p>
    #[serde(rename = "DirectoryLimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_limits: Option<DirectoryLimits>,
}

/// <p>Contains the inputs for the <a>GetSnapshotLimits</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSnapshotLimitsRequest {
    /// <p>Contains the identifier of the directory to obtain the limits for.</p>
    #[serde(rename = "DirectoryId")]
    pub directory_id: String,
}

/// <p>Contains the results of the <a>GetSnapshotLimits</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetSnapshotLimitsResult {
    /// <p>A <a>SnapshotLimits</a> object that contains the manual snapshot limits for the specified directory.</p>
    #[serde(rename = "SnapshotLimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_limits: Option<SnapshotLimits>,
}

/// <p>IP address block. This is often the address block of the DNS server used for your on-premises domain. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct IpRoute {
    /// <p>IP address block using CIDR format, for example 10.0.0.0/24. This is often the address block of the DNS server used for your on-premises domain. For a single IP address use a CIDR address block with /32. For example 10.0.0.0/32.</p>
    #[serde(rename = "CidrIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_ip: Option<String>,
    /// <p>Description of the address block.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// <p>Information about one or more IP address blocks.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct IpRouteInfo {
    /// <p>The date and time the address block was added to the directory.</p>
    #[serde(rename = "AddedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added_date_time: Option<f64>,
    /// <p>IP address block in the <a>IpRoute</a>.</p>
    #[serde(rename = "CidrIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_ip: Option<String>,
    /// <p>Description of the <a>IpRouteInfo</a>.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Identifier (ID) of the directory associated with the IP addresses.</p>
    #[serde(rename = "DirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    /// <p>The status of the IP address block.</p>
    #[serde(rename = "IpRouteStatusMsg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_route_status_msg: Option<String>,
    /// <p>The reason for the IpRouteStatusMsg.</p>
    #[serde(rename = "IpRouteStatusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_route_status_reason: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListIpRoutesRequest {
    /// <p>Identifier (ID) of the directory for which you want to retrieve the IP addresses.</p>
    #[serde(rename = "DirectoryId")]
    pub directory_id: String,
    /// <p>Maximum number of items to return. If this value is zero, the maximum number of items is specified by the limitations of the operation.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <i>ListIpRoutes.NextToken</i> value from a previous call to <a>ListIpRoutes</a>. Pass null if this is the first call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListIpRoutesResult {
    /// <p>A list of <a>IpRoute</a>s.</p>
    #[serde(rename = "IpRoutesInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_routes_info: Option<Vec<IpRouteInfo>>,
    /// <p>If not null, more results are available. Pass this value for the <i>NextToken</i> parameter in a subsequent call to <a>ListIpRoutes</a> to retrieve the next set of items.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListLogSubscriptionsRequest {
    /// <p>If a <i>DirectoryID</i> is provided, lists only the log subscription associated with that directory. If no <i>DirectoryId</i> is provided, lists all log subscriptions associated with your AWS account. If there are no log subscriptions for the AWS account or the directory, an empty list will be returned.</p>
    #[serde(rename = "DirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    /// <p>The maximum number of items returned.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The token for the next set of items to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListLogSubscriptionsResult {
    /// <p>A list of active <a>LogSubscription</a> objects for calling the AWS account.</p>
    #[serde(rename = "LogSubscriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_subscriptions: Option<Vec<LogSubscription>>,
    /// <p>The token for the next set of items to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListSchemaExtensionsRequest {
    /// <p>The identifier of the directory from which to retrieve the schema extension information.</p>
    #[serde(rename = "DirectoryId")]
    pub directory_id: String,
    /// <p>The maximum number of items to return.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <code>ListSchemaExtensions.NextToken</code> value from a previous call to <code>ListSchemaExtensions</code>. Pass null if this is the first call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListSchemaExtensionsResult {
    /// <p>If not null, more results are available. Pass this value for the <code>NextToken</code> parameter in a subsequent call to <code>ListSchemaExtensions</code> to retrieve the next set of items.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the schema extensions applied to the directory.</p>
    #[serde(rename = "SchemaExtensionsInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_extensions_info: Option<Vec<SchemaExtensionInfo>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTagsForResourceRequest {
    /// <p>Reserved for future use.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>Reserved for future use.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Identifier (ID) of the directory for which you want to retrieve tags.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListTagsForResourceResult {
    /// <p>Reserved for future use.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>List of tags returned by the ListTagsForResource operation.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Represents a log subscription, which tracks real-time data from a chosen log group to a specified destination.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct LogSubscription {
    /// <p>Identifier (ID) of the directory that you want to associate with the log subscription.</p>
    #[serde(rename = "DirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    /// <p>The name of the log group.</p>
    #[serde(rename = "LogGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
    /// <p>The date and time that the log subscription was created.</p>
    #[serde(rename = "SubscriptionCreatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_created_date_time: Option<f64>,
}

/// <p>Describes the directory owner account details that have been shared to the directory consumer account.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct OwnerDirectoryDescription {
    /// <p>Identifier of the directory owner account.</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>Identifier of the AWS Managed Microsoft AD directory in the directory owner account.</p>
    #[serde(rename = "DirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    /// <p>IP address of the directory’s domain controllers.</p>
    #[serde(rename = "DnsIpAddrs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_ip_addrs: Option<Vec<String>>,
    /// <p>A <a>RadiusSettings</a> object that contains information about the RADIUS server.</p>
    #[serde(rename = "RadiusSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_settings: Option<RadiusSettings>,
    /// <p>Information about the status of the RADIUS server.</p>
    #[serde(rename = "RadiusStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_status: Option<String>,
    /// <p>Information about the VPC settings for the directory.</p>
    #[serde(rename = "VpcSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_settings: Option<DirectoryVpcSettingsDescription>,
}

/// <p>Contains information about a Remote Authentication Dial In User Service (RADIUS) server.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RadiusSettings {
    /// <p>The protocol specified for your RADIUS endpoints.</p>
    #[serde(rename = "AuthenticationProtocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_protocol: Option<String>,
    /// <p>Not currently used.</p>
    #[serde(rename = "DisplayLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_label: Option<String>,
    /// <p>The port that your RADIUS server is using for communications. Your on-premises network must allow inbound traffic over this port from the AWS Directory Service servers.</p>
    #[serde(rename = "RadiusPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_port: Option<i64>,
    /// <p>The maximum number of times that communication with the RADIUS server is attempted.</p>
    #[serde(rename = "RadiusRetries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_retries: Option<i64>,
    /// <p>An array of strings that contains the IP addresses of the RADIUS server endpoints, or the IP addresses of your RADIUS server load balancer.</p>
    #[serde(rename = "RadiusServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_servers: Option<Vec<String>>,
    /// <p>The amount of time, in seconds, to wait for the RADIUS server to respond.</p>
    #[serde(rename = "RadiusTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_timeout: Option<i64>,
    /// <p>Required for enabling RADIUS on the directory.</p>
    #[serde(rename = "SharedSecret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_secret: Option<String>,
    /// <p>Not currently used.</p>
    #[serde(rename = "UseSameUsername")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_same_username: Option<bool>,
}

/// <p>Registers a new event topic.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RegisterEventTopicRequest {
    /// <p>The Directory ID that will publish status messages to the SNS topic.</p>
    #[serde(rename = "DirectoryId")]
    pub directory_id: String,
    /// <p>The SNS topic name to which the directory will publish status messages. This SNS topic must be in the same region as the specified Directory ID.</p>
    #[serde(rename = "TopicName")]
    pub topic_name: String,
}

/// <p>The result of a RegisterEventTopic request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RegisterEventTopicResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RejectSharedDirectoryRequest {
    /// <p>Identifier of the shared directory in the directory consumer account. This identifier is different for each directory owner account.</p>
    #[serde(rename = "SharedDirectoryId")]
    pub shared_directory_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RejectSharedDirectoryResult {
    /// <p>Identifier of the shared directory in the directory consumer account.</p>
    #[serde(rename = "SharedDirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_directory_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RemoveIpRoutesRequest {
    /// <p>IP address blocks that you want to remove.</p>
    #[serde(rename = "CidrIps")]
    pub cidr_ips: Vec<String>,
    /// <p>Identifier (ID) of the directory from which you want to remove the IP addresses.</p>
    #[serde(rename = "DirectoryId")]
    pub directory_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RemoveIpRoutesResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RemoveTagsFromResourceRequest {
    /// <p>Identifier (ID) of the directory from which to remove the tag.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>The tag key (name) of the tag to be removed.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RemoveTagsFromResourceResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ResetUserPasswordRequest {
    /// <p>Identifier of the AWS Managed Microsoft AD or Simple AD directory in which the user resides.</p>
    #[serde(rename = "DirectoryId")]
    pub directory_id: String,
    /// <p>The new password that will be reset.</p>
    #[serde(rename = "NewPassword")]
    pub new_password: String,
    /// <p>The user name of the user whose password will be reset.</p>
    #[serde(rename = "UserName")]
    pub user_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ResetUserPasswordResult {}

/// <p>An object representing the inputs for the <a>RestoreFromSnapshot</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RestoreFromSnapshotRequest {
    /// <p>The identifier of the snapshot to restore from.</p>
    #[serde(rename = "SnapshotId")]
    pub snapshot_id: String,
}

/// <p>Contains the results of the <a>RestoreFromSnapshot</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RestoreFromSnapshotResult {}

/// <p>Information about a schema extension.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SchemaExtensionInfo {
    /// <p>A description of the schema extension.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The identifier of the directory to which the schema extension is applied.</p>
    #[serde(rename = "DirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    /// <p>The date and time that the schema extension was completed.</p>
    #[serde(rename = "EndDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_time: Option<f64>,
    /// <p>The identifier of the schema extension.</p>
    #[serde(rename = "SchemaExtensionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_extension_id: Option<String>,
    /// <p>The current status of the schema extension.</p>
    #[serde(rename = "SchemaExtensionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_extension_status: Option<String>,
    /// <p>The reason for the <code>SchemaExtensionStatus</code>.</p>
    #[serde(rename = "SchemaExtensionStatusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_extension_status_reason: Option<String>,
    /// <p>The date and time that the schema extension started being applied to the directory.</p>
    #[serde(rename = "StartDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date_time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ShareDirectoryRequest {
    /// <p>Identifier of the AWS Managed Microsoft AD directory that you want to share with other AWS accounts.</p>
    #[serde(rename = "DirectoryId")]
    pub directory_id: String,
    /// <p>The method used when sharing a directory to determine whether the directory should be shared within your AWS organization (<code>ORGANIZATIONS</code>) or with any AWS account by sending a directory sharing request (<code>HANDSHAKE</code>).</p>
    #[serde(rename = "ShareMethod")]
    pub share_method: String,
    /// <p>A directory share request that is sent by the directory owner to the directory consumer. The request includes a typed message to help the directory consumer administrator determine whether to approve or reject the share invitation.</p>
    #[serde(rename = "ShareNotes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_notes: Option<String>,
    /// <p>Identifier for the directory consumer account with whom the directory is to be shared.</p>
    #[serde(rename = "ShareTarget")]
    pub share_target: ShareTarget,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ShareDirectoryResult {
    /// <p>Identifier of the directory that is stored in the directory consumer account that is shared from the specified directory (<code>DirectoryId</code>).</p>
    #[serde(rename = "SharedDirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_directory_id: Option<String>,
}

/// <p>Identifier that contains details about the directory consumer account.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ShareTarget {
    /// <p>Identifier of the directory consumer account.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>Type of identifier to be used in the <code>Id</code> field.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

/// <p>Details about the shared directory in the directory owner account for which the share request in the directory consumer account has been accepted.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SharedDirectory {
    /// <p>The date and time that the shared directory was created.</p>
    #[serde(rename = "CreatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<f64>,
    /// <p>The date and time that the shared directory was last updated.</p>
    #[serde(rename = "LastUpdatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    /// <p>Identifier of the directory owner account, which contains the directory that has been shared to the consumer account.</p>
    #[serde(rename = "OwnerAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account_id: Option<String>,
    /// <p>Identifier of the directory in the directory owner account. </p>
    #[serde(rename = "OwnerDirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_directory_id: Option<String>,
    /// <p>The method used when sharing a directory to determine whether the directory should be shared within your AWS organization (<code>ORGANIZATIONS</code>) or with any AWS account by sending a shared directory request (<code>HANDSHAKE</code>).</p>
    #[serde(rename = "ShareMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_method: Option<String>,
    /// <p>A directory share request that is sent by the directory owner to the directory consumer. The request includes a typed message to help the directory consumer administrator determine whether to approve or reject the share invitation.</p>
    #[serde(rename = "ShareNotes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_notes: Option<String>,
    /// <p>Current directory status of the shared AWS Managed Microsoft AD directory.</p>
    #[serde(rename = "ShareStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_status: Option<String>,
    /// <p>Identifier of the directory consumer account that has access to the shared directory (<code>OwnerDirectoryId</code>) in the directory owner account.</p>
    #[serde(rename = "SharedAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_account_id: Option<String>,
    /// <p>Identifier of the shared directory in the directory consumer account. This identifier is different for each directory owner account.</p>
    #[serde(rename = "SharedDirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_directory_id: Option<String>,
}

/// <p>Describes a directory snapshot.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Snapshot {
    /// <p>The directory identifier.</p>
    #[serde(rename = "DirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    /// <p>The descriptive name of the snapshot.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The snapshot identifier.</p>
    #[serde(rename = "SnapshotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    /// <p>The date and time that the snapshot was taken.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The snapshot status.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The snapshot type.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Contains manual snapshot limit information for a directory.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SnapshotLimits {
    /// <p>The current number of manual snapshots of the directory.</p>
    #[serde(rename = "ManualSnapshotsCurrentCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_snapshots_current_count: Option<i64>,
    /// <p>The maximum number of manual snapshots allowed.</p>
    #[serde(rename = "ManualSnapshotsLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_snapshots_limit: Option<i64>,
    /// <p>Indicates if the manual snapshot limit has been reached.</p>
    #[serde(rename = "ManualSnapshotsLimitReached")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_snapshots_limit_reached: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartSchemaExtensionRequest {
    /// <p>If true, creates a snapshot of the directory before applying the schema extension.</p>
    #[serde(rename = "CreateSnapshotBeforeSchemaExtension")]
    pub create_snapshot_before_schema_extension: bool,
    /// <p>A description of the schema extension.</p>
    #[serde(rename = "Description")]
    pub description: String,
    /// <p>The identifier of the directory for which the schema extension will be applied to.</p>
    #[serde(rename = "DirectoryId")]
    pub directory_id: String,
    /// <p>The LDIF file represented as a string. To construct the LdifContent string, precede each line as it would be formatted in an ldif file with \n. See the example request below for more details. The file size can be no larger than 1MB.</p>
    #[serde(rename = "LdifContent")]
    pub ldif_content: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartSchemaExtensionResult {
    /// <p>The identifier of the schema extension that will be applied.</p>
    #[serde(rename = "SchemaExtensionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_extension_id: Option<String>,
}

/// <p>Metadata assigned to a directory consisting of a key-value pair.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>Required name of the tag. The string value can be Unicode characters and cannot be prefixed with "aws:". The string can contain only the set of Unicode letters, digits, white-space, '_', '.', '/', '=', '+', '-' (Java regex: "^([\\p{L}\\p{Z}\\p{N}_.:/=+\\-]*)$").</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The optional value of the tag. The string value can be Unicode characters. The string can contain only the set of Unicode letters, digits, white-space, '_', '.', '/', '=', '+', '-' (Java regex: "^([\\p{L}\\p{Z}\\p{N}_.:/=+\\-]*)$").</p>
    #[serde(rename = "Value")]
    pub value: String,
}

/// <p>Describes a trust relationship between an AWS Managed Microsoft AD directory and an external domain.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Trust {
    /// <p>The date and time that the trust relationship was created.</p>
    #[serde(rename = "CreatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<f64>,
    /// <p>The Directory ID of the AWS directory involved in the trust relationship.</p>
    #[serde(rename = "DirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    /// <p>The date and time that the trust relationship was last updated.</p>
    #[serde(rename = "LastUpdatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    /// <p>The Fully Qualified Domain Name (FQDN) of the external domain involved in the trust relationship.</p>
    #[serde(rename = "RemoteDomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_domain_name: Option<String>,
    /// <p>Current state of selective authentication for the trust.</p>
    #[serde(rename = "SelectiveAuth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective_auth: Option<String>,
    /// <p>The date and time that the TrustState was last updated.</p>
    #[serde(rename = "StateLastUpdatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_last_updated_date_time: Option<f64>,
    /// <p>The trust relationship direction.</p>
    #[serde(rename = "TrustDirection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_direction: Option<String>,
    /// <p>The unique ID of the trust relationship.</p>
    #[serde(rename = "TrustId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_id: Option<String>,
    /// <p>The trust relationship state.</p>
    #[serde(rename = "TrustState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_state: Option<String>,
    /// <p>The reason for the TrustState.</p>
    #[serde(rename = "TrustStateReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_state_reason: Option<String>,
    /// <p>The trust relationship type. <code>Forest</code> is the default.</p>
    #[serde(rename = "TrustType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UnshareDirectoryRequest {
    /// <p>The identifier of the AWS Managed Microsoft AD directory that you want to stop sharing.</p>
    #[serde(rename = "DirectoryId")]
    pub directory_id: String,
    /// <p>Identifier for the directory consumer account with whom the directory has to be unshared.</p>
    #[serde(rename = "UnshareTarget")]
    pub unshare_target: UnshareTarget,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UnshareDirectoryResult {
    /// <p>Identifier of the directory stored in the directory consumer account that is to be unshared from the specified directory (<code>DirectoryId</code>).</p>
    #[serde(rename = "SharedDirectoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_directory_id: Option<String>,
}

/// <p>Identifier that contains details about the directory consumer account with whom the directory is being unshared.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UnshareTarget {
    /// <p>Identifier of the directory consumer account.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>Type of identifier to be used in the <i>Id</i> field.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

/// <p>Updates a conditional forwarder.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateConditionalForwarderRequest {
    /// <p>The directory ID of the AWS directory for which to update the conditional forwarder.</p>
    #[serde(rename = "DirectoryId")]
    pub directory_id: String,
    /// <p>The updated IP addresses of the remote DNS server associated with the conditional forwarder.</p>
    #[serde(rename = "DnsIpAddrs")]
    pub dns_ip_addrs: Vec<String>,
    /// <p>The fully qualified domain name (FQDN) of the remote domain with which you will set up a trust relationship.</p>
    #[serde(rename = "RemoteDomainName")]
    pub remote_domain_name: String,
}

/// <p>The result of an UpdateConditionalForwarder request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateConditionalForwarderResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateNumberOfDomainControllersRequest {
    /// <p>The number of domain controllers desired in the directory.</p>
    #[serde(rename = "DesiredNumber")]
    pub desired_number: i64,
    /// <p>Identifier of the directory to which the domain controllers will be added or removed.</p>
    #[serde(rename = "DirectoryId")]
    pub directory_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateNumberOfDomainControllersResult {}

/// <p>Contains the inputs for the <a>UpdateRadius</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateRadiusRequest {
    /// <p>The identifier of the directory for which to update the RADIUS server information.</p>
    #[serde(rename = "DirectoryId")]
    pub directory_id: String,
    /// <p>A <a>RadiusSettings</a> object that contains information about the RADIUS server.</p>
    #[serde(rename = "RadiusSettings")]
    pub radius_settings: RadiusSettings,
}

/// <p>Contains the results of the <a>UpdateRadius</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateRadiusResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateTrustRequest {
    /// <p>Updates selective authentication for the trust.</p>
    #[serde(rename = "SelectiveAuth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective_auth: Option<String>,
    /// <p>Identifier of the trust relationship.</p>
    #[serde(rename = "TrustId")]
    pub trust_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateTrustResult {
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>Identifier of the trust relationship.</p>
    #[serde(rename = "TrustId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_id: Option<String>,
}

/// <p>Initiates the verification of an existing trust relationship between an AWS Managed Microsoft AD directory and an external domain.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct VerifyTrustRequest {
    /// <p>The unique Trust ID of the trust relationship to verify.</p>
    #[serde(rename = "TrustId")]
    pub trust_id: String,
}

/// <p>Result of a VerifyTrust request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct VerifyTrustResult {
    /// <p>The unique Trust ID of the trust relationship that was verified.</p>
    #[serde(rename = "TrustId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_id: Option<String>,
}

/// Errors returned by AcceptSharedDirectory
#[derive(Debug, PartialEq)]
pub enum AcceptSharedDirectoryError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified directory has already been shared with this AWS account.</p>
    DirectoryAlreadyShared(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl AcceptSharedDirectoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AcceptSharedDirectoryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(AcceptSharedDirectoryError::Client(err.msg))
                }
                "DirectoryAlreadySharedException" => {
                    return RusotoError::Service(
                        AcceptSharedDirectoryError::DirectoryAlreadyShared(err.msg),
                    )
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(AcceptSharedDirectoryError::EntityDoesNotExist(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AcceptSharedDirectoryError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(AcceptSharedDirectoryError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AcceptSharedDirectoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AcceptSharedDirectoryError {
    fn description(&self) -> &str {
        match *self {
            AcceptSharedDirectoryError::Client(ref cause) => cause,
            AcceptSharedDirectoryError::DirectoryAlreadyShared(ref cause) => cause,
            AcceptSharedDirectoryError::EntityDoesNotExist(ref cause) => cause,
            AcceptSharedDirectoryError::InvalidParameter(ref cause) => cause,
            AcceptSharedDirectoryError::Service(ref cause) => cause,
        }
    }
}
/// Errors returned by AddIpRoutes
#[derive(Debug, PartialEq)]
pub enum AddIpRoutesError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified directory is unavailable or could not be found.</p>
    DirectoryUnavailable(String),
    /// <p>The specified entity already exists.</p>
    EntityAlreadyExists(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The maximum allowed number of IP addresses was exceeded. The default limit is 100 IP address blocks.</p>
    IpRouteLimitExceeded(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl AddIpRoutesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddIpRoutesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(AddIpRoutesError::Client(err.msg))
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(AddIpRoutesError::DirectoryUnavailable(err.msg))
                }
                "EntityAlreadyExistsException" => {
                    return RusotoError::Service(AddIpRoutesError::EntityAlreadyExists(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(AddIpRoutesError::EntityDoesNotExist(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AddIpRoutesError::InvalidParameter(err.msg))
                }
                "IpRouteLimitExceededException" => {
                    return RusotoError::Service(AddIpRoutesError::IpRouteLimitExceeded(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(AddIpRoutesError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AddIpRoutesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddIpRoutesError {
    fn description(&self) -> &str {
        match *self {
            AddIpRoutesError::Client(ref cause) => cause,
            AddIpRoutesError::DirectoryUnavailable(ref cause) => cause,
            AddIpRoutesError::EntityAlreadyExists(ref cause) => cause,
            AddIpRoutesError::EntityDoesNotExist(ref cause) => cause,
            AddIpRoutesError::InvalidParameter(ref cause) => cause,
            AddIpRoutesError::IpRouteLimitExceeded(ref cause) => cause,
            AddIpRoutesError::Service(ref cause) => cause,
        }
    }
}
/// Errors returned by AddTagsToResource
#[derive(Debug, PartialEq)]
pub enum AddTagsToResourceError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The maximum allowed number of tags was exceeded.</p>
    TagLimitExceeded(String),
}

impl AddTagsToResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddTagsToResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(AddTagsToResourceError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(AddTagsToResourceError::EntityDoesNotExist(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AddTagsToResourceError::InvalidParameter(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(AddTagsToResourceError::Service(err.msg))
                }
                "TagLimitExceededException" => {
                    return RusotoError::Service(AddTagsToResourceError::TagLimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
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
            AddTagsToResourceError::Client(ref cause) => cause,
            AddTagsToResourceError::EntityDoesNotExist(ref cause) => cause,
            AddTagsToResourceError::InvalidParameter(ref cause) => cause,
            AddTagsToResourceError::Service(ref cause) => cause,
            AddTagsToResourceError::TagLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by CancelSchemaExtension
#[derive(Debug, PartialEq)]
pub enum CancelSchemaExtensionError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl CancelSchemaExtensionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CancelSchemaExtensionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(CancelSchemaExtensionError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(CancelSchemaExtensionError::EntityDoesNotExist(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(CancelSchemaExtensionError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CancelSchemaExtensionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CancelSchemaExtensionError {
    fn description(&self) -> &str {
        match *self {
            CancelSchemaExtensionError::Client(ref cause) => cause,
            CancelSchemaExtensionError::EntityDoesNotExist(ref cause) => cause,
            CancelSchemaExtensionError::Service(ref cause) => cause,
        }
    }
}
/// Errors returned by ConnectDirectory
#[derive(Debug, PartialEq)]
pub enum ConnectDirectoryError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The maximum number of directories in the region has been reached. You can use the <a>GetDirectoryLimits</a> operation to determine your directory limits in the region.</p>
    DirectoryLimitExceeded(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl ConnectDirectoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ConnectDirectoryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(ConnectDirectoryError::Client(err.msg))
                }
                "DirectoryLimitExceededException" => {
                    return RusotoError::Service(ConnectDirectoryError::DirectoryLimitExceeded(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ConnectDirectoryError::InvalidParameter(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(ConnectDirectoryError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ConnectDirectoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ConnectDirectoryError {
    fn description(&self) -> &str {
        match *self {
            ConnectDirectoryError::Client(ref cause) => cause,
            ConnectDirectoryError::DirectoryLimitExceeded(ref cause) => cause,
            ConnectDirectoryError::InvalidParameter(ref cause) => cause,
            ConnectDirectoryError::Service(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateAlias
#[derive(Debug, PartialEq)]
pub enum CreateAliasError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity already exists.</p>
    EntityAlreadyExists(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl CreateAliasError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateAliasError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(CreateAliasError::Client(err.msg))
                }
                "EntityAlreadyExistsException" => {
                    return RusotoError::Service(CreateAliasError::EntityAlreadyExists(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(CreateAliasError::EntityDoesNotExist(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateAliasError::InvalidParameter(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateAliasError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateAliasError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateAliasError {
    fn description(&self) -> &str {
        match *self {
            CreateAliasError::Client(ref cause) => cause,
            CreateAliasError::EntityAlreadyExists(ref cause) => cause,
            CreateAliasError::EntityDoesNotExist(ref cause) => cause,
            CreateAliasError::InvalidParameter(ref cause) => cause,
            CreateAliasError::Service(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateComputer
#[derive(Debug, PartialEq)]
pub enum CreateComputerError {
    /// <p>An authentication error occurred.</p>
    AuthenticationFailed(String),
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified directory is unavailable or could not be found.</p>
    DirectoryUnavailable(String),
    /// <p>The specified entity already exists.</p>
    EntityAlreadyExists(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
}

impl CreateComputerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateComputerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AuthenticationFailedException" => {
                    return RusotoError::Service(CreateComputerError::AuthenticationFailed(err.msg))
                }
                "ClientException" => {
                    return RusotoError::Service(CreateComputerError::Client(err.msg))
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(CreateComputerError::DirectoryUnavailable(err.msg))
                }
                "EntityAlreadyExistsException" => {
                    return RusotoError::Service(CreateComputerError::EntityAlreadyExists(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(CreateComputerError::EntityDoesNotExist(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateComputerError::InvalidParameter(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateComputerError::Service(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(CreateComputerError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateComputerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateComputerError {
    fn description(&self) -> &str {
        match *self {
            CreateComputerError::AuthenticationFailed(ref cause) => cause,
            CreateComputerError::Client(ref cause) => cause,
            CreateComputerError::DirectoryUnavailable(ref cause) => cause,
            CreateComputerError::EntityAlreadyExists(ref cause) => cause,
            CreateComputerError::EntityDoesNotExist(ref cause) => cause,
            CreateComputerError::InvalidParameter(ref cause) => cause,
            CreateComputerError::Service(ref cause) => cause,
            CreateComputerError::UnsupportedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateConditionalForwarder
#[derive(Debug, PartialEq)]
pub enum CreateConditionalForwarderError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified directory is unavailable or could not be found.</p>
    DirectoryUnavailable(String),
    /// <p>The specified entity already exists.</p>
    EntityAlreadyExists(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
}

impl CreateConditionalForwarderError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateConditionalForwarderError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(CreateConditionalForwarderError::Client(err.msg))
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(
                        CreateConditionalForwarderError::DirectoryUnavailable(err.msg),
                    )
                }
                "EntityAlreadyExistsException" => {
                    return RusotoError::Service(
                        CreateConditionalForwarderError::EntityAlreadyExists(err.msg),
                    )
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(
                        CreateConditionalForwarderError::EntityDoesNotExist(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateConditionalForwarderError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateConditionalForwarderError::Service(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(
                        CreateConditionalForwarderError::UnsupportedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateConditionalForwarderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateConditionalForwarderError {
    fn description(&self) -> &str {
        match *self {
            CreateConditionalForwarderError::Client(ref cause) => cause,
            CreateConditionalForwarderError::DirectoryUnavailable(ref cause) => cause,
            CreateConditionalForwarderError::EntityAlreadyExists(ref cause) => cause,
            CreateConditionalForwarderError::EntityDoesNotExist(ref cause) => cause,
            CreateConditionalForwarderError::InvalidParameter(ref cause) => cause,
            CreateConditionalForwarderError::Service(ref cause) => cause,
            CreateConditionalForwarderError::UnsupportedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDirectory
#[derive(Debug, PartialEq)]
pub enum CreateDirectoryError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The maximum number of directories in the region has been reached. You can use the <a>GetDirectoryLimits</a> operation to determine your directory limits in the region.</p>
    DirectoryLimitExceeded(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl CreateDirectoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDirectoryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(CreateDirectoryError::Client(err.msg))
                }
                "DirectoryLimitExceededException" => {
                    return RusotoError::Service(CreateDirectoryError::DirectoryLimitExceeded(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateDirectoryError::InvalidParameter(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateDirectoryError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateDirectoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDirectoryError {
    fn description(&self) -> &str {
        match *self {
            CreateDirectoryError::Client(ref cause) => cause,
            CreateDirectoryError::DirectoryLimitExceeded(ref cause) => cause,
            CreateDirectoryError::InvalidParameter(ref cause) => cause,
            CreateDirectoryError::Service(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateLogSubscription
#[derive(Debug, PartialEq)]
pub enum CreateLogSubscriptionError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity already exists.</p>
    EntityAlreadyExists(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>The account does not have sufficient permission to perform the operation.</p>
    InsufficientPermissions(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
}

impl CreateLogSubscriptionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateLogSubscriptionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(CreateLogSubscriptionError::Client(err.msg))
                }
                "EntityAlreadyExistsException" => {
                    return RusotoError::Service(CreateLogSubscriptionError::EntityAlreadyExists(
                        err.msg,
                    ))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(CreateLogSubscriptionError::EntityDoesNotExist(
                        err.msg,
                    ))
                }
                "InsufficientPermissionsException" => {
                    return RusotoError::Service(
                        CreateLogSubscriptionError::InsufficientPermissions(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateLogSubscriptionError::Service(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(CreateLogSubscriptionError::UnsupportedOperation(
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
impl fmt::Display for CreateLogSubscriptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateLogSubscriptionError {
    fn description(&self) -> &str {
        match *self {
            CreateLogSubscriptionError::Client(ref cause) => cause,
            CreateLogSubscriptionError::EntityAlreadyExists(ref cause) => cause,
            CreateLogSubscriptionError::EntityDoesNotExist(ref cause) => cause,
            CreateLogSubscriptionError::InsufficientPermissions(ref cause) => cause,
            CreateLogSubscriptionError::Service(ref cause) => cause,
            CreateLogSubscriptionError::UnsupportedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateMicrosoftAD
#[derive(Debug, PartialEq)]
pub enum CreateMicrosoftADError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The maximum number of directories in the region has been reached. You can use the <a>GetDirectoryLimits</a> operation to determine your directory limits in the region.</p>
    DirectoryLimitExceeded(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
}

impl CreateMicrosoftADError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateMicrosoftADError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(CreateMicrosoftADError::Client(err.msg))
                }
                "DirectoryLimitExceededException" => {
                    return RusotoError::Service(CreateMicrosoftADError::DirectoryLimitExceeded(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateMicrosoftADError::InvalidParameter(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateMicrosoftADError::Service(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(CreateMicrosoftADError::UnsupportedOperation(
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
impl fmt::Display for CreateMicrosoftADError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateMicrosoftADError {
    fn description(&self) -> &str {
        match *self {
            CreateMicrosoftADError::Client(ref cause) => cause,
            CreateMicrosoftADError::DirectoryLimitExceeded(ref cause) => cause,
            CreateMicrosoftADError::InvalidParameter(ref cause) => cause,
            CreateMicrosoftADError::Service(ref cause) => cause,
            CreateMicrosoftADError::UnsupportedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateSnapshot
#[derive(Debug, PartialEq)]
pub enum CreateSnapshotError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The maximum number of manual snapshots for the directory has been reached. You can use the <a>GetSnapshotLimits</a> operation to determine the snapshot limits for a directory.</p>
    SnapshotLimitExceeded(String),
}

impl CreateSnapshotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateSnapshotError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(CreateSnapshotError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(CreateSnapshotError::EntityDoesNotExist(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateSnapshotError::InvalidParameter(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateSnapshotError::Service(err.msg))
                }
                "SnapshotLimitExceededException" => {
                    return RusotoError::Service(CreateSnapshotError::SnapshotLimitExceeded(
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
impl fmt::Display for CreateSnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateSnapshotError {
    fn description(&self) -> &str {
        match *self {
            CreateSnapshotError::Client(ref cause) => cause,
            CreateSnapshotError::EntityDoesNotExist(ref cause) => cause,
            CreateSnapshotError::InvalidParameter(ref cause) => cause,
            CreateSnapshotError::Service(ref cause) => cause,
            CreateSnapshotError::SnapshotLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateTrust
#[derive(Debug, PartialEq)]
pub enum CreateTrustError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity already exists.</p>
    EntityAlreadyExists(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
}

impl CreateTrustError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateTrustError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(CreateTrustError::Client(err.msg))
                }
                "EntityAlreadyExistsException" => {
                    return RusotoError::Service(CreateTrustError::EntityAlreadyExists(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(CreateTrustError::EntityDoesNotExist(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateTrustError::InvalidParameter(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateTrustError::Service(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(CreateTrustError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateTrustError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateTrustError {
    fn description(&self) -> &str {
        match *self {
            CreateTrustError::Client(ref cause) => cause,
            CreateTrustError::EntityAlreadyExists(ref cause) => cause,
            CreateTrustError::EntityDoesNotExist(ref cause) => cause,
            CreateTrustError::InvalidParameter(ref cause) => cause,
            CreateTrustError::Service(ref cause) => cause,
            CreateTrustError::UnsupportedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteConditionalForwarder
#[derive(Debug, PartialEq)]
pub enum DeleteConditionalForwarderError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified directory is unavailable or could not be found.</p>
    DirectoryUnavailable(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
}

impl DeleteConditionalForwarderError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteConditionalForwarderError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DeleteConditionalForwarderError::Client(err.msg))
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(
                        DeleteConditionalForwarderError::DirectoryUnavailable(err.msg),
                    )
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(
                        DeleteConditionalForwarderError::EntityDoesNotExist(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteConditionalForwarderError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteConditionalForwarderError::Service(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(
                        DeleteConditionalForwarderError::UnsupportedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteConditionalForwarderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteConditionalForwarderError {
    fn description(&self) -> &str {
        match *self {
            DeleteConditionalForwarderError::Client(ref cause) => cause,
            DeleteConditionalForwarderError::DirectoryUnavailable(ref cause) => cause,
            DeleteConditionalForwarderError::EntityDoesNotExist(ref cause) => cause,
            DeleteConditionalForwarderError::InvalidParameter(ref cause) => cause,
            DeleteConditionalForwarderError::Service(ref cause) => cause,
            DeleteConditionalForwarderError::UnsupportedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDirectory
#[derive(Debug, PartialEq)]
pub enum DeleteDirectoryError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl DeleteDirectoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDirectoryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DeleteDirectoryError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(DeleteDirectoryError::EntityDoesNotExist(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteDirectoryError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteDirectoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDirectoryError {
    fn description(&self) -> &str {
        match *self {
            DeleteDirectoryError::Client(ref cause) => cause,
            DeleteDirectoryError::EntityDoesNotExist(ref cause) => cause,
            DeleteDirectoryError::Service(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteLogSubscription
#[derive(Debug, PartialEq)]
pub enum DeleteLogSubscriptionError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
}

impl DeleteLogSubscriptionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteLogSubscriptionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DeleteLogSubscriptionError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(DeleteLogSubscriptionError::EntityDoesNotExist(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteLogSubscriptionError::Service(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(DeleteLogSubscriptionError::UnsupportedOperation(
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
impl fmt::Display for DeleteLogSubscriptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteLogSubscriptionError {
    fn description(&self) -> &str {
        match *self {
            DeleteLogSubscriptionError::Client(ref cause) => cause,
            DeleteLogSubscriptionError::EntityDoesNotExist(ref cause) => cause,
            DeleteLogSubscriptionError::Service(ref cause) => cause,
            DeleteLogSubscriptionError::UnsupportedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteSnapshot
#[derive(Debug, PartialEq)]
pub enum DeleteSnapshotError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl DeleteSnapshotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteSnapshotError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DeleteSnapshotError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(DeleteSnapshotError::EntityDoesNotExist(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteSnapshotError::InvalidParameter(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteSnapshotError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteSnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteSnapshotError {
    fn description(&self) -> &str {
        match *self {
            DeleteSnapshotError::Client(ref cause) => cause,
            DeleteSnapshotError::EntityDoesNotExist(ref cause) => cause,
            DeleteSnapshotError::InvalidParameter(ref cause) => cause,
            DeleteSnapshotError::Service(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteTrust
#[derive(Debug, PartialEq)]
pub enum DeleteTrustError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
}

impl DeleteTrustError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteTrustError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DeleteTrustError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(DeleteTrustError::EntityDoesNotExist(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteTrustError::InvalidParameter(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteTrustError::Service(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(DeleteTrustError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteTrustError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteTrustError {
    fn description(&self) -> &str {
        match *self {
            DeleteTrustError::Client(ref cause) => cause,
            DeleteTrustError::EntityDoesNotExist(ref cause) => cause,
            DeleteTrustError::InvalidParameter(ref cause) => cause,
            DeleteTrustError::Service(ref cause) => cause,
            DeleteTrustError::UnsupportedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by DeregisterEventTopic
#[derive(Debug, PartialEq)]
pub enum DeregisterEventTopicError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl DeregisterEventTopicError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeregisterEventTopicError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DeregisterEventTopicError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(DeregisterEventTopicError::EntityDoesNotExist(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeregisterEventTopicError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeregisterEventTopicError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeregisterEventTopicError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeregisterEventTopicError {
    fn description(&self) -> &str {
        match *self {
            DeregisterEventTopicError::Client(ref cause) => cause,
            DeregisterEventTopicError::EntityDoesNotExist(ref cause) => cause,
            DeregisterEventTopicError::InvalidParameter(ref cause) => cause,
            DeregisterEventTopicError::Service(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeConditionalForwarders
#[derive(Debug, PartialEq)]
pub enum DescribeConditionalForwardersError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified directory is unavailable or could not be found.</p>
    DirectoryUnavailable(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
}

impl DescribeConditionalForwardersError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeConditionalForwardersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DescribeConditionalForwardersError::Client(
                        err.msg,
                    ))
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(
                        DescribeConditionalForwardersError::DirectoryUnavailable(err.msg),
                    )
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(
                        DescribeConditionalForwardersError::EntityDoesNotExist(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        DescribeConditionalForwardersError::InvalidParameter(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(DescribeConditionalForwardersError::Service(
                        err.msg,
                    ))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(
                        DescribeConditionalForwardersError::UnsupportedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeConditionalForwardersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeConditionalForwardersError {
    fn description(&self) -> &str {
        match *self {
            DescribeConditionalForwardersError::Client(ref cause) => cause,
            DescribeConditionalForwardersError::DirectoryUnavailable(ref cause) => cause,
            DescribeConditionalForwardersError::EntityDoesNotExist(ref cause) => cause,
            DescribeConditionalForwardersError::InvalidParameter(ref cause) => cause,
            DescribeConditionalForwardersError::Service(ref cause) => cause,
            DescribeConditionalForwardersError::UnsupportedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeDirectories
#[derive(Debug, PartialEq)]
pub enum DescribeDirectoriesError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>The <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl DescribeDirectoriesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeDirectoriesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DescribeDirectoriesError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(DescribeDirectoriesError::EntityDoesNotExist(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(DescribeDirectoriesError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeDirectoriesError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(DescribeDirectoriesError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeDirectoriesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDirectoriesError {
    fn description(&self) -> &str {
        match *self {
            DescribeDirectoriesError::Client(ref cause) => cause,
            DescribeDirectoriesError::EntityDoesNotExist(ref cause) => cause,
            DescribeDirectoriesError::InvalidNextToken(ref cause) => cause,
            DescribeDirectoriesError::InvalidParameter(ref cause) => cause,
            DescribeDirectoriesError::Service(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeDomainControllers
#[derive(Debug, PartialEq)]
pub enum DescribeDomainControllersError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>The <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
}

impl DescribeDomainControllersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeDomainControllersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DescribeDomainControllersError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(
                        DescribeDomainControllersError::EntityDoesNotExist(err.msg),
                    )
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(DescribeDomainControllersError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeDomainControllersError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(DescribeDomainControllersError::Service(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(
                        DescribeDomainControllersError::UnsupportedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeDomainControllersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDomainControllersError {
    fn description(&self) -> &str {
        match *self {
            DescribeDomainControllersError::Client(ref cause) => cause,
            DescribeDomainControllersError::EntityDoesNotExist(ref cause) => cause,
            DescribeDomainControllersError::InvalidNextToken(ref cause) => cause,
            DescribeDomainControllersError::InvalidParameter(ref cause) => cause,
            DescribeDomainControllersError::Service(ref cause) => cause,
            DescribeDomainControllersError::UnsupportedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeEventTopics
#[derive(Debug, PartialEq)]
pub enum DescribeEventTopicsError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl DescribeEventTopicsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeEventTopicsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DescribeEventTopicsError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(DescribeEventTopicsError::EntityDoesNotExist(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeEventTopicsError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(DescribeEventTopicsError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeEventTopicsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEventTopicsError {
    fn description(&self) -> &str {
        match *self {
            DescribeEventTopicsError::Client(ref cause) => cause,
            DescribeEventTopicsError::EntityDoesNotExist(ref cause) => cause,
            DescribeEventTopicsError::InvalidParameter(ref cause) => cause,
            DescribeEventTopicsError::Service(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeSharedDirectories
#[derive(Debug, PartialEq)]
pub enum DescribeSharedDirectoriesError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>The <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
}

impl DescribeSharedDirectoriesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeSharedDirectoriesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DescribeSharedDirectoriesError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(
                        DescribeSharedDirectoriesError::EntityDoesNotExist(err.msg),
                    )
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(DescribeSharedDirectoriesError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeSharedDirectoriesError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(DescribeSharedDirectoriesError::Service(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(
                        DescribeSharedDirectoriesError::UnsupportedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeSharedDirectoriesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeSharedDirectoriesError {
    fn description(&self) -> &str {
        match *self {
            DescribeSharedDirectoriesError::Client(ref cause) => cause,
            DescribeSharedDirectoriesError::EntityDoesNotExist(ref cause) => cause,
            DescribeSharedDirectoriesError::InvalidNextToken(ref cause) => cause,
            DescribeSharedDirectoriesError::InvalidParameter(ref cause) => cause,
            DescribeSharedDirectoriesError::Service(ref cause) => cause,
            DescribeSharedDirectoriesError::UnsupportedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeSnapshots
#[derive(Debug, PartialEq)]
pub enum DescribeSnapshotsError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>The <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl DescribeSnapshotsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeSnapshotsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DescribeSnapshotsError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(DescribeSnapshotsError::EntityDoesNotExist(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(DescribeSnapshotsError::InvalidNextToken(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeSnapshotsError::InvalidParameter(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DescribeSnapshotsError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeSnapshotsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeSnapshotsError {
    fn description(&self) -> &str {
        match *self {
            DescribeSnapshotsError::Client(ref cause) => cause,
            DescribeSnapshotsError::EntityDoesNotExist(ref cause) => cause,
            DescribeSnapshotsError::InvalidNextToken(ref cause) => cause,
            DescribeSnapshotsError::InvalidParameter(ref cause) => cause,
            DescribeSnapshotsError::Service(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeTrusts
#[derive(Debug, PartialEq)]
pub enum DescribeTrustsError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>The <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
}

impl DescribeTrustsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeTrustsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DescribeTrustsError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(DescribeTrustsError::EntityDoesNotExist(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(DescribeTrustsError::InvalidNextToken(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeTrustsError::InvalidParameter(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DescribeTrustsError::Service(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(DescribeTrustsError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeTrustsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeTrustsError {
    fn description(&self) -> &str {
        match *self {
            DescribeTrustsError::Client(ref cause) => cause,
            DescribeTrustsError::EntityDoesNotExist(ref cause) => cause,
            DescribeTrustsError::InvalidNextToken(ref cause) => cause,
            DescribeTrustsError::InvalidParameter(ref cause) => cause,
            DescribeTrustsError::Service(ref cause) => cause,
            DescribeTrustsError::UnsupportedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by DisableRadius
#[derive(Debug, PartialEq)]
pub enum DisableRadiusError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl DisableRadiusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisableRadiusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DisableRadiusError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(DisableRadiusError::EntityDoesNotExist(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DisableRadiusError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DisableRadiusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisableRadiusError {
    fn description(&self) -> &str {
        match *self {
            DisableRadiusError::Client(ref cause) => cause,
            DisableRadiusError::EntityDoesNotExist(ref cause) => cause,
            DisableRadiusError::Service(ref cause) => cause,
        }
    }
}
/// Errors returned by DisableSso
#[derive(Debug, PartialEq)]
pub enum DisableSsoError {
    /// <p>An authentication error occurred.</p>
    AuthenticationFailed(String),
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>The account does not have sufficient permission to perform the operation.</p>
    InsufficientPermissions(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl DisableSsoError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisableSsoError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AuthenticationFailedException" => {
                    return RusotoError::Service(DisableSsoError::AuthenticationFailed(err.msg))
                }
                "ClientException" => return RusotoError::Service(DisableSsoError::Client(err.msg)),
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(DisableSsoError::EntityDoesNotExist(err.msg))
                }
                "InsufficientPermissionsException" => {
                    return RusotoError::Service(DisableSsoError::InsufficientPermissions(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DisableSsoError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DisableSsoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisableSsoError {
    fn description(&self) -> &str {
        match *self {
            DisableSsoError::AuthenticationFailed(ref cause) => cause,
            DisableSsoError::Client(ref cause) => cause,
            DisableSsoError::EntityDoesNotExist(ref cause) => cause,
            DisableSsoError::InsufficientPermissions(ref cause) => cause,
            DisableSsoError::Service(ref cause) => cause,
        }
    }
}
/// Errors returned by EnableRadius
#[derive(Debug, PartialEq)]
pub enum EnableRadiusError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity already exists.</p>
    EntityAlreadyExists(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl EnableRadiusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<EnableRadiusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(EnableRadiusError::Client(err.msg))
                }
                "EntityAlreadyExistsException" => {
                    return RusotoError::Service(EnableRadiusError::EntityAlreadyExists(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(EnableRadiusError::EntityDoesNotExist(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(EnableRadiusError::InvalidParameter(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(EnableRadiusError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for EnableRadiusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for EnableRadiusError {
    fn description(&self) -> &str {
        match *self {
            EnableRadiusError::Client(ref cause) => cause,
            EnableRadiusError::EntityAlreadyExists(ref cause) => cause,
            EnableRadiusError::EntityDoesNotExist(ref cause) => cause,
            EnableRadiusError::InvalidParameter(ref cause) => cause,
            EnableRadiusError::Service(ref cause) => cause,
        }
    }
}
/// Errors returned by EnableSso
#[derive(Debug, PartialEq)]
pub enum EnableSsoError {
    /// <p>An authentication error occurred.</p>
    AuthenticationFailed(String),
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>The account does not have sufficient permission to perform the operation.</p>
    InsufficientPermissions(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl EnableSsoError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<EnableSsoError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AuthenticationFailedException" => {
                    return RusotoError::Service(EnableSsoError::AuthenticationFailed(err.msg))
                }
                "ClientException" => return RusotoError::Service(EnableSsoError::Client(err.msg)),
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(EnableSsoError::EntityDoesNotExist(err.msg))
                }
                "InsufficientPermissionsException" => {
                    return RusotoError::Service(EnableSsoError::InsufficientPermissions(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(EnableSsoError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for EnableSsoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for EnableSsoError {
    fn description(&self) -> &str {
        match *self {
            EnableSsoError::AuthenticationFailed(ref cause) => cause,
            EnableSsoError::Client(ref cause) => cause,
            EnableSsoError::EntityDoesNotExist(ref cause) => cause,
            EnableSsoError::InsufficientPermissions(ref cause) => cause,
            EnableSsoError::Service(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDirectoryLimits
#[derive(Debug, PartialEq)]
pub enum GetDirectoryLimitsError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl GetDirectoryLimitsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDirectoryLimitsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(GetDirectoryLimitsError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(GetDirectoryLimitsError::EntityDoesNotExist(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetDirectoryLimitsError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetDirectoryLimitsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDirectoryLimitsError {
    fn description(&self) -> &str {
        match *self {
            GetDirectoryLimitsError::Client(ref cause) => cause,
            GetDirectoryLimitsError::EntityDoesNotExist(ref cause) => cause,
            GetDirectoryLimitsError::Service(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSnapshotLimits
#[derive(Debug, PartialEq)]
pub enum GetSnapshotLimitsError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl GetSnapshotLimitsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSnapshotLimitsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(GetSnapshotLimitsError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(GetSnapshotLimitsError::EntityDoesNotExist(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetSnapshotLimitsError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetSnapshotLimitsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSnapshotLimitsError {
    fn description(&self) -> &str {
        match *self {
            GetSnapshotLimitsError::Client(ref cause) => cause,
            GetSnapshotLimitsError::EntityDoesNotExist(ref cause) => cause,
            GetSnapshotLimitsError::Service(ref cause) => cause,
        }
    }
}
/// Errors returned by ListIpRoutes
#[derive(Debug, PartialEq)]
pub enum ListIpRoutesError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>The <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl ListIpRoutesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListIpRoutesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(ListIpRoutesError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(ListIpRoutesError::EntityDoesNotExist(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListIpRoutesError::InvalidNextToken(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListIpRoutesError::InvalidParameter(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListIpRoutesError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListIpRoutesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListIpRoutesError {
    fn description(&self) -> &str {
        match *self {
            ListIpRoutesError::Client(ref cause) => cause,
            ListIpRoutesError::EntityDoesNotExist(ref cause) => cause,
            ListIpRoutesError::InvalidNextToken(ref cause) => cause,
            ListIpRoutesError::InvalidParameter(ref cause) => cause,
            ListIpRoutesError::Service(ref cause) => cause,
        }
    }
}
/// Errors returned by ListLogSubscriptions
#[derive(Debug, PartialEq)]
pub enum ListLogSubscriptionsError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>The <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl ListLogSubscriptionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListLogSubscriptionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(ListLogSubscriptionsError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(ListLogSubscriptionsError::EntityDoesNotExist(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListLogSubscriptionsError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListLogSubscriptionsError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListLogSubscriptionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListLogSubscriptionsError {
    fn description(&self) -> &str {
        match *self {
            ListLogSubscriptionsError::Client(ref cause) => cause,
            ListLogSubscriptionsError::EntityDoesNotExist(ref cause) => cause,
            ListLogSubscriptionsError::InvalidNextToken(ref cause) => cause,
            ListLogSubscriptionsError::Service(ref cause) => cause,
        }
    }
}
/// Errors returned by ListSchemaExtensions
#[derive(Debug, PartialEq)]
pub enum ListSchemaExtensionsError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>The <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl ListSchemaExtensionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListSchemaExtensionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(ListSchemaExtensionsError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(ListSchemaExtensionsError::EntityDoesNotExist(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListSchemaExtensionsError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListSchemaExtensionsError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListSchemaExtensionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListSchemaExtensionsError {
    fn description(&self) -> &str {
        match *self {
            ListSchemaExtensionsError::Client(ref cause) => cause,
            ListSchemaExtensionsError::EntityDoesNotExist(ref cause) => cause,
            ListSchemaExtensionsError::InvalidNextToken(ref cause) => cause,
            ListSchemaExtensionsError::Service(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>The <code>NextToken</code> value is not valid.</p>
    InvalidNextToken(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(ListTagsForResourceError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(ListTagsForResourceError::EntityDoesNotExist(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListTagsForResourceError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
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
            ListTagsForResourceError::Client(ref cause) => cause,
            ListTagsForResourceError::EntityDoesNotExist(ref cause) => cause,
            ListTagsForResourceError::InvalidNextToken(ref cause) => cause,
            ListTagsForResourceError::InvalidParameter(ref cause) => cause,
            ListTagsForResourceError::Service(ref cause) => cause,
        }
    }
}
/// Errors returned by RegisterEventTopic
#[derive(Debug, PartialEq)]
pub enum RegisterEventTopicError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl RegisterEventTopicError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RegisterEventTopicError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(RegisterEventTopicError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(RegisterEventTopicError::EntityDoesNotExist(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(RegisterEventTopicError::InvalidParameter(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(RegisterEventTopicError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RegisterEventTopicError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RegisterEventTopicError {
    fn description(&self) -> &str {
        match *self {
            RegisterEventTopicError::Client(ref cause) => cause,
            RegisterEventTopicError::EntityDoesNotExist(ref cause) => cause,
            RegisterEventTopicError::InvalidParameter(ref cause) => cause,
            RegisterEventTopicError::Service(ref cause) => cause,
        }
    }
}
/// Errors returned by RejectSharedDirectory
#[derive(Debug, PartialEq)]
pub enum RejectSharedDirectoryError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified directory has already been shared with this AWS account.</p>
    DirectoryAlreadyShared(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl RejectSharedDirectoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RejectSharedDirectoryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(RejectSharedDirectoryError::Client(err.msg))
                }
                "DirectoryAlreadySharedException" => {
                    return RusotoError::Service(
                        RejectSharedDirectoryError::DirectoryAlreadyShared(err.msg),
                    )
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(RejectSharedDirectoryError::EntityDoesNotExist(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(RejectSharedDirectoryError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(RejectSharedDirectoryError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RejectSharedDirectoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RejectSharedDirectoryError {
    fn description(&self) -> &str {
        match *self {
            RejectSharedDirectoryError::Client(ref cause) => cause,
            RejectSharedDirectoryError::DirectoryAlreadyShared(ref cause) => cause,
            RejectSharedDirectoryError::EntityDoesNotExist(ref cause) => cause,
            RejectSharedDirectoryError::InvalidParameter(ref cause) => cause,
            RejectSharedDirectoryError::Service(ref cause) => cause,
        }
    }
}
/// Errors returned by RemoveIpRoutes
#[derive(Debug, PartialEq)]
pub enum RemoveIpRoutesError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified directory is unavailable or could not be found.</p>
    DirectoryUnavailable(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl RemoveIpRoutesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RemoveIpRoutesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(RemoveIpRoutesError::Client(err.msg))
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(RemoveIpRoutesError::DirectoryUnavailable(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(RemoveIpRoutesError::EntityDoesNotExist(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(RemoveIpRoutesError::InvalidParameter(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(RemoveIpRoutesError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RemoveIpRoutesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RemoveIpRoutesError {
    fn description(&self) -> &str {
        match *self {
            RemoveIpRoutesError::Client(ref cause) => cause,
            RemoveIpRoutesError::DirectoryUnavailable(ref cause) => cause,
            RemoveIpRoutesError::EntityDoesNotExist(ref cause) => cause,
            RemoveIpRoutesError::InvalidParameter(ref cause) => cause,
            RemoveIpRoutesError::Service(ref cause) => cause,
        }
    }
}
/// Errors returned by RemoveTagsFromResource
#[derive(Debug, PartialEq)]
pub enum RemoveTagsFromResourceError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl RemoveTagsFromResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RemoveTagsFromResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(RemoveTagsFromResourceError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(RemoveTagsFromResourceError::EntityDoesNotExist(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(RemoveTagsFromResourceError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(RemoveTagsFromResourceError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
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
            RemoveTagsFromResourceError::Client(ref cause) => cause,
            RemoveTagsFromResourceError::EntityDoesNotExist(ref cause) => cause,
            RemoveTagsFromResourceError::InvalidParameter(ref cause) => cause,
            RemoveTagsFromResourceError::Service(ref cause) => cause,
        }
    }
}
/// Errors returned by ResetUserPassword
#[derive(Debug, PartialEq)]
pub enum ResetUserPasswordError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified directory is unavailable or could not be found.</p>
    DirectoryUnavailable(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>The new password provided by the user does not meet the password complexity requirements defined in your directory.</p>
    InvalidPassword(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
    /// <p>The user provided a username that does not exist in your directory.</p>
    UserDoesNotExist(String),
}

impl ResetUserPasswordError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ResetUserPasswordError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(ResetUserPasswordError::Client(err.msg))
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(ResetUserPasswordError::DirectoryUnavailable(
                        err.msg,
                    ))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(ResetUserPasswordError::EntityDoesNotExist(
                        err.msg,
                    ))
                }
                "InvalidPasswordException" => {
                    return RusotoError::Service(ResetUserPasswordError::InvalidPassword(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(ResetUserPasswordError::Service(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(ResetUserPasswordError::UnsupportedOperation(
                        err.msg,
                    ))
                }
                "UserDoesNotExistException" => {
                    return RusotoError::Service(ResetUserPasswordError::UserDoesNotExist(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ResetUserPasswordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ResetUserPasswordError {
    fn description(&self) -> &str {
        match *self {
            ResetUserPasswordError::Client(ref cause) => cause,
            ResetUserPasswordError::DirectoryUnavailable(ref cause) => cause,
            ResetUserPasswordError::EntityDoesNotExist(ref cause) => cause,
            ResetUserPasswordError::InvalidPassword(ref cause) => cause,
            ResetUserPasswordError::Service(ref cause) => cause,
            ResetUserPasswordError::UnsupportedOperation(ref cause) => cause,
            ResetUserPasswordError::UserDoesNotExist(ref cause) => cause,
        }
    }
}
/// Errors returned by RestoreFromSnapshot
#[derive(Debug, PartialEq)]
pub enum RestoreFromSnapshotError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl RestoreFromSnapshotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RestoreFromSnapshotError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(RestoreFromSnapshotError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(RestoreFromSnapshotError::EntityDoesNotExist(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(RestoreFromSnapshotError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(RestoreFromSnapshotError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RestoreFromSnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RestoreFromSnapshotError {
    fn description(&self) -> &str {
        match *self {
            RestoreFromSnapshotError::Client(ref cause) => cause,
            RestoreFromSnapshotError::EntityDoesNotExist(ref cause) => cause,
            RestoreFromSnapshotError::InvalidParameter(ref cause) => cause,
            RestoreFromSnapshotError::Service(ref cause) => cause,
        }
    }
}
/// Errors returned by ShareDirectory
#[derive(Debug, PartialEq)]
pub enum ShareDirectoryError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified directory has already been shared with this AWS account.</p>
    DirectoryAlreadyShared(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The specified shared target is not valid.</p>
    InvalidTarget(String),
    /// <p>Exception encountered while trying to access your AWS organization.</p>
    Organizations(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The maximum number of AWS accounts that you can share with this directory has been reached.</p>
    ShareLimitExceeded(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
}

impl ShareDirectoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ShareDirectoryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ShareDirectoryError::AccessDenied(err.msg))
                }
                "ClientException" => {
                    return RusotoError::Service(ShareDirectoryError::Client(err.msg))
                }
                "DirectoryAlreadySharedException" => {
                    return RusotoError::Service(ShareDirectoryError::DirectoryAlreadyShared(
                        err.msg,
                    ))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(ShareDirectoryError::EntityDoesNotExist(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ShareDirectoryError::InvalidParameter(err.msg))
                }
                "InvalidTargetException" => {
                    return RusotoError::Service(ShareDirectoryError::InvalidTarget(err.msg))
                }
                "OrganizationsException" => {
                    return RusotoError::Service(ShareDirectoryError::Organizations(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(ShareDirectoryError::Service(err.msg))
                }
                "ShareLimitExceededException" => {
                    return RusotoError::Service(ShareDirectoryError::ShareLimitExceeded(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(ShareDirectoryError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ShareDirectoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ShareDirectoryError {
    fn description(&self) -> &str {
        match *self {
            ShareDirectoryError::AccessDenied(ref cause) => cause,
            ShareDirectoryError::Client(ref cause) => cause,
            ShareDirectoryError::DirectoryAlreadyShared(ref cause) => cause,
            ShareDirectoryError::EntityDoesNotExist(ref cause) => cause,
            ShareDirectoryError::InvalidParameter(ref cause) => cause,
            ShareDirectoryError::InvalidTarget(ref cause) => cause,
            ShareDirectoryError::Organizations(ref cause) => cause,
            ShareDirectoryError::Service(ref cause) => cause,
            ShareDirectoryError::ShareLimitExceeded(ref cause) => cause,
            ShareDirectoryError::UnsupportedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by StartSchemaExtension
#[derive(Debug, PartialEq)]
pub enum StartSchemaExtensionError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified directory is unavailable or could not be found.</p>
    DirectoryUnavailable(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The maximum number of manual snapshots for the directory has been reached. You can use the <a>GetSnapshotLimits</a> operation to determine the snapshot limits for a directory.</p>
    SnapshotLimitExceeded(String),
}

impl StartSchemaExtensionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartSchemaExtensionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(StartSchemaExtensionError::Client(err.msg))
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(StartSchemaExtensionError::DirectoryUnavailable(
                        err.msg,
                    ))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(StartSchemaExtensionError::EntityDoesNotExist(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StartSchemaExtensionError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(StartSchemaExtensionError::Service(err.msg))
                }
                "SnapshotLimitExceededException" => {
                    return RusotoError::Service(StartSchemaExtensionError::SnapshotLimitExceeded(
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
impl fmt::Display for StartSchemaExtensionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartSchemaExtensionError {
    fn description(&self) -> &str {
        match *self {
            StartSchemaExtensionError::Client(ref cause) => cause,
            StartSchemaExtensionError::DirectoryUnavailable(ref cause) => cause,
            StartSchemaExtensionError::EntityDoesNotExist(ref cause) => cause,
            StartSchemaExtensionError::InvalidParameter(ref cause) => cause,
            StartSchemaExtensionError::Service(ref cause) => cause,
            StartSchemaExtensionError::SnapshotLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by UnshareDirectory
#[derive(Debug, PartialEq)]
pub enum UnshareDirectoryError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified directory has not been shared with this AWS account.</p>
    DirectoryNotShared(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>The specified shared target is not valid.</p>
    InvalidTarget(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl UnshareDirectoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UnshareDirectoryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(UnshareDirectoryError::Client(err.msg))
                }
                "DirectoryNotSharedException" => {
                    return RusotoError::Service(UnshareDirectoryError::DirectoryNotShared(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(UnshareDirectoryError::EntityDoesNotExist(err.msg))
                }
                "InvalidTargetException" => {
                    return RusotoError::Service(UnshareDirectoryError::InvalidTarget(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(UnshareDirectoryError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UnshareDirectoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UnshareDirectoryError {
    fn description(&self) -> &str {
        match *self {
            UnshareDirectoryError::Client(ref cause) => cause,
            UnshareDirectoryError::DirectoryNotShared(ref cause) => cause,
            UnshareDirectoryError::EntityDoesNotExist(ref cause) => cause,
            UnshareDirectoryError::InvalidTarget(ref cause) => cause,
            UnshareDirectoryError::Service(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateConditionalForwarder
#[derive(Debug, PartialEq)]
pub enum UpdateConditionalForwarderError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified directory is unavailable or could not be found.</p>
    DirectoryUnavailable(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
}

impl UpdateConditionalForwarderError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateConditionalForwarderError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(UpdateConditionalForwarderError::Client(err.msg))
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(
                        UpdateConditionalForwarderError::DirectoryUnavailable(err.msg),
                    )
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(
                        UpdateConditionalForwarderError::EntityDoesNotExist(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateConditionalForwarderError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(UpdateConditionalForwarderError::Service(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(
                        UpdateConditionalForwarderError::UnsupportedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateConditionalForwarderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateConditionalForwarderError {
    fn description(&self) -> &str {
        match *self {
            UpdateConditionalForwarderError::Client(ref cause) => cause,
            UpdateConditionalForwarderError::DirectoryUnavailable(ref cause) => cause,
            UpdateConditionalForwarderError::EntityDoesNotExist(ref cause) => cause,
            UpdateConditionalForwarderError::InvalidParameter(ref cause) => cause,
            UpdateConditionalForwarderError::Service(ref cause) => cause,
            UpdateConditionalForwarderError::UnsupportedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateNumberOfDomainControllers
#[derive(Debug, PartialEq)]
pub enum UpdateNumberOfDomainControllersError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified directory is unavailable or could not be found.</p>
    DirectoryUnavailable(String),
    /// <p>The maximum allowed number of domain controllers per directory was exceeded. The default limit per directory is 20 domain controllers.</p>
    DomainControllerLimitExceeded(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
}

impl UpdateNumberOfDomainControllersError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateNumberOfDomainControllersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(UpdateNumberOfDomainControllersError::Client(
                        err.msg,
                    ))
                }
                "DirectoryUnavailableException" => {
                    return RusotoError::Service(
                        UpdateNumberOfDomainControllersError::DirectoryUnavailable(err.msg),
                    )
                }
                "DomainControllerLimitExceededException" => {
                    return RusotoError::Service(
                        UpdateNumberOfDomainControllersError::DomainControllerLimitExceeded(
                            err.msg,
                        ),
                    )
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(
                        UpdateNumberOfDomainControllersError::EntityDoesNotExist(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        UpdateNumberOfDomainControllersError::InvalidParameter(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(UpdateNumberOfDomainControllersError::Service(
                        err.msg,
                    ))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(
                        UpdateNumberOfDomainControllersError::UnsupportedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateNumberOfDomainControllersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateNumberOfDomainControllersError {
    fn description(&self) -> &str {
        match *self {
            UpdateNumberOfDomainControllersError::Client(ref cause) => cause,
            UpdateNumberOfDomainControllersError::DirectoryUnavailable(ref cause) => cause,
            UpdateNumberOfDomainControllersError::DomainControllerLimitExceeded(ref cause) => cause,
            UpdateNumberOfDomainControllersError::EntityDoesNotExist(ref cause) => cause,
            UpdateNumberOfDomainControllersError::InvalidParameter(ref cause) => cause,
            UpdateNumberOfDomainControllersError::Service(ref cause) => cause,
            UpdateNumberOfDomainControllersError::UnsupportedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateRadius
#[derive(Debug, PartialEq)]
pub enum UpdateRadiusError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl UpdateRadiusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateRadiusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(UpdateRadiusError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(UpdateRadiusError::EntityDoesNotExist(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateRadiusError::InvalidParameter(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(UpdateRadiusError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateRadiusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateRadiusError {
    fn description(&self) -> &str {
        match *self {
            UpdateRadiusError::Client(ref cause) => cause,
            UpdateRadiusError::EntityDoesNotExist(ref cause) => cause,
            UpdateRadiusError::InvalidParameter(ref cause) => cause,
            UpdateRadiusError::Service(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateTrust
#[derive(Debug, PartialEq)]
pub enum UpdateTrustError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
}

impl UpdateTrustError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateTrustError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(UpdateTrustError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(UpdateTrustError::EntityDoesNotExist(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateTrustError::InvalidParameter(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(UpdateTrustError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateTrustError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateTrustError {
    fn description(&self) -> &str {
        match *self {
            UpdateTrustError::Client(ref cause) => cause,
            UpdateTrustError::EntityDoesNotExist(ref cause) => cause,
            UpdateTrustError::InvalidParameter(ref cause) => cause,
            UpdateTrustError::Service(ref cause) => cause,
        }
    }
}
/// Errors returned by VerifyTrust
#[derive(Debug, PartialEq)]
pub enum VerifyTrustError {
    /// <p>A client exception has occurred.</p>
    Client(String),
    /// <p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// <p>The operation is not supported.</p>
    UnsupportedOperation(String),
}

impl VerifyTrustError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<VerifyTrustError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(VerifyTrustError::Client(err.msg))
                }
                "EntityDoesNotExistException" => {
                    return RusotoError::Service(VerifyTrustError::EntityDoesNotExist(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(VerifyTrustError::InvalidParameter(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(VerifyTrustError::Service(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(VerifyTrustError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for VerifyTrustError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for VerifyTrustError {
    fn description(&self) -> &str {
        match *self {
            VerifyTrustError::Client(ref cause) => cause,
            VerifyTrustError::EntityDoesNotExist(ref cause) => cause,
            VerifyTrustError::InvalidParameter(ref cause) => cause,
            VerifyTrustError::Service(ref cause) => cause,
            VerifyTrustError::UnsupportedOperation(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Directory Service API. Directory Service clients implement this trait.
pub trait DirectoryService {
    /// <p>Accepts a directory sharing request that was sent from the directory owner account.</p>
    fn accept_shared_directory(
        &self,
        input: AcceptSharedDirectoryRequest,
    ) -> RusotoFuture<AcceptSharedDirectoryResult, AcceptSharedDirectoryError>;

    /// <p>If the DNS server for your on-premises domain uses a publicly addressable IP address, you must add a CIDR address block to correctly route traffic to and from your Microsoft AD on Amazon Web Services. <i>AddIpRoutes</i> adds this address block. You can also use <i>AddIpRoutes</i> to facilitate routing traffic that uses public IP ranges from your Microsoft AD on AWS to a peer VPC. </p> <p>Before you call <i>AddIpRoutes</i>, ensure that all of the required permissions have been explicitly granted through a policy. For details about what permissions are required to run the <i>AddIpRoutes</i> operation, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/UsingWithDS_IAM_ResourcePermissions.html">AWS Directory Service API Permissions: Actions, Resources, and Conditions Reference</a>.</p>
    fn add_ip_routes(
        &self,
        input: AddIpRoutesRequest,
    ) -> RusotoFuture<AddIpRoutesResult, AddIpRoutesError>;

    /// <p>Adds or overwrites one or more tags for the specified directory. Each directory can have a maximum of 50 tags. Each tag consists of a key and optional value. Tag keys must be unique to each resource.</p>
    fn add_tags_to_resource(
        &self,
        input: AddTagsToResourceRequest,
    ) -> RusotoFuture<AddTagsToResourceResult, AddTagsToResourceError>;

    /// <p>Cancels an in-progress schema extension to a Microsoft AD directory. Once a schema extension has started replicating to all domain controllers, the task can no longer be canceled. A schema extension can be canceled during any of the following states; <code>Initializing</code>, <code>CreatingSnapshot</code>, and <code>UpdatingSchema</code>.</p>
    fn cancel_schema_extension(
        &self,
        input: CancelSchemaExtensionRequest,
    ) -> RusotoFuture<CancelSchemaExtensionResult, CancelSchemaExtensionError>;

    /// <p>Creates an AD Connector to connect to an on-premises directory.</p> <p>Before you call <code>ConnectDirectory</code>, ensure that all of the required permissions have been explicitly granted through a policy. For details about what permissions are required to run the <code>ConnectDirectory</code> operation, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/UsingWithDS_IAM_ResourcePermissions.html">AWS Directory Service API Permissions: Actions, Resources, and Conditions Reference</a>.</p>
    fn connect_directory(
        &self,
        input: ConnectDirectoryRequest,
    ) -> RusotoFuture<ConnectDirectoryResult, ConnectDirectoryError>;

    /// <p><p>Creates an alias for a directory and assigns the alias to the directory. The alias is used to construct the access URL for the directory, such as <code>http://&lt;alias&gt;.awsapps.com</code>.</p> <important> <p>After an alias has been created, it cannot be deleted or reused, so this operation should only be used when absolutely necessary.</p> </important></p>
    fn create_alias(
        &self,
        input: CreateAliasRequest,
    ) -> RusotoFuture<CreateAliasResult, CreateAliasError>;

    /// <p>Creates a computer account in the specified directory, and joins the computer to the directory.</p>
    fn create_computer(
        &self,
        input: CreateComputerRequest,
    ) -> RusotoFuture<CreateComputerResult, CreateComputerError>;

    /// <p>Creates a conditional forwarder associated with your AWS directory. Conditional forwarders are required in order to set up a trust relationship with another domain. The conditional forwarder points to the trusted domain.</p>
    fn create_conditional_forwarder(
        &self,
        input: CreateConditionalForwarderRequest,
    ) -> RusotoFuture<CreateConditionalForwarderResult, CreateConditionalForwarderError>;

    /// <p>Creates a Simple AD directory.</p> <p>Before you call <code>CreateDirectory</code>, ensure that all of the required permissions have been explicitly granted through a policy. For details about what permissions are required to run the <code>CreateDirectory</code> operation, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/UsingWithDS_IAM_ResourcePermissions.html">AWS Directory Service API Permissions: Actions, Resources, and Conditions Reference</a>.</p>
    fn create_directory(
        &self,
        input: CreateDirectoryRequest,
    ) -> RusotoFuture<CreateDirectoryResult, CreateDirectoryError>;

    /// <p>Creates a subscription to forward real time Directory Service domain controller security logs to the specified CloudWatch log group in your AWS account.</p>
    fn create_log_subscription(
        &self,
        input: CreateLogSubscriptionRequest,
    ) -> RusotoFuture<CreateLogSubscriptionResult, CreateLogSubscriptionError>;

    /// <p>Creates an AWS Managed Microsoft AD directory.</p> <p>Before you call <i>CreateMicrosoftAD</i>, ensure that all of the required permissions have been explicitly granted through a policy. For details about what permissions are required to run the <i>CreateMicrosoftAD</i> operation, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/UsingWithDS_IAM_ResourcePermissions.html">AWS Directory Service API Permissions: Actions, Resources, and Conditions Reference</a>.</p>
    fn create_microsoft_ad(
        &self,
        input: CreateMicrosoftADRequest,
    ) -> RusotoFuture<CreateMicrosoftADResult, CreateMicrosoftADError>;

    /// <p><p>Creates a snapshot of a Simple AD or Microsoft AD directory in the AWS cloud.</p> <note> <p>You cannot take snapshots of AD Connector directories.</p> </note></p>
    fn create_snapshot(
        &self,
        input: CreateSnapshotRequest,
    ) -> RusotoFuture<CreateSnapshotResult, CreateSnapshotError>;

    /// <p>AWS Directory Service for Microsoft Active Directory allows you to configure trust relationships. For example, you can establish a trust between your AWS Managed Microsoft AD directory, and your existing on-premises Microsoft Active Directory. This would allow you to provide users and groups access to resources in either domain, with a single set of credentials.</p> <p>This action initiates the creation of the AWS side of a trust relationship between an AWS Managed Microsoft AD directory and an external domain. You can create either a forest trust or an external trust.</p>
    fn create_trust(
        &self,
        input: CreateTrustRequest,
    ) -> RusotoFuture<CreateTrustResult, CreateTrustError>;

    /// <p>Deletes a conditional forwarder that has been set up for your AWS directory.</p>
    fn delete_conditional_forwarder(
        &self,
        input: DeleteConditionalForwarderRequest,
    ) -> RusotoFuture<DeleteConditionalForwarderResult, DeleteConditionalForwarderError>;

    /// <p>Deletes an AWS Directory Service directory.</p> <p>Before you call <code>DeleteDirectory</code>, ensure that all of the required permissions have been explicitly granted through a policy. For details about what permissions are required to run the <code>DeleteDirectory</code> operation, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/UsingWithDS_IAM_ResourcePermissions.html">AWS Directory Service API Permissions: Actions, Resources, and Conditions Reference</a>.</p>
    fn delete_directory(
        &self,
        input: DeleteDirectoryRequest,
    ) -> RusotoFuture<DeleteDirectoryResult, DeleteDirectoryError>;

    /// <p>Deletes the specified log subscription.</p>
    fn delete_log_subscription(
        &self,
        input: DeleteLogSubscriptionRequest,
    ) -> RusotoFuture<DeleteLogSubscriptionResult, DeleteLogSubscriptionError>;

    /// <p>Deletes a directory snapshot.</p>
    fn delete_snapshot(
        &self,
        input: DeleteSnapshotRequest,
    ) -> RusotoFuture<DeleteSnapshotResult, DeleteSnapshotError>;

    /// <p>Deletes an existing trust relationship between your AWS Managed Microsoft AD directory and an external domain.</p>
    fn delete_trust(
        &self,
        input: DeleteTrustRequest,
    ) -> RusotoFuture<DeleteTrustResult, DeleteTrustError>;

    /// <p>Removes the specified directory as a publisher to the specified SNS topic.</p>
    fn deregister_event_topic(
        &self,
        input: DeregisterEventTopicRequest,
    ) -> RusotoFuture<DeregisterEventTopicResult, DeregisterEventTopicError>;

    /// <p>Obtains information about the conditional forwarders for this account.</p> <p>If no input parameters are provided for RemoteDomainNames, this request describes all conditional forwarders for the specified directory ID.</p>
    fn describe_conditional_forwarders(
        &self,
        input: DescribeConditionalForwardersRequest,
    ) -> RusotoFuture<DescribeConditionalForwardersResult, DescribeConditionalForwardersError>;

    /// <p>Obtains information about the directories that belong to this account.</p> <p>You can retrieve information about specific directories by passing the directory identifiers in the <code>DirectoryIds</code> parameter. Otherwise, all directories that belong to the current account are returned.</p> <p>This operation supports pagination with the use of the <code>NextToken</code> request and response parameters. If more results are available, the <code>DescribeDirectoriesResult.NextToken</code> member contains a token that you pass in the next call to <a>DescribeDirectories</a> to retrieve the next set of items.</p> <p>You can also specify a maximum number of return results with the <code>Limit</code> parameter.</p>
    fn describe_directories(
        &self,
        input: DescribeDirectoriesRequest,
    ) -> RusotoFuture<DescribeDirectoriesResult, DescribeDirectoriesError>;

    /// <p>Provides information about any domain controllers in your directory.</p>
    fn describe_domain_controllers(
        &self,
        input: DescribeDomainControllersRequest,
    ) -> RusotoFuture<DescribeDomainControllersResult, DescribeDomainControllersError>;

    /// <p>Obtains information about which SNS topics receive status messages from the specified directory.</p> <p>If no input parameters are provided, such as DirectoryId or TopicName, this request describes all of the associations in the account.</p>
    fn describe_event_topics(
        &self,
        input: DescribeEventTopicsRequest,
    ) -> RusotoFuture<DescribeEventTopicsResult, DescribeEventTopicsError>;

    /// <p>Returns the shared directories in your account. </p>
    fn describe_shared_directories(
        &self,
        input: DescribeSharedDirectoriesRequest,
    ) -> RusotoFuture<DescribeSharedDirectoriesResult, DescribeSharedDirectoriesError>;

    /// <p>Obtains information about the directory snapshots that belong to this account.</p> <p>This operation supports pagination with the use of the <i>NextToken</i> request and response parameters. If more results are available, the <i>DescribeSnapshots.NextToken</i> member contains a token that you pass in the next call to <a>DescribeSnapshots</a> to retrieve the next set of items.</p> <p>You can also specify a maximum number of return results with the <i>Limit</i> parameter.</p>
    fn describe_snapshots(
        &self,
        input: DescribeSnapshotsRequest,
    ) -> RusotoFuture<DescribeSnapshotsResult, DescribeSnapshotsError>;

    /// <p>Obtains information about the trust relationships for this account.</p> <p>If no input parameters are provided, such as DirectoryId or TrustIds, this request describes all the trust relationships belonging to the account.</p>
    fn describe_trusts(
        &self,
        input: DescribeTrustsRequest,
    ) -> RusotoFuture<DescribeTrustsResult, DescribeTrustsError>;

    /// <p>Disables multi-factor authentication (MFA) with the Remote Authentication Dial In User Service (RADIUS) server for an AD Connector or Microsoft AD directory.</p>
    fn disable_radius(
        &self,
        input: DisableRadiusRequest,
    ) -> RusotoFuture<DisableRadiusResult, DisableRadiusError>;

    /// <p>Disables single-sign on for a directory.</p>
    fn disable_sso(
        &self,
        input: DisableSsoRequest,
    ) -> RusotoFuture<DisableSsoResult, DisableSsoError>;

    /// <p>Enables multi-factor authentication (MFA) with the Remote Authentication Dial In User Service (RADIUS) server for an AD Connector or Microsoft AD directory.</p>
    fn enable_radius(
        &self,
        input: EnableRadiusRequest,
    ) -> RusotoFuture<EnableRadiusResult, EnableRadiusError>;

    /// <p>Enables single sign-on for a directory.</p>
    fn enable_sso(&self, input: EnableSsoRequest) -> RusotoFuture<EnableSsoResult, EnableSsoError>;

    /// <p>Obtains directory limit information for the current region.</p>
    fn get_directory_limits(
        &self,
    ) -> RusotoFuture<GetDirectoryLimitsResult, GetDirectoryLimitsError>;

    /// <p>Obtains the manual snapshot limits for a directory.</p>
    fn get_snapshot_limits(
        &self,
        input: GetSnapshotLimitsRequest,
    ) -> RusotoFuture<GetSnapshotLimitsResult, GetSnapshotLimitsError>;

    /// <p>Lists the address blocks that you have added to a directory.</p>
    fn list_ip_routes(
        &self,
        input: ListIpRoutesRequest,
    ) -> RusotoFuture<ListIpRoutesResult, ListIpRoutesError>;

    /// <p>Lists the active log subscriptions for the AWS account.</p>
    fn list_log_subscriptions(
        &self,
        input: ListLogSubscriptionsRequest,
    ) -> RusotoFuture<ListLogSubscriptionsResult, ListLogSubscriptionsError>;

    /// <p>Lists all schema extensions applied to a Microsoft AD Directory.</p>
    fn list_schema_extensions(
        &self,
        input: ListSchemaExtensionsRequest,
    ) -> RusotoFuture<ListSchemaExtensionsResult, ListSchemaExtensionsError>;

    /// <p>Lists all tags on a directory.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> RusotoFuture<ListTagsForResourceResult, ListTagsForResourceError>;

    /// <p>Associates a directory with an SNS topic. This establishes the directory as a publisher to the specified SNS topic. You can then receive email or text (SMS) messages when the status of your directory changes. You get notified if your directory goes from an Active status to an Impaired or Inoperable status. You also receive a notification when the directory returns to an Active status.</p>
    fn register_event_topic(
        &self,
        input: RegisterEventTopicRequest,
    ) -> RusotoFuture<RegisterEventTopicResult, RegisterEventTopicError>;

    /// <p>Rejects a directory sharing request that was sent from the directory owner account.</p>
    fn reject_shared_directory(
        &self,
        input: RejectSharedDirectoryRequest,
    ) -> RusotoFuture<RejectSharedDirectoryResult, RejectSharedDirectoryError>;

    /// <p>Removes IP address blocks from a directory.</p>
    fn remove_ip_routes(
        &self,
        input: RemoveIpRoutesRequest,
    ) -> RusotoFuture<RemoveIpRoutesResult, RemoveIpRoutesError>;

    /// <p>Removes tags from a directory.</p>
    fn remove_tags_from_resource(
        &self,
        input: RemoveTagsFromResourceRequest,
    ) -> RusotoFuture<RemoveTagsFromResourceResult, RemoveTagsFromResourceError>;

    /// <p>Resets the password for any user in your AWS Managed Microsoft AD or Simple AD directory.</p>
    fn reset_user_password(
        &self,
        input: ResetUserPasswordRequest,
    ) -> RusotoFuture<ResetUserPasswordResult, ResetUserPasswordError>;

    /// <p>Restores a directory using an existing directory snapshot.</p> <p>When you restore a directory from a snapshot, any changes made to the directory after the snapshot date are overwritten.</p> <p>This action returns as soon as the restore operation is initiated. You can monitor the progress of the restore operation by calling the <a>DescribeDirectories</a> operation with the directory identifier. When the <b>DirectoryDescription.Stage</b> value changes to <code>Active</code>, the restore operation is complete.</p>
    fn restore_from_snapshot(
        &self,
        input: RestoreFromSnapshotRequest,
    ) -> RusotoFuture<RestoreFromSnapshotResult, RestoreFromSnapshotError>;

    /// <p>Shares a specified directory (<code>DirectoryId</code>) in your AWS account (directory owner) with another AWS account (directory consumer). With this operation you can use your directory from any AWS account and from any Amazon VPC within an AWS Region.</p> <p>When you share your AWS Managed Microsoft AD directory, AWS Directory Service creates a shared directory in the directory consumer account. This shared directory contains the metadata to provide access to the directory within the directory owner account. The shared directory is visible in all VPCs in the directory consumer account.</p> <p>The <code>ShareMethod</code> parameter determines whether the specified directory can be shared between AWS accounts inside the same AWS organization (<code>ORGANIZATIONS</code>). It also determines whether you can share the directory with any other AWS account either inside or outside of the organization (<code>HANDSHAKE</code>).</p> <p>The <code>ShareNotes</code> parameter is only used when <code>HANDSHAKE</code> is called, which sends a directory sharing request to the directory consumer. </p>
    fn share_directory(
        &self,
        input: ShareDirectoryRequest,
    ) -> RusotoFuture<ShareDirectoryResult, ShareDirectoryError>;

    /// <p>Applies a schema extension to a Microsoft AD directory.</p>
    fn start_schema_extension(
        &self,
        input: StartSchemaExtensionRequest,
    ) -> RusotoFuture<StartSchemaExtensionResult, StartSchemaExtensionError>;

    /// <p>Stops the directory sharing between the directory owner and consumer accounts. </p>
    fn unshare_directory(
        &self,
        input: UnshareDirectoryRequest,
    ) -> RusotoFuture<UnshareDirectoryResult, UnshareDirectoryError>;

    /// <p>Updates a conditional forwarder that has been set up for your AWS directory.</p>
    fn update_conditional_forwarder(
        &self,
        input: UpdateConditionalForwarderRequest,
    ) -> RusotoFuture<UpdateConditionalForwarderResult, UpdateConditionalForwarderError>;

    /// <p>Adds or removes domain controllers to or from the directory. Based on the difference between current value and new value (provided through this API call), domain controllers will be added or removed. It may take up to 45 minutes for any new domain controllers to become fully active once the requested number of domain controllers is updated. During this time, you cannot make another update request.</p>
    fn update_number_of_domain_controllers(
        &self,
        input: UpdateNumberOfDomainControllersRequest,
    ) -> RusotoFuture<UpdateNumberOfDomainControllersResult, UpdateNumberOfDomainControllersError>;

    /// <p>Updates the Remote Authentication Dial In User Service (RADIUS) server information for an AD Connector or Microsoft AD directory.</p>
    fn update_radius(
        &self,
        input: UpdateRadiusRequest,
    ) -> RusotoFuture<UpdateRadiusResult, UpdateRadiusError>;

    /// <p>Updates the trust that has been set up between your AWS Managed Microsoft AD directory and an on-premises Active Directory.</p>
    fn update_trust(
        &self,
        input: UpdateTrustRequest,
    ) -> RusotoFuture<UpdateTrustResult, UpdateTrustError>;

    /// <p>AWS Directory Service for Microsoft Active Directory allows you to configure and verify trust relationships.</p> <p>This action verifies a trust relationship between your AWS Managed Microsoft AD directory and an external domain.</p>
    fn verify_trust(
        &self,
        input: VerifyTrustRequest,
    ) -> RusotoFuture<VerifyTrustResult, VerifyTrustError>;
}
/// A client for the Directory Service API.
#[derive(Clone)]
pub struct DirectoryServiceClient {
    client: Client,
    region: region::Region,
}

impl DirectoryServiceClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> DirectoryServiceClient {
        Self::new_with_client(Client::shared(), region)
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> DirectoryServiceClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        Self::new_with_client(
            Client::new_with(credentials_provider, request_dispatcher),
            region,
        )
    }

    pub fn new_with_client(client: Client, region: region::Region) -> DirectoryServiceClient {
        DirectoryServiceClient { client, region }
    }
}

impl DirectoryService for DirectoryServiceClient {
    /// <p>Accepts a directory sharing request that was sent from the directory owner account.</p>
    fn accept_shared_directory(
        &self,
        input: AcceptSharedDirectoryRequest,
    ) -> RusotoFuture<AcceptSharedDirectoryResult, AcceptSharedDirectoryError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.AcceptSharedDirectory",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<AcceptSharedDirectoryResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(AcceptSharedDirectoryError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>If the DNS server for your on-premises domain uses a publicly addressable IP address, you must add a CIDR address block to correctly route traffic to and from your Microsoft AD on Amazon Web Services. <i>AddIpRoutes</i> adds this address block. You can also use <i>AddIpRoutes</i> to facilitate routing traffic that uses public IP ranges from your Microsoft AD on AWS to a peer VPC. </p> <p>Before you call <i>AddIpRoutes</i>, ensure that all of the required permissions have been explicitly granted through a policy. For details about what permissions are required to run the <i>AddIpRoutes</i> operation, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/UsingWithDS_IAM_ResourcePermissions.html">AWS Directory Service API Permissions: Actions, Resources, and Conditions Reference</a>.</p>
    fn add_ip_routes(
        &self,
        input: AddIpRoutesRequest,
    ) -> RusotoFuture<AddIpRoutesResult, AddIpRoutesError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DirectoryService_20150416.AddIpRoutes");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<AddIpRoutesResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(AddIpRoutesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Adds or overwrites one or more tags for the specified directory. Each directory can have a maximum of 50 tags. Each tag consists of a key and optional value. Tag keys must be unique to each resource.</p>
    fn add_tags_to_resource(
        &self,
        input: AddTagsToResourceRequest,
    ) -> RusotoFuture<AddTagsToResourceResult, AddTagsToResourceError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.AddTagsToResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<AddTagsToResourceResult, _>()
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

    /// <p>Cancels an in-progress schema extension to a Microsoft AD directory. Once a schema extension has started replicating to all domain controllers, the task can no longer be canceled. A schema extension can be canceled during any of the following states; <code>Initializing</code>, <code>CreatingSnapshot</code>, and <code>UpdatingSchema</code>.</p>
    fn cancel_schema_extension(
        &self,
        input: CancelSchemaExtensionRequest,
    ) -> RusotoFuture<CancelSchemaExtensionResult, CancelSchemaExtensionError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.CancelSchemaExtension",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CancelSchemaExtensionResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CancelSchemaExtensionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Creates an AD Connector to connect to an on-premises directory.</p> <p>Before you call <code>ConnectDirectory</code>, ensure that all of the required permissions have been explicitly granted through a policy. For details about what permissions are required to run the <code>ConnectDirectory</code> operation, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/UsingWithDS_IAM_ResourcePermissions.html">AWS Directory Service API Permissions: Actions, Resources, and Conditions Reference</a>.</p>
    fn connect_directory(
        &self,
        input: ConnectDirectoryRequest,
    ) -> RusotoFuture<ConnectDirectoryResult, ConnectDirectoryError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DirectoryService_20150416.ConnectDirectory");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ConnectDirectoryResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ConnectDirectoryError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Creates an alias for a directory and assigns the alias to the directory. The alias is used to construct the access URL for the directory, such as <code>http://&lt;alias&gt;.awsapps.com</code>.</p> <important> <p>After an alias has been created, it cannot be deleted or reused, so this operation should only be used when absolutely necessary.</p> </important></p>
    fn create_alias(
        &self,
        input: CreateAliasRequest,
    ) -> RusotoFuture<CreateAliasResult, CreateAliasError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DirectoryService_20150416.CreateAlias");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateAliasResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateAliasError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a computer account in the specified directory, and joins the computer to the directory.</p>
    fn create_computer(
        &self,
        input: CreateComputerRequest,
    ) -> RusotoFuture<CreateComputerResult, CreateComputerError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DirectoryService_20150416.CreateComputer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateComputerResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateComputerError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a conditional forwarder associated with your AWS directory. Conditional forwarders are required in order to set up a trust relationship with another domain. The conditional forwarder points to the trusted domain.</p>
    fn create_conditional_forwarder(
        &self,
        input: CreateConditionalForwarderRequest,
    ) -> RusotoFuture<CreateConditionalForwarderResult, CreateConditionalForwarderError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.CreateConditionalForwarder",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateConditionalForwarderResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateConditionalForwarderError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates a Simple AD directory.</p> <p>Before you call <code>CreateDirectory</code>, ensure that all of the required permissions have been explicitly granted through a policy. For details about what permissions are required to run the <code>CreateDirectory</code> operation, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/UsingWithDS_IAM_ResourcePermissions.html">AWS Directory Service API Permissions: Actions, Resources, and Conditions Reference</a>.</p>
    fn create_directory(
        &self,
        input: CreateDirectoryRequest,
    ) -> RusotoFuture<CreateDirectoryResult, CreateDirectoryError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DirectoryService_20150416.CreateDirectory");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateDirectoryResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateDirectoryError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a subscription to forward real time Directory Service domain controller security logs to the specified CloudWatch log group in your AWS account.</p>
    fn create_log_subscription(
        &self,
        input: CreateLogSubscriptionRequest,
    ) -> RusotoFuture<CreateLogSubscriptionResult, CreateLogSubscriptionError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.CreateLogSubscription",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateLogSubscriptionResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateLogSubscriptionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Creates an AWS Managed Microsoft AD directory.</p> <p>Before you call <i>CreateMicrosoftAD</i>, ensure that all of the required permissions have been explicitly granted through a policy. For details about what permissions are required to run the <i>CreateMicrosoftAD</i> operation, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/UsingWithDS_IAM_ResourcePermissions.html">AWS Directory Service API Permissions: Actions, Resources, and Conditions Reference</a>.</p>
    fn create_microsoft_ad(
        &self,
        input: CreateMicrosoftADRequest,
    ) -> RusotoFuture<CreateMicrosoftADResult, CreateMicrosoftADError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.CreateMicrosoftAD",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateMicrosoftADResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateMicrosoftADError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Creates a snapshot of a Simple AD or Microsoft AD directory in the AWS cloud.</p> <note> <p>You cannot take snapshots of AD Connector directories.</p> </note></p>
    fn create_snapshot(
        &self,
        input: CreateSnapshotRequest,
    ) -> RusotoFuture<CreateSnapshotResult, CreateSnapshotError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DirectoryService_20150416.CreateSnapshot");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateSnapshotResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateSnapshotError::from_response(response))),
                )
            }
        })
    }

    /// <p>AWS Directory Service for Microsoft Active Directory allows you to configure trust relationships. For example, you can establish a trust between your AWS Managed Microsoft AD directory, and your existing on-premises Microsoft Active Directory. This would allow you to provide users and groups access to resources in either domain, with a single set of credentials.</p> <p>This action initiates the creation of the AWS side of a trust relationship between an AWS Managed Microsoft AD directory and an external domain. You can create either a forest trust or an external trust.</p>
    fn create_trust(
        &self,
        input: CreateTrustRequest,
    ) -> RusotoFuture<CreateTrustResult, CreateTrustError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DirectoryService_20150416.CreateTrust");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateTrustResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateTrustError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a conditional forwarder that has been set up for your AWS directory.</p>
    fn delete_conditional_forwarder(
        &self,
        input: DeleteConditionalForwarderRequest,
    ) -> RusotoFuture<DeleteConditionalForwarderResult, DeleteConditionalForwarderError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.DeleteConditionalForwarder",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteConditionalForwarderResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteConditionalForwarderError::from_response(response))
                }))
            }
        })
    }

    /// <p>Deletes an AWS Directory Service directory.</p> <p>Before you call <code>DeleteDirectory</code>, ensure that all of the required permissions have been explicitly granted through a policy. For details about what permissions are required to run the <code>DeleteDirectory</code> operation, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/UsingWithDS_IAM_ResourcePermissions.html">AWS Directory Service API Permissions: Actions, Resources, and Conditions Reference</a>.</p>
    fn delete_directory(
        &self,
        input: DeleteDirectoryRequest,
    ) -> RusotoFuture<DeleteDirectoryResult, DeleteDirectoryError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DirectoryService_20150416.DeleteDirectory");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteDirectoryResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteDirectoryError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the specified log subscription.</p>
    fn delete_log_subscription(
        &self,
        input: DeleteLogSubscriptionRequest,
    ) -> RusotoFuture<DeleteLogSubscriptionResult, DeleteLogSubscriptionError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.DeleteLogSubscription",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteLogSubscriptionResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteLogSubscriptionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes a directory snapshot.</p>
    fn delete_snapshot(
        &self,
        input: DeleteSnapshotRequest,
    ) -> RusotoFuture<DeleteSnapshotResult, DeleteSnapshotError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DirectoryService_20150416.DeleteSnapshot");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteSnapshotResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteSnapshotError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes an existing trust relationship between your AWS Managed Microsoft AD directory and an external domain.</p>
    fn delete_trust(
        &self,
        input: DeleteTrustRequest,
    ) -> RusotoFuture<DeleteTrustResult, DeleteTrustError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DirectoryService_20150416.DeleteTrust");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteTrustResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteTrustError::from_response(response))),
                )
            }
        })
    }

    /// <p>Removes the specified directory as a publisher to the specified SNS topic.</p>
    fn deregister_event_topic(
        &self,
        input: DeregisterEventTopicRequest,
    ) -> RusotoFuture<DeregisterEventTopicResult, DeregisterEventTopicError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.DeregisterEventTopic",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeregisterEventTopicResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeregisterEventTopicError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Obtains information about the conditional forwarders for this account.</p> <p>If no input parameters are provided for RemoteDomainNames, this request describes all conditional forwarders for the specified directory ID.</p>
    fn describe_conditional_forwarders(
        &self,
        input: DescribeConditionalForwardersRequest,
    ) -> RusotoFuture<DescribeConditionalForwardersResult, DescribeConditionalForwardersError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.DescribeConditionalForwarders",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeConditionalForwardersResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeConditionalForwardersError::from_response(response))
                }))
            }
        })
    }

    /// <p>Obtains information about the directories that belong to this account.</p> <p>You can retrieve information about specific directories by passing the directory identifiers in the <code>DirectoryIds</code> parameter. Otherwise, all directories that belong to the current account are returned.</p> <p>This operation supports pagination with the use of the <code>NextToken</code> request and response parameters. If more results are available, the <code>DescribeDirectoriesResult.NextToken</code> member contains a token that you pass in the next call to <a>DescribeDirectories</a> to retrieve the next set of items.</p> <p>You can also specify a maximum number of return results with the <code>Limit</code> parameter.</p>
    fn describe_directories(
        &self,
        input: DescribeDirectoriesRequest,
    ) -> RusotoFuture<DescribeDirectoriesResult, DescribeDirectoriesError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.DescribeDirectories",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeDirectoriesResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeDirectoriesError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Provides information about any domain controllers in your directory.</p>
    fn describe_domain_controllers(
        &self,
        input: DescribeDomainControllersRequest,
    ) -> RusotoFuture<DescribeDomainControllersResult, DescribeDomainControllersError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.DescribeDomainControllers",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeDomainControllersResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeDomainControllersError::from_response(response))
                }))
            }
        })
    }

    /// <p>Obtains information about which SNS topics receive status messages from the specified directory.</p> <p>If no input parameters are provided, such as DirectoryId or TopicName, this request describes all of the associations in the account.</p>
    fn describe_event_topics(
        &self,
        input: DescribeEventTopicsRequest,
    ) -> RusotoFuture<DescribeEventTopicsResult, DescribeEventTopicsError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.DescribeEventTopics",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeEventTopicsResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeEventTopicsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns the shared directories in your account. </p>
    fn describe_shared_directories(
        &self,
        input: DescribeSharedDirectoriesRequest,
    ) -> RusotoFuture<DescribeSharedDirectoriesResult, DescribeSharedDirectoriesError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.DescribeSharedDirectories",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeSharedDirectoriesResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeSharedDirectoriesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Obtains information about the directory snapshots that belong to this account.</p> <p>This operation supports pagination with the use of the <i>NextToken</i> request and response parameters. If more results are available, the <i>DescribeSnapshots.NextToken</i> member contains a token that you pass in the next call to <a>DescribeSnapshots</a> to retrieve the next set of items.</p> <p>You can also specify a maximum number of return results with the <i>Limit</i> parameter.</p>
    fn describe_snapshots(
        &self,
        input: DescribeSnapshotsRequest,
    ) -> RusotoFuture<DescribeSnapshotsResult, DescribeSnapshotsError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.DescribeSnapshots",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeSnapshotsResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeSnapshotsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Obtains information about the trust relationships for this account.</p> <p>If no input parameters are provided, such as DirectoryId or TrustIds, this request describes all the trust relationships belonging to the account.</p>
    fn describe_trusts(
        &self,
        input: DescribeTrustsRequest,
    ) -> RusotoFuture<DescribeTrustsResult, DescribeTrustsError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DirectoryService_20150416.DescribeTrusts");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeTrustsResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeTrustsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Disables multi-factor authentication (MFA) with the Remote Authentication Dial In User Service (RADIUS) server for an AD Connector or Microsoft AD directory.</p>
    fn disable_radius(
        &self,
        input: DisableRadiusRequest,
    ) -> RusotoFuture<DisableRadiusResult, DisableRadiusError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DirectoryService_20150416.DisableRadius");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DisableRadiusResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DisableRadiusError::from_response(response))),
                )
            }
        })
    }

    /// <p>Disables single-sign on for a directory.</p>
    fn disable_sso(
        &self,
        input: DisableSsoRequest,
    ) -> RusotoFuture<DisableSsoResult, DisableSsoError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DirectoryService_20150416.DisableSso");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DisableSsoResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DisableSsoError::from_response(response))),
                )
            }
        })
    }

    /// <p>Enables multi-factor authentication (MFA) with the Remote Authentication Dial In User Service (RADIUS) server for an AD Connector or Microsoft AD directory.</p>
    fn enable_radius(
        &self,
        input: EnableRadiusRequest,
    ) -> RusotoFuture<EnableRadiusResult, EnableRadiusError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DirectoryService_20150416.EnableRadius");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<EnableRadiusResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(EnableRadiusError::from_response(response))),
                )
            }
        })
    }

    /// <p>Enables single sign-on for a directory.</p>
    fn enable_sso(&self, input: EnableSsoRequest) -> RusotoFuture<EnableSsoResult, EnableSsoError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DirectoryService_20150416.EnableSso");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response).deserialize::<EnableSsoResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(EnableSsoError::from_response(response))),
                )
            }
        })
    }

    /// <p>Obtains directory limit information for the current region.</p>
    fn get_directory_limits(
        &self,
    ) -> RusotoFuture<GetDirectoryLimitsResult, GetDirectoryLimitsError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.GetDirectoryLimits",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetDirectoryLimitsResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetDirectoryLimitsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Obtains the manual snapshot limits for a directory.</p>
    fn get_snapshot_limits(
        &self,
        input: GetSnapshotLimitsRequest,
    ) -> RusotoFuture<GetSnapshotLimitsResult, GetSnapshotLimitsError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.GetSnapshotLimits",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetSnapshotLimitsResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetSnapshotLimitsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists the address blocks that you have added to a directory.</p>
    fn list_ip_routes(
        &self,
        input: ListIpRoutesRequest,
    ) -> RusotoFuture<ListIpRoutesResult, ListIpRoutesError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DirectoryService_20150416.ListIpRoutes");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListIpRoutesResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListIpRoutesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists the active log subscriptions for the AWS account.</p>
    fn list_log_subscriptions(
        &self,
        input: ListLogSubscriptionsRequest,
    ) -> RusotoFuture<ListLogSubscriptionsResult, ListLogSubscriptionsError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.ListLogSubscriptions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListLogSubscriptionsResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListLogSubscriptionsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Lists all schema extensions applied to a Microsoft AD Directory.</p>
    fn list_schema_extensions(
        &self,
        input: ListSchemaExtensionsRequest,
    ) -> RusotoFuture<ListSchemaExtensionsResult, ListSchemaExtensionsError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.ListSchemaExtensions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListSchemaExtensionsResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListSchemaExtensionsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Lists all tags on a directory.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> RusotoFuture<ListTagsForResourceResult, ListTagsForResourceError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.ListTagsForResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListTagsForResourceResult, _>()
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

    /// <p>Associates a directory with an SNS topic. This establishes the directory as a publisher to the specified SNS topic. You can then receive email or text (SMS) messages when the status of your directory changes. You get notified if your directory goes from an Active status to an Impaired or Inoperable status. You also receive a notification when the directory returns to an Active status.</p>
    fn register_event_topic(
        &self,
        input: RegisterEventTopicRequest,
    ) -> RusotoFuture<RegisterEventTopicResult, RegisterEventTopicError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.RegisterEventTopic",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<RegisterEventTopicResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(RegisterEventTopicError::from_response(response))),
                )
            }
        })
    }

    /// <p>Rejects a directory sharing request that was sent from the directory owner account.</p>
    fn reject_shared_directory(
        &self,
        input: RejectSharedDirectoryRequest,
    ) -> RusotoFuture<RejectSharedDirectoryResult, RejectSharedDirectoryError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.RejectSharedDirectory",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<RejectSharedDirectoryResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(RejectSharedDirectoryError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Removes IP address blocks from a directory.</p>
    fn remove_ip_routes(
        &self,
        input: RemoveIpRoutesRequest,
    ) -> RusotoFuture<RemoveIpRoutesResult, RemoveIpRoutesError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DirectoryService_20150416.RemoveIpRoutes");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<RemoveIpRoutesResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(RemoveIpRoutesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Removes tags from a directory.</p>
    fn remove_tags_from_resource(
        &self,
        input: RemoveTagsFromResourceRequest,
    ) -> RusotoFuture<RemoveTagsFromResourceResult, RemoveTagsFromResourceError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.RemoveTagsFromResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<RemoveTagsFromResourceResult, _>()
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

    /// <p>Resets the password for any user in your AWS Managed Microsoft AD or Simple AD directory.</p>
    fn reset_user_password(
        &self,
        input: ResetUserPasswordRequest,
    ) -> RusotoFuture<ResetUserPasswordResult, ResetUserPasswordError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.ResetUserPassword",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ResetUserPasswordResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ResetUserPasswordError::from_response(response))),
                )
            }
        })
    }

    /// <p>Restores a directory using an existing directory snapshot.</p> <p>When you restore a directory from a snapshot, any changes made to the directory after the snapshot date are overwritten.</p> <p>This action returns as soon as the restore operation is initiated. You can monitor the progress of the restore operation by calling the <a>DescribeDirectories</a> operation with the directory identifier. When the <b>DirectoryDescription.Stage</b> value changes to <code>Active</code>, the restore operation is complete.</p>
    fn restore_from_snapshot(
        &self,
        input: RestoreFromSnapshotRequest,
    ) -> RusotoFuture<RestoreFromSnapshotResult, RestoreFromSnapshotError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.RestoreFromSnapshot",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<RestoreFromSnapshotResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(RestoreFromSnapshotError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Shares a specified directory (<code>DirectoryId</code>) in your AWS account (directory owner) with another AWS account (directory consumer). With this operation you can use your directory from any AWS account and from any Amazon VPC within an AWS Region.</p> <p>When you share your AWS Managed Microsoft AD directory, AWS Directory Service creates a shared directory in the directory consumer account. This shared directory contains the metadata to provide access to the directory within the directory owner account. The shared directory is visible in all VPCs in the directory consumer account.</p> <p>The <code>ShareMethod</code> parameter determines whether the specified directory can be shared between AWS accounts inside the same AWS organization (<code>ORGANIZATIONS</code>). It also determines whether you can share the directory with any other AWS account either inside or outside of the organization (<code>HANDSHAKE</code>).</p> <p>The <code>ShareNotes</code> parameter is only used when <code>HANDSHAKE</code> is called, which sends a directory sharing request to the directory consumer. </p>
    fn share_directory(
        &self,
        input: ShareDirectoryRequest,
    ) -> RusotoFuture<ShareDirectoryResult, ShareDirectoryError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DirectoryService_20150416.ShareDirectory");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ShareDirectoryResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ShareDirectoryError::from_response(response))),
                )
            }
        })
    }

    /// <p>Applies a schema extension to a Microsoft AD directory.</p>
    fn start_schema_extension(
        &self,
        input: StartSchemaExtensionRequest,
    ) -> RusotoFuture<StartSchemaExtensionResult, StartSchemaExtensionError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.StartSchemaExtension",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartSchemaExtensionResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(StartSchemaExtensionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Stops the directory sharing between the directory owner and consumer accounts. </p>
    fn unshare_directory(
        &self,
        input: UnshareDirectoryRequest,
    ) -> RusotoFuture<UnshareDirectoryResult, UnshareDirectoryError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DirectoryService_20150416.UnshareDirectory");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UnshareDirectoryResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UnshareDirectoryError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates a conditional forwarder that has been set up for your AWS directory.</p>
    fn update_conditional_forwarder(
        &self,
        input: UpdateConditionalForwarderRequest,
    ) -> RusotoFuture<UpdateConditionalForwarderResult, UpdateConditionalForwarderError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.UpdateConditionalForwarder",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateConditionalForwarderResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateConditionalForwarderError::from_response(response))
                }))
            }
        })
    }

    /// <p>Adds or removes domain controllers to or from the directory. Based on the difference between current value and new value (provided through this API call), domain controllers will be added or removed. It may take up to 45 minutes for any new domain controllers to become fully active once the requested number of domain controllers is updated. During this time, you cannot make another update request.</p>
    fn update_number_of_domain_controllers(
        &self,
        input: UpdateNumberOfDomainControllersRequest,
    ) -> RusotoFuture<UpdateNumberOfDomainControllersResult, UpdateNumberOfDomainControllersError>
    {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "DirectoryService_20150416.UpdateNumberOfDomainControllers",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateNumberOfDomainControllersResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateNumberOfDomainControllersError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Updates the Remote Authentication Dial In User Service (RADIUS) server information for an AD Connector or Microsoft AD directory.</p>
    fn update_radius(
        &self,
        input: UpdateRadiusRequest,
    ) -> RusotoFuture<UpdateRadiusResult, UpdateRadiusError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DirectoryService_20150416.UpdateRadius");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateRadiusResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateRadiusError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates the trust that has been set up between your AWS Managed Microsoft AD directory and an on-premises Active Directory.</p>
    fn update_trust(
        &self,
        input: UpdateTrustRequest,
    ) -> RusotoFuture<UpdateTrustResult, UpdateTrustError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DirectoryService_20150416.UpdateTrust");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateTrustResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateTrustError::from_response(response))),
                )
            }
        })
    }

    /// <p>AWS Directory Service for Microsoft Active Directory allows you to configure and verify trust relationships.</p> <p>This action verifies a trust relationship between your AWS Managed Microsoft AD directory and an external domain.</p>
    fn verify_trust(
        &self,
        input: VerifyTrustRequest,
    ) -> RusotoFuture<VerifyTrustResult, VerifyTrustError> {
        let mut request = SignedRequest::new("POST", "ds", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DirectoryService_20150416.VerifyTrust");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<VerifyTrustResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(VerifyTrustError::from_response(response))),
                )
            }
        })
    }
}
