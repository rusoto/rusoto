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
/// <p>Unit of work sent to an activity worker.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p> The priority to assign to the scheduled activity task. If set, this overrides any default priority value that was assigned when the activity type was registered.</p> <p>Valid values are integers that range from Java's <code>Integer.MIN_VALUE</code> (-2147483648) to <code>Integer.MAX_VALUE</code> (2147483647). Higher numbers indicate higher priority.</p> <p>For more information about setting task priority, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/programming-priority.html">Setting Task Priority</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    #[serde(rename = "taskPriority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_priority: Option<String>,
}

/// <p>Provides the details of the <code>ActivityTaskStarted</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ActivityTaskStatus {
    /// <p>Set to <code>true</code> if cancellation of the task is requested.</p>
    #[serde(rename = "cancelRequested")]
    pub cancel_requested: bool,
}

/// <p>Provides the details of the <code>ActivityTaskTimedOut</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ActivityTypeConfiguration {
    /// <p> The default maximum time, in seconds, before which a worker processing a task must report progress by calling <a>RecordActivityTaskHeartbeat</a>.</p> <p>You can specify this value only when <i>registering</i> an activity type. The registered default value can be overridden when you schedule a task through the <code>ScheduleActivityTask</code> <a>Decision</a>. If the activity worker subsequently attempts to record a heartbeat or returns a result, the activity worker receives an <code>UnknownResource</code> fault. In this case, Amazon SWF no longer considers the activity task to be valid; the activity worker should clean up the activity task.</p> <p>The duration is specified in seconds, an integer greater than or equal to <code>0</code>. You can use <code>NONE</code> to specify unlimited duration.</p>
    #[serde(rename = "defaultTaskHeartbeatTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_task_heartbeat_timeout: Option<String>,
    /// <p> The default task list specified for this activity type at registration. This default is used if a task list isn't provided when a task is scheduled through the <code>ScheduleActivityTask</code> <a>Decision</a>. You can override the default registered task list when scheduling a task through the <code>ScheduleActivityTask</code> <a>Decision</a>.</p>
    #[serde(rename = "defaultTaskList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_task_list: Option<TaskList>,
    /// <p> The default task priority for tasks of this activity type, specified at registration. If not set, then <code>0</code> is used as the default priority. This default can be overridden when scheduling an activity task.</p> <p>Valid values are integers that range from Java's <code>Integer.MIN_VALUE</code> (-2147483648) to <code>Integer.MAX_VALUE</code> (2147483647). Higher numbers indicate higher priority.</p> <p>For more information about setting task priority, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/programming-priority.html">Setting Task Priority</a> in the <i>Amazon SWF Developer Guide</i>.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ActivityTypeInfos {
    /// <p>If a <code>NextPageToken</code> was returned by a previous call, there are more results available. To retrieve the next page of results, make the call again using the returned token in <code>nextPageToken</code>. Keep all other arguments unchanged.</p> <p>The configured <code>maximumPageSize</code> determines how many results can be returned in a single call.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>List of activity type information.</p>
    #[serde(rename = "typeInfos")]
    pub type_infos: Vec<ActivityTypeInfo>,
}

/// <p>Provides the details of the <code>CancelTimer</code> decision.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this decision's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CancelTimerDecisionAttributes {
    /// <p> The unique ID of the timer to cancel.</p>
    #[serde(rename = "timerId")]
    pub timer_id: String,
}

/// <p>Provides the details of the <code>CancelTimerFailed</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CancelTimerFailedEventAttributes {
    /// <p><p>The cause of the failure. This information is generated by the system and can be useful for diagnostic purposes.</p> <note> <p>If <code>cause</code> is set to <code>OPERATION<em>NOT</em>PERMITTED</code>, the decision failed because it lacked sufficient permissions. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p> </note></p>
    #[serde(rename = "cause")]
    pub cause: String,
    /// <p>The ID of the <code>DecisionTaskCompleted</code> event corresponding to the decision task that resulted in the <code>CancelTimer</code> decision to cancel this timer. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "decisionTaskCompletedEventId")]
    pub decision_task_completed_event_id: i64,
    /// <p>The timerId provided in the <code>CancelTimer</code> decision that failed.</p>
    #[serde(rename = "timerId")]
    pub timer_id: String,
}

/// <p>Provides the details of the <code>CancelWorkflowExecution</code> decision.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this decision's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CancelWorkflowExecutionDecisionAttributes {
    /// <p> Details of the cancellation.</p>
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

/// <p>Provides the details of the <code>CancelWorkflowExecutionFailed</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CancelWorkflowExecutionFailedEventAttributes {
    /// <p><p>The cause of the failure. This information is generated by the system and can be useful for diagnostic purposes.</p> <note> <p>If <code>cause</code> is set to <code>OPERATION<em>NOT</em>PERMITTED</code>, the decision failed because it lacked sufficient permissions. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p> </note></p>
    #[serde(rename = "cause")]
    pub cause: String,
    /// <p>The ID of the <code>DecisionTaskCompleted</code> event corresponding to the decision task that resulted in the <code>CancelWorkflowExecution</code> decision for this cancellation request. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "decisionTaskCompletedEventId")]
    pub decision_task_completed_event_id: i64,
}

/// <p>Provide details of the <code>ChildWorkflowExecutionCanceled</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CloseStatusFilter {
    /// <p> The close status that must match the close status of an execution for it to meet the criteria of this filter.</p>
    #[serde(rename = "status")]
    pub status: String,
}

/// <p>Provides the details of the <code>CompleteWorkflowExecution</code> decision.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this decision's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CompleteWorkflowExecutionDecisionAttributes {
    /// <p>The result of the workflow execution. The form of the result is implementation defined.</p>
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}

/// <p>Provides the details of the <code>CompleteWorkflowExecutionFailed</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CompleteWorkflowExecutionFailedEventAttributes {
    /// <p><p>The cause of the failure. This information is generated by the system and can be useful for diagnostic purposes.</p> <note> <p>If <code>cause</code> is set to <code>OPERATION<em>NOT</em>PERMITTED</code>, the decision failed because it lacked sufficient permissions. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p> </note></p>
    #[serde(rename = "cause")]
    pub cause: String,
    /// <p>The ID of the <code>DecisionTaskCompleted</code> event corresponding to the decision task that resulted in the <code>CompleteWorkflowExecution</code> decision to complete this execution. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "decisionTaskCompletedEventId")]
    pub decision_task_completed_event_id: i64,
}

/// <p>Provides the details of the <code>ContinueAsNewWorkflowExecution</code> decision.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this decision's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>tag</code> – A tag used to identify the workflow execution</p> </li> <li> <p> <code>taskList</code> – String constraint. The key is <code>swf:taskList.name</code>.</p> </li> <li> <p> <code>workflowType.version</code> – String constraint. The key is <code>swf:workflowType.version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p> The task priority that, if set, specifies the priority for the decision tasks for this workflow execution. This overrides the defaultTaskPriority specified when registering the workflow type. Valid values are integers that range from Java's <code>Integer.MIN_VALUE</code> (-2147483648) to <code>Integer.MAX_VALUE</code> (2147483647). Higher numbers indicate higher priority.</p> <p>For more information about setting task priority, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/programming-priority.html">Setting Task Priority</a> in the <i>Amazon SWF Developer Guide</i>.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ContinueAsNewWorkflowExecutionFailedEventAttributes {
    /// <p><p>The cause of the failure. This information is generated by the system and can be useful for diagnostic purposes.</p> <note> <p>If <code>cause</code> is set to <code>OPERATION<em>NOT</em>PERMITTED</code>, the decision failed because it lacked sufficient permissions. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p> </note></p>
    #[serde(rename = "cause")]
    pub cause: String,
    /// <p>The ID of the <code>DecisionTaskCompleted</code> event corresponding to the decision task that resulted in the <code>ContinueAsNewWorkflowExecution</code> decision that started this execution. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "decisionTaskCompletedEventId")]
    pub decision_task_completed_event_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CountPendingActivityTasksInput {
    /// <p>The name of the domain that contains the task list.</p>
    #[serde(rename = "domain")]
    pub domain: String,
    /// <p>The name of the task list.</p>
    #[serde(rename = "taskList")]
    pub task_list: TaskList,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CountPendingDecisionTasksInput {
    /// <p>The name of the domain that contains the task list.</p>
    #[serde(rename = "domain")]
    pub domain: String,
    /// <p>The name of the task list.</p>
    #[serde(rename = "taskList")]
    pub task_list: TaskList,
}

/// <p><p>Specifies a decision made by the decider. A decision can be one of these types:</p> <ul> <li> <p> <code>CancelTimer</code> – Cancels a previously started timer and records a <code>TimerCanceled</code> event in the history.</p> </li> <li> <p> <code>CancelWorkflowExecution</code> – Closes the workflow execution and records a <code>WorkflowExecutionCanceled</code> event in the history.</p> </li> <li> <p> <code>CompleteWorkflowExecution</code> – Closes the workflow execution and records a <code>WorkflowExecutionCompleted</code> event in the history .</p> </li> <li> <p> <code>ContinueAsNewWorkflowExecution</code> – Closes the workflow execution and starts a new workflow execution of the same type using the same workflow ID and a unique run Id. A <code>WorkflowExecutionContinuedAsNew</code> event is recorded in the history.</p> </li> <li> <p> <code>FailWorkflowExecution</code> – Closes the workflow execution and records a <code>WorkflowExecutionFailed</code> event in the history.</p> </li> <li> <p> <code>RecordMarker</code> – Records a <code>MarkerRecorded</code> event in the history. Markers can be used for adding custom information in the history for instance to let deciders know that they don&#39;t need to look at the history beyond the marker event.</p> </li> <li> <p> <code>RequestCancelActivityTask</code> – Attempts to cancel a previously scheduled activity task. If the activity task was scheduled but has not been assigned to a worker, then it is canceled. If the activity task was already assigned to a worker, then the worker is informed that cancellation has been requested in the response to <a>RecordActivityTaskHeartbeat</a>.</p> </li> <li> <p> <code>RequestCancelExternalWorkflowExecution</code> – Requests that a request be made to cancel the specified external workflow execution and records a <code>RequestCancelExternalWorkflowExecutionInitiated</code> event in the history.</p> </li> <li> <p> <code>ScheduleActivityTask</code> – Schedules an activity task.</p> </li> <li> <p> <code>SignalExternalWorkflowExecution</code> – Requests a signal to be delivered to the specified external workflow execution and records a <code>SignalExternalWorkflowExecutionInitiated</code> event in the history.</p> </li> <li> <p> <code>StartChildWorkflowExecution</code> – Requests that a child workflow execution be started and records a <code>StartChildWorkflowExecutionInitiated</code> event in the history. The child workflow execution is a separate workflow execution with its own history.</p> </li> <li> <p> <code>StartTimer</code> – Starts a timer for this workflow execution and records a <code>TimerStarted</code> event in the history. This timer fires after the specified delay and record a <code>TimerFired</code> event.</p> </li> </ul> <p> <b>Access Control</b> </p> <p>If you grant permission to use <code>RespondDecisionTaskCompleted</code>, you can use IAM policies to express permissions for the list of decisions returned by this action as if they were members of the API. Treating decisions as a pseudo API maintains a uniform conceptual model and helps keep policies readable. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p> <p> <b>Decision Failure</b> </p> <p>Decisions can fail for several reasons</p> <ul> <li> <p>The ordering of decisions should follow a logical flow. Some decisions might not make sense in the current context of the workflow execution and therefore fails.</p> </li> <li> <p>A limit on your account was reached.</p> </li> <li> <p>The decision lacks sufficient permissions.</p> </li> </ul> <p>One of the following events might be added to the history to indicate an error. The event attribute&#39;s <code>cause</code> parameter indicates the cause. If <code>cause</code> is set to <code>OPERATION<em>NOT</em>PERMITTED</code>, the decision failed because it lacked sufficient permissions. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p> <ul> <li> <p> <code>ScheduleActivityTaskFailed</code> – A <code>ScheduleActivityTask</code> decision failed. This could happen if the activity type specified in the decision isn&#39;t registered, is in a deprecated state, or the decision isn&#39;t properly configured.</p> </li> <li> <p> <code>RequestCancelActivityTaskFailed</code> – A <code>RequestCancelActivityTask</code> decision failed. This could happen if there is no open activity task with the specified activityId.</p> </li> <li> <p> <code>StartTimerFailed</code> – A <code>StartTimer</code> decision failed. This could happen if there is another open timer with the same timerId.</p> </li> <li> <p> <code>CancelTimerFailed</code> – A <code>CancelTimer</code> decision failed. This could happen if there is no open timer with the specified timerId.</p> </li> <li> <p> <code>StartChildWorkflowExecutionFailed</code> – A <code>StartChildWorkflowExecution</code> decision failed. This could happen if the workflow type specified isn&#39;t registered, is deprecated, or the decision isn&#39;t properly configured.</p> </li> <li> <p> <code>SignalExternalWorkflowExecutionFailed</code> – A <code>SignalExternalWorkflowExecution</code> decision failed. This could happen if the <code>workflowID</code> specified in the decision was incorrect.</p> </li> <li> <p> <code>RequestCancelExternalWorkflowExecutionFailed</code> – A <code>RequestCancelExternalWorkflowExecution</code> decision failed. This could happen if the <code>workflowID</code> specified in the decision was incorrect.</p> </li> <li> <p> <code>CancelWorkflowExecutionFailed</code> – A <code>CancelWorkflowExecution</code> decision failed. This could happen if there is an unhandled decision task pending in the workflow execution.</p> </li> <li> <p> <code>CompleteWorkflowExecutionFailed</code> – A <code>CompleteWorkflowExecution</code> decision failed. This could happen if there is an unhandled decision task pending in the workflow execution.</p> </li> <li> <p> <code>ContinueAsNewWorkflowExecutionFailed</code> – A <code>ContinueAsNewWorkflowExecution</code> decision failed. This could happen if there is an unhandled decision task pending in the workflow execution or the ContinueAsNewWorkflowExecution decision was not configured correctly.</p> </li> <li> <p> <code>FailWorkflowExecutionFailed</code> – A <code>FailWorkflowExecution</code> decision failed. This could happen if there is an unhandled decision task pending in the workflow execution.</p> </li> </ul> <p>The preceding error events might occur due to an error in the decider logic, which might put the workflow execution in an unstable state The cause field in the event structure for the error event indicates the cause of the error.</p> <note> <p>A workflow execution may be closed by the decider by returning one of the following decisions when completing a decision task: <code>CompleteWorkflowExecution</code>, <code>FailWorkflowExecution</code>, <code>CancelWorkflowExecution</code> and <code>ContinueAsNewWorkflowExecution</code>. An <code>UnhandledDecision</code> fault is returned if a workflow closing decision is specified and a signal or activity event had been added to the history while the decision task was being performed by the decider. Unlike the above situations which are logic issues, this fault is always possible because of race conditions in a distributed system. The right action here is to call <a>RespondDecisionTaskCompleted</a> without any decisions. This would result in another decision task with these new events included in the history. The decider should handle the new events and may decide to close the workflow execution.</p> </note> <p> <b>How to Code a Decision</b> </p> <p>You code a decision by first setting the decision type field to one of the above decision values, and then set the corresponding attributes field shown below:</p> <ul> <li> <p> <code> <a>ScheduleActivityTaskDecisionAttributes</a> </code> </p> </li> <li> <p> <code> <a>RequestCancelActivityTaskDecisionAttributes</a> </code> </p> </li> <li> <p> <code> <a>CompleteWorkflowExecutionDecisionAttributes</a> </code> </p> </li> <li> <p> <code> <a>FailWorkflowExecutionDecisionAttributes</a> </code> </p> </li> <li> <p> <code> <a>CancelWorkflowExecutionDecisionAttributes</a> </code> </p> </li> <li> <p> <code> <a>ContinueAsNewWorkflowExecutionDecisionAttributes</a> </code> </p> </li> <li> <p> <code> <a>RecordMarkerDecisionAttributes</a> </code> </p> </li> <li> <p> <code> <a>StartTimerDecisionAttributes</a> </code> </p> </li> <li> <p> <code> <a>CancelTimerDecisionAttributes</a> </code> </p> </li> <li> <p> <code> <a>SignalExternalWorkflowExecutionDecisionAttributes</a> </code> </p> </li> <li> <p> <code> <a>RequestCancelExternalWorkflowExecutionDecisionAttributes</a> </code> </p> </li> <li> <p> <code> <a>StartChildWorkflowExecutionDecisionAttributes</a> </code> </p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DecisionTaskScheduledEventAttributes {
    /// <p>The maximum duration for this decision task. The task is considered timed out if it doesn't completed within this duration.</p> <p>The duration is specified in seconds, an integer greater than or equal to <code>0</code>. You can use <code>NONE</code> to specify unlimited duration.</p>
    #[serde(rename = "startToCloseTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_to_close_timeout: Option<String>,
    /// <p>The name of the task list in which the decision task was scheduled.</p>
    #[serde(rename = "taskList")]
    pub task_list: TaskList,
    /// <p> A task priority that, if set, specifies the priority for this decision task. Valid values are integers that range from Java's <code>Integer.MIN_VALUE</code> (-2147483648) to <code>Integer.MAX_VALUE</code> (2147483647). Higher numbers indicate higher priority.</p> <p>For more information about setting task priority, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/programming-priority.html">Setting Task Priority</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    #[serde(rename = "taskPriority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_priority: Option<String>,
}

