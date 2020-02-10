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
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use futures::prelude::*;
use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
use std::pin::Pin;
/// <p>An action to be performed when the <code>"condition"</code> is TRUE.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Action {
    /// <p>Information needed to clear the timer.</p>
    #[serde(rename = "clearTimer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear_timer: Option<ClearTimerAction>,
    /// <p>Sends information about the detector model instance and the event that triggered the action to a Kinesis Data Firehose delivery stream.</p>
    #[serde(rename = "firehose")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firehose: Option<FirehoseAction>,
    /// <p>Sends an IoT Events input, passing in information about the detector model instance and the event that triggered the action.</p>
    #[serde(rename = "iotEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iot_events: Option<IotEventsAction>,
    /// <p>Publishes an MQTT message with the given topic to the AWS IoT message broker.</p>
    #[serde(rename = "iotTopicPublish")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iot_topic_publish: Option<IotTopicPublishAction>,
    /// <p>Calls a Lambda function, passing in information about the detector model instance and the event that triggered the action.</p>
    #[serde(rename = "lambda")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda: Option<LambdaAction>,
    /// <p>Information needed to reset the timer.</p>
    #[serde(rename = "resetTimer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_timer: Option<ResetTimerAction>,
    /// <p>Information needed to set the timer.</p>
    #[serde(rename = "setTimer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_timer: Option<SetTimerAction>,
    /// <p>Sets a variable to a specified value.</p>
    #[serde(rename = "setVariable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_variable: Option<SetVariableAction>,
    /// <p>Sends an Amazon SNS message.</p>
    #[serde(rename = "sns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns: Option<SNSTopicPublishAction>,
    /// <p>Sends information about the detector model instance and the event that triggered the action to an Amazon SQS queue.</p>
    #[serde(rename = "sqs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sqs: Option<SqsAction>,
}

/// <p>The attributes from the JSON payload that are made available by the input. Inputs are derived from messages sent to the AWS IoT Events system using <code>BatchPutMessage</code>. Each such message contains a JSON payload, and those attributes (and their paired values) specified here are available for use in the <code>condition</code> expressions used by detectors. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attribute {
    /// <p>An expression that specifies an attribute-value pair in a JSON structure. Use this to specify an attribute from the JSON payload that is made available by the input. Inputs are derived from messages sent to the AWS IoT Events system (<code>BatchPutMessage</code>). Each such message contains a JSON payload, and the attribute (and its paired value) specified here are available for use in the <code>"condition"</code> expressions used by detectors. </p> <p>Syntax: <code>&lt;field-name&gt;.&lt;field-name&gt;...</code> </p>
    #[serde(rename = "jsonPath")]
    pub json_path: String,
}

