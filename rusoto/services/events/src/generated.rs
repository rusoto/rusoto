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

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ActivateEventSourceRequest {
    /// <p>The name of the partner event source to activate.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// <p>This structure specifies the VPC subnets and security groups for the task and whether a public IP address is to be used. This structure is relevant only for ECS tasks that use the <code>awsvpc</code> network mode.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsVpcConfiguration {
    /// <p>Specifies whether the task's elastic network interface receives a public IP address. You can specify <code>ENABLED</code> only when <code>LaunchType</code> in <code>EcsParameters</code> is set to <code>FARGATE</code>.</p>
    #[serde(rename = "AssignPublicIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assign_public_ip: Option<String>,
    /// <p>Specifies the security groups associated with the task. These security groups must all be in the same VPC. You can specify as many as five security groups. If you don't specify a security group, the default security group for the VPC is used.</p>
    #[serde(rename = "SecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    /// <p>Specifies the subnets associated with the task. These subnets must all be in the same VPC. You can specify as many as 16 subnets.</p>
    #[serde(rename = "Subnets")]
    pub subnets: Vec<String>,
}

/// <p>The array properties for the submitted job, such as the size of the array. The array size can be between 2 and 10,000. If you specify array properties for a job, it becomes an array job. This parameter is used only if the target is an AWS Batch job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BatchArrayProperties {
    /// <p>The size of the array, if this is an array batch job. Valid values are integers between 2 and 10,000.</p>
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

/// <p>The custom parameters to be used when the target is an AWS Batch job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BatchParameters {
    /// <p>The array properties for the submitted job, such as the size of the array. The array size can be between 2 and 10,000. If you specify array properties for a job, it becomes an array job. This parameter is used only if the target is an AWS Batch job.</p>
    #[serde(rename = "ArrayProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array_properties: Option<BatchArrayProperties>,
    /// <p>The ARN or name of the job definition to use if the event target is an AWS Batch job. This job definition must already exist.</p>
    #[serde(rename = "JobDefinition")]
    pub job_definition: String,
    /// <p>The name to use for this execution of the job, if the target is an AWS Batch job.</p>
    #[serde(rename = "JobName")]
    pub job_name: String,
    /// <p>The retry strategy to use for failed jobs if the target is an AWS Batch job. The retry strategy is the number of times to retry the failed job execution. Valid values are 1–10. When you specify a retry strategy here, it overrides the retry strategy defined in the job definition.</p>
    #[serde(rename = "RetryStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_strategy: Option<BatchRetryStrategy>,
}

/// <p>The retry strategy to use for failed jobs if the target is an AWS Batch job. If you specify a retry strategy here, it overrides the retry strategy defined in the job definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BatchRetryStrategy {
    /// <p>The number of times to attempt to retry, if the job fails. Valid values are 1–10.</p>
    #[serde(rename = "Attempts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempts: Option<i64>,
}

/// <p>A JSON string that you can use to limit the event bus permissions that you're granting to only accounts that fulfill the condition. Currently, the only supported condition is membership in a certain AWS organization. The string must contain <code>Type</code>, <code>Key</code>, and <code>Value</code> fields. The <code>Value</code> field specifies the ID of the AWS organization. The following is an example value for <code>Condition</code>:</p> <p> <code>'{"Type" : "StringEquals", "Key": "aws:PrincipalOrgID", "Value": "o-1234567890"}'</code> </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Condition {
    /// <p>The key for the condition. Currently, the only supported key is <code>aws:PrincipalOrgID</code>.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The type of condition. Currently, the only supported value is <code>StringEquals</code>.</p>
    #[serde(rename = "Type")]
    pub type_: String,
    /// <p>The value for the key. Currently, this must be the ID of the organization.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateEventBusRequest {
    /// <p>If you're creating a partner event bus, this specifies the partner event source that the new event bus will be matched with.</p>
    #[serde(rename = "EventSourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source_name: Option<String>,
    /// <p>The name of the new event bus. </p> <p>The names of custom event buses can't contain the <code>/</code> character. You can't use the name <code>default</code> for a custom event bus because this name is already used for your account's default event bus.</p> <p>If this is a partner event bus, the name must exactly match the name of the partner event source that this event bus is matched to. This name will include the <code>/</code> character.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateEventBusResponse {
    /// <p>The ARN of the new event bus.</p>
    #[serde(rename = "EventBusArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreatePartnerEventSourceRequest {
    /// <p>The AWS account ID of the customer who is permitted to create a matching partner event bus for this partner event source.</p>
    #[serde(rename = "Account")]
    pub account: String,
    /// <p>The name of the partner event source. This name must be unique and must be in the format <code> <i>partner_name</i>/<i>event_namespace</i>/<i>event_name</i> </code>. The AWS account that wants to use this partner event source must create a partner event bus with a name that matches the name of the partner event source.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreatePartnerEventSourceResponse {
    /// <p>The ARN of the partner event source.</p>
    #[serde(rename = "EventSourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeactivateEventSourceRequest {
    /// <p>The name of the partner event source to deactivate.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteEventBusRequest {
    /// <p>The name of the event bus to delete.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeletePartnerEventSourceRequest {
    /// <p>The AWS account ID of the AWS customer that the event source was created for.</p>
    #[serde(rename = "Account")]
    pub account: String,
    /// <p>The name of the event source to delete.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteRuleRequest {
    /// <p>The event bus associated with the rule. If you omit this, the default event bus is used.</p>
    #[serde(rename = "EventBusName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    /// <p>If this is a managed rule, created by an AWS service on your behalf, you must specify <code>Force</code> as <code>True</code> to delete the rule. This parameter is ignored for rules that are not managed rules. You can check whether a rule is a managed rule by using <code>DescribeRule</code> or <code>ListRules</code> and checking the <code>ManagedBy</code> field of the response.</p>
    #[serde(rename = "Force")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    /// <p>The name of the rule.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEventBusRequest {
    /// <p>The name of the event bus to show details for. If you omit this, the default event bus is displayed.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeEventBusResponse {
    /// <p>The Amazon Resource Name (ARN) of the account permitted to write events to the current account.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The name of the event bus. Currently, this is always <code>default</code>.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The policy that enables the external account to send events to your account.</p>
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEventSourceRequest {
    /// <p>The name of the partner event source to display the details of.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeEventSourceResponse {
    /// <p>The ARN of the partner event source.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The name of the SaaS partner that created the event source.</p>
    #[serde(rename = "CreatedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// <p>The date and time that the event source was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The date and time that the event source will expire if you don't create a matching event bus.</p>
    #[serde(rename = "ExpirationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<f64>,
    /// <p>The name of the partner event source.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The state of the event source. If it's <code>ACTIVE</code>, you have already created a matching event bus for this event source, and that event bus is active. If it's <code>PENDING</code>, either you haven't yet created a matching event bus, or that event bus is deactivated. If it's <code>DELETED</code>, you have created a matching event bus, but the event source has since been deleted.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribePartnerEventSourceRequest {
    /// <p>The name of the event source to display.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribePartnerEventSourceResponse {
    /// <p>The ARN of the event source.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The name of the event source.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeRuleRequest {
    /// <p>The event bus associated with the rule. If you omit this, the default event bus is used.</p>
    #[serde(rename = "EventBusName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    /// <p>The name of the rule.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeRuleResponse {
    /// <p>The Amazon Resource Name (ARN) of the rule.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The description of the rule.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The event bus associated with the rule.</p>
    #[serde(rename = "EventBusName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    /// <p>The event pattern. For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/eventbridge-and-event-patterns.html">Event Patterns</a> in the <i>Amazon EventBridge User Guide</i>.</p>
    #[serde(rename = "EventPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_pattern: Option<String>,
    /// <p>If this is a managed rule, created by an AWS service on your behalf, this field displays the principal name of the AWS service that created the rule.</p>
    #[serde(rename = "ManagedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
    /// <p>The name of the rule.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the IAM role associated with the rule.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The scheduling expression: for example, <code>"cron(0 20 * * ? *)"</code> or <code>"rate(5 minutes)"</code>.</p>
    #[serde(rename = "ScheduleExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    /// <p>Specifies whether the rule is enabled or disabled.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisableRuleRequest {
    /// <p>The event bus associated with the rule. If you omit this, the default event bus is used.</p>
    #[serde(rename = "EventBusName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    /// <p>The name of the rule.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// <p>The custom parameters to be used when the target is an Amazon ECS task.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EcsParameters {
    /// <p>Specifies an ECS task group for the task. The maximum length is 255 characters.</p>
    #[serde(rename = "Group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// <p>Specifies the launch type on which your task is running. The launch type that you specify here must match one of the launch type (compatibilities) of the target task. The <code>FARGATE</code> value is supported only in the Regions where AWS Fargate with Amazon ECS is supported. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/AWS-Fargate.html">AWS Fargate on Amazon ECS</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "LaunchType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_type: Option<String>,
    /// <p>Use this structure if the ECS task uses the <code>awsvpc</code> network mode. This structure specifies the VPC subnets and security groups associated with the task and whether a public IP address is to be used. This structure is required if <code>LaunchType</code> is <code>FARGATE</code> because the <code>awsvpc</code> mode is required for Fargate tasks.</p> <p>If you specify <code>NetworkConfiguration</code> when the target ECS task doesn't use the <code>awsvpc</code> network mode, the task fails.</p>
    #[serde(rename = "NetworkConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    /// <p>Specifies the platform version for the task. Specify only the numeric portion of the platform version, such as <code>1.1.0</code>.</p> <p>This structure is used only if <code>LaunchType</code> is <code>FARGATE</code>. For more information about valid platform versions, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/platform_versions.html">AWS Fargate Platform Versions</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "PlatformVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    /// <p>The number of tasks to create based on <code>TaskDefinition</code>. The default is 1.</p>
    #[serde(rename = "TaskCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_count: Option<i64>,
    /// <p>The ARN of the task definition to use if the event target is an Amazon ECS task. </p>
    #[serde(rename = "TaskDefinitionArn")]
    pub task_definition_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EnableRuleRequest {
    /// <p>The event bus associated with the rule. If you omit this, the default event bus is used.</p>
    #[serde(rename = "EventBusName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    /// <p>The name of the rule.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// <p>An event bus receives events from a source and routes them to rules associated with that event bus. Your account's default event bus receives rules from AWS services. A custom event bus can receive rules from AWS services as well as your custom applications and services. A partner event bus receives events from an event source created by an SaaS partner. These events come from the partners services or applications.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EventBus {
    /// <p>The ARN of the event bus.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The name of the event bus.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The permissions policy of the event bus, describing which other AWS accounts can write events to this event bus.</p>
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

/// <p>A partner event source is created by an SaaS partner. If a customer creates a partner event bus that matches this event source, that AWS account can receive events from the partner's applications or services.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EventSource {
    /// <p>The ARN of the event source.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The name of the partner that created the event source.</p>
    #[serde(rename = "CreatedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// <p>The date and time when the event source was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The date and time when the event source will expire if the AWS account doesn't create a matching event bus for it.</p>
    #[serde(rename = "ExpirationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<f64>,
    /// <p>The name of the event source.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The state of the event source. If it's <code>ACTIVE</code>, you have already created a matching event bus for this event source, and that event bus is active. If it's <code>PENDING</code>, either you haven't yet created a matching event bus, or that event bus is deactivated. If it's <code>DELETED</code>, you have created a matching event bus, but the event source has since been deleted.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// <p>Contains the parameters needed for you to provide custom input to a target based on one or more pieces of data extracted from the event.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputTransformer {
    /// <p>Map of JSON paths to be extracted from the event. You can then insert these in the template in <code>InputTemplate</code> to produce the output to be sent to the target.</p> <p> <code>InputPathsMap</code> is an array key-value pairs, where each value is a valid JSON path. You can have as many as 10 key-value pairs. You must use JSON dot notation, not bracket notation.</p> <p>The keys can't start with <code>"AWS"</code>.</p>
    #[serde(rename = "InputPathsMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_paths_map: Option<::std::collections::HashMap<String, String>>,
    /// <p>Input template where you specify placeholders that will be filled with the values of the keys from <code>InputPathsMap</code> to customize the data sent to the target. Enclose each <code>InputPathsMaps</code> value in brackets: &lt;<i>value</i>&gt;. The InputTemplate must be valid JSON.</p> <p>If <code>InputTemplate</code> is a JSON object (surrounded by curly braces), the following restrictions apply:</p> <ul> <li> <p>The placeholder can't be used as an object key</p> </li> <li> <p>Object values can't include quote marks</p> </li> </ul> <p>The following example shows the syntax for using <code>InputPathsMap</code> and <code>InputTemplate</code>.</p> <p> <code> "InputTransformer":</code> </p> <p> <code>{</code> </p> <p> <code>"InputPathsMap": {"instance": "$.detail.instance","status": "$.detail.status"},</code> </p> <p> <code>"InputTemplate": "&lt;instance&gt; is in state &lt;status&gt;"</code> </p> <p> <code>}</code> </p> <p>To have the <code>InputTemplate</code> include quote marks within a JSON string, escape each quote marks with a slash, as in the following example:</p> <p> <code> "InputTransformer":</code> </p> <p> <code>{</code> </p> <p> <code>"InputPathsMap": {"instance": "$.detail.instance","status": "$.detail.status"},</code> </p> <p> <code>"InputTemplate": "&lt;instance&gt; is in state \"&lt;status&gt;\""</code> </p> <p> <code>}</code> </p>
    #[serde(rename = "InputTemplate")]
    pub input_template: String,
}

/// <p>This object enables you to specify a JSON path to extract from the event and use as the partition key for the Amazon Kinesis data stream so that you can control the shard that the event goes to. If you don't include this parameter, the default is to use the <code>eventId</code> as the partition key.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KinesisParameters {
    /// <p>The JSON path to be extracted from the event and used as the partition key. For more information, see <a href="https://docs.aws.amazon.com/streams/latest/dev/key-concepts.html#partition-key">Amazon Kinesis Streams Key Concepts</a> in the <i>Amazon Kinesis Streams Developer Guide</i>.</p>
    #[serde(rename = "PartitionKeyPath")]
    pub partition_key_path: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListEventBusesRequest {
    /// <p>Specifying this limits the number of results returned by this operation. The operation also returns a <code>NextToken</code> that you can use in a subsequent operation to retrieve the next set of results.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>Specifying this limits the results to only those event buses with names that start with the specified prefix.</p>
    #[serde(rename = "NamePrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_prefix: Option<String>,
    /// <p>The token returned by a previous call to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListEventBusesResponse {
    /// <p>This list of event buses.</p>
    #[serde(rename = "EventBuses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_buses: Option<Vec<EventBus>>,
    /// <p>A token you can use in a subsequent operation to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListEventSourcesRequest {
    /// <p>Specifying this limits the number of results returned by this operation. The operation also returns a <code>NextToken</code> that you can use in a subsequent operation to retrieve the next set of results.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>Specifying this limits the results to only those partner event sources with names that start with the specified prefix.</p>
    #[serde(rename = "NamePrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_prefix: Option<String>,
    /// <p>The token returned by a previous call to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListEventSourcesResponse {
    /// <p>The list of event sources.</p>
    #[serde(rename = "EventSources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_sources: Option<Vec<EventSource>>,
    /// <p>A token you can use in a subsequent operation to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListPartnerEventSourceAccountsRequest {
    /// <p>The name of the partner event source to display account information about.</p>
    #[serde(rename = "EventSourceName")]
    pub event_source_name: String,
    /// <p>Specifying this limits the number of results returned by this operation. The operation also returns a <code>NextToken</code> that you can use in a subsequent operation to retrieve the next set of results.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The token returned by a previous call to this operation. Specifying this retrieves the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListPartnerEventSourceAccountsResponse {
    /// <p>A token you can use in a subsequent operation to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of partner event sources returned by the operation.</p>
    #[serde(rename = "PartnerEventSourceAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_event_source_accounts: Option<Vec<PartnerEventSourceAccount>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListPartnerEventSourcesRequest {
    /// <p>pecifying this limits the number of results returned by this operation. The operation also returns a <code>NextToken</code> that you can use in a subsequent operation to retrieve the next set of results.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>If you specify this, the results are limited to only those partner event sources that start with the string you specify.</p>
    #[serde(rename = "NamePrefix")]
    pub name_prefix: String,
    /// <p>The token returned by a previous call to this operation. Specifying this retrieves the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListPartnerEventSourcesResponse {
    /// <p>A token you can use in a subsequent operation to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of partner event sources returned by the operation.</p>
    #[serde(rename = "PartnerEventSources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_event_sources: Option<Vec<PartnerEventSource>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListRuleNamesByTargetRequest {
    /// <p>Limits the results to show only the rules associated with the specified event bus.</p>
    #[serde(rename = "EventBusName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    /// <p>The maximum number of results to return.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The token returned by a previous call to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the target resource.</p>
    #[serde(rename = "TargetArn")]
    pub target_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListRuleNamesByTargetResponse {
    /// <p>Indicates whether there are additional results to retrieve. If there are no more results, the value is null.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The names of the rules that can invoke the given target.</p>
    #[serde(rename = "RuleNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_names: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListRulesRequest {
    /// <p>Limits the results to show only the rules associated with the specified event bus.</p>
    #[serde(rename = "EventBusName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    /// <p>The maximum number of results to return.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The prefix matching the rule name.</p>
    #[serde(rename = "NamePrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_prefix: Option<String>,
    /// <p>The token returned by a previous call to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListRulesResponse {
    /// <p>Indicates whether there are additional results to retrieve. If there are no more results, the value is null.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The rules that match the specified criteria.</p>
    #[serde(rename = "Rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<Rule>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The ARN of the rule for which you want to view tags.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>The list of tag keys and values associated with the rule that you specified.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTargetsByRuleRequest {
    /// <p>The event bus associated with the rule. If you omit this, the default event bus is used.</p>
    #[serde(rename = "EventBusName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    /// <p>The maximum number of results to return.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The token returned by a previous call to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the rule.</p>
    #[serde(rename = "Rule")]
    pub rule: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTargetsByRuleResponse {
    /// <p>Indicates whether there are additional results to retrieve. If there are no more results, the value is null.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The targets assigned to the rule.</p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
}

/// <p>This structure specifies the network configuration for an ECS task.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkConfiguration {
    /// <p>Use this structure to specify the VPC subnets and security groups for the task and whether a public IP address is to be used. This structure is relevant only for ECS tasks that use the <code>awsvpc</code> network mode.</p>
    #[serde(rename = "awsvpcConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub awsvpc_configuration: Option<AwsVpcConfiguration>,
}

/// <p>A partner event source is created by an SaaS partner. If a customer creates a partner event bus that matches this event source, that AWS account can receive events from the partner's applications or services.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PartnerEventSource {
    /// <p>The ARN of the partner event source.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The name of the partner event source.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>The AWS account that a partner event source has been offered to.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PartnerEventSourceAccount {
    /// <p>The AWS account ID that the partner event source was offered to.</p>
    #[serde(rename = "Account")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// <p>The date and time when the event source was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The date and time when the event source will expire if the AWS account doesn't create a matching event bus for it.</p>
    #[serde(rename = "ExpirationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<f64>,
    /// <p>The state of the event source. If it's <code>ACTIVE</code>, you have already created a matching event bus for this event source, and that event bus is active. If it's <code>PENDING</code>, either you haven't yet created a matching event bus, or that event bus is deactivated. If it's <code>DELETED</code>, you have created a matching event bus, but the event source has since been deleted.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutEventsRequest {
    /// <p>The entry that defines an event in your system. You can specify several parameters for the entry such as the source and type of the event, resources associated with the event, and so on.</p>
    #[serde(rename = "Entries")]
    pub entries: Vec<PutEventsRequestEntry>,
}

/// <p>Represents an event to be submitted.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutEventsRequestEntry {
    /// <p>A valid JSON object. There is no other schema imposed. The JSON object can contain fields and nested subobjects.</p> <p>This field is required.</p>
    #[serde(rename = "Detail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// <p>Free-form string used to decide which fields to expect in the event detail. This field is required.</p>
    #[serde(rename = "DetailType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_type: Option<String>,
    /// <p>The event bus that will receive the event. Only the rules that are associated with this event bus can match the event.</p>
    #[serde(rename = "EventBusName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    /// <p>AWS resources, identified by Amazon Resource Name (ARN), that the event primarily concerns. Any number, including zero, can be present.</p>
    #[serde(rename = "Resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<String>>,
    /// <p>The source of the event. This field is required.</p>
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// <p>The timestamp of the event, per <a href="https://www.rfc-editor.org/rfc/rfc3339.txt">RFC3339</a>. If no timestamp is provided, the timestamp of the <a>PutEvents</a> call is used.</p>
    #[serde(rename = "Time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutEventsResponse {
    /// <p>The successfully and unsuccessfully ingested events results. If the ingestion was successful, the entry has the event ID in it. Otherwise, you can use the error code and error message to identify the problem with the entry.</p>
    #[serde(rename = "Entries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<PutEventsResultEntry>>,
    /// <p>The number of failed entries.</p>
    #[serde(rename = "FailedEntryCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_entry_count: Option<i64>,
}

