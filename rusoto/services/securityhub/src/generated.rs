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
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AcceptInvitationRequest {
    /// <p>The ID of the invitation sent from the Security Hub master account.</p>
    #[serde(rename = "InvitationId")]
    pub invitation_id: String,
    /// <p>The account ID of the Security Hub master account that sent the invitation.</p>
    #[serde(rename = "MasterId")]
    pub master_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AcceptInvitationResponse {}

/// <p>The details of an AWS account.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AccountDetails {
    /// <p>The ID of an AWS account.</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The email of an AWS account.</p>
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

/// <p>An <code>ActionTarget</code> object.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ActionTarget {
    /// <p>The ARN for the target action.</p>
    #[serde(rename = "ActionTargetArn")]
    pub action_target_arn: String,
    /// <p>The description of the target action.</p>
    #[serde(rename = "Description")]
    pub description: String,
    /// <p>The name of the action target.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// <p>Information about an Availability Zone.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AvailabilityZone {
    /// <p>The ID of the subnet. You can specify one subnet per Availability Zone.</p>
    #[serde(rename = "SubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    /// <p>The name of the Availability Zone.</p>
    #[serde(rename = "ZoneName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_name: Option<String>,
}

/// <p>A distribution configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsCloudFrontDistributionDetails {
    /// <p>The domain name corresponding to the distribution.</p>
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// <p>The entity tag is a hash of the object.</p>
    #[serde(rename = "ETag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    /// <p>The date and time that the distribution was last modified.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    /// <p>A complex type that controls whether access logs are written for the distribution.</p>
    #[serde(rename = "Logging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<AwsCloudFrontDistributionLogging>,
    /// <p>A complex type that contains information about origins for this distribution.</p>
    #[serde(rename = "Origins")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origins: Option<AwsCloudFrontDistributionOrigins>,
    /// <p>Indicates the current status of the distribution.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A unique identifier that specifies the AWS WAF web ACL, if any, to associate with this distribution.</p>
    #[serde(rename = "WebAclId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_acl_id: Option<String>,
}

/// <p>A complex type that controls whether access logs are written for the distribution.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsCloudFrontDistributionLogging {
    /// <p>The Amazon S3 bucket to store the access logs in.</p>
    #[serde(rename = "Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    /// <p>With this field, you can enable or disable the selected distribution.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>Specifies whether you want CloudFront to include cookies in access logs.</p>
    #[serde(rename = "IncludeCookies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_cookies: Option<bool>,
    /// <p>An optional string that you want CloudFront to use as a prefix to the access log filenames for this distribution.</p>
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

/// <p>A complex type that describes the Amazon S3 bucket, HTTP server (for example, a web server), Amazon Elemental MediaStore, or other server from which CloudFront gets your files.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsCloudFrontDistributionOriginItem {
    /// <p>Amazon S3 origins: The DNS name of the Amazon S3 bucket from which you want CloudFront to get objects for this origin.</p>
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// <p>A unique identifier for the origin or origin group.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>An optional element that causes CloudFront to request your content from a directory in your Amazon S3 bucket or your custom origin.</p>
    #[serde(rename = "OriginPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_path: Option<String>,
}

/// <p>A complex type that contains information about origins and origin groups for this distribution.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsCloudFrontDistributionOrigins {
    /// <p>A complex type that contains origins or origin groups for this distribution.</p>
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<AwsCloudFrontDistributionOriginItem>>,
}

/// <p>Information about an AWS CodeBuild project.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsCodeBuildProjectDetails {
    /// <p>The AWS Key Management Service (AWS KMS) customer master key (CMK) used to encrypt the build output artifacts.</p> <p>You can specify either the Amazon Resource Name (ARN) of the CMK or, if available, the CMK alias (using the format alias/alias-name). </p>
    #[serde(rename = "EncryptionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<String>,
    /// <p>Information about the build environment for this build project.</p>
    #[serde(rename = "Environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<AwsCodeBuildProjectEnvironment>,
    /// <p>The name of the build project.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ARN of the IAM role that enables AWS CodeBuild to interact with dependent AWS services on behalf of the AWS account.</p>
    #[serde(rename = "ServiceRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    /// <p>Information about the build input source code for this build project.</p>
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<AwsCodeBuildProjectSource>,
    /// <p>Information about the VPC configuration that AWS CodeBuild accesses.</p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<AwsCodeBuildProjectVpcConfig>,
}

/// <p>Information about the build environment for this build project.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsCodeBuildProjectEnvironment {
    /// <p>The certificate to use with this build project.</p>
    #[serde(rename = "Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    /// <p>The type of credentials AWS CodeBuild uses to pull images in your build.</p> <p>Valid values:</p> <ul> <li> <p> <code>CODEBUILD</code> specifies that AWS CodeBuild uses its own credentials. This requires that you modify your ECR repository policy to trust the AWS CodeBuild service principal.</p> </li> <li> <p> <code>SERVICE_ROLE</code> specifies that AWS CodeBuild uses your build project's service role.</p> </li> </ul> <p>When you use a cross-account or private registry image, you must use <code>SERVICE_ROLE</code> credentials. When you use an AWS CodeBuild curated image, you must use <code>CODEBUILD</code> credentials.</p>
    #[serde(rename = "ImagePullCredentialsType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_pull_credentials_type: Option<String>,
    /// <p>The credentials for access to a private registry.</p>
    #[serde(rename = "RegistryCredential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_credential: Option<AwsCodeBuildProjectEnvironmentRegistryCredential>,
    /// <p>The type of build environment to use for related builds.</p> <p>The environment type <code>ARM_CONTAINER</code> is available only in Regions US East (N. Virginia), US East (Ohio), US West (Oregon), Europe (Ireland), Asia Pacific (Mumbai), Asia Pacific (Tokyo), Asia Pacific (Sydney), and Europe (Frankfurt).</p> <p>The environment type <code>LINUX_CONTAINER</code> with compute type build.general1.2xlarge is available only in Regions US East (N. Virginia), US East (N. Virginia), US West (Oregon), Canada (Central), Europe (Ireland), Europe (London), Europe (Frankfurt), Asia Pacific (Tokyo), Asia Pacific (Seoul), Asia Pacific (Singapore), Asia Pacific (Sydney), China (Beijing), and China (Ningxia).</p> <p>The environment type <code>LINUX_GPU_CONTAINER</code> is available only in Regions US East (N. Virginia), US East (N. Virginia), US West (Oregon), Canada (Central), Europe (Ireland), Europe (London), Europe (Frankfurt), Asia Pacific (Tokyo), Asia Pacific (Seoul), Asia Pacific (Singapore), Asia Pacific (Sydney), China (Beijing), and China (Ningxia).</p> <p>Valid values: <code>WINDOWS_CONTAINER</code> | <code>LINUX_CONTAINER</code> | <code>LINUX_GPU_CONTAINER</code> | <code>ARM_CONTAINER</code> </p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>The credentials for access to a private registry.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsCodeBuildProjectEnvironmentRegistryCredential {
    /// <p><p>The Amazon Resource Name (ARN) or name of credentials created using AWS Secrets Manager.</p> <note> <p>The credential can use the name of the credentials only if they exist in your current AWS Region. </p> </note></p>
    #[serde(rename = "Credential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential: Option<String>,
    /// <p>The service that created the credentials to access a private Docker registry.</p> <p>The valid value,<code> SECRETS_MANAGER</code>, is for AWS Secrets Manager.</p>
    #[serde(rename = "CredentialProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_provider: Option<String>,
}

/// <p>Information about the build input source code for this build project.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsCodeBuildProjectSource {
    /// <p>Information about the Git clone depth for the build project.</p>
    #[serde(rename = "GitCloneDepth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_clone_depth: Option<i64>,
    /// <p>Whether to ignore SSL warnings while connecting to the project source code.</p>
    #[serde(rename = "InsecureSsl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insecure_ssl: Option<bool>,
    /// <p><p>Information about the location of the source code to be built.</p> <p>Valid values include:</p> <ul> <li> <p>For source code settings that are specified in the source action of a pipeline in AWS CodePipeline, location should not be specified. If it is specified, AWS CodePipeline ignores it. This is because AWS CodePipeline uses the settings in a pipeline&#39;s source action instead of this value.</p> </li> <li> <p>For source code in an AWS CodeCommit repository, the HTTPS clone URL to the repository that contains the source code and the build spec file (for example, <code>https://git-codecommit.region-ID.amazonaws.com/v1/repos/repo-name</code> ).</p> </li> <li> <p>For source code in an S3 input bucket, one of the following.</p> <ul> <li> <p>The path to the ZIP file that contains the source code (for example, <code>bucket-name/path/to/object-name.zip</code>).</p> </li> <li> <p> The path to the folder that contains the source code (for example, <code>bucket-name/path/to/source-code/folder/</code>).</p> </li> </ul> </li> <li> <p>For source code in a GitHub repository, the HTTPS clone URL to the repository that contains the source and the build spec file.</p> </li> <li> <p>For source code in a Bitbucket repository, the HTTPS clone URL to the repository that contains the source and the build spec file. </p> </li> </ul></p>
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// <p><p>The type of repository that contains the source code to be built. Valid values are:</p> <ul> <li> <p> <code>BITBUCKET</code> - The source code is in a Bitbucket repository.</p> </li> <li> <p> <code>CODECOMMIT</code> - The source code is in an AWS CodeCommit repository.</p> </li> <li> <p> <code>CODEPIPELINE</code> - The source code settings are specified in the source action of a pipeline in AWS CodePipeline.</p> </li> <li> <p> <code>GITHUB</code> - The source code is in a GitHub repository.</p> </li> <li> <p> <code>GITHUB<em>ENTERPRISE</code> - The source code is in a GitHub Enterprise repository.</p> </li> <li> <p> <code>NO</em>SOURCE</code> - The project does not have input source code.</p> </li> <li> <p> <code>S3</code> - The source code is in an S3 input bucket. </p> </li> </ul></p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Information about the VPC configuration that AWS CodeBuild accesses.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsCodeBuildProjectVpcConfig {
    /// <p>A list of one or more security group IDs in your Amazon VPC.</p>
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p>A list of one or more subnet IDs in your Amazon VPC.</p>
    #[serde(rename = "Subnets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<String>>,
    /// <p>The ID of the VPC.</p>
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

/// <p>The details of an Amazon EC2 instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsEc2InstanceDetails {
    /// <p>The IAM profile ARN of the instance.</p>
    #[serde(rename = "IamInstanceProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_instance_profile_arn: Option<String>,
    /// <p>The Amazon Machine Image (AMI) ID of the instance.</p>
    #[serde(rename = "ImageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    /// <p>The IPv4 addresses associated with the instance.</p>
    #[serde(rename = "IpV4Addresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_v4_addresses: Option<Vec<String>>,
    /// <p>The IPv6 addresses associated with the instance.</p>
    #[serde(rename = "IpV6Addresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_v6_addresses: Option<Vec<String>>,
    /// <p>The key name associated with the instance.</p>
    #[serde(rename = "KeyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    /// <p>The date/time the instance was launched.</p>
    #[serde(rename = "LaunchedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launched_at: Option<String>,
    /// <p>The identifier of the subnet that the instance was launched in.</p>
    #[serde(rename = "SubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    /// <p>The instance type of the instance. </p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The identifier of the VPC that the instance was launched in.</p>
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

/// <p>Information about the network interface attachment.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsEc2NetworkInterfaceAttachment {
    /// <p>The timestamp indicating when the attachment initiated.</p>
    #[serde(rename = "AttachTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_time: Option<String>,
    /// <p>The identifier of the network interface attachment</p>
    #[serde(rename = "AttachmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_id: Option<String>,
    /// <p>Indicates whether the network interface is deleted when the instance is terminated.</p>
    #[serde(rename = "DeleteOnTermination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_on_termination: Option<bool>,
    /// <p>The device index of the network interface attachment on the instance.</p>
    #[serde(rename = "DeviceIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_index: Option<i64>,
    /// <p>The ID of the instance.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>The AWS account ID of the owner of the instance.</p>
    #[serde(rename = "InstanceOwnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_owner_id: Option<String>,
    /// <p>The attachment state.</p> <p>Valid values: <code>attaching</code> | <code>attached</code> | <code>detaching</code> | <code>detached</code> </p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Details about the network interface</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsEc2NetworkInterfaceDetails {
    /// <p>The network interface attachment.</p>
    #[serde(rename = "Attachment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<AwsEc2NetworkInterfaceAttachment>,
    /// <p>The ID of the network interface.</p>
    #[serde(rename = "NetworkInterfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<String>,
    /// <p>Security groups for the network interface.</p>
    #[serde(rename = "SecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<AwsEc2NetworkInterfaceSecurityGroup>>,
    /// <p>Indicates whether traffic to or from the instance is validated.</p>
    #[serde(rename = "SourceDestCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_dest_check: Option<bool>,
}

/// <p>A security group associated with the network interface.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsEc2NetworkInterfaceSecurityGroup {
    /// <p>The ID of the security group.</p>
    #[serde(rename = "GroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// <p>The name of the security group.</p>
    #[serde(rename = "GroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}

/// <p>Details about an EC2 security group.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsEc2SecurityGroupDetails {
    /// <p>The ID of the security group.</p>
    #[serde(rename = "GroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// <p>The name of the security group.</p>
    #[serde(rename = "GroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// <p>The inbound rules associated with the security group.</p>
    #[serde(rename = "IpPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_permissions: Option<Vec<AwsEc2SecurityGroupIpPermission>>,
    /// <p>[VPC only] The outbound rules associated with the security group.</p>
    #[serde(rename = "IpPermissionsEgress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_permissions_egress: Option<Vec<AwsEc2SecurityGroupIpPermission>>,
    /// <p>The AWS account ID of the owner of the security group.</p>
    #[serde(rename = "OwnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    /// <p>[VPC only] The ID of the VPC for the security group.</p>
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

/// <p>An IP permission for an EC2 security group.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsEc2SecurityGroupIpPermission {
    /// <p>The start of the port range for the TCP and UDP protocols, or an ICMP/ICMPv6 type number.</p> <p>A value of -1 indicates all ICMP/ICMPv6 types. If you specify all ICMP/ICMPv6 types, you must specify all codes. </p>
    #[serde(rename = "FromPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_port: Option<i64>,
    /// <p>The IP protocol name (<code>tcp</code>, <code>udp</code>, <code>icmp</code>, <code>icmpv6</code>) or number.</p> <p>[VPC only] Use <code>-1</code> to specify all protocols.</p> <p>When authorizing security group rules, specifying -1 or a protocol number other than <code>tcp</code>, <code>udp</code>, <code>icmp</code>, or <code>icmpv6</code> allows traffic on all ports, regardless of any port range you specify.</p> <p>For <code>tcp</code>, <code>udp</code>, and <code>icmp</code>, you must specify a port range.</p> <p>For <code>icmpv6</code>, the port range is optional. If you omit the port range, traffic for all types and codes is allowed. </p>
    #[serde(rename = "IpProtocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_protocol: Option<String>,
    /// <p>The IPv4 ranges.</p>
    #[serde(rename = "IpRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_ranges: Option<Vec<AwsEc2SecurityGroupIpRange>>,
    /// <p>The IPv6 ranges.</p>
    #[serde(rename = "Ipv6Ranges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv_6_ranges: Option<Vec<AwsEc2SecurityGroupIpv6Range>>,
    /// <p>[VPC only] The prefix list IDs for an AWS service. With outbound rules, this is the AWS service to access through a VPC endpoint from instances associated with the security group.</p>
    #[serde(rename = "PrefixListIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_list_ids: Option<Vec<AwsEc2SecurityGroupPrefixListId>>,
    /// <p>The end of the port range for the TCP and UDP protocols, or an ICMP/ICMPv6 code.</p> <p>A value of -1 indicates all ICMP/ICMPv6 codes. If you specify all ICMP/ICMPv6 types, you must specify all codes.</p>
    #[serde(rename = "ToPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_port: Option<i64>,
    /// <p>The security group and AWS account ID pairs.</p>
    #[serde(rename = "UserIdGroupPairs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_group_pairs: Option<Vec<AwsEc2SecurityGroupUserIdGroupPair>>,
}

/// <p>A range of IPv4 addresses.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsEc2SecurityGroupIpRange {
    /// <p>The IPv4 CIDR range. You can specify either a CIDR range or a source security group, but not both. To specify a single IPv4 address, use the /32 prefix length.</p>
    #[serde(rename = "CidrIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_ip: Option<String>,
}

/// <p>A range of IPv6 addresses.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsEc2SecurityGroupIpv6Range {
    /// <p>The IPv6 CIDR range. You can specify either a CIDR range or a source security group, but not both. To specify a single IPv6 address, use the /128 prefix length.</p>
    #[serde(rename = "CidrIpv6")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_ipv_6: Option<String>,
}

/// <p>A prefix list ID.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsEc2SecurityGroupPrefixListId {
    /// <p>The ID of the prefix.</p>
    #[serde(rename = "PrefixListId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_list_id: Option<String>,
}

/// <p>A relationship between a security group and a user.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsEc2SecurityGroupUserIdGroupPair {
    /// <p>The ID of the security group.</p>
    #[serde(rename = "GroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// <p>The name of the security group.</p>
    #[serde(rename = "GroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// <p>The status of a VPC peering connection, if applicable.</p>
    #[serde(rename = "PeeringStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peering_status: Option<String>,
    /// <p>The ID of an AWS account.</p> <p>For a referenced security group in another VPC, the account ID of the referenced security group is returned in the response. If the referenced security group is deleted, this value is not returned.</p> <p>[EC2-Classic] Required when adding or removing rules that reference a security group in another AWS. </p>
    #[serde(rename = "UserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// <p>The ID of the VPC for the referenced security group, if applicable.</p>
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// <p>The ID of the VPC peering connection, if applicable.</p>
    #[serde(rename = "VpcPeeringConnectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_peering_connection_id: Option<String>,
}

/// <p>Information about an Elasticsearch domain.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsElasticsearchDomainDetails {
    /// <p>IAM policy document specifying the access policies for the new Amazon ES domain.</p>
    #[serde(rename = "AccessPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_policies: Option<String>,
    /// <p>Additional options for the domain endpoint.</p>
    #[serde(rename = "DomainEndpointOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_endpoint_options: Option<AwsElasticsearchDomainDomainEndpointOptions>,
    /// <p>Unique identifier for an Amazon ES domain.</p>
    #[serde(rename = "DomainId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
    /// <p>Name of an Amazon ES domain.</p> <p>Domain names are unique across all domains owned by the same account within an AWS Region.</p> <p>Domain names must start with a lowercase letter and must be between 3 and 28 characters.</p> <p>Valid characters are a-z (lowercase only), 0-9, and – (hyphen). </p>
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// <p>Elasticsearch version.</p>
    #[serde(rename = "ElasticsearchVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch_version: Option<String>,
    /// <p>Details about the configuration for encryption at rest.</p>
    #[serde(rename = "EncryptionAtRestOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_at_rest_options: Option<AwsElasticsearchDomainEncryptionAtRestOptions>,
    /// <p>Domain-specific endpoint used to submit index, search, and data upload requests to an Amazon ES domain.</p> <p>The endpoint is a service URL. </p>
    #[serde(rename = "Endpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    /// <p>The key-value pair that exists if the Amazon ES domain uses VPC endpoints.</p>
    #[serde(rename = "Endpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<::std::collections::HashMap<String, String>>,
    /// <p>Details about the configuration for node-to-node encryption.</p>
    #[serde(rename = "NodeToNodeEncryptionOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_to_node_encryption_options: Option<AwsElasticsearchDomainNodeToNodeEncryptionOptions>,
    /// <p>Information that Amazon ES derives based on <code>VPCOptions</code> for the domain.</p>
    #[serde(rename = "VPCOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_options: Option<AwsElasticsearchDomainVPCOptions>,
}

/// <p>Additional options for the domain endpoint, such as whether to require HTTPS for all traffic.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsElasticsearchDomainDomainEndpointOptions {
    /// <p>Whether to require that all traffic to the domain arrive over HTTPS.</p>
    #[serde(rename = "EnforceHTTPS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforce_https: Option<bool>,
    /// <p><p>The TLS security policy to apply to the HTTPS endpoint of the Elasticsearch domain.</p> <p>Valid values:</p> <ul> <li> <p> <code>Policy-Min-TLS-1-0-2019-07</code>, which supports TLSv1.0 and higher</p> </li> <li> <p> <code>Policy-Min-TLS-1-2-2019-07</code>, which only supports TLSv1.2</p> </li> </ul></p>
    #[serde(rename = "TLSSecurityPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_security_policy: Option<String>,
}

/// <p>Details about the configuration for encryption at rest.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsElasticsearchDomainEncryptionAtRestOptions {
    /// <p>Whether encryption at rest is enabled.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The KMS key ID. Takes the form 1a2a3a4-1a2a-3a4a-5a6a-1a2a3a4a5a6a.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
}

/// <p>Details about the configuration for node-to-node encryption.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsElasticsearchDomainNodeToNodeEncryptionOptions {
    /// <p>Whether node-to-node encryption is enabled.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// <p>Information that Amazon ES derives based on <code>VPCOptions</code> for the domain.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsElasticsearchDomainVPCOptions {
    /// <p>The list of Availability Zones associated with the VPC subnets.</p>
    #[serde(rename = "AvailabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    /// <p>The list of security group IDs associated with the VPC endpoints for the domain.</p>
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p>A list of subnet IDs associated with the VPC endpoints for the domain.</p>
    #[serde(rename = "SubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    /// <p>ID for the VPC.</p>
    #[serde(rename = "VPCId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

