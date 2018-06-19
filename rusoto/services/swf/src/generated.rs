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
/// <p>Unit of work sent to an activity worker.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ActivityTask {
    /// <p>The unique ID of the task.</p>
    #[serde(rename = "activityId")]
    pub activity_id: String,
    /// <p>The type of this activity task.</p>
    #[serde(rename = "activityType")]
    pub activity_type: ActivityType,
    /// <p>The inputs provided when the activity task was scheduled. The form of the input is user defined and should be meaningful to the activity implementation.</p>
    #[serde(rename = "input")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    /// <p>The ID of the <code>ActivityTaskStarted</code> event recorded in the history.</p>
    #[serde(rename = "startedEventId")]
    pub started_event_id: i64,
    /// <p>The opaque string used as a handle on the task. This token is used by workers to communicate progress and response information back to the system about the task.</p>
    #[serde(rename = "taskToken")]
    pub task_token: String,
    /// <p>The workflow execution that started this activity task.</p>
    #[serde(rename = "workflowExecution")]
    pub workflow_execution: WorkflowExecution,
}

/// <p>Provides the details of the <code>ActivityTaskCancelRequested</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ActivityTaskCancelRequestedEventAttributes {
    /// <p>The unique ID of the task.</p>
    #[serde(rename = "activityId")]
    pub activity_id: String,
    /// <p>The ID of the <code>DecisionTaskCompleted</code> event corresponding to the decision task that resulted in the <code>RequestCancelActivityTask</code> decision for this cancellation request. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "decisionTaskCompletedEventId")]
    pub decision_task_completed_event_id: i64,
}

/// <p>Provides the details of the <code>ActivityTaskCanceled</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ActivityTaskCanceledEventAttributes {
    /// <p>Details of the cancellation.</p>
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// <p>If set, contains the ID of the last <code>ActivityTaskCancelRequested</code> event recorded for this activity task. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "latestCancelRequestedEventId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_cancel_requested_event_id: Option<i64>,
    /// <p>The ID of the <code>ActivityTaskScheduled</code> event that was recorded when this activity task was scheduled. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "scheduledEventId")]
    pub scheduled_event_id: i64,
    /// <p>The ID of the <code>ActivityTaskStarted</code> event recorded when this activity task was started. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "startedEventId")]
    pub started_event_id: i64,
}

/// <p>Provides the details of the <code>ActivityTaskCompleted</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ActivityTaskCompletedEventAttributes {
    /// <p>The results of the activity task.</p>
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    /// <p>The ID of the <code>ActivityTaskScheduled</code> event that was recorded when this activity task was scheduled. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "scheduledEventId")]
    pub scheduled_event_id: i64,
    /// <p>The ID of the <code>ActivityTaskStarted</code> event recorded when this activity task was started. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "startedEventId")]
    pub started_event_id: i64,
}

/// <p>Provides the details of the <code>ActivityTaskFailed</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ActivityTaskFailedEventAttributes {
    /// <p>The details of the failure.</p>
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// <p>The reason provided for the failure.</p>
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// <p>The ID of the <code>ActivityTaskScheduled</code> event that was recorded when this activity task was scheduled. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "scheduledEventId")]
    pub scheduled_event_id: i64,
    /// <p>The ID of the <code>ActivityTaskStarted</code> event recorded when this activity task was started. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "startedEventId")]
    pub started_event_id: i64,
}

/// <p>Provides the details of the <code>ActivityTaskScheduled</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ActivityTaskScheduledEventAttributes {
    /// <p>The unique ID of the activity task.</p>
    #[serde(rename = "activityId")]
    pub activity_id: String,
    /// <p>The type of the activity task.</p>
    #[serde(rename = "activityType")]
    pub activity_type: ActivityType,
    /// <p>Data attached to the event that can be used by the decider in subsequent workflow tasks. This data isn't sent to the activity.</p>
    #[serde(rename = "control")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<String>,
    /// <p>The ID of the <code>DecisionTaskCompleted</code> event corresponding to the decision that resulted in the scheduling of this activity task. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "decisionTaskCompletedEventId")]
    pub decision_task_completed_event_id: i64,
    /// <p>The maximum time before which the worker processing this task must report progress by calling <a>RecordActivityTaskHeartbeat</a>. If the timeout is exceeded, the activity task is automatically timed out. If the worker subsequently attempts to record a heartbeat or return a result, it is ignored.</p>
    #[serde(rename = "heartbeatTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heartbeat_timeout: Option<String>,
    /// <p>The input provided to the activity task.</p>
    #[serde(rename = "input")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    /// <p>The maximum amount of time for this activity task.</p>
    #[serde(rename = "scheduleToCloseTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_to_close_timeout: Option<String>,
    /// <p>The maximum amount of time the activity task can wait to be assigned to a worker.</p>
    #[serde(rename = "scheduleToStartTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_to_start_timeout: Option<String>,
    /// <p>The maximum amount of time a worker may take to process the activity task.</p>
    #[serde(rename = "startToCloseTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_to_close_timeout: Option<String>,
    /// <p>The task list in which the activity task has been scheduled.</p>
    #[serde(rename = "taskList")]
    pub task_list: TaskList,
    /// <p> The priority to assign to the scheduled activity task. If set, this overrides any default priority value that was assigned when the activity type was registered.</p> <p>Valid values are integers that range from Java's <code>Integer.MIN_VALUE</code> (-2147483648) to <code>Integer.MAX_VALUE</code> (2147483647). Higher numbers indicate higher priority.</p> <p>For more information about setting task priority, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/programming-priority.html">Setting Task Priority</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    #[serde(rename = "taskPriority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_priority: Option<String>,
}

/// <p>Provides the details of the <code>ActivityTaskStarted</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ActivityTaskStartedEventAttributes {
    /// <p>Identity of the worker that was assigned this task. This aids diagnostics when problems arise. The form of this identity is user defined.</p>
    #[serde(rename = "identity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    /// <p>The ID of the <code>ActivityTaskScheduled</code> event that was recorded when this activity task was scheduled. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "scheduledEventId")]
    pub scheduled_event_id: i64,
}

/// <p>Status information about an activity task.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ActivityTaskStatus {
    /// <p>Set to <code>true</code> if cancellation of the task is requested.</p>
    #[serde(rename = "cancelRequested")]
    pub cancel_requested: bool,
}

/// <p>Provides the details of the <code>ActivityTaskTimedOut</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ActivityTaskTimedOutEventAttributes {
    /// <p>Contains the content of the <code>details</code> parameter for the last call made by the activity to <code>RecordActivityTaskHeartbeat</code>.</p>
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// <p>The ID of the <code>ActivityTaskScheduled</code> event that was recorded when this activity task was scheduled. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "scheduledEventId")]
    pub scheduled_event_id: i64,
    /// <p>The ID of the <code>ActivityTaskStarted</code> event recorded when this activity task was started. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "startedEventId")]
    pub started_event_id: i64,
    /// <p>The type of the timeout that caused this event.</p>
    #[serde(rename = "timeoutType")]
    pub timeout_type: String,
}

/// <p>Represents an activity type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActivityType {
    /// <p><p>The name of this activity.</p> <note> <p>The combination of activity type name and version must be unique within a domain.</p> </note></p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p><p>The version of this activity.</p> <note> <p>The combination of activity type name and version must be unique with in a domain.</p> </note></p>
    #[serde(rename = "version")]
    pub version: String,
}

/// <p>Configuration settings registered with the activity type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ActivityTypeConfiguration {
    /// <p> The default maximum time, in seconds, before which a worker processing a task must report progress by calling <a>RecordActivityTaskHeartbeat</a>.</p> <p>You can specify this value only when <i>registering</i> an activity type. The registered default value can be overridden when you schedule a task through the <code>ScheduleActivityTask</code> <a>Decision</a>. If the activity worker subsequently attempts to record a heartbeat or returns a result, the activity worker receives an <code>UnknownResource</code> fault. In this case, Amazon SWF no longer considers the activity task to be valid; the activity worker should clean up the activity task.</p> <p>The duration is specified in seconds, an integer greater than or equal to <code>0</code>. You can use <code>NONE</code> to specify unlimited duration.</p>
    #[serde(rename = "defaultTaskHeartbeatTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_task_heartbeat_timeout: Option<String>,
    /// <p> The default task list specified for this activity type at registration. This default is used if a task list isn't provided when a task is scheduled through the <code>ScheduleActivityTask</code> <a>Decision</a>. You can override the default registered task list when scheduling a task through the <code>ScheduleActivityTask</code> <a>Decision</a>.</p>
    #[serde(rename = "defaultTaskList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_task_list: Option<TaskList>,
    /// <p> The default task priority for tasks of this activity type, specified at registration. If not set, then <code>0</code> is used as the default priority. This default can be overridden when scheduling an activity task.</p> <p>Valid values are integers that range from Java's <code>Integer.MIN_VALUE</code> (-2147483648) to <code>Integer.MAX_VALUE</code> (2147483647). Higher numbers indicate higher priority.</p> <p>For more information about setting task priority, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/programming-priority.html">Setting Task Priority</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    #[serde(rename = "defaultTaskPriority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_task_priority: Option<String>,
    /// <p> The default maximum duration, specified when registering the activity type, for tasks of this activity type. You can override this default when scheduling a task through the <code>ScheduleActivityTask</code> <a>Decision</a>.</p> <p>The duration is specified in seconds, an integer greater than or equal to <code>0</code>. You can use <code>NONE</code> to specify unlimited duration.</p>
    #[serde(rename = "defaultTaskScheduleToCloseTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_task_schedule_to_close_timeout: Option<String>,
    /// <p> The default maximum duration, specified when registering the activity type, that a task of an activity type can wait before being assigned to a worker. You can override this default when scheduling a task through the <code>ScheduleActivityTask</code> <a>Decision</a>.</p> <p>The duration is specified in seconds, an integer greater than or equal to <code>0</code>. You can use <code>NONE</code> to specify unlimited duration.</p>
    #[serde(rename = "defaultTaskScheduleToStartTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_task_schedule_to_start_timeout: Option<String>,
    /// <p> The default maximum duration for tasks of an activity type specified when registering the activity type. You can override this default when scheduling a task through the <code>ScheduleActivityTask</code> <a>Decision</a>.</p> <p>The duration is specified in seconds, an integer greater than or equal to <code>0</code>. You can use <code>NONE</code> to specify unlimited duration.</p>
    #[serde(rename = "defaultTaskStartToCloseTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_task_start_to_close_timeout: Option<String>,
}

/// <p>Detailed information about an activity type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ActivityTypeDetail {
    /// <p>The configuration settings registered with the activity type.</p>
    #[serde(rename = "configuration")]
    pub configuration: ActivityTypeConfiguration,
    /// <p><p>General information about the activity type.</p> <p>The status of activity type (returned in the ActivityTypeInfo structure) can be one of the following.</p> <ul> <li> <p> <code>REGISTERED</code> – The type is registered and available. Workers supporting this type should be running. </p> </li> <li> <p> <code>DEPRECATED</code> – The type was deprecated using <a>DeprecateActivityType</a>, but is still in use. You should keep workers supporting this type running. You cannot create new tasks of this type. </p> </li> </ul></p>
    #[serde(rename = "typeInfo")]
    pub type_info: ActivityTypeInfo,
}

/// <p>Detailed information about an activity type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ActivityTypeInfo {
    /// <p>The <a>ActivityType</a> type structure representing the activity type.</p>
    #[serde(rename = "activityType")]
    pub activity_type: ActivityType,
    /// <p>The date and time this activity type was created through <a>RegisterActivityType</a>.</p>
    #[serde(rename = "creationDate")]
    pub creation_date: f64,
    /// <p>If DEPRECATED, the date and time <a>DeprecateActivityType</a> was called.</p>
    #[serde(rename = "deprecationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_date: Option<f64>,
    /// <p>The description of the activity type provided in <a>RegisterActivityType</a>.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The current status of the activity type.</p>
    #[serde(rename = "status")]
    pub status: String,
}

/// <p>Contains a paginated list of activity type information structures.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ActivityTypeInfos {
    /// <p>If a <code>NextPageToken</code> was returned by a previous call, there are more results available. To retrieve the next page of results, make the call again using the returned token in <code>nextPageToken</code>. Keep all other arguments unchanged.</p> <p>The configured <code>maximumPageSize</code> determines how many results can be returned in a single call.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>List of activity type information.</p>
    #[serde(rename = "typeInfos")]
    pub type_infos: Vec<ActivityTypeInfo>,
}

/// <p>Provides the details of the <code>CancelTimer</code> decision.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this decision's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CancelTimerDecisionAttributes {
    /// <p> The unique ID of the timer to cancel.</p>
    #[serde(rename = "timerId")]
    pub timer_id: String,
}

/// <p>Provides the details of the <code>CancelTimerFailed</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CancelTimerFailedEventAttributes {
    /// <p><p>The cause of the failure. This information is generated by the system and can be useful for diagnostic purposes.</p> <note> <p>If <code>cause</code> is set to <code>OPERATION<em>NOT</em>PERMITTED</code>, the decision failed because it lacked sufficient permissions. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p> </note></p>
    #[serde(rename = "cause")]
    pub cause: String,
    /// <p>The ID of the <code>DecisionTaskCompleted</code> event corresponding to the decision task that resulted in the <code>CancelTimer</code> decision to cancel this timer. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "decisionTaskCompletedEventId")]
    pub decision_task_completed_event_id: i64,
    /// <p>The timerId provided in the <code>CancelTimer</code> decision that failed.</p>
    #[serde(rename = "timerId")]
    pub timer_id: String,
}

/// <p>Provides the details of the <code>CancelWorkflowExecution</code> decision.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this decision's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CancelWorkflowExecutionDecisionAttributes {
    /// <p> Details of the cancellation.</p>
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

/// <p>Provides the details of the <code>CancelWorkflowExecutionFailed</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CancelWorkflowExecutionFailedEventAttributes {
    /// <p><p>The cause of the failure. This information is generated by the system and can be useful for diagnostic purposes.</p> <note> <p>If <code>cause</code> is set to <code>OPERATION<em>NOT</em>PERMITTED</code>, the decision failed because it lacked sufficient permissions. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p> </note></p>
    #[serde(rename = "cause")]
    pub cause: String,
    /// <p>The ID of the <code>DecisionTaskCompleted</code> event corresponding to the decision task that resulted in the <code>CancelWorkflowExecution</code> decision for this cancellation request. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "decisionTaskCompletedEventId")]
    pub decision_task_completed_event_id: i64,
}

/// <p>Provide details of the <code>ChildWorkflowExecutionCanceled</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ChildWorkflowExecutionCanceledEventAttributes {
    /// <p>Details of the cancellation (if provided).</p>
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// <p>The ID of the <code>StartChildWorkflowExecutionInitiated</code> event corresponding to the <code>StartChildWorkflowExecution</code> <a>Decision</a> to start this child workflow execution. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "initiatedEventId")]
    pub initiated_event_id: i64,
    /// <p>The ID of the <code>ChildWorkflowExecutionStarted</code> event recorded when this child workflow execution was started. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "startedEventId")]
    pub started_event_id: i64,
    /// <p>The child workflow execution that was canceled.</p>
    #[serde(rename = "workflowExecution")]
    pub workflow_execution: WorkflowExecution,
    /// <p>The type of the child workflow execution.</p>
    #[serde(rename = "workflowType")]
    pub workflow_type: WorkflowType,
}

/// <p>Provides the details of the <code>ChildWorkflowExecutionCompleted</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ChildWorkflowExecutionCompletedEventAttributes {
    /// <p>The ID of the <code>StartChildWorkflowExecutionInitiated</code> event corresponding to the <code>StartChildWorkflowExecution</code> <a>Decision</a> to start this child workflow execution. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "initiatedEventId")]
    pub initiated_event_id: i64,
    /// <p>The result of the child workflow execution.</p>
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    /// <p>The ID of the <code>ChildWorkflowExecutionStarted</code> event recorded when this child workflow execution was started. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "startedEventId")]
    pub started_event_id: i64,
    /// <p>The child workflow execution that was completed.</p>
    #[serde(rename = "workflowExecution")]
    pub workflow_execution: WorkflowExecution,
    /// <p>The type of the child workflow execution.</p>
    #[serde(rename = "workflowType")]
    pub workflow_type: WorkflowType,
}

/// <p>Provides the details of the <code>ChildWorkflowExecutionFailed</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ChildWorkflowExecutionFailedEventAttributes {
    /// <p>The details of the failure (if provided).</p>
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// <p>The ID of the <code>StartChildWorkflowExecutionInitiated</code> event corresponding to the <code>StartChildWorkflowExecution</code> <a>Decision</a> to start this child workflow execution. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "initiatedEventId")]
    pub initiated_event_id: i64,
    /// <p>The reason for the failure (if provided).</p>
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// <p>The ID of the <code>ChildWorkflowExecutionStarted</code> event recorded when this child workflow execution was started. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "startedEventId")]
    pub started_event_id: i64,
    /// <p>The child workflow execution that failed.</p>
    #[serde(rename = "workflowExecution")]
    pub workflow_execution: WorkflowExecution,
    /// <p>The type of the child workflow execution.</p>
    #[serde(rename = "workflowType")]
    pub workflow_type: WorkflowType,
}

/// <p>Provides the details of the <code>ChildWorkflowExecutionStarted</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ChildWorkflowExecutionStartedEventAttributes {
    /// <p>The ID of the <code>StartChildWorkflowExecutionInitiated</code> event corresponding to the <code>StartChildWorkflowExecution</code> <a>Decision</a> to start this child workflow execution. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "initiatedEventId")]
    pub initiated_event_id: i64,
    /// <p>The child workflow execution that was started.</p>
    #[serde(rename = "workflowExecution")]
    pub workflow_execution: WorkflowExecution,
    /// <p>The type of the child workflow execution.</p>
    #[serde(rename = "workflowType")]
    pub workflow_type: WorkflowType,
}

/// <p>Provides the details of the <code>ChildWorkflowExecutionTerminated</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ChildWorkflowExecutionTerminatedEventAttributes {
    /// <p>The ID of the <code>StartChildWorkflowExecutionInitiated</code> event corresponding to the <code>StartChildWorkflowExecution</code> <a>Decision</a> to start this child workflow execution. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "initiatedEventId")]
    pub initiated_event_id: i64,
    /// <p>The ID of the <code>ChildWorkflowExecutionStarted</code> event recorded when this child workflow execution was started. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "startedEventId")]
    pub started_event_id: i64,
    /// <p>The child workflow execution that was terminated.</p>
    #[serde(rename = "workflowExecution")]
    pub workflow_execution: WorkflowExecution,
    /// <p>The type of the child workflow execution.</p>
    #[serde(rename = "workflowType")]
    pub workflow_type: WorkflowType,
}

/// <p>Provides the details of the <code>ChildWorkflowExecutionTimedOut</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ChildWorkflowExecutionTimedOutEventAttributes {
    /// <p>The ID of the <code>StartChildWorkflowExecutionInitiated</code> event corresponding to the <code>StartChildWorkflowExecution</code> <a>Decision</a> to start this child workflow execution. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "initiatedEventId")]
    pub initiated_event_id: i64,
    /// <p>The ID of the <code>ChildWorkflowExecutionStarted</code> event recorded when this child workflow execution was started. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "startedEventId")]
    pub started_event_id: i64,
    /// <p>The type of the timeout that caused the child workflow execution to time out.</p>
    #[serde(rename = "timeoutType")]
    pub timeout_type: String,
    /// <p>The child workflow execution that timed out.</p>
    #[serde(rename = "workflowExecution")]
    pub workflow_execution: WorkflowExecution,
    /// <p>The type of the child workflow execution.</p>
    #[serde(rename = "workflowType")]
    pub workflow_type: WorkflowType,
}

/// <p>Used to filter the closed workflow executions in visibility APIs by their close status.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CloseStatusFilter {
    /// <p> The close status that must match the close status of an execution for it to meet the criteria of this filter.</p>
    #[serde(rename = "status")]
    pub status: String,
}

/// <p>Provides the details of the <code>CompleteWorkflowExecution</code> decision.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this decision's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CompleteWorkflowExecutionDecisionAttributes {
    /// <p>The result of the workflow execution. The form of the result is implementation defined.</p>
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}

/// <p>Provides the details of the <code>CompleteWorkflowExecutionFailed</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CompleteWorkflowExecutionFailedEventAttributes {
    /// <p><p>The cause of the failure. This information is generated by the system and can be useful for diagnostic purposes.</p> <note> <p>If <code>cause</code> is set to <code>OPERATION<em>NOT</em>PERMITTED</code>, the decision failed because it lacked sufficient permissions. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p> </note></p>
    #[serde(rename = "cause")]
    pub cause: String,
    /// <p>The ID of the <code>DecisionTaskCompleted</code> event corresponding to the decision task that resulted in the <code>CompleteWorkflowExecution</code> decision to complete this execution. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "decisionTaskCompletedEventId")]
    pub decision_task_completed_event_id: i64,
}

/// <p>Provides the details of the <code>ContinueAsNewWorkflowExecution</code> decision.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this decision's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>tag</code> – A tag used to identify the workflow execution</p> </li> <li> <p> <code>taskList</code> – String constraint. The key is <code>swf:taskList.name</code>.</p> </li> <li> <p> <code>workflowType.version</code> – String constraint. The key is <code>swf:workflowType.version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ContinueAsNewWorkflowExecutionDecisionAttributes {
    /// <p><p>If set, specifies the policy to use for the child workflow executions of the new execution if it is terminated by calling the <a>TerminateWorkflowExecution</a> action explicitly or due to an expired timeout. This policy overrides the default child policy specified when registering the workflow type using <a>RegisterWorkflowType</a>.</p> <p>The supported child policies are:</p> <ul> <li> <p> <code>TERMINATE</code> – The child executions are terminated.</p> </li> <li> <p> <code>REQUEST_CANCEL</code> – A request to cancel is attempted for each child execution by recording a <code>WorkflowExecutionCancelRequested</code> event in its history. It is up to the decider to take appropriate actions when it receives an execution history with this event.</p> </li> <li> <p> <code>ABANDON</code> – No action is taken. The child executions continue to run.</p> </li> </ul> <note> <p>A child policy for this workflow execution must be specified either as a default for the workflow type or through this parameter. If neither this parameter is set nor a default child policy was specified at registration time then a fault is returned.</p> </note></p>
    #[serde(rename = "childPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_policy: Option<String>,
    /// <p><p>If set, specifies the total duration for this workflow execution. This overrides the <code>defaultExecutionStartToCloseTimeout</code> specified when registering the workflow type.</p> <p>The duration is specified in seconds, an integer greater than or equal to <code>0</code>. You can use <code>NONE</code> to specify unlimited duration.</p> <note> <p>An execution start-to-close timeout for this workflow execution must be specified either as a default for the workflow type or through this field. If neither this field is set nor a default execution start-to-close timeout was specified at registration time then a fault is returned.</p> </note></p>
    #[serde(rename = "executionStartToCloseTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_start_to_close_timeout: Option<String>,
    /// <p>The input provided to the new workflow execution.</p>
    #[serde(rename = "input")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    /// <p>The IAM role to attach to the new (continued) execution.</p>
    #[serde(rename = "lambdaRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_role: Option<String>,
    /// <p>The list of tags to associate with the new workflow execution. A maximum of 5 tags can be specified. You can list workflow executions with a specific tag by calling <a>ListOpenWorkflowExecutions</a> or <a>ListClosedWorkflowExecutions</a> and specifying a <a>TagFilter</a>.</p>
    #[serde(rename = "tagList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<String>>,
    /// <p>The task list to use for the decisions of the new (continued) workflow execution.</p>
    #[serde(rename = "taskList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_list: Option<TaskList>,
    /// <p> The task priority that, if set, specifies the priority for the decision tasks for this workflow execution. This overrides the defaultTaskPriority specified when registering the workflow type. Valid values are integers that range from Java's <code>Integer.MIN_VALUE</code> (-2147483648) to <code>Integer.MAX_VALUE</code> (2147483647). Higher numbers indicate higher priority.</p> <p>For more information about setting task priority, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/programming-priority.html">Setting Task Priority</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    #[serde(rename = "taskPriority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_priority: Option<String>,
    /// <p><p>Specifies the maximum duration of decision tasks for the new workflow execution. This parameter overrides the <code>defaultTaskStartToCloseTimout</code> specified when registering the workflow type using <a>RegisterWorkflowType</a>.</p> <p>The duration is specified in seconds, an integer greater than or equal to <code>0</code>. You can use <code>NONE</code> to specify unlimited duration.</p> <note> <p>A task start-to-close timeout for the new workflow execution must be specified either as a default for the workflow type or through this parameter. If neither this parameter is set nor a default task start-to-close timeout was specified at registration time then a fault is returned.</p> </note></p>
    #[serde(rename = "taskStartToCloseTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_start_to_close_timeout: Option<String>,
    /// <p>The version of the workflow to start.</p>
    #[serde(rename = "workflowTypeVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_type_version: Option<String>,
}

/// <p>Provides the details of the <code>ContinueAsNewWorkflowExecutionFailed</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ContinueAsNewWorkflowExecutionFailedEventAttributes {
    /// <p><p>The cause of the failure. This information is generated by the system and can be useful for diagnostic purposes.</p> <note> <p>If <code>cause</code> is set to <code>OPERATION<em>NOT</em>PERMITTED</code>, the decision failed because it lacked sufficient permissions. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p> </note></p>
    #[serde(rename = "cause")]
    pub cause: String,
    /// <p>The ID of the <code>DecisionTaskCompleted</code> event corresponding to the decision task that resulted in the <code>ContinueAsNewWorkflowExecution</code> decision that started this execution. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "decisionTaskCompletedEventId")]
    pub decision_task_completed_event_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CountClosedWorkflowExecutionsInput {
    /// <p><p>If specified, only workflow executions that match this close status are counted. This filter has an affect only if <code>executionStatus</code> is specified as <code>CLOSED</code>.</p> <note> <p> <code>closeStatusFilter</code>, <code>executionFilter</code>, <code>typeFilter</code> and <code>tagFilter</code> are mutually exclusive. You can specify at most one of these in a request.</p> </note></p>
    #[serde(rename = "closeStatusFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_status_filter: Option<CloseStatusFilter>,
    /// <p><p>If specified, only workflow executions that meet the close time criteria of the filter are counted.</p> <note> <p> <code>startTimeFilter</code> and <code>closeTimeFilter</code> are mutually exclusive. You must specify one of these in a request but not both.</p> </note></p>
    #[serde(rename = "closeTimeFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_time_filter: Option<ExecutionTimeFilter>,
    /// <p>The name of the domain containing the workflow executions to count.</p>
    #[serde(rename = "domain")]
    pub domain: String,
    /// <p><p>If specified, only workflow executions matching the <code>WorkflowId</code> in the filter are counted.</p> <note> <p> <code>closeStatusFilter</code>, <code>executionFilter</code>, <code>typeFilter</code> and <code>tagFilter</code> are mutually exclusive. You can specify at most one of these in a request.</p> </note></p>
    #[serde(rename = "executionFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_filter: Option<WorkflowExecutionFilter>,
    /// <p><p>If specified, only workflow executions that meet the start time criteria of the filter are counted.</p> <note> <p> <code>startTimeFilter</code> and <code>closeTimeFilter</code> are mutually exclusive. You must specify one of these in a request but not both.</p> </note></p>
    #[serde(rename = "startTimeFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time_filter: Option<ExecutionTimeFilter>,
    /// <p><p>If specified, only executions that have a tag that matches the filter are counted.</p> <note> <p> <code>closeStatusFilter</code>, <code>executionFilter</code>, <code>typeFilter</code> and <code>tagFilter</code> are mutually exclusive. You can specify at most one of these in a request.</p> </note></p>
    #[serde(rename = "tagFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_filter: Option<TagFilter>,
    /// <p><p>If specified, indicates the type of the workflow executions to be counted.</p> <note> <p> <code>closeStatusFilter</code>, <code>executionFilter</code>, <code>typeFilter</code> and <code>tagFilter</code> are mutually exclusive. You can specify at most one of these in a request.</p> </note></p>
    #[serde(rename = "typeFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_filter: Option<WorkflowTypeFilter>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CountOpenWorkflowExecutionsInput {
    /// <p>The name of the domain containing the workflow executions to count.</p>
    #[serde(rename = "domain")]
    pub domain: String,
    /// <p><p>If specified, only workflow executions matching the <code>WorkflowId</code> in the filter are counted.</p> <note> <p> <code>executionFilter</code>, <code>typeFilter</code> and <code>tagFilter</code> are mutually exclusive. You can specify at most one of these in a request.</p> </note></p>
    #[serde(rename = "executionFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_filter: Option<WorkflowExecutionFilter>,
    /// <p>Specifies the start time criteria that workflow executions must meet in order to be counted.</p>
    #[serde(rename = "startTimeFilter")]
    pub start_time_filter: ExecutionTimeFilter,
    /// <p><p>If specified, only executions that have a tag that matches the filter are counted.</p> <note> <p> <code>executionFilter</code>, <code>typeFilter</code> and <code>tagFilter</code> are mutually exclusive. You can specify at most one of these in a request.</p> </note></p>
    #[serde(rename = "tagFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_filter: Option<TagFilter>,
    /// <p><p>Specifies the type of the workflow executions to be counted.</p> <note> <p> <code>executionFilter</code>, <code>typeFilter</code> and <code>tagFilter</code> are mutually exclusive. You can specify at most one of these in a request.</p> </note></p>
    #[serde(rename = "typeFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_filter: Option<WorkflowTypeFilter>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CountPendingActivityTasksInput {
    /// <p>The name of the domain that contains the task list.</p>
    #[serde(rename = "domain")]
    pub domain: String,
    /// <p>The name of the task list.</p>
    #[serde(rename = "taskList")]
    pub task_list: TaskList,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CountPendingDecisionTasksInput {
    /// <p>The name of the domain that contains the task list.</p>
    #[serde(rename = "domain")]
    pub domain: String,
    /// <p>The name of the task list.</p>
    #[serde(rename = "taskList")]
    pub task_list: TaskList,
}

/// <p><p>Specifies a decision made by the decider. A decision can be one of these types:</p> <ul> <li> <p> <code>CancelTimer</code> – Cancels a previously started timer and records a <code>TimerCanceled</code> event in the history.</p> </li> <li> <p> <code>CancelWorkflowExecution</code> – Closes the workflow execution and records a <code>WorkflowExecutionCanceled</code> event in the history.</p> </li> <li> <p> <code>CompleteWorkflowExecution</code> – Closes the workflow execution and records a <code>WorkflowExecutionCompleted</code> event in the history .</p> </li> <li> <p> <code>ContinueAsNewWorkflowExecution</code> – Closes the workflow execution and starts a new workflow execution of the same type using the same workflow ID and a unique run Id. A <code>WorkflowExecutionContinuedAsNew</code> event is recorded in the history.</p> </li> <li> <p> <code>FailWorkflowExecution</code> – Closes the workflow execution and records a <code>WorkflowExecutionFailed</code> event in the history.</p> </li> <li> <p> <code>RecordMarker</code> – Records a <code>MarkerRecorded</code> event in the history. Markers can be used for adding custom information in the history for instance to let deciders know that they don&#39;t need to look at the history beyond the marker event.</p> </li> <li> <p> <code>RequestCancelActivityTask</code> – Attempts to cancel a previously scheduled activity task. If the activity task was scheduled but has not been assigned to a worker, then it is canceled. If the activity task was already assigned to a worker, then the worker is informed that cancellation has been requested in the response to <a>RecordActivityTaskHeartbeat</a>.</p> </li> <li> <p> <code>RequestCancelExternalWorkflowExecution</code> – Requests that a request be made to cancel the specified external workflow execution and records a <code>RequestCancelExternalWorkflowExecutionInitiated</code> event in the history.</p> </li> <li> <p> <code>ScheduleActivityTask</code> – Schedules an activity task.</p> </li> <li> <p> <code>SignalExternalWorkflowExecution</code> – Requests a signal to be delivered to the specified external workflow execution and records a <code>SignalExternalWorkflowExecutionInitiated</code> event in the history.</p> </li> <li> <p> <code>StartChildWorkflowExecution</code> – Requests that a child workflow execution be started and records a <code>StartChildWorkflowExecutionInitiated</code> event in the history. The child workflow execution is a separate workflow execution with its own history.</p> </li> <li> <p> <code>StartTimer</code> – Starts a timer for this workflow execution and records a <code>TimerStarted</code> event in the history. This timer fires after the specified delay and record a <code>TimerFired</code> event.</p> </li> </ul> <p> <b>Access Control</b> </p> <p>If you grant permission to use <code>RespondDecisionTaskCompleted</code>, you can use IAM policies to express permissions for the list of decisions returned by this action as if they were members of the API. Treating decisions as a pseudo API maintains a uniform conceptual model and helps keep policies readable. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p> <p> <b>Decision Failure</b> </p> <p>Decisions can fail for several reasons</p> <ul> <li> <p>The ordering of decisions should follow a logical flow. Some decisions might not make sense in the current context of the workflow execution and therefore fails.</p> </li> <li> <p>A limit on your account was reached.</p> </li> <li> <p>The decision lacks sufficient permissions.</p> </li> </ul> <p>One of the following events might be added to the history to indicate an error. The event attribute&#39;s <code>cause</code> parameter indicates the cause. If <code>cause</code> is set to <code>OPERATION<em>NOT</em>PERMITTED</code>, the decision failed because it lacked sufficient permissions. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p> <ul> <li> <p> <code>ScheduleActivityTaskFailed</code> – A <code>ScheduleActivityTask</code> decision failed. This could happen if the activity type specified in the decision isn&#39;t registered, is in a deprecated state, or the decision isn&#39;t properly configured.</p> </li> <li> <p> <code>RequestCancelActivityTaskFailed</code> – A <code>RequestCancelActivityTask</code> decision failed. This could happen if there is no open activity task with the specified activityId.</p> </li> <li> <p> <code>StartTimerFailed</code> – A <code>StartTimer</code> decision failed. This could happen if there is another open timer with the same timerId.</p> </li> <li> <p> <code>CancelTimerFailed</code> – A <code>CancelTimer</code> decision failed. This could happen if there is no open timer with the specified timerId.</p> </li> <li> <p> <code>StartChildWorkflowExecutionFailed</code> – A <code>StartChildWorkflowExecution</code> decision failed. This could happen if the workflow type specified isn&#39;t registered, is deprecated, or the decision isn&#39;t properly configured.</p> </li> <li> <p> <code>SignalExternalWorkflowExecutionFailed</code> – A <code>SignalExternalWorkflowExecution</code> decision failed. This could happen if the <code>workflowID</code> specified in the decision was incorrect.</p> </li> <li> <p> <code>RequestCancelExternalWorkflowExecutionFailed</code> – A <code>RequestCancelExternalWorkflowExecution</code> decision failed. This could happen if the <code>workflowID</code> specified in the decision was incorrect.</p> </li> <li> <p> <code>CancelWorkflowExecutionFailed</code> – A <code>CancelWorkflowExecution</code> decision failed. This could happen if there is an unhandled decision task pending in the workflow execution.</p> </li> <li> <p> <code>CompleteWorkflowExecutionFailed</code> – A <code>CompleteWorkflowExecution</code> decision failed. This could happen if there is an unhandled decision task pending in the workflow execution.</p> </li> <li> <p> <code>ContinueAsNewWorkflowExecutionFailed</code> – A <code>ContinueAsNewWorkflowExecution</code> decision failed. This could happen if there is an unhandled decision task pending in the workflow execution or the ContinueAsNewWorkflowExecution decision was not configured correctly.</p> </li> <li> <p> <code>FailWorkflowExecutionFailed</code> – A <code>FailWorkflowExecution</code> decision failed. This could happen if there is an unhandled decision task pending in the workflow execution.</p> </li> </ul> <p>The preceding error events might occur due to an error in the decider logic, which might put the workflow execution in an unstable state The cause field in the event structure for the error event indicates the cause of the error.</p> <note> <p>A workflow execution may be closed by the decider by returning one of the following decisions when completing a decision task: <code>CompleteWorkflowExecution</code>, <code>FailWorkflowExecution</code>, <code>CancelWorkflowExecution</code> and <code>ContinueAsNewWorkflowExecution</code>. An <code>UnhandledDecision</code> fault is returned if a workflow closing decision is specified and a signal or activity event had been added to the history while the decision task was being performed by the decider. Unlike the above situations which are logic issues, this fault is always possible because of race conditions in a distributed system. The right action here is to call <a>RespondDecisionTaskCompleted</a> without any decisions. This would result in another decision task with these new events included in the history. The decider should handle the new events and may decide to close the workflow execution.</p> </note> <p> <b>How to Code a Decision</b> </p> <p>You code a decision by first setting the decision type field to one of the above decision values, and then set the corresponding attributes field shown below:</p> <ul> <li> <p> <code> <a>ScheduleActivityTaskDecisionAttributes</a> </code> </p> </li> <li> <p> <code> <a>RequestCancelActivityTaskDecisionAttributes</a> </code> </p> </li> <li> <p> <code> <a>CompleteWorkflowExecutionDecisionAttributes</a> </code> </p> </li> <li> <p> <code> <a>FailWorkflowExecutionDecisionAttributes</a> </code> </p> </li> <li> <p> <code> <a>CancelWorkflowExecutionDecisionAttributes</a> </code> </p> </li> <li> <p> <code> <a>ContinueAsNewWorkflowExecutionDecisionAttributes</a> </code> </p> </li> <li> <p> <code> <a>RecordMarkerDecisionAttributes</a> </code> </p> </li> <li> <p> <code> <a>StartTimerDecisionAttributes</a> </code> </p> </li> <li> <p> <code> <a>CancelTimerDecisionAttributes</a> </code> </p> </li> <li> <p> <code> <a>SignalExternalWorkflowExecutionDecisionAttributes</a> </code> </p> </li> <li> <p> <code> <a>RequestCancelExternalWorkflowExecutionDecisionAttributes</a> </code> </p> </li> <li> <p> <code> <a>StartChildWorkflowExecutionDecisionAttributes</a> </code> </p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Decision {
    /// <p>Provides the details of the <code>CancelTimer</code> decision. It isn't set for other decision types.</p>
    #[serde(rename = "cancelTimerDecisionAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_timer_decision_attributes: Option<CancelTimerDecisionAttributes>,
    /// <p>Provides the details of the <code>CancelWorkflowExecution</code> decision. It isn't set for other decision types.</p>
    #[serde(rename = "cancelWorkflowExecutionDecisionAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_workflow_execution_decision_attributes:
        Option<CancelWorkflowExecutionDecisionAttributes>,
    /// <p>Provides the details of the <code>CompleteWorkflowExecution</code> decision. It isn't set for other decision types.</p>
    #[serde(rename = "completeWorkflowExecutionDecisionAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete_workflow_execution_decision_attributes:
        Option<CompleteWorkflowExecutionDecisionAttributes>,
    /// <p>Provides the details of the <code>ContinueAsNewWorkflowExecution</code> decision. It isn't set for other decision types.</p>
    #[serde(rename = "continueAsNewWorkflowExecutionDecisionAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continue_as_new_workflow_execution_decision_attributes:
        Option<ContinueAsNewWorkflowExecutionDecisionAttributes>,
    /// <p>Specifies the type of the decision.</p>
    #[serde(rename = "decisionType")]
    pub decision_type: String,
    /// <p>Provides the details of the <code>FailWorkflowExecution</code> decision. It isn't set for other decision types.</p>
    #[serde(rename = "failWorkflowExecutionDecisionAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_workflow_execution_decision_attributes:
        Option<FailWorkflowExecutionDecisionAttributes>,
    /// <p>Provides the details of the <code>RecordMarker</code> decision. It isn't set for other decision types.</p>
    #[serde(rename = "recordMarkerDecisionAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_marker_decision_attributes: Option<RecordMarkerDecisionAttributes>,
    /// <p>Provides the details of the <code>RequestCancelActivityTask</code> decision. It isn't set for other decision types.</p>
    #[serde(rename = "requestCancelActivityTaskDecisionAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_cancel_activity_task_decision_attributes:
        Option<RequestCancelActivityTaskDecisionAttributes>,
    /// <p>Provides the details of the <code>RequestCancelExternalWorkflowExecution</code> decision. It isn't set for other decision types.</p>
    #[serde(rename = "requestCancelExternalWorkflowExecutionDecisionAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_cancel_external_workflow_execution_decision_attributes:
        Option<RequestCancelExternalWorkflowExecutionDecisionAttributes>,
    /// <p>Provides the details of the <code>ScheduleActivityTask</code> decision. It isn't set for other decision types.</p>
    #[serde(rename = "scheduleActivityTaskDecisionAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_activity_task_decision_attributes: Option<ScheduleActivityTaskDecisionAttributes>,
    /// <p>Provides the details of the <code>ScheduleLambdaFunction</code> decision. It isn't set for other decision types.</p>
    #[serde(rename = "scheduleLambdaFunctionDecisionAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_lambda_function_decision_attributes:
        Option<ScheduleLambdaFunctionDecisionAttributes>,
    /// <p>Provides the details of the <code>SignalExternalWorkflowExecution</code> decision. It isn't set for other decision types.</p>
    #[serde(rename = "signalExternalWorkflowExecutionDecisionAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal_external_workflow_execution_decision_attributes:
        Option<SignalExternalWorkflowExecutionDecisionAttributes>,
    /// <p>Provides the details of the <code>StartChildWorkflowExecution</code> decision. It isn't set for other decision types.</p>
    #[serde(rename = "startChildWorkflowExecutionDecisionAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_child_workflow_execution_decision_attributes:
        Option<StartChildWorkflowExecutionDecisionAttributes>,
    /// <p>Provides the details of the <code>StartTimer</code> decision. It isn't set for other decision types.</p>
    #[serde(rename = "startTimerDecisionAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timer_decision_attributes: Option<StartTimerDecisionAttributes>,
}

