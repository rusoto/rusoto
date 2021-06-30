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
/// <p>Contains the configuration information of an acknowledge action.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AcknowledgeActionConfiguration {
    /// <p>The note that you can leave when you acknowledge the alarm.</p>
    #[serde(rename = "note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

/// <p>Information needed to acknowledge the alarm.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AcknowledgeAlarmActionRequest {
    /// <p>The name of the alarm model.</p>
    #[serde(rename = "alarmModelName")]
    pub alarm_model_name: String,
    /// <p>The value of the key used as a filter to select only the alarms associated with the <a href="https://docs.aws.amazon.com/iotevents/latest/apireference/API_CreateAlarmModel.html#iotevents-CreateAlarmModel-request-key">key</a>.</p>
    #[serde(rename = "keyValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_value: Option<String>,
    /// <p>The note that you can leave when you acknowledge the alarm.</p>
    #[serde(rename = "note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// <p>The request ID. Each ID must be unique within each batch.</p>
    #[serde(rename = "requestId")]
    pub request_id: String,
}

/// <p>Contains information about an alarm.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Alarm {
    /// <p>The name of the alarm model.</p>
    #[serde(rename = "alarmModelName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_model_name: Option<String>,
    /// <p>The version of the alarm model.</p>
    #[serde(rename = "alarmModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_model_version: Option<String>,
    /// <p>Contains information about the current state of the alarm.</p>
    #[serde(rename = "alarmState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_state: Option<AlarmState>,
    /// <p>The time the alarm was created, in the Unix epoch format.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The value of the key used as a filter to select only the alarms associated with the <a href="https://docs.aws.amazon.com/iotevents/latest/apireference/API_CreateAlarmModel.html#iotevents-CreateAlarmModel-request-key">key</a>.</p>
    #[serde(rename = "keyValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_value: Option<String>,
    /// <p>The time the alarm was last updated, in the Unix epoch format.</p>
    #[serde(rename = "lastUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    /// <p>A non-negative integer that reflects the severity level of the alarm.</p>
    #[serde(rename = "severity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<i64>,
}

/// <p>Contains information about the current state of the alarm.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AlarmState {
    /// <p>Contains information about the action that you can take to respond to the alarm.</p>
    #[serde(rename = "customerAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_action: Option<CustomerAction>,
    /// <p>Information needed to evaluate data.</p>
    #[serde(rename = "ruleEvaluation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_evaluation: Option<RuleEvaluation>,
    /// <p><p>The name of the alarm state. The state name can be one of the following values:</p> <ul> <li> <p> <code>DISABLED</code> - When the alarm is in the <code>DISABLED</code> state, it isn&#39;t ready to evaluate data. To enable the alarm, you must change the alarm to the <code>NORMAL</code> state.</p> </li> <li> <p> <code>NORMAL</code> - When the alarm is in the <code>NORMAL</code> state, it&#39;s ready to evaluate data.</p> </li> <li> <p> <code>ACTIVE</code> - If the alarm is in the <code>ACTIVE</code> state, the alarm is invoked.</p> </li> <li> <p> <code>ACKNOWLEDGED</code> - When the alarm is in the <code>ACKNOWLEDGED</code> state, the alarm was invoked and you acknowledged the alarm.</p> </li> <li> <p> <code>SNOOZE<em>DISABLED</code> - When the alarm is in the <code>SNOOZE</em>DISABLED</code> state, the alarm is disabled for a specified period of time. After the snooze time, the alarm automatically changes to the <code>NORMAL</code> state. </p> </li> <li> <p> <code>LATCHED</code> - When the alarm is in the <code>LATCHED</code> state, the alarm was invoked. However, the data that the alarm is currently evaluating is within the specified range. To change the alarm to the <code>NORMAL</code> state, you must acknowledge the alarm.</p> </li> </ul></p>
    #[serde(rename = "stateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_name: Option<String>,
    /// <p>Contains information about alarm state changes.</p>
    #[serde(rename = "systemEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_event: Option<SystemEvent>,
}

/// <p>Contains a summary of an alarm.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AlarmSummary {
    /// <p>The name of the alarm model.</p>
    #[serde(rename = "alarmModelName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_model_name: Option<String>,
    /// <p>The version of the alarm model.</p>
    #[serde(rename = "alarmModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_model_version: Option<String>,
    /// <p>The time the alarm was created, in the Unix epoch format.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The value of the key used as a filter to select only the alarms associated with the <a href="https://docs.aws.amazon.com/iotevents/latest/apireference/API_CreateAlarmModel.html#iotevents-CreateAlarmModel-request-key">key</a>.</p>
    #[serde(rename = "keyValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_value: Option<String>,
    /// <p>The time the alarm was last updated, in the Unix epoch format.</p>
    #[serde(rename = "lastUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    /// <p><p>The name of the alarm state. The state name can be one of the following values:</p> <ul> <li> <p> <code>DISABLED</code> - When the alarm is in the <code>DISABLED</code> state, it isn&#39;t ready to evaluate data. To enable the alarm, you must change the alarm to the <code>NORMAL</code> state.</p> </li> <li> <p> <code>NORMAL</code> - When the alarm is in the <code>NORMAL</code> state, it&#39;s ready to evaluate data.</p> </li> <li> <p> <code>ACTIVE</code> - If the alarm is in the <code>ACTIVE</code> state, the alarm is invoked.</p> </li> <li> <p> <code>ACKNOWLEDGED</code> - When the alarm is in the <code>ACKNOWLEDGED</code> state, the alarm was invoked and you acknowledged the alarm.</p> </li> <li> <p> <code>SNOOZE<em>DISABLED</code> - When the alarm is in the <code>SNOOZE</em>DISABLED</code> state, the alarm is disabled for a specified period of time. After the snooze time, the alarm automatically changes to the <code>NORMAL</code> state. </p> </li> <li> <p> <code>LATCHED</code> - When the alarm is in the <code>LATCHED</code> state, the alarm was invoked. However, the data that the alarm is currently evaluating is within the specified range. To change the alarm to the <code>NORMAL</code> state, you must acknowledge the alarm.</p> </li> </ul></p>
    #[serde(rename = "stateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchAcknowledgeAlarmRequest {
    /// <p>The list of acknowledge action requests. You can specify up to 10 requests per operation.</p>
    #[serde(rename = "acknowledgeActionRequests")]
    pub acknowledge_action_requests: Vec<AcknowledgeAlarmActionRequest>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchAcknowledgeAlarmResponse {
    /// <p>A list of errors associated with the request, or <code>null</code> if there are no errors. Each error entry contains an entry ID that helps you identify the entry that failed.</p>
    #[serde(rename = "errorEntries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_entries: Option<Vec<BatchAlarmActionErrorEntry>>,
}