/// <p>Provides the details of the <code>DecisionTaskStarted</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeprecateActivityTypeInput {
    /// <p>The activity type to deprecate.</p>
    #[serde(rename = "activityType")]
    pub activity_type: ActivityType,
    /// <p>The name of the domain in which the activity type is registered.</p>
    #[serde(rename = "domain")]
    pub domain: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeprecateDomainInput {
    /// <p>The name of the domain to deprecate.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeprecateWorkflowTypeInput {
    /// <p>The name of the domain in which the workflow type is registered.</p>
    #[serde(rename = "domain")]
    pub domain: String,
    /// <p>The workflow type to deprecate.</p>
    #[serde(rename = "workflowType")]
    pub workflow_type: WorkflowType,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeActivityTypeInput {
    /// <p>The activity type to get information about. Activity types are identified by the <code>name</code> and <code>version</code> that were supplied when the activity was registered.</p>
    #[serde(rename = "activityType")]
    pub activity_type: ActivityType,
    /// <p>The name of the domain in which the activity type is registered.</p>
    #[serde(rename = "domain")]
    pub domain: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDomainInput {
    /// <p>The name of the domain to describe.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeWorkflowExecutionInput {
    /// <p>The name of the domain containing the workflow execution.</p>
    #[serde(rename = "domain")]
    pub domain: String,
    /// <p>The workflow execution to describe.</p>
    #[serde(rename = "execution")]
    pub execution: WorkflowExecution,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DomainConfiguration {
    /// <p>The retention period for workflow executions in this domain.</p>
    #[serde(rename = "workflowExecutionRetentionPeriodInDays")]
    pub workflow_execution_retention_period_in_days: String,
}

/// <p>Contains details of a domain.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DomainInfo {
    /// <p>The ARN of the domain.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ExternalWorkflowExecutionSignaledEventAttributes {
    /// <p>The ID of the <code>SignalExternalWorkflowExecutionInitiated</code> event corresponding to the <code>SignalExternalWorkflowExecution</code> decision to request this signal. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "initiatedEventId")]
    pub initiated_event_id: i64,
    /// <p>The external workflow execution that the signal was delivered to.</p>
    #[serde(rename = "workflowExecution")]
    pub workflow_execution: WorkflowExecution,
}

/// <p>Provides the details of the <code>FailWorkflowExecution</code> decision.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this decision's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FailWorkflowExecutionFailedEventAttributes {
    /// <p><p>The cause of the failure. This information is generated by the system and can be useful for diagnostic purposes.</p> <note> <p>If <code>cause</code> is set to <code>OPERATION<em>NOT</em>PERMITTED</code>, the decision failed because it lacked sufficient permissions. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p> </note></p>
    #[serde(rename = "cause")]
    pub cause: String,
    /// <p>The ID of the <code>DecisionTaskCompleted</code> event corresponding to the decision task that resulted in the <code>FailWorkflowExecution</code> decision to fail this execution. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "decisionTaskCompletedEventId")]
    pub decision_task_completed_event_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetWorkflowExecutionHistoryInput {
    /// <p>The name of the domain containing the workflow execution.</p>
    #[serde(rename = "domain")]
    pub domain: String,
    /// <p>Specifies the workflow execution for which to return the history.</p>
    #[serde(rename = "execution")]
    pub execution: WorkflowExecution,
    /// <p>The maximum number of results that are returned per call. Use <code>nextPageToken</code> to obtain further pages of results. </p>
    #[serde(rename = "maximumPageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_page_size: Option<i64>,
    /// <p>If <code>NextPageToken</code> is returned there are more results available. The value of <code>NextPageToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 60 seconds. Using an expired pagination token will return a <code>400</code> error: "<code>Specified token has exceeded its maximum lifetime</code>". </p> <p>The configured <code>maximumPageSize</code> determines how many results can be returned in a single call. </p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LambdaFunctionStartedEventAttributes {
    /// <p>The ID of the <code>LambdaFunctionScheduled</code> event that was recorded when this activity task was scheduled. To help diagnose issues, use this information to trace back the chain of events leading up to this event.</p>
    #[serde(rename = "scheduledEventId")]
    pub scheduled_event_id: i64,
}

/// <p>Provides details of the <code>LambdaFunctionTimedOut</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListActivityTypesInput {
    /// <p>The name of the domain in which the activity types have been registered.</p>
    #[serde(rename = "domain")]
    pub domain: String,
    /// <p>The maximum number of results that are returned per call. Use <code>nextPageToken</code> to obtain further pages of results. </p>
    #[serde(rename = "maximumPageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_page_size: Option<i64>,
    /// <p>If specified, only lists the activity types that have this name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>If <code>NextPageToken</code> is returned there are more results available. The value of <code>NextPageToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 60 seconds. Using an expired pagination token will return a <code>400</code> error: "<code>Specified token has exceeded its maximum lifetime</code>". </p> <p>The configured <code>maximumPageSize</code> determines how many results can be returned in a single call. </p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>The maximum number of results that are returned per call. Use <code>nextPageToken</code> to obtain further pages of results. </p>
    #[serde(rename = "maximumPageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_page_size: Option<i64>,
    /// <p>If <code>NextPageToken</code> is returned there are more results available. The value of <code>NextPageToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 60 seconds. Using an expired pagination token will return a <code>400</code> error: "<code>Specified token has exceeded its maximum lifetime</code>". </p> <p>The configured <code>maximumPageSize</code> determines how many results can be returned in a single call. </p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDomainsInput {
    /// <p>The maximum number of results that are returned per call. Use <code>nextPageToken</code> to obtain further pages of results. </p>
    #[serde(rename = "maximumPageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_page_size: Option<i64>,
    /// <p>If <code>NextPageToken</code> is returned there are more results available. The value of <code>NextPageToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 60 seconds. Using an expired pagination token will return a <code>400</code> error: "<code>Specified token has exceeded its maximum lifetime</code>". </p> <p>The configured <code>maximumPageSize</code> determines how many results can be returned in a single call. </p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListOpenWorkflowExecutionsInput {
    /// <p>The name of the domain that contains the workflow executions to list.</p>
    #[serde(rename = "domain")]
    pub domain: String,
    /// <p><p>If specified, only workflow executions matching the workflow ID specified in the filter are returned.</p> <note> <p> <code>executionFilter</code>, <code>typeFilter</code> and <code>tagFilter</code> are mutually exclusive. You can specify at most one of these in a request.</p> </note></p>
    #[serde(rename = "executionFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_filter: Option<WorkflowExecutionFilter>,
    /// <p>The maximum number of results that are returned per call. Use <code>nextPageToken</code> to obtain further pages of results. </p>
    #[serde(rename = "maximumPageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_page_size: Option<i64>,
    /// <p>If <code>NextPageToken</code> is returned there are more results available. The value of <code>NextPageToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 60 seconds. Using an expired pagination token will return a <code>400</code> error: "<code>Specified token has exceeded its maximum lifetime</code>". </p> <p>The configured <code>maximumPageSize</code> determines how many results can be returned in a single call. </p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceInput {
    /// <p>The Amazon Resource Name (ARN) for the Amazon SWF domain.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceOutput {
    /// <p>An array of tags associated with the domain.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<ResourceTag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListWorkflowTypesInput {
    /// <p>The name of the domain in which the workflow types have been registered.</p>
    #[serde(rename = "domain")]
    pub domain: String,
    /// <p>The maximum number of results that are returned per call. Use <code>nextPageToken</code> to obtain further pages of results. </p>
    #[serde(rename = "maximumPageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_page_size: Option<i64>,
    /// <p>If specified, lists the workflow type with this name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>If <code>NextPageToken</code> is returned there are more results available. The value of <code>NextPageToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 60 seconds. Using an expired pagination token will return a <code>400</code> error: "<code>Specified token has exceeded its maximum lifetime</code>". </p> <p>The configured <code>maximumPageSize</code> determines how many results can be returned in a single call. </p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PollForActivityTaskInput {
    /// <p>The name of the domain that contains the task lists being polled.</p>
    #[serde(rename = "domain")]
    pub domain: String,
    /// <p>Identity of the worker making the request, recorded in the <code>ActivityTaskStarted</code> event in the workflow history. This enables diagnostic tracing when problems arise. The form of this identity is user defined.</p>
    #[serde(rename = "identity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    /// <p>Specifies the task list to poll for activity tasks.</p> <p>The specified string must not start or end with whitespace. It must not contain a <code>:</code> (colon), <code>/</code> (slash), <code>|</code> (vertical bar), or any control characters (<code>\u0000-\u001f</code> | <code>\u007f-\u009f</code>). Also, it must not <i>be</i> the literal string <code>arn</code>.</p>
    #[serde(rename = "taskList")]
    pub task_list: TaskList,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PollForDecisionTaskInput {
    /// <p>The name of the domain containing the task lists to poll.</p>
    #[serde(rename = "domain")]
    pub domain: String,
    /// <p>Identity of the decider making the request, which is recorded in the DecisionTaskStarted event in the workflow history. This enables diagnostic tracing when problems arise. The form of this identity is user defined.</p>
    #[serde(rename = "identity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    /// <p>The maximum number of results that are returned per call. Use <code>nextPageToken</code> to obtain further pages of results. </p> <p>This is an upper limit only; the actual number of results returned per call may be fewer than the specified maximum.</p>
    #[serde(rename = "maximumPageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_page_size: Option<i64>,
    /// <p><p>If <code>NextPageToken</code> is returned there are more results available. The value of <code>NextPageToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 60 seconds. Using an expired pagination token will return a <code>400</code> error: &quot;<code>Specified token has exceeded its maximum lifetime</code>&quot;. </p> <p>The configured <code>maximumPageSize</code> determines how many results can be returned in a single call. </p> <note> <p>The <code>nextPageToken</code> returned by this action cannot be used with <a>GetWorkflowExecutionHistory</a> to get the next page. You must call <a>PollForDecisionTask</a> again (with the <code>nextPageToken</code>) to retrieve the next page of history records. Calling <a>PollForDecisionTask</a> with a <code>nextPageToken</code> doesn&#39;t return a new decision task.</p> </note></p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>When set to <code>true</code>, returns the events in reverse order. By default the results are returned in ascending order of the <code>eventTimestamp</code> of the events.</p>
    #[serde(rename = "reverseOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_order: Option<bool>,
    /// <p>Specifies the task list to poll for decision tasks.</p> <p>The specified string must not start or end with whitespace. It must not contain a <code>:</code> (colon), <code>/</code> (slash), <code>|</code> (vertical bar), or any control characters (<code>\u0000-\u001f</code> | <code>\u007f-\u009f</code>). Also, it must not <i>be</i> the literal string <code>arn</code>.</p>
    #[serde(rename = "taskList")]
    pub task_list: TaskList,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RecordActivityTaskHeartbeatInput {
    /// <p>If specified, contains details about the progress of the task.</p>
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// <p><p>The <code>taskToken</code> of the <a>ActivityTask</a>.</p> <important> <p> <code>taskToken</code> is generated by the service and should be treated as an opaque value. If the task is passed to another process, its <code>taskToken</code> must also be passed. This enables it to provide its progress and respond with results. </p> </important></p>
    #[serde(rename = "taskToken")]
    pub task_token: String,
}

/// <p>Provides the details of the <code>RecordMarker</code> decision.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this decision's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RecordMarkerFailedEventAttributes {
    /// <p><p>The cause of the failure. This information is generated by the system and can be useful for diagnostic purposes.</p> <note> <p>If <code>cause</code> is set to <code>OPERATION<em>NOT</em>PERMITTED</code>, the decision failed because it lacked sufficient permissions. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p> </note></p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RegisterActivityTypeInput {
    /// <p>If set, specifies the default maximum time before which a worker processing a task of this type must report progress by calling <a>RecordActivityTaskHeartbeat</a>. If the timeout is exceeded, the activity task is automatically timed out. This default can be overridden when scheduling an activity task using the <code>ScheduleActivityTask</code> <a>Decision</a>. If the activity worker subsequently attempts to record a heartbeat or returns a result, the activity worker receives an <code>UnknownResource</code> fault. In this case, Amazon SWF no longer considers the activity task to be valid; the activity worker should clean up the activity task.</p> <p>The duration is specified in seconds, an integer greater than or equal to <code>0</code>. You can use <code>NONE</code> to specify unlimited duration.</p>
    #[serde(rename = "defaultTaskHeartbeatTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_task_heartbeat_timeout: Option<String>,
    /// <p>If set, specifies the default task list to use for scheduling tasks of this activity type. This default task list is used if a task list isn't provided when a task is scheduled through the <code>ScheduleActivityTask</code> <a>Decision</a>.</p>
    #[serde(rename = "defaultTaskList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_task_list: Option<TaskList>,
    /// <p>The default task priority to assign to the activity type. If not assigned, then <code>0</code> is used. Valid values are integers that range from Java's <code>Integer.MIN_VALUE</code> (-2147483648) to <code>Integer.MAX_VALUE</code> (2147483647). Higher numbers indicate higher priority.</p> <p>For more information about setting task priority, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/programming-priority.html">Setting Task Priority</a> in the <i>in the <i>Amazon SWF Developer Guide</i>.</i>.</p>
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
    /// <p>The name of the activity type within the domain.</p> <p>The specified string must not start or end with whitespace. It must not contain a <code>:</code> (colon), <code>/</code> (slash), <code>|</code> (vertical bar), or any control characters (<code>\u0000-\u001f</code> | <code>\u007f-\u009f</code>). Also, it must not <i>be</i> the literal string <code>arn</code>.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The version of the activity type.</p> <note> <p>The activity type consists of the name and version, the combination of which must be unique within the domain.</p> </note> <p>The specified string must not start or end with whitespace. It must not contain a <code>:</code> (colon), <code>/</code> (slash), <code>|</code> (vertical bar), or any control characters (<code>\u0000-\u001f</code> | <code>\u007f-\u009f</code>). Also, it must not <i>be</i> the literal string <code>arn</code>.</p>
    #[serde(rename = "version")]
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RegisterDomainInput {
    /// <p>A text description of the domain.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Name of the domain to register. The name must be unique in the region that the domain is registered in.</p> <p>The specified string must not start or end with whitespace. It must not contain a <code>:</code> (colon), <code>/</code> (slash), <code>|</code> (vertical bar), or any control characters (<code>\u0000-\u001f</code> | <code>\u007f-\u009f</code>). Also, it must not <i>be</i> the literal string <code>arn</code>.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>Tags to be added when registering a domain.</p> <p>Tags may only contain unicode letters, digits, whitespace, or these symbols: <code>_ . : / = + - @</code>.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<ResourceTag>>,
    /// <p>The duration (in days) that records and histories of workflow executions on the domain should be kept by the service. After the retention period, the workflow execution isn't available in the results of visibility calls.</p> <p>If you pass the value <code>NONE</code> or <code>0</code> (zero), then the workflow execution history isn't retained. As soon as the workflow execution completes, the execution record and its history are deleted.</p> <p>The maximum workflow execution retention period is 90 days. For more information about Amazon SWF service limits, see: <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dg-limits.html">Amazon SWF Service Limits</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    #[serde(rename = "workflowExecutionRetentionPeriodInDays")]
    pub workflow_execution_retention_period_in_days: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RegisterWorkflowTypeInput {
    /// <p><p>If set, specifies the default policy to use for the child workflow executions when a workflow execution of this type is terminated, by calling the <a>TerminateWorkflowExecution</a> action explicitly or due to an expired timeout. This default can be overridden when starting a workflow execution using the <a>StartWorkflowExecution</a> action or the <code>StartChildWorkflowExecution</code> <a>Decision</a>.</p> <p>The supported child policies are:</p> <ul> <li> <p> <code>TERMINATE</code> – The child executions are terminated.</p> </li> <li> <p> <code>REQUEST_CANCEL</code> – A request to cancel is attempted for each child execution by recording a <code>WorkflowExecutionCancelRequested</code> event in its history. It is up to the decider to take appropriate actions when it receives an execution history with this event.</p> </li> <li> <p> <code>ABANDON</code> – No action is taken. The child executions continue to run.</p> </li> </ul></p>
    #[serde(rename = "defaultChildPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_child_policy: Option<String>,
    /// <p>If set, specifies the default maximum duration for executions of this workflow type. You can override this default when starting an execution through the <a>StartWorkflowExecution</a> Action or <code>StartChildWorkflowExecution</code> <a>Decision</a>.</p> <p>The duration is specified in seconds; an integer greater than or equal to 0. Unlike some of the other timeout parameters in Amazon SWF, you cannot specify a value of "NONE" for <code>defaultExecutionStartToCloseTimeout</code>; there is a one-year max limit on the time that a workflow execution can run. Exceeding this limit always causes the workflow execution to time out.</p>
    #[serde(rename = "defaultExecutionStartToCloseTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_execution_start_to_close_timeout: Option<String>,
    /// <p><p>The default IAM role attached to this workflow type.</p> <note> <p>Executions of this workflow type need IAM roles to invoke Lambda functions. If you don&#39;t specify an IAM role when you start this workflow type, the default Lambda role is attached to the execution. For more information, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/lambda-task.html">https://docs.aws.amazon.com/amazonswf/latest/developerguide/lambda-task.html</a> in the <i>Amazon SWF Developer Guide</i>.</p> </note></p>
    #[serde(rename = "defaultLambdaRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_lambda_role: Option<String>,
    /// <p>If set, specifies the default task list to use for scheduling decision tasks for executions of this workflow type. This default is used only if a task list isn't provided when starting the execution through the <a>StartWorkflowExecution</a> Action or <code>StartChildWorkflowExecution</code> <a>Decision</a>.</p>
    #[serde(rename = "defaultTaskList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_task_list: Option<TaskList>,
    /// <p>The default task priority to assign to the workflow type. If not assigned, then <code>0</code> is used. Valid values are integers that range from Java's <code>Integer.MIN_VALUE</code> (-2147483648) to <code>Integer.MAX_VALUE</code> (2147483647). Higher numbers indicate higher priority.</p> <p>For more information about setting task priority, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/programming-priority.html">Setting Task Priority</a> in the <i>Amazon SWF Developer Guide</i>.</p>
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
    /// <p>The name of the workflow type.</p> <p>The specified string must not start or end with whitespace. It must not contain a <code>:</code> (colon), <code>/</code> (slash), <code>|</code> (vertical bar), or any control characters (<code>\u0000-\u001f</code> | <code>\u007f-\u009f</code>). Also, it must not <i>be</i> the literal string <code>arn</code>.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The version of the workflow type.</p> <note> <p>The workflow type consists of the name and version, the combination of which must be unique within the domain. To get a list of all currently registered workflow types, use the <a>ListWorkflowTypes</a> action.</p> </note> <p>The specified string must not start or end with whitespace. It must not contain a <code>:</code> (colon), <code>/</code> (slash), <code>|</code> (vertical bar), or any control characters (<code>\u0000-\u001f</code> | <code>\u007f-\u009f</code>). Also, it must not <i>be</i> the literal string <code>arn</code>.</p>
    #[serde(rename = "version")]
    pub version: String,
}

/// <p>Provides the details of the <code>RequestCancelActivityTask</code> decision.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this decision's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RequestCancelActivityTaskDecisionAttributes {
    /// <p>The <code>activityId</code> of the activity task to be canceled.</p>
    #[serde(rename = "activityId")]
    pub activity_id: String,
}

/// <p>Provides the details of the <code>RequestCancelActivityTaskFailed</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RequestCancelActivityTaskFailedEventAttributes {
    /// <p>The activityId provided in the <code>RequestCancelActivityTask</code> decision that failed.</p>
    #[serde(rename = "activityId")]
    pub activity_id: String,
    /// <p><p>The cause of the failure. This information is generated by the system and can be useful for diagnostic purposes.</p> <note> <p>If <code>cause</code> is set to <code>OPERATION<em>NOT</em>PERMITTED</code>, the decision failed because it lacked sufficient permissions. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p> </note></p>
    #[serde(rename = "cause")]
    pub cause: String,
    /// <p>The ID of the <code>DecisionTaskCompleted</code> event corresponding to the decision task that resulted in the <code>RequestCancelActivityTask</code> decision for this cancellation request. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "decisionTaskCompletedEventId")]
    pub decision_task_completed_event_id: i64,
}

/// <p>Provides the details of the <code>RequestCancelExternalWorkflowExecution</code> decision.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this decision's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RequestCancelExternalWorkflowExecutionFailedEventAttributes {
    /// <p><p>The cause of the failure. This information is generated by the system and can be useful for diagnostic purposes.</p> <note> <p>If <code>cause</code> is set to <code>OPERATION<em>NOT</em>PERMITTED</code>, the decision failed because it lacked sufficient permissions. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p> </note></p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

/// <p>Tags are key-value pairs that can be associated with Amazon SWF state machines and activities.</p> <p>Tags may only contain unicode letters, digits, whitespace, or these symbols: <code>_ . : / = + - @</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceTag {
    /// <p>The key of a tag.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>The value of a tag.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Run {
    /// <p>The <code>runId</code> of a workflow execution. This ID is generated by the service and can be used to uniquely identify the workflow execution within a domain.</p>
    #[serde(rename = "runId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
}

/// <p>Provides the details of the <code>ScheduleActivityTask</code> decision.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this decision's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>activityType.name</code> – String constraint. The key is <code>swf:activityType.name</code>.</p> </li> <li> <p> <code>activityType.version</code> – String constraint. The key is <code>swf:activityType.version</code>.</p> </li> <li> <p> <code>taskList</code> – String constraint. The key is <code>swf:taskList.name</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p> If set, specifies the priority with which the activity task is to be assigned to a worker. This overrides the defaultTaskPriority specified when registering the activity type using <a>RegisterActivityType</a>. Valid values are integers that range from Java's <code>Integer.MIN_VALUE</code> (-2147483648) to <code>Integer.MAX_VALUE</code> (2147483647). Higher numbers indicate higher priority.</p> <p>For more information about setting task priority, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/programming-priority.html">Setting Task Priority</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    #[serde(rename = "taskPriority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_priority: Option<String>,
}