/// <p>A structure that represents a decision task. Decision tasks are sent to deciders in order for them to make decisions.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DecisionTask {
    /// <p>A paginated list of history events of the workflow execution. The decider uses this during the processing of the decision task.</p>
    #[serde(rename = "events")]
    pub events: Vec<HistoryEvent>,
    /// <p>If a <code>NextPageToken</code> was returned by a previous call, there are more results available. To retrieve the next page of results, make the call again using the returned token in <code>nextPageToken</code>. Keep all other arguments unchanged.</p> <p>The configured <code>maximumPageSize</code> determines how many results can be returned in a single call.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>The ID of the DecisionTaskStarted event of the previous decision task of this workflow execution that was processed by the decider. This can be used to determine the events in the history new since the last decision task received by the decider.</p>
    #[serde(rename = "previousStartedEventId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_started_event_id: Option<i64>,
    /// <p>The ID of the <code>DecisionTaskStarted</code> event recorded in the history.</p>
    #[serde(rename = "startedEventId")]
    pub started_event_id: i64,
    /// <p>The opaque string used as a handle on the task. This token is used by workers to communicate progress and response information back to the system about the task.</p>
    #[serde(rename = "taskToken")]
    pub task_token: String,
    /// <p>The workflow execution for which this decision task was created.</p>
    #[serde(rename = "workflowExecution")]
    pub workflow_execution: WorkflowExecution,
    /// <p>The type of the workflow execution for which this decision task was created.</p>
    #[serde(rename = "workflowType")]
    pub workflow_type: WorkflowType,
}

/// <p>Provides the details of the <code>DecisionTaskCompleted</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DecisionTaskCompletedEventAttributes {
    /// <p>User defined context for the workflow execution.</p>
    #[serde(rename = "executionContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_context: Option<String>,
    /// <p>The ID of the <code>DecisionTaskScheduled</code> event that was recorded when this decision task was scheduled. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "scheduledEventId")]
    pub scheduled_event_id: i64,
    /// <p>The ID of the <code>DecisionTaskStarted</code> event recorded when this decision task was started. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "startedEventId")]
    pub started_event_id: i64,
}

/// <p>Provides details about the <code>DecisionTaskScheduled</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DecisionTaskScheduledEventAttributes {
    /// <p>The maximum duration for this decision task. The task is considered timed out if it doesn't completed within this duration.</p> <p>The duration is specified in seconds, an integer greater than or equal to <code>0</code>. You can use <code>NONE</code> to specify unlimited duration.</p>
    #[serde(rename = "startToCloseTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_to_close_timeout: Option<String>,
    /// <p>The name of the task list in which the decision task was scheduled.</p>
    #[serde(rename = "taskList")]
    pub task_list: TaskList,
    /// <p> A task priority that, if set, specifies the priority for this decision task. Valid values are integers that range from Java's <code>Integer.MIN_VALUE</code> (-2147483648) to <code>Integer.MAX_VALUE</code> (2147483647). Higher numbers indicate higher priority.</p> <p>For more information about setting task priority, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/programming-priority.html">Setting Task Priority</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    #[serde(rename = "taskPriority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_priority: Option<String>,
}

/// <p>Provides the details of the <code>DecisionTaskStarted</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DecisionTaskStartedEventAttributes {
    /// <p>Identity of the decider making the request. This enables diagnostic tracing when problems arise. The form of this identity is user defined.</p>
    #[serde(rename = "identity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    /// <p>The ID of the <code>DecisionTaskScheduled</code> event that was recorded when this decision task was scheduled. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "scheduledEventId")]
    pub scheduled_event_id: i64,
}

/// <p>Provides the details of the <code>DecisionTaskTimedOut</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DecisionTaskTimedOutEventAttributes {
    /// <p>The ID of the <code>DecisionTaskScheduled</code> event that was recorded when this decision task was scheduled. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "scheduledEventId")]
    pub scheduled_event_id: i64,
    /// <p>The ID of the <code>DecisionTaskStarted</code> event recorded when this decision task was started. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "startedEventId")]
    pub started_event_id: i64,
    /// <p>The type of timeout that expired before the decision task could be completed.</p>
    #[serde(rename = "timeoutType")]
    pub timeout_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeprecateActivityTypeInput {
    /// <p>The activity type to deprecate.</p>
    #[serde(rename = "activityType")]
    pub activity_type: ActivityType,
    /// <p>The name of the domain in which the activity type is registered.</p>
    #[serde(rename = "domain")]
    pub domain: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeprecateDomainInput {
    /// <p>The name of the domain to deprecate.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeprecateWorkflowTypeInput {
    /// <p>The name of the domain in which the workflow type is registered.</p>
    #[serde(rename = "domain")]
    pub domain: String,
    /// <p>The workflow type to deprecate.</p>
    #[serde(rename = "workflowType")]
    pub workflow_type: WorkflowType,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeActivityTypeInput {
    /// <p>The activity type to get information about. Activity types are identified by the <code>name</code> and <code>version</code> that were supplied when the activity was registered.</p>
    #[serde(rename = "activityType")]
    pub activity_type: ActivityType,
    /// <p>The name of the domain in which the activity type is registered.</p>
    #[serde(rename = "domain")]
    pub domain: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeDomainInput {
    /// <p>The name of the domain to describe.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeWorkflowExecutionInput {
    /// <p>The name of the domain containing the workflow execution.</p>
    #[serde(rename = "domain")]
    pub domain: String,
    /// <p>The workflow execution to describe.</p>
    #[serde(rename = "execution")]
    pub execution: WorkflowExecution,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeWorkflowTypeInput {
    /// <p>The name of the domain in which this workflow type is registered.</p>
    #[serde(rename = "domain")]
    pub domain: String,
    /// <p>The workflow type to describe.</p>
    #[serde(rename = "workflowType")]
    pub workflow_type: WorkflowType,
}

/// <p>Contains the configuration settings of a domain.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DomainConfiguration {
    /// <p>The retention period for workflow executions in this domain.</p>
    #[serde(rename = "workflowExecutionRetentionPeriodInDays")]
    pub workflow_execution_retention_period_in_days: String,
}

/// <p>Contains details of a domain.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DomainDetail {
    /// <p>The domain configuration. Currently, this includes only the domain's retention period.</p>
    #[serde(rename = "configuration")]
    pub configuration: DomainConfiguration,
    /// <p>The basic information about a domain, such as its name, status, and description.</p>
    #[serde(rename = "domainInfo")]
    pub domain_info: DomainInfo,
}

/// <p>Contains general information about a domain.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DomainInfo {
    /// <p>The description of the domain provided through <a>RegisterDomain</a>.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the domain. This name is unique within the account.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p><p>The status of the domain:</p> <ul> <li> <p> <code>REGISTERED</code> – The domain is properly registered and available. You can use this domain for registering types and creating new workflow executions. </p> </li> <li> <p> <code>DEPRECATED</code> – The domain was deprecated using <a>DeprecateDomain</a>, but is still in use. You should not create new workflow executions in this domain. </p> </li> </ul></p>
    #[serde(rename = "status")]
    pub status: String,
}

/// <p>Contains a paginated collection of DomainInfo structures.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DomainInfos {
    /// <p>A list of DomainInfo structures.</p>
    #[serde(rename = "domainInfos")]
    pub domain_infos: Vec<DomainInfo>,
    /// <p>If a <code>NextPageToken</code> was returned by a previous call, there are more results available. To retrieve the next page of results, make the call again using the returned token in <code>nextPageToken</code>. Keep all other arguments unchanged.</p> <p>The configured <code>maximumPageSize</code> determines how many results can be returned in a single call.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

/// <p>Used to filter the workflow executions in visibility APIs by various time-based rules. Each parameter, if specified, defines a rule that must be satisfied by each returned query result. The parameter values are in the <a href="https://en.wikipedia.org/wiki/Unix_time">Unix Time format</a>. For example: <code>"oldestDate": 1325376070.</code> </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ExecutionTimeFilter {
    /// <p>Specifies the latest start or close date and time to return.</p>
    #[serde(rename = "latestDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_date: Option<f64>,
    /// <p>Specifies the oldest start or close date and time to return.</p>
    #[serde(rename = "oldestDate")]
    pub oldest_date: f64,
}

/// <p>Provides the details of the <code>ExternalWorkflowExecutionCancelRequested</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ExternalWorkflowExecutionCancelRequestedEventAttributes {
    /// <p>The ID of the <code>RequestCancelExternalWorkflowExecutionInitiated</code> event corresponding to the <code>RequestCancelExternalWorkflowExecution</code> decision to cancel this external workflow execution. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "initiatedEventId")]
    pub initiated_event_id: i64,
    /// <p>The external workflow execution to which the cancellation request was delivered.</p>
    #[serde(rename = "workflowExecution")]
    pub workflow_execution: WorkflowExecution,
}

/// <p>Provides the details of the <code>ExternalWorkflowExecutionSignaled</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ExternalWorkflowExecutionSignaledEventAttributes {
    /// <p>The ID of the <code>SignalExternalWorkflowExecutionInitiated</code> event corresponding to the <code>SignalExternalWorkflowExecution</code> decision to request this signal. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "initiatedEventId")]
    pub initiated_event_id: i64,
    /// <p>The external workflow execution that the signal was delivered to.</p>
    #[serde(rename = "workflowExecution")]
    pub workflow_execution: WorkflowExecution,
}

/// <p>Provides the details of the <code>FailWorkflowExecution</code> decision.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this decision's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct FailWorkflowExecutionDecisionAttributes {
    /// <p> Details of the failure.</p>
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// <p>A descriptive reason for the failure that may help in diagnostics.</p>
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// <p>Provides the details of the <code>FailWorkflowExecutionFailed</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct FailWorkflowExecutionFailedEventAttributes {
    /// <p><p>The cause of the failure. This information is generated by the system and can be useful for diagnostic purposes.</p> <note> <p>If <code>cause</code> is set to <code>OPERATION<em>NOT</em>PERMITTED</code>, the decision failed because it lacked sufficient permissions. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p> </note></p>
    #[serde(rename = "cause")]
    pub cause: String,
    /// <p>The ID of the <code>DecisionTaskCompleted</code> event corresponding to the decision task that resulted in the <code>FailWorkflowExecution</code> decision to fail this execution. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "decisionTaskCompletedEventId")]
    pub decision_task_completed_event_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetWorkflowExecutionHistoryInput {
    /// <p>The name of the domain containing the workflow execution.</p>
    #[serde(rename = "domain")]
    pub domain: String,
    /// <p>Specifies the workflow execution for which to return the history.</p>
    #[serde(rename = "execution")]
    pub execution: WorkflowExecution,
    /// <p>The maximum number of results that are returned per call. <code>nextPageToken</code> can be used to obtain futher pages of results. The default is 1000, which is the maximum allowed page size. You can, however, specify a page size <i>smaller</i> than the maximum.</p> <p>This is an upper limit only; the actual number of results returned per call may be fewer than the specified maximum.</p>
    #[serde(rename = "maximumPageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_page_size: Option<i64>,
    /// <p>If a <code>NextPageToken</code> was returned by a previous call, there are more results available. To retrieve the next page of results, make the call again using the returned token in <code>nextPageToken</code>. Keep all other arguments unchanged.</p> <p>The configured <code>maximumPageSize</code> determines how many results can be returned in a single call.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>When set to <code>true</code>, returns the events in reverse order. By default the results are returned in ascending order of the <code>eventTimeStamp</code> of the events.</p>
    #[serde(rename = "reverseOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_order: Option<bool>,
}

/// <p>Paginated representation of a workflow history for a workflow execution. This is the up to date, complete and authoritative record of the events related to all tasks and events in the life of the workflow execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct History {
    /// <p>The list of history events.</p>
    #[serde(rename = "events")]
    pub events: Vec<HistoryEvent>,
    /// <p>If a <code>NextPageToken</code> was returned by a previous call, there are more results available. To retrieve the next page of results, make the call again using the returned token in <code>nextPageToken</code>. Keep all other arguments unchanged.</p> <p>The configured <code>maximumPageSize</code> determines how many results can be returned in a single call.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

/// <p><p>Event within a workflow execution. A history event can be one of these types:</p> <ul> <li> <p> <code>ActivityTaskCancelRequested</code> – A <code>RequestCancelActivityTask</code> decision was received by the system.</p> </li> <li> <p> <code>ActivityTaskCanceled</code> – The activity task was successfully canceled.</p> </li> <li> <p> <code>ActivityTaskCompleted</code> – An activity worker successfully completed an activity task by calling <a>RespondActivityTaskCompleted</a>.</p> </li> <li> <p> <code>ActivityTaskFailed</code> – An activity worker failed an activity task by calling <a>RespondActivityTaskFailed</a>.</p> </li> <li> <p> <code>ActivityTaskScheduled</code> – An activity task was scheduled for execution.</p> </li> <li> <p> <code>ActivityTaskStarted</code> – The scheduled activity task was dispatched to a worker.</p> </li> <li> <p> <code>ActivityTaskTimedOut</code> – The activity task timed out.</p> </li> <li> <p> <code>CancelTimerFailed</code> – Failed to process CancelTimer decision. This happens when the decision isn&#39;t configured properly, for example no timer exists with the specified timer Id.</p> </li> <li> <p> <code>CancelWorkflowExecutionFailed</code> – A request to cancel a workflow execution failed.</p> </li> <li> <p> <code>ChildWorkflowExecutionCanceled</code> – A child workflow execution, started by this workflow execution, was canceled and closed.</p> </li> <li> <p> <code>ChildWorkflowExecutionCompleted</code> – A child workflow execution, started by this workflow execution, completed successfully and was closed.</p> </li> <li> <p> <code>ChildWorkflowExecutionFailed</code> – A child workflow execution, started by this workflow execution, failed to complete successfully and was closed.</p> </li> <li> <p> <code>ChildWorkflowExecutionStarted</code> – A child workflow execution was successfully started.</p> </li> <li> <p> <code>ChildWorkflowExecutionTerminated</code> – A child workflow execution, started by this workflow execution, was terminated.</p> </li> <li> <p> <code>ChildWorkflowExecutionTimedOut</code> – A child workflow execution, started by this workflow execution, timed out and was closed.</p> </li> <li> <p> <code>CompleteWorkflowExecutionFailed</code> – The workflow execution failed to complete.</p> </li> <li> <p> <code>ContinueAsNewWorkflowExecutionFailed</code> – The workflow execution failed to complete after being continued as a new workflow execution.</p> </li> <li> <p> <code>DecisionTaskCompleted</code> – The decider successfully completed a decision task by calling <a>RespondDecisionTaskCompleted</a>.</p> </li> <li> <p> <code>DecisionTaskScheduled</code> – A decision task was scheduled for the workflow execution.</p> </li> <li> <p> <code>DecisionTaskStarted</code> – The decision task was dispatched to a decider.</p> </li> <li> <p> <code>DecisionTaskTimedOut</code> – The decision task timed out.</p> </li> <li> <p> <code>ExternalWorkflowExecutionCancelRequested</code> – Request to cancel an external workflow execution was successfully delivered to the target execution.</p> </li> <li> <p> <code>ExternalWorkflowExecutionSignaled</code> – A signal, requested by this workflow execution, was successfully delivered to the target external workflow execution.</p> </li> <li> <p> <code>FailWorkflowExecutionFailed</code> – A request to mark a workflow execution as failed, itself failed.</p> </li> <li> <p> <code>MarkerRecorded</code> – A marker was recorded in the workflow history as the result of a <code>RecordMarker</code> decision.</p> </li> <li> <p> <code>RecordMarkerFailed</code> – A <code>RecordMarker</code> decision was returned as failed.</p> </li> <li> <p> <code>RequestCancelActivityTaskFailed</code> – Failed to process RequestCancelActivityTask decision. This happens when the decision isn&#39;t configured properly.</p> </li> <li> <p> <code>RequestCancelExternalWorkflowExecutionFailed</code> – Request to cancel an external workflow execution failed.</p> </li> <li> <p> <code>RequestCancelExternalWorkflowExecutionInitiated</code> – A request was made to request the cancellation of an external workflow execution.</p> </li> <li> <p> <code>ScheduleActivityTaskFailed</code> – Failed to process ScheduleActivityTask decision. This happens when the decision isn&#39;t configured properly, for example the activity type specified isn&#39;t registered.</p> </li> <li> <p> <code>SignalExternalWorkflowExecutionFailed</code> – The request to signal an external workflow execution failed.</p> </li> <li> <p> <code>SignalExternalWorkflowExecutionInitiated</code> – A request to signal an external workflow was made.</p> </li> <li> <p> <code>StartActivityTaskFailed</code> – A scheduled activity task failed to start.</p> </li> <li> <p> <code>StartChildWorkflowExecutionFailed</code> – Failed to process StartChildWorkflowExecution decision. This happens when the decision isn&#39;t configured properly, for example the workflow type specified isn&#39;t registered.</p> </li> <li> <p> <code>StartChildWorkflowExecutionInitiated</code> – A request was made to start a child workflow execution.</p> </li> <li> <p> <code>StartTimerFailed</code> – Failed to process StartTimer decision. This happens when the decision isn&#39;t configured properly, for example a timer already exists with the specified timer Id.</p> </li> <li> <p> <code>TimerCanceled</code> – A timer, previously started for this workflow execution, was successfully canceled.</p> </li> <li> <p> <code>TimerFired</code> – A timer, previously started for this workflow execution, fired.</p> </li> <li> <p> <code>TimerStarted</code> – A timer was started for the workflow execution due to a <code>StartTimer</code> decision.</p> </li> <li> <p> <code>WorkflowExecutionCancelRequested</code> – A request to cancel this workflow execution was made.</p> </li> <li> <p> <code>WorkflowExecutionCanceled</code> – The workflow execution was successfully canceled and closed.</p> </li> <li> <p> <code>WorkflowExecutionCompleted</code> – The workflow execution was closed due to successful completion.</p> </li> <li> <p> <code>WorkflowExecutionContinuedAsNew</code> – The workflow execution was closed and a new execution of the same type was created with the same workflowId.</p> </li> <li> <p> <code>WorkflowExecutionFailed</code> – The workflow execution closed due to a failure.</p> </li> <li> <p> <code>WorkflowExecutionSignaled</code> – An external signal was received for the workflow execution.</p> </li> <li> <p> <code>WorkflowExecutionStarted</code> – The workflow execution was started.</p> </li> <li> <p> <code>WorkflowExecutionTerminated</code> – The workflow execution was terminated.</p> </li> <li> <p> <code>WorkflowExecutionTimedOut</code> – The workflow execution was closed because a time out was exceeded.</p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct HistoryEvent {
    /// <p>If the event is of type <code>ActivityTaskcancelRequested</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "activityTaskCancelRequestedEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_task_cancel_requested_event_attributes:
        Option<ActivityTaskCancelRequestedEventAttributes>,
    /// <p>If the event is of type <code>ActivityTaskCanceled</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "activityTaskCanceledEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_task_canceled_event_attributes: Option<ActivityTaskCanceledEventAttributes>,
    /// <p>If the event is of type <code>ActivityTaskCompleted</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "activityTaskCompletedEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_task_completed_event_attributes: Option<ActivityTaskCompletedEventAttributes>,
    /// <p>If the event is of type <code>ActivityTaskFailed</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "activityTaskFailedEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_task_failed_event_attributes: Option<ActivityTaskFailedEventAttributes>,
    /// <p>If the event is of type <code>ActivityTaskScheduled</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "activityTaskScheduledEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_task_scheduled_event_attributes: Option<ActivityTaskScheduledEventAttributes>,
    /// <p>If the event is of type <code>ActivityTaskStarted</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "activityTaskStartedEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_task_started_event_attributes: Option<ActivityTaskStartedEventAttributes>,
    /// <p>If the event is of type <code>ActivityTaskTimedOut</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "activityTaskTimedOutEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_task_timed_out_event_attributes: Option<ActivityTaskTimedOutEventAttributes>,
    /// <p>If the event is of type <code>CancelTimerFailed</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "cancelTimerFailedEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_timer_failed_event_attributes: Option<CancelTimerFailedEventAttributes>,
    /// <p>If the event is of type <code>CancelWorkflowExecutionFailed</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "cancelWorkflowExecutionFailedEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_workflow_execution_failed_event_attributes:
        Option<CancelWorkflowExecutionFailedEventAttributes>,
    /// <p>If the event is of type <code>ChildWorkflowExecutionCanceled</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "childWorkflowExecutionCanceledEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_workflow_execution_canceled_event_attributes:
        Option<ChildWorkflowExecutionCanceledEventAttributes>,
    /// <p>If the event is of type <code>ChildWorkflowExecutionCompleted</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "childWorkflowExecutionCompletedEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_workflow_execution_completed_event_attributes:
        Option<ChildWorkflowExecutionCompletedEventAttributes>,
    /// <p>If the event is of type <code>ChildWorkflowExecutionFailed</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "childWorkflowExecutionFailedEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_workflow_execution_failed_event_attributes:
        Option<ChildWorkflowExecutionFailedEventAttributes>,
    /// <p>If the event is of type <code>ChildWorkflowExecutionStarted</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "childWorkflowExecutionStartedEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_workflow_execution_started_event_attributes:
        Option<ChildWorkflowExecutionStartedEventAttributes>,
    /// <p>If the event is of type <code>ChildWorkflowExecutionTerminated</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "childWorkflowExecutionTerminatedEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_workflow_execution_terminated_event_attributes:
        Option<ChildWorkflowExecutionTerminatedEventAttributes>,
    /// <p>If the event is of type <code>ChildWorkflowExecutionTimedOut</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "childWorkflowExecutionTimedOutEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_workflow_execution_timed_out_event_attributes:
        Option<ChildWorkflowExecutionTimedOutEventAttributes>,
    /// <p>If the event is of type <code>CompleteWorkflowExecutionFailed</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "completeWorkflowExecutionFailedEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete_workflow_execution_failed_event_attributes:
        Option<CompleteWorkflowExecutionFailedEventAttributes>,
    /// <p>If the event is of type <code>ContinueAsNewWorkflowExecutionFailed</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "continueAsNewWorkflowExecutionFailedEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continue_as_new_workflow_execution_failed_event_attributes:
        Option<ContinueAsNewWorkflowExecutionFailedEventAttributes>,
    /// <p>If the event is of type <code>DecisionTaskCompleted</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "decisionTaskCompletedEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_task_completed_event_attributes: Option<DecisionTaskCompletedEventAttributes>,
    /// <p>If the event is of type <code>DecisionTaskScheduled</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "decisionTaskScheduledEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_task_scheduled_event_attributes: Option<DecisionTaskScheduledEventAttributes>,
    /// <p>If the event is of type <code>DecisionTaskStarted</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "decisionTaskStartedEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_task_started_event_attributes: Option<DecisionTaskStartedEventAttributes>,
    /// <p>If the event is of type <code>DecisionTaskTimedOut</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "decisionTaskTimedOutEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_task_timed_out_event_attributes: Option<DecisionTaskTimedOutEventAttributes>,
    /// <p>The system generated ID of the event. This ID uniquely identifies the event with in the workflow execution history.</p>
    #[serde(rename = "eventId")]
    pub event_id: i64,
    /// <p>The date and time when the event occurred.</p>
    #[serde(rename = "eventTimestamp")]
    pub event_timestamp: f64,
    /// <p>The type of the history event.</p>
    #[serde(rename = "eventType")]
    pub event_type: String,
    /// <p>If the event is of type <code>ExternalWorkflowExecutionCancelRequested</code> then this member is set and provides detailed information about the event. It isn't set for other event types. </p>
    #[serde(rename = "externalWorkflowExecutionCancelRequestedEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_workflow_execution_cancel_requested_event_attributes:
        Option<ExternalWorkflowExecutionCancelRequestedEventAttributes>,
    /// <p>If the event is of type <code>ExternalWorkflowExecutionSignaled</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "externalWorkflowExecutionSignaledEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_workflow_execution_signaled_event_attributes:
        Option<ExternalWorkflowExecutionSignaledEventAttributes>,
    /// <p>If the event is of type <code>FailWorkflowExecutionFailed</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "failWorkflowExecutionFailedEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_workflow_execution_failed_event_attributes:
        Option<FailWorkflowExecutionFailedEventAttributes>,
    /// <p>Provides the details of the <code>LambdaFunctionCompleted</code> event. It isn't set for other event types.</p>
    #[serde(rename = "lambdaFunctionCompletedEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_completed_event_attributes: Option<LambdaFunctionCompletedEventAttributes>,
    /// <p>Provides the details of the <code>LambdaFunctionFailed</code> event. It isn't set for other event types.</p>
    #[serde(rename = "lambdaFunctionFailedEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_failed_event_attributes: Option<LambdaFunctionFailedEventAttributes>,
    /// <p>Provides the details of the <code>LambdaFunctionScheduled</code> event. It isn't set for other event types.</p>
    #[serde(rename = "lambdaFunctionScheduledEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_scheduled_event_attributes: Option<LambdaFunctionScheduledEventAttributes>,
    /// <p>Provides the details of the <code>LambdaFunctionStarted</code> event. It isn't set for other event types.</p>
    #[serde(rename = "lambdaFunctionStartedEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_started_event_attributes: Option<LambdaFunctionStartedEventAttributes>,
    /// <p>Provides the details of the <code>LambdaFunctionTimedOut</code> event. It isn't set for other event types.</p>
    #[serde(rename = "lambdaFunctionTimedOutEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_timed_out_event_attributes: Option<LambdaFunctionTimedOutEventAttributes>,
    /// <p>If the event is of type <code>MarkerRecorded</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "markerRecordedEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker_recorded_event_attributes: Option<MarkerRecordedEventAttributes>,
    /// <p>If the event is of type <code>DecisionTaskFailed</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "recordMarkerFailedEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_marker_failed_event_attributes: Option<RecordMarkerFailedEventAttributes>,
    /// <p>If the event is of type <code>RequestCancelActivityTaskFailed</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "requestCancelActivityTaskFailedEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_cancel_activity_task_failed_event_attributes:
        Option<RequestCancelActivityTaskFailedEventAttributes>,
    /// <p>If the event is of type <code>RequestCancelExternalWorkflowExecutionFailed</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "requestCancelExternalWorkflowExecutionFailedEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_cancel_external_workflow_execution_failed_event_attributes:
        Option<RequestCancelExternalWorkflowExecutionFailedEventAttributes>,
    /// <p>If the event is of type <code>RequestCancelExternalWorkflowExecutionInitiated</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "requestCancelExternalWorkflowExecutionInitiatedEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_cancel_external_workflow_execution_initiated_event_attributes:
        Option<RequestCancelExternalWorkflowExecutionInitiatedEventAttributes>,
    /// <p>If the event is of type <code>ScheduleActivityTaskFailed</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "scheduleActivityTaskFailedEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_activity_task_failed_event_attributes:
        Option<ScheduleActivityTaskFailedEventAttributes>,
    /// <p>Provides the details of the <code>ScheduleLambdaFunctionFailed</code> event. It isn't set for other event types.</p>
    #[serde(rename = "scheduleLambdaFunctionFailedEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_lambda_function_failed_event_attributes:
        Option<ScheduleLambdaFunctionFailedEventAttributes>,
    /// <p>If the event is of type <code>SignalExternalWorkflowExecutionFailed</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "signalExternalWorkflowExecutionFailedEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal_external_workflow_execution_failed_event_attributes:
        Option<SignalExternalWorkflowExecutionFailedEventAttributes>,
    /// <p>If the event is of type <code>SignalExternalWorkflowExecutionInitiated</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "signalExternalWorkflowExecutionInitiatedEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal_external_workflow_execution_initiated_event_attributes:
        Option<SignalExternalWorkflowExecutionInitiatedEventAttributes>,
    /// <p>If the event is of type <code>StartChildWorkflowExecutionFailed</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "startChildWorkflowExecutionFailedEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_child_workflow_execution_failed_event_attributes:
        Option<StartChildWorkflowExecutionFailedEventAttributes>,
    /// <p>If the event is of type <code>StartChildWorkflowExecutionInitiated</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "startChildWorkflowExecutionInitiatedEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_child_workflow_execution_initiated_event_attributes:
        Option<StartChildWorkflowExecutionInitiatedEventAttributes>,
    /// <p>Provides the details of the <code>StartLambdaFunctionFailed</code> event. It isn't set for other event types.</p>
    #[serde(rename = "startLambdaFunctionFailedEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_lambda_function_failed_event_attributes:
        Option<StartLambdaFunctionFailedEventAttributes>,
    /// <p>If the event is of type <code>StartTimerFailed</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "startTimerFailedEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timer_failed_event_attributes: Option<StartTimerFailedEventAttributes>,
    /// <p>If the event is of type <code>TimerCanceled</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "timerCanceledEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timer_canceled_event_attributes: Option<TimerCanceledEventAttributes>,
    /// <p>If the event is of type <code>TimerFired</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "timerFiredEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timer_fired_event_attributes: Option<TimerFiredEventAttributes>,
    /// <p>If the event is of type <code>TimerStarted</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "timerStartedEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timer_started_event_attributes: Option<TimerStartedEventAttributes>,
    /// <p>If the event is of type <code>WorkflowExecutionCancelRequested</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "workflowExecutionCancelRequestedEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_execution_cancel_requested_event_attributes:
        Option<WorkflowExecutionCancelRequestedEventAttributes>,
    /// <p>If the event is of type <code>WorkflowExecutionCanceled</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "workflowExecutionCanceledEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_execution_canceled_event_attributes:
        Option<WorkflowExecutionCanceledEventAttributes>,
    /// <p>If the event is of type <code>WorkflowExecutionCompleted</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "workflowExecutionCompletedEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_execution_completed_event_attributes:
        Option<WorkflowExecutionCompletedEventAttributes>,
    /// <p>If the event is of type <code>WorkflowExecutionContinuedAsNew</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "workflowExecutionContinuedAsNewEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_execution_continued_as_new_event_attributes:
        Option<WorkflowExecutionContinuedAsNewEventAttributes>,
    /// <p>If the event is of type <code>WorkflowExecutionFailed</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "workflowExecutionFailedEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_execution_failed_event_attributes: Option<WorkflowExecutionFailedEventAttributes>,
    /// <p>If the event is of type <code>WorkflowExecutionSignaled</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "workflowExecutionSignaledEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_execution_signaled_event_attributes:
        Option<WorkflowExecutionSignaledEventAttributes>,
    /// <p>If the event is of type <code>WorkflowExecutionStarted</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "workflowExecutionStartedEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_execution_started_event_attributes:
        Option<WorkflowExecutionStartedEventAttributes>,
    /// <p>If the event is of type <code>WorkflowExecutionTerminated</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "workflowExecutionTerminatedEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_execution_terminated_event_attributes:
        Option<WorkflowExecutionTerminatedEventAttributes>,
    /// <p>If the event is of type <code>WorkflowExecutionTimedOut</code> then this member is set and provides detailed information about the event. It isn't set for other event types.</p>
    #[serde(rename = "workflowExecutionTimedOutEventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_execution_timed_out_event_attributes:
        Option<WorkflowExecutionTimedOutEventAttributes>,
}

