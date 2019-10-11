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

use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
#[allow(warnings)]
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};

use futures::{FutureExt, TryFutureExt};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde::{Deserialize, Serialize};
use serde_json;
/// <p>This structure specifies the VPC subnets and security groups for the task, and whether a public IP address is to be used. This structure is relevant only for ECS tasks that use the <code>awsvpc</code> network mode.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsVpcConfiguration {
    /// <p>Specifies whether the task's elastic network interface receives a public IP address. You can specify <code>ENABLED</code> only when <code>LaunchType</code> in <code>EcsParameters</code> is set to <code>FARGATE</code>.</p>
    #[serde(rename = "AssignPublicIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assign_public_ip: Option<String>,
    /// <p>Specifies the security groups associated with the task. These security groups must all be in the same VPC. You can specify as many as five security groups. If you do not specify a security group, the default security group for the VPC is used.</p>
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
    /// <p>The retry strategy to use for failed jobs, if the target is an AWS Batch job. The retry strategy is the number of times to retry the failed job execution. Valid values are 1–10. When you specify a retry strategy here, it overrides the retry strategy defined in the job definition.</p>
    #[serde(rename = "RetryStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_strategy: Option<BatchRetryStrategy>,
}

/// <p>The retry strategy to use for failed jobs, if the target is an AWS Batch job. If you specify a retry strategy here, it overrides the retry strategy defined in the job definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BatchRetryStrategy {
    /// <p>The number of times to attempt to retry, if the job fails. Valid values are 1–10.</p>
    #[serde(rename = "Attempts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempts: Option<i64>,
}

/// <p>A JSON string which you can use to limit the event bus permissions you are granting to only accounts that fulfill the condition. Currently, the only supported condition is membership in a certain AWS organization. The string must contain <code>Type</code>, <code>Key</code>, and <code>Value</code> fields. The <code>Value</code> field specifies the ID of the AWS organization. Following is an example value for <code>Condition</code>:</p> <p> <code>'{"Type" : "StringEquals", "Key": "aws:PrincipalOrgID", "Value": "o-1234567890"}'</code> </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Condition {
    /// <p>Specifies the key for the condition. Currently the only supported key is <code>aws:PrincipalOrgID</code>.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>Specifies the type of condition. Currently the only supported value is <code>StringEquals</code>.</p>
    #[serde(rename = "Type")]
    pub type_: String,
    /// <p>Specifies the value for the key. Currently, this must be the ID of the organization.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteRuleRequest {
    /// <p>If this is a managed rule, created by an AWS service on your behalf, you must specify <code>Force</code> as <code>True</code> to delete the rule. This parameter is ignored for rules that are not managed rules. You can check whether a rule is a managed rule by using <code>DescribeRule</code> or <code>ListRules</code> and checking the <code>ManagedBy</code> field of the response.</p>
    #[serde(rename = "Force")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    /// <p>The name of the rule.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeEventBusRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