/// <p>Information about a load balancer.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsElbv2LoadBalancerDetails {
    /// <p>The Availability Zones for the load balancer.</p>
    #[serde(rename = "AvailabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<AvailabilityZone>>,
    /// <p>The ID of the Amazon Route 53 hosted zone associated with the load balancer.</p>
    #[serde(rename = "CanonicalHostedZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical_hosted_zone_id: Option<String>,
    /// <p>The date and time the load balancer was created.</p>
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// <p>The public DNS name of the load balancer.</p>
    #[serde(rename = "DNSName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,
    /// <p>The type of IP addresses used by the subnets for your load balancer. The possible values are <code>ipv4</code> (for IPv4 addresses) and <code>dualstack</code> (for IPv4 and IPv6 addresses).</p>
    #[serde(rename = "IpAddressType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    /// <p>The nodes of an Internet-facing load balancer have public IP addresses.</p>
    #[serde(rename = "Scheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
    /// <p>The IDs of the security groups for the load balancer.</p>
    #[serde(rename = "SecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    /// <p>The state of the load balancer.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<LoadBalancerState>,
    /// <p>The type of load balancer.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The ID of the VPC for the load balancer.</p>
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

/// <p>IAM access key details related to a finding.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsIamAccessKeyDetails {
    /// <p>The creation date/time of the IAM access key related to a finding.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// <p>The ID of the principal associated with an access key.</p>
    #[serde(rename = "PrincipalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    /// <p>The name of the principal.</p>
    #[serde(rename = "PrincipalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_name: Option<String>,
    /// <p>The type of principal associated with an access key.</p>
    #[serde(rename = "PrincipalType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<String>,
    /// <p>The status of the IAM access key related to a finding.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Contains information about an IAM role, including all of the role's policies.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsIamRoleDetails {
    /// <p>The trust policy that grants permission to assume the role.</p>
    #[serde(rename = "AssumeRolePolicyDocument")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assume_role_policy_document: Option<String>,
    /// <p>The date and time, in ISO 8601 date-time format, when the role was created.</p>
    #[serde(rename = "CreateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    /// <p>The maximum session duration (in seconds) that you want to set for the specified role.</p>
    #[serde(rename = "MaxSessionDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_session_duration: Option<i64>,
    /// <p>The path to the role.</p>
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// <p>The stable and unique string identifying the role.</p>
    #[serde(rename = "RoleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
    /// <p>The friendly name that identifies the role.</p>
    #[serde(rename = "RoleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
}

/// <p>Contains metadata about a customer master key (CMK).</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsKmsKeyDetails {
    /// <p>The twelve-digit account ID of the AWS account that owns the CMK.</p>
    #[serde(rename = "AWSAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,
    /// <p>The date and time when the CMK was created.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>The globally unique identifier for the CMK.</p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>The manager of the CMK. CMKs in your AWS account are either customer managed or AWS managed.</p>
    #[serde(rename = "KeyManager")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_manager: Option<String>,
    /// <p>The state of the CMK.</p>
    #[serde(rename = "KeyState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_state: Option<String>,
    /// <p>The source of the CMK's key material.</p> <p>When this value is <code>AWS_KMS</code>, AWS KMS created the key material.</p> <p>When this value is <code>EXTERNAL</code>, the key material was imported from your existing key management infrastructure or the CMK lacks key material.</p> <p>When this value is <code>AWS_CLOUDHSM</code>, the key material was created in the AWS CloudHSM cluster associated with a custom key store.</p>
    #[serde(rename = "Origin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}

/// <p>The code for the Lambda function. You can specify either an object in Amazon S3, or upload a deployment package directly.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsLambdaFunctionCode {
    /// <p>An Amazon S3 bucket in the same AWS Region as your function. The bucket can be in a different AWS account.</p>
    #[serde(rename = "S3Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket: Option<String>,
    /// <p>The Amazon S3 key of the deployment package.</p>
    #[serde(rename = "S3Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key: Option<String>,
    /// <p>For versioned objects, the version of the deployment package object to use.</p>
    #[serde(rename = "S3ObjectVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object_version: Option<String>,
    /// <p>The base64-encoded contents of the deployment package. AWS SDK and AWS CLI clients handle the encoding for you.</p>
    #[serde(rename = "ZipFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_file: Option<String>,
}

/// <p>The dead-letter queue for failed asynchronous invocations.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsLambdaFunctionDeadLetterConfig {
    /// <p>The Amazon Resource Name (ARN) of an Amazon SQS queue or Amazon SNS topic.</p>
    #[serde(rename = "TargetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
}

/// <p>Details about a function's configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsLambdaFunctionDetails {
    /// <p>An <code>AwsLambdaFunctionCode</code> object.</p>
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<AwsLambdaFunctionCode>,
    /// <p>The SHA256 hash of the function's deployment package.</p>
    #[serde(rename = "CodeSha256")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_sha_256: Option<String>,
    /// <p>The function's dead letter queue.</p>
    #[serde(rename = "DeadLetterConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_letter_config: Option<AwsLambdaFunctionDeadLetterConfig>,
    /// <p>The function's environment variables.</p>
    #[serde(rename = "Environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<AwsLambdaFunctionEnvironment>,
    /// <p>The name of the function.</p>
    #[serde(rename = "FunctionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_name: Option<String>,
    /// <p>The function that Lambda calls to begin executing your function.</p>
    #[serde(rename = "Handler")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handler: Option<String>,
    /// <p>The KMS key that's used to encrypt the function's environment variables. This key is only returned if you've configured a customer managed CMK.</p>
    #[serde(rename = "KmsKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    /// <p>The date and time that the function was last updated, in ISO-8601 format (YYYY-MM-DDThh:mm:ss.sTZD).</p>
    #[serde(rename = "LastModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    /// <p>The function's layers.</p>
    #[serde(rename = "Layers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<AwsLambdaFunctionLayer>>,
    /// <p>For Lambda@Edge functions, the ARN of the master function.</p>
    #[serde(rename = "MasterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_arn: Option<String>,
    /// <p>The memory that's allocated to the function.</p>
    #[serde(rename = "MemorySize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_size: Option<i64>,
    /// <p>The latest updated revision of the function or alias.</p>
    #[serde(rename = "RevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    /// <p>The function's execution role.</p>
    #[serde(rename = "Role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// <p>The runtime environment for the Lambda function.</p>
    #[serde(rename = "Runtime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
    /// <p>The amount of time that Lambda allows a function to run before stopping it.</p>
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    /// <p>The function's AWS X-Ray tracing configuration.</p>
    #[serde(rename = "TracingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracing_config: Option<AwsLambdaFunctionTracingConfig>,
    /// <p>The version of the Lambda function.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// <p>The function's networking configuration.</p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<AwsLambdaFunctionVpcConfig>,
}

/// <p>A function's environment variable settings.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsLambdaFunctionEnvironment {
    /// <p>An <code>AwsLambdaFunctionEnvironmentError</code> object.</p>
    #[serde(rename = "Error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<AwsLambdaFunctionEnvironmentError>,
    /// <p>Environment variable key-value pairs.</p>
    #[serde(rename = "Variables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Error messages for environment variables that couldn't be applied.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsLambdaFunctionEnvironmentError {
    /// <p>The error code.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The error message.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// <p>An AWS Lambda layer.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsLambdaFunctionLayer {
    /// <p>The Amazon Resource Name (ARN) of the function layer.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The size of the layer archive in bytes.</p>
    #[serde(rename = "CodeSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_size: Option<i64>,
}

/// <p>The function's AWS X-Ray tracing configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsLambdaFunctionTracingConfig {
    /// <p>The tracing mode.</p>
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

/// <p>The VPC security groups and subnets that are attached to a Lambda function. For more information, see VPC Settings.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsLambdaFunctionVpcConfig {
    /// <p>A list of VPC security groups IDs.</p>
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p>A list of VPC subnet IDs.</p>
    #[serde(rename = "SubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    /// <p>The ID of the VPC.</p>
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

/// <p>Details about a Lambda layer version.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsLambdaLayerVersionDetails {
    /// <p>The layer's compatible runtimes. Maximum number of five items.</p> <p>Valid values: <code>nodejs10.x</code> | <code>nodejs12.x</code> | <code>java8</code> | <code>java11</code> | <code>python2.7</code> | <code>python3.6</code> | <code>python3.7</code> | <code>python3.8</code> | <code>dotnetcore1.0</code> | <code>dotnetcore2.1</code> | <code>go1.x</code> | <code>ruby2.5</code> | <code>provided</code> </p>
    #[serde(rename = "CompatibleRuntimes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_runtimes: Option<Vec<String>>,
    /// <p>The date that the version was created, in ISO 8601 format. For example, 2018-11-27T15:10:45.123+0000. </p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    /// <p>The version number.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>An AWS Identity and Access Management (IAM) role associated with the DB instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsRdsDbInstanceAssociatedRole {
    /// <p>The name of the feature associated with the IAM)role.</p>
    #[serde(rename = "FeatureName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the IAM role that is associated with the DB instance.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p><p>Describes the state of the association between the IAM role and the DB instance. The <code>Status</code> property returns one of the following values:</p> <ul> <li> <p> <code>ACTIVE</code> - The IAM role ARN is associated with the DB instance and can be used to access other AWS services on your behalf.</p> </li> <li> <p> <code>PENDING</code> - The IAM role ARN is being associated with the DB instance.</p> </li> <li> <p> <code>INVALID</code> - The IAM role ARN is associated with the DB instance. But the DB instance is unable to assume the IAM role in order to access other AWS services on your behalf. </p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Contains the details of an Amazon RDS DB instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsRdsDbInstanceDetails {
    /// <p>The AWS Identity and Access Management (IAM) roles associated with the DB instance.</p>
    #[serde(rename = "AssociatedRoles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_roles: Option<Vec<AwsRdsDbInstanceAssociatedRole>>,
    /// <p>The identifier of the CA certificate for this DB instance.</p>
    #[serde(rename = "CACertificateIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_certificate_identifier: Option<String>,
    /// <p>If the DB instance is a member of a DB cluster, contains the name of the DB cluster that the DB instance is a member of.</p>
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_cluster_identifier: Option<String>,
    /// <p>Contains the name of the compute and memory capacity class of the DB instance.</p>
    #[serde(rename = "DBInstanceClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_class: Option<String>,
    /// <p>Contains a user-supplied database identifier. This identifier is the unique key that identifies a DB instance.</p>
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_identifier: Option<String>,
    /// <p>The meaning of this parameter differs according to the database engine you use.</p> <p> <b>MySQL, MariaDB, SQL Server, PostgreSQL</b> </p> <p>Contains the name of the initial database of this instance that was provided at create time, if one was specified when the DB instance was created. This same name is returned for the life of the DB instance.</p> <p> <b>Oracle</b> </p> <p>Contains the Oracle System ID (SID) of the created DB instance. Not shown when the returned parameters do not apply to an Oracle DB instance. </p>
    #[serde(rename = "DBName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_name: Option<String>,
    /// <p>Specifies the port that the DB instance listens on. If the DB instance is part of a DB cluster, this can be a different port than the DB cluster port.</p>
    #[serde(rename = "DbInstancePort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_port: Option<i64>,
    /// <p>The AWS Region-unique, immutable identifier for the DB instance. This identifier is found in AWS CloudTrail log entries whenever the AWS KMS key for the DB instance is accessed. </p>
    #[serde(rename = "DbiResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbi_resource_id: Option<String>,
    /// <p>Indicates whether the DB instance has deletion protection enabled.</p> <p>When deletion protection is enabled, the database cannot be deleted.</p>
    #[serde(rename = "DeletionProtection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    /// <p>Specifies the connection endpoint.</p>
    #[serde(rename = "Endpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<AwsRdsDbInstanceEndpoint>,
    /// <p>Provides the name of the database engine to use for this DB instance.</p>
    #[serde(rename = "Engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    /// <p>Indicates the database engine version.</p>
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// <p><p>True if mapping of AWS Identity and Access Management (IAM) accounts to database accounts is enabled, and otherwise false.</p> <p>IAM database authentication can be enabled for the following database engines.</p> <ul> <li> <p>For MySQL 5.6, minor version 5.6.34 or higher</p> </li> <li> <p>For MySQL 5.7, minor version 5.7.16 or higher</p> </li> <li> <p>Aurora 5.6 or higher</p> </li> </ul></p>
    #[serde(rename = "IAMDatabaseAuthenticationEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_database_authentication_enabled: Option<bool>,
    /// <p>Provides the date and time the DB instance was created.</p>
    #[serde(rename = "InstanceCreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_create_time: Option<String>,
    /// <p>If <code>StorageEncrypted</code> is true, the AWS KMS key identifier for the encrypted DB instance.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>Specifies the accessibility options for the DB instance.</p> <p>A value of true specifies an Internet-facing instance with a publicly resolvable DNS name, which resolves to a public IP address.</p> <p>A value of false specifies an internal instance with a DNS name that resolves to a private IP address. </p>
    #[serde(rename = "PubliclyAccessible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    /// <p>Specifies whether the DB instance is encrypted.</p>
    #[serde(rename = "StorageEncrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encrypted: Option<bool>,
    /// <p>The ARN from the key store with which the instance is associated for TDE encryption.</p>
    #[serde(rename = "TdeCredentialArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tde_credential_arn: Option<String>,
    /// <p>A list of VPC security groups that the DB instance belongs to.</p>
    #[serde(rename = "VpcSecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_groups: Option<Vec<AwsRdsDbInstanceVpcSecurityGroup>>,
}

/// <p>Specifies the connection endpoint.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsRdsDbInstanceEndpoint {
    /// <p>Specifies the DNS address of the DB instance.</p>
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// <p>Specifies the ID that Amazon Route 53 assigns when you create a hosted zone.</p>
    #[serde(rename = "HostedZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_id: Option<String>,
    /// <p>Specifies the port that the database engine is listening on.</p>
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
}

/// <p>A VPC security groups that the DB instance belongs to.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsRdsDbInstanceVpcSecurityGroup {
    /// <p>The status of the VPC security group.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The name of the VPC security group.</p>
    #[serde(rename = "VpcSecurityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_id: Option<String>,
}

/// <p>The details of an Amazon S3 bucket.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsS3BucketDetails {
    /// <p>The date and time when the S3 bucket was created.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// <p>The canonical user ID of the owner of the S3 bucket.</p>
    #[serde(rename = "OwnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    /// <p>The display name of the owner of the S3 bucket.</p>
    #[serde(rename = "OwnerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_name: Option<String>,
    /// <p>The encryption rules that are applied to the S3 bucket.</p>
    #[serde(rename = "ServerSideEncryptionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_side_encryption_configuration: Option<AwsS3BucketServerSideEncryptionConfiguration>,
}

/// <p>Specifies the default server-side encryption to apply to new objects in the bucket.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsS3BucketServerSideEncryptionByDefault {
    /// <p>AWS KMS customer master key (CMK) ID to use for the default encryption.</p>
    #[serde(rename = "KMSMasterKeyID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_master_key_id: Option<String>,
    /// <p>Server-side encryption algorithm to use for the default encryption.</p>
    #[serde(rename = "SSEAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sse_algorithm: Option<String>,
}

/// <p>The encryption configuration for the S3 bucket.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsS3BucketServerSideEncryptionConfiguration {
    /// <p>The encryption rules that are applied to the S3 bucket.</p>
    #[serde(rename = "Rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<AwsS3BucketServerSideEncryptionRule>>,
}

/// <p>An encryption rule to apply to the S3 bucket.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsS3BucketServerSideEncryptionRule {
    /// <p>Specifies the default server-side encryption to apply to new objects in the bucket. If a <code>PUT</code> object request doesn't specify any server-side encryption, this default encryption is applied.</p>
    #[serde(rename = "ApplyServerSideEncryptionByDefault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_server_side_encryption_by_default: Option<AwsS3BucketServerSideEncryptionByDefault>,
}

/// <p>Details about an Amazon S3 object.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsS3ObjectDetails {
    /// <p>A standard MIME type describing the format of the object data.</p>
    #[serde(rename = "ContentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// <p>The opaque identifier assigned by a web server to a specific version of a resource found at a URL.</p>
    #[serde(rename = "ETag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    /// <p>The date and time when the object was last modified.</p>
    #[serde(rename = "LastModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    /// <p>The identifier of the AWS Key Management Service (AWS KMS) symmetric customer managed customer master key (CMK) that was used for the object.</p>
    #[serde(rename = "SSEKMSKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssekms_key_id: Option<String>,
    /// <p>If the object is stored using server-side encryption, the value of the server-side encryption algorithm used when storing this object in Amazon S3.</p>
    #[serde(rename = "ServerSideEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_side_encryption: Option<String>,
    /// <p>The version of the object.</p>
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

/// <p><p>Provides consistent format for the contents of the Security Hub-aggregated findings. <code>AwsSecurityFinding</code> format enables you to share findings between AWS security services and third-party solutions, and security standards checks.</p> <note> <p>A finding is a potential security issue generated either by AWS services (Amazon GuardDuty, Amazon Inspector, and Amazon Macie) or by the integrated third-party solutions and standards checks.</p> </note></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsSecurityFinding {
    /// <p>The AWS account ID that a finding is generated in.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>This data type is exclusive to findings that are generated as the result of a check run against a specific rule in a supported security standard, such as CIS AWS Foundations. Contains security standard-related finding details.</p>
    #[serde(rename = "Compliance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance: Option<Compliance>,
    /// <p>A finding's confidence. Confidence is defined as the likelihood that a finding accurately identifies the behavior or issue that it was intended to identify.</p> <p>Confidence is scored on a 0-100 basis using a ratio scale, where 0 means zero percent confidence and 100 means 100 percent confidence.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<i64>,
    /// <p>An ISO8601-formatted timestamp that indicates when the security-findings provider created the potential security issue that a finding captured.</p>
    #[serde(rename = "CreatedAt")]
    pub created_at: String,
    /// <p>The level of importance assigned to the resources associated with the finding.</p> <p>A score of 0 means that the underlying resources have no criticality, and a score of 100 is reserved for the most critical resources.</p>
    #[serde(rename = "Criticality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criticality: Option<i64>,
    /// <p><p>A finding&#39;s description.</p> <note> <p>In this release, <code>Description</code> is a required property.</p> </note></p>
    #[serde(rename = "Description")]
    pub description: String,
    /// <p>An ISO8601-formatted timestamp that indicates when the security-findings provider first observed the potential security issue that a finding captured.</p>
    #[serde(rename = "FirstObservedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_observed_at: Option<String>,
    /// <p>The identifier for the solution-specific component (a discrete unit of logic) that generated a finding. In various security-findings providers' solutions, this generator can be called a rule, a check, a detector, a plugin, etc. </p>
    #[serde(rename = "GeneratorId")]
    pub generator_id: String,
    /// <p>The security findings provider-specific identifier for a finding.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>An ISO8601-formatted timestamp that indicates when the security-findings provider most recently observed the potential security issue that a finding captured.</p>
    #[serde(rename = "LastObservedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_observed_at: Option<String>,
    /// <p>A list of malware related to a finding.</p>
    #[serde(rename = "Malware")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malware: Option<Vec<Malware>>,
    /// <p>The details of network-related information about a finding.</p>
    #[serde(rename = "Network")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<Network>,
    /// <p>A user-defined note added to a finding.</p>
    #[serde(rename = "Note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<Note>,
    /// <p>The details of process-related information about a finding.</p>
    #[serde(rename = "Process")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process: Option<ProcessDetails>,
    /// <p>The ARN generated by Security Hub that uniquely identifies a product that generates findings. This can be the ARN for a third-party product that is integrated with Security Hub, or the ARN for a custom integration.</p>
    #[serde(rename = "ProductArn")]
    pub product_arn: String,
    /// <p>A data type where security-findings providers can include additional solution-specific details that aren't part of the defined <code>AwsSecurityFinding</code> format.</p>
    #[serde(rename = "ProductFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_fields: Option<::std::collections::HashMap<String, String>>,
    /// <p>The record state of a finding.</p>
    #[serde(rename = "RecordState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_state: Option<String>,
    /// <p>A list of related findings.</p>
    #[serde(rename = "RelatedFindings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_findings: Option<Vec<RelatedFinding>>,
    /// <p>A data type that describes the remediation options for a finding.</p>
    #[serde(rename = "Remediation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediation: Option<Remediation>,
    /// <p>A set of resource data types that describe the resources that the finding refers to.</p>
    #[serde(rename = "Resources")]
    pub resources: Vec<Resource>,
    /// <p>The schema version that a finding is formatted for.</p>
    #[serde(rename = "SchemaVersion")]
    pub schema_version: String,
    /// <p>A finding's severity.</p>
    #[serde(rename = "Severity")]
    pub severity: Severity,
    /// <p>A URL that links to a page about the current finding in the security-findings provider's solution.</p>
    #[serde(rename = "SourceUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    /// <p>Threat intelligence details related to a finding.</p>
    #[serde(rename = "ThreatIntelIndicators")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_intel_indicators: Option<Vec<ThreatIntelIndicator>>,
    /// <p><p>A finding&#39;s title.</p> <note> <p>In this release, <code>Title</code> is a required property.</p> </note></p>
    #[serde(rename = "Title")]
    pub title: String,
    /// <p>One or more finding types in the format of <code>namespace/category/classifier</code> that classify a finding.</p> <p>Valid namespace values are: Software and Configuration Checks | TTPs | Effects | Unusual Behaviors | Sensitive Data Identifications</p>
    #[serde(rename = "Types")]
    pub types: Vec<String>,
    /// <p>An ISO8601-formatted timestamp that indicates when the security-findings provider last updated the finding record. </p>
    #[serde(rename = "UpdatedAt")]
    pub updated_at: String,
    /// <p>A list of name/value string pairs associated with the finding. These are custom, user-defined fields added to a finding. </p>
    #[serde(rename = "UserDefinedFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_defined_fields: Option<::std::collections::HashMap<String, String>>,
    /// <p>Indicates the veracity of a finding. </p>
    #[serde(rename = "VerificationState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_state: Option<String>,
    /// <p>Provides information about the status of the investigation into a finding.</p>
    #[serde(rename = "Workflow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow: Option<Workflow>,
    /// <p>The workflow state of a finding. </p>
    #[serde(rename = "WorkflowState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_state: Option<String>,
}

