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
#![allow(warnings)]

use futures::future;
use futures::Future;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};
use std::error::Error;
use std::fmt;

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde_json;
/// <p>Name of the availability zone.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AvailabilityZone {
    /// <p>Id for the availability zone.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Types of broker engines.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BrokerEngineType {
    /// <p>The type of broker engine.</p>
    #[serde(rename = "EngineType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_type: Option<String>,
    /// <p>The list of engine versions.</p>
    #[serde(rename = "EngineVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_versions: Option<Vec<EngineVersion>>,
}

/// <p>Returns information about all brokers.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BrokerInstance {
    /// <p>The URL of the broker&#39;s ActiveMQ Web Console.</p>
    #[serde(rename = "ConsoleURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub console_url: Option<String>,
    /// <p>The broker&#39;s wire-level protocol endpoints.</p>
    #[serde(rename = "Endpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<String>>,
    /// <p>The IP address of the Elastic Network Interface (ENI) attached to the broker.</p>
    #[serde(rename = "IpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}

/// <p>Option for host instance type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BrokerInstanceOption {
    /// <p>The list of available az.</p>
    #[serde(rename = "AvailabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<AvailabilityZone>>,
    /// <p>The type of broker engine.</p>
    #[serde(rename = "EngineType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_type: Option<String>,
    /// <p>The type of broker instance.</p>
    #[serde(rename = "HostInstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_instance_type: Option<String>,
    /// <p>The list of supported engine versions.</p>
    #[serde(rename = "SupportedEngineVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_engine_versions: Option<Vec<String>>,
}

/// <p>The Amazon Resource Name (ARN) of the broker.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BrokerSummary {
    /// <p>The Amazon Resource Name (ARN) of the broker.</p>
    #[serde(rename = "BrokerArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_arn: Option<String>,
    /// <p>The unique ID that Amazon MQ generates for the broker.</p>
    #[serde(rename = "BrokerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_id: Option<String>,
    /// <p>The name of the broker. This value must be unique in your AWS account, 1-50 characters long, must contain only letters, numbers, dashes, and underscores, and must not contain whitespaces, brackets, wildcard characters, or special characters.</p>
    #[serde(rename = "BrokerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_name: Option<String>,
    /// <p>The status of the broker.</p>
    #[serde(rename = "BrokerState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_state: Option<String>,
    /// <p>The time when the broker was created.</p>
    #[serde(rename = "Created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    /// <p>Required. The deployment mode of the broker.</p>
    #[serde(rename = "DeploymentMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_mode: Option<String>,
    /// <p>The broker&#39;s instance type.</p>
    #[serde(rename = "HostInstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_instance_type: Option<String>,
}

/// <p>Returns information about all configurations.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Configuration {
    /// <p>Required. The ARN of the configuration.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Required. The date and time of the configuration revision.</p>
    #[serde(rename = "Created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    /// <p>Required. The description of the configuration.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Required. The type of broker engine. Note: Currently, Amazon MQ supports only ACTIVEMQ.</p>
    #[serde(rename = "EngineType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_type: Option<String>,
    /// <p>Required. The version of the broker engine. For a list of supported engine versions, see https://docs.aws.amazon.com/amazon-mq/latest/developer-guide/broker-engine.html</p>
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// <p>Required. The unique ID that Amazon MQ generates for the configuration.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Required. The latest revision of the configuration.</p>
    #[serde(rename = "LatestRevision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_revision: Option<ConfigurationRevision>,
    /// <p>Required. The name of the configuration. This value can contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _ ~). This value must be 1-150 characters long.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The list of all tags associated with this configuration.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>A list of information about the configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationId {
    /// <p>Required. The unique ID that Amazon MQ generates for the configuration.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The revision number of the configuration.</p>
    #[serde(rename = "Revision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
}

/// <p>Returns information about the specified configuration revision.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConfigurationRevision {
    /// <p>Required. The date and time of the configuration revision.</p>
    #[serde(rename = "Created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    /// <p>The description of the configuration revision.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Required. The revision number of the configuration.</p>
    #[serde(rename = "Revision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
}

/// <p>Broker configuration information</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Configurations {
    /// <p>The current configuration of the broker.</p>
    #[serde(rename = "Current")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<ConfigurationId>,
    /// <p>The history of configurations applied to the broker.</p>
    #[serde(rename = "History")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history: Option<Vec<ConfigurationId>>,
    /// <p>The pending configuration of the broker.</p>
    #[serde(rename = "Pending")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<ConfigurationId>,
}

/// <p>Creates a broker using the specified properties.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateBrokerRequest {
    /// <p>Required. Enables automatic upgrades to new minor versions for brokers, as Apache releases the versions. The automatic upgrades occur during the maintenance window of the broker or after a manual broker reboot.</p>
    #[serde(rename = "AutoMinorVersionUpgrade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    /// <p>Required. The name of the broker. This value must be unique in your AWS account, 1-50 characters long, must contain only letters, numbers, dashes, and underscores, and must not contain whitespaces, brackets, wildcard characters, or special characters.</p>
    #[serde(rename = "BrokerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_name: Option<String>,
    /// <p>A list of information about the configuration.</p>
    #[serde(rename = "Configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ConfigurationId>,
    /// <p>The unique ID that the requester receives for the created broker. Amazon MQ passes your ID with the API action. Note: We recommend using a Universally Unique Identifier (UUID) for the creatorRequestId. You may omit the creatorRequestId if your application doesn&#39;t require idempotency.</p>
    #[serde(rename = "CreatorRequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    /// <p>Required. The deployment mode of the broker.</p>
    #[serde(rename = "DeploymentMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_mode: Option<String>,
    /// <p>Encryption options for the broker.</p>
    #[serde(rename = "EncryptionOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_options: Option<EncryptionOptions>,
    /// <p>Required. The type of broker engine. Note: Currently, Amazon MQ supports only ACTIVEMQ.</p>
    #[serde(rename = "EngineType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_type: Option<String>,
    /// <p>Required. The version of the broker engine. For a list of supported engine versions, see https://docs.aws.amazon.com/amazon-mq/latest/developer-guide/broker-engine.html</p>
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// <p>Required. The broker&#39;s instance type.</p>
    #[serde(rename = "HostInstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_instance_type: Option<String>,
    /// <p>Enables Amazon CloudWatch logging for brokers.</p>
    #[serde(rename = "Logs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<Logs>,
    /// <p>The parameters that determine the WeeklyStartTime.</p>
    #[serde(rename = "MaintenanceWindowStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_window_start_time: Option<WeeklyStartTime>,
    /// <p>Required. Enables connections from applications outside of the VPC that hosts the broker&#39;s subnets.</p>
    #[serde(rename = "PubliclyAccessible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    /// <p>The list of security groups (1 minimum, 5 maximum) that authorize connections to brokers.</p>
    #[serde(rename = "SecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    /// <p>The list of groups (2 maximum) that define which subnets and IP ranges the broker can use from different Availability Zones. A SINGLE<em>INSTANCE deployment requires one subnet (for example, the default subnet). An ACTIVE</em>STANDBY<em>MULTI</em>AZ deployment requires two subnets.</p>
    #[serde(rename = "SubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    /// <p>Create tags when creating the broker.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>Required. The list of ActiveMQ users (persons or applications) who can access queues and topics. This value can contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _ ~). This value must be 2-100 characters long.</p>
    #[serde(rename = "Users")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<User>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateBrokerResponse {
    /// <p>The Amazon Resource Name (ARN) of the broker.</p>
    #[serde(rename = "BrokerArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_arn: Option<String>,
    /// <p>The unique ID that Amazon MQ generates for the broker.</p>
    #[serde(rename = "BrokerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_id: Option<String>,
}