/// <p>Provides the details of the <code>LambdaFunctionCompleted</code> event. It isn't set for other event types.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct LambdaFunctionCompletedEventAttributes {
    /// <p>The results of the Lambda task.</p>
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    /// <p>The ID of the <code>LambdaFunctionScheduled</code> event that was recorded when this Lambda task was scheduled. To help diagnose issues, use this information to trace back the chain of events leading up to this event.</p>
    #[serde(rename = "scheduledEventId")]
    pub scheduled_event_id: i64,
    /// <p>The ID of the <code>LambdaFunctionStarted</code> event recorded when this activity task started. To help diagnose issues, use this information to trace back the chain of events leading up to this event.</p>
    #[serde(rename = "startedEventId")]
    pub started_event_id: i64,
}

/// <p>Provides the details of the <code>LambdaFunctionFailed</code> event. It isn't set for other event types.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct LambdaFunctionFailedEventAttributes {
    /// <p>The details of the failure.</p>
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// <p>The reason provided for the failure.</p>
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// <p>The ID of the <code>LambdaFunctionScheduled</code> event that was recorded when this activity task was scheduled. To help diagnose issues, use this information to trace back the chain of events leading up to this event.</p>
    #[serde(rename = "scheduledEventId")]
    pub scheduled_event_id: i64,
    /// <p>The ID of the <code>LambdaFunctionStarted</code> event recorded when this activity task started. To help diagnose issues, use this information to trace back the chain of events leading up to this event.</p>
    #[serde(rename = "startedEventId")]
    pub started_event_id: i64,
}

/// <p>Provides the details of the <code>LambdaFunctionScheduled</code> event. It isn't set for other event types.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct LambdaFunctionScheduledEventAttributes {
    /// <p>Data attached to the event that the decider can use in subsequent workflow tasks. This data isn't sent to the Lambda task.</p>
    #[serde(rename = "control")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<String>,
    /// <p>The ID of the <code>LambdaFunctionCompleted</code> event corresponding to the decision that resulted in scheduling this activity task. To help diagnose issues, use this information to trace back the chain of events leading up to this event.</p>
    #[serde(rename = "decisionTaskCompletedEventId")]
    pub decision_task_completed_event_id: i64,
    /// <p>The unique ID of the Lambda task.</p>
    #[serde(rename = "id")]
    pub id: String,
    /// <p>The input provided to the Lambda task.</p>
    #[serde(rename = "input")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    /// <p>The name of the Lambda function.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The maximum amount of time a worker can take to process the Lambda task.</p>
    #[serde(rename = "startToCloseTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_to_close_timeout: Option<String>,
}

/// <p>Provides the details of the <code>LambdaFunctionStarted</code> event. It isn't set for other event types.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct LambdaFunctionStartedEventAttributes {
    /// <p>The ID of the <code>LambdaFunctionScheduled</code> event that was recorded when this activity task was scheduled. To help diagnose issues, use this information to trace back the chain of events leading up to this event.</p>
    #[serde(rename = "scheduledEventId")]
    pub scheduled_event_id: i64,
}

/// <p>Provides details of the <code>LambdaFunctionTimedOut</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct LambdaFunctionTimedOutEventAttributes {
    /// <p>The ID of the <code>LambdaFunctionScheduled</code> event that was recorded when this activity task was scheduled. To help diagnose issues, use this information to trace back the chain of events leading up to this event.</p>
    #[serde(rename = "scheduledEventId")]
    pub scheduled_event_id: i64,
    /// <p>The ID of the <code>ActivityTaskStarted</code> event that was recorded when this activity task started. To help diagnose issues, use this information to trace back the chain of events leading up to this event.</p>
    #[serde(rename = "startedEventId")]
    pub started_event_id: i64,
    /// <p>The type of the timeout that caused this event.</p>
    #[serde(rename = "timeoutType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListActivityTypesInput {
    /// <p>The name of the domain in which the activity types have been registered.</p>
    #[serde(rename = "domain")]
    pub domain: String,
    /// <p>The maximum number of results that are returned per call. <code>nextPageToken</code> can be used to obtain futher pages of results. The default is 1000, which is the maximum allowed page size. You can, however, specify a page size <i>smaller</i> than the maximum.</p> <p>This is an upper limit only; the actual number of results returned per call may be fewer than the specified maximum.</p>
    #[serde(rename = "maximumPageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_page_size: Option<i64>,
    /// <p>If specified, only lists the activity types that have this name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>If a <code>NextPageToken</code> was returned by a previous call, there are more results available. To retrieve the next page of results, make the call again using the returned token in <code>nextPageToken</code>. Keep all other arguments unchanged.</p> <p>The configured <code>maximumPageSize</code> determines how many results can be returned in a single call.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>Specifies the registration status of the activity types to list.</p>
    #[serde(rename = "registrationStatus")]
    pub registration_status: String,
    /// <p>When set to <code>true</code>, returns the results in reverse order. By default, the results are returned in ascending alphabetical order by <code>name</code> of the activity types.</p>
    #[serde(rename = "reverseOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_order: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListClosedWorkflowExecutionsInput {
    /// <p><p>If specified, only workflow executions that match this <i>close status</i> are listed. For example, if TERMINATED is specified, then only TERMINATED workflow executions are listed.</p> <note> <p> <code>closeStatusFilter</code>, <code>executionFilter</code>, <code>typeFilter</code> and <code>tagFilter</code> are mutually exclusive. You can specify at most one of these in a request.</p> </note></p>
    #[serde(rename = "closeStatusFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_status_filter: Option<CloseStatusFilter>,
    /// <p><p>If specified, the workflow executions are included in the returned results based on whether their close times are within the range specified by this filter. Also, if this parameter is specified, the returned results are ordered by their close times.</p> <note> <p> <code>startTimeFilter</code> and <code>closeTimeFilter</code> are mutually exclusive. You must specify one of these in a request but not both.</p> </note></p>
    #[serde(rename = "closeTimeFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_time_filter: Option<ExecutionTimeFilter>,
    /// <p>The name of the domain that contains the workflow executions to list.</p>
    #[serde(rename = "domain")]
    pub domain: String,
    /// <p><p>If specified, only workflow executions matching the workflow ID specified in the filter are returned.</p> <note> <p> <code>closeStatusFilter</code>, <code>executionFilter</code>, <code>typeFilter</code> and <code>tagFilter</code> are mutually exclusive. You can specify at most one of these in a request.</p> </note></p>
    #[serde(rename = "executionFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_filter: Option<WorkflowExecutionFilter>,
    /// <p>The maximum number of results that are returned per call. <code>nextPageToken</code> can be used to obtain futher pages of results. The default is 1000, which is the maximum allowed page size. You can, however, specify a page size <i>smaller</i> than the maximum.</p> <p>This is an upper limit only; the actual number of results returned per call may be fewer than the specified maximum.</p>
    #[serde(rename = "maximumPageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_page_size: Option<i64>,
    /// <p>If a <code>NextPageToken</code> was returned by a previous call, there are more results available. To retrieve the next page of results, make the call again using the returned token in <code>nextPageToken</code>. Keep all other arguments unchanged.</p> <p>The configured <code>maximumPageSize</code> determines how many results can be returned in a single call.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>When set to <code>true</code>, returns the results in reverse order. By default the results are returned in descending order of the start or the close time of the executions.</p>
    #[serde(rename = "reverseOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_order: Option<bool>,
    /// <p><p>If specified, the workflow executions are included in the returned results based on whether their start times are within the range specified by this filter. Also, if this parameter is specified, the returned results are ordered by their start times.</p> <note> <p> <code>startTimeFilter</code> and <code>closeTimeFilter</code> are mutually exclusive. You must specify one of these in a request but not both.</p> </note></p>
    #[serde(rename = "startTimeFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time_filter: Option<ExecutionTimeFilter>,
    /// <p><p>If specified, only executions that have the matching tag are listed.</p> <note> <p> <code>closeStatusFilter</code>, <code>executionFilter</code>, <code>typeFilter</code> and <code>tagFilter</code> are mutually exclusive. You can specify at most one of these in a request.</p> </note></p>
    #[serde(rename = "tagFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_filter: Option<TagFilter>,
    /// <p><p>If specified, only executions of the type specified in the filter are returned.</p> <note> <p> <code>closeStatusFilter</code>, <code>executionFilter</code>, <code>typeFilter</code> and <code>tagFilter</code> are mutually exclusive. You can specify at most one of these in a request.</p> </note></p>
    #[serde(rename = "typeFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_filter: Option<WorkflowTypeFilter>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListDomainsInput {
    /// <p>The maximum number of results that are returned per call. <code>nextPageToken</code> can be used to obtain futher pages of results. The default is 1000, which is the maximum allowed page size. You can, however, specify a page size <i>smaller</i> than the maximum.</p> <p>This is an upper limit only; the actual number of results returned per call may be fewer than the specified maximum.</p>
    #[serde(rename = "maximumPageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_page_size: Option<i64>,
    /// <p>If a <code>NextPageToken</code> was returned by a previous call, there are more results available. To retrieve the next page of results, make the call again using the returned token in <code>nextPageToken</code>. Keep all other arguments unchanged.</p> <p>The configured <code>maximumPageSize</code> determines how many results can be returned in a single call.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>Specifies the registration status of the domains to list.</p>
    #[serde(rename = "registrationStatus")]
    pub registration_status: String,
    /// <p>When set to <code>true</code>, returns the results in reverse order. By default, the results are returned in ascending alphabetical order by <code>name</code> of the domains.</p>
    #[serde(rename = "reverseOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_order: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListOpenWorkflowExecutionsInput {
    /// <p>The name of the domain that contains the workflow executions to list.</p>
    #[serde(rename = "domain")]
    pub domain: String,
    /// <p><p>If specified, only workflow executions matching the workflow ID specified in the filter are returned.</p> <note> <p> <code>executionFilter</code>, <code>typeFilter</code> and <code>tagFilter</code> are mutually exclusive. You can specify at most one of these in a request.</p> </note></p>
    #[serde(rename = "executionFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_filter: Option<WorkflowExecutionFilter>,
    /// <p>The maximum number of results that are returned per call. <code>nextPageToken</code> can be used to obtain futher pages of results. The default is 1000, which is the maximum allowed page size. You can, however, specify a page size <i>smaller</i> than the maximum.</p> <p>This is an upper limit only; the actual number of results returned per call may be fewer than the specified maximum.</p>
    #[serde(rename = "maximumPageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_page_size: Option<i64>,
    /// <p>If a <code>NextPageToken</code> was returned by a previous call, there are more results available. To retrieve the next page of results, make the call again using the returned token in <code>nextPageToken</code>. Keep all other arguments unchanged.</p> <p>The configured <code>maximumPageSize</code> determines how many results can be returned in a single call.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>When set to <code>true</code>, returns the results in reverse order. By default the results are returned in descending order of the start time of the executions.</p>
    #[serde(rename = "reverseOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_order: Option<bool>,
    /// <p>Workflow executions are included in the returned results based on whether their start times are within the range specified by this filter.</p>
    #[serde(rename = "startTimeFilter")]
    pub start_time_filter: ExecutionTimeFilter,
    /// <p><p>If specified, only executions that have the matching tag are listed.</p> <note> <p> <code>executionFilter</code>, <code>typeFilter</code> and <code>tagFilter</code> are mutually exclusive. You can specify at most one of these in a request.</p> </note></p>
    #[serde(rename = "tagFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_filter: Option<TagFilter>,
    /// <p><p>If specified, only executions of the type specified in the filter are returned.</p> <note> <p> <code>executionFilter</code>, <code>typeFilter</code> and <code>tagFilter</code> are mutually exclusive. You can specify at most one of these in a request.</p> </note></p>
    #[serde(rename = "typeFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_filter: Option<WorkflowTypeFilter>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListWorkflowTypesInput {
    /// <p>The name of the domain in which the workflow types have been registered.</p>
    #[serde(rename = "domain")]
    pub domain: String,
    /// <p>The maximum number of results that are returned per call. <code>nextPageToken</code> can be used to obtain futher pages of results. The default is 1000, which is the maximum allowed page size. You can, however, specify a page size <i>smaller</i> than the maximum.</p> <p>This is an upper limit only; the actual number of results returned per call may be fewer than the specified maximum.</p>
    #[serde(rename = "maximumPageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_page_size: Option<i64>,
    /// <p>If specified, lists the workflow type with this name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>If a <code>NextPageToken</code> was returned by a previous call, there are more results available. To retrieve the next page of results, make the call again using the returned token in <code>nextPageToken</code>. Keep all other arguments unchanged.</p> <p>The configured <code>maximumPageSize</code> determines how many results can be returned in a single call.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>Specifies the registration status of the workflow types to list.</p>
    #[serde(rename = "registrationStatus")]
    pub registration_status: String,
    /// <p>When set to <code>true</code>, returns the results in reverse order. By default the results are returned in ascending alphabetical order of the <code>name</code> of the workflow types.</p>
    #[serde(rename = "reverseOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_order: Option<bool>,
}

/// <p>Provides the details of the <code>MarkerRecorded</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct MarkerRecordedEventAttributes {
    /// <p>The ID of the <code>DecisionTaskCompleted</code> event corresponding to the decision task that resulted in the <code>RecordMarker</code> decision that requested this marker. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "decisionTaskCompletedEventId")]
    pub decision_task_completed_event_id: i64,
    /// <p>The details of the marker.</p>
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// <p>The name of the marker.</p>
    #[serde(rename = "markerName")]
    pub marker_name: String,
}

/// <p>Contains the count of tasks in a task list.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PendingTaskCount {
    /// <p>The number of tasks in the task list.</p>
    #[serde(rename = "count")]
    pub count: i64,
    /// <p>If set to true, indicates that the actual count was more than the maximum supported by this API and the count returned is the truncated value.</p>
    #[serde(rename = "truncated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PollForActivityTaskInput {
    /// <p>The name of the domain that contains the task lists being polled.</p>
    #[serde(rename = "domain")]
    pub domain: String,
    /// <p>Identity of the worker making the request, recorded in the <code>ActivityTaskStarted</code> event in the workflow history. This enables diagnostic tracing when problems arise. The form of this identity is user defined.</p>
    #[serde(rename = "identity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    /// <p>Specifies the task list to poll for activity tasks.</p> <p>The specified string must not start or end with whitespace. It must not contain a <code>:</code> (colon), <code>/</code> (slash), <code>|</code> (vertical bar), or any control characters (<code>\u0000-\u001f</code> | <code>\u007f-\u009f</code>). Also, it must not contain the literal string <code>arn</code>.</p>
    #[serde(rename = "taskList")]
    pub task_list: TaskList,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PollForDecisionTaskInput {
    /// <p>The name of the domain containing the task lists to poll.</p>
    #[serde(rename = "domain")]
    pub domain: String,
    /// <p>Identity of the decider making the request, which is recorded in the DecisionTaskStarted event in the workflow history. This enables diagnostic tracing when problems arise. The form of this identity is user defined.</p>
    #[serde(rename = "identity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    /// <p>The maximum number of results that are returned per call. <code>nextPageToken</code> can be used to obtain futher pages of results. The default is 1000, which is the maximum allowed page size. You can, however, specify a page size <i>smaller</i> than the maximum.</p> <p>This is an upper limit only; the actual number of results returned per call may be fewer than the specified maximum.</p>
    #[serde(rename = "maximumPageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_page_size: Option<i64>,
    /// <p><p>If a <code>NextPageToken</code> was returned by a previous call, there are more results available. To retrieve the next page of results, make the call again using the returned token in <code>nextPageToken</code>. Keep all other arguments unchanged.</p> <p>The configured <code>maximumPageSize</code> determines how many results can be returned in a single call.</p> <note> <p>The <code>nextPageToken</code> returned by this action cannot be used with <a>GetWorkflowExecutionHistory</a> to get the next page. You must call <a>PollForDecisionTask</a> again (with the <code>nextPageToken</code>) to retrieve the next page of history records. Calling <a>PollForDecisionTask</a> with a <code>nextPageToken</code> doesn&#39;t return a new decision task.</p> </note></p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>When set to <code>true</code>, returns the events in reverse order. By default the results are returned in ascending order of the <code>eventTimestamp</code> of the events.</p>
    #[serde(rename = "reverseOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_order: Option<bool>,
    /// <p>Specifies the task list to poll for decision tasks.</p> <p>The specified string must not start or end with whitespace. It must not contain a <code>:</code> (colon), <code>/</code> (slash), <code>|</code> (vertical bar), or any control characters (<code>\u0000-\u001f</code> | <code>\u007f-\u009f</code>). Also, it must not contain the literal string <code>arn</code>.</p>
    #[serde(rename = "taskList")]
    pub task_list: TaskList,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RecordActivityTaskHeartbeatInput {
    /// <p>If specified, contains details about the progress of the task.</p>
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// <p><p>The <code>taskToken</code> of the <a>ActivityTask</a>.</p> <important> <p> <code>taskToken</code> is generated by the service and should be treated as an opaque value. If the task is passed to another process, its <code>taskToken</code> must also be passed. This enables it to provide its progress and respond with results. </p> </important></p>
    #[serde(rename = "taskToken")]
    pub task_token: String,
}

/// <p>Provides the details of the <code>RecordMarker</code> decision.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this decision's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RecordMarkerDecisionAttributes {
    /// <p> The details of the marker.</p>
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// <p> The name of the marker.</p>
    #[serde(rename = "markerName")]
    pub marker_name: String,
}

/// <p>Provides the details of the <code>RecordMarkerFailed</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RecordMarkerFailedEventAttributes {
    /// <p><p>The cause of the failure. This information is generated by the system and can be useful for diagnostic purposes.</p> <note> <p>If <code>cause</code> is set to <code>OPERATION<em>NOT</em>PERMITTED</code>, the decision failed because it lacked sufficient permissions. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p> </note></p>
    #[serde(rename = "cause")]
    pub cause: String,
    /// <p>The ID of the <code>DecisionTaskCompleted</code> event corresponding to the decision task that resulted in the <code>RecordMarkerFailed</code> decision for this cancellation request. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "decisionTaskCompletedEventId")]
    pub decision_task_completed_event_id: i64,
    /// <p>The marker's name.</p>
    #[serde(rename = "markerName")]
    pub marker_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RegisterActivityTypeInput {
    /// <p>If set, specifies the default maximum time before which a worker processing a task of this type must report progress by calling <a>RecordActivityTaskHeartbeat</a>. If the timeout is exceeded, the activity task is automatically timed out. This default can be overridden when scheduling an activity task using the <code>ScheduleActivityTask</code> <a>Decision</a>. If the activity worker subsequently attempts to record a heartbeat or returns a result, the activity worker receives an <code>UnknownResource</code> fault. In this case, Amazon SWF no longer considers the activity task to be valid; the activity worker should clean up the activity task.</p> <p>The duration is specified in seconds, an integer greater than or equal to <code>0</code>. You can use <code>NONE</code> to specify unlimited duration.</p>
    #[serde(rename = "defaultTaskHeartbeatTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_task_heartbeat_timeout: Option<String>,
    /// <p>If set, specifies the default task list to use for scheduling tasks of this activity type. This default task list is used if a task list isn't provided when a task is scheduled through the <code>ScheduleActivityTask</code> <a>Decision</a>.</p>
    #[serde(rename = "defaultTaskList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_task_list: Option<TaskList>,
    /// <p>The default task priority to assign to the activity type. If not assigned, then <code>0</code> is used. Valid values are integers that range from Java's <code>Integer.MIN_VALUE</code> (-2147483648) to <code>Integer.MAX_VALUE</code> (2147483647). Higher numbers indicate higher priority.</p> <p>For more information about setting task priority, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/programming-priority.html">Setting Task Priority</a> in the <i>in the <i>Amazon SWF Developer Guide</i>.</i>.</p>
    #[serde(rename = "defaultTaskPriority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_task_priority: Option<String>,
    /// <p>If set, specifies the default maximum duration for a task of this activity type. This default can be overridden when scheduling an activity task using the <code>ScheduleActivityTask</code> <a>Decision</a>.</p> <p>The duration is specified in seconds, an integer greater than or equal to <code>0</code>. You can use <code>NONE</code> to specify unlimited duration.</p>
    #[serde(rename = "defaultTaskScheduleToCloseTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_task_schedule_to_close_timeout: Option<String>,
    /// <p>If set, specifies the default maximum duration that a task of this activity type can wait before being assigned to a worker. This default can be overridden when scheduling an activity task using the <code>ScheduleActivityTask</code> <a>Decision</a>.</p> <p>The duration is specified in seconds, an integer greater than or equal to <code>0</code>. You can use <code>NONE</code> to specify unlimited duration.</p>
    #[serde(rename = "defaultTaskScheduleToStartTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_task_schedule_to_start_timeout: Option<String>,
    /// <p>If set, specifies the default maximum duration that a worker can take to process tasks of this activity type. This default can be overridden when scheduling an activity task using the <code>ScheduleActivityTask</code> <a>Decision</a>.</p> <p>The duration is specified in seconds, an integer greater than or equal to <code>0</code>. You can use <code>NONE</code> to specify unlimited duration.</p>
    #[serde(rename = "defaultTaskStartToCloseTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_task_start_to_close_timeout: Option<String>,
    /// <p>A textual description of the activity type.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the domain in which this activity is to be registered.</p>
    #[serde(rename = "domain")]
    pub domain: String,
    /// <p>The name of the activity type within the domain.</p> <p>The specified string must not start or end with whitespace. It must not contain a <code>:</code> (colon), <code>/</code> (slash), <code>|</code> (vertical bar), or any control characters (<code>\u0000-\u001f</code> | <code>\u007f-\u009f</code>). Also, it must not contain the literal string <code>arn</code>.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The version of the activity type.</p> <note> <p>The activity type consists of the name and version, the combination of which must be unique within the domain.</p> </note> <p>The specified string must not start or end with whitespace. It must not contain a <code>:</code> (colon), <code>/</code> (slash), <code>|</code> (vertical bar), or any control characters (<code>\u0000-\u001f</code> | <code>\u007f-\u009f</code>). Also, it must not contain the literal string <code>arn</code>.</p>
    #[serde(rename = "version")]
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RegisterDomainInput {
    /// <p>A text description of the domain.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Name of the domain to register. The name must be unique in the region that the domain is registered in.</p> <p>The specified string must not start or end with whitespace. It must not contain a <code>:</code> (colon), <code>/</code> (slash), <code>|</code> (vertical bar), or any control characters (<code>\u0000-\u001f</code> | <code>\u007f-\u009f</code>). Also, it must not contain the literal string <code>arn</code>.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The duration (in days) that records and histories of workflow executions on the domain should be kept by the service. After the retention period, the workflow execution isn't available in the results of visibility calls.</p> <p>If you pass the value <code>NONE</code> or <code>0</code> (zero), then the workflow execution history isn't retained. As soon as the workflow execution completes, the execution record and its history are deleted.</p> <p>The maximum workflow execution retention period is 90 days. For more information about Amazon SWF service limits, see: <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dg-limits.html">Amazon SWF Service Limits</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    #[serde(rename = "workflowExecutionRetentionPeriodInDays")]
    pub workflow_execution_retention_period_in_days: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RegisterWorkflowTypeInput {
    /// <p><p>If set, specifies the default policy to use for the child workflow executions when a workflow execution of this type is terminated, by calling the <a>TerminateWorkflowExecution</a> action explicitly or due to an expired timeout. This default can be overridden when starting a workflow execution using the <a>StartWorkflowExecution</a> action or the <code>StartChildWorkflowExecution</code> <a>Decision</a>.</p> <p>The supported child policies are:</p> <ul> <li> <p> <code>TERMINATE</code> – The child executions are terminated.</p> </li> <li> <p> <code>REQUEST_CANCEL</code> – A request to cancel is attempted for each child execution by recording a <code>WorkflowExecutionCancelRequested</code> event in its history. It is up to the decider to take appropriate actions when it receives an execution history with this event.</p> </li> <li> <p> <code>ABANDON</code> – No action is taken. The child executions continue to run.</p> </li> </ul></p>
    #[serde(rename = "defaultChildPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_child_policy: Option<String>,
    /// <p>If set, specifies the default maximum duration for executions of this workflow type. You can override this default when starting an execution through the <a>StartWorkflowExecution</a> Action or <code>StartChildWorkflowExecution</code> <a>Decision</a>.</p> <p>The duration is specified in seconds; an integer greater than or equal to 0. Unlike some of the other timeout parameters in Amazon SWF, you cannot specify a value of "NONE" for <code>defaultExecutionStartToCloseTimeout</code>; there is a one-year max limit on the time that a workflow execution can run. Exceeding this limit always causes the workflow execution to time out.</p>
    #[serde(rename = "defaultExecutionStartToCloseTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_execution_start_to_close_timeout: Option<String>,
    /// <p><p>The default IAM role attached to this workflow type.</p> <note> <p>Executions of this workflow type need IAM roles to invoke Lambda functions. If you don&#39;t specify an IAM role when you start this workflow type, the default Lambda role is attached to the execution. For more information, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/lambda-task.html">http://docs.aws.amazon.com/amazonswf/latest/developerguide/lambda-task.html</a> in the <i>Amazon SWF Developer Guide</i>.</p> </note></p>
    #[serde(rename = "defaultLambdaRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_lambda_role: Option<String>,
    /// <p>If set, specifies the default task list to use for scheduling decision tasks for executions of this workflow type. This default is used only if a task list isn't provided when starting the execution through the <a>StartWorkflowExecution</a> Action or <code>StartChildWorkflowExecution</code> <a>Decision</a>.</p>
    #[serde(rename = "defaultTaskList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_task_list: Option<TaskList>,
    /// <p>The default task priority to assign to the workflow type. If not assigned, then <code>0</code> is used. Valid values are integers that range from Java's <code>Integer.MIN_VALUE</code> (-2147483648) to <code>Integer.MAX_VALUE</code> (2147483647). Higher numbers indicate higher priority.</p> <p>For more information about setting task priority, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/programming-priority.html">Setting Task Priority</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    #[serde(rename = "defaultTaskPriority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_task_priority: Option<String>,
    /// <p>If set, specifies the default maximum duration of decision tasks for this workflow type. This default can be overridden when starting a workflow execution using the <a>StartWorkflowExecution</a> action or the <code>StartChildWorkflowExecution</code> <a>Decision</a>.</p> <p>The duration is specified in seconds, an integer greater than or equal to <code>0</code>. You can use <code>NONE</code> to specify unlimited duration.</p>
    #[serde(rename = "defaultTaskStartToCloseTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_task_start_to_close_timeout: Option<String>,
    /// <p>Textual description of the workflow type.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the domain in which to register the workflow type.</p>
    #[serde(rename = "domain")]
    pub domain: String,
    /// <p>The name of the workflow type.</p> <p>The specified string must not start or end with whitespace. It must not contain a <code>:</code> (colon), <code>/</code> (slash), <code>|</code> (vertical bar), or any control characters (<code>\u0000-\u001f</code> | <code>\u007f-\u009f</code>). Also, it must not contain the literal string <code>arn</code>.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The version of the workflow type.</p> <note> <p>The workflow type consists of the name and version, the combination of which must be unique within the domain. To get a list of all currently registered workflow types, use the <a>ListWorkflowTypes</a> action.</p> </note> <p>The specified string must not start or end with whitespace. It must not contain a <code>:</code> (colon), <code>/</code> (slash), <code>|</code> (vertical bar), or any control characters (<code>\u0000-\u001f</code> | <code>\u007f-\u009f</code>). Also, it must not contain the literal string <code>arn</code>.</p>
    #[serde(rename = "version")]
    pub version: String,
}

/// <p>Provides the details of the <code>RequestCancelActivityTask</code> decision.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this decision's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RequestCancelActivityTaskDecisionAttributes {
    /// <p>The <code>activityId</code> of the activity task to be canceled.</p>
    #[serde(rename = "activityId")]
    pub activity_id: String,
}

/// <p>Provides the details of the <code>RequestCancelActivityTaskFailed</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RequestCancelActivityTaskFailedEventAttributes {
    /// <p>The activityId provided in the <code>RequestCancelActivityTask</code> decision that failed.</p>
    #[serde(rename = "activityId")]
    pub activity_id: String,
    /// <p><p>The cause of the failure. This information is generated by the system and can be useful for diagnostic purposes.</p> <note> <p>If <code>cause</code> is set to <code>OPERATION<em>NOT</em>PERMITTED</code>, the decision failed because it lacked sufficient permissions. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p> </note></p>
    #[serde(rename = "cause")]
    pub cause: String,
    /// <p>The ID of the <code>DecisionTaskCompleted</code> event corresponding to the decision task that resulted in the <code>RequestCancelActivityTask</code> decision for this cancellation request. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "decisionTaskCompletedEventId")]
    pub decision_task_completed_event_id: i64,
}

/// <p>Provides the details of the <code>RequestCancelExternalWorkflowExecution</code> decision.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this decision's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RequestCancelExternalWorkflowExecutionDecisionAttributes {
    /// <p>The data attached to the event that can be used by the decider in subsequent workflow tasks.</p>
    #[serde(rename = "control")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<String>,
    /// <p>The <code>runId</code> of the external workflow execution to cancel.</p>
    #[serde(rename = "runId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    /// <p> The <code>workflowId</code> of the external workflow execution to cancel.</p>
    #[serde(rename = "workflowId")]
    pub workflow_id: String,
}

