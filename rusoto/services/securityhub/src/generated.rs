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
    /// <p>An optional string that you want CloudFront to prefix to the access log filenames for this distribution.</p>
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

/// <p>A complex type that describes the Amazon S3 bucket, HTTP server (for example, a web server), Amazon MediaStore, or other server from which CloudFront gets your files.</p>
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
    /// <p>The type of IP addresses used by the subnets for your load balancer. The possible values are ipv4 (for IPv4 addresses) and dualstack (for IPv4 and IPv6 addresses).</p>
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
    /// <p>The source of the CMK's key material. When this value is AWS_KMS, AWS KMS created the key material. When this value is EXTERNAL, the key material was imported from your existing key management infrastructure or the CMK lacks key material. When this value is AWS_CLOUDHSM, the key material was created in the AWS CloudHSM cluster associated with a custom key store.</p>
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

/// <p>The details of an Amazon S3 bucket.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsS3BucketDetails {
    /// <p>The canonical user ID of the owner of the S3 bucket.</p>
    #[serde(rename = "OwnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    /// <p>The display name of the owner of the S3 bucket.</p>
    #[serde(rename = "OwnerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_name: Option<String>,
}

/// <p><p>Provides consistent format for the contents of the Security Hub-aggregated findings. <code>AwsSecurityFinding</code> format enables you to share findings between AWS security services and third-party solutions, and compliance checks.</p> <note> <p>A finding is a potential security issue generated either by AWS services (Amazon GuardDuty, Amazon Inspector, and Amazon Macie) or by the integrated third-party solutions and compliance checks.</p> </note></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsSecurityFinding {
    /// <p>The AWS account ID that a finding is generated in.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>This data type is exclusive to findings that are generated as the result of a check run against a specific rule in a supported standard (for example, CIS AWS Foundations). Contains compliance-related finding details.</p>
    #[serde(rename = "Compliance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance: Option<Compliance>,
    /// <p>A finding's confidence. Confidence is defined as the likelihood that a finding accurately identifies the behavior or issue that it was intended to identify. Confidence is scored on a 0-100 basis using a ratio scale, where 0 means zero percent confidence and 100 means 100 percent confidence.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<i64>,
    /// <p>An ISO8601-formatted timestamp that indicates when the security-findings provider created the potential security issue that a finding captured.</p>
    #[serde(rename = "CreatedAt")]
    pub created_at: String,
    /// <p>The level of importance assigned to the resources associated with the finding. A score of 0 means that the underlying resources have no criticality, and a score of 100 is reserved for the most critical resources.</p>
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
    /// <p>The identifier for the solution-specific component (a discrete unit of logic) that generated a finding. In various security-findings providers' solutions, this generator can be called a rule, a check, a detector, a plug-in, etc. </p>
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
    /// <p>The ARN generated by Security Hub that uniquely identifies a third-party company (security-findings provider) after this provider's product (solution that generates findings) is registered with Security Hub. </p>
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
    /// <p>Threat intel details related to a finding.</p>
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
    /// <p>The workflow state of a finding. </p>
    #[serde(rename = "WorkflowState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_state: Option<String>,
}

/// <p>A collection of attributes that are applied to all active Security Hub-aggregated findings and that result in a subset of findings that are included in this insight. </p>
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
    /// <p>Exclusive to findings that are generated as the result of a check run against a specific rule in a supported standard (for example, CIS AWS Foundations). Contains compliance-related finding details.</p>
    #[serde(rename = "ComplianceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_status: Option<Vec<StringFilter>>,
    /// <p>A finding's confidence. Confidence is defined as the likelihood that a finding accurately identifies the behavior or issue that it was intended to identify. Confidence is scored on a 0-100 basis using a ratio scale, where 0 means zero percent confidence and 100 means 100 percent confidence.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<Vec<NumberFilter>>,
    /// <p>An ISO8601-formatted timestamp that indicates when the security-findings provider captured the potential security issue that a finding captured.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<DateFilter>>,
    /// <p>The level of importance assigned to the resources associated with the finding. A score of 0 means that the underlying resources have no criticality, and a score of 100 is reserved for the most critical resources.</p>
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
    /// <p>The identifier for the solution-specific component (a discrete unit of logic) that generated a finding. In various security-findings providers' solutions, this generator can be called a rule, a check, a detector, a plug-in, etc.</p>
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
    /// <p>The date/time the instance was launched.</p>
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
    /// <p>The category of a threat intel indicator.</p>
    #[serde(rename = "ThreatIntelIndicatorCategory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_intel_indicator_category: Option<Vec<StringFilter>>,
    /// <p>The date/time of the last observation of a threat intel indicator.</p>
    #[serde(rename = "ThreatIntelIndicatorLastObservedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_intel_indicator_last_observed_at: Option<Vec<DateFilter>>,
    /// <p>The source of the threat intel.</p>
    #[serde(rename = "ThreatIntelIndicatorSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_intel_indicator_source: Option<Vec<StringFilter>>,
    /// <p>The URL for more details from the source of the threat intel.</p>
    #[serde(rename = "ThreatIntelIndicatorSourceUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_intel_indicator_source_url: Option<Vec<StringFilter>>,
    /// <p>The type of a threat intel indicator.</p>
    #[serde(rename = "ThreatIntelIndicatorType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_intel_indicator_type: Option<Vec<StringFilter>>,
    /// <p>The value of a threat intel indicator.</p>
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
}

