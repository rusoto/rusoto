#[allow(warnings)]
use hyper::Client;
use hyper::status::StatusCode;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::region;

use std::fmt;
use std::error::Error;
use rusoto_core::request::HttpDispatchError;
use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};

use serde_json;
use rusoto_core::signature::SignedRequest;
use serde_json::Value as SerdeJsonValue;
use serde_json::from_str;
pub type AccessUrl = String;
#[derive(Default,Debug,Clone,Serialize)]
pub struct AddIpRoutesRequest {
    #[doc="<p>Identifier (ID) of the directory to which to add the address block.</p>"]
    #[serde(rename="DirectoryId")]
    pub directory_id: DirectoryId,
    #[doc="<p>IP address blocks, using CIDR format, of the traffic to route. This is often the IP address block of the DNS server used for your on-premises domain.</p>"]
    #[serde(rename="IpRoutes")]
    pub ip_routes: IpRoutes,
    #[doc="<p>If set to true, updates the inbound and outbound rules of the security group that has the description: \"AWS created security group for <i>directory ID</i> directory controllers.\" Following are the new rules: </p> <p>Inbound:</p> <ul> <li> <p>Type: Custom UDP Rule, Protocol: UDP, Range: 88, Source: 0.0.0.0/0</p> </li> <li> <p>Type: Custom UDP Rule, Protocol: UDP, Range: 123, Source: 0.0.0.0/0</p> </li> <li> <p>Type: Custom UDP Rule, Protocol: UDP, Range: 138, Source: 0.0.0.0/0</p> </li> <li> <p>Type: Custom UDP Rule, Protocol: UDP, Range: 389, Source: 0.0.0.0/0</p> </li> <li> <p>Type: Custom UDP Rule, Protocol: UDP, Range: 464, Source: 0.0.0.0/0</p> </li> <li> <p>Type: Custom UDP Rule, Protocol: UDP, Range: 445, Source: 0.0.0.0/0</p> </li> <li> <p>Type: Custom TCP Rule, Protocol: TCP, Range: 88, Source: 0.0.0.0/0</p> </li> <li> <p>Type: Custom TCP Rule, Protocol: TCP, Range: 135, Source: 0.0.0.0/0</p> </li> <li> <p>Type: Custom TCP Rule, Protocol: TCP, Range: 445, Source: 0.0.0.0/0</p> </li> <li> <p>Type: Custom TCP Rule, Protocol: TCP, Range: 464, Source: 0.0.0.0/0</p> </li> <li> <p>Type: Custom TCP Rule, Protocol: TCP, Range: 636, Source: 0.0.0.0/0</p> </li> <li> <p>Type: Custom TCP Rule, Protocol: TCP, Range: 1024-65535, Source: 0.0.0.0/0</p> </li> <li> <p>Type: Custom TCP Rule, Protocol: TCP, Range: 3268-33269, Source: 0.0.0.0/0</p> </li> <li> <p>Type: DNS (UDP), Protocol: UDP, Range: 53, Source: 0.0.0.0/0</p> </li> <li> <p>Type: DNS (TCP), Protocol: TCP, Range: 53, Source: 0.0.0.0/0</p> </li> <li> <p>Type: LDAP, Protocol: TCP, Range: 389, Source: 0.0.0.0/0</p> </li> <li> <p>Type: All ICMP, Protocol: All, Range: N/A, Source: 0.0.0.0/0</p> </li> </ul> <p/> <p>Outbound:</p> <ul> <li> <p>Type: All traffic, Protocol: All, Range: All, Destination: 0.0.0.0/0</p> </li> </ul> <p>These security rules impact an internal network interface that is not exposed publicly.</p>"]
    #[serde(rename="UpdateSecurityGroupForDirectoryControllers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub update_security_group_for_directory_controllers:
        Option<UpdateSecurityGroupForDirectoryControllers>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct AddIpRoutesResult;

#[derive(Default,Debug,Clone,Serialize)]
pub struct AddTagsToResourceRequest {
    #[doc="<p>Identifier (ID) for the directory to which to add the tag.</p>"]
    #[serde(rename="ResourceId")]
    pub resource_id: ResourceId,
    #[doc="<p>The tags to be assigned to the directory.</p>"]
    #[serde(rename="Tags")]
    pub tags: Tags,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct AddTagsToResourceResult;

pub type AddedDateTime = f64;
pub type AliasName = String;
#[doc="<p>Represents a named directory attribute.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct Attribute {
    #[doc="<p>The name of the attribute.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<AttributeName>,
    #[doc="<p>The value of the attribute.</p>"]
    #[serde(rename="Value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<AttributeValue>,
}

pub type AttributeName = String;
pub type AttributeValue = String;
pub type Attributes = Vec<Attribute>;
pub type AvailabilityZone = String;
pub type AvailabilityZones = Vec<AvailabilityZone>;
#[derive(Default,Debug,Clone,Serialize)]
pub struct CancelSchemaExtensionRequest {
    #[doc="<p>The identifier of the directory whose schema extension will be canceled.</p>"]
    #[serde(rename="DirectoryId")]
    pub directory_id: DirectoryId,
    #[doc="<p>The identifier of the schema extension that will be canceled.</p>"]
    #[serde(rename="SchemaExtensionId")]
    pub schema_extension_id: SchemaExtensionId,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CancelSchemaExtensionResult;

pub type CidrIp = String;
pub type CidrIps = Vec<CidrIp>;
pub type CloudOnlyDirectoriesLimitReached = bool;
#[doc="<p>Contains information about a computer account in a directory.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Computer {
    #[doc="<p>An array of <a>Attribute</a> objects containing the LDAP attributes that belong to the computer account.</p>"]
    #[serde(rename="ComputerAttributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub computer_attributes: Option<Attributes>,
    #[doc="<p>The identifier of the computer.</p>"]
    #[serde(rename="ComputerId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub computer_id: Option<SID>,
    #[doc="<p>The computer name.</p>"]
    #[serde(rename="ComputerName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub computer_name: Option<ComputerName>,
}

pub type ComputerName = String;
pub type ComputerPassword = String;
#[doc="<p>Points to a remote domain with which you are setting up a trust relationship. Conditional forwarders are required in order to set up a trust relationship with another domain.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ConditionalForwarder {
    #[doc="<p>The IP addresses of the remote DNS server associated with RemoteDomainName. This is the IP address of the DNS server that your conditional forwarder points to.</p>"]
    #[serde(rename="DnsIpAddrs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dns_ip_addrs: Option<DnsIpAddrs>,
    #[doc="<p>The fully qualified domain name (FQDN) of the remote domains pointed to by the conditional forwarder.</p>"]
    #[serde(rename="RemoteDomainName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub remote_domain_name: Option<RemoteDomainName>,
    #[doc="<p>The replication scope of the conditional forwarder. The only allowed value is <code>Domain</code>, which will replicate the conditional forwarder to all of the domain controllers for your AWS directory.</p>"]
    #[serde(rename="ReplicationScope")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub replication_scope: Option<ReplicationScope>,
}

pub type ConditionalForwarders = Vec<ConditionalForwarder>;
#[doc="<p>Contains the inputs for the <a>ConnectDirectory</a> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ConnectDirectoryRequest {
    #[doc="<p>A <a>DirectoryConnectSettings</a> object that contains additional information for the operation.</p>"]
    #[serde(rename="ConnectSettings")]
    pub connect_settings: DirectoryConnectSettings,
    #[doc="<p>A textual description for the directory.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<Description>,
    #[doc="<p>The fully-qualified name of the on-premises directory, such as <code>corp.example.com</code>.</p>"]
    #[serde(rename="Name")]
    pub name: DirectoryName,
    #[doc="<p>The password for the on-premises user account.</p>"]
    #[serde(rename="Password")]
    pub password: ConnectPassword,
    #[doc="<p>The NetBIOS name of the on-premises directory, such as <code>CORP</code>.</p>"]
    #[serde(rename="ShortName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub short_name: Option<DirectoryShortName>,
    #[doc="<p>The size of the directory.</p>"]
    #[serde(rename="Size")]
    pub size: DirectorySize,
}

#[doc="<p>Contains the results of the <a>ConnectDirectory</a> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ConnectDirectoryResult {
    #[doc="<p>The identifier of the new directory.</p>"]
    #[serde(rename="DirectoryId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub directory_id: Option<DirectoryId>,
}

pub type ConnectPassword = String;
pub type ConnectedDirectoriesLimitReached = bool;
#[doc="<p>Contains the inputs for the <a>CreateAlias</a> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateAliasRequest {
    #[doc="<p>The requested alias.</p> <p>The alias must be unique amongst all aliases in AWS. This operation throws an <code>EntityAlreadyExistsException</code> error if the alias already exists.</p>"]
    #[serde(rename="Alias")]
    pub alias: AliasName,
    #[doc="<p>The identifier of the directory for which to create the alias.</p>"]
    #[serde(rename="DirectoryId")]
    pub directory_id: DirectoryId,
}

#[doc="<p>Contains the results of the <a>CreateAlias</a> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateAliasResult {
    #[doc="<p>The alias for the directory.</p>"]
    #[serde(rename="Alias")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub alias: Option<AliasName>,
    #[doc="<p>The identifier of the directory.</p>"]
    #[serde(rename="DirectoryId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub directory_id: Option<DirectoryId>,
}

#[doc="<p>Contains the inputs for the <a>CreateComputer</a> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateComputerRequest {
    #[doc="<p>An array of <a>Attribute</a> objects that contain any LDAP attributes to apply to the computer account.</p>"]
    #[serde(rename="ComputerAttributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub computer_attributes: Option<Attributes>,
    #[doc="<p>The name of the computer account.</p>"]
    #[serde(rename="ComputerName")]
    pub computer_name: ComputerName,
    #[doc="<p>The identifier of the directory in which to create the computer account.</p>"]
    #[serde(rename="DirectoryId")]
    pub directory_id: DirectoryId,
    #[doc="<p>The fully-qualified distinguished name of the organizational unit to place the computer account in.</p>"]
    #[serde(rename="OrganizationalUnitDistinguishedName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub organizational_unit_distinguished_name: Option<OrganizationalUnitDN>,
    #[doc="<p>A one-time password that is used to join the computer to the directory. You should generate a random, strong password to use for this parameter.</p>"]
    #[serde(rename="Password")]
    pub password: ComputerPassword,
}

#[doc="<p>Contains the results for the <a>CreateComputer</a> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateComputerResult {
    #[doc="<p>A <a>Computer</a> object that represents the computer account.</p>"]
    #[serde(rename="Computer")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub computer: Option<Computer>,
}

#[doc="<p>Initiates the creation of a conditional forwarder for your AWS Directory Service for Microsoft Active Directory. Conditional forwarders are required in order to set up a trust relationship with another domain.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateConditionalForwarderRequest {
    #[doc="<p>The directory ID of the AWS directory for which you are creating the conditional forwarder.</p>"]
    #[serde(rename="DirectoryId")]
    pub directory_id: DirectoryId,
    #[doc="<p>The IP addresses of the remote DNS server associated with RemoteDomainName.</p>"]
    #[serde(rename="DnsIpAddrs")]
    pub dns_ip_addrs: DnsIpAddrs,
    #[doc="<p>The fully qualified domain name (FQDN) of the remote domain with which you will set up a trust relationship.</p>"]
    #[serde(rename="RemoteDomainName")]
    pub remote_domain_name: RemoteDomainName,
}

#[doc="<p>The result of a CreateConditinalForwarder request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateConditionalForwarderResult;

#[doc="<p>Contains the inputs for the <a>CreateDirectory</a> operation. </p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateDirectoryRequest {
    #[doc="<p>A textual description for the directory.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<Description>,
    #[doc="<p>The fully qualified name for the directory, such as <code>corp.example.com</code>.</p>"]
    #[serde(rename="Name")]
    pub name: DirectoryName,
    #[doc="<p>The password for the directory administrator. The directory creation process creates a directory administrator account with the username <code>Administrator</code> and this password.</p>"]
    #[serde(rename="Password")]
    pub password: Password,
    #[doc="<p>The short name of the directory, such as <code>CORP</code>.</p>"]
    #[serde(rename="ShortName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub short_name: Option<DirectoryShortName>,
    #[doc="<p>The size of the directory.</p>"]
    #[serde(rename="Size")]
    pub size: DirectorySize,
    #[doc="<p>A <a>DirectoryVpcSettings</a> object that contains additional information for the operation.</p>"]
    #[serde(rename="VpcSettings")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vpc_settings: Option<DirectoryVpcSettings>,
}

#[doc="<p>Contains the results of the <a>CreateDirectory</a> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateDirectoryResult {
    #[doc="<p>The identifier of the directory that was created.</p>"]
    #[serde(rename="DirectoryId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub directory_id: Option<DirectoryId>,
}

#[doc="<p>Creates a Microsoft AD in the AWS cloud.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateMicrosoftADRequest {
    #[doc="<p>A textual description for the directory. This label will appear on the AWS console <code>Directory Details</code> page after the directory is created.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<Description>,
    #[doc="<p>The fully qualified domain name for the directory, such as <code>corp.example.com</code>. This name will resolve inside your VPC only. It does not need to be publicly resolvable.</p>"]
    #[serde(rename="Name")]
    pub name: DirectoryName,
    #[doc="<p>The password for the default administrative user named <code>Admin</code>.</p>"]
    #[serde(rename="Password")]
    pub password: Password,
    #[doc="<p>The NetBIOS name for your domain. A short identifier for your domain, such as <code>CORP</code>. If you don't specify a NetBIOS name, it will default to the first part of your directory DNS. For example, <code>CORP</code> for the directory DNS <code>corp.example.com</code>. </p>"]
    #[serde(rename="ShortName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub short_name: Option<DirectoryShortName>,
    #[serde(rename="VpcSettings")]
    pub vpc_settings: DirectoryVpcSettings,
}

#[doc="<p>Result of a CreateMicrosoftAD request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateMicrosoftADResult {
    #[doc="<p>The identifier of the directory that was created.</p>"]
    #[serde(rename="DirectoryId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub directory_id: Option<DirectoryId>,
}

pub type CreateSnapshotBeforeSchemaExtension = bool;
#[doc="<p>Contains the inputs for the <a>CreateSnapshot</a> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateSnapshotRequest {
    #[doc="<p>The identifier of the directory of which to take a snapshot.</p>"]
    #[serde(rename="DirectoryId")]
    pub directory_id: DirectoryId,
    #[doc="<p>The descriptive name to apply to the snapshot.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<SnapshotName>,
}

#[doc="<p>Contains the results of the <a>CreateSnapshot</a> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateSnapshotResult {
    #[doc="<p>The identifier of the snapshot that was created.</p>"]
    #[serde(rename="SnapshotId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub snapshot_id: Option<SnapshotId>,
}

#[doc="<p>AWS Directory Service for Microsoft Active Directory allows you to configure trust relationships. For example, you can establish a trust between your Microsoft AD in the AWS cloud, and your existing on-premises Microsoft Active Directory. This would allow you to provide users and groups access to resources in either domain, with a single set of credentials.</p> <p>This action initiates the creation of the AWS side of a trust relationship between a Microsoft AD in the AWS cloud and an external domain.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateTrustRequest {
    #[doc="<p>The IP addresses of the remote DNS server associated with RemoteDomainName.</p>"]
    #[serde(rename="ConditionalForwarderIpAddrs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub conditional_forwarder_ip_addrs: Option<DnsIpAddrs>,
    #[doc="<p>The Directory ID of the Microsoft AD in the AWS cloud for which to establish the trust relationship.</p>"]
    #[serde(rename="DirectoryId")]
    pub directory_id: DirectoryId,
    #[doc="<p>The Fully Qualified Domain Name (FQDN) of the external domain for which to create the trust relationship.</p>"]
    #[serde(rename="RemoteDomainName")]
    pub remote_domain_name: RemoteDomainName,
    #[doc="<p>The direction of the trust relationship.</p>"]
    #[serde(rename="TrustDirection")]
    pub trust_direction: TrustDirection,
    #[doc="<p>The trust password. The must be the same password that was used when creating the trust relationship on the external domain.</p>"]
    #[serde(rename="TrustPassword")]
    pub trust_password: TrustPassword,
    #[doc="<p>The trust relationship type.</p>"]
    #[serde(rename="TrustType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub trust_type: Option<TrustType>,
}

#[doc="<p>The result of a CreateTrust request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateTrustResult {
    #[doc="<p>A unique identifier for the trust relationship that was created.</p>"]
    #[serde(rename="TrustId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub trust_id: Option<TrustId>,
}

pub type CreatedDateTime = f64;
pub type DeleteAssociatedConditionalForwarder = bool;
#[doc="<p>Deletes a conditional forwarder.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteConditionalForwarderRequest {
    #[doc="<p>The directory ID for which you are deleting the conditional forwarder.</p>"]
    #[serde(rename="DirectoryId")]
    pub directory_id: DirectoryId,
    #[doc="<p>The fully qualified domain name (FQDN) of the remote domain with which you are deleting the conditional forwarder.</p>"]
    #[serde(rename="RemoteDomainName")]
    pub remote_domain_name: RemoteDomainName,
}

#[doc="<p>The result of a DeleteConditionalForwarder request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteConditionalForwarderResult;

#[doc="<p>Contains the inputs for the <a>DeleteDirectory</a> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteDirectoryRequest {
    #[doc="<p>The identifier of the directory to delete.</p>"]
    #[serde(rename="DirectoryId")]
    pub directory_id: DirectoryId,
}

#[doc="<p>Contains the results of the <a>DeleteDirectory</a> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteDirectoryResult {
    #[doc="<p>The directory identifier.</p>"]
    #[serde(rename="DirectoryId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub directory_id: Option<DirectoryId>,
}

#[doc="<p>Contains the inputs for the <a>DeleteSnapshot</a> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteSnapshotRequest {
    #[doc="<p>The identifier of the directory snapshot to be deleted.</p>"]
    #[serde(rename="SnapshotId")]
    pub snapshot_id: SnapshotId,
}

#[doc="<p>Contains the results of the <a>DeleteSnapshot</a> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteSnapshotResult {
    #[doc="<p>The identifier of the directory snapshot that was deleted.</p>"]
    #[serde(rename="SnapshotId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub snapshot_id: Option<SnapshotId>,
}

#[doc="<p>Deletes the local side of an existing trust relationship between the Microsoft AD in the AWS cloud and the external domain.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteTrustRequest {
    #[doc="<p>Delete a conditional forwarder as part of a DeleteTrustRequest.</p>"]
    #[serde(rename="DeleteAssociatedConditionalForwarder")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub delete_associated_conditional_forwarder: Option<DeleteAssociatedConditionalForwarder>,
    #[doc="<p>The Trust ID of the trust relationship to be deleted.</p>"]
    #[serde(rename="TrustId")]
    pub trust_id: TrustId,
}

#[doc="<p>The result of a DeleteTrust request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteTrustResult {
    #[doc="<p>The Trust ID of the trust relationship that was deleted.</p>"]
    #[serde(rename="TrustId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub trust_id: Option<TrustId>,
}

#[doc="<p>Removes the specified directory as a publisher to the specified SNS topic.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeregisterEventTopicRequest {
    #[doc="<p>The Directory ID to remove as a publisher. This directory will no longer send messages to the specified SNS topic.</p>"]
    #[serde(rename="DirectoryId")]
    pub directory_id: DirectoryId,
    #[doc="<p>The name of the SNS topic from which to remove the directory as a publisher.</p>"]
    #[serde(rename="TopicName")]
    pub topic_name: TopicName,
}