/// <p>Represents an event that failed to be submitted.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutEventsResultEntry {
    /// <p>The error code that indicates why the event submission failed.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The error message that explains why the event submission failed.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The ID of the event.</p>
    #[serde(rename = "EventId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutPartnerEventsRequest {
    /// <p>The list of events to write to the event bus.</p>
    #[serde(rename = "Entries")]
    pub entries: Vec<PutPartnerEventsRequestEntry>,
}

/// <p>The details about an event generated by an SaaS partner.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutPartnerEventsRequestEntry {
    /// <p>A valid JSON object. There is no other schema imposed. The JSON object can contain fields and nested subobjects. This field is required.</p>
    #[serde(rename = "Detail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// <p>A free-form string used to decide which fields to expect in the event detail. This field is required.</p>
    #[serde(rename = "DetailType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_type: Option<String>,
    /// <p>AWS resources, identified by Amazon Resource Name (ARN), that the event primarily concerns. Any number, including zero, can be present.</p>
    #[serde(rename = "Resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<String>>,
    /// <p>The event source that is generating the evntry. This field is required.</p>
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// <p>The date and time of the event.</p>
    #[serde(rename = "Time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutPartnerEventsResponse {
    /// <p>The list of events from this operation that were successfully written to the partner event bus.</p>
    #[serde(rename = "Entries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<PutPartnerEventsResultEntry>>,
    /// <p>The number of events from this operation that couldn't be written to the partner event bus.</p>
    #[serde(rename = "FailedEntryCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_entry_count: Option<i64>,
}