/// <p><p>Contains error messages associated with one of the following requests:</p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/iotevents/latest/apireference/API_iotevents-data_BatchAcknowledgeAlarm.html">BatchAcknowledgeAlarm</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/iotevents/latest/apireference/API_iotevents-data_BatchDisableAlarm.html">BatchDisableAlarm</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/iotevents/latest/apireference/API_iotevents-data_BatchEnableAlarm.html">BatchEnableAlarm</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/iotevents/latest/apireference/API_iotevents-data_BatchResetAlarm.html">BatchResetAlarm</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/iotevents/latest/apireference/API_iotevents-data_BatchSnoozeAlarm.html">BatchSnoozeAlarm</a> </p> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchAlarmActionErrorEntry {
    /// <p>The error code.</p>
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>A message that describes the error.</p>
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The request ID. Each ID must be unique within each batch.</p>
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchDisableAlarmRequest {
    /// <p>The list of disable action requests. You can specify up to 10 requests per operation.</p>
    #[serde(rename = "disableActionRequests")]
    pub disable_action_requests: Vec<DisableAlarmActionRequest>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchDisableAlarmResponse {
    /// <p>A list of errors associated with the request, or <code>null</code> if there are no errors. Each error entry contains an entry ID that helps you identify the entry that failed.</p>
    #[serde(rename = "errorEntries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_entries: Option<Vec<BatchAlarmActionErrorEntry>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchEnableAlarmRequest {
    /// <p>The list of enable action requests. You can specify up to 10 requests per operation.</p>
    #[serde(rename = "enableActionRequests")]
    pub enable_action_requests: Vec<EnableAlarmActionRequest>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchEnableAlarmResponse {
    /// <p>A list of errors associated with the request, or <code>null</code> if there are no errors. Each error entry contains an entry ID that helps you identify the entry that failed.</p>
    #[serde(rename = "errorEntries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_entries: Option<Vec<BatchAlarmActionErrorEntry>>,
}

/// <p>Contains information about the errors encountered.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchPutMessageErrorEntry {
    /// <p>The error code.</p>
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>A message that describes the error.</p>
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The ID of the message that caused the error. (See the value corresponding to the <code>"messageId"</code> key in the <code>"message"</code> object.)</p>
    #[serde(rename = "messageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchPutMessageRequest {
    /// <p>The list of messages to send. Each message has the following format: <code>'{ "messageId": "string", "inputName": "string", "payload": "string"}'</code> </p>
    #[serde(rename = "messages")]
    pub messages: Vec<Message>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchPutMessageResponse {
    /// <p>A list of any errors encountered when sending the messages.</p>
    #[serde(rename = "BatchPutMessageErrorEntries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_put_message_error_entries: Option<Vec<BatchPutMessageErrorEntry>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchResetAlarmRequest {
    /// <p>The list of reset action requests. You can specify up to 10 requests per operation.</p>
    #[serde(rename = "resetActionRequests")]
    pub reset_action_requests: Vec<ResetAlarmActionRequest>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchResetAlarmResponse {
    /// <p>A list of errors associated with the request, or <code>null</code> if there are no errors. Each error entry contains an entry ID that helps you identify the entry that failed.</p>
    #[serde(rename = "errorEntries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_entries: Option<Vec<BatchAlarmActionErrorEntry>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchSnoozeAlarmRequest {
    /// <p>The list of snooze action requests. You can specify up to 10 requests per operation.</p>
    #[serde(rename = "snoozeActionRequests")]
    pub snooze_action_requests: Vec<SnoozeAlarmActionRequest>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchSnoozeAlarmResponse {
    /// <p>A list of errors associated with the request, or <code>null</code> if there are no errors. Each error entry contains an entry ID that helps you identify the entry that failed.</p>
    #[serde(rename = "errorEntries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_entries: Option<Vec<BatchAlarmActionErrorEntry>>,
}

/// <p>Information about the error that occurred when attempting to update a detector.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchUpdateDetectorErrorEntry {
    /// <p>The error code.</p>
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>A message that describes the error.</p>
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The <code>"messageId"</code> of the update request that caused the error. (The value of the <code>"messageId"</code> in the update request <code>"Detector"</code> object.)</p>
    #[serde(rename = "messageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchUpdateDetectorRequest {
    /// <p>The list of detectors (instances) to update, along with the values to update.</p>
    #[serde(rename = "detectors")]
    pub detectors: Vec<UpdateDetectorRequest>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchUpdateDetectorResponse {
    /// <p>A list of those detector updates that resulted in errors. (If an error is listed here, the specific update did not occur.)</p>
    #[serde(rename = "batchUpdateDetectorErrorEntries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_update_detector_error_entries: Option<Vec<BatchUpdateDetectorErrorEntry>>,
}

/// <p>Contains information about the action that you can take to respond to the alarm.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CustomerAction {
    /// <p>Contains the configuration information of an acknowledge action.</p>
    #[serde(rename = "acknowledgeActionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acknowledge_action_configuration: Option<AcknowledgeActionConfiguration>,
    /// <p>The name of the action. The action name can be one of the following values:</p> <ul> <li> <p> <code>SNOOZE</code> - When you snooze the alarm, the alarm state changes to <code>SNOOZE_DISABLED</code>.</p> </li> <li> <p> <code>ENABLE</code> - When you enable the alarm, the alarm state changes to <code>NORMAL</code>.</p> </li> <li> <p> <code>DISABLE</code> - When you disable the alarm, the alarm state changes to <code>DISABLED</code>.</p> </li> <li> <p> <code>ACKNOWLEDGE</code> - When you acknowledge the alarm, the alarm state changes to <code>ACKNOWLEDGED</code>.</p> </li> <li> <p> <code>RESET</code> - When you reset the alarm, the alarm state changes to <code>NORMAL</code>.</p> </li> </ul> <p>For more information, see the <a href="https://docs.aws.amazon.com/iotevents/latest/apireference/API_iotevents-data_AlarmState.html">AlarmState</a> API.</p>
    #[serde(rename = "actionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_name: Option<String>,
    /// <p>Contains the configuration information of a disable action.</p>
    #[serde(rename = "disableActionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_action_configuration: Option<DisableActionConfiguration>,
    /// <p>Contains the configuration information of an enable action.</p>
    #[serde(rename = "enableActionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_action_configuration: Option<EnableActionConfiguration>,
    /// <p>Contains the configuration information of a reset action.</p>
    #[serde(rename = "resetActionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_action_configuration: Option<ResetActionConfiguration>,
    /// <p>Contains the configuration information of a snooze action.</p>
    #[serde(rename = "snoozeActionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snooze_action_configuration: Option<SnoozeActionConfiguration>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeAlarmRequest {
    /// <p>The name of the alarm model.</p>
    #[serde(rename = "alarmModelName")]
    pub alarm_model_name: String,
    /// <p>The value of the key used as a filter to select only the alarms associated with the <a href="https://docs.aws.amazon.com/iotevents/latest/apireference/API_CreateAlarmModel.html#iotevents-CreateAlarmModel-request-key">key</a>.</p>
    #[serde(rename = "keyValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_value: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAlarmResponse {
    /// <p>Contains information about an alarm.</p>
    #[serde(rename = "alarm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm: Option<Alarm>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDetectorRequest {
    /// <p>The name of the detector model whose detectors (instances) you want information about.</p>
    #[serde(rename = "detectorModelName")]
    pub detector_model_name: String,
    /// <p>A filter used to limit results to detectors (instances) created because of the given key ID.</p>
    #[serde(rename = "keyValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_value: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeDetectorResponse {
    /// <p>Information about the detector (instance).</p>
    #[serde(rename = "detector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector: Option<Detector>,
}

/// <p>Information about the detector (instance).</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Detector {
    /// <p>The time the detector (instance) was created.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The name of the detector model that created this detector (instance).</p>
    #[serde(rename = "detectorModelName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_model_name: Option<String>,
    /// <p>The version of the detector model that created this detector (instance).</p>
    #[serde(rename = "detectorModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_model_version: Option<String>,
    /// <p>The value of the key (identifying the device or system) that caused the creation of this detector (instance).</p>
    #[serde(rename = "keyValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_value: Option<String>,
    /// <p>The time the detector (instance) was last updated.</p>
    #[serde(rename = "lastUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    /// <p>The current state of the detector (instance).</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<DetectorState>,
}

/// <p>Information about the current state of the detector instance.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DetectorState {
    /// <p>The name of the state.</p>
    #[serde(rename = "stateName")]
    pub state_name: String,
    /// <p>The current state of the detector's timers.</p>
    #[serde(rename = "timers")]
    pub timers: Vec<Timer>,
    /// <p>The current values of the detector's variables.</p>
    #[serde(rename = "variables")]
    pub variables: Vec<Variable>,
}

/// <p>The new state, variable values, and timer settings of the detector (instance).</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DetectorStateDefinition {
    /// <p>The name of the new state of the detector (instance).</p>
    #[serde(rename = "stateName")]
    pub state_name: String,
    /// <p>The new values of the detector's timers. Any timer whose value isn't specified is cleared, and its timeout event won't occur.</p>
    #[serde(rename = "timers")]
    pub timers: Vec<TimerDefinition>,
    /// <p>The new values of the detector's variables. Any variable whose value isn't specified is cleared.</p>
    #[serde(rename = "variables")]
    pub variables: Vec<VariableDefinition>,
}