#[doc="<p>The result of a DeregisterEventTopic request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeregisterEventTopicResult;

#[doc="<p>Describes a conditional forwarder.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeConditionalForwardersRequest {
    #[doc="<p>The directory ID for which to get the list of associated conditional forwarders.</p>"]
    #[serde(rename="DirectoryId")]
    pub directory_id: DirectoryId,
    #[doc="<p>The fully qualified domain names (FQDN) of the remote domains for which to get the list of associated conditional forwarders. If this member is null, all conditional forwarders are returned.</p>"]
    #[serde(rename="RemoteDomainNames")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub remote_domain_names: Option<RemoteDomainNames>,
}

#[doc="<p>The result of a DescribeConditionalForwarder request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeConditionalForwardersResult {
    #[doc="<p>The list of conditional forwarders that have been created.</p>"]
    #[serde(rename="ConditionalForwarders")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub conditional_forwarders: Option<ConditionalForwarders>,
}

#[doc="<p>Contains the inputs for the <a>DescribeDirectories</a> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeDirectoriesRequest {
    #[doc="<p>A list of identifiers of the directories for which to obtain the information. If this member is null, all directories that belong to the current account are returned.</p> <p>An empty list results in an <code>InvalidParameterException</code> being thrown.</p>"]
    #[serde(rename="DirectoryIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub directory_ids: Option<DirectoryIds>,
    #[doc="<p>The maximum number of items to return. If this value is zero, the maximum number of items is specified by the limitations of the operation.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<Limit>,
    #[doc="<p>The <i>DescribeDirectoriesResult.NextToken</i> value from a previous call to <a>DescribeDirectories</a>. Pass null if this is the first call.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<NextToken>,
}

#[doc="<p>Contains the results of the <a>DescribeDirectories</a> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeDirectoriesResult {
    #[doc="<p>The list of <a>DirectoryDescription</a> objects that were retrieved.</p> <p>It is possible that this list contains less than the number of items specified in the <i>Limit</i> member of the request. This occurs if there are less than the requested number of items left to retrieve, or if the limitations of the operation have been exceeded.</p>"]
    #[serde(rename="DirectoryDescriptions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub directory_descriptions: Option<DirectoryDescriptions>,
    #[doc="<p>If not null, more results are available. Pass this value for the <i>NextToken</i> parameter in a subsequent call to <a>DescribeDirectories</a> to retrieve the next set of items.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<NextToken>,
}

#[doc="<p>Describes event topics.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeEventTopicsRequest {
    #[doc="<p>The Directory ID for which to get the list of associated SNS topics. If this member is null, associations for all Directory IDs are returned.</p>"]
    #[serde(rename="DirectoryId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub directory_id: Option<DirectoryId>,
    #[doc="<p>A list of SNS topic names for which to obtain the information. If this member is null, all associations for the specified Directory ID are returned.</p> <p>An empty list results in an <code>InvalidParameterException</code> being thrown.</p>"]
    #[serde(rename="TopicNames")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub topic_names: Option<TopicNames>,
}

#[doc="<p>The result of a DescribeEventTopic request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeEventTopicsResult {
    #[doc="<p>A list of SNS topic names that receive status messages from the specified Directory ID.</p>"]
    #[serde(rename="EventTopics")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event_topics: Option<EventTopics>,
}

#[doc="<p>Contains the inputs for the <a>DescribeSnapshots</a> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeSnapshotsRequest {
    #[doc="<p>The identifier of the directory for which to retrieve snapshot information.</p>"]
    #[serde(rename="DirectoryId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub directory_id: Option<DirectoryId>,
    #[doc="<p>The maximum number of objects to return.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<Limit>,
    #[doc="<p>The <i>DescribeSnapshotsResult.NextToken</i> value from a previous call to <a>DescribeSnapshots</a>. Pass null if this is the first call.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<NextToken>,
    #[doc="<p>A list of identifiers of the snapshots to obtain the information for. If this member is null or empty, all snapshots are returned using the <i>Limit</i> and <i>NextToken</i> members.</p>"]
    #[serde(rename="SnapshotIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub snapshot_ids: Option<SnapshotIds>,
}

#[doc="<p>Contains the results of the <a>DescribeSnapshots</a> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeSnapshotsResult {
    #[doc="<p>If not null, more results are available. Pass this value in the <i>NextToken</i> member of a subsequent call to <a>DescribeSnapshots</a>.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<NextToken>,
    #[doc="<p>The list of <a>Snapshot</a> objects that were retrieved.</p> <p>It is possible that this list contains less than the number of items specified in the <i>Limit</i> member of the request. This occurs if there are less than the requested number of items left to retrieve, or if the limitations of the operation have been exceeded.</p>"]
    #[serde(rename="Snapshots")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub snapshots: Option<Snapshots>,
}

#[doc="<p>Describes the trust relationships for a particular Microsoft AD in the AWS cloud. If no input parameters are are provided, such as directory ID or trust ID, this request describes all the trust relationships.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeTrustsRequest {
    #[doc="<p>The Directory ID of the AWS directory that is a part of the requested trust relationship.</p>"]
    #[serde(rename="DirectoryId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub directory_id: Option<DirectoryId>,
    #[doc="<p>The maximum number of objects to return.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<Limit>,
    #[doc="<p>The <i>DescribeTrustsResult.NextToken</i> value from a previous call to <a>DescribeTrusts</a>. Pass null if this is the first call.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<NextToken>,
    #[doc="<p>A list of identifiers of the trust relationships for which to obtain the information. If this member is null, all trust relationships that belong to the current account are returned.</p> <p>An empty list results in an <code>InvalidParameterException</code> being thrown.</p>"]
    #[serde(rename="TrustIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub trust_ids: Option<TrustIds>,
}

#[doc="<p>The result of a DescribeTrust request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeTrustsResult {
    #[doc="<p>If not null, more results are available. Pass this value for the <i>NextToken</i> parameter in a subsequent call to <a>DescribeTrusts</a> to retrieve the next set of items.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<NextToken>,
    #[doc="<p>The list of Trust objects that were retrieved.</p> <p>It is possible that this list contains less than the number of items specified in the <i>Limit</i> member of the request. This occurs if there are less than the requested number of items left to retrieve, or if the limitations of the operation have been exceeded.</p>"]
    #[serde(rename="Trusts")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub trusts: Option<Trusts>,
}

pub type Description = String;
#[doc="<p>Contains information for the <a>ConnectDirectory</a> operation when an AD Connector directory is being created.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DirectoryConnectSettings {
    #[doc="<p>A list of one or more IP addresses of DNS servers or domain controllers in the on-premises directory.</p>"]
    #[serde(rename="CustomerDnsIps")]
    pub customer_dns_ips: DnsIpAddrs,
    #[doc="<p>The username of an account in the on-premises directory that is used to connect to the directory. This account must have the following privileges:</p> <ul> <li> <p>Read users and groups</p> </li> <li> <p>Create computer objects</p> </li> <li> <p>Join computers to the domain</p> </li> </ul>"]
    #[serde(rename="CustomerUserName")]
    pub customer_user_name: UserName,
    #[doc="<p>A list of subnet identifiers in the VPC in which the AD Connector is created.</p>"]
    #[serde(rename="SubnetIds")]
    pub subnet_ids: SubnetIds,
    #[doc="<p>The identifier of the VPC in which the AD Connector is created.</p>"]
    #[serde(rename="VpcId")]
    pub vpc_id: VpcId,
}

#[doc="<p>Contains information about an AD Connector directory.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DirectoryConnectSettingsDescription {
    #[doc="<p>A list of the Availability Zones that the directory is in.</p>"]
    #[serde(rename="AvailabilityZones")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub availability_zones: Option<AvailabilityZones>,
    #[doc="<p>The IP addresses of the AD Connector servers.</p>"]
    #[serde(rename="ConnectIps")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub connect_ips: Option<IpAddrs>,
    #[doc="<p>The username of the service account in the on-premises directory.</p>"]
    #[serde(rename="CustomerUserName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub customer_user_name: Option<UserName>,
    #[doc="<p>The security group identifier for the AD Connector directory.</p>"]
    #[serde(rename="SecurityGroupId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub security_group_id: Option<SecurityGroupId>,
    #[doc="<p>A list of subnet identifiers in the VPC that the AD connector is in.</p>"]
    #[serde(rename="SubnetIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subnet_ids: Option<SubnetIds>,
    #[doc="<p>The identifier of the VPC that the AD Connector is in.</p>"]
    #[serde(rename="VpcId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vpc_id: Option<VpcId>,
}

#[doc="<p>Contains information about an AWS Directory Service directory.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DirectoryDescription {
    #[doc="<p>The access URL for the directory, such as <code>http://&lt;alias&gt;.awsapps.com</code>. If no alias has been created for the directory, <code>&lt;alias&gt;</code> is the directory identifier, such as <code>d-XXXXXXXXXX</code>.</p>"]
    #[serde(rename="AccessUrl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub access_url: Option<AccessUrl>,
    #[doc="<p>The alias for the directory. If no alias has been created for the directory, the alias is the directory identifier, such as <code>d-XXXXXXXXXX</code>.</p>"]
    #[serde(rename="Alias")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub alias: Option<AliasName>,
    #[doc="<p>A <a>DirectoryConnectSettingsDescription</a> object that contains additional information about an AD Connector directory. This member is only present if the directory is an AD Connector directory.</p>"]
    #[serde(rename="ConnectSettings")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub connect_settings: Option<DirectoryConnectSettingsDescription>,
    #[doc="<p>The textual description for the directory.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<Description>,
    #[doc="<p>The directory identifier.</p>"]
    #[serde(rename="DirectoryId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub directory_id: Option<DirectoryId>,
    #[doc="<p>The IP addresses of the DNS servers for the directory. For a Simple AD or Microsoft AD directory, these are the IP addresses of the Simple AD or Microsoft AD directory servers. For an AD Connector directory, these are the IP addresses of the DNS servers or domain controllers in the on-premises directory to which the AD Connector is connected.</p>"]
    #[serde(rename="DnsIpAddrs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dns_ip_addrs: Option<DnsIpAddrs>,
    #[doc="<p>Specifies when the directory was created.</p>"]
    #[serde(rename="LaunchTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub launch_time: Option<LaunchTime>,
    #[doc="<p>The fully-qualified name of the directory.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<DirectoryName>,
    #[doc="<p>A <a>RadiusSettings</a> object that contains information about the RADIUS server configured for this directory.</p>"]
    #[serde(rename="RadiusSettings")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub radius_settings: Option<RadiusSettings>,
    #[doc="<p>The status of the RADIUS MFA server connection.</p>"]
    #[serde(rename="RadiusStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub radius_status: Option<RadiusStatus>,
    #[doc="<p>The short name of the directory.</p>"]
    #[serde(rename="ShortName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub short_name: Option<DirectoryShortName>,
    #[doc="<p>The directory size.</p>"]
    #[serde(rename="Size")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub size: Option<DirectorySize>,
    #[doc="<p>Indicates if single-sign on is enabled for the directory. For more information, see <a>EnableSso</a> and <a>DisableSso</a>.</p>"]
    #[serde(rename="SsoEnabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sso_enabled: Option<SsoEnabled>,
    #[doc="<p>The current stage of the directory.</p>"]
    #[serde(rename="Stage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stage: Option<DirectoryStage>,
    #[doc="<p>The date and time that the stage was last updated.</p>"]
    #[serde(rename="StageLastUpdatedDateTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stage_last_updated_date_time: Option<LastUpdatedDateTime>,
    #[doc="<p>Additional information about the directory stage.</p>"]
    #[serde(rename="StageReason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stage_reason: Option<StageReason>,
    #[doc="<p>The directory size.</p>"]
    #[serde(rename="Type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub type_: Option<DirectoryType>,
    #[doc="<p>A <a>DirectoryVpcSettingsDescription</a> object that contains additional information about a directory. This member is only present if the directory is a Simple AD or Managed AD directory.</p>"]
    #[serde(rename="VpcSettings")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vpc_settings: Option<DirectoryVpcSettingsDescription>,
}

#[doc="<p>A list of directory descriptions.</p>"]
pub type DirectoryDescriptions = Vec<DirectoryDescription>;
pub type DirectoryId = String;
#[doc="<p>A list of directory identifiers.</p>"]
pub type DirectoryIds = Vec<DirectoryId>;
#[doc="<p>Contains directory limit information for a region.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DirectoryLimits {
    #[doc="<p>The current number of cloud directories in the region.</p>"]
    #[serde(rename="CloudOnlyDirectoriesCurrentCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cloud_only_directories_current_count: Option<Limit>,
    #[doc="<p>The maximum number of cloud directories allowed in the region.</p>"]
    #[serde(rename="CloudOnlyDirectoriesLimit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cloud_only_directories_limit: Option<Limit>,
    #[doc="<p>Indicates if the cloud directory limit has been reached.</p>"]
    #[serde(rename="CloudOnlyDirectoriesLimitReached")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cloud_only_directories_limit_reached: Option<CloudOnlyDirectoriesLimitReached>,
    #[doc="<p>The current number of Microsoft AD directories in the region.</p>"]
    #[serde(rename="CloudOnlyMicrosoftADCurrentCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cloud_only_microsoft_ad_current_count: Option<Limit>,
    #[doc="<p>The maximum number of Microsoft AD directories allowed in the region.</p>"]
    #[serde(rename="CloudOnlyMicrosoftADLimit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cloud_only_microsoft_ad_limit: Option<Limit>,
    #[doc="<p>Indicates if the Microsoft AD directory limit has been reached.</p>"]
    #[serde(rename="CloudOnlyMicrosoftADLimitReached")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cloud_only_microsoft_ad_limit_reached: Option<CloudOnlyDirectoriesLimitReached>,
    #[doc="<p>The current number of connected directories in the region.</p>"]
    #[serde(rename="ConnectedDirectoriesCurrentCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub connected_directories_current_count: Option<Limit>,
    #[doc="<p>The maximum number of connected directories allowed in the region.</p>"]
    #[serde(rename="ConnectedDirectoriesLimit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub connected_directories_limit: Option<Limit>,
    #[doc="<p>Indicates if the connected directory limit has been reached.</p>"]
    #[serde(rename="ConnectedDirectoriesLimitReached")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub connected_directories_limit_reached: Option<ConnectedDirectoriesLimitReached>,
}

pub type DirectoryName = String;
pub type DirectoryShortName = String;
pub type DirectorySize = String;
pub type DirectoryStage = String;
pub type DirectoryType = String;
#[doc="<p>Contains VPC information for the <a>CreateDirectory</a> or <a>CreateMicrosoftAD</a> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DirectoryVpcSettings {
    #[doc="<p>The identifiers of the subnets for the directory servers. The two subnets must be in different Availability Zones. AWS Directory Service creates a directory server and a DNS server in each of these subnets.</p>"]
    #[serde(rename="SubnetIds")]
    pub subnet_ids: SubnetIds,
    #[doc="<p>The identifier of the VPC in which to create the directory.</p>"]
    #[serde(rename="VpcId")]
    pub vpc_id: VpcId,
}

#[doc="<p>Contains information about the directory.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DirectoryVpcSettingsDescription {
    #[doc="<p>The list of Availability Zones that the directory is in.</p>"]
    #[serde(rename="AvailabilityZones")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub availability_zones: Option<AvailabilityZones>,
    #[doc="<p>The security group identifier for the directory. If the directory was created before 8/1/2014, this is the identifier of the directory members security group that was created when the directory was created. If the directory was created after this date, this value is null.</p>"]
    #[serde(rename="SecurityGroupId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub security_group_id: Option<SecurityGroupId>,
    #[doc="<p>The identifiers of the subnets for the directory servers.</p>"]
    #[serde(rename="SubnetIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subnet_ids: Option<SubnetIds>,
    #[doc="<p>The identifier of the VPC that the directory is in.</p>"]
    #[serde(rename="VpcId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vpc_id: Option<VpcId>,
}

#[doc="<p>Contains the inputs for the <a>DisableRadius</a> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DisableRadiusRequest {
    #[doc="<p>The identifier of the directory for which to disable MFA.</p>"]
    #[serde(rename="DirectoryId")]
    pub directory_id: DirectoryId,
}

#[doc="<p>Contains the results of the <a>DisableRadius</a> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DisableRadiusResult;

#[doc="<p>Contains the inputs for the <a>DisableSso</a> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DisableSsoRequest {
    #[doc="<p>The identifier of the directory for which to disable single-sign on.</p>"]
    #[serde(rename="DirectoryId")]
    pub directory_id: DirectoryId,
    #[doc="<p>The password of an alternate account to use to disable single-sign on. This is only used for AD Connector directories. For more information, see the <i>UserName</i> parameter.</p>"]
    #[serde(rename="Password")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub password: Option<ConnectPassword>,
    #[doc="<p>The username of an alternate account to use to disable single-sign on. This is only used for AD Connector directories. This account must have privileges to remove a service principal name.</p> <p>If the AD Connector service account does not have privileges to remove a service principal name, you can specify an alternate account with the <i>UserName</i> and <i>Password</i> parameters. These credentials are only used to disable single sign-on and are not stored by the service. The AD Connector service account is not changed.</p>"]
    #[serde(rename="UserName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_name: Option<UserName>,
}

#[doc="<p>Contains the results of the <a>DisableSso</a> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DisableSsoResult;

pub type DnsIpAddrs = Vec<IpAddr>;
#[doc="<p>Contains the inputs for the <a>EnableRadius</a> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct EnableRadiusRequest {
    #[doc="<p>The identifier of the directory for which to enable MFA.</p>"]
    #[serde(rename="DirectoryId")]
    pub directory_id: DirectoryId,
    #[doc="<p>A <a>RadiusSettings</a> object that contains information about the RADIUS server.</p>"]
    #[serde(rename="RadiusSettings")]
    pub radius_settings: RadiusSettings,
}

#[doc="<p>Contains the results of the <a>EnableRadius</a> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct EnableRadiusResult;

#[doc="<p>Contains the inputs for the <a>EnableSso</a> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct EnableSsoRequest {
    #[doc="<p>The identifier of the directory for which to enable single-sign on.</p>"]
    #[serde(rename="DirectoryId")]
    pub directory_id: DirectoryId,
    #[doc="<p>The password of an alternate account to use to enable single-sign on. This is only used for AD Connector directories. For more information, see the <i>UserName</i> parameter.</p>"]
    #[serde(rename="Password")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub password: Option<ConnectPassword>,
    #[doc="<p>The username of an alternate account to use to enable single-sign on. This is only used for AD Connector directories. This account must have privileges to add a service principal name.</p> <p>If the AD Connector service account does not have privileges to add a service principal name, you can specify an alternate account with the <i>UserName</i> and <i>Password</i> parameters. These credentials are only used to enable single sign-on and are not stored by the service. The AD Connector service account is not changed.</p>"]
    #[serde(rename="UserName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_name: Option<UserName>,
}

#[doc="<p>Contains the results of the <a>EnableSso</a> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct EnableSsoResult;

pub type EndDateTime = f64;
#[doc="<p>Information about SNS topic and AWS Directory Service directory associations.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct EventTopic {
    #[doc="<p>The date and time of when you associated your directory with the SNS topic.</p>"]
    #[serde(rename="CreatedDateTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_date_time: Option<CreatedDateTime>,
    #[doc="<p>The Directory ID of an AWS Directory Service directory that will publish status messages to an SNS topic.</p>"]
    #[serde(rename="DirectoryId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub directory_id: Option<DirectoryId>,
    #[doc="<p>The topic registration status.</p>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<TopicStatus>,
    #[doc="<p>The SNS topic ARN (Amazon Resource Name).</p>"]
    #[serde(rename="TopicArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub topic_arn: Option<TopicArn>,
    #[doc="<p>The name of an AWS SNS topic the receives status messages from the directory.</p>"]
    #[serde(rename="TopicName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub topic_name: Option<TopicName>,
}