/// <p>Information needed to clear the timer.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearTimerAction {
    /// <p>The name of the timer to clear.</p>
    #[serde(rename = "timerName")]
    pub timer_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDetectorModelRequest {
    /// <p>Information that defines how the detectors operate.</p>
    #[serde(rename = "detectorModelDefinition")]
    pub detector_model_definition: DetectorModelDefinition,
    /// <p>A brief description of the detector model.</p>
    #[serde(rename = "detectorModelDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_model_description: Option<String>,
    /// <p>The name of the detector model.</p>
    #[serde(rename = "detectorModelName")]
    pub detector_model_name: String,
    /// <p>Information about the order in which events are evaluated and how actions are executed. </p>
    #[serde(rename = "evaluationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_method: Option<String>,
    /// <p>The input attribute key used to identify a device or system in order to create a detector (an instance of the detector model) and then to route each input received to the appropriate detector (instance). This parameter uses a JSON-path expression to specify the attribute-value pair in the message payload of each input that is used to identify the device associated with the input.</p>
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The ARN of the role that grants permission to AWS IoT Events to perform its operations.</p>
    #[serde(rename = "roleArn")]
    pub role_arn: String,
    /// <p>Metadata that can be used to manage the detector model.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDetectorModelResponse {
    /// <p>Information about how the detector model is configured.</p>
    #[serde(rename = "detectorModelConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_model_configuration: Option<DetectorModelConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateInputRequest {
    /// <p>The definition of the input.</p>
    #[serde(rename = "inputDefinition")]
    pub input_definition: InputDefinition,
    /// <p>A brief description of the input.</p>
    #[serde(rename = "inputDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_description: Option<String>,
    /// <p>The name you want to give to the input.</p>
    #[serde(rename = "inputName")]
    pub input_name: String,
    /// <p>Metadata that can be used to manage the input.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateInputResponse {
    /// <p>Information about the configuration of the input.</p>
    #[serde(rename = "inputConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_configuration: Option<InputConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDetectorModelRequest {
    /// <p>The name of the detector model to be deleted.</p>
    #[serde(rename = "detectorModelName")]
    pub detector_model_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteDetectorModelResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteInputRequest {
    /// <p>The name of the input to delete.</p>
    #[serde(rename = "inputName")]
    pub input_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteInputResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDetectorModelRequest {
    /// <p>The name of the detector model.</p>
    #[serde(rename = "detectorModelName")]
    pub detector_model_name: String,
    /// <p>The version of the detector model.</p>
    #[serde(rename = "detectorModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_model_version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeDetectorModelResponse {
    /// <p>Information about the detector model.</p>
    #[serde(rename = "detectorModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_model: Option<DetectorModel>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeInputRequest {
    /// <p>The name of the input.</p>
    #[serde(rename = "inputName")]
    pub input_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeInputResponse {
    /// <p>Information about the input.</p>
    #[serde(rename = "input")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<Input>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeLoggingOptionsRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeLoggingOptionsResponse {
    /// <p>The current settings of the AWS IoT Events logging options.</p>
    #[serde(rename = "loggingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_options: Option<LoggingOptions>,
}

/// <p>The detector model and the specific detectors (instances) for which the logging level is given.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetectorDebugOption {
    /// <p>The name of the detector model.</p>
    #[serde(rename = "detectorModelName")]
    pub detector_model_name: String,
    /// <p>The value of the input attribute key used to create the detector (the instance of the detector model).</p>
    #[serde(rename = "keyValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_value: Option<String>,
}

/// <p>Information about the detector model.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DetectorModel {
    /// <p>Information about how the detector is configured.</p>
    #[serde(rename = "detectorModelConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_model_configuration: Option<DetectorModelConfiguration>,
    /// <p>Information that defines how a detector operates.</p>
    #[serde(rename = "detectorModelDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_model_definition: Option<DetectorModelDefinition>,
}

/// <p>Information about how the detector model is configured.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DetectorModelConfiguration {
    /// <p>The time the detector model was created.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The ARN of the detector model.</p>
    #[serde(rename = "detectorModelArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_model_arn: Option<String>,
    /// <p>A brief description of the detector model.</p>
    #[serde(rename = "detectorModelDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_model_description: Option<String>,
    /// <p>The name of the detector model.</p>
    #[serde(rename = "detectorModelName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_model_name: Option<String>,
    /// <p>The version of the detector model.</p>
    #[serde(rename = "detectorModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_model_version: Option<String>,
    /// <p>Information about the order in which events are evaluated and how actions are executed. </p>
    #[serde(rename = "evaluationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_method: Option<String>,
    /// <p>The input attribute key used to identify a device or system in order to create a detector (an instance of the detector model) and then to route each input received to the appropriate detector (instance). This parameter uses a JSON-path expression to specify the attribute-value pair in the message payload of each input that is used to identify the device associated with the input.</p>
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The time the detector model was last updated.</p>
    #[serde(rename = "lastUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    /// <p>The ARN of the role that grants permission to AWS IoT Events to perform its operations.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The status of the detector model.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Information that defines how a detector operates.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetectorModelDefinition {
    /// <p>The state that is entered at the creation of each detector (instance).</p>
    #[serde(rename = "initialStateName")]
    pub initial_state_name: String,
    /// <p>Information about the states of the detector.</p>
    #[serde(rename = "states")]
    pub states: Vec<State>,
}

/// <p>Information about the detector model.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DetectorModelSummary {
    /// <p>The time the detector model was created.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>A brief description of the detector model.</p>
    #[serde(rename = "detectorModelDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_model_description: Option<String>,
    /// <p>The name of the detector model.</p>
    #[serde(rename = "detectorModelName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_model_name: Option<String>,
}

/// <p>Information about the detector model version.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DetectorModelVersionSummary {
    /// <p>The time the detector model version was created.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The ARN of the detector model version.</p>
    #[serde(rename = "detectorModelArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_model_arn: Option<String>,
    /// <p>The name of the detector model.</p>
    #[serde(rename = "detectorModelName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_model_name: Option<String>,
    /// <p>The ID of the detector model version.</p>
    #[serde(rename = "detectorModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_model_version: Option<String>,
    /// <p>Information about the order in which events are evaluated and how actions are executed. </p>
    #[serde(rename = "evaluationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_method: Option<String>,
    /// <p>The last time the detector model version was updated.</p>
    #[serde(rename = "lastUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    /// <p>The ARN of the role that grants the detector model permission to perform its tasks.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The status of the detector model version.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Specifies the <code>"actions"</code> to be performed when the <code>"condition"</code> evaluates to TRUE.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Event {
    /// <p>The actions to be performed.</p>
    #[serde(rename = "actions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<Action>>,
    /// <p>[Optional] The Boolean expression that when TRUE causes the <code>"actions"</code> to be performed. If not present, the actions are performed (=TRUE); if the expression result is not a Boolean value, the actions are NOT performed (=FALSE).</p>
    #[serde(rename = "condition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    /// <p>The name of the event.</p>
    #[serde(rename = "eventName")]
    pub event_name: String,
}

/// <p>Sends information about the detector model instance and the event that triggered the action to a Kinesis Data Firehose delivery stream.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FirehoseAction {
    /// <p>The name of the Kinesis Data Firehose delivery stream where the data is written.</p>
    #[serde(rename = "deliveryStreamName")]
    pub delivery_stream_name: String,
    /// <p>A character separator that is used to separate records written to the Kinesis Data Firehose delivery stream. Valid values are: '\n' (newline), '\t' (tab), '\r\n' (Windows newline), ',' (comma).</p>
    #[serde(rename = "separator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator: Option<String>,
}

/// <p>Information about the input.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Input {
    /// <p>Information about the configuration of an input.</p>
    #[serde(rename = "inputConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_configuration: Option<InputConfiguration>,
    /// <p>The definition of the input.</p>
    #[serde(rename = "inputDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_definition: Option<InputDefinition>,
}