/// <p>A collection of attributes that are applied to all active Security Hub-aggregated findings and that result in a subset of findings that are included in this insight.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsSecurityFindingFilters {
    /// <p>The AWS account ID that a finding is generated in.</p>
    #[serde(rename = "AwsAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<Vec<StringFilter>>,
    /// <p>The name of the findings provider (company) that owns the solution (product) that generates findings.</p>
    #[serde(rename = "CompanyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<Vec<StringFilter>>,
    /// <p>Exclusive to findings that are generated as the result of a check run against a specific rule in a supported standard, such as CIS AWS Foundations. Contains security standard-related finding details.</p>
    #[serde(rename = "ComplianceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_status: Option<Vec<StringFilter>>,
    /// <p>A finding's confidence. Confidence is defined as the likelihood that a finding accurately identifies the behavior or issue that it was intended to identify.</p> <p>Confidence is scored on a 0-100 basis using a ratio scale, where 0 means zero percent confidence and 100 means 100 percent confidence.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<Vec<NumberFilter>>,
    /// <p>An ISO8601-formatted timestamp that indicates when the security-findings provider captured the potential security issue that a finding captured.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<DateFilter>>,
    /// <p>The level of importance assigned to the resources associated with the finding.</p> <p>A score of 0 means that the underlying resources have no criticality, and a score of 100 is reserved for the most critical resources.</p>
    #[serde(rename = "Criticality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criticality: Option<Vec<NumberFilter>>,
    /// <p>A finding's description.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<StringFilter>>,
    /// <p>An ISO8601-formatted timestamp that indicates when the security-findings provider first observed the potential security issue that a finding captured.</p>
    #[serde(rename = "FirstObservedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_observed_at: Option<Vec<DateFilter>>,
    /// <p>The identifier for the solution-specific component (a discrete unit of logic) that generated a finding. In various security-findings providers' solutions, this generator can be called a rule, a check, a detector, a plugin, etc.</p>
    #[serde(rename = "GeneratorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generator_id: Option<Vec<StringFilter>>,
    /// <p>The security findings provider-specific identifier for a finding.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Vec<StringFilter>>,
    /// <p>A keyword for a finding.</p>
    #[serde(rename = "Keyword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<Vec<KeywordFilter>>,
    /// <p>An ISO8601-formatted timestamp that indicates when the security-findings provider most recently observed the potential security issue that a finding captured.</p>
    #[serde(rename = "LastObservedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_observed_at: Option<Vec<DateFilter>>,
    /// <p>The name of the malware that was observed.</p>
    #[serde(rename = "MalwareName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malware_name: Option<Vec<StringFilter>>,
    /// <p>The filesystem path of the malware that was observed.</p>
    #[serde(rename = "MalwarePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malware_path: Option<Vec<StringFilter>>,
    /// <p>The state of the malware that was observed.</p>
    #[serde(rename = "MalwareState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malware_state: Option<Vec<StringFilter>>,
    /// <p>The type of the malware that was observed.</p>
    #[serde(rename = "MalwareType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malware_type: Option<Vec<StringFilter>>,
    /// <p>The destination domain of network-related information about a finding.</p>
    #[serde(rename = "NetworkDestinationDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_destination_domain: Option<Vec<StringFilter>>,
    /// <p>The destination IPv4 address of network-related information about a finding.</p>
    #[serde(rename = "NetworkDestinationIpV4")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_destination_ip_v4: Option<Vec<IpFilter>>,
    /// <p>The destination IPv6 address of network-related information about a finding.</p>
    #[serde(rename = "NetworkDestinationIpV6")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_destination_ip_v6: Option<Vec<IpFilter>>,
    /// <p>The destination port of network-related information about a finding.</p>
    #[serde(rename = "NetworkDestinationPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_destination_port: Option<Vec<NumberFilter>>,
    /// <p>Indicates the direction of network traffic associated with a finding.</p>
    #[serde(rename = "NetworkDirection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_direction: Option<Vec<StringFilter>>,
    /// <p>The protocol of network-related information about a finding.</p>
    #[serde(rename = "NetworkProtocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_protocol: Option<Vec<StringFilter>>,
    /// <p>The source domain of network-related information about a finding.</p>
    #[serde(rename = "NetworkSourceDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_source_domain: Option<Vec<StringFilter>>,
    /// <p>The source IPv4 address of network-related information about a finding.</p>
    #[serde(rename = "NetworkSourceIpV4")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_source_ip_v4: Option<Vec<IpFilter>>,
    /// <p>The source IPv6 address of network-related information about a finding.</p>
    #[serde(rename = "NetworkSourceIpV6")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_source_ip_v6: Option<Vec<IpFilter>>,
    /// <p>The source media access control (MAC) address of network-related information about a finding.</p>
    #[serde(rename = "NetworkSourceMac")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_source_mac: Option<Vec<StringFilter>>,
    /// <p>The source port of network-related information about a finding.</p>
    #[serde(rename = "NetworkSourcePort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_source_port: Option<Vec<NumberFilter>>,
    /// <p>The text of a note.</p>
    #[serde(rename = "NoteText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_text: Option<Vec<StringFilter>>,
    /// <p>The timestamp of when the note was updated.</p>
    #[serde(rename = "NoteUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_updated_at: Option<Vec<DateFilter>>,
    /// <p>The principal that created a note.</p>
    #[serde(rename = "NoteUpdatedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_updated_by: Option<Vec<StringFilter>>,
    /// <p>The date/time that the process was launched.</p>
    #[serde(rename = "ProcessLaunchedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_launched_at: Option<Vec<DateFilter>>,
    /// <p>The name of the process.</p>
    #[serde(rename = "ProcessName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_name: Option<Vec<StringFilter>>,
    /// <p>The parent process ID.</p>
    #[serde(rename = "ProcessParentPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_parent_pid: Option<Vec<NumberFilter>>,
    /// <p>The path to the process executable.</p>
    #[serde(rename = "ProcessPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_path: Option<Vec<StringFilter>>,
    /// <p>The process ID.</p>
    #[serde(rename = "ProcessPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_pid: Option<Vec<NumberFilter>>,
    /// <p>The date/time that the process was terminated.</p>
    #[serde(rename = "ProcessTerminatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_terminated_at: Option<Vec<DateFilter>>,
    /// <p>The ARN generated by Security Hub that uniquely identifies a third-party company (security findings provider) after this provider's product (solution that generates findings) is registered with Security Hub.</p>
    #[serde(rename = "ProductArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_arn: Option<Vec<StringFilter>>,
    /// <p>A data type where security-findings providers can include additional solution-specific details that aren't part of the defined <code>AwsSecurityFinding</code> format.</p>
    #[serde(rename = "ProductFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_fields: Option<Vec<MapFilter>>,
    /// <p>The name of the solution (product) that generates findings.</p>
    #[serde(rename = "ProductName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_name: Option<Vec<StringFilter>>,
    /// <p>The recommendation of what to do about the issue described in a finding.</p>
    #[serde(rename = "RecommendationText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_text: Option<Vec<StringFilter>>,
    /// <p>The updated record state for the finding.</p>
    #[serde(rename = "RecordState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_state: Option<Vec<StringFilter>>,
    /// <p>The solution-generated identifier for a related finding.</p>
    #[serde(rename = "RelatedFindingsId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_findings_id: Option<Vec<StringFilter>>,
    /// <p>The ARN of the solution that generated a related finding.</p>
    #[serde(rename = "RelatedFindingsProductArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_findings_product_arn: Option<Vec<StringFilter>>,
    /// <p>The IAM profile ARN of the instance.</p>
    #[serde(rename = "ResourceAwsEc2InstanceIamInstanceProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_aws_ec_2_instance_iam_instance_profile_arn: Option<Vec<StringFilter>>,
    /// <p>The Amazon Machine Image (AMI) ID of the instance.</p>
    #[serde(rename = "ResourceAwsEc2InstanceImageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_aws_ec_2_instance_image_id: Option<Vec<StringFilter>>,
    /// <p>The IPv4 addresses associated with the instance.</p>
    #[serde(rename = "ResourceAwsEc2InstanceIpV4Addresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_aws_ec_2_instance_ip_v4_addresses: Option<Vec<IpFilter>>,
    /// <p>The IPv6 addresses associated with the instance.</p>
    #[serde(rename = "ResourceAwsEc2InstanceIpV6Addresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_aws_ec_2_instance_ip_v6_addresses: Option<Vec<IpFilter>>,
    /// <p>The key name associated with the instance.</p>
    #[serde(rename = "ResourceAwsEc2InstanceKeyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_aws_ec_2_instance_key_name: Option<Vec<StringFilter>>,
    /// <p>The date and time the instance was launched.</p>
    #[serde(rename = "ResourceAwsEc2InstanceLaunchedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_aws_ec_2_instance_launched_at: Option<Vec<DateFilter>>,
    /// <p>The identifier of the subnet that the instance was launched in.</p>
    #[serde(rename = "ResourceAwsEc2InstanceSubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_aws_ec_2_instance_subnet_id: Option<Vec<StringFilter>>,
    /// <p>The instance type of the instance.</p>
    #[serde(rename = "ResourceAwsEc2InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_aws_ec_2_instance_type: Option<Vec<StringFilter>>,
    /// <p>The identifier of the VPC that the instance was launched in.</p>
    #[serde(rename = "ResourceAwsEc2InstanceVpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_aws_ec_2_instance_vpc_id: Option<Vec<StringFilter>>,
    /// <p>The creation date/time of the IAM access key related to a finding.</p>
    #[serde(rename = "ResourceAwsIamAccessKeyCreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_aws_iam_access_key_created_at: Option<Vec<DateFilter>>,
    /// <p>The status of the IAM access key related to a finding.</p>
    #[serde(rename = "ResourceAwsIamAccessKeyStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_aws_iam_access_key_status: Option<Vec<StringFilter>>,
    /// <p>The user associated with the IAM access key related to a finding.</p>
    #[serde(rename = "ResourceAwsIamAccessKeyUserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_aws_iam_access_key_user_name: Option<Vec<StringFilter>>,
    /// <p>The canonical user ID of the owner of the S3 bucket.</p>
    #[serde(rename = "ResourceAwsS3BucketOwnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_aws_s3_bucket_owner_id: Option<Vec<StringFilter>>,
    /// <p>The display name of the owner of the S3 bucket.</p>
    #[serde(rename = "ResourceAwsS3BucketOwnerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_aws_s3_bucket_owner_name: Option<Vec<StringFilter>>,
    /// <p>The identifier of the image related to a finding.</p>
    #[serde(rename = "ResourceContainerImageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_container_image_id: Option<Vec<StringFilter>>,
    /// <p>The name of the image related to a finding.</p>
    #[serde(rename = "ResourceContainerImageName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_container_image_name: Option<Vec<StringFilter>>,
    /// <p>The date/time that the container was started.</p>
    #[serde(rename = "ResourceContainerLaunchedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_container_launched_at: Option<Vec<DateFilter>>,
    /// <p>The name of the container related to a finding.</p>
    #[serde(rename = "ResourceContainerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_container_name: Option<Vec<StringFilter>>,
    /// <p>The details of a resource that doesn't have a specific subfield for the resource type defined.</p>
    #[serde(rename = "ResourceDetailsOther")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_details_other: Option<Vec<MapFilter>>,
    /// <p>The canonical identifier for the given resource type.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<Vec<StringFilter>>,
    /// <p>The canonical AWS partition name that the Region is assigned to.</p>
    #[serde(rename = "ResourcePartition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_partition: Option<Vec<StringFilter>>,
    /// <p>The canonical AWS external Region name where this resource is located.</p>
    #[serde(rename = "ResourceRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_region: Option<Vec<StringFilter>>,
    /// <p>A list of AWS tags associated with a resource at the time the finding was processed.</p>
    #[serde(rename = "ResourceTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<Vec<MapFilter>>,
    /// <p>Specifies the type of the resource that details are provided for.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<Vec<StringFilter>>,
    /// <p>The label of a finding's severity.</p>
    #[serde(rename = "SeverityLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_label: Option<Vec<StringFilter>>,
    /// <p>The normalized severity of a finding.</p>
    #[serde(rename = "SeverityNormalized")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_normalized: Option<Vec<NumberFilter>>,
    /// <p>The native severity as defined by the security-findings provider's solution that generated the finding.</p>
    #[serde(rename = "SeverityProduct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_product: Option<Vec<NumberFilter>>,
    /// <p>A URL that links to a page about the current finding in the security-findings provider's solution.</p>
    #[serde(rename = "SourceUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<StringFilter>>,
    /// <p>The category of a threat intelligence indicator.</p>
    #[serde(rename = "ThreatIntelIndicatorCategory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_intel_indicator_category: Option<Vec<StringFilter>>,
    /// <p>The date/time of the last observation of a threat intelligence indicator.</p>
    #[serde(rename = "ThreatIntelIndicatorLastObservedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_intel_indicator_last_observed_at: Option<Vec<DateFilter>>,
    /// <p>The source of the threat intelligence.</p>
    #[serde(rename = "ThreatIntelIndicatorSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_intel_indicator_source: Option<Vec<StringFilter>>,
    /// <p>The URL for more details from the source of the threat intelligence.</p>
    #[serde(rename = "ThreatIntelIndicatorSourceUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_intel_indicator_source_url: Option<Vec<StringFilter>>,
    /// <p>The type of a threat intelligence indicator.</p>
    #[serde(rename = "ThreatIntelIndicatorType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_intel_indicator_type: Option<Vec<StringFilter>>,
    /// <p>The value of a threat intelligence indicator.</p>
    #[serde(rename = "ThreatIntelIndicatorValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_intel_indicator_value: Option<Vec<StringFilter>>,
    /// <p>A finding's title.</p>
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Vec<StringFilter>>,
    /// <p>A finding type in the format of <code>namespace/category/classifier</code> that classifies a finding.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<Vec<StringFilter>>,
    /// <p>An ISO8601-formatted timestamp that indicates when the security-findings provider last updated the finding record. </p>
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Vec<DateFilter>>,
    /// <p>A list of name/value string pairs associated with the finding. These are custom, user-defined fields added to a finding. </p>
    #[serde(rename = "UserDefinedFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_defined_fields: Option<Vec<MapFilter>>,
    /// <p>The veracity of a finding.</p>
    #[serde(rename = "VerificationState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_state: Option<Vec<StringFilter>>,
    /// <p>The workflow state of a finding.</p>
    #[serde(rename = "WorkflowState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_state: Option<Vec<StringFilter>>,
    /// <p><p>The status of the investigation into a finding. Allowed values are the following.</p> <ul> <li> <p> <code>NEW</code> - The initial state of a finding, before it is reviewed.</p> </li> <li> <p> <code>NOTIFIED</code> - Indicates that the resource owner has been notified about the security issue. Used when the initial reviewer is not the resource owner, and needs intervention from the resource owner.</p> </li> <li> <p> <code>SUPPRESSED</code> - The finding will not be reviewed again and will not be acted upon.</p> </li> <li> <p> <code>RESOLVED</code> - The finding was reviewed and remediated and is now considered resolved. </p> </li> </ul></p>
    #[serde(rename = "WorkflowStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_status: Option<Vec<StringFilter>>,
}

/// <p>Identifies a finding to update using <code>BatchUpdateFindings</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsSecurityFindingIdentifier {
    /// <p>The identifier of the finding that was specified by the finding provider.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The ARN generated by Security Hub that uniquely identifies a product that generates findings. This can be the ARN for a third-party product that is integrated with Security Hub, or the ARN for a custom integration.</p>
    #[serde(rename = "ProductArn")]
    pub product_arn: String,
}

/// <p>A wrapper type for the topic's Amazon Resource Name (ARN).</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsSnsTopicDetails {
    /// <p>The ID of an AWS managed customer master key (CMK) for Amazon SNS or a custom CMK.</p>
    #[serde(rename = "KmsMasterKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_master_key_id: Option<String>,
    /// <p>The subscription's owner.</p>
    #[serde(rename = "Owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// <p>Subscription is an embedded property that describes the subscription endpoints of an Amazon SNS topic.</p>
    #[serde(rename = "Subscription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<Vec<AwsSnsTopicSubscription>>,
    /// <p>The name of the topic.</p>
    #[serde(rename = "TopicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
}

/// <p>A wrapper type for the attributes of an Amazon SNS subscription.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsSnsTopicSubscription {
    /// <p>The subscription's endpoint (format depends on the protocol).</p>
    #[serde(rename = "Endpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    /// <p>The subscription's protocol.</p>
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

/// <p>Data about a queue.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsSqsQueueDetails {
    /// <p>The Amazon Resource Name (ARN) of the dead-letter queue to which Amazon SQS moves messages after the value of <code>maxReceiveCount</code> is exceeded. </p>
    #[serde(rename = "DeadLetterTargetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_letter_target_arn: Option<String>,
    /// <p>The length of time, in seconds, for which Amazon SQS can reuse a data key to encrypt or decrypt messages before calling AWS KMS again.</p>
    #[serde(rename = "KmsDataKeyReusePeriodSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_data_key_reuse_period_seconds: Option<i64>,
    /// <p>The ID of an AWS managed customer master key (CMK) for Amazon SQS or a custom CMK.</p>
    #[serde(rename = "KmsMasterKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_master_key_id: Option<String>,
    /// <p>The name of the new queue.</p>
    #[serde(rename = "QueueName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_name: Option<String>,
}

/// <p>Details about a WAF WebACL.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsWafWebAclDetails {
    /// <p>The action to perform if none of the rules contained in the WebACL match.</p>
    #[serde(rename = "DefaultAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_action: Option<String>,
    /// <p>A friendly name or description of the WebACL. You can't change the name of a WebACL after you create it.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>An array that contains the action for each rule in a WebACL, the priority of the rule, and the ID of the rule.</p>
    #[serde(rename = "Rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<AwsWafWebAclRule>>,
    /// <p>A unique identifier for a WebACL.</p>
    #[serde(rename = "WebAclId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_acl_id: Option<String>,
}

