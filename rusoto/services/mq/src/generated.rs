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
use rusoto_core::region;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::{Client, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_str;
use serde_json::Value as SerdeJsonValue;
/// <p>Returns information about all brokers.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BrokerInstance {
    /// <p>The URL of the broker&#39;s ActiveMQ Web Console.</p>
    #[serde(rename = "ConsoleURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub console_url: Option<String>,
    /// <p>The broker&#39;s wire-level protocol endpoints.</p>
    #[serde(rename = "Endpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<String>>,
    /// <p>The IP address of the ENI attached to the broker.</p>
    #[serde(rename = "IpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}

/// <p>The Amazon Resource Name (ARN) of the broker.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
    /// <p>Required. The version of the broker engine.</p>
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

/// <p>Required. The time period during which Amazon MQ applies pending updates or patches to the broker.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateBrokerInput {
    /// <p>Required. Enables automatic upgrades to new minor versions for brokers, as Apache releases the versions. The automatic upgrades occur during the maintenance window of the broker or after a manual broker reboot.</p>
    pub auto_minor_version_upgrade: Option<bool>,
    /// <p>Required. The name of the broker. This value must be unique in your AWS account, 1-50 characters long, must contain only letters, numbers, dashes, and underscores, and must not contain whitespaces, brackets, wildcard characters, or special characters.</p>
    pub broker_name: Option<String>,
    /// <p>A list of information about the configuration.</p>
    pub configuration: Option<ConfigurationId>,
    /// <p>The unique ID that the requester receives for the created broker. Amazon MQ passes your ID with the API action. Note: We recommend using a Universally Unique Identifier (UUID) for the creatorRequestId. You may omit the creatorRequestId if your application doesn&#39;t require idempotency.</p>
    pub creator_request_id: Option<String>,
    /// <p>Required. The deployment mode of the broker.</p>
    pub deployment_mode: Option<String>,
    /// <p>Required. The type of broker engine. Note: Currently, Amazon MQ supports only ACTIVEMQ.</p>
    pub engine_type: Option<String>,
    /// <p>Required. The version of the broker engine. Note: Currently, Amazon MQ supports only 5.15.0.</p>
    pub engine_version: Option<String>,
    /// <p>Required. The broker&#39;s instance type.</p>
    pub host_instance_type: Option<String>,
    /// <p>Enables Amazon CloudWatch logging for brokers.</p>
    pub logs: Option<Logs>,
    /// <p>The parameters that determine the WeeklyStartTime.</p>
    pub maintenance_window_start_time: Option<WeeklyStartTime>,
    /// <p>Required. Enables connections from applications outside of the VPC that hosts the broker&#39;s subnets.</p>
    pub publicly_accessible: Option<bool>,
    /// <p>The list of rules (1 minimum, 125 maximum) that authorize connections to brokers.</p>
    pub security_groups: Option<Vec<String>>,
    /// <p>The list of groups (2 maximum) that define which subnets and IP ranges the broker can use from different Availability Zones. A SINGLE<em>INSTANCE deployment requires one subnet (for example, the default subnet). An ACTIVE</em>STANDBY<em>MULTI</em>AZ deployment requires two subnets.</p>
    pub subnet_ids: Option<Vec<String>>,
    /// <p>Required. The list of ActiveMQ users (persons or applications) who can access queues and topics. This value can contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _ ~). This value must be 2-100 characters long.</p>
    pub users: Option<Vec<User>>,
}

/// <p>Returns information about the created broker.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateBrokerOutput {
    /// <p>The Amazon Resource Name (ARN) of the broker.</p>
    pub broker_arn: Option<String>,
    /// <p>The unique ID that Amazon MQ generates for the broker.</p>
    pub broker_id: Option<String>,
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
    /// <p>Required. The type of broker engine. Note: Currently, Amazon MQ supports only ACTIVEMQ.</p>
    #[serde(rename = "EngineType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_type: Option<String>,
    /// <p>Required. The version of the broker engine. Note: Currently, Amazon MQ supports only 5.15.0.</p>
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
    /// <p>The list of rules (1 minimum, 125 maximum) that authorize connections to brokers.</p>
    #[serde(rename = "SecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    /// <p>The list of groups (2 maximum) that define which subnets and IP ranges the broker can use from different Availability Zones. A SINGLE<em>INSTANCE deployment requires one subnet (for example, the default subnet). An ACTIVE</em>STANDBY<em>MULTI</em>AZ deployment requires two subnets.</p>
    #[serde(rename = "SubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    /// <p>Required. The list of ActiveMQ users (persons or applications) who can access queues and topics. This value can contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _ ~). This value must be 2-100 characters long.</p>
    #[serde(rename = "Users")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<User>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateConfigurationInput {
    /// <p>Required. The type of broker engine. Note: Currently, Amazon MQ supports only ACTIVEMQ.</p>
    pub engine_type: Option<String>,
    /// <p>Required. The version of the broker engine. Note: Currently, Amazon MQ supports only 5.15.0.</p>
    pub engine_version: Option<String>,
    /// <p>Required. The name of the configuration. This value can contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _ ~). This value must be 1-150 characters long.</p>
    pub name: Option<String>,
}