/// <p>A wrapper type for the topic's Amazon Resource Name (ARN).</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsSnsTopicDetails {
    /// <p>The ID of an AWS-managed customer master key (CMK) for Amazon SNS or a custom CMK.</p>
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
    /// <p>The Amazon Resource Name (ARN) of the dead-letter queue to which Amazon SQS moves messages after the value of maxReceiveCount is exceeded. </p>
    #[serde(rename = "DeadLetterTargetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_letter_target_arn: Option<String>,
    /// <p>The length of time, in seconds, for which Amazon SQS can reuse a data key to encrypt or decrypt messages before calling AWS KMS again.</p>
    #[serde(rename = "KmsDataKeyReusePeriodSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_data_key_reuse_period_seconds: Option<i64>,
    /// <p>The ID of an AWS-managed customer master key (CMK) for Amazon SQS or a custom CMK.</p>
    #[serde(rename = "KmsMasterKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_master_key_id: Option<String>,
    /// <p>The name of the new queue.</p>
    #[serde(rename = "QueueName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_name: Option<String>,
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
    /// <p><p>The list of standards compliance checks to enable.</p> <important> <p>In this release, Security Hub supports only the CIS AWS Foundations standard.</p> <p>The ARN for the standard is <code>arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0</code>.</p> </important></p>
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
    /// <p>The list of the findings that failed to import.</p>
    #[serde(rename = "FailedFindings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_findings: Option<Vec<ImportFindingsError>>,
    /// <p>The number of findings that were successfully imported.</p>
    #[serde(rename = "SuccessCount")]
    pub success_count: i64,
}