/// <p>Information about the detector state.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DetectorStateSummary {
    /// <p>The name of the state.</p>
    #[serde(rename = "stateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_name: Option<String>,
}

/// <p>Information about the detector (instance).</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DetectorSummary {
    /// <p>The time the detector (instance) was created.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The name of the detector model that created this detector (instance).</p>
    #[serde(rename = "detectorModelName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_model_name: Option<String>,
    /// <p>The version of the detector model that created this detector (instance).</p>
    #[serde(rename = "detectorModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_model_version: Option<String>,
    /// <p>The value of the key (identifying the device or system) that caused the creation of this detector (instance).</p>
    #[serde(rename = "keyValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_value: Option<String>,
    /// <p>The time the detector (instance) was last updated.</p>
    #[serde(rename = "lastUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    /// <p>The current state of the detector (instance).</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<DetectorStateSummary>,
}

/// <p>Contains the configuration information of a disable action.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisableActionConfiguration {
    /// <p>The note that you can leave when you disable the alarm.</p>
    #[serde(rename = "note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

/// <p>Information used to disable the alarm.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisableAlarmActionRequest {
    /// <p>The name of the alarm model.</p>
    #[serde(rename = "alarmModelName")]
    pub alarm_model_name: String,
    /// <p>The value of the key used as a filter to select only the alarms associated with the <a href="https://docs.aws.amazon.com/iotevents/latest/apireference/API_CreateAlarmModel.html#iotevents-CreateAlarmModel-request-key">key</a>.</p>
    #[serde(rename = "keyValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_value: Option<String>,
    /// <p>The note that you can leave when you disable the alarm.</p>
    #[serde(rename = "note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// <p>The request ID. Each ID must be unique within each batch.</p>
    #[serde(rename = "requestId")]
    pub request_id: String,
}