/// <p>Creates a new configuration for the specified configuration name. Amazon MQ uses the default configuration (the engine type and version).</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateConfigurationRequest {
    /// <p>Required. The type of broker engine. Note: Currently, Amazon MQ supports only ACTIVEMQ.</p>
    #[serde(rename = "EngineType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_type: Option<String>,
    /// <p>Required. The version of the broker engine. For a list of supported engine versions, see https://docs.aws.amazon.com/amazon-mq/latest/developer-guide/broker-engine.html</p>
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// <p>Required. The name of the configuration. This value can contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _ ~). This value must be 1-150 characters long.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Create tags when creating the configuration.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateConfigurationResponse {
    /// <p>Required. The Amazon Resource Name (ARN) of the configuration.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Required. The date and time of the configuration.</p>
    #[serde(rename = "Created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    /// <p>Required. The unique ID that Amazon MQ generates for the configuration.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The latest revision of the configuration.</p>
    #[serde(rename = "LatestRevision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_revision: Option<ConfigurationRevision>,
    /// <p>Required. The name of the configuration. This value can contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _ ~). This value must be 1-150 characters long.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>A map of the key-value pairs for the resource tag.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateTagsRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource tag.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The key-value pair for the resource tag.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Creates a new ActiveMQ user.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateUserRequest {
    /// <p>The unique ID that Amazon MQ generates for the broker.</p>
    #[serde(rename = "BrokerId")]
    pub broker_id: String,
    /// <p>Enables access to the the ActiveMQ Web Console for the ActiveMQ user.</p>
    #[serde(rename = "ConsoleAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub console_access: Option<bool>,
    /// <p>The list of groups (20 maximum) to which the ActiveMQ user belongs. This value can contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _ ~). This value must be 2-100 characters long.</p>
    #[serde(rename = "Groups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    /// <p>Required. The password of the user. This value must be at least 12 characters long, must contain at least 4 unique characters, and must not contain commas.</p>
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// <p>The username of the ActiveMQ user. This value can contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _ ~). This value must be 2-100 characters long.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateUserResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteBrokerRequest {
    /// <p>The name of the broker. This value must be unique in your AWS account, 1-50 characters long, must contain only letters, numbers, dashes, and underscores, and must not contain whitespaces, brackets, wildcard characters, or special characters.</p>
    #[serde(rename = "BrokerId")]
    pub broker_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteBrokerResponse {
    /// <p>The unique ID that Amazon MQ generates for the broker.</p>
    #[serde(rename = "BrokerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteTagsRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource tag.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>An array of tag keys to delete</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteUserRequest {
    /// <p>The unique ID that Amazon MQ generates for the broker.</p>
    #[serde(rename = "BrokerId")]
    pub broker_id: String,
    /// <p>The username of the ActiveMQ user. This value can contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _ ~). This value must be 2-100 characters long.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteUserResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeBrokerEngineTypesRequest {
    /// <p>Filter response by engine type.</p>
    #[serde(rename = "EngineType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_type: Option<String>,
    /// <p>The maximum number of engine types that Amazon MQ can return per page (20 by default). This value must be an integer from 5 to 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that specifies the next page of results Amazon MQ should return. To request the first page, leave nextToken empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeBrokerEngineTypesResponse {
    /// <p>List of available engine types and versions.</p>
    #[serde(rename = "BrokerEngineTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_engine_types: Option<Vec<BrokerEngineType>>,
    /// <p>Required. The maximum number of engine types that can be returned per page (20 by default). This value must be an integer from 5 to 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that specifies the next page of results Amazon MQ should return. To request the first page, leave nextToken empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeBrokerInstanceOptionsRequest {
    /// <p>Filter response by engine type.</p>
    #[serde(rename = "EngineType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_type: Option<String>,
    /// <p>Filter response by host instance type.</p>
    #[serde(rename = "HostInstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_instance_type: Option<String>,
    /// <p>The maximum number of instance options that Amazon MQ can return per page (20 by default). This value must be an integer from 5 to 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that specifies the next page of results Amazon MQ should return. To request the first page, leave nextToken empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeBrokerInstanceOptionsResponse {
    /// <p>List of available broker instance options.</p>
    #[serde(rename = "BrokerInstanceOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_instance_options: Option<Vec<BrokerInstanceOption>>,
    /// <p>Required. The maximum number of instance options that can be returned per page (20 by default). This value must be an integer from 5 to 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that specifies the next page of results Amazon MQ should return. To request the first page, leave nextToken empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeBrokerRequest {
    /// <p>The name of the broker. This value must be unique in your AWS account, 1-50 characters long, must contain only letters, numbers, dashes, and underscores, and must not contain whitespaces, brackets, wildcard characters, or special characters.</p>
    #[serde(rename = "BrokerId")]
    pub broker_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeBrokerResponse {
    /// <p>Required. Enables automatic upgrades to new minor versions for brokers, as Apache releases the versions. The automatic upgrades occur during the maintenance window of the broker or after a manual broker reboot.</p>
    #[serde(rename = "AutoMinorVersionUpgrade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) of the broker.</p>
    #[serde(rename = "BrokerArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_arn: Option<String>,
    /// <p>The unique ID that Amazon MQ generates for the broker.</p>
    #[serde(rename = "BrokerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_id: Option<String>,
    /// <p>A list of information about allocated brokers.</p>
    #[serde(rename = "BrokerInstances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_instances: Option<Vec<BrokerInstance>>,
    /// <p>The name of the broker. This value must be unique in your AWS account, 1-50 characters long, must contain only letters, numbers, dashes, and underscores, and must not contain whitespaces, brackets, wildcard characters, or special characters.</p>
    #[serde(rename = "BrokerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_name: Option<String>,
    /// <p>The status of the broker.</p>
    #[serde(rename = "BrokerState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_state: Option<String>,
    /// <p>The list of all revisions for the specified configuration.</p>
    #[serde(rename = "Configurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations: Option<Configurations>,
    /// <p>The time when the broker was created.</p>
    #[serde(rename = "Created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    /// <p>Required. The deployment mode of the broker.</p>
    #[serde(rename = "DeploymentMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_mode: Option<String>,
    /// <p>Encryption options for the broker.</p>
    #[serde(rename = "EncryptionOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_options: Option<EncryptionOptions>,
    /// <p>Required. The type of broker engine. Note: Currently, Amazon MQ supports only ACTIVEMQ.</p>
    #[serde(rename = "EngineType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_type: Option<String>,
    /// <p>The version of the broker engine. For a list of supported engine versions, see https://docs.aws.amazon.com/amazon-mq/latest/developer-guide/broker-engine.html</p>
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// <p>The broker&#39;s instance type.</p>
    #[serde(rename = "HostInstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_instance_type: Option<String>,
    /// <p>The list of information about logs currently enabled and pending to be deployed for the specified broker.</p>
    #[serde(rename = "Logs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<LogsSummary>,
    /// <p>The parameters that determine the WeeklyStartTime.</p>
    #[serde(rename = "MaintenanceWindowStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_window_start_time: Option<WeeklyStartTime>,
    /// <p>The version of the broker engine to upgrade to. For a list of supported engine versions, see https://docs.aws.amazon.com/amazon-mq/latest/developer-guide/broker-engine.html</p>
    #[serde(rename = "PendingEngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_engine_version: Option<String>,
    /// <p>The host instance type of the broker to upgrade to. For a list of supported instance types, see https://docs.aws.amazon.com/amazon-mq/latest/developer-guide//broker.html#broker-instance-types</p>
    #[serde(rename = "PendingHostInstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_host_instance_type: Option<String>,
    /// <p>The list of pending security groups to authorize connections to brokers.</p>
    #[serde(rename = "PendingSecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_security_groups: Option<Vec<String>>,
    /// <p>Required. Enables connections from applications outside of the VPC that hosts the broker&#39;s subnets.</p>
    #[serde(rename = "PubliclyAccessible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    /// <p>The list of security groups (1 minimum, 5 maximum) that authorize connections to brokers.</p>
    #[serde(rename = "SecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    /// <p>The list of groups (2 maximum) that define which subnets and IP ranges the broker can use from different Availability Zones. A SINGLE<em>INSTANCE deployment requires one subnet (for example, the default subnet). An ACTIVE</em>STANDBY<em>MULTI</em>AZ deployment requires two subnets.</p>
    #[serde(rename = "SubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    /// <p>The list of all tags associated with this broker.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The list of all ActiveMQ usernames for the specified broker.</p>
    #[serde(rename = "Users")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<UserSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeConfigurationRequest {
    /// <p>The unique ID that Amazon MQ generates for the configuration.</p>
    #[serde(rename = "ConfigurationId")]
    pub configuration_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeConfigurationResponse {
    /// <p>Required. The ARN of the configuration.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Required. The date and time of the configuration revision.</p>
    #[serde(rename = "Created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    /// <p>Required. The description of the configuration.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Required. The type of broker engine. Note: Currently, Amazon MQ supports only ACTIVEMQ.</p>
    #[serde(rename = "EngineType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_type: Option<String>,
    /// <p>Required. The version of the broker engine. For a list of supported engine versions, see https://docs.aws.amazon.com/amazon-mq/latest/developer-guide/broker-engine.html</p>
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// <p>Required. The unique ID that Amazon MQ generates for the configuration.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Required. The latest revision of the configuration.</p>
    #[serde(rename = "LatestRevision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_revision: Option<ConfigurationRevision>,
    /// <p>Required. The name of the configuration. This value can contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _ ~). This value must be 1-150 characters long.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The list of all tags associated with this configuration.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeConfigurationRevisionRequest {
    /// <p>The unique ID that Amazon MQ generates for the configuration.</p>
    #[serde(rename = "ConfigurationId")]
    pub configuration_id: String,
    /// <p>The revision of the configuration.</p>
    #[serde(rename = "ConfigurationRevision")]
    pub configuration_revision: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeConfigurationRevisionResponse {
    /// <p>Required. The unique ID that Amazon MQ generates for the configuration.</p>
    #[serde(rename = "ConfigurationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_id: Option<String>,
    /// <p>Required. The date and time of the configuration.</p>
    #[serde(rename = "Created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    /// <p>Required. The base64-encoded XML configuration.</p>
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// <p>The description of the configuration.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeUserRequest {
    /// <p>The unique ID that Amazon MQ generates for the broker.</p>
    #[serde(rename = "BrokerId")]
    pub broker_id: String,
    /// <p>The username of the ActiveMQ user. This value can contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _ ~). This value must be 2-100 characters long.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeUserResponse {
    /// <p>Required. The unique ID that Amazon MQ generates for the broker.</p>
    #[serde(rename = "BrokerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_id: Option<String>,
    /// <p>Enables access to the the ActiveMQ Web Console for the ActiveMQ user.</p>
    #[serde(rename = "ConsoleAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub console_access: Option<bool>,
    /// <p>The list of groups (20 maximum) to which the ActiveMQ user belongs. This value can contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _ ~). This value must be 2-100 characters long.</p>
    #[serde(rename = "Groups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    /// <p>The status of the changes pending for the ActiveMQ user.</p>
    #[serde(rename = "Pending")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<UserPendingChanges>,
    /// <p>Required. The username of the ActiveMQ user. This value can contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _ ~). This value must be 2-100 characters long.</p>
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// <p>Encryption options for the broker.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EncryptionOptions {
    /// <p>The customer master key (CMK) to use for the AWS Key Management Service (KMS). This key is used to encrypt your data at rest. If not provided, Amazon MQ will use a default CMK to encrypt your data.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>Enables the use of an AWS owned CMK using AWS Key Management Service (KMS).</p>
    #[serde(rename = "UseAwsOwnedKey")]
    pub use_aws_owned_key: bool,
}

/// <p>Id of the engine version.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EngineVersion {
    /// <p>Id for the version.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListBrokersRequest {
    /// <p>The maximum number of brokers that Amazon MQ can return per page (20 by default). This value must be an integer from 5 to 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that specifies the next page of results Amazon MQ should return. To request the first page, leave nextToken empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListBrokersResponse {
    /// <p>A list of information about all brokers.</p>
    #[serde(rename = "BrokerSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_summaries: Option<Vec<BrokerSummary>>,
    /// <p>The token that specifies the next page of results Amazon MQ should return. To request the first page, leave nextToken empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListConfigurationRevisionsRequest {
    /// <p>The unique ID that Amazon MQ generates for the configuration.</p>
    #[serde(rename = "ConfigurationId")]
    pub configuration_id: String,
    /// <p>The maximum number of configurations that Amazon MQ can return per page (20 by default). This value must be an integer from 5 to 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that specifies the next page of results Amazon MQ should return. To request the first page, leave nextToken empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListConfigurationRevisionsResponse {
    /// <p>The unique ID that Amazon MQ generates for the configuration.</p>
    #[serde(rename = "ConfigurationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_id: Option<String>,
    /// <p>The maximum number of configuration revisions that can be returned per page (20 by default). This value must be an integer from 5 to 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that specifies the next page of results Amazon MQ should return. To request the first page, leave nextToken empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of all revisions for the specified configuration.</p>
    #[serde(rename = "Revisions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revisions: Option<Vec<ConfigurationRevision>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListConfigurationsRequest {
    /// <p>The maximum number of configurations that Amazon MQ can return per page (20 by default). This value must be an integer from 5 to 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that specifies the next page of results Amazon MQ should return. To request the first page, leave nextToken empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListConfigurationsResponse {
    /// <p>The list of all revisions for the specified configuration.</p>
    #[serde(rename = "Configurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations: Option<Vec<Configuration>>,
    /// <p>The maximum number of configurations that Amazon MQ can return per page (20 by default). This value must be an integer from 5 to 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that specifies the next page of results Amazon MQ should return. To request the first page, leave nextToken empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTagsRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource tag.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsResponse {
    /// <p>The key-value pair for the resource tag.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListUsersRequest {
    /// <p>The unique ID that Amazon MQ generates for the broker.</p>
    #[serde(rename = "BrokerId")]
    pub broker_id: String,
    /// <p>The maximum number of ActiveMQ users that can be returned per page (20 by default). This value must be an integer from 5 to 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that specifies the next page of results Amazon MQ should return. To request the first page, leave nextToken empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListUsersResponse {
    /// <p>Required. The unique ID that Amazon MQ generates for the broker.</p>
    #[serde(rename = "BrokerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_id: Option<String>,
    /// <p>Required. The maximum number of ActiveMQ users that can be returned per page (20 by default). This value must be an integer from 5 to 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that specifies the next page of results Amazon MQ should return. To request the first page, leave nextToken empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Required. The list of all ActiveMQ usernames for the specified broker.</p>
    #[serde(rename = "Users")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<UserSummary>>,
}

/// <p>The list of information about logs to be enabled for the specified broker.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Logs {
    /// <p>Enables audit logging. Every user management action made using JMX or the ActiveMQ Web Console is logged.</p>
    #[serde(rename = "Audit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit: Option<bool>,
    /// <p>Enables general logging.</p>
    #[serde(rename = "General")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general: Option<bool>,
}

/// <p>The list of information about logs currently enabled and pending to be deployed for the specified broker.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LogsSummary {
    /// <p>Enables audit logging. Every user management action made using JMX or the ActiveMQ Web Console is logged.</p>
    #[serde(rename = "Audit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit: Option<bool>,
    /// <p>The location of the CloudWatch Logs log group where audit logs are sent.</p>
    #[serde(rename = "AuditLogGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_log_group: Option<String>,
    /// <p>Enables general logging.</p>
    #[serde(rename = "General")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general: Option<bool>,
    /// <p>The location of the CloudWatch Logs log group where general logs are sent.</p>
    #[serde(rename = "GeneralLogGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general_log_group: Option<String>,
    /// <p>The list of information about logs pending to be deployed for the specified broker.</p>
    #[serde(rename = "Pending")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<PendingLogs>,
}