/// <p>Returns information about the created configuration.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateConfigurationOutput {
    /// <p>Required. The Amazon Resource Name (ARN) of the configuration.</p>
    pub arn: Option<String>,
    /// <p>Required. The date and time of the configuration.</p>
    pub created: Option<f64>,
    /// <p>Required. The unique ID that Amazon MQ generates for the configuration.</p>
    pub id: Option<String>,
    /// <p>The latest revision of the configuration.</p>
    pub latest_revision: Option<ConfigurationRevision>,
    /// <p>Required. The name of the configuration. This value can contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _ ~). This value must be 1-150 characters long.</p>
    pub name: Option<String>,
}

/// <p>Creates a new configuration for the specified configuration name. Amazon MQ uses the default configuration (the engine type and version).</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateConfigurationRequest {
    /// <p>Required. The type of broker engine. Note: Currently, Amazon MQ supports only ACTIVEMQ.</p>
    #[serde(rename = "EngineType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_type: Option<String>,
    /// <p>Required. The version of the broker engine. Note: Currently, Amazon MQ supports only 5.15.0.</p>
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// <p>Required. The name of the configuration. This value can contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _ ~). This value must be 1-150 characters long.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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

/// <p>Creates a new ActiveMQ user.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateUserInput {
    /// <p>Enables access to the the ActiveMQ Web Console for the ActiveMQ user.</p>
    pub console_access: Option<bool>,
    /// <p>The list of groups (20 maximum) to which the ActiveMQ user belongs. This value can contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _ ~). This value must be 2-100 characters long.</p>
    pub groups: Option<Vec<String>>,
    /// <p>Required. The password of the user. This value must be at least 12 characters long, must contain at least 4 unique characters, and must not contain commas.</p>
    pub password: Option<String>,
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
pub struct CreateUserResponse {}

/// <p>Returns information about the deleted broker.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteBrokerOutput {
    /// <p>The unique ID that Amazon MQ generates for the broker.</p>
    pub broker_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteBrokerRequest {
    /// <p>The name of the broker. This value must be unique in your AWS account, 1-50 characters long, must contain only letters, numbers, dashes, and underscores, and must not contain whitespaces, brackets, wildcard characters, or special characters.</p>
    #[serde(rename = "BrokerId")]
    pub broker_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteBrokerResponse {
    /// <p>The unique ID that Amazon MQ generates for the broker.</p>
    #[serde(rename = "BrokerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_id: Option<String>,
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
pub struct DeleteUserResponse {}

/// <p>The version of the broker engine. Note: Currently, Amazon MQ supports only 5.15.0.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeBrokerOutput {
    /// <p>Required. Enables automatic upgrades to new minor versions for brokers, as Apache releases the versions. The automatic upgrades occur during the maintenance window of the broker or after a manual broker reboot.</p>
    pub auto_minor_version_upgrade: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) of the broker.</p>
    pub broker_arn: Option<String>,
    /// <p>The unique ID that Amazon MQ generates for the broker.</p>
    pub broker_id: Option<String>,
    /// <p>A list of information about allocated brokers.</p>
    pub broker_instances: Option<Vec<BrokerInstance>>,
    /// <p>The name of the broker. This value must be unique in your AWS account, 1-50 characters long, must contain only letters, numbers, dashes, and underscores, and must not contain whitespaces, brackets, wildcard characters, or special characters.</p>
    pub broker_name: Option<String>,
    /// <p>The status of the broker.</p>
    pub broker_state: Option<String>,
    /// <p>The list of all revisions for the specified configuration.</p>
    pub configurations: Option<Configurations>,
    /// <p>The time when the broker was created.</p>
    pub created: Option<f64>,
    /// <p>Required. The deployment mode of the broker.</p>
    pub deployment_mode: Option<String>,
    /// <p>Required. The type of broker engine. Note: Currently, Amazon MQ supports only ACTIVEMQ.</p>
    pub engine_type: Option<String>,
    /// <p>The version of the broker engine. Note: Currently, Amazon MQ supports only 5.15.0.</p>
    pub engine_version: Option<String>,
    /// <p>The broker&#39;s instance type.</p>
    pub host_instance_type: Option<String>,
    /// <p>The list of information about logs currently enabled and pending to be deployed for the specified broker.</p>
    pub logs: Option<LogsSummary>,
    /// <p>The parameters that determine the WeeklyStartTime.</p>
    pub maintenance_window_start_time: Option<WeeklyStartTime>,
    /// <p>Required. Enables connections from applications outside of the VPC that hosts the broker&#39;s subnets.</p>
    pub publicly_accessible: Option<bool>,
    /// <p>Required. The list of rules (1 minimum, 125 maximum) that authorize connections to brokers.</p>
    pub security_groups: Option<Vec<String>>,
    /// <p>The list of groups (2 maximum) that define which subnets and IP ranges the broker can use from different Availability Zones. A SINGLE<em>INSTANCE deployment requires one subnet (for example, the default subnet). An ACTIVE</em>STANDBY<em>MULTI</em>AZ deployment requires two subnets.</p>
    pub subnet_ids: Option<Vec<String>>,
    /// <p>The list of all ActiveMQ usernames for the specified broker.</p>
    pub users: Option<Vec<UserSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeBrokerRequest {
    /// <p>The name of the broker. This value must be unique in your AWS account, 1-50 characters long, must contain only letters, numbers, dashes, and underscores, and must not contain whitespaces, brackets, wildcard characters, or special characters.</p>
    #[serde(rename = "BrokerId")]
    pub broker_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
    /// <p>Required. The type of broker engine. Note: Currently, Amazon MQ supports only ACTIVEMQ.</p>
    #[serde(rename = "EngineType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_type: Option<String>,
    /// <p>The version of the broker engine. Note: Currently, Amazon MQ supports only 5.15.0.</p>
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
    /// <p>Required. Enables connections from applications outside of the VPC that hosts the broker&#39;s subnets.</p>
    #[serde(rename = "PubliclyAccessible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    /// <p>Required. The list of rules (1 minimum, 125 maximum) that authorize connections to brokers.</p>
    #[serde(rename = "SecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    /// <p>The list of groups (2 maximum) that define which subnets and IP ranges the broker can use from different Availability Zones. A SINGLE<em>INSTANCE deployment requires one subnet (for example, the default subnet). An ACTIVE</em>STANDBY<em>MULTI</em>AZ deployment requires two subnets.</p>
    #[serde(rename = "SubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
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
    /// <p>Required. The version of the broker engine.</p>
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
}

/// <p>Returns the specified configuration revision for the specified configuration.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeConfigurationRevisionOutput {
    /// <p>Required. The unique ID that Amazon MQ generates for the configuration.</p>
    pub configuration_id: Option<String>,
    /// <p>Required. The date and time of the configuration.</p>
    pub created: Option<f64>,
    /// <p>Required. The base64-encoded XML configuration.</p>
    pub data: Option<String>,
    /// <p>The description of the configuration.</p>
    pub description: Option<String>,
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

/// <p>Returns information about an ActiveMQ user.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeUserOutput {
    /// <p>Required. The unique ID that Amazon MQ generates for the broker.</p>
    pub broker_id: Option<String>,
    /// <p>Enables access to the the ActiveMQ Web Console for the ActiveMQ user.</p>
    pub console_access: Option<bool>,
    /// <p>The list of groups (20 maximum) to which the ActiveMQ user belongs. This value can contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _ ~). This value must be 2-100 characters long.</p>
    pub groups: Option<Vec<String>>,
    /// <p>The status of the changes pending for the ActiveMQ user.</p>
    pub pending: Option<UserPendingChanges>,
    /// <p>Required. The username of the ActiveMQ user. This value can contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _ ~). This value must be 2-100 characters long.</p>
    pub username: Option<String>,
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

/// <p>Returns information about an error.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MQError {
    /// <p>The attribute which caused the error.</p>
    pub error_attribute: Option<String>,
    /// <p>The explanation of the error.</p>
    pub message: Option<String>,
}