/// <p>Represents an event that a partner tried to generate but failed.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutPartnerEventsResultEntry {
    /// <p>The error code that indicates why the event submission failed.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The error message that explains why the event submission failed.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The ID of the event.</p>
    #[serde(rename = "EventId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutPermissionRequest {
    /// <p>The action that you're enabling the other account to perform. Currently, this must be <code>events:PutEvents</code>.</p>
    #[serde(rename = "Action")]
    pub action: String,
    /// <p>This parameter enables you to limit the permission to accounts that fulfill a certain condition, such as being a member of a certain AWS organization. For more information about AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_introduction.html">What Is AWS Organizations?</a> in the <i>AWS Organizations User Guide</i>.</p> <p>If you specify <code>Condition</code> with an AWS organization ID and specify "*" as the value for <code>Principal</code>, you grant permission to all the accounts in the named organization.</p> <p>The <code>Condition</code> is a JSON string that must contain <code>Type</code>, <code>Key</code>, and <code>Value</code> fields.</p>
    #[serde(rename = "Condition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<Condition>,
    /// <p>The event bus associated with the rule. If you omit this, the default event bus is used.</p>
    #[serde(rename = "EventBusName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    /// <p>The 12-digit AWS account ID that you are permitting to put events to your default event bus. Specify "*" to permit any account to put events to your default event bus.</p> <p>If you specify "*" without specifying <code>Condition</code>, avoid creating rules that might match undesirable events. To create more secure rules, make sure that the event pattern for each rule contains an <code>account</code> field with a specific account ID to receive events from. Rules that have an account field match events sent only from accounts that are listed in the rule's <code>account</code> field.</p>
    #[serde(rename = "Principal")]
    pub principal: String,
    /// <p>An identifier string for the external account that you're granting permissions to. If you later want to revoke the permission for this external account, specify this <code>StatementId</code> when you run <a>RemovePermission</a>.</p>
    #[serde(rename = "StatementId")]
    pub statement_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutRuleRequest {
    /// <p>A description of the rule.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The event bus to associate with this rule. If you omit this, the default event bus is used.</p>
    #[serde(rename = "EventBusName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    /// <p>The event pattern. For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/eventbridge-and-event-patterns.html">Event Patterns</a> in the <i>Amazon EventBridge User Guide</i>.</p>
    #[serde(rename = "EventPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_pattern: Option<String>,
    /// <p>The name of the rule that you're creating or updating.</p> <p>A rule can't have the same name as another rule in the same Region or on the same event bus.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The Amazon Resource Name (ARN) of the IAM role associated with the rule.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The scheduling expression: for example, <code>"cron(0 20 * * ? *)"</code> or <code>"rate(5 minutes)"</code>.</p>
    #[serde(rename = "ScheduleExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    /// <p>Indicates whether the rule is enabled or disabled.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The list of key-value pairs to associate with the rule.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutRuleResponse {
    /// <p>The Amazon Resource Name (ARN) of the rule.</p>
    #[serde(rename = "RuleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutTargetsRequest {
    /// <p>The name of the event bus associated with the rule. If you omit this, the default event bus is used.</p>
    #[serde(rename = "EventBusName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    /// <p>The name of the rule.</p>
    #[serde(rename = "Rule")]
    pub rule: String,
    /// <p>The targets to update or add to the rule.</p>
    #[serde(rename = "Targets")]
    pub targets: Vec<Target>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutTargetsResponse {
    /// <p>The failed target entries.</p>
    #[serde(rename = "FailedEntries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_entries: Option<Vec<PutTargetsResultEntry>>,
    /// <p>The number of failed entries.</p>
    #[serde(rename = "FailedEntryCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_entry_count: Option<i64>,
}

/// <p>Represents a target that failed to be added to a rule.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutTargetsResultEntry {
    /// <p>The error code that indicates why the target addition failed. If the value is <code>ConcurrentModificationException</code>, too many requests were made at the same time.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The error message that explains why the target addition failed.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The ID of the target.</p>
    #[serde(rename = "TargetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RemovePermissionRequest {
    /// <p>The name of the event bus to revoke permissions for. If you omit this, the default event bus is used.</p>
    #[serde(rename = "EventBusName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    /// <p>The statement ID corresponding to the account that is no longer allowed to put events to the default event bus.</p>
    #[serde(rename = "StatementId")]
    pub statement_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RemoveTargetsRequest {
    /// <p>The name of the event bus associated with the rule.</p>
    #[serde(rename = "EventBusName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    /// <p>If this is a managed rule created by an AWS service on your behalf, you must specify <code>Force</code> as <code>True</code> to remove targets. This parameter is ignored for rules that aren't managed rules. You can check whether a rule is a managed rule by using <code>DescribeRule</code> or <code>ListRules</code> and checking the <code>ManagedBy</code> field of the response.</p>
    #[serde(rename = "Force")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    /// <p>The IDs of the targets to remove from the rule.</p>
    #[serde(rename = "Ids")]
    pub ids: Vec<String>,
    /// <p>The name of the rule.</p>
    #[serde(rename = "Rule")]
    pub rule: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RemoveTargetsResponse {
    /// <p>The failed target entries.</p>
    #[serde(rename = "FailedEntries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_entries: Option<Vec<RemoveTargetsResultEntry>>,
    /// <p>The number of failed entries.</p>
    #[serde(rename = "FailedEntryCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_entry_count: Option<i64>,
}

/// <p>Represents a target that failed to be removed from a rule.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RemoveTargetsResultEntry {
    /// <p>The error code that indicates why the target removal failed. If the value is <code>ConcurrentModificationException</code>, too many requests were made at the same time.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The error message that explains why the target removal failed.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The ID of the target.</p>
    #[serde(rename = "TargetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
}

/// <p>Contains information about a rule in Amazon EventBridge.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Rule {
    /// <p>The Amazon Resource Name (ARN) of the rule.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The description of the rule.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The event bus associated with the rule.</p>
    #[serde(rename = "EventBusName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    /// <p>The event pattern of the rule. For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/eventbridge-and-event-patterns.html">Event Patterns</a> in the <i>Amazon EventBridge User Guide</i>.</p>
    #[serde(rename = "EventPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_pattern: Option<String>,
    /// <p>If an AWS service created the rule on behalf of your account, this field displays the principal name of the service that created the rule.</p>
    #[serde(rename = "ManagedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
    /// <p>The name of the rule.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the role that is used for target invocation.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The scheduling expression: for example, <code>"cron(0 20 * * ? *)"</code> or <code>"rate(5 minutes)"</code>.</p>
    #[serde(rename = "ScheduleExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    /// <p>The state of the rule.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// <p>This parameter contains the criteria (either <code>InstanceIds</code> or a tag) used to specify which EC2 instances are to be sent the command. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunCommandParameters {
    /// <p>Currently, we support including only one <code>RunCommandTarget</code> block, which specifies either an array of <code>InstanceIds</code> or a tag.</p>
    #[serde(rename = "RunCommandTargets")]
    pub run_command_targets: Vec<RunCommandTarget>,
}

/// <p>Information about the EC2 instances that are to be sent the command, specified as key-value pairs. Each <code>RunCommandTarget</code> block can include only one key, but this key can specify multiple values.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunCommandTarget {
    /// <p>Can be either <code>tag:</code> <i>tag-key</i> or <code>InstanceIds</code>.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>If <code>Key</code> is <code>tag:</code> <i>tag-key</i>, <code>Values</code> is a list of tag values. If <code>Key</code> is <code>InstanceIds</code>, <code>Values</code> is a list of Amazon EC2 instance IDs.</p>
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}

/// <p>This structure includes the custom parameter to be used when the target is an SQS FIFO queue.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SqsParameters {
    /// <p>The FIFO message group ID to use as the target.</p>
    #[serde(rename = "MessageGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_group_id: Option<String>,
}

/// <p>A key-value pair associated with an AWS resource. In EventBridge, rules support tagging.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>A string that you can use to assign a value. The combination of tag keys and values can help you organize and categorize your resources.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The value for the specified tag key.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The ARN of the rule that you're adding tags to.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    /// <p>The list of key-value pairs to associate with the rule.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

/// <p>Targets are the resources to be invoked when a rule is triggered. For a complete list of services and resources that can be set as a target, see <a>PutTargets</a>.</p> <p>If you're setting the event bus of another account as the target and that account granted permission to your account through an organization instead of directly by the account ID, you must specify a <code>RoleArn</code> with proper permissions in the <code>Target</code> structure. For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/eventbridge-cross-account-event-delivery.html">Sending and Receiving Events Between AWS Accounts</a> in the <i>Amazon EventBridge User Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Target {
    /// <p>The Amazon Resource Name (ARN) of the target.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>If the event target is an AWS Batch job, this contains the job definition, job name, and other parameters. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/jobs.html">Jobs</a> in the <i>AWS Batch User Guide</i>.</p>
    #[serde(rename = "BatchParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_parameters: Option<BatchParameters>,
    /// <p>Contains the Amazon ECS task definition and task count to be used if the event target is an Amazon ECS task. For more information about Amazon ECS tasks, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task_defintions.html">Task Definitions </a> in the <i>Amazon EC2 Container Service Developer Guide</i>.</p>
    #[serde(rename = "EcsParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecs_parameters: Option<EcsParameters>,
    /// <p>A name for the target. Use a string that will help you identify the target. Each target associated with a rule must have an <code>Id</code> unique for that rule.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>Valid JSON text passed to the target. In this case, nothing from the event itself is passed to the target. For more information, see <a href="http://www.rfc-editor.org/rfc/rfc7159.txt">The JavaScript Object Notation (JSON) Data Interchange Format</a>.</p>
    #[serde(rename = "Input")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    /// <p>The value of the JSONPath that is used for extracting part of the matched event when passing it to the target. You must use JSON dot notation, not bracket notation. For more information about JSON paths, see <a href="http://goessner.net/articles/JsonPath/">JSONPath</a>.</p>
    #[serde(rename = "InputPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_path: Option<String>,
    /// <p>Settings to enable you to provide custom input to a target based on certain event data. You can extract one or more key-value pairs from the event and then use that data to send customized input to the target.</p>
    #[serde(rename = "InputTransformer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_transformer: Option<InputTransformer>,
    /// <p>The custom parameter that you can use to control the shard assignment when the target is a Kinesis data stream. If you don't include this parameter, the default is to use the <code>eventId</code> as the partition key.</p>
    #[serde(rename = "KinesisParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_parameters: Option<KinesisParameters>,
    /// <p>The Amazon Resource Name (ARN) of the IAM role to be used for this target when the rule is triggered. If one rule triggers multiple targets, you can use a different IAM role for each target.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>Parameters used when you are using the rule to invoke Amazon EC2 Run Command.</p>
    #[serde(rename = "RunCommandParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_command_parameters: Option<RunCommandParameters>,
    /// <p>Contains the message group ID to use when the target is a FIFO queue.</p> <p>If you specify an SQS FIFO queue as a target, the queue must have content-based deduplication enabled.</p>
    #[serde(rename = "SqsParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sqs_parameters: Option<SqsParameters>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TestEventPatternRequest {
    /// <p>The event, in JSON format, to test against the event pattern.</p>
    #[serde(rename = "Event")]
    pub event: String,
    /// <p>The event pattern. For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/eventbridge-and-event-patterns.html">Event Patterns</a> in the <i>Amazon EventBridge User Guide</i>.</p>
    #[serde(rename = "EventPattern")]
    pub event_pattern: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TestEventPatternResponse {
    /// <p>Indicates whether the event matches the event pattern.</p>
    #[serde(rename = "Result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The ARN of the rule that you're removing tags from.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    /// <p>The list of tag keys to remove from the resource.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

/// Errors returned by ActivateEventSource
#[derive(Debug, PartialEq)]
pub enum ActivateEventSourceError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>The specified state isn't a valid state for an event source.</p>
    InvalidState(String),
    /// <p>An entity that you specified doesn't exist.</p>
    ResourceNotFound(String),
}