/// <p>Contains the configuration information of an enable action.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EnableActionConfiguration {
    /// <p>The note that you can leave when you enable the alarm.</p>
    #[serde(rename = "note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

/// <p>Information needed to enable the alarm.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EnableAlarmActionRequest {
    /// <p>The name of the alarm model.</p>
    #[serde(rename = "alarmModelName")]
    pub alarm_model_name: String,
    /// <p>The value of the key used as a filter to select only the alarms associated with the <a href="https://docs.aws.amazon.com/iotevents/latest/apireference/API_CreateAlarmModel.html#iotevents-CreateAlarmModel-request-key">key</a>.</p>
    #[serde(rename = "keyValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_value: Option<String>,
    /// <p>The note that you can leave when you enable the alarm.</p>
    #[serde(rename = "note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// <p>The request ID. Each ID must be unique within each batch.</p>
    #[serde(rename = "requestId")]
    pub request_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListAlarmsRequest {
    /// <p>The name of the alarm model.</p>
    #[serde(rename = "alarmModelName")]
    pub alarm_model_name: String,
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that you can use to return the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListAlarmsResponse {
    /// <p>A list that summarizes each alarm.</p>
    #[serde(rename = "alarmSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_summaries: Option<Vec<AlarmSummary>>,
    /// <p>The token that you can use to return the next set of results, or <code>null</code> if there are no more results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDetectorsRequest {
    /// <p>The name of the detector model whose detectors (instances) are listed.</p>
    #[serde(rename = "detectorModelName")]
    pub detector_model_name: String,
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that you can use to return the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A filter that limits results to those detectors (instances) in the given state.</p>
    #[serde(rename = "stateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_name: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDetectorsResponse {
    /// <p>A list of summary information about the detectors (instances).</p>
    #[serde(rename = "detectorSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_summaries: Option<Vec<DetectorSummary>>,
    /// <p>The token that you can use to return the next set of results, or <code>null</code> if there are no more results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Information about a message.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Message {
    /// <p>The name of the input into which the message payload is transformed.</p>
    #[serde(rename = "inputName")]
    pub input_name: String,
    /// <p>The ID to assign to the message. Within each batch sent, each <code>"messageId"</code> must be unique.</p>
    #[serde(rename = "messageId")]
    pub message_id: String,
    /// <p>The payload of the message. This can be a JSON string or a Base-64-encoded string representing binary data (in which case you must decode it).</p>
    #[serde(rename = "payload")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub payload: bytes::Bytes,
    /// <p>The timestamp associated with the message.</p>
    #[serde(rename = "timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<TimestampValue>,
}

/// <p>Contains the configuration information of a reset action.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResetActionConfiguration {
    /// <p>The note that you can leave when you reset the alarm.</p>
    #[serde(rename = "note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

/// <p>Information needed to reset the alarm.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ResetAlarmActionRequest {
    /// <p>The name of the alarm model.</p>
    #[serde(rename = "alarmModelName")]
    pub alarm_model_name: String,
    /// <p>The value of the key used as a filter to select only the alarms associated with the <a href="https://docs.aws.amazon.com/iotevents/latest/apireference/API_CreateAlarmModel.html#iotevents-CreateAlarmModel-request-key">key</a>.</p>
    #[serde(rename = "keyValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_value: Option<String>,
    /// <p>The note that you can leave when you reset the alarm.</p>
    #[serde(rename = "note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// <p>The request ID. Each ID must be unique within each batch.</p>
    #[serde(rename = "requestId")]
    pub request_id: String,
}

/// <p>Information needed to evaluate data.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RuleEvaluation {
    /// <p>Information needed to compare two values with a comparison operator.</p>
    #[serde(rename = "simpleRuleEvaluation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_rule_evaluation: Option<SimpleRuleEvaluation>,
}

/// <p>Information needed to compare two values with a comparison operator.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SimpleRuleEvaluation {
    /// <p>The value of the input property, on the left side of the comparison operator.</p>
    #[serde(rename = "inputPropertyValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_property_value: Option<String>,
    /// <p>The comparison operator.</p>
    #[serde(rename = "operator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    /// <p>The threshold value, on the right side of the comparison operator.</p>
    #[serde(rename = "thresholdValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_value: Option<String>,
}