/// <p>A list of information about all brokers.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListBrokersOutput {
    /// <p>A list of information about all brokers.</p>
    pub broker_summaries: Option<Vec<BrokerSummary>>,
    /// <p>The token that specifies the next page of results Amazon MQ should return. To request the first page, leave nextToken empty.</p>
    pub next_token: Option<String>,
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

/// <p>Returns a list of all revisions for the specified configuration.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListConfigurationRevisionsOutput {
    /// <p>The unique ID that Amazon MQ generates for the configuration.</p>
    pub configuration_id: Option<String>,
    /// <p>The maximum number of configuration revisions that can be returned per page (20 by default). This value must be an integer from 5 to 100.</p>
    pub max_results: Option<i64>,
    /// <p>The token that specifies the next page of results Amazon MQ should return. To request the first page, leave nextToken empty.</p>
    pub next_token: Option<String>,
    /// <p>The list of all revisions for the specified configuration.</p>
    pub revisions: Option<Vec<ConfigurationRevision>>,
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

/// <p>Returns a list of all configurations.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListConfigurationsOutput {
    /// <p>The list of all revisions for the specified configuration.</p>
    pub configurations: Option<Vec<Configuration>>,
    /// <p>The maximum number of configurations that Amazon MQ can return per page (20 by default). This value must be an integer from 5 to 100.</p>
    pub max_results: Option<i64>,
    /// <p>The token that specifies the next page of results Amazon MQ should return. To request the first page, leave nextToken empty.</p>
    pub next_token: Option<String>,
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