/// <p>The list of information about logs to be enabled for the specified broker.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PendingLogs {
    /// <p>Enables audit logging. Every user management action made using JMX or the ActiveMQ Web Console is logged.</p>
    #[serde(rename = "Audit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit: Option<bool>,
    /// <p>Enables general logging.</p>
    #[serde(rename = "General")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RebootBrokerRequest {
    /// <p>The unique ID that Amazon MQ generates for the broker.</p>
    #[serde(rename = "BrokerId")]
    pub broker_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RebootBrokerResponse {}

/// <p>Returns information about the XML element or attribute that was sanitized in the configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SanitizationWarning {
    /// <p>The name of the XML attribute that has been sanitized.</p>
    #[serde(rename = "AttributeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    /// <p>The name of the XML element that has been sanitized.</p>
    #[serde(rename = "ElementName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_name: Option<String>,
    /// <p>Required. The reason for which the XML elements or attributes were sanitized.</p>
    #[serde(rename = "Reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// <p>Updates the broker using the specified properties.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateBrokerRequest {
    /// <p>Enables automatic upgrades to new minor versions for brokers, as Apache releases the versions. The automatic upgrades occur during the maintenance window of the broker or after a manual broker reboot.</p>
    #[serde(rename = "AutoMinorVersionUpgrade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    /// <p>The name of the broker. This value must be unique in your AWS account, 1-50 characters long, must contain only letters, numbers, dashes, and underscores, and must not contain whitespaces, brackets, wildcard characters, or special characters.</p>
    #[serde(rename = "BrokerId")]
    pub broker_id: String,
    /// <p>A list of information about the configuration.</p>
    #[serde(rename = "Configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ConfigurationId>,
    /// <p>The version of the broker engine. For a list of supported engine versions, see https://docs.aws.amazon.com/amazon-mq/latest/developer-guide/broker-engine.html</p>
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// <p>The host instance type of the broker to upgrade to. For a list of supported instance types, see https://docs.aws.amazon.com/amazon-mq/latest/developer-guide//broker.html#broker-instance-types</p>
    #[serde(rename = "HostInstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_instance_type: Option<String>,
    /// <p>Enables Amazon CloudWatch logging for brokers.</p>
    #[serde(rename = "Logs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<Logs>,
    /// <p>The list of security groups (1 minimum, 5 maximum) that authorize connections to brokers.</p>
    #[serde(rename = "SecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateBrokerResponse {
    /// <p>The new value of automatic upgrades to new minor version for brokers.</p>
    #[serde(rename = "AutoMinorVersionUpgrade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    /// <p>Required. The unique ID that Amazon MQ generates for the broker.</p>
    #[serde(rename = "BrokerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_id: Option<String>,
    /// <p>The ID of the updated configuration.</p>
    #[serde(rename = "Configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ConfigurationId>,
    /// <p>The version of the broker engine to upgrade to. For a list of supported engine versions, see https://docs.aws.amazon.com/amazon-mq/latest/developer-guide/broker-engine.html</p>
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// <p>The host instance type of the broker to upgrade to. For a list of supported instance types, see https://docs.aws.amazon.com/amazon-mq/latest/developer-guide//broker.html#broker-instance-types</p>
    #[serde(rename = "HostInstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_instance_type: Option<String>,
    /// <p>The list of information about logs to be enabled for the specified broker.</p>
    #[serde(rename = "Logs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<Logs>,
    /// <p>The list of security groups (1 minimum, 5 maximum) that authorize connections to brokers.</p>
    #[serde(rename = "SecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
}