/// <p>Provides the details of the <code>ScheduleActivityTaskFailed</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ScheduleActivityTaskFailedEventAttributes {
    /// <p>The activityId provided in the <code>ScheduleActivityTask</code> decision that failed.</p>
    #[serde(rename = "activityId")]
    pub activity_id: String,
    /// <p>The activity type provided in the <code>ScheduleActivityTask</code> decision that failed.</p>
    #[serde(rename = "activityType")]
    pub activity_type: ActivityType,
    /// <p><p>The cause of the failure. This information is generated by the system and can be useful for diagnostic purposes.</p> <note> <p>If <code>cause</code> is set to <code>OPERATION<em>NOT</em>PERMITTED</code>, the decision failed because it lacked sufficient permissions. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p> </note></p>
    #[serde(rename = "cause")]
    pub cause: String,
    /// <p>The ID of the <code>DecisionTaskCompleted</code> event corresponding to the decision that resulted in the scheduling of this activity task. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[serde(rename = "decisionTaskCompletedEventId")]
    pub decision_task_completed_event_id: i64,
}

/// <p>Decision attributes specified in <code>scheduleLambdaFunctionDecisionAttributes</code> within the list of decisions <code>decisions</code> passed to <a>RespondDecisionTaskCompleted</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ScheduleLambdaFunctionFailedEventAttributes {
    /// <p><p>The cause of the failure. To help diagnose issues, use this information to trace back the chain of events leading up to this event.</p> <note> <p>If <code>cause</code> is set to <code>OPERATION<em>NOT</em>PERMITTED</code>, the decision failed because it lacked sufficient permissions. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p> </note></p>
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

/// <p>Provides the details of the <code>SignalExternalWorkflowExecution</code> decision.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this decision's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SignalExternalWorkflowExecutionFailedEventAttributes {
    /// <p><p>The cause of the failure. This information is generated by the system and can be useful for diagnostic purposes.</p> <note> <p>If <code>cause</code> is set to <code>OPERATION<em>NOT</em>PERMITTED</code>, the decision failed because it lacked sufficient permissions. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p> </note></p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

/// <p>Provides the details of the <code>StartChildWorkflowExecution</code> decision.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this decision's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>tagList.member.N</code> – The key is "swf:tagList.N" where N is the tag number from 0 to 4, inclusive.</p> </li> <li> <p> <code>taskList</code> – String constraint. The key is <code>swf:taskList.name</code>.</p> </li> <li> <p> <code>workflowType.name</code> – String constraint. The key is <code>swf:workflowType.name</code>.</p> </li> <li> <p> <code>workflowType.version</code> – String constraint. The key is <code>swf:workflowType.version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p> A task priority that, if set, specifies the priority for a decision task of this workflow execution. This overrides the defaultTaskPriority specified when registering the workflow type. Valid values are integers that range from Java's <code>Integer.MIN_VALUE</code> (-2147483648) to <code>Integer.MAX_VALUE</code> (2147483647). Higher numbers indicate higher priority.</p> <p>For more information about setting task priority, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/programming-priority.html">Setting Task Priority</a> in the <i>Amazon SWF Developer Guide</i>.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartChildWorkflowExecutionFailedEventAttributes {
    /// <p><p>The cause of the failure. This information is generated by the system and can be useful for diagnostic purposes.</p> <note> <p>When <code>cause</code> is set to <code>OPERATION<em>NOT</em>PERMITTED</code>, the decision fails because it lacks sufficient permissions. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html"> Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p> </note></p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p> The priority assigned for the decision tasks for this workflow execution. Valid values are integers that range from Java's <code>Integer.MIN_VALUE</code> (-2147483648) to <code>Integer.MAX_VALUE</code> (2147483647). Higher numbers indicate higher priority.</p> <p>For more information about setting task priority, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/programming-priority.html">Setting Task Priority</a> in the <i>Amazon SWF Developer Guide</i>.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartLambdaFunctionFailedEventAttributes {
    /// <p><p>The cause of the failure. To help diagnose issues, use this information to trace back the chain of events leading up to this event.</p> <note> <p>If <code>cause</code> is set to <code>OPERATION<em>NOT</em>PERMITTED</code>, the decision failed because the IAM role attached to the execution lacked sufficient permissions. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/lambda-task.html">Lambda Tasks</a> in the <i>Amazon SWF Developer Guide</i>.</p> </note></p>
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

/// <p>Provides the details of the <code>StartTimer</code> decision.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this decision's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartTimerFailedEventAttributes {
    /// <p><p>The cause of the failure. This information is generated by the system and can be useful for diagnostic purposes.</p> <note> <p>If <code>cause</code> is set to <code>OPERATION<em>NOT</em>PERMITTED</code>, the decision failed because it lacked sufficient permissions. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p> </note></p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p><p>The IAM role to attach to this workflow execution.</p> <note> <p>Executions of this workflow type need IAM roles to invoke Lambda functions. If you don&#39;t attach an IAM role, any attempt to schedule a Lambda task fails. This results in a <code>ScheduleLambdaFunctionFailed</code> history event. For more information, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/lambda-task.html">https://docs.aws.amazon.com/amazonswf/latest/developerguide/lambda-task.html</a> in the <i>Amazon SWF Developer Guide</i>.</p> </note></p>
    #[serde(rename = "lambdaRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_role: Option<String>,
    /// <p>The list of tags to associate with the workflow execution. You can specify a maximum of 5 tags. You can list workflow executions with a specific tag by calling <a>ListOpenWorkflowExecutions</a> or <a>ListClosedWorkflowExecutions</a> and specifying a <a>TagFilter</a>.</p>
    #[serde(rename = "tagList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<String>>,
    /// <p>The task list to use for the decision tasks generated for this workflow execution. This overrides the <code>defaultTaskList</code> specified when registering the workflow type.</p> <note> <p>A task list for this workflow execution must be specified either as a default for the workflow type or through this parameter. If neither this parameter is set nor a default task list was specified at registration time then a fault is returned.</p> </note> <p>The specified string must not start or end with whitespace. It must not contain a <code>:</code> (colon), <code>/</code> (slash), <code>|</code> (vertical bar), or any control characters (<code>\u0000-\u001f</code> | <code>\u007f-\u009f</code>). Also, it must not <i>be</i> the literal string <code>arn</code>.</p>
    #[serde(rename = "taskList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_list: Option<TaskList>,
    /// <p>The task priority to use for this workflow execution. This overrides any default priority that was assigned when the workflow type was registered. If not set, then the default task priority for the workflow type is used. Valid values are integers that range from Java's <code>Integer.MIN_VALUE</code> (-2147483648) to <code>Integer.MAX_VALUE</code> (2147483647). Higher numbers indicate higher priority.</p> <p>For more information about setting task priority, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/programming-priority.html">Setting Task Priority</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    #[serde(rename = "taskPriority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_priority: Option<String>,
    /// <p><p>Specifies the maximum duration of decision tasks for this workflow execution. This parameter overrides the <code>defaultTaskStartToCloseTimout</code> specified when registering the workflow type using <a>RegisterWorkflowType</a>.</p> <p>The duration is specified in seconds, an integer greater than or equal to <code>0</code>. You can use <code>NONE</code> to specify unlimited duration.</p> <note> <p>A task start-to-close timeout for this workflow execution must be specified either as a default for the workflow type or through this parameter. If neither this parameter is set nor a default task start-to-close timeout was specified at registration time then a fault is returned.</p> </note></p>
    #[serde(rename = "taskStartToCloseTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_start_to_close_timeout: Option<String>,
    /// <p>The user defined identifier associated with the workflow execution. You can use this to associate a custom identifier with the workflow execution. You may specify the same identifier if a workflow execution is logically a <i>restart</i> of a previous execution. You cannot have two open workflow executions with the same <code>workflowId</code> at the same time within the same domain.</p> <p>The specified string must not start or end with whitespace. It must not contain a <code>:</code> (colon), <code>/</code> (slash), <code>|</code> (vertical bar), or any control characters (<code>\u0000-\u001f</code> | <code>\u007f-\u009f</code>). Also, it must not <i>be</i> the literal string <code>arn</code>.</p>
    #[serde(rename = "workflowId")]
    pub workflow_id: String,
    /// <p>The type of the workflow to start.</p>
    #[serde(rename = "workflowType")]
    pub workflow_type: WorkflowType,
}

/// <p>Used to filter the workflow executions in visibility APIs based on a tag.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagFilter {
    /// <p> Specifies the tag that must be associated with the execution for it to meet the filter criteria.</p> <p>Tags may only contain unicode letters, digits, whitespace, or these symbols: <code>_ . : / = + - @</code>.</p>
    #[serde(rename = "tag")]
    pub tag: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceInput {
    /// <p>The Amazon Resource Name (ARN) for the Amazon SWF domain.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The list of tags to add to a domain. </p> <p>Tags may only contain unicode letters, digits, whitespace, or these symbols: <code>_ . : / = + - @</code>.</p>
    #[serde(rename = "tags")]
    pub tags: Vec<ResourceTag>,
}

/// <p>Represents a task list.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskList {
    /// <p>The name of the task list.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UndeprecateActivityTypeInput {
    /// <p>The activity type to undeprecate.</p>
    #[serde(rename = "activityType")]
    pub activity_type: ActivityType,
    /// <p>The name of the domain of the deprecated activity type.</p>
    #[serde(rename = "domain")]
    pub domain: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UndeprecateDomainInput {
    /// <p>The name of the domain of the deprecated workflow type.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UndeprecateWorkflowTypeInput {
    /// <p>The name of the domain of the deprecated workflow type.</p>
    #[serde(rename = "domain")]
    pub domain: String,
    /// <p>The name of the domain of the deprecated workflow type.</p>
    #[serde(rename = "workflowType")]
    pub workflow_type: WorkflowType,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceInput {
    /// <p>The Amazon Resource Name (ARN) for the Amazon SWF domain.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The list of tags to remove from the Amazon SWF domain.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The priority assigned to decision tasks for this workflow execution. Valid values are integers that range from Java's <code>Integer.MIN_VALUE</code> (-2147483648) to <code>Integer.MAX_VALUE</code> (2147483647). Higher numbers indicate higher priority.</p> <p>For more information about setting task priority, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/programming-priority.html">Setting Task Priority</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    #[serde(rename = "taskPriority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_priority: Option<String>,
    /// <p>The maximum duration allowed for decision tasks for this workflow execution.</p> <p>The duration is specified in seconds, an integer greater than or equal to <code>0</code>. You can use <code>NONE</code> to specify unlimited duration.</p>
    #[serde(rename = "taskStartToCloseTimeout")]
    pub task_start_to_close_timeout: String,
}

/// <p>Provides the details of the <code>WorkflowExecutionContinuedAsNew</code> event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct WorkflowExecutionFilter {
    /// <p>The workflowId to pass of match the criteria of this filter.</p>
    #[serde(rename = "workflowId")]
    pub workflow_id: String,
}

/// <p>Contains information about a workflow execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct WorkflowTypeConfiguration {
    /// <p><p> The default policy to use for the child workflow executions when a workflow execution of this type is terminated, by calling the <a>TerminateWorkflowExecution</a> action explicitly or due to an expired timeout. This default can be overridden when starting a workflow execution using the <a>StartWorkflowExecution</a> action or the <code>StartChildWorkflowExecution</code> <a>Decision</a>.</p> <p>The supported child policies are:</p> <ul> <li> <p> <code>TERMINATE</code> – The child executions are terminated.</p> </li> <li> <p> <code>REQUEST_CANCEL</code> – A request to cancel is attempted for each child execution by recording a <code>WorkflowExecutionCancelRequested</code> event in its history. It is up to the decider to take appropriate actions when it receives an execution history with this event.</p> </li> <li> <p> <code>ABANDON</code> – No action is taken. The child executions continue to run.</p> </li> </ul></p>
    #[serde(rename = "defaultChildPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_child_policy: Option<String>,
    /// <p> The default maximum duration, specified when registering the workflow type, for executions of this workflow type. This default can be overridden when starting a workflow execution using the <a>StartWorkflowExecution</a> action or the <code>StartChildWorkflowExecution</code> <a>Decision</a>.</p> <p>The duration is specified in seconds, an integer greater than or equal to <code>0</code>. You can use <code>NONE</code> to specify unlimited duration.</p>
    #[serde(rename = "defaultExecutionStartToCloseTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_execution_start_to_close_timeout: Option<String>,
    /// <p><p>The default IAM role attached to this workflow type.</p> <note> <p>Executions of this workflow type need IAM roles to invoke Lambda functions. If you don&#39;t specify an IAM role when starting this workflow type, the default Lambda role is attached to the execution. For more information, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/lambda-task.html">https://docs.aws.amazon.com/amazonswf/latest/developerguide/lambda-task.html</a> in the <i>Amazon SWF Developer Guide</i>.</p> </note></p>
    #[serde(rename = "defaultLambdaRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_lambda_role: Option<String>,
    /// <p> The default task list, specified when registering the workflow type, for decisions tasks scheduled for workflow executions of this type. This default can be overridden when starting a workflow execution using the <a>StartWorkflowExecution</a> action or the <code>StartChildWorkflowExecution</code> <a>Decision</a>.</p>
    #[serde(rename = "defaultTaskList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_task_list: Option<TaskList>,
    /// <p> The default task priority, specified when registering the workflow type, for all decision tasks of this workflow type. This default can be overridden when starting a workflow execution using the <a>StartWorkflowExecution</a> action or the <code>StartChildWorkflowExecution</code> decision.</p> <p>Valid values are integers that range from Java's <code>Integer.MIN_VALUE</code> (-2147483648) to <code>Integer.MAX_VALUE</code> (2147483647). Higher numbers indicate higher priority.</p> <p>For more information about setting task priority, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/programming-priority.html">Setting Task Priority</a> in the <i>Amazon SWF Developer Guide</i>.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
}

impl CountClosedWorkflowExecutionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CountClosedWorkflowExecutionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "OperationNotPermittedFault" => {
                    return RusotoError::Service(
                        CountClosedWorkflowExecutionsError::OperationNotPermittedFault(err.msg),
                    )
                }
                "UnknownResourceFault" => {
                    return RusotoError::Service(
                        CountClosedWorkflowExecutionsError::UnknownResourceFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CountClosedWorkflowExecutionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CountClosedWorkflowExecutionsError::OperationNotPermittedFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CountClosedWorkflowExecutionsError::UnknownResourceFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CountClosedWorkflowExecutionsError {}
/// Errors returned by CountOpenWorkflowExecutions
#[derive(Debug, PartialEq)]
pub enum CountOpenWorkflowExecutionsError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
}

impl CountOpenWorkflowExecutionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CountOpenWorkflowExecutionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "OperationNotPermittedFault" => {
                    return RusotoError::Service(
                        CountOpenWorkflowExecutionsError::OperationNotPermittedFault(err.msg),
                    )
                }
                "UnknownResourceFault" => {
                    return RusotoError::Service(
                        CountOpenWorkflowExecutionsError::UnknownResourceFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CountOpenWorkflowExecutionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CountOpenWorkflowExecutionsError::OperationNotPermittedFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CountOpenWorkflowExecutionsError::UnknownResourceFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CountOpenWorkflowExecutionsError {}
/// Errors returned by CountPendingActivityTasks
#[derive(Debug, PartialEq)]
pub enum CountPendingActivityTasksError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
}

impl CountPendingActivityTasksError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CountPendingActivityTasksError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "OperationNotPermittedFault" => {
                    return RusotoError::Service(
                        CountPendingActivityTasksError::OperationNotPermittedFault(err.msg),
                    )
                }
                "UnknownResourceFault" => {
                    return RusotoError::Service(
                        CountPendingActivityTasksError::UnknownResourceFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CountPendingActivityTasksError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CountPendingActivityTasksError::OperationNotPermittedFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CountPendingActivityTasksError::UnknownResourceFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CountPendingActivityTasksError {}
/// Errors returned by CountPendingDecisionTasks
#[derive(Debug, PartialEq)]
pub enum CountPendingDecisionTasksError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
}

impl CountPendingDecisionTasksError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CountPendingDecisionTasksError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "OperationNotPermittedFault" => {
                    return RusotoError::Service(
                        CountPendingDecisionTasksError::OperationNotPermittedFault(err.msg),
                    )
                }
                "UnknownResourceFault" => {
                    return RusotoError::Service(
                        CountPendingDecisionTasksError::UnknownResourceFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CountPendingDecisionTasksError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CountPendingDecisionTasksError::OperationNotPermittedFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CountPendingDecisionTasksError::UnknownResourceFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CountPendingDecisionTasksError {}
/// Errors returned by DeprecateActivityType
#[derive(Debug, PartialEq)]
pub enum DeprecateActivityTypeError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the specified activity or workflow type was already deprecated.</p>
    TypeDeprecatedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
}

impl DeprecateActivityTypeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeprecateActivityTypeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "OperationNotPermittedFault" => {
                    return RusotoError::Service(
                        DeprecateActivityTypeError::OperationNotPermittedFault(err.msg),
                    )
                }
                "TypeDeprecatedFault" => {
                    return RusotoError::Service(DeprecateActivityTypeError::TypeDeprecatedFault(
                        err.msg,
                    ))
                }
                "UnknownResourceFault" => {
                    return RusotoError::Service(DeprecateActivityTypeError::UnknownResourceFault(
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
impl fmt::Display for DeprecateActivityTypeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeprecateActivityTypeError::OperationNotPermittedFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DeprecateActivityTypeError::TypeDeprecatedFault(ref cause) => write!(f, "{}", cause),
            DeprecateActivityTypeError::UnknownResourceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeprecateActivityTypeError {}
/// Errors returned by DeprecateDomain
#[derive(Debug, PartialEq)]
pub enum DeprecateDomainError {
    /// <p>Returned when the specified domain has been deprecated.</p>
    DomainDeprecatedFault(String),
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
}

impl DeprecateDomainError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeprecateDomainError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DomainDeprecatedFault" => {
                    return RusotoError::Service(DeprecateDomainError::DomainDeprecatedFault(
                        err.msg,
                    ))
                }
                "OperationNotPermittedFault" => {
                    return RusotoError::Service(DeprecateDomainError::OperationNotPermittedFault(
                        err.msg,
                    ))
                }
                "UnknownResourceFault" => {
                    return RusotoError::Service(DeprecateDomainError::UnknownResourceFault(
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
impl fmt::Display for DeprecateDomainError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeprecateDomainError::DomainDeprecatedFault(ref cause) => write!(f, "{}", cause),
            DeprecateDomainError::OperationNotPermittedFault(ref cause) => write!(f, "{}", cause),
            DeprecateDomainError::UnknownResourceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeprecateDomainError {}
/// Errors returned by DeprecateWorkflowType
#[derive(Debug, PartialEq)]
pub enum DeprecateWorkflowTypeError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the specified activity or workflow type was already deprecated.</p>
    TypeDeprecatedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
}

impl DeprecateWorkflowTypeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeprecateWorkflowTypeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "OperationNotPermittedFault" => {
                    return RusotoError::Service(
                        DeprecateWorkflowTypeError::OperationNotPermittedFault(err.msg),
                    )
                }
                "TypeDeprecatedFault" => {
                    return RusotoError::Service(DeprecateWorkflowTypeError::TypeDeprecatedFault(
                        err.msg,
                    ))
                }
                "UnknownResourceFault" => {
                    return RusotoError::Service(DeprecateWorkflowTypeError::UnknownResourceFault(
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
impl fmt::Display for DeprecateWorkflowTypeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeprecateWorkflowTypeError::OperationNotPermittedFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DeprecateWorkflowTypeError::TypeDeprecatedFault(ref cause) => write!(f, "{}", cause),
            DeprecateWorkflowTypeError::UnknownResourceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeprecateWorkflowTypeError {}
/// Errors returned by DescribeActivityType
#[derive(Debug, PartialEq)]
pub enum DescribeActivityTypeError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
}

impl DescribeActivityTypeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeActivityTypeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "OperationNotPermittedFault" => {
                    return RusotoError::Service(
                        DescribeActivityTypeError::OperationNotPermittedFault(err.msg),
                    )
                }
                "UnknownResourceFault" => {
                    return RusotoError::Service(DescribeActivityTypeError::UnknownResourceFault(
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
impl fmt::Display for DescribeActivityTypeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeActivityTypeError::OperationNotPermittedFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeActivityTypeError::UnknownResourceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeActivityTypeError {}
/// Errors returned by DescribeDomain
#[derive(Debug, PartialEq)]
pub enum DescribeDomainError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
}

impl DescribeDomainError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeDomainError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "OperationNotPermittedFault" => {
                    return RusotoError::Service(DescribeDomainError::OperationNotPermittedFault(
                        err.msg,
                    ))
                }
                "UnknownResourceFault" => {
                    return RusotoError::Service(DescribeDomainError::UnknownResourceFault(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeDomainError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDomainError::OperationNotPermittedFault(ref cause) => write!(f, "{}", cause),
            DescribeDomainError::UnknownResourceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeDomainError {}
/// Errors returned by DescribeWorkflowExecution
#[derive(Debug, PartialEq)]
pub enum DescribeWorkflowExecutionError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
}

impl DescribeWorkflowExecutionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeWorkflowExecutionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "OperationNotPermittedFault" => {
                    return RusotoError::Service(
                        DescribeWorkflowExecutionError::OperationNotPermittedFault(err.msg),
                    )
                }
                "UnknownResourceFault" => {
                    return RusotoError::Service(
                        DescribeWorkflowExecutionError::UnknownResourceFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeWorkflowExecutionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeWorkflowExecutionError::OperationNotPermittedFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeWorkflowExecutionError::UnknownResourceFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeWorkflowExecutionError {}
/// Errors returned by DescribeWorkflowType
#[derive(Debug, PartialEq)]
pub enum DescribeWorkflowTypeError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
}

impl DescribeWorkflowTypeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeWorkflowTypeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "OperationNotPermittedFault" => {
                    return RusotoError::Service(
                        DescribeWorkflowTypeError::OperationNotPermittedFault(err.msg),
                    )
                }
                "UnknownResourceFault" => {
                    return RusotoError::Service(DescribeWorkflowTypeError::UnknownResourceFault(
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
impl fmt::Display for DescribeWorkflowTypeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeWorkflowTypeError::OperationNotPermittedFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeWorkflowTypeError::UnknownResourceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeWorkflowTypeError {}
/// Errors returned by GetWorkflowExecutionHistory
#[derive(Debug, PartialEq)]
pub enum GetWorkflowExecutionHistoryError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
}

impl GetWorkflowExecutionHistoryError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetWorkflowExecutionHistoryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "OperationNotPermittedFault" => {
                    return RusotoError::Service(
                        GetWorkflowExecutionHistoryError::OperationNotPermittedFault(err.msg),
                    )
                }
                "UnknownResourceFault" => {
                    return RusotoError::Service(
                        GetWorkflowExecutionHistoryError::UnknownResourceFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetWorkflowExecutionHistoryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetWorkflowExecutionHistoryError::OperationNotPermittedFault(ref cause) => {
                write!(f, "{}", cause)
            }
            GetWorkflowExecutionHistoryError::UnknownResourceFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetWorkflowExecutionHistoryError {}
/// Errors returned by ListActivityTypes
#[derive(Debug, PartialEq)]
pub enum ListActivityTypesError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
}

impl ListActivityTypesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListActivityTypesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "OperationNotPermittedFault" => {
                    return RusotoError::Service(
                        ListActivityTypesError::OperationNotPermittedFault(err.msg),
                    )
                }
                "UnknownResourceFault" => {
                    return RusotoError::Service(ListActivityTypesError::UnknownResourceFault(
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
impl fmt::Display for ListActivityTypesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListActivityTypesError::OperationNotPermittedFault(ref cause) => write!(f, "{}", cause),
            ListActivityTypesError::UnknownResourceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListActivityTypesError {}
/// Errors returned by ListClosedWorkflowExecutions
#[derive(Debug, PartialEq)]
pub enum ListClosedWorkflowExecutionsError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
}

impl ListClosedWorkflowExecutionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListClosedWorkflowExecutionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "OperationNotPermittedFault" => {
                    return RusotoError::Service(
                        ListClosedWorkflowExecutionsError::OperationNotPermittedFault(err.msg),
                    )
                }
                "UnknownResourceFault" => {
                    return RusotoError::Service(
                        ListClosedWorkflowExecutionsError::UnknownResourceFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListClosedWorkflowExecutionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListClosedWorkflowExecutionsError::OperationNotPermittedFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ListClosedWorkflowExecutionsError::UnknownResourceFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListClosedWorkflowExecutionsError {}
/// Errors returned by ListDomains
#[derive(Debug, PartialEq)]
pub enum ListDomainsError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
}

impl ListDomainsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDomainsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "OperationNotPermittedFault" => {
                    return RusotoError::Service(ListDomainsError::OperationNotPermittedFault(
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
impl fmt::Display for ListDomainsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDomainsError::OperationNotPermittedFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDomainsError {}
/// Errors returned by ListOpenWorkflowExecutions
#[derive(Debug, PartialEq)]
pub enum ListOpenWorkflowExecutionsError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
}

impl ListOpenWorkflowExecutionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListOpenWorkflowExecutionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "OperationNotPermittedFault" => {
                    return RusotoError::Service(
                        ListOpenWorkflowExecutionsError::OperationNotPermittedFault(err.msg),
                    )
                }
                "UnknownResourceFault" => {
                    return RusotoError::Service(
                        ListOpenWorkflowExecutionsError::UnknownResourceFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListOpenWorkflowExecutionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListOpenWorkflowExecutionsError::OperationNotPermittedFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ListOpenWorkflowExecutionsError::UnknownResourceFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListOpenWorkflowExecutionsError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>Returned by any operation if a system imposed limitation has been reached. To address this fault you should either clean up unused resources or increase the limit by contacting AWS.</p>
    LimitExceededFault(String),
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "LimitExceededFault" => {
                    return RusotoError::Service(ListTagsForResourceError::LimitExceededFault(
                        err.msg,
                    ))
                }
                "OperationNotPermittedFault" => {
                    return RusotoError::Service(
                        ListTagsForResourceError::OperationNotPermittedFault(err.msg),
                    )
                }
                "UnknownResourceFault" => {
                    return RusotoError::Service(ListTagsForResourceError::UnknownResourceFault(
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
            ListTagsForResourceError::LimitExceededFault(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::OperationNotPermittedFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ListTagsForResourceError::UnknownResourceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by ListWorkflowTypes
#[derive(Debug, PartialEq)]
pub enum ListWorkflowTypesError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
}

impl ListWorkflowTypesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListWorkflowTypesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "OperationNotPermittedFault" => {
                    return RusotoError::Service(
                        ListWorkflowTypesError::OperationNotPermittedFault(err.msg),
                    )
                }
                "UnknownResourceFault" => {
                    return RusotoError::Service(ListWorkflowTypesError::UnknownResourceFault(
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
impl fmt::Display for ListWorkflowTypesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListWorkflowTypesError::OperationNotPermittedFault(ref cause) => write!(f, "{}", cause),
            ListWorkflowTypesError::UnknownResourceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListWorkflowTypesError {}
/// Errors returned by PollForActivityTask
#[derive(Debug, PartialEq)]
pub enum PollForActivityTaskError {
    /// <p>Returned by any operation if a system imposed limitation has been reached. To address this fault you should either clean up unused resources or increase the limit by contacting AWS.</p>
    LimitExceededFault(String),
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
}

impl PollForActivityTaskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PollForActivityTaskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "LimitExceededFault" => {
                    return RusotoError::Service(PollForActivityTaskError::LimitExceededFault(
                        err.msg,
                    ))
                }
                "OperationNotPermittedFault" => {
                    return RusotoError::Service(
                        PollForActivityTaskError::OperationNotPermittedFault(err.msg),
                    )
                }
                "UnknownResourceFault" => {
                    return RusotoError::Service(PollForActivityTaskError::UnknownResourceFault(
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
impl fmt::Display for PollForActivityTaskError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PollForActivityTaskError::LimitExceededFault(ref cause) => write!(f, "{}", cause),
            PollForActivityTaskError::OperationNotPermittedFault(ref cause) => {
                write!(f, "{}", cause)
            }
            PollForActivityTaskError::UnknownResourceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PollForActivityTaskError {}
/// Errors returned by PollForDecisionTask
#[derive(Debug, PartialEq)]
pub enum PollForDecisionTaskError {
    /// <p>Returned by any operation if a system imposed limitation has been reached. To address this fault you should either clean up unused resources or increase the limit by contacting AWS.</p>
    LimitExceededFault(String),
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
}

impl PollForDecisionTaskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PollForDecisionTaskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "LimitExceededFault" => {
                    return RusotoError::Service(PollForDecisionTaskError::LimitExceededFault(
                        err.msg,
                    ))
                }
                "OperationNotPermittedFault" => {
                    return RusotoError::Service(
                        PollForDecisionTaskError::OperationNotPermittedFault(err.msg),
                    )
                }
                "UnknownResourceFault" => {
                    return RusotoError::Service(PollForDecisionTaskError::UnknownResourceFault(
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
impl fmt::Display for PollForDecisionTaskError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PollForDecisionTaskError::LimitExceededFault(ref cause) => write!(f, "{}", cause),
            PollForDecisionTaskError::OperationNotPermittedFault(ref cause) => {
                write!(f, "{}", cause)
            }
            PollForDecisionTaskError::UnknownResourceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PollForDecisionTaskError {}
/// Errors returned by RecordActivityTaskHeartbeat
#[derive(Debug, PartialEq)]
pub enum RecordActivityTaskHeartbeatError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
}

impl RecordActivityTaskHeartbeatError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<RecordActivityTaskHeartbeatError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "OperationNotPermittedFault" => {
                    return RusotoError::Service(
                        RecordActivityTaskHeartbeatError::OperationNotPermittedFault(err.msg),
                    )
                }
                "UnknownResourceFault" => {
                    return RusotoError::Service(
                        RecordActivityTaskHeartbeatError::UnknownResourceFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RecordActivityTaskHeartbeatError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RecordActivityTaskHeartbeatError::OperationNotPermittedFault(ref cause) => {
                write!(f, "{}", cause)
            }
            RecordActivityTaskHeartbeatError::UnknownResourceFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for RecordActivityTaskHeartbeatError {}
/// Errors returned by RegisterActivityType
#[derive(Debug, PartialEq)]
pub enum RegisterActivityTypeError {
    /// <p>Returned by any operation if a system imposed limitation has been reached. To address this fault you should either clean up unused resources or increase the limit by contacting AWS.</p>
    LimitExceededFault(String),
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned if the type already exists in the specified domain. You may get this fault if you are registering a type that is either already registered or deprecated, or if you undeprecate a type that is currently registered.</p>
    TypeAlreadyExistsFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
}

impl RegisterActivityTypeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RegisterActivityTypeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "LimitExceededFault" => {
                    return RusotoError::Service(RegisterActivityTypeError::LimitExceededFault(
                        err.msg,
                    ))
                }
                "OperationNotPermittedFault" => {
                    return RusotoError::Service(
                        RegisterActivityTypeError::OperationNotPermittedFault(err.msg),
                    )
                }
                "TypeAlreadyExistsFault" => {
                    return RusotoError::Service(RegisterActivityTypeError::TypeAlreadyExistsFault(
                        err.msg,
                    ))
                }
                "UnknownResourceFault" => {
                    return RusotoError::Service(RegisterActivityTypeError::UnknownResourceFault(
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
impl fmt::Display for RegisterActivityTypeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RegisterActivityTypeError::LimitExceededFault(ref cause) => write!(f, "{}", cause),
            RegisterActivityTypeError::OperationNotPermittedFault(ref cause) => {
                write!(f, "{}", cause)
            }
            RegisterActivityTypeError::TypeAlreadyExistsFault(ref cause) => write!(f, "{}", cause),
            RegisterActivityTypeError::UnknownResourceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RegisterActivityTypeError {}
/// Errors returned by RegisterDomain
#[derive(Debug, PartialEq)]
pub enum RegisterDomainError {
    /// <p>Returned if the domain already exists. You may get this fault if you are registering a domain that is either already registered or deprecated, or if you undeprecate a domain that is currently registered.</p>
    DomainAlreadyExistsFault(String),
    /// <p>Returned by any operation if a system imposed limitation has been reached. To address this fault you should either clean up unused resources or increase the limit by contacting AWS.</p>
    LimitExceededFault(String),
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>You've exceeded the number of tags allowed for a domain.</p>
    TooManyTagsFault(String),
}

impl RegisterDomainError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RegisterDomainError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DomainAlreadyExistsFault" => {
                    return RusotoError::Service(RegisterDomainError::DomainAlreadyExistsFault(
                        err.msg,
                    ))
                }
                "LimitExceededFault" => {
                    return RusotoError::Service(RegisterDomainError::LimitExceededFault(err.msg))
                }
                "OperationNotPermittedFault" => {
                    return RusotoError::Service(RegisterDomainError::OperationNotPermittedFault(
                        err.msg,
                    ))
                }
                "TooManyTagsFault" => {
                    return RusotoError::Service(RegisterDomainError::TooManyTagsFault(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RegisterDomainError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RegisterDomainError::DomainAlreadyExistsFault(ref cause) => write!(f, "{}", cause),
            RegisterDomainError::LimitExceededFault(ref cause) => write!(f, "{}", cause),
            RegisterDomainError::OperationNotPermittedFault(ref cause) => write!(f, "{}", cause),
            RegisterDomainError::TooManyTagsFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RegisterDomainError {}
/// Errors returned by RegisterWorkflowType
#[derive(Debug, PartialEq)]
pub enum RegisterWorkflowTypeError {
    /// <p>Returned by any operation if a system imposed limitation has been reached. To address this fault you should either clean up unused resources or increase the limit by contacting AWS.</p>
    LimitExceededFault(String),
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned if the type already exists in the specified domain. You may get this fault if you are registering a type that is either already registered or deprecated, or if you undeprecate a type that is currently registered.</p>
    TypeAlreadyExistsFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
}

impl RegisterWorkflowTypeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RegisterWorkflowTypeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "LimitExceededFault" => {
                    return RusotoError::Service(RegisterWorkflowTypeError::LimitExceededFault(
                        err.msg,
                    ))
                }
                "OperationNotPermittedFault" => {
                    return RusotoError::Service(
                        RegisterWorkflowTypeError::OperationNotPermittedFault(err.msg),
                    )
                }
                "TypeAlreadyExistsFault" => {
                    return RusotoError::Service(RegisterWorkflowTypeError::TypeAlreadyExistsFault(
                        err.msg,
                    ))
                }
                "UnknownResourceFault" => {
                    return RusotoError::Service(RegisterWorkflowTypeError::UnknownResourceFault(
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
impl fmt::Display for RegisterWorkflowTypeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RegisterWorkflowTypeError::LimitExceededFault(ref cause) => write!(f, "{}", cause),
            RegisterWorkflowTypeError::OperationNotPermittedFault(ref cause) => {
                write!(f, "{}", cause)
            }
            RegisterWorkflowTypeError::TypeAlreadyExistsFault(ref cause) => write!(f, "{}", cause),
            RegisterWorkflowTypeError::UnknownResourceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RegisterWorkflowTypeError {}
/// Errors returned by RequestCancelWorkflowExecution
#[derive(Debug, PartialEq)]
pub enum RequestCancelWorkflowExecutionError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
}

impl RequestCancelWorkflowExecutionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<RequestCancelWorkflowExecutionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "OperationNotPermittedFault" => {
                    return RusotoError::Service(
                        RequestCancelWorkflowExecutionError::OperationNotPermittedFault(err.msg),
                    )
                }
                "UnknownResourceFault" => {
                    return RusotoError::Service(
                        RequestCancelWorkflowExecutionError::UnknownResourceFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RequestCancelWorkflowExecutionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RequestCancelWorkflowExecutionError::OperationNotPermittedFault(ref cause) => {
                write!(f, "{}", cause)
            }
            RequestCancelWorkflowExecutionError::UnknownResourceFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for RequestCancelWorkflowExecutionError {}
/// Errors returned by RespondActivityTaskCanceled
#[derive(Debug, PartialEq)]
pub enum RespondActivityTaskCanceledError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
}

impl RespondActivityTaskCanceledError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<RespondActivityTaskCanceledError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "OperationNotPermittedFault" => {
                    return RusotoError::Service(
                        RespondActivityTaskCanceledError::OperationNotPermittedFault(err.msg),
                    )
                }
                "UnknownResourceFault" => {
                    return RusotoError::Service(
                        RespondActivityTaskCanceledError::UnknownResourceFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RespondActivityTaskCanceledError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RespondActivityTaskCanceledError::OperationNotPermittedFault(ref cause) => {
                write!(f, "{}", cause)
            }
            RespondActivityTaskCanceledError::UnknownResourceFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for RespondActivityTaskCanceledError {}
/// Errors returned by RespondActivityTaskCompleted
#[derive(Debug, PartialEq)]
pub enum RespondActivityTaskCompletedError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
}

impl RespondActivityTaskCompletedError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<RespondActivityTaskCompletedError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "OperationNotPermittedFault" => {
                    return RusotoError::Service(
                        RespondActivityTaskCompletedError::OperationNotPermittedFault(err.msg),
                    )
                }
                "UnknownResourceFault" => {
                    return RusotoError::Service(
                        RespondActivityTaskCompletedError::UnknownResourceFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RespondActivityTaskCompletedError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RespondActivityTaskCompletedError::OperationNotPermittedFault(ref cause) => {
                write!(f, "{}", cause)
            }
            RespondActivityTaskCompletedError::UnknownResourceFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for RespondActivityTaskCompletedError {}
/// Errors returned by RespondActivityTaskFailed
#[derive(Debug, PartialEq)]
pub enum RespondActivityTaskFailedError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
}

impl RespondActivityTaskFailedError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RespondActivityTaskFailedError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "OperationNotPermittedFault" => {
                    return RusotoError::Service(
                        RespondActivityTaskFailedError::OperationNotPermittedFault(err.msg),
                    )
                }
                "UnknownResourceFault" => {
                    return RusotoError::Service(
                        RespondActivityTaskFailedError::UnknownResourceFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RespondActivityTaskFailedError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RespondActivityTaskFailedError::OperationNotPermittedFault(ref cause) => {
                write!(f, "{}", cause)
            }
            RespondActivityTaskFailedError::UnknownResourceFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for RespondActivityTaskFailedError {}
/// Errors returned by RespondDecisionTaskCompleted
#[derive(Debug, PartialEq)]
pub enum RespondDecisionTaskCompletedError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
}

impl RespondDecisionTaskCompletedError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<RespondDecisionTaskCompletedError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "OperationNotPermittedFault" => {
                    return RusotoError::Service(
                        RespondDecisionTaskCompletedError::OperationNotPermittedFault(err.msg),
                    )
                }
                "UnknownResourceFault" => {
                    return RusotoError::Service(
                        RespondDecisionTaskCompletedError::UnknownResourceFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RespondDecisionTaskCompletedError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RespondDecisionTaskCompletedError::OperationNotPermittedFault(ref cause) => {
                write!(f, "{}", cause)
            }
            RespondDecisionTaskCompletedError::UnknownResourceFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for RespondDecisionTaskCompletedError {}
/// Errors returned by SignalWorkflowExecution
#[derive(Debug, PartialEq)]
pub enum SignalWorkflowExecutionError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
}

impl SignalWorkflowExecutionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SignalWorkflowExecutionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "OperationNotPermittedFault" => {
                    return RusotoError::Service(
                        SignalWorkflowExecutionError::OperationNotPermittedFault(err.msg),
                    )
                }
                "UnknownResourceFault" => {
                    return RusotoError::Service(
                        SignalWorkflowExecutionError::UnknownResourceFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SignalWorkflowExecutionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SignalWorkflowExecutionError::OperationNotPermittedFault(ref cause) => {
                write!(f, "{}", cause)
            }
            SignalWorkflowExecutionError::UnknownResourceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SignalWorkflowExecutionError {}
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
}

impl StartWorkflowExecutionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartWorkflowExecutionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DefaultUndefinedFault" => {
                    return RusotoError::Service(
                        StartWorkflowExecutionError::DefaultUndefinedFault(err.msg),
                    )
                }
                "LimitExceededFault" => {
                    return RusotoError::Service(StartWorkflowExecutionError::LimitExceededFault(
                        err.msg,
                    ))
                }
                "OperationNotPermittedFault" => {
                    return RusotoError::Service(
                        StartWorkflowExecutionError::OperationNotPermittedFault(err.msg),
                    )
                }
                "TypeDeprecatedFault" => {
                    return RusotoError::Service(StartWorkflowExecutionError::TypeDeprecatedFault(
                        err.msg,
                    ))
                }
                "UnknownResourceFault" => {
                    return RusotoError::Service(StartWorkflowExecutionError::UnknownResourceFault(
                        err.msg,
                    ))
                }
                "WorkflowExecutionAlreadyStartedFault" => {
                    return RusotoError::Service(
                        StartWorkflowExecutionError::WorkflowExecutionAlreadyStartedFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartWorkflowExecutionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartWorkflowExecutionError::DefaultUndefinedFault(ref cause) => write!(f, "{}", cause),
            StartWorkflowExecutionError::LimitExceededFault(ref cause) => write!(f, "{}", cause),
            StartWorkflowExecutionError::OperationNotPermittedFault(ref cause) => {
                write!(f, "{}", cause)
            }
            StartWorkflowExecutionError::TypeDeprecatedFault(ref cause) => write!(f, "{}", cause),
            StartWorkflowExecutionError::UnknownResourceFault(ref cause) => write!(f, "{}", cause),
            StartWorkflowExecutionError::WorkflowExecutionAlreadyStartedFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for StartWorkflowExecutionError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>Returned by any operation if a system imposed limitation has been reached. To address this fault you should either clean up unused resources or increase the limit by contacting AWS.</p>
    LimitExceededFault(String),
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>You've exceeded the number of tags allowed for a domain.</p>
    TooManyTagsFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "LimitExceededFault" => {
                    return RusotoError::Service(TagResourceError::LimitExceededFault(err.msg))
                }
                "OperationNotPermittedFault" => {
                    return RusotoError::Service(TagResourceError::OperationNotPermittedFault(
                        err.msg,
                    ))
                }
                "TooManyTagsFault" => {
                    return RusotoError::Service(TagResourceError::TooManyTagsFault(err.msg))
                }
                "UnknownResourceFault" => {
                    return RusotoError::Service(TagResourceError::UnknownResourceFault(err.msg))
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
            TagResourceError::LimitExceededFault(ref cause) => write!(f, "{}", cause),
            TagResourceError::OperationNotPermittedFault(ref cause) => write!(f, "{}", cause),
            TagResourceError::TooManyTagsFault(ref cause) => write!(f, "{}", cause),
            TagResourceError::UnknownResourceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by TerminateWorkflowExecution
#[derive(Debug, PartialEq)]
pub enum TerminateWorkflowExecutionError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
}

impl TerminateWorkflowExecutionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<TerminateWorkflowExecutionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "OperationNotPermittedFault" => {
                    return RusotoError::Service(
                        TerminateWorkflowExecutionError::OperationNotPermittedFault(err.msg),
                    )
                }
                "UnknownResourceFault" => {
                    return RusotoError::Service(
                        TerminateWorkflowExecutionError::UnknownResourceFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for TerminateWorkflowExecutionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TerminateWorkflowExecutionError::OperationNotPermittedFault(ref cause) => {
                write!(f, "{}", cause)
            }
            TerminateWorkflowExecutionError::UnknownResourceFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for TerminateWorkflowExecutionError {}
/// Errors returned by UndeprecateActivityType
#[derive(Debug, PartialEq)]
pub enum UndeprecateActivityTypeError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned if the type already exists in the specified domain. You may get this fault if you are registering a type that is either already registered or deprecated, or if you undeprecate a type that is currently registered.</p>
    TypeAlreadyExistsFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
}

impl UndeprecateActivityTypeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UndeprecateActivityTypeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "OperationNotPermittedFault" => {
                    return RusotoError::Service(
                        UndeprecateActivityTypeError::OperationNotPermittedFault(err.msg),
                    )
                }
                "TypeAlreadyExistsFault" => {
                    return RusotoError::Service(
                        UndeprecateActivityTypeError::TypeAlreadyExistsFault(err.msg),
                    )
                }
                "UnknownResourceFault" => {
                    return RusotoError::Service(
                        UndeprecateActivityTypeError::UnknownResourceFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UndeprecateActivityTypeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UndeprecateActivityTypeError::OperationNotPermittedFault(ref cause) => {
                write!(f, "{}", cause)
            }
            UndeprecateActivityTypeError::TypeAlreadyExistsFault(ref cause) => {
                write!(f, "{}", cause)
            }
            UndeprecateActivityTypeError::UnknownResourceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UndeprecateActivityTypeError {}
/// Errors returned by UndeprecateDomain
#[derive(Debug, PartialEq)]
pub enum UndeprecateDomainError {
    /// <p>Returned if the domain already exists. You may get this fault if you are registering a domain that is either already registered or deprecated, or if you undeprecate a domain that is currently registered.</p>
    DomainAlreadyExistsFault(String),
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
}

impl UndeprecateDomainError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UndeprecateDomainError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DomainAlreadyExistsFault" => {
                    return RusotoError::Service(UndeprecateDomainError::DomainAlreadyExistsFault(
                        err.msg,
                    ))
                }
                "OperationNotPermittedFault" => {
                    return RusotoError::Service(
                        UndeprecateDomainError::OperationNotPermittedFault(err.msg),
                    )
                }
                "UnknownResourceFault" => {
                    return RusotoError::Service(UndeprecateDomainError::UnknownResourceFault(
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
impl fmt::Display for UndeprecateDomainError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UndeprecateDomainError::DomainAlreadyExistsFault(ref cause) => write!(f, "{}", cause),
            UndeprecateDomainError::OperationNotPermittedFault(ref cause) => write!(f, "{}", cause),
            UndeprecateDomainError::UnknownResourceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UndeprecateDomainError {}
/// Errors returned by UndeprecateWorkflowType
#[derive(Debug, PartialEq)]
pub enum UndeprecateWorkflowTypeError {
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned if the type already exists in the specified domain. You may get this fault if you are registering a type that is either already registered or deprecated, or if you undeprecate a type that is currently registered.</p>
    TypeAlreadyExistsFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
}

impl UndeprecateWorkflowTypeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UndeprecateWorkflowTypeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "OperationNotPermittedFault" => {
                    return RusotoError::Service(
                        UndeprecateWorkflowTypeError::OperationNotPermittedFault(err.msg),
                    )
                }
                "TypeAlreadyExistsFault" => {
                    return RusotoError::Service(
                        UndeprecateWorkflowTypeError::TypeAlreadyExistsFault(err.msg),
                    )
                }
                "UnknownResourceFault" => {
                    return RusotoError::Service(
                        UndeprecateWorkflowTypeError::UnknownResourceFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UndeprecateWorkflowTypeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UndeprecateWorkflowTypeError::OperationNotPermittedFault(ref cause) => {
                write!(f, "{}", cause)
            }
            UndeprecateWorkflowTypeError::TypeAlreadyExistsFault(ref cause) => {
                write!(f, "{}", cause)
            }
            UndeprecateWorkflowTypeError::UnknownResourceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UndeprecateWorkflowTypeError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>Returned by any operation if a system imposed limitation has been reached. To address this fault you should either clean up unused resources or increase the limit by contacting AWS.</p>
    LimitExceededFault(String),
    /// <p>Returned when the caller doesn't have sufficient permissions to invoke the action.</p>
    OperationNotPermittedFault(String),
    /// <p>Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never created or is no longer available for this operation.</p>
    UnknownResourceFault(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "LimitExceededFault" => {
                    return RusotoError::Service(UntagResourceError::LimitExceededFault(err.msg))
                }
                "OperationNotPermittedFault" => {
                    return RusotoError::Service(UntagResourceError::OperationNotPermittedFault(
                        err.msg,
                    ))
                }
                "UnknownResourceFault" => {
                    return RusotoError::Service(UntagResourceError::UnknownResourceFault(err.msg))
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
            UntagResourceError::LimitExceededFault(ref cause) => write!(f, "{}", cause),
            UntagResourceError::OperationNotPermittedFault(ref cause) => write!(f, "{}", cause),
            UntagResourceError::UnknownResourceFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Trait representing the capabilities of the Amazon SWF API. Amazon SWF clients implement this trait.
#[async_trait]
pub trait Swf {
    /// <p>Returns the number of closed workflow executions within the given domain that meet the specified filtering criteria.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>tagFilter.tag</code>: String constraint. The key is <code>swf:tagFilter.tag</code>.</p> </li> <li> <p> <code>typeFilter.name</code>: String constraint. The key is <code>swf:typeFilter.name</code>.</p> </li> <li> <p> <code>typeFilter.version</code>: String constraint. The key is <code>swf:typeFilter.version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn count_closed_workflow_executions(
        &self,
        input: CountClosedWorkflowExecutionsInput,
    ) -> Result<WorkflowExecutionCount, RusotoError<CountClosedWorkflowExecutionsError>>;

    /// <p>Returns the number of open workflow executions within the given domain that meet the specified filtering criteria.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>tagFilter.tag</code>: String constraint. The key is <code>swf:tagFilter.tag</code>.</p> </li> <li> <p> <code>typeFilter.name</code>: String constraint. The key is <code>swf:typeFilter.name</code>.</p> </li> <li> <p> <code>typeFilter.version</code>: String constraint. The key is <code>swf:typeFilter.version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn count_open_workflow_executions(
        &self,
        input: CountOpenWorkflowExecutionsInput,
    ) -> Result<WorkflowExecutionCount, RusotoError<CountOpenWorkflowExecutionsError>>;

    /// <p>Returns the estimated number of activity tasks in the specified task list. The count returned is an approximation and isn't guaranteed to be exact. If you specify a task list that no activity task was ever scheduled in then <code>0</code> is returned.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the <code>taskList.name</code> parameter by using a <code>Condition</code> element with the <code>swf:taskList.name</code> key to allow the action to access only certain task lists.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn count_pending_activity_tasks(
        &self,
        input: CountPendingActivityTasksInput,
    ) -> Result<PendingTaskCount, RusotoError<CountPendingActivityTasksError>>;

    /// <p>Returns the estimated number of decision tasks in the specified task list. The count returned is an approximation and isn't guaranteed to be exact. If you specify a task list that no decision task was ever scheduled in then <code>0</code> is returned.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the <code>taskList.name</code> parameter by using a <code>Condition</code> element with the <code>swf:taskList.name</code> key to allow the action to access only certain task lists.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn count_pending_decision_tasks(
        &self,
        input: CountPendingDecisionTasksInput,
    ) -> Result<PendingTaskCount, RusotoError<CountPendingDecisionTasksError>>;

    /// <p>Deprecates the specified <i>activity type</i>. After an activity type has been deprecated, you cannot create new tasks of that activity type. Tasks of this type that were scheduled before the type was deprecated continue to run.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>activityType.name</code>: String constraint. The key is <code>swf:activityType.name</code>.</p> </li> <li> <p> <code>activityType.version</code>: String constraint. The key is <code>swf:activityType.version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn deprecate_activity_type(
        &self,
        input: DeprecateActivityTypeInput,
    ) -> Result<(), RusotoError<DeprecateActivityTypeError>>;

    /// <p>Deprecates the specified domain. After a domain has been deprecated it cannot be used to create new workflow executions or register new types. However, you can still use visibility actions on this domain. Deprecating a domain also deprecates all activity and workflow types registered in the domain. Executions that were started before the domain was deprecated continues to run.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn deprecate_domain(
        &self,
        input: DeprecateDomainInput,
    ) -> Result<(), RusotoError<DeprecateDomainError>>;

    /// <p>Deprecates the specified <i>workflow type</i>. After a workflow type has been deprecated, you cannot create new executions of that type. Executions that were started before the type was deprecated continues to run. A deprecated workflow type may still be used when calling visibility actions.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>workflowType.name</code>: String constraint. The key is <code>swf:workflowType.name</code>.</p> </li> <li> <p> <code>workflowType.version</code>: String constraint. The key is <code>swf:workflowType.version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn deprecate_workflow_type(
        &self,
        input: DeprecateWorkflowTypeInput,
    ) -> Result<(), RusotoError<DeprecateWorkflowTypeError>>;

    /// <p>Returns information about the specified activity type. This includes configuration settings provided when the type was registered and other general information about the type.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>activityType.name</code>: String constraint. The key is <code>swf:activityType.name</code>.</p> </li> <li> <p> <code>activityType.version</code>: String constraint. The key is <code>swf:activityType.version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn describe_activity_type(
        &self,
        input: DescribeActivityTypeInput,
    ) -> Result<ActivityTypeDetail, RusotoError<DescribeActivityTypeError>>;

    /// <p>Returns information about the specified domain, including description and status.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn describe_domain(
        &self,
        input: DescribeDomainInput,
    ) -> Result<DomainDetail, RusotoError<DescribeDomainError>>;

    /// <p>Returns information about the specified workflow execution including its type and some statistics.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn describe_workflow_execution(
        &self,
        input: DescribeWorkflowExecutionInput,
    ) -> Result<WorkflowExecutionDetail, RusotoError<DescribeWorkflowExecutionError>>;

    /// <p>Returns information about the specified <i>workflow type</i>. This includes configuration settings specified when the type was registered and other information such as creation date, current status, etc.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>workflowType.name</code>: String constraint. The key is <code>swf:workflowType.name</code>.</p> </li> <li> <p> <code>workflowType.version</code>: String constraint. The key is <code>swf:workflowType.version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn describe_workflow_type(
        &self,
        input: DescribeWorkflowTypeInput,
    ) -> Result<WorkflowTypeDetail, RusotoError<DescribeWorkflowTypeError>>;

    /// <p>Returns the history of the specified workflow execution. The results may be split into multiple pages. To retrieve subsequent pages, make the call again using the <code>nextPageToken</code> returned by the initial call.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn get_workflow_execution_history(
        &self,
        input: GetWorkflowExecutionHistoryInput,
    ) -> Result<History, RusotoError<GetWorkflowExecutionHistoryError>>;

    /// <p>Returns information about all activities registered in the specified domain that match the specified name and registration status. The result includes information like creation date, current status of the activity, etc. The results may be split into multiple pages. To retrieve subsequent pages, make the call again using the <code>nextPageToken</code> returned by the initial call.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn list_activity_types(
        &self,
        input: ListActivityTypesInput,
    ) -> Result<ActivityTypeInfos, RusotoError<ListActivityTypesError>>;

    /// <p>Returns a list of closed workflow executions in the specified domain that meet the filtering criteria. The results may be split into multiple pages. To retrieve subsequent pages, make the call again using the nextPageToken returned by the initial call.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>tagFilter.tag</code>: String constraint. The key is <code>swf:tagFilter.tag</code>.</p> </li> <li> <p> <code>typeFilter.name</code>: String constraint. The key is <code>swf:typeFilter.name</code>.</p> </li> <li> <p> <code>typeFilter.version</code>: String constraint. The key is <code>swf:typeFilter.version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn list_closed_workflow_executions(
        &self,
        input: ListClosedWorkflowExecutionsInput,
    ) -> Result<WorkflowExecutionInfos, RusotoError<ListClosedWorkflowExecutionsError>>;

    /// <p>Returns the list of domains registered in the account. The results may be split into multiple pages. To retrieve subsequent pages, make the call again using the nextPageToken returned by the initial call.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains. The element must be set to <code>arn:aws:swf::AccountID:domain/*</code>, where <i>AccountID</i> is the account ID, with no dashes.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn list_domains(
        &self,
        input: ListDomainsInput,
    ) -> Result<DomainInfos, RusotoError<ListDomainsError>>;

    /// <p>Returns a list of open workflow executions in the specified domain that meet the filtering criteria. The results may be split into multiple pages. To retrieve subsequent pages, make the call again using the nextPageToken returned by the initial call.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>tagFilter.tag</code>: String constraint. The key is <code>swf:tagFilter.tag</code>.</p> </li> <li> <p> <code>typeFilter.name</code>: String constraint. The key is <code>swf:typeFilter.name</code>.</p> </li> <li> <p> <code>typeFilter.version</code>: String constraint. The key is <code>swf:typeFilter.version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn list_open_workflow_executions(
        &self,
        input: ListOpenWorkflowExecutionsInput,
    ) -> Result<WorkflowExecutionInfos, RusotoError<ListOpenWorkflowExecutionsError>>;

    /// <p>List tags for a given domain.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceInput,
    ) -> Result<ListTagsForResourceOutput, RusotoError<ListTagsForResourceError>>;

    /// <p>Returns information about workflow types in the specified domain. The results may be split into multiple pages that can be retrieved by making the call repeatedly.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn list_workflow_types(
        &self,
        input: ListWorkflowTypesInput,
    ) -> Result<WorkflowTypeInfos, RusotoError<ListWorkflowTypesError>>;

    /// <p>Used by workers to get an <a>ActivityTask</a> from the specified activity <code>taskList</code>. This initiates a long poll, where the service holds the HTTP connection open and responds as soon as a task becomes available. The maximum time the service holds on to the request before responding is 60 seconds. If no task is available within 60 seconds, the poll returns an empty result. An empty result, in this context, means that an ActivityTask is returned, but that the value of taskToken is an empty string. If a task is returned, the worker should use its type to identify and process it correctly.</p> <important> <p>Workers should set their client side socket timeout to at least 70 seconds (10 seconds higher than the maximum time service may hold the poll request).</p> </important> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the <code>taskList.name</code> parameter by using a <code>Condition</code> element with the <code>swf:taskList.name</code> key to allow the action to access only certain task lists.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn poll_for_activity_task(
        &self,
        input: PollForActivityTaskInput,
    ) -> Result<ActivityTask, RusotoError<PollForActivityTaskError>>;

    /// <p>Used by deciders to get a <a>DecisionTask</a> from the specified decision <code>taskList</code>. A decision task may be returned for any open workflow execution that is using the specified task list. The task includes a paginated view of the history of the workflow execution. The decider should use the workflow type and the history to determine how to properly handle the task.</p> <p>This action initiates a long poll, where the service holds the HTTP connection open and responds as soon a task becomes available. If no decision task is available in the specified task list before the timeout of 60 seconds expires, an empty result is returned. An empty result, in this context, means that a DecisionTask is returned, but that the value of taskToken is an empty string.</p> <important> <p>Deciders should set their client side socket timeout to at least 70 seconds (10 seconds higher than the timeout).</p> </important> <important> <p>Because the number of workflow history events for a single workflow execution might be very large, the result returned might be split up across a number of pages. To retrieve subsequent pages, make additional calls to <code>PollForDecisionTask</code> using the <code>nextPageToken</code> returned by the initial call. Note that you do <i>not</i> call <code>GetWorkflowExecutionHistory</code> with this <code>nextPageToken</code>. Instead, call <code>PollForDecisionTask</code> again.</p> </important> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the <code>taskList.name</code> parameter by using a <code>Condition</code> element with the <code>swf:taskList.name</code> key to allow the action to access only certain task lists.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn poll_for_decision_task(
        &self,
        input: PollForDecisionTaskInput,
    ) -> Result<DecisionTask, RusotoError<PollForDecisionTaskError>>;

    /// <p>Used by activity workers to report to the service that the <a>ActivityTask</a> represented by the specified <code>taskToken</code> is still making progress. The worker can also specify details of the progress, for example percent complete, using the <code>details</code> parameter. This action can also be used by the worker as a mechanism to check if cancellation is being requested for the activity task. If a cancellation is being attempted for the specified task, then the boolean <code>cancelRequested</code> flag returned by the service is set to <code>true</code>.</p> <p>This action resets the <code>taskHeartbeatTimeout</code> clock. The <code>taskHeartbeatTimeout</code> is specified in <a>RegisterActivityType</a>.</p> <p>This action doesn't in itself create an event in the workflow execution history. However, if the task times out, the workflow execution history contains a <code>ActivityTaskTimedOut</code> event that contains the information from the last heartbeat generated by the activity worker.</p> <note> <p>The <code>taskStartToCloseTimeout</code> of an activity type is the maximum duration of an activity task, regardless of the number of <a>RecordActivityTaskHeartbeat</a> requests received. The <code>taskStartToCloseTimeout</code> is also specified in <a>RegisterActivityType</a>.</p> </note> <note> <p>This operation is only useful for long-lived activities to report liveliness of the task and to determine if a cancellation is being attempted.</p> </note> <important> <p>If the <code>cancelRequested</code> flag returns <code>true</code>, a cancellation is being attempted. If the worker can cancel the activity, it should respond with <a>RespondActivityTaskCanceled</a>. Otherwise, it should ignore the cancellation request.</p> </important> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn record_activity_task_heartbeat(
        &self,
        input: RecordActivityTaskHeartbeatInput,
    ) -> Result<ActivityTaskStatus, RusotoError<RecordActivityTaskHeartbeatError>>;

    /// <p>Registers a new <i>activity type</i> along with its configuration settings in the specified domain.</p> <important> <p>A <code>TypeAlreadyExists</code> fault is returned if the type already exists in the domain. You cannot change any configuration settings of the type after its registration, and it must be registered as a new version.</p> </important> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>defaultTaskList.name</code>: String constraint. The key is <code>swf:defaultTaskList.name</code>.</p> </li> <li> <p> <code>name</code>: String constraint. The key is <code>swf:name</code>.</p> </li> <li> <p> <code>version</code>: String constraint. The key is <code>swf:version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn register_activity_type(
        &self,
        input: RegisterActivityTypeInput,
    ) -> Result<(), RusotoError<RegisterActivityTypeError>>;

    /// <p>Registers a new domain.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>You cannot use an IAM policy to control domain access for this action. The name of the domain being registered is available as the resource of this action.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn register_domain(
        &self,
        input: RegisterDomainInput,
    ) -> Result<(), RusotoError<RegisterDomainError>>;

    /// <p>Registers a new <i>workflow type</i> and its configuration settings in the specified domain.</p> <p>The retention period for the workflow history is set by the <a>RegisterDomain</a> action.</p> <important> <p>If the type already exists, then a <code>TypeAlreadyExists</code> fault is returned. You cannot change the configuration settings of a workflow type once it is registered and it must be registered as a new version.</p> </important> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>defaultTaskList.name</code>: String constraint. The key is <code>swf:defaultTaskList.name</code>.</p> </li> <li> <p> <code>name</code>: String constraint. The key is <code>swf:name</code>.</p> </li> <li> <p> <code>version</code>: String constraint. The key is <code>swf:version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn register_workflow_type(
        &self,
        input: RegisterWorkflowTypeInput,
    ) -> Result<(), RusotoError<RegisterWorkflowTypeError>>;

    /// <p>Records a <code>WorkflowExecutionCancelRequested</code> event in the currently running workflow execution identified by the given domain, workflowId, and runId. This logically requests the cancellation of the workflow execution as a whole. It is up to the decider to take appropriate actions when it receives an execution history with this event.</p> <note> <p>If the runId isn't specified, the <code>WorkflowExecutionCancelRequested</code> event is recorded in the history of the current open workflow execution with the specified workflowId in the domain.</p> </note> <note> <p>Because this action allows the workflow to properly clean up and gracefully close, it should be used instead of <a>TerminateWorkflowExecution</a> when possible.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn request_cancel_workflow_execution(
        &self,
        input: RequestCancelWorkflowExecutionInput,
    ) -> Result<(), RusotoError<RequestCancelWorkflowExecutionError>>;

    /// <p>Used by workers to tell the service that the <a>ActivityTask</a> identified by the <code>taskToken</code> was successfully canceled. Additional <code>details</code> can be provided using the <code>details</code> argument.</p> <p>These <code>details</code> (if provided) appear in the <code>ActivityTaskCanceled</code> event added to the workflow history.</p> <important> <p>Only use this operation if the <code>canceled</code> flag of a <a>RecordActivityTaskHeartbeat</a> request returns <code>true</code> and if the activity can be safely undone or abandoned.</p> </important> <p>A task is considered open from the time that it is scheduled until it is closed. Therefore a task is reported as open while a worker is processing it. A task is closed after it has been specified in a call to <a>RespondActivityTaskCompleted</a>, RespondActivityTaskCanceled, <a>RespondActivityTaskFailed</a>, or the task has <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dg-basic.html#swf-dev-timeout-types">timed out</a>.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn respond_activity_task_canceled(
        &self,
        input: RespondActivityTaskCanceledInput,
    ) -> Result<(), RusotoError<RespondActivityTaskCanceledError>>;

    /// <p>Used by workers to tell the service that the <a>ActivityTask</a> identified by the <code>taskToken</code> completed successfully with a <code>result</code> (if provided). The <code>result</code> appears in the <code>ActivityTaskCompleted</code> event in the workflow history.</p> <important> <p>If the requested task doesn't complete successfully, use <a>RespondActivityTaskFailed</a> instead. If the worker finds that the task is canceled through the <code>canceled</code> flag returned by <a>RecordActivityTaskHeartbeat</a>, it should cancel the task, clean up and then call <a>RespondActivityTaskCanceled</a>.</p> </important> <p>A task is considered open from the time that it is scheduled until it is closed. Therefore a task is reported as open while a worker is processing it. A task is closed after it has been specified in a call to RespondActivityTaskCompleted, <a>RespondActivityTaskCanceled</a>, <a>RespondActivityTaskFailed</a>, or the task has <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dg-basic.html#swf-dev-timeout-types">timed out</a>.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn respond_activity_task_completed(
        &self,
        input: RespondActivityTaskCompletedInput,
    ) -> Result<(), RusotoError<RespondActivityTaskCompletedError>>;

    /// <p>Used by workers to tell the service that the <a>ActivityTask</a> identified by the <code>taskToken</code> has failed with <code>reason</code> (if specified). The <code>reason</code> and <code>details</code> appear in the <code>ActivityTaskFailed</code> event added to the workflow history.</p> <p>A task is considered open from the time that it is scheduled until it is closed. Therefore a task is reported as open while a worker is processing it. A task is closed after it has been specified in a call to <a>RespondActivityTaskCompleted</a>, <a>RespondActivityTaskCanceled</a>, RespondActivityTaskFailed, or the task has <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dg-basic.html#swf-dev-timeout-types">timed out</a>.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn respond_activity_task_failed(
        &self,
        input: RespondActivityTaskFailedInput,
    ) -> Result<(), RusotoError<RespondActivityTaskFailedError>>;

    /// <p>Used by deciders to tell the service that the <a>DecisionTask</a> identified by the <code>taskToken</code> has successfully completed. The <code>decisions</code> argument specifies the list of decisions made while processing the task.</p> <p>A <code>DecisionTaskCompleted</code> event is added to the workflow history. The <code>executionContext</code> specified is attached to the event in the workflow execution history.</p> <p> <b>Access Control</b> </p> <p>If an IAM policy grants permission to use <code>RespondDecisionTaskCompleted</code>, it can express permissions for the list of decisions in the <code>decisions</code> parameter. Each of the decisions has one or more parameters, much like a regular API call. To allow for policies to be as readable as possible, you can express permissions on decisions as if they were actual API calls, including applying conditions to some parameters. For more information, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn respond_decision_task_completed(
        &self,
        input: RespondDecisionTaskCompletedInput,
    ) -> Result<(), RusotoError<RespondDecisionTaskCompletedError>>;

    /// <p>Records a <code>WorkflowExecutionSignaled</code> event in the workflow execution history and creates a decision task for the workflow execution identified by the given domain, workflowId and runId. The event is recorded with the specified user defined signalName and input (if provided).</p> <note> <p>If a runId isn't specified, then the <code>WorkflowExecutionSignaled</code> event is recorded in the history of the current open workflow with the matching workflowId in the domain.</p> </note> <note> <p>If the specified workflow execution isn't open, this method fails with <code>UnknownResource</code>.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn signal_workflow_execution(
        &self,
        input: SignalWorkflowExecutionInput,
    ) -> Result<(), RusotoError<SignalWorkflowExecutionError>>;

    /// <p>Starts an execution of the workflow type in the specified domain using the provided <code>workflowId</code> and input data.</p> <p>This action returns the newly started workflow execution.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>tagList.member.0</code>: The key is <code>swf:tagList.member.0</code>.</p> </li> <li> <p> <code>tagList.member.1</code>: The key is <code>swf:tagList.member.1</code>.</p> </li> <li> <p> <code>tagList.member.2</code>: The key is <code>swf:tagList.member.2</code>.</p> </li> <li> <p> <code>tagList.member.3</code>: The key is <code>swf:tagList.member.3</code>.</p> </li> <li> <p> <code>tagList.member.4</code>: The key is <code>swf:tagList.member.4</code>.</p> </li> <li> <p> <code>taskList</code>: String constraint. The key is <code>swf:taskList.name</code>.</p> </li> <li> <p> <code>workflowType.name</code>: String constraint. The key is <code>swf:workflowType.name</code>.</p> </li> <li> <p> <code>workflowType.version</code>: String constraint. The key is <code>swf:workflowType.version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn start_workflow_execution(
        &self,
        input: StartWorkflowExecutionInput,
    ) -> Result<Run, RusotoError<StartWorkflowExecutionError>>;

    /// <p><p>Add a tag to a Amazon SWF domain.</p> <note> <p>Amazon SWF supports a maximum of 50 tags per resource.</p> </note></p>
    async fn tag_resource(
        &self,
        input: TagResourceInput,
    ) -> Result<(), RusotoError<TagResourceError>>;

    /// <p>Records a <code>WorkflowExecutionTerminated</code> event and forces closure of the workflow execution identified by the given domain, runId, and workflowId. The child policy, registered with the workflow type or specified when starting this execution, is applied to any open child workflow executions of this workflow execution.</p> <important> <p>If the identified workflow execution was in progress, it is terminated immediately.</p> </important> <note> <p>If a runId isn't specified, then the <code>WorkflowExecutionTerminated</code> event is recorded in the history of the current open workflow with the matching workflowId in the domain.</p> </note> <note> <p>You should consider using <a>RequestCancelWorkflowExecution</a> action instead because it allows the workflow to gracefully close while <a>TerminateWorkflowExecution</a> doesn't.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn terminate_workflow_execution(
        &self,
        input: TerminateWorkflowExecutionInput,
    ) -> Result<(), RusotoError<TerminateWorkflowExecutionError>>;

    /// <p>Undeprecates a previously deprecated <i>activity type</i>. After an activity type has been undeprecated, you can create new tasks of that activity type.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>activityType.name</code>: String constraint. The key is <code>swf:activityType.name</code>.</p> </li> <li> <p> <code>activityType.version</code>: String constraint. The key is <code>swf:activityType.version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn undeprecate_activity_type(
        &self,
        input: UndeprecateActivityTypeInput,
    ) -> Result<(), RusotoError<UndeprecateActivityTypeError>>;

    /// <p>Undeprecates a previously deprecated domain. After a domain has been undeprecated it can be used to create new workflow executions or register new types.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn undeprecate_domain(
        &self,
        input: UndeprecateDomainInput,
    ) -> Result<(), RusotoError<UndeprecateDomainError>>;

    /// <p>Undeprecates a previously deprecated <i>workflow type</i>. After a workflow type has been undeprecated, you can create new executions of that type. </p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>workflowType.name</code>: String constraint. The key is <code>swf:workflowType.name</code>.</p> </li> <li> <p> <code>workflowType.version</code>: String constraint. The key is <code>swf:workflowType.version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn undeprecate_workflow_type(
        &self,
        input: UndeprecateWorkflowTypeInput,
    ) -> Result<(), RusotoError<UndeprecateWorkflowTypeError>>;

    /// <p>Remove a tag from a Amazon SWF domain.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceInput,
    ) -> Result<(), RusotoError<UntagResourceError>>;
}
/// A client for the Amazon SWF API.
#[derive(Clone)]
pub struct SwfClient {
    client: Client,
    region: region::Region,
}

impl SwfClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> SwfClient {
        SwfClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> SwfClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        SwfClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> SwfClient {
        SwfClient { client, region }
    }
}

#[async_trait]
impl Swf for SwfClient {
    /// <p>Returns the number of closed workflow executions within the given domain that meet the specified filtering criteria.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>tagFilter.tag</code>: String constraint. The key is <code>swf:tagFilter.tag</code>.</p> </li> <li> <p> <code>typeFilter.name</code>: String constraint. The key is <code>swf:typeFilter.name</code>.</p> </li> <li> <p> <code>typeFilter.version</code>: String constraint. The key is <code>swf:typeFilter.version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn count_closed_workflow_executions(
        &self,
        input: CountClosedWorkflowExecutionsInput,
    ) -> Result<WorkflowExecutionCount, RusotoError<CountClosedWorkflowExecutionsError>> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "SimpleWorkflowService.CountClosedWorkflowExecutions",
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
            proto::json::ResponsePayload::new(&response).deserialize::<WorkflowExecutionCount, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CountClosedWorkflowExecutionsError::from_response(response))
        }
    }

    /// <p>Returns the number of open workflow executions within the given domain that meet the specified filtering criteria.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>tagFilter.tag</code>: String constraint. The key is <code>swf:tagFilter.tag</code>.</p> </li> <li> <p> <code>typeFilter.name</code>: String constraint. The key is <code>swf:typeFilter.name</code>.</p> </li> <li> <p> <code>typeFilter.version</code>: String constraint. The key is <code>swf:typeFilter.version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn count_open_workflow_executions(
        &self,
        input: CountOpenWorkflowExecutionsInput,
    ) -> Result<WorkflowExecutionCount, RusotoError<CountOpenWorkflowExecutionsError>> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "SimpleWorkflowService.CountOpenWorkflowExecutions",
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
            proto::json::ResponsePayload::new(&response).deserialize::<WorkflowExecutionCount, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CountOpenWorkflowExecutionsError::from_response(response))
        }
    }

    /// <p>Returns the estimated number of activity tasks in the specified task list. The count returned is an approximation and isn't guaranteed to be exact. If you specify a task list that no activity task was ever scheduled in then <code>0</code> is returned.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the <code>taskList.name</code> parameter by using a <code>Condition</code> element with the <code>swf:taskList.name</code> key to allow the action to access only certain task lists.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn count_pending_activity_tasks(
        &self,
        input: CountPendingActivityTasksInput,
    ) -> Result<PendingTaskCount, RusotoError<CountPendingActivityTasksError>> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "SimpleWorkflowService.CountPendingActivityTasks",
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
            proto::json::ResponsePayload::new(&response).deserialize::<PendingTaskCount, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CountPendingActivityTasksError::from_response(response))
        }
    }

    /// <p>Returns the estimated number of decision tasks in the specified task list. The count returned is an approximation and isn't guaranteed to be exact. If you specify a task list that no decision task was ever scheduled in then <code>0</code> is returned.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the <code>taskList.name</code> parameter by using a <code>Condition</code> element with the <code>swf:taskList.name</code> key to allow the action to access only certain task lists.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn count_pending_decision_tasks(
        &self,
        input: CountPendingDecisionTasksInput,
    ) -> Result<PendingTaskCount, RusotoError<CountPendingDecisionTasksError>> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "SimpleWorkflowService.CountPendingDecisionTasks",
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
            proto::json::ResponsePayload::new(&response).deserialize::<PendingTaskCount, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CountPendingDecisionTasksError::from_response(response))
        }
    }

    /// <p>Deprecates the specified <i>activity type</i>. After an activity type has been deprecated, you cannot create new tasks of that activity type. Tasks of this type that were scheduled before the type was deprecated continue to run.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>activityType.name</code>: String constraint. The key is <code>swf:activityType.name</code>.</p> </li> <li> <p> <code>activityType.version</code>: String constraint. The key is <code>swf:activityType.version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn deprecate_activity_type(
        &self,
        input: DeprecateActivityTypeInput,
    ) -> Result<(), RusotoError<DeprecateActivityTypeError>> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "SimpleWorkflowService.DeprecateActivityType",
        );
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
            Err(DeprecateActivityTypeError::from_response(response))
        }
    }

    /// <p>Deprecates the specified domain. After a domain has been deprecated it cannot be used to create new workflow executions or register new types. However, you can still use visibility actions on this domain. Deprecating a domain also deprecates all activity and workflow types registered in the domain. Executions that were started before the domain was deprecated continues to run.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn deprecate_domain(
        &self,
        input: DeprecateDomainInput,
    ) -> Result<(), RusotoError<DeprecateDomainError>> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "SimpleWorkflowService.DeprecateDomain");
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
            Err(DeprecateDomainError::from_response(response))
        }
    }

    /// <p>Deprecates the specified <i>workflow type</i>. After a workflow type has been deprecated, you cannot create new executions of that type. Executions that were started before the type was deprecated continues to run. A deprecated workflow type may still be used when calling visibility actions.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>workflowType.name</code>: String constraint. The key is <code>swf:workflowType.name</code>.</p> </li> <li> <p> <code>workflowType.version</code>: String constraint. The key is <code>swf:workflowType.version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn deprecate_workflow_type(
        &self,
        input: DeprecateWorkflowTypeInput,
    ) -> Result<(), RusotoError<DeprecateWorkflowTypeError>> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "SimpleWorkflowService.DeprecateWorkflowType",
        );
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
            Err(DeprecateWorkflowTypeError::from_response(response))
        }
    }

    /// <p>Returns information about the specified activity type. This includes configuration settings provided when the type was registered and other general information about the type.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>activityType.name</code>: String constraint. The key is <code>swf:activityType.name</code>.</p> </li> <li> <p> <code>activityType.version</code>: String constraint. The key is <code>swf:activityType.version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn describe_activity_type(
        &self,
        input: DescribeActivityTypeInput,
    ) -> Result<ActivityTypeDetail, RusotoError<DescribeActivityTypeError>> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "SimpleWorkflowService.DescribeActivityType");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ActivityTypeDetail, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeActivityTypeError::from_response(response))
        }
    }

    /// <p>Returns information about the specified domain, including description and status.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn describe_domain(
        &self,
        input: DescribeDomainInput,
    ) -> Result<DomainDetail, RusotoError<DescribeDomainError>> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "SimpleWorkflowService.DescribeDomain");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DomainDetail, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeDomainError::from_response(response))
        }
    }

    /// <p>Returns information about the specified workflow execution including its type and some statistics.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn describe_workflow_execution(
        &self,
        input: DescribeWorkflowExecutionInput,
    ) -> Result<WorkflowExecutionDetail, RusotoError<DescribeWorkflowExecutionError>> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "SimpleWorkflowService.DescribeWorkflowExecution",
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
            proto::json::ResponsePayload::new(&response).deserialize::<WorkflowExecutionDetail, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeWorkflowExecutionError::from_response(response))
        }
    }

    /// <p>Returns information about the specified <i>workflow type</i>. This includes configuration settings specified when the type was registered and other information such as creation date, current status, etc.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>workflowType.name</code>: String constraint. The key is <code>swf:workflowType.name</code>.</p> </li> <li> <p> <code>workflowType.version</code>: String constraint. The key is <code>swf:workflowType.version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn describe_workflow_type(
        &self,
        input: DescribeWorkflowTypeInput,
    ) -> Result<WorkflowTypeDetail, RusotoError<DescribeWorkflowTypeError>> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "SimpleWorkflowService.DescribeWorkflowType");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<WorkflowTypeDetail, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeWorkflowTypeError::from_response(response))
        }
    }

    /// <p>Returns the history of the specified workflow execution. The results may be split into multiple pages. To retrieve subsequent pages, make the call again using the <code>nextPageToken</code> returned by the initial call.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn get_workflow_execution_history(
        &self,
        input: GetWorkflowExecutionHistoryInput,
    ) -> Result<History, RusotoError<GetWorkflowExecutionHistoryError>> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "SimpleWorkflowService.GetWorkflowExecutionHistory",
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
            proto::json::ResponsePayload::new(&response).deserialize::<History, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetWorkflowExecutionHistoryError::from_response(response))
        }
    }

    /// <p>Returns information about all activities registered in the specified domain that match the specified name and registration status. The result includes information like creation date, current status of the activity, etc. The results may be split into multiple pages. To retrieve subsequent pages, make the call again using the <code>nextPageToken</code> returned by the initial call.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn list_activity_types(
        &self,
        input: ListActivityTypesInput,
    ) -> Result<ActivityTypeInfos, RusotoError<ListActivityTypesError>> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "SimpleWorkflowService.ListActivityTypes");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ActivityTypeInfos, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListActivityTypesError::from_response(response))
        }
    }

    /// <p>Returns a list of closed workflow executions in the specified domain that meet the filtering criteria. The results may be split into multiple pages. To retrieve subsequent pages, make the call again using the nextPageToken returned by the initial call.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>tagFilter.tag</code>: String constraint. The key is <code>swf:tagFilter.tag</code>.</p> </li> <li> <p> <code>typeFilter.name</code>: String constraint. The key is <code>swf:typeFilter.name</code>.</p> </li> <li> <p> <code>typeFilter.version</code>: String constraint. The key is <code>swf:typeFilter.version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn list_closed_workflow_executions(
        &self,
        input: ListClosedWorkflowExecutionsInput,
    ) -> Result<WorkflowExecutionInfos, RusotoError<ListClosedWorkflowExecutionsError>> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "SimpleWorkflowService.ListClosedWorkflowExecutions",
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
            proto::json::ResponsePayload::new(&response).deserialize::<WorkflowExecutionInfos, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListClosedWorkflowExecutionsError::from_response(response))
        }
    }

    /// <p>Returns the list of domains registered in the account. The results may be split into multiple pages. To retrieve subsequent pages, make the call again using the nextPageToken returned by the initial call.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains. The element must be set to <code>arn:aws:swf::AccountID:domain/*</code>, where <i>AccountID</i> is the account ID, with no dashes.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn list_domains(
        &self,
        input: ListDomainsInput,
    ) -> Result<DomainInfos, RusotoError<ListDomainsError>> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "SimpleWorkflowService.ListDomains");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DomainInfos, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListDomainsError::from_response(response))
        }
    }

    /// <p>Returns a list of open workflow executions in the specified domain that meet the filtering criteria. The results may be split into multiple pages. To retrieve subsequent pages, make the call again using the nextPageToken returned by the initial call.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>tagFilter.tag</code>: String constraint. The key is <code>swf:tagFilter.tag</code>.</p> </li> <li> <p> <code>typeFilter.name</code>: String constraint. The key is <code>swf:typeFilter.name</code>.</p> </li> <li> <p> <code>typeFilter.version</code>: String constraint. The key is <code>swf:typeFilter.version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn list_open_workflow_executions(
        &self,
        input: ListOpenWorkflowExecutionsInput,
    ) -> Result<WorkflowExecutionInfos, RusotoError<ListOpenWorkflowExecutionsError>> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "SimpleWorkflowService.ListOpenWorkflowExecutions",
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
            proto::json::ResponsePayload::new(&response).deserialize::<WorkflowExecutionInfos, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListOpenWorkflowExecutionsError::from_response(response))
        }
    }

    /// <p>List tags for a given domain.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceInput,
    ) -> Result<ListTagsForResourceOutput, RusotoError<ListTagsForResourceError>> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "SimpleWorkflowService.ListTagsForResource");
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

    /// <p>Returns information about workflow types in the specified domain. The results may be split into multiple pages that can be retrieved by making the call repeatedly.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn list_workflow_types(
        &self,
        input: ListWorkflowTypesInput,
    ) -> Result<WorkflowTypeInfos, RusotoError<ListWorkflowTypesError>> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "SimpleWorkflowService.ListWorkflowTypes");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<WorkflowTypeInfos, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListWorkflowTypesError::from_response(response))
        }
    }

    /// <p>Used by workers to get an <a>ActivityTask</a> from the specified activity <code>taskList</code>. This initiates a long poll, where the service holds the HTTP connection open and responds as soon as a task becomes available. The maximum time the service holds on to the request before responding is 60 seconds. If no task is available within 60 seconds, the poll returns an empty result. An empty result, in this context, means that an ActivityTask is returned, but that the value of taskToken is an empty string. If a task is returned, the worker should use its type to identify and process it correctly.</p> <important> <p>Workers should set their client side socket timeout to at least 70 seconds (10 seconds higher than the maximum time service may hold the poll request).</p> </important> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the <code>taskList.name</code> parameter by using a <code>Condition</code> element with the <code>swf:taskList.name</code> key to allow the action to access only certain task lists.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn poll_for_activity_task(
        &self,
        input: PollForActivityTaskInput,
    ) -> Result<ActivityTask, RusotoError<PollForActivityTaskError>> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "SimpleWorkflowService.PollForActivityTask");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ActivityTask, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PollForActivityTaskError::from_response(response))
        }
    }

    /// <p>Used by deciders to get a <a>DecisionTask</a> from the specified decision <code>taskList</code>. A decision task may be returned for any open workflow execution that is using the specified task list. The task includes a paginated view of the history of the workflow execution. The decider should use the workflow type and the history to determine how to properly handle the task.</p> <p>This action initiates a long poll, where the service holds the HTTP connection open and responds as soon a task becomes available. If no decision task is available in the specified task list before the timeout of 60 seconds expires, an empty result is returned. An empty result, in this context, means that a DecisionTask is returned, but that the value of taskToken is an empty string.</p> <important> <p>Deciders should set their client side socket timeout to at least 70 seconds (10 seconds higher than the timeout).</p> </important> <important> <p>Because the number of workflow history events for a single workflow execution might be very large, the result returned might be split up across a number of pages. To retrieve subsequent pages, make additional calls to <code>PollForDecisionTask</code> using the <code>nextPageToken</code> returned by the initial call. Note that you do <i>not</i> call <code>GetWorkflowExecutionHistory</code> with this <code>nextPageToken</code>. Instead, call <code>PollForDecisionTask</code> again.</p> </important> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the <code>taskList.name</code> parameter by using a <code>Condition</code> element with the <code>swf:taskList.name</code> key to allow the action to access only certain task lists.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn poll_for_decision_task(
        &self,
        input: PollForDecisionTaskInput,
    ) -> Result<DecisionTask, RusotoError<PollForDecisionTaskError>> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "SimpleWorkflowService.PollForDecisionTask");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DecisionTask, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PollForDecisionTaskError::from_response(response))
        }
    }

    /// <p>Used by activity workers to report to the service that the <a>ActivityTask</a> represented by the specified <code>taskToken</code> is still making progress. The worker can also specify details of the progress, for example percent complete, using the <code>details</code> parameter. This action can also be used by the worker as a mechanism to check if cancellation is being requested for the activity task. If a cancellation is being attempted for the specified task, then the boolean <code>cancelRequested</code> flag returned by the service is set to <code>true</code>.</p> <p>This action resets the <code>taskHeartbeatTimeout</code> clock. The <code>taskHeartbeatTimeout</code> is specified in <a>RegisterActivityType</a>.</p> <p>This action doesn't in itself create an event in the workflow execution history. However, if the task times out, the workflow execution history contains a <code>ActivityTaskTimedOut</code> event that contains the information from the last heartbeat generated by the activity worker.</p> <note> <p>The <code>taskStartToCloseTimeout</code> of an activity type is the maximum duration of an activity task, regardless of the number of <a>RecordActivityTaskHeartbeat</a> requests received. The <code>taskStartToCloseTimeout</code> is also specified in <a>RegisterActivityType</a>.</p> </note> <note> <p>This operation is only useful for long-lived activities to report liveliness of the task and to determine if a cancellation is being attempted.</p> </note> <important> <p>If the <code>cancelRequested</code> flag returns <code>true</code>, a cancellation is being attempted. If the worker can cancel the activity, it should respond with <a>RespondActivityTaskCanceled</a>. Otherwise, it should ignore the cancellation request.</p> </important> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn record_activity_task_heartbeat(
        &self,
        input: RecordActivityTaskHeartbeatInput,
    ) -> Result<ActivityTaskStatus, RusotoError<RecordActivityTaskHeartbeatError>> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "SimpleWorkflowService.RecordActivityTaskHeartbeat",
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
            proto::json::ResponsePayload::new(&response).deserialize::<ActivityTaskStatus, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(RecordActivityTaskHeartbeatError::from_response(response))
        }
    }

    /// <p>Registers a new <i>activity type</i> along with its configuration settings in the specified domain.</p> <important> <p>A <code>TypeAlreadyExists</code> fault is returned if the type already exists in the domain. You cannot change any configuration settings of the type after its registration, and it must be registered as a new version.</p> </important> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>defaultTaskList.name</code>: String constraint. The key is <code>swf:defaultTaskList.name</code>.</p> </li> <li> <p> <code>name</code>: String constraint. The key is <code>swf:name</code>.</p> </li> <li> <p> <code>version</code>: String constraint. The key is <code>swf:version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn register_activity_type(
        &self,
        input: RegisterActivityTypeInput,
    ) -> Result<(), RusotoError<RegisterActivityTypeError>> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "SimpleWorkflowService.RegisterActivityType");
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
            Err(RegisterActivityTypeError::from_response(response))
        }
    }

    /// <p>Registers a new domain.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>You cannot use an IAM policy to control domain access for this action. The name of the domain being registered is available as the resource of this action.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn register_domain(
        &self,
        input: RegisterDomainInput,
    ) -> Result<(), RusotoError<RegisterDomainError>> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "SimpleWorkflowService.RegisterDomain");
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
            Err(RegisterDomainError::from_response(response))
        }
    }

    /// <p>Registers a new <i>workflow type</i> and its configuration settings in the specified domain.</p> <p>The retention period for the workflow history is set by the <a>RegisterDomain</a> action.</p> <important> <p>If the type already exists, then a <code>TypeAlreadyExists</code> fault is returned. You cannot change the configuration settings of a workflow type once it is registered and it must be registered as a new version.</p> </important> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>defaultTaskList.name</code>: String constraint. The key is <code>swf:defaultTaskList.name</code>.</p> </li> <li> <p> <code>name</code>: String constraint. The key is <code>swf:name</code>.</p> </li> <li> <p> <code>version</code>: String constraint. The key is <code>swf:version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn register_workflow_type(
        &self,
        input: RegisterWorkflowTypeInput,
    ) -> Result<(), RusotoError<RegisterWorkflowTypeError>> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "SimpleWorkflowService.RegisterWorkflowType");
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
            Err(RegisterWorkflowTypeError::from_response(response))
        }
    }

    /// <p>Records a <code>WorkflowExecutionCancelRequested</code> event in the currently running workflow execution identified by the given domain, workflowId, and runId. This logically requests the cancellation of the workflow execution as a whole. It is up to the decider to take appropriate actions when it receives an execution history with this event.</p> <note> <p>If the runId isn't specified, the <code>WorkflowExecutionCancelRequested</code> event is recorded in the history of the current open workflow execution with the specified workflowId in the domain.</p> </note> <note> <p>Because this action allows the workflow to properly clean up and gracefully close, it should be used instead of <a>TerminateWorkflowExecution</a> when possible.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn request_cancel_workflow_execution(
        &self,
        input: RequestCancelWorkflowExecutionInput,
    ) -> Result<(), RusotoError<RequestCancelWorkflowExecutionError>> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "SimpleWorkflowService.RequestCancelWorkflowExecution",
        );
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
            Err(RequestCancelWorkflowExecutionError::from_response(response))
        }
    }

    /// <p>Used by workers to tell the service that the <a>ActivityTask</a> identified by the <code>taskToken</code> was successfully canceled. Additional <code>details</code> can be provided using the <code>details</code> argument.</p> <p>These <code>details</code> (if provided) appear in the <code>ActivityTaskCanceled</code> event added to the workflow history.</p> <important> <p>Only use this operation if the <code>canceled</code> flag of a <a>RecordActivityTaskHeartbeat</a> request returns <code>true</code> and if the activity can be safely undone or abandoned.</p> </important> <p>A task is considered open from the time that it is scheduled until it is closed. Therefore a task is reported as open while a worker is processing it. A task is closed after it has been specified in a call to <a>RespondActivityTaskCompleted</a>, RespondActivityTaskCanceled, <a>RespondActivityTaskFailed</a>, or the task has <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dg-basic.html#swf-dev-timeout-types">timed out</a>.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn respond_activity_task_canceled(
        &self,
        input: RespondActivityTaskCanceledInput,
    ) -> Result<(), RusotoError<RespondActivityTaskCanceledError>> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "SimpleWorkflowService.RespondActivityTaskCanceled",
        );
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
            Err(RespondActivityTaskCanceledError::from_response(response))
        }
    }

    /// <p>Used by workers to tell the service that the <a>ActivityTask</a> identified by the <code>taskToken</code> completed successfully with a <code>result</code> (if provided). The <code>result</code> appears in the <code>ActivityTaskCompleted</code> event in the workflow history.</p> <important> <p>If the requested task doesn't complete successfully, use <a>RespondActivityTaskFailed</a> instead. If the worker finds that the task is canceled through the <code>canceled</code> flag returned by <a>RecordActivityTaskHeartbeat</a>, it should cancel the task, clean up and then call <a>RespondActivityTaskCanceled</a>.</p> </important> <p>A task is considered open from the time that it is scheduled until it is closed. Therefore a task is reported as open while a worker is processing it. A task is closed after it has been specified in a call to RespondActivityTaskCompleted, <a>RespondActivityTaskCanceled</a>, <a>RespondActivityTaskFailed</a>, or the task has <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dg-basic.html#swf-dev-timeout-types">timed out</a>.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn respond_activity_task_completed(
        &self,
        input: RespondActivityTaskCompletedInput,
    ) -> Result<(), RusotoError<RespondActivityTaskCompletedError>> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "SimpleWorkflowService.RespondActivityTaskCompleted",
        );
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
            Err(RespondActivityTaskCompletedError::from_response(response))
        }
    }

    /// <p>Used by workers to tell the service that the <a>ActivityTask</a> identified by the <code>taskToken</code> has failed with <code>reason</code> (if specified). The <code>reason</code> and <code>details</code> appear in the <code>ActivityTaskFailed</code> event added to the workflow history.</p> <p>A task is considered open from the time that it is scheduled until it is closed. Therefore a task is reported as open while a worker is processing it. A task is closed after it has been specified in a call to <a>RespondActivityTaskCompleted</a>, <a>RespondActivityTaskCanceled</a>, RespondActivityTaskFailed, or the task has <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dg-basic.html#swf-dev-timeout-types">timed out</a>.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn respond_activity_task_failed(
        &self,
        input: RespondActivityTaskFailedInput,
    ) -> Result<(), RusotoError<RespondActivityTaskFailedError>> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "SimpleWorkflowService.RespondActivityTaskFailed",
        );
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
            Err(RespondActivityTaskFailedError::from_response(response))
        }
    }

    /// <p>Used by deciders to tell the service that the <a>DecisionTask</a> identified by the <code>taskToken</code> has successfully completed. The <code>decisions</code> argument specifies the list of decisions made while processing the task.</p> <p>A <code>DecisionTaskCompleted</code> event is added to the workflow history. The <code>executionContext</code> specified is attached to the event in the workflow execution history.</p> <p> <b>Access Control</b> </p> <p>If an IAM policy grants permission to use <code>RespondDecisionTaskCompleted</code>, it can express permissions for the list of decisions in the <code>decisions</code> parameter. Each of the decisions has one or more parameters, much like a regular API call. To allow for policies to be as readable as possible, you can express permissions on decisions as if they were actual API calls, including applying conditions to some parameters. For more information, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn respond_decision_task_completed(
        &self,
        input: RespondDecisionTaskCompletedInput,
    ) -> Result<(), RusotoError<RespondDecisionTaskCompletedError>> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "SimpleWorkflowService.RespondDecisionTaskCompleted",
        );
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
            Err(RespondDecisionTaskCompletedError::from_response(response))
        }
    }

    /// <p>Records a <code>WorkflowExecutionSignaled</code> event in the workflow execution history and creates a decision task for the workflow execution identified by the given domain, workflowId and runId. The event is recorded with the specified user defined signalName and input (if provided).</p> <note> <p>If a runId isn't specified, then the <code>WorkflowExecutionSignaled</code> event is recorded in the history of the current open workflow with the matching workflowId in the domain.</p> </note> <note> <p>If the specified workflow execution isn't open, this method fails with <code>UnknownResource</code>.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn signal_workflow_execution(
        &self,
        input: SignalWorkflowExecutionInput,
    ) -> Result<(), RusotoError<SignalWorkflowExecutionError>> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "SimpleWorkflowService.SignalWorkflowExecution",
        );
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
            Err(SignalWorkflowExecutionError::from_response(response))
        }
    }

    /// <p>Starts an execution of the workflow type in the specified domain using the provided <code>workflowId</code> and input data.</p> <p>This action returns the newly started workflow execution.</p> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>tagList.member.0</code>: The key is <code>swf:tagList.member.0</code>.</p> </li> <li> <p> <code>tagList.member.1</code>: The key is <code>swf:tagList.member.1</code>.</p> </li> <li> <p> <code>tagList.member.2</code>: The key is <code>swf:tagList.member.2</code>.</p> </li> <li> <p> <code>tagList.member.3</code>: The key is <code>swf:tagList.member.3</code>.</p> </li> <li> <p> <code>tagList.member.4</code>: The key is <code>swf:tagList.member.4</code>.</p> </li> <li> <p> <code>taskList</code>: String constraint. The key is <code>swf:taskList.name</code>.</p> </li> <li> <p> <code>workflowType.name</code>: String constraint. The key is <code>swf:workflowType.name</code>.</p> </li> <li> <p> <code>workflowType.version</code>: String constraint. The key is <code>swf:workflowType.version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn start_workflow_execution(
        &self,
        input: StartWorkflowExecutionInput,
    ) -> Result<Run, RusotoError<StartWorkflowExecutionError>> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "SimpleWorkflowService.StartWorkflowExecution",
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
            proto::json::ResponsePayload::new(&response).deserialize::<Run, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StartWorkflowExecutionError::from_response(response))
        }
    }

    /// <p><p>Add a tag to a Amazon SWF domain.</p> <note> <p>Amazon SWF supports a maximum of 50 tags per resource.</p> </note></p>
    async fn tag_resource(
        &self,
        input: TagResourceInput,
    ) -> Result<(), RusotoError<TagResourceError>> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "SimpleWorkflowService.TagResource");
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
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Records a <code>WorkflowExecutionTerminated</code> event and forces closure of the workflow execution identified by the given domain, runId, and workflowId. The child policy, registered with the workflow type or specified when starting this execution, is applied to any open child workflow executions of this workflow execution.</p> <important> <p>If the identified workflow execution was in progress, it is terminated immediately.</p> </important> <note> <p>If a runId isn't specified, then the <code>WorkflowExecutionTerminated</code> event is recorded in the history of the current open workflow with the matching workflowId in the domain.</p> </note> <note> <p>You should consider using <a>RequestCancelWorkflowExecution</a> action instead because it allows the workflow to gracefully close while <a>TerminateWorkflowExecution</a> doesn't.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn terminate_workflow_execution(
        &self,
        input: TerminateWorkflowExecutionInput,
    ) -> Result<(), RusotoError<TerminateWorkflowExecutionError>> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "SimpleWorkflowService.TerminateWorkflowExecution",
        );
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
            Err(TerminateWorkflowExecutionError::from_response(response))
        }
    }

    /// <p>Undeprecates a previously deprecated <i>activity type</i>. After an activity type has been undeprecated, you can create new tasks of that activity type.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>activityType.name</code>: String constraint. The key is <code>swf:activityType.name</code>.</p> </li> <li> <p> <code>activityType.version</code>: String constraint. The key is <code>swf:activityType.version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn undeprecate_activity_type(
        &self,
        input: UndeprecateActivityTypeInput,
    ) -> Result<(), RusotoError<UndeprecateActivityTypeError>> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "SimpleWorkflowService.UndeprecateActivityType",
        );
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
            Err(UndeprecateActivityTypeError::from_response(response))
        }
    }

    /// <p>Undeprecates a previously deprecated domain. After a domain has been undeprecated it can be used to create new workflow executions or register new types.</p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>You cannot use an IAM policy to constrain this action's parameters.</p> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn undeprecate_domain(
        &self,
        input: UndeprecateDomainInput,
    ) -> Result<(), RusotoError<UndeprecateDomainError>> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "SimpleWorkflowService.UndeprecateDomain");
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
            Err(UndeprecateDomainError::from_response(response))
        }
    }

    /// <p>Undeprecates a previously deprecated <i>workflow type</i>. After a workflow type has been undeprecated, you can create new executions of that type. </p> <note> <p>This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and changes.</p> </note> <p> <b>Access Control</b> </p> <p>You can use IAM policies to control this action's access to Amazon SWF resources as follows:</p> <ul> <li> <p>Use a <code>Resource</code> element with the domain name to limit the action to only specified domains.</p> </li> <li> <p>Use an <code>Action</code> element to allow or deny permission to call this action.</p> </li> <li> <p>Constrain the following parameters by using a <code>Condition</code> element with the appropriate keys.</p> <ul> <li> <p> <code>workflowType.name</code>: String constraint. The key is <code>swf:workflowType.name</code>.</p> </li> <li> <p> <code>workflowType.version</code>: String constraint. The key is <code>swf:workflowType.version</code>.</p> </li> </ul> </li> </ul> <p>If the caller doesn't have sufficient permissions to invoke the action, or the parameter values fall outside the specified constraints, the action fails. The associated event attribute's <code>cause</code> parameter is set to <code>OPERATION_NOT_PERMITTED</code>. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    async fn undeprecate_workflow_type(
        &self,
        input: UndeprecateWorkflowTypeInput,
    ) -> Result<(), RusotoError<UndeprecateWorkflowTypeError>> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "SimpleWorkflowService.UndeprecateWorkflowType",
        );
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
            Err(UndeprecateWorkflowTypeError::from_response(response))
        }
    }

    /// <p>Remove a tag from a Amazon SWF domain.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceInput,
    ) -> Result<(), RusotoError<UntagResourceError>> {
        let mut request = SignedRequest::new("POST", "swf", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "SimpleWorkflowService.UntagResource");
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
            Err(UntagResourceError::from_response(response))
        }
    }
}