/// <p>Details for a rule in a WAF WebACL.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsWafWebAclRule {
    /// <p>Specifies the action that CloudFront or AWS WAF takes when a web request matches the conditions in the rule. </p>
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<WafAction>,
    /// <p>Rules to exclude from a rule group.</p>
    #[serde(rename = "ExcludedRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_rules: Option<Vec<WafExcludedRule>>,
    /// <p>Use the <code>OverrideAction</code> to test your RuleGroup.</p> <p>Any rule in a RuleGroup can potentially block a request. If you set the <code>OverrideAction</code> to <code>None</code>, the RuleGroup blocks a request if any individual rule in the RuleGroup matches the request and is configured to block that request.</p> <p>However, if you first want to test the RuleGroup, set the <code>OverrideAction</code> to <code>Count</code>. The RuleGroup then overrides any block action specified by individual rules contained within the group. Instead of blocking matching requests, those requests are counted.</p> <p> <code>ActivatedRule</code>|<code>OverrideAction</code> applies only when updating or adding a RuleGroup to a WebACL. In this case you do not use <code>ActivatedRule</code>|<code>Action</code>. For all other update requests, <code>ActivatedRule</code>|<code>Action</code> is used instead of <code>ActivatedRule</code>|<code>OverrideAction</code>. </p>
    #[serde(rename = "OverrideAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_action: Option<WafOverrideAction>,
    /// <p>Specifies the order in which the rules in a WebACL are evaluated. Rules with a lower value for <code>Priority</code> are evaluated before rules with a higher value. The value must be a unique integer. If you add multiple rules to a WebACL, the values do not need to be consecutive.</p>
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// <p>The identifier for a rule.</p>
    #[serde(rename = "RuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    /// <p>The rule type.</p> <p>Valid values: <code>REGULAR</code> | <code>RATE_BASED</code> | <code>GROUP</code> </p> <p>The default is <code>REGULAR</code>.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchDisableStandardsRequest {
    /// <p>The ARNs of the standards subscriptions to disable.</p>
    #[serde(rename = "StandardsSubscriptionArns")]
    pub standards_subscription_arns: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchDisableStandardsResponse {
    /// <p>The details of the standards subscriptions that were disabled.</p>
    #[serde(rename = "StandardsSubscriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_subscriptions: Option<Vec<StandardsSubscription>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchEnableStandardsRequest {
    /// <p>The list of standards checks to enable.</p>
    #[serde(rename = "StandardsSubscriptionRequests")]
    pub standards_subscription_requests: Vec<StandardsSubscriptionRequest>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchEnableStandardsResponse {
    /// <p>The details of the standards subscriptions that were enabled.</p>
    #[serde(rename = "StandardsSubscriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_subscriptions: Option<Vec<StandardsSubscription>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchImportFindingsRequest {
    /// <p>A list of findings to import. To successfully import a finding, it must follow the <a href="https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-findings-format.html">AWS Security Finding Format</a>. Maximum of 100 findings per request.</p>
    #[serde(rename = "Findings")]
    pub findings: Vec<AwsSecurityFinding>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchImportFindingsResponse {
    /// <p>The number of findings that failed to import.</p>
    #[serde(rename = "FailedCount")]
    pub failed_count: i64,
    /// <p>The list of findings that failed to import.</p>
    #[serde(rename = "FailedFindings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_findings: Option<Vec<ImportFindingsError>>,
    /// <p>The number of findings that were successfully imported.</p>
    #[serde(rename = "SuccessCount")]
    pub success_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchUpdateFindingsRequest {
    /// <p>The updated value for the finding confidence. Confidence is defined as the likelihood that a finding accurately identifies the behavior or issue that it was intended to identify.</p> <p>Confidence is scored on a 0-100 basis using a ratio scale, where 0 means zero percent confidence and 100 means 100 percent confidence.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<i64>,
    /// <p>The updated value for the level of importance assigned to the resources associated with the findings.</p> <p>A score of 0 means that the underlying resources have no criticality, and a score of 100 is reserved for the most critical resources. </p>
    #[serde(rename = "Criticality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criticality: Option<i64>,
    /// <p>The list of findings to update. <code>BatchUpdateFindings</code> can be used to update up to 100 findings at a time.</p> <p>For each finding, the list provides the finding identifier and the ARN of the finding provider.</p>
    #[serde(rename = "FindingIdentifiers")]
    pub finding_identifiers: Vec<AwsSecurityFindingIdentifier>,
    #[serde(rename = "Note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<NoteUpdate>,
    /// <p>A list of findings that are related to the updated findings.</p>
    #[serde(rename = "RelatedFindings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_findings: Option<Vec<RelatedFinding>>,
    /// <p>Used to update the finding severity.</p>
    #[serde(rename = "Severity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<SeverityUpdate>,
    /// <p><p>One or more finding types in the format of namespace/category/classifier that classify a finding.</p> <p>Valid namespace values are as follows.</p> <ul> <li> <p>Software and Configuration Checks</p> </li> <li> <p>TTPs</p> </li> <li> <p>Effects</p> </li> <li> <p>Unusual Behaviors</p> </li> <li> <p>Sensitive Data Identifications </p> </li> </ul></p>
    #[serde(rename = "Types")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
    /// <p>A list of name/value string pairs associated with the finding. These are custom, user-defined fields added to a finding.</p>
    #[serde(rename = "UserDefinedFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_defined_fields: Option<::std::collections::HashMap<String, String>>,
    /// <p><p>Indicates the veracity of a finding.</p> <p>The available values for <code>VerificationState</code> are as follows.</p> <ul> <li> <p> <code>UNKNOWN</code> – The default disposition of a security finding</p> </li> <li> <p> <code>TRUE<em>POSITIVE</code> – The security finding is confirmed</p> </li> <li> <p> <code>FALSE</em>POSITIVE</code> – The security finding was determined to be a false alarm</p> </li> <li> <p> <code>BENIGN<em>POSITIVE</code> – A special case of <code>TRUE</em>POSITIVE</code> where the finding doesn&#39;t pose any threat, is expected, or both</p> </li> </ul></p>
    #[serde(rename = "VerificationState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_state: Option<String>,
    /// <p>Used to update the workflow status of a finding.</p> <p>The workflow status indicates the progress of the investigation into the finding. </p>
    #[serde(rename = "Workflow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow: Option<WorkflowUpdate>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchUpdateFindingsResponse {
    /// <p>The list of findings that were updated successfully.</p>
    #[serde(rename = "ProcessedFindings")]
    pub processed_findings: Vec<AwsSecurityFindingIdentifier>,
    /// <p>The list of findings that were not updated.</p>
    #[serde(rename = "UnprocessedFindings")]
    pub unprocessed_findings: Vec<BatchUpdateFindingsUnprocessedFinding>,
}

/// <p>A finding from a <code>BatchUpdateFindings</code> request that Security Hub was unable to update.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchUpdateFindingsUnprocessedFinding {
    /// <p>The code associated with the error.</p>
    #[serde(rename = "ErrorCode")]
    pub error_code: String,
    /// <p>The message associated with the error.</p>
    #[serde(rename = "ErrorMessage")]
    pub error_message: String,
    /// <p>The identifier of the finding that was not updated.</p>
    #[serde(rename = "FindingIdentifier")]
    pub finding_identifier: AwsSecurityFindingIdentifier,
}

/// <p>Contains finding details that are specific to control-based findings. Only returned for findings generated from controls.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Compliance {
    /// <p>For a control, the industry or regulatory framework requirements that are related to the control. The check for that control is aligned with these requirements.</p>
    #[serde(rename = "RelatedRequirements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_requirements: Option<Vec<String>>,
    /// <p><p>The result of a standards check.</p> <p>The valid values for <code>Status</code> are as follows.</p> <ul> <li> <ul> <li> <p> <code>PASSED</code> - Standards check passed for all evaluated resources.</p> </li> <li> <p> <code>WARNING</code> - Some information is missing or this check is not supported for your configuration.</p> </li> <li> <p> <code>FAILED</code> - Standards check failed for at least one evaluated resource.</p> </li> <li> <p> <code>NOT<em>AVAILABLE</code> - Check could not be performed due to a service outage, API error, or because the result of the AWS Config evaluation was <code>NOT</em>APPLICABLE</code>. If the AWS Config evaluation result was <code>NOT_APPLICABLE</code>, then after 3 days, Security Hub automatically archives the finding.</p> </li> </ul> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>For findings generated from controls, a list of reasons behind the value of <code>Status</code>. For the list of status reason codes and their meanings, see <a href="https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-standards-results.html#securityhub-standards-results-asff">Standards-related information in the ASFF</a> in the <i>AWS Security Hub User Guide</i>. </p>
    #[serde(rename = "StatusReasons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reasons: Option<Vec<StatusReason>>,
}

/// <p>Container details related to a finding.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContainerDetails {
    /// <p>The identifier of the image related to a finding.</p>
    #[serde(rename = "ImageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    /// <p>The name of the image related to a finding.</p>
    #[serde(rename = "ImageName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    /// <p>The date and time when the container started.</p>
    #[serde(rename = "LaunchedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launched_at: Option<String>,
    /// <p>The name of the container related to a finding.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateActionTargetRequest {
    /// <p>The description for the custom action target.</p>
    #[serde(rename = "Description")]
    pub description: String,
    /// <p>The ID for the custom action target.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The name of the custom action target.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateActionTargetResponse {
    /// <p>The ARN for the custom action target.</p>
    #[serde(rename = "ActionTargetArn")]
    pub action_target_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateInsightRequest {
    /// <p>One or more attributes used to filter the findings included in the insight. The insight only includes findings that match the criteria defined in the filters.</p>
    #[serde(rename = "Filters")]
    pub filters: AwsSecurityFindingFilters,
    /// <p>The attribute used to group the findings for the insight. The grouping attribute identifies the type of item that the insight applies to. For example, if an insight is grouped by resource identifier, then the insight produces a list of resource identifiers.</p>
    #[serde(rename = "GroupByAttribute")]
    pub group_by_attribute: String,
    /// <p>The name of the custom insight to create.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateInsightResponse {
    /// <p>The ARN of the insight created.</p>
    #[serde(rename = "InsightArn")]
    pub insight_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateMembersRequest {
    /// <p>The list of accounts to associate with the Security Hub master account. For each account, the list includes the account ID and the email address.</p>
    #[serde(rename = "AccountDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_details: Option<Vec<AccountDetails>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateMembersResponse {
    /// <p>The list of AWS accounts that were not processed. For each account, the list includes the account ID and the email address.</p>
    #[serde(rename = "UnprocessedAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<SecurityHubResult>>,
}

/// <p>A date filter for querying findings.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DateFilter {
    /// <p>A date range for the date filter.</p>
    #[serde(rename = "DateRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_range: Option<DateRange>,
    /// <p>An end date for the date filter.</p>
    #[serde(rename = "End")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// <p>A start date for the date filter.</p>
    #[serde(rename = "Start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
}

/// <p>A date range for the date filter.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DateRange {
    /// <p>A date range unit for the date filter.</p>
    #[serde(rename = "Unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// <p>A date range value for the date filter.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeclineInvitationsRequest {
    /// <p>The list of account IDs for the accounts from which to decline the invitations to Security Hub.</p>
    #[serde(rename = "AccountIds")]
    pub account_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeclineInvitationsResponse {
    /// <p>The list of AWS accounts that were not processed. For each account, the list includes the account ID and the email address.</p>
    #[serde(rename = "UnprocessedAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<SecurityHubResult>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteActionTargetRequest {
    /// <p>The ARN of the custom action target to delete.</p>
    #[serde(rename = "ActionTargetArn")]
    pub action_target_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteActionTargetResponse {
    /// <p>The ARN of the custom action target that was deleted.</p>
    #[serde(rename = "ActionTargetArn")]
    pub action_target_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteInsightRequest {
    /// <p>The ARN of the insight to delete.</p>
    #[serde(rename = "InsightArn")]
    pub insight_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteInsightResponse {
    /// <p>The ARN of the insight that was deleted.</p>
    #[serde(rename = "InsightArn")]
    pub insight_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteInvitationsRequest {
    /// <p>The list of the account IDs that sent the invitations to delete.</p>
    #[serde(rename = "AccountIds")]
    pub account_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteInvitationsResponse {
    /// <p>The list of AWS accounts for which the invitations were not deleted. For each account, the list includes the account ID and the email address.</p>
    #[serde(rename = "UnprocessedAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<SecurityHubResult>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteMembersRequest {
    /// <p>The list of account IDs for the member accounts to delete.</p>
    #[serde(rename = "AccountIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteMembersResponse {
    /// <p>The list of AWS accounts that were not deleted. For each account, the list includes the account ID and the email address.</p>
    #[serde(rename = "UnprocessedAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<SecurityHubResult>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeActionTargetsRequest {
    /// <p>A list of custom action target ARNs for the custom action targets to retrieve.</p>
    #[serde(rename = "ActionTargetArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_target_arns: Option<Vec<String>>,
    /// <p>The maximum number of results to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that is required for pagination. On your first call to the <code>DescribeActionTargets</code> operation, set the value of this parameter to <code>NULL</code>.</p> <p>For subsequent calls to the operation, to continue listing data, set the value of this parameter to the value returned from the previous response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeActionTargetsResponse {
    /// <p>A list of <code>ActionTarget</code> objects. Each object includes the <code>ActionTargetArn</code>, <code>Description</code>, and <code>Name</code> of a custom action target available in Security Hub.</p>
    #[serde(rename = "ActionTargets")]
    pub action_targets: Vec<ActionTarget>,
    /// <p>The pagination token to use to request the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeHubRequest {
    /// <p>The ARN of the Hub resource to retrieve.</p>
    #[serde(rename = "HubArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hub_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeHubResponse {
    /// <p>The ARN of the Hub resource that was retrieved.</p>
    #[serde(rename = "HubArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hub_arn: Option<String>,
    /// <p>The date and time when Security Hub was enabled in the account.</p>
    #[serde(rename = "SubscribedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribed_at: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeProductsRequest {
    /// <p>The maximum number of results to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that is required for pagination. On your first call to the <code>DescribeProducts</code> operation, set the value of this parameter to <code>NULL</code>.</p> <p>For subsequent calls to the operation, to continue listing data, set the value of this parameter to the value returned from the previous response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeProductsResponse {
    /// <p>The pagination token to use to request the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of products, including details for each product.</p>
    #[serde(rename = "Products")]
    pub products: Vec<Product>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeStandardsControlsRequest {
    /// <p>The maximum number of security standard controls to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that is required for pagination. On your first call to the <code>DescribeStandardsControls</code> operation, set the value of this parameter to <code>NULL</code>.</p> <p>For subsequent calls to the operation, to continue listing data, set the value of this parameter to the value returned from the previous response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ARN of a resource that represents your subscription to a supported standard.</p>
    #[serde(rename = "StandardsSubscriptionArn")]
    pub standards_subscription_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeStandardsControlsResponse {
    /// <p>A list of security standards controls.</p>
    #[serde(rename = "Controls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controls: Option<Vec<StandardsControl>>,
    /// <p>The pagination token to use to request the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeStandardsRequest {
    /// <p>The maximum number of standards to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that is required for pagination. On your first call to the <code>DescribeStandards</code> operation, set the value of this parameter to <code>NULL</code>.</p> <p>For subsequent calls to the operation, to continue listing data, set the value of this parameter to the value returned from the previous response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeStandardsResponse {
    /// <p>The pagination token to use to request the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of available standards.</p>
    #[serde(rename = "Standards")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards: Option<Vec<Standard>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisableImportFindingsForProductRequest {
    /// <p>The ARN of the integrated product to disable the integration for.</p>
    #[serde(rename = "ProductSubscriptionArn")]
    pub product_subscription_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisableImportFindingsForProductResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisableSecurityHubRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisableSecurityHubResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateFromMasterAccountRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateFromMasterAccountResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateMembersRequest {
    /// <p>The account IDs of the member accounts to disassociate from the master account.</p>
    #[serde(rename = "AccountIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateMembersResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EnableImportFindingsForProductRequest {
    /// <p>The ARN of the product to enable the integration for.</p>
    #[serde(rename = "ProductArn")]
    pub product_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EnableImportFindingsForProductResponse {
    /// <p>The ARN of your subscription to the product to enable integrations for.</p>
    #[serde(rename = "ProductSubscriptionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_subscription_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EnableSecurityHubRequest {
    /// <p>Whether to enable the security standards that Security Hub has designated as automatically enabled. If you do not provide a value for <code>EnableDefaultStandards</code>, it is set to <code>true</code>. To not enable the automatically enabled standards, set <code>EnableDefaultStandards</code> to <code>false</code>.</p>
    #[serde(rename = "EnableDefaultStandards")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_default_standards: Option<bool>,
    /// <p>The tags to add to the hub resource when you enable Security Hub.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EnableSecurityHubResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetEnabledStandardsRequest {
    /// <p>The maximum number of results to return in the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that is required for pagination. On your first call to the <code>GetEnabledStandards</code> operation, set the value of this parameter to <code>NULL</code>.</p> <p>For subsequent calls to the operation, to continue listing data, set the value of this parameter to the value returned from the previous response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of the standards subscription ARNs for the standards to retrieve.</p>
    #[serde(rename = "StandardsSubscriptionArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_subscription_arns: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetEnabledStandardsResponse {
    /// <p>The pagination token to use to request the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of <code>StandardsSubscriptions</code> objects that include information about the enabled standards.</p>
    #[serde(rename = "StandardsSubscriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_subscriptions: Option<Vec<StandardsSubscription>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetFindingsRequest {
    /// <p>The finding attributes used to define a condition to filter the returned findings.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<AwsSecurityFindingFilters>,
    /// <p>The maximum number of findings to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that is required for pagination. On your first call to the <code>GetFindings</code> operation, set the value of this parameter to <code>NULL</code>.</p> <p>For subsequent calls to the operation, to continue listing data, set the value of this parameter to the value returned from the previous response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The finding attributes used to sort the list of returned findings.</p>
    #[serde(rename = "SortCriteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_criteria: Option<Vec<SortCriterion>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetFindingsResponse {
    /// <p>The findings that matched the filters specified in the request.</p>
    #[serde(rename = "Findings")]
    pub findings: Vec<AwsSecurityFinding>,
    /// <p>The pagination token to use to request the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetInsightResultsRequest {
    /// <p>The ARN of the insight for which to return results.</p>
    #[serde(rename = "InsightArn")]
    pub insight_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetInsightResultsResponse {
    /// <p>The insight results returned by the operation.</p>
    #[serde(rename = "InsightResults")]
    pub insight_results: InsightResults,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetInsightsRequest {
    /// <p>The ARNs of the insights to describe. If you do not provide any insight ARNs, then <code>GetInsights</code> returns all of your custom insights. It does not return any managed insights.</p>
    #[serde(rename = "InsightArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_arns: Option<Vec<String>>,
    /// <p>The maximum number of items to return in the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that is required for pagination. On your first call to the <code>GetInsights</code> operation, set the value of this parameter to <code>NULL</code>.</p> <p>For subsequent calls to the operation, to continue listing data, set the value of this parameter to the value returned from the previous response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetInsightsResponse {
    /// <p>The insights returned by the operation.</p>
    #[serde(rename = "Insights")]
    pub insights: Vec<Insight>,
    /// <p>The pagination token to use to request the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetInvitationsCountRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetInvitationsCountResponse {
    /// <p>The number of all membership invitations sent to this Security Hub member account, not including the currently accepted invitation.</p>
    #[serde(rename = "InvitationsCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitations_count: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetMasterAccountRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetMasterAccountResponse {
    /// <p>A list of details about the Security Hub master account for the current member account. </p>
    #[serde(rename = "Master")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master: Option<Invitation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetMembersRequest {
    /// <p>The list of account IDs for the Security Hub member accounts to return the details for. </p>
    #[serde(rename = "AccountIds")]
    pub account_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetMembersResponse {
    /// <p>The list of details about the Security Hub member accounts.</p>
    #[serde(rename = "Members")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<Member>>,
    /// <p>The list of AWS accounts that could not be processed. For each account, the list includes the account ID and the email address.</p>
    #[serde(rename = "UnprocessedAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<SecurityHubResult>>,
}

/// <p>The list of the findings that cannot be imported. For each finding, the list provides the error.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImportFindingsError {
    /// <p>The code of the error returned by the <code>BatchImportFindings</code> operation.</p>
    #[serde(rename = "ErrorCode")]
    pub error_code: String,
    /// <p>The message of the error returned by the <code>BatchImportFindings</code> operation.</p>
    #[serde(rename = "ErrorMessage")]
    pub error_message: String,
    /// <p>The identifier of the finding that could not be updated.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

/// <p>Contains information about a Security Hub insight.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Insight {
    /// <p>One or more attributes used to filter the findings included in the insight. The insight only includes findings that match the criteria defined in the filters.</p>
    #[serde(rename = "Filters")]
    pub filters: AwsSecurityFindingFilters,
    /// <p>The grouping attribute for the insight's findings. Indicates how to group the matching findings, and identifies the type of item that the insight applies to. For example, if an insight is grouped by resource identifier, then the insight produces a list of resource identifiers.</p>
    #[serde(rename = "GroupByAttribute")]
    pub group_by_attribute: String,
    /// <p>The ARN of a Security Hub insight.</p>
    #[serde(rename = "InsightArn")]
    pub insight_arn: String,
    /// <p>The name of a Security Hub insight.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// <p>The insight result values returned by the <code>GetInsightResults</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InsightResultValue {
    /// <p>The number of findings returned for each <code>GroupByAttributeValue</code>.</p>
    #[serde(rename = "Count")]
    pub count: i64,
    /// <p>The value of the attribute that the findings are grouped by for the insight whose results are returned by the <code>GetInsightResults</code> operation.</p>
    #[serde(rename = "GroupByAttributeValue")]
    pub group_by_attribute_value: String,
}

/// <p>The insight results returned by the <code>GetInsightResults</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InsightResults {
    /// <p>The attribute that the findings are grouped by for the insight whose results are returned by the <code>GetInsightResults</code> operation.</p>
    #[serde(rename = "GroupByAttribute")]
    pub group_by_attribute: String,
    /// <p>The ARN of the insight whose results are returned by the <code>GetInsightResults</code> operation.</p>
    #[serde(rename = "InsightArn")]
    pub insight_arn: String,
    /// <p>The list of insight result values returned by the <code>GetInsightResults</code> operation.</p>
    #[serde(rename = "ResultValues")]
    pub result_values: Vec<InsightResultValue>,
}

/// <p>Details about an invitation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Invitation {
    /// <p>The account ID of the Security Hub master account that the invitation was sent from.</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The ID of the invitation sent to the member account.</p>
    #[serde(rename = "InvitationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitation_id: Option<String>,
    /// <p>The timestamp of when the invitation was sent.</p>
    #[serde(rename = "InvitedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_at: Option<f64>,
    /// <p>The current status of the association between the member and master accounts.</p>
    #[serde(rename = "MemberStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InviteMembersRequest {
    /// <p>The list of account IDs of the AWS accounts to invite to Security Hub as members. </p>
    #[serde(rename = "AccountIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InviteMembersResponse {
    /// <p>The list of AWS accounts that could not be processed. For each account, the list includes the account ID and the email address.</p>
    #[serde(rename = "UnprocessedAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<SecurityHubResult>>,
}

/// <p>The IP filter for querying findings.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpFilter {
    /// <p>A finding's CIDR value.</p>
    #[serde(rename = "Cidr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,
}

/// <p>A keyword filter for querying findings.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeywordFilter {
    /// <p>A value for the keyword.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListEnabledProductsForImportRequest {
    /// <p>The maximum number of items to return in the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that is required for pagination. On your first call to the <code>ListEnabledProductsForImport</code> operation, set the value of this parameter to <code>NULL</code>.</p> <p>For subsequent calls to the operation, to continue listing data, set the value of this parameter to the value returned from the previous response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListEnabledProductsForImportResponse {
    /// <p>The pagination token to use to request the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of ARNs for the resources that represent your subscriptions to products. </p>
    #[serde(rename = "ProductSubscriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_subscriptions: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListInvitationsRequest {
    /// <p>The maximum number of items to return in the response. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that is required for pagination. On your first call to the <code>ListInvitations</code> operation, set the value of this parameter to <code>NULL</code>.</p> <p>For subsequent calls to the operation, to continue listing data, set the value of this parameter to the value returned from the previous response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListInvitationsResponse {
    /// <p>The details of the invitations returned by the operation.</p>
    #[serde(rename = "Invitations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitations: Option<Vec<Invitation>>,
    /// <p>The pagination token to use to request the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListMembersRequest {
    /// <p>The maximum number of items to return in the response. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that is required for pagination. On your first call to the <code>ListMembers</code> operation, set the value of this parameter to <code>NULL</code>.</p> <p>For subsequent calls to the operation, to continue listing data, set the value of this parameter to the value returned from the previous response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Specifies which member accounts to include in the response based on their relationship status with the master account. The default value is <code>TRUE</code>.</p> <p>If <code>OnlyAssociated</code> is set to <code>TRUE</code>, the response includes member accounts whose relationship status with the master is set to <code>ENABLED</code> or <code>DISABLED</code>.</p> <p>If <code>OnlyAssociated</code> is set to <code>FALSE</code>, the response includes all existing member accounts. </p>
    #[serde(rename = "OnlyAssociated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_associated: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListMembersResponse {
    /// <p>Member details returned by the operation.</p>
    #[serde(rename = "Members")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<Member>>,
    /// <p>The pagination token to use to request the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The ARN of the resource to retrieve tags for.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>The tags associated with a resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Information about the state of the load balancer.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadBalancerState {
    /// <p>The state code. The initial state of the load balancer is provisioning.</p> <p>After the load balancer is fully set up and ready to route traffic, its state is active.</p> <p>If the load balancer could not be set up, its state is failed. </p>
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// <p>A description of the state.</p>
    #[serde(rename = "Reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// <p>A list of malware related to a finding.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Malware {
    /// <p>The name of the malware that was observed.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The file system path of the malware that was observed.</p>
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// <p>The state of the malware that was observed.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The type of the malware that was observed.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>The map filter for querying findings.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapFilter {
    /// <p>The condition to apply to a key value when querying for findings with a map filter.</p>
    #[serde(rename = "Comparison")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison: Option<String>,
    /// <p>The key of the map filter.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The value for the key in the map filter.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>The details about a member account.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Member {
    /// <p>The AWS account ID of the member account.</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The email address of the member account.</p>
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// <p>A timestamp for the date and time when the invitation was sent to the member account.</p>
    #[serde(rename = "InvitedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_at: Option<f64>,
    /// <p>The AWS account ID of the Security Hub master account associated with this member account.</p>
    #[serde(rename = "MasterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_id: Option<String>,
    /// <p>The status of the relationship between the member account and its master account. </p>
    #[serde(rename = "MemberStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_status: Option<String>,
    /// <p>The timestamp for the date and time when the member account was updated.</p>
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

/// <p>The details of network-related information about a finding.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Network {
    /// <p>The destination domain of network-related information about a finding.</p>
    #[serde(rename = "DestinationDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_domain: Option<String>,
    /// <p>The destination IPv4 address of network-related information about a finding.</p>
    #[serde(rename = "DestinationIpV4")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_ip_v4: Option<String>,
    /// <p>The destination IPv6 address of network-related information about a finding.</p>
    #[serde(rename = "DestinationIpV6")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_ip_v6: Option<String>,
    /// <p>The destination port of network-related information about a finding.</p>
    #[serde(rename = "DestinationPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_port: Option<i64>,
    /// <p>The direction of network traffic associated with a finding.</p>
    #[serde(rename = "Direction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    /// <p>The protocol of network-related information about a finding.</p>
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// <p>The source domain of network-related information about a finding.</p>
    #[serde(rename = "SourceDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_domain: Option<String>,
    /// <p>The source IPv4 address of network-related information about a finding.</p>
    #[serde(rename = "SourceIpV4")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip_v4: Option<String>,
    /// <p>The source IPv6 address of network-related information about a finding.</p>
    #[serde(rename = "SourceIpV6")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip_v6: Option<String>,
    /// <p>The source media access control (MAC) address of network-related information about a finding.</p>
    #[serde(rename = "SourceMac")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_mac: Option<String>,
    /// <p>The source port of network-related information about a finding.</p>
    #[serde(rename = "SourcePort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_port: Option<i64>,
}

/// <p>A user-defined note added to a finding.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Note {
    /// <p>The text of a note.</p>
    #[serde(rename = "Text")]
    pub text: String,
    /// <p>The timestamp of when the note was updated.</p>
    #[serde(rename = "UpdatedAt")]
    pub updated_at: String,
    /// <p>The principal that created a note.</p>
    #[serde(rename = "UpdatedBy")]
    pub updated_by: String,
}

/// <p>The updated note.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct NoteUpdate {
    /// <p>The updated note text.</p>
    #[serde(rename = "Text")]
    pub text: String,
    /// <p>The principal that updated the note.</p>
    #[serde(rename = "UpdatedBy")]
    pub updated_by: String,
}

/// <p>A number filter for querying findings.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NumberFilter {
    /// <p>The equal-to condition to be applied to a single field when querying for findings.</p>
    #[serde(rename = "Eq")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eq: Option<f64>,
    /// <p>The greater-than-equal condition to be applied to a single field when querying for findings. </p>
    #[serde(rename = "Gte")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gte: Option<f64>,
    /// <p>The less-than-equal condition to be applied to a single field when querying for findings. </p>
    #[serde(rename = "Lte")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lte: Option<f64>,
}

/// <p>The details of process-related information about a finding.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessDetails {
    /// <p>The date/time that the process was launched.</p>
    #[serde(rename = "LaunchedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launched_at: Option<String>,
    /// <p>The name of the process.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The parent process ID.</p>
    #[serde(rename = "ParentPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_pid: Option<i64>,
    /// <p>The path to the process executable.</p>
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// <p>The process ID.</p>
    #[serde(rename = "Pid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<i64>,
    /// <p>The date and time when the process was terminated.</p>
    #[serde(rename = "TerminatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminated_at: Option<String>,
}

/// <p>Contains details about a product.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Product {
    /// <p>The URL used to activate the product.</p>
    #[serde(rename = "ActivationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_url: Option<String>,
    /// <p>The categories assigned to the product.</p>
    #[serde(rename = "Categories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,
    /// <p>The name of the company that provides the product.</p>
    #[serde(rename = "CompanyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    /// <p>A description of the product.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p><p>The types of integration that the product supports. Available values are the following.</p> <ul> <li> <p> <code>SEND<em>FINDINGS</em>TO<em>SECURITY</em>HUB</code> - Indicates that the integration sends findings to Security Hub.</p> </li> <li> <p> <code>RECEIVE<em>FINDINGS</em>FROM<em>SECURITY</em>HUB</code> - Indicates that the integration receives findings from Security Hub.</p> </li> </ul></p>
    #[serde(rename = "IntegrationTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_types: Option<Vec<String>>,
    /// <p>The URL for the page that contains more information about the product.</p>
    #[serde(rename = "MarketplaceUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketplace_url: Option<String>,
    /// <p>The ARN assigned to the product.</p>
    #[serde(rename = "ProductArn")]
    pub product_arn: String,
    /// <p>The name of the product.</p>
    #[serde(rename = "ProductName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    /// <p>The resource policy associated with the product.</p>
    #[serde(rename = "ProductSubscriptionResourcePolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_subscription_resource_policy: Option<String>,
}

/// <p>A recommendation on how to remediate the issue identified in a finding.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Recommendation {
    /// <p>Describes the recommended steps to take to remediate an issue identified in a finding.</p>
    #[serde(rename = "Text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// <p>A URL to a page or site that contains information about how to remediate a finding.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p>Details about a related finding.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelatedFinding {
    /// <p>The product-generated identifier for a related finding.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The ARN of the product that generated a related finding.</p>
    #[serde(rename = "ProductArn")]
    pub product_arn: String,
}

/// <p>Details about the remediation steps for a finding.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Remediation {
    /// <p>A recommendation on the steps to take to remediate the issue identified by a finding.</p>
    #[serde(rename = "Recommendation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation: Option<Recommendation>,
}

/// <p>A resource related to a finding.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    /// <p>Additional details about the resource related to a finding.</p>
    #[serde(rename = "Details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<ResourceDetails>,
    /// <p>The canonical identifier for the given resource type.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The canonical AWS partition name that the Region is assigned to.</p>
    #[serde(rename = "Partition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition: Option<String>,
    /// <p>The canonical AWS external Region name where this resource is located.</p>
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>A list of AWS tags associated with a resource at the time the finding was processed.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The type of the resource that details are provided for. If possible, set <code>Type</code> to one of the supported resource types. For example, if the resource is an EC2 instance, then set <code>Type</code> to <code>AwsEc2Instance</code>.</p> <p>If the resource does not match any of the provided types, then set <code>Type</code> to <code>Other</code>. </p>
    #[serde(rename = "Type")]
    pub type_: String,
}

