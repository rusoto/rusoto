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
/// <p>Contains details about an activity that failed during an execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ActivityFailedEventDetails {
    /// <p>A more detailed explanation of the cause of the failure.</p>
    #[serde(rename = "cause")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    /// <p>The error code of the failure.</p>
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// <p>Contains details about an activity.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ActivityListItem {
    /// <p>The Amazon Resource Name (ARN) that identifies the activity.</p>
    #[serde(rename = "activityArn")]
    pub activity_arn: String,
    /// <p>The date the activity is created.</p>
    #[serde(rename = "creationDate")]
    pub creation_date: f64,
    /// <p><p>The name of the activity.</p> <p>A name must <i>not</i> contain:</p> <ul> <li> <p>white space</p> </li> <li> <p>brackets <code>&lt; &gt; { } [ ]</code> </p> </li> <li> <p>wildcard characters <code>? *</code> </p> </li> <li> <p>special characters <code>&quot; # % \ ^ | ~ ` $ &amp; , ; : /</code> </p> </li> <li> <p>control characters (<code>U+0000-001F</code>, <code>U+007F-009F</code>)</p> </li> </ul></p>
    #[serde(rename = "name")]
    pub name: String,
}

/// <p>Contains details about an activity schedule failure that occurred during an execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ActivityScheduleFailedEventDetails {
    /// <p>A more detailed explanation of the cause of the failure.</p>
    #[serde(rename = "cause")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    /// <p>The error code of the failure.</p>
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// <p>Contains details about an activity scheduled during an execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ActivityScheduledEventDetails {
    /// <p>The maximum allowed duration between two heartbeats for the activity task.</p>
    #[serde(rename = "heartbeatInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heartbeat_in_seconds: Option<i64>,
    /// <p>The JSON data input to the activity task.</p>
    #[serde(rename = "input")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the scheduled activity.</p>
    #[serde(rename = "resource")]
    pub resource: String,
    /// <p>The maximum allowed duration of the activity task.</p>
    #[serde(rename = "timeoutInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_seconds: Option<i64>,
}

/// <p>Contains details about the start of an activity during an execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ActivityStartedEventDetails {
    /// <p>The name of the worker that the task is assigned to. These names are provided by the workers when calling <a>GetActivityTask</a>.</p>
    #[serde(rename = "workerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_name: Option<String>,
}

/// <p>Contains details about an activity that successfully terminated during an execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ActivitySucceededEventDetails {
    /// <p>The JSON data output by the activity task.</p>
    #[serde(rename = "output")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
}

