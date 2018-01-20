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

#[allow(warnings)]
use hyper::Client;
use hyper::status::StatusCode;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::region;

use std::fmt;
use std::error::Error;
use std::io;
use std::io::Read;
use rusoto_core::request::HttpDispatchError;
use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};

use serde_json;
use rusoto_core::signature::SignedRequest;
use serde_json::Value as SerdeJsonValue;
use serde_json::from_str;
#[derive(Default, Debug, Clone, Deserialize)]
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

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ActivityListItem {
    /// <p>The Amazon Resource Name (ARN) that identifies the activity.</p>
    #[serde(rename = "activityArn")]
    pub activity_arn: String,
    /// <p>The date the activity was created.</p>
    #[serde(rename = "creationDate")]
    pub creation_date: f64,
    /// <p>The name of the activity.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
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

#[derive(Default, Debug, Clone, Deserialize)]
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

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ActivityStartedEventDetails {
    /// <p>The name of the worker that the task was assigned to. These names are provided by the workers when calling <a>GetActivityTask</a>.</p>
    #[serde(rename = "workerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_name: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ActivitySucceededEventDetails {
    /// <p>The JSON data output by the activity task.</p>
    #[serde(rename = "output")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
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

#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateActivityInput {
    /// <p>The name of the activity to create. This name must be unique for your AWS account and region.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreateActivityOutput {
    /// <p>The Amazon Resource Name (ARN) that identifies the created activity.</p>
    #[serde(rename = "activityArn")]
    pub activity_arn: String,
    /// <p>The date the activity was created.</p>
    #[serde(rename = "creationDate")]
    pub creation_date: f64,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateStateMachineInput {
    /// <p>The Amazon States Language definition of the state machine.</p>
    #[serde(rename = "definition")]
    pub definition: String,
    /// <p>The name of the state machine. This name must be unique for your AWS account and region.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The Amazon Resource Name (ARN) of the IAM role to use for this state machine.</p>
    #[serde(rename = "roleArn")]
    pub role_arn: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreateStateMachineOutput {
    /// <p>The date the state machine was created.</p>
    #[serde(rename = "creationDate")]
    pub creation_date: f64,
    /// <p>The Amazon Resource Name (ARN) that identifies the created state machine.</p>
    #[serde(rename = "stateMachineArn")]
    pub state_machine_arn: String,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct DeleteActivityInput {
    /// <p>The Amazon Resource Name (ARN) of the activity to delete.</p>
    #[serde(rename = "activityArn")]
    pub activity_arn: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DeleteActivityOutput;

#[derive(Default, Debug, Clone, Serialize)]
pub struct DeleteStateMachineInput {
    /// <p>The Amazon Resource Name (ARN) of the state machine to delete.</p>
    #[serde(rename = "stateMachineArn")]
    pub state_machine_arn: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DeleteStateMachineOutput;

#[derive(Default, Debug, Clone, Serialize)]
pub struct DescribeActivityInput {
    /// <p>The Amazon Resource Name (ARN) of the activity to describe.</p>
    #[serde(rename = "activityArn")]
    pub activity_arn: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DescribeActivityOutput {
    /// <p>The Amazon Resource Name (ARN) that identifies the activity.</p>
    #[serde(rename = "activityArn")]
    pub activity_arn: String,
    /// <p>The date the activity was created.</p>
    #[serde(rename = "creationDate")]
    pub creation_date: f64,
    /// <p>The name of the activity.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct DescribeExecutionInput {
    /// <p>The Amazon Resource Name (ARN) of the execution to describe.</p>
    #[serde(rename = "executionArn")]
    pub execution_arn: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DescribeExecutionOutput {
    /// <p>The Amazon Resource Name (ARN) that identifies the execution.</p>
    #[serde(rename = "executionArn")]
    pub execution_arn: String,
    /// <p>The JSON input data of the execution.</p>
    #[serde(rename = "input")]
    pub input: String,
    /// <p>The name of the execution.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The JSON output data of the execution.</p>
    #[serde(rename = "output")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    /// <p>The date the execution was started.</p>
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

#[derive(Default, Debug, Clone, Serialize)]
pub struct DescribeStateMachineInput {
    /// <p>The Amazon Resource Name (ARN) of the state machine to describe.</p>
    #[serde(rename = "stateMachineArn")]
    pub state_machine_arn: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DescribeStateMachineOutput {
    /// <p>The date the state machine was created.</p>
    #[serde(rename = "creationDate")]
    pub creation_date: f64,
    /// <p>The Amazon States Language definition of the state machine.</p>
    #[serde(rename = "definition")]
    pub definition: String,
    /// <p>The name of the state machine.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The Amazon Resource Name (ARN) of the IAM role used for executing this state machine.</p>
    #[serde(rename = "roleArn")]
    pub role_arn: String,
    /// <p>The Amazon Resource Name (ARN) that identifies the state machine.</p>
    #[serde(rename = "stateMachineArn")]
    pub state_machine_arn: String,
    /// <p>The current status of the state machine.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
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

#[derive(Default, Debug, Clone, Deserialize)]
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

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ExecutionListItem {
    /// <p>The Amazon Resource Name (ARN) that identifies the execution.</p>
    #[serde(rename = "executionArn")]
    pub execution_arn: String,
    /// <p>The name of the execution.</p>
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

#[derive(Default, Debug, Clone, Deserialize)]
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

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ExecutionSucceededEventDetails {
    /// <p>The JSON data output by the execution.</p>
    #[serde(rename = "output")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
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

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetActivityTaskInput {
    /// <p>The Amazon Resource Name (ARN) of the activity to retrieve tasks from.</p>
    #[serde(rename = "activityArn")]
    pub activity_arn: String,
    /// <p>An arbitrary name may be provided in order to identify the worker that the task is assigned to. This name will be used when it is logged in the execution history.</p>
    #[serde(rename = "workerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_name: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetActivityTaskOutput {
    /// <p>The JSON input data for the task.</p>
    #[serde(rename = "input")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    /// <p>A token that identifies the scheduled task. This token must be copied and included in subsequent calls to <a>SendTaskHeartbeat</a>, <a>SendTaskSuccess</a> or <a>SendTaskFailure</a> in order to report the progress or completion of the task.</p>
    #[serde(rename = "taskToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_token: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetExecutionHistoryInput {
    /// <p>The Amazon Resource Name (ARN) of the execution.</p>
    #[serde(rename = "executionArn")]
    pub execution_arn: String,
    /// <p>The maximum number of results that will be returned per call. <code>nextToken</code> can be used to obtain further pages of results. The default is 100 and the maximum allowed page size is 1000.</p> <p>This is an upper limit only; the actual number of results returned per call may be fewer than the specified maximum.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If a <code>nextToken</code> was returned by a previous call, there are more results available. To retrieve the next page of results, make the call again using the returned token in <code>nextToken</code>. Keep all other arguments unchanged.</p> <p>The configured <code>maxResults</code> determines how many results can be returned in a single call.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Lists events in descending order of their <code>timeStamp</code>.</p>
    #[serde(rename = "reverseOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_order: Option<bool>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetExecutionHistoryOutput {
    /// <p>The list of events that occurred in the execution.</p>
    #[serde(rename = "events")]
    pub events: Vec<HistoryEvent>,
    /// <p>If a <code>nextToken</code> is returned, there are more results available. To retrieve the next page of results, make the call again using the returned token in <code>nextToken</code>. Keep all other arguments unchanged.</p> <p>The configured <code>maxResults</code> determines how many results can be returned in a single call.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct HistoryEvent {
    #[serde(rename = "activityFailedEventDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_failed_event_details: Option<ActivityFailedEventDetails>,
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
    #[serde(rename = "lambdaFunctionStartFailedEventDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_start_failed_event_details: Option<LambdaFunctionStartFailedEventDetails>,
    #[serde(rename = "lambdaFunctionSucceededEventDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_succeeded_event_details: Option<LambdaFunctionSucceededEventDetails>,
    #[serde(rename = "lambdaFunctionTimedOutEventDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_timed_out_event_details: Option<LambdaFunctionTimedOutEventDetails>,
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
    /// <p>The date the event occured.</p>
    #[serde(rename = "timestamp")]
    pub timestamp: f64,
    /// <p>The type of the event.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
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

#[derive(Default, Debug, Clone, Deserialize)]
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

#[derive(Default, Debug, Clone, Deserialize)]
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

#[derive(Default, Debug, Clone, Deserialize)]
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

#[derive(Default, Debug, Clone, Deserialize)]
pub struct LambdaFunctionSucceededEventDetails {
    /// <p>The JSON data output by the lambda function.</p>
    #[serde(rename = "output")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
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

#[derive(Default, Debug, Clone, Serialize)]
pub struct ListActivitiesInput {
    /// <p>The maximum number of results that will be returned per call. <code>nextToken</code> can be used to obtain further pages of results. The default is 100 and the maximum allowed page size is 1000.</p> <p>This is an upper limit only; the actual number of results returned per call may be fewer than the specified maximum.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If a <code>nextToken</code> was returned by a previous call, there are more results available. To retrieve the next page of results, make the call again using the returned token in <code>nextToken</code>. Keep all other arguments unchanged.</p> <p>The configured <code>maxResults</code> determines how many results can be returned in a single call.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListActivitiesOutput {
    /// <p>The list of activities.</p>
    #[serde(rename = "activities")]
    pub activities: Vec<ActivityListItem>,
    /// <p>If a <code>nextToken</code> is returned, there are more results available. To retrieve the next page of results, make the call again using the returned token in <code>nextToken</code>. Keep all other arguments unchanged.</p> <p>The configured <code>maxResults</code> determines how many results can be returned in a single call.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct ListExecutionsInput {
    /// <p>The maximum number of results that will be returned per call. <code>nextToken</code> can be used to obtain further pages of results. The default is 100 and the maximum allowed page size is 1000.</p> <p>This is an upper limit only; the actual number of results returned per call may be fewer than the specified maximum.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If a <code>nextToken</code> was returned by a previous call, there are more results available. To retrieve the next page of results, make the call again using the returned token in <code>nextToken</code>. Keep all other arguments unchanged.</p> <p>The configured <code>maxResults</code> determines how many results can be returned in a single call.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the state machine whose executions will be listed.</p>
    #[serde(rename = "stateMachineArn")]
    pub state_machine_arn: String,
    /// <p>If specified, only list the executions whose current execution status matches the given filter.</p>
    #[serde(rename = "statusFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_filter: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListExecutionsOutput {
    /// <p>The list of matching executions.</p>
    #[serde(rename = "executions")]
    pub executions: Vec<ExecutionListItem>,
    /// <p>If a <code>nextToken</code> is returned, there are more results available. To retrieve the next page of results, make the call again using the returned token in <code>nextToken</code>. Keep all other arguments unchanged.</p> <p>The configured <code>maxResults</code> determines how many results can be returned in a single call.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct ListStateMachinesInput {
    /// <p>The maximum number of results that will be returned per call. <code>nextToken</code> can be used to obtain further pages of results. The default is 100 and the maximum allowed page size is 1000.</p> <p>This is an upper limit only; the actual number of results returned per call may be fewer than the specified maximum.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If a <code>nextToken</code> was returned by a previous call, there are more results available. To retrieve the next page of results, make the call again using the returned token in <code>nextToken</code>. Keep all other arguments unchanged.</p> <p>The configured <code>maxResults</code> determines how many results can be returned in a single call.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListStateMachinesOutput {
    /// <p>If a <code>nextToken</code> is returned, there are more results available. To retrieve the next page of results, make the call again using the returned token in <code>nextToken</code>. Keep all other arguments unchanged.</p> <p>The configured <code>maxResults</code> determines how many results can be returned in a single call.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "stateMachines")] pub state_machines: Vec<StateMachineListItem>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct SendTaskFailureInput {
    /// <p>A more detailed explanation of the cause of the failure.</p>
    #[serde(rename = "cause")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    /// <p>An arbitrary error code that identifies the cause of the failure.</p>
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// <p>The token that represents this task. Task tokens are generated by the service when the tasks are assigned to a worker (see GetActivityTask::taskToken).</p>
    #[serde(rename = "taskToken")]
    pub task_token: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct SendTaskFailureOutput;

#[derive(Default, Debug, Clone, Serialize)]
pub struct SendTaskHeartbeatInput {
    /// <p>The token that represents this task. Task tokens are generated by the service when the tasks are assigned to a worker (see GetActivityTask::taskToken).</p>
    #[serde(rename = "taskToken")]
    pub task_token: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct SendTaskHeartbeatOutput;

#[derive(Default, Debug, Clone, Serialize)]
pub struct SendTaskSuccessInput {
    /// <p>The JSON output of the task.</p>
    #[serde(rename = "output")]
    pub output: String,
    /// <p>The token that represents this task. Task tokens are generated by the service when the tasks are assigned to a worker (see GetActivityTask::taskToken).</p>
    #[serde(rename = "taskToken")]
    pub task_token: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct SendTaskSuccessOutput;

#[derive(Default, Debug, Clone, Serialize)]
pub struct StartExecutionInput {
    /// <p>The JSON input data for the execution.</p>
    #[serde(rename = "input")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    /// <p>The name of the execution. This name must be unique for your AWS account and region.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the state machine to execute.</p>
    #[serde(rename = "stateMachineArn")]
    pub state_machine_arn: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct StartExecutionOutput {
    /// <p>The Amazon Resource Name (ARN) that identifies the execution.</p>
    #[serde(rename = "executionArn")]
    pub execution_arn: String,
    /// <p>The date the execution was started.</p>
    #[serde(rename = "startDate")]
    pub start_date: f64,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct StateEnteredEventDetails {
    /// <p>The JSON input data to the state.</p>
    #[serde(rename = "input")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    /// <p>The name of the state.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct StateExitedEventDetails {
    /// <p>The name of the state.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The JSON output data of the state.</p>
    #[serde(rename = "output")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct StateMachineListItem {
    /// <p>The date the state machine was created.</p>
    #[serde(rename = "creationDate")]
    pub creation_date: f64,
    /// <p>The name of the state machine.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The Amazon Resource Name (ARN) that identifies the state machine.</p>
    #[serde(rename = "stateMachineArn")]
    pub state_machine_arn: String,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct StopExecutionInput {
    /// <p>A more detailed explanation of the cause of the termination.</p>
    #[serde(rename = "cause")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    /// <p>An arbitrary error code that identifies the cause of the termination.</p>
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the execution to stop.</p>
    #[serde(rename = "executionArn")]
    pub execution_arn: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct StopExecutionOutput {
    /// <p>The date the execution was stopped.</p>
    #[serde(rename = "stopDate")]
    pub stop_date: f64,
}

/// Errors returned by CreateActivity
#[derive(Debug, PartialEq)]
pub enum CreateActivityError {
    /// <p>The maximum number of activities has been reached. Existing activities must be deleted before a new activity can be created.</p>
    ActivityLimitExceeded(String),
    /// <p>The provided name is invalid.</p>
    InvalidName(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateActivityError {
    pub fn from_body(body: &str) -> CreateActivityError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ActivityLimitExceeded" => {
                        CreateActivityError::ActivityLimitExceeded(String::from(error_message))
                    }
                    "InvalidName" => CreateActivityError::InvalidName(String::from(error_message)),
                    "ValidationException" => {
                        CreateActivityError::Validation(error_message.to_string())
                    }
                    _ => CreateActivityError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateActivityError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateActivityError {
    fn from(err: serde_json::error::Error) -> CreateActivityError {
        CreateActivityError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateActivityError {
    fn from(err: CredentialsError) -> CreateActivityError {
        CreateActivityError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateActivityError {
    fn from(err: HttpDispatchError) -> CreateActivityError {
        CreateActivityError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateActivityError {
    fn from(err: io::Error) -> CreateActivityError {
        CreateActivityError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateActivityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateActivityError {
    fn description(&self) -> &str {
        match *self {
            CreateActivityError::ActivityLimitExceeded(ref cause) => cause,
            CreateActivityError::InvalidName(ref cause) => cause,
            CreateActivityError::Validation(ref cause) => cause,
            CreateActivityError::Credentials(ref err) => err.description(),
            CreateActivityError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateActivityError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateStateMachine
#[derive(Debug, PartialEq)]
pub enum CreateStateMachineError {
    /// <p>The provided Amazon Resource Name (ARN) is invalid.</p>
    InvalidArn(String),
    /// <p>The provided Amazon States Language definition is invalid.</p>
    InvalidDefinition(String),
    /// <p>The provided name is invalid.</p>
    InvalidName(String),
    /// <p>A state machine with the same name but a different definition or role ARN already exists.</p>
    StateMachineAlreadyExists(String),
    /// <p>The specified state machine is being deleted.</p>
    StateMachineDeleting(String),
    /// <p>The maximum number of state machines has been reached. Existing state machines must be deleted before a new state machine can be created.</p>
    StateMachineLimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateStateMachineError {
    pub fn from_body(body: &str) -> CreateStateMachineError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidArn" => {
                        CreateStateMachineError::InvalidArn(String::from(error_message))
                    }
                    "InvalidDefinition" => {
                        CreateStateMachineError::InvalidDefinition(String::from(error_message))
                    }
                    "InvalidName" => {
                        CreateStateMachineError::InvalidName(String::from(error_message))
                    }
                    "StateMachineAlreadyExists" => {
                        CreateStateMachineError::StateMachineAlreadyExists(String::from(
                            error_message,
                        ))
                    }
                    "StateMachineDeleting" => {
                        CreateStateMachineError::StateMachineDeleting(String::from(error_message))
                    }
                    "StateMachineLimitExceeded" => {
                        CreateStateMachineError::StateMachineLimitExceeded(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        CreateStateMachineError::Validation(error_message.to_string())
                    }
                    _ => CreateStateMachineError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateStateMachineError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateStateMachineError {
    fn from(err: serde_json::error::Error) -> CreateStateMachineError {
        CreateStateMachineError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateStateMachineError {
    fn from(err: CredentialsError) -> CreateStateMachineError {
        CreateStateMachineError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateStateMachineError {
    fn from(err: HttpDispatchError) -> CreateStateMachineError {
        CreateStateMachineError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateStateMachineError {
    fn from(err: io::Error) -> CreateStateMachineError {
        CreateStateMachineError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateStateMachineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateStateMachineError {
    fn description(&self) -> &str {
        match *self {
            CreateStateMachineError::InvalidArn(ref cause) => cause,
            CreateStateMachineError::InvalidDefinition(ref cause) => cause,
            CreateStateMachineError::InvalidName(ref cause) => cause,
            CreateStateMachineError::StateMachineAlreadyExists(ref cause) => cause,
            CreateStateMachineError::StateMachineDeleting(ref cause) => cause,
            CreateStateMachineError::StateMachineLimitExceeded(ref cause) => cause,
            CreateStateMachineError::Validation(ref cause) => cause,
            CreateStateMachineError::Credentials(ref err) => err.description(),
            CreateStateMachineError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateStateMachineError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteActivity
#[derive(Debug, PartialEq)]
pub enum DeleteActivityError {
    /// <p>The provided Amazon Resource Name (ARN) is invalid.</p>
    InvalidArn(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteActivityError {
    pub fn from_body(body: &str) -> DeleteActivityError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidArn" => DeleteActivityError::InvalidArn(String::from(error_message)),
                    "ValidationException" => {
                        DeleteActivityError::Validation(error_message.to_string())
                    }
                    _ => DeleteActivityError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteActivityError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteActivityError {
    fn from(err: serde_json::error::Error) -> DeleteActivityError {
        DeleteActivityError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteActivityError {
    fn from(err: CredentialsError) -> DeleteActivityError {
        DeleteActivityError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteActivityError {
    fn from(err: HttpDispatchError) -> DeleteActivityError {
        DeleteActivityError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteActivityError {
    fn from(err: io::Error) -> DeleteActivityError {
        DeleteActivityError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteActivityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteActivityError {
    fn description(&self) -> &str {
        match *self {
            DeleteActivityError::InvalidArn(ref cause) => cause,
            DeleteActivityError::Validation(ref cause) => cause,
            DeleteActivityError::Credentials(ref err) => err.description(),
            DeleteActivityError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteActivityError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteStateMachine
#[derive(Debug, PartialEq)]
pub enum DeleteStateMachineError {
    /// <p>The provided Amazon Resource Name (ARN) is invalid.</p>
    InvalidArn(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteStateMachineError {
    pub fn from_body(body: &str) -> DeleteStateMachineError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidArn" => {
                        DeleteStateMachineError::InvalidArn(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteStateMachineError::Validation(error_message.to_string())
                    }
                    _ => DeleteStateMachineError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteStateMachineError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteStateMachineError {
    fn from(err: serde_json::error::Error) -> DeleteStateMachineError {
        DeleteStateMachineError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteStateMachineError {
    fn from(err: CredentialsError) -> DeleteStateMachineError {
        DeleteStateMachineError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteStateMachineError {
    fn from(err: HttpDispatchError) -> DeleteStateMachineError {
        DeleteStateMachineError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteStateMachineError {
    fn from(err: io::Error) -> DeleteStateMachineError {
        DeleteStateMachineError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteStateMachineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteStateMachineError {
    fn description(&self) -> &str {
        match *self {
            DeleteStateMachineError::InvalidArn(ref cause) => cause,
            DeleteStateMachineError::Validation(ref cause) => cause,
            DeleteStateMachineError::Credentials(ref err) => err.description(),
            DeleteStateMachineError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteStateMachineError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeActivity
#[derive(Debug, PartialEq)]
pub enum DescribeActivityError {
    /// <p>The specified activity does not exist.</p>
    ActivityDoesNotExist(String),
    /// <p>The provided Amazon Resource Name (ARN) is invalid.</p>
    InvalidArn(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeActivityError {
    pub fn from_body(body: &str) -> DescribeActivityError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ActivityDoesNotExist" => {
                        DescribeActivityError::ActivityDoesNotExist(String::from(error_message))
                    }
                    "InvalidArn" => DescribeActivityError::InvalidArn(String::from(error_message)),
                    "ValidationException" => {
                        DescribeActivityError::Validation(error_message.to_string())
                    }
                    _ => DescribeActivityError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeActivityError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeActivityError {
    fn from(err: serde_json::error::Error) -> DescribeActivityError {
        DescribeActivityError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeActivityError {
    fn from(err: CredentialsError) -> DescribeActivityError {
        DescribeActivityError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeActivityError {
    fn from(err: HttpDispatchError) -> DescribeActivityError {
        DescribeActivityError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeActivityError {
    fn from(err: io::Error) -> DescribeActivityError {
        DescribeActivityError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeActivityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeActivityError {
    fn description(&self) -> &str {
        match *self {
            DescribeActivityError::ActivityDoesNotExist(ref cause) => cause,
            DescribeActivityError::InvalidArn(ref cause) => cause,
            DescribeActivityError::Validation(ref cause) => cause,
            DescribeActivityError::Credentials(ref err) => err.description(),
            DescribeActivityError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeActivityError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeExecution
#[derive(Debug, PartialEq)]
pub enum DescribeExecutionError {
    /// <p>The specified execution does not exist.</p>
    ExecutionDoesNotExist(String),
    /// <p>The provided Amazon Resource Name (ARN) is invalid.</p>
    InvalidArn(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeExecutionError {
    pub fn from_body(body: &str) -> DescribeExecutionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ExecutionDoesNotExist" => {
                        DescribeExecutionError::ExecutionDoesNotExist(String::from(error_message))
                    }
                    "InvalidArn" => DescribeExecutionError::InvalidArn(String::from(error_message)),
                    "ValidationException" => {
                        DescribeExecutionError::Validation(error_message.to_string())
                    }
                    _ => DescribeExecutionError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeExecutionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeExecutionError {
    fn from(err: serde_json::error::Error) -> DescribeExecutionError {
        DescribeExecutionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeExecutionError {
    fn from(err: CredentialsError) -> DescribeExecutionError {
        DescribeExecutionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeExecutionError {
    fn from(err: HttpDispatchError) -> DescribeExecutionError {
        DescribeExecutionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeExecutionError {
    fn from(err: io::Error) -> DescribeExecutionError {
        DescribeExecutionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeExecutionError {
    fn description(&self) -> &str {
        match *self {
            DescribeExecutionError::ExecutionDoesNotExist(ref cause) => cause,
            DescribeExecutionError::InvalidArn(ref cause) => cause,
            DescribeExecutionError::Validation(ref cause) => cause,
            DescribeExecutionError::Credentials(ref err) => err.description(),
            DescribeExecutionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeExecutionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeStateMachine
#[derive(Debug, PartialEq)]
pub enum DescribeStateMachineError {
    /// <p>The provided Amazon Resource Name (ARN) is invalid.</p>
    InvalidArn(String),
    /// <p>The specified state machine does not exist.</p>
    StateMachineDoesNotExist(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeStateMachineError {
    pub fn from_body(body: &str) -> DescribeStateMachineError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidArn" => {
                        DescribeStateMachineError::InvalidArn(String::from(error_message))
                    }
                    "StateMachineDoesNotExist" => {
                        DescribeStateMachineError::StateMachineDoesNotExist(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DescribeStateMachineError::Validation(error_message.to_string())
                    }
                    _ => DescribeStateMachineError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeStateMachineError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeStateMachineError {
    fn from(err: serde_json::error::Error) -> DescribeStateMachineError {
        DescribeStateMachineError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeStateMachineError {
    fn from(err: CredentialsError) -> DescribeStateMachineError {
        DescribeStateMachineError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeStateMachineError {
    fn from(err: HttpDispatchError) -> DescribeStateMachineError {
        DescribeStateMachineError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeStateMachineError {
    fn from(err: io::Error) -> DescribeStateMachineError {
        DescribeStateMachineError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeStateMachineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeStateMachineError {
    fn description(&self) -> &str {
        match *self {
            DescribeStateMachineError::InvalidArn(ref cause) => cause,
            DescribeStateMachineError::StateMachineDoesNotExist(ref cause) => cause,
            DescribeStateMachineError::Validation(ref cause) => cause,
            DescribeStateMachineError::Credentials(ref err) => err.description(),
            DescribeStateMachineError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeStateMachineError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetActivityTask
#[derive(Debug, PartialEq)]
pub enum GetActivityTaskError {
    /// <p>The specified activity does not exist.</p>
    ActivityDoesNotExist(String),
    /// <p>The maximum number of workers concurrently polling for activity tasks has been reached.</p>
    ActivityWorkerLimitExceeded(String),
    /// <p>The provided Amazon Resource Name (ARN) is invalid.</p>
    InvalidArn(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetActivityTaskError {
    pub fn from_body(body: &str) -> GetActivityTaskError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ActivityDoesNotExist" => {
                        GetActivityTaskError::ActivityDoesNotExist(String::from(error_message))
                    }
                    "ActivityWorkerLimitExceeded" => {
                        GetActivityTaskError::ActivityWorkerLimitExceeded(String::from(
                            error_message,
                        ))
                    }
                    "InvalidArn" => GetActivityTaskError::InvalidArn(String::from(error_message)),
                    "ValidationException" => {
                        GetActivityTaskError::Validation(error_message.to_string())
                    }
                    _ => GetActivityTaskError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetActivityTaskError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetActivityTaskError {
    fn from(err: serde_json::error::Error) -> GetActivityTaskError {
        GetActivityTaskError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetActivityTaskError {
    fn from(err: CredentialsError) -> GetActivityTaskError {
        GetActivityTaskError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetActivityTaskError {
    fn from(err: HttpDispatchError) -> GetActivityTaskError {
        GetActivityTaskError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetActivityTaskError {
    fn from(err: io::Error) -> GetActivityTaskError {
        GetActivityTaskError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetActivityTaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetActivityTaskError {
    fn description(&self) -> &str {
        match *self {
            GetActivityTaskError::ActivityDoesNotExist(ref cause) => cause,
            GetActivityTaskError::ActivityWorkerLimitExceeded(ref cause) => cause,
            GetActivityTaskError::InvalidArn(ref cause) => cause,
            GetActivityTaskError::Validation(ref cause) => cause,
            GetActivityTaskError::Credentials(ref err) => err.description(),
            GetActivityTaskError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetActivityTaskError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetExecutionHistory
#[derive(Debug, PartialEq)]
pub enum GetExecutionHistoryError {
    /// <p>The specified execution does not exist.</p>
    ExecutionDoesNotExist(String),
    /// <p>The provided Amazon Resource Name (ARN) is invalid.</p>
    InvalidArn(String),
    /// <p>The provided token is invalid.</p>
    InvalidToken(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetExecutionHistoryError {
    pub fn from_body(body: &str) -> GetExecutionHistoryError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ExecutionDoesNotExist" => {
                        GetExecutionHistoryError::ExecutionDoesNotExist(String::from(error_message))
                    }
                    "InvalidArn" => {
                        GetExecutionHistoryError::InvalidArn(String::from(error_message))
                    }
                    "InvalidToken" => {
                        GetExecutionHistoryError::InvalidToken(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetExecutionHistoryError::Validation(error_message.to_string())
                    }
                    _ => GetExecutionHistoryError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetExecutionHistoryError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetExecutionHistoryError {
    fn from(err: serde_json::error::Error) -> GetExecutionHistoryError {
        GetExecutionHistoryError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetExecutionHistoryError {
    fn from(err: CredentialsError) -> GetExecutionHistoryError {
        GetExecutionHistoryError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetExecutionHistoryError {
    fn from(err: HttpDispatchError) -> GetExecutionHistoryError {
        GetExecutionHistoryError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetExecutionHistoryError {
    fn from(err: io::Error) -> GetExecutionHistoryError {
        GetExecutionHistoryError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetExecutionHistoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetExecutionHistoryError {
    fn description(&self) -> &str {
        match *self {
            GetExecutionHistoryError::ExecutionDoesNotExist(ref cause) => cause,
            GetExecutionHistoryError::InvalidArn(ref cause) => cause,
            GetExecutionHistoryError::InvalidToken(ref cause) => cause,
            GetExecutionHistoryError::Validation(ref cause) => cause,
            GetExecutionHistoryError::Credentials(ref err) => err.description(),
            GetExecutionHistoryError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetExecutionHistoryError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListActivities
#[derive(Debug, PartialEq)]
pub enum ListActivitiesError {
    /// <p>The provided token is invalid.</p>
    InvalidToken(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListActivitiesError {
    pub fn from_body(body: &str) -> ListActivitiesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidToken" => {
                        ListActivitiesError::InvalidToken(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListActivitiesError::Validation(error_message.to_string())
                    }
                    _ => ListActivitiesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListActivitiesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListActivitiesError {
    fn from(err: serde_json::error::Error) -> ListActivitiesError {
        ListActivitiesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListActivitiesError {
    fn from(err: CredentialsError) -> ListActivitiesError {
        ListActivitiesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListActivitiesError {
    fn from(err: HttpDispatchError) -> ListActivitiesError {
        ListActivitiesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListActivitiesError {
    fn from(err: io::Error) -> ListActivitiesError {
        ListActivitiesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListActivitiesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListActivitiesError {
    fn description(&self) -> &str {
        match *self {
            ListActivitiesError::InvalidToken(ref cause) => cause,
            ListActivitiesError::Validation(ref cause) => cause,
            ListActivitiesError::Credentials(ref err) => err.description(),
            ListActivitiesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListActivitiesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListExecutions
#[derive(Debug, PartialEq)]
pub enum ListExecutionsError {
    /// <p>The provided Amazon Resource Name (ARN) is invalid.</p>
    InvalidArn(String),
    /// <p>The provided token is invalid.</p>
    InvalidToken(String),
    /// <p>The specified state machine does not exist.</p>
    StateMachineDoesNotExist(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListExecutionsError {
    pub fn from_body(body: &str) -> ListExecutionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidArn" => ListExecutionsError::InvalidArn(String::from(error_message)),
                    "InvalidToken" => {
                        ListExecutionsError::InvalidToken(String::from(error_message))
                    }
                    "StateMachineDoesNotExist" => {
                        ListExecutionsError::StateMachineDoesNotExist(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListExecutionsError::Validation(error_message.to_string())
                    }
                    _ => ListExecutionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListExecutionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListExecutionsError {
    fn from(err: serde_json::error::Error) -> ListExecutionsError {
        ListExecutionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListExecutionsError {
    fn from(err: CredentialsError) -> ListExecutionsError {
        ListExecutionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListExecutionsError {
    fn from(err: HttpDispatchError) -> ListExecutionsError {
        ListExecutionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListExecutionsError {
    fn from(err: io::Error) -> ListExecutionsError {
        ListExecutionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListExecutionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListExecutionsError {
    fn description(&self) -> &str {
        match *self {
            ListExecutionsError::InvalidArn(ref cause) => cause,
            ListExecutionsError::InvalidToken(ref cause) => cause,
            ListExecutionsError::StateMachineDoesNotExist(ref cause) => cause,
            ListExecutionsError::Validation(ref cause) => cause,
            ListExecutionsError::Credentials(ref err) => err.description(),
            ListExecutionsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListExecutionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListStateMachines
#[derive(Debug, PartialEq)]
pub enum ListStateMachinesError {
    /// <p>The provided token is invalid.</p>
    InvalidToken(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListStateMachinesError {
    pub fn from_body(body: &str) -> ListStateMachinesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidToken" => {
                        ListStateMachinesError::InvalidToken(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListStateMachinesError::Validation(error_message.to_string())
                    }
                    _ => ListStateMachinesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListStateMachinesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListStateMachinesError {
    fn from(err: serde_json::error::Error) -> ListStateMachinesError {
        ListStateMachinesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListStateMachinesError {
    fn from(err: CredentialsError) -> ListStateMachinesError {
        ListStateMachinesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListStateMachinesError {
    fn from(err: HttpDispatchError) -> ListStateMachinesError {
        ListStateMachinesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListStateMachinesError {
    fn from(err: io::Error) -> ListStateMachinesError {
        ListStateMachinesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListStateMachinesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListStateMachinesError {
    fn description(&self) -> &str {
        match *self {
            ListStateMachinesError::InvalidToken(ref cause) => cause,
            ListStateMachinesError::Validation(ref cause) => cause,
            ListStateMachinesError::Credentials(ref err) => err.description(),
            ListStateMachinesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListStateMachinesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SendTaskFailure
#[derive(Debug, PartialEq)]
pub enum SendTaskFailureError {
    /// <p>The provided token is invalid.</p>
    InvalidToken(String),

    TaskDoesNotExist(String),

    TaskTimedOut(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SendTaskFailureError {
    pub fn from_body(body: &str) -> SendTaskFailureError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidToken" => {
                        SendTaskFailureError::InvalidToken(String::from(error_message))
                    }
                    "TaskDoesNotExist" => {
                        SendTaskFailureError::TaskDoesNotExist(String::from(error_message))
                    }
                    "TaskTimedOut" => {
                        SendTaskFailureError::TaskTimedOut(String::from(error_message))
                    }
                    "ValidationException" => {
                        SendTaskFailureError::Validation(error_message.to_string())
                    }
                    _ => SendTaskFailureError::Unknown(String::from(body)),
                }
            }
            Err(_) => SendTaskFailureError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SendTaskFailureError {
    fn from(err: serde_json::error::Error) -> SendTaskFailureError {
        SendTaskFailureError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SendTaskFailureError {
    fn from(err: CredentialsError) -> SendTaskFailureError {
        SendTaskFailureError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SendTaskFailureError {
    fn from(err: HttpDispatchError) -> SendTaskFailureError {
        SendTaskFailureError::HttpDispatch(err)
    }
}
impl From<io::Error> for SendTaskFailureError {
    fn from(err: io::Error) -> SendTaskFailureError {
        SendTaskFailureError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SendTaskFailureError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SendTaskFailureError {
    fn description(&self) -> &str {
        match *self {
            SendTaskFailureError::InvalidToken(ref cause) => cause,
            SendTaskFailureError::TaskDoesNotExist(ref cause) => cause,
            SendTaskFailureError::TaskTimedOut(ref cause) => cause,
            SendTaskFailureError::Validation(ref cause) => cause,
            SendTaskFailureError::Credentials(ref err) => err.description(),
            SendTaskFailureError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            SendTaskFailureError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SendTaskHeartbeat
#[derive(Debug, PartialEq)]
pub enum SendTaskHeartbeatError {
    /// <p>The provided token is invalid.</p>
    InvalidToken(String),

    TaskDoesNotExist(String),

    TaskTimedOut(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SendTaskHeartbeatError {
    pub fn from_body(body: &str) -> SendTaskHeartbeatError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidToken" => {
                        SendTaskHeartbeatError::InvalidToken(String::from(error_message))
                    }
                    "TaskDoesNotExist" => {
                        SendTaskHeartbeatError::TaskDoesNotExist(String::from(error_message))
                    }
                    "TaskTimedOut" => {
                        SendTaskHeartbeatError::TaskTimedOut(String::from(error_message))
                    }
                    "ValidationException" => {
                        SendTaskHeartbeatError::Validation(error_message.to_string())
                    }
                    _ => SendTaskHeartbeatError::Unknown(String::from(body)),
                }
            }
            Err(_) => SendTaskHeartbeatError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SendTaskHeartbeatError {
    fn from(err: serde_json::error::Error) -> SendTaskHeartbeatError {
        SendTaskHeartbeatError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SendTaskHeartbeatError {
    fn from(err: CredentialsError) -> SendTaskHeartbeatError {
        SendTaskHeartbeatError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SendTaskHeartbeatError {
    fn from(err: HttpDispatchError) -> SendTaskHeartbeatError {
        SendTaskHeartbeatError::HttpDispatch(err)
    }
}
impl From<io::Error> for SendTaskHeartbeatError {
    fn from(err: io::Error) -> SendTaskHeartbeatError {
        SendTaskHeartbeatError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SendTaskHeartbeatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SendTaskHeartbeatError {
    fn description(&self) -> &str {
        match *self {
            SendTaskHeartbeatError::InvalidToken(ref cause) => cause,
            SendTaskHeartbeatError::TaskDoesNotExist(ref cause) => cause,
            SendTaskHeartbeatError::TaskTimedOut(ref cause) => cause,
            SendTaskHeartbeatError::Validation(ref cause) => cause,
            SendTaskHeartbeatError::Credentials(ref err) => err.description(),
            SendTaskHeartbeatError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SendTaskHeartbeatError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SendTaskSuccess
#[derive(Debug, PartialEq)]
pub enum SendTaskSuccessError {
    /// <p>The provided JSON output data is invalid.</p>
    InvalidOutput(String),
    /// <p>The provided token is invalid.</p>
    InvalidToken(String),

    TaskDoesNotExist(String),

    TaskTimedOut(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SendTaskSuccessError {
    pub fn from_body(body: &str) -> SendTaskSuccessError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidOutput" => {
                        SendTaskSuccessError::InvalidOutput(String::from(error_message))
                    }
                    "InvalidToken" => {
                        SendTaskSuccessError::InvalidToken(String::from(error_message))
                    }
                    "TaskDoesNotExist" => {
                        SendTaskSuccessError::TaskDoesNotExist(String::from(error_message))
                    }
                    "TaskTimedOut" => {
                        SendTaskSuccessError::TaskTimedOut(String::from(error_message))
                    }
                    "ValidationException" => {
                        SendTaskSuccessError::Validation(error_message.to_string())
                    }
                    _ => SendTaskSuccessError::Unknown(String::from(body)),
                }
            }
            Err(_) => SendTaskSuccessError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SendTaskSuccessError {
    fn from(err: serde_json::error::Error) -> SendTaskSuccessError {
        SendTaskSuccessError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SendTaskSuccessError {
    fn from(err: CredentialsError) -> SendTaskSuccessError {
        SendTaskSuccessError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SendTaskSuccessError {
    fn from(err: HttpDispatchError) -> SendTaskSuccessError {
        SendTaskSuccessError::HttpDispatch(err)
    }
}
impl From<io::Error> for SendTaskSuccessError {
    fn from(err: io::Error) -> SendTaskSuccessError {
        SendTaskSuccessError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SendTaskSuccessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SendTaskSuccessError {
    fn description(&self) -> &str {
        match *self {
            SendTaskSuccessError::InvalidOutput(ref cause) => cause,
            SendTaskSuccessError::InvalidToken(ref cause) => cause,
            SendTaskSuccessError::TaskDoesNotExist(ref cause) => cause,
            SendTaskSuccessError::TaskTimedOut(ref cause) => cause,
            SendTaskSuccessError::Validation(ref cause) => cause,
            SendTaskSuccessError::Credentials(ref err) => err.description(),
            SendTaskSuccessError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            SendTaskSuccessError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StartExecution
#[derive(Debug, PartialEq)]
pub enum StartExecutionError {
    /// <p>An execution with the same name already exists.</p>
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl StartExecutionError {
    pub fn from_body(body: &str) -> StartExecutionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ExecutionAlreadyExists" => {
                        StartExecutionError::ExecutionAlreadyExists(String::from(error_message))
                    }
                    "ExecutionLimitExceeded" => {
                        StartExecutionError::ExecutionLimitExceeded(String::from(error_message))
                    }
                    "InvalidArn" => StartExecutionError::InvalidArn(String::from(error_message)),
                    "InvalidExecutionInput" => {
                        StartExecutionError::InvalidExecutionInput(String::from(error_message))
                    }
                    "InvalidName" => StartExecutionError::InvalidName(String::from(error_message)),
                    "StateMachineDeleting" => {
                        StartExecutionError::StateMachineDeleting(String::from(error_message))
                    }
                    "StateMachineDoesNotExist" => {
                        StartExecutionError::StateMachineDoesNotExist(String::from(error_message))
                    }
                    "ValidationException" => {
                        StartExecutionError::Validation(error_message.to_string())
                    }
                    _ => StartExecutionError::Unknown(String::from(body)),
                }
            }
            Err(_) => StartExecutionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StartExecutionError {
    fn from(err: serde_json::error::Error) -> StartExecutionError {
        StartExecutionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StartExecutionError {
    fn from(err: CredentialsError) -> StartExecutionError {
        StartExecutionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartExecutionError {
    fn from(err: HttpDispatchError) -> StartExecutionError {
        StartExecutionError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartExecutionError {
    fn from(err: io::Error) -> StartExecutionError {
        StartExecutionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartExecutionError {
    fn description(&self) -> &str {
        match *self {
            StartExecutionError::ExecutionAlreadyExists(ref cause) => cause,
            StartExecutionError::ExecutionLimitExceeded(ref cause) => cause,
            StartExecutionError::InvalidArn(ref cause) => cause,
            StartExecutionError::InvalidExecutionInput(ref cause) => cause,
            StartExecutionError::InvalidName(ref cause) => cause,
            StartExecutionError::StateMachineDeleting(ref cause) => cause,
            StartExecutionError::StateMachineDoesNotExist(ref cause) => cause,
            StartExecutionError::Validation(ref cause) => cause,
            StartExecutionError::Credentials(ref err) => err.description(),
            StartExecutionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            StartExecutionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StopExecution
#[derive(Debug, PartialEq)]
pub enum StopExecutionError {
    /// <p>The specified execution does not exist.</p>
    ExecutionDoesNotExist(String),
    /// <p>The provided Amazon Resource Name (ARN) is invalid.</p>
    InvalidArn(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl StopExecutionError {
    pub fn from_body(body: &str) -> StopExecutionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ExecutionDoesNotExist" => {
                        StopExecutionError::ExecutionDoesNotExist(String::from(error_message))
                    }
                    "InvalidArn" => StopExecutionError::InvalidArn(String::from(error_message)),
                    "ValidationException" => {
                        StopExecutionError::Validation(error_message.to_string())
                    }
                    _ => StopExecutionError::Unknown(String::from(body)),
                }
            }
            Err(_) => StopExecutionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StopExecutionError {
    fn from(err: serde_json::error::Error) -> StopExecutionError {
        StopExecutionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StopExecutionError {
    fn from(err: CredentialsError) -> StopExecutionError {
        StopExecutionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopExecutionError {
    fn from(err: HttpDispatchError) -> StopExecutionError {
        StopExecutionError::HttpDispatch(err)
    }
}
impl From<io::Error> for StopExecutionError {
    fn from(err: io::Error) -> StopExecutionError {
        StopExecutionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StopExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopExecutionError {
    fn description(&self) -> &str {
        match *self {
            StopExecutionError::ExecutionDoesNotExist(ref cause) => cause,
            StopExecutionError::InvalidArn(ref cause) => cause,
            StopExecutionError::Validation(ref cause) => cause,
            StopExecutionError::Credentials(ref err) => err.description(),
            StopExecutionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            StopExecutionError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS SFN API. AWS SFN clients implement this trait.
pub trait StepFunctions {
    /// <p>Creates an activity.</p>
    fn create_activity(
        &self,
        input: &CreateActivityInput,
    ) -> Result<CreateActivityOutput, CreateActivityError>;

    /// <p>Creates a state machine.</p>
    fn create_state_machine(
        &self,
        input: &CreateStateMachineInput,
    ) -> Result<CreateStateMachineOutput, CreateStateMachineError>;

    /// <p>Deletes an activity.</p>
    fn delete_activity(
        &self,
        input: &DeleteActivityInput,
    ) -> Result<DeleteActivityOutput, DeleteActivityError>;

    /// <p>Deletes a state machine. This is an asynchronous operation-- it sets the state machine's status to "DELETING" and begins the delete process.</p>
    fn delete_state_machine(
        &self,
        input: &DeleteStateMachineInput,
    ) -> Result<DeleteStateMachineOutput, DeleteStateMachineError>;

    /// <p>Describes an activity.</p>
    fn describe_activity(
        &self,
        input: &DescribeActivityInput,
    ) -> Result<DescribeActivityOutput, DescribeActivityError>;

    /// <p>Describes an execution.</p>
    fn describe_execution(
        &self,
        input: &DescribeExecutionInput,
    ) -> Result<DescribeExecutionOutput, DescribeExecutionError>;

    /// <p>Describes a state machine.</p>
    fn describe_state_machine(
        &self,
        input: &DescribeStateMachineInput,
    ) -> Result<DescribeStateMachineOutput, DescribeStateMachineError>;

    /// <p><p>Used by workers to retrieve a task (with the specified activity ARN) scheduled for execution by a running state machine. This initiates a long poll, where the service holds the HTTP connection open and responds as soon as a task becomes available (i.e. an execution of a task of this type is needed.) The maximum time the service holds on to the request before responding is 60 seconds. If no task is available within 60 seconds, the poll will return an empty result, that is, the <code>taskToken</code> returned is an empty string.</p> <important> <p>Workers should set their client side socket timeout to at least 65 seconds (5 seconds higher than the maximum time the service may hold the poll request).</p> </important></p>
    fn get_activity_task(
        &self,
        input: &GetActivityTaskInput,
    ) -> Result<GetActivityTaskOutput, GetActivityTaskError>;

    /// <p>Returns the history of the specified execution as a list of events. By default, the results are returned in ascending order of the <code>timeStamp</code> of the events. Use the <code>reverseOrder</code> parameter to get the latest events first. The results may be split into multiple pages. To retrieve subsequent pages, make the call again using the <code>nextToken</code> returned by the previous call.</p>
    fn get_execution_history(
        &self,
        input: &GetExecutionHistoryInput,
    ) -> Result<GetExecutionHistoryOutput, GetExecutionHistoryError>;

    /// <p>Lists the existing activities. The results may be split into multiple pages. To retrieve subsequent pages, make the call again using the <code>nextToken</code> returned by the previous call.</p>
    fn list_activities(
        &self,
        input: &ListActivitiesInput,
    ) -> Result<ListActivitiesOutput, ListActivitiesError>;

    /// <p>Lists the executions of a state machine that meet the filtering criteria. The results may be split into multiple pages. To retrieve subsequent pages, make the call again using the <code>nextToken</code> returned by the previous call.</p>
    fn list_executions(
        &self,
        input: &ListExecutionsInput,
    ) -> Result<ListExecutionsOutput, ListExecutionsError>;

    /// <p>Lists the existing state machines. The results may be split into multiple pages. To retrieve subsequent pages, make the call again using the <code>nextToken</code> returned by the previous call.</p>
    fn list_state_machines(
        &self,
        input: &ListStateMachinesInput,
    ) -> Result<ListStateMachinesOutput, ListStateMachinesError>;

    /// <p>Used by workers to report that the task identified by the <code>taskToken</code> failed.</p>
    fn send_task_failure(
        &self,
        input: &SendTaskFailureInput,
    ) -> Result<SendTaskFailureOutput, SendTaskFailureError>;

    /// <p><p>Used by workers to report to the service that the task represented by the specified <code>taskToken</code> is still making progress. This action resets the <code>Heartbeat</code> clock. The <code>Heartbeat</code> threshold is specified in the state machine&#39;s Amazon States Language definition. This action does not in itself create an event in the execution history. However, if the task times out, the execution history will contain an <code>ActivityTimedOut</code> event.</p> <note> <p>The <code>Timeout</code> of a task, defined in the state machine&#39;s Amazon States Language definition, is its maximum allowed duration, regardless of the number of <a>SendTaskHeartbeat</a> requests received.</p> </note> <note> <p>This operation is only useful for long-lived tasks to report the liveliness of the task.</p> </note></p>
    fn send_task_heartbeat(
        &self,
        input: &SendTaskHeartbeatInput,
    ) -> Result<SendTaskHeartbeatOutput, SendTaskHeartbeatError>;

    /// <p>Used by workers to report that the task identified by the <code>taskToken</code> completed successfully.</p>
    fn send_task_success(
        &self,
        input: &SendTaskSuccessInput,
    ) -> Result<SendTaskSuccessOutput, SendTaskSuccessError>;

    /// <p>Starts a state machine execution.</p>
    fn start_execution(
        &self,
        input: &StartExecutionInput,
    ) -> Result<StartExecutionOutput, StartExecutionError>;

    /// <p>Stops an execution.</p>
    fn stop_execution(
        &self,
        input: &StopExecutionInput,
    ) -> Result<StopExecutionOutput, StopExecutionError>;
}
/// A client for the AWS SFN API.
pub struct StepFunctionsClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> StepFunctionsClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        StepFunctionsClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> StepFunctions for StepFunctionsClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    /// <p>Creates an activity.</p>
    fn create_activity(
        &self,
        input: &CreateActivityInput,
    ) -> Result<CreateActivityOutput, CreateActivityError> {
        let mut request = SignedRequest::new("POST", "states", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "AWSStepFunctions.CreateActivity");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<CreateActivityOutput>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateActivityError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Creates a state machine.</p>
    fn create_state_machine(
        &self,
        input: &CreateStateMachineInput,
    ) -> Result<CreateStateMachineOutput, CreateStateMachineError> {
        let mut request = SignedRequest::new("POST", "states", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "AWSStepFunctions.CreateStateMachine");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<CreateStateMachineOutput>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateStateMachineError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Deletes an activity.</p>
    fn delete_activity(
        &self,
        input: &DeleteActivityInput,
    ) -> Result<DeleteActivityOutput, DeleteActivityError> {
        let mut request = SignedRequest::new("POST", "states", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "AWSStepFunctions.DeleteActivity");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DeleteActivityOutput>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteActivityError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Deletes a state machine. This is an asynchronous operation-- it sets the state machine's status to "DELETING" and begins the delete process.</p>
    fn delete_state_machine(
        &self,
        input: &DeleteStateMachineInput,
    ) -> Result<DeleteStateMachineOutput, DeleteStateMachineError> {
        let mut request = SignedRequest::new("POST", "states", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "AWSStepFunctions.DeleteStateMachine");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DeleteStateMachineOutput>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteStateMachineError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Describes an activity.</p>
    fn describe_activity(
        &self,
        input: &DescribeActivityInput,
    ) -> Result<DescribeActivityOutput, DescribeActivityError> {
        let mut request = SignedRequest::new("POST", "states", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "AWSStepFunctions.DescribeActivity");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeActivityOutput>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeActivityError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Describes an execution.</p>
    fn describe_execution(
        &self,
        input: &DescribeExecutionInput,
    ) -> Result<DescribeExecutionOutput, DescribeExecutionError> {
        let mut request = SignedRequest::new("POST", "states", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "AWSStepFunctions.DescribeExecution");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeExecutionOutput>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeExecutionError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Describes a state machine.</p>
    fn describe_state_machine(
        &self,
        input: &DescribeStateMachineInput,
    ) -> Result<DescribeStateMachineOutput, DescribeStateMachineError> {
        let mut request = SignedRequest::new("POST", "states", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "AWSStepFunctions.DescribeStateMachine");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeStateMachineOutput>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeStateMachineError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p><p>Used by workers to retrieve a task (with the specified activity ARN) scheduled for execution by a running state machine. This initiates a long poll, where the service holds the HTTP connection open and responds as soon as a task becomes available (i.e. an execution of a task of this type is needed.) The maximum time the service holds on to the request before responding is 60 seconds. If no task is available within 60 seconds, the poll will return an empty result, that is, the <code>taskToken</code> returned is an empty string.</p> <important> <p>Workers should set their client side socket timeout to at least 65 seconds (5 seconds higher than the maximum time the service may hold the poll request).</p> </important></p>
    fn get_activity_task(
        &self,
        input: &GetActivityTaskInput,
    ) -> Result<GetActivityTaskOutput, GetActivityTaskError> {
        let mut request = SignedRequest::new("POST", "states", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "AWSStepFunctions.GetActivityTask");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetActivityTaskOutput>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetActivityTaskError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Returns the history of the specified execution as a list of events. By default, the results are returned in ascending order of the <code>timeStamp</code> of the events. Use the <code>reverseOrder</code> parameter to get the latest events first. The results may be split into multiple pages. To retrieve subsequent pages, make the call again using the <code>nextToken</code> returned by the previous call.</p>
    fn get_execution_history(
        &self,
        input: &GetExecutionHistoryInput,
    ) -> Result<GetExecutionHistoryOutput, GetExecutionHistoryError> {
        let mut request = SignedRequest::new("POST", "states", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "AWSStepFunctions.GetExecutionHistory");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetExecutionHistoryOutput>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetExecutionHistoryError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Lists the existing activities. The results may be split into multiple pages. To retrieve subsequent pages, make the call again using the <code>nextToken</code> returned by the previous call.</p>
    fn list_activities(
        &self,
        input: &ListActivitiesInput,
    ) -> Result<ListActivitiesOutput, ListActivitiesError> {
        let mut request = SignedRequest::new("POST", "states", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "AWSStepFunctions.ListActivities");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<ListActivitiesOutput>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListActivitiesError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Lists the executions of a state machine that meet the filtering criteria. The results may be split into multiple pages. To retrieve subsequent pages, make the call again using the <code>nextToken</code> returned by the previous call.</p>
    fn list_executions(
        &self,
        input: &ListExecutionsInput,
    ) -> Result<ListExecutionsOutput, ListExecutionsError> {
        let mut request = SignedRequest::new("POST", "states", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "AWSStepFunctions.ListExecutions");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<ListExecutionsOutput>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListExecutionsError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Lists the existing state machines. The results may be split into multiple pages. To retrieve subsequent pages, make the call again using the <code>nextToken</code> returned by the previous call.</p>
    fn list_state_machines(
        &self,
        input: &ListStateMachinesInput,
    ) -> Result<ListStateMachinesOutput, ListStateMachinesError> {
        let mut request = SignedRequest::new("POST", "states", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "AWSStepFunctions.ListStateMachines");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<ListStateMachinesOutput>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListStateMachinesError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Used by workers to report that the task identified by the <code>taskToken</code> failed.</p>
    fn send_task_failure(
        &self,
        input: &SendTaskFailureInput,
    ) -> Result<SendTaskFailureOutput, SendTaskFailureError> {
        let mut request = SignedRequest::new("POST", "states", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "AWSStepFunctions.SendTaskFailure");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<SendTaskFailureOutput>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(SendTaskFailureError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p><p>Used by workers to report to the service that the task represented by the specified <code>taskToken</code> is still making progress. This action resets the <code>Heartbeat</code> clock. The <code>Heartbeat</code> threshold is specified in the state machine&#39;s Amazon States Language definition. This action does not in itself create an event in the execution history. However, if the task times out, the execution history will contain an <code>ActivityTimedOut</code> event.</p> <note> <p>The <code>Timeout</code> of a task, defined in the state machine&#39;s Amazon States Language definition, is its maximum allowed duration, regardless of the number of <a>SendTaskHeartbeat</a> requests received.</p> </note> <note> <p>This operation is only useful for long-lived tasks to report the liveliness of the task.</p> </note></p>
    fn send_task_heartbeat(
        &self,
        input: &SendTaskHeartbeatInput,
    ) -> Result<SendTaskHeartbeatOutput, SendTaskHeartbeatError> {
        let mut request = SignedRequest::new("POST", "states", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "AWSStepFunctions.SendTaskHeartbeat");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<SendTaskHeartbeatOutput>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(SendTaskHeartbeatError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Used by workers to report that the task identified by the <code>taskToken</code> completed successfully.</p>
    fn send_task_success(
        &self,
        input: &SendTaskSuccessInput,
    ) -> Result<SendTaskSuccessOutput, SendTaskSuccessError> {
        let mut request = SignedRequest::new("POST", "states", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "AWSStepFunctions.SendTaskSuccess");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<SendTaskSuccessOutput>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(SendTaskSuccessError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Starts a state machine execution.</p>
    fn start_execution(
        &self,
        input: &StartExecutionInput,
    ) -> Result<StartExecutionOutput, StartExecutionError> {
        let mut request = SignedRequest::new("POST", "states", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "AWSStepFunctions.StartExecution");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<StartExecutionOutput>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(StartExecutionError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Stops an execution.</p>
    fn stop_execution(
        &self,
        input: &StopExecutionInput,
    ) -> Result<StopExecutionOutput, StopExecutionError> {
        let mut request = SignedRequest::new("POST", "states", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "AWSStepFunctions.StopExecution");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<StopExecutionOutput>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(StopExecutionError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }
}

#[cfg(test)]
mod protocol_tests {}