pub type EventTopics = Vec<EventTopic>;
#[doc="<p>The descriptive message for the exception.</p>"]
pub type ExceptionMessage = String;
#[doc="<p>Contains the inputs for the <a>GetDirectoryLimits</a> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct GetDirectoryLimitsRequest;

#[doc="<p>Contains the results of the <a>GetDirectoryLimits</a> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetDirectoryLimitsResult {
    #[doc="<p>A <a>DirectoryLimits</a> object that contains the directory limits for the current region.</p>"]
    #[serde(rename="DirectoryLimits")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub directory_limits: Option<DirectoryLimits>,
}

#[doc="<p>Contains the inputs for the <a>GetSnapshotLimits</a> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct GetSnapshotLimitsRequest {
    #[doc="<p>Contains the identifier of the directory to obtain the limits for.</p>"]
    #[serde(rename="DirectoryId")]
    pub directory_id: DirectoryId,
}

#[doc="<p>Contains the results of the <a>GetSnapshotLimits</a> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetSnapshotLimitsResult {
    #[doc="<p>A <a>SnapshotLimits</a> object that contains the manual snapshot limits for the specified directory.</p>"]
    #[serde(rename="SnapshotLimits")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub snapshot_limits: Option<SnapshotLimits>,
}

pub type IpAddr = String;
pub type IpAddrs = Vec<IpAddr>;
#[doc="<p>IP address block. This is often the address block of the DNS server used for your on-premises domain. </p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct IpRoute {
    #[doc="<p>IP address block using CIDR format, for example 10.0.0.0/24. This is often the address block of the DNS server used for your on-premises domain. For a single IP address use a CIDR address block with /32. For example 10.0.0.0/32.</p>"]
    #[serde(rename="CidrIp")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cidr_ip: Option<CidrIp>,
    #[doc="<p>Description of the address block.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<Description>,
}

#[doc="<p>Information about one or more IP address blocks.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct IpRouteInfo {
    #[doc="<p>The date and time the address block was added to the directory.</p>"]
    #[serde(rename="AddedDateTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub added_date_time: Option<AddedDateTime>,
    #[doc="<p>IP address block in the <a>IpRoute</a>.</p>"]
    #[serde(rename="CidrIp")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cidr_ip: Option<CidrIp>,
    #[doc="<p>Description of the <a>IpRouteInfo</a>.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<Description>,
    #[doc="<p>Identifier (ID) of the directory associated with the IP addresses.</p>"]
    #[serde(rename="DirectoryId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub directory_id: Option<DirectoryId>,
    #[doc="<p>The status of the IP address block.</p>"]
    #[serde(rename="IpRouteStatusMsg")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ip_route_status_msg: Option<IpRouteStatusMsg>,
    #[doc="<p>The reason for the IpRouteStatusMsg.</p>"]
    #[serde(rename="IpRouteStatusReason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ip_route_status_reason: Option<IpRouteStatusReason>,
}

pub type IpRouteStatusMsg = String;
pub type IpRouteStatusReason = String;
pub type IpRoutes = Vec<IpRoute>;
pub type IpRoutesInfo = Vec<IpRouteInfo>;
pub type LastUpdatedDateTime = f64;
pub type LaunchTime = f64;
pub type LdifContent = String;
pub type Limit = i64;
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListIpRoutesRequest {
    #[doc="<p>Identifier (ID) of the directory for which you want to retrieve the IP addresses.</p>"]
    #[serde(rename="DirectoryId")]
    pub directory_id: DirectoryId,
    #[doc="<p>Maximum number of items to return. If this value is zero, the maximum number of items is specified by the limitations of the operation.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<Limit>,
    #[doc="<p>The <i>ListIpRoutes.NextToken</i> value from a previous call to <a>ListIpRoutes</a>. Pass null if this is the first call.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<NextToken>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListIpRoutesResult {
    #[doc="<p>A list of <a>IpRoute</a>s.</p>"]
    #[serde(rename="IpRoutesInfo")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ip_routes_info: Option<IpRoutesInfo>,
    #[doc="<p>If not null, more results are available. Pass this value for the <i>NextToken</i> parameter in a subsequent call to <a>ListIpRoutes</a> to retrieve the next set of items.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<NextToken>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListSchemaExtensionsRequest {
    #[doc="<p>The identifier of the directory from which to retrieve the schema extension information.</p>"]
    #[serde(rename="DirectoryId")]
    pub directory_id: DirectoryId,
    #[doc="<p>The maximum number of items to return.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<Limit>,
    #[doc="<p>The <code>ListSchemaExtensions.NextToken</code> value from a previous call to <code>ListSchemaExtensions</code>. Pass null if this is the first call.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<NextToken>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListSchemaExtensionsResult {
    #[doc="<p>If not null, more results are available. Pass this value for the <code>NextToken</code> parameter in a subsequent call to <code>ListSchemaExtensions</code> to retrieve the next set of items.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<NextToken>,
    #[doc="<p>Information about the schema extensions applied to the directory.</p>"]
    #[serde(rename="SchemaExtensionsInfo")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schema_extensions_info: Option<SchemaExtensionsInfo>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListTagsForResourceRequest {
    #[doc="<p>Reserved for future use.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<Limit>,
    #[doc="<p>Reserved for future use.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<NextToken>,
    #[doc="<p>Identifier (ID) of the directory for which you want to retrieve tags.</p>"]
    #[serde(rename="ResourceId")]
    pub resource_id: ResourceId,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListTagsForResourceResult {
    #[doc="<p>Reserved for future use.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<NextToken>,
    #[doc="<p>List of tags returned by the ListTagsForResource operation.</p>"]
    #[serde(rename="Tags")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tags: Option<Tags>,
}

pub type ManualSnapshotsLimitReached = bool;
pub type NextToken = String;
pub type OrganizationalUnitDN = String;
pub type Password = String;
pub type PortNumber = i64;
pub type RadiusAuthenticationProtocol = String;
pub type RadiusDisplayLabel = String;
pub type RadiusRetries = i64;
#[doc="<p>Contains information about a Remote Authentication Dial In User Service (RADIUS) server.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct RadiusSettings {
    #[doc="<p>The protocol specified for your RADIUS endpoints.</p>"]
    #[serde(rename="AuthenticationProtocol")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub authentication_protocol: Option<RadiusAuthenticationProtocol>,
    #[doc="<p>Not currently used.</p>"]
    #[serde(rename="DisplayLabel")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub display_label: Option<RadiusDisplayLabel>,
    #[doc="<p>The port that your RADIUS server is using for communications. Your on-premises network must allow inbound traffic over this port from the AWS Directory Service servers.</p>"]
    #[serde(rename="RadiusPort")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub radius_port: Option<PortNumber>,
    #[doc="<p>The maximum number of times that communication with the RADIUS server is attempted.</p>"]
    #[serde(rename="RadiusRetries")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub radius_retries: Option<RadiusRetries>,
    #[doc="<p>An array of strings that contains the IP addresses of the RADIUS server endpoints, or the IP addresses of your RADIUS server load balancer.</p>"]
    #[serde(rename="RadiusServers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub radius_servers: Option<Servers>,
    #[doc="<p>The amount of time, in seconds, to wait for the RADIUS server to respond.</p>"]
    #[serde(rename="RadiusTimeout")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub radius_timeout: Option<RadiusTimeout>,
    #[doc="<p>Not currently used.</p>"]
    #[serde(rename="SharedSecret")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub shared_secret: Option<RadiusSharedSecret>,
    #[doc="<p>Not currently used.</p>"]
    #[serde(rename="UseSameUsername")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub use_same_username: Option<UseSameUsername>,
}

pub type RadiusSharedSecret = String;
pub type RadiusStatus = String;
pub type RadiusTimeout = i64;
#[doc="<p>Registers a new event topic.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct RegisterEventTopicRequest {
    #[doc="<p>The Directory ID that will publish status messages to the SNS topic.</p>"]
    #[serde(rename="DirectoryId")]
    pub directory_id: DirectoryId,
    #[doc="<p>The SNS topic name to which the directory will publish status messages. This SNS topic must be in the same region as the specified Directory ID.</p>"]
    #[serde(rename="TopicName")]
    pub topic_name: TopicName,
}

#[doc="<p>The result of a RegisterEventTopic request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct RegisterEventTopicResult;

pub type RemoteDomainName = String;
pub type RemoteDomainNames = Vec<RemoteDomainName>;
#[derive(Default,Debug,Clone,Serialize)]
pub struct RemoveIpRoutesRequest {
    #[doc="<p>IP address blocks that you want to remove.</p>"]
    #[serde(rename="CidrIps")]
    pub cidr_ips: CidrIps,
    #[doc="<p>Identifier (ID) of the directory from which you want to remove the IP addresses.</p>"]
    #[serde(rename="DirectoryId")]
    pub directory_id: DirectoryId,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct RemoveIpRoutesResult;

#[derive(Default,Debug,Clone,Serialize)]
pub struct RemoveTagsFromResourceRequest {
    #[doc="<p>Identifier (ID) of the directory from which to remove the tag.</p>"]
    #[serde(rename="ResourceId")]
    pub resource_id: ResourceId,
    #[doc="<p>The tag key (name) of the tag to be removed.</p>"]
    #[serde(rename="TagKeys")]
    pub tag_keys: TagKeys,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct RemoveTagsFromResourceResult;

pub type ReplicationScope = String;
#[doc="<p>The AWS request identifier.</p>"]
pub type RequestId = String;
pub type ResourceId = String;
#[doc="<p>An object representing the inputs for the <a>RestoreFromSnapshot</a> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct RestoreFromSnapshotRequest {
    #[doc="<p>The identifier of the snapshot to restore from.</p>"]
    #[serde(rename="SnapshotId")]
    pub snapshot_id: SnapshotId,
}

#[doc="<p>Contains the results of the <a>RestoreFromSnapshot</a> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct RestoreFromSnapshotResult;

pub type SID = String;
pub type SchemaExtensionId = String;
#[doc="<p>Information about a schema extension.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct SchemaExtensionInfo {
    #[doc="<p>A description of the schema extension.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<Description>,
    #[doc="<p>The identifier of the directory to which the schema extension is applied.</p>"]
    #[serde(rename="DirectoryId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub directory_id: Option<DirectoryId>,
    #[doc="<p>The date and time that the schema extension was completed.</p>"]
    #[serde(rename="EndDateTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub end_date_time: Option<EndDateTime>,
    #[doc="<p>The identifier of the schema extension.</p>"]
    #[serde(rename="SchemaExtensionId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schema_extension_id: Option<SchemaExtensionId>,
    #[doc="<p>The current status of the schema extension.</p>"]
    #[serde(rename="SchemaExtensionStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schema_extension_status: Option<SchemaExtensionStatus>,
    #[doc="<p>The reason for the <code>SchemaExtensionStatus</code>.</p>"]
    #[serde(rename="SchemaExtensionStatusReason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schema_extension_status_reason: Option<SchemaExtensionStatusReason>,
    #[doc="<p>The date and time that the schema extension started being applied to the directory.</p>"]
    #[serde(rename="StartDateTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_date_time: Option<StartDateTime>,
}

pub type SchemaExtensionStatus = String;
pub type SchemaExtensionStatusReason = String;
pub type SchemaExtensionsInfo = Vec<SchemaExtensionInfo>;
pub type SecurityGroupId = String;
pub type Server = String;
pub type Servers = Vec<Server>;
#[doc="<p>Describes a directory snapshot.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Snapshot {
    #[doc="<p>The directory identifier.</p>"]
    #[serde(rename="DirectoryId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub directory_id: Option<DirectoryId>,
    #[doc="<p>The descriptive name of the snapshot.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<SnapshotName>,
    #[doc="<p>The snapshot identifier.</p>"]
    #[serde(rename="SnapshotId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub snapshot_id: Option<SnapshotId>,
    #[doc="<p>The date and time that the snapshot was taken.</p>"]
    #[serde(rename="StartTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_time: Option<StartTime>,
    #[doc="<p>The snapshot status.</p>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<SnapshotStatus>,
    #[doc="<p>The snapshot type.</p>"]
    #[serde(rename="Type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub type_: Option<SnapshotType>,
}

pub type SnapshotId = String;
#[doc="<p>A list of directory snapshot identifiers.</p>"]
pub type SnapshotIds = Vec<SnapshotId>;
#[doc="<p>Contains manual snapshot limit information for a directory.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct SnapshotLimits {
    #[doc="<p>The current number of manual snapshots of the directory.</p>"]
    #[serde(rename="ManualSnapshotsCurrentCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub manual_snapshots_current_count: Option<Limit>,
    #[doc="<p>The maximum number of manual snapshots allowed.</p>"]
    #[serde(rename="ManualSnapshotsLimit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub manual_snapshots_limit: Option<Limit>,
    #[doc="<p>Indicates if the manual snapshot limit has been reached.</p>"]
    #[serde(rename="ManualSnapshotsLimitReached")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub manual_snapshots_limit_reached: Option<ManualSnapshotsLimitReached>,
}

pub type SnapshotName = String;
pub type SnapshotStatus = String;
pub type SnapshotType = String;
#[doc="<p>A list of descriptions of directory snapshots.</p>"]
pub type Snapshots = Vec<Snapshot>;
pub type SsoEnabled = bool;
pub type StageReason = String;
pub type StartDateTime = f64;
#[derive(Default,Debug,Clone,Serialize)]
pub struct StartSchemaExtensionRequest {
    #[doc="<p>If true, creates a snapshot of the directory before applying the schema extension.</p>"]
    #[serde(rename="CreateSnapshotBeforeSchemaExtension")]
    pub create_snapshot_before_schema_extension: CreateSnapshotBeforeSchemaExtension,
    #[doc="<p>A description of the schema extension.</p>"]
    #[serde(rename="Description")]
    pub description: Description,
    #[doc="<p>The identifier of the directory for which the schema extension will be applied to.</p>"]
    #[serde(rename="DirectoryId")]
    pub directory_id: DirectoryId,
    #[doc="<p>The LDIF file represented as a string. To construct the LdifContent string, precede each line as it would be formatted in an ldif file with \\n. See the example request below for more details. The file size can be no larger than 1MB.</p>"]
    #[serde(rename="LdifContent")]
    pub ldif_content: LdifContent,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct StartSchemaExtensionResult {
    #[doc="<p>The identifier of the schema extension that will be applied.</p>"]
    #[serde(rename="SchemaExtensionId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schema_extension_id: Option<SchemaExtensionId>,
}

pub type StartTime = f64;
pub type StateLastUpdatedDateTime = f64;
pub type SubnetId = String;
pub type SubnetIds = Vec<SubnetId>;
#[doc="<p>Metadata assigned to a directory consisting of a key-value pair.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct Tag {
    #[doc="<p>Required name of the tag. The string value can be Unicode characters and cannot be prefixed with \"aws:\". The string can contain only the set of Unicode letters, digits, white-space, '_', '.', '/', '=', '+', '-' (Java regex: \"^([\\\\p{L}\\\\p{Z}\\\\p{N}_.:/=+\\\\-]*)$\").</p>"]
    #[serde(rename="Key")]
    pub key: TagKey,
    #[doc="<p>The optional value of the tag. The string value can be Unicode characters. The string can contain only the set of Unicode letters, digits, white-space, '_', '.', '/', '=', '+', '-' (Java regex: \"^([\\\\p{L}\\\\p{Z}\\\\p{N}_.:/=+\\\\-]*)$\").</p>"]
    #[serde(rename="Value")]
    pub value: TagValue,
}

pub type TagKey = String;
pub type TagKeys = Vec<TagKey>;
pub type TagValue = String;
pub type Tags = Vec<Tag>;
pub type TopicArn = String;
pub type TopicName = String;
pub type TopicNames = Vec<TopicName>;
pub type TopicStatus = String;
#[doc="<p>Describes a trust relationship between an Microsoft AD in the AWS cloud and an external domain.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Trust {
    #[doc="<p>The date and time that the trust relationship was created.</p>"]
    #[serde(rename="CreatedDateTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_date_time: Option<CreatedDateTime>,
    #[doc="<p>The Directory ID of the AWS directory involved in the trust relationship.</p>"]
    #[serde(rename="DirectoryId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub directory_id: Option<DirectoryId>,
    #[doc="<p>The date and time that the trust relationship was last updated.</p>"]
    #[serde(rename="LastUpdatedDateTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_updated_date_time: Option<LastUpdatedDateTime>,
    #[doc="<p>The Fully Qualified Domain Name (FQDN) of the external domain involved in the trust relationship.</p>"]
    #[serde(rename="RemoteDomainName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub remote_domain_name: Option<RemoteDomainName>,
    #[doc="<p>The date and time that the TrustState was last updated.</p>"]
    #[serde(rename="StateLastUpdatedDateTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state_last_updated_date_time: Option<StateLastUpdatedDateTime>,
    #[doc="<p>The trust relationship direction.</p>"]
    #[serde(rename="TrustDirection")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub trust_direction: Option<TrustDirection>,
    #[doc="<p>The unique ID of the trust relationship.</p>"]
    #[serde(rename="TrustId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub trust_id: Option<TrustId>,
    #[doc="<p>The trust relationship state.</p>"]
    #[serde(rename="TrustState")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub trust_state: Option<TrustState>,
    #[doc="<p>The reason for the TrustState.</p>"]
    #[serde(rename="TrustStateReason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub trust_state_reason: Option<TrustStateReason>,
    #[doc="<p>The trust relationship type.</p>"]
    #[serde(rename="TrustType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub trust_type: Option<TrustType>,
}

pub type TrustDirection = String;
pub type TrustId = String;
pub type TrustIds = Vec<TrustId>;
pub type TrustPassword = String;
pub type TrustState = String;
pub type TrustStateReason = String;
pub type TrustType = String;
pub type Trusts = Vec<Trust>;
#[doc="<p>Updates a conditional forwarder.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateConditionalForwarderRequest {
    #[doc="<p>The directory ID of the AWS directory for which to update the conditional forwarder.</p>"]
    #[serde(rename="DirectoryId")]
    pub directory_id: DirectoryId,
    #[doc="<p>The updated IP addresses of the remote DNS server associated with the conditional forwarder.</p>"]
    #[serde(rename="DnsIpAddrs")]
    pub dns_ip_addrs: DnsIpAddrs,
    #[doc="<p>The fully qualified domain name (FQDN) of the remote domain with which you will set up a trust relationship.</p>"]
    #[serde(rename="RemoteDomainName")]
    pub remote_domain_name: RemoteDomainName,
}

#[doc="<p>The result of an UpdateConditionalForwarder request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateConditionalForwarderResult;

#[doc="<p>Contains the inputs for the <a>UpdateRadius</a> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateRadiusRequest {
    #[doc="<p>The identifier of the directory for which to update the RADIUS server information.</p>"]
    #[serde(rename="DirectoryId")]
    pub directory_id: DirectoryId,
    #[doc="<p>A <a>RadiusSettings</a> object that contains information about the RADIUS server.</p>"]
    #[serde(rename="RadiusSettings")]
    pub radius_settings: RadiusSettings,
}