/// <p>Information about the configuration of an input.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InputConfiguration {
    /// <p>The time the input was created.</p>
    #[serde(rename = "creationTime")]
    pub creation_time: f64,
    /// <p>The ARN of the input.</p>
    #[serde(rename = "inputArn")]
    pub input_arn: String,
    /// <p>A brief description of the input.</p>
    #[serde(rename = "inputDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_description: Option<String>,
    /// <p>The name of the input.</p>
    #[serde(rename = "inputName")]
    pub input_name: String,
    /// <p>The last time the input was updated.</p>
    #[serde(rename = "lastUpdateTime")]
    pub last_update_time: f64,
    /// <p>The status of the input.</p>
    #[serde(rename = "status")]
    pub status: String,
}

/// <p>The definition of the input.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputDefinition {
    /// <p>The attributes from the JSON payload that are made available by the input. Inputs are derived from messages sent to the AWS IoT Events system using <code>BatchPutMessage</code>. Each such message contains a JSON payload, and those attributes (and their paired values) specified here are available for use in the <code>"condition"</code> expressions used by detectors that monitor this input. </p>
    #[serde(rename = "attributes")]
    pub attributes: Vec<Attribute>,
}

/// <p>Information about the input.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InputSummary {
    /// <p>The time the input was created.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The ARN of the input.</p>
    #[serde(rename = "inputArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_arn: Option<String>,
    /// <p>A brief description of the input.</p>
    #[serde(rename = "inputDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_description: Option<String>,
    /// <p>The name of the input.</p>
    #[serde(rename = "inputName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_name: Option<String>,
    /// <p>The last time the input was updated.</p>
    #[serde(rename = "lastUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    /// <p>The status of the input.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Sends an AWS IoT Events input, passing in information about the detector model instance and the event that triggered the action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IotEventsAction {
    /// <p>The name of the AWS IoT Events input where the data is sent.</p>
    #[serde(rename = "inputName")]
    pub input_name: String,
}

/// <p>Information required to publish the MQTT message via the AWS IoT message broker.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IotTopicPublishAction {
    /// <p>The MQTT topic of the message.</p>
    #[serde(rename = "mqttTopic")]
    pub mqtt_topic: String,
}

/// <p>Calls a Lambda function, passing in information about the detector model instance and the event that triggered the action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LambdaAction {
    /// <p>The ARN of the Lambda function that is executed.</p>
    #[serde(rename = "functionArn")]
    pub function_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDetectorModelVersionsRequest {
    /// <p>The name of the detector model whose versions are returned.</p>
    #[serde(rename = "detectorModelName")]
    pub detector_model_name: String,
    /// <p>The maximum number of results to return at one time.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDetectorModelVersionsResponse {
    /// <p>Summary information about the detector model versions.</p>
    #[serde(rename = "detectorModelVersionSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_model_version_summaries: Option<Vec<DetectorModelVersionSummary>>,
    /// <p>A token to retrieve the next set of results, or <code>null</code> if there are no additional results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDetectorModelsRequest {
    /// <p>The maximum number of results to return at one time.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDetectorModelsResponse {
    /// <p>Summary information about the detector models.</p>
    #[serde(rename = "detectorModelSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_model_summaries: Option<Vec<DetectorModelSummary>>,
    /// <p>A token to retrieve the next set of results, or <code>null</code> if there are no additional results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListInputsRequest {
    /// <p>The maximum number of results to return at one time.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListInputsResponse {
    /// <p>Summary information about the inputs.</p>
    #[serde(rename = "inputSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_summaries: Option<Vec<InputSummary>>,
    /// <p>A token to retrieve the next set of results, or <code>null</code> if there are no additional results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The ARN of the resource.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>The list of tags assigned to the resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>The values of the AWS IoT Events logging options.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoggingOptions {
    /// <p>Information that identifies those detector models and their detectors (instances) for which the logging level is given.</p>
    #[serde(rename = "detectorDebugOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_debug_options: Option<Vec<DetectorDebugOption>>,
    /// <p>If TRUE, logging is enabled for AWS IoT Events.</p>
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// <p>The logging level.</p>
    #[serde(rename = "level")]
    pub level: String,
    /// <p>The ARN of the role that grants permission to AWS IoT Events to perform logging.</p>
    #[serde(rename = "roleArn")]
    pub role_arn: String,
}

/// <p>When entering this state, perform these <code>actions</code> if the <code>condition</code> is TRUE.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OnEnterLifecycle {
    /// <p>Specifies the actions that are performed when the state is entered and the <code>"condition"</code> is TRUE.</p>
    #[serde(rename = "events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<Event>>,
}

/// <p>When exiting this state, perform these <code>"actions"</code> if the specified <code>"condition"</code> is TRUE.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OnExitLifecycle {
    /// <p>Specifies the <code>"actions"</code> that are performed when the state is exited and the <code>"condition"</code> is TRUE.</p>
    #[serde(rename = "events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<Event>>,
}