/// <p>Returns a list of all ActiveMQ users.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListUsersOutput {
    /// <p>Required. The unique ID that Amazon MQ generates for the broker.</p>
    pub broker_id: Option<String>,
    /// <p>Required. The maximum number of ActiveMQ users that can be returned per page (20 by default). This value must be an integer from 5 to 100.</p>
    pub max_results: Option<i64>,
    /// <p>The token that specifies the next page of results Amazon MQ should return. To request the first page, leave nextToken empty.</p>
    pub next_token: Option<String>,
    /// <p>Required. The list of all ActiveMQ usernames for the specified broker.</p>
    pub users: Option<Vec<UserSummary>>,
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
pub struct LogsSummary {
    /// <p>Enables audit logging. Every user management action made using JMX or the ActiveMQ Web Console is logged.</p>
    #[serde(rename = "Audit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit: Option<bool>,
    /// <p>Location of CloudWatch Log group where audit logs will be sent.</p>
    #[serde(rename = "AuditLogGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_log_group: Option<String>,
    /// <p>Enables general logging.</p>
    #[serde(rename = "General")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general: Option<bool>,
    /// <p>Location of CloudWatch Log group where general logs will be sent.</p>
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
pub struct RebootBrokerResponse {}

/// <p>Returns information about the XML element or attribute that was sanitized in the configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateBrokerInput {
    /// <p>A list of information about the configuration.</p>
    pub configuration: Option<ConfigurationId>,
    /// <p>Enables Amazon CloudWatch logging for brokers.</p>
    pub logs: Option<Logs>,
}

/// <p>Returns information about the updated broker.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateBrokerOutput {
    /// <p>Required. The unique ID that Amazon MQ generates for the broker.</p>
    pub broker_id: Option<String>,
    /// <p>The ID of the updated configuration.</p>
    pub configuration: Option<ConfigurationId>,
    /// <p>The list of information about logs to be enabled for the specified broker.</p>
    pub logs: Option<Logs>,
}

/// <p>Updates the broker using the specified properties.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateBrokerRequest {
    /// <p>The name of the broker. This value must be unique in your AWS account, 1-50 characters long, must contain only letters, numbers, dashes, and underscores, and must not contain whitespaces, brackets, wildcard characters, or special characters.</p>
    #[serde(rename = "BrokerId")]
    pub broker_id: String,
    /// <p>A list of information about the configuration.</p>
    #[serde(rename = "Configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ConfigurationId>,
    /// <p>Enables Amazon CloudWatch logging for brokers.</p>
    #[serde(rename = "Logs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<Logs>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateBrokerResponse {
    /// <p>Required. The unique ID that Amazon MQ generates for the broker.</p>
    #[serde(rename = "BrokerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_id: Option<String>,
    /// <p>The ID of the updated configuration.</p>
    #[serde(rename = "Configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ConfigurationId>,
    /// <p>The list of information about logs to be enabled for the specified broker.</p>
    #[serde(rename = "Logs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<Logs>,
}

/// <p>Updates the specified configuration.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateConfigurationInput {
    /// <p>Required. The base64-encoded XML configuration.</p>
    pub data: Option<String>,
    /// <p>The description of the configuration.</p>
    pub description: Option<String>,
}