/// <p>Contains details about an activity timeout that occurred during an execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ActivityTimedOutEventDetails {
    /// <p>A more detailed explanation of the cause of the timeout.</p>
    #[serde(rename = "cause")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    /// <p>The error code of the failure.</p>
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CloudWatchLogsLogGroup {
    /// <p>The ARN of the the CloudWatch log group to which you want your logs emitted to. The ARN must end with <code>:*</code> </p>
    #[serde(rename = "logGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateActivityInput {
    /// <p><p>The name of the activity to create. This name must be unique for your AWS account and region for 90 days. For more information, see <a href="https://docs.aws.amazon.com/step-functions/latest/dg/limits.html#service-limits-state-machine-executions"> Limits Related to State Machine Executions</a> in the <i>AWS Step Functions Developer Guide</i>.</p> <p>A name must <i>not</i> contain:</p> <ul> <li> <p>white space</p> </li> <li> <p>brackets <code>&lt; &gt; { } [ ]</code> </p> </li> <li> <p>wildcard characters <code>? *</code> </p> </li> <li> <p>special characters <code>&quot; # % \ ^ | ~ ` $ &amp; , ; : /</code> </p> </li> <li> <p>control characters (<code>U+0000-001F</code>, <code>U+007F-009F</code>)</p> </li> </ul></p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The list of tags to add to a resource.</p> <p>An array of key-value pairs. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>, and <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_iam-tags.html">Controlling Access Using IAM Tags</a>.</p> <p>Tags may only contain Unicode letters, digits, white space, or these symbols: <code>_ . : / = + - @</code>.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateActivityOutput {
    /// <p>The Amazon Resource Name (ARN) that identifies the created activity.</p>
    #[serde(rename = "activityArn")]
    pub activity_arn: String,
    /// <p>The date the activity is created.</p>
    #[serde(rename = "creationDate")]
    pub creation_date: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateStateMachineInput {
    /// <p>The Amazon States Language definition of the state machine. See <a href="https://docs.aws.amazon.com/step-functions/latest/dg/concepts-amazon-states-language.html">Amazon States Language</a>.</p>
    #[serde(rename = "definition")]
    pub definition: String,
    /// <p>Defines what execution history events are logged and where they are logged.</p>
    #[serde(rename = "loggingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_configuration: Option<LoggingConfiguration>,
    /// <p><p>The name of the state machine. </p> <p>A name must <i>not</i> contain:</p> <ul> <li> <p>white space</p> </li> <li> <p>brackets <code>&lt; &gt; { } [ ]</code> </p> </li> <li> <p>wildcard characters <code>? *</code> </p> </li> <li> <p>special characters <code>&quot; # % \ ^ | ~ ` $ &amp; , ; : /</code> </p> </li> <li> <p>control characters (<code>U+0000-001F</code>, <code>U+007F-009F</code>)</p> </li> </ul></p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The Amazon Resource Name (ARN) of the IAM role to use for this state machine.</p>
    #[serde(rename = "roleArn")]
    pub role_arn: String,
    /// <p>Tags to be added when creating a state machine.</p> <p>An array of key-value pairs. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>, and <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_iam-tags.html">Controlling Access Using IAM Tags</a>.</p> <p>Tags may only contain Unicode letters, digits, white space, or these symbols: <code>_ . : / = + - @</code>.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>Determines whether a Standard or Express state machine is created. If not set, Standard is created.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateStateMachineOutput {
    /// <p>The date the state machine is created.</p>
    #[serde(rename = "creationDate")]
    pub creation_date: f64,
    /// <p>The Amazon Resource Name (ARN) that identifies the created state machine.</p>
    #[serde(rename = "stateMachineArn")]
    pub state_machine_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteActivityInput {
    /// <p>The Amazon Resource Name (ARN) of the activity to delete.</p>
    #[serde(rename = "activityArn")]
    pub activity_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteActivityOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteStateMachineInput {
    /// <p>The Amazon Resource Name (ARN) of the state machine to delete.</p>
    #[serde(rename = "stateMachineArn")]
    pub state_machine_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteStateMachineOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeActivityInput {
    /// <p>The Amazon Resource Name (ARN) of the activity to describe.</p>
    #[serde(rename = "activityArn")]
    pub activity_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeActivityOutput {
    /// <p>The Amazon Resource Name (ARN) that identifies the activity.</p>
    #[serde(rename = "activityArn")]
    pub activity_arn: String,
    /// <p>The date the activity is created.</p>
    #[serde(rename = "creationDate")]
    pub creation_date: f64,
    /// <p><p>The name of the activity.</p> <p>A name must <i>not</i> contain:</p> <ul> <li> <p>white space</p> </li> <li> <p>brackets <code>&lt; &gt; { } [ ]</code> </p> </li> <li> <p>wildcard characters <code>? *</code> </p> </li> <li> <p>special characters <code>&quot; # % \ ^ | ~ ` $ &amp; , ; : /</code> </p> </li> <li> <p>control characters (<code>U+0000-001F</code>, <code>U+007F-009F</code>)</p> </li> </ul></p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeExecutionInput {
    /// <p>The Amazon Resource Name (ARN) of the execution to describe.</p>
    #[serde(rename = "executionArn")]
    pub execution_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeExecutionOutput {
    /// <p>The Amazon Resource Name (ARN) that identifies the execution.</p>
    #[serde(rename = "executionArn")]
    pub execution_arn: String,
    /// <p>The string that contains the JSON input data of the execution.</p>
    #[serde(rename = "input")]
    pub input: String,
    /// <p><p>The name of the execution.</p> <p>A name must <i>not</i> contain:</p> <ul> <li> <p>white space</p> </li> <li> <p>brackets <code>&lt; &gt; { } [ ]</code> </p> </li> <li> <p>wildcard characters <code>? *</code> </p> </li> <li> <p>special characters <code>&quot; # % \ ^ | ~ ` $ &amp; , ; : /</code> </p> </li> <li> <p>control characters (<code>U+0000-001F</code>, <code>U+007F-009F</code>)</p> </li> </ul></p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p><p>The JSON output data of the execution.</p> <note> <p>This field is set only if the execution succeeds. If the execution fails, this field is null.</p> </note></p>
    #[serde(rename = "output")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    /// <p>The date the execution is started.</p>
    #[serde(rename = "startDate")]
    pub start_date: f64,
    /// <p>The Amazon Resource Name (ARN) of the executed stated machine.</p>
    #[serde(rename = "stateMachineArn")]
    pub state_machine_arn: String,
    /// <p>The current status of the execution.</p>
    #[serde(rename = "status")]
    pub status: String,
    /// <p>If the execution has already ended, the date the execution stopped.</p>
    #[serde(rename = "stopDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_date: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeStateMachineForExecutionInput {
    /// <p>The Amazon Resource Name (ARN) of the execution you want state machine information for.</p>
    #[serde(rename = "executionArn")]
    pub execution_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeStateMachineForExecutionOutput {
    /// <p>The Amazon States Language definition of the state machine. See <a href="https://docs.aws.amazon.com/step-functions/latest/dg/concepts-amazon-states-language.html">Amazon States Language</a>.</p>
    #[serde(rename = "definition")]
    pub definition: String,
    /// <p>The name of the state machine associated with the execution.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The Amazon Resource Name (ARN) of the IAM role of the State Machine for the execution. </p>
    #[serde(rename = "roleArn")]
    pub role_arn: String,
    /// <p>The Amazon Resource Name (ARN) of the state machine associated with the execution.</p>
    #[serde(rename = "stateMachineArn")]
    pub state_machine_arn: String,
    /// <p>The date and time the state machine associated with an execution was updated. For a newly created state machine, this is the creation date.</p>
    #[serde(rename = "updateDate")]
    pub update_date: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeStateMachineInput {
    /// <p>The Amazon Resource Name (ARN) of the state machine to describe.</p>
    #[serde(rename = "stateMachineArn")]
    pub state_machine_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeStateMachineOutput {
    /// <p>The date the state machine is created.</p>
    #[serde(rename = "creationDate")]
    pub creation_date: f64,
    /// <p>The Amazon States Language definition of the state machine. See <a href="https://docs.aws.amazon.com/step-functions/latest/dg/concepts-amazon-states-language.html">Amazon States Language</a>.</p>
    #[serde(rename = "definition")]
    pub definition: String,
    /// <p><p/></p>
    #[serde(rename = "loggingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_configuration: Option<LoggingConfiguration>,
    /// <p><p>The name of the state machine.</p> <p>A name must <i>not</i> contain:</p> <ul> <li> <p>white space</p> </li> <li> <p>brackets <code>&lt; &gt; { } [ ]</code> </p> </li> <li> <p>wildcard characters <code>? *</code> </p> </li> <li> <p>special characters <code>&quot; # % \ ^ | ~ ` $ &amp; , ; : /</code> </p> </li> <li> <p>control characters (<code>U+0000-001F</code>, <code>U+007F-009F</code>)</p> </li> </ul></p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The Amazon Resource Name (ARN) of the IAM role used when creating this state machine. (The IAM role maintains security by granting Step Functions access to AWS resources.)</p>
    #[serde(rename = "roleArn")]
    pub role_arn: String,
    /// <p>The Amazon Resource Name (ARN) that identifies the state machine.</p>
    #[serde(rename = "stateMachineArn")]
    pub state_machine_arn: String,
    /// <p>The current status of the state machine.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p><p/></p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p>Contains details about an abort of an execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ExecutionAbortedEventDetails {
    /// <p>A more detailed explanation of the cause of the failure.</p>
    #[serde(rename = "cause")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    /// <p>The error code of the failure.</p>
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// <p>Contains details about an execution failure event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ExecutionFailedEventDetails {
    /// <p>A more detailed explanation of the cause of the failure.</p>
    #[serde(rename = "cause")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    /// <p>The error code of the failure.</p>
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// <p>Contains details about an execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ExecutionListItem {
    /// <p>The Amazon Resource Name (ARN) that identifies the execution.</p>
    #[serde(rename = "executionArn")]
    pub execution_arn: String,
    /// <p><p>The name of the execution.</p> <p>A name must <i>not</i> contain:</p> <ul> <li> <p>white space</p> </li> <li> <p>brackets <code>&lt; &gt; { } [ ]</code> </p> </li> <li> <p>wildcard characters <code>? *</code> </p> </li> <li> <p>special characters <code>&quot; # % \ ^ | ~ ` $ &amp; , ; : /</code> </p> </li> <li> <p>control characters (<code>U+0000-001F</code>, <code>U+007F-009F</code>)</p> </li> </ul></p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The date the execution started.</p>
    #[serde(rename = "startDate")]
    pub start_date: f64,
    /// <p>The Amazon Resource Name (ARN) of the executed state machine.</p>
    #[serde(rename = "stateMachineArn")]
    pub state_machine_arn: String,
    /// <p>The current status of the execution.</p>
    #[serde(rename = "status")]
    pub status: String,
    /// <p>If the execution already ended, the date the execution stopped.</p>
    #[serde(rename = "stopDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_date: Option<f64>,
}

/// <p>Contains details about the start of the execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ExecutionStartedEventDetails {
    /// <p>The JSON data input to the execution.</p>
    #[serde(rename = "input")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the IAM role used for executing AWS Lambda tasks.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

/// <p>Contains details about the successful termination of the execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ExecutionSucceededEventDetails {
    /// <p>The JSON data output by the execution.</p>
    #[serde(rename = "output")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
}

/// <p>Contains details about the execution timeout that occurred during the execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ExecutionTimedOutEventDetails {
    /// <p>A more detailed explanation of the cause of the timeout.</p>
    #[serde(rename = "cause")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    /// <p>The error code of the failure.</p>
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetActivityTaskInput {
    /// <p>The Amazon Resource Name (ARN) of the activity to retrieve tasks from (assigned when you create the task using <a>CreateActivity</a>.)</p>
    #[serde(rename = "activityArn")]
    pub activity_arn: String,
    /// <p>You can provide an arbitrary name in order to identify the worker that the task is assigned to. This name is used when it is logged in the execution history.</p>
    #[serde(rename = "workerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetActivityTaskOutput {
    /// <p>The string that contains the JSON input data for the task.</p>
    #[serde(rename = "input")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    /// <p>A token that identifies the scheduled task. This token must be copied and included in subsequent calls to <a>SendTaskHeartbeat</a>, <a>SendTaskSuccess</a> or <a>SendTaskFailure</a> in order to report the progress or completion of the task.</p>
    #[serde(rename = "taskToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetExecutionHistoryInput {
    /// <p>The Amazon Resource Name (ARN) of the execution.</p>
    #[serde(rename = "executionArn")]
    pub execution_arn: String,
    /// <p>The maximum number of results that are returned per call. You can use <code>nextToken</code> to obtain further pages of results. The default is 100 and the maximum allowed page size is 1000. A value of 0 uses the default.</p> <p>This is only an upper limit. The actual number of results returned per call might be fewer than the specified maximum.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If <code>nextToken</code> is returned, there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return an <i>HTTP 400 InvalidToken</i> error.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Lists events in descending order of their <code>timeStamp</code>.</p>
    #[serde(rename = "reverseOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_order: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetExecutionHistoryOutput {
    /// <p>The list of events that occurred in the execution.</p>
    #[serde(rename = "events")]
    pub events: Vec<HistoryEvent>,
    /// <p>If <code>nextToken</code> is returned, there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return an <i>HTTP 400 InvalidToken</i> error.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Contains details about the events of an execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HistoryEvent {
    #[serde(rename = "activityFailedEventDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_failed_event_details: Option<ActivityFailedEventDetails>,
    /// <p>Contains details about an activity schedule event that failed during an execution.</p>
    #[serde(rename = "activityScheduleFailedEventDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_schedule_failed_event_details: Option<ActivityScheduleFailedEventDetails>,
    #[serde(rename = "activityScheduledEventDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_scheduled_event_details: Option<ActivityScheduledEventDetails>,
    #[serde(rename = "activityStartedEventDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_started_event_details: Option<ActivityStartedEventDetails>,
    #[serde(rename = "activitySucceededEventDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_succeeded_event_details: Option<ActivitySucceededEventDetails>,
    #[serde(rename = "activityTimedOutEventDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_timed_out_event_details: Option<ActivityTimedOutEventDetails>,
    #[serde(rename = "executionAbortedEventDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_aborted_event_details: Option<ExecutionAbortedEventDetails>,
    #[serde(rename = "executionFailedEventDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_failed_event_details: Option<ExecutionFailedEventDetails>,
    #[serde(rename = "executionStartedEventDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_started_event_details: Option<ExecutionStartedEventDetails>,
    #[serde(rename = "executionSucceededEventDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_succeeded_event_details: Option<ExecutionSucceededEventDetails>,
    #[serde(rename = "executionTimedOutEventDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_timed_out_event_details: Option<ExecutionTimedOutEventDetails>,
    /// <p>The id of the event. Events are numbered sequentially, starting at one.</p>
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "lambdaFunctionFailedEventDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_failed_event_details: Option<LambdaFunctionFailedEventDetails>,
    #[serde(rename = "lambdaFunctionScheduleFailedEventDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_schedule_failed_event_details:
        Option<LambdaFunctionScheduleFailedEventDetails>,
    #[serde(rename = "lambdaFunctionScheduledEventDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_scheduled_event_details: Option<LambdaFunctionScheduledEventDetails>,
    /// <p>Contains details about a lambda function that failed to start during an execution.</p>
    #[serde(rename = "lambdaFunctionStartFailedEventDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_start_failed_event_details: Option<LambdaFunctionStartFailedEventDetails>,
    /// <p>Contains details about a lambda function that terminated successfully during an execution.</p>
    #[serde(rename = "lambdaFunctionSucceededEventDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_succeeded_event_details: Option<LambdaFunctionSucceededEventDetails>,
    #[serde(rename = "lambdaFunctionTimedOutEventDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_timed_out_event_details: Option<LambdaFunctionTimedOutEventDetails>,
    /// <p>Contains details about an iteration of a Map state that was aborted.</p>
    #[serde(rename = "mapIterationAbortedEventDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_iteration_aborted_event_details: Option<MapIterationEventDetails>,
    /// <p>Contains details about an iteration of a Map state that failed.</p>
    #[serde(rename = "mapIterationFailedEventDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_iteration_failed_event_details: Option<MapIterationEventDetails>,
    /// <p>Contains details about an iteration of a Map state that was started.</p>
    #[serde(rename = "mapIterationStartedEventDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_iteration_started_event_details: Option<MapIterationEventDetails>,
    /// <p>Contains details about an iteration of a Map state that succeeded.</p>
    #[serde(rename = "mapIterationSucceededEventDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_iteration_succeeded_event_details: Option<MapIterationEventDetails>,
    /// <p>Contains details about Map state that was started.</p>
    #[serde(rename = "mapStateStartedEventDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_state_started_event_details: Option<MapStateStartedEventDetails>,
    /// <p>The id of the previous event.</p>
    #[serde(rename = "previousEventId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_event_id: Option<i64>,
    #[serde(rename = "stateEnteredEventDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_entered_event_details: Option<StateEnteredEventDetails>,
    #[serde(rename = "stateExitedEventDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_exited_event_details: Option<StateExitedEventDetails>,
    /// <p>Contains details about the failure of a task.</p>
    #[serde(rename = "taskFailedEventDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_failed_event_details: Option<TaskFailedEventDetails>,
    /// <p>Contains details about a task that was scheduled.</p>
    #[serde(rename = "taskScheduledEventDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_scheduled_event_details: Option<TaskScheduledEventDetails>,
    /// <p>Contains details about a task that failed to start.</p>
    #[serde(rename = "taskStartFailedEventDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_start_failed_event_details: Option<TaskStartFailedEventDetails>,
    /// <p>Contains details about a task that was started.</p>
    #[serde(rename = "taskStartedEventDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_started_event_details: Option<TaskStartedEventDetails>,
    /// <p>Contains details about a task that where the submit failed.</p>
    #[serde(rename = "taskSubmitFailedEventDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_submit_failed_event_details: Option<TaskSubmitFailedEventDetails>,
    /// <p>Contains details about a submitted task.</p>
    #[serde(rename = "taskSubmittedEventDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_submitted_event_details: Option<TaskSubmittedEventDetails>,
    /// <p>Contains details about a task that succeeded.</p>
    #[serde(rename = "taskSucceededEventDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_succeeded_event_details: Option<TaskSucceededEventDetails>,
    /// <p>Contains details about a task that timed out.</p>
    #[serde(rename = "taskTimedOutEventDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_timed_out_event_details: Option<TaskTimedOutEventDetails>,
    /// <p>The date and time the event occurred.</p>
    #[serde(rename = "timestamp")]
    pub timestamp: f64,
    /// <p>The type of the event.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p>Contains details about a lambda function that failed during an execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LambdaFunctionFailedEventDetails {
    /// <p>A more detailed explanation of the cause of the failure.</p>
    #[serde(rename = "cause")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    /// <p>The error code of the failure.</p>
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// <p>Contains details about a failed lambda function schedule event that occurred during an execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LambdaFunctionScheduleFailedEventDetails {
    /// <p>A more detailed explanation of the cause of the failure.</p>
    #[serde(rename = "cause")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    /// <p>The error code of the failure.</p>
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// <p>Contains details about a lambda function scheduled during an execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LambdaFunctionScheduledEventDetails {
    /// <p>The JSON data input to the lambda function.</p>
    #[serde(rename = "input")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the scheduled lambda function.</p>
    #[serde(rename = "resource")]
    pub resource: String,
    /// <p>The maximum allowed duration of the lambda function.</p>
    #[serde(rename = "timeoutInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_seconds: Option<i64>,
}

/// <p>Contains details about a lambda function that failed to start during an execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LambdaFunctionStartFailedEventDetails {
    /// <p>A more detailed explanation of the cause of the failure.</p>
    #[serde(rename = "cause")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    /// <p>The error code of the failure.</p>
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// <p>Contains details about a lambda function that successfully terminated during an execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LambdaFunctionSucceededEventDetails {
    /// <p>The JSON data output by the lambda function.</p>
    #[serde(rename = "output")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
}

/// <p>Contains details about a lambda function timeout that occurred during an execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LambdaFunctionTimedOutEventDetails {
    /// <p>A more detailed explanation of the cause of the timeout.</p>
    #[serde(rename = "cause")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    /// <p>The error code of the failure.</p>
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListActivitiesInput {
    /// <p>The maximum number of results that are returned per call. You can use <code>nextToken</code> to obtain further pages of results. The default is 100 and the maximum allowed page size is 1000. A value of 0 uses the default.</p> <p>This is only an upper limit. The actual number of results returned per call might be fewer than the specified maximum.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If <code>nextToken</code> is returned, there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return an <i>HTTP 400 InvalidToken</i> error.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListActivitiesOutput {
    /// <p>The list of activities.</p>
    #[serde(rename = "activities")]
    pub activities: Vec<ActivityListItem>,
    /// <p>If <code>nextToken</code> is returned, there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return an <i>HTTP 400 InvalidToken</i> error.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListExecutionsInput {
    /// <p>The maximum number of results that are returned per call. You can use <code>nextToken</code> to obtain further pages of results. The default is 100 and the maximum allowed page size is 1000. A value of 0 uses the default.</p> <p>This is only an upper limit. The actual number of results returned per call might be fewer than the specified maximum.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If <code>nextToken</code> is returned, there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return an <i>HTTP 400 InvalidToken</i> error.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the state machine whose executions is listed.</p>
    #[serde(rename = "stateMachineArn")]
    pub state_machine_arn: String,
    /// <p>If specified, only list the executions whose current execution status matches the given filter.</p>
    #[serde(rename = "statusFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_filter: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListExecutionsOutput {
    /// <p>The list of matching executions.</p>
    #[serde(rename = "executions")]
    pub executions: Vec<ExecutionListItem>,
    /// <p>If <code>nextToken</code> is returned, there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return an <i>HTTP 400 InvalidToken</i> error.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListStateMachinesInput {
    /// <p>The maximum number of results that are returned per call. You can use <code>nextToken</code> to obtain further pages of results. The default is 100 and the maximum allowed page size is 1000. A value of 0 uses the default.</p> <p>This is only an upper limit. The actual number of results returned per call might be fewer than the specified maximum.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If <code>nextToken</code> is returned, there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return an <i>HTTP 400 InvalidToken</i> error.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListStateMachinesOutput {
    /// <p>If <code>nextToken</code> is returned, there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return an <i>HTTP 400 InvalidToken</i> error.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "stateMachines")]
    pub state_machines: Vec<StateMachineListItem>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceInput {
    /// <p>The Amazon Resource Name (ARN) for the Step Functions state machine or activity.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceOutput {
    /// <p>An array of tags associated with the resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogDestination {
    /// <p>An object describing a CloudWatch log group. For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-loggroup.html">AWS::Logs::LogGroup</a> in the AWS CloudFormation User Guide.</p>
    #[serde(rename = "cloudWatchLogsLogGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_log_group: Option<CloudWatchLogsLogGroup>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoggingConfiguration {
    /// <p>An object that describes where your execution history events will be logged. Limited to size 1. Required, if your log level is not set to <code>OFF</code>.</p>
    #[serde(rename = "destinations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<LogDestination>>,
    /// <p>Determines whether execution history data is included in your log. When set to <code>FALSE</code>, data is excluded.</p>
    #[serde(rename = "includeExecutionData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_execution_data: Option<bool>,
    /// <p>Defines which category of execution history events are logged.</p>
    #[serde(rename = "level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
}

/// <p>Contains details about an iteration of a Map state.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MapIterationEventDetails {
    /// <p>The index of the array belonging to the Map state iteration.</p>
    #[serde(rename = "index")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
    /// <p>The name of the iteration’s parent Map state.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Details about a Map state that was started.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MapStateStartedEventDetails {
    /// <p>The size of the array for Map state iterations.</p>
    #[serde(rename = "length")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SendTaskFailureInput {
    /// <p>A more detailed explanation of the cause of the failure.</p>
    #[serde(rename = "cause")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    /// <p>The error code of the failure.</p>
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// <p>The token that represents this task. Task tokens are generated by Step Functions when tasks are assigned to a worker, or in the <a href="https://docs.aws.amazon.com/step-functions/latest/dg/input-output-contextobject.html">context object</a> when a workflow enters a task state. See <a>GetActivityTaskOutput$taskToken</a>.</p>
    #[serde(rename = "taskToken")]
    pub task_token: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SendTaskFailureOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SendTaskHeartbeatInput {
    /// <p>The token that represents this task. Task tokens are generated by Step Functions when tasks are assigned to a worker, or in the <a href="https://docs.aws.amazon.com/step-functions/latest/dg/input-output-contextobject.html">context object</a> when a workflow enters a task state. See <a>GetActivityTaskOutput$taskToken</a>.</p>
    #[serde(rename = "taskToken")]
    pub task_token: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SendTaskHeartbeatOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SendTaskSuccessInput {
    /// <p>The JSON output of the task.</p>
    #[serde(rename = "output")]
    pub output: String,
    /// <p>The token that represents this task. Task tokens are generated by Step Functions when tasks are assigned to a worker, or in the <a href="https://docs.aws.amazon.com/step-functions/latest/dg/input-output-contextobject.html">context object</a> when a workflow enters a task state. See <a>GetActivityTaskOutput$taskToken</a>.</p>
    #[serde(rename = "taskToken")]
    pub task_token: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SendTaskSuccessOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartExecutionInput {
    /// <p><p>The string that contains the JSON input data for the execution, for example:</p> <p> <code>&quot;input&quot;: &quot;{&quot;first_name&quot; : &quot;test&quot;}&quot;</code> </p> <note> <p>If you don&#39;t include any JSON input data, you still must include the two braces, for example: <code>&quot;input&quot;: &quot;{}&quot;</code> </p> </note></p>
    #[serde(rename = "input")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    /// <p><p>The name of the execution. This name must be unique for your AWS account, region, and state machine for 90 days. For more information, see <a href="https://docs.aws.amazon.com/step-functions/latest/dg/limits.html#service-limits-state-machine-executions"> Limits Related to State Machine Executions</a> in the <i>AWS Step Functions Developer Guide</i>.</p> <p>A name must <i>not</i> contain:</p> <ul> <li> <p>white space</p> </li> <li> <p>brackets <code>&lt; &gt; { } [ ]</code> </p> </li> <li> <p>wildcard characters <code>? *</code> </p> </li> <li> <p>special characters <code>&quot; # % \ ^ | ~ ` $ &amp; , ; : /</code> </p> </li> <li> <p>control characters (<code>U+0000-001F</code>, <code>U+007F-009F</code>)</p> </li> </ul></p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the state machine to execute.</p>
    #[serde(rename = "stateMachineArn")]
    pub state_machine_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartExecutionOutput {
    /// <p>The Amazon Resource Name (ARN) that identifies the execution.</p>
    #[serde(rename = "executionArn")]
    pub execution_arn: String,
    /// <p>The date the execution is started.</p>
    #[serde(rename = "startDate")]
    pub start_date: f64,
}

/// <p>Contains details about a state entered during an execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StateEnteredEventDetails {
    /// <p>The string that contains the JSON input data for the state.</p>
    #[serde(rename = "input")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    /// <p>The name of the state.</p>
    #[serde(rename = "name")]
    pub name: String,
}

/// <p>Contains details about an exit from a state during an execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StateExitedEventDetails {
    /// <p><p>The name of the state.</p> <p>A name must <i>not</i> contain:</p> <ul> <li> <p>white space</p> </li> <li> <p>brackets <code>&lt; &gt; { } [ ]</code> </p> </li> <li> <p>wildcard characters <code>? *</code> </p> </li> <li> <p>special characters <code>&quot; # % \ ^ | ~ ` $ &amp; , ; : /</code> </p> </li> <li> <p>control characters (<code>U+0000-001F</code>, <code>U+007F-009F</code>)</p> </li> </ul></p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The JSON output data of the state.</p>
    #[serde(rename = "output")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
}

/// <p>Contains details about the state machine.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StateMachineListItem {
    /// <p>The date the state machine is created.</p>
    #[serde(rename = "creationDate")]
    pub creation_date: f64,
    /// <p><p>The name of the state machine.</p> <p>A name must <i>not</i> contain:</p> <ul> <li> <p>white space</p> </li> <li> <p>brackets <code>&lt; &gt; { } [ ]</code> </p> </li> <li> <p>wildcard characters <code>? *</code> </p> </li> <li> <p>special characters <code>&quot; # % \ ^ | ~ ` $ &amp; , ; : /</code> </p> </li> <li> <p>control characters (<code>U+0000-001F</code>, <code>U+007F-009F</code>)</p> </li> </ul></p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The Amazon Resource Name (ARN) that identifies the state machine.</p>
    #[serde(rename = "stateMachineArn")]
    pub state_machine_arn: String,
    /// <p><p/></p>
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopExecutionInput {
    /// <p>A more detailed explanation of the cause of the failure.</p>
    #[serde(rename = "cause")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    /// <p>The error code of the failure.</p>
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the execution to stop.</p>
    #[serde(rename = "executionArn")]
    pub execution_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopExecutionOutput {
    /// <p>The date the execution is stopped.</p>
    #[serde(rename = "stopDate")]
    pub stop_date: f64,
}

/// <p>Tags are key-value pairs that can be associated with Step Functions state machines and activities.</p> <p>An array of key-value pairs. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>, and <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_iam-tags.html">Controlling Access Using IAM Tags</a>.</p> <p>Tags may only contain Unicode letters, digits, white space, or these symbols: <code>_ . : / = + - @</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>The key of a tag.</p>
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The value of a tag.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceInput {
    /// <p>The Amazon Resource Name (ARN) for the Step Functions state machine or activity.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The list of tags to add to a resource.</p> <p>Tags may only contain Unicode letters, digits, white space, or these symbols: <code>_ . : / = + - @</code>.</p>
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceOutput {}

/// <p>Contains details about a task failure event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TaskFailedEventDetails {
    /// <p>A more detailed explanation of the cause of the failure.</p>
    #[serde(rename = "cause")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    /// <p>The error code of the failure.</p>
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// <p>The service name of the resource in a task state.</p>
    #[serde(rename = "resource")]
    pub resource: String,
    /// <p>The action of the resource called by a task state.</p>
    #[serde(rename = "resourceType")]
    pub resource_type: String,
}

/// <p>Contains details about a task scheduled during an execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TaskScheduledEventDetails {
    /// <p>The JSON data passed to the resource referenced in a task state.</p>
    #[serde(rename = "parameters")]
    pub parameters: String,
    /// <p>The region of the scheduled task</p>
    #[serde(rename = "region")]
    pub region: String,
    /// <p>The service name of the resource in a task state.</p>
    #[serde(rename = "resource")]
    pub resource: String,
    /// <p>The action of the resource called by a task state.</p>
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    /// <p>The maximum allowed duration of the task.</p>
    #[serde(rename = "timeoutInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_seconds: Option<i64>,
}

/// <p>Contains details about a task that failed to start during an execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TaskStartFailedEventDetails {
    /// <p>A more detailed explanation of the cause of the failure.</p>
    #[serde(rename = "cause")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    /// <p>The error code of the failure.</p>
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// <p>The service name of the resource in a task state.</p>
    #[serde(rename = "resource")]
    pub resource: String,
    /// <p>The action of the resource called by a task state.</p>
    #[serde(rename = "resourceType")]
    pub resource_type: String,
}

/// <p>Contains details about the start of a task during an execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TaskStartedEventDetails {
    /// <p>The service name of the resource in a task state.</p>
    #[serde(rename = "resource")]
    pub resource: String,
    /// <p>The action of the resource called by a task state.</p>
    #[serde(rename = "resourceType")]
    pub resource_type: String,
}

/// <p>Contains details about a task that failed to submit during an execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TaskSubmitFailedEventDetails {
    /// <p>A more detailed explanation of the cause of the failure.</p>
    #[serde(rename = "cause")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    /// <p>The error code of the failure.</p>
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// <p>The service name of the resource in a task state.</p>
    #[serde(rename = "resource")]
    pub resource: String,
    /// <p>The action of the resource called by a task state.</p>
    #[serde(rename = "resourceType")]
    pub resource_type: String,
}

/// <p>Contains details about a task submitted to a resource .</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TaskSubmittedEventDetails {
    /// <p>The response from a resource when a task has started.</p>
    #[serde(rename = "output")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    /// <p>The service name of the resource in a task state.</p>
    #[serde(rename = "resource")]
    pub resource: String,
    /// <p>The action of the resource called by a task state.</p>
    #[serde(rename = "resourceType")]
    pub resource_type: String,
}

/// <p>Contains details about the successful completion of a task state.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TaskSucceededEventDetails {
    /// <p>The full JSON response from a resource when a task has succeeded. This response becomes the output of the related task.</p>
    #[serde(rename = "output")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    /// <p>The service name of the resource in a task state.</p>
    #[serde(rename = "resource")]
    pub resource: String,
    /// <p>The action of the resource called by a task state.</p>
    #[serde(rename = "resourceType")]
    pub resource_type: String,
}

/// <p>Contains details about a resource timeout that occurred during an execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TaskTimedOutEventDetails {
    /// <p>A more detailed explanation of the cause of the failure.</p>
    #[serde(rename = "cause")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    /// <p>The error code of the failure.</p>
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// <p>The service name of the resource in a task state.</p>
    #[serde(rename = "resource")]
    pub resource: String,
    /// <p>The action of the resource called by a task state.</p>
    #[serde(rename = "resourceType")]
    pub resource_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceInput {
    /// <p>The Amazon Resource Name (ARN) for the Step Functions state machine or activity.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The list of tags to remove from the resource.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateStateMachineInput {
    /// <p>The Amazon States Language definition of the state machine. See <a href="https://docs.aws.amazon.com/step-functions/latest/dg/concepts-amazon-states-language.html">Amazon States Language</a>.</p>
    #[serde(rename = "definition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
    /// <p><p/></p>
    #[serde(rename = "loggingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_configuration: Option<LoggingConfiguration>,
    /// <p>The Amazon Resource Name (ARN) of the IAM role of the state machine.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the state machine.</p>
    #[serde(rename = "stateMachineArn")]
    pub state_machine_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateStateMachineOutput {
    /// <p>The date and time the state machine was updated.</p>
    #[serde(rename = "updateDate")]
    pub update_date: f64,
}

/// Errors returned by CreateActivity
#[derive(Debug, PartialEq)]
pub enum CreateActivityError {
    /// <p>The maximum number of activities has been reached. Existing activities must be deleted before a new activity can be created.</p>
    ActivityLimitExceeded(String),
    /// <p>The provided name is invalid.</p>
    InvalidName(String),
    /// <p>You've exceeded the number of tags allowed for a resource. See the <a href="https://docs.aws.amazon.com/step-functions/latest/dg/limits.html"> Limits Topic</a> in the AWS Step Functions Developer Guide.</p>
    TooManyTags(String),
}

impl CreateActivityError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateActivityError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ActivityLimitExceeded" => {
                    return RusotoError::Service(CreateActivityError::ActivityLimitExceeded(
                        err.msg,
                    ))
                }
                "InvalidName" => {
                    return RusotoError::Service(CreateActivityError::InvalidName(err.msg))
                }
                "TooManyTags" => {
                    return RusotoError::Service(CreateActivityError::TooManyTags(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateActivityError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateActivityError::ActivityLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateActivityError::InvalidName(ref cause) => write!(f, "{}", cause),
            CreateActivityError::TooManyTags(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateActivityError {}
/// Errors returned by CreateStateMachine
#[derive(Debug, PartialEq)]
pub enum CreateStateMachineError {
    /// <p>The provided Amazon Resource Name (ARN) is invalid.</p>
    InvalidArn(String),
    /// <p>The provided Amazon States Language definition is invalid.</p>
    InvalidDefinition(String),
    /// <p><p/></p>
    InvalidLoggingConfiguration(String),
    /// <p>The provided name is invalid.</p>
    InvalidName(String),
    /// <p>A state machine with the same name but a different definition or role ARN already exists.</p>
    StateMachineAlreadyExists(String),
    /// <p>The specified state machine is being deleted.</p>
    StateMachineDeleting(String),
    /// <p>The maximum number of state machines has been reached. Existing state machines must be deleted before a new state machine can be created.</p>
    StateMachineLimitExceeded(String),
    /// <p><p/></p>
    StateMachineTypeNotSupported(String),
    /// <p>You've exceeded the number of tags allowed for a resource. See the <a href="https://docs.aws.amazon.com/step-functions/latest/dg/limits.html"> Limits Topic</a> in the AWS Step Functions Developer Guide.</p>
    TooManyTags(String),
}

impl CreateStateMachineError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateStateMachineError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArn" => {
                    return RusotoError::Service(CreateStateMachineError::InvalidArn(err.msg))
                }
                "InvalidDefinition" => {
                    return RusotoError::Service(CreateStateMachineError::InvalidDefinition(
                        err.msg,
                    ))
                }
                "InvalidLoggingConfiguration" => {
                    return RusotoError::Service(
                        CreateStateMachineError::InvalidLoggingConfiguration(err.msg),
                    )
                }
                "InvalidName" => {
                    return RusotoError::Service(CreateStateMachineError::InvalidName(err.msg))
                }
                "StateMachineAlreadyExists" => {
                    return RusotoError::Service(
                        CreateStateMachineError::StateMachineAlreadyExists(err.msg),
                    )
                }
                "StateMachineDeleting" => {
                    return RusotoError::Service(CreateStateMachineError::StateMachineDeleting(
                        err.msg,
                    ))
                }
                "StateMachineLimitExceeded" => {
                    return RusotoError::Service(
                        CreateStateMachineError::StateMachineLimitExceeded(err.msg),
                    )
                }
                "StateMachineTypeNotSupported" => {
                    return RusotoError::Service(
                        CreateStateMachineError::StateMachineTypeNotSupported(err.msg),
                    )
                }
                "TooManyTags" => {
                    return RusotoError::Service(CreateStateMachineError::TooManyTags(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateStateMachineError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateStateMachineError::InvalidArn(ref cause) => write!(f, "{}", cause),
            CreateStateMachineError::InvalidDefinition(ref cause) => write!(f, "{}", cause),
            CreateStateMachineError::InvalidLoggingConfiguration(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateStateMachineError::InvalidName(ref cause) => write!(f, "{}", cause),
            CreateStateMachineError::StateMachineAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateStateMachineError::StateMachineDeleting(ref cause) => write!(f, "{}", cause),
            CreateStateMachineError::StateMachineLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateStateMachineError::StateMachineTypeNotSupported(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateStateMachineError::TooManyTags(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateStateMachineError {}
/// Errors returned by DeleteActivity
#[derive(Debug, PartialEq)]
pub enum DeleteActivityError {
    /// <p>The provided Amazon Resource Name (ARN) is invalid.</p>
    InvalidArn(String),
}

impl DeleteActivityError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteActivityError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArn" => {
                    return RusotoError::Service(DeleteActivityError::InvalidArn(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteActivityError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteActivityError::InvalidArn(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteActivityError {}
/// Errors returned by DeleteStateMachine
#[derive(Debug, PartialEq)]
pub enum DeleteStateMachineError {
    /// <p>The provided Amazon Resource Name (ARN) is invalid.</p>
    InvalidArn(String),
}

impl DeleteStateMachineError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteStateMachineError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArn" => {
                    return RusotoError::Service(DeleteStateMachineError::InvalidArn(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteStateMachineError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteStateMachineError::InvalidArn(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteStateMachineError {}
/// Errors returned by DescribeActivity
#[derive(Debug, PartialEq)]
pub enum DescribeActivityError {
    /// <p>The specified activity does not exist.</p>
    ActivityDoesNotExist(String),
    /// <p>The provided Amazon Resource Name (ARN) is invalid.</p>
    InvalidArn(String),
}

impl DescribeActivityError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeActivityError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ActivityDoesNotExist" => {
                    return RusotoError::Service(DescribeActivityError::ActivityDoesNotExist(
                        err.msg,
                    ))
                }
                "InvalidArn" => {
                    return RusotoError::Service(DescribeActivityError::InvalidArn(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeActivityError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeActivityError::ActivityDoesNotExist(ref cause) => write!(f, "{}", cause),
            DescribeActivityError::InvalidArn(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeActivityError {}
/// Errors returned by DescribeExecution
#[derive(Debug, PartialEq)]
pub enum DescribeExecutionError {
    /// <p>The specified execution does not exist.</p>
    ExecutionDoesNotExist(String),
    /// <p>The provided Amazon Resource Name (ARN) is invalid.</p>
    InvalidArn(String),
}

impl DescribeExecutionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeExecutionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ExecutionDoesNotExist" => {
                    return RusotoError::Service(DescribeExecutionError::ExecutionDoesNotExist(
                        err.msg,
                    ))
                }
                "InvalidArn" => {
                    return RusotoError::Service(DescribeExecutionError::InvalidArn(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeExecutionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeExecutionError::ExecutionDoesNotExist(ref cause) => write!(f, "{}", cause),
            DescribeExecutionError::InvalidArn(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeExecutionError {}
/// Errors returned by DescribeStateMachine
#[derive(Debug, PartialEq)]
pub enum DescribeStateMachineError {
    /// <p>The provided Amazon Resource Name (ARN) is invalid.</p>
    InvalidArn(String),
    /// <p>The specified state machine does not exist.</p>
    StateMachineDoesNotExist(String),
}

impl DescribeStateMachineError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeStateMachineError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArn" => {
                    return RusotoError::Service(DescribeStateMachineError::InvalidArn(err.msg))
                }
                "StateMachineDoesNotExist" => {
                    return RusotoError::Service(
                        DescribeStateMachineError::StateMachineDoesNotExist(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeStateMachineError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeStateMachineError::InvalidArn(ref cause) => write!(f, "{}", cause),
            DescribeStateMachineError::StateMachineDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeStateMachineError {}
/// Errors returned by DescribeStateMachineForExecution
#[derive(Debug, PartialEq)]
pub enum DescribeStateMachineForExecutionError {
    /// <p>The specified execution does not exist.</p>
    ExecutionDoesNotExist(String),
    /// <p>The provided Amazon Resource Name (ARN) is invalid.</p>
    InvalidArn(String),
}

impl DescribeStateMachineForExecutionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeStateMachineForExecutionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ExecutionDoesNotExist" => {
                    return RusotoError::Service(
                        DescribeStateMachineForExecutionError::ExecutionDoesNotExist(err.msg),
                    )
                }
                "InvalidArn" => {
                    return RusotoError::Service(DescribeStateMachineForExecutionError::InvalidArn(
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
impl fmt::Display for DescribeStateMachineForExecutionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeStateMachineForExecutionError::ExecutionDoesNotExist(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeStateMachineForExecutionError::InvalidArn(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeStateMachineForExecutionError {}
/// Errors returned by GetActivityTask
#[derive(Debug, PartialEq)]
pub enum GetActivityTaskError {
    /// <p>The specified activity does not exist.</p>
    ActivityDoesNotExist(String),
    /// <p>The maximum number of workers concurrently polling for activity tasks has been reached.</p>
    ActivityWorkerLimitExceeded(String),
    /// <p>The provided Amazon Resource Name (ARN) is invalid.</p>
    InvalidArn(String),
}

impl GetActivityTaskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetActivityTaskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ActivityDoesNotExist" => {
                    return RusotoError::Service(GetActivityTaskError::ActivityDoesNotExist(
                        err.msg,
                    ))
                }
                "ActivityWorkerLimitExceeded" => {
                    return RusotoError::Service(GetActivityTaskError::ActivityWorkerLimitExceeded(
                        err.msg,
                    ))
                }
                "InvalidArn" => {
                    return RusotoError::Service(GetActivityTaskError::InvalidArn(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetActivityTaskError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetActivityTaskError::ActivityDoesNotExist(ref cause) => write!(f, "{}", cause),
            GetActivityTaskError::ActivityWorkerLimitExceeded(ref cause) => write!(f, "{}", cause),
            GetActivityTaskError::InvalidArn(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetActivityTaskError {}
/// Errors returned by GetExecutionHistory
#[derive(Debug, PartialEq)]
pub enum GetExecutionHistoryError {
    /// <p>The specified execution does not exist.</p>
    ExecutionDoesNotExist(String),
    /// <p>The provided Amazon Resource Name (ARN) is invalid.</p>
    InvalidArn(String),
    /// <p>The provided token is invalid.</p>
    InvalidToken(String),
}

impl GetExecutionHistoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetExecutionHistoryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ExecutionDoesNotExist" => {
                    return RusotoError::Service(GetExecutionHistoryError::ExecutionDoesNotExist(
                        err.msg,
                    ))
                }
                "InvalidArn" => {
                    return RusotoError::Service(GetExecutionHistoryError::InvalidArn(err.msg))
                }
                "InvalidToken" => {
                    return RusotoError::Service(GetExecutionHistoryError::InvalidToken(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetExecutionHistoryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetExecutionHistoryError::ExecutionDoesNotExist(ref cause) => write!(f, "{}", cause),
            GetExecutionHistoryError::InvalidArn(ref cause) => write!(f, "{}", cause),
            GetExecutionHistoryError::InvalidToken(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetExecutionHistoryError {}
/// Errors returned by ListActivities
#[derive(Debug, PartialEq)]
pub enum ListActivitiesError {
    /// <p>The provided token is invalid.</p>
    InvalidToken(String),
}

impl ListActivitiesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListActivitiesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidToken" => {
                    return RusotoError::Service(ListActivitiesError::InvalidToken(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListActivitiesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListActivitiesError::InvalidToken(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListActivitiesError {}
/// Errors returned by ListExecutions
#[derive(Debug, PartialEq)]
pub enum ListExecutionsError {
    /// <p>The provided Amazon Resource Name (ARN) is invalid.</p>
    InvalidArn(String),
    /// <p>The provided token is invalid.</p>
    InvalidToken(String),
    /// <p>The specified state machine does not exist.</p>
    StateMachineDoesNotExist(String),
    /// <p><p/></p>
    StateMachineTypeNotSupported(String),
}

impl ListExecutionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListExecutionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArn" => {
                    return RusotoError::Service(ListExecutionsError::InvalidArn(err.msg))
                }
                "InvalidToken" => {
                    return RusotoError::Service(ListExecutionsError::InvalidToken(err.msg))
                }
                "StateMachineDoesNotExist" => {
                    return RusotoError::Service(ListExecutionsError::StateMachineDoesNotExist(
                        err.msg,
                    ))
                }
                "StateMachineTypeNotSupported" => {
                    return RusotoError::Service(ListExecutionsError::StateMachineTypeNotSupported(
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
impl fmt::Display for ListExecutionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListExecutionsError::InvalidArn(ref cause) => write!(f, "{}", cause),
            ListExecutionsError::InvalidToken(ref cause) => write!(f, "{}", cause),
            ListExecutionsError::StateMachineDoesNotExist(ref cause) => write!(f, "{}", cause),
            ListExecutionsError::StateMachineTypeNotSupported(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListExecutionsError {}
/// Errors returned by ListStateMachines
#[derive(Debug, PartialEq)]
pub enum ListStateMachinesError {
    /// <p>The provided token is invalid.</p>
    InvalidToken(String),
}

impl ListStateMachinesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListStateMachinesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidToken" => {
                    return RusotoError::Service(ListStateMachinesError::InvalidToken(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListStateMachinesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListStateMachinesError::InvalidToken(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListStateMachinesError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>The provided Amazon Resource Name (ARN) is invalid.</p>
    InvalidArn(String),
    /// <p>Could not find the referenced resource. Only state machine and activity ARNs are supported.</p>
    ResourceNotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArn" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidArn(err.msg))
                }
                "ResourceNotFound" => {
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
            ListTagsForResourceError::InvalidArn(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by SendTaskFailure
#[derive(Debug, PartialEq)]
pub enum SendTaskFailureError {
    /// <p>The provided token is invalid.</p>
    InvalidToken(String),

    TaskDoesNotExist(String),

    TaskTimedOut(String),
}

impl SendTaskFailureError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SendTaskFailureError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidToken" => {
                    return RusotoError::Service(SendTaskFailureError::InvalidToken(err.msg))
                }
                "TaskDoesNotExist" => {
                    return RusotoError::Service(SendTaskFailureError::TaskDoesNotExist(err.msg))
                }
                "TaskTimedOut" => {
                    return RusotoError::Service(SendTaskFailureError::TaskTimedOut(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SendTaskFailureError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SendTaskFailureError::InvalidToken(ref cause) => write!(f, "{}", cause),
            SendTaskFailureError::TaskDoesNotExist(ref cause) => write!(f, "{}", cause),
            SendTaskFailureError::TaskTimedOut(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SendTaskFailureError {}
/// Errors returned by SendTaskHeartbeat
#[derive(Debug, PartialEq)]
pub enum SendTaskHeartbeatError {
    /// <p>The provided token is invalid.</p>
    InvalidToken(String),

    TaskDoesNotExist(String),

    TaskTimedOut(String),
}

impl SendTaskHeartbeatError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SendTaskHeartbeatError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidToken" => {
                    return RusotoError::Service(SendTaskHeartbeatError::InvalidToken(err.msg))
                }
                "TaskDoesNotExist" => {
                    return RusotoError::Service(SendTaskHeartbeatError::TaskDoesNotExist(err.msg))
                }
                "TaskTimedOut" => {
                    return RusotoError::Service(SendTaskHeartbeatError::TaskTimedOut(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SendTaskHeartbeatError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SendTaskHeartbeatError::InvalidToken(ref cause) => write!(f, "{}", cause),
            SendTaskHeartbeatError::TaskDoesNotExist(ref cause) => write!(f, "{}", cause),
            SendTaskHeartbeatError::TaskTimedOut(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SendTaskHeartbeatError {}
/// Errors returned by SendTaskSuccess
#[derive(Debug, PartialEq)]
pub enum SendTaskSuccessError {
    /// <p>The provided JSON output data is invalid.</p>
    InvalidOutput(String),
    /// <p>The provided token is invalid.</p>
    InvalidToken(String),

    TaskDoesNotExist(String),

    TaskTimedOut(String),
}

impl SendTaskSuccessError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SendTaskSuccessError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidOutput" => {
                    return RusotoError::Service(SendTaskSuccessError::InvalidOutput(err.msg))
                }
                "InvalidToken" => {
                    return RusotoError::Service(SendTaskSuccessError::InvalidToken(err.msg))
                }
                "TaskDoesNotExist" => {
                    return RusotoError::Service(SendTaskSuccessError::TaskDoesNotExist(err.msg))
                }
                "TaskTimedOut" => {
                    return RusotoError::Service(SendTaskSuccessError::TaskTimedOut(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SendTaskSuccessError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SendTaskSuccessError::InvalidOutput(ref cause) => write!(f, "{}", cause),
            SendTaskSuccessError::InvalidToken(ref cause) => write!(f, "{}", cause),
            SendTaskSuccessError::TaskDoesNotExist(ref cause) => write!(f, "{}", cause),
            SendTaskSuccessError::TaskTimedOut(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SendTaskSuccessError {}
/// Errors returned by StartExecution
#[derive(Debug, PartialEq)]
pub enum StartExecutionError {
    /// <p><p>The execution has the same <code>name</code> as another execution (but a different <code>input</code>).</p> <note> <p>Executions with the same <code>name</code> and <code>input</code> are considered idempotent.</p> </note></p>
    ExecutionAlreadyExists(String),
    /// <p>The maximum number of running executions has been reached. Running executions must end or be stopped before a new execution can be started.</p>
    ExecutionLimitExceeded(String),
    /// <p>The provided Amazon Resource Name (ARN) is invalid.</p>
    InvalidArn(String),
    /// <p>The provided JSON input data is invalid.</p>
    InvalidExecutionInput(String),
    /// <p>The provided name is invalid.</p>
    InvalidName(String),
    /// <p>The specified state machine is being deleted.</p>
    StateMachineDeleting(String),
    /// <p>The specified state machine does not exist.</p>
    StateMachineDoesNotExist(String),
}

impl StartExecutionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartExecutionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ExecutionAlreadyExists" => {
                    return RusotoError::Service(StartExecutionError::ExecutionAlreadyExists(
                        err.msg,
                    ))
                }
                "ExecutionLimitExceeded" => {
                    return RusotoError::Service(StartExecutionError::ExecutionLimitExceeded(
                        err.msg,
                    ))
                }
                "InvalidArn" => {
                    return RusotoError::Service(StartExecutionError::InvalidArn(err.msg))
                }
                "InvalidExecutionInput" => {
                    return RusotoError::Service(StartExecutionError::InvalidExecutionInput(
                        err.msg,
                    ))
                }
                "InvalidName" => {
                    return RusotoError::Service(StartExecutionError::InvalidName(err.msg))
                }
                "StateMachineDeleting" => {
                    return RusotoError::Service(StartExecutionError::StateMachineDeleting(err.msg))
                }
                "StateMachineDoesNotExist" => {
                    return RusotoError::Service(StartExecutionError::StateMachineDoesNotExist(
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
impl fmt::Display for StartExecutionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartExecutionError::ExecutionAlreadyExists(ref cause) => write!(f, "{}", cause),
            StartExecutionError::ExecutionLimitExceeded(ref cause) => write!(f, "{}", cause),
            StartExecutionError::InvalidArn(ref cause) => write!(f, "{}", cause),
            StartExecutionError::InvalidExecutionInput(ref cause) => write!(f, "{}", cause),
            StartExecutionError::InvalidName(ref cause) => write!(f, "{}", cause),
            StartExecutionError::StateMachineDeleting(ref cause) => write!(f, "{}", cause),
            StartExecutionError::StateMachineDoesNotExist(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartExecutionError {}
/// Errors returned by StopExecution
#[derive(Debug, PartialEq)]
pub enum StopExecutionError {
    /// <p>The specified execution does not exist.</p>
    ExecutionDoesNotExist(String),
    /// <p>The provided Amazon Resource Name (ARN) is invalid.</p>
    InvalidArn(String),
}

impl StopExecutionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopExecutionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ExecutionDoesNotExist" => {
                    return RusotoError::Service(StopExecutionError::ExecutionDoesNotExist(err.msg))
                }
                "InvalidArn" => {
                    return RusotoError::Service(StopExecutionError::InvalidArn(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StopExecutionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopExecutionError::ExecutionDoesNotExist(ref cause) => write!(f, "{}", cause),
            StopExecutionError::InvalidArn(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopExecutionError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>The provided Amazon Resource Name (ARN) is invalid.</p>
    InvalidArn(String),
    /// <p>Could not find the referenced resource. Only state machine and activity ARNs are supported.</p>
    ResourceNotFound(String),
    /// <p>You've exceeded the number of tags allowed for a resource. See the <a href="https://docs.aws.amazon.com/step-functions/latest/dg/limits.html"> Limits Topic</a> in the AWS Step Functions Developer Guide.</p>
    TooManyTags(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArn" => return RusotoError::Service(TagResourceError::InvalidArn(err.msg)),
                "ResourceNotFound" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
                }
                "TooManyTags" => {
                    return RusotoError::Service(TagResourceError::TooManyTags(err.msg))
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
            TagResourceError::InvalidArn(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::TooManyTags(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>The provided Amazon Resource Name (ARN) is invalid.</p>
    InvalidArn(String),
    /// <p>Could not find the referenced resource. Only state machine and activity ARNs are supported.</p>
    ResourceNotFound(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArn" => {
                    return RusotoError::Service(UntagResourceError::InvalidArn(err.msg))
                }
                "ResourceNotFound" => {
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
            UntagResourceError::InvalidArn(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateStateMachine
#[derive(Debug, PartialEq)]
pub enum UpdateStateMachineError {
    /// <p>The provided Amazon Resource Name (ARN) is invalid.</p>
    InvalidArn(String),
    /// <p>The provided Amazon States Language definition is invalid.</p>
    InvalidDefinition(String),
    /// <p><p/></p>
    InvalidLoggingConfiguration(String),
    /// <p>Request is missing a required parameter. This error occurs if both <code>definition</code> and <code>roleArn</code> are not specified.</p>
    MissingRequiredParameter(String),
    /// <p>The specified state machine is being deleted.</p>
    StateMachineDeleting(String),
    /// <p>The specified state machine does not exist.</p>
    StateMachineDoesNotExist(String),
}

impl UpdateStateMachineError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateStateMachineError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArn" => {
                    return RusotoError::Service(UpdateStateMachineError::InvalidArn(err.msg))
                }
                "InvalidDefinition" => {
                    return RusotoError::Service(UpdateStateMachineError::InvalidDefinition(
                        err.msg,
                    ))
                }
                "InvalidLoggingConfiguration" => {
                    return RusotoError::Service(
                        UpdateStateMachineError::InvalidLoggingConfiguration(err.msg),
                    )
                }
                "MissingRequiredParameter" => {
                    return RusotoError::Service(UpdateStateMachineError::MissingRequiredParameter(
                        err.msg,
                    ))
                }
                "StateMachineDeleting" => {
                    return RusotoError::Service(UpdateStateMachineError::StateMachineDeleting(
                        err.msg,
                    ))
                }
                "StateMachineDoesNotExist" => {
                    return RusotoError::Service(UpdateStateMachineError::StateMachineDoesNotExist(
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
impl fmt::Display for UpdateStateMachineError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateStateMachineError::InvalidArn(ref cause) => write!(f, "{}", cause),
            UpdateStateMachineError::InvalidDefinition(ref cause) => write!(f, "{}", cause),
            UpdateStateMachineError::InvalidLoggingConfiguration(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateStateMachineError::MissingRequiredParameter(ref cause) => write!(f, "{}", cause),
            UpdateStateMachineError::StateMachineDeleting(ref cause) => write!(f, "{}", cause),
            UpdateStateMachineError::StateMachineDoesNotExist(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateStateMachineError {}
/// Trait representing the capabilities of the AWS SFN API. AWS SFN clients implement this trait.
#[async_trait]
pub trait StepFunctions {
    /// <p><p>Creates an activity. An activity is a task that you write in any programming language and host on any machine that has access to AWS Step Functions. Activities must poll Step Functions using the <code>GetActivityTask</code> API action and respond using <code>SendTask*</code> API actions. This function lets Step Functions know the existence of your activity and returns an identifier for use in a state machine and when polling from the activity.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not reflect very recent updates and changes.</p> </note> <note> <p> <code>CreateActivity</code> is an idempotent API. Subsequent requests won’t create a duplicate resource if it was already created. <code>CreateActivity</code>&#39;s idempotency check is based on the activity <code>name</code>. If a following request has different <code>tags</code> values, Step Functions will ignore these differences and treat it as an idempotent request of the previous. In this case, <code>tags</code> will not be updated, even if they are different.</p> </note></p>
    async fn create_activity(
        &self,
        input: CreateActivityInput,
    ) -> Result<CreateActivityOutput, RusotoError<CreateActivityError>>;

    /// <p><p>Creates a state machine. A state machine consists of a collection of states that can do work (<code>Task</code> states), determine to which states to transition next (<code>Choice</code> states), stop an execution with an error (<code>Fail</code> states), and so on. State machines are specified using a JSON-based, structured language.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not reflect very recent updates and changes.</p> </note> <note> <p> <code>CreateStateMachine</code> is an idempotent API. Subsequent requests won’t create a duplicate resource if it was already created. <code>CreateStateMachine</code>&#39;s idempotency check is based on the state machine <code>name</code> and <code>definition</code>. If a following request has a different <code>roleArn</code> or <code>tags</code>, Step Functions will ignore these differences and treat it as an idempotent request of the previous. In this case, <code>roleArn</code> and <code>tags</code> will not be updated, even if they are different.</p> </note></p>
    async fn create_state_machine(
        &self,
        input: CreateStateMachineInput,
    ) -> Result<CreateStateMachineOutput, RusotoError<CreateStateMachineError>>;

    /// <p>Deletes an activity.</p>
    async fn delete_activity(
        &self,
        input: DeleteActivityInput,
    ) -> Result<DeleteActivityOutput, RusotoError<DeleteActivityError>>;

    /// <p><p>Deletes a state machine. This is an asynchronous operation: It sets the state machine&#39;s status to <code>DELETING</code> and begins the deletion process. Each state machine execution is deleted the next time it makes a state transition.</p> <note> <p>The state machine itself is deleted after all executions are completed or deleted.</p> </note></p>
    async fn delete_state_machine(
        &self,
        input: DeleteStateMachineInput,
    ) -> Result<DeleteStateMachineOutput, RusotoError<DeleteStateMachineError>>;

    /// <p><p>Describes an activity.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not reflect very recent updates and changes.</p> </note></p>
    async fn describe_activity(
        &self,
        input: DescribeActivityInput,
    ) -> Result<DescribeActivityOutput, RusotoError<DescribeActivityError>>;

    /// <p><p>Describes an execution.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not reflect very recent updates and changes.</p> </note></p>
    async fn describe_execution(
        &self,
        input: DescribeExecutionInput,
    ) -> Result<DescribeExecutionOutput, RusotoError<DescribeExecutionError>>;

    /// <p><p>Describes a state machine.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not reflect very recent updates and changes.</p> </note></p>
    async fn describe_state_machine(
        &self,
        input: DescribeStateMachineInput,
    ) -> Result<DescribeStateMachineOutput, RusotoError<DescribeStateMachineError>>;

    /// <p><p>Describes the state machine associated with a specific execution.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not reflect very recent updates and changes.</p> </note></p>
    async fn describe_state_machine_for_execution(
        &self,
        input: DescribeStateMachineForExecutionInput,
    ) -> Result<
        DescribeStateMachineForExecutionOutput,
        RusotoError<DescribeStateMachineForExecutionError>,
    >;

    /// <p><p>Used by workers to retrieve a task (with the specified activity ARN) which has been scheduled for execution by a running state machine. This initiates a long poll, where the service holds the HTTP connection open and responds as soon as a task becomes available (i.e. an execution of a task of this type is needed.) The maximum time the service holds on to the request before responding is 60 seconds. If no task is available within 60 seconds, the poll returns a <code>taskToken</code> with a null string.</p> <important> <p>Workers should set their client side socket timeout to at least 65 seconds (5 seconds higher than the maximum time the service may hold the poll request).</p> <p>Polling with <code>GetActivityTask</code> can cause latency in some implementations. See <a href="https://docs.aws.amazon.com/step-functions/latest/dg/bp-activity-pollers.html">Avoid Latency When Polling for Activity Tasks</a> in the Step Functions Developer Guide.</p> </important></p>
    async fn get_activity_task(
        &self,
        input: GetActivityTaskInput,
    ) -> Result<GetActivityTaskOutput, RusotoError<GetActivityTaskError>>;

    /// <p>Returns the history of the specified execution as a list of events. By default, the results are returned in ascending order of the <code>timeStamp</code> of the events. Use the <code>reverseOrder</code> parameter to get the latest events first.</p> <p>If <code>nextToken</code> is returned, there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return an <i>HTTP 400 InvalidToken</i> error.</p>
    async fn get_execution_history(
        &self,
        input: GetExecutionHistoryInput,
    ) -> Result<GetExecutionHistoryOutput, RusotoError<GetExecutionHistoryError>>;

    /// <p><p>Lists the existing activities.</p> <p>If <code>nextToken</code> is returned, there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return an <i>HTTP 400 InvalidToken</i> error.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not reflect very recent updates and changes.</p> </note></p>
    async fn list_activities(
        &self,
        input: ListActivitiesInput,
    ) -> Result<ListActivitiesOutput, RusotoError<ListActivitiesError>>;

    /// <p><p>Lists the executions of a state machine that meet the filtering criteria. Results are sorted by time, with the most recent execution first.</p> <p>If <code>nextToken</code> is returned, there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return an <i>HTTP 400 InvalidToken</i> error.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not reflect very recent updates and changes.</p> </note></p>
    async fn list_executions(
        &self,
        input: ListExecutionsInput,
    ) -> Result<ListExecutionsOutput, RusotoError<ListExecutionsError>>;

    /// <p><p>Lists the existing state machines.</p> <p>If <code>nextToken</code> is returned, there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return an <i>HTTP 400 InvalidToken</i> error.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not reflect very recent updates and changes.</p> </note></p>
    async fn list_state_machines(
        &self,
        input: ListStateMachinesInput,
    ) -> Result<ListStateMachinesOutput, RusotoError<ListStateMachinesError>>;

    /// <p>List tags for a given resource.</p> <p>Tags may only contain Unicode letters, digits, white space, or these symbols: <code>_ . : / = + - @</code>.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceInput,
    ) -> Result<ListTagsForResourceOutput, RusotoError<ListTagsForResourceError>>;

    /// <p>Used by activity workers and task states using the <a href="https://docs.aws.amazon.com/step-functions/latest/dg/connect-to-resource.html#connect-wait-token">callback</a> pattern to report that the task identified by the <code>taskToken</code> failed.</p>
    async fn send_task_failure(
        &self,
        input: SendTaskFailureInput,
    ) -> Result<SendTaskFailureOutput, RusotoError<SendTaskFailureError>>;

    /// <p><p>Used by activity workers and task states using the <a href="https://docs.aws.amazon.com/step-functions/latest/dg/connect-to-resource.html#connect-wait-token">callback</a> pattern to report to Step Functions that the task represented by the specified <code>taskToken</code> is still making progress. This action resets the <code>Heartbeat</code> clock. The <code>Heartbeat</code> threshold is specified in the state machine&#39;s Amazon States Language definition (<code>HeartbeatSeconds</code>). This action does not in itself create an event in the execution history. However, if the task times out, the execution history contains an <code>ActivityTimedOut</code> entry for activities, or a <code>TaskTimedOut</code> entry for for tasks using the <a href="https://docs.aws.amazon.com/step-functions/latest/dg/connect-to-resource.html#connect-sync">job run</a> or <a href="https://docs.aws.amazon.com/step-functions/latest/dg/connect-to-resource.html#connect-wait-token">callback</a> pattern.</p> <note> <p>The <code>Timeout</code> of a task, defined in the state machine&#39;s Amazon States Language definition, is its maximum allowed duration, regardless of the number of <a>SendTaskHeartbeat</a> requests received. Use <code>HeartbeatSeconds</code> to configure the timeout interval for heartbeats.</p> </note></p>
    async fn send_task_heartbeat(
        &self,
        input: SendTaskHeartbeatInput,
    ) -> Result<SendTaskHeartbeatOutput, RusotoError<SendTaskHeartbeatError>>;

    /// <p>Used by activity workers and task states using the <a href="https://docs.aws.amazon.com/step-functions/latest/dg/connect-to-resource.html#connect-wait-token">callback</a> pattern to report that the task identified by the <code>taskToken</code> completed successfully.</p>
    async fn send_task_success(
        &self,
        input: SendTaskSuccessInput,
    ) -> Result<SendTaskSuccessOutput, RusotoError<SendTaskSuccessError>>;

    /// <p><p>Starts a state machine execution.</p> <note> <p> <code>StartExecution</code> is idempotent. If <code>StartExecution</code> is called with the same name and input as a running execution, the call will succeed and return the same response as the original request. If the execution is closed or if the input is different, it will return a 400 <code>ExecutionAlreadyExists</code> error. Names can be reused after 90 days. </p> </note></p>
    async fn start_execution(
        &self,
        input: StartExecutionInput,
    ) -> Result<StartExecutionOutput, RusotoError<StartExecutionError>>;

    /// <p>Stops an execution.</p>
    async fn stop_execution(
        &self,
        input: StopExecutionInput,
    ) -> Result<StopExecutionOutput, RusotoError<StopExecutionError>>;

    /// <p>Add a tag to a Step Functions resource.</p> <p>An array of key-value pairs. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>, and <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_iam-tags.html">Controlling Access Using IAM Tags</a>.</p> <p>Tags may only contain Unicode letters, digits, white space, or these symbols: <code>_ . : / = + - @</code>.</p>
    async fn tag_resource(
        &self,
        input: TagResourceInput,
    ) -> Result<TagResourceOutput, RusotoError<TagResourceError>>;

    /// <p>Remove a tag from a Step Functions resource</p>
    async fn untag_resource(
        &self,
        input: UntagResourceInput,
    ) -> Result<UntagResourceOutput, RusotoError<UntagResourceError>>;

    /// <p><p>Updates an existing state machine by modifying its <code>definition</code> and/or <code>roleArn</code>. Running executions will continue to use the previous <code>definition</code> and <code>roleArn</code>. You must include at least one of <code>definition</code> or <code>roleArn</code> or you will receive a <code>MissingRequiredParameter</code> error.</p> <note> <p>All <code>StartExecution</code> calls within a few seconds will use the updated <code>definition</code> and <code>roleArn</code>. Executions started immediately after calling <code>UpdateStateMachine</code> may use the previous state machine <code>definition</code> and <code>roleArn</code>. </p> </note></p>
    async fn update_state_machine(
        &self,
        input: UpdateStateMachineInput,
    ) -> Result<UpdateStateMachineOutput, RusotoError<UpdateStateMachineError>>;
}
/// A client for the AWS SFN API.
#[derive(Clone)]
pub struct StepFunctionsClient {
    client: Client,
    region: region::Region,
}

impl StepFunctionsClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> StepFunctionsClient {
        StepFunctionsClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> StepFunctionsClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        StepFunctionsClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> StepFunctionsClient {
        StepFunctionsClient { client, region }
    }
}

#[async_trait]
impl StepFunctions for StepFunctionsClient {
    /// <p><p>Creates an activity. An activity is a task that you write in any programming language and host on any machine that has access to AWS Step Functions. Activities must poll Step Functions using the <code>GetActivityTask</code> API action and respond using <code>SendTask*</code> API actions. This function lets Step Functions know the existence of your activity and returns an identifier for use in a state machine and when polling from the activity.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not reflect very recent updates and changes.</p> </note> <note> <p> <code>CreateActivity</code> is an idempotent API. Subsequent requests won’t create a duplicate resource if it was already created. <code>CreateActivity</code>&#39;s idempotency check is based on the activity <code>name</code>. If a following request has different <code>tags</code> values, Step Functions will ignore these differences and treat it as an idempotent request of the previous. In this case, <code>tags</code> will not be updated, even if they are different.</p> </note></p>
    async fn create_activity(
        &self,
        input: CreateActivityInput,
    ) -> Result<CreateActivityOutput, RusotoError<CreateActivityError>> {
        let mut request = SignedRequest::new("POST", "states", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "AWSStepFunctions.CreateActivity");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateActivityOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateActivityError::from_response(response))
        }
    }

    /// <p><p>Creates a state machine. A state machine consists of a collection of states that can do work (<code>Task</code> states), determine to which states to transition next (<code>Choice</code> states), stop an execution with an error (<code>Fail</code> states), and so on. State machines are specified using a JSON-based, structured language.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not reflect very recent updates and changes.</p> </note> <note> <p> <code>CreateStateMachine</code> is an idempotent API. Subsequent requests won’t create a duplicate resource if it was already created. <code>CreateStateMachine</code>&#39;s idempotency check is based on the state machine <code>name</code> and <code>definition</code>. If a following request has a different <code>roleArn</code> or <code>tags</code>, Step Functions will ignore these differences and treat it as an idempotent request of the previous. In this case, <code>roleArn</code> and <code>tags</code> will not be updated, even if they are different.</p> </note></p>
    async fn create_state_machine(
        &self,
        input: CreateStateMachineInput,
    ) -> Result<CreateStateMachineOutput, RusotoError<CreateStateMachineError>> {
        let mut request = SignedRequest::new("POST", "states", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "AWSStepFunctions.CreateStateMachine");
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
                .deserialize::<CreateStateMachineOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateStateMachineError::from_response(response))
        }
    }

    /// <p>Deletes an activity.</p>
    async fn delete_activity(
        &self,
        input: DeleteActivityInput,
    ) -> Result<DeleteActivityOutput, RusotoError<DeleteActivityError>> {
        let mut request = SignedRequest::new("POST", "states", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "AWSStepFunctions.DeleteActivity");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteActivityOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteActivityError::from_response(response))
        }
    }

    /// <p><p>Deletes a state machine. This is an asynchronous operation: It sets the state machine&#39;s status to <code>DELETING</code> and begins the deletion process. Each state machine execution is deleted the next time it makes a state transition.</p> <note> <p>The state machine itself is deleted after all executions are completed or deleted.</p> </note></p>
    async fn delete_state_machine(
        &self,
        input: DeleteStateMachineInput,
    ) -> Result<DeleteStateMachineOutput, RusotoError<DeleteStateMachineError>> {
        let mut request = SignedRequest::new("POST", "states", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "AWSStepFunctions.DeleteStateMachine");
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
                .deserialize::<DeleteStateMachineOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteStateMachineError::from_response(response))
        }
    }

    /// <p><p>Describes an activity.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not reflect very recent updates and changes.</p> </note></p>
    async fn describe_activity(
        &self,
        input: DescribeActivityInput,
    ) -> Result<DescribeActivityOutput, RusotoError<DescribeActivityError>> {
        let mut request = SignedRequest::new("POST", "states", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "AWSStepFunctions.DescribeActivity");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeActivityOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeActivityError::from_response(response))
        }
    }

    /// <p><p>Describes an execution.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not reflect very recent updates and changes.</p> </note></p>
    async fn describe_execution(
        &self,
        input: DescribeExecutionInput,
    ) -> Result<DescribeExecutionOutput, RusotoError<DescribeExecutionError>> {
        let mut request = SignedRequest::new("POST", "states", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "AWSStepFunctions.DescribeExecution");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeExecutionOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeExecutionError::from_response(response))
        }
    }

    /// <p><p>Describes a state machine.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not reflect very recent updates and changes.</p> </note></p>
    async fn describe_state_machine(
        &self,
        input: DescribeStateMachineInput,
    ) -> Result<DescribeStateMachineOutput, RusotoError<DescribeStateMachineError>> {
        let mut request = SignedRequest::new("POST", "states", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "AWSStepFunctions.DescribeStateMachine");
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
                .deserialize::<DescribeStateMachineOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeStateMachineError::from_response(response))
        }
    }

    /// <p><p>Describes the state machine associated with a specific execution.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not reflect very recent updates and changes.</p> </note></p>
    async fn describe_state_machine_for_execution(
        &self,
        input: DescribeStateMachineForExecutionInput,
    ) -> Result<
        DescribeStateMachineForExecutionOutput,
        RusotoError<DescribeStateMachineForExecutionError>,
    > {
        let mut request = SignedRequest::new("POST", "states", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSStepFunctions.DescribeStateMachineForExecution",
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
                .deserialize::<DescribeStateMachineForExecutionOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeStateMachineForExecutionError::from_response(
                response,
            ))
        }
    }

    /// <p><p>Used by workers to retrieve a task (with the specified activity ARN) which has been scheduled for execution by a running state machine. This initiates a long poll, where the service holds the HTTP connection open and responds as soon as a task becomes available (i.e. an execution of a task of this type is needed.) The maximum time the service holds on to the request before responding is 60 seconds. If no task is available within 60 seconds, the poll returns a <code>taskToken</code> with a null string.</p> <important> <p>Workers should set their client side socket timeout to at least 65 seconds (5 seconds higher than the maximum time the service may hold the poll request).</p> <p>Polling with <code>GetActivityTask</code> can cause latency in some implementations. See <a href="https://docs.aws.amazon.com/step-functions/latest/dg/bp-activity-pollers.html">Avoid Latency When Polling for Activity Tasks</a> in the Step Functions Developer Guide.</p> </important></p>
    async fn get_activity_task(
        &self,
        input: GetActivityTaskInput,
    ) -> Result<GetActivityTaskOutput, RusotoError<GetActivityTaskError>> {
        let mut request = SignedRequest::new("POST", "states", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "AWSStepFunctions.GetActivityTask");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetActivityTaskOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetActivityTaskError::from_response(response))
        }
    }

    /// <p>Returns the history of the specified execution as a list of events. By default, the results are returned in ascending order of the <code>timeStamp</code> of the events. Use the <code>reverseOrder</code> parameter to get the latest events first.</p> <p>If <code>nextToken</code> is returned, there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return an <i>HTTP 400 InvalidToken</i> error.</p>
    async fn get_execution_history(
        &self,
        input: GetExecutionHistoryInput,
    ) -> Result<GetExecutionHistoryOutput, RusotoError<GetExecutionHistoryError>> {
        let mut request = SignedRequest::new("POST", "states", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "AWSStepFunctions.GetExecutionHistory");
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
                .deserialize::<GetExecutionHistoryOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetExecutionHistoryError::from_response(response))
        }
    }

    /// <p><p>Lists the existing activities.</p> <p>If <code>nextToken</code> is returned, there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return an <i>HTTP 400 InvalidToken</i> error.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not reflect very recent updates and changes.</p> </note></p>
    async fn list_activities(
        &self,
        input: ListActivitiesInput,
    ) -> Result<ListActivitiesOutput, RusotoError<ListActivitiesError>> {
        let mut request = SignedRequest::new("POST", "states", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "AWSStepFunctions.ListActivities");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListActivitiesOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListActivitiesError::from_response(response))
        }
    }

    /// <p><p>Lists the executions of a state machine that meet the filtering criteria. Results are sorted by time, with the most recent execution first.</p> <p>If <code>nextToken</code> is returned, there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return an <i>HTTP 400 InvalidToken</i> error.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not reflect very recent updates and changes.</p> </note></p>
    async fn list_executions(
        &self,
        input: ListExecutionsInput,
    ) -> Result<ListExecutionsOutput, RusotoError<ListExecutionsError>> {
        let mut request = SignedRequest::new("POST", "states", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "AWSStepFunctions.ListExecutions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListExecutionsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListExecutionsError::from_response(response))
        }
    }

    /// <p><p>Lists the existing state machines.</p> <p>If <code>nextToken</code> is returned, there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return an <i>HTTP 400 InvalidToken</i> error.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not reflect very recent updates and changes.</p> </note></p>
    async fn list_state_machines(
        &self,
        input: ListStateMachinesInput,
    ) -> Result<ListStateMachinesOutput, RusotoError<ListStateMachinesError>> {
        let mut request = SignedRequest::new("POST", "states", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "AWSStepFunctions.ListStateMachines");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListStateMachinesOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListStateMachinesError::from_response(response))
        }
    }

    /// <p>List tags for a given resource.</p> <p>Tags may only contain Unicode letters, digits, white space, or these symbols: <code>_ . : / = + - @</code>.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceInput,
    ) -> Result<ListTagsForResourceOutput, RusotoError<ListTagsForResourceError>> {
        let mut request = SignedRequest::new("POST", "states", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "AWSStepFunctions.ListTagsForResource");
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
                .deserialize::<ListTagsForResourceOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p>Used by activity workers and task states using the <a href="https://docs.aws.amazon.com/step-functions/latest/dg/connect-to-resource.html#connect-wait-token">callback</a> pattern to report that the task identified by the <code>taskToken</code> failed.</p>
    async fn send_task_failure(
        &self,
        input: SendTaskFailureInput,
    ) -> Result<SendTaskFailureOutput, RusotoError<SendTaskFailureError>> {
        let mut request = SignedRequest::new("POST", "states", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "AWSStepFunctions.SendTaskFailure");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<SendTaskFailureOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(SendTaskFailureError::from_response(response))
        }
    }

    /// <p><p>Used by activity workers and task states using the <a href="https://docs.aws.amazon.com/step-functions/latest/dg/connect-to-resource.html#connect-wait-token">callback</a> pattern to report to Step Functions that the task represented by the specified <code>taskToken</code> is still making progress. This action resets the <code>Heartbeat</code> clock. The <code>Heartbeat</code> threshold is specified in the state machine&#39;s Amazon States Language definition (<code>HeartbeatSeconds</code>). This action does not in itself create an event in the execution history. However, if the task times out, the execution history contains an <code>ActivityTimedOut</code> entry for activities, or a <code>TaskTimedOut</code> entry for for tasks using the <a href="https://docs.aws.amazon.com/step-functions/latest/dg/connect-to-resource.html#connect-sync">job run</a> or <a href="https://docs.aws.amazon.com/step-functions/latest/dg/connect-to-resource.html#connect-wait-token">callback</a> pattern.</p> <note> <p>The <code>Timeout</code> of a task, defined in the state machine&#39;s Amazon States Language definition, is its maximum allowed duration, regardless of the number of <a>SendTaskHeartbeat</a> requests received. Use <code>HeartbeatSeconds</code> to configure the timeout interval for heartbeats.</p> </note></p>
    async fn send_task_heartbeat(
        &self,
        input: SendTaskHeartbeatInput,
    ) -> Result<SendTaskHeartbeatOutput, RusotoError<SendTaskHeartbeatError>> {
        let mut request = SignedRequest::new("POST", "states", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "AWSStepFunctions.SendTaskHeartbeat");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<SendTaskHeartbeatOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(SendTaskHeartbeatError::from_response(response))
        }
    }

    /// <p>Used by activity workers and task states using the <a href="https://docs.aws.amazon.com/step-functions/latest/dg/connect-to-resource.html#connect-wait-token">callback</a> pattern to report that the task identified by the <code>taskToken</code> completed successfully.</p>
    async fn send_task_success(
        &self,
        input: SendTaskSuccessInput,
    ) -> Result<SendTaskSuccessOutput, RusotoError<SendTaskSuccessError>> {
        let mut request = SignedRequest::new("POST", "states", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "AWSStepFunctions.SendTaskSuccess");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<SendTaskSuccessOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(SendTaskSuccessError::from_response(response))
        }
    }

    /// <p><p>Starts a state machine execution.</p> <note> <p> <code>StartExecution</code> is idempotent. If <code>StartExecution</code> is called with the same name and input as a running execution, the call will succeed and return the same response as the original request. If the execution is closed or if the input is different, it will return a 400 <code>ExecutionAlreadyExists</code> error. Names can be reused after 90 days. </p> </note></p>
    async fn start_execution(
        &self,
        input: StartExecutionInput,
    ) -> Result<StartExecutionOutput, RusotoError<StartExecutionError>> {
        let mut request = SignedRequest::new("POST", "states", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "AWSStepFunctions.StartExecution");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<StartExecutionOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StartExecutionError::from_response(response))
        }
    }

    /// <p>Stops an execution.</p>
    async fn stop_execution(
        &self,
        input: StopExecutionInput,
    ) -> Result<StopExecutionOutput, RusotoError<StopExecutionError>> {
        let mut request = SignedRequest::new("POST", "states", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "AWSStepFunctions.StopExecution");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<StopExecutionOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StopExecutionError::from_response(response))
        }
    }

    /// <p>Add a tag to a Step Functions resource.</p> <p>An array of key-value pairs. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>, and <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_iam-tags.html">Controlling Access Using IAM Tags</a>.</p> <p>Tags may only contain Unicode letters, digits, white space, or these symbols: <code>_ . : / = + - @</code>.</p>
    async fn tag_resource(
        &self,
        input: TagResourceInput,
    ) -> Result<TagResourceOutput, RusotoError<TagResourceError>> {
        let mut request = SignedRequest::new("POST", "states", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "AWSStepFunctions.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<TagResourceOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Remove a tag from a Step Functions resource</p>
    async fn untag_resource(
        &self,
        input: UntagResourceInput,
    ) -> Result<UntagResourceOutput, RusotoError<UntagResourceError>> {
        let mut request = SignedRequest::new("POST", "states", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "AWSStepFunctions.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UntagResourceOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p><p>Updates an existing state machine by modifying its <code>definition</code> and/or <code>roleArn</code>. Running executions will continue to use the previous <code>definition</code> and <code>roleArn</code>. You must include at least one of <code>definition</code> or <code>roleArn</code> or you will receive a <code>MissingRequiredParameter</code> error.</p> <note> <p>All <code>StartExecution</code> calls within a few seconds will use the updated <code>definition</code> and <code>roleArn</code>. Executions started immediately after calling <code>UpdateStateMachine</code> may use the previous state machine <code>definition</code> and <code>roleArn</code>. </p> </note></p>
    async fn update_state_machine(
        &self,
        input: UpdateStateMachineInput,
    ) -> Result<UpdateStateMachineOutput, RusotoError<UpdateStateMachineError>> {
        let mut request = SignedRequest::new("POST", "states", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "AWSStepFunctions.UpdateStateMachine");
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
                .deserialize::<UpdateStateMachineOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateStateMachineError::from_response(response))
        }
    }
}