impl ActivateEventSourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ActivateEventSourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(ActivateEventSourceError::Internal(err.msg))
                }
                "InvalidStateException" => {
                    return RusotoError::Service(ActivateEventSourceError::InvalidState(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ActivateEventSourceError::ResourceNotFound(
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
impl fmt::Display for ActivateEventSourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ActivateEventSourceError::Internal(ref cause) => write!(f, "{}", cause),
            ActivateEventSourceError::InvalidState(ref cause) => write!(f, "{}", cause),
            ActivateEventSourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ActivateEventSourceError {}
/// Errors returned by CreateEventBus
#[derive(Debug, PartialEq)]
pub enum CreateEventBusError {
    /// <p>There is concurrent modification on a resource.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>The specified state isn't a valid state for an event source.</p>
    InvalidState(String),
    /// <p>You tried to create more resources than is allowed.</p>
    LimitExceeded(String),
    /// <p>The resource that you're trying to create already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>An entity that you specified doesn't exist.</p>
    ResourceNotFound(String),
}

impl CreateEventBusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateEventBusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(CreateEventBusError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InternalException" => {
                    return RusotoError::Service(CreateEventBusError::Internal(err.msg))
                }
                "InvalidStateException" => {
                    return RusotoError::Service(CreateEventBusError::InvalidState(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateEventBusError::LimitExceeded(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateEventBusError::ResourceAlreadyExists(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateEventBusError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateEventBusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateEventBusError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            CreateEventBusError::Internal(ref cause) => write!(f, "{}", cause),
            CreateEventBusError::InvalidState(ref cause) => write!(f, "{}", cause),
            CreateEventBusError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateEventBusError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateEventBusError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateEventBusError {}
/// Errors returned by CreatePartnerEventSource
#[derive(Debug, PartialEq)]
pub enum CreatePartnerEventSourceError {
    /// <p>There is concurrent modification on a resource.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>You tried to create more resources than is allowed.</p>
    LimitExceeded(String),
    /// <p>The resource that you're trying to create already exists.</p>
    ResourceAlreadyExists(String),
}

impl CreatePartnerEventSourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreatePartnerEventSourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        CreatePartnerEventSourceError::ConcurrentModification(err.msg),
                    )
                }
                "InternalException" => {
                    return RusotoError::Service(CreatePartnerEventSourceError::Internal(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreatePartnerEventSourceError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(
                        CreatePartnerEventSourceError::ResourceAlreadyExists(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreatePartnerEventSourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreatePartnerEventSourceError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            CreatePartnerEventSourceError::Internal(ref cause) => write!(f, "{}", cause),
            CreatePartnerEventSourceError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreatePartnerEventSourceError::ResourceAlreadyExists(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreatePartnerEventSourceError {}
/// Errors returned by DeactivateEventSource
#[derive(Debug, PartialEq)]
pub enum DeactivateEventSourceError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>The specified state isn't a valid state for an event source.</p>
    InvalidState(String),
    /// <p>An entity that you specified doesn't exist.</p>
    ResourceNotFound(String),
}

impl DeactivateEventSourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeactivateEventSourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DeactivateEventSourceError::Internal(err.msg))
                }
                "InvalidStateException" => {
                    return RusotoError::Service(DeactivateEventSourceError::InvalidState(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeactivateEventSourceError::ResourceNotFound(
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
impl fmt::Display for DeactivateEventSourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeactivateEventSourceError::Internal(ref cause) => write!(f, "{}", cause),
            DeactivateEventSourceError::InvalidState(ref cause) => write!(f, "{}", cause),
            DeactivateEventSourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeactivateEventSourceError {}
/// Errors returned by DeleteEventBus
#[derive(Debug, PartialEq)]
pub enum DeleteEventBusError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
}

impl DeleteEventBusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteEventBusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DeleteEventBusError::Internal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteEventBusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteEventBusError::Internal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteEventBusError {}
/// Errors returned by DeletePartnerEventSource
#[derive(Debug, PartialEq)]
pub enum DeletePartnerEventSourceError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
}

impl DeletePartnerEventSourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeletePartnerEventSourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DeletePartnerEventSourceError::Internal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeletePartnerEventSourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeletePartnerEventSourceError::Internal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeletePartnerEventSourceError {}
/// Errors returned by DeleteRule
#[derive(Debug, PartialEq)]
pub enum DeleteRuleError {
    /// <p>There is concurrent modification on a resource.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>An AWS service created this rule on behalf of your account. That service manages it. If you see this error in response to <code>DeleteRule</code> or <code>RemoveTargets</code>, you can use the <code>Force</code> parameter in those calls to delete the rule or remove targets from the rule. You can't modify these managed rules by using <code>DisableRule</code>, <code>EnableRule</code>, <code>PutTargets</code>, <code>PutRule</code>, <code>TagResource</code>, or <code>UntagResource</code>. </p>
    ManagedRule(String),
    /// <p>An entity that you specified doesn't exist.</p>
    ResourceNotFound(String),
}

impl DeleteRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DeleteRuleError::ConcurrentModification(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(DeleteRuleError::Internal(err.msg))
                }
                "ManagedRuleException" => {
                    return RusotoError::Service(DeleteRuleError::ManagedRule(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteRuleError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteRuleError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            DeleteRuleError::Internal(ref cause) => write!(f, "{}", cause),
            DeleteRuleError::ManagedRule(ref cause) => write!(f, "{}", cause),
            DeleteRuleError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteRuleError {}
/// Errors returned by DescribeEventBus
#[derive(Debug, PartialEq)]
pub enum DescribeEventBusError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>An entity that you specified doesn't exist.</p>
    ResourceNotFound(String),
}

impl DescribeEventBusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeEventBusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DescribeEventBusError::Internal(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeEventBusError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeEventBusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeEventBusError::Internal(ref cause) => write!(f, "{}", cause),
            DescribeEventBusError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeEventBusError {}
/// Errors returned by DescribeEventSource
#[derive(Debug, PartialEq)]
pub enum DescribeEventSourceError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>An entity that you specified doesn't exist.</p>
    ResourceNotFound(String),
}

impl DescribeEventSourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeEventSourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DescribeEventSourceError::Internal(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeEventSourceError::ResourceNotFound(
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
impl fmt::Display for DescribeEventSourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeEventSourceError::Internal(ref cause) => write!(f, "{}", cause),
            DescribeEventSourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeEventSourceError {}
/// Errors returned by DescribePartnerEventSource
#[derive(Debug, PartialEq)]
pub enum DescribePartnerEventSourceError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>An entity that you specified doesn't exist.</p>
    ResourceNotFound(String),
}

impl DescribePartnerEventSourceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribePartnerEventSourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DescribePartnerEventSourceError::Internal(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribePartnerEventSourceError::ResourceNotFound(
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
impl fmt::Display for DescribePartnerEventSourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribePartnerEventSourceError::Internal(ref cause) => write!(f, "{}", cause),
            DescribePartnerEventSourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribePartnerEventSourceError {}
/// Errors returned by DescribeRule
#[derive(Debug, PartialEq)]
pub enum DescribeRuleError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>An entity that you specified doesn't exist.</p>
    ResourceNotFound(String),
}

impl DescribeRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DescribeRuleError::Internal(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeRuleError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeRuleError::Internal(ref cause) => write!(f, "{}", cause),
            DescribeRuleError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeRuleError {}
/// Errors returned by DisableRule
#[derive(Debug, PartialEq)]
pub enum DisableRuleError {
    /// <p>There is concurrent modification on a resource.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>An AWS service created this rule on behalf of your account. That service manages it. If you see this error in response to <code>DeleteRule</code> or <code>RemoveTargets</code>, you can use the <code>Force</code> parameter in those calls to delete the rule or remove targets from the rule. You can't modify these managed rules by using <code>DisableRule</code>, <code>EnableRule</code>, <code>PutTargets</code>, <code>PutRule</code>, <code>TagResource</code>, or <code>UntagResource</code>. </p>
    ManagedRule(String),
    /// <p>An entity that you specified doesn't exist.</p>
    ResourceNotFound(String),
}

impl DisableRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisableRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DisableRuleError::ConcurrentModification(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(DisableRuleError::Internal(err.msg))
                }
                "ManagedRuleException" => {
                    return RusotoError::Service(DisableRuleError::ManagedRule(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DisableRuleError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisableRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisableRuleError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            DisableRuleError::Internal(ref cause) => write!(f, "{}", cause),
            DisableRuleError::ManagedRule(ref cause) => write!(f, "{}", cause),
            DisableRuleError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisableRuleError {}
/// Errors returned by EnableRule
#[derive(Debug, PartialEq)]
pub enum EnableRuleError {
    /// <p>There is concurrent modification on a resource.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>An AWS service created this rule on behalf of your account. That service manages it. If you see this error in response to <code>DeleteRule</code> or <code>RemoveTargets</code>, you can use the <code>Force</code> parameter in those calls to delete the rule or remove targets from the rule. You can't modify these managed rules by using <code>DisableRule</code>, <code>EnableRule</code>, <code>PutTargets</code>, <code>PutRule</code>, <code>TagResource</code>, or <code>UntagResource</code>. </p>
    ManagedRule(String),
    /// <p>An entity that you specified doesn't exist.</p>
    ResourceNotFound(String),
}

impl EnableRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<EnableRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(EnableRuleError::ConcurrentModification(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(EnableRuleError::Internal(err.msg))
                }
                "ManagedRuleException" => {
                    return RusotoError::Service(EnableRuleError::ManagedRule(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(EnableRuleError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for EnableRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EnableRuleError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            EnableRuleError::Internal(ref cause) => write!(f, "{}", cause),
            EnableRuleError::ManagedRule(ref cause) => write!(f, "{}", cause),
            EnableRuleError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for EnableRuleError {}
/// Errors returned by ListEventBuses
#[derive(Debug, PartialEq)]
pub enum ListEventBusesError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
}

impl ListEventBusesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListEventBusesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(ListEventBusesError::Internal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListEventBusesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListEventBusesError::Internal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListEventBusesError {}
/// Errors returned by ListEventSources
#[derive(Debug, PartialEq)]
pub enum ListEventSourcesError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
}

impl ListEventSourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListEventSourcesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(ListEventSourcesError::Internal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListEventSourcesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListEventSourcesError::Internal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListEventSourcesError {}
/// Errors returned by ListPartnerEventSourceAccounts
#[derive(Debug, PartialEq)]
pub enum ListPartnerEventSourceAccountsError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>An entity that you specified doesn't exist.</p>
    ResourceNotFound(String),
}

impl ListPartnerEventSourceAccountsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListPartnerEventSourceAccountsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(ListPartnerEventSourceAccountsError::Internal(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        ListPartnerEventSourceAccountsError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListPartnerEventSourceAccountsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListPartnerEventSourceAccountsError::Internal(ref cause) => write!(f, "{}", cause),
            ListPartnerEventSourceAccountsError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListPartnerEventSourceAccountsError {}
/// Errors returned by ListPartnerEventSources
#[derive(Debug, PartialEq)]
pub enum ListPartnerEventSourcesError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
}

impl ListPartnerEventSourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListPartnerEventSourcesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(ListPartnerEventSourcesError::Internal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListPartnerEventSourcesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListPartnerEventSourcesError::Internal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListPartnerEventSourcesError {}
/// Errors returned by ListRuleNamesByTarget
#[derive(Debug, PartialEq)]
pub enum ListRuleNamesByTargetError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>An entity that you specified doesn't exist.</p>
    ResourceNotFound(String),
}

impl ListRuleNamesByTargetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRuleNamesByTargetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(ListRuleNamesByTargetError::Internal(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListRuleNamesByTargetError::ResourceNotFound(
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
impl fmt::Display for ListRuleNamesByTargetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListRuleNamesByTargetError::Internal(ref cause) => write!(f, "{}", cause),
            ListRuleNamesByTargetError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListRuleNamesByTargetError {}
/// Errors returned by ListRules
#[derive(Debug, PartialEq)]
pub enum ListRulesError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>An entity that you specified doesn't exist.</p>
    ResourceNotFound(String),
}

impl ListRulesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRulesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(ListRulesError::Internal(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListRulesError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListRulesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListRulesError::Internal(ref cause) => write!(f, "{}", cause),
            ListRulesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListRulesError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>An entity that you specified doesn't exist.</p>
    ResourceNotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(ListTagsForResourceError::Internal(err.msg))
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
            ListTagsForResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by ListTargetsByRule
#[derive(Debug, PartialEq)]
pub enum ListTargetsByRuleError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>An entity that you specified doesn't exist.</p>
    ResourceNotFound(String),
}

impl ListTargetsByRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTargetsByRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(ListTargetsByRuleError::Internal(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTargetsByRuleError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTargetsByRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTargetsByRuleError::Internal(ref cause) => write!(f, "{}", cause),
            ListTargetsByRuleError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTargetsByRuleError {}
/// Errors returned by PutEvents
#[derive(Debug, PartialEq)]
pub enum PutEventsError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
}

impl PutEventsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutEventsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(PutEventsError::Internal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutEventsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutEventsError::Internal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutEventsError {}
/// Errors returned by PutPartnerEvents
#[derive(Debug, PartialEq)]
pub enum PutPartnerEventsError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
}

impl PutPartnerEventsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutPartnerEventsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(PutPartnerEventsError::Internal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutPartnerEventsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutPartnerEventsError::Internal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutPartnerEventsError {}
/// Errors returned by PutPermission
#[derive(Debug, PartialEq)]
pub enum PutPermissionError {
    /// <p>There is concurrent modification on a resource.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>The event bus policy is too long. For more information, see the limits.</p>
    PolicyLengthExceeded(String),
    /// <p>An entity that you specified doesn't exist.</p>
    ResourceNotFound(String),
}

impl PutPermissionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutPermissionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(PutPermissionError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InternalException" => {
                    return RusotoError::Service(PutPermissionError::Internal(err.msg))
                }
                "PolicyLengthExceededException" => {
                    return RusotoError::Service(PutPermissionError::PolicyLengthExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(PutPermissionError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutPermissionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutPermissionError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            PutPermissionError::Internal(ref cause) => write!(f, "{}", cause),
            PutPermissionError::PolicyLengthExceeded(ref cause) => write!(f, "{}", cause),
            PutPermissionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutPermissionError {}
/// Errors returned by PutRule
#[derive(Debug, PartialEq)]
pub enum PutRuleError {
    /// <p>There is concurrent modification on a resource.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>The event pattern isn't valid.</p>
    InvalidEventPattern(String),
    /// <p>You tried to create more resources than is allowed.</p>
    LimitExceeded(String),
    /// <p>An AWS service created this rule on behalf of your account. That service manages it. If you see this error in response to <code>DeleteRule</code> or <code>RemoveTargets</code>, you can use the <code>Force</code> parameter in those calls to delete the rule or remove targets from the rule. You can't modify these managed rules by using <code>DisableRule</code>, <code>EnableRule</code>, <code>PutTargets</code>, <code>PutRule</code>, <code>TagResource</code>, or <code>UntagResource</code>. </p>
    ManagedRule(String),
    /// <p>An entity that you specified doesn't exist.</p>
    ResourceNotFound(String),
}

impl PutRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(PutRuleError::ConcurrentModification(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(PutRuleError::Internal(err.msg))
                }
                "InvalidEventPatternException" => {
                    return RusotoError::Service(PutRuleError::InvalidEventPattern(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(PutRuleError::LimitExceeded(err.msg))
                }
                "ManagedRuleException" => {
                    return RusotoError::Service(PutRuleError::ManagedRule(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(PutRuleError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutRuleError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            PutRuleError::Internal(ref cause) => write!(f, "{}", cause),
            PutRuleError::InvalidEventPattern(ref cause) => write!(f, "{}", cause),
            PutRuleError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            PutRuleError::ManagedRule(ref cause) => write!(f, "{}", cause),
            PutRuleError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutRuleError {}
/// Errors returned by PutTargets
#[derive(Debug, PartialEq)]
pub enum PutTargetsError {
    /// <p>There is concurrent modification on a resource.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>You tried to create more resources than is allowed.</p>
    LimitExceeded(String),
    /// <p>An AWS service created this rule on behalf of your account. That service manages it. If you see this error in response to <code>DeleteRule</code> or <code>RemoveTargets</code>, you can use the <code>Force</code> parameter in those calls to delete the rule or remove targets from the rule. You can't modify these managed rules by using <code>DisableRule</code>, <code>EnableRule</code>, <code>PutTargets</code>, <code>PutRule</code>, <code>TagResource</code>, or <code>UntagResource</code>. </p>
    ManagedRule(String),
    /// <p>An entity that you specified doesn't exist.</p>
    ResourceNotFound(String),
}

impl PutTargetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutTargetsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(PutTargetsError::ConcurrentModification(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(PutTargetsError::Internal(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(PutTargetsError::LimitExceeded(err.msg))
                }
                "ManagedRuleException" => {
                    return RusotoError::Service(PutTargetsError::ManagedRule(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(PutTargetsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutTargetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutTargetsError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            PutTargetsError::Internal(ref cause) => write!(f, "{}", cause),
            PutTargetsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            PutTargetsError::ManagedRule(ref cause) => write!(f, "{}", cause),
            PutTargetsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutTargetsError {}
/// Errors returned by RemovePermission
#[derive(Debug, PartialEq)]
pub enum RemovePermissionError {
    /// <p>There is concurrent modification on a resource.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>An entity that you specified doesn't exist.</p>
    ResourceNotFound(String),
}

impl RemovePermissionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RemovePermissionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(RemovePermissionError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InternalException" => {
                    return RusotoError::Service(RemovePermissionError::Internal(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(RemovePermissionError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RemovePermissionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RemovePermissionError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            RemovePermissionError::Internal(ref cause) => write!(f, "{}", cause),
            RemovePermissionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RemovePermissionError {}
/// Errors returned by RemoveTargets
#[derive(Debug, PartialEq)]
pub enum RemoveTargetsError {
    /// <p>There is concurrent modification on a resource.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>An AWS service created this rule on behalf of your account. That service manages it. If you see this error in response to <code>DeleteRule</code> or <code>RemoveTargets</code>, you can use the <code>Force</code> parameter in those calls to delete the rule or remove targets from the rule. You can't modify these managed rules by using <code>DisableRule</code>, <code>EnableRule</code>, <code>PutTargets</code>, <code>PutRule</code>, <code>TagResource</code>, or <code>UntagResource</code>. </p>
    ManagedRule(String),
    /// <p>An entity that you specified doesn't exist.</p>
    ResourceNotFound(String),
}

impl RemoveTargetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RemoveTargetsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(RemoveTargetsError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InternalException" => {
                    return RusotoError::Service(RemoveTargetsError::Internal(err.msg))
                }
                "ManagedRuleException" => {
                    return RusotoError::Service(RemoveTargetsError::ManagedRule(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(RemoveTargetsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RemoveTargetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RemoveTargetsError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            RemoveTargetsError::Internal(ref cause) => write!(f, "{}", cause),
            RemoveTargetsError::ManagedRule(ref cause) => write!(f, "{}", cause),
            RemoveTargetsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RemoveTargetsError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>There is concurrent modification on a resource.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>An AWS service created this rule on behalf of your account. That service manages it. If you see this error in response to <code>DeleteRule</code> or <code>RemoveTargets</code>, you can use the <code>Force</code> parameter in those calls to delete the rule or remove targets from the rule. You can't modify these managed rules by using <code>DisableRule</code>, <code>EnableRule</code>, <code>PutTargets</code>, <code>PutRule</code>, <code>TagResource</code>, or <code>UntagResource</code>. </p>
    ManagedRule(String),
    /// <p>An entity that you specified doesn't exist.</p>
    ResourceNotFound(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(TagResourceError::ConcurrentModification(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(TagResourceError::Internal(err.msg))
                }
                "ManagedRuleException" => {
                    return RusotoError::Service(TagResourceError::ManagedRule(err.msg))
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
            TagResourceError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            TagResourceError::Internal(ref cause) => write!(f, "{}", cause),
            TagResourceError::ManagedRule(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by TestEventPattern
#[derive(Debug, PartialEq)]
pub enum TestEventPatternError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>The event pattern isn't valid.</p>
    InvalidEventPattern(String),
}

impl TestEventPatternError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TestEventPatternError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(TestEventPatternError::Internal(err.msg))
                }
                "InvalidEventPatternException" => {
                    return RusotoError::Service(TestEventPatternError::InvalidEventPattern(
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
impl fmt::Display for TestEventPatternError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TestEventPatternError::Internal(ref cause) => write!(f, "{}", cause),
            TestEventPatternError::InvalidEventPattern(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TestEventPatternError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>There is concurrent modification on a resource.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>An AWS service created this rule on behalf of your account. That service manages it. If you see this error in response to <code>DeleteRule</code> or <code>RemoveTargets</code>, you can use the <code>Force</code> parameter in those calls to delete the rule or remove targets from the rule. You can't modify these managed rules by using <code>DisableRule</code>, <code>EnableRule</code>, <code>PutTargets</code>, <code>PutRule</code>, <code>TagResource</code>, or <code>UntagResource</code>. </p>
    ManagedRule(String),
    /// <p>An entity that you specified doesn't exist.</p>
    ResourceNotFound(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UntagResourceError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InternalException" => {
                    return RusotoError::Service(UntagResourceError::Internal(err.msg))
                }
                "ManagedRuleException" => {
                    return RusotoError::Service(UntagResourceError::ManagedRule(err.msg))
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
            UntagResourceError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            UntagResourceError::Internal(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ManagedRule(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Trait representing the capabilities of the Amazon EventBridge API. Amazon EventBridge clients implement this trait.
#[async_trait]
pub trait EventBridge {
    /// <p><p>Activates a partner event source that has been deactivated. Once activated, your matching event bus will start receiving events from the event source.</p> <note> <p>This operation is performed by AWS customers, not by SaaS partners.</p> </note></p>
    async fn activate_event_source(
        &self,
        input: ActivateEventSourceRequest,
    ) -> Result<(), RusotoError<ActivateEventSourceError>>;

    /// <p><p>Creates a new event bus within your account. This can be a custom event bus which you can use to receive events from your own custom applications and services, or it can be a partner event bus which can be matched to a partner event source.</p> <note> <p>This operation is used by AWS customers, not by SaaS partners.</p> </note></p>
    async fn create_event_bus(
        &self,
        input: CreateEventBusRequest,
    ) -> Result<CreateEventBusResponse, RusotoError<CreateEventBusError>>;

    /// <p><p>Called by an SaaS partner to create a partner event source.</p> <note> <p>This operation is not used by AWS customers.</p> </note> <p>Each partner event source can be used by one AWS account to create a matching partner event bus in that AWS account. A SaaS partner must create one partner event source for each AWS account that wants to receive those event types. </p> <p>A partner event source creates events based on resources in the SaaS partner&#39;s service or application.</p> <p>An AWS account that creates a partner event bus that matches the partner event source can use that event bus to receive events from the partner, and then process them using AWS Events rules and targets.</p> <p>Partner event source names follow this format:</p> <p> <code>aws.partner/<i>partner<em>name</i>/<i>event</em>namespace</i>/<i>event<em>name</i> </code> </p> <ul> <li> <p> <i>partner</em>name</i> is determined during partner registration and identifies the partner to AWS customers.</p> </li> <li> <p>For <i>event<em>namespace</i>, we recommend that partners use a string that identifies the AWS customer within the partner&#39;s system. This should not be the customer&#39;s AWS account ID.</p> </li> <li> <p> <i>event</em>name</i> is determined by the partner, and should uniquely identify an event-generating resource within the partner system. This should help AWS customers decide whether to create an event bus to receive these events.</p> </li> </ul></p>
    async fn create_partner_event_source(
        &self,
        input: CreatePartnerEventSourceRequest,
    ) -> Result<CreatePartnerEventSourceResponse, RusotoError<CreatePartnerEventSourceError>>;

    /// <p>An AWS customer uses this operation to temporarily stop receiving events from the specified partner event source. The matching event bus isn't deleted. </p> <p>When you deactivate a partner event source, the source goes into <code>PENDING</code> state. If it remains in <code>PENDING</code> state for more than two weeks, it's deleted.</p> <p>To activate a deactivated partner event source, use <a>ActivateEventSource</a>.</p>
    async fn deactivate_event_source(
        &self,
        input: DeactivateEventSourceRequest,
    ) -> Result<(), RusotoError<DeactivateEventSourceError>>;

    /// <p><p>Deletes the specified custom event bus or partner event bus. All rules associated with this event bus are also deleted. You can&#39;t delete your account&#39;s default event bus.</p> <note> <p>This operation is performed by AWS customers, not by SaaS partners.</p> </note></p>
    async fn delete_event_bus(
        &self,
        input: DeleteEventBusRequest,
    ) -> Result<(), RusotoError<DeleteEventBusError>>;

    /// <p>This operation is used by SaaS partners to delete a partner event source. AWS customers don't use this operation.</p> <p>When you delete an event source, the status of the corresponding partner event bus in the AWS customer account becomes <code>DELETED</code>.</p>
    async fn delete_partner_event_source(
        &self,
        input: DeletePartnerEventSourceRequest,
    ) -> Result<(), RusotoError<DeletePartnerEventSourceError>>;

    /// <p>Deletes the specified rule.</p> <p>Before you can delete the rule, you must remove all targets, using <a>RemoveTargets</a>.</p> <p>When you delete a rule, incoming events might continue to match to the deleted rule. Allow a short period of time for changes to take effect.</p> <p>Managed rules are rules created and managed by another AWS service on your behalf. These rules are created by those other AWS services to support functionality in those services. You can delete these rules using the <code>Force</code> option, but you should do so only if you're sure that the other service isn't still using that rule.</p>
    async fn delete_rule(
        &self,
        input: DeleteRuleRequest,
    ) -> Result<(), RusotoError<DeleteRuleError>>;

    /// <p>Displays details about an event bus in your account. This can include the external AWS accounts that are permitted to write events to your default event bus, and the associated policy. For custom event buses and partner event buses, it displays the name, ARN, policy, state, and creation time.</p> <p> To enable your account to receive events from other accounts on its default event bus, use <a>PutPermission</a>.</p> <p>For more information about partner event buses, see <a>CreateEventBus</a>.</p>
    async fn describe_event_bus(
        &self,
        input: DescribeEventBusRequest,
    ) -> Result<DescribeEventBusResponse, RusotoError<DescribeEventBusError>>;

    /// <p><p>This operation lists details about a partner event source that is shared with your account.</p> <note> <p>This operation is run by AWS customers, not by SaaS partners.</p> </note></p>
    async fn describe_event_source(
        &self,
        input: DescribeEventSourceRequest,
    ) -> Result<DescribeEventSourceResponse, RusotoError<DescribeEventSourceError>>;

    /// <p><p>An SaaS partner can use this operation to list details about a partner event source that they have created.</p> <note> <p>AWS customers do not use this operation. Instead, AWS customers can use <a>DescribeEventSource</a> to see details about a partner event source that is shared with them.</p> </note></p>
    async fn describe_partner_event_source(
        &self,
        input: DescribePartnerEventSourceRequest,
    ) -> Result<DescribePartnerEventSourceResponse, RusotoError<DescribePartnerEventSourceError>>;

    /// <p>Describes the specified rule.</p> <p> <code>DescribeRule</code> doesn't list the targets of a rule. To see the targets associated with a rule, use <a>ListTargetsByRule</a>.</p>
    async fn describe_rule(
        &self,
        input: DescribeRuleRequest,
    ) -> Result<DescribeRuleResponse, RusotoError<DescribeRuleError>>;

    /// <p>Disables the specified rule. A disabled rule won't match any events and won't self-trigger if it has a schedule expression.</p> <p>When you disable a rule, incoming events might continue to match to the disabled rule. Allow a short period of time for changes to take effect.</p>
    async fn disable_rule(
        &self,
        input: DisableRuleRequest,
    ) -> Result<(), RusotoError<DisableRuleError>>;

    /// <p>Enables the specified rule. If the rule doesn't exist, the operation fails.</p> <p>When you enable a rule, incoming events might not immediately start matching to a newly enabled rule. Allow a short period of time for changes to take effect.</p>
    async fn enable_rule(
        &self,
        input: EnableRuleRequest,
    ) -> Result<(), RusotoError<EnableRuleError>>;

    /// <p><p>Lists all the event buses in your account, including the default event bus, custom event buses, and partner event buses.</p> <note> <p>This operation is run by AWS customers, not by SaaS partners.</p> </note></p>
    async fn list_event_buses(
        &self,
        input: ListEventBusesRequest,
    ) -> Result<ListEventBusesResponse, RusotoError<ListEventBusesError>>;

    /// <p><p>You can use this to see all the partner event sources that have been shared with your AWS account. For more information about partner event sources, see <a>CreateEventBus</a>.</p> <note> <p>This operation is run by AWS customers, not by SaaS partners.</p> </note></p>
    async fn list_event_sources(
        &self,
        input: ListEventSourcesRequest,
    ) -> Result<ListEventSourcesResponse, RusotoError<ListEventSourcesError>>;

    /// <p><p>An SaaS partner can use this operation to display the AWS account ID that a particular partner event source name is associated with.</p> <note> <p>This operation is used by SaaS partners, not by AWS customers.</p> </note></p>
    async fn list_partner_event_source_accounts(
        &self,
        input: ListPartnerEventSourceAccountsRequest,
    ) -> Result<
        ListPartnerEventSourceAccountsResponse,
        RusotoError<ListPartnerEventSourceAccountsError>,
    >;

    /// <p><p>An SaaS partner can use this operation to list all the partner event source names that they have created.</p> <note> <p>This operation is not used by AWS customers.</p> </note></p>
    async fn list_partner_event_sources(
        &self,
        input: ListPartnerEventSourcesRequest,
    ) -> Result<ListPartnerEventSourcesResponse, RusotoError<ListPartnerEventSourcesError>>;

    /// <p>Lists the rules for the specified target. You can see which rules can invoke a specific target in your account.</p>
    async fn list_rule_names_by_target(
        &self,
        input: ListRuleNamesByTargetRequest,
    ) -> Result<ListRuleNamesByTargetResponse, RusotoError<ListRuleNamesByTargetError>>;

    /// <p>Lists your EventBridge rules. You can either list all the rules or provide a prefix to match to the rule names.</p> <p> <code>ListRules</code> doesn't list the targets of a rule. To see the targets associated with a rule, use <a>ListTargetsByRule</a>.</p>
    async fn list_rules(
        &self,
        input: ListRulesRequest,
    ) -> Result<ListRulesResponse, RusotoError<ListRulesError>>;

    /// <p>Displays the tags associated with an EventBridge resource. In EventBridge, rules can be tagged.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Lists the targets assigned to the specified rule.</p>
    async fn list_targets_by_rule(
        &self,
        input: ListTargetsByRuleRequest,
    ) -> Result<ListTargetsByRuleResponse, RusotoError<ListTargetsByRuleError>>;

    /// <p>Sends custom events to EventBridge so that they can be matched to rules. These events can be from your custom applications and services.</p>
    async fn put_events(
        &self,
        input: PutEventsRequest,
    ) -> Result<PutEventsResponse, RusotoError<PutEventsError>>;

    /// <p><p>This is used by SaaS partners to write events to a customer&#39;s partner event bus.</p> <note> <p>AWS customers do not use this operation. Instead, AWS customers can use <a>PutEvents</a> to write custom events from their own applications to an event bus.</p> </note></p>
    async fn put_partner_events(
        &self,
        input: PutPartnerEventsRequest,
    ) -> Result<PutPartnerEventsResponse, RusotoError<PutPartnerEventsError>>;

    /// <p>Running <code>PutPermission</code> permits the specified AWS account or AWS organization to put events to the specified <i>event bus</i>. Rules in your account are triggered by these events arriving to an event bus in your account. </p> <p>For another account to send events to your account, that external account must have a rule with your account's event bus as a target.</p> <p>To enable multiple AWS accounts to put events to an event bus, run <code>PutPermission</code> once for each of these accounts. Or, if all the accounts are members of the same AWS organization, you can run <code>PutPermission</code> once specifying <code>Principal</code> as "*" and specifying the AWS organization ID in <code>Condition</code>, to grant permissions to all accounts in that organization.</p> <p>If you grant permissions using an organization, then accounts in that organization must specify a <code>RoleArn</code> with proper permissions when they use <code>PutTarget</code> to add your account's event bus as a target. For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/eventbridge-cross-account-event-delivery.html">Sending and Receiving Events Between AWS Accounts</a> in the <i>Amazon EventBridge User Guide</i>.</p> <p>The permission policy on an event bus can't exceed 10 KB in size.</p>
    async fn put_permission(
        &self,
        input: PutPermissionRequest,
    ) -> Result<(), RusotoError<PutPermissionError>>;

    /// <p>Creates or updates the specified rule. Rules are enabled by default or based on value of the state. You can disable a rule using <a>DisableRule</a>.</p> <p>A single rule watches for events from a single event bus. Events generated by AWS services go to your account's default event bus. Events generated by SaaS partner services or applications go to the matching partner event bus. If you have custom applications or services, you can specify whether their events go to your default event bus or a custom event bus that you have created. For more information, see <a>CreateEventBus</a>.</p> <p>If you're updating an existing rule, the rule is replaced with what you specify in this <code>PutRule</code> command. If you omit arguments in <code>PutRule</code>, the old values for those arguments aren't kept. Instead, they're replaced with null values.</p> <p>When you create or update a rule, incoming events might not immediately start matching to new or updated rules. Allow a short period of time for changes to take effect.</p> <p>A rule must contain at least an <code>EventPattern</code> or <code>ScheduleExpression</code>. Rules with <code>EventPatterns</code> are triggered when a matching event is observed. Rules with <code>ScheduleExpressions</code> self-trigger based on the given schedule. A rule can have both an <code>EventPattern</code> and a <code>ScheduleExpression</code>, in which case the rule triggers on matching events as well as on a schedule.</p> <p>When you initially create a rule, you can optionally assign one or more tags to the rule. Tags can help you organize and categorize your resources. You can also use them to scope user permissions, by granting a user permission to access or change only rules with certain tag values. To use the <code>PutRule</code> operation and assign tags, you must have both the <code>events:PutRule</code> and <code>events:TagResource</code> permissions.</p> <p>If you are updating an existing rule, any tags you specify in the <code>PutRule</code> operation are ignored. To update the tags of an existing rule, use <a>TagResource</a> and <a>UntagResource</a>.</p> <p>Most services in AWS treat <code>:</code> or <code>/</code> as the same character in Amazon Resource Names (ARNs). However, EventBridge uses an exact match in event patterns and rules. Be sure to use the correct ARN characters when creating event patterns so that they match the ARN syntax in the event that you want to match.</p> <p>In EventBridge, you could create rules that lead to infinite loops, where a rule is fired repeatedly. For example, a rule might detect that ACLs have changed on an S3 bucket, and trigger software to change them to the desired state. If you don't write the rule carefully, the subsequent change to the ACLs fires the rule again, creating an infinite loop.</p> <p>To prevent this, write the rules so that the triggered actions don't refire the same rule. For example, your rule could fire only if ACLs are found to be in a bad state, instead of after any change. </p> <p>An infinite loop can quickly cause higher than expected charges. We recommend that you use budgeting, which alerts you when charges exceed your specified limit. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/budgets-managing-costs.html">Managing Your Costs with Budgets</a>.</p>
    async fn put_rule(
        &self,
        input: PutRuleRequest,
    ) -> Result<PutRuleResponse, RusotoError<PutRuleError>>;

    /// <p>Adds the specified targets to the specified rule, or updates the targets if they're already associated with the rule.</p> <p>Targets are the resources that are invoked when a rule is triggered.</p> <p>You can configure the following as targets in EventBridge:</p> <ul> <li> <p>EC2 instances</p> </li> <li> <p>SSM Run Command</p> </li> <li> <p>SSM Automation</p> </li> <li> <p>AWS Lambda functions</p> </li> <li> <p>Data streams in Amazon Kinesis Data Streams</p> </li> <li> <p>Data delivery streams in Amazon Kinesis Data Firehose</p> </li> <li> <p>Amazon ECS tasks</p> </li> <li> <p>AWS Step Functions state machines</p> </li> <li> <p>AWS Batch jobs</p> </li> <li> <p>AWS CodeBuild projects</p> </li> <li> <p>Pipelines in AWS CodePipeline</p> </li> <li> <p>Amazon Inspector assessment templates</p> </li> <li> <p>Amazon SNS topics</p> </li> <li> <p>Amazon SQS queues, including FIFO queues</p> </li> <li> <p>The default event bus of another AWS account</p> </li> </ul> <p>Creating rules with built-in targets is supported only on the AWS Management Console. The built-in targets are <code>EC2 CreateSnapshot API call</code>, <code>EC2 RebootInstances API call</code>, <code>EC2 StopInstances API call</code>, and <code>EC2 TerminateInstances API call</code>. </p> <p>For some target types, <code>PutTargets</code> provides target-specific parameters. If the target is a Kinesis data stream, you can optionally specify which shard the event goes to by using the <code>KinesisParameters</code> argument. To invoke a command on multiple EC2 instances with one rule, you can use the <code>RunCommandParameters</code> field.</p> <p>To be able to make API calls against the resources that you own, Amazon EventBridge needs the appropriate permissions. For AWS Lambda and Amazon SNS resources, EventBridge relies on resource-based policies. For EC2 instances, Kinesis data streams, and AWS Step Functions state machines, EventBridge relies on IAM roles that you specify in the <code>RoleARN</code> argument in <code>PutTargets</code>. For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/auth-and-access-control-eventbridge.html">Authentication and Access Control</a> in the <i>Amazon EventBridge User Guide</i>.</p> <p>If another AWS account is in the same Region and has granted you permission (using <code>PutPermission</code>), you can send events to that account. Set that account's event bus as a target of the rules in your account. To send the matched events to the other account, specify that account's event bus as the <code>Arn</code> value when you run <code>PutTargets</code>. If your account sends events to another account, your account is charged for each sent event. Each event sent to another account is charged as a custom event. The account receiving the event isn't charged. For more information, see <a href="https://aws.amazon.com/eventbridge/pricing/">Amazon EventBridge Pricing</a>.</p> <p>If you're setting an event bus in another account as the target and that account granted permission to your account through an organization instead of directly by the account ID, you must specify a <code>RoleArn</code> with proper permissions in the <code>Target</code> structure. For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/eventbridge-cross-account-event-delivery.html">Sending and Receiving Events Between AWS Accounts</a> in the <i>Amazon EventBridge User Guide</i>.</p> <p>For more information about enabling cross-account events, see <a>PutPermission</a>.</p> <p> <code>Input</code>, <code>InputPath</code>, and <code>InputTransformer</code> are mutually exclusive and optional parameters of a target. When a rule is triggered due to a matched event:</p> <ul> <li> <p>If none of the following arguments are specified for a target, the entire event is passed to the target in JSON format (unless the target is Amazon EC2 Run Command or Amazon ECS task, in which case nothing from the event is passed to the target).</p> </li> <li> <p>If <code>Input</code> is specified in the form of valid JSON, then the matched event is overridden with this constant.</p> </li> <li> <p>If <code>InputPath</code> is specified in the form of JSONPath (for example, <code>$.detail</code>), only the part of the event specified in the path is passed to the target (for example, only the detail part of the event is passed).</p> </li> <li> <p>If <code>InputTransformer</code> is specified, one or more specified JSONPaths are extracted from the event and used as values in a template that you specify as the input to the target.</p> </li> </ul> <p>When you specify <code>InputPath</code> or <code>InputTransformer</code>, you must use JSON dot notation, not bracket notation.</p> <p>When you add targets to a rule and the associated rule triggers soon after, new or updated targets might not be immediately invoked. Allow a short period of time for changes to take effect.</p> <p>This action can partially fail if too many requests are made at the same time. If that happens, <code>FailedEntryCount</code> is nonzero in the response, and each entry in <code>FailedEntries</code> provides the ID of the failed target and the error code.</p>
    async fn put_targets(
        &self,
        input: PutTargetsRequest,
    ) -> Result<PutTargetsResponse, RusotoError<PutTargetsError>>;

    /// <p>Revokes the permission of another AWS account to be able to put events to the specified event bus. Specify the account to revoke by the <code>StatementId</code> value that you associated with the account when you granted it permission with <code>PutPermission</code>. You can find the <code>StatementId</code> by using <a>DescribeEventBus</a>.</p>
    async fn remove_permission(
        &self,
        input: RemovePermissionRequest,
    ) -> Result<(), RusotoError<RemovePermissionError>>;

    /// <p>Removes the specified targets from the specified rule. When the rule is triggered, those targets are no longer be invoked.</p> <p>When you remove a target, when the associated rule triggers, removed targets might continue to be invoked. Allow a short period of time for changes to take effect.</p> <p>This action can partially fail if too many requests are made at the same time. If that happens, <code>FailedEntryCount</code> is non-zero in the response and each entry in <code>FailedEntries</code> provides the ID of the failed target and the error code.</p>
    async fn remove_targets(
        &self,
        input: RemoveTargetsRequest,
    ) -> Result<RemoveTargetsResponse, RusotoError<RemoveTargetsError>>;

    /// <p>Assigns one or more tags (key-value pairs) to the specified EventBridge resource. Tags can help you organize and categorize your resources. You can also use them to scope user permissions by granting a user permission to access or change only resources with certain tag values. In EventBridge, rules can be tagged.</p> <p>Tags don't have any semantic meaning to AWS and are interpreted strictly as strings of characters.</p> <p>You can use the <code>TagResource</code> action with a rule that already has tags. If you specify a new tag key for the rule, this tag is appended to the list of tags associated with the rule. If you specify a tag key that is already associated with the rule, the new tag value that you specify replaces the previous value for that tag.</p> <p>You can associate as many as 50 tags with a resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Tests whether the specified event pattern matches the provided event.</p> <p>Most services in AWS treat <code>:</code> or <code>/</code> as the same character in Amazon Resource Names (ARNs). However, EventBridge uses an exact match in event patterns and rules. Be sure to use the correct ARN characters when creating event patterns so that they match the ARN syntax in the event that you want to match.</p>
    async fn test_event_pattern(
        &self,
        input: TestEventPatternRequest,
    ) -> Result<TestEventPatternResponse, RusotoError<TestEventPatternError>>;

    /// <p>Removes one or more tags from the specified EventBridge resource. In EventBridge, rules can be tagged.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;
}
/// A client for the Amazon EventBridge API.
#[derive(Clone)]
pub struct EventBridgeClient {
    client: Client,
    region: region::Region,
}

impl EventBridgeClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> EventBridgeClient {
        EventBridgeClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> EventBridgeClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        EventBridgeClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> EventBridgeClient {
        EventBridgeClient { client, region }
    }
}

#[async_trait]
impl EventBridge for EventBridgeClient {
    /// <p><p>Activates a partner event source that has been deactivated. Once activated, your matching event bus will start receiving events from the event source.</p> <note> <p>This operation is performed by AWS customers, not by SaaS partners.</p> </note></p>
    async fn activate_event_source(
        &self,
        input: ActivateEventSourceRequest,
    ) -> Result<(), RusotoError<ActivateEventSourceError>> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.ActivateEventSource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            std::mem::drop(response);
            Ok(())
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ActivateEventSourceError::from_response(response))
        }
    }

    /// <p><p>Creates a new event bus within your account. This can be a custom event bus which you can use to receive events from your own custom applications and services, or it can be a partner event bus which can be matched to a partner event source.</p> <note> <p>This operation is used by AWS customers, not by SaaS partners.</p> </note></p>
    async fn create_event_bus(
        &self,
        input: CreateEventBusRequest,
    ) -> Result<CreateEventBusResponse, RusotoError<CreateEventBusError>> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.CreateEventBus");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateEventBusResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateEventBusError::from_response(response))
        }
    }

    /// <p><p>Called by an SaaS partner to create a partner event source.</p> <note> <p>This operation is not used by AWS customers.</p> </note> <p>Each partner event source can be used by one AWS account to create a matching partner event bus in that AWS account. A SaaS partner must create one partner event source for each AWS account that wants to receive those event types. </p> <p>A partner event source creates events based on resources in the SaaS partner&#39;s service or application.</p> <p>An AWS account that creates a partner event bus that matches the partner event source can use that event bus to receive events from the partner, and then process them using AWS Events rules and targets.</p> <p>Partner event source names follow this format:</p> <p> <code>aws.partner/<i>partner<em>name</i>/<i>event</em>namespace</i>/<i>event<em>name</i> </code> </p> <ul> <li> <p> <i>partner</em>name</i> is determined during partner registration and identifies the partner to AWS customers.</p> </li> <li> <p>For <i>event<em>namespace</i>, we recommend that partners use a string that identifies the AWS customer within the partner&#39;s system. This should not be the customer&#39;s AWS account ID.</p> </li> <li> <p> <i>event</em>name</i> is determined by the partner, and should uniquely identify an event-generating resource within the partner system. This should help AWS customers decide whether to create an event bus to receive these events.</p> </li> </ul></p>
    async fn create_partner_event_source(
        &self,
        input: CreatePartnerEventSourceRequest,
    ) -> Result<CreatePartnerEventSourceResponse, RusotoError<CreatePartnerEventSourceError>> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.CreatePartnerEventSource");
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
                .deserialize::<CreatePartnerEventSourceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreatePartnerEventSourceError::from_response(response))
        }
    }

    /// <p>An AWS customer uses this operation to temporarily stop receiving events from the specified partner event source. The matching event bus isn't deleted. </p> <p>When you deactivate a partner event source, the source goes into <code>PENDING</code> state. If it remains in <code>PENDING</code> state for more than two weeks, it's deleted.</p> <p>To activate a deactivated partner event source, use <a>ActivateEventSource</a>.</p>
    async fn deactivate_event_source(
        &self,
        input: DeactivateEventSourceRequest,
    ) -> Result<(), RusotoError<DeactivateEventSourceError>> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.DeactivateEventSource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            std::mem::drop(response);
            Ok(())
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeactivateEventSourceError::from_response(response))
        }
    }

    /// <p><p>Deletes the specified custom event bus or partner event bus. All rules associated with this event bus are also deleted. You can&#39;t delete your account&#39;s default event bus.</p> <note> <p>This operation is performed by AWS customers, not by SaaS partners.</p> </note></p>
    async fn delete_event_bus(
        &self,
        input: DeleteEventBusRequest,
    ) -> Result<(), RusotoError<DeleteEventBusError>> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.DeleteEventBus");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            std::mem::drop(response);
            Ok(())
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteEventBusError::from_response(response))
        }
    }

    /// <p>This operation is used by SaaS partners to delete a partner event source. AWS customers don't use this operation.</p> <p>When you delete an event source, the status of the corresponding partner event bus in the AWS customer account becomes <code>DELETED</code>.</p>
    async fn delete_partner_event_source(
        &self,
        input: DeletePartnerEventSourceRequest,
    ) -> Result<(), RusotoError<DeletePartnerEventSourceError>> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.DeletePartnerEventSource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            std::mem::drop(response);
            Ok(())
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeletePartnerEventSourceError::from_response(response))
        }
    }

    /// <p>Deletes the specified rule.</p> <p>Before you can delete the rule, you must remove all targets, using <a>RemoveTargets</a>.</p> <p>When you delete a rule, incoming events might continue to match to the deleted rule. Allow a short period of time for changes to take effect.</p> <p>Managed rules are rules created and managed by another AWS service on your behalf. These rules are created by those other AWS services to support functionality in those services. You can delete these rules using the <code>Force</code> option, but you should do so only if you're sure that the other service isn't still using that rule.</p>
    async fn delete_rule(
        &self,
        input: DeleteRuleRequest,
    ) -> Result<(), RusotoError<DeleteRuleError>> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.DeleteRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            std::mem::drop(response);
            Ok(())
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteRuleError::from_response(response))
        }
    }

    /// <p>Displays details about an event bus in your account. This can include the external AWS accounts that are permitted to write events to your default event bus, and the associated policy. For custom event buses and partner event buses, it displays the name, ARN, policy, state, and creation time.</p> <p> To enable your account to receive events from other accounts on its default event bus, use <a>PutPermission</a>.</p> <p>For more information about partner event buses, see <a>CreateEventBus</a>.</p>
    async fn describe_event_bus(
        &self,
        input: DescribeEventBusRequest,
    ) -> Result<DescribeEventBusResponse, RusotoError<DescribeEventBusError>> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.DescribeEventBus");
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
                .deserialize::<DescribeEventBusResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeEventBusError::from_response(response))
        }
    }

    /// <p><p>This operation lists details about a partner event source that is shared with your account.</p> <note> <p>This operation is run by AWS customers, not by SaaS partners.</p> </note></p>
    async fn describe_event_source(
        &self,
        input: DescribeEventSourceRequest,
    ) -> Result<DescribeEventSourceResponse, RusotoError<DescribeEventSourceError>> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.DescribeEventSource");
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
                .deserialize::<DescribeEventSourceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeEventSourceError::from_response(response))
        }
    }

    /// <p><p>An SaaS partner can use this operation to list details about a partner event source that they have created.</p> <note> <p>AWS customers do not use this operation. Instead, AWS customers can use <a>DescribeEventSource</a> to see details about a partner event source that is shared with them.</p> </note></p>
    async fn describe_partner_event_source(
        &self,
        input: DescribePartnerEventSourceRequest,
    ) -> Result<DescribePartnerEventSourceResponse, RusotoError<DescribePartnerEventSourceError>>
    {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.DescribePartnerEventSource");
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
                .deserialize::<DescribePartnerEventSourceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribePartnerEventSourceError::from_response(response))
        }
    }

    /// <p>Describes the specified rule.</p> <p> <code>DescribeRule</code> doesn't list the targets of a rule. To see the targets associated with a rule, use <a>ListTargetsByRule</a>.</p>
    async fn describe_rule(
        &self,
        input: DescribeRuleRequest,
    ) -> Result<DescribeRuleResponse, RusotoError<DescribeRuleError>> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.DescribeRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeRuleResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeRuleError::from_response(response))
        }
    }

    /// <p>Disables the specified rule. A disabled rule won't match any events and won't self-trigger if it has a schedule expression.</p> <p>When you disable a rule, incoming events might continue to match to the disabled rule. Allow a short period of time for changes to take effect.</p>
    async fn disable_rule(
        &self,
        input: DisableRuleRequest,
    ) -> Result<(), RusotoError<DisableRuleError>> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.DisableRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            std::mem::drop(response);
            Ok(())
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DisableRuleError::from_response(response))
        }
    }

    /// <p>Enables the specified rule. If the rule doesn't exist, the operation fails.</p> <p>When you enable a rule, incoming events might not immediately start matching to a newly enabled rule. Allow a short period of time for changes to take effect.</p>
    async fn enable_rule(
        &self,
        input: EnableRuleRequest,
    ) -> Result<(), RusotoError<EnableRuleError>> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.EnableRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            std::mem::drop(response);
            Ok(())
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(EnableRuleError::from_response(response))
        }
    }

    /// <p><p>Lists all the event buses in your account, including the default event bus, custom event buses, and partner event buses.</p> <note> <p>This operation is run by AWS customers, not by SaaS partners.</p> </note></p>
    async fn list_event_buses(
        &self,
        input: ListEventBusesRequest,
    ) -> Result<ListEventBusesResponse, RusotoError<ListEventBusesError>> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.ListEventBuses");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListEventBusesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListEventBusesError::from_response(response))
        }
    }

    /// <p><p>You can use this to see all the partner event sources that have been shared with your AWS account. For more information about partner event sources, see <a>CreateEventBus</a>.</p> <note> <p>This operation is run by AWS customers, not by SaaS partners.</p> </note></p>
    async fn list_event_sources(
        &self,
        input: ListEventSourcesRequest,
    ) -> Result<ListEventSourcesResponse, RusotoError<ListEventSourcesError>> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.ListEventSources");
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
                .deserialize::<ListEventSourcesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListEventSourcesError::from_response(response))
        }
    }

    /// <p><p>An SaaS partner can use this operation to display the AWS account ID that a particular partner event source name is associated with.</p> <note> <p>This operation is used by SaaS partners, not by AWS customers.</p> </note></p>
    async fn list_partner_event_source_accounts(
        &self,
        input: ListPartnerEventSourceAccountsRequest,
    ) -> Result<
        ListPartnerEventSourceAccountsResponse,
        RusotoError<ListPartnerEventSourceAccountsError>,
    > {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.ListPartnerEventSourceAccounts");
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
                .deserialize::<ListPartnerEventSourceAccountsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListPartnerEventSourceAccountsError::from_response(response))
        }
    }

    /// <p><p>An SaaS partner can use this operation to list all the partner event source names that they have created.</p> <note> <p>This operation is not used by AWS customers.</p> </note></p>
    async fn list_partner_event_sources(
        &self,
        input: ListPartnerEventSourcesRequest,
    ) -> Result<ListPartnerEventSourcesResponse, RusotoError<ListPartnerEventSourcesError>> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.ListPartnerEventSources");
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
                .deserialize::<ListPartnerEventSourcesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListPartnerEventSourcesError::from_response(response))
        }
    }

    /// <p>Lists the rules for the specified target. You can see which rules can invoke a specific target in your account.</p>
    async fn list_rule_names_by_target(
        &self,
        input: ListRuleNamesByTargetRequest,
    ) -> Result<ListRuleNamesByTargetResponse, RusotoError<ListRuleNamesByTargetError>> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.ListRuleNamesByTarget");
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
                .deserialize::<ListRuleNamesByTargetResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListRuleNamesByTargetError::from_response(response))
        }
    }

    /// <p>Lists your EventBridge rules. You can either list all the rules or provide a prefix to match to the rule names.</p> <p> <code>ListRules</code> doesn't list the targets of a rule. To see the targets associated with a rule, use <a>ListTargetsByRule</a>.</p>
    async fn list_rules(
        &self,
        input: ListRulesRequest,
    ) -> Result<ListRulesResponse, RusotoError<ListRulesError>> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.ListRules");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListRulesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListRulesError::from_response(response))
        }
    }

    /// <p>Displays the tags associated with an EventBridge resource. In EventBridge, rules can be tagged.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.ListTagsForResource");
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

    /// <p>Lists the targets assigned to the specified rule.</p>
    async fn list_targets_by_rule(
        &self,
        input: ListTargetsByRuleRequest,
    ) -> Result<ListTargetsByRuleResponse, RusotoError<ListTargetsByRuleError>> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.ListTargetsByRule");
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
                .deserialize::<ListTargetsByRuleResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListTargetsByRuleError::from_response(response))
        }
    }

    /// <p>Sends custom events to EventBridge so that they can be matched to rules. These events can be from your custom applications and services.</p>
    async fn put_events(
        &self,
        input: PutEventsRequest,
    ) -> Result<PutEventsResponse, RusotoError<PutEventsError>> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.PutEvents");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<PutEventsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutEventsError::from_response(response))
        }
    }

    /// <p><p>This is used by SaaS partners to write events to a customer&#39;s partner event bus.</p> <note> <p>AWS customers do not use this operation. Instead, AWS customers can use <a>PutEvents</a> to write custom events from their own applications to an event bus.</p> </note></p>
    async fn put_partner_events(
        &self,
        input: PutPartnerEventsRequest,
    ) -> Result<PutPartnerEventsResponse, RusotoError<PutPartnerEventsError>> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.PutPartnerEvents");
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
                .deserialize::<PutPartnerEventsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutPartnerEventsError::from_response(response))
        }
    }

    /// <p>Running <code>PutPermission</code> permits the specified AWS account or AWS organization to put events to the specified <i>event bus</i>. Rules in your account are triggered by these events arriving to an event bus in your account. </p> <p>For another account to send events to your account, that external account must have a rule with your account's event bus as a target.</p> <p>To enable multiple AWS accounts to put events to an event bus, run <code>PutPermission</code> once for each of these accounts. Or, if all the accounts are members of the same AWS organization, you can run <code>PutPermission</code> once specifying <code>Principal</code> as "*" and specifying the AWS organization ID in <code>Condition</code>, to grant permissions to all accounts in that organization.</p> <p>If you grant permissions using an organization, then accounts in that organization must specify a <code>RoleArn</code> with proper permissions when they use <code>PutTarget</code> to add your account's event bus as a target. For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/eventbridge-cross-account-event-delivery.html">Sending and Receiving Events Between AWS Accounts</a> in the <i>Amazon EventBridge User Guide</i>.</p> <p>The permission policy on an event bus can't exceed 10 KB in size.</p>
    async fn put_permission(
        &self,
        input: PutPermissionRequest,
    ) -> Result<(), RusotoError<PutPermissionError>> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.PutPermission");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            std::mem::drop(response);
            Ok(())
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutPermissionError::from_response(response))
        }
    }

    /// <p>Creates or updates the specified rule. Rules are enabled by default or based on value of the state. You can disable a rule using <a>DisableRule</a>.</p> <p>A single rule watches for events from a single event bus. Events generated by AWS services go to your account's default event bus. Events generated by SaaS partner services or applications go to the matching partner event bus. If you have custom applications or services, you can specify whether their events go to your default event bus or a custom event bus that you have created. For more information, see <a>CreateEventBus</a>.</p> <p>If you're updating an existing rule, the rule is replaced with what you specify in this <code>PutRule</code> command. If you omit arguments in <code>PutRule</code>, the old values for those arguments aren't kept. Instead, they're replaced with null values.</p> <p>When you create or update a rule, incoming events might not immediately start matching to new or updated rules. Allow a short period of time for changes to take effect.</p> <p>A rule must contain at least an <code>EventPattern</code> or <code>ScheduleExpression</code>. Rules with <code>EventPatterns</code> are triggered when a matching event is observed. Rules with <code>ScheduleExpressions</code> self-trigger based on the given schedule. A rule can have both an <code>EventPattern</code> and a <code>ScheduleExpression</code>, in which case the rule triggers on matching events as well as on a schedule.</p> <p>When you initially create a rule, you can optionally assign one or more tags to the rule. Tags can help you organize and categorize your resources. You can also use them to scope user permissions, by granting a user permission to access or change only rules with certain tag values. To use the <code>PutRule</code> operation and assign tags, you must have both the <code>events:PutRule</code> and <code>events:TagResource</code> permissions.</p> <p>If you are updating an existing rule, any tags you specify in the <code>PutRule</code> operation are ignored. To update the tags of an existing rule, use <a>TagResource</a> and <a>UntagResource</a>.</p> <p>Most services in AWS treat <code>:</code> or <code>/</code> as the same character in Amazon Resource Names (ARNs). However, EventBridge uses an exact match in event patterns and rules. Be sure to use the correct ARN characters when creating event patterns so that they match the ARN syntax in the event that you want to match.</p> <p>In EventBridge, you could create rules that lead to infinite loops, where a rule is fired repeatedly. For example, a rule might detect that ACLs have changed on an S3 bucket, and trigger software to change them to the desired state. If you don't write the rule carefully, the subsequent change to the ACLs fires the rule again, creating an infinite loop.</p> <p>To prevent this, write the rules so that the triggered actions don't refire the same rule. For example, your rule could fire only if ACLs are found to be in a bad state, instead of after any change. </p> <p>An infinite loop can quickly cause higher than expected charges. We recommend that you use budgeting, which alerts you when charges exceed your specified limit. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/budgets-managing-costs.html">Managing Your Costs with Budgets</a>.</p>
    async fn put_rule(
        &self,
        input: PutRuleRequest,
    ) -> Result<PutRuleResponse, RusotoError<PutRuleError>> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.PutRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<PutRuleResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutRuleError::from_response(response))
        }
    }

    /// <p>Adds the specified targets to the specified rule, or updates the targets if they're already associated with the rule.</p> <p>Targets are the resources that are invoked when a rule is triggered.</p> <p>You can configure the following as targets in EventBridge:</p> <ul> <li> <p>EC2 instances</p> </li> <li> <p>SSM Run Command</p> </li> <li> <p>SSM Automation</p> </li> <li> <p>AWS Lambda functions</p> </li> <li> <p>Data streams in Amazon Kinesis Data Streams</p> </li> <li> <p>Data delivery streams in Amazon Kinesis Data Firehose</p> </li> <li> <p>Amazon ECS tasks</p> </li> <li> <p>AWS Step Functions state machines</p> </li> <li> <p>AWS Batch jobs</p> </li> <li> <p>AWS CodeBuild projects</p> </li> <li> <p>Pipelines in AWS CodePipeline</p> </li> <li> <p>Amazon Inspector assessment templates</p> </li> <li> <p>Amazon SNS topics</p> </li> <li> <p>Amazon SQS queues, including FIFO queues</p> </li> <li> <p>The default event bus of another AWS account</p> </li> </ul> <p>Creating rules with built-in targets is supported only on the AWS Management Console. The built-in targets are <code>EC2 CreateSnapshot API call</code>, <code>EC2 RebootInstances API call</code>, <code>EC2 StopInstances API call</code>, and <code>EC2 TerminateInstances API call</code>. </p> <p>For some target types, <code>PutTargets</code> provides target-specific parameters. If the target is a Kinesis data stream, you can optionally specify which shard the event goes to by using the <code>KinesisParameters</code> argument. To invoke a command on multiple EC2 instances with one rule, you can use the <code>RunCommandParameters</code> field.</p> <p>To be able to make API calls against the resources that you own, Amazon EventBridge needs the appropriate permissions. For AWS Lambda and Amazon SNS resources, EventBridge relies on resource-based policies. For EC2 instances, Kinesis data streams, and AWS Step Functions state machines, EventBridge relies on IAM roles that you specify in the <code>RoleARN</code> argument in <code>PutTargets</code>. For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/auth-and-access-control-eventbridge.html">Authentication and Access Control</a> in the <i>Amazon EventBridge User Guide</i>.</p> <p>If another AWS account is in the same Region and has granted you permission (using <code>PutPermission</code>), you can send events to that account. Set that account's event bus as a target of the rules in your account. To send the matched events to the other account, specify that account's event bus as the <code>Arn</code> value when you run <code>PutTargets</code>. If your account sends events to another account, your account is charged for each sent event. Each event sent to another account is charged as a custom event. The account receiving the event isn't charged. For more information, see <a href="https://aws.amazon.com/eventbridge/pricing/">Amazon EventBridge Pricing</a>.</p> <p>If you're setting an event bus in another account as the target and that account granted permission to your account through an organization instead of directly by the account ID, you must specify a <code>RoleArn</code> with proper permissions in the <code>Target</code> structure. For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/eventbridge-cross-account-event-delivery.html">Sending and Receiving Events Between AWS Accounts</a> in the <i>Amazon EventBridge User Guide</i>.</p> <p>For more information about enabling cross-account events, see <a>PutPermission</a>.</p> <p> <code>Input</code>, <code>InputPath</code>, and <code>InputTransformer</code> are mutually exclusive and optional parameters of a target. When a rule is triggered due to a matched event:</p> <ul> <li> <p>If none of the following arguments are specified for a target, the entire event is passed to the target in JSON format (unless the target is Amazon EC2 Run Command or Amazon ECS task, in which case nothing from the event is passed to the target).</p> </li> <li> <p>If <code>Input</code> is specified in the form of valid JSON, then the matched event is overridden with this constant.</p> </li> <li> <p>If <code>InputPath</code> is specified in the form of JSONPath (for example, <code>$.detail</code>), only the part of the event specified in the path is passed to the target (for example, only the detail part of the event is passed).</p> </li> <li> <p>If <code>InputTransformer</code> is specified, one or more specified JSONPaths are extracted from the event and used as values in a template that you specify as the input to the target.</p> </li> </ul> <p>When you specify <code>InputPath</code> or <code>InputTransformer</code>, you must use JSON dot notation, not bracket notation.</p> <p>When you add targets to a rule and the associated rule triggers soon after, new or updated targets might not be immediately invoked. Allow a short period of time for changes to take effect.</p> <p>This action can partially fail if too many requests are made at the same time. If that happens, <code>FailedEntryCount</code> is nonzero in the response, and each entry in <code>FailedEntries</code> provides the ID of the failed target and the error code.</p>
    async fn put_targets(
        &self,
        input: PutTargetsRequest,
    ) -> Result<PutTargetsResponse, RusotoError<PutTargetsError>> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.PutTargets");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<PutTargetsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutTargetsError::from_response(response))
        }
    }

    /// <p>Revokes the permission of another AWS account to be able to put events to the specified event bus. Specify the account to revoke by the <code>StatementId</code> value that you associated with the account when you granted it permission with <code>PutPermission</code>. You can find the <code>StatementId</code> by using <a>DescribeEventBus</a>.</p>
    async fn remove_permission(
        &self,
        input: RemovePermissionRequest,
    ) -> Result<(), RusotoError<RemovePermissionError>> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.RemovePermission");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            std::mem::drop(response);
            Ok(())
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(RemovePermissionError::from_response(response))
        }
    }

    /// <p>Removes the specified targets from the specified rule. When the rule is triggered, those targets are no longer be invoked.</p> <p>When you remove a target, when the associated rule triggers, removed targets might continue to be invoked. Allow a short period of time for changes to take effect.</p> <p>This action can partially fail if too many requests are made at the same time. If that happens, <code>FailedEntryCount</code> is non-zero in the response and each entry in <code>FailedEntries</code> provides the ID of the failed target and the error code.</p>
    async fn remove_targets(
        &self,
        input: RemoveTargetsRequest,
    ) -> Result<RemoveTargetsResponse, RusotoError<RemoveTargetsError>> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.RemoveTargets");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<RemoveTargetsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(RemoveTargetsError::from_response(response))
        }
    }

    /// <p>Assigns one or more tags (key-value pairs) to the specified EventBridge resource. Tags can help you organize and categorize your resources. You can also use them to scope user permissions by granting a user permission to access or change only resources with certain tag values. In EventBridge, rules can be tagged.</p> <p>Tags don't have any semantic meaning to AWS and are interpreted strictly as strings of characters.</p> <p>You can use the <code>TagResource</code> action with a rule that already has tags. If you specify a new tag key for the rule, this tag is appended to the list of tags associated with the rule. If you specify a tag key that is already associated with the rule, the new tag value that you specify replaces the previous value for that tag.</p> <p>You can associate as many as 50 tags with a resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.TagResource");
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

    /// <p>Tests whether the specified event pattern matches the provided event.</p> <p>Most services in AWS treat <code>:</code> or <code>/</code> as the same character in Amazon Resource Names (ARNs). However, EventBridge uses an exact match in event patterns and rules. Be sure to use the correct ARN characters when creating event patterns so that they match the ARN syntax in the event that you want to match.</p>
    async fn test_event_pattern(
        &self,
        input: TestEventPatternRequest,
    ) -> Result<TestEventPatternResponse, RusotoError<TestEventPatternError>> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.TestEventPattern");
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
                .deserialize::<TestEventPatternResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(TestEventPatternError::from_response(response))
        }
    }

    /// <p>Removes one or more tags from the specified EventBridge resource. In EventBridge, rules can be tagged.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.UntagResource");
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
}