/// <p>Specifies the actions performed when the <code>"condition"</code> evaluates to TRUE.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OnInputLifecycle {
    /// <p>Specifies the actions performed when the <code>"condition"</code> evaluates to TRUE.</p>
    #[serde(rename = "events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<Event>>,
    /// <p>Specifies the actions performed, and the next state entered, when a <code>"condition"</code> evaluates to TRUE.</p>
    #[serde(rename = "transitionEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition_events: Option<Vec<TransitionEvent>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutLoggingOptionsRequest {
    /// <p>The new values of the AWS IoT Events logging options.</p>
    #[serde(rename = "loggingOptions")]
    pub logging_options: LoggingOptions,
}

/// <p>Information needed to reset the timer.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResetTimerAction {
    /// <p>The name of the timer to reset.</p>
    #[serde(rename = "timerName")]
    pub timer_name: String,
}

/// <p>Information required to publish the Amazon SNS message.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SNSTopicPublishAction {
    /// <p>The ARN of the Amazon SNS target where the message is sent.</p>
    #[serde(rename = "targetArn")]
    pub target_arn: String,
}

/// <p>Information needed to set the timer.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetTimerAction {
    /// <p>The number of seconds until the timer expires. The minimum value is 60 seconds to ensure accuracy.</p>
    #[serde(rename = "seconds")]
    pub seconds: i64,
    /// <p>The name of the timer.</p>
    #[serde(rename = "timerName")]
    pub timer_name: String,
}

/// <p>Information about the variable and its new value.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetVariableAction {
    /// <p>The new value of the variable.</p>
    #[serde(rename = "value")]
    pub value: String,
    /// <p>The name of the variable.</p>
    #[serde(rename = "variableName")]
    pub variable_name: String,
}

/// <p>Sends information about the detector model instance and the event that triggered the action to an Amazon SQS queue.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SqsAction {
    /// <p>The URL of the SQS queue where the data is written.</p>
    #[serde(rename = "queueUrl")]
    pub queue_url: String,
    /// <p>Set this to TRUE if you want the data to be Base-64 encoded before it is written to the queue. Otherwise, set this to FALSE.</p>
    #[serde(rename = "useBase64")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_base_64: Option<bool>,
}

/// <p>Information that defines a state of a detector.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct State {
    /// <p>When entering this state, perform these <code>"actions"</code> if the <code>"condition"</code> is TRUE.</p>
    #[serde(rename = "onEnter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_enter: Option<OnEnterLifecycle>,
    /// <p>When exiting this state, perform these <code>"actions"</code> if the specified <code>"condition"</code> is TRUE.</p>
    #[serde(rename = "onExit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_exit: Option<OnExitLifecycle>,
    /// <p>When an input is received and the <code>"condition"</code> is TRUE, perform the specified <code>"actions"</code>.</p>
    #[serde(rename = "onInput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_input: Option<OnInputLifecycle>,
    /// <p>The name of the state.</p>
    #[serde(rename = "stateName")]
    pub state_name: String,
}

/// <p>Metadata that can be used to manage the resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>The tag's key.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>The tag's value.</p>
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The ARN of the resource.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The new or modified tags for the resource.</p>
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