/// <p>Updates the specified configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateConfigurationRequest {
    /// <p>The unique ID that Amazon MQ generates for the configuration.</p>
    #[serde(rename = "ConfigurationId")]
    pub configuration_id: String,
    /// <p>Required. The base64-encoded XML configuration.</p>
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// <p>The description of the configuration.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateConfigurationResponse {
    /// <p>Required. The Amazon Resource Name (ARN) of the configuration.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Required. The date and time of the configuration.</p>
    #[serde(rename = "Created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    /// <p>Required. The unique ID that Amazon MQ generates for the configuration.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The latest revision of the configuration.</p>
    #[serde(rename = "LatestRevision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_revision: Option<ConfigurationRevision>,
    /// <p>Required. The name of the configuration. This value can contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _ ~). This value must be 1-150 characters long.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The list of the first 20 warnings about the configuration XML elements or attributes that were sanitized.</p>
    #[serde(rename = "Warnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<SanitizationWarning>>,
}

/// <p>Updates the information for an ActiveMQ user.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateUserRequest {
    /// <p>The unique ID that Amazon MQ generates for the broker.</p>
    #[serde(rename = "BrokerId")]
    pub broker_id: String,
    /// <p>Enables access to the the ActiveMQ Web Console for the ActiveMQ user.</p>
    #[serde(rename = "ConsoleAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub console_access: Option<bool>,
    /// <p>The list of groups (20 maximum) to which the ActiveMQ user belongs. This value can contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _ ~). This value must be 2-100 characters long.</p>
    #[serde(rename = "Groups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    /// <p>The password of the user. This value must be at least 12 characters long, must contain at least 4 unique characters, and must not contain commas.</p>
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// <p>Required. The username of the ActiveMQ user. This value can contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _ ~). This value must be 2-100 characters long.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateUserResponse {}

/// <p>An ActiveMQ user associated with the broker.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct User {
    /// <p>Enables access to the the ActiveMQ Web Console for the ActiveMQ user.</p>
    #[serde(rename = "ConsoleAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub console_access: Option<bool>,
    /// <p>The list of groups (20 maximum) to which the ActiveMQ user belongs. This value can contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _ ~). This value must be 2-100 characters long.</p>
    #[serde(rename = "Groups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    /// <p>Required. The password of the ActiveMQ user. This value must be at least 12 characters long, must contain at least 4 unique characters, and must not contain commas.</p>
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// <p>Required. The username of the ActiveMQ user. This value can contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _ ~). This value must be 2-100 characters long.</p>
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// <p>Returns information about the status of the changes pending for the ActiveMQ user.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UserPendingChanges {
    /// <p>Enables access to the the ActiveMQ Web Console for the ActiveMQ user.</p>
    #[serde(rename = "ConsoleAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub console_access: Option<bool>,
    /// <p>The list of groups (20 maximum) to which the ActiveMQ user belongs. This value can contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _ ~). This value must be 2-100 characters long.</p>
    #[serde(rename = "Groups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    /// <p>Required. The type of change pending for the ActiveMQ user.</p>
    #[serde(rename = "PendingChange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_change: Option<String>,
}

/// <p>Returns a list of all ActiveMQ users.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UserSummary {
    /// <p>The type of change pending for the ActiveMQ user.</p>
    #[serde(rename = "PendingChange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_change: Option<String>,
    /// <p>Required. The username of the ActiveMQ user. This value can contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _ ~). This value must be 2-100 characters long.</p>
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// <p>The scheduled time period relative to UTC during which Amazon MQ begins to apply pending updates or patches to the broker.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeeklyStartTime {
    /// <p>Required. The day of the week.</p>
    #[serde(rename = "DayOfWeek")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<String>,
    /// <p>Required. The time, in 24-hour format.</p>
    #[serde(rename = "TimeOfDay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_of_day: Option<String>,
    /// <p>The time zone, UTC by default, in either the Country/City format, or the UTC offset format.</p>
    #[serde(rename = "TimeZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

/// Errors returned by CreateBroker
#[derive(Debug, PartialEq)]
pub enum CreateBrokerError {
    /// <p>Returns information about an error.</p>
    BadRequest(String),
    /// <p>Returns information about an error.</p>
    Conflict(String),
    /// <p>Returns information about an error.</p>
    Forbidden(String),
    /// <p>Returns information about an error.</p>
    InternalServerError(String),
    /// <p>Returns information about an error.</p>
    Unauthorized(String),
}