pub struct DescribeRuleRequest {
    /// <p>The name of the rule.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeRuleResponse {
    /// <p>The Amazon Resource Name (ARN) of the rule.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The description of the rule.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The event pattern. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/events/CloudWatchEventsandEventPatterns.html">Events and Event Patterns</a> in the <i>Amazon CloudWatch Events User Guide</i>.</p>
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
    /// <p>The scheduling expression. For example, "cron(0 20 * * ? *)", "rate(5 minutes)".</p>
    #[serde(rename = "ScheduleExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    /// <p>Specifies whether the rule is enabled or disabled.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisableRuleRequest {
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
    /// <p>Use this structure if the ECS task uses the <code>awsvpc</code> network mode. This structure specifies the VPC subnets and security groups associated with the task, and whether a public IP address is to be used. This structure is required if <code>LaunchType</code> is <code>FARGATE</code> because the <code>awsvpc</code> mode is required for Fargate tasks.</p> <p>If you specify <code>NetworkConfiguration</code> when the target ECS task does not use the <code>awsvpc</code> network mode, the task fails.</p>
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
pub struct EnableRuleRequest {
    /// <p>The name of the rule.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// <p>Contains the parameters needed for you to provide custom input to a target based on one or more pieces of data extracted from the event.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputTransformer {
    /// <p>Map of JSON paths to be extracted from the event. You can then insert these in the template in <code>InputTemplate</code> to produce the output you want to be sent to the target.</p> <p> <code>InputPathsMap</code> is an array key-value pairs, where each value is a valid JSON path. You can have as many as 10 key-value pairs. You must use JSON dot notation, not bracket notation.</p> <p>The keys cannot start with "AWS." </p>
    #[serde(rename = "InputPathsMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_paths_map: Option<::std::collections::HashMap<String, String>>,
    /// <p>Input template where you specify placeholders that will be filled with the values of the keys from <code>InputPathsMap</code> to customize the data sent to the target. Enclose each <code>InputPathsMaps</code> value in brackets: &lt;<i>value</i>&gt; The InputTemplate must be valid JSON.</p> <p>If <code>InputTemplate</code> is a JSON object (surrounded by curly braces), the following restrictions apply:</p> <ul> <li> <p>The placeholder cannot be used as an object key.</p> </li> <li> <p>Object values cannot include quote marks.</p> </li> </ul> <p>The following example shows the syntax for using <code>InputPathsMap</code> and <code>InputTemplate</code>.</p> <p> <code> "InputTransformer":</code> </p> <p> <code>{</code> </p> <p> <code>"InputPathsMap": {"instance": "$.detail.instance","status": "$.detail.status"},</code> </p> <p> <code>"InputTemplate": "&lt;instance&gt; is in state &lt;status&gt;"</code> </p> <p> <code>}</code> </p> <p>To have the <code>InputTemplate</code> include quote marks within a JSON string, escape each quote marks with a slash, as in the following example:</p> <p> <code> "InputTransformer":</code> </p> <p> <code>{</code> </p> <p> <code>"InputPathsMap": {"instance": "$.detail.instance","status": "$.detail.status"},</code> </p> <p> <code>"InputTemplate": "&lt;instance&gt; is in state \"&lt;status&gt;\""</code> </p> <p> <code>}</code> </p>
    #[serde(rename = "InputTemplate")]
    pub input_template: String,
}

/// <p>This object enables you to specify a JSON path to extract from the event and use as the partition key for the Amazon Kinesis data stream, so that you can control the shard to which the event goes. If you do not include this parameter, the default is to use the <code>eventId</code> as the partition key.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KinesisParameters {
    /// <p>The JSON path to be extracted from the event and used as the partition key. For more information, see <a href="https://docs.aws.amazon.com/streams/latest/dev/key-concepts.html#partition-key">Amazon Kinesis Streams Key Concepts</a> in the <i>Amazon Kinesis Streams Developer Guide</i>.</p>
    #[serde(rename = "PartitionKeyPath")]
    pub partition_key_path: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListRuleNamesByTargetRequest {
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
#[cfg_attr(test, derive(Serialize))]
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
pub struct ListRulesRequest {
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
#[cfg_attr(test, derive(Serialize))]
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
pub struct ListTagsForResourceRequest {
    /// <p>The ARN of the CloudWatch Events rule for which you want to view tags.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>The list of tag keys and values associated with the rule you specified</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTargetsByRuleRequest {
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
#[cfg_attr(test, derive(Serialize))]
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
    /// <p>Use this structure to specify the VPC subnets and security groups for the task, and whether a public IP address is to be used. This structure is relevant only for ECS tasks that use the <code>awsvpc</code> network mode.</p>
    #[serde(rename = "awsvpcConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub awsvpc_configuration: Option<AwsVpcConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutEventsRequest {
    /// <p>The entry that defines an event in your system. You can specify several parameters for the entry such as the source and type of the event, resources associated with the event, and so on.</p>
    #[serde(rename = "Entries")]
    pub entries: Vec<PutEventsRequestEntry>,
}

/// <p>Represents an event to be submitted.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutEventsRequestEntry {
    /// <p>A valid JSON string. There is no other schema imposed. The JSON string may contain fields and nested subobjects.</p>
    #[serde(rename = "Detail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// <p>Free-form string used to decide what fields to expect in the event detail.</p>
    #[serde(rename = "DetailType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_type: Option<String>,
    /// <p>AWS resources, identified by Amazon Resource Name (ARN), which the event primarily concerns. Any number, including zero, may be present.</p>
    #[serde(rename = "Resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<String>>,
    /// <p>The source of the event. This field is required.</p>
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// <p>The time stamp of the event, per <a href="https://www.rfc-editor.org/rfc/rfc3339.txt">RFC3339</a>. If no time stamp is provided, the time stamp of the <a>PutEvents</a> call is used.</p>
    #[serde(rename = "Time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
pub struct PutPermissionRequest {
    /// <p>The action that you are enabling the other account to perform. Currently, this must be <code>events:PutEvents</code>.</p>
    #[serde(rename = "Action")]
    pub action: String,
    /// <p>This parameter enables you to limit the permission to accounts that fulfill a certain condition, such as being a member of a certain AWS organization. For more information about AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_introduction.html">What Is AWS Organizations</a> in the <i>AWS Organizations User Guide</i>.</p> <p>If you specify <code>Condition</code> with an AWS organization ID, and specify "*" as the value for <code>Principal</code>, you grant permission to all the accounts in the named organization.</p> <p>The <code>Condition</code> is a JSON string which must contain <code>Type</code>, <code>Key</code>, and <code>Value</code> fields.</p>
    #[serde(rename = "Condition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<Condition>,
    /// <p>The 12-digit AWS account ID that you are permitting to put events to your default event bus. Specify "*" to permit any account to put events to your default event bus.</p> <p>If you specify "*" without specifying <code>Condition</code>, avoid creating rules that may match undesirable events. To create more secure rules, make sure that the event pattern for each rule contains an <code>account</code> field with a specific account ID from which to receive events. Rules with an account field do not match any events sent from other accounts.</p>
    #[serde(rename = "Principal")]
    pub principal: String,
    /// <p>An identifier string for the external account that you are granting permissions to. If you later want to revoke the permission for this external account, specify this <code>StatementId</code> when you run <a>RemovePermission</a>.</p>
    #[serde(rename = "StatementId")]
    pub statement_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutRuleRequest {
    /// <p>A description of the rule.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The event pattern. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/events/CloudWatchEventsandEventPatterns.html">Events and Event Patterns</a> in the <i>Amazon CloudWatch Events User Guide</i>.</p>
    #[serde(rename = "EventPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_pattern: Option<String>,
    /// <p>The name of the rule that you are creating or updating.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The Amazon Resource Name (ARN) of the IAM role associated with the rule.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The scheduling expression. For example, "cron(0 20 * * ? *)" or "rate(5 minutes)".</p>
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
#[cfg_attr(test, derive(Serialize))]
pub struct PutRuleResponse {
    /// <p>The Amazon Resource Name (ARN) of the rule.</p>
    #[serde(rename = "RuleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutTargetsRequest {
    /// <p>The name of the rule.</p>
    #[serde(rename = "Rule")]
    pub rule: String,
    /// <p>The targets to update or add to the rule.</p>
    #[serde(rename = "Targets")]
    pub targets: Vec<Target>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
pub struct RemovePermissionRequest {
    /// <p>The statement ID corresponding to the account that is no longer allowed to put events to the default event bus.</p>
    #[serde(rename = "StatementId")]
    pub statement_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RemoveTargetsRequest {
    /// <p>If this is a managed rule, created by an AWS service on your behalf, you must specify <code>Force</code> as <code>True</code> to remove targets. This parameter is ignored for rules that are not managed rules. You can check whether a rule is a managed rule by using <code>DescribeRule</code> or <code>ListRules</code> and checking the <code>ManagedBy</code> field of the response.</p>
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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

/// <p>Contains information about a rule in Amazon CloudWatch Events.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Rule {
    /// <p>The Amazon Resource Name (ARN) of the rule.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The description of the rule.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The event pattern of the rule. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/events/CloudWatchEventsandEventPatterns.html">Events and Event Patterns</a> in the <i>Amazon CloudWatch Events User Guide</i>.</p>
    #[serde(rename = "EventPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_pattern: Option<String>,
    /// <p>If the rule was created on behalf of your account by an AWS service, this field displays the principal name of the service that created the rule.</p>
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
    /// <p>The scheduling expression. For example, "cron(0 20 * * ? *)", "rate(5 minutes)".</p>
    #[serde(rename = "ScheduleExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    /// <p>The state of the rule.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// <p>This parameter contains the criteria (either InstanceIds or a tag) used to specify which EC2 instances are to be sent the command. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunCommandParameters {
    /// <p>Currently, we support including only one RunCommandTarget block, which specifies either an array of InstanceIds or a tag.</p>
    #[serde(rename = "RunCommandTargets")]
    pub run_command_targets: Vec<RunCommandTarget>,
}

/// <p>Information about the EC2 instances that are to be sent the command, specified as key-value pairs. Each <code>RunCommandTarget</code> block can include only one key, but this key may specify multiple values.</p>
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

/// <p>A key-value pair associated with an AWS resource. In CloudWatch Events, rules support tagging.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>A string you can use to assign a value. The combination of tag keys and values can help you organize and categorize your resources.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The value for the specified tag key.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagResourceRequest {
    /// <p>The ARN of the CloudWatch Events rule that you're adding tags to.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    /// <p>The list of key-value pairs to associate with the rule.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TagResourceResponse {}

/// <p>Targets are the resources to be invoked when a rule is triggered. For a complete list of services and resources that can be set as a target, see <a>PutTargets</a>.</p> <p>If you are setting the event bus of another account as the target, and that account granted permission to your account through an organization instead of directly by the account ID, then you must specify a <code>RoleArn</code> with proper permissions in the <code>Target</code> structure. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/events/CloudWatchEvents-CrossAccountEventDelivery.html">Sending and Receiving Events Between AWS Accounts</a> in the <i>Amazon CloudWatch Events User Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Target {
    /// <p>The Amazon Resource Name (ARN) of the target.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>If the event target is an AWS Batch job, this contains the job definition, job name, and other parameters. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/jobs.html">Jobs</a> in the <i>AWS Batch User Guide</i>.</p>
    #[serde(rename = "BatchParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_parameters: Option<BatchParameters>,
    /// <p>Contains the Amazon ECS task definition and task count to be used, if the event target is an Amazon ECS task. For more information about Amazon ECS tasks, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task_defintions.html">Task Definitions </a> in the <i>Amazon EC2 Container Service Developer Guide</i>.</p>
    #[serde(rename = "EcsParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecs_parameters: Option<EcsParameters>,
    /// <p>The ID of the target.</p>
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
    /// <p>The custom parameter you can use to control the shard assignment, when the target is a Kinesis data stream. If you do not include this parameter, the default is to use the <code>eventId</code> as the partition key.</p>
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
pub struct TestEventPatternRequest {
    /// <p>The event, in JSON format, to test against the event pattern.</p>
    #[serde(rename = "Event")]
    pub event: String,
    /// <p>The event pattern. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/events/CloudWatchEventsandEventPatterns.html">Events and Event Patterns</a> in the <i>Amazon CloudWatch Events User Guide</i>.</p>
    #[serde(rename = "EventPattern")]
    pub event_pattern: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TestEventPatternResponse {
    /// <p>Indicates whether the event matches the event pattern.</p>
    #[serde(rename = "Result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UntagResourceRequest {
    /// <p>The ARN of the CloudWatch Events rule from which you are removing tags.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    /// <p>The list of tag keys to remove from the resource.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UntagResourceResponse {}

/// Errors returned by DeleteRule
#[derive(Debug, PartialEq)]
pub enum DeleteRuleError {
    /// <p>There is concurrent modification on a rule or target.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>This rule was created by an AWS service on behalf of your account. It is managed by that service. If you see this error in response to <code>DeleteRule</code> or <code>RemoveTargets</code>, you can use the <code>Force</code> parameter in those calls to delete the rule or remove targets from the rule. You cannot modify these managed rules by using <code>DisableRule</code>, <code>EnableRule</code>, <code>PutTargets</code>, <code>PutRule</code>, <code>TagResource</code>, or <code>UntagResource</code>. </p>
    ManagedRule(String),
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
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteRuleError {
    fn description(&self) -> &str {
        match *self {
            DeleteRuleError::ConcurrentModification(ref cause) => cause,
            DeleteRuleError::Internal(ref cause) => cause,
            DeleteRuleError::ManagedRule(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeEventBus
#[derive(Debug, PartialEq)]
pub enum DescribeEventBusError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>An entity that you specified does not exist.</p>
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeEventBusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEventBusError {
    fn description(&self) -> &str {
        match *self {
            DescribeEventBusError::Internal(ref cause) => cause,
            DescribeEventBusError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeRule
#[derive(Debug, PartialEq)]
pub enum DescribeRuleError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>An entity that you specified does not exist.</p>
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeRuleError {
    fn description(&self) -> &str {
        match *self {
            DescribeRuleError::Internal(ref cause) => cause,
            DescribeRuleError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by DisableRule
#[derive(Debug, PartialEq)]
pub enum DisableRuleError {
    /// <p>There is concurrent modification on a rule or target.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>This rule was created by an AWS service on behalf of your account. It is managed by that service. If you see this error in response to <code>DeleteRule</code> or <code>RemoveTargets</code>, you can use the <code>Force</code> parameter in those calls to delete the rule or remove targets from the rule. You cannot modify these managed rules by using <code>DisableRule</code>, <code>EnableRule</code>, <code>PutTargets</code>, <code>PutRule</code>, <code>TagResource</code>, or <code>UntagResource</code>. </p>
    ManagedRule(String),
    /// <p>An entity that you specified does not exist.</p>
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DisableRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisableRuleError {
    fn description(&self) -> &str {
        match *self {
            DisableRuleError::ConcurrentModification(ref cause) => cause,
            DisableRuleError::Internal(ref cause) => cause,
            DisableRuleError::ManagedRule(ref cause) => cause,
            DisableRuleError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by EnableRule
#[derive(Debug, PartialEq)]
pub enum EnableRuleError {
    /// <p>There is concurrent modification on a rule or target.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>This rule was created by an AWS service on behalf of your account. It is managed by that service. If you see this error in response to <code>DeleteRule</code> or <code>RemoveTargets</code>, you can use the <code>Force</code> parameter in those calls to delete the rule or remove targets from the rule. You cannot modify these managed rules by using <code>DisableRule</code>, <code>EnableRule</code>, <code>PutTargets</code>, <code>PutRule</code>, <code>TagResource</code>, or <code>UntagResource</code>. </p>
    ManagedRule(String),
    /// <p>An entity that you specified does not exist.</p>
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for EnableRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for EnableRuleError {
    fn description(&self) -> &str {
        match *self {
            EnableRuleError::ConcurrentModification(ref cause) => cause,
            EnableRuleError::Internal(ref cause) => cause,
            EnableRuleError::ManagedRule(ref cause) => cause,
            EnableRuleError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by ListRuleNamesByTarget
#[derive(Debug, PartialEq)]
pub enum ListRuleNamesByTargetError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
}

impl ListRuleNamesByTargetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRuleNamesByTargetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(ListRuleNamesByTargetError::Internal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListRuleNamesByTargetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListRuleNamesByTargetError {
    fn description(&self) -> &str {
        match *self {
            ListRuleNamesByTargetError::Internal(ref cause) => cause,
        }
    }
}
/// Errors returned by ListRules
#[derive(Debug, PartialEq)]
pub enum ListRulesError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
}

impl ListRulesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRulesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(ListRulesError::Internal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListRulesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListRulesError {
    fn description(&self) -> &str {
        match *self {
            ListRulesError::Internal(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>An entity that you specified does not exist.</p>
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
            ListTagsForResourceError::Internal(ref cause) => cause,
            ListTagsForResourceError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTargetsByRule
#[derive(Debug, PartialEq)]
pub enum ListTargetsByRuleError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>An entity that you specified does not exist.</p>
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListTargetsByRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTargetsByRuleError {
    fn description(&self) -> &str {
        match *self {
            ListTargetsByRuleError::Internal(ref cause) => cause,
            ListTargetsByRuleError::ResourceNotFound(ref cause) => cause,
        }
    }
}
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutEventsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutEventsError {
    fn description(&self) -> &str {
        match *self {
            PutEventsError::Internal(ref cause) => cause,
        }
    }
}
/// Errors returned by PutPermission
#[derive(Debug, PartialEq)]
pub enum PutPermissionError {
    /// <p>There is concurrent modification on a rule or target.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>The event bus policy is too long. For more information, see the limits.</p>
    PolicyLengthExceeded(String),
    /// <p>An entity that you specified does not exist.</p>
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutPermissionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutPermissionError {
    fn description(&self) -> &str {
        match *self {
            PutPermissionError::ConcurrentModification(ref cause) => cause,
            PutPermissionError::Internal(ref cause) => cause,
            PutPermissionError::PolicyLengthExceeded(ref cause) => cause,
            PutPermissionError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by PutRule
#[derive(Debug, PartialEq)]
pub enum PutRuleError {
    /// <p>There is concurrent modification on a rule or target.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>The event pattern is not valid.</p>
    InvalidEventPattern(String),
    /// <p>You tried to create more rules or add more targets to a rule than is allowed.</p>
    LimitExceeded(String),
    /// <p>This rule was created by an AWS service on behalf of your account. It is managed by that service. If you see this error in response to <code>DeleteRule</code> or <code>RemoveTargets</code>, you can use the <code>Force</code> parameter in those calls to delete the rule or remove targets from the rule. You cannot modify these managed rules by using <code>DisableRule</code>, <code>EnableRule</code>, <code>PutTargets</code>, <code>PutRule</code>, <code>TagResource</code>, or <code>UntagResource</code>. </p>
    ManagedRule(String),
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
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutRuleError {
    fn description(&self) -> &str {
        match *self {
            PutRuleError::ConcurrentModification(ref cause) => cause,
            PutRuleError::Internal(ref cause) => cause,
            PutRuleError::InvalidEventPattern(ref cause) => cause,
            PutRuleError::LimitExceeded(ref cause) => cause,
            PutRuleError::ManagedRule(ref cause) => cause,
        }
    }
}
/// Errors returned by PutTargets
#[derive(Debug, PartialEq)]
pub enum PutTargetsError {
    /// <p>There is concurrent modification on a rule or target.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>You tried to create more rules or add more targets to a rule than is allowed.</p>
    LimitExceeded(String),
    /// <p>This rule was created by an AWS service on behalf of your account. It is managed by that service. If you see this error in response to <code>DeleteRule</code> or <code>RemoveTargets</code>, you can use the <code>Force</code> parameter in those calls to delete the rule or remove targets from the rule. You cannot modify these managed rules by using <code>DisableRule</code>, <code>EnableRule</code>, <code>PutTargets</code>, <code>PutRule</code>, <code>TagResource</code>, or <code>UntagResource</code>. </p>
    ManagedRule(String),
    /// <p>An entity that you specified does not exist.</p>
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutTargetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutTargetsError {
    fn description(&self) -> &str {
        match *self {
            PutTargetsError::ConcurrentModification(ref cause) => cause,
            PutTargetsError::Internal(ref cause) => cause,
            PutTargetsError::LimitExceeded(ref cause) => cause,
            PutTargetsError::ManagedRule(ref cause) => cause,
            PutTargetsError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by RemovePermission
#[derive(Debug, PartialEq)]
pub enum RemovePermissionError {
    /// <p>There is concurrent modification on a rule or target.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>An entity that you specified does not exist.</p>
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RemovePermissionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RemovePermissionError {
    fn description(&self) -> &str {
        match *self {
            RemovePermissionError::ConcurrentModification(ref cause) => cause,
            RemovePermissionError::Internal(ref cause) => cause,
            RemovePermissionError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by RemoveTargets
#[derive(Debug, PartialEq)]
pub enum RemoveTargetsError {
    /// <p>There is concurrent modification on a rule or target.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>This rule was created by an AWS service on behalf of your account. It is managed by that service. If you see this error in response to <code>DeleteRule</code> or <code>RemoveTargets</code>, you can use the <code>Force</code> parameter in those calls to delete the rule or remove targets from the rule. You cannot modify these managed rules by using <code>DisableRule</code>, <code>EnableRule</code>, <code>PutTargets</code>, <code>PutRule</code>, <code>TagResource</code>, or <code>UntagResource</code>. </p>
    ManagedRule(String),
    /// <p>An entity that you specified does not exist.</p>
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RemoveTargetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RemoveTargetsError {
    fn description(&self) -> &str {
        match *self {
            RemoveTargetsError::ConcurrentModification(ref cause) => cause,
            RemoveTargetsError::Internal(ref cause) => cause,
            RemoveTargetsError::ManagedRule(ref cause) => cause,
            RemoveTargetsError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>There is concurrent modification on a rule or target.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>This rule was created by an AWS service on behalf of your account. It is managed by that service. If you see this error in response to <code>DeleteRule</code> or <code>RemoveTargets</code>, you can use the <code>Force</code> parameter in those calls to delete the rule or remove targets from the rule. You cannot modify these managed rules by using <code>DisableRule</code>, <code>EnableRule</code>, <code>PutTargets</code>, <code>PutRule</code>, <code>TagResource</code>, or <code>UntagResource</code>. </p>
    ManagedRule(String),
    /// <p>An entity that you specified does not exist.</p>
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for TagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TagResourceError {
    fn description(&self) -> &str {
        match *self {
            TagResourceError::ConcurrentModification(ref cause) => cause,
            TagResourceError::Internal(ref cause) => cause,
            TagResourceError::ManagedRule(ref cause) => cause,
            TagResourceError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by TestEventPattern
#[derive(Debug, PartialEq)]
pub enum TestEventPatternError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>The event pattern is not valid.</p>
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for TestEventPatternError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TestEventPatternError {
    fn description(&self) -> &str {
        match *self {
            TestEventPatternError::Internal(ref cause) => cause,
            TestEventPatternError::InvalidEventPattern(ref cause) => cause,
        }
    }
}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>There is concurrent modification on a rule or target.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// <p>This rule was created by an AWS service on behalf of your account. It is managed by that service. If you see this error in response to <code>DeleteRule</code> or <code>RemoveTargets</code>, you can use the <code>Force</code> parameter in those calls to delete the rule or remove targets from the rule. You cannot modify these managed rules by using <code>DisableRule</code>, <code>EnableRule</code>, <code>PutTargets</code>, <code>PutRule</code>, <code>TagResource</code>, or <code>UntagResource</code>. </p>
    ManagedRule(String),
    /// <p>An entity that you specified does not exist.</p>
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UntagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UntagResourceError {
    fn description(&self) -> &str {
        match *self {
            UntagResourceError::ConcurrentModification(ref cause) => cause,
            UntagResourceError::Internal(ref cause) => cause,
            UntagResourceError::ManagedRule(ref cause) => cause,
            UntagResourceError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon CloudWatch Events API. Amazon CloudWatch Events clients implement this trait.
pub trait CloudWatchEvents {
    /// <p>Deletes the specified rule.</p> <p>Before you can delete the rule, you must remove all targets, using <a>RemoveTargets</a>.</p> <p>When you delete a rule, incoming events might continue to match to the deleted rule. Allow a short period of time for changes to take effect.</p> <p>Managed rules are rules created and managed by another AWS service on your behalf. These rules are created by those other AWS services to support functionality in those services. You can delete these rules using the <code>Force</code> option, but you should do so only if you are sure the other service is not still using that rule.</p>
    fn delete_rule(&self, input: DeleteRuleRequest) -> RusotoFuture<(), DeleteRuleError>;

    /// <p>Displays the external AWS accounts that are permitted to write events to your account using your account's event bus, and the associated policy. To enable your account to receive events from other accounts, use <a>PutPermission</a>.</p>
    fn describe_event_bus(&self) -> RusotoFuture<DescribeEventBusResponse, DescribeEventBusError>;

    /// <p>Describes the specified rule.</p> <p>DescribeRule does not list the targets of a rule. To see the targets associated with a rule, use <a>ListTargetsByRule</a>.</p>
    fn describe_rule(
        &self,
        input: DescribeRuleRequest,
    ) -> RusotoFuture<DescribeRuleResponse, DescribeRuleError>;

    /// <p>Disables the specified rule. A disabled rule won't match any events, and won't self-trigger if it has a schedule expression.</p> <p>When you disable a rule, incoming events might continue to match to the disabled rule. Allow a short period of time for changes to take effect.</p>
    fn disable_rule(&self, input: DisableRuleRequest) -> RusotoFuture<(), DisableRuleError>;

    /// <p>Enables the specified rule. If the rule does not exist, the operation fails.</p> <p>When you enable a rule, incoming events might not immediately start matching to a newly enabled rule. Allow a short period of time for changes to take effect.</p>
    fn enable_rule(&self, input: EnableRuleRequest) -> RusotoFuture<(), EnableRuleError>;

    /// <p>Lists the rules for the specified target. You can see which of the rules in Amazon CloudWatch Events can invoke a specific target in your account.</p>
    fn list_rule_names_by_target(
        &self,
        input: ListRuleNamesByTargetRequest,
    ) -> RusotoFuture<ListRuleNamesByTargetResponse, ListRuleNamesByTargetError>;

    /// <p>Lists your Amazon CloudWatch Events rules. You can either list all the rules or you can provide a prefix to match to the rule names.</p> <p>ListRules does not list the targets of a rule. To see the targets associated with a rule, use <a>ListTargetsByRule</a>.</p>
    fn list_rules(
        &self,
        input: ListRulesRequest,
    ) -> RusotoFuture<ListRulesResponse, ListRulesError>;

    /// <p>Displays the tags associated with a CloudWatch Events resource. In CloudWatch Events, rules can be tagged.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> RusotoFuture<ListTagsForResourceResponse, ListTagsForResourceError>;

    /// <p>Lists the targets assigned to the specified rule.</p>
    fn list_targets_by_rule(
        &self,
        input: ListTargetsByRuleRequest,
    ) -> RusotoFuture<ListTargetsByRuleResponse, ListTargetsByRuleError>;

    /// <p>Sends custom events to Amazon CloudWatch Events so that they can be matched to rules.</p>
    fn put_events(
        &self,
        input: PutEventsRequest,
    ) -> RusotoFuture<PutEventsResponse, PutEventsError>;

    /// <p>Running <code>PutPermission</code> permits the specified AWS account or AWS organization to put events to your account's default <i>event bus</i>. CloudWatch Events rules in your account are triggered by these events arriving to your default event bus. </p> <p>For another account to send events to your account, that external account must have a CloudWatch Events rule with your account's default event bus as a target.</p> <p>To enable multiple AWS accounts to put events to your default event bus, run <code>PutPermission</code> once for each of these accounts. Or, if all the accounts are members of the same AWS organization, you can run <code>PutPermission</code> once specifying <code>Principal</code> as "*" and specifying the AWS organization ID in <code>Condition</code>, to grant permissions to all accounts in that organization.</p> <p>If you grant permissions using an organization, then accounts in that organization must specify a <code>RoleArn</code> with proper permissions when they use <code>PutTarget</code> to add your account's event bus as a target. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/events/CloudWatchEvents-CrossAccountEventDelivery.html">Sending and Receiving Events Between AWS Accounts</a> in the <i>Amazon CloudWatch Events User Guide</i>.</p> <p>The permission policy on the default event bus cannot exceed 10 KB in size.</p>
    fn put_permission(&self, input: PutPermissionRequest) -> RusotoFuture<(), PutPermissionError>;

    /// <p>Creates or updates the specified rule. Rules are enabled by default, or based on value of the state. You can disable a rule using <a>DisableRule</a>.</p> <p>If you are updating an existing rule, the rule is replaced with what you specify in this <code>PutRule</code> command. If you omit arguments in <code>PutRule</code>, the old values for those arguments are not kept. Instead, they are replaced with null values.</p> <p>When you create or update a rule, incoming events might not immediately start matching to new or updated rules. Allow a short period of time for changes to take effect.</p> <p>A rule must contain at least an EventPattern or ScheduleExpression. Rules with EventPatterns are triggered when a matching event is observed. Rules with ScheduleExpressions self-trigger based on the given schedule. A rule can have both an EventPattern and a ScheduleExpression, in which case the rule triggers on matching events as well as on a schedule.</p> <p>When you initially create a rule, you can optionally assign one or more tags to the rule. Tags can help you organize and categorize your resources. You can also use them to scope user permissions, by granting a user permission to access or change only rules with certain tag values. To use the <code>PutRule</code> operation and assign tags, you must have both the <code>events:PutRule</code> and <code>events:TagResource</code> permissions.</p> <p>If you are updating an existing rule, any tags you specify in the <code>PutRule</code> operation are ignored. To update the tags of an existing rule, use <a>TagResource</a> and <a>UntagResource</a>.</p> <p>Most services in AWS treat : or / as the same character in Amazon Resource Names (ARNs). However, CloudWatch Events uses an exact match in event patterns and rules. Be sure to use the correct ARN characters when creating event patterns so that they match the ARN syntax in the event you want to match.</p> <p>In CloudWatch Events, it is possible to create rules that lead to infinite loops, where a rule is fired repeatedly. For example, a rule might detect that ACLs have changed on an S3 bucket, and trigger software to change them to the desired state. If the rule is not written carefully, the subsequent change to the ACLs fires the rule again, creating an infinite loop.</p> <p>To prevent this, write the rules so that the triggered actions do not re-fire the same rule. For example, your rule could fire only if ACLs are found to be in a bad state, instead of after any change. </p> <p>An infinite loop can quickly cause higher than expected charges. We recommend that you use budgeting, which alerts you when charges exceed your specified limit. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/budgets-managing-costs.html">Managing Your Costs with Budgets</a>.</p>
    fn put_rule(&self, input: PutRuleRequest) -> RusotoFuture<PutRuleResponse, PutRuleError>;

    /// <p>Adds the specified targets to the specified rule, or updates the targets if they are already associated with the rule.</p> <p>Targets are the resources that are invoked when a rule is triggered.</p> <p>You can configure the following as targets for CloudWatch Events:</p> <ul> <li> <p>EC2 instances</p> </li> <li> <p>SSM Run Command</p> </li> <li> <p>SSM Automation</p> </li> <li> <p>AWS Lambda functions</p> </li> <li> <p>Data streams in Amazon Kinesis Data Streams</p> </li> <li> <p>Data delivery streams in Amazon Kinesis Data Firehose</p> </li> <li> <p>Amazon ECS tasks</p> </li> <li> <p>AWS Step Functions state machines</p> </li> <li> <p>AWS Batch jobs</p> </li> <li> <p>AWS CodeBuild projects</p> </li> <li> <p>Pipelines in AWS CodePipeline</p> </li> <li> <p>Amazon Inspector assessment templates</p> </li> <li> <p>Amazon SNS topics</p> </li> <li> <p>Amazon SQS queues, including FIFO queues</p> </li> <li> <p>The default event bus of another AWS account</p> </li> </ul> <p>Creating rules with built-in targets is supported only in the AWS Management Console. The built-in targets are <code>EC2 CreateSnapshot API call</code>, <code>EC2 RebootInstances API call</code>, <code>EC2 StopInstances API call</code>, and <code>EC2 TerminateInstances API call</code>. </p> <p>For some target types, <code>PutTargets</code> provides target-specific parameters. If the target is a Kinesis data stream, you can optionally specify which shard the event goes to by using the <code>KinesisParameters</code> argument. To invoke a command on multiple EC2 instances with one rule, you can use the <code>RunCommandParameters</code> field.</p> <p>To be able to make API calls against the resources that you own, Amazon CloudWatch Events needs the appropriate permissions. For AWS Lambda and Amazon SNS resources, CloudWatch Events relies on resource-based policies. For EC2 instances, Kinesis data streams, and AWS Step Functions state machines, CloudWatch Events relies on IAM roles that you specify in the <code>RoleARN</code> argument in <code>PutTargets</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/events/auth-and-access-control-cwe.html">Authentication and Access Control</a> in the <i>Amazon CloudWatch Events User Guide</i>.</p> <p>If another AWS account is in the same region and has granted you permission (using <code>PutPermission</code>), you can send events to that account. Set that account's event bus as a target of the rules in your account. To send the matched events to the other account, specify that account's event bus as the <code>Arn</code> value when you run <code>PutTargets</code>. If your account sends events to another account, your account is charged for each sent event. Each event sent to another account is charged as a custom event. The account receiving the event is not charged. For more information, see <a href="https://aws.amazon.com/cloudwatch/pricing/">Amazon CloudWatch Pricing</a>.</p> <p>If you are setting the event bus of another account as the target, and that account granted permission to your account through an organization instead of directly by the account ID, then you must specify a <code>RoleArn</code> with proper permissions in the <code>Target</code> structure. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/events/CloudWatchEvents-CrossAccountEventDelivery.html">Sending and Receiving Events Between AWS Accounts</a> in the <i>Amazon CloudWatch Events User Guide</i>.</p> <p>For more information about enabling cross-account events, see <a>PutPermission</a>.</p> <p> <b>Input</b>, <b>InputPath</b>, and <b>InputTransformer</b> are mutually exclusive and optional parameters of a target. When a rule is triggered due to a matched event:</p> <ul> <li> <p>If none of the following arguments are specified for a target, then the entire event is passed to the target in JSON format (unless the target is Amazon EC2 Run Command or Amazon ECS task, in which case nothing from the event is passed to the target).</p> </li> <li> <p>If <b>Input</b> is specified in the form of valid JSON, then the matched event is overridden with this constant.</p> </li> <li> <p>If <b>InputPath</b> is specified in the form of JSONPath (for example, <code>$.detail</code>), then only the part of the event specified in the path is passed to the target (for example, only the detail part of the event is passed).</p> </li> <li> <p>If <b>InputTransformer</b> is specified, then one or more specified JSONPaths are extracted from the event and used as values in a template that you specify as the input to the target.</p> </li> </ul> <p>When you specify <code>InputPath</code> or <code>InputTransformer</code>, you must use JSON dot notation, not bracket notation.</p> <p>When you add targets to a rule and the associated rule triggers soon after, new or updated targets might not be immediately invoked. Allow a short period of time for changes to take effect.</p> <p>This action can partially fail if too many requests are made at the same time. If that happens, <code>FailedEntryCount</code> is non-zero in the response and each entry in <code>FailedEntries</code> provides the ID of the failed target and the error code.</p>
    fn put_targets(
        &self,
        input: PutTargetsRequest,
    ) -> RusotoFuture<PutTargetsResponse, PutTargetsError>;

    /// <p>Revokes the permission of another AWS account to be able to put events to your default event bus. Specify the account to revoke by the <code>StatementId</code> value that you associated with the account when you granted it permission with <code>PutPermission</code>. You can find the <code>StatementId</code> by using <a>DescribeEventBus</a>.</p>
    fn remove_permission(
        &self,
        input: RemovePermissionRequest,
    ) -> RusotoFuture<(), RemovePermissionError>;

    /// <p>Removes the specified targets from the specified rule. When the rule is triggered, those targets are no longer be invoked.</p> <p>When you remove a target, when the associated rule triggers, removed targets might continue to be invoked. Allow a short period of time for changes to take effect.</p> <p>This action can partially fail if too many requests are made at the same time. If that happens, <code>FailedEntryCount</code> is non-zero in the response and each entry in <code>FailedEntries</code> provides the ID of the failed target and the error code.</p>
    fn remove_targets(
        &self,
        input: RemoveTargetsRequest,
    ) -> RusotoFuture<RemoveTargetsResponse, RemoveTargetsError>;

    /// <p>Assigns one or more tags (key-value pairs) to the specified CloudWatch Events resource. Tags can help you organize and categorize your resources. You can also use them to scope user permissions by granting a user permission to access or change only resources with certain tag values. In CloudWatch Events, rules can be tagged.</p> <p>Tags don't have any semantic meaning to AWS and are interpreted strictly as strings of characters.</p> <p>You can use the <code>TagResource</code> action with a rule that already has tags. If you specify a new tag key for the rule, this tag is appended to the list of tags associated with the rule. If you specify a tag key that is already associated with the rule, the new tag value that you specify replaces the previous value for that tag.</p> <p>You can associate as many as 50 tags with a resource.</p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> RusotoFuture<TagResourceResponse, TagResourceError>;

    /// <p>Tests whether the specified event pattern matches the provided event.</p> <p>Most services in AWS treat : or / as the same character in Amazon Resource Names (ARNs). However, CloudWatch Events uses an exact match in event patterns and rules. Be sure to use the correct ARN characters when creating event patterns so that they match the ARN syntax in the event you want to match.</p>
    fn test_event_pattern(
        &self,
        input: TestEventPatternRequest,
    ) -> RusotoFuture<TestEventPatternResponse, TestEventPatternError>;

    /// <p>Removes one or more tags from the specified CloudWatch Events resource. In CloudWatch Events, rules can be tagged.</p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> RusotoFuture<UntagResourceResponse, UntagResourceError>;
}
/// A client for the Amazon CloudWatch Events API.
#[derive(Clone)]
pub struct CloudWatchEventsClient {
    client: Client,
    region: region::Region,
}

impl CloudWatchEventsClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> CloudWatchEventsClient {
        CloudWatchEventsClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> CloudWatchEventsClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        CloudWatchEventsClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }
}

impl CloudWatchEvents for CloudWatchEventsClient {
    /// <p>Deletes the specified rule.</p> <p>Before you can delete the rule, you must remove all targets, using <a>RemoveTargets</a>.</p> <p>When you delete a rule, incoming events might continue to match to the deleted rule. Allow a short period of time for changes to take effect.</p> <p>Managed rules are rules created and managed by another AWS service on your behalf. These rules are created by those other AWS services to support functionality in those services. You can delete these rules using the <code>Force</code> option, but you should do so only if you are sure the other service is not still using that rule.</p>
    fn delete_rule(&self, input: DeleteRuleRequest) -> RusotoFuture<(), DeleteRuleError> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.DeleteRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                futures::future::ready(Ok(std::mem::drop(response))).boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<DeleteRuleError>
                            })
                            .and_then(|response| Err(DeleteRuleError::from_response(response)))
                    })
                    .boxed()
            }
        })
    }

    /// <p>Displays the external AWS accounts that are permitted to write events to your account using your account's event bus, and the associated policy. To enable your account to receive events from other accounts, use <a>PutPermission</a>.</p>
    fn describe_event_bus(&self) -> RusotoFuture<DescribeEventBusResponse, DescribeEventBusError> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.DescribeEventBus");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| DescribeEventBusError::from(e))
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<DescribeEventBusError>
                            })
                            .and_then(|response| {
                                proto::json::ResponsePayload::new(&response)
                                    .deserialize::<DescribeEventBusResponse, _>()
                            })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<DescribeEventBusError>
                            })
                            .and_then(|response| {
                                Err(DescribeEventBusError::from_response(response))
                            })
                    })
                    .boxed()
            }
        })
    }

    /// <p>Describes the specified rule.</p> <p>DescribeRule does not list the targets of a rule. To see the targets associated with a rule, use <a>ListTargetsByRule</a>.</p>
    fn describe_rule(
        &self,
        input: DescribeRuleRequest,
    ) -> RusotoFuture<DescribeRuleResponse, DescribeRuleError> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.DescribeRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| DescribeRuleError::from(e))
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<DescribeRuleError>
                            })
                            .and_then(|response| {
                                proto::json::ResponsePayload::new(&response)
                                    .deserialize::<DescribeRuleResponse, _>()
                            })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<DescribeRuleError>
                            })
                            .and_then(|response| Err(DescribeRuleError::from_response(response)))
                    })
                    .boxed()
            }
        })
    }

    /// <p>Disables the specified rule. A disabled rule won't match any events, and won't self-trigger if it has a schedule expression.</p> <p>When you disable a rule, incoming events might continue to match to the disabled rule. Allow a short period of time for changes to take effect.</p>
    fn disable_rule(&self, input: DisableRuleRequest) -> RusotoFuture<(), DisableRuleError> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.DisableRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                futures::future::ready(Ok(std::mem::drop(response))).boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<DisableRuleError>
                            })
                            .and_then(|response| Err(DisableRuleError::from_response(response)))
                    })
                    .boxed()
            }
        })
    }

    /// <p>Enables the specified rule. If the rule does not exist, the operation fails.</p> <p>When you enable a rule, incoming events might not immediately start matching to a newly enabled rule. Allow a short period of time for changes to take effect.</p>
    fn enable_rule(&self, input: EnableRuleRequest) -> RusotoFuture<(), EnableRuleError> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.EnableRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                futures::future::ready(Ok(std::mem::drop(response))).boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<EnableRuleError>
                            })
                            .and_then(|response| Err(EnableRuleError::from_response(response)))
                    })
                    .boxed()
            }
        })
    }

    /// <p>Lists the rules for the specified target. You can see which of the rules in Amazon CloudWatch Events can invoke a specific target in your account.</p>
    fn list_rule_names_by_target(
        &self,
        input: ListRuleNamesByTargetRequest,
    ) -> RusotoFuture<ListRuleNamesByTargetResponse, ListRuleNamesByTargetError> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.ListRuleNamesByTarget");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| ListRuleNamesByTargetError::from(e))
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e)
                                    as RusotoError<ListRuleNamesByTargetError>
                            })
                            .and_then(|response| {
                                proto::json::ResponsePayload::new(&response)
                                    .deserialize::<ListRuleNamesByTargetResponse, _>()
                            })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e)
                                    as RusotoError<ListRuleNamesByTargetError>
                            })
                            .and_then(|response| {
                                Err(ListRuleNamesByTargetError::from_response(response))
                            })
                    })
                    .boxed()
            }
        })
    }

    /// <p>Lists your Amazon CloudWatch Events rules. You can either list all the rules or you can provide a prefix to match to the rule names.</p> <p>ListRules does not list the targets of a rule. To see the targets associated with a rule, use <a>ListTargetsByRule</a>.</p>
    fn list_rules(
        &self,
        input: ListRulesRequest,
    ) -> RusotoFuture<ListRulesResponse, ListRulesError> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.ListRules");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| ListRulesError::from(e))
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<ListRulesError>
                            })
                            .and_then(|response| {
                                proto::json::ResponsePayload::new(&response)
                                    .deserialize::<ListRulesResponse, _>()
                            })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<ListRulesError>
                            })
                            .and_then(|response| Err(ListRulesError::from_response(response)))
                    })
                    .boxed()
            }
        })
    }

    /// <p>Displays the tags associated with a CloudWatch Events resource. In CloudWatch Events, rules can be tagged.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> RusotoFuture<ListTagsForResourceResponse, ListTagsForResourceError> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.ListTagsForResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| ListTagsForResourceError::from(e))
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e)
                                    as RusotoError<ListTagsForResourceError>
                            })
                            .and_then(|response| {
                                proto::json::ResponsePayload::new(&response)
                                    .deserialize::<ListTagsForResourceResponse, _>()
                            })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e)
                                    as RusotoError<ListTagsForResourceError>
                            })
                            .and_then(|response| {
                                Err(ListTagsForResourceError::from_response(response))
                            })
                    })
                    .boxed()
            }
        })
    }

    /// <p>Lists the targets assigned to the specified rule.</p>
    fn list_targets_by_rule(
        &self,
        input: ListTargetsByRuleRequest,
    ) -> RusotoFuture<ListTargetsByRuleResponse, ListTargetsByRuleError> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.ListTargetsByRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| ListTargetsByRuleError::from(e))
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<ListTargetsByRuleError>
                            })
                            .and_then(|response| {
                                proto::json::ResponsePayload::new(&response)
                                    .deserialize::<ListTargetsByRuleResponse, _>()
                            })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<ListTargetsByRuleError>
                            })
                            .and_then(|response| {
                                Err(ListTargetsByRuleError::from_response(response))
                            })
                    })
                    .boxed()
            }
        })
    }

    /// <p>Sends custom events to Amazon CloudWatch Events so that they can be matched to rules.</p>
    fn put_events(
        &self,
        input: PutEventsRequest,
    ) -> RusotoFuture<PutEventsResponse, PutEventsError> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.PutEvents");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| PutEventsError::from(e))
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<PutEventsError>
                            })
                            .and_then(|response| {
                                proto::json::ResponsePayload::new(&response)
                                    .deserialize::<PutEventsResponse, _>()
                            })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<PutEventsError>
                            })
                            .and_then(|response| Err(PutEventsError::from_response(response)))
                    })
                    .boxed()
            }
        })
    }

    /// <p>Running <code>PutPermission</code> permits the specified AWS account or AWS organization to put events to your account's default <i>event bus</i>. CloudWatch Events rules in your account are triggered by these events arriving to your default event bus. </p> <p>For another account to send events to your account, that external account must have a CloudWatch Events rule with your account's default event bus as a target.</p> <p>To enable multiple AWS accounts to put events to your default event bus, run <code>PutPermission</code> once for each of these accounts. Or, if all the accounts are members of the same AWS organization, you can run <code>PutPermission</code> once specifying <code>Principal</code> as "*" and specifying the AWS organization ID in <code>Condition</code>, to grant permissions to all accounts in that organization.</p> <p>If you grant permissions using an organization, then accounts in that organization must specify a <code>RoleArn</code> with proper permissions when they use <code>PutTarget</code> to add your account's event bus as a target. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/events/CloudWatchEvents-CrossAccountEventDelivery.html">Sending and Receiving Events Between AWS Accounts</a> in the <i>Amazon CloudWatch Events User Guide</i>.</p> <p>The permission policy on the default event bus cannot exceed 10 KB in size.</p>
    fn put_permission(&self, input: PutPermissionRequest) -> RusotoFuture<(), PutPermissionError> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.PutPermission");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                futures::future::ready(Ok(std::mem::drop(response))).boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<PutPermissionError>
                            })
                            .and_then(|response| Err(PutPermissionError::from_response(response)))
                    })
                    .boxed()
            }
        })
    }

    /// <p>Creates or updates the specified rule. Rules are enabled by default, or based on value of the state. You can disable a rule using <a>DisableRule</a>.</p> <p>If you are updating an existing rule, the rule is replaced with what you specify in this <code>PutRule</code> command. If you omit arguments in <code>PutRule</code>, the old values for those arguments are not kept. Instead, they are replaced with null values.</p> <p>When you create or update a rule, incoming events might not immediately start matching to new or updated rules. Allow a short period of time for changes to take effect.</p> <p>A rule must contain at least an EventPattern or ScheduleExpression. Rules with EventPatterns are triggered when a matching event is observed. Rules with ScheduleExpressions self-trigger based on the given schedule. A rule can have both an EventPattern and a ScheduleExpression, in which case the rule triggers on matching events as well as on a schedule.</p> <p>When you initially create a rule, you can optionally assign one or more tags to the rule. Tags can help you organize and categorize your resources. You can also use them to scope user permissions, by granting a user permission to access or change only rules with certain tag values. To use the <code>PutRule</code> operation and assign tags, you must have both the <code>events:PutRule</code> and <code>events:TagResource</code> permissions.</p> <p>If you are updating an existing rule, any tags you specify in the <code>PutRule</code> operation are ignored. To update the tags of an existing rule, use <a>TagResource</a> and <a>UntagResource</a>.</p> <p>Most services in AWS treat : or / as the same character in Amazon Resource Names (ARNs). However, CloudWatch Events uses an exact match in event patterns and rules. Be sure to use the correct ARN characters when creating event patterns so that they match the ARN syntax in the event you want to match.</p> <p>In CloudWatch Events, it is possible to create rules that lead to infinite loops, where a rule is fired repeatedly. For example, a rule might detect that ACLs have changed on an S3 bucket, and trigger software to change them to the desired state. If the rule is not written carefully, the subsequent change to the ACLs fires the rule again, creating an infinite loop.</p> <p>To prevent this, write the rules so that the triggered actions do not re-fire the same rule. For example, your rule could fire only if ACLs are found to be in a bad state, instead of after any change. </p> <p>An infinite loop can quickly cause higher than expected charges. We recommend that you use budgeting, which alerts you when charges exceed your specified limit. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/budgets-managing-costs.html">Managing Your Costs with Budgets</a>.</p>
    fn put_rule(&self, input: PutRuleRequest) -> RusotoFuture<PutRuleResponse, PutRuleError> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.PutRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| PutRuleError::from(e))
                    .map(|try_response| {
                        try_response
                            .map_err(|e| RusotoError::HttpDispatch(e) as RusotoError<PutRuleError>)
                            .and_then(|response| {
                                proto::json::ResponsePayload::new(&response)
                                    .deserialize::<PutRuleResponse, _>()
                            })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| RusotoError::HttpDispatch(e) as RusotoError<PutRuleError>)
                            .and_then(|response| Err(PutRuleError::from_response(response)))
                    })
                    .boxed()
            }
        })
    }

    /// <p>Adds the specified targets to the specified rule, or updates the targets if they are already associated with the rule.</p> <p>Targets are the resources that are invoked when a rule is triggered.</p> <p>You can configure the following as targets for CloudWatch Events:</p> <ul> <li> <p>EC2 instances</p> </li> <li> <p>SSM Run Command</p> </li> <li> <p>SSM Automation</p> </li> <li> <p>AWS Lambda functions</p> </li> <li> <p>Data streams in Amazon Kinesis Data Streams</p> </li> <li> <p>Data delivery streams in Amazon Kinesis Data Firehose</p> </li> <li> <p>Amazon ECS tasks</p> </li> <li> <p>AWS Step Functions state machines</p> </li> <li> <p>AWS Batch jobs</p> </li> <li> <p>AWS CodeBuild projects</p> </li> <li> <p>Pipelines in AWS CodePipeline</p> </li> <li> <p>Amazon Inspector assessment templates</p> </li> <li> <p>Amazon SNS topics</p> </li> <li> <p>Amazon SQS queues, including FIFO queues</p> </li> <li> <p>The default event bus of another AWS account</p> </li> </ul> <p>Creating rules with built-in targets is supported only in the AWS Management Console. The built-in targets are <code>EC2 CreateSnapshot API call</code>, <code>EC2 RebootInstances API call</code>, <code>EC2 StopInstances API call</code>, and <code>EC2 TerminateInstances API call</code>. </p> <p>For some target types, <code>PutTargets</code> provides target-specific parameters. If the target is a Kinesis data stream, you can optionally specify which shard the event goes to by using the <code>KinesisParameters</code> argument. To invoke a command on multiple EC2 instances with one rule, you can use the <code>RunCommandParameters</code> field.</p> <p>To be able to make API calls against the resources that you own, Amazon CloudWatch Events needs the appropriate permissions. For AWS Lambda and Amazon SNS resources, CloudWatch Events relies on resource-based policies. For EC2 instances, Kinesis data streams, and AWS Step Functions state machines, CloudWatch Events relies on IAM roles that you specify in the <code>RoleARN</code> argument in <code>PutTargets</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/events/auth-and-access-control-cwe.html">Authentication and Access Control</a> in the <i>Amazon CloudWatch Events User Guide</i>.</p> <p>If another AWS account is in the same region and has granted you permission (using <code>PutPermission</code>), you can send events to that account. Set that account's event bus as a target of the rules in your account. To send the matched events to the other account, specify that account's event bus as the <code>Arn</code> value when you run <code>PutTargets</code>. If your account sends events to another account, your account is charged for each sent event. Each event sent to another account is charged as a custom event. The account receiving the event is not charged. For more information, see <a href="https://aws.amazon.com/cloudwatch/pricing/">Amazon CloudWatch Pricing</a>.</p> <p>If you are setting the event bus of another account as the target, and that account granted permission to your account through an organization instead of directly by the account ID, then you must specify a <code>RoleArn</code> with proper permissions in the <code>Target</code> structure. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/events/CloudWatchEvents-CrossAccountEventDelivery.html">Sending and Receiving Events Between AWS Accounts</a> in the <i>Amazon CloudWatch Events User Guide</i>.</p> <p>For more information about enabling cross-account events, see <a>PutPermission</a>.</p> <p> <b>Input</b>, <b>InputPath</b>, and <b>InputTransformer</b> are mutually exclusive and optional parameters of a target. When a rule is triggered due to a matched event:</p> <ul> <li> <p>If none of the following arguments are specified for a target, then the entire event is passed to the target in JSON format (unless the target is Amazon EC2 Run Command or Amazon ECS task, in which case nothing from the event is passed to the target).</p> </li> <li> <p>If <b>Input</b> is specified in the form of valid JSON, then the matched event is overridden with this constant.</p> </li> <li> <p>If <b>InputPath</b> is specified in the form of JSONPath (for example, <code>$.detail</code>), then only the part of the event specified in the path is passed to the target (for example, only the detail part of the event is passed).</p> </li> <li> <p>If <b>InputTransformer</b> is specified, then one or more specified JSONPaths are extracted from the event and used as values in a template that you specify as the input to the target.</p> </li> </ul> <p>When you specify <code>InputPath</code> or <code>InputTransformer</code>, you must use JSON dot notation, not bracket notation.</p> <p>When you add targets to a rule and the associated rule triggers soon after, new or updated targets might not be immediately invoked. Allow a short period of time for changes to take effect.</p> <p>This action can partially fail if too many requests are made at the same time. If that happens, <code>FailedEntryCount</code> is non-zero in the response and each entry in <code>FailedEntries</code> provides the ID of the failed target and the error code.</p>
    fn put_targets(
        &self,
        input: PutTargetsRequest,
    ) -> RusotoFuture<PutTargetsResponse, PutTargetsError> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.PutTargets");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| PutTargetsError::from(e))
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<PutTargetsError>
                            })
                            .and_then(|response| {
                                proto::json::ResponsePayload::new(&response)
                                    .deserialize::<PutTargetsResponse, _>()
                            })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<PutTargetsError>
                            })
                            .and_then(|response| Err(PutTargetsError::from_response(response)))
                    })
                    .boxed()
            }
        })
    }

    /// <p>Revokes the permission of another AWS account to be able to put events to your default event bus. Specify the account to revoke by the <code>StatementId</code> value that you associated with the account when you granted it permission with <code>PutPermission</code>. You can find the <code>StatementId</code> by using <a>DescribeEventBus</a>.</p>
    fn remove_permission(
        &self,
        input: RemovePermissionRequest,
    ) -> RusotoFuture<(), RemovePermissionError> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.RemovePermission");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                futures::future::ready(Ok(std::mem::drop(response))).boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<RemovePermissionError>
                            })
                            .and_then(|response| {
                                Err(RemovePermissionError::from_response(response))
                            })
                    })
                    .boxed()
            }
        })
    }

    /// <p>Removes the specified targets from the specified rule. When the rule is triggered, those targets are no longer be invoked.</p> <p>When you remove a target, when the associated rule triggers, removed targets might continue to be invoked. Allow a short period of time for changes to take effect.</p> <p>This action can partially fail if too many requests are made at the same time. If that happens, <code>FailedEntryCount</code> is non-zero in the response and each entry in <code>FailedEntries</code> provides the ID of the failed target and the error code.</p>
    fn remove_targets(
        &self,
        input: RemoveTargetsRequest,
    ) -> RusotoFuture<RemoveTargetsResponse, RemoveTargetsError> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.RemoveTargets");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| RemoveTargetsError::from(e))
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<RemoveTargetsError>
                            })
                            .and_then(|response| {
                                proto::json::ResponsePayload::new(&response)
                                    .deserialize::<RemoveTargetsResponse, _>()
                            })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<RemoveTargetsError>
                            })
                            .and_then(|response| Err(RemoveTargetsError::from_response(response)))
                    })
                    .boxed()
            }
        })
    }

    /// <p>Assigns one or more tags (key-value pairs) to the specified CloudWatch Events resource. Tags can help you organize and categorize your resources. You can also use them to scope user permissions by granting a user permission to access or change only resources with certain tag values. In CloudWatch Events, rules can be tagged.</p> <p>Tags don't have any semantic meaning to AWS and are interpreted strictly as strings of characters.</p> <p>You can use the <code>TagResource</code> action with a rule that already has tags. If you specify a new tag key for the rule, this tag is appended to the list of tags associated with the rule. If you specify a tag key that is already associated with the rule, the new tag value that you specify replaces the previous value for that tag.</p> <p>You can associate as many as 50 tags with a resource.</p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> RusotoFuture<TagResourceResponse, TagResourceError> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| TagResourceError::from(e))
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<TagResourceError>
                            })
                            .and_then(|response| {
                                proto::json::ResponsePayload::new(&response)
                                    .deserialize::<TagResourceResponse, _>()
                            })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<TagResourceError>
                            })
                            .and_then(|response| Err(TagResourceError::from_response(response)))
                    })
                    .boxed()
            }
        })
    }

    /// <p>Tests whether the specified event pattern matches the provided event.</p> <p>Most services in AWS treat : or / as the same character in Amazon Resource Names (ARNs). However, CloudWatch Events uses an exact match in event patterns and rules. Be sure to use the correct ARN characters when creating event patterns so that they match the ARN syntax in the event you want to match.</p>
    fn test_event_pattern(
        &self,
        input: TestEventPatternRequest,
    ) -> RusotoFuture<TestEventPatternResponse, TestEventPatternError> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.TestEventPattern");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| TestEventPatternError::from(e))
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<TestEventPatternError>
                            })
                            .and_then(|response| {
                                proto::json::ResponsePayload::new(&response)
                                    .deserialize::<TestEventPatternResponse, _>()
                            })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<TestEventPatternError>
                            })
                            .and_then(|response| {
                                Err(TestEventPatternError::from_response(response))
                            })
                    })
                    .boxed()
            }
        })
    }

    /// <p>Removes one or more tags from the specified CloudWatch Events resource. In CloudWatch Events, rules can be tagged.</p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> RusotoFuture<UntagResourceResponse, UntagResourceError> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| UntagResourceError::from(e))
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<UntagResourceError>
                            })
                            .and_then(|response| {
                                proto::json::ResponsePayload::new(&response)
                                    .deserialize::<UntagResourceResponse, _>()
                            })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<UntagResourceError>
                            })
                            .and_then(|response| Err(UntagResourceError::from_response(response)))
                    })
                    .boxed()
            }
        })
    }
}