/// <p>Specifies the actions performed and the next state entered when a <code>"condition"</code> evaluates to TRUE.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransitionEvent {
    /// <p>The actions to be performed.</p>
    #[serde(rename = "actions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<Action>>,
    /// <p>[Required] A Boolean expression that when TRUE causes the actions to be performed and the <code>"nextState"</code> to be entered.</p>
    #[serde(rename = "condition")]
    pub condition: String,
    /// <p>The name of the transition event.</p>
    #[serde(rename = "eventName")]
    pub event_name: String,
    /// <p>The next state to enter.</p>
    #[serde(rename = "nextState")]
    pub next_state: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The ARN of the resource.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>A list of the keys of the tags to be removed from the resource.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateDetectorModelRequest {
    /// <p>Information that defines how a detector operates.</p>
    #[serde(rename = "detectorModelDefinition")]
    pub detector_model_definition: DetectorModelDefinition,
    /// <p>A brief description of the detector model.</p>
    #[serde(rename = "detectorModelDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_model_description: Option<String>,
    /// <p>The name of the detector model that is updated.</p>
    #[serde(rename = "detectorModelName")]
    pub detector_model_name: String,
    /// <p>Information about the order in which events are evaluated and how actions are executed. </p>
    #[serde(rename = "evaluationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_method: Option<String>,
    /// <p>The ARN of the role that grants permission to AWS IoT Events to perform its operations.</p>
    #[serde(rename = "roleArn")]
    pub role_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDetectorModelResponse {
    /// <p>Information about how the detector model is configured.</p>
    #[serde(rename = "detectorModelConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_model_configuration: Option<DetectorModelConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateInputRequest {
    /// <p>The definition of the input.</p>
    #[serde(rename = "inputDefinition")]
    pub input_definition: InputDefinition,
    /// <p>A brief description of the input.</p>
    #[serde(rename = "inputDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_description: Option<String>,
    /// <p>The name of the input you want to update.</p>
    #[serde(rename = "inputName")]
    pub input_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateInputResponse {
    /// <p>Information about the configuration of the input.</p>
    #[serde(rename = "inputConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_configuration: Option<InputConfiguration>,
}

/// Errors returned by CreateDetectorModel
#[derive(Debug, PartialEq)]
pub enum CreateDetectorModelError {
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>The request was invalid.</p>
    InvalidRequest(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The resource already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>The resource is in use.</p>
    ResourceInUse(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request could not be completed due to throttling.</p>
    Throttling(String),
}

impl CreateDetectorModelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDetectorModelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(CreateDetectorModelError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateDetectorModelError::InvalidRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateDetectorModelError::LimitExceeded(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateDetectorModelError::ResourceAlreadyExists(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(CreateDetectorModelError::ResourceInUse(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateDetectorModelError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateDetectorModelError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateDetectorModelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDetectorModelError::InternalFailure(ref cause) => write!(f, "{}", cause),
            CreateDetectorModelError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateDetectorModelError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateDetectorModelError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateDetectorModelError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateDetectorModelError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateDetectorModelError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDetectorModelError {}
/// Errors returned by CreateInput
#[derive(Debug, PartialEq)]
pub enum CreateInputError {
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>The request was invalid.</p>
    InvalidRequest(String),
    /// <p>The resource already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request could not be completed due to throttling.</p>
    Throttling(String),
}

impl CreateInputError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateInputError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(CreateInputError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateInputError::InvalidRequest(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateInputError::ResourceAlreadyExists(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateInputError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateInputError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateInputError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateInputError::InternalFailure(ref cause) => write!(f, "{}", cause),
            CreateInputError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateInputError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateInputError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateInputError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateInputError {}
/// Errors returned by DeleteDetectorModel
#[derive(Debug, PartialEq)]
pub enum DeleteDetectorModelError {
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>The request was invalid.</p>
    InvalidRequest(String),
    /// <p>The resource is in use.</p>
    ResourceInUse(String),
    /// <p>The resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request could not be completed due to throttling.</p>
    Throttling(String),
}

impl DeleteDetectorModelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDetectorModelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteDetectorModelError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteDetectorModelError::InvalidRequest(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteDetectorModelError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteDetectorModelError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteDetectorModelError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteDetectorModelError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteDetectorModelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDetectorModelError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteDetectorModelError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteDetectorModelError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteDetectorModelError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteDetectorModelError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteDetectorModelError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDetectorModelError {}
/// Errors returned by DeleteInput
#[derive(Debug, PartialEq)]
pub enum DeleteInputError {
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>The request was invalid.</p>
    InvalidRequest(String),
    /// <p>The resource is in use.</p>
    ResourceInUse(String),
    /// <p>The resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request could not be completed due to throttling.</p>
    Throttling(String),
}

impl DeleteInputError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteInputError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteInputError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteInputError::InvalidRequest(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteInputError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteInputError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteInputError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteInputError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteInputError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteInputError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteInputError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteInputError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteInputError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteInputError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteInputError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteInputError {}
/// Errors returned by DescribeDetectorModel
#[derive(Debug, PartialEq)]
pub enum DescribeDetectorModelError {
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

impl DescribeDetectorModelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeDetectorModelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(DescribeDetectorModelError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeDetectorModelError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeDetectorModelError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeDetectorModelError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeDetectorModelError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeDetectorModelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDetectorModelError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DescribeDetectorModelError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeDetectorModelError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeDetectorModelError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DescribeDetectorModelError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeDetectorModelError {}
/// Errors returned by DescribeInput
#[derive(Debug, PartialEq)]
pub enum DescribeInputError {
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

impl DescribeInputError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeInputError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(DescribeInputError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeInputError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeInputError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeInputError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeInputError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeInputError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeInputError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DescribeInputError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeInputError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeInputError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DescribeInputError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeInputError {}
/// Errors returned by DescribeLoggingOptions
#[derive(Debug, PartialEq)]
pub enum DescribeLoggingOptionsError {
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
    /// <p>The requested operation is not supported.</p>
    UnsupportedOperation(String),
}

impl DescribeLoggingOptionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeLoggingOptionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(DescribeLoggingOptionsError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeLoggingOptionsError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeLoggingOptionsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeLoggingOptionsError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeLoggingOptionsError::Throttling(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(DescribeLoggingOptionsError::UnsupportedOperation(
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
impl fmt::Display for DescribeLoggingOptionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeLoggingOptionsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DescribeLoggingOptionsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeLoggingOptionsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeLoggingOptionsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DescribeLoggingOptionsError::Throttling(ref cause) => write!(f, "{}", cause),
            DescribeLoggingOptionsError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeLoggingOptionsError {}
/// Errors returned by ListDetectorModelVersions
#[derive(Debug, PartialEq)]
pub enum ListDetectorModelVersionsError {
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

impl ListDetectorModelVersionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDetectorModelVersionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(ListDetectorModelVersionsError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListDetectorModelVersionsError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListDetectorModelVersionsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        ListDetectorModelVersionsError::ServiceUnavailable(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListDetectorModelVersionsError::Throttling(
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
impl fmt::Display for ListDetectorModelVersionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDetectorModelVersionsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListDetectorModelVersionsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListDetectorModelVersionsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListDetectorModelVersionsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListDetectorModelVersionsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDetectorModelVersionsError {}
/// Errors returned by ListDetectorModels
#[derive(Debug, PartialEq)]
pub enum ListDetectorModelsError {
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>The request was invalid.</p>
    InvalidRequest(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request could not be completed due to throttling.</p>
    Throttling(String),
}

impl ListDetectorModelsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDetectorModelsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(ListDetectorModelsError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListDetectorModelsError::InvalidRequest(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListDetectorModelsError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListDetectorModelsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListDetectorModelsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDetectorModelsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListDetectorModelsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListDetectorModelsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListDetectorModelsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDetectorModelsError {}
/// Errors returned by ListInputs
#[derive(Debug, PartialEq)]
pub enum ListInputsError {
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>The request was invalid.</p>
    InvalidRequest(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request could not be completed due to throttling.</p>
    Throttling(String),
}

impl ListInputsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListInputsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(ListInputsError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListInputsError::InvalidRequest(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListInputsError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListInputsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListInputsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListInputsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListInputsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListInputsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListInputsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListInputsError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>The request was invalid.</p>
    InvalidRequest(String),
    /// <p>The resource is in use.</p>
    ResourceInUse(String),
    /// <p>The resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The request could not be completed due to throttling.</p>
    Throttling(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(ListTagsForResourceError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidRequest(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(ListTagsForResourceError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListTagsForResourceError::Throttling(err.msg))
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
            ListTagsForResourceError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by PutLoggingOptions
#[derive(Debug, PartialEq)]
pub enum PutLoggingOptionsError {
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>The request was invalid.</p>
    InvalidRequest(String),
    /// <p>The resource is in use.</p>
    ResourceInUse(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request could not be completed due to throttling.</p>
    Throttling(String),
    /// <p>The requested operation is not supported.</p>
    UnsupportedOperation(String),
}

impl PutLoggingOptionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutLoggingOptionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(PutLoggingOptionsError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(PutLoggingOptionsError::InvalidRequest(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(PutLoggingOptionsError::ResourceInUse(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(PutLoggingOptionsError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(PutLoggingOptionsError::Throttling(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(PutLoggingOptionsError::UnsupportedOperation(
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
impl fmt::Display for PutLoggingOptionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutLoggingOptionsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            PutLoggingOptionsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            PutLoggingOptionsError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            PutLoggingOptionsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            PutLoggingOptionsError::Throttling(ref cause) => write!(f, "{}", cause),
            PutLoggingOptionsError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutLoggingOptionsError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>The request was invalid.</p>
    InvalidRequest(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The resource is in use.</p>
    ResourceInUse(String),
    /// <p>The resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The request could not be completed due to throttling.</p>
    Throttling(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(TagResourceError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(TagResourceError::InvalidRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(TagResourceError::LimitExceeded(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(TagResourceError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(TagResourceError::Throttling(err.msg))
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
            TagResourceError::InternalFailure(ref cause) => write!(f, "{}", cause),
            TagResourceError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            TagResourceError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>The request was invalid.</p>
    InvalidRequest(String),
    /// <p>The resource is in use.</p>
    ResourceInUse(String),
    /// <p>The resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The request could not be completed due to throttling.</p>
    Throttling(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(UntagResourceError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UntagResourceError::InvalidRequest(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(UntagResourceError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UntagResourceError::Throttling(err.msg))
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
            UntagResourceError::InternalFailure(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UntagResourceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateDetectorModel
#[derive(Debug, PartialEq)]
pub enum UpdateDetectorModelError {
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>The request was invalid.</p>
    InvalidRequest(String),
    /// <p>The resource is in use.</p>
    ResourceInUse(String),
    /// <p>The resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request could not be completed due to throttling.</p>
    Throttling(String),
}

impl UpdateDetectorModelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDetectorModelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateDetectorModelError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateDetectorModelError::InvalidRequest(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(UpdateDetectorModelError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateDetectorModelError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateDetectorModelError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateDetectorModelError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateDetectorModelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateDetectorModelError::InternalFailure(ref cause) => write!(f, "{}", cause),
            UpdateDetectorModelError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateDetectorModelError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            UpdateDetectorModelError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateDetectorModelError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateDetectorModelError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateDetectorModelError {}
/// Errors returned by UpdateInput
#[derive(Debug, PartialEq)]
pub enum UpdateInputError {
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>The request was invalid.</p>
    InvalidRequest(String),
    /// <p>The resource is in use.</p>
    ResourceInUse(String),
    /// <p>The resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request could not be completed due to throttling.</p>
    Throttling(String),
}

impl UpdateInputError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateInputError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateInputError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateInputError::InvalidRequest(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(UpdateInputError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateInputError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateInputError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateInputError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateInputError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateInputError::InternalFailure(ref cause) => write!(f, "{}", cause),
            UpdateInputError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateInputError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            UpdateInputError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateInputError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateInputError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateInputError {}
/// Trait representing the capabilities of the AWS IoT Events API. AWS IoT Events clients implement this trait.
pub trait IotEvents {
    /// <p>Creates a detector model.</p>
    fn create_detector_model(
        &self,
        input: CreateDetectorModelRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        CreateDetectorModelResponse,
                        RusotoError<CreateDetectorModelError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Creates an input.</p>
    fn create_input(
        &self,
        input: CreateInputRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<CreateInputResponse, RusotoError<CreateInputError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Deletes a detector model. Any active instances of the detector model are also deleted.</p>
    fn delete_detector_model(
        &self,
        input: DeleteDetectorModelRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DeleteDetectorModelResponse,
                        RusotoError<DeleteDetectorModelError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Deletes an input.</p>
    fn delete_input(
        &self,
        input: DeleteInputRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<DeleteInputResponse, RusotoError<DeleteInputError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Describes a detector model. If the <code>"version"</code> parameter is not specified, information about the latest version is returned.</p>
    fn describe_detector_model(
        &self,
        input: DescribeDetectorModelRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeDetectorModelResponse,
                        RusotoError<DescribeDetectorModelError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Describes an input.</p>
    fn describe_input(
        &self,
        input: DescribeInputRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<DescribeInputResponse, RusotoError<DescribeInputError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Retrieves the current settings of the AWS IoT Events logging options.</p>
    fn describe_logging_options(
        &self,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeLoggingOptionsResponse,
                        RusotoError<DescribeLoggingOptionsError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Lists all the versions of a detector model. Only the metadata associated with each detector model version is returned.</p>
    fn list_detector_model_versions(
        &self,
        input: ListDetectorModelVersionsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListDetectorModelVersionsResponse,
                        RusotoError<ListDetectorModelVersionsError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Lists the detector models you have created. Only the metadata associated with each detector model is returned.</p>
    fn list_detector_models(
        &self,
        input: ListDetectorModelsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListDetectorModelsResponse,
                        RusotoError<ListDetectorModelsError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Lists the inputs you have created.</p>
    fn list_inputs(
        &self,
        input: ListInputsRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<ListInputsResponse, RusotoError<ListInputsError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Lists the tags (metadata) you have assigned to the resource.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListTagsForResourceResponse,
                        RusotoError<ListTagsForResourceError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Sets or updates the AWS IoT Events logging options.</p> <p>If you update the value of any <code>"loggingOptions"</code> field, it takes up to one minute for the change to take effect. Also, if you change the policy attached to the role you specified in the <code>"roleArn"</code> field (for example, to correct an invalid policy) it takes up to five minutes for that change to take effect.</p>
    fn put_logging_options(
        &self,
        input: PutLoggingOptionsRequest,
    ) -> Pin<
        Box<dyn Future<Output = Result<(), RusotoError<PutLoggingOptionsError>>> + Send + 'static>,
    >;

    /// <p>Adds to or modifies the tags of the given resource. Tags are metadata that can be used to manage a resource.</p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<TagResourceResponse, RusotoError<TagResourceError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Removes the given tags (metadata) from the resource.</p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<UntagResourceResponse, RusotoError<UntagResourceError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Updates a detector model. Detectors (instances) spawned by the previous version are deleted and then re-created as new inputs arrive.</p>
    fn update_detector_model(
        &self,
        input: UpdateDetectorModelRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        UpdateDetectorModelResponse,
                        RusotoError<UpdateDetectorModelError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Updates an input.</p>
    fn update_input(
        &self,
        input: UpdateInputRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<UpdateInputResponse, RusotoError<UpdateInputError>>>
                + Send
                + 'static,
        >,
    >;
}
/// A client for the AWS IoT Events API.
#[derive(Clone)]
pub struct IotEventsClient {
    client: Client,
    region: region::Region,
}

impl IotEventsClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> IotEventsClient {
        IotEventsClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> IotEventsClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        IotEventsClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> IotEventsClient {
        IotEventsClient { client, region }
    }
}

impl IotEvents for IotEventsClient {
    /// <p>Creates a detector model.</p>
    fn create_detector_model(
        &self,
        input: CreateDetectorModelRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        CreateDetectorModelResponse,
                        RusotoError<CreateDetectorModelError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = "/detector-models";

        let mut request = SignedRequest::new("POST", "iotevents", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<CreateDetectorModelResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(CreateDetectorModelError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Creates an input.</p>
    fn create_input(
        &self,
        input: CreateInputRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<CreateInputResponse, RusotoError<CreateInputError>>>
                + Send
                + 'static,
        >,
    > {
        let request_uri = "/inputs";

        let mut request = SignedRequest::new("POST", "iotevents", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 201 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<CreateInputResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(CreateInputError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Deletes a detector model. Any active instances of the detector model are also deleted.</p>
    fn delete_detector_model(
        &self,
        input: DeleteDetectorModelRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DeleteDetectorModelResponse,
                        RusotoError<DeleteDetectorModelError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = format!(
            "/detector-models/{detector_model_name}",
            detector_model_name = input.detector_model_name
        );

        let mut request = SignedRequest::new("DELETE", "iotevents", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.as_u16() == 204 {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<DeleteDetectorModelResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(DeleteDetectorModelError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Deletes an input.</p>
    fn delete_input(
        &self,
        input: DeleteInputRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<DeleteInputResponse, RusotoError<DeleteInputError>>>
                + Send
                + 'static,
        >,
    > {
        let request_uri = format!("/inputs/{input_name}", input_name = input.input_name);

        let mut request = SignedRequest::new("DELETE", "iotevents", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<DeleteInputResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(DeleteInputError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Describes a detector model. If the <code>"version"</code> parameter is not specified, information about the latest version is returned.</p>
    fn describe_detector_model(
        &self,
        input: DescribeDetectorModelRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeDetectorModelResponse,
                        RusotoError<DescribeDetectorModelError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = format!(
            "/detector-models/{detector_model_name}",
            detector_model_name = input.detector_model_name
        );

        let mut request = SignedRequest::new("GET", "iotevents", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.detector_model_version {
            params.put("version", x);
        }
        request.set_params(params);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<DescribeDetectorModelResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(DescribeDetectorModelError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Describes an input.</p>
    fn describe_input(
        &self,
        input: DescribeInputRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<DescribeInputResponse, RusotoError<DescribeInputError>>>
                + Send
                + 'static,
        >,
    > {
        let request_uri = format!("/inputs/{input_name}", input_name = input.input_name);

        let mut request = SignedRequest::new("GET", "iotevents", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<DescribeInputResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(DescribeInputError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Retrieves the current settings of the AWS IoT Events logging options.</p>
    fn describe_logging_options(
        &self,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeLoggingOptionsResponse,
                        RusotoError<DescribeLoggingOptionsError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = "/logging";

        let mut request = SignedRequest::new("GET", "iotevents", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<DescribeLoggingOptionsResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(DescribeLoggingOptionsError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Lists all the versions of a detector model. Only the metadata associated with each detector model version is returned.</p>
    fn list_detector_model_versions(
        &self,
        input: ListDetectorModelVersionsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListDetectorModelVersionsResponse,
                        RusotoError<ListDetectorModelVersionsError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = format!(
            "/detector-models/{detector_model_name}/versions",
            detector_model_name = input.detector_model_name
        );

        let mut request = SignedRequest::new("GET", "iotevents", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<ListDetectorModelVersionsResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(ListDetectorModelVersionsError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Lists the detector models you have created. Only the metadata associated with each detector model is returned.</p>
    fn list_detector_models(
        &self,
        input: ListDetectorModelsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListDetectorModelsResponse,
                        RusotoError<ListDetectorModelsError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = "/detector-models";

        let mut request = SignedRequest::new("GET", "iotevents", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<ListDetectorModelsResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(ListDetectorModelsError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Lists the inputs you have created.</p>
    fn list_inputs(
        &self,
        input: ListInputsRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<ListInputsResponse, RusotoError<ListInputsError>>>
                + Send
                + 'static,
        >,
    > {
        let request_uri = "/inputs";

        let mut request = SignedRequest::new("GET", "iotevents", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<ListInputsResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(ListInputsError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Lists the tags (metadata) you have assigned to the resource.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListTagsForResourceResponse,
                        RusotoError<ListTagsForResourceError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = "/tags";

        let mut request = SignedRequest::new("GET", "iotevents", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("resourceArn", &input.resource_arn);
        request.set_params(params);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<ListTagsForResourceResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(ListTagsForResourceError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Sets or updates the AWS IoT Events logging options.</p> <p>If you update the value of any <code>"loggingOptions"</code> field, it takes up to one minute for the change to take effect. Also, if you change the policy attached to the role you specified in the <code>"roleArn"</code> field (for example, to correct an invalid policy) it takes up to five minutes for that change to take effect.</p>
    fn put_logging_options(
        &self,
        input: PutLoggingOptionsRequest,
    ) -> Pin<
        Box<dyn Future<Output = Result<(), RusotoError<PutLoggingOptionsError>>> + Send + 'static>,
    > {
        let request_uri = "/logging";

        let mut request = SignedRequest::new("PUT", "iotevents", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = ::std::mem::drop(response);

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(PutLoggingOptionsError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Adds to or modifies the tags of the given resource. Tags are metadata that can be used to manage a resource.</p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<TagResourceResponse, RusotoError<TagResourceError>>>
                + Send
                + 'static,
        >,
    > {
        let request_uri = "/tags";

        let mut request = SignedRequest::new("POST", "iotevents", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        params.put("resourceArn", &input.resource_arn);
        request.set_params(params);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<TagResourceResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(TagResourceError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Removes the given tags (metadata) from the resource.</p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<UntagResourceResponse, RusotoError<UntagResourceError>>>
                + Send
                + 'static,
        >,
    > {
        let request_uri = "/tags";

        let mut request = SignedRequest::new("DELETE", "iotevents", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("resourceArn", &input.resource_arn);
        for item in input.tag_keys.iter() {
            params.put("tagKeys", item);
        }
        request.set_params(params);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<UntagResourceResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(UntagResourceError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Updates a detector model. Detectors (instances) spawned by the previous version are deleted and then re-created as new inputs arrive.</p>
    fn update_detector_model(
        &self,
        input: UpdateDetectorModelRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        UpdateDetectorModelResponse,
                        RusotoError<UpdateDetectorModelError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = format!(
            "/detector-models/{detector_model_name}",
            detector_model_name = input.detector_model_name
        );

        let mut request = SignedRequest::new("POST", "iotevents", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<UpdateDetectorModelResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(UpdateDetectorModelError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Updates an input.</p>
    fn update_input(
        &self,
        input: UpdateInputRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<UpdateInputResponse, RusotoError<UpdateInputError>>>
                + Send
                + 'static,
        >,
    > {
        let request_uri = format!("/inputs/{input_name}", input_name = input.input_name);

        let mut request = SignedRequest::new("PUT", "iotevents", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<UpdateInputResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(UpdateInputError::from_response(response))
            }
        }
        .boxed()
    }
}