/// <p>Additional details about a resource related to a finding.</p> <p>To provide the details, use the object that corresponds to the resource type. For example, if the resource type is <code>AwsEc2Instance</code>, then you use the <code>AwsEc2Instance</code> object to provide the details.</p> <p>If the type-specific object does not contain all of the fields you want to populate, then you use the <code>Other</code> object to populate those additional fields.</p> <p>You also use the <code>Other</code> object to populate the details when the selected type does not have a corresponding object.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceDetails {
    /// <p>Details about a CloudFront distribution.</p>
    #[serde(rename = "AwsCloudFrontDistribution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_cloud_front_distribution: Option<AwsCloudFrontDistributionDetails>,
    /// <p>Details for an AWS CodeBuild project.</p>
    #[serde(rename = "AwsCodeBuildProject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_code_build_project: Option<AwsCodeBuildProjectDetails>,
    /// <p>Details about an Amazon EC2 instance related to a finding.</p>
    #[serde(rename = "AwsEc2Instance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ec_2_instance: Option<AwsEc2InstanceDetails>,
    /// <p>Details for an Amazon EC2 network interface.</p>
    #[serde(rename = "AwsEc2NetworkInterface")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ec_2_network_interface: Option<AwsEc2NetworkInterfaceDetails>,
    /// <p>Details for an EC2 security group.</p>
    #[serde(rename = "AwsEc2SecurityGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ec_2_security_group: Option<AwsEc2SecurityGroupDetails>,
    /// <p>Details for an Elasticsearch domain.</p>
    #[serde(rename = "AwsElasticsearchDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_elasticsearch_domain: Option<AwsElasticsearchDomainDetails>,
    /// <p>Details about a load balancer.</p>
    #[serde(rename = "AwsElbv2LoadBalancer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_elbv_2_load_balancer: Option<AwsElbv2LoadBalancerDetails>,
    /// <p>Details about an IAM access key related to a finding.</p>
    #[serde(rename = "AwsIamAccessKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_iam_access_key: Option<AwsIamAccessKeyDetails>,
    /// <p>Details about an IAM role.</p>
    #[serde(rename = "AwsIamRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_iam_role: Option<AwsIamRoleDetails>,
    /// <p>Details about a KMS key.</p>
    #[serde(rename = "AwsKmsKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_kms_key: Option<AwsKmsKeyDetails>,
    /// <p>Details about a Lambda function.</p>
    #[serde(rename = "AwsLambdaFunction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_lambda_function: Option<AwsLambdaFunctionDetails>,
    /// <p>Details for a Lambda layer version.</p>
    #[serde(rename = "AwsLambdaLayerVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_lambda_layer_version: Option<AwsLambdaLayerVersionDetails>,
    /// <p>Details for an Amazon RDS database instance.</p>
    #[serde(rename = "AwsRdsDbInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_rds_db_instance: Option<AwsRdsDbInstanceDetails>,
    /// <p>Details about an Amazon S3 bucket related to a finding.</p>
    #[serde(rename = "AwsS3Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_s3_bucket: Option<AwsS3BucketDetails>,
    /// <p>Details about an Amazon S3 object related to a finding.</p>
    #[serde(rename = "AwsS3Object")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_s3_object: Option<AwsS3ObjectDetails>,
    /// <p>Details about an SNS topic.</p>
    #[serde(rename = "AwsSnsTopic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_sns_topic: Option<AwsSnsTopicDetails>,
    /// <p>Details about an SQS queue.</p>
    #[serde(rename = "AwsSqsQueue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_sqs_queue: Option<AwsSqsQueueDetails>,
    /// <p>Details for a WAF WebACL.</p>
    #[serde(rename = "AwsWafWebAcl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_waf_web_acl: Option<AwsWafWebAclDetails>,
    /// <p>Details about a container resource related to a finding.</p>
    #[serde(rename = "Container")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<ContainerDetails>,
    /// <p><p>Details about a resource that are not available in a type-specific details object. Use the <code>Other</code> object in the following cases.</p> <ul> <li> <p>The type-specific object does not contain all of the fields that you want to populate. In this case, first use the type-specific object to populate those fields. Use the <code>Other</code> object to populate the fields that are missing from the type-specific object.</p> </li> <li> <p>The resource type does not have a corresponding object. This includes resources for which the type is <code>Other</code>. </p> </li> </ul></p>
    #[serde(rename = "Other")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Details about the account that was not processed.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SecurityHubResult {
    /// <p>An AWS account ID of the account that was not processed.</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The reason that the account was not processed.</p>
    #[serde(rename = "ProcessingResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_result: Option<String>,
}

/// <p>The severity of the finding.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Severity {
    /// <p><p>The severity value of the finding. The allowed values are the following.</p> <ul> <li> <p> <code>INFORMATIONAL</code> - No issue was found.</p> </li> <li> <p> <code>LOW</code> - The issue does not require action on its own.</p> </li> <li> <p> <code>MEDIUM</code> - The issue must be addressed but not urgently.</p> </li> <li> <p> <code>HIGH</code> - The issue must be addressed as a priority.</p> </li> <li> <p> <code>CRITICAL</code> - The issue must be remediated immediately to avoid it escalating.</p> </li> </ul></p>
    #[serde(rename = "Label")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// <p><p>Deprecated. This attribute is being deprecated. Instead of providing <code>Normalized</code>, provide <code>Label</code>.</p> <p>If you provide <code>Normalized</code> and do not provide <code>Label</code>, <code>Label</code> is set automatically as follows. </p> <ul> <li> <p>0 - <code>INFORMATIONAL</code> </p> </li> <li> <p>1–39 - <code>LOW</code> </p> </li> <li> <p>40–69 - <code>MEDIUM</code> </p> </li> <li> <p>70–89 - <code>HIGH</code> </p> </li> <li> <p>90–100 - <code>CRITICAL</code> </p> </li> </ul></p>
    #[serde(rename = "Normalized")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalized: Option<i64>,
    /// <p>The native severity from the finding product that generated the finding.</p>
    #[serde(rename = "Original")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original: Option<String>,
    /// <p>Deprecated. This attribute is being deprecated. Instead of providing <code>Product</code>, provide <code>Original</code>.</p> <p>The native severity as defined by the AWS service or integrated partner product that generated the finding.</p>
    #[serde(rename = "Product")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<f64>,
}

/// <p>Updates to the severity information for a finding.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SeverityUpdate {
    /// <p><p>The severity value of the finding. The allowed values are the following.</p> <ul> <li> <p> <code>INFORMATIONAL</code> - No issue was found.</p> </li> <li> <p> <code>LOW</code> - The issue does not require action on its own.</p> </li> <li> <p> <code>MEDIUM</code> - The issue must be addressed but not urgently.</p> </li> <li> <p> <code>HIGH</code> - The issue must be addressed as a priority.</p> </li> <li> <p> <code>CRITICAL</code> - The issue must be remediated immediately to avoid it escalating.</p> </li> </ul></p>
    #[serde(rename = "Label")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// <p><p>The normalized severity for the finding. This attribute is to be deprecated in favor of <code>Label</code>.</p> <p>If you provide <code>Normalized</code> and do not provide <code>Label</code>, <code>Label</code> is set automatically as follows.</p> <ul> <li> <p>0 - <code>INFORMATIONAL</code> </p> </li> <li> <p>1–39 - <code>LOW</code> </p> </li> <li> <p>40–69 - <code>MEDIUM</code> </p> </li> <li> <p>70–89 - <code>HIGH</code> </p> </li> <li> <p>90–100 - <code>CRITICAL</code> </p> </li> </ul></p>
    #[serde(rename = "Normalized")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalized: Option<i64>,
    /// <p>The native severity as defined by the AWS service or integrated partner product that generated the finding.</p>
    #[serde(rename = "Product")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<f64>,
}

/// <p>A collection of finding attributes used to sort findings.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SortCriterion {
    /// <p>The finding attribute used to sort findings.</p>
    #[serde(rename = "Field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// <p>The order used to sort findings.</p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

/// <p>Provides information about a specific standard.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Standard {
    /// <p>A description of the standard.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Whether the standard is enabled by default. When Security Hub is enabled from the console, if a standard is enabled by default, the check box for that standard is selected by default.</p> <p>When Security Hub is enabled using the <code>EnableSecurityHub</code> API operation, the standard is enabled by default unless <code>EnableDefaultStandards</code> is set to <code>false</code>.</p>
    #[serde(rename = "EnabledByDefault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_by_default: Option<bool>,
    /// <p>The name of the standard.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ARN of a standard.</p>
    #[serde(rename = "StandardsArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_arn: Option<String>,
}

/// <p>Details for an individual security standard control.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StandardsControl {
    /// <p>The identifier of the security standard control.</p>
    #[serde(rename = "ControlId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_id: Option<String>,
    /// <p>The current status of the security standard control. Indicates whether the control is enabled or disabled. Security Hub does not check against disabled controls.</p>
    #[serde(rename = "ControlStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_status: Option<String>,
    /// <p>The date and time that the status of the security standard control was most recently updated.</p>
    #[serde(rename = "ControlStatusUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_status_updated_at: Option<f64>,
    /// <p>The longer description of the security standard control. Provides information about what the control is checking for.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The reason provided for the most recent change in status for the control.</p>
    #[serde(rename = "DisabledReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<String>,
    /// <p>The list of requirements that are related to this control.</p>
    #[serde(rename = "RelatedRequirements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_requirements: Option<Vec<String>>,
    /// <p>A link to remediation information for the control in the Security Hub user documentation.</p>
    #[serde(rename = "RemediationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediation_url: Option<String>,
    /// <p>The severity of findings generated from this security standard control.</p> <p>The finding severity is based on an assessment of how easy it would be to compromise AWS resources if the issue is detected.</p>
    #[serde(rename = "SeverityRating")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_rating: Option<String>,
    /// <p>The ARN of the security standard control.</p>
    #[serde(rename = "StandardsControlArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_control_arn: Option<String>,
    /// <p>The title of the security standard control.</p>
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// <p>A resource that represents your subscription to a supported standard.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StandardsSubscription {
    /// <p>The ARN of a standard.</p>
    #[serde(rename = "StandardsArn")]
    pub standards_arn: String,
    /// <p>A key-value pair of input for the standard.</p>
    #[serde(rename = "StandardsInput")]
    pub standards_input: ::std::collections::HashMap<String, String>,
    /// <p>The status of the standards subscription.</p>
    #[serde(rename = "StandardsStatus")]
    pub standards_status: String,
    /// <p>The ARN of a resource that represents your subscription to a supported standard.</p>
    #[serde(rename = "StandardsSubscriptionArn")]
    pub standards_subscription_arn: String,
}

/// <p>The standard that you want to enable.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StandardsSubscriptionRequest {
    /// <p>The ARN of the standard that you want to enable. To view the list of available standards and their ARNs, use the <code> <a>DescribeStandards</a> </code> operation.</p>
    #[serde(rename = "StandardsArn")]
    pub standards_arn: String,
    /// <p>A key-value pair of input for the standard.</p>
    #[serde(rename = "StandardsInput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_input: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Provides additional context for the value of <code>Compliance.Status</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusReason {
    /// <p>The corresponding description for the status reason code.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A code that represents a reason for the control status. For the list of status reason codes and their meanings, see <a href="https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-standards-results.html#securityhub-standards-results-asff">Standards-related information in the ASFF</a> in the <i>AWS Security Hub User Guide</i>. </p>
    #[serde(rename = "ReasonCode")]
    pub reason_code: String,
}

/// <p>A string filter for querying findings.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StringFilter {
    /// <p>The condition to be applied to a string value when querying for findings. </p>
    #[serde(rename = "Comparison")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison: Option<String>,
    /// <p>The string filter value.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The ARN of the resource to apply the tags to.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The tags to add to the resource.</p>
    #[serde(rename = "Tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

/// <p>Details about the threat intelligence related to a finding.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThreatIntelIndicator {
    /// <p>The category of a threat intelligence indicator.</p>
    #[serde(rename = "Category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// <p>The date and time when the most recent instance of a threat intelligence indicator was observed.</p>
    #[serde(rename = "LastObservedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_observed_at: Option<String>,
    /// <p>The source of the threat intelligence indicator.</p>
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// <p>The URL to the page or site where you can get more information about the threat intelligence indicator.</p>
    #[serde(rename = "SourceUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    /// <p>The type of threat intelligence indicator.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The value of a threat intelligence indicator.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The ARN of the resource to remove the tags from.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The tag keys associated with the tags to remove from the resource.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateActionTargetRequest {
    /// <p>The ARN of the custom action target to update.</p>
    #[serde(rename = "ActionTargetArn")]
    pub action_target_arn: String,
    /// <p>The updated description for the custom action target.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The updated name of the custom action target.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateActionTargetResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateFindingsRequest {
    /// <p>A collection of attributes that specify which findings you want to update.</p>
    #[serde(rename = "Filters")]
    pub filters: AwsSecurityFindingFilters,
    /// <p>The updated note for the finding.</p>
    #[serde(rename = "Note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<NoteUpdate>,
    /// <p>The updated record state for the finding.</p>
    #[serde(rename = "RecordState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateFindingsResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateInsightRequest {
    /// <p>The updated filters that define this insight.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<AwsSecurityFindingFilters>,
    /// <p>The updated <code>GroupBy</code> attribute that defines this insight.</p>
    #[serde(rename = "GroupByAttribute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by_attribute: Option<String>,
    /// <p>The ARN of the insight that you want to update.</p>
    #[serde(rename = "InsightArn")]
    pub insight_arn: String,
    /// <p>The updated name for the insight.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateInsightResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateStandardsControlRequest {
    /// <p>The updated status of the security standard control.</p>
    #[serde(rename = "ControlStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_status: Option<String>,
    /// <p>A description of the reason why you are disabling a security standard control.</p>
    #[serde(rename = "DisabledReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<String>,
    /// <p>The ARN of the security standard control to enable or disable.</p>
    #[serde(rename = "StandardsControlArn")]
    pub standards_control_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateStandardsControlResponse {}