/// <p>Returns information about the updated configuration.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateConfigurationOutput {
    /// <p>Required. The Amazon Resource Name (ARN) of the configuration.</p>
    pub arn: Option<String>,
    /// <p>Required. The date and time of the configuration.</p>
    pub created: Option<f64>,
    /// <p>Required. The unique ID that Amazon MQ generates for the configuration.</p>
    pub id: Option<String>,
    /// <p>The latest revision of the configuration.</p>
    pub latest_revision: Option<ConfigurationRevision>,
    /// <p>Required. The name of the configuration. This value can contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _ ~). This value must be 1-150 characters long.</p>
    pub name: Option<String>,
    /// <p>The list of the first 20 warnings about the configuration XML elements or attributes that were sanitized.</p>
    pub warnings: Option<Vec<SanitizationWarning>>,
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
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateUserInput {
    /// <p>Enables access to the the ActiveMQ Web Console for the ActiveMQ user.</p>
    pub console_access: Option<bool>,
    /// <p>The list of groups (20 maximum) to which the ActiveMQ user belongs. This value can contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _ ~). This value must be 2-100 characters long.</p>
    pub groups: Option<Vec<String>>,
    /// <p>The password of the user. This value must be at least 12 characters long, must contain at least 4 unique characters, and must not contain commas.</p>
    pub password: Option<String>,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateBrokerError {
    pub fn from_body(body: &str) -> CreateBrokerError {
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
                    "BadRequestException" => {
                        CreateBrokerError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => CreateBrokerError::Conflict(String::from(error_message)),
                    "ForbiddenException" => {
                        CreateBrokerError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        CreateBrokerError::InternalServerError(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        CreateBrokerError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateBrokerError::Validation(error_message.to_string())
                    }
                    _ => CreateBrokerError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateBrokerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateBrokerError {
    fn from(err: serde_json::error::Error) -> CreateBrokerError {
        CreateBrokerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateBrokerError {
    fn from(err: CredentialsError) -> CreateBrokerError {
        CreateBrokerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateBrokerError {
    fn from(err: HttpDispatchError) -> CreateBrokerError {
        CreateBrokerError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateBrokerError {
    fn from(err: io::Error) -> CreateBrokerError {
        CreateBrokerError::HttpDispatch(HttpDispatchError::from(err))
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
            CreateBrokerError::Validation(ref cause) => cause,
            CreateBrokerError::Credentials(ref err) => err.description(),
            CreateBrokerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateBrokerError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateConfigurationError {
    pub fn from_body(body: &str) -> CreateConfigurationError {
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
                    "BadRequestException" => {
                        CreateConfigurationError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        CreateConfigurationError::Conflict(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        CreateConfigurationError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        CreateConfigurationError::InternalServerError(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateConfigurationError::Validation(error_message.to_string())
                    }
                    _ => CreateConfigurationError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateConfigurationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateConfigurationError {
    fn from(err: serde_json::error::Error) -> CreateConfigurationError {
        CreateConfigurationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateConfigurationError {
    fn from(err: CredentialsError) -> CreateConfigurationError {
        CreateConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateConfigurationError {
    fn from(err: HttpDispatchError) -> CreateConfigurationError {
        CreateConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateConfigurationError {
    fn from(err: io::Error) -> CreateConfigurationError {
        CreateConfigurationError::HttpDispatch(HttpDispatchError::from(err))
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
            CreateConfigurationError::Validation(ref cause) => cause,
            CreateConfigurationError::Credentials(ref err) => err.description(),
            CreateConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateConfigurationError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateUserError {
    pub fn from_body(body: &str) -> CreateUserError {
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
                    "BadRequestException" => {
                        CreateUserError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => CreateUserError::Conflict(String::from(error_message)),
                    "ForbiddenException" => CreateUserError::Forbidden(String::from(error_message)),
                    "InternalServerErrorException" => {
                        CreateUserError::InternalServerError(String::from(error_message))
                    }
                    "NotFoundException" => CreateUserError::NotFound(String::from(error_message)),
                    "ValidationException" => CreateUserError::Validation(error_message.to_string()),
                    _ => CreateUserError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateUserError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateUserError {
    fn from(err: serde_json::error::Error) -> CreateUserError {
        CreateUserError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateUserError {
    fn from(err: CredentialsError) -> CreateUserError {
        CreateUserError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateUserError {
    fn from(err: HttpDispatchError) -> CreateUserError {
        CreateUserError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateUserError {
    fn from(err: io::Error) -> CreateUserError {
        CreateUserError::HttpDispatch(HttpDispatchError::from(err))
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
            CreateUserError::Validation(ref cause) => cause,
            CreateUserError::Credentials(ref err) => err.description(),
            CreateUserError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateUserError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteBrokerError {
    pub fn from_body(body: &str) -> DeleteBrokerError {
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
                    "BadRequestException" => {
                        DeleteBrokerError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        DeleteBrokerError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        DeleteBrokerError::InternalServerError(String::from(error_message))
                    }
                    "NotFoundException" => DeleteBrokerError::NotFound(String::from(error_message)),
                    "ValidationException" => {
                        DeleteBrokerError::Validation(error_message.to_string())
                    }
                    _ => DeleteBrokerError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteBrokerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteBrokerError {
    fn from(err: serde_json::error::Error) -> DeleteBrokerError {
        DeleteBrokerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteBrokerError {
    fn from(err: CredentialsError) -> DeleteBrokerError {
        DeleteBrokerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteBrokerError {
    fn from(err: HttpDispatchError) -> DeleteBrokerError {
        DeleteBrokerError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteBrokerError {
    fn from(err: io::Error) -> DeleteBrokerError {
        DeleteBrokerError::HttpDispatch(HttpDispatchError::from(err))
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
            DeleteBrokerError::Validation(ref cause) => cause,
            DeleteBrokerError::Credentials(ref err) => err.description(),
            DeleteBrokerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteBrokerError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteUserError {
    pub fn from_body(body: &str) -> DeleteUserError {
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
                    "BadRequestException" => {
                        DeleteUserError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => DeleteUserError::Forbidden(String::from(error_message)),
                    "InternalServerErrorException" => {
                        DeleteUserError::InternalServerError(String::from(error_message))
                    }
                    "NotFoundException" => DeleteUserError::NotFound(String::from(error_message)),
                    "ValidationException" => DeleteUserError::Validation(error_message.to_string()),
                    _ => DeleteUserError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteUserError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteUserError {
    fn from(err: serde_json::error::Error) -> DeleteUserError {
        DeleteUserError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteUserError {
    fn from(err: CredentialsError) -> DeleteUserError {
        DeleteUserError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteUserError {
    fn from(err: HttpDispatchError) -> DeleteUserError {
        DeleteUserError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteUserError {
    fn from(err: io::Error) -> DeleteUserError {
        DeleteUserError::HttpDispatch(HttpDispatchError::from(err))
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
            DeleteUserError::Validation(ref cause) => cause,
            DeleteUserError::Credentials(ref err) => err.description(),
            DeleteUserError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteUserError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeBrokerError {
    pub fn from_body(body: &str) -> DescribeBrokerError {
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
                    "BadRequestException" => {
                        DescribeBrokerError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        DescribeBrokerError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        DescribeBrokerError::InternalServerError(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DescribeBrokerError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeBrokerError::Validation(error_message.to_string())
                    }
                    _ => DescribeBrokerError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeBrokerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeBrokerError {
    fn from(err: serde_json::error::Error) -> DescribeBrokerError {
        DescribeBrokerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeBrokerError {
    fn from(err: CredentialsError) -> DescribeBrokerError {
        DescribeBrokerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeBrokerError {
    fn from(err: HttpDispatchError) -> DescribeBrokerError {
        DescribeBrokerError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeBrokerError {
    fn from(err: io::Error) -> DescribeBrokerError {
        DescribeBrokerError::HttpDispatch(HttpDispatchError::from(err))
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
            DescribeBrokerError::Validation(ref cause) => cause,
            DescribeBrokerError::Credentials(ref err) => err.description(),
            DescribeBrokerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeBrokerError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeConfigurationError {
    pub fn from_body(body: &str) -> DescribeConfigurationError {
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
                    "BadRequestException" => {
                        DescribeConfigurationError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        DescribeConfigurationError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        DescribeConfigurationError::InternalServerError(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DescribeConfigurationError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeConfigurationError::Validation(error_message.to_string())
                    }
                    _ => DescribeConfigurationError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeConfigurationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeConfigurationError {
    fn from(err: serde_json::error::Error) -> DescribeConfigurationError {
        DescribeConfigurationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeConfigurationError {
    fn from(err: CredentialsError) -> DescribeConfigurationError {
        DescribeConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeConfigurationError {
    fn from(err: HttpDispatchError) -> DescribeConfigurationError {
        DescribeConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeConfigurationError {
    fn from(err: io::Error) -> DescribeConfigurationError {
        DescribeConfigurationError::HttpDispatch(HttpDispatchError::from(err))
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
            DescribeConfigurationError::Validation(ref cause) => cause,
            DescribeConfigurationError::Credentials(ref err) => err.description(),
            DescribeConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeConfigurationError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeConfigurationRevisionError {
    pub fn from_body(body: &str) -> DescribeConfigurationRevisionError {
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
                    "BadRequestException" => {
                        DescribeConfigurationRevisionError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        DescribeConfigurationRevisionError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        DescribeConfigurationRevisionError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
                    "NotFoundException" => {
                        DescribeConfigurationRevisionError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeConfigurationRevisionError::Validation(error_message.to_string())
                    }
                    _ => DescribeConfigurationRevisionError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeConfigurationRevisionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeConfigurationRevisionError {
    fn from(err: serde_json::error::Error) -> DescribeConfigurationRevisionError {
        DescribeConfigurationRevisionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeConfigurationRevisionError {
    fn from(err: CredentialsError) -> DescribeConfigurationRevisionError {
        DescribeConfigurationRevisionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeConfigurationRevisionError {
    fn from(err: HttpDispatchError) -> DescribeConfigurationRevisionError {
        DescribeConfigurationRevisionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeConfigurationRevisionError {
    fn from(err: io::Error) -> DescribeConfigurationRevisionError {
        DescribeConfigurationRevisionError::HttpDispatch(HttpDispatchError::from(err))
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
            DescribeConfigurationRevisionError::Validation(ref cause) => cause,
            DescribeConfigurationRevisionError::Credentials(ref err) => err.description(),
            DescribeConfigurationRevisionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeConfigurationRevisionError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeUserError {
    pub fn from_body(body: &str) -> DescribeUserError {
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
                    "BadRequestException" => {
                        DescribeUserError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        DescribeUserError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        DescribeUserError::InternalServerError(String::from(error_message))
                    }
                    "NotFoundException" => DescribeUserError::NotFound(String::from(error_message)),
                    "ValidationException" => {
                        DescribeUserError::Validation(error_message.to_string())
                    }
                    _ => DescribeUserError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeUserError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeUserError {
    fn from(err: serde_json::error::Error) -> DescribeUserError {
        DescribeUserError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeUserError {
    fn from(err: CredentialsError) -> DescribeUserError {
        DescribeUserError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeUserError {
    fn from(err: HttpDispatchError) -> DescribeUserError {
        DescribeUserError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeUserError {
    fn from(err: io::Error) -> DescribeUserError {
        DescribeUserError::HttpDispatch(HttpDispatchError::from(err))
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
            DescribeUserError::Validation(ref cause) => cause,
            DescribeUserError::Credentials(ref err) => err.description(),
            DescribeUserError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeUserError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListBrokersError {
    pub fn from_body(body: &str) -> ListBrokersError {
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
                    "BadRequestException" => {
                        ListBrokersError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        ListBrokersError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        ListBrokersError::InternalServerError(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListBrokersError::Validation(error_message.to_string())
                    }
                    _ => ListBrokersError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListBrokersError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListBrokersError {
    fn from(err: serde_json::error::Error) -> ListBrokersError {
        ListBrokersError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListBrokersError {
    fn from(err: CredentialsError) -> ListBrokersError {
        ListBrokersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListBrokersError {
    fn from(err: HttpDispatchError) -> ListBrokersError {
        ListBrokersError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListBrokersError {
    fn from(err: io::Error) -> ListBrokersError {
        ListBrokersError::HttpDispatch(HttpDispatchError::from(err))
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
            ListBrokersError::Validation(ref cause) => cause,
            ListBrokersError::Credentials(ref err) => err.description(),
            ListBrokersError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListBrokersError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListConfigurationRevisionsError {
    pub fn from_body(body: &str) -> ListConfigurationRevisionsError {
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
                    "BadRequestException" => {
                        ListConfigurationRevisionsError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        ListConfigurationRevisionsError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        ListConfigurationRevisionsError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
                    "NotFoundException" => {
                        ListConfigurationRevisionsError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListConfigurationRevisionsError::Validation(error_message.to_string())
                    }
                    _ => ListConfigurationRevisionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListConfigurationRevisionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListConfigurationRevisionsError {
    fn from(err: serde_json::error::Error) -> ListConfigurationRevisionsError {
        ListConfigurationRevisionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListConfigurationRevisionsError {
    fn from(err: CredentialsError) -> ListConfigurationRevisionsError {
        ListConfigurationRevisionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListConfigurationRevisionsError {
    fn from(err: HttpDispatchError) -> ListConfigurationRevisionsError {
        ListConfigurationRevisionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListConfigurationRevisionsError {
    fn from(err: io::Error) -> ListConfigurationRevisionsError {
        ListConfigurationRevisionsError::HttpDispatch(HttpDispatchError::from(err))
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
            ListConfigurationRevisionsError::Validation(ref cause) => cause,
            ListConfigurationRevisionsError::Credentials(ref err) => err.description(),
            ListConfigurationRevisionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListConfigurationRevisionsError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListConfigurationsError {
    pub fn from_body(body: &str) -> ListConfigurationsError {
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
                    "BadRequestException" => {
                        ListConfigurationsError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        ListConfigurationsError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        ListConfigurationsError::InternalServerError(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListConfigurationsError::Validation(error_message.to_string())
                    }
                    _ => ListConfigurationsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListConfigurationsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListConfigurationsError {
    fn from(err: serde_json::error::Error) -> ListConfigurationsError {
        ListConfigurationsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListConfigurationsError {
    fn from(err: CredentialsError) -> ListConfigurationsError {
        ListConfigurationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListConfigurationsError {
    fn from(err: HttpDispatchError) -> ListConfigurationsError {
        ListConfigurationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListConfigurationsError {
    fn from(err: io::Error) -> ListConfigurationsError {
        ListConfigurationsError::HttpDispatch(HttpDispatchError::from(err))
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
            ListConfigurationsError::Validation(ref cause) => cause,
            ListConfigurationsError::Credentials(ref err) => err.description(),
            ListConfigurationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListConfigurationsError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListUsersError {
    pub fn from_body(body: &str) -> ListUsersError {
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
                    "BadRequestException" => {
                        ListUsersError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => ListUsersError::Forbidden(String::from(error_message)),
                    "InternalServerErrorException" => {
                        ListUsersError::InternalServerError(String::from(error_message))
                    }
                    "NotFoundException" => ListUsersError::NotFound(String::from(error_message)),
                    "ValidationException" => ListUsersError::Validation(error_message.to_string()),
                    _ => ListUsersError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListUsersError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListUsersError {
    fn from(err: serde_json::error::Error) -> ListUsersError {
        ListUsersError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListUsersError {
    fn from(err: CredentialsError) -> ListUsersError {
        ListUsersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListUsersError {
    fn from(err: HttpDispatchError) -> ListUsersError {
        ListUsersError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListUsersError {
    fn from(err: io::Error) -> ListUsersError {
        ListUsersError::HttpDispatch(HttpDispatchError::from(err))
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
            ListUsersError::Validation(ref cause) => cause,
            ListUsersError::Credentials(ref err) => err.description(),
            ListUsersError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListUsersError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RebootBrokerError {
    pub fn from_body(body: &str) -> RebootBrokerError {
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
                    "BadRequestException" => {
                        RebootBrokerError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        RebootBrokerError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        RebootBrokerError::InternalServerError(String::from(error_message))
                    }
                    "NotFoundException" => RebootBrokerError::NotFound(String::from(error_message)),
                    "ValidationException" => {
                        RebootBrokerError::Validation(error_message.to_string())
                    }
                    _ => RebootBrokerError::Unknown(String::from(body)),
                }
            }
            Err(_) => RebootBrokerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RebootBrokerError {
    fn from(err: serde_json::error::Error) -> RebootBrokerError {
        RebootBrokerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RebootBrokerError {
    fn from(err: CredentialsError) -> RebootBrokerError {
        RebootBrokerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RebootBrokerError {
    fn from(err: HttpDispatchError) -> RebootBrokerError {
        RebootBrokerError::HttpDispatch(err)
    }
}
impl From<io::Error> for RebootBrokerError {
    fn from(err: io::Error) -> RebootBrokerError {
        RebootBrokerError::HttpDispatch(HttpDispatchError::from(err))
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
            RebootBrokerError::Validation(ref cause) => cause,
            RebootBrokerError::Credentials(ref err) => err.description(),
            RebootBrokerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RebootBrokerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateBroker
#[derive(Debug, PartialEq)]
pub enum UpdateBrokerError {
    /// <p>Returns information about an error.</p>
    BadRequest(String),
    /// <p>Returns information about an error.</p>
    Forbidden(String),
    /// <p>Returns information about an error.</p>
    InternalServerError(String),
    /// <p>Returns information about an error.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateBrokerError {
    pub fn from_body(body: &str) -> UpdateBrokerError {
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
                    "BadRequestException" => {
                        UpdateBrokerError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        UpdateBrokerError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        UpdateBrokerError::InternalServerError(String::from(error_message))
                    }
                    "NotFoundException" => UpdateBrokerError::NotFound(String::from(error_message)),
                    "ValidationException" => {
                        UpdateBrokerError::Validation(error_message.to_string())
                    }
                    _ => UpdateBrokerError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateBrokerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateBrokerError {
    fn from(err: serde_json::error::Error) -> UpdateBrokerError {
        UpdateBrokerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateBrokerError {
    fn from(err: CredentialsError) -> UpdateBrokerError {
        UpdateBrokerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateBrokerError {
    fn from(err: HttpDispatchError) -> UpdateBrokerError {
        UpdateBrokerError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateBrokerError {
    fn from(err: io::Error) -> UpdateBrokerError {
        UpdateBrokerError::HttpDispatch(HttpDispatchError::from(err))
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
            UpdateBrokerError::Forbidden(ref cause) => cause,
            UpdateBrokerError::InternalServerError(ref cause) => cause,
            UpdateBrokerError::NotFound(ref cause) => cause,
            UpdateBrokerError::Validation(ref cause) => cause,
            UpdateBrokerError::Credentials(ref err) => err.description(),
            UpdateBrokerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateBrokerError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateConfigurationError {
    pub fn from_body(body: &str) -> UpdateConfigurationError {
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
                    "BadRequestException" => {
                        UpdateConfigurationError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        UpdateConfigurationError::Conflict(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        UpdateConfigurationError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        UpdateConfigurationError::InternalServerError(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateConfigurationError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateConfigurationError::Validation(error_message.to_string())
                    }
                    _ => UpdateConfigurationError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateConfigurationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateConfigurationError {
    fn from(err: serde_json::error::Error) -> UpdateConfigurationError {
        UpdateConfigurationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateConfigurationError {
    fn from(err: CredentialsError) -> UpdateConfigurationError {
        UpdateConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateConfigurationError {
    fn from(err: HttpDispatchError) -> UpdateConfigurationError {
        UpdateConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateConfigurationError {
    fn from(err: io::Error) -> UpdateConfigurationError {
        UpdateConfigurationError::HttpDispatch(HttpDispatchError::from(err))
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
            UpdateConfigurationError::Validation(ref cause) => cause,
            UpdateConfigurationError::Credentials(ref err) => err.description(),
            UpdateConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateConfigurationError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateUserError {
    pub fn from_body(body: &str) -> UpdateUserError {
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
                    "BadRequestException" => {
                        UpdateUserError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => UpdateUserError::Conflict(String::from(error_message)),
                    "ForbiddenException" => UpdateUserError::Forbidden(String::from(error_message)),
                    "InternalServerErrorException" => {
                        UpdateUserError::InternalServerError(String::from(error_message))
                    }
                    "NotFoundException" => UpdateUserError::NotFound(String::from(error_message)),
                    "ValidationException" => UpdateUserError::Validation(error_message.to_string()),
                    _ => UpdateUserError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateUserError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateUserError {
    fn from(err: serde_json::error::Error) -> UpdateUserError {
        UpdateUserError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateUserError {
    fn from(err: CredentialsError) -> UpdateUserError {
        UpdateUserError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateUserError {
    fn from(err: HttpDispatchError) -> UpdateUserError {
        UpdateUserError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateUserError {
    fn from(err: io::Error) -> UpdateUserError {
        UpdateUserError::HttpDispatch(HttpDispatchError::from(err))
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
            UpdateUserError::Validation(ref cause) => cause,
            UpdateUserError::Credentials(ref err) => err.description(),
            UpdateUserError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateUserError::Unknown(ref cause) => cause,
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
pub struct MQClient {
    client: Client,
    region: region::Region,
}

impl MQClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> MQClient {
        MQClient {
            client: Client::shared(),
            region: region,
        }
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
        MQClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CreateBrokerResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateBrokerError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<CreateConfigurationResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateConfigurationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CreateUserResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateUserError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DeleteBrokerResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteBrokerError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DeleteUserResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteUserError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DescribeBrokerResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeBrokerError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<DescribeConfigurationResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeConfigurationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<DescribeConfigurationRevisionResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeConfigurationRevisionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DescribeUserResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeUserError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListBrokersResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListBrokersError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListConfigurationRevisionsResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListConfigurationRevisionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListConfigurationsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListConfigurationsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListUsersResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListUsersError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<RebootBrokerResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(RebootBrokerError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<UpdateBrokerResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateBrokerError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<UpdateConfigurationResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateConfigurationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<UpdateUserResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateUserError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