/// <p>Provides the details of the <code>RequestCancelExternalWorkflowExecutionFailed</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RequestCancelExternalWorkflowExecutionFailedEventAttributes {
    /// <p><p>The cause of the failure. This information is generated by the system and can be useful for diagnostic purposes.</p> <note> <p>If <code>cause</code> is set to <code>OPERATION<em>NOT</em>PERMITTED</code>, the decision failed because it lacked sufficient permissions. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p> </note></p>
    #[serde(rename = "cause")]
    pub cause: String,
    /// <p>The data attached to the event that the decider can use in subsequent workflow tasks. This data isn't sent to the workflow execution.</p>
    #[serde(rename = "control")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<String>,
    /// <p>The ID of the <code>DecisionTaskCompleted</code> event corresponding to the decision task that resulted in the <code>RequestCancelExternalWorkflowExecution</code> decision for this cancellation request. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "decisionTaskCompletedEventId")]
    pub decision_task_completed_event_id: i64,
    /// <p>The ID of the <code>RequestCancelExternalWorkflowExecutionInitiated</code> event corresponding to the <code>RequestCancelExternalWorkflowExecution</code> decision to cancel this external workflow execution. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "initiatedEventId")]
    pub initiated_event_id: i64,
    /// <p>The <code>runId</code> of the external workflow execution.</p>
    #[serde(rename = "runId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    /// <p>The <code>workflowId</code> of the external workflow to which the cancel request was to be delivered.</p>
    #[serde(rename = "workflowId")]
    pub workflow_id: String,
}

/// <p>Provides the details of the <code>RequestCancelExternalWorkflowExecutionInitiated</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RequestCancelExternalWorkflowExecutionInitiatedEventAttributes {
    /// <p>Data attached to the event that can be used by the decider in subsequent workflow tasks.</p>
    #[serde(rename = "control")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<String>,
    /// <p>The ID of the <code>DecisionTaskCompleted</code> event corresponding to the decision task that resulted in the <code>RequestCancelExternalWorkflowExecution</code> decision for this cancellation request. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "decisionTaskCompletedEventId")]
    pub decision_task_completed_event_id: i64,
    /// <p>The <code>runId</code> of the external workflow execution to be canceled.</p>
    #[serde(rename = "runId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    /// <p>The <code>workflowId</code> of the external workflow execution to be canceled.</p>
    #[serde(rename = "workflowId")]
    pub workflow_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RequestCancelWorkflowExecutionInput {
    /// <p>The name of the domain containing the workflow execution to cancel.</p>
    #[serde(rename = "domain")]
    pub domain: String,
    /// <p>The runId of the workflow execution to cancel.</p>
    #[serde(rename = "runId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    /// <p>The workflowId of the workflow execution to cancel.</p>
    #[serde(rename = "workflowId")]
    pub workflow_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RespondActivityTaskCanceledInput {
    /// <p> Information about the cancellation.</p>
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// <p><p>The <code>taskToken</code> of the <a>ActivityTask</a>.</p> <important> <p> <code>taskToken</code> is generated by the service and should be treated as an opaque value. If the task is passed to another process, its <code>taskToken</code> must also be passed. This enables it to provide its progress and respond with results.</p> </important></p>
    #[serde(rename = "taskToken")]
    pub task_token: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RespondActivityTaskCompletedInput {
    /// <p>The result of the activity task. It is a free form string that is implementation specific.</p>
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    /// <p><p>The <code>taskToken</code> of the <a>ActivityTask</a>.</p> <important> <p> <code>taskToken</code> is generated by the service and should be treated as an opaque value. If the task is passed to another process, its <code>taskToken</code> must also be passed. This enables it to provide its progress and respond with results.</p> </important></p>
    #[serde(rename = "taskToken")]
    pub task_token: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RespondActivityTaskFailedInput {
    /// <p> Detailed information about the failure.</p>
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// <p>Description of the error that may assist in diagnostics.</p>
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// <p><p>The <code>taskToken</code> of the <a>ActivityTask</a>.</p> <important> <p> <code>taskToken</code> is generated by the service and should be treated as an opaque value. If the task is passed to another process, its <code>taskToken</code> must also be passed. This enables it to provide its progress and respond with results.</p> </important></p>
    #[serde(rename = "taskToken")]
    pub task_token: String,
}

/// <p>Input data for a TaskCompleted response to a decision task.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RespondDecisionTaskCompletedInput {
    /// <p>The list of decisions (possibly empty) made by the decider while processing this decision task. See the docs for the <a>Decision</a> structure for details.</p>
    #[serde(rename = "decisions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decisions: Option<Vec<Decision>>,
    /// <p>User defined context to add to workflow execution.</p>
    #[serde(rename = "executionContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_context: Option<String>,
    /// <p><p>The <code>taskToken</code> from the <a>DecisionTask</a>.</p> <important> <p> <code>taskToken</code> is generated by the service and should be treated as an opaque value. If the task is passed to another process, its <code>taskToken</code> must also be passed. This enables it to provide its progress and respond with results.</p> </important></p>
    #[serde(rename = "taskToken")]
    pub task_token: String,
}

/// <p>Specifies the <code>runId</code> of a workflow execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Run {
    /// <p>The <code>runId</code> of a workflow execution. This ID is generated by the service and can be used to uniquely identify the workflow execution within a domain.</p>
    #[serde(rename = "runId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
}

/// <p>Provides the details of the <code>ScheduleActivityTask</code> decision.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this decision's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>activityType.name</code> – String constraint. The key is <code>swf:activityType.name</code>.</p> </li> <li> <p> <code>activityType.version</code> – String constraint. The key is <code>swf:activityType.version</code>.</p> </li> <li> <p> <code>taskList</code> – String constraint. The key is <code>swf:taskList.name</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ScheduleActivityTaskDecisionAttributes {
    /// <p> The <code>activityId</code> of the activity task.</p> <p>The specified string must not start or end with whitespace. It must not contain a <code>:</code> (colon), <code>/</code> (slash), <code>|</code> (vertical bar), or any control characters (<code>\u0000-\u001f</code> | <code>\u007f-\u009f</code>). Also, it must not contain the literal string <code>arn</code>.</p>
    #[serde(rename = "activityId")]
    pub activity_id: String,
    /// <p> The type of the activity task to schedule.</p>
    #[serde(rename = "activityType")]
    pub activity_type: ActivityType,
    /// <p>Data attached to the event that can be used by the decider in subsequent workflow tasks. This data isn't sent to the activity.</p>
    #[serde(rename = "control")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<String>,
    /// <p>If set, specifies the maximum time before which a worker processing a task of this type must report progress by calling <a>RecordActivityTaskHeartbeat</a>. If the timeout is exceeded, the activity task is automatically timed out. If the worker subsequently attempts to record a heartbeat or returns a result, it is ignored. This overrides the default heartbeat timeout specified when registering the activity type using <a>RegisterActivityType</a>.</p> <p>The duration is specified in seconds, an integer greater than or equal to <code>0</code>. You can use <code>NONE</code> to specify unlimited duration.</p>
    #[serde(rename = "heartbeatTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heartbeat_timeout: Option<String>,
    /// <p>The input provided to the activity task.</p>
    #[serde(rename = "input")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    /// <p><p>The maximum duration for this activity task.</p> <p>The duration is specified in seconds, an integer greater than or equal to <code>0</code>. You can use <code>NONE</code> to specify unlimited duration.</p> <note> <p>A schedule-to-close timeout for this activity task must be specified either as a default for the activity type or through this field. If neither this field is set nor a default schedule-to-close timeout was specified at registration time then a fault is returned.</p> </note></p>
    #[serde(rename = "scheduleToCloseTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_to_close_timeout: Option<String>,
    /// <p><p> If set, specifies the maximum duration the activity task can wait to be assigned to a worker. This overrides the default schedule-to-start timeout specified when registering the activity type using <a>RegisterActivityType</a>.</p> <p>The duration is specified in seconds, an integer greater than or equal to <code>0</code>. You can use <code>NONE</code> to specify unlimited duration.</p> <note> <p>A schedule-to-start timeout for this activity task must be specified either as a default for the activity type or through this field. If neither this field is set nor a default schedule-to-start timeout was specified at registration time then a fault is returned.</p> </note></p>
    #[serde(rename = "scheduleToStartTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_to_start_timeout: Option<String>,
    /// <p><p>If set, specifies the maximum duration a worker may take to process this activity task. This overrides the default start-to-close timeout specified when registering the activity type using <a>RegisterActivityType</a>.</p> <p>The duration is specified in seconds, an integer greater than or equal to <code>0</code>. You can use <code>NONE</code> to specify unlimited duration.</p> <note> <p>A start-to-close timeout for this activity task must be specified either as a default for the activity type or through this field. If neither this field is set nor a default start-to-close timeout was specified at registration time then a fault is returned.</p> </note></p>
    #[serde(rename = "startToCloseTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_to_close_timeout: Option<String>,
    /// <p>If set, specifies the name of the task list in which to schedule the activity task. If not specified, the <code>defaultTaskList</code> registered with the activity type is used.</p> <note> <p>A task list for this activity task must be specified either as a default for the activity type or through this field. If neither this field is set nor a default task list was specified at registration time then a fault is returned.</p> </note> <p>The specified string must not start or end with whitespace. It must not contain a <code>:</code> (colon), <code>/</code> (slash), <code>|</code> (vertical bar), or any control characters (<code>\u0000-\u001f</code> | <code>\u007f-\u009f</code>). Also, it must not contain the literal string <code>arn</code>.</p>
    #[serde(rename = "taskList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_list: Option<TaskList>,
    /// <p> If set, specifies the priority with which the activity task is to be assigned to a worker. This overrides the defaultTaskPriority specified when registering the activity type using <a>RegisterActivityType</a>. Valid values are integers that range from Java's <code>Integer.MIN_VALUE</code> (-2147483648) to <code>Integer.MAX_VALUE</code> (2147483647). Higher numbers indicate higher priority.</p> <p>For more information about setting task priority, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/programming-priority.html">Setting Task Priority</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    #[serde(rename = "taskPriority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_priority: Option<String>,
}

/// <p>Provides the details of the <code>ScheduleActivityTaskFailed</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ScheduleActivityTaskFailedEventAttributes {
    /// <p>The activityId provided in the <code>ScheduleActivityTask</code> decision that failed.</p>
    #[serde(rename = "activityId")]
    pub activity_id: String,
    /// <p>The activity type provided in the <code>ScheduleActivityTask</code> decision that failed.</p>
    #[serde(rename = "activityType")]
    pub activity_type: ActivityType,
    /// <p><p>The cause of the failure. This information is generated by the system and can be useful for diagnostic purposes.</p> <note> <p>If <code>cause</code> is set to <code>OPERATION<em>NOT</em>PERMITTED</code>, the decision failed because it lacked sufficient permissions. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p> </note></p>
    #[serde(rename = "cause")]
    pub cause: String,
    /// <p>The ID of the <code>DecisionTaskCompleted</code> event corresponding to the decision that resulted in the scheduling of this activity task. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "decisionTaskCompletedEventId")]
    pub decision_task_completed_event_id: i64,
}

/// <p>Decision attributes specified in <code>scheduleLambdaFunctionDecisionAttributes</code> within the list of decisions <code>decisions</code> passed to <a>RespondDecisionTaskCompleted</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ScheduleLambdaFunctionDecisionAttributes {
    /// <p>The data attached to the event that the decider can use in subsequent workflow tasks. This data isn't sent to the Lambda task.</p>
    #[serde(rename = "control")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<String>,
    /// <p>A string that identifies the Lambda function execution in the event history.</p>
    #[serde(rename = "id")]
    pub id: String,
    /// <p>The optional input data to be supplied to the Lambda function.</p>
    #[serde(rename = "input")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    /// <p>The name, or ARN, of the Lambda function to schedule.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The timeout value, in seconds, after which the Lambda function is considered to be failed once it has started. This can be any integer from 1-300 (1s-5m). If no value is supplied, than a default value of 300s is assumed.</p>
    #[serde(rename = "startToCloseTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_to_close_timeout: Option<String>,
}

/// <p>Provides the details of the <code>ScheduleLambdaFunctionFailed</code> event. It isn't set for other event types.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ScheduleLambdaFunctionFailedEventAttributes {
    /// <p><p>The cause of the failure. To help diagnose issues, use this information to trace back the chain of events leading up to this event.</p> <note> <p>If <code>cause</code> is set to <code>OPERATION<em>NOT</em>PERMITTED</code>, the decision failed because it lacked sufficient permissions. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p> </note></p>
    #[serde(rename = "cause")]
    pub cause: String,
    /// <p>The ID of the <code>LambdaFunctionCompleted</code> event corresponding to the decision that resulted in scheduling this Lambda task. To help diagnose issues, use this information to trace back the chain of events leading up to this event.</p>
    #[serde(rename = "decisionTaskCompletedEventId")]
    pub decision_task_completed_event_id: i64,
    /// <p>The ID provided in the <code>ScheduleLambdaFunction</code> decision that failed. </p>
    #[serde(rename = "id")]
    pub id: String,
    /// <p>The name of the Lambda function.</p>
    #[serde(rename = "name")]
    pub name: String,
}

/// <p>Provides the details of the <code>SignalExternalWorkflowExecution</code> decision.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this decision's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SignalExternalWorkflowExecutionDecisionAttributes {
    /// <p>The data attached to the event that can be used by the decider in subsequent decision tasks.</p>
    #[serde(rename = "control")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<String>,
    /// <p> The input data to be provided with the signal. The target workflow execution uses the signal name and input data to process the signal.</p>
    #[serde(rename = "input")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    /// <p>The <code>runId</code> of the workflow execution to be signaled.</p>
    #[serde(rename = "runId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    /// <p> The name of the signal.The target workflow execution uses the signal name and input to process the signal.</p>
    #[serde(rename = "signalName")]
    pub signal_name: String,
    /// <p> The <code>workflowId</code> of the workflow execution to be signaled.</p>
    #[serde(rename = "workflowId")]
    pub workflow_id: String,
}

/// <p>Provides the details of the <code>SignalExternalWorkflowExecutionFailed</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SignalExternalWorkflowExecutionFailedEventAttributes {
    /// <p><p>The cause of the failure. This information is generated by the system and can be useful for diagnostic purposes.</p> <note> <p>If <code>cause</code> is set to <code>OPERATION<em>NOT</em>PERMITTED</code>, the decision failed because it lacked sufficient permissions. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p> </note></p>
    #[serde(rename = "cause")]
    pub cause: String,
    /// <p>The data attached to the event that the decider can use in subsequent workflow tasks. This data isn't sent to the workflow execution.</p>
    #[serde(rename = "control")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<String>,
    /// <p>The ID of the <code>DecisionTaskCompleted</code> event corresponding to the decision task that resulted in the <code>SignalExternalWorkflowExecution</code> decision for this signal. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "decisionTaskCompletedEventId")]
    pub decision_task_completed_event_id: i64,
    /// <p>The ID of the <code>SignalExternalWorkflowExecutionInitiated</code> event corresponding to the <code>SignalExternalWorkflowExecution</code> decision to request this signal. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "initiatedEventId")]
    pub initiated_event_id: i64,
    /// <p>The <code>runId</code> of the external workflow execution that the signal was being delivered to.</p>
    #[serde(rename = "runId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    /// <p>The <code>workflowId</code> of the external workflow execution that the signal was being delivered to.</p>
    #[serde(rename = "workflowId")]
    pub workflow_id: String,
}

/// <p>Provides the details of the <code>SignalExternalWorkflowExecutionInitiated</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SignalExternalWorkflowExecutionInitiatedEventAttributes {
    /// <p>Data attached to the event that can be used by the decider in subsequent decision tasks.</p>
    #[serde(rename = "control")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<String>,
    /// <p>The ID of the <code>DecisionTaskCompleted</code> event corresponding to the decision task that resulted in the <code>SignalExternalWorkflowExecution</code> decision for this signal. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "decisionTaskCompletedEventId")]
    pub decision_task_completed_event_id: i64,
    /// <p>The input provided to the signal.</p>
    #[serde(rename = "input")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    /// <p>The <code>runId</code> of the external workflow execution to send the signal to.</p>
    #[serde(rename = "runId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    /// <p>The name of the signal.</p>
    #[serde(rename = "signalName")]
    pub signal_name: String,
    /// <p>The <code>workflowId</code> of the external workflow execution.</p>
    #[serde(rename = "workflowId")]
    pub workflow_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SignalWorkflowExecutionInput {
    /// <p>The name of the domain containing the workflow execution to signal.</p>
    #[serde(rename = "domain")]
    pub domain: String,
    /// <p>Data to attach to the <code>WorkflowExecutionSignaled</code> event in the target workflow execution's history.</p>
    #[serde(rename = "input")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    /// <p>The runId of the workflow execution to signal.</p>
    #[serde(rename = "runId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    /// <p>The name of the signal. This name must be meaningful to the target workflow.</p>
    #[serde(rename = "signalName")]
    pub signal_name: String,
    /// <p>The workflowId of the workflow execution to signal.</p>
    #[serde(rename = "workflowId")]
    pub workflow_id: String,
}

/// <p>Provides the details of the <code>StartChildWorkflowExecution</code> decision.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this decision's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>tagList.member.N</code> – The key is "swf:tagList.N" where N is the tag number from 0 to 4, inclusive.</p> </li> <li> <p> <code>taskList</code> – String constraint. The key is <code>swf:taskList.name</code>.</p> </li> <li> <p> <code>workflowType.name</code> – String constraint. The key is <code>swf:workflowType.name</code>.</p> </li> <li> <p> <code>workflowType.version</code> – String constraint. The key is <code>swf:workflowType.version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartChildWorkflowExecutionDecisionAttributes {
    /// <p><p> If set, specifies the policy to use for the child workflow executions if the workflow execution being started is terminated by calling the <a>TerminateWorkflowExecution</a> action explicitly or due to an expired timeout. This policy overrides the default child policy specified when registering the workflow type using <a>RegisterWorkflowType</a>.</p> <p>The supported child policies are:</p> <ul> <li> <p> <code>TERMINATE</code> – The child executions are terminated.</p> </li> <li> <p> <code>REQUEST_CANCEL</code> – A request to cancel is attempted for each child execution by recording a <code>WorkflowExecutionCancelRequested</code> event in its history. It is up to the decider to take appropriate actions when it receives an execution history with this event.</p> </li> <li> <p> <code>ABANDON</code> – No action is taken. The child executions continue to run.</p> </li> </ul> <note> <p>A child policy for this workflow execution must be specified either as a default for the workflow type or through this parameter. If neither this parameter is set nor a default child policy was specified at registration time then a fault is returned.</p> </note></p>
    #[serde(rename = "childPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_policy: Option<String>,
    /// <p>The data attached to the event that can be used by the decider in subsequent workflow tasks. This data isn't sent to the child workflow execution.</p>
    #[serde(rename = "control")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<String>,
    /// <p><p>The total duration for this workflow execution. This overrides the defaultExecutionStartToCloseTimeout specified when registering the workflow type.</p> <p>The duration is specified in seconds, an integer greater than or equal to <code>0</code>. You can use <code>NONE</code> to specify unlimited duration.</p> <note> <p>An execution start-to-close timeout for this workflow execution must be specified either as a default for the workflow type or through this parameter. If neither this parameter is set nor a default execution start-to-close timeout was specified at registration time then a fault is returned.</p> </note></p>
    #[serde(rename = "executionStartToCloseTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_start_to_close_timeout: Option<String>,
    /// <p>The input to be provided to the workflow execution.</p>
    #[serde(rename = "input")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    /// <p>The IAM role attached to the child workflow execution.</p>
    #[serde(rename = "lambdaRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_role: Option<String>,
    /// <p>The list of tags to associate with the child workflow execution. A maximum of 5 tags can be specified. You can list workflow executions with a specific tag by calling <a>ListOpenWorkflowExecutions</a> or <a>ListClosedWorkflowExecutions</a> and specifying a <a>TagFilter</a>.</p>
    #[serde(rename = "tagList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<String>>,
    /// <p>The name of the task list to be used for decision tasks of the child workflow execution.</p> <note> <p>A task list for this workflow execution must be specified either as a default for the workflow type or through this parameter. If neither this parameter is set nor a default task list was specified at registration time then a fault is returned.</p> </note> <p>The specified string must not start or end with whitespace. It must not contain a <code>:</code> (colon), <code>/</code> (slash), <code>|</code> (vertical bar), or any control characters (<code>\u0000-\u001f</code> | <code>\u007f-\u009f</code>). Also, it must not contain the literal string <code>arn</code>.</p>
    #[serde(rename = "taskList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_list: Option<TaskList>,
    /// <p> A task priority that, if set, specifies the priority for a decision task of this workflow execution. This overrides the defaultTaskPriority specified when registering the workflow type. Valid values are integers that range from Java's <code>Integer.MIN_VALUE</code> (-2147483648) to <code>Integer.MAX_VALUE</code> (2147483647). Higher numbers indicate higher priority.</p> <p>For more information about setting task priority, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/programming-priority.html">Setting Task Priority</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    #[serde(rename = "taskPriority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_priority: Option<String>,
    /// <p><p>Specifies the maximum duration of decision tasks for this workflow execution. This parameter overrides the <code>defaultTaskStartToCloseTimout</code> specified when registering the workflow type using <a>RegisterWorkflowType</a>.</p> <p>The duration is specified in seconds, an integer greater than or equal to <code>0</code>. You can use <code>NONE</code> to specify unlimited duration.</p> <note> <p>A task start-to-close timeout for this workflow execution must be specified either as a default for the workflow type or through this parameter. If neither this parameter is set nor a default task start-to-close timeout was specified at registration time then a fault is returned.</p> </note></p>
    #[serde(rename = "taskStartToCloseTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_start_to_close_timeout: Option<String>,
    /// <p> The <code>workflowId</code> of the workflow execution.</p> <p>The specified string must not start or end with whitespace. It must not contain a <code>:</code> (colon), <code>/</code> (slash), <code>|</code> (vertical bar), or any control characters (<code>\u0000-\u001f</code> | <code>\u007f-\u009f</code>). Also, it must not contain the literal string <code>arn</code>.</p>
    #[serde(rename = "workflowId")]
    pub workflow_id: String,
    /// <p> The type of the workflow execution to be started.</p>
    #[serde(rename = "workflowType")]
    pub workflow_type: WorkflowType,
}

/// <p>Provides the details of the <code>StartChildWorkflowExecutionFailed</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StartChildWorkflowExecutionFailedEventAttributes {
    /// <p><p>The cause of the failure. This information is generated by the system and can be useful for diagnostic purposes.</p> <note> <p>When <code>cause</code> is set to <code>OPERATION<em>NOT</em>PERMITTED</code>, the decision fails because it lacks sufficient permissions. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html"> Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p> </note></p>
    #[serde(rename = "cause")]
    pub cause: String,
    /// <p>The data attached to the event that the decider can use in subsequent workflow tasks. This data isn't sent to the child workflow execution.</p>
    #[serde(rename = "control")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<String>,
    /// <p>The ID of the <code>DecisionTaskCompleted</code> event corresponding to the decision task that resulted in the <code>StartChildWorkflowExecution</code> <a>Decision</a> to request this child workflow execution. This information can be useful for diagnosing problems by tracing back the chain of events.</p>
    #[serde(rename = "decisionTaskCompletedEventId")]
    pub decision_task_completed_event_id: i64,
    /// <p>When the <code>cause</code> is <code>WORKFLOW_ALREADY_RUNNING</code>, <code>initiatedEventId</code> is the ID of the <code>StartChildWorkflowExecutionInitiated</code> event that corresponds to the <code>StartChildWorkflowExecution</code> <a>Decision</a> to start the workflow execution. You can use this information to diagnose problems by tracing back the chain of events leading up to this event.</p> <p>When the <code>cause</code> isn't <code>WORKFLOW_ALREADY_RUNNING</code>, <code>initiatedEventId</code> is set to <code>0</code> because the <code>StartChildWorkflowExecutionInitiated</code> event doesn't exist.</p>
    #[serde(rename = "initiatedEventId")]
    pub initiated_event_id: i64,
    /// <p>The <code>workflowId</code> of the child workflow execution.</p>
    #[serde(rename = "workflowId")]
    pub workflow_id: String,
    /// <p>The workflow type provided in the <code>StartChildWorkflowExecution</code> <a>Decision</a> that failed.</p>
    #[serde(rename = "workflowType")]
    pub workflow_type: WorkflowType,
}

/// <p>Provides the details of the <code>StartChildWorkflowExecutionInitiated</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StartChildWorkflowExecutionInitiatedEventAttributes {
    /// <p><p>The policy to use for the child workflow executions if this execution gets terminated by explicitly calling the <a>TerminateWorkflowExecution</a> action or due to an expired timeout.</p> <p>The supported child policies are:</p> <ul> <li> <p> <code>TERMINATE</code> – The child executions are terminated.</p> </li> <li> <p> <code>REQUEST_CANCEL</code> – A request to cancel is attempted for each child execution by recording a <code>WorkflowExecutionCancelRequested</code> event in its history. It is up to the decider to take appropriate actions when it receives an execution history with this event.</p> </li> <li> <p> <code>ABANDON</code> – No action is taken. The child executions continue to run.</p> </li> </ul></p>
    #[serde(rename = "childPolicy")]
    pub child_policy: String,
    /// <p>Data attached to the event that can be used by the decider in subsequent decision tasks. This data isn't sent to the activity.</p>
    #[serde(rename = "control")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<String>,
    /// <p>The ID of the <code>DecisionTaskCompleted</code> event corresponding to the decision task that resulted in the <code>StartChildWorkflowExecution</code> <a>Decision</a> to request this child workflow execution. This information can be useful for diagnosing problems by tracing back the cause of events.</p>
    #[serde(rename = "decisionTaskCompletedEventId")]
    pub decision_task_completed_event_id: i64,
    /// <p>The maximum duration for the child workflow execution. If the workflow execution isn't closed within this duration, it is timed out and force-terminated.</p> <p>The duration is specified in seconds, an integer greater than or equal to <code>0</code>. You can use <code>NONE</code> to specify unlimited duration.</p>
    #[serde(rename = "executionStartToCloseTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_start_to_close_timeout: Option<String>,
    /// <p>The inputs provided to the child workflow execution.</p>
    #[serde(rename = "input")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    /// <p>The IAM role to attach to the child workflow execution.</p>
    #[serde(rename = "lambdaRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_role: Option<String>,
    /// <p>The list of tags to associated with the child workflow execution.</p>
    #[serde(rename = "tagList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<String>>,
    /// <p>The name of the task list used for the decision tasks of the child workflow execution.</p>
    #[serde(rename = "taskList")]
    pub task_list: TaskList,
    /// <p> The priority assigned for the decision tasks for this workflow execution. Valid values are integers that range from Java's <code>Integer.MIN_VALUE</code> (-2147483648) to <code>Integer.MAX_VALUE</code> (2147483647). Higher numbers indicate higher priority.</p> <p>For more information about setting task priority, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/programming-priority.html">Setting Task Priority</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    #[serde(rename = "taskPriority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_priority: Option<String>,
    /// <p>The maximum duration allowed for the decision tasks for this workflow execution.</p> <p>The duration is specified in seconds, an integer greater than or equal to <code>0</code>. You can use <code>NONE</code> to specify unlimited duration.</p>
    #[serde(rename = "taskStartToCloseTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_start_to_close_timeout: Option<String>,
    /// <p>The <code>workflowId</code> of the child workflow execution.</p>
    #[serde(rename = "workflowId")]
    pub workflow_id: String,
    /// <p>The type of the child workflow execution.</p>
    #[serde(rename = "workflowType")]
    pub workflow_type: WorkflowType,
}

/// <p>Provides the details of the <code>StartLambdaFunctionFailed</code> event. It isn't set for other event types.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StartLambdaFunctionFailedEventAttributes {
    /// <p><p>The cause of the failure. To help diagnose issues, use this information to trace back the chain of events leading up to this event.</p> <note> <p>If <code>cause</code> is set to <code>OPERATION<em>NOT</em>PERMITTED</code>, the decision failed because the IAM role attached to the execution lacked sufficient permissions. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/lambda-task.html">Lambda Tasks</a> in the <i>Amazon SWF Developer Guide</i>.</p> </note></p>
    #[serde(rename = "cause")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    /// <p>A description that can help diagnose the cause of the fault.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The ID of the <code>ActivityTaskScheduled</code> event that was recorded when this activity task was scheduled. To help diagnose issues, use this information to trace back the chain of events leading up to this event.</p>
    #[serde(rename = "scheduledEventId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_event_id: Option<i64>,
}

/// <p>Provides the details of the <code>StartTimer</code> decision.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this decision's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartTimerDecisionAttributes {
    /// <p>The data attached to the event that can be used by the decider in subsequent workflow tasks.</p>
    #[serde(rename = "control")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<String>,
    /// <p> The duration to wait before firing the timer.</p> <p>The duration is specified in seconds, an integer greater than or equal to <code>0</code>.</p>
    #[serde(rename = "startToFireTimeout")]
    pub start_to_fire_timeout: String,
    /// <p> The unique ID of the timer.</p> <p>The specified string must not start or end with whitespace. It must not contain a <code>:</code> (colon), <code>/</code> (slash), <code>|</code> (vertical bar), or any control characters (<code>\u0000-\u001f</code> | <code>\u007f-\u009f</code>). Also, it must not contain the literal string <code>arn</code>.</p>
    #[serde(rename = "timerId")]
    pub timer_id: String,
}

/// <p>Provides the details of the <code>StartTimerFailed</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StartTimerFailedEventAttributes {
    /// <p><p>The cause of the failure. This information is generated by the system and can be useful for diagnostic purposes.</p> <note> <p>If <code>cause</code> is set to <code>OPERATION<em>NOT</em>PERMITTED</code>, the decision failed because it lacked sufficient permissions. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p> </note></p>
    #[serde(rename = "cause")]
    pub cause: String,
    /// <p>The ID of the <code>DecisionTaskCompleted</code> event corresponding to the decision task that resulted in the <code>StartTimer</code> decision for this activity task. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "decisionTaskCompletedEventId")]
    pub decision_task_completed_event_id: i64,
    /// <p>The timerId provided in the <code>StartTimer</code> decision that failed.</p>
    #[serde(rename = "timerId")]
    pub timer_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartWorkflowExecutionInput {
    /// <p><p>If set, specifies the policy to use for the child workflow executions of this workflow execution if it is terminated, by calling the <a>TerminateWorkflowExecution</a> action explicitly or due to an expired timeout. This policy overrides the default child policy specified when registering the workflow type using <a>RegisterWorkflowType</a>.</p> <p>The supported child policies are:</p> <ul> <li> <p> <code>TERMINATE</code> – The child executions are terminated.</p> </li> <li> <p> <code>REQUEST_CANCEL</code> – A request to cancel is attempted for each child execution by recording a <code>WorkflowExecutionCancelRequested</code> event in its history. It is up to the decider to take appropriate actions when it receives an execution history with this event.</p> </li> <li> <p> <code>ABANDON</code> – No action is taken. The child executions continue to run.</p> </li> </ul> <note> <p>A child policy for this workflow execution must be specified either as a default for the workflow type or through this parameter. If neither this parameter is set nor a default child policy was specified at registration time then a fault is returned.</p> </note></p>
    #[serde(rename = "childPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_policy: Option<String>,
    /// <p>The name of the domain in which the workflow execution is created.</p>
    #[serde(rename = "domain")]
    pub domain: String,
    /// <p><p>The total duration for this workflow execution. This overrides the defaultExecutionStartToCloseTimeout specified when registering the workflow type.</p> <p>The duration is specified in seconds; an integer greater than or equal to <code>0</code>. Exceeding this limit causes the workflow execution to time out. Unlike some of the other timeout parameters in Amazon SWF, you cannot specify a value of &quot;NONE&quot; for this timeout; there is a one-year max limit on the time that a workflow execution can run.</p> <note> <p>An execution start-to-close timeout must be specified either through this parameter or as a default when the workflow type is registered. If neither this parameter nor a default execution start-to-close timeout is specified, a fault is returned.</p> </note></p>
    #[serde(rename = "executionStartToCloseTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_start_to_close_timeout: Option<String>,
    /// <p>The input for the workflow execution. This is a free form string which should be meaningful to the workflow you are starting. This <code>input</code> is made available to the new workflow execution in the <code>WorkflowExecutionStarted</code> history event.</p>
    #[serde(rename = "input")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    /// <p><p>The IAM role to attach to this workflow execution.</p> <note> <p>Executions of this workflow type need IAM roles to invoke Lambda functions. If you don&#39;t attach an IAM role, any attempt to schedule a Lambda task fails. This results in a <code>ScheduleLambdaFunctionFailed</code> history event. For more information, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/lambda-task.html">http://docs.aws.amazon.com/amazonswf/latest/developerguide/lambda-task.html</a> in the <i>Amazon SWF Developer Guide</i>.</p> </note></p>
    #[serde(rename = "lambdaRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_role: Option<String>,
    /// <p>The list of tags to associate with the workflow execution. You can specify a maximum of 5 tags. You can list workflow executions with a specific tag by calling <a>ListOpenWorkflowExecutions</a> or <a>ListClosedWorkflowExecutions</a> and specifying a <a>TagFilter</a>.</p>
    #[serde(rename = "tagList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<String>>,
    /// <p>The task list to use for the decision tasks generated for this workflow execution. This overrides the <code>defaultTaskList</code> specified when registering the workflow type.</p> <note> <p>A task list for this workflow execution must be specified either as a default for the workflow type or through this parameter. If neither this parameter is set nor a default task list was specified at registration time then a fault is returned.</p> </note> <p>The specified string must not start or end with whitespace. It must not contain a <code>:</code> (colon), <code>/</code> (slash), <code>|</code> (vertical bar), or any control characters (<code>\u0000-\u001f</code> | <code>\u007f-\u009f</code>). Also, it must not contain the literal string <code>arn</code>.</p>
    #[serde(rename = "taskList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_list: Option<TaskList>,
    /// <p>The task priority to use for this workflow execution. This overrides any default priority that was assigned when the workflow type was registered. If not set, then the default task priority for the workflow type is used. Valid values are integers that range from Java's <code>Integer.MIN_VALUE</code> (-2147483648) to <code>Integer.MAX_VALUE</code> (2147483647). Higher numbers indicate higher priority.</p> <p>For more information about setting task priority, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/programming-priority.html">Setting Task Priority</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    #[serde(rename = "taskPriority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_priority: Option<String>,
    /// <p><p>Specifies the maximum duration of decision tasks for this workflow execution. This parameter overrides the <code>defaultTaskStartToCloseTimout</code> specified when registering the workflow type using <a>RegisterWorkflowType</a>.</p> <p>The duration is specified in seconds, an integer greater than or equal to <code>0</code>. You can use <code>NONE</code> to specify unlimited duration.</p> <note> <p>A task start-to-close timeout for this workflow execution must be specified either as a default for the workflow type or through this parameter. If neither this parameter is set nor a default task start-to-close timeout was specified at registration time then a fault is returned.</p> </note></p>
    #[serde(rename = "taskStartToCloseTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_start_to_close_timeout: Option<String>,
    /// <p>The user defined identifier associated with the workflow execution. You can use this to associate a custom identifier with the workflow execution. You may specify the same identifier if a workflow execution is logically a <i>restart</i> of a previous execution. You cannot have two open workflow executions with the same <code>workflowId</code> at the same time.</p> <p>The specified string must not start or end with whitespace. It must not contain a <code>:</code> (colon), <code>/</code> (slash), <code>|</code> (vertical bar), or any control characters (<code>\u0000-\u001f</code> | <code>\u007f-\u009f</code>). Also, it must not contain the literal string <code>arn</code>.</p>
    #[serde(rename = "workflowId")]
    pub workflow_id: String,
    /// <p>The type of the workflow to start.</p>
    #[serde(rename = "workflowType")]
    pub workflow_type: WorkflowType,
}