#[doc="<p>Contains the results of the <a>UpdateRadius</a> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateRadiusResult;

pub type UpdateSecurityGroupForDirectoryControllers = bool;
pub type UseSameUsername = bool;
pub type UserName = String;
#[doc="<p>Initiates the verification of an existing trust relationship between a Microsoft AD in the AWS cloud and an external domain.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct VerifyTrustRequest {
    #[doc="<p>The unique Trust ID of the trust relationship to verify.</p>"]
    #[serde(rename="TrustId")]
    pub trust_id: TrustId,
}

#[doc="<p>Result of a VerifyTrust request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct VerifyTrustResult {
    #[doc="<p>The unique Trust ID of the trust relationship that was verified.</p>"]
    #[serde(rename="TrustId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub trust_id: Option<TrustId>,
}

pub type VpcId = String;
/// Errors returned by AddIpRoutes
#[derive(Debug, PartialEq)]
pub enum AddIpRoutesError {
    ///<p>A client exception has occurred.</p>
    Client(String),
    ///<p>The specified directory is unavailable or could not be found.</p>
    DirectoryUnavailable(String),
    ///<p>The specified entity already exists.</p>
    EntityAlreadyExists(String),
    ///<p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    ///<p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    ///<p>The maximum allowed number of IP addresses was exceeded. The default limit is 100 IP address blocks.</p>
    IpRouteLimitExceeded(String),
    ///<p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AddIpRoutesError {
    pub fn from_body(body: &str) -> AddIpRoutesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => AddIpRoutesError::Client(String::from(error_message)),
                    "DirectoryUnavailableException" => {
                        AddIpRoutesError::DirectoryUnavailable(String::from(error_message))
                    }
                    "EntityAlreadyExistsException" => {
                        AddIpRoutesError::EntityAlreadyExists(String::from(error_message))
                    }
                    "EntityDoesNotExistException" => {
                        AddIpRoutesError::EntityDoesNotExist(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        AddIpRoutesError::InvalidParameter(String::from(error_message))
                    }
                    "IpRouteLimitExceededException" => {
                        AddIpRoutesError::IpRouteLimitExceeded(String::from(error_message))
                    }
                    "ServiceException" => AddIpRoutesError::Service(String::from(error_message)),
                    "ValidationException" => {
                        AddIpRoutesError::Validation(error_message.to_string())
                    }
                    _ => AddIpRoutesError::Unknown(String::from(body)),
                }
            }
            Err(_) => AddIpRoutesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AddIpRoutesError {
    fn from(err: serde_json::error::Error) -> AddIpRoutesError {
        AddIpRoutesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AddIpRoutesError {
    fn from(err: CredentialsError) -> AddIpRoutesError {
        AddIpRoutesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AddIpRoutesError {
    fn from(err: HttpDispatchError) -> AddIpRoutesError {
        AddIpRoutesError::HttpDispatch(err)
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
            AddIpRoutesError::Validation(ref cause) => cause,
            AddIpRoutesError::Credentials(ref err) => err.description(),
            AddIpRoutesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AddIpRoutesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AddTagsToResource
#[derive(Debug, PartialEq)]
pub enum AddTagsToResourceError {
    ///<p>A client exception has occurred.</p>
    Client(String),
    ///<p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    ///<p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    ///<p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    ///<p>The maximum allowed number of tags was exceeded.</p>
    TagLimitExceeded(String),
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
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => {
                        AddTagsToResourceError::Client(String::from(error_message))
                    }
                    "EntityDoesNotExistException" => {
                        AddTagsToResourceError::EntityDoesNotExist(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        AddTagsToResourceError::InvalidParameter(String::from(error_message))
                    }
                    "ServiceException" => {
                        AddTagsToResourceError::Service(String::from(error_message))
                    }
                    "TagLimitExceededException" => {
                        AddTagsToResourceError::TagLimitExceeded(String::from(error_message))
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
            AddTagsToResourceError::Validation(ref cause) => cause,
            AddTagsToResourceError::Credentials(ref err) => err.description(),
            AddTagsToResourceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AddTagsToResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CancelSchemaExtension
#[derive(Debug, PartialEq)]
pub enum CancelSchemaExtensionError {
    ///<p>A client exception has occurred.</p>
    Client(String),
    ///<p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    ///<p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CancelSchemaExtensionError {
    pub fn from_body(body: &str) -> CancelSchemaExtensionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => {
                        CancelSchemaExtensionError::Client(String::from(error_message))
                    }
                    "EntityDoesNotExistException" => {
                        CancelSchemaExtensionError::EntityDoesNotExist(String::from(error_message))
                    }
                    "ServiceException" => {
                        CancelSchemaExtensionError::Service(String::from(error_message))
                    }
                    "ValidationException" => {
                        CancelSchemaExtensionError::Validation(error_message.to_string())
                    }
                    _ => CancelSchemaExtensionError::Unknown(String::from(body)),
                }
            }
            Err(_) => CancelSchemaExtensionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CancelSchemaExtensionError {
    fn from(err: serde_json::error::Error) -> CancelSchemaExtensionError {
        CancelSchemaExtensionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CancelSchemaExtensionError {
    fn from(err: CredentialsError) -> CancelSchemaExtensionError {
        CancelSchemaExtensionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CancelSchemaExtensionError {
    fn from(err: HttpDispatchError) -> CancelSchemaExtensionError {
        CancelSchemaExtensionError::HttpDispatch(err)
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
            CancelSchemaExtensionError::Validation(ref cause) => cause,
            CancelSchemaExtensionError::Credentials(ref err) => err.description(),
            CancelSchemaExtensionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CancelSchemaExtensionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ConnectDirectory
#[derive(Debug, PartialEq)]
pub enum ConnectDirectoryError {
    ///<p>A client exception has occurred.</p>
    Client(String),
    ///<p>The maximum number of directories in the region has been reached. You can use the <a>GetDirectoryLimits</a> operation to determine your directory limits in the region.</p>
    DirectoryLimitExceeded(String),
    ///<p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    ///<p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ConnectDirectoryError {
    pub fn from_body(body: &str) -> ConnectDirectoryError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => ConnectDirectoryError::Client(String::from(error_message)),
                    "DirectoryLimitExceededException" => {
                        ConnectDirectoryError::DirectoryLimitExceeded(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        ConnectDirectoryError::InvalidParameter(String::from(error_message))
                    }
                    "ServiceException" => {
                        ConnectDirectoryError::Service(String::from(error_message))
                    }
                    "ValidationException" => {
                        ConnectDirectoryError::Validation(error_message.to_string())
                    }
                    _ => ConnectDirectoryError::Unknown(String::from(body)),
                }
            }
            Err(_) => ConnectDirectoryError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ConnectDirectoryError {
    fn from(err: serde_json::error::Error) -> ConnectDirectoryError {
        ConnectDirectoryError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ConnectDirectoryError {
    fn from(err: CredentialsError) -> ConnectDirectoryError {
        ConnectDirectoryError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ConnectDirectoryError {
    fn from(err: HttpDispatchError) -> ConnectDirectoryError {
        ConnectDirectoryError::HttpDispatch(err)
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
            ConnectDirectoryError::Validation(ref cause) => cause,
            ConnectDirectoryError::Credentials(ref err) => err.description(),
            ConnectDirectoryError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ConnectDirectoryError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateAlias
#[derive(Debug, PartialEq)]
pub enum CreateAliasError {
    ///<p>A client exception has occurred.</p>
    Client(String),
    ///<p>The specified entity already exists.</p>
    EntityAlreadyExists(String),
    ///<p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    ///<p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    ///<p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateAliasError {
    pub fn from_body(body: &str) -> CreateAliasError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => CreateAliasError::Client(String::from(error_message)),
                    "EntityAlreadyExistsException" => {
                        CreateAliasError::EntityAlreadyExists(String::from(error_message))
                    }
                    "EntityDoesNotExistException" => {
                        CreateAliasError::EntityDoesNotExist(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        CreateAliasError::InvalidParameter(String::from(error_message))
                    }
                    "ServiceException" => CreateAliasError::Service(String::from(error_message)),
                    "ValidationException" => {
                        CreateAliasError::Validation(error_message.to_string())
                    }
                    _ => CreateAliasError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateAliasError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateAliasError {
    fn from(err: serde_json::error::Error) -> CreateAliasError {
        CreateAliasError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateAliasError {
    fn from(err: CredentialsError) -> CreateAliasError {
        CreateAliasError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateAliasError {
    fn from(err: HttpDispatchError) -> CreateAliasError {
        CreateAliasError::HttpDispatch(err)
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
            CreateAliasError::Validation(ref cause) => cause,
            CreateAliasError::Credentials(ref err) => err.description(),
            CreateAliasError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateAliasError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateComputer
#[derive(Debug, PartialEq)]
pub enum CreateComputerError {
    ///<p>An authentication error occurred.</p>
    AuthenticationFailed(String),
    ///<p>A client exception has occurred.</p>
    Client(String),
    ///<p>The specified directory is unavailable or could not be found.</p>
    DirectoryUnavailable(String),
    ///<p>The specified entity already exists.</p>
    EntityAlreadyExists(String),
    ///<p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    ///<p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    ///<p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    ///<p>The operation is not supported.</p>
    UnsupportedOperation(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateComputerError {
    pub fn from_body(body: &str) -> CreateComputerError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AuthenticationFailedException" => {
                        CreateComputerError::AuthenticationFailed(String::from(error_message))
                    }
                    "ClientException" => CreateComputerError::Client(String::from(error_message)),
                    "DirectoryUnavailableException" => {
                        CreateComputerError::DirectoryUnavailable(String::from(error_message))
                    }
                    "EntityAlreadyExistsException" => {
                        CreateComputerError::EntityAlreadyExists(String::from(error_message))
                    }
                    "EntityDoesNotExistException" => {
                        CreateComputerError::EntityDoesNotExist(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        CreateComputerError::InvalidParameter(String::from(error_message))
                    }
                    "ServiceException" => CreateComputerError::Service(String::from(error_message)),
                    "UnsupportedOperationException" => {
                        CreateComputerError::UnsupportedOperation(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateComputerError::Validation(error_message.to_string())
                    }
                    _ => CreateComputerError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateComputerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateComputerError {
    fn from(err: serde_json::error::Error) -> CreateComputerError {
        CreateComputerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateComputerError {
    fn from(err: CredentialsError) -> CreateComputerError {
        CreateComputerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateComputerError {
    fn from(err: HttpDispatchError) -> CreateComputerError {
        CreateComputerError::HttpDispatch(err)
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
            CreateComputerError::Validation(ref cause) => cause,
            CreateComputerError::Credentials(ref err) => err.description(),
            CreateComputerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateComputerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateConditionalForwarder
#[derive(Debug, PartialEq)]
pub enum CreateConditionalForwarderError {
    ///<p>A client exception has occurred.</p>
    Client(String),
    ///<p>The specified directory is unavailable or could not be found.</p>
    DirectoryUnavailable(String),
    ///<p>The specified entity already exists.</p>
    EntityAlreadyExists(String),
    ///<p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    ///<p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    ///<p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    ///<p>The operation is not supported.</p>
    UnsupportedOperation(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateConditionalForwarderError {
    pub fn from_body(body: &str) -> CreateConditionalForwarderError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => {
                        CreateConditionalForwarderError::Client(String::from(error_message))
                    }
                    "DirectoryUnavailableException" => CreateConditionalForwarderError::DirectoryUnavailable(String::from(error_message)),
                    "EntityAlreadyExistsException" => CreateConditionalForwarderError::EntityAlreadyExists(String::from(error_message)),
                    "EntityDoesNotExistException" => CreateConditionalForwarderError::EntityDoesNotExist(String::from(error_message)),
                    "InvalidParameterException" => CreateConditionalForwarderError::InvalidParameter(String::from(error_message)),
                    "ServiceException" => {
                        CreateConditionalForwarderError::Service(String::from(error_message))
                    }
                    "UnsupportedOperationException" => CreateConditionalForwarderError::UnsupportedOperation(String::from(error_message)),
                    "ValidationException" => {
                        CreateConditionalForwarderError::Validation(error_message.to_string())
                    }
                    _ => CreateConditionalForwarderError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateConditionalForwarderError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateConditionalForwarderError {
    fn from(err: serde_json::error::Error) -> CreateConditionalForwarderError {
        CreateConditionalForwarderError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateConditionalForwarderError {
    fn from(err: CredentialsError) -> CreateConditionalForwarderError {
        CreateConditionalForwarderError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateConditionalForwarderError {
    fn from(err: HttpDispatchError) -> CreateConditionalForwarderError {
        CreateConditionalForwarderError::HttpDispatch(err)
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
            CreateConditionalForwarderError::Validation(ref cause) => cause,
            CreateConditionalForwarderError::Credentials(ref err) => err.description(),
            CreateConditionalForwarderError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateConditionalForwarderError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDirectory
#[derive(Debug, PartialEq)]
pub enum CreateDirectoryError {
    ///<p>A client exception has occurred.</p>
    Client(String),
    ///<p>The maximum number of directories in the region has been reached. You can use the <a>GetDirectoryLimits</a> operation to determine your directory limits in the region.</p>
    DirectoryLimitExceeded(String),
    ///<p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    ///<p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateDirectoryError {
    pub fn from_body(body: &str) -> CreateDirectoryError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => CreateDirectoryError::Client(String::from(error_message)),
                    "DirectoryLimitExceededException" => {
                        CreateDirectoryError::DirectoryLimitExceeded(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        CreateDirectoryError::InvalidParameter(String::from(error_message))
                    }
                    "ServiceException" => {
                        CreateDirectoryError::Service(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateDirectoryError::Validation(error_message.to_string())
                    }
                    _ => CreateDirectoryError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateDirectoryError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateDirectoryError {
    fn from(err: serde_json::error::Error) -> CreateDirectoryError {
        CreateDirectoryError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateDirectoryError {
    fn from(err: CredentialsError) -> CreateDirectoryError {
        CreateDirectoryError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDirectoryError {
    fn from(err: HttpDispatchError) -> CreateDirectoryError {
        CreateDirectoryError::HttpDispatch(err)
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
            CreateDirectoryError::Validation(ref cause) => cause,
            CreateDirectoryError::Credentials(ref err) => err.description(),
            CreateDirectoryError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateDirectoryError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateMicrosoftAD
#[derive(Debug, PartialEq)]
pub enum CreateMicrosoftADError {
    ///<p>A client exception has occurred.</p>
    Client(String),
    ///<p>The maximum number of directories in the region has been reached. You can use the <a>GetDirectoryLimits</a> operation to determine your directory limits in the region.</p>
    DirectoryLimitExceeded(String),
    ///<p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    ///<p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    ///<p>The operation is not supported.</p>
    UnsupportedOperation(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateMicrosoftADError {
    pub fn from_body(body: &str) -> CreateMicrosoftADError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => {
                        CreateMicrosoftADError::Client(String::from(error_message))
                    }
                    "DirectoryLimitExceededException" => {
                        CreateMicrosoftADError::DirectoryLimitExceeded(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        CreateMicrosoftADError::InvalidParameter(String::from(error_message))
                    }
                    "ServiceException" => {
                        CreateMicrosoftADError::Service(String::from(error_message))
                    }
                    "UnsupportedOperationException" => {
                        CreateMicrosoftADError::UnsupportedOperation(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateMicrosoftADError::Validation(error_message.to_string())
                    }
                    _ => CreateMicrosoftADError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateMicrosoftADError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateMicrosoftADError {
    fn from(err: serde_json::error::Error) -> CreateMicrosoftADError {
        CreateMicrosoftADError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateMicrosoftADError {
    fn from(err: CredentialsError) -> CreateMicrosoftADError {
        CreateMicrosoftADError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateMicrosoftADError {
    fn from(err: HttpDispatchError) -> CreateMicrosoftADError {
        CreateMicrosoftADError::HttpDispatch(err)
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
            CreateMicrosoftADError::Validation(ref cause) => cause,
            CreateMicrosoftADError::Credentials(ref err) => err.description(),
            CreateMicrosoftADError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateMicrosoftADError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateSnapshot
#[derive(Debug, PartialEq)]
pub enum CreateSnapshotError {
    ///<p>A client exception has occurred.</p>
    Client(String),
    ///<p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    ///<p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    ///<p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    ///<p>The maximum number of manual snapshots for the directory has been reached. You can use the <a>GetSnapshotLimits</a> operation to determine the snapshot limits for a directory.</p>
    SnapshotLimitExceeded(String),
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
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => CreateSnapshotError::Client(String::from(error_message)),
                    "EntityDoesNotExistException" => {
                        CreateSnapshotError::EntityDoesNotExist(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        CreateSnapshotError::InvalidParameter(String::from(error_message))
                    }
                    "ServiceException" => CreateSnapshotError::Service(String::from(error_message)),
                    "SnapshotLimitExceededException" => {
                        CreateSnapshotError::SnapshotLimitExceeded(String::from(error_message))
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
            CreateSnapshotError::Validation(ref cause) => cause,
            CreateSnapshotError::Credentials(ref err) => err.description(),
            CreateSnapshotError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateSnapshotError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateTrust
#[derive(Debug, PartialEq)]
pub enum CreateTrustError {
    ///<p>A client exception has occurred.</p>
    Client(String),
    ///<p>The specified entity already exists.</p>
    EntityAlreadyExists(String),
    ///<p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    ///<p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    ///<p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    ///<p>The operation is not supported.</p>
    UnsupportedOperation(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateTrustError {
    pub fn from_body(body: &str) -> CreateTrustError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => CreateTrustError::Client(String::from(error_message)),
                    "EntityAlreadyExistsException" => {
                        CreateTrustError::EntityAlreadyExists(String::from(error_message))
                    }
                    "EntityDoesNotExistException" => {
                        CreateTrustError::EntityDoesNotExist(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        CreateTrustError::InvalidParameter(String::from(error_message))
                    }
                    "ServiceException" => CreateTrustError::Service(String::from(error_message)),
                    "UnsupportedOperationException" => {
                        CreateTrustError::UnsupportedOperation(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateTrustError::Validation(error_message.to_string())
                    }
                    _ => CreateTrustError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateTrustError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateTrustError {
    fn from(err: serde_json::error::Error) -> CreateTrustError {
        CreateTrustError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateTrustError {
    fn from(err: CredentialsError) -> CreateTrustError {
        CreateTrustError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateTrustError {
    fn from(err: HttpDispatchError) -> CreateTrustError {
        CreateTrustError::HttpDispatch(err)
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
            CreateTrustError::Validation(ref cause) => cause,
            CreateTrustError::Credentials(ref err) => err.description(),
            CreateTrustError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateTrustError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteConditionalForwarder
#[derive(Debug, PartialEq)]
pub enum DeleteConditionalForwarderError {
    ///<p>A client exception has occurred.</p>
    Client(String),
    ///<p>The specified directory is unavailable or could not be found.</p>
    DirectoryUnavailable(String),
    ///<p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    ///<p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    ///<p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    ///<p>The operation is not supported.</p>
    UnsupportedOperation(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteConditionalForwarderError {
    pub fn from_body(body: &str) -> DeleteConditionalForwarderError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => {
                        DeleteConditionalForwarderError::Client(String::from(error_message))
                    }
                    "DirectoryUnavailableException" => DeleteConditionalForwarderError::DirectoryUnavailable(String::from(error_message)),
                    "EntityDoesNotExistException" => DeleteConditionalForwarderError::EntityDoesNotExist(String::from(error_message)),
                    "InvalidParameterException" => DeleteConditionalForwarderError::InvalidParameter(String::from(error_message)),
                    "ServiceException" => {
                        DeleteConditionalForwarderError::Service(String::from(error_message))
                    }
                    "UnsupportedOperationException" => DeleteConditionalForwarderError::UnsupportedOperation(String::from(error_message)),
                    "ValidationException" => {
                        DeleteConditionalForwarderError::Validation(error_message.to_string())
                    }
                    _ => DeleteConditionalForwarderError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteConditionalForwarderError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteConditionalForwarderError {
    fn from(err: serde_json::error::Error) -> DeleteConditionalForwarderError {
        DeleteConditionalForwarderError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteConditionalForwarderError {
    fn from(err: CredentialsError) -> DeleteConditionalForwarderError {
        DeleteConditionalForwarderError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteConditionalForwarderError {
    fn from(err: HttpDispatchError) -> DeleteConditionalForwarderError {
        DeleteConditionalForwarderError::HttpDispatch(err)
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
            DeleteConditionalForwarderError::Validation(ref cause) => cause,
            DeleteConditionalForwarderError::Credentials(ref err) => err.description(),
            DeleteConditionalForwarderError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteConditionalForwarderError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDirectory
#[derive(Debug, PartialEq)]
pub enum DeleteDirectoryError {
    ///<p>A client exception has occurred.</p>
    Client(String),
    ///<p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    ///<p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteDirectoryError {
    pub fn from_body(body: &str) -> DeleteDirectoryError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => DeleteDirectoryError::Client(String::from(error_message)),
                    "EntityDoesNotExistException" => {
                        DeleteDirectoryError::EntityDoesNotExist(String::from(error_message))
                    }
                    "ServiceException" => {
                        DeleteDirectoryError::Service(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteDirectoryError::Validation(error_message.to_string())
                    }
                    _ => DeleteDirectoryError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteDirectoryError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteDirectoryError {
    fn from(err: serde_json::error::Error) -> DeleteDirectoryError {
        DeleteDirectoryError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteDirectoryError {
    fn from(err: CredentialsError) -> DeleteDirectoryError {
        DeleteDirectoryError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteDirectoryError {
    fn from(err: HttpDispatchError) -> DeleteDirectoryError {
        DeleteDirectoryError::HttpDispatch(err)
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
            DeleteDirectoryError::Validation(ref cause) => cause,
            DeleteDirectoryError::Credentials(ref err) => err.description(),
            DeleteDirectoryError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteDirectoryError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteSnapshot
#[derive(Debug, PartialEq)]
pub enum DeleteSnapshotError {
    ///<p>A client exception has occurred.</p>
    Client(String),
    ///<p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    ///<p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    ///<p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteSnapshotError {
    pub fn from_body(body: &str) -> DeleteSnapshotError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => DeleteSnapshotError::Client(String::from(error_message)),
                    "EntityDoesNotExistException" => {
                        DeleteSnapshotError::EntityDoesNotExist(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DeleteSnapshotError::InvalidParameter(String::from(error_message))
                    }
                    "ServiceException" => DeleteSnapshotError::Service(String::from(error_message)),
                    "ValidationException" => {
                        DeleteSnapshotError::Validation(error_message.to_string())
                    }
                    _ => DeleteSnapshotError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteSnapshotError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteSnapshotError {
    fn from(err: serde_json::error::Error) -> DeleteSnapshotError {
        DeleteSnapshotError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteSnapshotError {
    fn from(err: CredentialsError) -> DeleteSnapshotError {
        DeleteSnapshotError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteSnapshotError {
    fn from(err: HttpDispatchError) -> DeleteSnapshotError {
        DeleteSnapshotError::HttpDispatch(err)
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
            DeleteSnapshotError::Validation(ref cause) => cause,
            DeleteSnapshotError::Credentials(ref err) => err.description(),
            DeleteSnapshotError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteSnapshotError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteTrust
#[derive(Debug, PartialEq)]
pub enum DeleteTrustError {
    ///<p>A client exception has occurred.</p>
    Client(String),
    ///<p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    ///<p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    ///<p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    ///<p>The operation is not supported.</p>
    UnsupportedOperation(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteTrustError {
    pub fn from_body(body: &str) -> DeleteTrustError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => DeleteTrustError::Client(String::from(error_message)),
                    "EntityDoesNotExistException" => {
                        DeleteTrustError::EntityDoesNotExist(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DeleteTrustError::InvalidParameter(String::from(error_message))
                    }
                    "ServiceException" => DeleteTrustError::Service(String::from(error_message)),
                    "UnsupportedOperationException" => {
                        DeleteTrustError::UnsupportedOperation(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteTrustError::Validation(error_message.to_string())
                    }
                    _ => DeleteTrustError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteTrustError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteTrustError {
    fn from(err: serde_json::error::Error) -> DeleteTrustError {
        DeleteTrustError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteTrustError {
    fn from(err: CredentialsError) -> DeleteTrustError {
        DeleteTrustError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteTrustError {
    fn from(err: HttpDispatchError) -> DeleteTrustError {
        DeleteTrustError::HttpDispatch(err)
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
            DeleteTrustError::Validation(ref cause) => cause,
            DeleteTrustError::Credentials(ref err) => err.description(),
            DeleteTrustError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteTrustError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeregisterEventTopic
#[derive(Debug, PartialEq)]
pub enum DeregisterEventTopicError {
    ///<p>A client exception has occurred.</p>
    Client(String),
    ///<p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    ///<p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    ///<p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeregisterEventTopicError {
    pub fn from_body(body: &str) -> DeregisterEventTopicError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => {
                        DeregisterEventTopicError::Client(String::from(error_message))
                    }
                    "EntityDoesNotExistException" => {
                        DeregisterEventTopicError::EntityDoesNotExist(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DeregisterEventTopicError::InvalidParameter(String::from(error_message))
                    }
                    "ServiceException" => {
                        DeregisterEventTopicError::Service(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeregisterEventTopicError::Validation(error_message.to_string())
                    }
                    _ => DeregisterEventTopicError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeregisterEventTopicError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeregisterEventTopicError {
    fn from(err: serde_json::error::Error) -> DeregisterEventTopicError {
        DeregisterEventTopicError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeregisterEventTopicError {
    fn from(err: CredentialsError) -> DeregisterEventTopicError {
        DeregisterEventTopicError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeregisterEventTopicError {
    fn from(err: HttpDispatchError) -> DeregisterEventTopicError {
        DeregisterEventTopicError::HttpDispatch(err)
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
            DeregisterEventTopicError::Validation(ref cause) => cause,
            DeregisterEventTopicError::Credentials(ref err) => err.description(),
            DeregisterEventTopicError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeregisterEventTopicError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeConditionalForwarders
#[derive(Debug, PartialEq)]
pub enum DescribeConditionalForwardersError {
    ///<p>A client exception has occurred.</p>
    Client(String),
    ///<p>The specified directory is unavailable or could not be found.</p>
    DirectoryUnavailable(String),
    ///<p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    ///<p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    ///<p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    ///<p>The operation is not supported.</p>
    UnsupportedOperation(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeConditionalForwardersError {
    pub fn from_body(body: &str) -> DescribeConditionalForwardersError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => {
                        DescribeConditionalForwardersError::Client(String::from(error_message))
                    }
                    "DirectoryUnavailableException" => DescribeConditionalForwardersError::DirectoryUnavailable(String::from(error_message)),
                    "EntityDoesNotExistException" => DescribeConditionalForwardersError::EntityDoesNotExist(String::from(error_message)),
                    "InvalidParameterException" => DescribeConditionalForwardersError::InvalidParameter(String::from(error_message)),
                    "ServiceException" => {
                        DescribeConditionalForwardersError::Service(String::from(error_message))
                    }
                    "UnsupportedOperationException" => DescribeConditionalForwardersError::UnsupportedOperation(String::from(error_message)),
                    "ValidationException" => {
                        DescribeConditionalForwardersError::Validation(error_message.to_string())
                    }
                    _ => DescribeConditionalForwardersError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeConditionalForwardersError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeConditionalForwardersError {
    fn from(err: serde_json::error::Error) -> DescribeConditionalForwardersError {
        DescribeConditionalForwardersError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeConditionalForwardersError {
    fn from(err: CredentialsError) -> DescribeConditionalForwardersError {
        DescribeConditionalForwardersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeConditionalForwardersError {
    fn from(err: HttpDispatchError) -> DescribeConditionalForwardersError {
        DescribeConditionalForwardersError::HttpDispatch(err)
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
            DescribeConditionalForwardersError::Validation(ref cause) => cause,
            DescribeConditionalForwardersError::Credentials(ref err) => err.description(),
            DescribeConditionalForwardersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeConditionalForwardersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeDirectories
#[derive(Debug, PartialEq)]
pub enum DescribeDirectoriesError {
    ///<p>A client exception has occurred.</p>
    Client(String),
    ///<p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    ///<p>The <i>NextToken</i> value is not valid.</p>
    InvalidNextToken(String),
    ///<p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    ///<p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeDirectoriesError {
    pub fn from_body(body: &str) -> DescribeDirectoriesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => {
                        DescribeDirectoriesError::Client(String::from(error_message))
                    }
                    "EntityDoesNotExistException" => {
                        DescribeDirectoriesError::EntityDoesNotExist(String::from(error_message))
                    }
                    "InvalidNextTokenException" => {
                        DescribeDirectoriesError::InvalidNextToken(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DescribeDirectoriesError::InvalidParameter(String::from(error_message))
                    }
                    "ServiceException" => {
                        DescribeDirectoriesError::Service(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeDirectoriesError::Validation(error_message.to_string())
                    }
                    _ => DescribeDirectoriesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeDirectoriesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeDirectoriesError {
    fn from(err: serde_json::error::Error) -> DescribeDirectoriesError {
        DescribeDirectoriesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeDirectoriesError {
    fn from(err: CredentialsError) -> DescribeDirectoriesError {
        DescribeDirectoriesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeDirectoriesError {
    fn from(err: HttpDispatchError) -> DescribeDirectoriesError {
        DescribeDirectoriesError::HttpDispatch(err)
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
            DescribeDirectoriesError::Validation(ref cause) => cause,
            DescribeDirectoriesError::Credentials(ref err) => err.description(),
            DescribeDirectoriesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeDirectoriesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeEventTopics
#[derive(Debug, PartialEq)]
pub enum DescribeEventTopicsError {
    ///<p>A client exception has occurred.</p>
    Client(String),
    ///<p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    ///<p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    ///<p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeEventTopicsError {
    pub fn from_body(body: &str) -> DescribeEventTopicsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => {
                        DescribeEventTopicsError::Client(String::from(error_message))
                    }
                    "EntityDoesNotExistException" => {
                        DescribeEventTopicsError::EntityDoesNotExist(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DescribeEventTopicsError::InvalidParameter(String::from(error_message))
                    }
                    "ServiceException" => {
                        DescribeEventTopicsError::Service(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeEventTopicsError::Validation(error_message.to_string())
                    }
                    _ => DescribeEventTopicsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeEventTopicsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeEventTopicsError {
    fn from(err: serde_json::error::Error) -> DescribeEventTopicsError {
        DescribeEventTopicsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeEventTopicsError {
    fn from(err: CredentialsError) -> DescribeEventTopicsError {
        DescribeEventTopicsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeEventTopicsError {
    fn from(err: HttpDispatchError) -> DescribeEventTopicsError {
        DescribeEventTopicsError::HttpDispatch(err)
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
            DescribeEventTopicsError::Validation(ref cause) => cause,
            DescribeEventTopicsError::Credentials(ref err) => err.description(),
            DescribeEventTopicsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeEventTopicsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeSnapshots
#[derive(Debug, PartialEq)]
pub enum DescribeSnapshotsError {
    ///<p>A client exception has occurred.</p>
    Client(String),
    ///<p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    ///<p>The <i>NextToken</i> value is not valid.</p>
    InvalidNextToken(String),
    ///<p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    ///<p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeSnapshotsError {
    pub fn from_body(body: &str) -> DescribeSnapshotsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => {
                        DescribeSnapshotsError::Client(String::from(error_message))
                    }
                    "EntityDoesNotExistException" => {
                        DescribeSnapshotsError::EntityDoesNotExist(String::from(error_message))
                    }
                    "InvalidNextTokenException" => {
                        DescribeSnapshotsError::InvalidNextToken(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DescribeSnapshotsError::InvalidParameter(String::from(error_message))
                    }
                    "ServiceException" => {
                        DescribeSnapshotsError::Service(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeSnapshotsError::Validation(error_message.to_string())
                    }
                    _ => DescribeSnapshotsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeSnapshotsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeSnapshotsError {
    fn from(err: serde_json::error::Error) -> DescribeSnapshotsError {
        DescribeSnapshotsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeSnapshotsError {
    fn from(err: CredentialsError) -> DescribeSnapshotsError {
        DescribeSnapshotsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeSnapshotsError {
    fn from(err: HttpDispatchError) -> DescribeSnapshotsError {
        DescribeSnapshotsError::HttpDispatch(err)
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
            DescribeSnapshotsError::Validation(ref cause) => cause,
            DescribeSnapshotsError::Credentials(ref err) => err.description(),
            DescribeSnapshotsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeSnapshotsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeTrusts
#[derive(Debug, PartialEq)]
pub enum DescribeTrustsError {
    ///<p>A client exception has occurred.</p>
    Client(String),
    ///<p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    ///<p>The <i>NextToken</i> value is not valid.</p>
    InvalidNextToken(String),
    ///<p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    ///<p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    ///<p>The operation is not supported.</p>
    UnsupportedOperation(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeTrustsError {
    pub fn from_body(body: &str) -> DescribeTrustsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => DescribeTrustsError::Client(String::from(error_message)),
                    "EntityDoesNotExistException" => {
                        DescribeTrustsError::EntityDoesNotExist(String::from(error_message))
                    }
                    "InvalidNextTokenException" => {
                        DescribeTrustsError::InvalidNextToken(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DescribeTrustsError::InvalidParameter(String::from(error_message))
                    }
                    "ServiceException" => DescribeTrustsError::Service(String::from(error_message)),
                    "UnsupportedOperationException" => {
                        DescribeTrustsError::UnsupportedOperation(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeTrustsError::Validation(error_message.to_string())
                    }
                    _ => DescribeTrustsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeTrustsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeTrustsError {
    fn from(err: serde_json::error::Error) -> DescribeTrustsError {
        DescribeTrustsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeTrustsError {
    fn from(err: CredentialsError) -> DescribeTrustsError {
        DescribeTrustsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeTrustsError {
    fn from(err: HttpDispatchError) -> DescribeTrustsError {
        DescribeTrustsError::HttpDispatch(err)
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
            DescribeTrustsError::Validation(ref cause) => cause,
            DescribeTrustsError::Credentials(ref err) => err.description(),
            DescribeTrustsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeTrustsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DisableRadius
#[derive(Debug, PartialEq)]
pub enum DisableRadiusError {
    ///<p>A client exception has occurred.</p>
    Client(String),
    ///<p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    ///<p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DisableRadiusError {
    pub fn from_body(body: &str) -> DisableRadiusError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => DisableRadiusError::Client(String::from(error_message)),
                    "EntityDoesNotExistException" => {
                        DisableRadiusError::EntityDoesNotExist(String::from(error_message))
                    }
                    "ServiceException" => DisableRadiusError::Service(String::from(error_message)),
                    "ValidationException" => {
                        DisableRadiusError::Validation(error_message.to_string())
                    }
                    _ => DisableRadiusError::Unknown(String::from(body)),
                }
            }
            Err(_) => DisableRadiusError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DisableRadiusError {
    fn from(err: serde_json::error::Error) -> DisableRadiusError {
        DisableRadiusError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DisableRadiusError {
    fn from(err: CredentialsError) -> DisableRadiusError {
        DisableRadiusError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisableRadiusError {
    fn from(err: HttpDispatchError) -> DisableRadiusError {
        DisableRadiusError::HttpDispatch(err)
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
            DisableRadiusError::Validation(ref cause) => cause,
            DisableRadiusError::Credentials(ref err) => err.description(),
            DisableRadiusError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DisableRadiusError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DisableSso
#[derive(Debug, PartialEq)]
pub enum DisableSsoError {
    ///<p>An authentication error occurred.</p>
    AuthenticationFailed(String),
    ///<p>A client exception has occurred.</p>
    Client(String),
    ///<p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    ///<p>The account does not have sufficient permission to perform the operation.</p>
    InsufficientPermissions(String),
    ///<p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DisableSsoError {
    pub fn from_body(body: &str) -> DisableSsoError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AuthenticationFailedException" => {
                        DisableSsoError::AuthenticationFailed(String::from(error_message))
                    }
                    "ClientException" => DisableSsoError::Client(String::from(error_message)),
                    "EntityDoesNotExistException" => {
                        DisableSsoError::EntityDoesNotExist(String::from(error_message))
                    }
                    "InsufficientPermissionsException" => {
                        DisableSsoError::InsufficientPermissions(String::from(error_message))
                    }
                    "ServiceException" => DisableSsoError::Service(String::from(error_message)),
                    "ValidationException" => DisableSsoError::Validation(error_message.to_string()),
                    _ => DisableSsoError::Unknown(String::from(body)),
                }
            }
            Err(_) => DisableSsoError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DisableSsoError {
    fn from(err: serde_json::error::Error) -> DisableSsoError {
        DisableSsoError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DisableSsoError {
    fn from(err: CredentialsError) -> DisableSsoError {
        DisableSsoError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisableSsoError {
    fn from(err: HttpDispatchError) -> DisableSsoError {
        DisableSsoError::HttpDispatch(err)
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
            DisableSsoError::Validation(ref cause) => cause,
            DisableSsoError::Credentials(ref err) => err.description(),
            DisableSsoError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DisableSsoError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by EnableRadius
#[derive(Debug, PartialEq)]
pub enum EnableRadiusError {
    ///<p>A client exception has occurred.</p>
    Client(String),
    ///<p>The specified entity already exists.</p>
    EntityAlreadyExists(String),
    ///<p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    ///<p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    ///<p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl EnableRadiusError {
    pub fn from_body(body: &str) -> EnableRadiusError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => EnableRadiusError::Client(String::from(error_message)),
                    "EntityAlreadyExistsException" => {
                        EnableRadiusError::EntityAlreadyExists(String::from(error_message))
                    }
                    "EntityDoesNotExistException" => {
                        EnableRadiusError::EntityDoesNotExist(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        EnableRadiusError::InvalidParameter(String::from(error_message))
                    }
                    "ServiceException" => EnableRadiusError::Service(String::from(error_message)),
                    "ValidationException" => {
                        EnableRadiusError::Validation(error_message.to_string())
                    }
                    _ => EnableRadiusError::Unknown(String::from(body)),
                }
            }
            Err(_) => EnableRadiusError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for EnableRadiusError {
    fn from(err: serde_json::error::Error) -> EnableRadiusError {
        EnableRadiusError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for EnableRadiusError {
    fn from(err: CredentialsError) -> EnableRadiusError {
        EnableRadiusError::Credentials(err)
    }
}
impl From<HttpDispatchError> for EnableRadiusError {
    fn from(err: HttpDispatchError) -> EnableRadiusError {
        EnableRadiusError::HttpDispatch(err)
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
            EnableRadiusError::Validation(ref cause) => cause,
            EnableRadiusError::Credentials(ref err) => err.description(),
            EnableRadiusError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            EnableRadiusError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by EnableSso
#[derive(Debug, PartialEq)]
pub enum EnableSsoError {
    ///<p>An authentication error occurred.</p>
    AuthenticationFailed(String),
    ///<p>A client exception has occurred.</p>
    Client(String),
    ///<p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    ///<p>The account does not have sufficient permission to perform the operation.</p>
    InsufficientPermissions(String),
    ///<p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl EnableSsoError {
    pub fn from_body(body: &str) -> EnableSsoError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AuthenticationFailedException" => {
                        EnableSsoError::AuthenticationFailed(String::from(error_message))
                    }
                    "ClientException" => EnableSsoError::Client(String::from(error_message)),
                    "EntityDoesNotExistException" => {
                        EnableSsoError::EntityDoesNotExist(String::from(error_message))
                    }
                    "InsufficientPermissionsException" => {
                        EnableSsoError::InsufficientPermissions(String::from(error_message))
                    }
                    "ServiceException" => EnableSsoError::Service(String::from(error_message)),
                    "ValidationException" => EnableSsoError::Validation(error_message.to_string()),
                    _ => EnableSsoError::Unknown(String::from(body)),
                }
            }
            Err(_) => EnableSsoError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for EnableSsoError {
    fn from(err: serde_json::error::Error) -> EnableSsoError {
        EnableSsoError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for EnableSsoError {
    fn from(err: CredentialsError) -> EnableSsoError {
        EnableSsoError::Credentials(err)
    }
}
impl From<HttpDispatchError> for EnableSsoError {
    fn from(err: HttpDispatchError) -> EnableSsoError {
        EnableSsoError::HttpDispatch(err)
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
            EnableSsoError::Validation(ref cause) => cause,
            EnableSsoError::Credentials(ref err) => err.description(),
            EnableSsoError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            EnableSsoError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDirectoryLimits
#[derive(Debug, PartialEq)]
pub enum GetDirectoryLimitsError {
    ///<p>A client exception has occurred.</p>
    Client(String),
    ///<p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    ///<p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetDirectoryLimitsError {
    pub fn from_body(body: &str) -> GetDirectoryLimitsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => {
                        GetDirectoryLimitsError::Client(String::from(error_message))
                    }
                    "EntityDoesNotExistException" => {
                        GetDirectoryLimitsError::EntityDoesNotExist(String::from(error_message))
                    }
                    "ServiceException" => {
                        GetDirectoryLimitsError::Service(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetDirectoryLimitsError::Validation(error_message.to_string())
                    }
                    _ => GetDirectoryLimitsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetDirectoryLimitsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetDirectoryLimitsError {
    fn from(err: serde_json::error::Error) -> GetDirectoryLimitsError {
        GetDirectoryLimitsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDirectoryLimitsError {
    fn from(err: CredentialsError) -> GetDirectoryLimitsError {
        GetDirectoryLimitsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDirectoryLimitsError {
    fn from(err: HttpDispatchError) -> GetDirectoryLimitsError {
        GetDirectoryLimitsError::HttpDispatch(err)
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
            GetDirectoryLimitsError::Validation(ref cause) => cause,
            GetDirectoryLimitsError::Credentials(ref err) => err.description(),
            GetDirectoryLimitsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetDirectoryLimitsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSnapshotLimits
#[derive(Debug, PartialEq)]
pub enum GetSnapshotLimitsError {
    ///<p>A client exception has occurred.</p>
    Client(String),
    ///<p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    ///<p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetSnapshotLimitsError {
    pub fn from_body(body: &str) -> GetSnapshotLimitsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => {
                        GetSnapshotLimitsError::Client(String::from(error_message))
                    }
                    "EntityDoesNotExistException" => {
                        GetSnapshotLimitsError::EntityDoesNotExist(String::from(error_message))
                    }
                    "ServiceException" => {
                        GetSnapshotLimitsError::Service(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetSnapshotLimitsError::Validation(error_message.to_string())
                    }
                    _ => GetSnapshotLimitsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetSnapshotLimitsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetSnapshotLimitsError {
    fn from(err: serde_json::error::Error) -> GetSnapshotLimitsError {
        GetSnapshotLimitsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetSnapshotLimitsError {
    fn from(err: CredentialsError) -> GetSnapshotLimitsError {
        GetSnapshotLimitsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetSnapshotLimitsError {
    fn from(err: HttpDispatchError) -> GetSnapshotLimitsError {
        GetSnapshotLimitsError::HttpDispatch(err)
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
            GetSnapshotLimitsError::Validation(ref cause) => cause,
            GetSnapshotLimitsError::Credentials(ref err) => err.description(),
            GetSnapshotLimitsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetSnapshotLimitsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListIpRoutes
#[derive(Debug, PartialEq)]
pub enum ListIpRoutesError {
    ///<p>A client exception has occurred.</p>
    Client(String),
    ///<p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    ///<p>The <i>NextToken</i> value is not valid.</p>
    InvalidNextToken(String),
    ///<p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    ///<p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListIpRoutesError {
    pub fn from_body(body: &str) -> ListIpRoutesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => ListIpRoutesError::Client(String::from(error_message)),
                    "EntityDoesNotExistException" => {
                        ListIpRoutesError::EntityDoesNotExist(String::from(error_message))
                    }
                    "InvalidNextTokenException" => {
                        ListIpRoutesError::InvalidNextToken(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        ListIpRoutesError::InvalidParameter(String::from(error_message))
                    }
                    "ServiceException" => ListIpRoutesError::Service(String::from(error_message)),
                    "ValidationException" => {
                        ListIpRoutesError::Validation(error_message.to_string())
                    }
                    _ => ListIpRoutesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListIpRoutesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListIpRoutesError {
    fn from(err: serde_json::error::Error) -> ListIpRoutesError {
        ListIpRoutesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListIpRoutesError {
    fn from(err: CredentialsError) -> ListIpRoutesError {
        ListIpRoutesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListIpRoutesError {
    fn from(err: HttpDispatchError) -> ListIpRoutesError {
        ListIpRoutesError::HttpDispatch(err)
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
            ListIpRoutesError::Validation(ref cause) => cause,
            ListIpRoutesError::Credentials(ref err) => err.description(),
            ListIpRoutesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListIpRoutesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListSchemaExtensions
#[derive(Debug, PartialEq)]
pub enum ListSchemaExtensionsError {
    ///<p>A client exception has occurred.</p>
    Client(String),
    ///<p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    ///<p>The <i>NextToken</i> value is not valid.</p>
    InvalidNextToken(String),
    ///<p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListSchemaExtensionsError {
    pub fn from_body(body: &str) -> ListSchemaExtensionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => {
                        ListSchemaExtensionsError::Client(String::from(error_message))
                    }
                    "EntityDoesNotExistException" => {
                        ListSchemaExtensionsError::EntityDoesNotExist(String::from(error_message))
                    }
                    "InvalidNextTokenException" => {
                        ListSchemaExtensionsError::InvalidNextToken(String::from(error_message))
                    }
                    "ServiceException" => {
                        ListSchemaExtensionsError::Service(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListSchemaExtensionsError::Validation(error_message.to_string())
                    }
                    _ => ListSchemaExtensionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListSchemaExtensionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListSchemaExtensionsError {
    fn from(err: serde_json::error::Error) -> ListSchemaExtensionsError {
        ListSchemaExtensionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListSchemaExtensionsError {
    fn from(err: CredentialsError) -> ListSchemaExtensionsError {
        ListSchemaExtensionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListSchemaExtensionsError {
    fn from(err: HttpDispatchError) -> ListSchemaExtensionsError {
        ListSchemaExtensionsError::HttpDispatch(err)
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
            ListSchemaExtensionsError::Validation(ref cause) => cause,
            ListSchemaExtensionsError::Credentials(ref err) => err.description(),
            ListSchemaExtensionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListSchemaExtensionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    ///<p>A client exception has occurred.</p>
    Client(String),
    ///<p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    ///<p>The <i>NextToken</i> value is not valid.</p>
    InvalidNextToken(String),
    ///<p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    ///<p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
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
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => {
                        ListTagsForResourceError::Client(String::from(error_message))
                    }
                    "EntityDoesNotExistException" => {
                        ListTagsForResourceError::EntityDoesNotExist(String::from(error_message))
                    }
                    "InvalidNextTokenException" => {
                        ListTagsForResourceError::InvalidNextToken(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        ListTagsForResourceError::InvalidParameter(String::from(error_message))
                    }
                    "ServiceException" => {
                        ListTagsForResourceError::Service(String::from(error_message))
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
            ListTagsForResourceError::Validation(ref cause) => cause,
            ListTagsForResourceError::Credentials(ref err) => err.description(),
            ListTagsForResourceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListTagsForResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RegisterEventTopic
#[derive(Debug, PartialEq)]
pub enum RegisterEventTopicError {
    ///<p>A client exception has occurred.</p>
    Client(String),
    ///<p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    ///<p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    ///<p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl RegisterEventTopicError {
    pub fn from_body(body: &str) -> RegisterEventTopicError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => {
                        RegisterEventTopicError::Client(String::from(error_message))
                    }
                    "EntityDoesNotExistException" => {
                        RegisterEventTopicError::EntityDoesNotExist(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        RegisterEventTopicError::InvalidParameter(String::from(error_message))
                    }
                    "ServiceException" => {
                        RegisterEventTopicError::Service(String::from(error_message))
                    }
                    "ValidationException" => {
                        RegisterEventTopicError::Validation(error_message.to_string())
                    }
                    _ => RegisterEventTopicError::Unknown(String::from(body)),
                }
            }
            Err(_) => RegisterEventTopicError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RegisterEventTopicError {
    fn from(err: serde_json::error::Error) -> RegisterEventTopicError {
        RegisterEventTopicError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RegisterEventTopicError {
    fn from(err: CredentialsError) -> RegisterEventTopicError {
        RegisterEventTopicError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RegisterEventTopicError {
    fn from(err: HttpDispatchError) -> RegisterEventTopicError {
        RegisterEventTopicError::HttpDispatch(err)
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
            RegisterEventTopicError::Validation(ref cause) => cause,
            RegisterEventTopicError::Credentials(ref err) => err.description(),
            RegisterEventTopicError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RegisterEventTopicError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RemoveIpRoutes
#[derive(Debug, PartialEq)]
pub enum RemoveIpRoutesError {
    ///<p>A client exception has occurred.</p>
    Client(String),
    ///<p>The specified directory is unavailable or could not be found.</p>
    DirectoryUnavailable(String),
    ///<p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    ///<p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    ///<p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl RemoveIpRoutesError {
    pub fn from_body(body: &str) -> RemoveIpRoutesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => RemoveIpRoutesError::Client(String::from(error_message)),
                    "DirectoryUnavailableException" => {
                        RemoveIpRoutesError::DirectoryUnavailable(String::from(error_message))
                    }
                    "EntityDoesNotExistException" => {
                        RemoveIpRoutesError::EntityDoesNotExist(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        RemoveIpRoutesError::InvalidParameter(String::from(error_message))
                    }
                    "ServiceException" => RemoveIpRoutesError::Service(String::from(error_message)),
                    "ValidationException" => {
                        RemoveIpRoutesError::Validation(error_message.to_string())
                    }
                    _ => RemoveIpRoutesError::Unknown(String::from(body)),
                }
            }
            Err(_) => RemoveIpRoutesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RemoveIpRoutesError {
    fn from(err: serde_json::error::Error) -> RemoveIpRoutesError {
        RemoveIpRoutesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RemoveIpRoutesError {
    fn from(err: CredentialsError) -> RemoveIpRoutesError {
        RemoveIpRoutesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RemoveIpRoutesError {
    fn from(err: HttpDispatchError) -> RemoveIpRoutesError {
        RemoveIpRoutesError::HttpDispatch(err)
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
            RemoveIpRoutesError::Validation(ref cause) => cause,
            RemoveIpRoutesError::Credentials(ref err) => err.description(),
            RemoveIpRoutesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RemoveIpRoutesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RemoveTagsFromResource
#[derive(Debug, PartialEq)]
pub enum RemoveTagsFromResourceError {
    ///<p>A client exception has occurred.</p>
    Client(String),
    ///<p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    ///<p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    ///<p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
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
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => {
                        RemoveTagsFromResourceError::Client(String::from(error_message))
                    }
                    "EntityDoesNotExistException" => {
                        RemoveTagsFromResourceError::EntityDoesNotExist(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        RemoveTagsFromResourceError::InvalidParameter(String::from(error_message))
                    }
                    "ServiceException" => {
                        RemoveTagsFromResourceError::Service(String::from(error_message))
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
            RemoveTagsFromResourceError::Validation(ref cause) => cause,
            RemoveTagsFromResourceError::Credentials(ref err) => err.description(),
            RemoveTagsFromResourceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RemoveTagsFromResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RestoreFromSnapshot
#[derive(Debug, PartialEq)]
pub enum RestoreFromSnapshotError {
    ///<p>A client exception has occurred.</p>
    Client(String),
    ///<p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    ///<p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    ///<p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl RestoreFromSnapshotError {
    pub fn from_body(body: &str) -> RestoreFromSnapshotError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => {
                        RestoreFromSnapshotError::Client(String::from(error_message))
                    }
                    "EntityDoesNotExistException" => {
                        RestoreFromSnapshotError::EntityDoesNotExist(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        RestoreFromSnapshotError::InvalidParameter(String::from(error_message))
                    }
                    "ServiceException" => {
                        RestoreFromSnapshotError::Service(String::from(error_message))
                    }
                    "ValidationException" => {
                        RestoreFromSnapshotError::Validation(error_message.to_string())
                    }
                    _ => RestoreFromSnapshotError::Unknown(String::from(body)),
                }
            }
            Err(_) => RestoreFromSnapshotError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RestoreFromSnapshotError {
    fn from(err: serde_json::error::Error) -> RestoreFromSnapshotError {
        RestoreFromSnapshotError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RestoreFromSnapshotError {
    fn from(err: CredentialsError) -> RestoreFromSnapshotError {
        RestoreFromSnapshotError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RestoreFromSnapshotError {
    fn from(err: HttpDispatchError) -> RestoreFromSnapshotError {
        RestoreFromSnapshotError::HttpDispatch(err)
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
            RestoreFromSnapshotError::Validation(ref cause) => cause,
            RestoreFromSnapshotError::Credentials(ref err) => err.description(),
            RestoreFromSnapshotError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RestoreFromSnapshotError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StartSchemaExtension
#[derive(Debug, PartialEq)]
pub enum StartSchemaExtensionError {
    ///<p>A client exception has occurred.</p>
    Client(String),
    ///<p>The specified directory is unavailable or could not be found.</p>
    DirectoryUnavailable(String),
    ///<p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    ///<p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    ///<p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    ///<p>The maximum number of manual snapshots for the directory has been reached. You can use the <a>GetSnapshotLimits</a> operation to determine the snapshot limits for a directory.</p>
    SnapshotLimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl StartSchemaExtensionError {
    pub fn from_body(body: &str) -> StartSchemaExtensionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => {
                        StartSchemaExtensionError::Client(String::from(error_message))
                    }
                    "DirectoryUnavailableException" => {
                        StartSchemaExtensionError::DirectoryUnavailable(String::from(error_message))
                    }
                    "EntityDoesNotExistException" => {
                        StartSchemaExtensionError::EntityDoesNotExist(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        StartSchemaExtensionError::InvalidParameter(String::from(error_message))
                    }
                    "ServiceException" => {
                        StartSchemaExtensionError::Service(String::from(error_message))
                    }
                    "SnapshotLimitExceededException" => StartSchemaExtensionError::SnapshotLimitExceeded(String::from(error_message)),
                    "ValidationException" => {
                        StartSchemaExtensionError::Validation(error_message.to_string())
                    }
                    _ => StartSchemaExtensionError::Unknown(String::from(body)),
                }
            }
            Err(_) => StartSchemaExtensionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StartSchemaExtensionError {
    fn from(err: serde_json::error::Error) -> StartSchemaExtensionError {
        StartSchemaExtensionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StartSchemaExtensionError {
    fn from(err: CredentialsError) -> StartSchemaExtensionError {
        StartSchemaExtensionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartSchemaExtensionError {
    fn from(err: HttpDispatchError) -> StartSchemaExtensionError {
        StartSchemaExtensionError::HttpDispatch(err)
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
            StartSchemaExtensionError::Validation(ref cause) => cause,
            StartSchemaExtensionError::Credentials(ref err) => err.description(),
            StartSchemaExtensionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StartSchemaExtensionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateConditionalForwarder
#[derive(Debug, PartialEq)]
pub enum UpdateConditionalForwarderError {
    ///<p>A client exception has occurred.</p>
    Client(String),
    ///<p>The specified directory is unavailable or could not be found.</p>
    DirectoryUnavailable(String),
    ///<p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    ///<p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    ///<p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    ///<p>The operation is not supported.</p>
    UnsupportedOperation(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateConditionalForwarderError {
    pub fn from_body(body: &str) -> UpdateConditionalForwarderError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => {
                        UpdateConditionalForwarderError::Client(String::from(error_message))
                    }
                    "DirectoryUnavailableException" => UpdateConditionalForwarderError::DirectoryUnavailable(String::from(error_message)),
                    "EntityDoesNotExistException" => UpdateConditionalForwarderError::EntityDoesNotExist(String::from(error_message)),
                    "InvalidParameterException" => UpdateConditionalForwarderError::InvalidParameter(String::from(error_message)),
                    "ServiceException" => {
                        UpdateConditionalForwarderError::Service(String::from(error_message))
                    }
                    "UnsupportedOperationException" => UpdateConditionalForwarderError::UnsupportedOperation(String::from(error_message)),
                    "ValidationException" => {
                        UpdateConditionalForwarderError::Validation(error_message.to_string())
                    }
                    _ => UpdateConditionalForwarderError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateConditionalForwarderError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateConditionalForwarderError {
    fn from(err: serde_json::error::Error) -> UpdateConditionalForwarderError {
        UpdateConditionalForwarderError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateConditionalForwarderError {
    fn from(err: CredentialsError) -> UpdateConditionalForwarderError {
        UpdateConditionalForwarderError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateConditionalForwarderError {
    fn from(err: HttpDispatchError) -> UpdateConditionalForwarderError {
        UpdateConditionalForwarderError::HttpDispatch(err)
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
            UpdateConditionalForwarderError::Validation(ref cause) => cause,
            UpdateConditionalForwarderError::Credentials(ref err) => err.description(),
            UpdateConditionalForwarderError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateConditionalForwarderError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateRadius
#[derive(Debug, PartialEq)]
pub enum UpdateRadiusError {
    ///<p>A client exception has occurred.</p>
    Client(String),
    ///<p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    ///<p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    ///<p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateRadiusError {
    pub fn from_body(body: &str) -> UpdateRadiusError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => UpdateRadiusError::Client(String::from(error_message)),
                    "EntityDoesNotExistException" => {
                        UpdateRadiusError::EntityDoesNotExist(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        UpdateRadiusError::InvalidParameter(String::from(error_message))
                    }
                    "ServiceException" => UpdateRadiusError::Service(String::from(error_message)),
                    "ValidationException" => {
                        UpdateRadiusError::Validation(error_message.to_string())
                    }
                    _ => UpdateRadiusError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateRadiusError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateRadiusError {
    fn from(err: serde_json::error::Error) -> UpdateRadiusError {
        UpdateRadiusError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateRadiusError {
    fn from(err: CredentialsError) -> UpdateRadiusError {
        UpdateRadiusError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateRadiusError {
    fn from(err: HttpDispatchError) -> UpdateRadiusError {
        UpdateRadiusError::HttpDispatch(err)
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
            UpdateRadiusError::Validation(ref cause) => cause,
            UpdateRadiusError::Credentials(ref err) => err.description(),
            UpdateRadiusError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateRadiusError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by VerifyTrust
#[derive(Debug, PartialEq)]
pub enum VerifyTrustError {
    ///<p>A client exception has occurred.</p>
    Client(String),
    ///<p>The specified entity could not be found.</p>
    EntityDoesNotExist(String),
    ///<p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    ///<p>An exception has occurred in AWS Directory Service.</p>
    Service(String),
    ///<p>The operation is not supported.</p>
    UnsupportedOperation(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl VerifyTrustError {
    pub fn from_body(body: &str) -> VerifyTrustError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => VerifyTrustError::Client(String::from(error_message)),
                    "EntityDoesNotExistException" => {
                        VerifyTrustError::EntityDoesNotExist(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        VerifyTrustError::InvalidParameter(String::from(error_message))
                    }
                    "ServiceException" => VerifyTrustError::Service(String::from(error_message)),
                    "UnsupportedOperationException" => {
                        VerifyTrustError::UnsupportedOperation(String::from(error_message))
                    }
                    "ValidationException" => {
                        VerifyTrustError::Validation(error_message.to_string())
                    }
                    _ => VerifyTrustError::Unknown(String::from(body)),
                }
            }
            Err(_) => VerifyTrustError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for VerifyTrustError {
    fn from(err: serde_json::error::Error) -> VerifyTrustError {
        VerifyTrustError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for VerifyTrustError {
    fn from(err: CredentialsError) -> VerifyTrustError {
        VerifyTrustError::Credentials(err)
    }
}
impl From<HttpDispatchError> for VerifyTrustError {
    fn from(err: HttpDispatchError) -> VerifyTrustError {
        VerifyTrustError::HttpDispatch(err)
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
            VerifyTrustError::Validation(ref cause) => cause,
            VerifyTrustError::Credentials(ref err) => err.description(),
            VerifyTrustError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            VerifyTrustError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Directory Service API. Directory Service clients implement this trait.
pub trait DirectoryService {
    #[doc="<p>If the DNS server for your on-premises domain uses a publicly addressable IP address, you must add a CIDR address block to correctly route traffic to and from your Microsoft AD on Amazon Web Services. <i>AddIpRoutes</i> adds this address block. You can also use <i>AddIpRoutes</i> to facilitate routing traffic that uses public IP ranges from your Microsoft AD on AWS to a peer VPC. </p> <p>Before you call <i>AddIpRoutes</i>, ensure that all of the required permissions have been explicitly granted through a policy. For details about what permissions are required to run the <i>AddIpRoutes</i> operation, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/UsingWithDS_IAM_ResourcePermissions.html\">AWS Directory Service API Permissions: Actions, Resources, and Conditions Reference</a>.</p>"]
    fn add_ip_routes(&self,
                     input: &AddIpRoutesRequest)
                     -> Result<AddIpRoutesResult, AddIpRoutesError>;


    #[doc="<p>Adds or overwrites one or more tags for the specified directory. Each directory can have a maximum of 50 tags. Each tag consists of a key and optional value. Tag keys must be unique to each resource.</p>"]
    fn add_tags_to_resource(&self,
                            input: &AddTagsToResourceRequest)
                            -> Result<AddTagsToResourceResult, AddTagsToResourceError>;


    #[doc="<p>Cancels an in-progress schema extension to a Microsoft AD directory. Once a schema extension has started replicating to all domain controllers, the task can no longer be canceled. A schema extension can be canceled during any of the following states; <code>Initializing</code>, <code>CreatingSnapshot</code>, and <code>UpdatingSchema</code>.</p>"]
    fn cancel_schema_extension
        (&self,
         input: &CancelSchemaExtensionRequest)
         -> Result<CancelSchemaExtensionResult, CancelSchemaExtensionError>;


    #[doc="<p>Creates an AD Connector to connect to an on-premises directory.</p> <p>Before you call <i>ConnectDirectory</i>, ensure that all of the required permissions have been explicitly granted through a policy. For details about what permissions are required to run the <i>ConnectDirectory</i> operation, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/UsingWithDS_IAM_ResourcePermissions.html\">AWS Directory Service API Permissions: Actions, Resources, and Conditions Reference</a>.</p>"]
    fn connect_directory(&self,
                         input: &ConnectDirectoryRequest)
                         -> Result<ConnectDirectoryResult, ConnectDirectoryError>;


    #[doc="<p>Creates an alias for a directory and assigns the alias to the directory. The alias is used to construct the access URL for the directory, such as <code>http://&lt;alias&gt;.awsapps.com</code>.</p> <important> <p>After an alias has been created, it cannot be deleted or reused, so this operation should only be used when absolutely necessary.</p> </important>"]
    fn create_alias(&self,
                    input: &CreateAliasRequest)
                    -> Result<CreateAliasResult, CreateAliasError>;


    #[doc="<p>Creates a computer account in the specified directory, and joins the computer to the directory.</p>"]
    fn create_computer(&self,
                       input: &CreateComputerRequest)
                       -> Result<CreateComputerResult, CreateComputerError>;


    #[doc="<p>Creates a conditional forwarder associated with your AWS directory. Conditional forwarders are required in order to set up a trust relationship with another domain. The conditional forwarder points to the trusted domain.</p>"]
    fn create_conditional_forwarder
        (&self,
         input: &CreateConditionalForwarderRequest)
         -> Result<CreateConditionalForwarderResult, CreateConditionalForwarderError>;


    #[doc="<p>Creates a Simple AD directory.</p> <p>Before you call <i>CreateDirectory</i>, ensure that all of the required permissions have been explicitly granted through a policy. For details about what permissions are required to run the <i>CreateDirectory</i> operation, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/UsingWithDS_IAM_ResourcePermissions.html\">AWS Directory Service API Permissions: Actions, Resources, and Conditions Reference</a>.</p>"]
    fn create_directory(&self,
                        input: &CreateDirectoryRequest)
                        -> Result<CreateDirectoryResult, CreateDirectoryError>;


    #[doc="<p>Creates a Microsoft AD in the AWS cloud.</p> <p>Before you call <i>CreateMicrosoftAD</i>, ensure that all of the required permissions have been explicitly granted through a policy. For details about what permissions are required to run the <i>CreateMicrosoftAD</i> operation, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/UsingWithDS_IAM_ResourcePermissions.html\">AWS Directory Service API Permissions: Actions, Resources, and Conditions Reference</a>.</p>"]
    fn create_microsoft_ad(&self,
                           input: &CreateMicrosoftADRequest)
                           -> Result<CreateMicrosoftADResult, CreateMicrosoftADError>;


    #[doc="<p>Creates a snapshot of a Simple AD or Microsoft AD directory in the AWS cloud.</p> <note> <p>You cannot take snapshots of AD Connector directories.</p> </note>"]
    fn create_snapshot(&self,
                       input: &CreateSnapshotRequest)
                       -> Result<CreateSnapshotResult, CreateSnapshotError>;


    #[doc="<p>AWS Directory Service for Microsoft Active Directory allows you to configure trust relationships. For example, you can establish a trust between your Microsoft AD in the AWS cloud, and your existing on-premises Microsoft Active Directory. This would allow you to provide users and groups access to resources in either domain, with a single set of credentials.</p> <p>This action initiates the creation of the AWS side of a trust relationship between a Microsoft AD in the AWS cloud and an external domain.</p>"]
    fn create_trust(&self,
                    input: &CreateTrustRequest)
                    -> Result<CreateTrustResult, CreateTrustError>;


    #[doc="<p>Deletes a conditional forwarder that has been set up for your AWS directory.</p>"]
    fn delete_conditional_forwarder
        (&self,
         input: &DeleteConditionalForwarderRequest)
         -> Result<DeleteConditionalForwarderResult, DeleteConditionalForwarderError>;


    #[doc="<p>Deletes an AWS Directory Service directory.</p> <p>Before you call <i>DeleteDirectory</i>, ensure that all of the required permissions have been explicitly granted through a policy. For details about what permissions are required to run the <i>DeleteDirectory</i> operation, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/UsingWithDS_IAM_ResourcePermissions.html\">AWS Directory Service API Permissions: Actions, Resources, and Conditions Reference</a>.</p>"]
    fn delete_directory(&self,
                        input: &DeleteDirectoryRequest)
                        -> Result<DeleteDirectoryResult, DeleteDirectoryError>;


    #[doc="<p>Deletes a directory snapshot.</p>"]
    fn delete_snapshot(&self,
                       input: &DeleteSnapshotRequest)
                       -> Result<DeleteSnapshotResult, DeleteSnapshotError>;


    #[doc="<p>Deletes an existing trust relationship between your Microsoft AD in the AWS cloud and an external domain.</p>"]
    fn delete_trust(&self,
                    input: &DeleteTrustRequest)
                    -> Result<DeleteTrustResult, DeleteTrustError>;


    #[doc="<p>Removes the specified directory as a publisher to the specified SNS topic.</p>"]
    fn deregister_event_topic(&self,
                              input: &DeregisterEventTopicRequest)
                              -> Result<DeregisterEventTopicResult, DeregisterEventTopicError>;


    #[doc="<p>Obtains information about the conditional forwarders for this account.</p> <p>If no input parameters are provided for RemoteDomainNames, this request describes all conditional forwarders for the specified directory ID.</p>"]
    fn describe_conditional_forwarders
        (&self,
         input: &DescribeConditionalForwardersRequest)
         -> Result<DescribeConditionalForwardersResult, DescribeConditionalForwardersError>;


    #[doc="<p>Obtains information about the directories that belong to this account.</p> <p>You can retrieve information about specific directories by passing the directory identifiers in the <i>DirectoryIds</i> parameter. Otherwise, all directories that belong to the current account are returned.</p> <p>This operation supports pagination with the use of the <i>NextToken</i> request and response parameters. If more results are available, the <i>DescribeDirectoriesResult.NextToken</i> member contains a token that you pass in the next call to <a>DescribeDirectories</a> to retrieve the next set of items.</p> <p>You can also specify a maximum number of return results with the <i>Limit</i> parameter.</p>"]
    fn describe_directories(&self,
                            input: &DescribeDirectoriesRequest)
                            -> Result<DescribeDirectoriesResult, DescribeDirectoriesError>;


    #[doc="<p>Obtains information about which SNS topics receive status messages from the specified directory.</p> <p>If no input parameters are provided, such as DirectoryId or TopicName, this request describes all of the associations in the account.</p>"]
    fn describe_event_topics(&self,
                             input: &DescribeEventTopicsRequest)
                             -> Result<DescribeEventTopicsResult, DescribeEventTopicsError>;


    #[doc="<p>Obtains information about the directory snapshots that belong to this account.</p> <p>This operation supports pagination with the use of the <i>NextToken</i> request and response parameters. If more results are available, the <i>DescribeSnapshots.NextToken</i> member contains a token that you pass in the next call to <a>DescribeSnapshots</a> to retrieve the next set of items.</p> <p>You can also specify a maximum number of return results with the <i>Limit</i> parameter.</p>"]
    fn describe_snapshots(&self,
                          input: &DescribeSnapshotsRequest)
                          -> Result<DescribeSnapshotsResult, DescribeSnapshotsError>;


    #[doc="<p>Obtains information about the trust relationships for this account.</p> <p>If no input parameters are provided, such as DirectoryId or TrustIds, this request describes all the trust relationships belonging to the account.</p>"]
    fn describe_trusts(&self,
                       input: &DescribeTrustsRequest)
                       -> Result<DescribeTrustsResult, DescribeTrustsError>;


    #[doc="<p>Disables multi-factor authentication (MFA) with the Remote Authentication Dial In User Service (RADIUS) server for an AD Connector directory.</p>"]
    fn disable_radius(&self,
                      input: &DisableRadiusRequest)
                      -> Result<DisableRadiusResult, DisableRadiusError>;


    #[doc="<p>Disables single-sign on for a directory.</p>"]
    fn disable_sso(&self, input: &DisableSsoRequest) -> Result<DisableSsoResult, DisableSsoError>;


    #[doc="<p>Enables multi-factor authentication (MFA) with the Remote Authentication Dial In User Service (RADIUS) server for an AD Connector directory.</p>"]
    fn enable_radius(&self,
                     input: &EnableRadiusRequest)
                     -> Result<EnableRadiusResult, EnableRadiusError>;


    #[doc="<p>Enables single sign-on for a directory.</p>"]
    fn enable_sso(&self, input: &EnableSsoRequest) -> Result<EnableSsoResult, EnableSsoError>;


    #[doc="<p>Obtains directory limit information for the current region.</p>"]
    fn get_directory_limits(&self,
                            input: &GetDirectoryLimitsRequest)
                            -> Result<GetDirectoryLimitsResult, GetDirectoryLimitsError>;


    #[doc="<p>Obtains the manual snapshot limits for a directory.</p>"]
    fn get_snapshot_limits(&self,
                           input: &GetSnapshotLimitsRequest)
                           -> Result<GetSnapshotLimitsResult, GetSnapshotLimitsError>;


    #[doc="<p>Lists the address blocks that you have added to a directory.</p>"]
    fn list_ip_routes(&self,
                      input: &ListIpRoutesRequest)
                      -> Result<ListIpRoutesResult, ListIpRoutesError>;


    #[doc="<p>Lists all schema extensions applied to a Microsoft AD Directory.</p>"]
    fn list_schema_extensions(&self,
                              input: &ListSchemaExtensionsRequest)
                              -> Result<ListSchemaExtensionsResult, ListSchemaExtensionsError>;


    #[doc="<p>Lists all tags on a directory.</p>"]
    fn list_tags_for_resource(&self,
                              input: &ListTagsForResourceRequest)
                              -> Result<ListTagsForResourceResult, ListTagsForResourceError>;


    #[doc="<p>Associates a directory with an SNS topic. This establishes the directory as a publisher to the specified SNS topic. You can then receive email or text (SMS) messages when the status of your directory changes. You get notified if your directory goes from an Active status to an Impaired or Inoperable status. You also receive a notification when the directory returns to an Active status.</p>"]
    fn register_event_topic(&self,
                            input: &RegisterEventTopicRequest)
                            -> Result<RegisterEventTopicResult, RegisterEventTopicError>;


    #[doc="<p>Removes IP address blocks from a directory.</p>"]
    fn remove_ip_routes(&self,
                        input: &RemoveIpRoutesRequest)
                        -> Result<RemoveIpRoutesResult, RemoveIpRoutesError>;


    #[doc="<p>Removes tags from a directory.</p>"]
    fn remove_tags_from_resource
        (&self,
         input: &RemoveTagsFromResourceRequest)
         -> Result<RemoveTagsFromResourceResult, RemoveTagsFromResourceError>;


    #[doc="<p>Restores a directory using an existing directory snapshot.</p> <p>When you restore a directory from a snapshot, any changes made to the directory after the snapshot date are overwritten.</p> <p>This action returns as soon as the restore operation is initiated. You can monitor the progress of the restore operation by calling the <a>DescribeDirectories</a> operation with the directory identifier. When the <b>DirectoryDescription.Stage</b> value changes to <code>Active</code>, the restore operation is complete.</p>"]
    fn restore_from_snapshot(&self,
                             input: &RestoreFromSnapshotRequest)
                             -> Result<RestoreFromSnapshotResult, RestoreFromSnapshotError>;


    #[doc="<p>Applies a schema extension to a Microsoft AD directory.</p>"]
    fn start_schema_extension(&self,
                              input: &StartSchemaExtensionRequest)
                              -> Result<StartSchemaExtensionResult, StartSchemaExtensionError>;


    #[doc="<p>Updates a conditional forwarder that has been set up for your AWS directory.</p>"]
    fn update_conditional_forwarder
        (&self,
         input: &UpdateConditionalForwarderRequest)
         -> Result<UpdateConditionalForwarderResult, UpdateConditionalForwarderError>;


    #[doc="<p>Updates the Remote Authentication Dial In User Service (RADIUS) server information for an AD Connector directory.</p>"]
    fn update_radius(&self,
                     input: &UpdateRadiusRequest)
                     -> Result<UpdateRadiusResult, UpdateRadiusError>;


    #[doc="<p>AWS Directory Service for Microsoft Active Directory allows you to configure and verify trust relationships.</p> <p>This action verifies a trust relationship between your Microsoft AD in the AWS cloud and an external domain.</p>"]
    fn verify_trust(&self,
                    input: &VerifyTrustRequest)
                    -> Result<VerifyTrustResult, VerifyTrustError>;
}
/// A client for the Directory Service API.
pub struct DirectoryServiceClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> DirectoryServiceClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        DirectoryServiceClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> DirectoryService for DirectoryServiceClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    #[doc="<p>If the DNS server for your on-premises domain uses a publicly addressable IP address, you must add a CIDR address block to correctly route traffic to and from your Microsoft AD on Amazon Web Services. <i>AddIpRoutes</i> adds this address block. You can also use <i>AddIpRoutes</i> to facilitate routing traffic that uses public IP ranges from your Microsoft AD on AWS to a peer VPC. </p> <p>Before you call <i>AddIpRoutes</i>, ensure that all of the required permissions have been explicitly granted through a policy. For details about what permissions are required to run the <i>AddIpRoutes</i> operation, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/UsingWithDS_IAM_ResourcePermissions.html\">AWS Directory Service API Permissions: Actions, Resources, and Conditions Reference</a>.</p>"]
    fn add_ip_routes(&self,
                     input: &AddIpRoutesRequest)
                     -> Result<AddIpRoutesResult, AddIpRoutesError> {
        let mut request = SignedRequest::new("POST", "ds", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DirectoryService_20150416.AddIpRoutes");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<AddIpRoutesResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(AddIpRoutesError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Adds or overwrites one or more tags for the specified directory. Each directory can have a maximum of 50 tags. Each tag consists of a key and optional value. Tag keys must be unique to each resource.</p>"]
    fn add_tags_to_resource(&self,
                            input: &AddTagsToResourceRequest)
                            -> Result<AddTagsToResourceResult, AddTagsToResourceError> {
        let mut request = SignedRequest::new("POST", "ds", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "DirectoryService_20150416.AddTagsToResource");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<AddTagsToResourceResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(AddTagsToResourceError::from_body(String::from_utf8_lossy(&response.body)
                                                          .as_ref()))
            }
        }
    }


    #[doc="<p>Cancels an in-progress schema extension to a Microsoft AD directory. Once a schema extension has started replicating to all domain controllers, the task can no longer be canceled. A schema extension can be canceled during any of the following states; <code>Initializing</code>, <code>CreatingSnapshot</code>, and <code>UpdatingSchema</code>.</p>"]
    fn cancel_schema_extension
        (&self,
         input: &CancelSchemaExtensionRequest)
         -> Result<CancelSchemaExtensionResult, CancelSchemaExtensionError> {
        let mut request = SignedRequest::new("POST", "ds", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "DirectoryService_20150416.CancelSchemaExtension");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<CancelSchemaExtensionResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(CancelSchemaExtensionError::from_body(String::from_utf8_lossy(&response.body)
                                                              .as_ref()))
            }
        }
    }


    #[doc="<p>Creates an AD Connector to connect to an on-premises directory.</p> <p>Before you call <i>ConnectDirectory</i>, ensure that all of the required permissions have been explicitly granted through a policy. For details about what permissions are required to run the <i>ConnectDirectory</i> operation, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/UsingWithDS_IAM_ResourcePermissions.html\">AWS Directory Service API Permissions: Actions, Resources, and Conditions Reference</a>.</p>"]
    fn connect_directory(&self,
                         input: &ConnectDirectoryRequest)
                         -> Result<ConnectDirectoryResult, ConnectDirectoryError> {
        let mut request = SignedRequest::new("POST", "ds", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DirectoryService_20150416.ConnectDirectory");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<ConnectDirectoryResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(ConnectDirectoryError::from_body(String::from_utf8_lossy(&response.body)
                                                         .as_ref()))
            }
        }
    }


    #[doc="<p>Creates an alias for a directory and assigns the alias to the directory. The alias is used to construct the access URL for the directory, such as <code>http://&lt;alias&gt;.awsapps.com</code>.</p> <important> <p>After an alias has been created, it cannot be deleted or reused, so this operation should only be used when absolutely necessary.</p> </important>"]
    fn create_alias(&self,
                    input: &CreateAliasRequest)
                    -> Result<CreateAliasResult, CreateAliasError> {
        let mut request = SignedRequest::new("POST", "ds", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DirectoryService_20150416.CreateAlias");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateAliasResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(CreateAliasError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Creates a computer account in the specified directory, and joins the computer to the directory.</p>"]
    fn create_computer(&self,
                       input: &CreateComputerRequest)
                       -> Result<CreateComputerResult, CreateComputerError> {
        let mut request = SignedRequest::new("POST", "ds", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DirectoryService_20150416.CreateComputer");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateComputerResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(CreateComputerError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Creates a conditional forwarder associated with your AWS directory. Conditional forwarders are required in order to set up a trust relationship with another domain. The conditional forwarder points to the trusted domain.</p>"]
    fn create_conditional_forwarder
        (&self,
         input: &CreateConditionalForwarderRequest)
         -> Result<CreateConditionalForwarderResult, CreateConditionalForwarderError> {
        let mut request = SignedRequest::new("POST", "ds", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "DirectoryService_20150416.CreateConditionalForwarder");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateConditionalForwarderResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(CreateConditionalForwarderError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Creates a Simple AD directory.</p> <p>Before you call <i>CreateDirectory</i>, ensure that all of the required permissions have been explicitly granted through a policy. For details about what permissions are required to run the <i>CreateDirectory</i> operation, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/UsingWithDS_IAM_ResourcePermissions.html\">AWS Directory Service API Permissions: Actions, Resources, and Conditions Reference</a>.</p>"]
    fn create_directory(&self,
                        input: &CreateDirectoryRequest)
                        -> Result<CreateDirectoryResult, CreateDirectoryError> {
        let mut request = SignedRequest::new("POST", "ds", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DirectoryService_20150416.CreateDirectory");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateDirectoryResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(CreateDirectoryError::from_body(String::from_utf8_lossy(&response.body)
                                                        .as_ref()))
            }
        }
    }


    #[doc="<p>Creates a Microsoft AD in the AWS cloud.</p> <p>Before you call <i>CreateMicrosoftAD</i>, ensure that all of the required permissions have been explicitly granted through a policy. For details about what permissions are required to run the <i>CreateMicrosoftAD</i> operation, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/UsingWithDS_IAM_ResourcePermissions.html\">AWS Directory Service API Permissions: Actions, Resources, and Conditions Reference</a>.</p>"]
    fn create_microsoft_ad(&self,
                           input: &CreateMicrosoftADRequest)
                           -> Result<CreateMicrosoftADResult, CreateMicrosoftADError> {
        let mut request = SignedRequest::new("POST", "ds", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "DirectoryService_20150416.CreateMicrosoftAD");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateMicrosoftADResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(CreateMicrosoftADError::from_body(String::from_utf8_lossy(&response.body)
                                                          .as_ref()))
            }
        }
    }


    #[doc="<p>Creates a snapshot of a Simple AD or Microsoft AD directory in the AWS cloud.</p> <note> <p>You cannot take snapshots of AD Connector directories.</p> </note>"]
    fn create_snapshot(&self,
                       input: &CreateSnapshotRequest)
                       -> Result<CreateSnapshotResult, CreateSnapshotError> {
        let mut request = SignedRequest::new("POST", "ds", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DirectoryService_20150416.CreateSnapshot");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateSnapshotResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(CreateSnapshotError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>AWS Directory Service for Microsoft Active Directory allows you to configure trust relationships. For example, you can establish a trust between your Microsoft AD in the AWS cloud, and your existing on-premises Microsoft Active Directory. This would allow you to provide users and groups access to resources in either domain, with a single set of credentials.</p> <p>This action initiates the creation of the AWS side of a trust relationship between a Microsoft AD in the AWS cloud and an external domain.</p>"]
    fn create_trust(&self,
                    input: &CreateTrustRequest)
                    -> Result<CreateTrustResult, CreateTrustError> {
        let mut request = SignedRequest::new("POST", "ds", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DirectoryService_20150416.CreateTrust");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateTrustResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(CreateTrustError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Deletes a conditional forwarder that has been set up for your AWS directory.</p>"]
    fn delete_conditional_forwarder
        (&self,
         input: &DeleteConditionalForwarderRequest)
         -> Result<DeleteConditionalForwarderResult, DeleteConditionalForwarderError> {
        let mut request = SignedRequest::new("POST", "ds", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "DirectoryService_20150416.DeleteConditionalForwarder");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeleteConditionalForwarderResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DeleteConditionalForwarderError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Deletes an AWS Directory Service directory.</p> <p>Before you call <i>DeleteDirectory</i>, ensure that all of the required permissions have been explicitly granted through a policy. For details about what permissions are required to run the <i>DeleteDirectory</i> operation, see <a href=\"http://docs.aws.amazon.com/directoryservice/latest/admin-guide/UsingWithDS_IAM_ResourcePermissions.html\">AWS Directory Service API Permissions: Actions, Resources, and Conditions Reference</a>.</p>"]
    fn delete_directory(&self,
                        input: &DeleteDirectoryRequest)
                        -> Result<DeleteDirectoryResult, DeleteDirectoryError> {
        let mut request = SignedRequest::new("POST", "ds", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DirectoryService_20150416.DeleteDirectory");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeleteDirectoryResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DeleteDirectoryError::from_body(String::from_utf8_lossy(&response.body)
                                                        .as_ref()))
            }
        }
    }


    #[doc="<p>Deletes a directory snapshot.</p>"]
    fn delete_snapshot(&self,
                       input: &DeleteSnapshotRequest)
                       -> Result<DeleteSnapshotResult, DeleteSnapshotError> {
        let mut request = SignedRequest::new("POST", "ds", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DirectoryService_20150416.DeleteSnapshot");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeleteSnapshotResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DeleteSnapshotError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Deletes an existing trust relationship between your Microsoft AD in the AWS cloud and an external domain.</p>"]
    fn delete_trust(&self,
                    input: &DeleteTrustRequest)
                    -> Result<DeleteTrustResult, DeleteTrustError> {
        let mut request = SignedRequest::new("POST", "ds", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DirectoryService_20150416.DeleteTrust");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeleteTrustResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DeleteTrustError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Removes the specified directory as a publisher to the specified SNS topic.</p>"]
    fn deregister_event_topic(&self,
                              input: &DeregisterEventTopicRequest)
                              -> Result<DeregisterEventTopicResult, DeregisterEventTopicError> {
        let mut request = SignedRequest::new("POST", "ds", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "DirectoryService_20150416.DeregisterEventTopic");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeregisterEventTopicResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DeregisterEventTopicError::from_body(String::from_utf8_lossy(&response.body)
                                                             .as_ref()))
            }
        }
    }


    #[doc="<p>Obtains information about the conditional forwarders for this account.</p> <p>If no input parameters are provided for RemoteDomainNames, this request describes all conditional forwarders for the specified directory ID.</p>"]
    fn describe_conditional_forwarders
        (&self,
         input: &DescribeConditionalForwardersRequest)
         -> Result<DescribeConditionalForwardersResult, DescribeConditionalForwardersError> {
        let mut request = SignedRequest::new("POST", "ds", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "DirectoryService_20150416.DescribeConditionalForwarders");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeConditionalForwardersResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DescribeConditionalForwardersError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Obtains information about the directories that belong to this account.</p> <p>You can retrieve information about specific directories by passing the directory identifiers in the <i>DirectoryIds</i> parameter. Otherwise, all directories that belong to the current account are returned.</p> <p>This operation supports pagination with the use of the <i>NextToken</i> request and response parameters. If more results are available, the <i>DescribeDirectoriesResult.NextToken</i> member contains a token that you pass in the next call to <a>DescribeDirectories</a> to retrieve the next set of items.</p> <p>You can also specify a maximum number of return results with the <i>Limit</i> parameter.</p>"]
    fn describe_directories(&self,
                            input: &DescribeDirectoriesRequest)
                            -> Result<DescribeDirectoriesResult, DescribeDirectoriesError> {
        let mut request = SignedRequest::new("POST", "ds", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "DirectoryService_20150416.DescribeDirectories");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeDirectoriesResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeDirectoriesError::from_body(String::from_utf8_lossy(&response.body)
                                                            .as_ref()))
            }
        }
    }


    #[doc="<p>Obtains information about which SNS topics receive status messages from the specified directory.</p> <p>If no input parameters are provided, such as DirectoryId or TopicName, this request describes all of the associations in the account.</p>"]
    fn describe_event_topics(&self,
                             input: &DescribeEventTopicsRequest)
                             -> Result<DescribeEventTopicsResult, DescribeEventTopicsError> {
        let mut request = SignedRequest::new("POST", "ds", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "DirectoryService_20150416.DescribeEventTopics");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeEventTopicsResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeEventTopicsError::from_body(String::from_utf8_lossy(&response.body)
                                                            .as_ref()))
            }
        }
    }


    #[doc="<p>Obtains information about the directory snapshots that belong to this account.</p> <p>This operation supports pagination with the use of the <i>NextToken</i> request and response parameters. If more results are available, the <i>DescribeSnapshots.NextToken</i> member contains a token that you pass in the next call to <a>DescribeSnapshots</a> to retrieve the next set of items.</p> <p>You can also specify a maximum number of return results with the <i>Limit</i> parameter.</p>"]
    fn describe_snapshots(&self,
                          input: &DescribeSnapshotsRequest)
                          -> Result<DescribeSnapshotsResult, DescribeSnapshotsError> {
        let mut request = SignedRequest::new("POST", "ds", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "DirectoryService_20150416.DescribeSnapshots");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeSnapshotsResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeSnapshotsError::from_body(String::from_utf8_lossy(&response.body)
                                                          .as_ref()))
            }
        }
    }


    #[doc="<p>Obtains information about the trust relationships for this account.</p> <p>If no input parameters are provided, such as DirectoryId or TrustIds, this request describes all the trust relationships belonging to the account.</p>"]
    fn describe_trusts(&self,
                       input: &DescribeTrustsRequest)
                       -> Result<DescribeTrustsResult, DescribeTrustsError> {
        let mut request = SignedRequest::new("POST", "ds", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DirectoryService_20150416.DescribeTrusts");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeTrustsResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeTrustsError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Disables multi-factor authentication (MFA) with the Remote Authentication Dial In User Service (RADIUS) server for an AD Connector directory.</p>"]
    fn disable_radius(&self,
                      input: &DisableRadiusRequest)
                      -> Result<DisableRadiusResult, DisableRadiusError> {
        let mut request = SignedRequest::new("POST", "ds", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DirectoryService_20150416.DisableRadius");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DisableRadiusResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DisableRadiusError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Disables single-sign on for a directory.</p>"]
    fn disable_sso(&self, input: &DisableSsoRequest) -> Result<DisableSsoResult, DisableSsoError> {
        let mut request = SignedRequest::new("POST", "ds", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DirectoryService_20150416.DisableSso");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DisableSsoResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DisableSsoError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Enables multi-factor authentication (MFA) with the Remote Authentication Dial In User Service (RADIUS) server for an AD Connector directory.</p>"]
    fn enable_radius(&self,
                     input: &EnableRadiusRequest)
                     -> Result<EnableRadiusResult, EnableRadiusError> {
        let mut request = SignedRequest::new("POST", "ds", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DirectoryService_20150416.EnableRadius");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<EnableRadiusResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(EnableRadiusError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Enables single sign-on for a directory.</p>"]
    fn enable_sso(&self, input: &EnableSsoRequest) -> Result<EnableSsoResult, EnableSsoError> {
        let mut request = SignedRequest::new("POST", "ds", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DirectoryService_20150416.EnableSso");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                Ok(serde_json::from_str::<EnableSsoResult>(String::from_utf8_lossy(&response.body)
                                                               .as_ref())
                           .unwrap())
            }
            _ => Err(EnableSsoError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Obtains directory limit information for the current region.</p>"]
    fn get_directory_limits(&self,
                            input: &GetDirectoryLimitsRequest)
                            -> Result<GetDirectoryLimitsResult, GetDirectoryLimitsError> {
        let mut request = SignedRequest::new("POST", "ds", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "DirectoryService_20150416.GetDirectoryLimits");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetDirectoryLimitsResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(GetDirectoryLimitsError::from_body(String::from_utf8_lossy(&response.body)
                                                           .as_ref()))
            }
        }
    }


    #[doc="<p>Obtains the manual snapshot limits for a directory.</p>"]
    fn get_snapshot_limits(&self,
                           input: &GetSnapshotLimitsRequest)
                           -> Result<GetSnapshotLimitsResult, GetSnapshotLimitsError> {
        let mut request = SignedRequest::new("POST", "ds", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "DirectoryService_20150416.GetSnapshotLimits");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetSnapshotLimitsResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(GetSnapshotLimitsError::from_body(String::from_utf8_lossy(&response.body)
                                                          .as_ref()))
            }
        }
    }


    #[doc="<p>Lists the address blocks that you have added to a directory.</p>"]
    fn list_ip_routes(&self,
                      input: &ListIpRoutesRequest)
                      -> Result<ListIpRoutesResult, ListIpRoutesError> {
        let mut request = SignedRequest::new("POST", "ds", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DirectoryService_20150416.ListIpRoutes");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListIpRoutesResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(ListIpRoutesError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Lists all schema extensions applied to a Microsoft AD Directory.</p>"]
    fn list_schema_extensions(&self,
                              input: &ListSchemaExtensionsRequest)
                              -> Result<ListSchemaExtensionsResult, ListSchemaExtensionsError> {
        let mut request = SignedRequest::new("POST", "ds", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "DirectoryService_20150416.ListSchemaExtensions");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListSchemaExtensionsResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(ListSchemaExtensionsError::from_body(String::from_utf8_lossy(&response.body)
                                                             .as_ref()))
            }
        }
    }


    #[doc="<p>Lists all tags on a directory.</p>"]
    fn list_tags_for_resource(&self,
                              input: &ListTagsForResourceRequest)
                              -> Result<ListTagsForResourceResult, ListTagsForResourceError> {
        let mut request = SignedRequest::new("POST", "ds", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "DirectoryService_20150416.ListTagsForResource");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListTagsForResourceResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(ListTagsForResourceError::from_body(String::from_utf8_lossy(&response.body)
                                                            .as_ref()))
            }
        }
    }


    #[doc="<p>Associates a directory with an SNS topic. This establishes the directory as a publisher to the specified SNS topic. You can then receive email or text (SMS) messages when the status of your directory changes. You get notified if your directory goes from an Active status to an Impaired or Inoperable status. You also receive a notification when the directory returns to an Active status.</p>"]
    fn register_event_topic(&self,
                            input: &RegisterEventTopicRequest)
                            -> Result<RegisterEventTopicResult, RegisterEventTopicError> {
        let mut request = SignedRequest::new("POST", "ds", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "DirectoryService_20150416.RegisterEventTopic");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<RegisterEventTopicResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(RegisterEventTopicError::from_body(String::from_utf8_lossy(&response.body)
                                                           .as_ref()))
            }
        }
    }


    #[doc="<p>Removes IP address blocks from a directory.</p>"]
    fn remove_ip_routes(&self,
                        input: &RemoveIpRoutesRequest)
                        -> Result<RemoveIpRoutesResult, RemoveIpRoutesError> {
        let mut request = SignedRequest::new("POST", "ds", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DirectoryService_20150416.RemoveIpRoutes");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<RemoveIpRoutesResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(RemoveIpRoutesError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Removes tags from a directory.</p>"]
    fn remove_tags_from_resource
        (&self,
         input: &RemoveTagsFromResourceRequest)
         -> Result<RemoveTagsFromResourceResult, RemoveTagsFromResourceError> {
        let mut request = SignedRequest::new("POST", "ds", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "DirectoryService_20150416.RemoveTagsFromResource");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<RemoveTagsFromResourceResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(RemoveTagsFromResourceError::from_body(String::from_utf8_lossy(&response.body)
                                                               .as_ref()))
            }
        }
    }


    #[doc="<p>Restores a directory using an existing directory snapshot.</p> <p>When you restore a directory from a snapshot, any changes made to the directory after the snapshot date are overwritten.</p> <p>This action returns as soon as the restore operation is initiated. You can monitor the progress of the restore operation by calling the <a>DescribeDirectories</a> operation with the directory identifier. When the <b>DirectoryDescription.Stage</b> value changes to <code>Active</code>, the restore operation is complete.</p>"]
    fn restore_from_snapshot(&self,
                             input: &RestoreFromSnapshotRequest)
                             -> Result<RestoreFromSnapshotResult, RestoreFromSnapshotError> {
        let mut request = SignedRequest::new("POST", "ds", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "DirectoryService_20150416.RestoreFromSnapshot");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<RestoreFromSnapshotResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(RestoreFromSnapshotError::from_body(String::from_utf8_lossy(&response.body)
                                                            .as_ref()))
            }
        }
    }


    #[doc="<p>Applies a schema extension to a Microsoft AD directory.</p>"]
    fn start_schema_extension(&self,
                              input: &StartSchemaExtensionRequest)
                              -> Result<StartSchemaExtensionResult, StartSchemaExtensionError> {
        let mut request = SignedRequest::new("POST", "ds", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "DirectoryService_20150416.StartSchemaExtension");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<StartSchemaExtensionResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(StartSchemaExtensionError::from_body(String::from_utf8_lossy(&response.body)
                                                             .as_ref()))
            }
        }
    }


    #[doc="<p>Updates a conditional forwarder that has been set up for your AWS directory.</p>"]
    fn update_conditional_forwarder
        (&self,
         input: &UpdateConditionalForwarderRequest)
         -> Result<UpdateConditionalForwarderResult, UpdateConditionalForwarderError> {
        let mut request = SignedRequest::new("POST", "ds", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "DirectoryService_20150416.UpdateConditionalForwarder");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<UpdateConditionalForwarderResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(UpdateConditionalForwarderError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Updates the Remote Authentication Dial In User Service (RADIUS) server information for an AD Connector directory.</p>"]
    fn update_radius(&self,
                     input: &UpdateRadiusRequest)
                     -> Result<UpdateRadiusResult, UpdateRadiusError> {
        let mut request = SignedRequest::new("POST", "ds", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DirectoryService_20150416.UpdateRadius");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<UpdateRadiusResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(UpdateRadiusError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>AWS Directory Service for Microsoft Active Directory allows you to configure and verify trust relationships.</p> <p>This action verifies a trust relationship between your Microsoft AD in the AWS cloud and an external domain.</p>"]
    fn verify_trust(&self,
                    input: &VerifyTrustRequest)
                    -> Result<VerifyTrustResult, VerifyTrustError> {
        let mut request = SignedRequest::new("POST", "ds", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DirectoryService_20150416.VerifyTrust");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<VerifyTrustResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(VerifyTrustError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }
}

#[cfg(test)]
mod protocol_tests {}