impl CreateBrokerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateBrokerError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateBrokerError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateBrokerError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateBrokerError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateBrokerError::InternalServerError(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreateBrokerError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateBrokerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateBrokerError {
    fn description(&self) -> &str {
        match *self {
            CreateBrokerError::BadRequest(ref cause) => cause,
            CreateBrokerError::Conflict(ref cause) => cause,
            CreateBrokerError::Forbidden(ref cause) => cause,
            CreateBrokerError::InternalServerError(ref cause) => cause,
            CreateBrokerError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateConfiguration
#[derive(Debug, PartialEq)]
pub enum CreateConfigurationError {
    /// <p>Returns information about an error.</p>
    BadRequest(String),
    /// <p>Returns information about an error.</p>
    Conflict(String),
    /// <p>Returns information about an error.</p>
    Forbidden(String),
    /// <p>Returns information about an error.</p>
    InternalServerError(String),
}

impl CreateConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateConfigurationError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateConfigurationError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateConfigurationError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateConfigurationError::InternalServerError(
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
impl fmt::Display for CreateConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateConfigurationError {
    fn description(&self) -> &str {
        match *self {
            CreateConfigurationError::BadRequest(ref cause) => cause,
            CreateConfigurationError::Conflict(ref cause) => cause,
            CreateConfigurationError::Forbidden(ref cause) => cause,
            CreateConfigurationError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateTags
#[derive(Debug, PartialEq)]
pub enum CreateTagsError {
    /// <p>Returns information about an error.</p>
    BadRequest(String),
    /// <p>Returns information about an error.</p>
    Forbidden(String),
    /// <p>Returns information about an error.</p>
    InternalServerError(String),
    /// <p>Returns information about an error.</p>
    NotFound(String),
}

impl CreateTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateTagsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateTagsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateTagsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateTagsError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateTagsError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateTagsError {
    fn description(&self) -> &str {
        match *self {
            CreateTagsError::BadRequest(ref cause) => cause,
            CreateTagsError::Forbidden(ref cause) => cause,
            CreateTagsError::InternalServerError(ref cause) => cause,
            CreateTagsError::NotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateUser
#[derive(Debug, PartialEq)]
pub enum CreateUserError {
    /// <p>Returns information about an error.</p>
    BadRequest(String),
    /// <p>Returns information about an error.</p>
    Conflict(String),
    /// <p>Returns information about an error.</p>
    Forbidden(String),
    /// <p>Returns information about an error.</p>
    InternalServerError(String),
    /// <p>Returns information about an error.</p>
    NotFound(String),
}

impl CreateUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateUserError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateUserError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateUserError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateUserError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateUserError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateUserError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateUserError {
    fn description(&self) -> &str {
        match *self {
            CreateUserError::BadRequest(ref cause) => cause,
            CreateUserError::Conflict(ref cause) => cause,
            CreateUserError::Forbidden(ref cause) => cause,
            CreateUserError::InternalServerError(ref cause) => cause,
            CreateUserError::NotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteBroker
#[derive(Debug, PartialEq)]
pub enum DeleteBrokerError {
    /// <p>Returns information about an error.</p>
    BadRequest(String),
    /// <p>Returns information about an error.</p>
    Forbidden(String),
    /// <p>Returns information about an error.</p>
    InternalServerError(String),
    /// <p>Returns information about an error.</p>
    NotFound(String),
}

impl DeleteBrokerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteBrokerError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteBrokerError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteBrokerError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteBrokerError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteBrokerError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteBrokerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBrokerError {
    fn description(&self) -> &str {
        match *self {
            DeleteBrokerError::BadRequest(ref cause) => cause,
            DeleteBrokerError::Forbidden(ref cause) => cause,
            DeleteBrokerError::InternalServerError(ref cause) => cause,
            DeleteBrokerError::NotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteTags
#[derive(Debug, PartialEq)]
pub enum DeleteTagsError {
    /// <p>Returns information about an error.</p>
    BadRequest(String),
    /// <p>Returns information about an error.</p>
    Forbidden(String),
    /// <p>Returns information about an error.</p>
    InternalServerError(String),
    /// <p>Returns information about an error.</p>
    NotFound(String),
}

impl DeleteTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteTagsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteTagsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteTagsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteTagsError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteTagsError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteTagsError {
    fn description(&self) -> &str {
        match *self {
            DeleteTagsError::BadRequest(ref cause) => cause,
            DeleteTagsError::Forbidden(ref cause) => cause,
            DeleteTagsError::InternalServerError(ref cause) => cause,
            DeleteTagsError::NotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteUser
#[derive(Debug, PartialEq)]
pub enum DeleteUserError {
    /// <p>Returns information about an error.</p>
    BadRequest(String),
    /// <p>Returns information about an error.</p>
    Forbidden(String),
    /// <p>Returns information about an error.</p>
    InternalServerError(String),
    /// <p>Returns information about an error.</p>
    NotFound(String),
}

impl DeleteUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteUserError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteUserError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteUserError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteUserError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteUserError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteUserError {
    fn description(&self) -> &str {
        match *self {
            DeleteUserError::BadRequest(ref cause) => cause,
            DeleteUserError::Forbidden(ref cause) => cause,
            DeleteUserError::InternalServerError(ref cause) => cause,
            DeleteUserError::NotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeBroker
#[derive(Debug, PartialEq)]
pub enum DescribeBrokerError {
    /// <p>Returns information about an error.</p>
    BadRequest(String),
    /// <p>Returns information about an error.</p>
    Forbidden(String),
    /// <p>Returns information about an error.</p>
    InternalServerError(String),
    /// <p>Returns information about an error.</p>
    NotFound(String),
}

impl DescribeBrokerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeBrokerError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DescribeBrokerError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DescribeBrokerError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DescribeBrokerError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeBrokerError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeBrokerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeBrokerError {
    fn description(&self) -> &str {
        match *self {
            DescribeBrokerError::BadRequest(ref cause) => cause,
            DescribeBrokerError::Forbidden(ref cause) => cause,
            DescribeBrokerError::InternalServerError(ref cause) => cause,
            DescribeBrokerError::NotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeBrokerEngineTypes
#[derive(Debug, PartialEq)]
pub enum DescribeBrokerEngineTypesError {
    /// <p>Returns information about an error.</p>
    BadRequest(String),
    /// <p>Returns information about an error.</p>
    Forbidden(String),
    /// <p>Returns information about an error.</p>
    InternalServerError(String),
}

impl DescribeBrokerEngineTypesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeBrokerEngineTypesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DescribeBrokerEngineTypesError::BadRequest(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DescribeBrokerEngineTypesError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        DescribeBrokerEngineTypesError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeBrokerEngineTypesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeBrokerEngineTypesError {
    fn description(&self) -> &str {
        match *self {
            DescribeBrokerEngineTypesError::BadRequest(ref cause) => cause,
            DescribeBrokerEngineTypesError::Forbidden(ref cause) => cause,
            DescribeBrokerEngineTypesError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeBrokerInstanceOptions
#[derive(Debug, PartialEq)]
pub enum DescribeBrokerInstanceOptionsError {
    /// <p>Returns information about an error.</p>
    BadRequest(String),
    /// <p>Returns information about an error.</p>
    Forbidden(String),
    /// <p>Returns information about an error.</p>
    InternalServerError(String),
}

impl DescribeBrokerInstanceOptionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeBrokerInstanceOptionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DescribeBrokerInstanceOptionsError::BadRequest(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DescribeBrokerInstanceOptionsError::Forbidden(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        DescribeBrokerInstanceOptionsError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeBrokerInstanceOptionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeBrokerInstanceOptionsError {
    fn description(&self) -> &str {
        match *self {
            DescribeBrokerInstanceOptionsError::BadRequest(ref cause) => cause,
            DescribeBrokerInstanceOptionsError::Forbidden(ref cause) => cause,
            DescribeBrokerInstanceOptionsError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeConfiguration
#[derive(Debug, PartialEq)]
pub enum DescribeConfigurationError {
    /// <p>Returns information about an error.</p>
    BadRequest(String),
    /// <p>Returns information about an error.</p>
    Forbidden(String),
    /// <p>Returns information about an error.</p>
    InternalServerError(String),
    /// <p>Returns information about an error.</p>
    NotFound(String),
}

impl DescribeConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DescribeConfigurationError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DescribeConfigurationError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DescribeConfigurationError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeConfigurationError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeConfigurationError {
    fn description(&self) -> &str {
        match *self {
            DescribeConfigurationError::BadRequest(ref cause) => cause,
            DescribeConfigurationError::Forbidden(ref cause) => cause,
            DescribeConfigurationError::InternalServerError(ref cause) => cause,
            DescribeConfigurationError::NotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeConfigurationRevision
#[derive(Debug, PartialEq)]
pub enum DescribeConfigurationRevisionError {
    /// <p>Returns information about an error.</p>
    BadRequest(String),
    /// <p>Returns information about an error.</p>
    Forbidden(String),
    /// <p>Returns information about an error.</p>
    InternalServerError(String),
    /// <p>Returns information about an error.</p>
    NotFound(String),
}

impl DescribeConfigurationRevisionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeConfigurationRevisionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DescribeConfigurationRevisionError::BadRequest(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DescribeConfigurationRevisionError::Forbidden(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        DescribeConfigurationRevisionError::InternalServerError(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeConfigurationRevisionError::NotFound(
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
impl fmt::Display for DescribeConfigurationRevisionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeConfigurationRevisionError {
    fn description(&self) -> &str {
        match *self {
            DescribeConfigurationRevisionError::BadRequest(ref cause) => cause,
            DescribeConfigurationRevisionError::Forbidden(ref cause) => cause,
            DescribeConfigurationRevisionError::InternalServerError(ref cause) => cause,
            DescribeConfigurationRevisionError::NotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeUser
#[derive(Debug, PartialEq)]
pub enum DescribeUserError {
    /// <p>Returns information about an error.</p>
    BadRequest(String),
    /// <p>Returns information about an error.</p>
    Forbidden(String),
    /// <p>Returns information about an error.</p>
    InternalServerError(String),
    /// <p>Returns information about an error.</p>
    NotFound(String),
}

impl DescribeUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeUserError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DescribeUserError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DescribeUserError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DescribeUserError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeUserError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeUserError {
    fn description(&self) -> &str {
        match *self {
            DescribeUserError::BadRequest(ref cause) => cause,
            DescribeUserError::Forbidden(ref cause) => cause,
            DescribeUserError::InternalServerError(ref cause) => cause,
            DescribeUserError::NotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by ListBrokers
#[derive(Debug, PartialEq)]
pub enum ListBrokersError {
    /// <p>Returns information about an error.</p>
    BadRequest(String),
    /// <p>Returns information about an error.</p>
    Forbidden(String),
    /// <p>Returns information about an error.</p>
    InternalServerError(String),
}

impl ListBrokersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListBrokersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListBrokersError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListBrokersError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListBrokersError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListBrokersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListBrokersError {
    fn description(&self) -> &str {
        match *self {
            ListBrokersError::BadRequest(ref cause) => cause,
            ListBrokersError::Forbidden(ref cause) => cause,
            ListBrokersError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by ListConfigurationRevisions
#[derive(Debug, PartialEq)]
pub enum ListConfigurationRevisionsError {
    /// <p>Returns information about an error.</p>
    BadRequest(String),
    /// <p>Returns information about an error.</p>
    Forbidden(String),
    /// <p>Returns information about an error.</p>
    InternalServerError(String),
    /// <p>Returns information about an error.</p>
    NotFound(String),
}

impl ListConfigurationRevisionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListConfigurationRevisionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListConfigurationRevisionsError::BadRequest(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListConfigurationRevisionsError::Forbidden(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        ListConfigurationRevisionsError::InternalServerError(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListConfigurationRevisionsError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListConfigurationRevisionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListConfigurationRevisionsError {
    fn description(&self) -> &str {
        match *self {
            ListConfigurationRevisionsError::BadRequest(ref cause) => cause,
            ListConfigurationRevisionsError::Forbidden(ref cause) => cause,
            ListConfigurationRevisionsError::InternalServerError(ref cause) => cause,
            ListConfigurationRevisionsError::NotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by ListConfigurations
#[derive(Debug, PartialEq)]
pub enum ListConfigurationsError {
    /// <p>Returns information about an error.</p>
    BadRequest(String),
    /// <p>Returns information about an error.</p>
    Forbidden(String),
    /// <p>Returns information about an error.</p>
    InternalServerError(String),
}

impl ListConfigurationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListConfigurationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListConfigurationsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListConfigurationsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListConfigurationsError::InternalServerError(
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
impl fmt::Display for ListConfigurationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListConfigurationsError {
    fn description(&self) -> &str {
        match *self {
            ListConfigurationsError::BadRequest(ref cause) => cause,
            ListConfigurationsError::Forbidden(ref cause) => cause,
            ListConfigurationsError::InternalServerError(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTags
#[derive(Debug, PartialEq)]
pub enum ListTagsError {
    /// <p>Returns information about an error.</p>
    BadRequest(String),
    /// <p>Returns information about an error.</p>
    Forbidden(String),
    /// <p>Returns information about an error.</p>
    InternalServerError(String),
    /// <p>Returns information about an error.</p>
    NotFound(String),
}

impl ListTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListTagsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListTagsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListTagsError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListTagsError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTagsError {
    fn description(&self) -> &str {
        match *self {
            ListTagsError::BadRequest(ref cause) => cause,
            ListTagsError::Forbidden(ref cause) => cause,
            ListTagsError::InternalServerError(ref cause) => cause,
            ListTagsError::NotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by ListUsers
#[derive(Debug, PartialEq)]
pub enum ListUsersError {
    /// <p>Returns information about an error.</p>
    BadRequest(String),
    /// <p>Returns information about an error.</p>
    Forbidden(String),
    /// <p>Returns information about an error.</p>
    InternalServerError(String),
    /// <p>Returns information about an error.</p>
    NotFound(String),
}

impl ListUsersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListUsersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListUsersError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListUsersError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListUsersError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListUsersError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListUsersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListUsersError {
    fn description(&self) -> &str {
        match *self {
            ListUsersError::BadRequest(ref cause) => cause,
            ListUsersError::Forbidden(ref cause) => cause,
            ListUsersError::InternalServerError(ref cause) => cause,
            ListUsersError::NotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by RebootBroker
#[derive(Debug, PartialEq)]
pub enum RebootBrokerError {
    /// <p>Returns information about an error.</p>
    BadRequest(String),
    /// <p>Returns information about an error.</p>
    Forbidden(String),
    /// <p>Returns information about an error.</p>
    InternalServerError(String),
    /// <p>Returns information about an error.</p>
    NotFound(String),
}

impl RebootBrokerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RebootBrokerError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(RebootBrokerError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(RebootBrokerError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(RebootBrokerError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(RebootBrokerError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RebootBrokerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RebootBrokerError {
    fn description(&self) -> &str {
        match *self {
            RebootBrokerError::BadRequest(ref cause) => cause,
            RebootBrokerError::Forbidden(ref cause) => cause,
            RebootBrokerError::InternalServerError(ref cause) => cause,
            RebootBrokerError::NotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateBroker
#[derive(Debug, PartialEq)]
pub enum UpdateBrokerError {
    /// <p>Returns information about an error.</p>
    BadRequest(String),
    /// <p>Returns information about an error.</p>
    Conflict(String),
    /// <p>Returns information about an error.</p>
    Forbidden(String),
    /// <p>Returns information about an error.</p>
    InternalServerError(String),
    /// <p>Returns information about an error.</p>
    NotFound(String),
}

impl UpdateBrokerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateBrokerError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateBrokerError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateBrokerError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateBrokerError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateBrokerError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateBrokerError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateBrokerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateBrokerError {
    fn description(&self) -> &str {
        match *self {
            UpdateBrokerError::BadRequest(ref cause) => cause,
            UpdateBrokerError::Conflict(ref cause) => cause,
            UpdateBrokerError::Forbidden(ref cause) => cause,
            UpdateBrokerError::InternalServerError(ref cause) => cause,
            UpdateBrokerError::NotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateConfiguration
#[derive(Debug, PartialEq)]
pub enum UpdateConfigurationError {
    /// <p>Returns information about an error.</p>
    BadRequest(String),
    /// <p>Returns information about an error.</p>
    Conflict(String),
    /// <p>Returns information about an error.</p>
    Forbidden(String),
    /// <p>Returns information about an error.</p>
    InternalServerError(String),
    /// <p>Returns information about an error.</p>
    NotFound(String),
}

impl UpdateConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateConfigurationError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateConfigurationError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateConfigurationError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateConfigurationError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateConfigurationError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateConfigurationError {
    fn description(&self) -> &str {
        match *self {
            UpdateConfigurationError::BadRequest(ref cause) => cause,
            UpdateConfigurationError::Conflict(ref cause) => cause,
            UpdateConfigurationError::Forbidden(ref cause) => cause,
            UpdateConfigurationError::InternalServerError(ref cause) => cause,
            UpdateConfigurationError::NotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateUser
#[derive(Debug, PartialEq)]
pub enum UpdateUserError {
    /// <p>Returns information about an error.</p>
    BadRequest(String),
    /// <p>Returns information about an error.</p>
    Conflict(String),
    /// <p>Returns information about an error.</p>
    Forbidden(String),
    /// <p>Returns information about an error.</p>
    InternalServerError(String),
    /// <p>Returns information about an error.</p>
    NotFound(String),
}

impl UpdateUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateUserError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateUserError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateUserError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateUserError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateUserError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateUserError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateUserError {
    fn description(&self) -> &str {
        match *self {
            UpdateUserError::BadRequest(ref cause) => cause,
            UpdateUserError::Conflict(ref cause) => cause,
            UpdateUserError::Forbidden(ref cause) => cause,
            UpdateUserError::InternalServerError(ref cause) => cause,
            UpdateUserError::NotFound(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AmazonMQ API. AmazonMQ clients implement this trait.
pub trait MQ {
    /// <p>Creates a broker. Note: This API is asynchronous.</p>
    fn create_broker(
        &self,
        input: CreateBrokerRequest,
    ) -> RusotoFuture<CreateBrokerResponse, CreateBrokerError>;

    /// <p>Creates a new configuration for the specified configuration name. Amazon MQ uses the default configuration (the engine type and version).</p>
    fn create_configuration(
        &self,
        input: CreateConfigurationRequest,
    ) -> RusotoFuture<CreateConfigurationResponse, CreateConfigurationError>;

    /// <p>Add a tag to a resource.</p>
    fn create_tags(&self, input: CreateTagsRequest) -> RusotoFuture<(), CreateTagsError>;

    /// <p>Creates an ActiveMQ user.</p>
    fn create_user(
        &self,
        input: CreateUserRequest,
    ) -> RusotoFuture<CreateUserResponse, CreateUserError>;

    /// <p>Deletes a broker. Note: This API is asynchronous.</p>
    fn delete_broker(
        &self,
        input: DeleteBrokerRequest,
    ) -> RusotoFuture<DeleteBrokerResponse, DeleteBrokerError>;

    /// <p>Removes a tag from a resource.</p>
    fn delete_tags(&self, input: DeleteTagsRequest) -> RusotoFuture<(), DeleteTagsError>;

    /// <p>Deletes an ActiveMQ user.</p>
    fn delete_user(
        &self,
        input: DeleteUserRequest,
    ) -> RusotoFuture<DeleteUserResponse, DeleteUserError>;

    /// <p>Returns information about the specified broker.</p>
    fn describe_broker(
        &self,
        input: DescribeBrokerRequest,
    ) -> RusotoFuture<DescribeBrokerResponse, DescribeBrokerError>;

    /// <p>Describe available engine types and versions.</p>
    fn describe_broker_engine_types(
        &self,
        input: DescribeBrokerEngineTypesRequest,
    ) -> RusotoFuture<DescribeBrokerEngineTypesResponse, DescribeBrokerEngineTypesError>;

    /// <p>Describe available broker instance options.</p>
    fn describe_broker_instance_options(
        &self,
        input: DescribeBrokerInstanceOptionsRequest,
    ) -> RusotoFuture<DescribeBrokerInstanceOptionsResponse, DescribeBrokerInstanceOptionsError>;

    /// <p>Returns information about the specified configuration.</p>
    fn describe_configuration(
        &self,
        input: DescribeConfigurationRequest,
    ) -> RusotoFuture<DescribeConfigurationResponse, DescribeConfigurationError>;

    /// <p>Returns the specified configuration revision for the specified configuration.</p>
    fn describe_configuration_revision(
        &self,
        input: DescribeConfigurationRevisionRequest,
    ) -> RusotoFuture<DescribeConfigurationRevisionResponse, DescribeConfigurationRevisionError>;

    /// <p>Returns information about an ActiveMQ user.</p>
    fn describe_user(
        &self,
        input: DescribeUserRequest,
    ) -> RusotoFuture<DescribeUserResponse, DescribeUserError>;

    /// <p>Returns a list of all brokers.</p>
    fn list_brokers(
        &self,
        input: ListBrokersRequest,
    ) -> RusotoFuture<ListBrokersResponse, ListBrokersError>;

    /// <p>Returns a list of all revisions for the specified configuration.</p>
    fn list_configuration_revisions(
        &self,
        input: ListConfigurationRevisionsRequest,
    ) -> RusotoFuture<ListConfigurationRevisionsResponse, ListConfigurationRevisionsError>;

    /// <p>Returns a list of all configurations.</p>
    fn list_configurations(
        &self,
        input: ListConfigurationsRequest,
    ) -> RusotoFuture<ListConfigurationsResponse, ListConfigurationsError>;

    /// <p>Lists tags for a resource.</p>
    fn list_tags(&self, input: ListTagsRequest) -> RusotoFuture<ListTagsResponse, ListTagsError>;

    /// <p>Returns a list of all ActiveMQ users.</p>
    fn list_users(
        &self,
        input: ListUsersRequest,
    ) -> RusotoFuture<ListUsersResponse, ListUsersError>;

    /// <p>Reboots a broker. Note: This API is asynchronous.</p>
    fn reboot_broker(
        &self,
        input: RebootBrokerRequest,
    ) -> RusotoFuture<RebootBrokerResponse, RebootBrokerError>;

    /// <p>Adds a pending configuration change to a broker.</p>
    fn update_broker(
        &self,
        input: UpdateBrokerRequest,
    ) -> RusotoFuture<UpdateBrokerResponse, UpdateBrokerError>;

    /// <p>Updates the specified configuration.</p>
    fn update_configuration(
        &self,
        input: UpdateConfigurationRequest,
    ) -> RusotoFuture<UpdateConfigurationResponse, UpdateConfigurationError>;

    /// <p>Updates the information for an ActiveMQ user.</p>
    fn update_user(
        &self,
        input: UpdateUserRequest,
    ) -> RusotoFuture<UpdateUserResponse, UpdateUserError>;
}
/// A client for the AmazonMQ API.
#[derive(Clone)]
pub struct MQClient {
    client: Client,
    region: region::Region,
}

impl MQClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> MQClient {
        Self::new_with_client(Client::shared(), region)
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> MQClient
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

    pub fn new_with_client(client: Client, region: region::Region) -> MQClient {
        MQClient { client, region }
    }
}

impl fmt::Debug for MQClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MQClient")
            .field("region", &self.region)
            .finish()
    }
}

impl MQ for MQClient {
    /// <p>Creates a broker. Note: This API is asynchronous.</p>
    fn create_broker(
        &self,
        input: CreateBrokerRequest,
    ) -> RusotoFuture<CreateBrokerResponse, CreateBrokerError> {
        let request_uri = "/v1/brokers";

        let mut request = SignedRequest::new("POST", "mq", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateBrokerResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateBrokerError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a new configuration for the specified configuration name. Amazon MQ uses the default configuration (the engine type and version).</p>
    fn create_configuration(
        &self,
        input: CreateConfigurationRequest,
    ) -> RusotoFuture<CreateConfigurationResponse, CreateConfigurationError> {
        let request_uri = "/v1/configurations";

        let mut request = SignedRequest::new("POST", "mq", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateConfigurationResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateConfigurationError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Add a tag to a resource.</p>
    fn create_tags(&self, input: CreateTagsRequest) -> RusotoFuture<(), CreateTagsError> {
        let request_uri = format!("/v1/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("POST", "mq", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateTagsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates an ActiveMQ user.</p>
    fn create_user(
        &self,
        input: CreateUserRequest,
    ) -> RusotoFuture<CreateUserResponse, CreateUserError> {
        let request_uri = format!(
            "/v1/brokers/{broker_id}/users/{username}",
            broker_id = input.broker_id,
            username = input.username
        );

        let mut request = SignedRequest::new("POST", "mq", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateUserResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateUserError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a broker. Note: This API is asynchronous.</p>
    fn delete_broker(
        &self,
        input: DeleteBrokerRequest,
    ) -> RusotoFuture<DeleteBrokerResponse, DeleteBrokerError> {
        let request_uri = format!("/v1/brokers/{broker_id}", broker_id = input.broker_id);

        let mut request = SignedRequest::new("DELETE", "mq", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteBrokerResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteBrokerError::from_response(response))),
                )
            }
        })
    }

    /// <p>Removes a tag from a resource.</p>
    fn delete_tags(&self, input: DeleteTagsRequest) -> RusotoFuture<(), DeleteTagsError> {
        let request_uri = format!("/v1/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("DELETE", "mq", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        for item in input.tag_keys.iter() {
            params.put("tagKeys", item);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteTagsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes an ActiveMQ user.</p>
    fn delete_user(
        &self,
        input: DeleteUserRequest,
    ) -> RusotoFuture<DeleteUserResponse, DeleteUserError> {
        let request_uri = format!(
            "/v1/brokers/{broker_id}/users/{username}",
            broker_id = input.broker_id,
            username = input.username
        );

        let mut request = SignedRequest::new("DELETE", "mq", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteUserResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteUserError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about the specified broker.</p>
    fn describe_broker(
        &self,
        input: DescribeBrokerRequest,
    ) -> RusotoFuture<DescribeBrokerResponse, DescribeBrokerError> {
        let request_uri = format!("/v1/brokers/{broker_id}", broker_id = input.broker_id);

        let mut request = SignedRequest::new("GET", "mq", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeBrokerResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeBrokerError::from_response(response))),
                )
            }
        })
    }

    /// <p>Describe available engine types and versions.</p>
    fn describe_broker_engine_types(
        &self,
        input: DescribeBrokerEngineTypesRequest,
    ) -> RusotoFuture<DescribeBrokerEngineTypesResponse, DescribeBrokerEngineTypesError> {
        let request_uri = "/v1/broker-engine-types";

        let mut request = SignedRequest::new("GET", "mq", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.engine_type {
            params.put("engineType", x);
        }
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeBrokerEngineTypesResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeBrokerEngineTypesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Describe available broker instance options.</p>
    fn describe_broker_instance_options(
        &self,
        input: DescribeBrokerInstanceOptionsRequest,
    ) -> RusotoFuture<DescribeBrokerInstanceOptionsResponse, DescribeBrokerInstanceOptionsError>
    {
        let request_uri = "/v1/broker-instance-options";

        let mut request = SignedRequest::new("GET", "mq", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.engine_type {
            params.put("engineType", x);
        }
        if let Some(ref x) = input.host_instance_type {
            params.put("hostInstanceType", x);
        }
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeBrokerInstanceOptionsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeBrokerInstanceOptionsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns information about the specified configuration.</p>
    fn describe_configuration(
        &self,
        input: DescribeConfigurationRequest,
    ) -> RusotoFuture<DescribeConfigurationResponse, DescribeConfigurationError> {
        let request_uri = format!(
            "/v1/configurations/{configuration_id}",
            configuration_id = input.configuration_id
        );

        let mut request = SignedRequest::new("GET", "mq", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeConfigurationResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeConfigurationError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns the specified configuration revision for the specified configuration.</p>
    fn describe_configuration_revision(
        &self,
        input: DescribeConfigurationRevisionRequest,
    ) -> RusotoFuture<DescribeConfigurationRevisionResponse, DescribeConfigurationRevisionError>
    {
        let request_uri = format!(
            "/v1/configurations/{configuration_id}/revisions/{configuration_revision}",
            configuration_id = input.configuration_id,
            configuration_revision = input.configuration_revision
        );

        let mut request = SignedRequest::new("GET", "mq", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeConfigurationRevisionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeConfigurationRevisionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns information about an ActiveMQ user.</p>
    fn describe_user(
        &self,
        input: DescribeUserRequest,
    ) -> RusotoFuture<DescribeUserResponse, DescribeUserError> {
        let request_uri = format!(
            "/v1/brokers/{broker_id}/users/{username}",
            broker_id = input.broker_id,
            username = input.username
        );

        let mut request = SignedRequest::new("GET", "mq", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeUserResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeUserError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns a list of all brokers.</p>
    fn list_brokers(
        &self,
        input: ListBrokersRequest,
    ) -> RusotoFuture<ListBrokersResponse, ListBrokersError> {
        let request_uri = "/v1/brokers";

        let mut request = SignedRequest::new("GET", "mq", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListBrokersResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListBrokersError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns a list of all revisions for the specified configuration.</p>
    fn list_configuration_revisions(
        &self,
        input: ListConfigurationRevisionsRequest,
    ) -> RusotoFuture<ListConfigurationRevisionsResponse, ListConfigurationRevisionsError> {
        let request_uri = format!(
            "/v1/configurations/{configuration_id}/revisions",
            configuration_id = input.configuration_id
        );

        let mut request = SignedRequest::new("GET", "mq", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListConfigurationRevisionsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListConfigurationRevisionsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns a list of all configurations.</p>
    fn list_configurations(
        &self,
        input: ListConfigurationsRequest,
    ) -> RusotoFuture<ListConfigurationsResponse, ListConfigurationsError> {
        let request_uri = "/v1/configurations";

        let mut request = SignedRequest::new("GET", "mq", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListConfigurationsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListConfigurationsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists tags for a resource.</p>
    fn list_tags(&self, input: ListTagsRequest) -> RusotoFuture<ListTagsResponse, ListTagsError> {
        let request_uri = format!("/v1/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("GET", "mq", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListTagsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListTagsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns a list of all ActiveMQ users.</p>
    fn list_users(
        &self,
        input: ListUsersRequest,
    ) -> RusotoFuture<ListUsersResponse, ListUsersError> {
        let request_uri = format!("/v1/brokers/{broker_id}/users", broker_id = input.broker_id);

        let mut request = SignedRequest::new("GET", "mq", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListUsersResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListUsersError::from_response(response))),
                )
            }
        })
    }

    /// <p>Reboots a broker. Note: This API is asynchronous.</p>
    fn reboot_broker(
        &self,
        input: RebootBrokerRequest,
    ) -> RusotoFuture<RebootBrokerResponse, RebootBrokerError> {
        let request_uri = format!(
            "/v1/brokers/{broker_id}/reboot",
            broker_id = input.broker_id
        );

        let mut request = SignedRequest::new("POST", "mq", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<RebootBrokerResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(RebootBrokerError::from_response(response))),
                )
            }
        })
    }

    /// <p>Adds a pending configuration change to a broker.</p>
    fn update_broker(
        &self,
        input: UpdateBrokerRequest,
    ) -> RusotoFuture<UpdateBrokerResponse, UpdateBrokerError> {
        let request_uri = format!("/v1/brokers/{broker_id}", broker_id = input.broker_id);

        let mut request = SignedRequest::new("PUT", "mq", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateBrokerResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateBrokerError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates the specified configuration.</p>
    fn update_configuration(
        &self,
        input: UpdateConfigurationRequest,
    ) -> RusotoFuture<UpdateConfigurationResponse, UpdateConfigurationError> {
        let request_uri = format!(
            "/v1/configurations/{configuration_id}",
            configuration_id = input.configuration_id
        );

        let mut request = SignedRequest::new("PUT", "mq", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateConfigurationResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateConfigurationError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Updates the information for an ActiveMQ user.</p>
    fn update_user(
        &self,
        input: UpdateUserRequest,
    ) -> RusotoFuture<UpdateUserResponse, UpdateUserError> {
        let request_uri = format!(
            "/v1/brokers/{broker_id}/users/{username}",
            broker_id = input.broker_id,
            username = input.username
        );

        let mut request = SignedRequest::new("PUT", "mq", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateUserResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateUserError::from_response(response))),
                )
            }
        })
    }
}