/// <p>Contains the configuration information of a snooze action.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SnoozeActionConfiguration {
    /// <p>The note that you can leave when you snooze the alarm.</p>
    #[serde(rename = "note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// <p>The snooze time in seconds. The alarm automatically changes to the <code>NORMAL</code> state after this duration.</p>
    #[serde(rename = "snoozeDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snooze_duration: Option<i64>,
}

/// <p>Information needed to snooze the alarm.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SnoozeAlarmActionRequest {
    /// <p>The name of the alarm model.</p>
    #[serde(rename = "alarmModelName")]
    pub alarm_model_name: String,
    /// <p>The value of the key used as a filter to select only the alarms associated with the <a href="https://docs.aws.amazon.com/iotevents/latest/apireference/API_CreateAlarmModel.html#iotevents-CreateAlarmModel-request-key">key</a>.</p>
    #[serde(rename = "keyValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_value: Option<String>,
    /// <p>The note that you can leave when you snooze the alarm.</p>
    #[serde(rename = "note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// <p>The request ID. Each ID must be unique within each batch.</p>
    #[serde(rename = "requestId")]
    pub request_id: String,
    /// <p>The snooze time in seconds. The alarm automatically changes to the <code>NORMAL</code> state after this duration.</p>
    #[serde(rename = "snoozeDuration")]
    pub snooze_duration: i64,
}

/// <p>Contains the configuration information of alarm state changes.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StateChangeConfiguration {
    /// <p>The trigger type. If the value is <code>SNOOZE_TIMEOUT</code>, the snooze duration ends and the alarm automatically changes to the <code>NORMAL</code> state.</p>
    #[serde(rename = "triggerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_type: Option<String>,
}

/// <p>Contains information about alarm state changes.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SystemEvent {
    /// <p>The event type. If the value is <code>STATE_CHANGE</code>, the event contains information about alarm state changes.</p>
    #[serde(rename = "eventType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    /// <p>Contains the configuration information of alarm state changes.</p>
    #[serde(rename = "stateChangeConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_change_configuration: Option<StateChangeConfiguration>,
}

/// <p>The current state of a timer.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Timer {
    /// <p>The name of the timer.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The number of seconds which have elapsed on the timer.</p>
    #[serde(rename = "timestamp")]
    pub timestamp: f64,
}

/// <p>The new setting of a timer.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TimerDefinition {
    /// <p>The name of the timer.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The new setting of the timer (the number of seconds before the timer elapses).</p>
    #[serde(rename = "seconds")]
    pub seconds: i64,
}

/// <p>Contains information about a timestamp.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TimestampValue {
    /// <p>The value of the timestamp, in the Unix epoch format.</p>
    #[serde(rename = "timeInMillis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_millis: Option<i64>,
}

/// <p>Information used to update the detector (instance).</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateDetectorRequest {
    /// <p>The name of the detector model that created the detectors (instances).</p>
    #[serde(rename = "detectorModelName")]
    pub detector_model_name: String,
    /// <p>The value of the input key attribute (identifying the device or system) that caused the creation of this detector (instance).</p>
    #[serde(rename = "keyValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_value: Option<String>,
    /// <p>The ID to assign to the detector update <code>"message"</code>. Each <code>"messageId"</code> must be unique within each batch sent.</p>
    #[serde(rename = "messageId")]
    pub message_id: String,
    /// <p>The new state, variable values, and timer settings of the detector (instance).</p>
    #[serde(rename = "state")]
    pub state: DetectorStateDefinition,
}

/// <p>The current state of the variable.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Variable {
    /// <p>The name of the variable.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The current value of the variable.</p>
    #[serde(rename = "value")]
    pub value: String,
}

/// <p>The new value of the variable.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct VariableDefinition {
    /// <p>The name of the variable.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The new value of the variable.</p>
    #[serde(rename = "value")]
    pub value: String,
}

