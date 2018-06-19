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
use rusoto_core::reactor::{CredentialsProvider, RequestDispatcher};
use rusoto_core::region;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::{ClientInner, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use hyper::StatusCode;
use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_str;
use serde_json::Value as SerdeJsonValue;
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
    /// <p>The retry strategy to use for failed jobs, if the target is an AWS Batch job. The retry strategy is the number of times to retry the failed job execution. Valid values are 1 to 10. When you specify a retry strategy here, it overrides the retry strategy defined in the job definition.</p>
    #[serde(rename = "RetryStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_strategy: Option<BatchRetryStrategy>,
}

/// <p>The retry strategy to use for failed jobs, if the target is an AWS Batch job. If you specify a retry strategy here, it overrides the retry strategy defined in the job definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BatchRetryStrategy {
    /// <p>The number of times to attempt to retry, if the job fails. Valid values are 1 to 10.</p>
    #[serde(rename = "Attempts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempts: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteRuleRequest {
    /// <p>The name of the rule.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeEventBusRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct DescribeRuleResponse {
    /// <p>The Amazon Resource Name (ARN) of the rule.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The description of the rule.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The event pattern. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudWatch/latest/events/CloudWatchEventsandEventPatterns.html">Events and Event Patterns</a> in the <i>Amazon CloudWatch Events User Guide</i>.</p>
    #[serde(rename = "EventPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_pattern: Option<String>,
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

/// <p>The custom parameters to be used when the target is an Amazon ECS cluster.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EcsParameters {
    /// <p>The number of tasks to create based on the <code>TaskDefinition</code>. The default is one.</p>
    #[serde(rename = "TaskCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_count: Option<i64>,
    /// <p>The ARN of the task definition to use if the event target is an Amazon ECS cluster. </p>
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
    /// <p>Map of JSON paths to be extracted from the event. These are key-value pairs, where each value is a JSON path. You must use JSON dot notation, not bracket notation.</p>
    #[serde(rename = "InputPathsMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_paths_map: Option<::std::collections::HashMap<String, String>>,
    /// <p>Input template where you can use the values of the keys from <code>InputPathsMap</code> to customize the data sent to the target.</p>
    #[serde(rename = "InputTemplate")]
    pub input_template: String,
}