/// <p>Used to filter the workflow executions in visibility APIs based on a tag.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagFilter {
    /// <p> Specifies the tag that must be associated with the execution for it to meet the filter criteria.</p>
    #[serde(rename = "tag")]
    pub tag: String,
}

/// <p>Represents a task list.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskList {
    /// <p>The name of the task list.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TerminateWorkflowExecutionInput {
    /// <p><p>If set, specifies the policy to use for the child workflow executions of the workflow execution being terminated. This policy overrides the child policy specified for the workflow execution at registration time or when starting the execution.</p> <p>The supported child policies are:</p> <ul> <li> <p> <code>TERMINATE</code> – The child executions are terminated.</p> </li> <li> <p> <code>REQUEST_CANCEL</code> – A request to cancel is attempted for each child execution by recording a <code>WorkflowExecutionCancelRequested</code> event in its history. It is up to the decider to take appropriate actions when it receives an execution history with this event.</p> </li> <li> <p> <code>ABANDON</code> – No action is taken. The child executions continue to run.</p> </li> </ul> <note> <p>A child policy for this workflow execution must be specified either as a default for the workflow type or through this parameter. If neither this parameter is set nor a default child policy was specified at registration time then a fault is returned.</p> </note></p>
    #[serde(rename = "childPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_policy: Option<String>,
    /// <p> Details for terminating the workflow execution.</p>
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// <p>The domain of the workflow execution to terminate.</p>
    #[serde(rename = "domain")]
    pub domain: String,
    /// <p> A descriptive reason for terminating the workflow execution.</p>
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// <p>The runId of the workflow execution to terminate.</p>
    #[serde(rename = "runId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    /// <p>The workflowId of the workflow execution to terminate.</p>
    #[serde(rename = "workflowId")]
    pub workflow_id: String,
}

/// <p> Provides the details of the <code>TimerCanceled</code> event. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TimerCanceledEventAttributes {
    /// <p>The ID of the <code>DecisionTaskCompleted</code> event corresponding to the decision task that resulted in the <code>CancelTimer</code> decision to cancel this timer. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "decisionTaskCompletedEventId")]
    pub decision_task_completed_event_id: i64,
    /// <p>The ID of the <code>TimerStarted</code> event that was recorded when this timer was started. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "startedEventId")]
    pub started_event_id: i64,
    /// <p>The unique ID of the timer that was canceled.</p>
    #[serde(rename = "timerId")]
    pub timer_id: String,
}

/// <p>Provides the details of the <code>TimerFired</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TimerFiredEventAttributes {
    /// <p>The ID of the <code>TimerStarted</code> event that was recorded when this timer was started. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "startedEventId")]
    pub started_event_id: i64,
    /// <p>The unique ID of the timer that fired.</p>
    #[serde(rename = "timerId")]
    pub timer_id: String,
}

/// <p>Provides the details of the <code>TimerStarted</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TimerStartedEventAttributes {
    /// <p>Data attached to the event that can be used by the decider in subsequent workflow tasks.</p>
    #[serde(rename = "control")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<String>,
    /// <p>The ID of the <code>DecisionTaskCompleted</code> event corresponding to the decision task that resulted in the <code>StartTimer</code> decision for this activity task. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "decisionTaskCompletedEventId")]
    pub decision_task_completed_event_id: i64,
    /// <p>The duration of time after which the timer fires.</p> <p>The duration is specified in seconds, an integer greater than or equal to <code>0</code>.</p>
    #[serde(rename = "startToFireTimeout")]
    pub start_to_fire_timeout: String,
    /// <p>The unique ID of the timer that was started.</p>
    #[serde(rename = "timerId")]
    pub timer_id: String,
}

/// <p>Represents a workflow execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowExecution {
    /// <p>A system-generated unique identifier for the workflow execution.</p>
    #[serde(rename = "runId")]
    pub run_id: String,
    /// <p>The user defined identifier associated with the workflow execution.</p>
    #[serde(rename = "workflowId")]
    pub workflow_id: String,
}

/// <p>Provides the details of the <code>WorkflowExecutionCancelRequested</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct WorkflowExecutionCancelRequestedEventAttributes {
    /// <p>If set, indicates that the request to cancel the workflow execution was automatically generated, and specifies the cause. This happens if the parent workflow execution times out or is terminated, and the child policy is set to cancel child executions.</p>
    #[serde(rename = "cause")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    /// <p>The ID of the <code>RequestCancelExternalWorkflowExecutionInitiated</code> event corresponding to the <code>RequestCancelExternalWorkflowExecution</code> decision to cancel this workflow execution.The source event with this ID can be found in the history of the source workflow execution. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "externalInitiatedEventId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_initiated_event_id: Option<i64>,
    /// <p>The external workflow execution for which the cancellation was requested.</p>
    #[serde(rename = "externalWorkflowExecution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_workflow_execution: Option<WorkflowExecution>,
}

/// <p>Provides the details of the <code>WorkflowExecutionCanceled</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct WorkflowExecutionCanceledEventAttributes {
    /// <p>The ID of the <code>DecisionTaskCompleted</code> event corresponding to the decision task that resulted in the <code>CancelWorkflowExecution</code> decision for this cancellation request. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "decisionTaskCompletedEventId")]
    pub decision_task_completed_event_id: i64,
    /// <p>The details of the cancellation.</p>
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

/// <p>Provides the details of the <code>WorkflowExecutionCompleted</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct WorkflowExecutionCompletedEventAttributes {
    /// <p>The ID of the <code>DecisionTaskCompleted</code> event corresponding to the decision task that resulted in the <code>CompleteWorkflowExecution</code> decision to complete this execution. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "decisionTaskCompletedEventId")]
    pub decision_task_completed_event_id: i64,
    /// <p>The result produced by the workflow execution upon successful completion.</p>
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}

/// <p>The configuration settings for a workflow execution including timeout values, tasklist etc. These configuration settings are determined from the defaults specified when registering the workflow type and those specified when starting the workflow execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct WorkflowExecutionConfiguration {
    /// <p><p>The policy to use for the child workflow executions if this workflow execution is terminated, by calling the <a>TerminateWorkflowExecution</a> action explicitly or due to an expired timeout.</p> <p>The supported child policies are:</p> <ul> <li> <p> <code>TERMINATE</code> – The child executions are terminated.</p> </li> <li> <p> <code>REQUEST_CANCEL</code> – A request to cancel is attempted for each child execution by recording a <code>WorkflowExecutionCancelRequested</code> event in its history. It is up to the decider to take appropriate actions when it receives an execution history with this event.</p> </li> <li> <p> <code>ABANDON</code> – No action is taken. The child executions continue to run.</p> </li> </ul></p>
    #[serde(rename = "childPolicy")]
    pub child_policy: String,
    /// <p>The total duration for this workflow execution.</p> <p>The duration is specified in seconds, an integer greater than or equal to <code>0</code>. You can use <code>NONE</code> to specify unlimited duration.</p>
    #[serde(rename = "executionStartToCloseTimeout")]
    pub execution_start_to_close_timeout: String,
    /// <p>The IAM role attached to the child workflow execution.</p>
    #[serde(rename = "lambdaRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_role: Option<String>,
    /// <p>The task list used for the decision tasks generated for this workflow execution.</p>
    #[serde(rename = "taskList")]
    pub task_list: TaskList,
    /// <p>The priority assigned to decision tasks for this workflow execution. Valid values are integers that range from Java's <code>Integer.MIN_VALUE</code> (-2147483648) to <code>Integer.MAX_VALUE</code> (2147483647). Higher numbers indicate higher priority.</p> <p>For more information about setting task priority, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/programming-priority.html">Setting Task Priority</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    #[serde(rename = "taskPriority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_priority: Option<String>,
    /// <p>The maximum duration allowed for decision tasks for this workflow execution.</p> <p>The duration is specified in seconds, an integer greater than or equal to <code>0</code>. You can use <code>NONE</code> to specify unlimited duration.</p>
    #[serde(rename = "taskStartToCloseTimeout")]
    pub task_start_to_close_timeout: String,
}

/// <p>Provides the details of the <code>WorkflowExecutionContinuedAsNew</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct WorkflowExecutionContinuedAsNewEventAttributes {
    /// <p><p>The policy to use for the child workflow executions of the new execution if it is terminated by calling the <a>TerminateWorkflowExecution</a> action explicitly or due to an expired timeout.</p> <p>The supported child policies are:</p> <ul> <li> <p> <code>TERMINATE</code> – The child executions are terminated.</p> </li> <li> <p> <code>REQUEST_CANCEL</code> – A request to cancel is attempted for each child execution by recording a <code>WorkflowExecutionCancelRequested</code> event in its history. It is up to the decider to take appropriate actions when it receives an execution history with this event.</p> </li> <li> <p> <code>ABANDON</code> – No action is taken. The child executions continue to run.</p> </li> </ul></p>
    #[serde(rename = "childPolicy")]
    pub child_policy: String,
    /// <p>The ID of the <code>DecisionTaskCompleted</code> event corresponding to the decision task that resulted in the <code>ContinueAsNewWorkflowExecution</code> decision that started this execution. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "decisionTaskCompletedEventId")]
    pub decision_task_completed_event_id: i64,
    /// <p>The total duration allowed for the new workflow execution.</p> <p>The duration is specified in seconds, an integer greater than or equal to <code>0</code>. You can use <code>NONE</code> to specify unlimited duration.</p>
    #[serde(rename = "executionStartToCloseTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_start_to_close_timeout: Option<String>,
    /// <p>The input provided to the new workflow execution.</p>
    #[serde(rename = "input")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    /// <p>The IAM role to attach to the new (continued) workflow execution.</p>
    #[serde(rename = "lambdaRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_role: Option<String>,
    /// <p>The <code>runId</code> of the new workflow execution.</p>
    #[serde(rename = "newExecutionRunId")]
    pub new_execution_run_id: String,
    /// <p>The list of tags associated with the new workflow execution.</p>
    #[serde(rename = "tagList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<String>>,
    /// <p>The task list to use for the decisions of the new (continued) workflow execution.</p>
    #[serde(rename = "taskList")]
    pub task_list: TaskList,
    /// <p>The priority of the task to use for the decisions of the new (continued) workflow execution.</p>
    #[serde(rename = "taskPriority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_priority: Option<String>,
    /// <p>The maximum duration of decision tasks for the new workflow execution.</p> <p>The duration is specified in seconds, an integer greater than or equal to <code>0</code>. You can use <code>NONE</code> to specify unlimited duration.</p>
    #[serde(rename = "taskStartToCloseTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_start_to_close_timeout: Option<String>,
    /// <p>The workflow type of this execution.</p>
    #[serde(rename = "workflowType")]
    pub workflow_type: WorkflowType,
}

/// <p>Contains the count of workflow executions returned from <a>CountOpenWorkflowExecutions</a> or <a>CountClosedWorkflowExecutions</a> </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct WorkflowExecutionCount {
    /// <p>The number of workflow executions.</p>
    #[serde(rename = "count")]
    pub count: i64,
    /// <p>If set to true, indicates that the actual count was more than the maximum supported by this API and the count returned is the truncated value.</p>
    #[serde(rename = "truncated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

/// <p>Contains details about a workflow execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct WorkflowExecutionDetail {
    /// <p>The configuration settings for this workflow execution including timeout values, tasklist etc.</p>
    #[serde(rename = "executionConfiguration")]
    pub execution_configuration: WorkflowExecutionConfiguration,
    /// <p>Information about the workflow execution.</p>
    #[serde(rename = "executionInfo")]
    pub execution_info: WorkflowExecutionInfo,
    /// <p>The time when the last activity task was scheduled for this workflow execution. You can use this information to determine if the workflow has not made progress for an unusually long period of time and might require a corrective action.</p>
    #[serde(rename = "latestActivityTaskTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_activity_task_timestamp: Option<f64>,
    /// <p>The latest executionContext provided by the decider for this workflow execution. A decider can provide an executionContext (a free-form string) when closing a decision task using <a>RespondDecisionTaskCompleted</a>.</p>
    #[serde(rename = "latestExecutionContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_execution_context: Option<String>,
    /// <p>The number of tasks for this workflow execution. This includes open and closed tasks of all types.</p>
    #[serde(rename = "openCounts")]
    pub open_counts: WorkflowExecutionOpenCounts,
}

/// <p>Provides the details of the <code>WorkflowExecutionFailed</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct WorkflowExecutionFailedEventAttributes {
    /// <p>The ID of the <code>DecisionTaskCompleted</code> event corresponding to the decision task that resulted in the <code>FailWorkflowExecution</code> decision to fail this execution. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "decisionTaskCompletedEventId")]
    pub decision_task_completed_event_id: i64,
    /// <p>The details of the failure.</p>
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// <p>The descriptive reason provided for the failure.</p>
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// <p>Used to filter the workflow executions in visibility APIs by their <code>workflowId</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct WorkflowExecutionFilter {
    /// <p>The workflowId to pass of match the criteria of this filter.</p>
    #[serde(rename = "workflowId")]
    pub workflow_id: String,
}

/// <p>Contains information about a workflow execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct WorkflowExecutionInfo {
    /// <p>Set to true if a cancellation is requested for this workflow execution.</p>
    #[serde(rename = "cancelRequested")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_requested: Option<bool>,
    /// <p><p>If the execution status is closed then this specifies how the execution was closed:</p> <ul> <li> <p> <code>COMPLETED</code> – the execution was successfully completed.</p> </li> <li> <p> <code>CANCELED</code> – the execution was canceled.Cancellation allows the implementation to gracefully clean up before the execution is closed.</p> </li> <li> <p> <code>TERMINATED</code> – the execution was force terminated.</p> </li> <li> <p> <code>FAILED</code> – the execution failed to complete.</p> </li> <li> <p> <code>TIMED<em>OUT</code> – the execution did not complete in the alloted time and was automatically timed out.</p> </li> <li> <p> <code>CONTINUED</em>AS_NEW</code> – the execution is logically continued. This means the current execution was completed and a new execution was started to carry on the workflow.</p> </li> </ul></p>
    #[serde(rename = "closeStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_status: Option<String>,
    /// <p>The time when the workflow execution was closed. Set only if the execution status is CLOSED.</p>
    #[serde(rename = "closeTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_timestamp: Option<f64>,
    /// <p>The workflow execution this information is about.</p>
    #[serde(rename = "execution")]
    pub execution: WorkflowExecution,
    /// <p>The current status of the execution.</p>
    #[serde(rename = "executionStatus")]
    pub execution_status: String,
    /// <p>If this workflow execution is a child of another execution then contains the workflow execution that started this execution.</p>
    #[serde(rename = "parent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<WorkflowExecution>,
    /// <p>The time when the execution was started.</p>
    #[serde(rename = "startTimestamp")]
    pub start_timestamp: f64,
    /// <p>The list of tags associated with the workflow execution. Tags can be used to identify and list workflow executions of interest through the visibility APIs. A workflow execution can have a maximum of 5 tags.</p>
    #[serde(rename = "tagList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<String>>,
    /// <p>The type of the workflow execution.</p>
    #[serde(rename = "workflowType")]
    pub workflow_type: WorkflowType,
}

/// <p>Contains a paginated list of information about workflow executions.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct WorkflowExecutionInfos {
    /// <p>The list of workflow information structures.</p>
    #[serde(rename = "executionInfos")]
    pub execution_infos: Vec<WorkflowExecutionInfo>,
    /// <p>If a <code>NextPageToken</code> was returned by a previous call, there are more results available. To retrieve the next page of results, make the call again using the returned token in <code>nextPageToken</code>. Keep all other arguments unchanged.</p> <p>The configured <code>maximumPageSize</code> determines how many results can be returned in a single call.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

/// <p>Contains the counts of open tasks, child workflow executions and timers for a workflow execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct WorkflowExecutionOpenCounts {
    /// <p>The count of activity tasks whose status is <code>OPEN</code>.</p>
    #[serde(rename = "openActivityTasks")]
    pub open_activity_tasks: i64,
    /// <p>The count of child workflow executions whose status is <code>OPEN</code>.</p>
    #[serde(rename = "openChildWorkflowExecutions")]
    pub open_child_workflow_executions: i64,
    /// <p>The count of decision tasks whose status is OPEN. A workflow execution can have at most one open decision task.</p>
    #[serde(rename = "openDecisionTasks")]
    pub open_decision_tasks: i64,
    /// <p>The count of Lambda tasks whose status is <code>OPEN</code>.</p>
    #[serde(rename = "openLambdaFunctions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_lambda_functions: Option<i64>,
    /// <p>The count of timers started by this workflow execution that have not fired yet.</p>
    #[serde(rename = "openTimers")]
    pub open_timers: i64,
}

/// <p>Provides the details of the <code>WorkflowExecutionSignaled</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct WorkflowExecutionSignaledEventAttributes {
    /// <p>The ID of the <code>SignalExternalWorkflowExecutionInitiated</code> event corresponding to the <code>SignalExternalWorkflow</code> decision to signal this workflow execution.The source event with this ID can be found in the history of the source workflow execution. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event. This field is set only if the signal was initiated by another workflow execution.</p>
    #[serde(rename = "externalInitiatedEventId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_initiated_event_id: Option<i64>,
    /// <p>The workflow execution that sent the signal. This is set only of the signal was sent by another workflow execution.</p>
    #[serde(rename = "externalWorkflowExecution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_workflow_execution: Option<WorkflowExecution>,
    /// <p>The inputs provided with the signal. The decider can use the signal name and inputs to determine how to process the signal.</p>
    #[serde(rename = "input")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    /// <p>The name of the signal received. The decider can use the signal name and inputs to determine how to the process the signal.</p>
    #[serde(rename = "signalName")]
    pub signal_name: String,
}

/// <p>Provides details of <code>WorkflowExecutionStarted</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct WorkflowExecutionStartedEventAttributes {
    /// <p><p>The policy to use for the child workflow executions if this workflow execution is terminated, by calling the <a>TerminateWorkflowExecution</a> action explicitly or due to an expired timeout.</p> <p>The supported child policies are:</p> <ul> <li> <p> <code>TERMINATE</code> – The child executions are terminated.</p> </li> <li> <p> <code>REQUEST_CANCEL</code> – A request to cancel is attempted for each child execution by recording a <code>WorkflowExecutionCancelRequested</code> event in its history. It is up to the decider to take appropriate actions when it receives an execution history with this event.</p> </li> <li> <p> <code>ABANDON</code> – No action is taken. The child executions continue to run.</p> </li> </ul></p>
    #[serde(rename = "childPolicy")]
    pub child_policy: String,
    /// <p>If this workflow execution was started due to a <code>ContinueAsNewWorkflowExecution</code> decision, then it contains the <code>runId</code> of the previous workflow execution that was closed and continued as this execution.</p>
    #[serde(rename = "continuedExecutionRunId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continued_execution_run_id: Option<String>,
    /// <p>The maximum duration for this workflow execution.</p> <p>The duration is specified in seconds, an integer greater than or equal to <code>0</code>. You can use <code>NONE</code> to specify unlimited duration.</p>
    #[serde(rename = "executionStartToCloseTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_start_to_close_timeout: Option<String>,
    /// <p>The input provided to the workflow execution.</p>
    #[serde(rename = "input")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    /// <p>The IAM role attached to the workflow execution.</p>
    #[serde(rename = "lambdaRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_role: Option<String>,
    /// <p>The ID of the <code>StartChildWorkflowExecutionInitiated</code> event corresponding to the <code>StartChildWorkflowExecution</code> <a>Decision</a> to start this workflow execution. The source event with this ID can be found in the history of the source workflow execution. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "parentInitiatedEventId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_initiated_event_id: Option<i64>,
    /// <p>The source workflow execution that started this workflow execution. The member isn't set if the workflow execution was not started by a workflow.</p>
    #[serde(rename = "parentWorkflowExecution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_workflow_execution: Option<WorkflowExecution>,
    /// <p>The list of tags associated with this workflow execution. An execution can have up to 5 tags.</p>
    #[serde(rename = "tagList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<String>>,
    /// <p>The name of the task list for scheduling the decision tasks for this workflow execution.</p>
    #[serde(rename = "taskList")]
    pub task_list: TaskList,
    /// <p>The priority of the decision tasks in the workflow execution.</p>
    #[serde(rename = "taskPriority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_priority: Option<String>,
    /// <p>The maximum duration of decision tasks for this workflow type.</p> <p>The duration is specified in seconds, an integer greater than or equal to <code>0</code>. You can use <code>NONE</code> to specify unlimited duration.</p>
    #[serde(rename = "taskStartToCloseTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_start_to_close_timeout: Option<String>,
    /// <p>The workflow type of this execution.</p>
    #[serde(rename = "workflowType")]
    pub workflow_type: WorkflowType,
}

/// <p>Provides the details of the <code>WorkflowExecutionTerminated</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct WorkflowExecutionTerminatedEventAttributes {
    /// <p>If set, indicates that the workflow execution was automatically terminated, and specifies the cause. This happens if the parent workflow execution times out or is terminated and the child policy is set to terminate child executions.</p>
    #[serde(rename = "cause")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    /// <p><p>The policy used for the child workflow executions of this workflow execution.</p> <p>The supported child policies are:</p> <ul> <li> <p> <code>TERMINATE</code> – The child executions are terminated.</p> </li> <li> <p> <code>REQUEST_CANCEL</code> – A request to cancel is attempted for each child execution by recording a <code>WorkflowExecutionCancelRequested</code> event in its history. It is up to the decider to take appropriate actions when it receives an execution history with this event.</p> </li> <li> <p> <code>ABANDON</code> – No action is taken. The child executions continue to run.</p> </li> </ul></p>
    #[serde(rename = "childPolicy")]
    pub child_policy: String,
    /// <p>The details provided for the termination.</p>
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// <p>The reason provided for the termination.</p>
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// <p>Provides the details of the <code>WorkflowExecutionTimedOut</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct WorkflowExecutionTimedOutEventAttributes {
    /// <p><p>The policy used for the child workflow executions of this workflow execution.</p> <p>The supported child policies are:</p> <ul> <li> <p> <code>TERMINATE</code> – The child executions are terminated.</p> </li> <li> <p> <code>REQUEST_CANCEL</code> – A request to cancel is attempted for each child execution by recording a <code>WorkflowExecutionCancelRequested</code> event in its history. It is up to the decider to take appropriate actions when it receives an execution history with this event.</p> </li> <li> <p> <code>ABANDON</code> – No action is taken. The child executions continue to run.</p> </li> </ul></p>
    #[serde(rename = "childPolicy")]
    pub child_policy: String,
    /// <p>The type of timeout that caused this event.</p>
    #[serde(rename = "timeoutType")]
    pub timeout_type: String,
}

/// <p>Represents a workflow type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowType {
    /// <p><p> The name of the workflow type.</p> <note> <p>The combination of workflow type name and version must be unique with in a domain.</p> </note></p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p><p> The version of the workflow type.</p> <note> <p>The combination of workflow type name and version must be unique with in a domain.</p> </note></p>
    #[serde(rename = "version")]
    pub version: String,
}

/// <p>The configuration settings of a workflow type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct WorkflowTypeConfiguration {
    /// <p><p> The default policy to use for the child workflow executions when a workflow execution of this type is terminated, by calling the <a>TerminateWorkflowExecution</a> action explicitly or due to an expired timeout. This default can be overridden when starting a workflow execution using the <a>StartWorkflowExecution</a> action or the <code>StartChildWorkflowExecution</code> <a>Decision</a>.</p> <p>The supported child policies are:</p> <ul> <li> <p> <code>TERMINATE</code> – The child executions are terminated.</p> </li> <li> <p> <code>REQUEST_CANCEL</code> – A request to cancel is attempted for each child execution by recording a <code>WorkflowExecutionCancelRequested</code> event in its history. It is up to the decider to take appropriate actions when it receives an execution history with this event.</p> </li> <li> <p> <code>ABANDON</code> – No action is taken. The child executions continue to run.</p> </li> </ul></p>
    #[serde(rename = "defaultChildPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_child_policy: Option<String>,
    /// <p> The default maximum duration, specified when registering the workflow type, for executions of this workflow type. This default can be overridden when starting a workflow execution using the <a>StartWorkflowExecution</a> action or the <code>StartChildWorkflowExecution</code> <a>Decision</a>.</p> <p>The duration is specified in seconds, an integer greater than or equal to <code>0</code>. You can use <code>NONE</code> to specify unlimited duration.</p>
    #[serde(rename = "defaultExecutionStartToCloseTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_execution_start_to_close_timeout: Option<String>,
    /// <p><p>The default IAM role attached to this workflow type.</p> <note> <p>Executions of this workflow type need IAM roles to invoke Lambda functions. If you don&#39;t specify an IAM role when starting this workflow type, the default Lambda role is attached to the execution. For more information, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/lambda-task.html">http://docs.aws.amazon.com/amazonswf/latest/developerguide/lambda-task.html</a> in the <i>Amazon SWF Developer Guide</i>.</p> </note></p>
    #[serde(rename = "defaultLambdaRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_lambda_role: Option<String>,
    /// <p> The default task list, specified when registering the workflow type, for decisions tasks scheduled for workflow executions of this type. This default can be overridden when starting a workflow execution using the <a>StartWorkflowExecution</a> action or the <code>StartChildWorkflowExecution</code> <a>Decision</a>.</p>
    #[serde(rename = "defaultTaskList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_task_list: Option<TaskList>,
    /// <p> The default task priority, specified when registering the workflow type, for all decision tasks of this workflow type. This default can be overridden when starting a workflow execution using the <a>StartWorkflowExecution</a> action or the <code>StartChildWorkflowExecution</code> decision.</p> <p>Valid values are integers that range from Java's <code>Integer.MIN_VALUE</code> (-2147483648) to <code>Integer.MAX_VALUE</code> (2147483647). Higher numbers indicate higher priority.</p> <p>For more information about setting task priority, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/programming-priority.html">Setting Task Priority</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    #[serde(rename = "defaultTaskPriority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_task_priority: Option<String>,
    /// <p> The default maximum duration, specified when registering the workflow type, that a decision task for executions of this workflow type might take before returning completion or failure. If the task doesn'tdo close in the specified time then the task is automatically timed out and rescheduled. If the decider eventually reports a completion or failure, it is ignored. This default can be overridden when starting a workflow execution using the <a>StartWorkflowExecution</a> action or the <code>StartChildWorkflowExecution</code> <a>Decision</a>.</p> <p>The duration is specified in seconds, an integer greater than or equal to <code>0</code>. You can use <code>NONE</code> to specify unlimited duration.</p>
    #[serde(rename = "defaultTaskStartToCloseTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_task_start_to_close_timeout: Option<String>,
}

/// <p>Contains details about a workflow type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct WorkflowTypeDetail {
    /// <p>Configuration settings of the workflow type registered through <a>RegisterWorkflowType</a> </p>
    #[serde(rename = "configuration")]
    pub configuration: WorkflowTypeConfiguration,
    /// <p><p>General information about the workflow type.</p> <p>The status of the workflow type (returned in the WorkflowTypeInfo structure) can be one of the following.</p> <ul> <li> <p> <code>REGISTERED</code> – The type is registered and available. Workers supporting this type should be running.</p> </li> <li> <p> <code>DEPRECATED</code> – The type was deprecated using <a>DeprecateWorkflowType</a>, but is still in use. You should keep workers supporting this type running. You cannot create new workflow executions of this type.</p> </li> </ul></p>
    #[serde(rename = "typeInfo")]
    pub type_info: WorkflowTypeInfo,
}

/// <p>Used to filter workflow execution query results by type. Each parameter, if specified, defines a rule that must be satisfied by each returned result.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct WorkflowTypeFilter {
    /// <p> Name of the workflow type.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>Version of the workflow type.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Contains information about a workflow type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct WorkflowTypeInfo {
    /// <p>The date when this type was registered.</p>
    #[serde(rename = "creationDate")]
    pub creation_date: f64,
    /// <p>If the type is in deprecated state, then it is set to the date when the type was deprecated.</p>
    #[serde(rename = "deprecationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_date: Option<f64>,
    /// <p>The description of the type registered through <a>RegisterWorkflowType</a>.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The current status of the workflow type.</p>
    #[serde(rename = "status")]
    pub status: String,
    /// <p>The workflow type this information is about.</p>
    #[serde(rename = "workflowType")]
    pub workflow_type: WorkflowType,
}

/// <p>Contains a paginated list of information structures about workflow types.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct WorkflowTypeInfos {
    /// <p>If a <code>NextPageToken</code> was returned by a previous call, there are more results available. To retrieve the next page of results, make the call again using the returned token in <code>nextPageToken</code>. Keep all other arguments unchanged.</p> <p>The configured <code>maximumPageSize</code> determines how many results can be returned in a single call.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>The list of workflow type information.</p>
    #[serde(rename = "typeInfos")]
    pub type_infos: Vec<WorkflowTypeInfo>,
}