/// <p>Details about the action that CloudFront or AWS WAF takes when a web request matches the conditions in the rule. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WafAction {
    /// <p><p>Specifies how you want AWS WAF to respond to requests that match the settings in a rule.</p> <p>Valid settings include the following:</p> <ul> <li> <p> <code>ALLOW</code> - AWS WAF allows requests</p> </li> <li> <p> <code>BLOCK</code> - AWS WAF blocks requests</p> </li> <li> <p> <code>COUNT</code> - AWS WAF increments a counter of the requests that match all of the conditions in the rule. AWS WAF then continues to inspect the web request based on the remaining rules in the web ACL. You can&#39;t specify <code>COUNT</code> for the default action for a WebACL.</p> </li> </ul></p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Details about a rule to exclude from a rule group.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WafExcludedRule {
    /// <p>The unique identifier for the rule to exclude from the rule group.</p>
    #[serde(rename = "RuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
}

/// <p>Details about an override action for a rule.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WafOverrideAction {
    /// <p> <code>COUNT</code> overrides the action specified by the individual rule within a <code>RuleGroup</code> .</p> <p>If set to <code>NONE</code>, the rule's action takes place.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Provides information about the status of the investigation into a finding.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Workflow {
    /// <p><p>The status of the investigation into the finding. The allowed values are the following.</p> <ul> <li> <p> <code>NEW</code> - The initial state of a finding, before it is reviewed.</p> </li> <li> <p> <code>NOTIFIED</code> - Indicates that you notified the resource owner about the security issue. Used when the initial reviewer is not the resource owner, and needs intervention from the resource owner.</p> </li> <li> <p> <code>SUPPRESSED</code> - The finding will not be reviewed again and will not be acted upon.</p> </li> <li> <p> <code>RESOLVED</code> - The finding was reviewed and remediated and is now considered resolved. </p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Used to update information about the investigation into the finding.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct WorkflowUpdate {
    /// <p><p>The status of the investigation into the finding. The allowed values are the following.</p> <ul> <li> <p> <code>NEW</code> - The initial state of a finding, before it is reviewed.</p> </li> <li> <p> <code>NOTIFIED</code> - Indicates that you notified the resource owner about the security issue. Used when the initial reviewer is not the resource owner, and needs intervention from the resource owner.</p> </li> <li> <p> <code>RESOLVED</code> - The finding was reviewed and remediated and is now considered resolved.</p> </li> <li> <p> <code>SUPPRESSED</code> - The finding will not be reviewed again and will not be acted upon.</p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Errors returned by AcceptInvitation
#[derive(Debug, PartialEq)]
pub enum AcceptInvitationError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>AWS Security Hub isn't enabled for the account used to make this request.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl AcceptInvitationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AcceptInvitationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(AcceptInvitationError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(AcceptInvitationError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(AcceptInvitationError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(AcceptInvitationError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AcceptInvitationError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AcceptInvitationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AcceptInvitationError::Internal(ref cause) => write!(f, "{}", cause),
            AcceptInvitationError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            AcceptInvitationError::InvalidInput(ref cause) => write!(f, "{}", cause),
            AcceptInvitationError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            AcceptInvitationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AcceptInvitationError {}
/// Errors returned by BatchDisableStandards
#[derive(Debug, PartialEq)]
pub enum BatchDisableStandardsError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>AWS Security Hub isn't enabled for the account used to make this request.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
}