/// <p><p>Exclusive to findings that are generated as the result of a check run against a specific rule in a supported standard (for example, CIS AWS Foundations). Contains compliance-related finding details.</p> <p>Values include the following:</p> <ul> <li> <p>Allowed values are the following:</p> <ul> <li> <p> <code>PASSED</code> - Compliance check passed for all evaluated resources.</p> </li> <li> <p> <code>WARNING</code> - Some information is missing or this check is not supported given your configuration.</p> </li> <li> <p> <code>FAILED</code> - Compliance check failed for at least one evaluated resource.</p> </li> <li> <p> <code>NOT_AVAILABLE</code> - Check could not be performed due to a service outage or API error.</p> </li> </ul> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Compliance {
    /// <p>The result of a compliance check.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
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
    /// <p>One or more attributes used to filter the findings included in the insight. Only findings that match the criteria defined in the filters are included in the insight.</p>
    #[serde(rename = "Filters")]
    pub filters: AwsSecurityFindingFilters,
    /// <p>The attribute used as the aggregator to group related findings for the insight.</p>
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
    /// <p>A list of account ID and email address pairs of the accounts to associate with the Security Hub master account.</p>
    #[serde(rename = "AccountDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_details: Option<Vec<AccountDetails>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateMembersResponse {
    /// <p>A list of account ID and email address pairs of the AWS accounts that weren't processed.</p>
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
    /// <p>A list of account IDs that specify the accounts that invitations to Security Hub are declined from.</p>
    #[serde(rename = "AccountIds")]
    pub account_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeclineInvitationsResponse {
    /// <p>A list of account ID and email address pairs of the AWS accounts that weren't processed.</p>
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
    /// <p>A list of the account IDs that sent the invitations to delete.</p>
    #[serde(rename = "AccountIds")]
    pub account_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteInvitationsResponse {
    /// <p>A list of account ID and email address pairs of the AWS accounts that invitations weren't deleted for.</p>
    #[serde(rename = "UnprocessedAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<SecurityHubResult>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteMembersRequest {
    /// <p>A list of account IDs of the member accounts to delete.</p>
    #[serde(rename = "AccountIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteMembersResponse {
    /// <p>A list of account ID and email address pairs of the AWS accounts that weren't deleted.</p>
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
    /// <p>The token that is required for pagination.</p>
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
    /// <p>The token that is required for pagination.</p>
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
    /// <p>The ARN of the Hub resource retrieved.</p>
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
    /// <p>The token that is required for pagination.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeProductsResponse {
    /// <p>The token that is required for pagination.</p>
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
    /// <p>The maximum number of compliance standard controls to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>For requests to get the next page of results, the pagination token that was returned with the previous set of results. The initial request does not include a pagination token.</p>
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
    /// <p>A list of compliance standards controls.</p>
    #[serde(rename = "Controls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controls: Option<Vec<StandardsControl>>,
    /// <p>If there are more compliance standards control remaining in the results, then this is the pagination token to use to request the next page of compliance standard controls.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
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
    /// <p>The tags to add to the Hub resource when you enable Security Hub.</p>
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
    /// <p>Paginates results. On your first call to the <code>GetEnabledStandards</code> operation, set the value of this parameter to <code>NULL</code>. For subsequent calls to the operation, fill <code>nextToken</code> in the request with the value of <code>nextToken</code> from the previous response to continue listing data.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of the standards subscription ARNs for the standards to retrieve.</p>
    #[serde(rename = "StandardsSubscriptionArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_subscription_arns: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetEnabledStandardsResponse {
    /// <p>The token that is required for pagination.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of <code>StandardsSubscriptions</code> objects that include information about the enabled standards.</p>
    #[serde(rename = "StandardsSubscriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_subscriptions: Option<Vec<StandardsSubscription>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetFindingsRequest {
    /// <p>The findings attributes used to define a condition to filter the findings returned.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<AwsSecurityFindingFilters>,
    /// <p>The maximum number of findings to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Paginates results. On your first call to the <code>GetFindings</code> operation, set the value of this parameter to <code>NULL</code>. For subsequent calls to the operation, fill <code>nextToken</code> in the request with the value of <code>nextToken</code> from the previous response to continue listing data.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Findings attributes used to sort the list of findings returned.</p>
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
    /// <p>The token that is required for pagination.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetInsightResultsRequest {
    /// <p>The ARN of the insight whose results you want to see.</p>
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
    /// <p>The ARNs of the insights that you want to describe.</p>
    #[serde(rename = "InsightArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_arns: Option<Vec<String>>,
    /// <p>The maximum number of items that you want in the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Paginates results. On your first call to the <code>GetInsights</code> operation, set the value of this parameter to <code>NULL</code>. For subsequent calls to the operation, fill <code>nextToken</code> in the request with the value of <code>nextToken</code> from the previous response to continue listing data.</p>
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
    /// <p>The token that is required for pagination.</p>
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
    /// <p>The number of all membership invitations sent to this Security Hub member account, not including the currently accepted invitation. </p>
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
    /// <p>A list of account IDs for the Security Hub member accounts that you want to return the details for. </p>
    #[serde(rename = "AccountIds")]
    pub account_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetMembersResponse {
    /// <p>A list of details about the Security Hub member accounts.</p>
    #[serde(rename = "Members")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<Member>>,
    /// <p>A list of account ID and email address pairs of the AWS accounts that couldn't be processed.</p>
    #[serde(rename = "UnprocessedAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<SecurityHubResult>>,
}

/// <p>Includes details of the list of the findings that can't be imported.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImportFindingsError {
    /// <p>The code of the error made during the <code>BatchImportFindings</code> operation.</p>
    #[serde(rename = "ErrorCode")]
    pub error_code: String,
    /// <p>The message of the error made during the <code>BatchImportFindings</code> operation.</p>
    #[serde(rename = "ErrorMessage")]
    pub error_message: String,
    /// <p>The ID of the error made during the <code>BatchImportFindings</code> operation.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

/// <p>Contains information about a Security Hub insight.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Insight {
    /// <p>One or more attributes used to filter the findings included in the insight. Only findings that match the criteria defined in the filters are included in the insight.</p>
    #[serde(rename = "Filters")]
    pub filters: AwsSecurityFindingFilters,
    /// <p>The attribute that the insight's findings are grouped by. This attribute is used as a findings aggregator for the purposes of viewing and managing multiple related findings under a single operand.</p>
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
    /// <p>The current status of the association between member and master accounts.</p>
    #[serde(rename = "MemberStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InviteMembersRequest {
    /// <p>A list of IDs of the AWS accounts that you want to invite to Security Hub as members. </p>
    #[serde(rename = "AccountIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InviteMembersResponse {
    /// <p>A list of account ID and email address pairs of the AWS accounts that couldn't be processed. </p>
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
    /// <p>The maximum number of items that you want in the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Paginates results. On your first call to the <code>ListEnabledProductsForImport</code> operation, set the value of this parameter to <code>NULL</code>. For subsequent calls to the operation, fill <code>nextToken</code> in the request with the value of <code>NextToken</code> from the previous response to continue listing data.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListEnabledProductsForImportResponse {
    /// <p>The token that is required for pagination.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of ARNs for the resources that represent your subscriptions to products. </p>
    #[serde(rename = "ProductSubscriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_subscriptions: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListInvitationsRequest {
    /// <p>The maximum number of items that you want in the response. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Paginates results. On your first call to the <code>ListInvitations</code> operation, set the value of this parameter to <code>NULL</code>. For subsequent calls to the operation, fill <code>nextToken</code> in the request with the value of <code>NextToken</code> from the previous response to continue listing data. </p>
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
    /// <p>The token that is required for pagination.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListMembersRequest {
    /// <p>The maximum number of items that you want in the response. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Paginates results. Set the value of this parameter to <code>NULL</code> on your first call to the <code>ListMembers</code> operation. For subsequent calls to the operation, fill <code>nextToken</code> in the request with the value of <code>nextToken</code> from the previous response to continue listing data. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Specifies which member accounts the response includes based on their relationship status with the master account. The default value is <code>TRUE</code>. If <code>onlyAssociated</code> is set to <code>TRUE</code>, the response includes member accounts whose relationship status with the master is set to <code>ENABLED</code> or <code>DISABLED</code>. If <code>onlyAssociated</code> is set to <code>FALSE</code>, the response includes all existing member accounts. </p>
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
    /// <p>The token that is required for pagination.</p>
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
    /// <p>The state code. The initial state of the load balancer is provisioning. After the load balancer is fully set up and ready to route traffic, its state is active. If the load balancer could not be set up, its state is failed. </p>
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
    /// <p>The type of the resource that details are provided for.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

/// <p>Additional details about a resource related to a finding.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceDetails {
    /// <p>Details about a CloudFront distribution.</p>
    #[serde(rename = "AwsCloudFrontDistribution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_cloud_front_distribution: Option<AwsCloudFrontDistributionDetails>,
    /// <p>Details about an Amazon EC2 instance related to a finding.</p>
    #[serde(rename = "AwsEc2Instance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ec_2_instance: Option<AwsEc2InstanceDetails>,
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
    /// <p>Details about an Amazon S3 Bucket related to a finding.</p>
    #[serde(rename = "AwsS3Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_s3_bucket: Option<AwsS3BucketDetails>,
    /// <p>Details about an SNS topic.</p>
    #[serde(rename = "AwsSnsTopic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_sns_topic: Option<AwsSnsTopicDetails>,
    /// <p>Details about an SQS queue.</p>
    #[serde(rename = "AwsSqsQueue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_sqs_queue: Option<AwsSqsQueueDetails>,
    /// <p>Details about a container resource related to a finding.</p>
    #[serde(rename = "Container")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<ContainerDetails>,
    /// <p>Details about a resource that doesn't have a specific type defined.</p>
    #[serde(rename = "Other")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Details about the account that wasn't processed.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SecurityHubResult {
    /// <p>An AWS account ID of the account that wasn't be processed.</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The reason that the account wasn't be processed.</p>
    #[serde(rename = "ProcessingResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_result: Option<String>,
}

/// <p>The severity of the finding.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Severity {
    /// <p>The normalized severity of a finding.</p>
    #[serde(rename = "Normalized")]
    pub normalized: i64,
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

/// <p>Details for an individual compliance standard control.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StandardsControl {
    /// <p>The identifier of the compliance standard control.</p>
    #[serde(rename = "ControlId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_id: Option<String>,
    /// <p>The current status of the compliance standard control. Indicates whether the control is enabled or disabled. Security Hub does not check against disabled controls.</p>
    #[serde(rename = "ControlStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_status: Option<String>,
    /// <p>The date and time that the status of the compliance standard control was most recently updated.</p>
    #[serde(rename = "ControlStatusUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_status_updated_at: Option<f64>,
    /// <p>The longer description of the compliance standard control. Provides information about what the control is checking for.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The reason provided for the most recent change in status for the control.</p>
    #[serde(rename = "DisabledReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<String>,
    /// <p>A link to remediation information for the control in the Security Hub user documentation</p>
    #[serde(rename = "RemediationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediation_url: Option<String>,
    /// <p>The severity of findings generated from this compliance standard control.</p> <p>The finding severity is based on an assessment of how easy it would be to compromise AWS resources if the compliance issue is detected.</p>
    #[serde(rename = "SeverityRating")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_rating: Option<String>,
    /// <p>The ARN of the compliance standard control.</p>
    #[serde(rename = "StandardsControlArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_control_arn: Option<String>,
    /// <p>The title of the compliance standard control.</p>
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// <p>A resource that represents your subscription to a supported standard.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StandardsSubscription {
    /// <p>The ARN of a standard.</p> <p>In this release, Security Hub supports only the CIS AWS Foundations standard, which uses the following ARN: <code>arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0.</code> </p>
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
    /// <p><p>The ARN of the standard that you want to enable.</p> <important> <p>In this release, Security Hub only supports the CIS AWS Foundations standard. </p> <p>Its ARN is <code>arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0</code>.</p> </important></p>
    #[serde(rename = "StandardsArn")]
    pub standards_arn: String,
    /// <p>A key-value pair of input for the standard.</p>
    #[serde(rename = "StandardsInput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_input: Option<::std::collections::HashMap<String, String>>,
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

/// <p>Details about the threat intel related to a finding.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThreatIntelIndicator {
    /// <p>The category of a threat intel indicator.</p>
    #[serde(rename = "Category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// <p>The date and time when the most recent instance of a threat intel indicator was observed.</p>
    #[serde(rename = "LastObservedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_observed_at: Option<String>,
    /// <p>The source of the threat intel indicator.</p>
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// <p>The URL to the page or site where you can get more information about the threat intel indicator.</p>
    #[serde(rename = "SourceUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    /// <p>The type of a threat intel indicator.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The value of a threat intel indicator.</p>
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
    /// <p>The updated status of the compliance standard control.</p>
    #[serde(rename = "ControlStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_status: Option<String>,
    /// <p>A description of the reason why you are disabling a compliance standard control.</p>
    #[serde(rename = "DisabledReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<String>,
    /// <p>The ARN of the compliance standard control to enable or disable.</p>
    #[serde(rename = "StandardsControlArn")]
    pub standards_control_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateStandardsControlResponse {}

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
    /// <p>Accepts the invitation to be a member account and be monitored by the Security Hub master account that the invitation was sent from. When the member account accepts the invitation, permission is granted to the master account to view findings generated in the member account.</p>
    async fn accept_invitation(
        &self,
        input: AcceptInvitationRequest,
    ) -> Result<AcceptInvitationResponse, RusotoError<AcceptInvitationError>>;

    /// <p>Disables the standards specified by the provided <code>StandardsSubscriptionArns</code>. For more information, see <a href="https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-standards.html">Standards Supported in AWS Security Hub</a>.</p>
    async fn batch_disable_standards(
        &self,
        input: BatchDisableStandardsRequest,
    ) -> Result<BatchDisableStandardsResponse, RusotoError<BatchDisableStandardsError>>;

    /// <p>Enables the standards specified by the provided <code>standardsArn</code>. In this release, only CIS AWS Foundations standards are supported. For more information, see <a href="https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-standards.html">Standards Supported in AWS Security Hub</a>.</p>
    async fn batch_enable_standards(
        &self,
        input: BatchEnableStandardsRequest,
    ) -> Result<BatchEnableStandardsResponse, RusotoError<BatchEnableStandardsError>>;

    /// <p>Imports security findings generated from an integrated third-party product into Security Hub. This action is requested by the integrated product to import its findings into Security Hub. The maximum allowed size for a finding is 240 Kb. An error is returned for any finding larger than 240 Kb.</p>
    async fn batch_import_findings(
        &self,
        input: BatchImportFindingsRequest,
    ) -> Result<BatchImportFindingsResponse, RusotoError<BatchImportFindingsError>>;

    /// <p>Creates a custom action target in Security Hub. You can use custom actions on findings and insights in Security Hub to trigger target actions in Amazon CloudWatch Events.</p>
    async fn create_action_target(
        &self,
        input: CreateActionTargetRequest,
    ) -> Result<CreateActionTargetResponse, RusotoError<CreateActionTargetError>>;

    /// <p>Creates a custom insight in Security Hub. An insight is a consolidation of findings that relate to a security issue that requires attention or remediation. Use the <code>GroupByAttribute</code> to group the related findings in the insight.</p>
    async fn create_insight(
        &self,
        input: CreateInsightRequest,
    ) -> Result<CreateInsightResponse, RusotoError<CreateInsightError>>;

    /// <p>Creates a member association in Security Hub between the specified accounts and the account used to make the request, which is the master account. To successfully create a member, you must use this action from an account that already has Security Hub enabled. You can use the <a>EnableSecurityHub</a> to enable Security Hub.</p> <p>After you use <code>CreateMembers</code> to create member account associations in Security Hub, you need to use the <a>InviteMembers</a> action, which invites the accounts to enable Security Hub and become member accounts in Security Hub. If the invitation is accepted by the account owner, the account becomes a member account in Security Hub, and a permission policy is added that permits the master account to view the findings generated in the member account. When Security Hub is enabled in the invited account, findings start being sent to both the member and master accounts.</p> <p>You can remove the association between the master and member accounts by using the <a>DisassociateFromMasterAccount</a> or <a>DisassociateMembers</a> operation.</p>
    async fn create_members(
        &self,
        input: CreateMembersRequest,
    ) -> Result<CreateMembersResponse, RusotoError<CreateMembersError>>;

    /// <p>Declines invitations to become a member account.</p>
    async fn decline_invitations(
        &self,
        input: DeclineInvitationsRequest,
    ) -> Result<DeclineInvitationsResponse, RusotoError<DeclineInvitationsError>>;

    /// <p>Deletes a custom action target from Security Hub. Deleting a custom action target doesn't affect any findings or insights that were already sent to Amazon CloudWatch Events using the custom action.</p>
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

    /// <p>Returns information about the products available that you can subscribe to and integrate with Security Hub to consolidate findings.</p>
    async fn describe_products(
        &self,
        input: DescribeProductsRequest,
    ) -> Result<DescribeProductsResponse, RusotoError<DescribeProductsError>>;

    /// <p>Returns a list of compliance standards controls.</p> <p>For each control, the results include information about whether it is currently enabled, the severity, and a link to remediation information.</p>
    async fn describe_standards_controls(
        &self,
        input: DescribeStandardsControlsRequest,
    ) -> Result<DescribeStandardsControlsResponse, RusotoError<DescribeStandardsControlsError>>;

    /// <p>Disables the integration of the specified product with Security Hub. Findings from that product are no longer sent to Security Hub after the integration is disabled.</p>
    async fn disable_import_findings_for_product(
        &self,
        input: DisableImportFindingsForProductRequest,
    ) -> Result<
        DisableImportFindingsForProductResponse,
        RusotoError<DisableImportFindingsForProductError>,
    >;

    /// <p>Disables Security Hub in your account only in the current Region. To disable Security Hub in all Regions, you must submit one request per Region where you have enabled Security Hub. When you disable Security Hub for a master account, it doesn't disable Security Hub for any associated member accounts.</p> <p>When you disable Security Hub, your existing findings and insights and any Security Hub configuration settings are deleted after 90 days and can't be recovered. Any standards that were enabled are disabled, and your master and member account associations are removed. If you want to save your existing findings, you must export them before you disable Security Hub.</p>
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

    /// <p>Enables the integration of a partner product with Security Hub. Integrated products send findings to Security Hub. When you enable a product integration, a permission policy that grants permission for the product to send findings to Security Hub is applied.</p>
    async fn enable_import_findings_for_product(
        &self,
        input: EnableImportFindingsForProductRequest,
    ) -> Result<
        EnableImportFindingsForProductResponse,
        RusotoError<EnableImportFindingsForProductError>,
    >;

    /// <p>Enables Security Hub for your account in the current Region or the Region you specify in the request. Enabling Security Hub also enables the CIS AWS Foundations standard. When you enable Security Hub, you grant to Security Hub the permissions necessary to gather findings from AWS Config, Amazon GuardDuty, Amazon Inspector, and Amazon Macie. To learn more, see <a href="https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-settingup.html">Setting Up AWS Security Hub</a>.</p>
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

    /// <p>Lists the results of the Security Hub insight that the insight ARN specifies.</p>
    async fn get_insight_results(
        &self,
        input: GetInsightResultsRequest,
    ) -> Result<GetInsightResultsResponse, RusotoError<GetInsightResultsError>>;

    /// <p>Lists and describes insights that insight ARNs specify.</p>
    async fn get_insights(
        &self,
        input: GetInsightsRequest,
    ) -> Result<GetInsightsResponse, RusotoError<GetInsightsError>>;

    /// <p>Returns the count of all Security Hub membership invitations that were sent to the current member account, not including the currently accepted invitation. </p>
    async fn get_invitations_count(
        &self,
    ) -> Result<GetInvitationsCountResponse, RusotoError<GetInvitationsCountError>>;

    /// <p>Provides the details for the Security Hub master account to the current member account. </p>
    async fn get_master_account(
        &self,
    ) -> Result<GetMasterAccountResponse, RusotoError<GetMasterAccountError>>;

    /// <p>Returns the details on the Security Hub member accounts that the account IDs specify.</p>
    async fn get_members(
        &self,
        input: GetMembersRequest,
    ) -> Result<GetMembersResponse, RusotoError<GetMembersError>>;

    /// <p>Invites other AWS accounts to become member accounts for the Security Hub master account that the invitation is sent from. Before you can use this action to invite a member, you must first create the member account in Security Hub by using the <a>CreateMembers</a> action. When the account owner accepts the invitation to become a member account and enables Security Hub, the master account can view the findings generated from member account.</p>
    async fn invite_members(
        &self,
        input: InviteMembersRequest,
    ) -> Result<InviteMembersResponse, RusotoError<InviteMembersError>>;

    /// <p>Lists all findings-generating solutions (products) whose findings you have subscribed to receive in Security Hub.</p>
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

    /// <p>Updates the <code>Note</code> and <code>RecordState</code> of the Security Hub-aggregated findings that the filter attributes specify. Any member account that can view the finding also sees the update to the finding.</p>
    async fn update_findings(
        &self,
        input: UpdateFindingsRequest,
    ) -> Result<UpdateFindingsResponse, RusotoError<UpdateFindingsError>>;

    /// <p>Updates the Security Hub insight that the insight ARN specifies.</p>
    async fn update_insight(
        &self,
        input: UpdateInsightRequest,
    ) -> Result<UpdateInsightResponse, RusotoError<UpdateInsightError>>;

    /// <p>Used to control whether an individual compliance standard control is enabled or disabled.</p>
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
    /// <p>Accepts the invitation to be a member account and be monitored by the Security Hub master account that the invitation was sent from. When the member account accepts the invitation, permission is granted to the master account to view findings generated in the member account.</p>
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

    /// <p>Disables the standards specified by the provided <code>StandardsSubscriptionArns</code>. For more information, see <a href="https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-standards.html">Standards Supported in AWS Security Hub</a>.</p>
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

    /// <p>Enables the standards specified by the provided <code>standardsArn</code>. In this release, only CIS AWS Foundations standards are supported. For more information, see <a href="https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-standards.html">Standards Supported in AWS Security Hub</a>.</p>
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

    /// <p>Imports security findings generated from an integrated third-party product into Security Hub. This action is requested by the integrated product to import its findings into Security Hub. The maximum allowed size for a finding is 240 Kb. An error is returned for any finding larger than 240 Kb.</p>
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

    /// <p>Creates a custom action target in Security Hub. You can use custom actions on findings and insights in Security Hub to trigger target actions in Amazon CloudWatch Events.</p>
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

    /// <p>Creates a custom insight in Security Hub. An insight is a consolidation of findings that relate to a security issue that requires attention or remediation. Use the <code>GroupByAttribute</code> to group the related findings in the insight.</p>
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

    /// <p>Creates a member association in Security Hub between the specified accounts and the account used to make the request, which is the master account. To successfully create a member, you must use this action from an account that already has Security Hub enabled. You can use the <a>EnableSecurityHub</a> to enable Security Hub.</p> <p>After you use <code>CreateMembers</code> to create member account associations in Security Hub, you need to use the <a>InviteMembers</a> action, which invites the accounts to enable Security Hub and become member accounts in Security Hub. If the invitation is accepted by the account owner, the account becomes a member account in Security Hub, and a permission policy is added that permits the master account to view the findings generated in the member account. When Security Hub is enabled in the invited account, findings start being sent to both the member and master accounts.</p> <p>You can remove the association between the master and member accounts by using the <a>DisassociateFromMasterAccount</a> or <a>DisassociateMembers</a> operation.</p>
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

    /// <p>Deletes a custom action target from Security Hub. Deleting a custom action target doesn't affect any findings or insights that were already sent to Amazon CloudWatch Events using the custom action.</p>
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

    /// <p>Returns information about the products available that you can subscribe to and integrate with Security Hub to consolidate findings.</p>
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

    /// <p>Returns a list of compliance standards controls.</p> <p>For each control, the results include information about whether it is currently enabled, the severity, and a link to remediation information.</p>
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

    /// <p>Disables the integration of the specified product with Security Hub. Findings from that product are no longer sent to Security Hub after the integration is disabled.</p>
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

    /// <p>Disables Security Hub in your account only in the current Region. To disable Security Hub in all Regions, you must submit one request per Region where you have enabled Security Hub. When you disable Security Hub for a master account, it doesn't disable Security Hub for any associated member accounts.</p> <p>When you disable Security Hub, your existing findings and insights and any Security Hub configuration settings are deleted after 90 days and can't be recovered. Any standards that were enabled are disabled, and your master and member account associations are removed. If you want to save your existing findings, you must export them before you disable Security Hub.</p>
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

    /// <p>Enables the integration of a partner product with Security Hub. Integrated products send findings to Security Hub. When you enable a product integration, a permission policy that grants permission for the product to send findings to Security Hub is applied.</p>
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

    /// <p>Enables Security Hub for your account in the current Region or the Region you specify in the request. Enabling Security Hub also enables the CIS AWS Foundations standard. When you enable Security Hub, you grant to Security Hub the permissions necessary to gather findings from AWS Config, Amazon GuardDuty, Amazon Inspector, and Amazon Macie. To learn more, see <a href="https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-settingup.html">Setting Up AWS Security Hub</a>.</p>
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

    /// <p>Lists the results of the Security Hub insight that the insight ARN specifies.</p>
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

    /// <p>Lists and describes insights that insight ARNs specify.</p>
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

    /// <p>Provides the details for the Security Hub master account to the current member account. </p>
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

    /// <p>Returns the details on the Security Hub member accounts that the account IDs specify.</p>
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

    /// <p>Invites other AWS accounts to become member accounts for the Security Hub master account that the invitation is sent from. Before you can use this action to invite a member, you must first create the member account in Security Hub by using the <a>CreateMembers</a> action. When the account owner accepts the invitation to become a member account and enables Security Hub, the master account can view the findings generated from member account.</p>
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

    /// <p>Lists all findings-generating solutions (products) whose findings you have subscribed to receive in Security Hub.</p>
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

    /// <p>Updates the <code>Note</code> and <code>RecordState</code> of the Security Hub-aggregated findings that the filter attributes specify. Any member account that can view the finding also sees the update to the finding.</p>
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

    /// <p>Updates the Security Hub insight that the insight ARN specifies.</p>
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

    /// <p>Used to control whether an individual compliance standard control is enabled or disabled.</p>
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