/// Errors returned by BatchAcknowledgeAlarm
#[derive(Debug, PartialEq)]
pub enum BatchAcknowledgeAlarmError {
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>The request was invalid.</p>
    InvalidRequest(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request could not be completed due to throttling.</p>
    Throttling(String),
}

impl BatchAcknowledgeAlarmError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchAcknowledgeAlarmError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(BatchAcknowledgeAlarmError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(BatchAcknowledgeAlarmError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(BatchAcknowledgeAlarmError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(BatchAcknowledgeAlarmError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchAcknowledgeAlarmError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchAcknowledgeAlarmError::InternalFailure(ref cause) => write!(f, "{}", cause),
            BatchAcknowledgeAlarmError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            BatchAcknowledgeAlarmError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            BatchAcknowledgeAlarmError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchAcknowledgeAlarmError {}
/// Errors returned by BatchDisableAlarm
#[derive(Debug, PartialEq)]
pub enum BatchDisableAlarmError {
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>The request was invalid.</p>
    InvalidRequest(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request could not be completed due to throttling.</p>
    Throttling(String),
}

impl BatchDisableAlarmError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchDisableAlarmError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(BatchDisableAlarmError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(BatchDisableAlarmError::InvalidRequest(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(BatchDisableAlarmError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(BatchDisableAlarmError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchDisableAlarmError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchDisableAlarmError::InternalFailure(ref cause) => write!(f, "{}", cause),
            BatchDisableAlarmError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            BatchDisableAlarmError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            BatchDisableAlarmError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchDisableAlarmError {}
/// Errors returned by BatchEnableAlarm
#[derive(Debug, PartialEq)]
pub enum BatchEnableAlarmError {
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>The request was invalid.</p>
    InvalidRequest(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request could not be completed due to throttling.</p>
    Throttling(String),
}

impl BatchEnableAlarmError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchEnableAlarmError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(BatchEnableAlarmError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(BatchEnableAlarmError::InvalidRequest(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(BatchEnableAlarmError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(BatchEnableAlarmError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchEnableAlarmError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchEnableAlarmError::InternalFailure(ref cause) => write!(f, "{}", cause),
            BatchEnableAlarmError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            BatchEnableAlarmError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            BatchEnableAlarmError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchEnableAlarmError {}
/// Errors returned by BatchPutMessage
#[derive(Debug, PartialEq)]
pub enum BatchPutMessageError {
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>The request was invalid.</p>
    InvalidRequest(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request could not be completed due to throttling.</p>
    Throttling(String),
}

impl BatchPutMessageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchPutMessageError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(BatchPutMessageError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(BatchPutMessageError::InvalidRequest(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(BatchPutMessageError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(BatchPutMessageError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchPutMessageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchPutMessageError::InternalFailure(ref cause) => write!(f, "{}", cause),
            BatchPutMessageError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            BatchPutMessageError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            BatchPutMessageError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchPutMessageError {}
/// Errors returned by BatchResetAlarm
#[derive(Debug, PartialEq)]
pub enum BatchResetAlarmError {
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>The request was invalid.</p>
    InvalidRequest(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request could not be completed due to throttling.</p>
    Throttling(String),
}

impl BatchResetAlarmError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchResetAlarmError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(BatchResetAlarmError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(BatchResetAlarmError::InvalidRequest(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(BatchResetAlarmError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(BatchResetAlarmError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchResetAlarmError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchResetAlarmError::InternalFailure(ref cause) => write!(f, "{}", cause),
            BatchResetAlarmError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            BatchResetAlarmError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            BatchResetAlarmError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchResetAlarmError {}
/// Errors returned by BatchSnoozeAlarm
#[derive(Debug, PartialEq)]
pub enum BatchSnoozeAlarmError {
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>The request was invalid.</p>
    InvalidRequest(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request could not be completed due to throttling.</p>
    Throttling(String),
}

impl BatchSnoozeAlarmError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchSnoozeAlarmError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(BatchSnoozeAlarmError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(BatchSnoozeAlarmError::InvalidRequest(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(BatchSnoozeAlarmError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(BatchSnoozeAlarmError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchSnoozeAlarmError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchSnoozeAlarmError::InternalFailure(ref cause) => write!(f, "{}", cause),
            BatchSnoozeAlarmError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            BatchSnoozeAlarmError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            BatchSnoozeAlarmError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchSnoozeAlarmError {}
/// Errors returned by BatchUpdateDetector
#[derive(Debug, PartialEq)]
pub enum BatchUpdateDetectorError {
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>The request was invalid.</p>
    InvalidRequest(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request could not be completed due to throttling.</p>
    Throttling(String),
}

impl BatchUpdateDetectorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchUpdateDetectorError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(BatchUpdateDetectorError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(BatchUpdateDetectorError::InvalidRequest(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(BatchUpdateDetectorError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(BatchUpdateDetectorError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchUpdateDetectorError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchUpdateDetectorError::InternalFailure(ref cause) => write!(f, "{}", cause),
            BatchUpdateDetectorError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            BatchUpdateDetectorError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            BatchUpdateDetectorError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchUpdateDetectorError {}
/// Errors returned by DescribeAlarm
#[derive(Debug, PartialEq)]
pub enum DescribeAlarmError {
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>The request was invalid.</p>
    InvalidRequest(String),
    /// <p>The resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request could not be completed due to throttling.</p>
    Throttling(String),
}

impl DescribeAlarmError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAlarmError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(DescribeAlarmError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeAlarmError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeAlarmError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeAlarmError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeAlarmError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeAlarmError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeAlarmError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DescribeAlarmError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeAlarmError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeAlarmError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DescribeAlarmError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeAlarmError {}
/// Errors returned by DescribeDetector
#[derive(Debug, PartialEq)]
pub enum DescribeDetectorError {
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>The request was invalid.</p>
    InvalidRequest(String),
    /// <p>The resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request could not be completed due to throttling.</p>
    Throttling(String),
}

impl DescribeDetectorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeDetectorError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(DescribeDetectorError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeDetectorError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeDetectorError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeDetectorError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeDetectorError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeDetectorError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDetectorError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DescribeDetectorError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeDetectorError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeDetectorError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DescribeDetectorError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeDetectorError {}
/// Errors returned by ListAlarms
#[derive(Debug, PartialEq)]
pub enum ListAlarmsError {
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>The request was invalid.</p>
    InvalidRequest(String),
    /// <p>The resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request could not be completed due to throttling.</p>
    Throttling(String),
}

impl ListAlarmsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAlarmsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(ListAlarmsError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListAlarmsError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListAlarmsError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListAlarmsError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListAlarmsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListAlarmsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListAlarmsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListAlarmsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListAlarmsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListAlarmsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListAlarmsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListAlarmsError {}
/// Errors returned by ListDetectors
#[derive(Debug, PartialEq)]
pub enum ListDetectorsError {
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>The request was invalid.</p>
    InvalidRequest(String),
    /// <p>The resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request could not be completed due to throttling.</p>
    Throttling(String),
}

impl ListDetectorsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDetectorsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(ListDetectorsError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListDetectorsError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListDetectorsError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListDetectorsError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListDetectorsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListDetectorsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDetectorsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListDetectorsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListDetectorsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListDetectorsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListDetectorsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDetectorsError {}
/// Trait representing the capabilities of the AWS IoT Events Data API. AWS IoT Events Data clients implement this trait.
#[async_trait]
pub trait IotEventsData {
    /// <p>Acknowledges one or more alarms. The alarms change to the <code>ACKNOWLEDGED</code> state after you acknowledge them.</p>
    async fn batch_acknowledge_alarm(
        &self,
        input: BatchAcknowledgeAlarmRequest,
    ) -> Result<BatchAcknowledgeAlarmResponse, RusotoError<BatchAcknowledgeAlarmError>>;

    /// <p>Disables one or more alarms. The alarms change to the <code>DISABLED</code> state after you disable them.</p>
    async fn batch_disable_alarm(
        &self,
        input: BatchDisableAlarmRequest,
    ) -> Result<BatchDisableAlarmResponse, RusotoError<BatchDisableAlarmError>>;

    /// <p>Enables one or more alarms. The alarms change to the <code>NORMAL</code> state after you enable them.</p>
    async fn batch_enable_alarm(
        &self,
        input: BatchEnableAlarmRequest,
    ) -> Result<BatchEnableAlarmResponse, RusotoError<BatchEnableAlarmError>>;

    /// <p>Sends a set of messages to the AWS IoT Events system. Each message payload is transformed into the input you specify (<code>"inputName"</code>) and ingested into any detectors that monitor that input. If multiple messages are sent, the order in which the messages are processed isn't guaranteed. To guarantee ordering, you must send messages one at a time and wait for a successful response.</p>
    async fn batch_put_message(
        &self,
        input: BatchPutMessageRequest,
    ) -> Result<BatchPutMessageResponse, RusotoError<BatchPutMessageError>>;

    /// <p>Resets one or more alarms. The alarms return to the <code>NORMAL</code> state after you reset them.</p>
    async fn batch_reset_alarm(
        &self,
        input: BatchResetAlarmRequest,
    ) -> Result<BatchResetAlarmResponse, RusotoError<BatchResetAlarmError>>;

    /// <p>Changes one or more alarms to the snooze mode. The alarms change to the <code>SNOOZE_DISABLED</code> state after you set them to the snooze mode.</p>
    async fn batch_snooze_alarm(
        &self,
        input: BatchSnoozeAlarmRequest,
    ) -> Result<BatchSnoozeAlarmResponse, RusotoError<BatchSnoozeAlarmError>>;

    /// <p>Updates the state, variable values, and timer settings of one or more detectors (instances) of a specified detector model.</p>
    async fn batch_update_detector(
        &self,
        input: BatchUpdateDetectorRequest,
    ) -> Result<BatchUpdateDetectorResponse, RusotoError<BatchUpdateDetectorError>>;

    /// <p>Retrieves information about an alarm.</p>
    async fn describe_alarm(
        &self,
        input: DescribeAlarmRequest,
    ) -> Result<DescribeAlarmResponse, RusotoError<DescribeAlarmError>>;

    /// <p>Returns information about the specified detector (instance).</p>
    async fn describe_detector(
        &self,
        input: DescribeDetectorRequest,
    ) -> Result<DescribeDetectorResponse, RusotoError<DescribeDetectorError>>;

    /// <p>Lists one or more alarms. The operation returns only the metadata associated with each alarm.</p>
    async fn list_alarms(
        &self,
        input: ListAlarmsRequest,
    ) -> Result<ListAlarmsResponse, RusotoError<ListAlarmsError>>;

    /// <p>Lists detectors (the instances of a detector model).</p>
    async fn list_detectors(
        &self,
        input: ListDetectorsRequest,
    ) -> Result<ListDetectorsResponse, RusotoError<ListDetectorsError>>;
}
/// A client for the AWS IoT Events Data API.
#[derive(Clone)]
pub struct IotEventsDataClient {
    client: Client,
    region: region::Region,
}

impl IotEventsDataClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> IotEventsDataClient {
        IotEventsDataClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> IotEventsDataClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        IotEventsDataClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> IotEventsDataClient {
        IotEventsDataClient { client, region }
    }
}

#[async_trait]
impl IotEventsData for IotEventsDataClient {
    /// <p>Acknowledges one or more alarms. The alarms change to the <code>ACKNOWLEDGED</code> state after you acknowledge them.</p>
    #[allow(unused_mut)]
    async fn batch_acknowledge_alarm(
        &self,
        input: BatchAcknowledgeAlarmRequest,
    ) -> Result<BatchAcknowledgeAlarmResponse, RusotoError<BatchAcknowledgeAlarmError>> {
        let request_uri = "/alarms/acknowledge";

        let mut request = SignedRequest::new("POST", "ioteventsdata", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("data.iotevents".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<BatchAcknowledgeAlarmResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(BatchAcknowledgeAlarmError::from_response(response))
        }
    }

    /// <p>Disables one or more alarms. The alarms change to the <code>DISABLED</code> state after you disable them.</p>
    #[allow(unused_mut)]
    async fn batch_disable_alarm(
        &self,
        input: BatchDisableAlarmRequest,
    ) -> Result<BatchDisableAlarmResponse, RusotoError<BatchDisableAlarmError>> {
        let request_uri = "/alarms/disable";

        let mut request = SignedRequest::new("POST", "ioteventsdata", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("data.iotevents".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<BatchDisableAlarmResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(BatchDisableAlarmError::from_response(response))
        }
    }

    /// <p>Enables one or more alarms. The alarms change to the <code>NORMAL</code> state after you enable them.</p>
    #[allow(unused_mut)]
    async fn batch_enable_alarm(
        &self,
        input: BatchEnableAlarmRequest,
    ) -> Result<BatchEnableAlarmResponse, RusotoError<BatchEnableAlarmError>> {
        let request_uri = "/alarms/enable";

        let mut request = SignedRequest::new("POST", "ioteventsdata", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("data.iotevents".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<BatchEnableAlarmResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(BatchEnableAlarmError::from_response(response))
        }
    }

    /// <p>Sends a set of messages to the AWS IoT Events system. Each message payload is transformed into the input you specify (<code>"inputName"</code>) and ingested into any detectors that monitor that input. If multiple messages are sent, the order in which the messages are processed isn't guaranteed. To guarantee ordering, you must send messages one at a time and wait for a successful response.</p>
    #[allow(unused_mut)]
    async fn batch_put_message(
        &self,
        input: BatchPutMessageRequest,
    ) -> Result<BatchPutMessageResponse, RusotoError<BatchPutMessageError>> {
        let request_uri = "/inputs/messages";

        let mut request = SignedRequest::new("POST", "ioteventsdata", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("data.iotevents".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<BatchPutMessageResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(BatchPutMessageError::from_response(response))
        }
    }

    /// <p>Resets one or more alarms. The alarms return to the <code>NORMAL</code> state after you reset them.</p>
    #[allow(unused_mut)]
    async fn batch_reset_alarm(
        &self,
        input: BatchResetAlarmRequest,
    ) -> Result<BatchResetAlarmResponse, RusotoError<BatchResetAlarmError>> {
        let request_uri = "/alarms/reset";

        let mut request = SignedRequest::new("POST", "ioteventsdata", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("data.iotevents".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<BatchResetAlarmResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(BatchResetAlarmError::from_response(response))
        }
    }

    /// <p>Changes one or more alarms to the snooze mode. The alarms change to the <code>SNOOZE_DISABLED</code> state after you set them to the snooze mode.</p>
    #[allow(unused_mut)]
    async fn batch_snooze_alarm(
        &self,
        input: BatchSnoozeAlarmRequest,
    ) -> Result<BatchSnoozeAlarmResponse, RusotoError<BatchSnoozeAlarmError>> {
        let request_uri = "/alarms/snooze";

        let mut request = SignedRequest::new("POST", "ioteventsdata", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("data.iotevents".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<BatchSnoozeAlarmResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(BatchSnoozeAlarmError::from_response(response))
        }
    }

    /// <p>Updates the state, variable values, and timer settings of one or more detectors (instances) of a specified detector model.</p>
    #[allow(unused_mut)]
    async fn batch_update_detector(
        &self,
        input: BatchUpdateDetectorRequest,
    ) -> Result<BatchUpdateDetectorResponse, RusotoError<BatchUpdateDetectorError>> {
        let request_uri = "/detectors";

        let mut request = SignedRequest::new("POST", "ioteventsdata", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("data.iotevents".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<BatchUpdateDetectorResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(BatchUpdateDetectorError::from_response(response))
        }
    }

    /// <p>Retrieves information about an alarm.</p>
    #[allow(unused_mut)]
    async fn describe_alarm(
        &self,
        input: DescribeAlarmRequest,
    ) -> Result<DescribeAlarmResponse, RusotoError<DescribeAlarmError>> {
        let request_uri = format!(
            "/alarms/{alarm_model_name}/keyValues/",
            alarm_model_name = input.alarm_model_name
        );

        let mut request = SignedRequest::new("GET", "ioteventsdata", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("data.iotevents".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.key_value {
            params.put("keyValue", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeAlarmResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeAlarmError::from_response(response))
        }
    }

    /// <p>Returns information about the specified detector (instance).</p>
    #[allow(unused_mut)]
    async fn describe_detector(
        &self,
        input: DescribeDetectorRequest,
    ) -> Result<DescribeDetectorResponse, RusotoError<DescribeDetectorError>> {
        let request_uri = format!(
            "/detectors/{detector_model_name}/keyValues/",
            detector_model_name = input.detector_model_name
        );

        let mut request = SignedRequest::new("GET", "ioteventsdata", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("data.iotevents".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.key_value {
            params.put("keyValue", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeDetectorResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeDetectorError::from_response(response))
        }
    }

    /// <p>Lists one or more alarms. The operation returns only the metadata associated with each alarm.</p>
    #[allow(unused_mut)]
    async fn list_alarms(
        &self,
        input: ListAlarmsRequest,
    ) -> Result<ListAlarmsResponse, RusotoError<ListAlarmsError>> {
        let request_uri = format!(
            "/alarms/{alarm_model_name}",
            alarm_model_name = input.alarm_model_name
        );

        let mut request = SignedRequest::new("GET", "ioteventsdata", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("data.iotevents".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListAlarmsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListAlarmsError::from_response(response))
        }
    }

    /// <p>Lists detectors (the instances of a detector model).</p>
    #[allow(unused_mut)]
    async fn list_detectors(
        &self,
        input: ListDetectorsRequest,
    ) -> Result<ListDetectorsResponse, RusotoError<ListDetectorsError>> {
        let request_uri = format!(
            "/detectors/{detector_model_name}",
            detector_model_name = input.detector_model_name
        );

        let mut request = SignedRequest::new("GET", "ioteventsdata", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("data.iotevents".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.state_name {
            params.put("stateName", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListDetectorsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListDetectorsError::from_response(response))
        }
    }
}