/// Errors returned by CountClosedWorkflowExecutions
#[derive(Debug, PartialEq)]
pub enum CountClosedWorkflowExecutionsError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CountClosedWorkflowExecutionsError {
    pub fn from_body(body: &str) -> CountClosedWorkflowExecutionsError {
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
                    "OperationNotPermittedFault" => {
                        CountClosedWorkflowExecutionsError::OperationNotPermittedFault(
                            String::from(error_message),
                        )
                    }
                    "UnknownResourceFault" => {
                        CountClosedWorkflowExecutionsError::UnknownResourceFault(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        CountClosedWorkflowExecutionsError::Validation(error_message.to_string())
                    }
                    _ => CountClosedWorkflowExecutionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => CountClosedWorkflowExecutionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CountClosedWorkflowExecutionsError {
    fn from(err: serde_json::error::Error) -> CountClosedWorkflowExecutionsError {
        CountClosedWorkflowExecutionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CountClosedWorkflowExecutionsError {
    fn from(err: CredentialsError) -> CountClosedWorkflowExecutionsError {
        CountClosedWorkflowExecutionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CountClosedWorkflowExecutionsError {
    fn from(err: HttpDispatchError) -> CountClosedWorkflowExecutionsError {
        CountClosedWorkflowExecutionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for CountClosedWorkflowExecutionsError {
    fn from(err: io::Error) -> CountClosedWorkflowExecutionsError {
        CountClosedWorkflowExecutionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CountClosedWorkflowExecutionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CountClosedWorkflowExecutionsError {
    fn description(&self) -> &str {
        match *self {
            CountClosedWorkflowExecutionsError::OperationNotPermittedFault(ref cause) => cause,
            CountClosedWorkflowExecutionsError::UnknownResourceFault(ref cause) => cause,
            CountClosedWorkflowExecutionsError::Validation(ref cause) => cause,
            CountClosedWorkflowExecutionsError::Credentials(ref err) => err.description(),
            CountClosedWorkflowExecutionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CountClosedWorkflowExecutionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CountOpenWorkflowExecutions
#[derive(Debug, PartialEq)]
pub enum CountOpenWorkflowExecutionsError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CountOpenWorkflowExecutionsError {
    pub fn from_body(body: &str) -> CountOpenWorkflowExecutionsError {
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
                    "OperationNotPermittedFault" => {
                        CountOpenWorkflowExecutionsError::OperationNotPermittedFault(String::from(
                            error_message,
                        ))
                    }
                    "UnknownResourceFault" => {
                        CountOpenWorkflowExecutionsError::UnknownResourceFault(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        CountOpenWorkflowExecutionsError::Validation(error_message.to_string())
                    }
                    _ => CountOpenWorkflowExecutionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => CountOpenWorkflowExecutionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CountOpenWorkflowExecutionsError {
    fn from(err: serde_json::error::Error) -> CountOpenWorkflowExecutionsError {
        CountOpenWorkflowExecutionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CountOpenWorkflowExecutionsError {
    fn from(err: CredentialsError) -> CountOpenWorkflowExecutionsError {
        CountOpenWorkflowExecutionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CountOpenWorkflowExecutionsError {
    fn from(err: HttpDispatchError) -> CountOpenWorkflowExecutionsError {
        CountOpenWorkflowExecutionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for CountOpenWorkflowExecutionsError {
    fn from(err: io::Error) -> CountOpenWorkflowExecutionsError {
        CountOpenWorkflowExecutionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CountOpenWorkflowExecutionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CountOpenWorkflowExecutionsError {
    fn description(&self) -> &str {
        match *self {
            CountOpenWorkflowExecutionsError::OperationNotPermittedFault(ref cause) => cause,
            CountOpenWorkflowExecutionsError::UnknownResourceFault(ref cause) => cause,
            CountOpenWorkflowExecutionsError::Validation(ref cause) => cause,
            CountOpenWorkflowExecutionsError::Credentials(ref err) => err.description(),
            CountOpenWorkflowExecutionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CountOpenWorkflowExecutionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CountPendingActivityTasks
#[derive(Debug, PartialEq)]
pub enum CountPendingActivityTasksError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CountPendingActivityTasksError {
    pub fn from_body(body: &str) -> CountPendingActivityTasksError {
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
                    "OperationNotPermittedFault" => {
                        CountPendingActivityTasksError::OperationNotPermittedFault(String::from(
                            error_message,
                        ))
                    }
                    "UnknownResourceFault" => CountPendingActivityTasksError::UnknownResourceFault(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        CountPendingActivityTasksError::Validation(error_message.to_string())
                    }
                    _ => CountPendingActivityTasksError::Unknown(String::from(body)),
                }
            }
            Err(_) => CountPendingActivityTasksError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CountPendingActivityTasksError {
    fn from(err: serde_json::error::Error) -> CountPendingActivityTasksError {
        CountPendingActivityTasksError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CountPendingActivityTasksError {
    fn from(err: CredentialsError) -> CountPendingActivityTasksError {
        CountPendingActivityTasksError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CountPendingActivityTasksError {
    fn from(err: HttpDispatchError) -> CountPendingActivityTasksError {
        CountPendingActivityTasksError::HttpDispatch(err)
    }
}
impl From<io::Error> for CountPendingActivityTasksError {
    fn from(err: io::Error) -> CountPendingActivityTasksError {
        CountPendingActivityTasksError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CountPendingActivityTasksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CountPendingActivityTasksError {
    fn description(&self) -> &str {
        match *self {
            CountPendingActivityTasksError::OperationNotPermittedFault(ref cause) => cause,
            CountPendingActivityTasksError::UnknownResourceFault(ref cause) => cause,
            CountPendingActivityTasksError::Validation(ref cause) => cause,
            CountPendingActivityTasksError::Credentials(ref err) => err.description(),
            CountPendingActivityTasksError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CountPendingActivityTasksError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CountPendingDecisionTasks
#[derive(Debug, PartialEq)]
pub enum CountPendingDecisionTasksError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CountPendingDecisionTasksError {
    pub fn from_body(body: &str) -> CountPendingDecisionTasksError {
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
                    "OperationNotPermittedFault" => {
                        CountPendingDecisionTasksError::OperationNotPermittedFault(String::from(
                            error_message,
                        ))
                    }
                    "UnknownResourceFault" => CountPendingDecisionTasksError::UnknownResourceFault(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        CountPendingDecisionTasksError::Validation(error_message.to_string())
                    }
                    _ => CountPendingDecisionTasksError::Unknown(String::from(body)),
                }
            }
            Err(_) => CountPendingDecisionTasksError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CountPendingDecisionTasksError {
    fn from(err: serde_json::error::Error) -> CountPendingDecisionTasksError {
        CountPendingDecisionTasksError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CountPendingDecisionTasksError {
    fn from(err: CredentialsError) -> CountPendingDecisionTasksError {
        CountPendingDecisionTasksError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CountPendingDecisionTasksError {
    fn from(err: HttpDispatchError) -> CountPendingDecisionTasksError {
        CountPendingDecisionTasksError::HttpDispatch(err)
    }
}
impl From<io::Error> for CountPendingDecisionTasksError {
    fn from(err: io::Error) -> CountPendingDecisionTasksError {
        CountPendingDecisionTasksError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CountPendingDecisionTasksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CountPendingDecisionTasksError {
    fn description(&self) -> &str {
        match *self {
            CountPendingDecisionTasksError::OperationNotPermittedFault(ref cause) => cause,
            CountPendingDecisionTasksError::UnknownResourceFault(ref cause) => cause,
            CountPendingDecisionTasksError::Validation(ref cause) => cause,
            CountPendingDecisionTasksError::Credentials(ref err) => err.description(),
            CountPendingDecisionTasksError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CountPendingDecisionTasksError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeprecateActivityType
#[derive(Debug, PartialEq)]
pub enum DeprecateActivityTypeError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the specified activity or workflow type was already deprecated.</p>
    TypeDeprecatedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeprecateActivityTypeError {
    pub fn from_body(body: &str) -> DeprecateActivityTypeError {
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
                    "OperationNotPermittedFault" => {
                        DeprecateActivityTypeError::OperationNotPermittedFault(String::from(
                            error_message,
                        ))
                    }
                    "TypeDeprecatedFault" => {
                        DeprecateActivityTypeError::TypeDeprecatedFault(String::from(error_message))
                    }
                    "UnknownResourceFault" => DeprecateActivityTypeError::UnknownResourceFault(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        DeprecateActivityTypeError::Validation(error_message.to_string())
                    }
                    _ => DeprecateActivityTypeError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeprecateActivityTypeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeprecateActivityTypeError {
    fn from(err: serde_json::error::Error) -> DeprecateActivityTypeError {
        DeprecateActivityTypeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeprecateActivityTypeError {
    fn from(err: CredentialsError) -> DeprecateActivityTypeError {
        DeprecateActivityTypeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeprecateActivityTypeError {
    fn from(err: HttpDispatchError) -> DeprecateActivityTypeError {
        DeprecateActivityTypeError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeprecateActivityTypeError {
    fn from(err: io::Error) -> DeprecateActivityTypeError {
        DeprecateActivityTypeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeprecateActivityTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeprecateActivityTypeError {
    fn description(&self) -> &str {
        match *self {
            DeprecateActivityTypeError::OperationNotPermittedFault(ref cause) => cause,
            DeprecateActivityTypeError::TypeDeprecatedFault(ref cause) => cause,
            DeprecateActivityTypeError::UnknownResourceFault(ref cause) => cause,
            DeprecateActivityTypeError::Validation(ref cause) => cause,
            DeprecateActivityTypeError::Credentials(ref err) => err.description(),
            DeprecateActivityTypeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeprecateActivityTypeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeprecateDomain
#[derive(Debug, PartialEq)]
pub enum DeprecateDomainError {
    /// <p>Returned when the specified domain has been deprecated.</p>
    DomainDeprecatedFault(String),
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeprecateDomainError {
    pub fn from_body(body: &str) -> DeprecateDomainError {
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
                    "DomainDeprecatedFault" => {
                        DeprecateDomainError::DomainDeprecatedFault(String::from(error_message))
                    }
                    "OperationNotPermittedFault" => {
                        DeprecateDomainError::OperationNotPermittedFault(String::from(
                            error_message,
                        ))
                    }
                    "UnknownResourceFault" => {
                        DeprecateDomainError::UnknownResourceFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeprecateDomainError::Validation(error_message.to_string())
                    }
                    _ => DeprecateDomainError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeprecateDomainError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeprecateDomainError {
    fn from(err: serde_json::error::Error) -> DeprecateDomainError {
        DeprecateDomainError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeprecateDomainError {
    fn from(err: CredentialsError) -> DeprecateDomainError {
        DeprecateDomainError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeprecateDomainError {
    fn from(err: HttpDispatchError) -> DeprecateDomainError {
        DeprecateDomainError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeprecateDomainError {
    fn from(err: io::Error) -> DeprecateDomainError {
        DeprecateDomainError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeprecateDomainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeprecateDomainError {
    fn description(&self) -> &str {
        match *self {
            DeprecateDomainError::DomainDeprecatedFault(ref cause) => cause,
            DeprecateDomainError::OperationNotPermittedFault(ref cause) => cause,
            DeprecateDomainError::UnknownResourceFault(ref cause) => cause,
            DeprecateDomainError::Validation(ref cause) => cause,
            DeprecateDomainError::Credentials(ref err) => err.description(),
            DeprecateDomainError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeprecateDomainError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeprecateWorkflowType
#[derive(Debug, PartialEq)]
pub enum DeprecateWorkflowTypeError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the specified activity or workflow type was already deprecated.</p>
    TypeDeprecatedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeprecateWorkflowTypeError {
    pub fn from_body(body: &str) -> DeprecateWorkflowTypeError {
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
                    "OperationNotPermittedFault" => {
                        DeprecateWorkflowTypeError::OperationNotPermittedFault(String::from(
                            error_message,
                        ))
                    }
                    "TypeDeprecatedFault" => {
                        DeprecateWorkflowTypeError::TypeDeprecatedFault(String::from(error_message))
                    }
                    "UnknownResourceFault" => DeprecateWorkflowTypeError::UnknownResourceFault(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        DeprecateWorkflowTypeError::Validation(error_message.to_string())
                    }
                    _ => DeprecateWorkflowTypeError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeprecateWorkflowTypeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeprecateWorkflowTypeError {
    fn from(err: serde_json::error::Error) -> DeprecateWorkflowTypeError {
        DeprecateWorkflowTypeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeprecateWorkflowTypeError {
    fn from(err: CredentialsError) -> DeprecateWorkflowTypeError {
        DeprecateWorkflowTypeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeprecateWorkflowTypeError {
    fn from(err: HttpDispatchError) -> DeprecateWorkflowTypeError {
        DeprecateWorkflowTypeError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeprecateWorkflowTypeError {
    fn from(err: io::Error) -> DeprecateWorkflowTypeError {
        DeprecateWorkflowTypeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeprecateWorkflowTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeprecateWorkflowTypeError {
    fn description(&self) -> &str {
        match *self {
            DeprecateWorkflowTypeError::OperationNotPermittedFault(ref cause) => cause,
            DeprecateWorkflowTypeError::TypeDeprecatedFault(ref cause) => cause,
            DeprecateWorkflowTypeError::UnknownResourceFault(ref cause) => cause,
            DeprecateWorkflowTypeError::Validation(ref cause) => cause,
            DeprecateWorkflowTypeError::Credentials(ref err) => err.description(),
            DeprecateWorkflowTypeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeprecateWorkflowTypeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeActivityType
#[derive(Debug, PartialEq)]
pub enum DescribeActivityTypeError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeActivityTypeError {
    pub fn from_body(body: &str) -> DescribeActivityTypeError {
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
                    "OperationNotPermittedFault" => {
                        DescribeActivityTypeError::OperationNotPermittedFault(String::from(
                            error_message,
                        ))
                    }
                    "UnknownResourceFault" => {
                        DescribeActivityTypeError::UnknownResourceFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeActivityTypeError::Validation(error_message.to_string())
                    }
                    _ => DescribeActivityTypeError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeActivityTypeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeActivityTypeError {
    fn from(err: serde_json::error::Error) -> DescribeActivityTypeError {
        DescribeActivityTypeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeActivityTypeError {
    fn from(err: CredentialsError) -> DescribeActivityTypeError {
        DescribeActivityTypeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeActivityTypeError {
    fn from(err: HttpDispatchError) -> DescribeActivityTypeError {
        DescribeActivityTypeError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeActivityTypeError {
    fn from(err: io::Error) -> DescribeActivityTypeError {
        DescribeActivityTypeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeActivityTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeActivityTypeError {
    fn description(&self) -> &str {
        match *self {
            DescribeActivityTypeError::OperationNotPermittedFault(ref cause) => cause,
            DescribeActivityTypeError::UnknownResourceFault(ref cause) => cause,
            DescribeActivityTypeError::Validation(ref cause) => cause,
            DescribeActivityTypeError::Credentials(ref err) => err.description(),
            DescribeActivityTypeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeActivityTypeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeDomain
#[derive(Debug, PartialEq)]
pub enum DescribeDomainError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeDomainError {
    pub fn from_body(body: &str) -> DescribeDomainError {
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
                    "OperationNotPermittedFault" => {
                        DescribeDomainError::OperationNotPermittedFault(String::from(error_message))
                    }
                    "UnknownResourceFault" => {
                        DescribeDomainError::UnknownResourceFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeDomainError::Validation(error_message.to_string())
                    }
                    _ => DescribeDomainError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeDomainError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeDomainError {
    fn from(err: serde_json::error::Error) -> DescribeDomainError {
        DescribeDomainError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeDomainError {
    fn from(err: CredentialsError) -> DescribeDomainError {
        DescribeDomainError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeDomainError {
    fn from(err: HttpDispatchError) -> DescribeDomainError {
        DescribeDomainError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeDomainError {
    fn from(err: io::Error) -> DescribeDomainError {
        DescribeDomainError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeDomainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDomainError {
    fn description(&self) -> &str {
        match *self {
            DescribeDomainError::OperationNotPermittedFault(ref cause) => cause,
            DescribeDomainError::UnknownResourceFault(ref cause) => cause,
            DescribeDomainError::Validation(ref cause) => cause,
            DescribeDomainError::Credentials(ref err) => err.description(),
            DescribeDomainError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeDomainError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeWorkflowExecution
#[derive(Debug, PartialEq)]
pub enum DescribeWorkflowExecutionError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeWorkflowExecutionError {
    pub fn from_body(body: &str) -> DescribeWorkflowExecutionError {
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
                    "OperationNotPermittedFault" => {
                        DescribeWorkflowExecutionError::OperationNotPermittedFault(String::from(
                            error_message,
                        ))
                    }
                    "UnknownResourceFault" => DescribeWorkflowExecutionError::UnknownResourceFault(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        DescribeWorkflowExecutionError::Validation(error_message.to_string())
                    }
                    _ => DescribeWorkflowExecutionError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeWorkflowExecutionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeWorkflowExecutionError {
    fn from(err: serde_json::error::Error) -> DescribeWorkflowExecutionError {
        DescribeWorkflowExecutionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeWorkflowExecutionError {
    fn from(err: CredentialsError) -> DescribeWorkflowExecutionError {
        DescribeWorkflowExecutionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeWorkflowExecutionError {
    fn from(err: HttpDispatchError) -> DescribeWorkflowExecutionError {
        DescribeWorkflowExecutionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeWorkflowExecutionError {
    fn from(err: io::Error) -> DescribeWorkflowExecutionError {
        DescribeWorkflowExecutionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeWorkflowExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeWorkflowExecutionError {
    fn description(&self) -> &str {
        match *self {
            DescribeWorkflowExecutionError::OperationNotPermittedFault(ref cause) => cause,
            DescribeWorkflowExecutionError::UnknownResourceFault(ref cause) => cause,
            DescribeWorkflowExecutionError::Validation(ref cause) => cause,
            DescribeWorkflowExecutionError::Credentials(ref err) => err.description(),
            DescribeWorkflowExecutionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeWorkflowExecutionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeWorkflowType
#[derive(Debug, PartialEq)]
pub enum DescribeWorkflowTypeError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeWorkflowTypeError {
    pub fn from_body(body: &str) -> DescribeWorkflowTypeError {
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
                    "OperationNotPermittedFault" => {
                        DescribeWorkflowTypeError::OperationNotPermittedFault(String::from(
                            error_message,
                        ))
                    }
                    "UnknownResourceFault" => {
                        DescribeWorkflowTypeError::UnknownResourceFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeWorkflowTypeError::Validation(error_message.to_string())
                    }
                    _ => DescribeWorkflowTypeError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeWorkflowTypeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeWorkflowTypeError {
    fn from(err: serde_json::error::Error) -> DescribeWorkflowTypeError {
        DescribeWorkflowTypeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeWorkflowTypeError {
    fn from(err: CredentialsError) -> DescribeWorkflowTypeError {
        DescribeWorkflowTypeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeWorkflowTypeError {
    fn from(err: HttpDispatchError) -> DescribeWorkflowTypeError {
        DescribeWorkflowTypeError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeWorkflowTypeError {
    fn from(err: io::Error) -> DescribeWorkflowTypeError {
        DescribeWorkflowTypeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeWorkflowTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeWorkflowTypeError {
    fn description(&self) -> &str {
        match *self {
            DescribeWorkflowTypeError::OperationNotPermittedFault(ref cause) => cause,
            DescribeWorkflowTypeError::UnknownResourceFault(ref cause) => cause,
            DescribeWorkflowTypeError::Validation(ref cause) => cause,
            DescribeWorkflowTypeError::Credentials(ref err) => err.description(),
            DescribeWorkflowTypeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeWorkflowTypeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetWorkflowExecutionHistory
#[derive(Debug, PartialEq)]
pub enum GetWorkflowExecutionHistoryError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetWorkflowExecutionHistoryError {
    pub fn from_body(body: &str) -> GetWorkflowExecutionHistoryError {
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
                    "OperationNotPermittedFault" => {
                        GetWorkflowExecutionHistoryError::OperationNotPermittedFault(String::from(
                            error_message,
                        ))
                    }
                    "UnknownResourceFault" => {
                        GetWorkflowExecutionHistoryError::UnknownResourceFault(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        GetWorkflowExecutionHistoryError::Validation(error_message.to_string())
                    }
                    _ => GetWorkflowExecutionHistoryError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetWorkflowExecutionHistoryError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetWorkflowExecutionHistoryError {
    fn from(err: serde_json::error::Error) -> GetWorkflowExecutionHistoryError {
        GetWorkflowExecutionHistoryError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetWorkflowExecutionHistoryError {
    fn from(err: CredentialsError) -> GetWorkflowExecutionHistoryError {
        GetWorkflowExecutionHistoryError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetWorkflowExecutionHistoryError {
    fn from(err: HttpDispatchError) -> GetWorkflowExecutionHistoryError {
        GetWorkflowExecutionHistoryError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetWorkflowExecutionHistoryError {
    fn from(err: io::Error) -> GetWorkflowExecutionHistoryError {
        GetWorkflowExecutionHistoryError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetWorkflowExecutionHistoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetWorkflowExecutionHistoryError {
    fn description(&self) -> &str {
        match *self {
            GetWorkflowExecutionHistoryError::OperationNotPermittedFault(ref cause) => cause,
            GetWorkflowExecutionHistoryError::UnknownResourceFault(ref cause) => cause,
            GetWorkflowExecutionHistoryError::Validation(ref cause) => cause,
            GetWorkflowExecutionHistoryError::Credentials(ref err) => err.description(),
            GetWorkflowExecutionHistoryError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetWorkflowExecutionHistoryError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListActivityTypes
#[derive(Debug, PartialEq)]
pub enum ListActivityTypesError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListActivityTypesError {
    pub fn from_body(body: &str) -> ListActivityTypesError {
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
                    "OperationNotPermittedFault" => {
                        ListActivityTypesError::OperationNotPermittedFault(String::from(
                            error_message,
                        ))
                    }
                    "UnknownResourceFault" => {
                        ListActivityTypesError::UnknownResourceFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListActivityTypesError::Validation(error_message.to_string())
                    }
                    _ => ListActivityTypesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListActivityTypesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListActivityTypesError {
    fn from(err: serde_json::error::Error) -> ListActivityTypesError {
        ListActivityTypesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListActivityTypesError {
    fn from(err: CredentialsError) -> ListActivityTypesError {
        ListActivityTypesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListActivityTypesError {
    fn from(err: HttpDispatchError) -> ListActivityTypesError {
        ListActivityTypesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListActivityTypesError {
    fn from(err: io::Error) -> ListActivityTypesError {
        ListActivityTypesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListActivityTypesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListActivityTypesError {
    fn description(&self) -> &str {
        match *self {
            ListActivityTypesError::OperationNotPermittedFault(ref cause) => cause,
            ListActivityTypesError::UnknownResourceFault(ref cause) => cause,
            ListActivityTypesError::Validation(ref cause) => cause,
            ListActivityTypesError::Credentials(ref err) => err.description(),
            ListActivityTypesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListActivityTypesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListClosedWorkflowExecutions
#[derive(Debug, PartialEq)]
pub enum ListClosedWorkflowExecutionsError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListClosedWorkflowExecutionsError {
    pub fn from_body(body: &str) -> ListClosedWorkflowExecutionsError {
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
                    "OperationNotPermittedFault" => {
                        ListClosedWorkflowExecutionsError::OperationNotPermittedFault(String::from(
                            error_message,
                        ))
                    }
                    "UnknownResourceFault" => {
                        ListClosedWorkflowExecutionsError::UnknownResourceFault(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        ListClosedWorkflowExecutionsError::Validation(error_message.to_string())
                    }
                    _ => ListClosedWorkflowExecutionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListClosedWorkflowExecutionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListClosedWorkflowExecutionsError {
    fn from(err: serde_json::error::Error) -> ListClosedWorkflowExecutionsError {
        ListClosedWorkflowExecutionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListClosedWorkflowExecutionsError {
    fn from(err: CredentialsError) -> ListClosedWorkflowExecutionsError {
        ListClosedWorkflowExecutionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListClosedWorkflowExecutionsError {
    fn from(err: HttpDispatchError) -> ListClosedWorkflowExecutionsError {
        ListClosedWorkflowExecutionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListClosedWorkflowExecutionsError {
    fn from(err: io::Error) -> ListClosedWorkflowExecutionsError {
        ListClosedWorkflowExecutionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListClosedWorkflowExecutionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListClosedWorkflowExecutionsError {
    fn description(&self) -> &str {
        match *self {
            ListClosedWorkflowExecutionsError::OperationNotPermittedFault(ref cause) => cause,
            ListClosedWorkflowExecutionsError::UnknownResourceFault(ref cause) => cause,
            ListClosedWorkflowExecutionsError::Validation(ref cause) => cause,
            ListClosedWorkflowExecutionsError::Credentials(ref err) => err.description(),
            ListClosedWorkflowExecutionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListClosedWorkflowExecutionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListDomains
#[derive(Debug, PartialEq)]
pub enum ListDomainsError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListDomainsError {
    pub fn from_body(body: &str) -> ListDomainsError {
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
                    "OperationNotPermittedFault" => {
                        ListDomainsError::OperationNotPermittedFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListDomainsError::Validation(error_message.to_string())
                    }
                    _ => ListDomainsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListDomainsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListDomainsError {
    fn from(err: serde_json::error::Error) -> ListDomainsError {
        ListDomainsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListDomainsError {
    fn from(err: CredentialsError) -> ListDomainsError {
        ListDomainsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListDomainsError {
    fn from(err: HttpDispatchError) -> ListDomainsError {
        ListDomainsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListDomainsError {
    fn from(err: io::Error) -> ListDomainsError {
        ListDomainsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListDomainsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDomainsError {
    fn description(&self) -> &str {
        match *self {
            ListDomainsError::OperationNotPermittedFault(ref cause) => cause,
            ListDomainsError::Validation(ref cause) => cause,
            ListDomainsError::Credentials(ref err) => err.description(),
            ListDomainsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListDomainsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListOpenWorkflowExecutions
#[derive(Debug, PartialEq)]
pub enum ListOpenWorkflowExecutionsError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListOpenWorkflowExecutionsError {
    pub fn from_body(body: &str) -> ListOpenWorkflowExecutionsError {
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
                    "OperationNotPermittedFault" => {
                        ListOpenWorkflowExecutionsError::OperationNotPermittedFault(String::from(
                            error_message,
                        ))
                    }
                    "UnknownResourceFault" => {
                        ListOpenWorkflowExecutionsError::UnknownResourceFault(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        ListOpenWorkflowExecutionsError::Validation(error_message.to_string())
                    }
                    _ => ListOpenWorkflowExecutionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListOpenWorkflowExecutionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListOpenWorkflowExecutionsError {
    fn from(err: serde_json::error::Error) -> ListOpenWorkflowExecutionsError {
        ListOpenWorkflowExecutionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListOpenWorkflowExecutionsError {
    fn from(err: CredentialsError) -> ListOpenWorkflowExecutionsError {
        ListOpenWorkflowExecutionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListOpenWorkflowExecutionsError {
    fn from(err: HttpDispatchError) -> ListOpenWorkflowExecutionsError {
        ListOpenWorkflowExecutionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListOpenWorkflowExecutionsError {
    fn from(err: io::Error) -> ListOpenWorkflowExecutionsError {
        ListOpenWorkflowExecutionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListOpenWorkflowExecutionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListOpenWorkflowExecutionsError {
    fn description(&self) -> &str {
        match *self {
            ListOpenWorkflowExecutionsError::OperationNotPermittedFault(ref cause) => cause,
            ListOpenWorkflowExecutionsError::UnknownResourceFault(ref cause) => cause,
            ListOpenWorkflowExecutionsError::Validation(ref cause) => cause,
            ListOpenWorkflowExecutionsError::Credentials(ref err) => err.description(),
            ListOpenWorkflowExecutionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListOpenWorkflowExecutionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListWorkflowTypes
#[derive(Debug, PartialEq)]
pub enum ListWorkflowTypesError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListWorkflowTypesError {
    pub fn from_body(body: &str) -> ListWorkflowTypesError {
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
                    "OperationNotPermittedFault" => {
                        ListWorkflowTypesError::OperationNotPermittedFault(String::from(
                            error_message,
                        ))
                    }
                    "UnknownResourceFault" => {
                        ListWorkflowTypesError::UnknownResourceFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListWorkflowTypesError::Validation(error_message.to_string())
                    }
                    _ => ListWorkflowTypesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListWorkflowTypesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListWorkflowTypesError {
    fn from(err: serde_json::error::Error) -> ListWorkflowTypesError {
        ListWorkflowTypesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListWorkflowTypesError {
    fn from(err: CredentialsError) -> ListWorkflowTypesError {
        ListWorkflowTypesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListWorkflowTypesError {
    fn from(err: HttpDispatchError) -> ListWorkflowTypesError {
        ListWorkflowTypesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListWorkflowTypesError {
    fn from(err: io::Error) -> ListWorkflowTypesError {
        ListWorkflowTypesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListWorkflowTypesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListWorkflowTypesError {
    fn description(&self) -> &str {
        match *self {
            ListWorkflowTypesError::OperationNotPermittedFault(ref cause) => cause,
            ListWorkflowTypesError::UnknownResourceFault(ref cause) => cause,
            ListWorkflowTypesError::Validation(ref cause) => cause,
            ListWorkflowTypesError::Credentials(ref err) => err.description(),
            ListWorkflowTypesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListWorkflowTypesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PollForActivityTask
#[derive(Debug, PartialEq)]
pub enum PollForActivityTaskError {
    /// <p>Returned by any operation if a system imposed limitation has been reached. To address this fault you should either clean up unused resources or increase the limit by contacting AWS.</p>
    LimitExceededFault(String),
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PollForActivityTaskError {
    pub fn from_body(body: &str) -> PollForActivityTaskError {
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
                    "LimitExceededFault" => {
                        PollForActivityTaskError::LimitExceededFault(String::from(error_message))
                    }
                    "OperationNotPermittedFault" => {
                        PollForActivityTaskError::OperationNotPermittedFault(String::from(
                            error_message,
                        ))
                    }
                    "UnknownResourceFault" => {
                        PollForActivityTaskError::UnknownResourceFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        PollForActivityTaskError::Validation(error_message.to_string())
                    }
                    _ => PollForActivityTaskError::Unknown(String::from(body)),
                }
            }
            Err(_) => PollForActivityTaskError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PollForActivityTaskError {
    fn from(err: serde_json::error::Error) -> PollForActivityTaskError {
        PollForActivityTaskError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PollForActivityTaskError {
    fn from(err: CredentialsError) -> PollForActivityTaskError {
        PollForActivityTaskError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PollForActivityTaskError {
    fn from(err: HttpDispatchError) -> PollForActivityTaskError {
        PollForActivityTaskError::HttpDispatch(err)
    }
}
impl From<io::Error> for PollForActivityTaskError {
    fn from(err: io::Error) -> PollForActivityTaskError {
        PollForActivityTaskError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PollForActivityTaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PollForActivityTaskError {
    fn description(&self) -> &str {
        match *self {
            PollForActivityTaskError::LimitExceededFault(ref cause) => cause,
            PollForActivityTaskError::OperationNotPermittedFault(ref cause) => cause,
            PollForActivityTaskError::UnknownResourceFault(ref cause) => cause,
            PollForActivityTaskError::Validation(ref cause) => cause,
            PollForActivityTaskError::Credentials(ref err) => err.description(),
            PollForActivityTaskError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PollForActivityTaskError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PollForDecisionTask
#[derive(Debug, PartialEq)]
pub enum PollForDecisionTaskError {
    /// <p>Returned by any operation if a system imposed limitation has been reached. To address this fault you should either clean up unused resources or increase the limit by contacting AWS.</p>
    LimitExceededFault(String),
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PollForDecisionTaskError {
    pub fn from_body(body: &str) -> PollForDecisionTaskError {
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
                    "LimitExceededFault" => {
                        PollForDecisionTaskError::LimitExceededFault(String::from(error_message))
                    }
                    "OperationNotPermittedFault" => {
                        PollForDecisionTaskError::OperationNotPermittedFault(String::from(
                            error_message,
                        ))
                    }
                    "UnknownResourceFault" => {
                        PollForDecisionTaskError::UnknownResourceFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        PollForDecisionTaskError::Validation(error_message.to_string())
                    }
                    _ => PollForDecisionTaskError::Unknown(String::from(body)),
                }
            }
            Err(_) => PollForDecisionTaskError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PollForDecisionTaskError {
    fn from(err: serde_json::error::Error) -> PollForDecisionTaskError {
        PollForDecisionTaskError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PollForDecisionTaskError {
    fn from(err: CredentialsError) -> PollForDecisionTaskError {
        PollForDecisionTaskError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PollForDecisionTaskError {
    fn from(err: HttpDispatchError) -> PollForDecisionTaskError {
        PollForDecisionTaskError::HttpDispatch(err)
    }
}
impl From<io::Error> for PollForDecisionTaskError {
    fn from(err: io::Error) -> PollForDecisionTaskError {
        PollForDecisionTaskError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PollForDecisionTaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PollForDecisionTaskError {
    fn description(&self) -> &str {
        match *self {
            PollForDecisionTaskError::LimitExceededFault(ref cause) => cause,
            PollForDecisionTaskError::OperationNotPermittedFault(ref cause) => cause,
            PollForDecisionTaskError::UnknownResourceFault(ref cause) => cause,
            PollForDecisionTaskError::Validation(ref cause) => cause,
            PollForDecisionTaskError::Credentials(ref err) => err.description(),
            PollForDecisionTaskError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PollForDecisionTaskError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RecordActivityTaskHeartbeat
#[derive(Debug, PartialEq)]
pub enum RecordActivityTaskHeartbeatError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RecordActivityTaskHeartbeatError {
    pub fn from_body(body: &str) -> RecordActivityTaskHeartbeatError {
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
                    "OperationNotPermittedFault" => {
                        RecordActivityTaskHeartbeatError::OperationNotPermittedFault(String::from(
                            error_message,
                        ))
                    }
                    "UnknownResourceFault" => {
                        RecordActivityTaskHeartbeatError::UnknownResourceFault(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        RecordActivityTaskHeartbeatError::Validation(error_message.to_string())
                    }
                    _ => RecordActivityTaskHeartbeatError::Unknown(String::from(body)),
                }
            }
            Err(_) => RecordActivityTaskHeartbeatError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RecordActivityTaskHeartbeatError {
    fn from(err: serde_json::error::Error) -> RecordActivityTaskHeartbeatError {
        RecordActivityTaskHeartbeatError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RecordActivityTaskHeartbeatError {
    fn from(err: CredentialsError) -> RecordActivityTaskHeartbeatError {
        RecordActivityTaskHeartbeatError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RecordActivityTaskHeartbeatError {
    fn from(err: HttpDispatchError) -> RecordActivityTaskHeartbeatError {
        RecordActivityTaskHeartbeatError::HttpDispatch(err)
    }
}
impl From<io::Error> for RecordActivityTaskHeartbeatError {
    fn from(err: io::Error) -> RecordActivityTaskHeartbeatError {
        RecordActivityTaskHeartbeatError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RecordActivityTaskHeartbeatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RecordActivityTaskHeartbeatError {
    fn description(&self) -> &str {
        match *self {
            RecordActivityTaskHeartbeatError::OperationNotPermittedFault(ref cause) => cause,
            RecordActivityTaskHeartbeatError::UnknownResourceFault(ref cause) => cause,
            RecordActivityTaskHeartbeatError::Validation(ref cause) => cause,
            RecordActivityTaskHeartbeatError::Credentials(ref err) => err.description(),
            RecordActivityTaskHeartbeatError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RecordActivityTaskHeartbeatError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RegisterActivityType
#[derive(Debug, PartialEq)]
pub enum RegisterActivityTypeError {
    /// <p>Returned by any operation if a system imposed limitation has been reached. To address this fault you should either clean up unused resources or increase the limit by contacting AWS.</p>
    LimitExceededFault(String),
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned if the type already exists in the specified domain. You get this fault even if the existing type is in deprecated status. You can specify another version if the intent is to create a new distinct version of the type.</p>
    TypeAlreadyExistsFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RegisterActivityTypeError {
    pub fn from_body(body: &str) -> RegisterActivityTypeError {
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
                    "LimitExceededFault" => {
                        RegisterActivityTypeError::LimitExceededFault(String::from(error_message))
                    }
                    "OperationNotPermittedFault" => {
                        RegisterActivityTypeError::OperationNotPermittedFault(String::from(
                            error_message,
                        ))
                    }
                    "TypeAlreadyExistsFault" => RegisterActivityTypeError::TypeAlreadyExistsFault(
                        String::from(error_message),
                    ),
                    "UnknownResourceFault" => {
                        RegisterActivityTypeError::UnknownResourceFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        RegisterActivityTypeError::Validation(error_message.to_string())
                    }
                    _ => RegisterActivityTypeError::Unknown(String::from(body)),
                }
            }
            Err(_) => RegisterActivityTypeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RegisterActivityTypeError {
    fn from(err: serde_json::error::Error) -> RegisterActivityTypeError {
        RegisterActivityTypeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RegisterActivityTypeError {
    fn from(err: CredentialsError) -> RegisterActivityTypeError {
        RegisterActivityTypeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RegisterActivityTypeError {
    fn from(err: HttpDispatchError) -> RegisterActivityTypeError {
        RegisterActivityTypeError::HttpDispatch(err)
    }
}
impl From<io::Error> for RegisterActivityTypeError {
    fn from(err: io::Error) -> RegisterActivityTypeError {
        RegisterActivityTypeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RegisterActivityTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RegisterActivityTypeError {
    fn description(&self) -> &str {
        match *self {
            RegisterActivityTypeError::LimitExceededFault(ref cause) => cause,
            RegisterActivityTypeError::OperationNotPermittedFault(ref cause) => cause,
            RegisterActivityTypeError::TypeAlreadyExistsFault(ref cause) => cause,
            RegisterActivityTypeError::UnknownResourceFault(ref cause) => cause,
            RegisterActivityTypeError::Validation(ref cause) => cause,
            RegisterActivityTypeError::Credentials(ref err) => err.description(),
            RegisterActivityTypeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RegisterActivityTypeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RegisterDomain
#[derive(Debug, PartialEq)]
pub enum RegisterDomainError {
    /// <p>Returned if the specified domain already exists. You get this fault even if the existing domain is in deprecated status.</p>
    DomainAlreadyExistsFault(String),
    /// <p>Returned by any operation if a system imposed limitation has been reached. To address this fault you should either clean up unused resources or increase the limit by contacting AWS.</p>
    LimitExceededFault(String),
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RegisterDomainError {
    pub fn from_body(body: &str) -> RegisterDomainError {
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
                    "DomainAlreadyExistsFault" => {
                        RegisterDomainError::DomainAlreadyExistsFault(String::from(error_message))
                    }
                    "LimitExceededFault" => {
                        RegisterDomainError::LimitExceededFault(String::from(error_message))
                    }
                    "OperationNotPermittedFault" => {
                        RegisterDomainError::OperationNotPermittedFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        RegisterDomainError::Validation(error_message.to_string())
                    }
                    _ => RegisterDomainError::Unknown(String::from(body)),
                }
            }
            Err(_) => RegisterDomainError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RegisterDomainError {
    fn from(err: serde_json::error::Error) -> RegisterDomainError {
        RegisterDomainError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RegisterDomainError {
    fn from(err: CredentialsError) -> RegisterDomainError {
        RegisterDomainError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RegisterDomainError {
    fn from(err: HttpDispatchError) -> RegisterDomainError {
        RegisterDomainError::HttpDispatch(err)
    }
}
impl From<io::Error> for RegisterDomainError {
    fn from(err: io::Error) -> RegisterDomainError {
        RegisterDomainError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RegisterDomainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RegisterDomainError {
    fn description(&self) -> &str {
        match *self {
            RegisterDomainError::DomainAlreadyExistsFault(ref cause) => cause,
            RegisterDomainError::LimitExceededFault(ref cause) => cause,
            RegisterDomainError::OperationNotPermittedFault(ref cause) => cause,
            RegisterDomainError::Validation(ref cause) => cause,
            RegisterDomainError::Credentials(ref err) => err.description(),
            RegisterDomainError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RegisterDomainError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RegisterWorkflowType
#[derive(Debug, PartialEq)]
pub enum RegisterWorkflowTypeError {
    /// <p>Returned by any operation if a system imposed limitation has been reached. To address this fault you should either clean up unused resources or increase the limit by contacting AWS.</p>
    LimitExceededFault(String),
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned if the type already exists in the specified domain. You get this fault even if the existing type is in deprecated status. You can specify another version if the intent is to create a new distinct version of the type.</p>
    TypeAlreadyExistsFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RegisterWorkflowTypeError {
    pub fn from_body(body: &str) -> RegisterWorkflowTypeError {
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
                    "LimitExceededFault" => {
                        RegisterWorkflowTypeError::LimitExceededFault(String::from(error_message))
                    }
                    "OperationNotPermittedFault" => {
                        RegisterWorkflowTypeError::OperationNotPermittedFault(String::from(
                            error_message,
                        ))
                    }
                    "TypeAlreadyExistsFault" => RegisterWorkflowTypeError::TypeAlreadyExistsFault(
                        String::from(error_message),
                    ),
                    "UnknownResourceFault" => {
                        RegisterWorkflowTypeError::UnknownResourceFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        RegisterWorkflowTypeError::Validation(error_message.to_string())
                    }
                    _ => RegisterWorkflowTypeError::Unknown(String::from(body)),
                }
            }
            Err(_) => RegisterWorkflowTypeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RegisterWorkflowTypeError {
    fn from(err: serde_json::error::Error) -> RegisterWorkflowTypeError {
        RegisterWorkflowTypeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RegisterWorkflowTypeError {
    fn from(err: CredentialsError) -> RegisterWorkflowTypeError {
        RegisterWorkflowTypeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RegisterWorkflowTypeError {
    fn from(err: HttpDispatchError) -> RegisterWorkflowTypeError {
        RegisterWorkflowTypeError::HttpDispatch(err)
    }
}
impl From<io::Error> for RegisterWorkflowTypeError {
    fn from(err: io::Error) -> RegisterWorkflowTypeError {
        RegisterWorkflowTypeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RegisterWorkflowTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RegisterWorkflowTypeError {
    fn description(&self) -> &str {
        match *self {
            RegisterWorkflowTypeError::LimitExceededFault(ref cause) => cause,
            RegisterWorkflowTypeError::OperationNotPermittedFault(ref cause) => cause,
            RegisterWorkflowTypeError::TypeAlreadyExistsFault(ref cause) => cause,
            RegisterWorkflowTypeError::UnknownResourceFault(ref cause) => cause,
            RegisterWorkflowTypeError::Validation(ref cause) => cause,
            RegisterWorkflowTypeError::Credentials(ref err) => err.description(),
            RegisterWorkflowTypeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RegisterWorkflowTypeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RequestCancelWorkflowExecution
#[derive(Debug, PartialEq)]
pub enum RequestCancelWorkflowExecutionError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RequestCancelWorkflowExecutionError {
    pub fn from_body(body: &str) -> RequestCancelWorkflowExecutionError {
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
                    "OperationNotPermittedFault" => {
                        RequestCancelWorkflowExecutionError::OperationNotPermittedFault(
                            String::from(error_message),
                        )
                    }
                    "UnknownResourceFault" => {
                        RequestCancelWorkflowExecutionError::UnknownResourceFault(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        RequestCancelWorkflowExecutionError::Validation(error_message.to_string())
                    }
                    _ => RequestCancelWorkflowExecutionError::Unknown(String::from(body)),
                }
            }
            Err(_) => RequestCancelWorkflowExecutionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RequestCancelWorkflowExecutionError {
    fn from(err: serde_json::error::Error) -> RequestCancelWorkflowExecutionError {
        RequestCancelWorkflowExecutionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RequestCancelWorkflowExecutionError {
    fn from(err: CredentialsError) -> RequestCancelWorkflowExecutionError {
        RequestCancelWorkflowExecutionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RequestCancelWorkflowExecutionError {
    fn from(err: HttpDispatchError) -> RequestCancelWorkflowExecutionError {
        RequestCancelWorkflowExecutionError::HttpDispatch(err)
    }
}
impl From<io::Error> for RequestCancelWorkflowExecutionError {
    fn from(err: io::Error) -> RequestCancelWorkflowExecutionError {
        RequestCancelWorkflowExecutionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RequestCancelWorkflowExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RequestCancelWorkflowExecutionError {
    fn description(&self) -> &str {
        match *self {
            RequestCancelWorkflowExecutionError::OperationNotPermittedFault(ref cause) => cause,
            RequestCancelWorkflowExecutionError::UnknownResourceFault(ref cause) => cause,
            RequestCancelWorkflowExecutionError::Validation(ref cause) => cause,
            RequestCancelWorkflowExecutionError::Credentials(ref err) => err.description(),
            RequestCancelWorkflowExecutionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RequestCancelWorkflowExecutionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RespondActivityTaskCanceled
#[derive(Debug, PartialEq)]
pub enum RespondActivityTaskCanceledError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RespondActivityTaskCanceledError {
    pub fn from_body(body: &str) -> RespondActivityTaskCanceledError {
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
                    "OperationNotPermittedFault" => {
                        RespondActivityTaskCanceledError::OperationNotPermittedFault(String::from(
                            error_message,
                        ))
                    }
                    "UnknownResourceFault" => {
                        RespondActivityTaskCanceledError::UnknownResourceFault(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        RespondActivityTaskCanceledError::Validation(error_message.to_string())
                    }
                    _ => RespondActivityTaskCanceledError::Unknown(String::from(body)),
                }
            }
            Err(_) => RespondActivityTaskCanceledError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RespondActivityTaskCanceledError {
    fn from(err: serde_json::error::Error) -> RespondActivityTaskCanceledError {
        RespondActivityTaskCanceledError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RespondActivityTaskCanceledError {
    fn from(err: CredentialsError) -> RespondActivityTaskCanceledError {
        RespondActivityTaskCanceledError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RespondActivityTaskCanceledError {
    fn from(err: HttpDispatchError) -> RespondActivityTaskCanceledError {
        RespondActivityTaskCanceledError::HttpDispatch(err)
    }
}
impl From<io::Error> for RespondActivityTaskCanceledError {
    fn from(err: io::Error) -> RespondActivityTaskCanceledError {
        RespondActivityTaskCanceledError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RespondActivityTaskCanceledError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RespondActivityTaskCanceledError {
    fn description(&self) -> &str {
        match *self {
            RespondActivityTaskCanceledError::OperationNotPermittedFault(ref cause) => cause,
            RespondActivityTaskCanceledError::UnknownResourceFault(ref cause) => cause,
            RespondActivityTaskCanceledError::Validation(ref cause) => cause,
            RespondActivityTaskCanceledError::Credentials(ref err) => err.description(),
            RespondActivityTaskCanceledError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RespondActivityTaskCanceledError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RespondActivityTaskCompleted
#[derive(Debug, PartialEq)]
pub enum RespondActivityTaskCompletedError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RespondActivityTaskCompletedError {
    pub fn from_body(body: &str) -> RespondActivityTaskCompletedError {
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
                    "OperationNotPermittedFault" => {
                        RespondActivityTaskCompletedError::OperationNotPermittedFault(String::from(
                            error_message,
                        ))
                    }
                    "UnknownResourceFault" => {
                        RespondActivityTaskCompletedError::UnknownResourceFault(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        RespondActivityTaskCompletedError::Validation(error_message.to_string())
                    }
                    _ => RespondActivityTaskCompletedError::Unknown(String::from(body)),
                }
            }
            Err(_) => RespondActivityTaskCompletedError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RespondActivityTaskCompletedError {
    fn from(err: serde_json::error::Error) -> RespondActivityTaskCompletedError {
        RespondActivityTaskCompletedError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RespondActivityTaskCompletedError {
    fn from(err: CredentialsError) -> RespondActivityTaskCompletedError {
        RespondActivityTaskCompletedError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RespondActivityTaskCompletedError {
    fn from(err: HttpDispatchError) -> RespondActivityTaskCompletedError {
        RespondActivityTaskCompletedError::HttpDispatch(err)
    }
}
impl From<io::Error> for RespondActivityTaskCompletedError {
    fn from(err: io::Error) -> RespondActivityTaskCompletedError {
        RespondActivityTaskCompletedError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RespondActivityTaskCompletedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RespondActivityTaskCompletedError {
    fn description(&self) -> &str {
        match *self {
            RespondActivityTaskCompletedError::OperationNotPermittedFault(ref cause) => cause,
            RespondActivityTaskCompletedError::UnknownResourceFault(ref cause) => cause,
            RespondActivityTaskCompletedError::Validation(ref cause) => cause,
            RespondActivityTaskCompletedError::Credentials(ref err) => err.description(),
            RespondActivityTaskCompletedError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RespondActivityTaskCompletedError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RespondActivityTaskFailed
#[derive(Debug, PartialEq)]
pub enum RespondActivityTaskFailedError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RespondActivityTaskFailedError {
    pub fn from_body(body: &str) -> RespondActivityTaskFailedError {
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
                    "OperationNotPermittedFault" => {
                        RespondActivityTaskFailedError::OperationNotPermittedFault(String::from(
                            error_message,
                        ))
                    }
                    "UnknownResourceFault" => RespondActivityTaskFailedError::UnknownResourceFault(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        RespondActivityTaskFailedError::Validation(error_message.to_string())
                    }
                    _ => RespondActivityTaskFailedError::Unknown(String::from(body)),
                }
            }
            Err(_) => RespondActivityTaskFailedError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RespondActivityTaskFailedError {
    fn from(err: serde_json::error::Error) -> RespondActivityTaskFailedError {
        RespondActivityTaskFailedError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RespondActivityTaskFailedError {
    fn from(err: CredentialsError) -> RespondActivityTaskFailedError {
        RespondActivityTaskFailedError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RespondActivityTaskFailedError {
    fn from(err: HttpDispatchError) -> RespondActivityTaskFailedError {
        RespondActivityTaskFailedError::HttpDispatch(err)
    }
}
impl From<io::Error> for RespondActivityTaskFailedError {
    fn from(err: io::Error) -> RespondActivityTaskFailedError {
        RespondActivityTaskFailedError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RespondActivityTaskFailedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RespondActivityTaskFailedError {
    fn description(&self) -> &str {
        match *self {
            RespondActivityTaskFailedError::OperationNotPermittedFault(ref cause) => cause,
            RespondActivityTaskFailedError::UnknownResourceFault(ref cause) => cause,
            RespondActivityTaskFailedError::Validation(ref cause) => cause,
            RespondActivityTaskFailedError::Credentials(ref err) => err.description(),
            RespondActivityTaskFailedError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RespondActivityTaskFailedError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RespondDecisionTaskCompleted
#[derive(Debug, PartialEq)]
pub enum RespondDecisionTaskCompletedError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RespondDecisionTaskCompletedError {
    pub fn from_body(body: &str) -> RespondDecisionTaskCompletedError {
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
                    "OperationNotPermittedFault" => {
                        RespondDecisionTaskCompletedError::OperationNotPermittedFault(String::from(
                            error_message,
                        ))
                    }
                    "UnknownResourceFault" => {
                        RespondDecisionTaskCompletedError::UnknownResourceFault(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        RespondDecisionTaskCompletedError::Validation(error_message.to_string())
                    }
                    _ => RespondDecisionTaskCompletedError::Unknown(String::from(body)),
                }
            }
            Err(_) => RespondDecisionTaskCompletedError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RespondDecisionTaskCompletedError {
    fn from(err: serde_json::error::Error) -> RespondDecisionTaskCompletedError {
        RespondDecisionTaskCompletedError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RespondDecisionTaskCompletedError {
    fn from(err: CredentialsError) -> RespondDecisionTaskCompletedError {
        RespondDecisionTaskCompletedError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RespondDecisionTaskCompletedError {
    fn from(err: HttpDispatchError) -> RespondDecisionTaskCompletedError {
        RespondDecisionTaskCompletedError::HttpDispatch(err)
    }
}
impl From<io::Error> for RespondDecisionTaskCompletedError {
    fn from(err: io::Error) -> RespondDecisionTaskCompletedError {
        RespondDecisionTaskCompletedError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RespondDecisionTaskCompletedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RespondDecisionTaskCompletedError {
    fn description(&self) -> &str {
        match *self {
            RespondDecisionTaskCompletedError::OperationNotPermittedFault(ref cause) => cause,
            RespondDecisionTaskCompletedError::UnknownResourceFault(ref cause) => cause,
            RespondDecisionTaskCompletedError::Validation(ref cause) => cause,
            RespondDecisionTaskCompletedError::Credentials(ref err) => err.description(),
            RespondDecisionTaskCompletedError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RespondDecisionTaskCompletedError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SignalWorkflowExecution
#[derive(Debug, PartialEq)]
pub enum SignalWorkflowExecutionError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SignalWorkflowExecutionError {
    pub fn from_body(body: &str) -> SignalWorkflowExecutionError {
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
                    "OperationNotPermittedFault" => {
                        SignalWorkflowExecutionError::OperationNotPermittedFault(String::from(
                            error_message,
                        ))
                    }
                    "UnknownResourceFault" => SignalWorkflowExecutionError::UnknownResourceFault(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        SignalWorkflowExecutionError::Validation(error_message.to_string())
                    }
                    _ => SignalWorkflowExecutionError::Unknown(String::from(body)),
                }
            }
            Err(_) => SignalWorkflowExecutionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SignalWorkflowExecutionError {
    fn from(err: serde_json::error::Error) -> SignalWorkflowExecutionError {
        SignalWorkflowExecutionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SignalWorkflowExecutionError {
    fn from(err: CredentialsError) -> SignalWorkflowExecutionError {
        SignalWorkflowExecutionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SignalWorkflowExecutionError {
    fn from(err: HttpDispatchError) -> SignalWorkflowExecutionError {
        SignalWorkflowExecutionError::HttpDispatch(err)
    }
}
impl From<io::Error> for SignalWorkflowExecutionError {
    fn from(err: io::Error) -> SignalWorkflowExecutionError {
        SignalWorkflowExecutionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SignalWorkflowExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SignalWorkflowExecutionError {
    fn description(&self) -> &str {
        match *self {
            SignalWorkflowExecutionError::OperationNotPermittedFault(ref cause) => cause,
            SignalWorkflowExecutionError::UnknownResourceFault(ref cause) => cause,
            SignalWorkflowExecutionError::Validation(ref cause) => cause,
            SignalWorkflowExecutionError::Credentials(ref err) => err.description(),
            SignalWorkflowExecutionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SignalWorkflowExecutionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StartWorkflowExecution
#[derive(Debug, PartialEq)]
pub enum StartWorkflowExecutionError {
    /// <p><p>The <code>StartWorkflowExecution</code> API action was called without the required parameters set.</p> <p>Some workflow execution parameters, such as the decision <code>taskList</code>, must be set to start the execution. However, these parameters might have been set as defaults when the workflow type was registered. In this case, you can omit these parameters from the <code>StartWorkflowExecution</code> call and Amazon SWF uses the values defined in the workflow type.</p> <note> <p>If these parameters aren&#39;t set and no default parameters were defined in the workflow type, this error is displayed.</p> </note></p>
    DefaultUndefinedFault(String),
    /// <p>Returned by any operation if a system imposed limitation has been reached. To address this fault you should either clean up unused resources or increase the limit by contacting AWS.</p>
    LimitExceededFault(String),
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the specified activity or workflow type was already deprecated.</p>
    TypeDeprecatedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
    /// <p>Returned by <a>StartWorkflowExecution</a> when an open execution with the same workflowId is already running in the specified domain.</p>
    WorkflowExecutionAlreadyStartedFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl StartWorkflowExecutionError {
    pub fn from_body(body: &str) -> StartWorkflowExecutionError {
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
                    "DefaultUndefinedFault" => StartWorkflowExecutionError::DefaultUndefinedFault(
                        String::from(error_message),
                    ),
                    "LimitExceededFault" => {
                        StartWorkflowExecutionError::LimitExceededFault(String::from(error_message))
                    }
                    "OperationNotPermittedFault" => {
                        StartWorkflowExecutionError::OperationNotPermittedFault(String::from(
                            error_message,
                        ))
                    }
                    "TypeDeprecatedFault" => StartWorkflowExecutionError::TypeDeprecatedFault(
                        String::from(error_message),
                    ),
                    "UnknownResourceFault" => StartWorkflowExecutionError::UnknownResourceFault(
                        String::from(error_message),
                    ),
                    "WorkflowExecutionAlreadyStartedFault" => {
                        StartWorkflowExecutionError::WorkflowExecutionAlreadyStartedFault(
                            String::from(error_message),
                        )
                    }
                    "ValidationException" => {
                        StartWorkflowExecutionError::Validation(error_message.to_string())
                    }
                    _ => StartWorkflowExecutionError::Unknown(String::from(body)),
                }
            }
            Err(_) => StartWorkflowExecutionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StartWorkflowExecutionError {
    fn from(err: serde_json::error::Error) -> StartWorkflowExecutionError {
        StartWorkflowExecutionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StartWorkflowExecutionError {
    fn from(err: CredentialsError) -> StartWorkflowExecutionError {
        StartWorkflowExecutionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartWorkflowExecutionError {
    fn from(err: HttpDispatchError) -> StartWorkflowExecutionError {
        StartWorkflowExecutionError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartWorkflowExecutionError {
    fn from(err: io::Error) -> StartWorkflowExecutionError {
        StartWorkflowExecutionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartWorkflowExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartWorkflowExecutionError {
    fn description(&self) -> &str {
        match *self {
            StartWorkflowExecutionError::DefaultUndefinedFault(ref cause) => cause,
            StartWorkflowExecutionError::LimitExceededFault(ref cause) => cause,
            StartWorkflowExecutionError::OperationNotPermittedFault(ref cause) => cause,
            StartWorkflowExecutionError::TypeDeprecatedFault(ref cause) => cause,
            StartWorkflowExecutionError::UnknownResourceFault(ref cause) => cause,
            StartWorkflowExecutionError::WorkflowExecutionAlreadyStartedFault(ref cause) => cause,
            StartWorkflowExecutionError::Validation(ref cause) => cause,
            StartWorkflowExecutionError::Credentials(ref err) => err.description(),
            StartWorkflowExecutionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StartWorkflowExecutionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by TerminateWorkflowExecution
#[derive(Debug, PartialEq)]
pub enum TerminateWorkflowExecutionError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl TerminateWorkflowExecutionError {
    pub fn from_body(body: &str) -> TerminateWorkflowExecutionError {
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
                    "OperationNotPermittedFault" => {
                        TerminateWorkflowExecutionError::OperationNotPermittedFault(String::from(
                            error_message,
                        ))
                    }
                    "UnknownResourceFault" => {
                        TerminateWorkflowExecutionError::UnknownResourceFault(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        TerminateWorkflowExecutionError::Validation(error_message.to_string())
                    }
                    _ => TerminateWorkflowExecutionError::Unknown(String::from(body)),
                }
            }
            Err(_) => TerminateWorkflowExecutionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for TerminateWorkflowExecutionError {
    fn from(err: serde_json::error::Error) -> TerminateWorkflowExecutionError {
        TerminateWorkflowExecutionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for TerminateWorkflowExecutionError {
    fn from(err: CredentialsError) -> TerminateWorkflowExecutionError {
        TerminateWorkflowExecutionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for TerminateWorkflowExecutionError {
    fn from(err: HttpDispatchError) -> TerminateWorkflowExecutionError {
        TerminateWorkflowExecutionError::HttpDispatch(err)
    }
}
impl From<io::Error> for TerminateWorkflowExecutionError {
    fn from(err: io::Error) -> TerminateWorkflowExecutionError {
        TerminateWorkflowExecutionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for TerminateWorkflowExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TerminateWorkflowExecutionError {
    fn description(&self) -> &str {
        match *self {
            TerminateWorkflowExecutionError::OperationNotPermittedFault(ref cause) => cause,
            TerminateWorkflowExecutionError::UnknownResourceFault(ref cause) => cause,
            TerminateWorkflowExecutionError::Validation(ref cause) => cause,
            TerminateWorkflowExecutionError::Credentials(ref err) => err.description(),
            TerminateWorkflowExecutionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            TerminateWorkflowExecutionError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon SWF API. Amazon SWF clients implement this trait.
pub trait Swf {
    /// <p>Returns the number of closed workflow executions within the given domain that meet the specified filtering criteria.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>tagFilter.tag</code>: String constraint. The key is <code>swf:tagFilter.tag</code>.</p> </li> <li> <p> <code>typeFilter.name</code>: String constraint. The key is <code>swf:typeFilter.name</code>.</p> </li> <li> <p> <code>typeFilter.version</code>: String constraint. The key is <code>swf:typeFilter.version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn count_closed_workflow_executions(
        &self,
        input: CountClosedWorkflowExecutionsInput,
    ) -> RusotoFuture<WorkflowExecutionCount, CountClosedWorkflowExecutionsError>;

    /// <p>Returns the number of open workflow executions within the given domain that meet the specified filtering criteria.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>tagFilter.tag</code>: String constraint. The key is <code>swf:tagFilter.tag</code>.</p> </li> <li> <p> <code>typeFilter.name</code>: String constraint. The key is <code>swf:typeFilter.name</code>.</p> </li> <li> <p> <code>typeFilter.version</code>: String constraint. The key is <code>swf:typeFilter.version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn count_open_workflow_executions(
        &self,
        input: CountOpenWorkflowExecutionsInput,
    ) -> RusotoFuture<WorkflowExecutionCount, CountOpenWorkflowExecutionsError>;

    /// <p>Returns the estimated number of activity tasks in the specified task list. The count returned is an approximation and isn't guaranteed to be exact. If you specify a task list that no activity task was ever scheduled in then <code>0</code> is returned.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the <code>taskList.name</code> parameter by using a <code>Condition</code> element with the <code>swf:taskList.name</code> key to allow the action to access only certain task lists.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn count_pending_activity_tasks(
        &self,
        input: CountPendingActivityTasksInput,
    ) -> RusotoFuture<PendingTaskCount, CountPendingActivityTasksError>;

    /// <p>Returns the estimated number of decision tasks in the specified task list. The count returned is an approximation and isn't guaranteed to be exact. If you specify a task list that no decision task was ever scheduled in then <code>0</code> is returned.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the <code>taskList.name</code> parameter by using a <code>Condition</code> element with the <code>swf:taskList.name</code> key to allow the action to access only certain task lists.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn count_pending_decision_tasks(
        &self,
        input: CountPendingDecisionTasksInput,
    ) -> RusotoFuture<PendingTaskCount, CountPendingDecisionTasksError>;

    /// <p>Deprecates the specified <i>activity type</i>. After an activity type has been deprecated, you cannot create new tasks of that activity type. Tasks of this type that were scheduled before the type was deprecated continue to run.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>activityType.name</code>: String constraint. The key is <code>swf:activityType.name</code>.</p> </li> <li> <p> <code>activityType.version</code>: String constraint. The key is <code>swf:activityType.version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn deprecate_activity_type(
        &self,
        input: DeprecateActivityTypeInput,
    ) -> RusotoFuture<(), DeprecateActivityTypeError>;

    /// <p>Deprecates the specified domain. After a domain has been deprecated it cannot be used to create new workflow executions or register new types. However, you can still use visibility actions on this domain. Deprecating a domain also deprecates all activity and workflow types registered in the domain. Executions that were started before the domain was deprecated continues to run.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn deprecate_domain(
        &self,
        input: DeprecateDomainInput,
    ) -> RusotoFuture<(), DeprecateDomainError>;

    /// <p>Deprecates the specified <i>workflow type</i>. After a workflow type has been deprecated, you cannot create new executions of that type. Executions that were started before the type was deprecated continues to run. A deprecated workflow type may still be used when calling visibility actions.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>workflowType.name</code>: String constraint. The key is <code>swf:workflowType.name</code>.</p> </li> <li> <p> <code>workflowType.version</code>: String constraint. The key is <code>swf:workflowType.version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn deprecate_workflow_type(
        &self,
        input: DeprecateWorkflowTypeInput,
    ) -> RusotoFuture<(), DeprecateWorkflowTypeError>;

    /// <p>Returns information about the specified activity type. This includes configuration settings provided when the type was registered and other general information about the type.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>activityType.name</code>: String constraint. The key is <code>swf:activityType.name</code>.</p> </li> <li> <p> <code>activityType.version</code>: String constraint. The key is <code>swf:activityType.version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn describe_activity_type(
        &self,
        input: DescribeActivityTypeInput,
    ) -> RusotoFuture<ActivityTypeDetail, DescribeActivityTypeError>;

    /// <p>Returns information about the specified domain, including description and status.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn describe_domain(
        &self,
        input: DescribeDomainInput,
    ) -> RusotoFuture<DomainDetail, DescribeDomainError>;

    /// <p>Returns information about the specified workflow execution including its type and some statistics.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn describe_workflow_execution(
        &self,
        input: DescribeWorkflowExecutionInput,
    ) -> RusotoFuture<WorkflowExecutionDetail, DescribeWorkflowExecutionError>;

    /// <p>Returns information about the specified <i>workflow type</i>. This includes configuration settings specified when the type was registered and other information such as creation date, current status, etc.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>workflowType.name</code>: String constraint. The key is <code>swf:workflowType.name</code>.</p> </li> <li> <p> <code>workflowType.version</code>: String constraint. The key is <code>swf:workflowType.version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn describe_workflow_type(
        &self,
        input: DescribeWorkflowTypeInput,
    ) -> RusotoFuture<WorkflowTypeDetail, DescribeWorkflowTypeError>;

    /// <p>Returns the history of the specified workflow execution. The results may be split into multiple pages. To retrieve subsequent pages, make the call again using the <code>nextPageToken</code> returned by the initial call.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn get_workflow_execution_history(
        &self,
        input: GetWorkflowExecutionHistoryInput,
    ) -> RusotoFuture<History, GetWorkflowExecutionHistoryError>;

    /// <p>Returns information about all activities registered in the specified domain that match the specified name and registration status. The result includes information like creation date, current status of the activity, etc. The results may be split into multiple pages. To retrieve subsequent pages, make the call again using the <code>nextPageToken</code> returned by the initial call.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn list_activity_types(
        &self,
        input: ListActivityTypesInput,
    ) -> RusotoFuture<ActivityTypeInfos, ListActivityTypesError>;

    /// <p>Returns a list of closed workflow executions in the specified domain that meet the filtering criteria. The results may be split into multiple pages. To retrieve subsequent pages, make the call again using the nextPageToken returned by the initial call.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>tagFilter.tag</code>: String constraint. The key is <code>swf:tagFilter.tag</code>.</p> </li> <li> <p> <code>typeFilter.name</code>: String constraint. The key is <code>swf:typeFilter.name</code>.</p> </li> <li> <p> <code>typeFilter.version</code>: String constraint. The key is <code>swf:typeFilter.version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn list_closed_workflow_executions(
        &self,
        input: ListClosedWorkflowExecutionsInput,
    ) -> RusotoFuture<WorkflowExecutionInfos, ListClosedWorkflowExecutionsError>;

    /// <p>Returns the list of domains registered in the account. The results may be split into multiple pages. To retrieve subsequent pages, make the call again using the nextPageToken returned by the initial call.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains. The element must be set to <code>arn:aws:swf::AccountID:domain/*</code>, where <i>AccountID</i> is the account ID, with no dashes.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn list_domains(&self, input: ListDomainsInput) -> RusotoFuture<DomainInfos, ListDomainsError>;

    /// <p>Returns a list of open workflow executions in the specified domain that meet the filtering criteria. The results may be split into multiple pages. To retrieve subsequent pages, make the call again using the nextPageToken returned by the initial call.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>tagFilter.tag</code>: String constraint. The key is <code>swf:tagFilter.tag</code>.</p> </li> <li> <p> <code>typeFilter.name</code>: String constraint. The key is <code>swf:typeFilter.name</code>.</p> </li> <li> <p> <code>typeFilter.version</code>: String constraint. The key is <code>swf:typeFilter.version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn list_open_workflow_executions(
        &self,
        input: ListOpenWorkflowExecutionsInput,
    ) -> RusotoFuture<WorkflowExecutionInfos, ListOpenWorkflowExecutionsError>;

    /// <p>Returns information about workflow types in the specified domain. The results may be split into multiple pages that can be retrieved by making the call repeatedly.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn list_workflow_types(
        &self,
        input: ListWorkflowTypesInput,
    ) -> RusotoFuture<WorkflowTypeInfos, ListWorkflowTypesError>;

    /// <p>Used by workers to get an <a>ActivityTask</a> from the specified activity <code>taskList</code>. This initiates a long poll, where the service holds the HTTP connection open and responds as soon as a task becomes available. The maximum time the service holds on to the request before responding is 60 seconds. If no task is available within 60 seconds, the poll returns an empty result. An empty result, in this context, means that an ActivityTask is returned, but that the value of taskToken is an empty string. If a task is returned, the worker should use its type to identify and process it correctly.</p> <important> <p>Workers should set their client side socket timeout to at least 70 seconds (10 seconds higher than the maximum time service may hold the poll request).</p> </important> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the <code>taskList.name</code> parameter by using a <code>Condition</code> element with the <code>swf:taskList.name</code> key to allow the action to access only certain task lists.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn poll_for_activity_task(
        &self,
        input: PollForActivityTaskInput,
    ) -> RusotoFuture<ActivityTask, PollForActivityTaskError>;

    /// <p>Used by deciders to get a <a>DecisionTask</a> from the specified decision <code>taskList</code>. A decision task may be returned for any open workflow execution that is using the specified task list. The task includes a paginated view of the history of the workflow execution. The decider should use the workflow type and the history to determine how to properly handle the task.</p> <p>This action initiates a long poll, where the service holds the HTTP connection open and responds as soon a task becomes available. If no decision task is available in the specified task list before the timeout of 60 seconds expires, an empty result is returned. An empty result, in this context, means that a DecisionTask is returned, but that the value of taskToken is an empty string.</p> <important> <p>Deciders should set their client side socket timeout to at least 70 seconds (10 seconds higher than the timeout).</p> </important> <important> <p>Because the number of workflow history events for a single workflow execution might be very large, the result returned might be split up across a number of pages. To retrieve subsequent pages, make additional calls to <code>PollForDecisionTask</code> using the <code>nextPageToken</code> returned by the initial call. Note that you do <i>not</i> call <code>GetWorkflowExecutionHistory</code> with this <code>nextPageToken</code>. Instead, call <code>PollForDecisionTask</code> again.</p> </important> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the <code>taskList.name</code> parameter by using a <code>Condition</code> element with the <code>swf:taskList.name</code> key to allow the action to access only certain task lists.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn poll_for_decision_task(
        &self,
        input: PollForDecisionTaskInput,
    ) -> RusotoFuture<DecisionTask, PollForDecisionTaskError>;

    /// <p>Used by activity workers to report to the service that the <a>ActivityTask</a> represented by the specified <code>taskToken</code> is still making progress. The worker can also specify details of the progress, for example percent complete, using the <code>details</code> parameter. This action can also be used by the worker as a mechanism to check if cancellation is being requested for the activity task. If a cancellation is being attempted for the specified task, then the boolean <code>cancelRequested</code> flag returned by the service is set to <code>true</code>.</p> <p>This action resets the <code>taskHeartbeatTimeout</code> clock. The <code>taskHeartbeatTimeout</code> is specified in <a>RegisterActivityType</a>.</p> <p>This action doesn't in itself create an event in the workflow execution history. However, if the task times out, the workflow execution history contains a <code>ActivityTaskTimedOut</code> event that contains the information from the last heartbeat generated by the activity worker.</p> <note> <p>The <code>taskStartToCloseTimeout</code> of an activity type is the maximum duration of an activity task, regardless of the number of <a>RecordActivityTaskHeartbeat</a> requests received. The <code>taskStartToCloseTimeout</code> is also specified in <a>RegisterActivityType</a>.</p> </note> <note> <p>This operation is only useful for long-lived activities to report liveliness of the task and to determine if a cancellation is being attempted.</p> </note> <important> <p>If the <code>cancelRequested</code> flag returns <code>true</code>, a cancellation is being attempted. If the worker can cancel the activity, it should respond with <a>RespondActivityTaskCanceled</a>. Otherwise, it should ignore the cancellation request.</p> </important> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn record_activity_task_heartbeat(
        &self,
        input: RecordActivityTaskHeartbeatInput,
    ) -> RusotoFuture<ActivityTaskStatus, RecordActivityTaskHeartbeatError>;

    /// <p>Registers a new <i>activity type</i> along with its configuration settings in the specified domain.</p> <important> <p>A <code>TypeAlreadyExists</code> fault is returned if the type already exists in the domain. You cannot change any configuration settings of the type after its registration, and it must be registered as a new version.</p> </important> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>defaultTaskList.name</code>: String constraint. The key is <code>swf:defaultTaskList.name</code>.</p> </li> <li> <p> <code>name</code>: String constraint. The key is <code>swf:name</code>.</p> </li> <li> <p> <code>version</code>: String constraint. The key is <code>swf:version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn register_activity_type(
        &self,
        input: RegisterActivityTypeInput,
    ) -> RusotoFuture<(), RegisterActivityTypeError>;

    /// <p>Registers a new domain.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>You cannot use an IAM policy to control domain access for this action. The name of the domain being registered is available as the resource of this action.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn register_domain(&self, input: RegisterDomainInput) -> RusotoFuture<(), RegisterDomainError>;

    /// <p>Registers a new <i>workflow type</i> and its configuration settings in the specified domain.</p> <p>The retention period for the workflow history is set by the <a>RegisterDomain</a> action.</p> <important> <p>If the type already exists, then a <code>TypeAlreadyExists</code> fault is returned. You cannot change the configuration settings of a workflow type once it is registered and it must be registered as a new version.</p> </important> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>defaultTaskList.name</code>: String constraint. The key is <code>swf:defaultTaskList.name</code>.</p> </li> <li> <p> <code>name</code>: String constraint. The key is <code>swf:name</code>.</p> </li> <li> <p> <code>version</code>: String constraint. The key is <code>swf:version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn register_workflow_type(
        &self,
        input: RegisterWorkflowTypeInput,
    ) -> RusotoFuture<(), RegisterWorkflowTypeError>;

    /// <p>Records a <code>WorkflowExecutionCancelRequested</code> event in the currently running workflow execution identified by the given domain, workflowId, and runId. This logically requests the cancellation of the workflow execution as a whole. It is up to the decider to take appropriate actions when it receives an execution history with this event.</p> <note> <p>If the runId isn't specified, the <code>WorkflowExecutionCancelRequested</code> event is recorded in the history of the current open workflow execution with the specified workflowId in the domain.</p> </note> <note> <p>Because this action allows the workflow to properly clean up and gracefully close, it should be used instead of <a>TerminateWorkflowExecution</a> when possible.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn request_cancel_workflow_execution(
        &self,
        input: RequestCancelWorkflowExecutionInput,
    ) -> RusotoFuture<(), RequestCancelWorkflowExecutionError>;

    /// <p>Used by workers to tell the service that the <a>ActivityTask</a> identified by the <code>taskToken</code> was successfully canceled. Additional <code>details</code> can be provided using the <code>details</code> argument.</p> <p>These <code>details</code> (if provided) appear in the <code>ActivityTaskCanceled</code> event added to the workflow history.</p> <important> <p>Only use this operation if the <code>canceled</code> flag of a <a>RecordActivityTaskHeartbeat</a> request returns <code>true</code> and if the activity can be safely undone or abandoned.</p> </important> <p>A task is considered open from the time that it is scheduled until it is closed. Therefore a task is reported as open while a worker is processing it. A task is closed after it has been specified in a call to <a>RespondActivityTaskCompleted</a>, RespondActivityTaskCanceled, <a>RespondActivityTaskFailed</a>, or the task has <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dg-basic.html#swf-dev-timeout-types">timed out</a>.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn respond_activity_task_canceled(
        &self,
        input: RespondActivityTaskCanceledInput,
    ) -> RusotoFuture<(), RespondActivityTaskCanceledError>;

    /// <p>Used by workers to tell the service that the <a>ActivityTask</a> identified by the <code>taskToken</code> completed successfully with a <code>result</code> (if provided). The <code>result</code> appears in the <code>ActivityTaskCompleted</code> event in the workflow history.</p> <important> <p>If the requested task doesn't complete successfully, use <a>RespondActivityTaskFailed</a> instead. If the worker finds that the task is canceled through the <code>canceled</code> flag returned by <a>RecordActivityTaskHeartbeat</a>, it should cancel the task, clean up and then call <a>RespondActivityTaskCanceled</a>.</p> </important> <p>A task is considered open from the time that it is scheduled until it is closed. Therefore a task is reported as open while a worker is processing it. A task is closed after it has been specified in a call to RespondActivityTaskCompleted, <a>RespondActivityTaskCanceled</a>, <a>RespondActivityTaskFailed</a>, or the task has <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dg-basic.html#swf-dev-timeout-types">timed out</a>.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn respond_activity_task_completed(
        &self,
        input: RespondActivityTaskCompletedInput,
    ) -> RusotoFuture<(), RespondActivityTaskCompletedError>;

    /// <p>Used by workers to tell the service that the <a>ActivityTask</a> identified by the <code>taskToken</code> has failed with <code>reason</code> (if specified). The <code>reason</code> and <code>details</code> appear in the <code>ActivityTaskFailed</code> event added to the workflow history.</p> <p>A task is considered open from the time that it is scheduled until it is closed. Therefore a task is reported as open while a worker is processing it. A task is closed after it has been specified in a call to <a>RespondActivityTaskCompleted</a>, <a>RespondActivityTaskCanceled</a>, RespondActivityTaskFailed, or the task has <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dg-basic.html#swf-dev-timeout-types">timed out</a>.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn respond_activity_task_failed(
        &self,
        input: RespondActivityTaskFailedInput,
    ) -> RusotoFuture<(), RespondActivityTaskFailedError>;

    /// <p>Used by deciders to tell the service that the <a>DecisionTask</a> identified by the <code>taskToken</code> has successfully completed. The <code>decisions</code> argument specifies the list of decisions made while processing the task.</p> <p>A <code>DecisionTaskCompleted</code> event is added to the workflow history. The <code>executionContext</code> specified is attached to the event in the workflow execution history.</p> <p> <b>Access Control</b> </p> <p>If an IAM policy grants permission to use <code>RespondDecisionTaskCompleted</code>, it can express permissions for the list of decisions in the <code>decisions</code> parameter. Each of the decisions has one or more parameters, much like a regular API call. To allow for policies to be as readable as possible, you can express permissions on decisions as if they were actual API calls, including applying conditions to some parameters. For more information, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn respond_decision_task_completed(
        &self,
        input: RespondDecisionTaskCompletedInput,
    ) -> RusotoFuture<(), RespondDecisionTaskCompletedError>;

    /// <p>Records a <code>WorkflowExecutionSignaled</code> event in the workflow execution history and creates a decision task for the workflow execution identified by the given domain, workflowId and runId. The event is recorded with the specified user defined signalName and input (if provided).</p> <note> <p>If a runId isn't specified, then the <code>WorkflowExecutionSignaled</code> event is recorded in the history of the current open workflow with the matching workflowId in the domain.</p> </note> <note> <p>If the specified workflow execution isn't open, this method fails with <code>UnknownResource</code>.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn signal_workflow_execution(
        &self,
        input: SignalWorkflowExecutionInput,
    ) -> RusotoFuture<(), SignalWorkflowExecutionError>;

    /// <p>Starts an execution of the workflow type in the specified domain using the provided <code>workflowId</code> and input data.</p> <p>This action returns the newly started workflow execution.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>tagList.member.0</code>: The key is <code>swf:tagList.member.0</code>.</p> </li> <li> <p> <code>tagList.member.1</code>: The key is <code>swf:tagList.member.1</code>.</p> </li> <li> <p> <code>tagList.member.2</code>: The key is <code>swf:tagList.member.2</code>.</p> </li> <li> <p> <code>tagList.member.3</code>: The key is <code>swf:tagList.member.3</code>.</p> </li> <li> <p> <code>tagList.member.4</code>: The key is <code>swf:tagList.member.4</code>.</p> </li> <li> <p> <code>taskList</code>: String constraint. The key is <code>swf:taskList.name</code>.</p> </li> <li> <p> <code>workflowType.name</code>: String constraint. The key is <code>swf:workflowType.name</code>.</p> </li> <li> <p> <code>workflowType.version</code>: String constraint. The key is <code>swf:workflowType.version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn start_workflow_execution(
        &self,
        input: StartWorkflowExecutionInput,
    ) -> RusotoFuture<Run, StartWorkflowExecutionError>;

    /// <p>Records a <code>WorkflowExecutionTerminated</code> event and forces closure of the workflow execution identified by the given domain, runId, and workflowId. The child policy, registered with the workflow type or specified when starting this execution, is applied to any open child workflow executions of this workflow execution.</p> <important> <p>If the identified workflow execution was in progress, it is terminated immediately.</p> </important> <note> <p>If a runId isn't specified, then the <code>WorkflowExecutionTerminated</code> event is recorded in the history of the current open workflow with the matching workflowId in the domain.</p> </note> <note> <p>You should consider using <a>RequestCancelWorkflowExecution</a> action instead because it allows the workflow to gracefully close while <a>TerminateWorkflowExecution</a> doesn't.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn terminate_workflow_execution(
        &self,
        input: TerminateWorkflowExecutionInput,
    ) -> RusotoFuture<(), TerminateWorkflowExecutionError>;
}
/// A client for the Amazon SWF API.
pub struct SwfClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl SwfClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> SwfClient {
        SwfClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> SwfClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        SwfClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> Swf for SwfClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p>Returns the number of closed workflow executions within the given domain that meet the specified filtering criteria.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>tagFilter.tag</code>: String constraint. The key is <code>swf:tagFilter.tag</code>.</p> </li> <li> <p> <code>typeFilter.name</code>: String constraint. The key is <code>swf:typeFilter.name</code>.</p> </li> <li> <p> <code>typeFilter.version</code>: String constraint. The key is <code>swf:typeFilter.version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn count_closed_workflow_executions(
        &self,
        input: CountClosedWorkflowExecutionsInput,
    ) -> RusotoFuture<WorkflowExecutionCount, CountClosedWorkflowExecutionsError> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "SimpleWorkflowService.CountClosedWorkflowExecutions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<WorkflowExecutionCount>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CountClosedWorkflowExecutionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns the number of open workflow executions within the given domain that meet the specified filtering criteria.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>tagFilter.tag</code>: String constraint. The key is <code>swf:tagFilter.tag</code>.</p> </li> <li> <p> <code>typeFilter.name</code>: String constraint. The key is <code>swf:typeFilter.name</code>.</p> </li> <li> <p> <code>typeFilter.version</code>: String constraint. The key is <code>swf:typeFilter.version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn count_open_workflow_executions(
        &self,
        input: CountOpenWorkflowExecutionsInput,
    ) -> RusotoFuture<WorkflowExecutionCount, CountOpenWorkflowExecutionsError> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "SimpleWorkflowService.CountOpenWorkflowExecutions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<WorkflowExecutionCount>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CountOpenWorkflowExecutionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns the estimated number of activity tasks in the specified task list. The count returned is an approximation and isn't guaranteed to be exact. If you specify a task list that no activity task was ever scheduled in then <code>0</code> is returned.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the <code>taskList.name</code> parameter by using a <code>Condition</code> element with the <code>swf:taskList.name</code> key to allow the action to access only certain task lists.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn count_pending_activity_tasks(
        &self,
        input: CountPendingActivityTasksInput,
    ) -> RusotoFuture<PendingTaskCount, CountPendingActivityTasksError> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "SimpleWorkflowService.CountPendingActivityTasks",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PendingTaskCount>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CountPendingActivityTasksError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns the estimated number of decision tasks in the specified task list. The count returned is an approximation and isn't guaranteed to be exact. If you specify a task list that no decision task was ever scheduled in then <code>0</code> is returned.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the <code>taskList.name</code> parameter by using a <code>Condition</code> element with the <code>swf:taskList.name</code> key to allow the action to access only certain task lists.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn count_pending_decision_tasks(
        &self,
        input: CountPendingDecisionTasksInput,
    ) -> RusotoFuture<PendingTaskCount, CountPendingDecisionTasksError> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "SimpleWorkflowService.CountPendingDecisionTasks",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PendingTaskCount>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CountPendingDecisionTasksError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deprecates the specified <i>activity type</i>. After an activity type has been deprecated, you cannot create new tasks of that activity type. Tasks of this type that were scheduled before the type was deprecated continue to run.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>activityType.name</code>: String constraint. The key is <code>swf:activityType.name</code>.</p> </li> <li> <p> <code>activityType.version</code>: String constraint. The key is <code>swf:activityType.version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn deprecate_activity_type(
        &self,
        input: DeprecateActivityTypeInput,
    ) -> RusotoFuture<(), DeprecateActivityTypeError> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "SimpleWorkflowService.DeprecateActivityType",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeprecateActivityTypeError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deprecates the specified domain. After a domain has been deprecated it cannot be used to create new workflow executions or register new types. However, you can still use visibility actions on this domain. Deprecating a domain also deprecates all activity and workflow types registered in the domain. Executions that were started before the domain was deprecated continues to run.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn deprecate_domain(
        &self,
        input: DeprecateDomainInput,
    ) -> RusotoFuture<(), DeprecateDomainError> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "SimpleWorkflowService.DeprecateDomain");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeprecateDomainError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deprecates the specified <i>workflow type</i>. After a workflow type has been deprecated, you cannot create new executions of that type. Executions that were started before the type was deprecated continues to run. A deprecated workflow type may still be used when calling visibility actions.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>workflowType.name</code>: String constraint. The key is <code>swf:workflowType.name</code>.</p> </li> <li> <p> <code>workflowType.version</code>: String constraint. The key is <code>swf:workflowType.version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn deprecate_workflow_type(
        &self,
        input: DeprecateWorkflowTypeInput,
    ) -> RusotoFuture<(), DeprecateWorkflowTypeError> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "SimpleWorkflowService.DeprecateWorkflowType",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeprecateWorkflowTypeError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns information about the specified activity type. This includes configuration settings provided when the type was registered and other general information about the type.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>activityType.name</code>: String constraint. The key is <code>swf:activityType.name</code>.</p> </li> <li> <p> <code>activityType.version</code>: String constraint. The key is <code>swf:activityType.version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn describe_activity_type(
        &self,
        input: DescribeActivityTypeInput,
    ) -> RusotoFuture<ActivityTypeDetail, DescribeActivityTypeError> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "SimpleWorkflowService.DescribeActivityType");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ActivityTypeDetail>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeActivityTypeError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns information about the specified domain, including description and status.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn describe_domain(
        &self,
        input: DescribeDomainInput,
    ) -> RusotoFuture<DomainDetail, DescribeDomainError> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "SimpleWorkflowService.DescribeDomain");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DomainDetail>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeDomainError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns information about the specified workflow execution including its type and some statistics.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn describe_workflow_execution(
        &self,
        input: DescribeWorkflowExecutionInput,
    ) -> RusotoFuture<WorkflowExecutionDetail, DescribeWorkflowExecutionError> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "SimpleWorkflowService.DescribeWorkflowExecution",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<WorkflowExecutionDetail>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeWorkflowExecutionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns information about the specified <i>workflow type</i>. This includes configuration settings specified when the type was registered and other information such as creation date, current status, etc.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>workflowType.name</code>: String constraint. The key is <code>swf:workflowType.name</code>.</p> </li> <li> <p> <code>workflowType.version</code>: String constraint. The key is <code>swf:workflowType.version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn describe_workflow_type(
        &self,
        input: DescribeWorkflowTypeInput,
    ) -> RusotoFuture<WorkflowTypeDetail, DescribeWorkflowTypeError> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "SimpleWorkflowService.DescribeWorkflowType");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<WorkflowTypeDetail>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeWorkflowTypeError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns the history of the specified workflow execution. The results may be split into multiple pages. To retrieve subsequent pages, make the call again using the <code>nextPageToken</code> returned by the initial call.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn get_workflow_execution_history(
        &self,
        input: GetWorkflowExecutionHistoryInput,
    ) -> RusotoFuture<History, GetWorkflowExecutionHistoryError> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "SimpleWorkflowService.GetWorkflowExecutionHistory",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<History>(String::from_utf8_lossy(body.as_ref()).as_ref())
                        .unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetWorkflowExecutionHistoryError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns information about all activities registered in the specified domain that match the specified name and registration status. The result includes information like creation date, current status of the activity, etc. The results may be split into multiple pages. To retrieve subsequent pages, make the call again using the <code>nextPageToken</code> returned by the initial call.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn list_activity_types(
        &self,
        input: ListActivityTypesInput,
    ) -> RusotoFuture<ActivityTypeInfos, ListActivityTypesError> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "SimpleWorkflowService.ListActivityTypes");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ActivityTypeInfos>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListActivityTypesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a list of closed workflow executions in the specified domain that meet the filtering criteria. The results may be split into multiple pages. To retrieve subsequent pages, make the call again using the nextPageToken returned by the initial call.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>tagFilter.tag</code>: String constraint. The key is <code>swf:tagFilter.tag</code>.</p> </li> <li> <p> <code>typeFilter.name</code>: String constraint. The key is <code>swf:typeFilter.name</code>.</p> </li> <li> <p> <code>typeFilter.version</code>: String constraint. The key is <code>swf:typeFilter.version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn list_closed_workflow_executions(
        &self,
        input: ListClosedWorkflowExecutionsInput,
    ) -> RusotoFuture<WorkflowExecutionInfos, ListClosedWorkflowExecutionsError> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "SimpleWorkflowService.ListClosedWorkflowExecutions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<WorkflowExecutionInfos>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListClosedWorkflowExecutionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns the list of domains registered in the account. The results may be split into multiple pages. To retrieve subsequent pages, make the call again using the nextPageToken returned by the initial call.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains. The element must be set to <code>arn:aws:swf::AccountID:domain/*</code>, where <i>AccountID</i> is the account ID, with no dashes.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn list_domains(&self, input: ListDomainsInput) -> RusotoFuture<DomainInfos, ListDomainsError> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "SimpleWorkflowService.ListDomains");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DomainInfos>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListDomainsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a list of open workflow executions in the specified domain that meet the filtering criteria. The results may be split into multiple pages. To retrieve subsequent pages, make the call again using the nextPageToken returned by the initial call.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>tagFilter.tag</code>: String constraint. The key is <code>swf:tagFilter.tag</code>.</p> </li> <li> <p> <code>typeFilter.name</code>: String constraint. The key is <code>swf:typeFilter.name</code>.</p> </li> <li> <p> <code>typeFilter.version</code>: String constraint. The key is <code>swf:typeFilter.version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn list_open_workflow_executions(
        &self,
        input: ListOpenWorkflowExecutionsInput,
    ) -> RusotoFuture<WorkflowExecutionInfos, ListOpenWorkflowExecutionsError> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "SimpleWorkflowService.ListOpenWorkflowExecutions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<WorkflowExecutionInfos>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListOpenWorkflowExecutionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns information about workflow types in the specified domain. The results may be split into multiple pages that can be retrieved by making the call repeatedly.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn list_workflow_types(
        &self,
        input: ListWorkflowTypesInput,
    ) -> RusotoFuture<WorkflowTypeInfos, ListWorkflowTypesError> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "SimpleWorkflowService.ListWorkflowTypes");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<WorkflowTypeInfos>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListWorkflowTypesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Used by workers to get an <a>ActivityTask</a> from the specified activity <code>taskList</code>. This initiates a long poll, where the service holds the HTTP connection open and responds as soon as a task becomes available. The maximum time the service holds on to the request before responding is 60 seconds. If no task is available within 60 seconds, the poll returns an empty result. An empty result, in this context, means that an ActivityTask is returned, but that the value of taskToken is an empty string. If a task is returned, the worker should use its type to identify and process it correctly.</p> <important> <p>Workers should set their client side socket timeout to at least 70 seconds (10 seconds higher than the maximum time service may hold the poll request).</p> </important> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the <code>taskList.name</code> parameter by using a <code>Condition</code> element with the <code>swf:taskList.name</code> key to allow the action to access only certain task lists.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn poll_for_activity_task(
        &self,
        input: PollForActivityTaskInput,
    ) -> RusotoFuture<ActivityTask, PollForActivityTaskError> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "SimpleWorkflowService.PollForActivityTask");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ActivityTask>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PollForActivityTaskError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Used by deciders to get a <a>DecisionTask</a> from the specified decision <code>taskList</code>. A decision task may be returned for any open workflow execution that is using the specified task list. The task includes a paginated view of the history of the workflow execution. The decider should use the workflow type and the history to determine how to properly handle the task.</p> <p>This action initiates a long poll, where the service holds the HTTP connection open and responds as soon a task becomes available. If no decision task is available in the specified task list before the timeout of 60 seconds expires, an empty result is returned. An empty result, in this context, means that a DecisionTask is returned, but that the value of taskToken is an empty string.</p> <important> <p>Deciders should set their client side socket timeout to at least 70 seconds (10 seconds higher than the timeout).</p> </important> <important> <p>Because the number of workflow history events for a single workflow execution might be very large, the result returned might be split up across a number of pages. To retrieve subsequent pages, make additional calls to <code>PollForDecisionTask</code> using the <code>nextPageToken</code> returned by the initial call. Note that you do <i>not</i> call <code>GetWorkflowExecutionHistory</code> with this <code>nextPageToken</code>. Instead, call <code>PollForDecisionTask</code> again.</p> </important> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the <code>taskList.name</code> parameter by using a <code>Condition</code> element with the <code>swf:taskList.name</code> key to allow the action to access only certain task lists.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn poll_for_decision_task(
        &self,
        input: PollForDecisionTaskInput,
    ) -> RusotoFuture<DecisionTask, PollForDecisionTaskError> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "SimpleWorkflowService.PollForDecisionTask");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DecisionTask>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PollForDecisionTaskError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Used by activity workers to report to the service that the <a>ActivityTask</a> represented by the specified <code>taskToken</code> is still making progress. The worker can also specify details of the progress, for example percent complete, using the <code>details</code> parameter. This action can also be used by the worker as a mechanism to check if cancellation is being requested for the activity task. If a cancellation is being attempted for the specified task, then the boolean <code>cancelRequested</code> flag returned by the service is set to <code>true</code>.</p> <p>This action resets the <code>taskHeartbeatTimeout</code> clock. The <code>taskHeartbeatTimeout</code> is specified in <a>RegisterActivityType</a>.</p> <p>This action doesn't in itself create an event in the workflow execution history. However, if the task times out, the workflow execution history contains a <code>ActivityTaskTimedOut</code> event that contains the information from the last heartbeat generated by the activity worker.</p> <note> <p>The <code>taskStartToCloseTimeout</code> of an activity type is the maximum duration of an activity task, regardless of the number of <a>RecordActivityTaskHeartbeat</a> requests received. The <code>taskStartToCloseTimeout</code> is also specified in <a>RegisterActivityType</a>.</p> </note> <note> <p>This operation is only useful for long-lived activities to report liveliness of the task and to determine if a cancellation is being attempted.</p> </note> <important> <p>If the <code>cancelRequested</code> flag returns <code>true</code>, a cancellation is being attempted. If the worker can cancel the activity, it should respond with <a>RespondActivityTaskCanceled</a>. Otherwise, it should ignore the cancellation request.</p> </important> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn record_activity_task_heartbeat(
        &self,
        input: RecordActivityTaskHeartbeatInput,
    ) -> RusotoFuture<ActivityTaskStatus, RecordActivityTaskHeartbeatError> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "SimpleWorkflowService.RecordActivityTaskHeartbeat",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ActivityTaskStatus>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RecordActivityTaskHeartbeatError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Registers a new <i>activity type</i> along with its configuration settings in the specified domain.</p> <important> <p>A <code>TypeAlreadyExists</code> fault is returned if the type already exists in the domain. You cannot change any configuration settings of the type after its registration, and it must be registered as a new version.</p> </important> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>defaultTaskList.name</code>: String constraint. The key is <code>swf:defaultTaskList.name</code>.</p> </li> <li> <p> <code>name</code>: String constraint. The key is <code>swf:name</code>.</p> </li> <li> <p> <code>version</code>: String constraint. The key is <code>swf:version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn register_activity_type(
        &self,
        input: RegisterActivityTypeInput,
    ) -> RusotoFuture<(), RegisterActivityTypeError> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "SimpleWorkflowService.RegisterActivityType");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RegisterActivityTypeError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Registers a new domain.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>You cannot use an IAM policy to control domain access for this action. The name of the domain being registered is available as the resource of this action.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn register_domain(&self, input: RegisterDomainInput) -> RusotoFuture<(), RegisterDomainError> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "SimpleWorkflowService.RegisterDomain");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RegisterDomainError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Registers a new <i>workflow type</i> and its configuration settings in the specified domain.</p> <p>The retention period for the workflow history is set by the <a>RegisterDomain</a> action.</p> <important> <p>If the type already exists, then a <code>TypeAlreadyExists</code> fault is returned. You cannot change the configuration settings of a workflow type once it is registered and it must be registered as a new version.</p> </important> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>defaultTaskList.name</code>: String constraint. The key is <code>swf:defaultTaskList.name</code>.</p> </li> <li> <p> <code>name</code>: String constraint. The key is <code>swf:name</code>.</p> </li> <li> <p> <code>version</code>: String constraint. The key is <code>swf:version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn register_workflow_type(
        &self,
        input: RegisterWorkflowTypeInput,
    ) -> RusotoFuture<(), RegisterWorkflowTypeError> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "SimpleWorkflowService.RegisterWorkflowType");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RegisterWorkflowTypeError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Records a <code>WorkflowExecutionCancelRequested</code> event in the currently running workflow execution identified by the given domain, workflowId, and runId. This logically requests the cancellation of the workflow execution as a whole. It is up to the decider to take appropriate actions when it receives an execution history with this event.</p> <note> <p>If the runId isn't specified, the <code>WorkflowExecutionCancelRequested</code> event is recorded in the history of the current open workflow execution with the specified workflowId in the domain.</p> </note> <note> <p>Because this action allows the workflow to properly clean up and gracefully close, it should be used instead of <a>TerminateWorkflowExecution</a> when possible.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn request_cancel_workflow_execution(
        &self,
        input: RequestCancelWorkflowExecutionInput,
    ) -> RusotoFuture<(), RequestCancelWorkflowExecutionError> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "SimpleWorkflowService.RequestCancelWorkflowExecution",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RequestCancelWorkflowExecutionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Used by workers to tell the service that the <a>ActivityTask</a> identified by the <code>taskToken</code> was successfully canceled. Additional <code>details</code> can be provided using the <code>details</code> argument.</p> <p>These <code>details</code> (if provided) appear in the <code>ActivityTaskCanceled</code> event added to the workflow history.</p> <important> <p>Only use this operation if the <code>canceled</code> flag of a <a>RecordActivityTaskHeartbeat</a> request returns <code>true</code> and if the activity can be safely undone or abandoned.</p> </important> <p>A task is considered open from the time that it is scheduled until it is closed. Therefore a task is reported as open while a worker is processing it. A task is closed after it has been specified in a call to <a>RespondActivityTaskCompleted</a>, RespondActivityTaskCanceled, <a>RespondActivityTaskFailed</a>, or the task has <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dg-basic.html#swf-dev-timeout-types">timed out</a>.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn respond_activity_task_canceled(
        &self,
        input: RespondActivityTaskCanceledInput,
    ) -> RusotoFuture<(), RespondActivityTaskCanceledError> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "SimpleWorkflowService.RespondActivityTaskCanceled",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RespondActivityTaskCanceledError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Used by workers to tell the service that the <a>ActivityTask</a> identified by the <code>taskToken</code> completed successfully with a <code>result</code> (if provided). The <code>result</code> appears in the <code>ActivityTaskCompleted</code> event in the workflow history.</p> <important> <p>If the requested task doesn't complete successfully, use <a>RespondActivityTaskFailed</a> instead. If the worker finds that the task is canceled through the <code>canceled</code> flag returned by <a>RecordActivityTaskHeartbeat</a>, it should cancel the task, clean up and then call <a>RespondActivityTaskCanceled</a>.</p> </important> <p>A task is considered open from the time that it is scheduled until it is closed. Therefore a task is reported as open while a worker is processing it. A task is closed after it has been specified in a call to RespondActivityTaskCompleted, <a>RespondActivityTaskCanceled</a>, <a>RespondActivityTaskFailed</a>, or the task has <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dg-basic.html#swf-dev-timeout-types">timed out</a>.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn respond_activity_task_completed(
        &self,
        input: RespondActivityTaskCompletedInput,
    ) -> RusotoFuture<(), RespondActivityTaskCompletedError> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "SimpleWorkflowService.RespondActivityTaskCompleted",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RespondActivityTaskCompletedError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Used by workers to tell the service that the <a>ActivityTask</a> identified by the <code>taskToken</code> has failed with <code>reason</code> (if specified). The <code>reason</code> and <code>details</code> appear in the <code>ActivityTaskFailed</code> event added to the workflow history.</p> <p>A task is considered open from the time that it is scheduled until it is closed. Therefore a task is reported as open while a worker is processing it. A task is closed after it has been specified in a call to <a>RespondActivityTaskCompleted</a>, <a>RespondActivityTaskCanceled</a>, RespondActivityTaskFailed, or the task has <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dg-basic.html#swf-dev-timeout-types">timed out</a>.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn respond_activity_task_failed(
        &self,
        input: RespondActivityTaskFailedInput,
    ) -> RusotoFuture<(), RespondActivityTaskFailedError> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "SimpleWorkflowService.RespondActivityTaskFailed",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RespondActivityTaskFailedError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Used by deciders to tell the service that the <a>DecisionTask</a> identified by the <code>taskToken</code> has successfully completed. The <code>decisions</code> argument specifies the list of decisions made while processing the task.</p> <p>A <code>DecisionTaskCompleted</code> event is added to the workflow history. The <code>executionContext</code> specified is attached to the event in the workflow execution history.</p> <p> <b>Access Control</b> </p> <p>If an IAM policy grants permission to use <code>RespondDecisionTaskCompleted</code>, it can express permissions for the list of decisions in the <code>decisions</code> parameter. Each of the decisions has one or more parameters, much like a regular API call. To allow for policies to be as readable as possible, you can express permissions on decisions as if they were actual API calls, including applying conditions to some parameters. For more information, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn respond_decision_task_completed(
        &self,
        input: RespondDecisionTaskCompletedInput,
    ) -> RusotoFuture<(), RespondDecisionTaskCompletedError> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "SimpleWorkflowService.RespondDecisionTaskCompleted",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RespondDecisionTaskCompletedError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Records a <code>WorkflowExecutionSignaled</code> event in the workflow execution history and creates a decision task for the workflow execution identified by the given domain, workflowId and runId. The event is recorded with the specified user defined signalName and input (if provided).</p> <note> <p>If a runId isn't specified, then the <code>WorkflowExecutionSignaled</code> event is recorded in the history of the current open workflow with the matching workflowId in the domain.</p> </note> <note> <p>If the specified workflow execution isn't open, this method fails with <code>UnknownResource</code>.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn signal_workflow_execution(
        &self,
        input: SignalWorkflowExecutionInput,
    ) -> RusotoFuture<(), SignalWorkflowExecutionError> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "SimpleWorkflowService.SignalWorkflowExecution",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SignalWorkflowExecutionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Starts an execution of the workflow type in the specified domain using the provided <code>workflowId</code> and input data.</p> <p>This action returns the newly started workflow execution.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>tagList.member.0</code>: The key is <code>swf:tagList.member.0</code>.</p> </li> <li> <p> <code>tagList.member.1</code>: The key is <code>swf:tagList.member.1</code>.</p> </li> <li> <p> <code>tagList.member.2</code>: The key is <code>swf:tagList.member.2</code>.</p> </li> <li> <p> <code>tagList.member.3</code>: The key is <code>swf:tagList.member.3</code>.</p> </li> <li> <p> <code>tagList.member.4</code>: The key is <code>swf:tagList.member.4</code>.</p> </li> <li> <p> <code>taskList</code>: String constraint. The key is <code>swf:taskList.name</code>.</p> </li> <li> <p> <code>workflowType.name</code>: String constraint. The key is <code>swf:workflowType.name</code>.</p> </li> <li> <p> <code>workflowType.version</code>: String constraint. The key is <code>swf:workflowType.version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn start_workflow_execution(
        &self,
        input: StartWorkflowExecutionInput,
    ) -> RusotoFuture<Run, StartWorkflowExecutionError> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "SimpleWorkflowService.StartWorkflowExecution",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<Run>(String::from_utf8_lossy(body.as_ref()).as_ref())
                        .unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(StartWorkflowExecutionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Records a <code>WorkflowExecutionTerminated</code> event and forces closure of the workflow execution identified by the given domain, runId, and workflowId. The child policy, registered with the workflow type or specified when starting this execution, is applied to any open child workflow executions of this workflow execution.</p> <important> <p>If the identified workflow execution was in progress, it is terminated immediately.</p> </important> <note> <p>If a runId isn't specified, then the <code>WorkflowExecutionTerminated</code> event is recorded in the history of the current open workflow with the matching workflowId in the domain.</p> </note> <note> <p>You should consider using <a>RequestCancelWorkflowExecution</a> action instead because it allows the workflow to gracefully close while <a>TerminateWorkflowExecution</a> doesn't.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="http://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    fn terminate_workflow_execution(
        &self,
        input: TerminateWorkflowExecutionInput,
    ) -> RusotoFuture<(), TerminateWorkflowExecutionError> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "SimpleWorkflowService.TerminateWorkflowExecution",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(TerminateWorkflowExecutionError::from_body(
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