impl BatchDisableStandardsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchDisableStandardsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(BatchDisableStandardsError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(BatchDisableStandardsError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(BatchDisableStandardsError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(BatchDisableStandardsError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchDisableStandardsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchDisableStandardsError::Internal(ref cause) => write!(f, "{}", cause),
            BatchDisableStandardsError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            BatchDisableStandardsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            BatchDisableStandardsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchDisableStandardsError {}
/// Errors returned by BatchEnableStandards
#[derive(Debug, PartialEq)]
pub enum BatchEnableStandardsError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>AWS Security Hub isn't enabled for the account used to make this request.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
}

impl BatchEnableStandardsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchEnableStandardsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(BatchEnableStandardsError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(BatchEnableStandardsError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(BatchEnableStandardsError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(BatchEnableStandardsError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchEnableStandardsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchEnableStandardsError::Internal(ref cause) => write!(f, "{}", cause),
            BatchEnableStandardsError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            BatchEnableStandardsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            BatchEnableStandardsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchEnableStandardsError {}
/// Errors returned by BatchImportFindings
#[derive(Debug, PartialEq)]
pub enum BatchImportFindingsError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>AWS Security Hub isn't enabled for the account used to make this request.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
}

impl BatchImportFindingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchImportFindingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(BatchImportFindingsError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(BatchImportFindingsError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(BatchImportFindingsError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(BatchImportFindingsError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchImportFindingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchImportFindingsError::Internal(ref cause) => write!(f, "{}", cause),
            BatchImportFindingsError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            BatchImportFindingsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            BatchImportFindingsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchImportFindingsError {}
/// Errors returned by BatchUpdateFindings
#[derive(Debug, PartialEq)]
pub enum BatchUpdateFindingsError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>AWS Security Hub isn't enabled for the account used to make this request.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
}

impl BatchUpdateFindingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchUpdateFindingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(BatchUpdateFindingsError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(BatchUpdateFindingsError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(BatchUpdateFindingsError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(BatchUpdateFindingsError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchUpdateFindingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchUpdateFindingsError::Internal(ref cause) => write!(f, "{}", cause),
            BatchUpdateFindingsError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            BatchUpdateFindingsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            BatchUpdateFindingsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchUpdateFindingsError {}
/// Errors returned by CreateActionTarget
#[derive(Debug, PartialEq)]
pub enum CreateActionTargetError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>AWS Security Hub isn't enabled for the account used to make this request.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The resource specified in the request conflicts with an existing resource.</p>
    ResourceConflict(String),
}

impl CreateActionTargetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateActionTargetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(CreateActionTargetError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(CreateActionTargetError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateActionTargetError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateActionTargetError::LimitExceeded(err.msg))
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(CreateActionTargetError::ResourceConflict(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateActionTargetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateActionTargetError::Internal(ref cause) => write!(f, "{}", cause),
            CreateActionTargetError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            CreateActionTargetError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateActionTargetError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateActionTargetError::ResourceConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateActionTargetError {}
/// Errors returned by CreateInsight
#[derive(Debug, PartialEq)]
pub enum CreateInsightError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>AWS Security Hub isn't enabled for the account used to make this request.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The resource specified in the request conflicts with an existing resource.</p>
    ResourceConflict(String),
}

impl CreateInsightError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateInsightError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(CreateInsightError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(CreateInsightError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateInsightError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateInsightError::LimitExceeded(err.msg))
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(CreateInsightError::ResourceConflict(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateInsightError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateInsightError::Internal(ref cause) => write!(f, "{}", cause),
            CreateInsightError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            CreateInsightError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateInsightError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateInsightError::ResourceConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateInsightError {}
/// Errors returned by CreateMembers
#[derive(Debug, PartialEq)]
pub enum CreateMembersError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>AWS Security Hub isn't enabled for the account used to make this request.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The resource specified in the request conflicts with an existing resource.</p>
    ResourceConflict(String),
}

impl CreateMembersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateMembersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(CreateMembersError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(CreateMembersError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateMembersError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateMembersError::LimitExceeded(err.msg))
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(CreateMembersError::ResourceConflict(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateMembersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateMembersError::Internal(ref cause) => write!(f, "{}", cause),
            CreateMembersError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            CreateMembersError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateMembersError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateMembersError::ResourceConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateMembersError {}
/// Errors returned by DeclineInvitations
#[derive(Debug, PartialEq)]
pub enum DeclineInvitationsError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>AWS Security Hub isn't enabled for the account used to make this request.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl DeclineInvitationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeclineInvitationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DeclineInvitationsError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(DeclineInvitationsError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeclineInvitationsError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeclineInvitationsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeclineInvitationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeclineInvitationsError::Internal(ref cause) => write!(f, "{}", cause),
            DeclineInvitationsError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            DeclineInvitationsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeclineInvitationsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeclineInvitationsError {}
/// Errors returned by DeleteActionTarget
#[derive(Debug, PartialEq)]
pub enum DeleteActionTargetError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>AWS Security Hub isn't enabled for the account used to make this request.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl DeleteActionTargetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteActionTargetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DeleteActionTargetError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(DeleteActionTargetError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteActionTargetError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteActionTargetError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteActionTargetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteActionTargetError::Internal(ref cause) => write!(f, "{}", cause),
            DeleteActionTargetError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            DeleteActionTargetError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteActionTargetError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteActionTargetError {}
/// Errors returned by DeleteInsight
#[derive(Debug, PartialEq)]
pub enum DeleteInsightError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>AWS Security Hub isn't enabled for the account used to make this request.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl DeleteInsightError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteInsightError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DeleteInsightError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(DeleteInsightError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteInsightError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteInsightError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteInsightError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteInsightError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteInsightError::Internal(ref cause) => write!(f, "{}", cause),
            DeleteInsightError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            DeleteInsightError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteInsightError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteInsightError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteInsightError {}
/// Errors returned by DeleteInvitations
#[derive(Debug, PartialEq)]
pub enum DeleteInvitationsError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>AWS Security Hub isn't enabled for the account used to make this request.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl DeleteInvitationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteInvitationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DeleteInvitationsError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(DeleteInvitationsError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteInvitationsError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteInvitationsError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteInvitationsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteInvitationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteInvitationsError::Internal(ref cause) => write!(f, "{}", cause),
            DeleteInvitationsError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            DeleteInvitationsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteInvitationsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteInvitationsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteInvitationsError {}
/// Errors returned by DeleteMembers
#[derive(Debug, PartialEq)]
pub enum DeleteMembersError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>AWS Security Hub isn't enabled for the account used to make this request.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl DeleteMembersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteMembersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DeleteMembersError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(DeleteMembersError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteMembersError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteMembersError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteMembersError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteMembersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteMembersError::Internal(ref cause) => write!(f, "{}", cause),
            DeleteMembersError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            DeleteMembersError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteMembersError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteMembersError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteMembersError {}
/// Errors returned by DescribeActionTargets
#[derive(Debug, PartialEq)]
pub enum DescribeActionTargetsError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>AWS Security Hub isn't enabled for the account used to make this request.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl DescribeActionTargetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeActionTargetsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DescribeActionTargetsError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(DescribeActionTargetsError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeActionTargetsError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeActionTargetsError::ResourceNotFound(
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
impl fmt::Display for DescribeActionTargetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeActionTargetsError::Internal(ref cause) => write!(f, "{}", cause),
            DescribeActionTargetsError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            DescribeActionTargetsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DescribeActionTargetsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeActionTargetsError {}
/// Errors returned by DescribeHub
#[derive(Debug, PartialEq)]
pub enum DescribeHubError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>AWS Security Hub isn't enabled for the account used to make this request.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl DescribeHubError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeHubError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DescribeHubError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(DescribeHubError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeHubError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DescribeHubError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeHubError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeHubError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeHubError::Internal(ref cause) => write!(f, "{}", cause),
            DescribeHubError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            DescribeHubError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DescribeHubError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DescribeHubError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeHubError {}
/// Errors returned by DescribeProducts
#[derive(Debug, PartialEq)]
pub enum DescribeProductsError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>AWS Security Hub isn't enabled for the account used to make this request.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
}

impl DescribeProductsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeProductsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DescribeProductsError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(DescribeProductsError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeProductsError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DescribeProductsError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeProductsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeProductsError::Internal(ref cause) => write!(f, "{}", cause),
            DescribeProductsError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            DescribeProductsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DescribeProductsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeProductsError {}
/// Errors returned by DescribeStandards
#[derive(Debug, PartialEq)]
pub enum DescribeStandardsError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>AWS Security Hub isn't enabled for the account used to make this request.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
}

impl DescribeStandardsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeStandardsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DescribeStandardsError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(DescribeStandardsError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeStandardsError::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeStandardsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeStandardsError::Internal(ref cause) => write!(f, "{}", cause),
            DescribeStandardsError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            DescribeStandardsError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeStandardsError {}
/// Errors returned by DescribeStandardsControls
#[derive(Debug, PartialEq)]
pub enum DescribeStandardsControlsError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>AWS Security Hub isn't enabled for the account used to make this request.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl DescribeStandardsControlsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeStandardsControlsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DescribeStandardsControlsError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(DescribeStandardsControlsError::InvalidAccess(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeStandardsControlsError::InvalidInput(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeStandardsControlsError::ResourceNotFound(
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
impl fmt::Display for DescribeStandardsControlsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeStandardsControlsError::Internal(ref cause) => write!(f, "{}", cause),
            DescribeStandardsControlsError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            DescribeStandardsControlsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DescribeStandardsControlsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeStandardsControlsError {}
/// Errors returned by DisableImportFindingsForProduct
#[derive(Debug, PartialEq)]
pub enum DisableImportFindingsForProductError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>AWS Security Hub isn't enabled for the account used to make this request.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl DisableImportFindingsForProductError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisableImportFindingsForProductError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DisableImportFindingsForProductError::Internal(
                        err.msg,
                    ))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(
                        DisableImportFindingsForProductError::InvalidAccess(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(
                        DisableImportFindingsForProductError::InvalidInput(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(
                        DisableImportFindingsForProductError::LimitExceeded(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DisableImportFindingsForProductError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisableImportFindingsForProductError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisableImportFindingsForProductError::Internal(ref cause) => write!(f, "{}", cause),
            DisableImportFindingsForProductError::InvalidAccess(ref cause) => {
                write!(f, "{}", cause)
            }
            DisableImportFindingsForProductError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DisableImportFindingsForProductError::LimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            DisableImportFindingsForProductError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DisableImportFindingsForProductError {}
/// Errors returned by DisableSecurityHub
#[derive(Debug, PartialEq)]
pub enum DisableSecurityHubError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>AWS Security Hub isn't enabled for the account used to make this request.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl DisableSecurityHubError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisableSecurityHubError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DisableSecurityHubError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(DisableSecurityHubError::InvalidAccess(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DisableSecurityHubError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DisableSecurityHubError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisableSecurityHubError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisableSecurityHubError::Internal(ref cause) => write!(f, "{}", cause),
            DisableSecurityHubError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            DisableSecurityHubError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DisableSecurityHubError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisableSecurityHubError {}
/// Errors returned by DisassociateFromMasterAccount
#[derive(Debug, PartialEq)]
pub enum DisassociateFromMasterAccountError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>AWS Security Hub isn't enabled for the account used to make this request.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl DisassociateFromMasterAccountError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateFromMasterAccountError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DisassociateFromMasterAccountError::Internal(
                        err.msg,
                    ))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(DisassociateFromMasterAccountError::InvalidAccess(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DisassociateFromMasterAccountError::InvalidInput(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DisassociateFromMasterAccountError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DisassociateFromMasterAccountError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisassociateFromMasterAccountError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateFromMasterAccountError::Internal(ref cause) => write!(f, "{}", cause),
            DisassociateFromMasterAccountError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            DisassociateFromMasterAccountError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DisassociateFromMasterAccountError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DisassociateFromMasterAccountError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DisassociateFromMasterAccountError {}
/// Errors returned by DisassociateMembers
#[derive(Debug, PartialEq)]
pub enum DisassociateMembersError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>AWS Security Hub isn't enabled for the account used to make this request.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl DisassociateMembersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisassociateMembersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DisassociateMembersError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(DisassociateMembersError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DisassociateMembersError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DisassociateMembersError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DisassociateMembersError::ResourceNotFound(
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
impl fmt::Display for DisassociateMembersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateMembersError::Internal(ref cause) => write!(f, "{}", cause),
            DisassociateMembersError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            DisassociateMembersError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DisassociateMembersError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DisassociateMembersError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisassociateMembersError {}
/// Errors returned by EnableImportFindingsForProduct
#[derive(Debug, PartialEq)]
pub enum EnableImportFindingsForProductError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>AWS Security Hub isn't enabled for the account used to make this request.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The resource specified in the request conflicts with an existing resource.</p>
    ResourceConflict(String),
}

impl EnableImportFindingsForProductError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<EnableImportFindingsForProductError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(EnableImportFindingsForProductError::Internal(
                        err.msg,
                    ))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(
                        EnableImportFindingsForProductError::InvalidAccess(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(EnableImportFindingsForProductError::InvalidInput(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(
                        EnableImportFindingsForProductError::LimitExceeded(err.msg),
                    )
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(
                        EnableImportFindingsForProductError::ResourceConflict(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for EnableImportFindingsForProductError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EnableImportFindingsForProductError::Internal(ref cause) => write!(f, "{}", cause),
            EnableImportFindingsForProductError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            EnableImportFindingsForProductError::InvalidInput(ref cause) => write!(f, "{}", cause),
            EnableImportFindingsForProductError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            EnableImportFindingsForProductError::ResourceConflict(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for EnableImportFindingsForProductError {}
/// Errors returned by EnableSecurityHub
#[derive(Debug, PartialEq)]
pub enum EnableSecurityHubError {
    /// <p>You don't have permission to perform the action specified in the request.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>AWS Security Hub isn't enabled for the account used to make this request.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The resource specified in the request conflicts with an existing resource.</p>
    ResourceConflict(String),
}

impl EnableSecurityHubError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<EnableSecurityHubError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(EnableSecurityHubError::AccessDenied(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(EnableSecurityHubError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(EnableSecurityHubError::InvalidAccess(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(EnableSecurityHubError::LimitExceeded(err.msg))
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(EnableSecurityHubError::ResourceConflict(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for EnableSecurityHubError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EnableSecurityHubError::AccessDenied(ref cause) => write!(f, "{}", cause),
            EnableSecurityHubError::Internal(ref cause) => write!(f, "{}", cause),
            EnableSecurityHubError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            EnableSecurityHubError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            EnableSecurityHubError::ResourceConflict(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for EnableSecurityHubError {}
/// Errors returned by GetEnabledStandards
#[derive(Debug, PartialEq)]
pub enum GetEnabledStandardsError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>AWS Security Hub isn't enabled for the account used to make this request.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
}

impl GetEnabledStandardsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetEnabledStandardsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(GetEnabledStandardsError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(GetEnabledStandardsError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetEnabledStandardsError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetEnabledStandardsError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetEnabledStandardsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetEnabledStandardsError::Internal(ref cause) => write!(f, "{}", cause),
            GetEnabledStandardsError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            GetEnabledStandardsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetEnabledStandardsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetEnabledStandardsError {}
/// Errors returned by GetFindings
#[derive(Debug, PartialEq)]
pub enum GetFindingsError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>AWS Security Hub isn't enabled for the account used to make this request.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
}

impl GetFindingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetFindingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(GetFindingsError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(GetFindingsError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetFindingsError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetFindingsError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetFindingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetFindingsError::Internal(ref cause) => write!(f, "{}", cause),
            GetFindingsError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            GetFindingsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetFindingsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetFindingsError {}
/// Errors returned by GetInsightResults
#[derive(Debug, PartialEq)]
pub enum GetInsightResultsError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>AWS Security Hub isn't enabled for the account used to make this request.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl GetInsightResultsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetInsightResultsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(GetInsightResultsError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(GetInsightResultsError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetInsightResultsError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetInsightResultsError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetInsightResultsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetInsightResultsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetInsightResultsError::Internal(ref cause) => write!(f, "{}", cause),
            GetInsightResultsError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            GetInsightResultsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetInsightResultsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetInsightResultsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetInsightResultsError {}
/// Errors returned by GetInsights
#[derive(Debug, PartialEq)]
pub enum GetInsightsError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>AWS Security Hub isn't enabled for the account used to make this request.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl GetInsightsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetInsightsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(GetInsightsError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(GetInsightsError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetInsightsError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetInsightsError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetInsightsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetInsightsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetInsightsError::Internal(ref cause) => write!(f, "{}", cause),
            GetInsightsError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            GetInsightsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetInsightsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetInsightsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetInsightsError {}
/// Errors returned by GetInvitationsCount
#[derive(Debug, PartialEq)]
pub enum GetInvitationsCountError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>AWS Security Hub isn't enabled for the account used to make this request.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
}

impl GetInvitationsCountError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetInvitationsCountError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(GetInvitationsCountError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(GetInvitationsCountError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetInvitationsCountError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetInvitationsCountError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetInvitationsCountError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetInvitationsCountError::Internal(ref cause) => write!(f, "{}", cause),
            GetInvitationsCountError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            GetInvitationsCountError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetInvitationsCountError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetInvitationsCountError {}
/// Errors returned by GetMasterAccount
#[derive(Debug, PartialEq)]
pub enum GetMasterAccountError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>AWS Security Hub isn't enabled for the account used to make this request.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl GetMasterAccountError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetMasterAccountError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(GetMasterAccountError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(GetMasterAccountError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetMasterAccountError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetMasterAccountError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetMasterAccountError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetMasterAccountError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetMasterAccountError::Internal(ref cause) => write!(f, "{}", cause),
            GetMasterAccountError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            GetMasterAccountError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetMasterAccountError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetMasterAccountError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetMasterAccountError {}
/// Errors returned by GetMembers
#[derive(Debug, PartialEq)]
pub enum GetMembersError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>AWS Security Hub isn't enabled for the account used to make this request.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl GetMembersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetMembersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(GetMembersError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(GetMembersError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetMembersError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetMembersError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetMembersError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetMembersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetMembersError::Internal(ref cause) => write!(f, "{}", cause),
            GetMembersError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            GetMembersError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetMembersError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetMembersError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetMembersError {}
/// Errors returned by InviteMembers
#[derive(Debug, PartialEq)]
pub enum InviteMembersError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>AWS Security Hub isn't enabled for the account used to make this request.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl InviteMembersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<InviteMembersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(InviteMembersError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(InviteMembersError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(InviteMembersError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(InviteMembersError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(InviteMembersError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for InviteMembersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InviteMembersError::Internal(ref cause) => write!(f, "{}", cause),
            InviteMembersError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            InviteMembersError::InvalidInput(ref cause) => write!(f, "{}", cause),
            InviteMembersError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            InviteMembersError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for InviteMembersError {}
/// Errors returned by ListEnabledProductsForImport
#[derive(Debug, PartialEq)]
pub enum ListEnabledProductsForImportError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>AWS Security Hub isn't enabled for the account used to make this request.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
}

impl ListEnabledProductsForImportError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListEnabledProductsForImportError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(ListEnabledProductsForImportError::Internal(
                        err.msg,
                    ))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(ListEnabledProductsForImportError::InvalidAccess(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListEnabledProductsForImportError::LimitExceeded(
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
impl fmt::Display for ListEnabledProductsForImportError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListEnabledProductsForImportError::Internal(ref cause) => write!(f, "{}", cause),
            ListEnabledProductsForImportError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            ListEnabledProductsForImportError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListEnabledProductsForImportError {}
/// Errors returned by ListInvitations
#[derive(Debug, PartialEq)]
pub enum ListInvitationsError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>AWS Security Hub isn't enabled for the account used to make this request.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
}

impl ListInvitationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListInvitationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(ListInvitationsError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(ListInvitationsError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListInvitationsError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListInvitationsError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListInvitationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListInvitationsError::Internal(ref cause) => write!(f, "{}", cause),
            ListInvitationsError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            ListInvitationsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListInvitationsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListInvitationsError {}
/// Errors returned by ListMembers
#[derive(Debug, PartialEq)]
pub enum ListMembersError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>AWS Security Hub isn't enabled for the account used to make this request.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
}

impl ListMembersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListMembersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(ListMembersError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(ListMembersError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListMembersError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListMembersError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListMembersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListMembersError::Internal(ref cause) => write!(f, "{}", cause),
            ListMembersError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            ListMembersError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListMembersError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListMembersError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(ListTagsForResourceError::Internal(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::ResourceNotFound(
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
impl fmt::Display for ListTagsForResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsForResourceError::Internal(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(TagResourceError::Internal(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(TagResourceError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
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
            TagResourceError::Internal(ref cause) => write!(f, "{}", cause),
            TagResourceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(UntagResourceError::Internal(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UntagResourceError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::ResourceNotFound(err.msg))
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
            UntagResourceError::Internal(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateActionTarget
#[derive(Debug, PartialEq)]
pub enum UpdateActionTargetError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>AWS Security Hub isn't enabled for the account used to make this request.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl UpdateActionTargetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateActionTargetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(UpdateActionTargetError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(UpdateActionTargetError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateActionTargetError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateActionTargetError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateActionTargetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateActionTargetError::Internal(ref cause) => write!(f, "{}", cause),
            UpdateActionTargetError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            UpdateActionTargetError::InvalidInput(ref cause) => write!(f, "{}", cause),
            UpdateActionTargetError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateActionTargetError {}
/// Errors returned by UpdateFindings
#[derive(Debug, PartialEq)]
pub enum UpdateFindingsError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>AWS Security Hub isn't enabled for the account used to make this request.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl UpdateFindingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateFindingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(UpdateFindingsError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(UpdateFindingsError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateFindingsError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateFindingsError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateFindingsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateFindingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateFindingsError::Internal(ref cause) => write!(f, "{}", cause),
            UpdateFindingsError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            UpdateFindingsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            UpdateFindingsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateFindingsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateFindingsError {}
/// Errors returned by UpdateInsight
#[derive(Debug, PartialEq)]
pub enum UpdateInsightError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>AWS Security Hub isn't enabled for the account used to make this request.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl UpdateInsightError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateInsightError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(UpdateInsightError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(UpdateInsightError::InvalidAccess(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateInsightError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateInsightError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateInsightError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateInsightError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateInsightError::Internal(ref cause) => write!(f, "{}", cause),
            UpdateInsightError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            UpdateInsightError::InvalidInput(ref cause) => write!(f, "{}", cause),
            UpdateInsightError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateInsightError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateInsightError {}
/// Errors returned by UpdateStandardsControl
#[derive(Debug, PartialEq)]
pub enum UpdateStandardsControlError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>AWS Security Hub isn't enabled for the account used to make this request.</p>
    InvalidAccess(String),
    /// <p>The request was rejected because you supplied an invalid or out-of-range value for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because we can't find the specified resource.</p>
    ResourceNotFound(String),
}

impl UpdateStandardsControlError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateStandardsControlError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(UpdateStandardsControlError::Internal(err.msg))
                }
                "InvalidAccessException" => {
                    return RusotoError::Service(UpdateStandardsControlError::InvalidAccess(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateStandardsControlError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateStandardsControlError::ResourceNotFound(
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
impl fmt::Display for UpdateStandardsControlError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateStandardsControlError::Internal(ref cause) => write!(f, "{}", cause),
            UpdateStandardsControlError::InvalidAccess(ref cause) => write!(f, "{}", cause),
            UpdateStandardsControlError::InvalidInput(ref cause) => write!(f, "{}", cause),
            UpdateStandardsControlError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateStandardsControlError {}
/// Trait representing the capabilities of the AWS SecurityHub API. AWS SecurityHub clients implement this trait.
#[async_trait]
pub trait SecurityHub {
    /// <p>Accepts the invitation to be a member account and be monitored by the Security Hub master account that the invitation was sent from.</p> <p>When the member account accepts the invitation, permission is granted to the master account to view findings generated in the member account.</p>
    async fn accept_invitation(
        &self,
        input: AcceptInvitationRequest,
    ) -> Result<AcceptInvitationResponse, RusotoError<AcceptInvitationError>>;

    /// <p>Disables the standards specified by the provided <code>StandardsSubscriptionArns</code>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-standards.html">Security Standards</a> section of the <i>AWS Security Hub User Guide</i>.</p>
    async fn batch_disable_standards(
        &self,
        input: BatchDisableStandardsRequest,
    ) -> Result<BatchDisableStandardsResponse, RusotoError<BatchDisableStandardsError>>;

    /// <p>Enables the standards specified by the provided <code>StandardsArn</code>. To obtain the ARN for a standard, use the <code> <a>DescribeStandards</a> </code> operation.</p> <p>For more information, see the <a href="https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-standards.html">Security Standards</a> section of the <i>AWS Security Hub User Guide</i>.</p>
    async fn batch_enable_standards(
        &self,
        input: BatchEnableStandardsRequest,
    ) -> Result<BatchEnableStandardsResponse, RusotoError<BatchEnableStandardsError>>;

    /// <p><p>Imports security findings generated from an integrated third-party product into Security Hub. This action is requested by the integrated product to import its findings into Security Hub.</p> <p>The maximum allowed size for a finding is 240 Kb. An error is returned for any finding larger than 240 Kb.</p> <p>After a finding is created, <code>BatchImportFindings</code> cannot be used to update the following finding fields and objects, which Security Hub customers use to manage their investigation workflow.</p> <ul> <li> <p> <code>Confidence</code> </p> </li> <li> <p> <code>Criticality</code> </p> </li> <li> <p> <code>Note</code> </p> </li> <li> <p> <code>RelatedFindings</code> </p> </li> <li> <p> <code>Severity</code> </p> </li> <li> <p> <code>Types</code> </p> </li> <li> <p> <code>UserDefinedFields</code> </p> </li> <li> <p> <code>VerificationState</code> </p> </li> <li> <p> <code>Workflow</code> </p> </li> </ul></p>
    async fn batch_import_findings(
        &self,
        input: BatchImportFindingsRequest,
    ) -> Result<BatchImportFindingsResponse, RusotoError<BatchImportFindingsError>>;

    /// <p>Used by Security Hub customers to update information about their investigation into a finding. Requested by master accounts or member accounts. Master accounts can update findings for their account and their member accounts. Member accounts can update findings for their account.</p> <p>Updates from <code>BatchUpdateFindings</code> do not affect the value of <code>UpdatedAt</code> for a finding.</p> <p>Master accounts can use <code>BatchUpdateFindings</code> to update the following finding fields and objects.</p> <ul> <li> <p> <code>Confidence</code> </p> </li> <li> <p> <code>Criticality</code> </p> </li> <li> <p> <code>Note</code> </p> </li> <li> <p> <code>RelatedFindings</code> </p> </li> <li> <p> <code>Severity</code> </p> </li> <li> <p> <code>Types</code> </p> </li> <li> <p> <code>UserDefinedFields</code> </p> </li> <li> <p> <code>VerificationState</code> </p> </li> <li> <p> <code>Workflow</code> </p> </li> </ul> <p>Member accounts can only use <code>BatchUpdateFindings</code> to update the Note object.</p>
    async fn batch_update_findings(
        &self,
        input: BatchUpdateFindingsRequest,
    ) -> Result<BatchUpdateFindingsResponse, RusotoError<BatchUpdateFindingsError>>;

    /// <p>Creates a custom action target in Security Hub.</p> <p>You can use custom actions on findings and insights in Security Hub to trigger target actions in Amazon CloudWatch Events.</p>
    async fn create_action_target(
        &self,
        input: CreateActionTargetRequest,
    ) -> Result<CreateActionTargetResponse, RusotoError<CreateActionTargetError>>;

    /// <p>Creates a custom insight in Security Hub. An insight is a consolidation of findings that relate to a security issue that requires attention or remediation.</p> <p>To group the related findings in the insight, use the <code>GroupByAttribute</code>.</p>
    async fn create_insight(
        &self,
        input: CreateInsightRequest,
    ) -> Result<CreateInsightResponse, RusotoError<CreateInsightError>>;

    /// <p>Creates a member association in Security Hub between the specified accounts and the account used to make the request, which is the master account. To successfully create a member, you must use this action from an account that already has Security Hub enabled. To enable Security Hub, you can use the <code> <a>EnableSecurityHub</a> </code> operation.</p> <p>After you use <code>CreateMembers</code> to create member account associations in Security Hub, you must use the <code> <a>InviteMembers</a> </code> operation to invite the accounts to enable Security Hub and become member accounts in Security Hub.</p> <p>If the account owner accepts the invitation, the account becomes a member account in Security Hub. A permissions policy is added that permits the master account to view the findings generated in the member account. When Security Hub is enabled in the invited account, findings start to be sent to both the member and master accounts.</p> <p>To remove the association between the master and member accounts, use the <code> <a>DisassociateFromMasterAccount</a> </code> or <code> <a>DisassociateMembers</a> </code> operation.</p>
    async fn create_members(
        &self,
        input: CreateMembersRequest,
    ) -> Result<CreateMembersResponse, RusotoError<CreateMembersError>>;

    /// <p>Declines invitations to become a member account.</p>
    async fn decline_invitations(
        &self,
        input: DeclineInvitationsRequest,
    ) -> Result<DeclineInvitationsResponse, RusotoError<DeclineInvitationsError>>;

    /// <p>Deletes a custom action target from Security Hub.</p> <p>Deleting a custom action target does not affect any findings or insights that were already sent to Amazon CloudWatch Events using the custom action.</p>
    async fn delete_action_target(
        &self,
        input: DeleteActionTargetRequest,
    ) -> Result<DeleteActionTargetResponse, RusotoError<DeleteActionTargetError>>;

    /// <p>Deletes the insight specified by the <code>InsightArn</code>.</p>
    async fn delete_insight(
        &self,
        input: DeleteInsightRequest,
    ) -> Result<DeleteInsightResponse, RusotoError<DeleteInsightError>>;

    /// <p>Deletes invitations received by the AWS account to become a member account.</p>
    async fn delete_invitations(
        &self,
        input: DeleteInvitationsRequest,
    ) -> Result<DeleteInvitationsResponse, RusotoError<DeleteInvitationsError>>;

    /// <p>Deletes the specified member accounts from Security Hub.</p>
    async fn delete_members(
        &self,
        input: DeleteMembersRequest,
    ) -> Result<DeleteMembersResponse, RusotoError<DeleteMembersError>>;

    /// <p>Returns a list of the custom action targets in Security Hub in your account.</p>
    async fn describe_action_targets(
        &self,
        input: DescribeActionTargetsRequest,
    ) -> Result<DescribeActionTargetsResponse, RusotoError<DescribeActionTargetsError>>;

    /// <p>Returns details about the Hub resource in your account, including the <code>HubArn</code> and the time when you enabled Security Hub.</p>
    async fn describe_hub(
        &self,
        input: DescribeHubRequest,
    ) -> Result<DescribeHubResponse, RusotoError<DescribeHubError>>;

    /// <p>Returns information about the available products that you can subscribe to and integrate with Security Hub in order to consolidate findings.</p>
    async fn describe_products(
        &self,
        input: DescribeProductsRequest,
    ) -> Result<DescribeProductsResponse, RusotoError<DescribeProductsError>>;

    /// <p>Returns a list of the available standards in Security Hub.</p> <p>For each standard, the results include the standard ARN, the name, and a description. </p>
    async fn describe_standards(
        &self,
        input: DescribeStandardsRequest,
    ) -> Result<DescribeStandardsResponse, RusotoError<DescribeStandardsError>>;

    /// <p>Returns a list of security standards controls.</p> <p>For each control, the results include information about whether it is currently enabled, the severity, and a link to remediation information.</p>
    async fn describe_standards_controls(
        &self,
        input: DescribeStandardsControlsRequest,
    ) -> Result<DescribeStandardsControlsResponse, RusotoError<DescribeStandardsControlsError>>;

    /// <p>Disables the integration of the specified product with Security Hub. After the integration is disabled, findings from that product are no longer sent to Security Hub.</p>
    async fn disable_import_findings_for_product(
        &self,
        input: DisableImportFindingsForProductRequest,
    ) -> Result<
        DisableImportFindingsForProductResponse,
        RusotoError<DisableImportFindingsForProductError>,
    >;

    /// <p>Disables Security Hub in your account only in the current Region. To disable Security Hub in all Regions, you must submit one request per Region where you have enabled Security Hub.</p> <p>When you disable Security Hub for a master account, it doesn't disable Security Hub for any associated member accounts.</p> <p>When you disable Security Hub, your existing findings and insights and any Security Hub configuration settings are deleted after 90 days and cannot be recovered. Any standards that were enabled are disabled, and your master and member account associations are removed.</p> <p>If you want to save your existing findings, you must export them before you disable Security Hub.</p>
    async fn disable_security_hub(
        &self,
    ) -> Result<DisableSecurityHubResponse, RusotoError<DisableSecurityHubError>>;

    /// <p>Disassociates the current Security Hub member account from the associated master account.</p>
    async fn disassociate_from_master_account(
        &self,
    ) -> Result<
        DisassociateFromMasterAccountResponse,
        RusotoError<DisassociateFromMasterAccountError>,
    >;

    /// <p>Disassociates the specified member accounts from the associated master account.</p>
    async fn disassociate_members(
        &self,
        input: DisassociateMembersRequest,
    ) -> Result<DisassociateMembersResponse, RusotoError<DisassociateMembersError>>;

    /// <p>Enables the integration of a partner product with Security Hub. Integrated products send findings to Security Hub.</p> <p>When you enable a product integration, a permissions policy that grants permission for the product to send findings to Security Hub is applied.</p>
    async fn enable_import_findings_for_product(
        &self,
        input: EnableImportFindingsForProductRequest,
    ) -> Result<
        EnableImportFindingsForProductResponse,
        RusotoError<EnableImportFindingsForProductError>,
    >;

    /// <p>Enables Security Hub for your account in the current Region or the Region you specify in the request.</p> <p>When you enable Security Hub, you grant to Security Hub the permissions necessary to gather findings from other services that are integrated with Security Hub.</p> <p>When you use the <code>EnableSecurityHub</code> operation to enable Security Hub, you also automatically enable the following standards.</p> <ul> <li> <p>CIS AWS Foundations</p> </li> <li> <p>AWS Foundational Security Best Practices</p> </li> </ul> <p>You do not enable the Payment Card Industry Data Security Standard (PCI DSS) standard. </p> <p>To not enable the automatically enabled standards, set <code>EnableDefaultStandards</code> to <code>false</code>.</p> <p>After you enable Security Hub, to enable a standard, use the <code> <a>BatchEnableStandards</a> </code> operation. To disable a standard, use the <code> <a>BatchDisableStandards</a> </code> operation.</p> <p>To learn more, see <a href="https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-settingup.html">Setting Up AWS Security Hub</a> in the <i>AWS Security Hub User Guide</i>.</p>
    async fn enable_security_hub(
        &self,
        input: EnableSecurityHubRequest,
    ) -> Result<EnableSecurityHubResponse, RusotoError<EnableSecurityHubError>>;

    /// <p>Returns a list of the standards that are currently enabled.</p>
    async fn get_enabled_standards(
        &self,
        input: GetEnabledStandardsRequest,
    ) -> Result<GetEnabledStandardsResponse, RusotoError<GetEnabledStandardsError>>;

    /// <p>Returns a list of findings that match the specified criteria.</p>
    async fn get_findings(
        &self,
        input: GetFindingsRequest,
    ) -> Result<GetFindingsResponse, RusotoError<GetFindingsError>>;

    /// <p>Lists the results of the Security Hub insight specified by the insight ARN.</p>
    async fn get_insight_results(
        &self,
        input: GetInsightResultsRequest,
    ) -> Result<GetInsightResultsResponse, RusotoError<GetInsightResultsError>>;

    /// <p>Lists and describes insights for the specified insight ARNs.</p>
    async fn get_insights(
        &self,
        input: GetInsightsRequest,
    ) -> Result<GetInsightsResponse, RusotoError<GetInsightsError>>;

    /// <p>Returns the count of all Security Hub membership invitations that were sent to the current member account, not including the currently accepted invitation. </p>
    async fn get_invitations_count(
        &self,
    ) -> Result<GetInvitationsCountResponse, RusotoError<GetInvitationsCountError>>;

    /// <p>Provides the details for the Security Hub master account for the current member account. </p>
    async fn get_master_account(
        &self,
    ) -> Result<GetMasterAccountResponse, RusotoError<GetMasterAccountError>>;

    /// <p>Returns the details for the Security Hub member accounts for the specified account IDs.</p>
    async fn get_members(
        &self,
        input: GetMembersRequest,
    ) -> Result<GetMembersResponse, RusotoError<GetMembersError>>;

    /// <p>Invites other AWS accounts to become member accounts for the Security Hub master account that the invitation is sent from.</p> <p>Before you can use this action to invite a member, you must first use the <code> <a>CreateMembers</a> </code> action to create the member account in Security Hub.</p> <p>When the account owner accepts the invitation to become a member account and enables Security Hub, the master account can view the findings generated from the member account.</p>
    async fn invite_members(
        &self,
        input: InviteMembersRequest,
    ) -> Result<InviteMembersResponse, RusotoError<InviteMembersError>>;

    /// <p>Lists all findings-generating solutions (products) that you are subscribed to receive findings from in Security Hub.</p>
    async fn list_enabled_products_for_import(
        &self,
        input: ListEnabledProductsForImportRequest,
    ) -> Result<ListEnabledProductsForImportResponse, RusotoError<ListEnabledProductsForImportError>>;

    /// <p>Lists all Security Hub membership invitations that were sent to the current AWS account. </p>
    async fn list_invitations(
        &self,
        input: ListInvitationsRequest,
    ) -> Result<ListInvitationsResponse, RusotoError<ListInvitationsError>>;

    /// <p>Lists details about all member accounts for the current Security Hub master account.</p>
    async fn list_members(
        &self,
        input: ListMembersRequest,
    ) -> Result<ListMembersResponse, RusotoError<ListMembersError>>;

    /// <p>Returns a list of tags associated with a resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Adds one or more tags to a resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Removes one or more tags from a resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p>Updates the name and description of a custom action target in Security Hub.</p>
    async fn update_action_target(
        &self,
        input: UpdateActionTargetRequest,
    ) -> Result<UpdateActionTargetResponse, RusotoError<UpdateActionTargetError>>;

    /// <p> <code>UpdateFindings</code> is deprecated. Instead of <code>UpdateFindings</code>, use <code>BatchUpdateFindings</code>.</p> <p>Updates the <code>Note</code> and <code>RecordState</code> of the Security Hub-aggregated findings that the filter attributes specify. Any member account that can view the finding also sees the update to the finding.</p>
    async fn update_findings(
        &self,
        input: UpdateFindingsRequest,
    ) -> Result<UpdateFindingsResponse, RusotoError<UpdateFindingsError>>;

    /// <p>Updates the Security Hub insight identified by the specified insight ARN.</p>
    async fn update_insight(
        &self,
        input: UpdateInsightRequest,
    ) -> Result<UpdateInsightResponse, RusotoError<UpdateInsightError>>;

    /// <p>Used to control whether an individual security standard control is enabled or disabled.</p>
    async fn update_standards_control(
        &self,
        input: UpdateStandardsControlRequest,
    ) -> Result<UpdateStandardsControlResponse, RusotoError<UpdateStandardsControlError>>;
}
/// A client for the AWS SecurityHub API.
#[derive(Clone)]
pub struct SecurityHubClient {
    client: Client,
    region: region::Region,
}

impl SecurityHubClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> SecurityHubClient {
        SecurityHubClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> SecurityHubClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        SecurityHubClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> SecurityHubClient {
        SecurityHubClient { client, region }
    }
}

#[async_trait]
impl SecurityHub for SecurityHubClient {
    /// <p>Accepts the invitation to be a member account and be monitored by the Security Hub master account that the invitation was sent from.</p> <p>When the member account accepts the invitation, permission is granted to the master account to view findings generated in the member account.</p>
    async fn accept_invitation(
        &self,
        input: AcceptInvitationRequest,
    ) -> Result<AcceptInvitationResponse, RusotoError<AcceptInvitationError>> {
        let request_uri = "/master";

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<AcceptInvitationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AcceptInvitationError::from_response(response))
        }
    }

    /// <p>Disables the standards specified by the provided <code>StandardsSubscriptionArns</code>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-standards.html">Security Standards</a> section of the <i>AWS Security Hub User Guide</i>.</p>
    async fn batch_disable_standards(
        &self,
        input: BatchDisableStandardsRequest,
    ) -> Result<BatchDisableStandardsResponse, RusotoError<BatchDisableStandardsError>> {
        let request_uri = "/standards/deregister";

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<BatchDisableStandardsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(BatchDisableStandardsError::from_response(response))
        }
    }

    /// <p>Enables the standards specified by the provided <code>StandardsArn</code>. To obtain the ARN for a standard, use the <code> <a>DescribeStandards</a> </code> operation.</p> <p>For more information, see the <a href="https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-standards.html">Security Standards</a> section of the <i>AWS Security Hub User Guide</i>.</p>
    async fn batch_enable_standards(
        &self,
        input: BatchEnableStandardsRequest,
    ) -> Result<BatchEnableStandardsResponse, RusotoError<BatchEnableStandardsError>> {
        let request_uri = "/standards/register";

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<BatchEnableStandardsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(BatchEnableStandardsError::from_response(response))
        }
    }

    /// <p><p>Imports security findings generated from an integrated third-party product into Security Hub. This action is requested by the integrated product to import its findings into Security Hub.</p> <p>The maximum allowed size for a finding is 240 Kb. An error is returned for any finding larger than 240 Kb.</p> <p>After a finding is created, <code>BatchImportFindings</code> cannot be used to update the following finding fields and objects, which Security Hub customers use to manage their investigation workflow.</p> <ul> <li> <p> <code>Confidence</code> </p> </li> <li> <p> <code>Criticality</code> </p> </li> <li> <p> <code>Note</code> </p> </li> <li> <p> <code>RelatedFindings</code> </p> </li> <li> <p> <code>Severity</code> </p> </li> <li> <p> <code>Types</code> </p> </li> <li> <p> <code>UserDefinedFields</code> </p> </li> <li> <p> <code>VerificationState</code> </p> </li> <li> <p> <code>Workflow</code> </p> </li> </ul></p>
    async fn batch_import_findings(
        &self,
        input: BatchImportFindingsRequest,
    ) -> Result<BatchImportFindingsResponse, RusotoError<BatchImportFindingsError>> {
        let request_uri = "/findings/import";

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<BatchImportFindingsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(BatchImportFindingsError::from_response(response))
        }
    }

    /// <p>Used by Security Hub customers to update information about their investigation into a finding. Requested by master accounts or member accounts. Master accounts can update findings for their account and their member accounts. Member accounts can update findings for their account.</p> <p>Updates from <code>BatchUpdateFindings</code> do not affect the value of <code>UpdatedAt</code> for a finding.</p> <p>Master accounts can use <code>BatchUpdateFindings</code> to update the following finding fields and objects.</p> <ul> <li> <p> <code>Confidence</code> </p> </li> <li> <p> <code>Criticality</code> </p> </li> <li> <p> <code>Note</code> </p> </li> <li> <p> <code>RelatedFindings</code> </p> </li> <li> <p> <code>Severity</code> </p> </li> <li> <p> <code>Types</code> </p> </li> <li> <p> <code>UserDefinedFields</code> </p> </li> <li> <p> <code>VerificationState</code> </p> </li> <li> <p> <code>Workflow</code> </p> </li> </ul> <p>Member accounts can only use <code>BatchUpdateFindings</code> to update the Note object.</p>
    async fn batch_update_findings(
        &self,
        input: BatchUpdateFindingsRequest,
    ) -> Result<BatchUpdateFindingsResponse, RusotoError<BatchUpdateFindingsError>> {
        let request_uri = "/findings/batchupdate";

        let mut request = SignedRequest::new("PATCH", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<BatchUpdateFindingsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(BatchUpdateFindingsError::from_response(response))
        }
    }

    /// <p>Creates a custom action target in Security Hub.</p> <p>You can use custom actions on findings and insights in Security Hub to trigger target actions in Amazon CloudWatch Events.</p>
    async fn create_action_target(
        &self,
        input: CreateActionTargetRequest,
    ) -> Result<CreateActionTargetResponse, RusotoError<CreateActionTargetError>> {
        let request_uri = "/actionTargets";

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateActionTargetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateActionTargetError::from_response(response))
        }
    }

    /// <p>Creates a custom insight in Security Hub. An insight is a consolidation of findings that relate to a security issue that requires attention or remediation.</p> <p>To group the related findings in the insight, use the <code>GroupByAttribute</code>.</p>
    async fn create_insight(
        &self,
        input: CreateInsightRequest,
    ) -> Result<CreateInsightResponse, RusotoError<CreateInsightError>> {
        let request_uri = "/insights";

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateInsightResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateInsightError::from_response(response))
        }
    }

    /// <p>Creates a member association in Security Hub between the specified accounts and the account used to make the request, which is the master account. To successfully create a member, you must use this action from an account that already has Security Hub enabled. To enable Security Hub, you can use the <code> <a>EnableSecurityHub</a> </code> operation.</p> <p>After you use <code>CreateMembers</code> to create member account associations in Security Hub, you must use the <code> <a>InviteMembers</a> </code> operation to invite the accounts to enable Security Hub and become member accounts in Security Hub.</p> <p>If the account owner accepts the invitation, the account becomes a member account in Security Hub. A permissions policy is added that permits the master account to view the findings generated in the member account. When Security Hub is enabled in the invited account, findings start to be sent to both the member and master accounts.</p> <p>To remove the association between the master and member accounts, use the <code> <a>DisassociateFromMasterAccount</a> </code> or <code> <a>DisassociateMembers</a> </code> operation.</p>
    async fn create_members(
        &self,
        input: CreateMembersRequest,
    ) -> Result<CreateMembersResponse, RusotoError<CreateMembersError>> {
        let request_uri = "/members";

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateMembersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateMembersError::from_response(response))
        }
    }

    /// <p>Declines invitations to become a member account.</p>
    async fn decline_invitations(
        &self,
        input: DeclineInvitationsRequest,
    ) -> Result<DeclineInvitationsResponse, RusotoError<DeclineInvitationsError>> {
        let request_uri = "/invitations/decline";

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeclineInvitationsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeclineInvitationsError::from_response(response))
        }
    }

    /// <p>Deletes a custom action target from Security Hub.</p> <p>Deleting a custom action target does not affect any findings or insights that were already sent to Amazon CloudWatch Events using the custom action.</p>
    async fn delete_action_target(
        &self,
        input: DeleteActionTargetRequest,
    ) -> Result<DeleteActionTargetResponse, RusotoError<DeleteActionTargetError>> {
        let request_uri = format!(
            "/actionTargets/{action_target_arn}",
            action_target_arn = input.action_target_arn
        );

        let mut request = SignedRequest::new("DELETE", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteActionTargetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteActionTargetError::from_response(response))
        }
    }

    /// <p>Deletes the insight specified by the <code>InsightArn</code>.</p>
    async fn delete_insight(
        &self,
        input: DeleteInsightRequest,
    ) -> Result<DeleteInsightResponse, RusotoError<DeleteInsightError>> {
        let request_uri = format!("/insights/{insight_arn}", insight_arn = input.insight_arn);

        let mut request = SignedRequest::new("DELETE", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteInsightResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteInsightError::from_response(response))
        }
    }

    /// <p>Deletes invitations received by the AWS account to become a member account.</p>
    async fn delete_invitations(
        &self,
        input: DeleteInvitationsRequest,
    ) -> Result<DeleteInvitationsResponse, RusotoError<DeleteInvitationsError>> {
        let request_uri = "/invitations/delete";

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteInvitationsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteInvitationsError::from_response(response))
        }
    }

    /// <p>Deletes the specified member accounts from Security Hub.</p>
    async fn delete_members(
        &self,
        input: DeleteMembersRequest,
    ) -> Result<DeleteMembersResponse, RusotoError<DeleteMembersError>> {
        let request_uri = "/members/delete";

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteMembersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteMembersError::from_response(response))
        }
    }

    /// <p>Returns a list of the custom action targets in Security Hub in your account.</p>
    async fn describe_action_targets(
        &self,
        input: DescribeActionTargetsRequest,
    ) -> Result<DescribeActionTargetsResponse, RusotoError<DescribeActionTargetsError>> {
        let request_uri = "/actionTargets/get";

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeActionTargetsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeActionTargetsError::from_response(response))
        }
    }

    /// <p>Returns details about the Hub resource in your account, including the <code>HubArn</code> and the time when you enabled Security Hub.</p>
    async fn describe_hub(
        &self,
        input: DescribeHubRequest,
    ) -> Result<DescribeHubResponse, RusotoError<DescribeHubError>> {
        let request_uri = "/accounts";

        let mut request = SignedRequest::new("GET", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.hub_arn {
            params.put("HubArn", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeHubResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeHubError::from_response(response))
        }
    }

    /// <p>Returns information about the available products that you can subscribe to and integrate with Security Hub in order to consolidate findings.</p>
    async fn describe_products(
        &self,
        input: DescribeProductsRequest,
    ) -> Result<DescribeProductsResponse, RusotoError<DescribeProductsError>> {
        let request_uri = "/products";

        let mut request = SignedRequest::new("GET", "securityhub", &self.region, &request_uri);
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
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeProductsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeProductsError::from_response(response))
        }
    }

    /// <p>Returns a list of the available standards in Security Hub.</p> <p>For each standard, the results include the standard ARN, the name, and a description. </p>
    async fn describe_standards(
        &self,
        input: DescribeStandardsRequest,
    ) -> Result<DescribeStandardsResponse, RusotoError<DescribeStandardsError>> {
        let request_uri = "/standards";

        let mut request = SignedRequest::new("GET", "securityhub", &self.region, &request_uri);
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
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeStandardsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeStandardsError::from_response(response))
        }
    }

    /// <p>Returns a list of security standards controls.</p> <p>For each control, the results include information about whether it is currently enabled, the severity, and a link to remediation information.</p>
    async fn describe_standards_controls(
        &self,
        input: DescribeStandardsControlsRequest,
    ) -> Result<DescribeStandardsControlsResponse, RusotoError<DescribeStandardsControlsError>>
    {
        let request_uri = format!(
            "/standards/controls/{standards_subscription_arn}",
            standards_subscription_arn = input.standards_subscription_arn
        );

        let mut request = SignedRequest::new("GET", "securityhub", &self.region, &request_uri);
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
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeStandardsControlsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeStandardsControlsError::from_response(response))
        }
    }

    /// <p>Disables the integration of the specified product with Security Hub. After the integration is disabled, findings from that product are no longer sent to Security Hub.</p>
    async fn disable_import_findings_for_product(
        &self,
        input: DisableImportFindingsForProductRequest,
    ) -> Result<
        DisableImportFindingsForProductResponse,
        RusotoError<DisableImportFindingsForProductError>,
    > {
        let request_uri = format!(
            "/productSubscriptions/{product_subscription_arn}",
            product_subscription_arn = input.product_subscription_arn
        );

        let mut request = SignedRequest::new("DELETE", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DisableImportFindingsForProductResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DisableImportFindingsForProductError::from_response(
                response,
            ))
        }
    }

    /// <p>Disables Security Hub in your account only in the current Region. To disable Security Hub in all Regions, you must submit one request per Region where you have enabled Security Hub.</p> <p>When you disable Security Hub for a master account, it doesn't disable Security Hub for any associated member accounts.</p> <p>When you disable Security Hub, your existing findings and insights and any Security Hub configuration settings are deleted after 90 days and cannot be recovered. Any standards that were enabled are disabled, and your master and member account associations are removed.</p> <p>If you want to save your existing findings, you must export them before you disable Security Hub.</p>
    async fn disable_security_hub(
        &self,
    ) -> Result<DisableSecurityHubResponse, RusotoError<DisableSecurityHubError>> {
        let request_uri = "/accounts";

        let mut request = SignedRequest::new("DELETE", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DisableSecurityHubResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DisableSecurityHubError::from_response(response))
        }
    }

    /// <p>Disassociates the current Security Hub member account from the associated master account.</p>
    async fn disassociate_from_master_account(
        &self,
    ) -> Result<
        DisassociateFromMasterAccountResponse,
        RusotoError<DisassociateFromMasterAccountError>,
    > {
        let request_uri = "/master/disassociate";

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DisassociateFromMasterAccountResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateFromMasterAccountError::from_response(response))
        }
    }

    /// <p>Disassociates the specified member accounts from the associated master account.</p>
    async fn disassociate_members(
        &self,
        input: DisassociateMembersRequest,
    ) -> Result<DisassociateMembersResponse, RusotoError<DisassociateMembersError>> {
        let request_uri = "/members/disassociate";

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DisassociateMembersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateMembersError::from_response(response))
        }
    }

    /// <p>Enables the integration of a partner product with Security Hub. Integrated products send findings to Security Hub.</p> <p>When you enable a product integration, a permissions policy that grants permission for the product to send findings to Security Hub is applied.</p>
    async fn enable_import_findings_for_product(
        &self,
        input: EnableImportFindingsForProductRequest,
    ) -> Result<
        EnableImportFindingsForProductResponse,
        RusotoError<EnableImportFindingsForProductError>,
    > {
        let request_uri = "/productSubscriptions";

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<EnableImportFindingsForProductResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(EnableImportFindingsForProductError::from_response(response))
        }
    }

    /// <p>Enables Security Hub for your account in the current Region or the Region you specify in the request.</p> <p>When you enable Security Hub, you grant to Security Hub the permissions necessary to gather findings from other services that are integrated with Security Hub.</p> <p>When you use the <code>EnableSecurityHub</code> operation to enable Security Hub, you also automatically enable the following standards.</p> <ul> <li> <p>CIS AWS Foundations</p> </li> <li> <p>AWS Foundational Security Best Practices</p> </li> </ul> <p>You do not enable the Payment Card Industry Data Security Standard (PCI DSS) standard. </p> <p>To not enable the automatically enabled standards, set <code>EnableDefaultStandards</code> to <code>false</code>.</p> <p>After you enable Security Hub, to enable a standard, use the <code> <a>BatchEnableStandards</a> </code> operation. To disable a standard, use the <code> <a>BatchDisableStandards</a> </code> operation.</p> <p>To learn more, see <a href="https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-settingup.html">Setting Up AWS Security Hub</a> in the <i>AWS Security Hub User Guide</i>.</p>
    async fn enable_security_hub(
        &self,
        input: EnableSecurityHubRequest,
    ) -> Result<EnableSecurityHubResponse, RusotoError<EnableSecurityHubError>> {
        let request_uri = "/accounts";

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<EnableSecurityHubResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(EnableSecurityHubError::from_response(response))
        }
    }

    /// <p>Returns a list of the standards that are currently enabled.</p>
    async fn get_enabled_standards(
        &self,
        input: GetEnabledStandardsRequest,
    ) -> Result<GetEnabledStandardsResponse, RusotoError<GetEnabledStandardsError>> {
        let request_uri = "/standards/get";

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetEnabledStandardsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetEnabledStandardsError::from_response(response))
        }
    }

    /// <p>Returns a list of findings that match the specified criteria.</p>
    async fn get_findings(
        &self,
        input: GetFindingsRequest,
    ) -> Result<GetFindingsResponse, RusotoError<GetFindingsError>> {
        let request_uri = "/findings";

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetFindingsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetFindingsError::from_response(response))
        }
    }

    /// <p>Lists the results of the Security Hub insight specified by the insight ARN.</p>
    async fn get_insight_results(
        &self,
        input: GetInsightResultsRequest,
    ) -> Result<GetInsightResultsResponse, RusotoError<GetInsightResultsError>> {
        let request_uri = format!(
            "/insights/results/{insight_arn}",
            insight_arn = input.insight_arn
        );

        let mut request = SignedRequest::new("GET", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetInsightResultsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetInsightResultsError::from_response(response))
        }
    }

    /// <p>Lists and describes insights for the specified insight ARNs.</p>
    async fn get_insights(
        &self,
        input: GetInsightsRequest,
    ) -> Result<GetInsightsResponse, RusotoError<GetInsightsError>> {
        let request_uri = "/insights/get";

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetInsightsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetInsightsError::from_response(response))
        }
    }

    /// <p>Returns the count of all Security Hub membership invitations that were sent to the current member account, not including the currently accepted invitation. </p>
    async fn get_invitations_count(
        &self,
    ) -> Result<GetInvitationsCountResponse, RusotoError<GetInvitationsCountError>> {
        let request_uri = "/invitations/count";

        let mut request = SignedRequest::new("GET", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetInvitationsCountResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetInvitationsCountError::from_response(response))
        }
    }

    /// <p>Provides the details for the Security Hub master account for the current member account. </p>
    async fn get_master_account(
        &self,
    ) -> Result<GetMasterAccountResponse, RusotoError<GetMasterAccountError>> {
        let request_uri = "/master";

        let mut request = SignedRequest::new("GET", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetMasterAccountResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetMasterAccountError::from_response(response))
        }
    }

    /// <p>Returns the details for the Security Hub member accounts for the specified account IDs.</p>
    async fn get_members(
        &self,
        input: GetMembersRequest,
    ) -> Result<GetMembersResponse, RusotoError<GetMembersError>> {
        let request_uri = "/members/get";

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetMembersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetMembersError::from_response(response))
        }
    }

    /// <p>Invites other AWS accounts to become member accounts for the Security Hub master account that the invitation is sent from.</p> <p>Before you can use this action to invite a member, you must first use the <code> <a>CreateMembers</a> </code> action to create the member account in Security Hub.</p> <p>When the account owner accepts the invitation to become a member account and enables Security Hub, the master account can view the findings generated from the member account.</p>
    async fn invite_members(
        &self,
        input: InviteMembersRequest,
    ) -> Result<InviteMembersResponse, RusotoError<InviteMembersError>> {
        let request_uri = "/members/invite";

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<InviteMembersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(InviteMembersError::from_response(response))
        }
    }

    /// <p>Lists all findings-generating solutions (products) that you are subscribed to receive findings from in Security Hub.</p>
    async fn list_enabled_products_for_import(
        &self,
        input: ListEnabledProductsForImportRequest,
    ) -> Result<ListEnabledProductsForImportResponse, RusotoError<ListEnabledProductsForImportError>>
    {
        let request_uri = "/productSubscriptions";

        let mut request = SignedRequest::new("GET", "securityhub", &self.region, &request_uri);
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
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListEnabledProductsForImportResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListEnabledProductsForImportError::from_response(response))
        }
    }

    /// <p>Lists all Security Hub membership invitations that were sent to the current AWS account. </p>
    async fn list_invitations(
        &self,
        input: ListInvitationsRequest,
    ) -> Result<ListInvitationsResponse, RusotoError<ListInvitationsError>> {
        let request_uri = "/invitations";

        let mut request = SignedRequest::new("GET", "securityhub", &self.region, &request_uri);
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
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListInvitationsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListInvitationsError::from_response(response))
        }
    }

    /// <p>Lists details about all member accounts for the current Security Hub master account.</p>
    async fn list_members(
        &self,
        input: ListMembersRequest,
    ) -> Result<ListMembersResponse, RusotoError<ListMembersError>> {
        let request_uri = "/members";

        let mut request = SignedRequest::new("GET", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        if let Some(ref x) = input.only_associated {
            params.put("OnlyAssociated", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListMembersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListMembersError::from_response(response))
        }
    }

    /// <p>Returns a list of tags associated with a resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("GET", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTagsForResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p>Adds one or more tags to a resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("POST", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<TagResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Removes one or more tags from a resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("DELETE", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        for item in input.tag_keys.iter() {
            params.put("tagKeys", item);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UntagResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p>Updates the name and description of a custom action target in Security Hub.</p>
    async fn update_action_target(
        &self,
        input: UpdateActionTargetRequest,
    ) -> Result<UpdateActionTargetResponse, RusotoError<UpdateActionTargetError>> {
        let request_uri = format!(
            "/actionTargets/{action_target_arn}",
            action_target_arn = input.action_target_arn
        );

        let mut request = SignedRequest::new("PATCH", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateActionTargetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateActionTargetError::from_response(response))
        }
    }

    /// <p> <code>UpdateFindings</code> is deprecated. Instead of <code>UpdateFindings</code>, use <code>BatchUpdateFindings</code>.</p> <p>Updates the <code>Note</code> and <code>RecordState</code> of the Security Hub-aggregated findings that the filter attributes specify. Any member account that can view the finding also sees the update to the finding.</p>
    async fn update_findings(
        &self,
        input: UpdateFindingsRequest,
    ) -> Result<UpdateFindingsResponse, RusotoError<UpdateFindingsError>> {
        let request_uri = "/findings";

        let mut request = SignedRequest::new("PATCH", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateFindingsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateFindingsError::from_response(response))
        }
    }

    /// <p>Updates the Security Hub insight identified by the specified insight ARN.</p>
    async fn update_insight(
        &self,
        input: UpdateInsightRequest,
    ) -> Result<UpdateInsightResponse, RusotoError<UpdateInsightError>> {
        let request_uri = format!("/insights/{insight_arn}", insight_arn = input.insight_arn);

        let mut request = SignedRequest::new("PATCH", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateInsightResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateInsightError::from_response(response))
        }
    }

    /// <p>Used to control whether an individual security standard control is enabled or disabled.</p>
    async fn update_standards_control(
        &self,
        input: UpdateStandardsControlRequest,
    ) -> Result<UpdateStandardsControlResponse, RusotoError<UpdateStandardsControlError>> {
        let request_uri = format!(
            "/standards/control/{standards_control_arn}",
            standards_control_arn = input.standards_control_arn
        );

        let mut request = SignedRequest::new("PATCH", "securityhub", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateStandardsControlResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateStandardsControlError::from_response(response))
        }
    }
}