/// <p>This object enables you to specify a JSON path to extract from the event and use as the partition key for the Amazon Kinesis stream, so that you can control the shard to which the event goes. If you do not include this parameter, the default is to use the <code>eventId</code> as the partition key.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KinesisParameters {
    /// <p>The JSON path to be extracted from the event and used as the partition key. For more information, see <a href="http://docs.aws.amazon.com/streams/latest/dev/key-concepts.html#partition-key">Amazon Kinesis Streams Key Concepts</a> in the <i>Amazon Kinesis Streams Developer Guide</i>.</p>
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
    /// <p>The source of the event.</p>
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// <p>The timestamp of the event, per <a href="https://www.rfc-editor.org/rfc/rfc3339.txt">RFC3339</a>. If no timestamp is provided, the timestamp of the <a>PutEvents</a> call is used.</p>
    #[serde(rename = "Time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
    /// <p>The 12-digit AWS account ID that you are permitting to put events to your default event bus. Specify "*" to permit any account to put events to your default event bus.</p> <p>If you specify "*", avoid creating rules that may match undesirable events. To create more secure rules, make sure that the event pattern for each rule contains an <code>account</code> field with a specific account ID from which to receive events. Rules with an account field do not match any events sent from other accounts.</p>
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
    /// <p>The event pattern. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudWatch/latest/events/CloudWatchEventsandEventPatterns.html">Events and Event Patterns</a> in the <i>Amazon CloudWatch Events User Guide</i>.</p>
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
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
    /// <p>The IDs of the targets to remove from the rule.</p>
    #[serde(rename = "Ids")]
    pub ids: Vec<String>,
    /// <p>The name of the rule.</p>
    #[serde(rename = "Rule")]
    pub rule: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct Rule {
    /// <p>The Amazon Resource Name (ARN) of the rule.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The description of the rule.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The event pattern of the rule. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudWatch/latest/events/CloudWatchEventsandEventPatterns.html">Events and Event Patterns</a> in the <i>Amazon CloudWatch Events User Guide</i>.</p>
    #[serde(rename = "EventPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_pattern: Option<String>,
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

/// <p>Targets are the resources to be invoked when a rule is triggered. Target types include EC2 instances, AWS Lambda functions, Amazon Kinesis streams, Amazon ECS tasks, AWS Step Functions state machines, Run Command, and built-in targets.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Target {
    /// <p>The Amazon Resource Name (ARN) of the target.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>Contains the job definition, job name, and other parameters if the event target is an AWS Batch job. For more information about AWS Batch, see <a href="http://docs.aws.amazon.com/batch/latest/userguide/jobs.html">Jobs</a> in the <i>AWS Batch User Guide</i>.</p>
    #[serde(rename = "BatchParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_parameters: Option<BatchParameters>,
    /// <p>Contains the Amazon ECS task definition and task count to be used, if the event target is an Amazon ECS task. For more information about Amazon ECS tasks, see <a href="http://docs.aws.amazon.com/AmazonECS/latest/developerguide/task_defintions.html">Task Definitions </a> in the <i>Amazon EC2 Container Service Developer Guide</i>.</p>
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
    /// <p>The custom parameter you can use to control shard assignment, when the target is an Amazon Kinesis stream. If you do not include this parameter, the default is to use the <code>eventId</code> as the partition key.</p>
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
    /// <p>Contains the message group ID to use when the target is a FIFO queue.</p>
    #[serde(rename = "SqsParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sqs_parameters: Option<SqsParameters>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TestEventPatternRequest {
    /// <p>The event, in JSON format, to test against the event pattern.</p>
    #[serde(rename = "Event")]
    pub event: String,
    /// <p>The event pattern. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudWatch/latest/events/CloudWatchEventsandEventPatterns.html">Events and Event Patterns</a> in the <i>Amazon CloudWatch Events User Guide</i>.</p>
    #[serde(rename = "EventPattern")]
    pub event_pattern: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TestEventPatternResponse {
    /// <p>Indicates whether the event matches the event pattern.</p>
    #[serde(rename = "Result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<bool>,
}

/// Errors returned by DeleteRule
#[derive(Debug, PartialEq)]
pub enum DeleteRuleError {
    /// <p>There is concurrent modification on a rule or target.</p>
    ConcurrentModification(String),
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteRuleError {
    pub fn from_body(body: &str) -> DeleteRuleError {
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
                    "ConcurrentModificationException" => {
                        DeleteRuleError::ConcurrentModification(String::from(error_message))
                    }
                    "InternalException" => DeleteRuleError::Internal(String::from(error_message)),
                    "ValidationException" => DeleteRuleError::Validation(error_message.to_string()),
                    _ => DeleteRuleError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteRuleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteRuleError {
    fn from(err: serde_json::error::Error) -> DeleteRuleError {
        DeleteRuleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteRuleError {
    fn from(err: CredentialsError) -> DeleteRuleError {
        DeleteRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteRuleError {
    fn from(err: HttpDispatchError) -> DeleteRuleError {
        DeleteRuleError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteRuleError {
    fn from(err: io::Error) -> DeleteRuleError {
        DeleteRuleError::HttpDispatch(HttpDispatchError::from(err))
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
            DeleteRuleError::Validation(ref cause) => cause,
            DeleteRuleError::Credentials(ref err) => err.description(),
            DeleteRuleError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteRuleError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeEventBusError {
    pub fn from_body(body: &str) -> DescribeEventBusError {
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
                    "InternalException" => {
                        DescribeEventBusError::Internal(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DescribeEventBusError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeEventBusError::Validation(error_message.to_string())
                    }
                    _ => DescribeEventBusError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeEventBusError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeEventBusError {
    fn from(err: serde_json::error::Error) -> DescribeEventBusError {
        DescribeEventBusError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeEventBusError {
    fn from(err: CredentialsError) -> DescribeEventBusError {
        DescribeEventBusError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeEventBusError {
    fn from(err: HttpDispatchError) -> DescribeEventBusError {
        DescribeEventBusError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeEventBusError {
    fn from(err: io::Error) -> DescribeEventBusError {
        DescribeEventBusError::HttpDispatch(HttpDispatchError::from(err))
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
            DescribeEventBusError::Validation(ref cause) => cause,
            DescribeEventBusError::Credentials(ref err) => err.description(),
            DescribeEventBusError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeEventBusError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeRuleError {
    pub fn from_body(body: &str) -> DescribeRuleError {
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
                    "InternalException" => DescribeRuleError::Internal(String::from(error_message)),
                    "ResourceNotFoundException" => {
                        DescribeRuleError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeRuleError::Validation(error_message.to_string())
                    }
                    _ => DescribeRuleError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeRuleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeRuleError {
    fn from(err: serde_json::error::Error) -> DescribeRuleError {
        DescribeRuleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeRuleError {
    fn from(err: CredentialsError) -> DescribeRuleError {
        DescribeRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeRuleError {
    fn from(err: HttpDispatchError) -> DescribeRuleError {
        DescribeRuleError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeRuleError {
    fn from(err: io::Error) -> DescribeRuleError {
        DescribeRuleError::HttpDispatch(HttpDispatchError::from(err))
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
            DescribeRuleError::Validation(ref cause) => cause,
            DescribeRuleError::Credentials(ref err) => err.description(),
            DescribeRuleError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeRuleError::Unknown(ref cause) => cause,
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
    /// <p>An entity that you specified does not exist.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DisableRuleError {
    pub fn from_body(body: &str) -> DisableRuleError {
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
                    "ConcurrentModificationException" => {
                        DisableRuleError::ConcurrentModification(String::from(error_message))
                    }
                    "InternalException" => DisableRuleError::Internal(String::from(error_message)),
                    "ResourceNotFoundException" => {
                        DisableRuleError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DisableRuleError::Validation(error_message.to_string())
                    }
                    _ => DisableRuleError::Unknown(String::from(body)),
                }
            }
            Err(_) => DisableRuleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DisableRuleError {
    fn from(err: serde_json::error::Error) -> DisableRuleError {
        DisableRuleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DisableRuleError {
    fn from(err: CredentialsError) -> DisableRuleError {
        DisableRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisableRuleError {
    fn from(err: HttpDispatchError) -> DisableRuleError {
        DisableRuleError::HttpDispatch(err)
    }
}
impl From<io::Error> for DisableRuleError {
    fn from(err: io::Error) -> DisableRuleError {
        DisableRuleError::HttpDispatch(HttpDispatchError::from(err))
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
            DisableRuleError::ResourceNotFound(ref cause) => cause,
            DisableRuleError::Validation(ref cause) => cause,
            DisableRuleError::Credentials(ref err) => err.description(),
            DisableRuleError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DisableRuleError::Unknown(ref cause) => cause,
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
    /// <p>An entity that you specified does not exist.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl EnableRuleError {
    pub fn from_body(body: &str) -> EnableRuleError {
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
                    "ConcurrentModificationException" => {
                        EnableRuleError::ConcurrentModification(String::from(error_message))
                    }
                    "InternalException" => EnableRuleError::Internal(String::from(error_message)),
                    "ResourceNotFoundException" => {
                        EnableRuleError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => EnableRuleError::Validation(error_message.to_string()),
                    _ => EnableRuleError::Unknown(String::from(body)),
                }
            }
            Err(_) => EnableRuleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for EnableRuleError {
    fn from(err: serde_json::error::Error) -> EnableRuleError {
        EnableRuleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for EnableRuleError {
    fn from(err: CredentialsError) -> EnableRuleError {
        EnableRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for EnableRuleError {
    fn from(err: HttpDispatchError) -> EnableRuleError {
        EnableRuleError::HttpDispatch(err)
    }
}
impl From<io::Error> for EnableRuleError {
    fn from(err: io::Error) -> EnableRuleError {
        EnableRuleError::HttpDispatch(HttpDispatchError::from(err))
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
            EnableRuleError::ResourceNotFound(ref cause) => cause,
            EnableRuleError::Validation(ref cause) => cause,
            EnableRuleError::Credentials(ref err) => err.description(),
            EnableRuleError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            EnableRuleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListRuleNamesByTarget
#[derive(Debug, PartialEq)]
pub enum ListRuleNamesByTargetError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListRuleNamesByTargetError {
    pub fn from_body(body: &str) -> ListRuleNamesByTargetError {
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
                    "InternalException" => {
                        ListRuleNamesByTargetError::Internal(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListRuleNamesByTargetError::Validation(error_message.to_string())
                    }
                    _ => ListRuleNamesByTargetError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListRuleNamesByTargetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListRuleNamesByTargetError {
    fn from(err: serde_json::error::Error) -> ListRuleNamesByTargetError {
        ListRuleNamesByTargetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListRuleNamesByTargetError {
    fn from(err: CredentialsError) -> ListRuleNamesByTargetError {
        ListRuleNamesByTargetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListRuleNamesByTargetError {
    fn from(err: HttpDispatchError) -> ListRuleNamesByTargetError {
        ListRuleNamesByTargetError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListRuleNamesByTargetError {
    fn from(err: io::Error) -> ListRuleNamesByTargetError {
        ListRuleNamesByTargetError::HttpDispatch(HttpDispatchError::from(err))
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
            ListRuleNamesByTargetError::Validation(ref cause) => cause,
            ListRuleNamesByTargetError::Credentials(ref err) => err.description(),
            ListRuleNamesByTargetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListRuleNamesByTargetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListRules
#[derive(Debug, PartialEq)]
pub enum ListRulesError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListRulesError {
    pub fn from_body(body: &str) -> ListRulesError {
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
                    "InternalException" => ListRulesError::Internal(String::from(error_message)),
                    "ValidationException" => ListRulesError::Validation(error_message.to_string()),
                    _ => ListRulesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListRulesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListRulesError {
    fn from(err: serde_json::error::Error) -> ListRulesError {
        ListRulesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListRulesError {
    fn from(err: CredentialsError) -> ListRulesError {
        ListRulesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListRulesError {
    fn from(err: HttpDispatchError) -> ListRulesError {
        ListRulesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListRulesError {
    fn from(err: io::Error) -> ListRulesError {
        ListRulesError::HttpDispatch(HttpDispatchError::from(err))
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
            ListRulesError::Validation(ref cause) => cause,
            ListRulesError::Credentials(ref err) => err.description(),
            ListRulesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListRulesError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListTargetsByRuleError {
    pub fn from_body(body: &str) -> ListTargetsByRuleError {
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
                    "InternalException" => {
                        ListTargetsByRuleError::Internal(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListTargetsByRuleError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListTargetsByRuleError::Validation(error_message.to_string())
                    }
                    _ => ListTargetsByRuleError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListTargetsByRuleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListTargetsByRuleError {
    fn from(err: serde_json::error::Error) -> ListTargetsByRuleError {
        ListTargetsByRuleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListTargetsByRuleError {
    fn from(err: CredentialsError) -> ListTargetsByRuleError {
        ListTargetsByRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTargetsByRuleError {
    fn from(err: HttpDispatchError) -> ListTargetsByRuleError {
        ListTargetsByRuleError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTargetsByRuleError {
    fn from(err: io::Error) -> ListTargetsByRuleError {
        ListTargetsByRuleError::HttpDispatch(HttpDispatchError::from(err))
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
            ListTargetsByRuleError::Validation(ref cause) => cause,
            ListTargetsByRuleError::Credentials(ref err) => err.description(),
            ListTargetsByRuleError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListTargetsByRuleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutEvents
#[derive(Debug, PartialEq)]
pub enum PutEventsError {
    /// <p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutEventsError {
    pub fn from_body(body: &str) -> PutEventsError {
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
                    "InternalException" => PutEventsError::Internal(String::from(error_message)),
                    "ValidationException" => PutEventsError::Validation(error_message.to_string()),
                    _ => PutEventsError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutEventsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutEventsError {
    fn from(err: serde_json::error::Error) -> PutEventsError {
        PutEventsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutEventsError {
    fn from(err: CredentialsError) -> PutEventsError {
        PutEventsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutEventsError {
    fn from(err: HttpDispatchError) -> PutEventsError {
        PutEventsError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutEventsError {
    fn from(err: io::Error) -> PutEventsError {
        PutEventsError::HttpDispatch(HttpDispatchError::from(err))
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
            PutEventsError::Validation(ref cause) => cause,
            PutEventsError::Credentials(ref err) => err.description(),
            PutEventsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutEventsError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutPermissionError {
    pub fn from_body(body: &str) -> PutPermissionError {
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
                    "ConcurrentModificationException" => {
                        PutPermissionError::ConcurrentModification(String::from(error_message))
                    }
                    "InternalException" => {
                        PutPermissionError::Internal(String::from(error_message))
                    }
                    "PolicyLengthExceededException" => {
                        PutPermissionError::PolicyLengthExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        PutPermissionError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        PutPermissionError::Validation(error_message.to_string())
                    }
                    _ => PutPermissionError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutPermissionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutPermissionError {
    fn from(err: serde_json::error::Error) -> PutPermissionError {
        PutPermissionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutPermissionError {
    fn from(err: CredentialsError) -> PutPermissionError {
        PutPermissionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutPermissionError {
    fn from(err: HttpDispatchError) -> PutPermissionError {
        PutPermissionError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutPermissionError {
    fn from(err: io::Error) -> PutPermissionError {
        PutPermissionError::HttpDispatch(HttpDispatchError::from(err))
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
            PutPermissionError::Validation(ref cause) => cause,
            PutPermissionError::Credentials(ref err) => err.description(),
            PutPermissionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutPermissionError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutRuleError {
    pub fn from_body(body: &str) -> PutRuleError {
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
                    "ConcurrentModificationException" => {
                        PutRuleError::ConcurrentModification(String::from(error_message))
                    }
                    "InternalException" => PutRuleError::Internal(String::from(error_message)),
                    "InvalidEventPatternException" => {
                        PutRuleError::InvalidEventPattern(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        PutRuleError::LimitExceeded(String::from(error_message))
                    }
                    "ValidationException" => PutRuleError::Validation(error_message.to_string()),
                    _ => PutRuleError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutRuleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutRuleError {
    fn from(err: serde_json::error::Error) -> PutRuleError {
        PutRuleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutRuleError {
    fn from(err: CredentialsError) -> PutRuleError {
        PutRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutRuleError {
    fn from(err: HttpDispatchError) -> PutRuleError {
        PutRuleError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutRuleError {
    fn from(err: io::Error) -> PutRuleError {
        PutRuleError::HttpDispatch(HttpDispatchError::from(err))
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
            PutRuleError::Validation(ref cause) => cause,
            PutRuleError::Credentials(ref err) => err.description(),
            PutRuleError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutRuleError::Unknown(ref cause) => cause,
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
    /// <p>An entity that you specified does not exist.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutTargetsError {
    pub fn from_body(body: &str) -> PutTargetsError {
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
                    "ConcurrentModificationException" => {
                        PutTargetsError::ConcurrentModification(String::from(error_message))
                    }
                    "InternalException" => PutTargetsError::Internal(String::from(error_message)),
                    "LimitExceededException" => {
                        PutTargetsError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        PutTargetsError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => PutTargetsError::Validation(error_message.to_string()),
                    _ => PutTargetsError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutTargetsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutTargetsError {
    fn from(err: serde_json::error::Error) -> PutTargetsError {
        PutTargetsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutTargetsError {
    fn from(err: CredentialsError) -> PutTargetsError {
        PutTargetsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutTargetsError {
    fn from(err: HttpDispatchError) -> PutTargetsError {
        PutTargetsError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutTargetsError {
    fn from(err: io::Error) -> PutTargetsError {
        PutTargetsError::HttpDispatch(HttpDispatchError::from(err))
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
            PutTargetsError::ResourceNotFound(ref cause) => cause,
            PutTargetsError::Validation(ref cause) => cause,
            PutTargetsError::Credentials(ref err) => err.description(),
            PutTargetsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutTargetsError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RemovePermissionError {
    pub fn from_body(body: &str) -> RemovePermissionError {
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
                    "ConcurrentModificationException" => {
                        RemovePermissionError::ConcurrentModification(String::from(error_message))
                    }
                    "InternalException" => {
                        RemovePermissionError::Internal(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        RemovePermissionError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        RemovePermissionError::Validation(error_message.to_string())
                    }
                    _ => RemovePermissionError::Unknown(String::from(body)),
                }
            }
            Err(_) => RemovePermissionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RemovePermissionError {
    fn from(err: serde_json::error::Error) -> RemovePermissionError {
        RemovePermissionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RemovePermissionError {
    fn from(err: CredentialsError) -> RemovePermissionError {
        RemovePermissionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RemovePermissionError {
    fn from(err: HttpDispatchError) -> RemovePermissionError {
        RemovePermissionError::HttpDispatch(err)
    }
}
impl From<io::Error> for RemovePermissionError {
    fn from(err: io::Error) -> RemovePermissionError {
        RemovePermissionError::HttpDispatch(HttpDispatchError::from(err))
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
            RemovePermissionError::Validation(ref cause) => cause,
            RemovePermissionError::Credentials(ref err) => err.description(),
            RemovePermissionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RemovePermissionError::Unknown(ref cause) => cause,
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
    /// <p>An entity that you specified does not exist.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RemoveTargetsError {
    pub fn from_body(body: &str) -> RemoveTargetsError {
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
                    "ConcurrentModificationException" => {
                        RemoveTargetsError::ConcurrentModification(String::from(error_message))
                    }
                    "InternalException" => {
                        RemoveTargetsError::Internal(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        RemoveTargetsError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        RemoveTargetsError::Validation(error_message.to_string())
                    }
                    _ => RemoveTargetsError::Unknown(String::from(body)),
                }
            }
            Err(_) => RemoveTargetsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RemoveTargetsError {
    fn from(err: serde_json::error::Error) -> RemoveTargetsError {
        RemoveTargetsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RemoveTargetsError {
    fn from(err: CredentialsError) -> RemoveTargetsError {
        RemoveTargetsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RemoveTargetsError {
    fn from(err: HttpDispatchError) -> RemoveTargetsError {
        RemoveTargetsError::HttpDispatch(err)
    }
}
impl From<io::Error> for RemoveTargetsError {
    fn from(err: io::Error) -> RemoveTargetsError {
        RemoveTargetsError::HttpDispatch(HttpDispatchError::from(err))
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
            RemoveTargetsError::ResourceNotFound(ref cause) => cause,
            RemoveTargetsError::Validation(ref cause) => cause,
            RemoveTargetsError::Credentials(ref err) => err.description(),
            RemoveTargetsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RemoveTargetsError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl TestEventPatternError {
    pub fn from_body(body: &str) -> TestEventPatternError {
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
                    "InternalException" => {
                        TestEventPatternError::Internal(String::from(error_message))
                    }
                    "InvalidEventPatternException" => {
                        TestEventPatternError::InvalidEventPattern(String::from(error_message))
                    }
                    "ValidationException" => {
                        TestEventPatternError::Validation(error_message.to_string())
                    }
                    _ => TestEventPatternError::Unknown(String::from(body)),
                }
            }
            Err(_) => TestEventPatternError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for TestEventPatternError {
    fn from(err: serde_json::error::Error) -> TestEventPatternError {
        TestEventPatternError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for TestEventPatternError {
    fn from(err: CredentialsError) -> TestEventPatternError {
        TestEventPatternError::Credentials(err)
    }
}
impl From<HttpDispatchError> for TestEventPatternError {
    fn from(err: HttpDispatchError) -> TestEventPatternError {
        TestEventPatternError::HttpDispatch(err)
    }
}
impl From<io::Error> for TestEventPatternError {
    fn from(err: io::Error) -> TestEventPatternError {
        TestEventPatternError::HttpDispatch(HttpDispatchError::from(err))
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
            TestEventPatternError::Validation(ref cause) => cause,
            TestEventPatternError::Credentials(ref err) => err.description(),
            TestEventPatternError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            TestEventPatternError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon CloudWatch Events API. Amazon CloudWatch Events clients implement this trait.
pub trait CloudWatchEvents {
    /// <p>Deletes the specified rule.</p> <p>You must remove all targets from a rule using <a>RemoveTargets</a> before you can delete the rule.</p> <p>When you delete a rule, incoming events might continue to match to the deleted rule. Please allow a short period of time for changes to take effect.</p>
    fn delete_rule(&self, input: DeleteRuleRequest) -> RusotoFuture<(), DeleteRuleError>;

    /// <p>Displays the external AWS accounts that are permitted to write events to your account using your account's event bus, and the associated policy. To enable your account to receive events from other accounts, use <a>PutPermission</a>.</p>
    fn describe_event_bus(&self) -> RusotoFuture<DescribeEventBusResponse, DescribeEventBusError>;

    /// <p>Describes the specified rule.</p>
    fn describe_rule(
        &self,
        input: DescribeRuleRequest,
    ) -> RusotoFuture<DescribeRuleResponse, DescribeRuleError>;

    /// <p>Disables the specified rule. A disabled rule won't match any events, and won't self-trigger if it has a schedule expression.</p> <p>When you disable a rule, incoming events might continue to match to the disabled rule. Please allow a short period of time for changes to take effect.</p>
    fn disable_rule(&self, input: DisableRuleRequest) -> RusotoFuture<(), DisableRuleError>;

    /// <p>Enables the specified rule. If the rule does not exist, the operation fails.</p> <p>When you enable a rule, incoming events might not immediately start matching to a newly enabled rule. Please allow a short period of time for changes to take effect.</p>
    fn enable_rule(&self, input: EnableRuleRequest) -> RusotoFuture<(), EnableRuleError>;

    /// <p>Lists the rules for the specified target. You can see which of the rules in Amazon CloudWatch Events can invoke a specific target in your account.</p>
    fn list_rule_names_by_target(
        &self,
        input: ListRuleNamesByTargetRequest,
    ) -> RusotoFuture<ListRuleNamesByTargetResponse, ListRuleNamesByTargetError>;

    /// <p>Lists your Amazon CloudWatch Events rules. You can either list all the rules or you can provide a prefix to match to the rule names.</p>
    fn list_rules(
        &self,
        input: ListRulesRequest,
    ) -> RusotoFuture<ListRulesResponse, ListRulesError>;

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

    /// <p>Running <code>PutPermission</code> permits the specified AWS account to put events to your account's default <i>event bus</i>. CloudWatch Events rules in your account are triggered by these events arriving to your default event bus. </p> <p>For another account to send events to your account, that external account must have a CloudWatch Events rule with your account's default event bus as a target.</p> <p>To enable multiple AWS accounts to put events to your default event bus, run <code>PutPermission</code> once for each of these accounts.</p> <p>The permission policy on the default event bus cannot exceed 10KB in size.</p>
    fn put_permission(&self, input: PutPermissionRequest) -> RusotoFuture<(), PutPermissionError>;

    /// <p>Creates or updates the specified rule. Rules are enabled by default, or based on value of the state. You can disable a rule using <a>DisableRule</a>.</p> <p>If you are updating an existing rule, the rule is completely replaced with what you specify in this <code>PutRule</code> command. If you omit arguments in <code>PutRule</code>, the old values for those arguments are not kept. Instead, they are replaced with null values.</p> <p>When you create or update a rule, incoming events might not immediately start matching to new or updated rules. Please allow a short period of time for changes to take effect.</p> <p>A rule must contain at least an EventPattern or ScheduleExpression. Rules with EventPatterns are triggered when a matching event is observed. Rules with ScheduleExpressions self-trigger based on the given schedule. A rule can have both an EventPattern and a ScheduleExpression, in which case the rule triggers on matching events as well as on a schedule.</p> <p>Most services in AWS treat : or / as the same character in Amazon Resource Names (ARNs). However, CloudWatch Events uses an exact match in event patterns and rules. Be sure to use the correct ARN characters when creating event patterns so that they match the ARN syntax in the event you want to match.</p>
    fn put_rule(&self, input: PutRuleRequest) -> RusotoFuture<PutRuleResponse, PutRuleError>;

    /// <p>Adds the specified targets to the specified rule, or updates the targets if they are already associated with the rule.</p> <p>Targets are the resources that are invoked when a rule is triggered.</p> <p>You can configure the following as targets for CloudWatch Events:</p> <ul> <li> <p>EC2 instances</p> </li> <li> <p>AWS Lambda functions</p> </li> <li> <p>Streams in Amazon Kinesis Streams</p> </li> <li> <p>Delivery streams in Amazon Kinesis Firehose</p> </li> <li> <p>Amazon ECS tasks</p> </li> <li> <p>AWS Step Functions state machines</p> </li> <li> <p>AWS Batch jobs</p> </li> <li> <p>Pipelines in Amazon Code Pipeline</p> </li> <li> <p>Amazon Inspector assessment templates</p> </li> <li> <p>Amazon SNS topics</p> </li> <li> <p>Amazon SQS queues, including FIFO queues</p> </li> <li> <p>The default event bus of another AWS account</p> </li> </ul> <p>Note that creating rules with built-in targets is supported only in the AWS Management Console.</p> <p>For some target types, <code>PutTargets</code> provides target-specific parameters. If the target is an Amazon Kinesis stream, you can optionally specify which shard the event goes to by using the <code>KinesisParameters</code> argument. To invoke a command on multiple EC2 instances with one rule, you can use the <code>RunCommandParameters</code> field.</p> <p>To be able to make API calls against the resources that you own, Amazon CloudWatch Events needs the appropriate permissions. For AWS Lambda and Amazon SNS resources, CloudWatch Events relies on resource-based policies. For EC2 instances, Amazon Kinesis streams, and AWS Step Functions state machines, CloudWatch Events relies on IAM roles that you specify in the <code>RoleARN</code> argument in <code>PutTargets</code>. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudWatch/latest/events/auth-and-access-control-cwe.html">Authentication and Access Control</a> in the <i>Amazon CloudWatch Events User Guide</i>.</p> <p>If another AWS account is in the same region and has granted you permission (using <code>PutPermission</code>), you can send events to that account by setting that account's event bus as a target of the rules in your account. To send the matched events to the other account, specify that account's event bus as the <code>Arn</code> when you run <code>PutTargets</code>. If your account sends events to another account, your account is charged for each sent event. Each event sent to antoher account is charged as a custom event. The account receiving the event is not charged. For more information on pricing, see <a href="https://aws.amazon.com/cloudwatch/pricing/">Amazon CloudWatch Pricing</a>.</p> <p>For more information about enabling cross-account events, see <a>PutPermission</a>.</p> <p> <b>Input</b>, <b>InputPath</b> and <b>InputTransformer</b> are mutually exclusive and optional parameters of a target. When a rule is triggered due to a matched event:</p> <ul> <li> <p>If none of the following arguments are specified for a target, then the entire event is passed to the target in JSON form (unless the target is Amazon EC2 Run Command or Amazon ECS task, in which case nothing from the event is passed to the target).</p> </li> <li> <p>If <b>Input</b> is specified in the form of valid JSON, then the matched event is overridden with this constant.</p> </li> <li> <p>If <b>InputPath</b> is specified in the form of JSONPath (for example, <code>$.detail</code>), then only the part of the event specified in the path is passed to the target (for example, only the detail part of the event is passed).</p> </li> <li> <p>If <b>InputTransformer</b> is specified, then one or more specified JSONPaths are extracted from the event and used as values in a template that you specify as the input to the target.</p> </li> </ul> <p>When you specify <code>InputPath</code> or <code>InputTransformer</code>, you must use JSON dot notation, not bracket notation.</p> <p>When you add targets to a rule and the associated rule triggers soon after, new or updated targets might not be immediately invoked. Please allow a short period of time for changes to take effect.</p> <p>This action can partially fail if too many requests are made at the same time. If that happens, <code>FailedEntryCount</code> is non-zero in the response and each entry in <code>FailedEntries</code> provides the ID of the failed target and the error code.</p>
    fn put_targets(
        &self,
        input: PutTargetsRequest,
    ) -> RusotoFuture<PutTargetsResponse, PutTargetsError>;

    /// <p>Revokes the permission of another AWS account to be able to put events to your default event bus. Specify the account to revoke by the <code>StatementId</code> value that you associated with the account when you granted it permission with <code>PutPermission</code>. You can find the <code>StatementId</code> by using <a>DescribeEventBus</a>.</p>
    fn remove_permission(
        &self,
        input: RemovePermissionRequest,
    ) -> RusotoFuture<(), RemovePermissionError>;

    /// <p>Removes the specified targets from the specified rule. When the rule is triggered, those targets are no longer be invoked.</p> <p>When you remove a target, when the associated rule triggers, removed targets might continue to be invoked. Please allow a short period of time for changes to take effect.</p> <p>This action can partially fail if too many requests are made at the same time. If that happens, <code>FailedEntryCount</code> is non-zero in the response and each entry in <code>FailedEntries</code> provides the ID of the failed target and the error code.</p>
    fn remove_targets(
        &self,
        input: RemoveTargetsRequest,
    ) -> RusotoFuture<RemoveTargetsResponse, RemoveTargetsError>;

    /// <p>Tests whether the specified event pattern matches the provided event.</p> <p>Most services in AWS treat : or / as the same character in Amazon Resource Names (ARNs). However, CloudWatch Events uses an exact match in event patterns and rules. Be sure to use the correct ARN characters when creating event patterns so that they match the ARN syntax in the event you want to match.</p>
    fn test_event_pattern(
        &self,
        input: TestEventPatternRequest,
    ) -> RusotoFuture<TestEventPatternResponse, TestEventPatternError>;
}
/// A client for the Amazon CloudWatch Events API.
pub struct CloudWatchEventsClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl CloudWatchEventsClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> CloudWatchEventsClient {
        CloudWatchEventsClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> CloudWatchEventsClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        CloudWatchEventsClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> CloudWatchEvents for CloudWatchEventsClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p>Deletes the specified rule.</p> <p>You must remove all targets from a rule using <a>RemoveTargets</a> before you can delete the rule.</p> <p>When you delete a rule, incoming events might continue to match to the deleted rule. Please allow a short period of time for changes to take effect.</p>
    fn delete_rule(&self, input: DeleteRuleRequest) -> RusotoFuture<(), DeleteRuleError> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.DeleteRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteRuleError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Displays the external AWS accounts that are permitted to write events to your account using your account's event bus, and the associated policy. To enable your account to receive events from other accounts, use <a>PutPermission</a>.</p>
    fn describe_event_bus(&self) -> RusotoFuture<DescribeEventBusResponse, DescribeEventBusError> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.DescribeEventBus");
        request.set_payload(Some(b"{}".to_vec()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeEventBusResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeEventBusError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Describes the specified rule.</p>
    fn describe_rule(
        &self,
        input: DescribeRuleRequest,
    ) -> RusotoFuture<DescribeRuleResponse, DescribeRuleError> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.DescribeRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeRuleResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeRuleError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Disables the specified rule. A disabled rule won't match any events, and won't self-trigger if it has a schedule expression.</p> <p>When you disable a rule, incoming events might continue to match to the disabled rule. Please allow a short period of time for changes to take effect.</p>
    fn disable_rule(&self, input: DisableRuleRequest) -> RusotoFuture<(), DisableRuleError> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.DisableRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DisableRuleError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Enables the specified rule. If the rule does not exist, the operation fails.</p> <p>When you enable a rule, incoming events might not immediately start matching to a newly enabled rule. Please allow a short period of time for changes to take effect.</p>
    fn enable_rule(&self, input: EnableRuleRequest) -> RusotoFuture<(), EnableRuleError> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.EnableRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(EnableRuleError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
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
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListRuleNamesByTargetResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListRuleNamesByTargetError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists your Amazon CloudWatch Events rules. You can either list all the rules or you can provide a prefix to match to the rule names.</p>
    fn list_rules(
        &self,
        input: ListRulesRequest,
    ) -> RusotoFuture<ListRulesResponse, ListRulesError> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.ListRules");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListRulesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListRulesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
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
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListTargetsByRuleResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListTargetsByRuleError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
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
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PutEventsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PutEventsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Running <code>PutPermission</code> permits the specified AWS account to put events to your account's default <i>event bus</i>. CloudWatch Events rules in your account are triggered by these events arriving to your default event bus. </p> <p>For another account to send events to your account, that external account must have a CloudWatch Events rule with your account's default event bus as a target.</p> <p>To enable multiple AWS accounts to put events to your default event bus, run <code>PutPermission</code> once for each of these accounts.</p> <p>The permission policy on the default event bus cannot exceed 10KB in size.</p>
    fn put_permission(&self, input: PutPermissionRequest) -> RusotoFuture<(), PutPermissionError> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.PutPermission");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PutPermissionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates or updates the specified rule. Rules are enabled by default, or based on value of the state. You can disable a rule using <a>DisableRule</a>.</p> <p>If you are updating an existing rule, the rule is completely replaced with what you specify in this <code>PutRule</code> command. If you omit arguments in <code>PutRule</code>, the old values for those arguments are not kept. Instead, they are replaced with null values.</p> <p>When you create or update a rule, incoming events might not immediately start matching to new or updated rules. Please allow a short period of time for changes to take effect.</p> <p>A rule must contain at least an EventPattern or ScheduleExpression. Rules with EventPatterns are triggered when a matching event is observed. Rules with ScheduleExpressions self-trigger based on the given schedule. A rule can have both an EventPattern and a ScheduleExpression, in which case the rule triggers on matching events as well as on a schedule.</p> <p>Most services in AWS treat : or / as the same character in Amazon Resource Names (ARNs). However, CloudWatch Events uses an exact match in event patterns and rules. Be sure to use the correct ARN characters when creating event patterns so that they match the ARN syntax in the event you want to match.</p>
    fn put_rule(&self, input: PutRuleRequest) -> RusotoFuture<PutRuleResponse, PutRuleError> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.PutRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PutRuleResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PutRuleError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Adds the specified targets to the specified rule, or updates the targets if they are already associated with the rule.</p> <p>Targets are the resources that are invoked when a rule is triggered.</p> <p>You can configure the following as targets for CloudWatch Events:</p> <ul> <li> <p>EC2 instances</p> </li> <li> <p>AWS Lambda functions</p> </li> <li> <p>Streams in Amazon Kinesis Streams</p> </li> <li> <p>Delivery streams in Amazon Kinesis Firehose</p> </li> <li> <p>Amazon ECS tasks</p> </li> <li> <p>AWS Step Functions state machines</p> </li> <li> <p>AWS Batch jobs</p> </li> <li> <p>Pipelines in Amazon Code Pipeline</p> </li> <li> <p>Amazon Inspector assessment templates</p> </li> <li> <p>Amazon SNS topics</p> </li> <li> <p>Amazon SQS queues, including FIFO queues</p> </li> <li> <p>The default event bus of another AWS account</p> </li> </ul> <p>Note that creating rules with built-in targets is supported only in the AWS Management Console.</p> <p>For some target types, <code>PutTargets</code> provides target-specific parameters. If the target is an Amazon Kinesis stream, you can optionally specify which shard the event goes to by using the <code>KinesisParameters</code> argument. To invoke a command on multiple EC2 instances with one rule, you can use the <code>RunCommandParameters</code> field.</p> <p>To be able to make API calls against the resources that you own, Amazon CloudWatch Events needs the appropriate permissions. For AWS Lambda and Amazon SNS resources, CloudWatch Events relies on resource-based policies. For EC2 instances, Amazon Kinesis streams, and AWS Step Functions state machines, CloudWatch Events relies on IAM roles that you specify in the <code>RoleARN</code> argument in <code>PutTargets</code>. For more information, see <a href="http://docs.aws.amazon.com/AmazonCloudWatch/latest/events/auth-and-access-control-cwe.html">Authentication and Access Control</a> in the <i>Amazon CloudWatch Events User Guide</i>.</p> <p>If another AWS account is in the same region and has granted you permission (using <code>PutPermission</code>), you can send events to that account by setting that account's event bus as a target of the rules in your account. To send the matched events to the other account, specify that account's event bus as the <code>Arn</code> when you run <code>PutTargets</code>. If your account sends events to another account, your account is charged for each sent event. Each event sent to antoher account is charged as a custom event. The account receiving the event is not charged. For more information on pricing, see <a href="https://aws.amazon.com/cloudwatch/pricing/">Amazon CloudWatch Pricing</a>.</p> <p>For more information about enabling cross-account events, see <a>PutPermission</a>.</p> <p> <b>Input</b>, <b>InputPath</b> and <b>InputTransformer</b> are mutually exclusive and optional parameters of a target. When a rule is triggered due to a matched event:</p> <ul> <li> <p>If none of the following arguments are specified for a target, then the entire event is passed to the target in JSON form (unless the target is Amazon EC2 Run Command or Amazon ECS task, in which case nothing from the event is passed to the target).</p> </li> <li> <p>If <b>Input</b> is specified in the form of valid JSON, then the matched event is overridden with this constant.</p> </li> <li> <p>If <b>InputPath</b> is specified in the form of JSONPath (for example, <code>$.detail</code>), then only the part of the event specified in the path is passed to the target (for example, only the detail part of the event is passed).</p> </li> <li> <p>If <b>InputTransformer</b> is specified, then one or more specified JSONPaths are extracted from the event and used as values in a template that you specify as the input to the target.</p> </li> </ul> <p>When you specify <code>InputPath</code> or <code>InputTransformer</code>, you must use JSON dot notation, not bracket notation.</p> <p>When you add targets to a rule and the associated rule triggers soon after, new or updated targets might not be immediately invoked. Please allow a short period of time for changes to take effect.</p> <p>This action can partially fail if too many requests are made at the same time. If that happens, <code>FailedEntryCount</code> is non-zero in the response and each entry in <code>FailedEntries</code> provides the ID of the failed target and the error code.</p>
    fn put_targets(
        &self,
        input: PutTargetsRequest,
    ) -> RusotoFuture<PutTargetsResponse, PutTargetsError> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.PutTargets");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PutTargetsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PutTargetsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
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
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RemovePermissionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Removes the specified targets from the specified rule. When the rule is triggered, those targets are no longer be invoked.</p> <p>When you remove a target, when the associated rule triggers, removed targets might continue to be invoked. Please allow a short period of time for changes to take effect.</p> <p>This action can partially fail if too many requests are made at the same time. If that happens, <code>FailedEntryCount</code> is non-zero in the response and each entry in <code>FailedEntries</code> provides the ID of the failed target and the error code.</p>
    fn remove_targets(
        &self,
        input: RemoveTargetsRequest,
    ) -> RusotoFuture<RemoveTargetsResponse, RemoveTargetsError> {
        let mut request = SignedRequest::new("POST", "events", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.RemoveTargets");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<RemoveTargetsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RemoveTargetsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
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
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<TestEventPatternResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(TestEventPatternError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }
}

#[cfg(test)]
mod protocol_tests {}
