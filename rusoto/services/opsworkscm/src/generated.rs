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
/// <p>Stores account attributes. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AccountAttribute {
    /// <p> The maximum allowed value. </p>
    #[serde(rename = "Maximum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,
    /// <p><p> The attribute name. The following are supported attribute names. </p> <ul> <li> <p> <i>ServerLimit:</i> The number of current servers/maximum number of servers allowed. By default, you can have a maximum of 10 servers. </p> </li> <li> <p> <i>ManualBackupLimit:</i> The number of current manual backups/maximum number of backups allowed. By default, you can have a maximum of 50 manual backups saved. </p> </li> </ul></p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p> The current usage, such as the current number of servers that are associated with the account. </p>
    #[serde(rename = "Used")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateNodeRequest {
    /// <p><p>Engine attributes used for associating the node. </p> <p class="title"> <b>Attributes accepted in a AssociateNode request for Chef</b> </p> <ul> <li> <p> <code>CHEF<em>ORGANIZATION</code>: The Chef organization with which the node is associated. By default only one organization named <code>default</code> can exist. </p> </li> <li> <p> <code>CHEF</em>NODE<em>PUBLIC</em>KEY</code>: A PEM-formatted public key. This key is required for the <code>chef-client</code> agent to access the Chef API. </p> </li> </ul> <p class="title"> <b>Attributes accepted in a AssociateNode request for Puppet</b> </p> <ul> <li> <p> <code>PUPPET<em>NODE</em>CSR</code>: A PEM-formatted certificate-signing request (CSR) that is created by the node. </p> </li> </ul></p>
    #[serde(rename = "EngineAttributes")]
    pub engine_attributes: Vec<EngineAttribute>,
    /// <p>The name of the node. </p>
    #[serde(rename = "NodeName")]
    pub node_name: String,
    /// <p>The name of the server with which to associate the node. </p>
    #[serde(rename = "ServerName")]
    pub server_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateNodeResponse {
    /// <p>Contains a token which can be passed to the <code>DescribeNodeAssociationStatus</code> API call to get the status of the association request. </p>
    #[serde(rename = "NodeAssociationStatusToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_association_status_token: Option<String>,
}

/// <p>Describes a single backup. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Backup {
    /// <p>The ARN of the backup. </p>
    #[serde(rename = "BackupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_arn: Option<String>,
    /// <p> The generated ID of the backup. Example: <code>myServerName-yyyyMMddHHmmssSSS</code> </p>
    #[serde(rename = "BackupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_id: Option<String>,
    /// <p> The backup type. Valid values are <code>automated</code> or <code>manual</code>. </p>
    #[serde(rename = "BackupType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_type: Option<String>,
    /// <p> The time stamp when the backup was created in the database. Example: <code>2016-07-29T13:38:47.520Z</code> </p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p> A user-provided description for a manual backup. This field is empty for automated backups. </p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p> The engine type that is obtained from the server when the backup is created. </p>
    #[serde(rename = "Engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    /// <p> The engine model that is obtained from the server when the backup is created. </p>
    #[serde(rename = "EngineModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_model: Option<String>,
    /// <p> The engine version that is obtained from the server when the backup is created. </p>
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// <p> The EC2 instance profile ARN that is obtained from the server when the backup is created. Because this value is stored, you are not required to provide the InstanceProfileArn again if you restore a backup. </p>
    #[serde(rename = "InstanceProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_profile_arn: Option<String>,
    /// <p> The instance type that is obtained from the server when the backup is created. </p>
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p> The key pair that is obtained from the server when the backup is created. </p>
    #[serde(rename = "KeyPair")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pair: Option<String>,
    /// <p> The preferred backup period that is obtained from the server when the backup is created. </p>
    #[serde(rename = "PreferredBackupWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_window: Option<String>,
    /// <p> The preferred maintenance period that is obtained from the server when the backup is created. </p>
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    /// <p> The Amazon S3 URL of the backup's log file. </p>
    #[serde(rename = "S3LogUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_log_url: Option<String>,
    /// <p> The security group IDs that are obtained from the server when the backup is created. </p>
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p> The name of the server from which the backup was made. </p>
    #[serde(rename = "ServerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    /// <p> The service role ARN that is obtained from the server when the backup is created. </p>
    #[serde(rename = "ServiceRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role_arn: Option<String>,
    /// <p>The status of a backup while in progress. </p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p> An informational message about backup status. </p>
    #[serde(rename = "StatusDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_description: Option<String>,
    /// <p> The subnet IDs that are obtained from the server when the backup is created. </p>
    #[serde(rename = "SubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    /// <p> The version of AWS OpsWorks CM-specific tools that is obtained from the server when the backup is created. </p>
    #[serde(rename = "ToolsVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools_version: Option<String>,
    /// <p> The IAM user ARN of the requester for manual backups. This field is empty for automated backups. </p>
    #[serde(rename = "UserArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateBackupRequest {
    /// <p> A user-defined description of the backup. </p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the server that you want to back up. </p>
    #[serde(rename = "ServerName")]
    pub server_name: String,
    /// <p><p>A map that contains tag keys and tag values to attach to an AWS OpsWorks-CM server backup.</p> <ul> <li> <p>The key cannot be empty.</p> </li> <li> <p>The key can be a maximum of 127 characters, and can contain only Unicode letters, numbers, or separators, or the following special characters: <code>+ - = . _ : /</code> </p> </li> <li> <p>The value can be a maximum 255 characters, and contain only Unicode letters, numbers, or separators, or the following special characters: <code>+ - = . _ : /</code> </p> </li> <li> <p>Leading and trailing white spaces are trimmed from both the key and value.</p> </li> <li> <p>A maximum of 50 user-applied tags is allowed for tag-supported AWS OpsWorks-CM resources.</p> </li> </ul></p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateBackupResponse {
    /// <p>Backup created by request.</p>
    #[serde(rename = "Backup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup: Option<Backup>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateServerRequest {
    /// <p> Associate a public IP address with a server that you are launching. Valid values are <code>true</code> or <code>false</code>. The default value is <code>true</code>. </p>
    #[serde(rename = "AssociatePublicIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associate_public_ip_address: Option<bool>,
    /// <p> If you specify this field, AWS OpsWorks CM creates the server by using the backup represented by BackupId. </p>
    #[serde(rename = "BackupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_id: Option<String>,
    /// <p> The number of automated backups that you want to keep. Whenever a new backup is created, AWS OpsWorks CM deletes the oldest backups if this number is exceeded. The default value is <code>1</code>. </p>
    #[serde(rename = "BackupRetentionCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_count: Option<i64>,
    /// <p><p>Supported on servers running Chef Automate 2. A PEM-formatted HTTPS certificate. The value can be be a single, self-signed certificate, or a certificate chain. If you specify a custom certificate, you must also specify values for <code>CustomDomain</code> and <code>CustomPrivateKey</code>. The following are requirements for the <code>CustomCertificate</code> value:</p> <ul> <li> <p>You can provide either a self-signed, custom certificate, or the full certificate chain.</p> </li> <li> <p>The certificate must be a valid X509 certificate, or a certificate chain in PEM format.</p> </li> <li> <p>The certificate must be valid at the time of upload. A certificate can&#39;t be used before its validity period begins (the certificate&#39;s <code>NotBefore</code> date), or after it expires (the certificate&#39;s <code>NotAfter</code> date).</p> </li> <li> <p>The certificate’s common name or subject alternative names (SANs), if present, must match the value of <code>CustomDomain</code>.</p> </li> <li> <p>The certificate must match the value of <code>CustomPrivateKey</code>.</p> </li> </ul></p>
    #[serde(rename = "CustomCertificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_certificate: Option<String>,
    /// <p>Supported on servers running Chef Automate 2. An optional public endpoint of a server, such as <code>https://aws.my-company.com</code>. To access the server, create a CNAME DNS record in your preferred DNS service that points the custom domain to the endpoint that is generated when the server is created (the value of the CreateServer Endpoint attribute). You cannot access the server by using the generated <code>Endpoint</code> value if the server is using a custom domain. If you specify a custom domain, you must also specify values for <code>CustomCertificate</code> and <code>CustomPrivateKey</code>.</p>
    #[serde(rename = "CustomDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_domain: Option<String>,
    /// <p>Supported on servers running Chef Automate 2. A private key in PEM format for connecting to the server by using HTTPS. The private key must not be encrypted; it cannot be protected by a password or passphrase. If you specify a custom private key, you must also specify values for <code>CustomDomain</code> and <code>CustomCertificate</code>.</p>
    #[serde(rename = "CustomPrivateKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_private_key: Option<String>,
    /// <p> Enable or disable scheduled backups. Valid values are <code>true</code> or <code>false</code>. The default value is <code>true</code>. </p>
    #[serde(rename = "DisableAutomatedBackup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_automated_backup: Option<bool>,
    /// <p> The configuration management engine to use. Valid values include <code>ChefAutomate</code> and <code>Puppet</code>. </p>
    #[serde(rename = "Engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    /// <p><p>Optional engine attributes on a specified server. </p> <p class="title"> <b>Attributes accepted in a Chef createServer request:</b> </p> <ul> <li> <p> <code>CHEF<em>AUTOMATE</em>PIVOTAL<em>KEY</code>: A base64-encoded RSA public key. The corresponding private key is required to access the Chef API. When no CHEF</em>AUTOMATE<em>PIVOTAL</em>KEY is set, a private key is generated and returned in the response. </p> </li> <li> <p> <code>CHEF<em>AUTOMATE</em>ADMIN<em>PASSWORD</code>: The password for the administrative user in the Chef Automate web-based dashboard. The password length is a minimum of eight characters, and a maximum of 32. The password can contain letters, numbers, and special characters (!/@#$%^&amp;+=</em>). The password must contain at least one lower case letter, one upper case letter, one number, and one special character. When no CHEF<em>AUTOMATE</em>ADMIN<em>PASSWORD is set, one is generated and returned in the response.</p> </li> </ul> <p class="title"> <b>Attributes accepted in a Puppet createServer request:</b> </p> <ul> <li> <p> <code>PUPPET</em>ADMIN<em>PASSWORD</code>: To work with the Puppet Enterprise console, a password must use ASCII characters.</p> </li> <li> <p> <code>PUPPET</em>R10K<em>REMOTE</code>: The r10k remote is the URL of your control repository (for example, ssh://git@your.git-repo.com:user/control-repo.git). Specifying an r10k remote opens TCP port 8170.</p> </li> <li> <p> <code>PUPPET</em>R10K<em>PRIVATE</em>KEY</code>: If you are using a private Git repository, add PUPPET<em>R10K</em>PRIVATE_KEY to specify a PEM-encoded private SSH key.</p> </li> </ul></p>
    #[serde(rename = "EngineAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_attributes: Option<Vec<EngineAttribute>>,
    /// <p> The engine model of the server. Valid values in this release include <code>Monolithic</code> for Puppet and <code>Single</code> for Chef. </p>
    #[serde(rename = "EngineModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_model: Option<String>,
    /// <p> The major release version of the engine that you want to use. For a Chef server, the valid value for EngineVersion is currently <code>12</code>. For a Puppet server, the valid value is <code>2017</code>. </p>
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// <p> The ARN of the instance profile that your Amazon EC2 instances use. Although the AWS OpsWorks console typically creates the instance profile for you, if you are using API commands instead, run the service-role-creation.yaml AWS CloudFormation template, located at https://s3.amazonaws.com/opsworks-cm-us-east-1-prod-default-assets/misc/opsworks-cm-roles.yaml. This template creates a CloudFormation stack that includes the instance profile you need. </p>
    #[serde(rename = "InstanceProfileArn")]
    pub instance_profile_arn: String,
    /// <p> The Amazon EC2 instance type to use. For example, <code>m5.large</code>. </p>
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
    /// <p> The Amazon EC2 key pair to set for the instance. This parameter is optional; if desired, you may specify this parameter to connect to your instances by using SSH. </p>
    #[serde(rename = "KeyPair")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pair: Option<String>,
    /// <p> The start time for a one-hour period during which AWS OpsWorks CM backs up application-level data on your server if automated backups are enabled. Valid values must be specified in one of the following formats: </p> <ul> <li> <p> <code>HH:MM</code> for daily backups</p> </li> <li> <p> <code>DDD:HH:MM</code> for weekly backups</p> </li> </ul> <p>The specified time is in coordinated universal time (UTC). The default value is a random, daily start time.</p> <p> <b>Example:</b> <code>08:00</code>, which represents a daily start time of 08:00 UTC.</p> <p> <b>Example:</b> <code>Mon:08:00</code>, which represents a start time of every Monday at 08:00 UTC. (8:00 a.m.)</p>
    #[serde(rename = "PreferredBackupWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_window: Option<String>,
    /// <p> The start time for a one-hour period each week during which AWS OpsWorks CM performs maintenance on the instance. Valid values must be specified in the following format: <code>DDD:HH:MM</code>. The specified time is in coordinated universal time (UTC). The default value is a random one-hour period on Tuesday, Wednesday, or Friday. See <code>TimeWindowDefinition</code> for more information. </p> <p> <b>Example:</b> <code>Mon:08:00</code>, which represents a start time of every Monday at 08:00 UTC. (8:00 a.m.) </p>
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    /// <p> A list of security group IDs to attach to the Amazon EC2 instance. If you add this parameter, the specified security groups must be within the VPC that is specified by <code>SubnetIds</code>. </p> <p> If you do not specify this parameter, AWS OpsWorks CM creates one new security group that uses TCP ports 22 and 443, open to 0.0.0.0/0 (everyone). </p>
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p> The name of the server. The server name must be unique within your AWS account, within each region. Server names must start with a letter; then letters, numbers, or hyphens (-) are allowed, up to a maximum of 40 characters. </p>
    #[serde(rename = "ServerName")]
    pub server_name: String,
    /// <p> The service role that the AWS OpsWorks CM service backend uses to work with your account. Although the AWS OpsWorks management console typically creates the service role for you, if you are using the AWS CLI or API commands, run the service-role-creation.yaml AWS CloudFormation template, located at https://s3.amazonaws.com/opsworks-cm-us-east-1-prod-default-assets/misc/opsworks-cm-roles.yaml. This template creates a CloudFormation stack that includes the service role and instance profile that you need. </p>
    #[serde(rename = "ServiceRoleArn")]
    pub service_role_arn: String,
    /// <p> The IDs of subnets in which to launch the server EC2 instance. </p> <p> Amazon EC2-Classic customers: This field is required. All servers must run within a VPC. The VPC must have "Auto Assign Public IP" enabled. </p> <p> EC2-VPC customers: This field is optional. If you do not specify subnet IDs, your EC2 instances are created in a default subnet that is selected by Amazon EC2. If you specify subnet IDs, the VPC must have "Auto Assign Public IP" enabled. </p> <p>For more information about supported Amazon EC2 platforms, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-supported-platforms.html">Supported Platforms</a>.</p>
    #[serde(rename = "SubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    /// <p><p>A map that contains tag keys and tag values to attach to an AWS OpsWorks for Chef Automate or AWS OpsWorks for Puppet Enterprise server.</p> <ul> <li> <p>The key cannot be empty.</p> </li> <li> <p>The key can be a maximum of 127 characters, and can contain only Unicode letters, numbers, or separators, or the following special characters: <code>+ - = . _ : /</code> </p> </li> <li> <p>The value can be a maximum 255 characters, and contain only Unicode letters, numbers, or separators, or the following special characters: <code>+ - = . _ : /</code> </p> </li> <li> <p>Leading and trailing white spaces are trimmed from both the key and value.</p> </li> <li> <p>A maximum of 50 user-applied tags is allowed for any AWS OpsWorks-CM server.</p> </li> </ul></p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateServerResponse {
    /// <p>The server that is created by the request. </p>
    #[serde(rename = "Server")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<Server>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteBackupRequest {
    /// <p>The ID of the backup to delete. Run the DescribeBackups command to get a list of backup IDs. Backup IDs are in the format <code>ServerName-yyyyMMddHHmmssSSS</code>. </p>
    #[serde(rename = "BackupId")]
    pub backup_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteBackupResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteServerRequest {
    /// <p>The ID of the server to delete.</p>
    #[serde(rename = "ServerName")]
    pub server_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteServerResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeAccountAttributesRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAccountAttributesResponse {
    /// <p> The attributes that are currently set for the account. </p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<AccountAttribute>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeBackupsRequest {
    /// <p>Describes a single backup. </p>
    #[serde(rename = "BackupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_id: Option<String>,
    /// <p>This is not currently implemented for <code>DescribeBackups</code> requests.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>This is not currently implemented for <code>DescribeBackups</code> requests.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Returns backups for the server with the specified ServerName. </p>
    #[serde(rename = "ServerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeBackupsResponse {
    /// <p>Contains the response to a <code>DescribeBackups</code> request. </p>
    #[serde(rename = "Backups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backups: Option<Vec<Backup>>,
    /// <p>This is not currently implemented for <code>DescribeBackups</code> requests.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEventsRequest {
    /// <p>To receive a paginated response, use this parameter to specify the maximum number of results to be returned with a single call. If the number of available results exceeds this maximum, the response includes a <code>NextToken</code> value that you can assign to the <code>NextToken</code> request parameter to get the next set of results. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>NextToken is a string that is returned in some command responses. It indicates that not all entries have been returned, and that you must run at least one more request to get remaining items. To get remaining results, call <code>DescribeEvents</code> again, and assign the token from the previous results as the value of the <code>nextToken</code> parameter. If there are no more results, the response object's <code>nextToken</code> parameter value is <code>null</code>. Setting a <code>nextToken</code> value that was not returned in your previous results causes an <code>InvalidNextTokenException</code> to occur. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the server for which you want to view events.</p>
    #[serde(rename = "ServerName")]
    pub server_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeEventsResponse {
    /// <p>NextToken is a string that is returned in some command responses. It indicates that not all entries have been returned, and that you must run at least one more request to get remaining items. To get remaining results, call <code>DescribeEvents</code> again, and assign the token from the previous results as the value of the <code>nextToken</code> parameter. If there are no more results, the response object's <code>nextToken</code> parameter value is <code>null</code>. Setting a <code>nextToken</code> value that was not returned in your previous results causes an <code>InvalidNextTokenException</code> to occur. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Contains the response to a <code>DescribeEvents</code> request. </p>
    #[serde(rename = "ServerEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_events: Option<Vec<ServerEvent>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeNodeAssociationStatusRequest {
    /// <p>The token returned in either the AssociateNodeResponse or the DisassociateNodeResponse. </p>
    #[serde(rename = "NodeAssociationStatusToken")]
    pub node_association_status_token: String,
    /// <p>The name of the server from which to disassociate the node. </p>
    #[serde(rename = "ServerName")]
    pub server_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeNodeAssociationStatusResponse {
    /// <p>Attributes specific to the node association. In Puppet, the attibute PUPPET_NODE_CERT contains the signed certificate (the result of the CSR). </p>
    #[serde(rename = "EngineAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_attributes: Option<Vec<EngineAttribute>>,
    /// <p><p>The status of the association or disassociation request. </p> <p class="title"> <b>Possible values:</b> </p> <ul> <li> <p> <code>SUCCESS</code>: The association or disassociation succeeded. </p> </li> <li> <p> <code>FAILED</code>: The association or disassociation failed. </p> </li> <li> <p> <code>IN_PROGRESS</code>: The association or disassociation is still in progress. </p> </li> </ul></p>
    #[serde(rename = "NodeAssociationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_association_status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeServersRequest {
    /// <p>This is not currently implemented for <code>DescribeServers</code> requests. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>This is not currently implemented for <code>DescribeServers</code> requests. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Describes the server with the specified ServerName.</p>
    #[serde(rename = "ServerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeServersResponse {
    /// <p>This is not currently implemented for <code>DescribeServers</code> requests. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Contains the response to a <code>DescribeServers</code> request.</p> <p> <i>For Chef Automate servers:</i> If <code>DescribeServersResponse$Servers$EngineAttributes</code> includes CHEF_MAJOR_UPGRADE_AVAILABLE, you can upgrade the Chef Automate server to Chef Automate 2. To be eligible for upgrade, a server running Chef Automate 1 must have had at least one successful maintenance run after November 1, 2019.</p> <p> <i>For Puppet Server:</i> <code>DescribeServersResponse$Servers$EngineAttributes</code> contains PUPPET_API_CA_CERT. This is the PEM-encoded CA certificate that is used by the Puppet API over TCP port number 8140. The CA certificate is also used to sign node certificates.</p>
    #[serde(rename = "Servers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<Server>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateNodeRequest {
    /// <p><p>Engine attributes that are used for disassociating the node. No attributes are required for Puppet. </p> <p class="title"> <b>Attributes required in a DisassociateNode request for Chef</b> </p> <ul> <li> <p> <code>CHEF_ORGANIZATION</code>: The Chef organization with which the node was associated. By default only one organization named <code>default</code> can exist. </p> </li> </ul></p>
    #[serde(rename = "EngineAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_attributes: Option<Vec<EngineAttribute>>,
    /// <p>The name of the client node. </p>
    #[serde(rename = "NodeName")]
    pub node_name: String,
    /// <p>The name of the server from which to disassociate the node. </p>
    #[serde(rename = "ServerName")]
    pub server_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateNodeResponse {
    /// <p>Contains a token which can be passed to the <code>DescribeNodeAssociationStatus</code> API call to get the status of the disassociation request. </p>
    #[serde(rename = "NodeAssociationStatusToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_association_status_token: Option<String>,
}

/// <p>A name and value pair that is specific to the engine of the server. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EngineAttribute {
    /// <p>The name of the engine attribute. </p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The value of the engine attribute. </p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ExportServerEngineAttributeRequest {
    /// <p>The name of the export attribute. Currently, the supported export attribute is <code>Userdata</code>. This exports a user data script that includes parameters and values provided in the <code>InputAttributes</code> list.</p>
    #[serde(rename = "ExportAttributeName")]
    pub export_attribute_name: String,
    /// <p><p>The list of engine attributes. The list type is <code>EngineAttribute</code>. An <code>EngineAttribute</code> list item is a pair that includes an attribute name and its value. For the <code>Userdata</code> ExportAttributeName, the following are supported engine attribute names.</p> <ul> <li> <p> <b>RunList</b> In Chef, a list of roles or recipes that are run in the specified order. In Puppet, this parameter is ignored.</p> </li> <li> <p> <b>OrganizationName</b> In Chef, an organization name. AWS OpsWorks for Chef Automate always creates the organization <code>default</code>. In Puppet, this parameter is ignored.</p> </li> <li> <p> <b>NodeEnvironment</b> In Chef, a node environment (for example, development, staging, or one-box). In Puppet, this parameter is ignored.</p> </li> <li> <p> <b>NodeClientVersion</b> In Chef, the version of the Chef engine (three numbers separated by dots, such as 13.8.5). If this attribute is empty, OpsWorks for Chef Automate uses the most current version. In Puppet, this parameter is ignored.</p> </li> </ul></p>
    #[serde(rename = "InputAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_attributes: Option<Vec<EngineAttribute>>,
    /// <p>The name of the server from which you are exporting the attribute.</p>
    #[serde(rename = "ServerName")]
    pub server_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ExportServerEngineAttributeResponse {
    /// <p>The requested engine attribute pair with attribute name and value.</p>
    #[serde(rename = "EngineAttribute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_attribute: Option<EngineAttribute>,
    /// <p>The server name used in the request.</p>
    #[serde(rename = "ServerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>To receive a paginated response, use this parameter to specify the maximum number of results to be returned with a single call. If the number of available results exceeds this maximum, the response includes a <code>NextToken</code> value that you can assign to the <code>NextToken</code> request parameter to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>NextToken is a string that is returned in some command responses. It indicates that not all entries have been returned, and that you must run at least one more request to get remaining items. To get remaining results, call <code>ListTagsForResource</code> again, and assign the token from the previous results as the value of the <code>nextToken</code> parameter. If there are no more results, the response object's <code>nextToken</code> parameter value is <code>null</code>. Setting a <code>nextToken</code> value that was not returned in your previous results causes an <code>InvalidNextTokenException</code> to occur.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The Amazon Resource Number (ARN) of an AWS OpsWorks for Chef Automate or AWS OpsWorks for Puppet Enterprise server for which you want to show applied tags. For example, <code>arn:aws:opsworks-cm:us-west-2:123456789012:server/test-owcm-server/EXAMPLE-66b0-4196-8274-d1a2bEXAMPLE</code>.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>A token that you can use as the value of <code>NextToken</code> in subsequent calls to the API to show more results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Tags that have been applied to the resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RestoreServerRequest {
    /// <p> The ID of the backup that you want to use to restore a server. </p>
    #[serde(rename = "BackupId")]
    pub backup_id: String,
    /// <p> The type of instance to restore. Valid values must be specified in the following format: <code>^([cm][34]|t2).*</code> For example, <code>m5.large</code>. Valid values are <code>m5.large</code>, <code>r5.xlarge</code>, and <code>r5.2xlarge</code>. If you do not specify this parameter, RestoreServer uses the instance type from the specified backup. </p>
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p> The name of the key pair to set on the new EC2 instance. This can be helpful if the administrator no longer has the SSH key. </p>
    #[serde(rename = "KeyPair")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pair: Option<String>,
    /// <p> The name of the server that you want to restore. </p>
    #[serde(rename = "ServerName")]
    pub server_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RestoreServerResponse {}

/// <p>Describes a configuration management server. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Server {
    /// <p>Associate a public IP address with a server that you are launching. </p>
    #[serde(rename = "AssociatePublicIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associate_public_ip_address: Option<bool>,
    /// <p>The number of automated backups to keep. </p>
    #[serde(rename = "BackupRetentionCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_count: Option<i64>,
    /// <p>The ARN of the CloudFormation stack that was used to create the server. </p>
    #[serde(rename = "CloudFormationStackArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_formation_stack_arn: Option<String>,
    /// <p>Time stamp of server creation. Example <code>2016-07-29T13:38:47.520Z</code> </p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>An optional public endpoint of a server, such as <code>https://aws.my-company.com</code>. You cannot access the server by using the <code>Endpoint</code> value if the server has a <code>CustomDomain</code> specified.</p>
    #[serde(rename = "CustomDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_domain: Option<String>,
    /// <p>Disables automated backups. The number of stored backups is dependent on the value of PreferredBackupCount. </p>
    #[serde(rename = "DisableAutomatedBackup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_automated_backup: Option<bool>,
    /// <p> A DNS name that can be used to access the engine. Example: <code>myserver-asdfghjkl.us-east-1.opsworks.io</code>. You cannot access the server by using the <code>Endpoint</code> value if the server has a <code>CustomDomain</code> specified. </p>
    #[serde(rename = "Endpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    /// <p>The engine type of the server. Valid values in this release include <code>ChefAutomate</code> and <code>Puppet</code>. </p>
    #[serde(rename = "Engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    /// <p><p>The response of a createServer() request returns the master credential to access the server in EngineAttributes. These credentials are not stored by AWS OpsWorks CM; they are returned only as part of the result of createServer(). </p> <p class="title"> <b>Attributes returned in a createServer response for Chef</b> </p> <ul> <li> <p> <code>CHEF<em>AUTOMATE</em>PIVOTAL<em>KEY</code>: A base64-encoded RSA private key that is generated by AWS OpsWorks for Chef Automate. This private key is required to access the Chef API.</p> </li> <li> <p> <code>CHEF</em>STARTER<em>KIT</code>: A base64-encoded ZIP file. The ZIP file contains a Chef starter kit, which includes a README, a configuration file, and the required RSA private key. Save this file, unzip it, and then change to the directory where you&#39;ve unzipped the file contents. From this directory, you can run Knife commands.</p> </li> </ul> <p class="title"> <b>Attributes returned in a createServer response for Puppet</b> </p> <ul> <li> <p> <code>PUPPET</em>STARTER<em>KIT</code>: A base64-encoded ZIP file. The ZIP file contains a Puppet starter kit, including a README and a required private key. Save this file, unzip it, and then change to the directory where you&#39;ve unzipped the file contents.</p> </li> <li> <p> <code>PUPPET</em>ADMIN_PASSWORD</code>: An administrator password that you can use to sign in to the Puppet Enterprise console after the server is online.</p> </li> </ul></p>
    #[serde(rename = "EngineAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_attributes: Option<Vec<EngineAttribute>>,
    /// <p>The engine model of the server. Valid values in this release include <code>Monolithic</code> for Puppet and <code>Single</code> for Chef. </p>
    #[serde(rename = "EngineModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_model: Option<String>,
    /// <p>The engine version of the server. For a Chef server, the valid value for EngineVersion is currently <code>12</code>. For a Puppet server, the valid value is <code>2017</code>. </p>
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// <p>The instance profile ARN of the server. </p>
    #[serde(rename = "InstanceProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_profile_arn: Option<String>,
    /// <p> The instance type for the server, as specified in the CloudFormation stack. This might not be the same instance type that is shown in the EC2 console. </p>
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p>The key pair associated with the server. </p>
    #[serde(rename = "KeyPair")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pair: Option<String>,
    /// <p>The status of the most recent server maintenance run. Shows <code>SUCCESS</code> or <code>FAILED</code>. </p>
    #[serde(rename = "MaintenanceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_status: Option<String>,
    /// <p>The preferred backup period specified for the server. </p>
    #[serde(rename = "PreferredBackupWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_window: Option<String>,
    /// <p>The preferred maintenance period specified for the server. </p>
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    /// <p> The security group IDs for the server, as specified in the CloudFormation stack. These might not be the same security groups that are shown in the EC2 console. </p>
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p>The ARN of the server. </p>
    #[serde(rename = "ServerArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_arn: Option<String>,
    /// <p>The name of the server. </p>
    #[serde(rename = "ServerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    /// <p>The service role ARN used to create the server. </p>
    #[serde(rename = "ServiceRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role_arn: Option<String>,
    /// <p> The server's status. This field displays the states of actions in progress, such as creating, running, or backing up the server, as well as the server's health state. </p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p> Depending on the server status, this field has either a human-readable message (such as a create or backup error), or an escaped block of JSON (used for health check results). </p>
    #[serde(rename = "StatusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    /// <p> The subnet IDs specified in a CreateServer request. </p>
    #[serde(rename = "SubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
}

/// <p>An event that is related to the server, such as the start of maintenance or backup. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ServerEvent {
    /// <p>The time when the event occurred. </p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The Amazon S3 URL of the event's log file.</p>
    #[serde(rename = "LogUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_url: Option<String>,
    /// <p>A human-readable informational or status message.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The name of the server on or for which the event occurred. </p>
    #[serde(rename = "ServerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartMaintenanceRequest {
    /// <p><p>Engine attributes that are specific to the server on which you want to run maintenance.</p> <p class="title"> <b>Attributes accepted in a StartMaintenance request for Chef</b> </p> <ul> <li> <p> <code>CHEF<em>MAJOR</em>UPGRADE</code>: If a Chef Automate server is eligible for upgrade to Chef Automate 2, add this engine attribute to a <code>StartMaintenance</code> request and set the value to <code>true</code> to upgrade the server to Chef Automate 2. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opscm-a2upgrade.html">Upgrade an AWS OpsWorks for Chef Automate Server to Chef Automate 2</a>. </p> </li> </ul></p>
    #[serde(rename = "EngineAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_attributes: Option<Vec<EngineAttribute>>,
    /// <p>The name of the server on which to run maintenance. </p>
    #[serde(rename = "ServerName")]
    pub server_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartMaintenanceResponse {
    /// <p>Contains the response to a <code>StartMaintenance</code> request. </p>
    #[serde(rename = "Server")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<Server>,
}

/// <p>A map that contains tag keys and tag values to attach to an AWS OpsWorks for Chef Automate or AWS OpsWorks for Puppet Enterprise server. Leading and trailing white spaces are trimmed from both the key and value. A maximum of 50 user-applied tags is allowed for tag-supported AWS OpsWorks-CM resources.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>A tag key, such as <code>Stage</code> or <code>Name</code>. A tag key cannot be empty. The key can be a maximum of 127 characters, and can contain only Unicode letters, numbers, or separators, or the following special characters: <code>+ - = . _ : /</code> </p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>An optional tag value, such as <code>Production</code> or <code>test-owcm-server</code>. The value can be a maximum of 255 characters, and contain only Unicode letters, numbers, or separators, or the following special characters: <code>+ - = . _ : /</code> </p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Number (ARN) of a resource to which you want to apply tags. For example, <code>arn:aws:opsworks-cm:us-west-2:123456789012:server/test-owcm-server/EXAMPLE-66b0-4196-8274-d1a2bEXAMPLE</code>.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p><p>A map that contains tag keys and tag values to attach to AWS OpsWorks-CM servers or backups.</p> <ul> <li> <p>The key cannot be empty.</p> </li> <li> <p>The key can be a maximum of 127 characters, and can contain only Unicode letters, numbers, or separators, or the following special characters: <code>+ - = . _ : /</code> </p> </li> <li> <p>The value can be a maximum 255 characters, and contain only Unicode letters, numbers, or separators, or the following special characters: <code>+ - = . _ : /</code> </p> </li> <li> <p>Leading and trailing white spaces are trimmed from both the key and value.</p> </li> <li> <p>A maximum of 50 user-applied tags is allowed for any AWS OpsWorks-CM server or backup.</p> </li> </ul></p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Number (ARN) of a resource from which you want to remove tags. For example, <code>arn:aws:opsworks-cm:us-west-2:123456789012:server/test-owcm-server/EXAMPLE-66b0-4196-8274-d1a2bEXAMPLE</code>.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The keys of tags that you want to remove.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateServerEngineAttributesRequest {
    /// <p>The name of the engine attribute to update. </p>
    #[serde(rename = "AttributeName")]
    pub attribute_name: String,
    /// <p>The value to set for the attribute. </p>
    #[serde(rename = "AttributeValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_value: Option<String>,
    /// <p>The name of the server to update. </p>
    #[serde(rename = "ServerName")]
    pub server_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateServerEngineAttributesResponse {
    /// <p>Contains the response to an <code>UpdateServerEngineAttributes</code> request. </p>
    #[serde(rename = "Server")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<Server>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateServerRequest {
    /// <p>Sets the number of automated backups that you want to keep. </p>
    #[serde(rename = "BackupRetentionCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_count: Option<i64>,
    /// <p>Setting DisableAutomatedBackup to <code>true</code> disables automated or scheduled backups. Automated backups are enabled by default. </p>
    #[serde(rename = "DisableAutomatedBackup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_automated_backup: Option<bool>,
    #[serde(rename = "PreferredBackupWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_window: Option<String>,
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    /// <p>The name of the server to update. </p>
    #[serde(rename = "ServerName")]
    pub server_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateServerResponse {
    /// <p>Contains the response to a <code>UpdateServer</code> request. </p>
    #[serde(rename = "Server")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<Server>,
}

/// Errors returned by AssociateNode
#[derive(Debug, PartialEq)]
pub enum AssociateNodeError {
    /// <p>The resource is in a state that does not allow you to perform a specified action. </p>
    InvalidState(String),
    /// <p>The requested resource does not exist, or access was denied. </p>
    ResourceNotFound(String),
}

impl AssociateNodeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssociateNodeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidStateException" => {
                    return RusotoError::Service(AssociateNodeError::InvalidState(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AssociateNodeError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AssociateNodeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateNodeError::InvalidState(ref cause) => write!(f, "{}", cause),
            AssociateNodeError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AssociateNodeError {}
/// Errors returned by CreateBackup
#[derive(Debug, PartialEq)]
pub enum CreateBackupError {
    /// <p>The resource is in a state that does not allow you to perform a specified action. </p>
    InvalidState(String),
    /// <p>The limit of servers or backups has been reached. </p>
    LimitExceeded(String),
    /// <p>The requested resource does not exist, or access was denied. </p>
    ResourceNotFound(String),
}

impl CreateBackupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateBackupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidStateException" => {
                    return RusotoError::Service(CreateBackupError::InvalidState(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateBackupError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateBackupError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateBackupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateBackupError::InvalidState(ref cause) => write!(f, "{}", cause),
            CreateBackupError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateBackupError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateBackupError {}
/// Errors returned by CreateServer
#[derive(Debug, PartialEq)]
pub enum CreateServerError {
    /// <p>The limit of servers or backups has been reached. </p>
    LimitExceeded(String),
    /// <p>The requested resource cannot be created because it already exists. </p>
    ResourceAlreadyExists(String),
    /// <p>The requested resource does not exist, or access was denied. </p>
    ResourceNotFound(String),
}

impl CreateServerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateServerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "LimitExceededException" => {
                    return RusotoError::Service(CreateServerError::LimitExceeded(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateServerError::ResourceAlreadyExists(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateServerError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateServerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateServerError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateServerError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateServerError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateServerError {}
/// Errors returned by DeleteBackup
#[derive(Debug, PartialEq)]
pub enum DeleteBackupError {
    /// <p>The resource is in a state that does not allow you to perform a specified action. </p>
    InvalidState(String),
    /// <p>The requested resource does not exist, or access was denied. </p>
    ResourceNotFound(String),
}

impl DeleteBackupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteBackupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidStateException" => {
                    return RusotoError::Service(DeleteBackupError::InvalidState(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteBackupError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteBackupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteBackupError::InvalidState(ref cause) => write!(f, "{}", cause),
            DeleteBackupError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteBackupError {}
/// Errors returned by DeleteServer
#[derive(Debug, PartialEq)]
pub enum DeleteServerError {
    /// <p>The resource is in a state that does not allow you to perform a specified action. </p>
    InvalidState(String),
    /// <p>The requested resource does not exist, or access was denied. </p>
    ResourceNotFound(String),
}

impl DeleteServerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteServerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidStateException" => {
                    return RusotoError::Service(DeleteServerError::InvalidState(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteServerError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteServerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteServerError::InvalidState(ref cause) => write!(f, "{}", cause),
            DeleteServerError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteServerError {}
/// Errors returned by DescribeAccountAttributes
#[derive(Debug, PartialEq)]
pub enum DescribeAccountAttributesError {}

impl DescribeAccountAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAccountAttributesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeAccountAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DescribeAccountAttributesError {}
/// Errors returned by DescribeBackups
#[derive(Debug, PartialEq)]
pub enum DescribeBackupsError {
    /// <p>This occurs when the provided nextToken is not valid. </p>
    InvalidNextToken(String),
    /// <p>The requested resource does not exist, or access was denied. </p>
    ResourceNotFound(String),
}

impl DescribeBackupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeBackupsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(DescribeBackupsError::InvalidNextToken(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeBackupsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeBackupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeBackupsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            DescribeBackupsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeBackupsError {}
/// Errors returned by DescribeEvents
#[derive(Debug, PartialEq)]
pub enum DescribeEventsError {
    /// <p>This occurs when the provided nextToken is not valid. </p>
    InvalidNextToken(String),
    /// <p>The requested resource does not exist, or access was denied. </p>
    ResourceNotFound(String),
}

impl DescribeEventsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeEventsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(DescribeEventsError::InvalidNextToken(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeEventsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeEventsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeEventsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            DescribeEventsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeEventsError {}
/// Errors returned by DescribeNodeAssociationStatus
#[derive(Debug, PartialEq)]
pub enum DescribeNodeAssociationStatusError {
    /// <p>The requested resource does not exist, or access was denied. </p>
    ResourceNotFound(String),
}

impl DescribeNodeAssociationStatusError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeNodeAssociationStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeNodeAssociationStatusError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeNodeAssociationStatusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeNodeAssociationStatusError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeNodeAssociationStatusError {}
/// Errors returned by DescribeServers
#[derive(Debug, PartialEq)]
pub enum DescribeServersError {
    /// <p>This occurs when the provided nextToken is not valid. </p>
    InvalidNextToken(String),
    /// <p>The requested resource does not exist, or access was denied. </p>
    ResourceNotFound(String),
}

impl DescribeServersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeServersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(DescribeServersError::InvalidNextToken(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeServersError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeServersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeServersError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            DescribeServersError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeServersError {}
/// Errors returned by DisassociateNode
#[derive(Debug, PartialEq)]
pub enum DisassociateNodeError {
    /// <p>The resource is in a state that does not allow you to perform a specified action. </p>
    InvalidState(String),
    /// <p>The requested resource does not exist, or access was denied. </p>
    ResourceNotFound(String),
}

impl DisassociateNodeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisassociateNodeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidStateException" => {
                    return RusotoError::Service(DisassociateNodeError::InvalidState(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DisassociateNodeError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DisassociateNodeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateNodeError::InvalidState(ref cause) => write!(f, "{}", cause),
            DisassociateNodeError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisassociateNodeError {}
/// Errors returned by ExportServerEngineAttribute
#[derive(Debug, PartialEq)]
pub enum ExportServerEngineAttributeError {
    /// <p>The resource is in a state that does not allow you to perform a specified action. </p>
    InvalidState(String),
    /// <p>The requested resource does not exist, or access was denied. </p>
    ResourceNotFound(String),
}

impl ExportServerEngineAttributeError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ExportServerEngineAttributeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidStateException" => {
                    return RusotoError::Service(ExportServerEngineAttributeError::InvalidState(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        ExportServerEngineAttributeError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ExportServerEngineAttributeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ExportServerEngineAttributeError::InvalidState(ref cause) => write!(f, "{}", cause),
            ExportServerEngineAttributeError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ExportServerEngineAttributeError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>The requested resource does not exist, or access was denied. </p>
    ResourceNotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::ResourceNotFound(
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
            ListTagsForResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by RestoreServer
#[derive(Debug, PartialEq)]
pub enum RestoreServerError {
    /// <p>The resource is in a state that does not allow you to perform a specified action. </p>
    InvalidState(String),
    /// <p>The requested resource does not exist, or access was denied. </p>
    ResourceNotFound(String),
}

impl RestoreServerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RestoreServerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidStateException" => {
                    return RusotoError::Service(RestoreServerError::InvalidState(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(RestoreServerError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RestoreServerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RestoreServerError::InvalidState(ref cause) => write!(f, "{}", cause),
            RestoreServerError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RestoreServerError {}
/// Errors returned by StartMaintenance
#[derive(Debug, PartialEq)]
pub enum StartMaintenanceError {
    /// <p>The resource is in a state that does not allow you to perform a specified action. </p>
    InvalidState(String),
    /// <p>The requested resource does not exist, or access was denied. </p>
    ResourceNotFound(String),
}

impl StartMaintenanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartMaintenanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidStateException" => {
                    return RusotoError::Service(StartMaintenanceError::InvalidState(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StartMaintenanceError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StartMaintenanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartMaintenanceError::InvalidState(ref cause) => write!(f, "{}", cause),
            StartMaintenanceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartMaintenanceError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>The resource is in a state that does not allow you to perform a specified action. </p>
    InvalidState(String),
    /// <p>The requested resource does not exist, or access was denied. </p>
    ResourceNotFound(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidStateException" => {
                    return RusotoError::Service(TagResourceError::InvalidState(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
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
            TagResourceError::InvalidState(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>The resource is in a state that does not allow you to perform a specified action. </p>
    InvalidState(String),
    /// <p>The requested resource does not exist, or access was denied. </p>
    ResourceNotFound(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidStateException" => {
                    return RusotoError::Service(UntagResourceError::InvalidState(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::ResourceNotFound(err.msg))
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
            UntagResourceError::InvalidState(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateServer
#[derive(Debug, PartialEq)]
pub enum UpdateServerError {
    /// <p>The resource is in a state that does not allow you to perform a specified action. </p>
    InvalidState(String),
    /// <p>The requested resource does not exist, or access was denied. </p>
    ResourceNotFound(String),
}

impl UpdateServerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateServerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidStateException" => {
                    return RusotoError::Service(UpdateServerError::InvalidState(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateServerError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateServerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateServerError::InvalidState(ref cause) => write!(f, "{}", cause),
            UpdateServerError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateServerError {}
/// Errors returned by UpdateServerEngineAttributes
#[derive(Debug, PartialEq)]
pub enum UpdateServerEngineAttributesError {
    /// <p>The resource is in a state that does not allow you to perform a specified action. </p>
    InvalidState(String),
    /// <p>The requested resource does not exist, or access was denied. </p>
    ResourceNotFound(String),
}

impl UpdateServerEngineAttributesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateServerEngineAttributesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidStateException" => {
                    return RusotoError::Service(UpdateServerEngineAttributesError::InvalidState(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        UpdateServerEngineAttributesError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateServerEngineAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateServerEngineAttributesError::InvalidState(ref cause) => write!(f, "{}", cause),
            UpdateServerEngineAttributesError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateServerEngineAttributesError {}
/// Trait representing the capabilities of the OpsWorksCM API. OpsWorksCM clients implement this trait.
#[async_trait]
pub trait OpsWorksCM {
    /// <p> Associates a new node with the server. For more information about how to disassociate a node, see <a>DisassociateNode</a>.</p> <p> On a Chef server: This command is an alternative to <code>knife bootstrap</code>.</p> <p> Example (Chef): <code>aws opsworks-cm associate-node --server-name <i>MyServer</i> --node-name <i>MyManagedNode</i> --engine-attributes "Name=<i>CHEF_ORGANIZATION</i>,Value=default" "Name=<i>CHEF_NODE_PUBLIC_KEY</i>,Value=<i>public-key-pem</i>"</code> </p> <p> On a Puppet server, this command is an alternative to the <code>puppet cert sign</code> command that signs a Puppet node CSR. </p> <p> Example (Chef): <code>aws opsworks-cm associate-node --server-name <i>MyServer</i> --node-name <i>MyManagedNode</i> --engine-attributes "Name=<i>PUPPET_NODE_CSR</i>,Value=<i>csr-pem</i>"</code> </p> <p> A node can can only be associated with servers that are in a <code>HEALTHY</code> state. Otherwise, an <code>InvalidStateException</code> is thrown. A <code>ResourceNotFoundException</code> is thrown when the server does not exist. A <code>ValidationException</code> is raised when parameters of the request are not valid. The AssociateNode API call can be integrated into Auto Scaling configurations, AWS Cloudformation templates, or the user data of a server's instance. </p>
    async fn associate_node(
        &self,
        input: AssociateNodeRequest,
    ) -> Result<AssociateNodeResponse, RusotoError<AssociateNodeError>>;

    /// <p> Creates an application-level backup of a server. While the server is in the <code>BACKING_UP</code> state, the server cannot be changed, and no additional backup can be created. </p> <p> Backups can be created for servers in <code>RUNNING</code>, <code>HEALTHY</code>, and <code>UNHEALTHY</code> states. By default, you can create a maximum of 50 manual backups. </p> <p> This operation is asynchronous. </p> <p> A <code>LimitExceededException</code> is thrown when the maximum number of manual backups is reached. An <code>InvalidStateException</code> is thrown when the server is not in any of the following states: RUNNING, HEALTHY, or UNHEALTHY. A <code>ResourceNotFoundException</code> is thrown when the server is not found. A <code>ValidationException</code> is thrown when parameters of the request are not valid. </p>
    async fn create_backup(
        &self,
        input: CreateBackupRequest,
    ) -> Result<CreateBackupResponse, RusotoError<CreateBackupError>>;

    /// <p> Creates and immedately starts a new server. The server is ready to use when it is in the <code>HEALTHY</code> state. By default, you can create a maximum of 10 servers. </p> <p> This operation is asynchronous. </p> <p> A <code>LimitExceededException</code> is thrown when you have created the maximum number of servers (10). A <code>ResourceAlreadyExistsException</code> is thrown when a server with the same name already exists in the account. A <code>ResourceNotFoundException</code> is thrown when you specify a backup ID that is not valid or is for a backup that does not exist. A <code>ValidationException</code> is thrown when parameters of the request are not valid. </p> <p> If you do not specify a security group by adding the <code>SecurityGroupIds</code> parameter, AWS OpsWorks creates a new security group. </p> <p> <i>Chef Automate:</i> The default security group opens the Chef server to the world on TCP port 443. If a KeyName is present, AWS OpsWorks enables SSH access. SSH is also open to the world on TCP port 22. </p> <p> <i>Puppet Enterprise:</i> The default security group opens TCP ports 22, 443, 4433, 8140, 8142, 8143, and 8170. If a KeyName is present, AWS OpsWorks enables SSH access. SSH is also open to the world on TCP port 22. </p> <p>By default, your server is accessible from any IP address. We recommend that you update your security group rules to allow access from known IP addresses and address ranges only. To edit security group rules, open Security Groups in the navigation pane of the EC2 management console. </p> <p>To specify your own domain for a server, and provide your own self-signed or CA-signed certificate and private key, specify values for <code>CustomDomain</code>, <code>CustomCertificate</code>, and <code>CustomPrivateKey</code>.</p>
    async fn create_server(
        &self,
        input: CreateServerRequest,
    ) -> Result<CreateServerResponse, RusotoError<CreateServerError>>;

    /// <p> Deletes a backup. You can delete both manual and automated backups. This operation is asynchronous. </p> <p> An <code>InvalidStateException</code> is thrown when a backup deletion is already in progress. A <code>ResourceNotFoundException</code> is thrown when the backup does not exist. A <code>ValidationException</code> is thrown when parameters of the request are not valid. </p>
    async fn delete_backup(
        &self,
        input: DeleteBackupRequest,
    ) -> Result<DeleteBackupResponse, RusotoError<DeleteBackupError>>;

    /// <p> Deletes the server and the underlying AWS CloudFormation stacks (including the server's EC2 instance). When you run this command, the server state is updated to <code>DELETING</code>. After the server is deleted, it is no longer returned by <code>DescribeServer</code> requests. If the AWS CloudFormation stack cannot be deleted, the server cannot be deleted. </p> <p> This operation is asynchronous. </p> <p> An <code>InvalidStateException</code> is thrown when a server deletion is already in progress. A <code>ResourceNotFoundException</code> is thrown when the server does not exist. A <code>ValidationException</code> is raised when parameters of the request are not valid. </p> <p> </p>
    async fn delete_server(
        &self,
        input: DeleteServerRequest,
    ) -> Result<DeleteServerResponse, RusotoError<DeleteServerError>>;

    /// <p> Describes your OpsWorks-CM account attributes. </p> <p> This operation is synchronous. </p>
    async fn describe_account_attributes(
        &self,
    ) -> Result<DescribeAccountAttributesResponse, RusotoError<DescribeAccountAttributesError>>;

    /// <p> Describes backups. The results are ordered by time, with newest backups first. If you do not specify a BackupId or ServerName, the command returns all backups. </p> <p> This operation is synchronous. </p> <p> A <code>ResourceNotFoundException</code> is thrown when the backup does not exist. A <code>ValidationException</code> is raised when parameters of the request are not valid. </p>
    async fn describe_backups(
        &self,
        input: DescribeBackupsRequest,
    ) -> Result<DescribeBackupsResponse, RusotoError<DescribeBackupsError>>;

    /// <p> Describes events for a specified server. Results are ordered by time, with newest events first. </p> <p> This operation is synchronous. </p> <p> A <code>ResourceNotFoundException</code> is thrown when the server does not exist. A <code>ValidationException</code> is raised when parameters of the request are not valid. </p>
    async fn describe_events(
        &self,
        input: DescribeEventsRequest,
    ) -> Result<DescribeEventsResponse, RusotoError<DescribeEventsError>>;

    /// <p> Returns the current status of an existing association or disassociation request. </p> <p> A <code>ResourceNotFoundException</code> is thrown when no recent association or disassociation request with the specified token is found, or when the server does not exist. A <code>ValidationException</code> is raised when parameters of the request are not valid. </p>
    async fn describe_node_association_status(
        &self,
        input: DescribeNodeAssociationStatusRequest,
    ) -> Result<
        DescribeNodeAssociationStatusResponse,
        RusotoError<DescribeNodeAssociationStatusError>,
    >;

    /// <p> Lists all configuration management servers that are identified with your account. Only the stored results from Amazon DynamoDB are returned. AWS OpsWorks CM does not query other services. </p> <p> This operation is synchronous. </p> <p> A <code>ResourceNotFoundException</code> is thrown when the server does not exist. A <code>ValidationException</code> is raised when parameters of the request are not valid. </p>
    async fn describe_servers(
        &self,
        input: DescribeServersRequest,
    ) -> Result<DescribeServersResponse, RusotoError<DescribeServersError>>;

    /// <p> Disassociates a node from an AWS OpsWorks CM server, and removes the node from the server's managed nodes. After a node is disassociated, the node key pair is no longer valid for accessing the configuration manager's API. For more information about how to associate a node, see <a>AssociateNode</a>. </p> <p>A node can can only be disassociated from a server that is in a <code>HEALTHY</code> state. Otherwise, an <code>InvalidStateException</code> is thrown. A <code>ResourceNotFoundException</code> is thrown when the server does not exist. A <code>ValidationException</code> is raised when parameters of the request are not valid. </p>
    async fn disassociate_node(
        &self,
        input: DisassociateNodeRequest,
    ) -> Result<DisassociateNodeResponse, RusotoError<DisassociateNodeError>>;

    /// <p> Exports a specified server engine attribute as a base64-encoded string. For example, you can export user data that you can use in EC2 to associate nodes with a server. </p> <p> This operation is synchronous. </p> <p> A <code>ValidationException</code> is raised when parameters of the request are not valid. A <code>ResourceNotFoundException</code> is thrown when the server does not exist. An <code>InvalidStateException</code> is thrown when the server is in any of the following states: CREATING, TERMINATED, FAILED or DELETING. </p>
    async fn export_server_engine_attribute(
        &self,
        input: ExportServerEngineAttributeRequest,
    ) -> Result<ExportServerEngineAttributeResponse, RusotoError<ExportServerEngineAttributeError>>;

    /// <p>Returns a list of tags that are applied to the specified AWS OpsWorks for Chef Automate or AWS OpsWorks for Puppet Enterprise servers or backups.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p> Restores a backup to a server that is in a <code>CONNECTION_LOST</code>, <code>HEALTHY</code>, <code>RUNNING</code>, <code>UNHEALTHY</code>, or <code>TERMINATED</code> state. When you run RestoreServer, the server's EC2 instance is deleted, and a new EC2 instance is configured. RestoreServer maintains the existing server endpoint, so configuration management of the server's client devices (nodes) should continue to work. </p> <p>Restoring from a backup is performed by creating a new EC2 instance. If restoration is successful, and the server is in a <code>HEALTHY</code> state, AWS OpsWorks CM switches traffic over to the new instance. After restoration is finished, the old EC2 instance is maintained in a <code>Running</code> or <code>Stopped</code> state, but is eventually terminated.</p> <p> This operation is asynchronous. </p> <p> An <code>InvalidStateException</code> is thrown when the server is not in a valid state. A <code>ResourceNotFoundException</code> is thrown when the server does not exist. A <code>ValidationException</code> is raised when parameters of the request are not valid. </p>
    async fn restore_server(
        &self,
        input: RestoreServerRequest,
    ) -> Result<RestoreServerResponse, RusotoError<RestoreServerError>>;

    /// <p> Manually starts server maintenance. This command can be useful if an earlier maintenance attempt failed, and the underlying cause of maintenance failure has been resolved. The server is in an <code>UNDER_MAINTENANCE</code> state while maintenance is in progress. </p> <p> Maintenance can only be started on servers in <code>HEALTHY</code> and <code>UNHEALTHY</code> states. Otherwise, an <code>InvalidStateException</code> is thrown. A <code>ResourceNotFoundException</code> is thrown when the server does not exist. A <code>ValidationException</code> is raised when parameters of the request are not valid. </p>
    async fn start_maintenance(
        &self,
        input: StartMaintenanceRequest,
    ) -> Result<StartMaintenanceResponse, RusotoError<StartMaintenanceError>>;

    /// <p>Applies tags to an AWS OpsWorks for Chef Automate or AWS OpsWorks for Puppet Enterprise server, or to server backups.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Removes specified tags from an AWS OpsWorks-CM server or backup.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p> Updates settings for a server. </p> <p> This operation is synchronous. </p>
    async fn update_server(
        &self,
        input: UpdateServerRequest,
    ) -> Result<UpdateServerResponse, RusotoError<UpdateServerError>>;

    /// <p> Updates engine-specific attributes on a specified server. The server enters the <code>MODIFYING</code> state when this operation is in progress. Only one update can occur at a time. You can use this command to reset a Chef server's public key (<code>CHEF_PIVOTAL_KEY</code>) or a Puppet server's admin password (<code>PUPPET_ADMIN_PASSWORD</code>). </p> <p> This operation is asynchronous. </p> <p> This operation can only be called for servers in <code>HEALTHY</code> or <code>UNHEALTHY</code> states. Otherwise, an <code>InvalidStateException</code> is raised. A <code>ResourceNotFoundException</code> is thrown when the server does not exist. A <code>ValidationException</code> is raised when parameters of the request are not valid. </p>
    async fn update_server_engine_attributes(
        &self,
        input: UpdateServerEngineAttributesRequest,
    ) -> Result<UpdateServerEngineAttributesResponse, RusotoError<UpdateServerEngineAttributesError>>;
}
/// A client for the OpsWorksCM API.
#[derive(Clone)]
pub struct OpsWorksCMClient {
    client: Client,
    region: region::Region,
}

impl OpsWorksCMClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> OpsWorksCMClient {
        OpsWorksCMClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> OpsWorksCMClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        OpsWorksCMClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> OpsWorksCMClient {
        OpsWorksCMClient { client, region }
    }
}

#[async_trait]
impl OpsWorksCM for OpsWorksCMClient {
    /// <p> Associates a new node with the server. For more information about how to disassociate a node, see <a>DisassociateNode</a>.</p> <p> On a Chef server: This command is an alternative to <code>knife bootstrap</code>.</p> <p> Example (Chef): <code>aws opsworks-cm associate-node --server-name <i>MyServer</i> --node-name <i>MyManagedNode</i> --engine-attributes "Name=<i>CHEF_ORGANIZATION</i>,Value=default" "Name=<i>CHEF_NODE_PUBLIC_KEY</i>,Value=<i>public-key-pem</i>"</code> </p> <p> On a Puppet server, this command is an alternative to the <code>puppet cert sign</code> command that signs a Puppet node CSR. </p> <p> Example (Chef): <code>aws opsworks-cm associate-node --server-name <i>MyServer</i> --node-name <i>MyManagedNode</i> --engine-attributes "Name=<i>PUPPET_NODE_CSR</i>,Value=<i>csr-pem</i>"</code> </p> <p> A node can can only be associated with servers that are in a <code>HEALTHY</code> state. Otherwise, an <code>InvalidStateException</code> is thrown. A <code>ResourceNotFoundException</code> is thrown when the server does not exist. A <code>ValidationException</code> is raised when parameters of the request are not valid. The AssociateNode API call can be integrated into Auto Scaling configurations, AWS Cloudformation templates, or the user data of a server's instance. </p>
    async fn associate_node(
        &self,
        input: AssociateNodeRequest,
    ) -> Result<AssociateNodeResponse, RusotoError<AssociateNodeError>> {
        let mut request = SignedRequest::new("POST", "opsworks-cm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorksCM_V2016_11_01.AssociateNode");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<AssociateNodeResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AssociateNodeError::from_response(response))
        }
    }

    /// <p> Creates an application-level backup of a server. While the server is in the <code>BACKING_UP</code> state, the server cannot be changed, and no additional backup can be created. </p> <p> Backups can be created for servers in <code>RUNNING</code>, <code>HEALTHY</code>, and <code>UNHEALTHY</code> states. By default, you can create a maximum of 50 manual backups. </p> <p> This operation is asynchronous. </p> <p> A <code>LimitExceededException</code> is thrown when the maximum number of manual backups is reached. An <code>InvalidStateException</code> is thrown when the server is not in any of the following states: RUNNING, HEALTHY, or UNHEALTHY. A <code>ResourceNotFoundException</code> is thrown when the server is not found. A <code>ValidationException</code> is thrown when parameters of the request are not valid. </p>
    async fn create_backup(
        &self,
        input: CreateBackupRequest,
    ) -> Result<CreateBackupResponse, RusotoError<CreateBackupError>> {
        let mut request = SignedRequest::new("POST", "opsworks-cm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorksCM_V2016_11_01.CreateBackup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateBackupResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateBackupError::from_response(response))
        }
    }

    /// <p> Creates and immedately starts a new server. The server is ready to use when it is in the <code>HEALTHY</code> state. By default, you can create a maximum of 10 servers. </p> <p> This operation is asynchronous. </p> <p> A <code>LimitExceededException</code> is thrown when you have created the maximum number of servers (10). A <code>ResourceAlreadyExistsException</code> is thrown when a server with the same name already exists in the account. A <code>ResourceNotFoundException</code> is thrown when you specify a backup ID that is not valid or is for a backup that does not exist. A <code>ValidationException</code> is thrown when parameters of the request are not valid. </p> <p> If you do not specify a security group by adding the <code>SecurityGroupIds</code> parameter, AWS OpsWorks creates a new security group. </p> <p> <i>Chef Automate:</i> The default security group opens the Chef server to the world on TCP port 443. If a KeyName is present, AWS OpsWorks enables SSH access. SSH is also open to the world on TCP port 22. </p> <p> <i>Puppet Enterprise:</i> The default security group opens TCP ports 22, 443, 4433, 8140, 8142, 8143, and 8170. If a KeyName is present, AWS OpsWorks enables SSH access. SSH is also open to the world on TCP port 22. </p> <p>By default, your server is accessible from any IP address. We recommend that you update your security group rules to allow access from known IP addresses and address ranges only. To edit security group rules, open Security Groups in the navigation pane of the EC2 management console. </p> <p>To specify your own domain for a server, and provide your own self-signed or CA-signed certificate and private key, specify values for <code>CustomDomain</code>, <code>CustomCertificate</code>, and <code>CustomPrivateKey</code>.</p>
    async fn create_server(
        &self,
        input: CreateServerRequest,
    ) -> Result<CreateServerResponse, RusotoError<CreateServerError>> {
        let mut request = SignedRequest::new("POST", "opsworks-cm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorksCM_V2016_11_01.CreateServer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateServerResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateServerError::from_response(response))
        }
    }

    /// <p> Deletes a backup. You can delete both manual and automated backups. This operation is asynchronous. </p> <p> An <code>InvalidStateException</code> is thrown when a backup deletion is already in progress. A <code>ResourceNotFoundException</code> is thrown when the backup does not exist. A <code>ValidationException</code> is thrown when parameters of the request are not valid. </p>
    async fn delete_backup(
        &self,
        input: DeleteBackupRequest,
    ) -> Result<DeleteBackupResponse, RusotoError<DeleteBackupError>> {
        let mut request = SignedRequest::new("POST", "opsworks-cm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorksCM_V2016_11_01.DeleteBackup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteBackupResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteBackupError::from_response(response))
        }
    }

    /// <p> Deletes the server and the underlying AWS CloudFormation stacks (including the server's EC2 instance). When you run this command, the server state is updated to <code>DELETING</code>. After the server is deleted, it is no longer returned by <code>DescribeServer</code> requests. If the AWS CloudFormation stack cannot be deleted, the server cannot be deleted. </p> <p> This operation is asynchronous. </p> <p> An <code>InvalidStateException</code> is thrown when a server deletion is already in progress. A <code>ResourceNotFoundException</code> is thrown when the server does not exist. A <code>ValidationException</code> is raised when parameters of the request are not valid. </p> <p> </p>
    async fn delete_server(
        &self,
        input: DeleteServerRequest,
    ) -> Result<DeleteServerResponse, RusotoError<DeleteServerError>> {
        let mut request = SignedRequest::new("POST", "opsworks-cm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorksCM_V2016_11_01.DeleteServer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteServerResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteServerError::from_response(response))
        }
    }

    /// <p> Describes your OpsWorks-CM account attributes. </p> <p> This operation is synchronous. </p>
    async fn describe_account_attributes(
        &self,
    ) -> Result<DescribeAccountAttributesResponse, RusotoError<DescribeAccountAttributesError>>
    {
        let mut request = SignedRequest::new("POST", "opsworks-cm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OpsWorksCM_V2016_11_01.DescribeAccountAttributes",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeAccountAttributesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeAccountAttributesError::from_response(response))
        }
    }

    /// <p> Describes backups. The results are ordered by time, with newest backups first. If you do not specify a BackupId or ServerName, the command returns all backups. </p> <p> This operation is synchronous. </p> <p> A <code>ResourceNotFoundException</code> is thrown when the backup does not exist. A <code>ValidationException</code> is raised when parameters of the request are not valid. </p>
    async fn describe_backups(
        &self,
        input: DescribeBackupsRequest,
    ) -> Result<DescribeBackupsResponse, RusotoError<DescribeBackupsError>> {
        let mut request = SignedRequest::new("POST", "opsworks-cm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorksCM_V2016_11_01.DescribeBackups");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeBackupsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeBackupsError::from_response(response))
        }
    }

    /// <p> Describes events for a specified server. Results are ordered by time, with newest events first. </p> <p> This operation is synchronous. </p> <p> A <code>ResourceNotFoundException</code> is thrown when the server does not exist. A <code>ValidationException</code> is raised when parameters of the request are not valid. </p>
    async fn describe_events(
        &self,
        input: DescribeEventsRequest,
    ) -> Result<DescribeEventsResponse, RusotoError<DescribeEventsError>> {
        let mut request = SignedRequest::new("POST", "opsworks-cm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorksCM_V2016_11_01.DescribeEvents");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeEventsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeEventsError::from_response(response))
        }
    }

    /// <p> Returns the current status of an existing association or disassociation request. </p> <p> A <code>ResourceNotFoundException</code> is thrown when no recent association or disassociation request with the specified token is found, or when the server does not exist. A <code>ValidationException</code> is raised when parameters of the request are not valid. </p>
    async fn describe_node_association_status(
        &self,
        input: DescribeNodeAssociationStatusRequest,
    ) -> Result<
        DescribeNodeAssociationStatusResponse,
        RusotoError<DescribeNodeAssociationStatusError>,
    > {
        let mut request = SignedRequest::new("POST", "opsworks-cm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OpsWorksCM_V2016_11_01.DescribeNodeAssociationStatus",
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
                .deserialize::<DescribeNodeAssociationStatusResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeNodeAssociationStatusError::from_response(response))
        }
    }

    /// <p> Lists all configuration management servers that are identified with your account. Only the stored results from Amazon DynamoDB are returned. AWS OpsWorks CM does not query other services. </p> <p> This operation is synchronous. </p> <p> A <code>ResourceNotFoundException</code> is thrown when the server does not exist. A <code>ValidationException</code> is raised when parameters of the request are not valid. </p>
    async fn describe_servers(
        &self,
        input: DescribeServersRequest,
    ) -> Result<DescribeServersResponse, RusotoError<DescribeServersError>> {
        let mut request = SignedRequest::new("POST", "opsworks-cm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorksCM_V2016_11_01.DescribeServers");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeServersResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeServersError::from_response(response))
        }
    }

    /// <p> Disassociates a node from an AWS OpsWorks CM server, and removes the node from the server's managed nodes. After a node is disassociated, the node key pair is no longer valid for accessing the configuration manager's API. For more information about how to associate a node, see <a>AssociateNode</a>. </p> <p>A node can can only be disassociated from a server that is in a <code>HEALTHY</code> state. Otherwise, an <code>InvalidStateException</code> is thrown. A <code>ResourceNotFoundException</code> is thrown when the server does not exist. A <code>ValidationException</code> is raised when parameters of the request are not valid. </p>
    async fn disassociate_node(
        &self,
        input: DisassociateNodeRequest,
    ) -> Result<DisassociateNodeResponse, RusotoError<DisassociateNodeError>> {
        let mut request = SignedRequest::new("POST", "opsworks-cm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorksCM_V2016_11_01.DisassociateNode");
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
                .deserialize::<DisassociateNodeResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateNodeError::from_response(response))
        }
    }

    /// <p> Exports a specified server engine attribute as a base64-encoded string. For example, you can export user data that you can use in EC2 to associate nodes with a server. </p> <p> This operation is synchronous. </p> <p> A <code>ValidationException</code> is raised when parameters of the request are not valid. A <code>ResourceNotFoundException</code> is thrown when the server does not exist. An <code>InvalidStateException</code> is thrown when the server is in any of the following states: CREATING, TERMINATED, FAILED or DELETING. </p>
    async fn export_server_engine_attribute(
        &self,
        input: ExportServerEngineAttributeRequest,
    ) -> Result<ExportServerEngineAttributeResponse, RusotoError<ExportServerEngineAttributeError>>
    {
        let mut request = SignedRequest::new("POST", "opsworks-cm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OpsWorksCM_V2016_11_01.ExportServerEngineAttribute",
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
                .deserialize::<ExportServerEngineAttributeResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ExportServerEngineAttributeError::from_response(response))
        }
    }

    /// <p>Returns a list of tags that are applied to the specified AWS OpsWorks for Chef Automate or AWS OpsWorks for Puppet Enterprise servers or backups.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let mut request = SignedRequest::new("POST", "opsworks-cm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorksCM_V2016_11_01.ListTagsForResource");
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
                .deserialize::<ListTagsForResourceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p> Restores a backup to a server that is in a <code>CONNECTION_LOST</code>, <code>HEALTHY</code>, <code>RUNNING</code>, <code>UNHEALTHY</code>, or <code>TERMINATED</code> state. When you run RestoreServer, the server's EC2 instance is deleted, and a new EC2 instance is configured. RestoreServer maintains the existing server endpoint, so configuration management of the server's client devices (nodes) should continue to work. </p> <p>Restoring from a backup is performed by creating a new EC2 instance. If restoration is successful, and the server is in a <code>HEALTHY</code> state, AWS OpsWorks CM switches traffic over to the new instance. After restoration is finished, the old EC2 instance is maintained in a <code>Running</code> or <code>Stopped</code> state, but is eventually terminated.</p> <p> This operation is asynchronous. </p> <p> An <code>InvalidStateException</code> is thrown when the server is not in a valid state. A <code>ResourceNotFoundException</code> is thrown when the server does not exist. A <code>ValidationException</code> is raised when parameters of the request are not valid. </p>
    async fn restore_server(
        &self,
        input: RestoreServerRequest,
    ) -> Result<RestoreServerResponse, RusotoError<RestoreServerError>> {
        let mut request = SignedRequest::new("POST", "opsworks-cm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorksCM_V2016_11_01.RestoreServer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<RestoreServerResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(RestoreServerError::from_response(response))
        }
    }

    /// <p> Manually starts server maintenance. This command can be useful if an earlier maintenance attempt failed, and the underlying cause of maintenance failure has been resolved. The server is in an <code>UNDER_MAINTENANCE</code> state while maintenance is in progress. </p> <p> Maintenance can only be started on servers in <code>HEALTHY</code> and <code>UNHEALTHY</code> states. Otherwise, an <code>InvalidStateException</code> is thrown. A <code>ResourceNotFoundException</code> is thrown when the server does not exist. A <code>ValidationException</code> is raised when parameters of the request are not valid. </p>
    async fn start_maintenance(
        &self,
        input: StartMaintenanceRequest,
    ) -> Result<StartMaintenanceResponse, RusotoError<StartMaintenanceError>> {
        let mut request = SignedRequest::new("POST", "opsworks-cm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorksCM_V2016_11_01.StartMaintenance");
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
                .deserialize::<StartMaintenanceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StartMaintenanceError::from_response(response))
        }
    }

    /// <p>Applies tags to an AWS OpsWorks for Chef Automate or AWS OpsWorks for Puppet Enterprise server, or to server backups.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let mut request = SignedRequest::new("POST", "opsworks-cm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorksCM_V2016_11_01.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<TagResourceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Removes specified tags from an AWS OpsWorks-CM server or backup.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let mut request = SignedRequest::new("POST", "opsworks-cm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorksCM_V2016_11_01.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UntagResourceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p> Updates settings for a server. </p> <p> This operation is synchronous. </p>
    async fn update_server(
        &self,
        input: UpdateServerRequest,
    ) -> Result<UpdateServerResponse, RusotoError<UpdateServerError>> {
        let mut request = SignedRequest::new("POST", "opsworks-cm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OpsWorksCM_V2016_11_01.UpdateServer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UpdateServerResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateServerError::from_response(response))
        }
    }

    /// <p> Updates engine-specific attributes on a specified server. The server enters the <code>MODIFYING</code> state when this operation is in progress. Only one update can occur at a time. You can use this command to reset a Chef server's public key (<code>CHEF_PIVOTAL_KEY</code>) or a Puppet server's admin password (<code>PUPPET_ADMIN_PASSWORD</code>). </p> <p> This operation is asynchronous. </p> <p> This operation can only be called for servers in <code>HEALTHY</code> or <code>UNHEALTHY</code> states. Otherwise, an <code>InvalidStateException</code> is raised. A <code>ResourceNotFoundException</code> is thrown when the server does not exist. A <code>ValidationException</code> is raised when parameters of the request are not valid. </p>
    async fn update_server_engine_attributes(
        &self,
        input: UpdateServerEngineAttributesRequest,
    ) -> Result<UpdateServerEngineAttributesResponse, RusotoError<UpdateServerEngineAttributesError>>
    {
        let mut request = SignedRequest::new("POST", "opsworks-cm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OpsWorksCM_V2016_11_01.UpdateServerEngineAttributes",
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
                .deserialize::<UpdateServerEngineAttributesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateServerEngineAttributesError::from_response(response))
        }
    }
}
