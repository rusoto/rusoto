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
/// <p>Specifies whether to get notified for alarm state changes.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AcknowledgeFlow {
    /// <p>The value must be <code>TRUE</code> or <code>FALSE</code>. If <code>TRUE</code>, you receive a notification when the alarm state changes. You must choose to acknowledge the notification before the alarm state can return to <code>NORMAL</code>. If <code>FALSE</code>, you won't receive notifications. The alarm automatically changes to the <code>NORMAL</code> state when the input property value returns to the specified range.</p>
    #[serde(rename = "enabled")]
    pub enabled: bool,
}

/// <p>An action to be performed when the <code>condition</code> is TRUE.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Action {
    /// <p>Information needed to clear the timer.</p>
    #[serde(rename = "clearTimer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear_timer: Option<ClearTimerAction>,
    /// <p>Writes to the DynamoDB table that you created. The default action payload contains all attribute-value pairs that have the information about the detector model instance and the event that triggered the action. You can customize the <a href="https://docs.aws.amazon.com/iotevents/latest/apireference/API_Payload.html">payload</a>. One column of the DynamoDB table receives all attribute-value pairs in the payload that you specify. For more information, see <a href="https://docs.aws.amazon.com/iotevents/latest/developerguide/iotevents-event-actions.html">Actions</a> in <i>AWS IoT Events Developer Guide</i>.</p>
    #[serde(rename = "dynamoDB")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamo_db: Option<DynamoDBAction>,
    /// <p>Writes to the DynamoDB table that you created. The default action payload contains all attribute-value pairs that have the information about the detector model instance and the event that triggered the action. You can customize the <a href="https://docs.aws.amazon.com/iotevents/latest/apireference/API_Payload.html">payload</a>. A separate column of the DynamoDB table receives one attribute-value pair in the payload that you specify. For more information, see <a href="https://docs.aws.amazon.com/iotevents/latest/developerguide/iotevents-event-actions.html">Actions</a> in <i>AWS IoT Events Developer Guide</i>.</p>
    #[serde(rename = "dynamoDBv2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamo_d_bv_2: Option<DynamoDBv2Action>,
    /// <p>Sends information about the detector model instance and the event that triggered the action to an Amazon Kinesis Data Firehose delivery stream.</p>
    #[serde(rename = "firehose")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firehose: Option<FirehoseAction>,
    /// <p>Sends AWS IoT Events input, which passes information about the detector model instance and the event that triggered the action.</p>
    #[serde(rename = "iotEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iot_events: Option<IotEventsAction>,
    /// <p>Sends information about the detector model instance and the event that triggered the action to an asset property in AWS IoT SiteWise .</p>
    #[serde(rename = "iotSiteWise")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iot_site_wise: Option<IotSiteWiseAction>,
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

/// <p>Specifies one of the following actions to receive notifications when the alarm state changes.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AlarmAction {
    #[serde(rename = "dynamoDB")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamo_db: Option<DynamoDBAction>,
    #[serde(rename = "dynamoDBv2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamo_d_bv_2: Option<DynamoDBv2Action>,
    #[serde(rename = "firehose")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firehose: Option<FirehoseAction>,
    #[serde(rename = "iotEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iot_events: Option<IotEventsAction>,
    #[serde(rename = "iotSiteWise")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iot_site_wise: Option<IotSiteWiseAction>,
    #[serde(rename = "iotTopicPublish")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iot_topic_publish: Option<IotTopicPublishAction>,
    #[serde(rename = "lambda")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda: Option<LambdaAction>,
    #[serde(rename = "sns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns: Option<SNSTopicPublishAction>,
    #[serde(rename = "sqs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sqs: Option<SqsAction>,
}

/// <p>Contains the configuration information of alarm state changes.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AlarmCapabilities {
    /// <p>Specifies whether to get notified for alarm state changes.</p>
    #[serde(rename = "acknowledgeFlow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acknowledge_flow: Option<AcknowledgeFlow>,
    /// <p>Specifies the default alarm state. The configuration applies to all alarms that were created based on this alarm model.</p>
    #[serde(rename = "initializationConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initialization_configuration: Option<InitializationConfiguration>,
}

/// <p>Contains information about one or more alarm actions.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AlarmEventActions {
    /// <p>Specifies one or more supported actions to receive notifications when the alarm state changes.</p>
    #[serde(rename = "alarmActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_actions: Option<Vec<AlarmAction>>,
}

/// <p>Contains a summary of an alarm model.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AlarmModelSummary {
    /// <p>The description of the alarm model.</p>
    #[serde(rename = "alarmModelDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_model_description: Option<String>,
    /// <p>The name of the alarm model.</p>
    #[serde(rename = "alarmModelName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_model_name: Option<String>,
    /// <p>The time the alarm model was created, in the Unix epoch format.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
}

/// <p>Contains a summary of an alarm model version.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AlarmModelVersionSummary {
    /// <p>The ARN of the alarm model. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>AWS General Reference</i>.</p>
    #[serde(rename = "alarmModelArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_model_arn: Option<String>,
    /// <p>The name of the alarm model.</p>
    #[serde(rename = "alarmModelName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_model_name: Option<String>,
    /// <p>The version of the alarm model.</p>
    #[serde(rename = "alarmModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_model_version: Option<String>,
    /// <p>The time the alarm model was created, in the Unix epoch format.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The time the alarm model was last updated, in the Unix epoch format.</p>
    #[serde(rename = "lastUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    /// <p>The ARN of the IAM role that allows the alarm to perform actions and access AWS resources. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>AWS General Reference</i>.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p><p>The status of the alarm model. The status can be one of the following values:</p> <ul> <li> <p> <code>ACTIVE</code> - The alarm model is active and it&#39;s ready to evaluate data.</p> </li> <li> <p> <code>ACTIVATING</code> - AWS IoT Events is activating your alarm model. Activating an alarm model can take up to a few minutes.</p> </li> <li> <p> <code>INACTIVE</code> - The alarm model is inactive, so it isn&#39;t ready to evaluate data. Check your alarm model information and update the alarm model.</p> </li> <li> <p> <code>FAILED</code> - You couldn&#39;t create or update the alarm model. Check your alarm model information and try again.</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p> Contains information about the status of the alarm model version. </p>
    #[serde(rename = "statusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

/// <p>Contains information about one or more notification actions.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AlarmNotification {
    /// <p>Contains the notification settings of an alarm model. The settings apply to all alarms that were created based on this alarm model.</p>
    #[serde(rename = "notificationActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_actions: Option<Vec<NotificationAction>>,
}

/// <p>Defines when your alarm is invoked.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AlarmRule {
    /// <p>A rule that compares an input property value to a threshold value with a comparison operator.</p>
    #[serde(rename = "simpleRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_rule: Option<SimpleRule>,
}

/// <p>Contains the result of the analysis.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AnalysisResult {
    /// <p><p>The severity level of the analysis result. Based on the severity level, analysis results fall into three general categories:</p> <ul> <li> <p> <code>INFO</code> - An information result tells you about a significant field in your detector model. This type of result usually doesn&#39;t require immediate action.</p> </li> <li> <p> <code>WARNING</code> - A warning result draws special attention to fields that might cause issues for your detector model. We recommend that you review warnings and take necessary actions before you use your detector model in production environments. Otherwise, the detector model might not work as expected.</p> </li> <li> <p> <code>ERROR</code> - An error result notifies you about a problem found in your detector model. You must fix all errors before you can publish your detector model.</p> </li> </ul></p>
    #[serde(rename = "level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    /// <p>Contains one or more locations that you can use to locate the fields in your detector model that the analysis result references.</p>
    #[serde(rename = "locations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<AnalysisResultLocation>>,
    /// <p>Contains additional information about the analysis result.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The type of the analysis result. Analyses fall into the following types based on the validators used to generate the analysis result:</p> <ul> <li> <p> <code>supported-actions</code> - You must specify AWS IoT Events supported actions that work with other AWS services in a supported AWS Region.</p> </li> <li> <p> <code>service-limits</code> - Resources or API operations can't exceed service quotas (also known as limits). Update your detector model or request a quota increase.</p> </li> <li> <p> <code>structure</code> - The detector model must follow a structure that AWS IoT Events supports. </p> </li> <li> <p> <code>expression-syntax</code> - Your expression must follow the required syntax.</p> </li> <li> <p> <code>data-type</code> - Data types referenced in the detector model must be compatible.</p> </li> <li> <p> <code>referenced-data</code> - You must define the data referenced in your detector model before you can use the data.</p> </li> <li> <p> <code>referenced-resource</code> - Resources that the detector model uses must be available.</p> </li> </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/iotevents/latest/developerguide/iotevents-analyze-api.html">Running detector model analyses</a> in the <i>AWS IoT Events Developer Guide</i>.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Contains information that you can use to locate the field in your detector model that the analysis result references.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AnalysisResultLocation {
    /// <p>A <a href="https://github.com/json-path/JsonPath">JsonPath</a> expression that identifies the error field in your detector model.</p>
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

/// <p>A structure that contains timestamp information. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/APIReference/API_TimeInNanos.html">TimeInNanos</a> in the <i>AWS IoT SiteWise API Reference</i>.</p> <p>You must use expressions for all parameters in <code>AssetPropertyTimestamp</code>. The expressions accept literals, operators, functions, references, and substitution templates.</p> <p class="title"> <b>Examples</b> </p> <ul> <li> <p>For literal values, the expressions must contain single quotes. For example, the value for the <code>timeInSeconds</code> parameter can be <code>'1586400675'</code>.</p> </li> <li> <p>For references, you must specify either variables or input values. For example, the value for the <code>offsetInNanos</code> parameter can be <code>$variable.time</code>.</p> </li> <li> <p>For a substitution template, you must use <code>${}</code>, and the template must be in single quotes. A substitution template can also contain a combination of literals, operators, functions, references, and substitution templates.</p> <p>In the following example, the value for the <code>timeInSeconds</code> parameter uses a substitution template.</p> <p> <code>'${$input.TemperatureInput.sensorData.timestamp / 1000}'</code> </p> </li> </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/iotevents/latest/developerguide/iotevents-expressions.html">Expressions</a> in the <i>AWS IoT Events Developer Guide</i>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AssetPropertyTimestamp {
    /// <p>The nanosecond offset converted from <code>timeInSeconds</code>. The valid range is between 0-999999999.</p>
    #[serde(rename = "offsetInNanos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_in_nanos: Option<String>,
    /// <p>The timestamp, in seconds, in the Unix epoch format. The valid range is between 1-31556889864403199.</p>
    #[serde(rename = "timeInSeconds")]
    pub time_in_seconds: String,
}

/// <p>A structure that contains value information. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/APIReference/API_AssetPropertyValue.html">AssetPropertyValue</a> in the <i>AWS IoT SiteWise API Reference</i>.</p> <p>You must use expressions for all parameters in <code>AssetPropertyValue</code>. The expressions accept literals, operators, functions, references, and substitution templates.</p> <p class="title"> <b>Examples</b> </p> <ul> <li> <p>For literal values, the expressions must contain single quotes. For example, the value for the <code>quality</code> parameter can be <code>'GOOD'</code>.</p> </li> <li> <p>For references, you must specify either variables or input values. For example, the value for the <code>quality</code> parameter can be <code>$input.TemperatureInput.sensorData.quality</code>.</p> </li> </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/iotevents/latest/developerguide/iotevents-expressions.html">Expressions</a> in the <i>AWS IoT Events Developer Guide</i>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AssetPropertyValue {
    /// <p>The quality of the asset property value. The value must be <code>'GOOD'</code>, <code>'BAD'</code>, or <code>'UNCERTAIN'</code>.</p>
    #[serde(rename = "quality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<String>,
    /// <p>The timestamp associated with the asset property value. The default is the current event time.</p>
    #[serde(rename = "timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<AssetPropertyTimestamp>,
    /// <p>The value to send to an asset property.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<AssetPropertyVariant>,
}

/// <p>A structure that contains an asset property value. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/APIReference/API_Variant.html">Variant</a> in the <i>AWS IoT SiteWise API Reference</i>.</p> <p>You must use expressions for all parameters in <code>AssetPropertyVariant</code>. The expressions accept literals, operators, functions, references, and substitution templates.</p> <p class="title"> <b>Examples</b> </p> <ul> <li> <p>For literal values, the expressions must contain single quotes. For example, the value for the <code>integerValue</code> parameter can be <code>'100'</code>.</p> </li> <li> <p>For references, you must specify either variables or parameters. For example, the value for the <code>booleanValue</code> parameter can be <code>$variable.offline</code>.</p> </li> <li> <p>For a substitution template, you must use <code>${}</code>, and the template must be in single quotes. A substitution template can also contain a combination of literals, operators, functions, references, and substitution templates. </p> <p>In the following example, the value for the <code>doubleValue</code> parameter uses a substitution template. </p> <p> <code>'${$input.TemperatureInput.sensorData.temperature * 6 / 5 + 32}'</code> </p> </li> </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/iotevents/latest/developerguide/iotevents-expressions.html">Expressions</a> in the <i>AWS IoT Events Developer Guide</i>.</p> <p>You must specify one of the following value types, depending on the <code>dataType</code> of the specified asset property. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/APIReference/API_AssetProperty.html">AssetProperty</a> in the <i>AWS IoT SiteWise API Reference</i>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AssetPropertyVariant {
    /// <p>The asset property value is a Boolean value that must be <code>'TRUE'</code> or <code>'FALSE'</code>. You must use an expression, and the evaluated result should be a Boolean value.</p>
    #[serde(rename = "booleanValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boolean_value: Option<String>,
    /// <p>The asset property value is a double. You must use an expression, and the evaluated result should be a double.</p>
    #[serde(rename = "doubleValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_value: Option<String>,
    /// <p>The asset property value is an integer. You must use an expression, and the evaluated result should be an integer.</p>
    #[serde(rename = "integerValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integer_value: Option<String>,
    /// <p>The asset property value is a string. You must use an expression, and the evaluated result should be a string.</p>
    #[serde(rename = "stringValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_value: Option<String>,
}

/// <p>The attributes from the JSON payload that are made available by the input. Inputs are derived from messages sent to the AWS IoT Events system using <code>BatchPutMessage</code>. Each such message contains a JSON payload. Those attributes (and their paired values) specified here are available for use in the <code>condition</code> expressions used by detectors. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Attribute {
    /// <p>An expression that specifies an attribute-value pair in a JSON structure. Use this to specify an attribute from the JSON payload that is made available by the input. Inputs are derived from messages sent to AWS IoT Events (<code>BatchPutMessage</code>). Each such message contains a JSON payload. The attribute (and its paired value) specified here are available for use in the <code>condition</code> expressions used by detectors. </p> <p>Syntax: <code>&lt;field-name&gt;.&lt;field-name&gt;...</code> </p>
    #[serde(rename = "jsonPath")]
    pub json_path: String,
}

/// <p>Information needed to clear the timer.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ClearTimerAction {
    /// <p>The name of the timer to clear.</p>
    #[serde(rename = "timerName")]
    pub timer_name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateAlarmModelRequest {
    /// <p>Contains the configuration information of alarm state changes.</p>
    #[serde(rename = "alarmCapabilities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_capabilities: Option<AlarmCapabilities>,
    /// <p>Contains information about one or more alarm actions.</p>
    #[serde(rename = "alarmEventActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_event_actions: Option<AlarmEventActions>,
    /// <p>A description that tells you what the alarm model detects.</p>
    #[serde(rename = "alarmModelDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_model_description: Option<String>,
    /// <p>A unique name that helps you identify the alarm model. You can't change this name after you create the alarm model.</p>
    #[serde(rename = "alarmModelName")]
    pub alarm_model_name: String,
    /// <p>Contains information about one or more notification actions.</p>
    #[serde(rename = "alarmNotification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_notification: Option<AlarmNotification>,
    /// <p>Defines when your alarm is invoked.</p>
    #[serde(rename = "alarmRule")]
    pub alarm_rule: AlarmRule,
    /// <p>An input attribute used as a key to create an alarm. AWS IoT Events routes <a href="https://docs.aws.amazon.com/iotevents/latest/apireference/API_Input.html">inputs</a> associated with this key to the alarm.</p>
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The ARN of the IAM role that allows the alarm to perform actions and access AWS resources. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>AWS General Reference</i>.</p>
    #[serde(rename = "roleArn")]
    pub role_arn: String,
    /// <p>A non-negative integer that reflects the severity level of the alarm.</p>
    #[serde(rename = "severity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<i64>,
    /// <p>A list of key-value pairs that contain metadata for the alarm model. The tags help you manage the alarm model. For more information, see <a href="https://docs.aws.amazon.com/iotevents/latest/developerguide/tagging-iotevents.html">Tagging your AWS IoT Events resources</a> in the <i>AWS IoT Events Developer Guide</i>.</p> <p>You can create up to 50 tags for one alarm model.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateAlarmModelResponse {
    /// <p>The ARN of the alarm model. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>AWS General Reference</i>.</p>
    #[serde(rename = "alarmModelArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_model_arn: Option<String>,
    /// <p>The version of the alarm model.</p>
    #[serde(rename = "alarmModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_model_version: Option<String>,
    /// <p>The time the alarm model was created, in the Unix epoch format.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The time the alarm model was last updated, in the Unix epoch format.</p>
    #[serde(rename = "lastUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    /// <p><p>The status of the alarm model. The status can be one of the following values:</p> <ul> <li> <p> <code>ACTIVE</code> - The alarm model is active and it&#39;s ready to evaluate data.</p> </li> <li> <p> <code>ACTIVATING</code> - AWS IoT Events is activating your alarm model. Activating an alarm model can take up to a few minutes.</p> </li> <li> <p> <code>INACTIVE</code> - The alarm model is inactive, so it isn&#39;t ready to evaluate data. Check your alarm model information and update the alarm model.</p> </li> <li> <p> <code>FAILED</code> - You couldn&#39;t create or update the alarm model. Check your alarm model information and try again.</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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
    /// <p>The input attribute key used to identify a device or system to create a detector (an instance of the detector model) and then to route each input received to the appropriate detector (instance). This parameter uses a JSON-path expression in the message payload of each input to specify the attribute-value pair that is used to identify the device associated with the input.</p>
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDetectorModelResponse {
    /// <p>Information about how the detector model is configured.</p>
    #[serde(rename = "detectorModelConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_model_configuration: Option<DetectorModelConfiguration>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateInputResponse {
    /// <p>Information about the configuration of the input.</p>
    #[serde(rename = "inputConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_configuration: Option<InputConfiguration>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteAlarmModelRequest {
    /// <p>The name of the alarm model.</p>
    #[serde(rename = "alarmModelName")]
    pub alarm_model_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteAlarmModelResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDetectorModelRequest {
    /// <p>The name of the detector model to be deleted.</p>
    #[serde(rename = "detectorModelName")]
    pub detector_model_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteDetectorModelResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteInputRequest {
    /// <p>The name of the input to delete.</p>
    #[serde(rename = "inputName")]
    pub input_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteInputResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeAlarmModelRequest {
    /// <p>The name of the alarm model.</p>
    #[serde(rename = "alarmModelName")]
    pub alarm_model_name: String,
    /// <p>The version of the alarm model.</p>
    #[serde(rename = "alarmModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_model_version: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAlarmModelResponse {
    /// <p>Contains the configuration information of alarm state changes.</p>
    #[serde(rename = "alarmCapabilities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_capabilities: Option<AlarmCapabilities>,
    /// <p>Contains information about one or more alarm actions.</p>
    #[serde(rename = "alarmEventActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_event_actions: Option<AlarmEventActions>,
    /// <p>The ARN of the alarm model. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>AWS General Reference</i>.</p>
    #[serde(rename = "alarmModelArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_model_arn: Option<String>,
    /// <p>The description of the alarm model.</p>
    #[serde(rename = "alarmModelDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_model_description: Option<String>,
    /// <p>The name of the alarm model.</p>
    #[serde(rename = "alarmModelName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_model_name: Option<String>,
    /// <p>The version of the alarm model.</p>
    #[serde(rename = "alarmModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_model_version: Option<String>,
    /// <p>Contains information about one or more notification actions.</p>
    #[serde(rename = "alarmNotification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_notification: Option<AlarmNotification>,
    /// <p>Defines when your alarm is invoked.</p>
    #[serde(rename = "alarmRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_rule: Option<AlarmRule>,
    /// <p>The time the alarm model was created, in the Unix epoch format.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>An input attribute used as a key to create an alarm. AWS IoT Events routes <a href="https://docs.aws.amazon.com/iotevents/latest/apireference/API_Input.html">inputs</a> associated with this key to the alarm.</p>
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The time the alarm model was last updated, in the Unix epoch format.</p>
    #[serde(rename = "lastUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    /// <p>The ARN of the IAM role that allows the alarm to perform actions and access AWS resources. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>AWS General Reference</i>.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>A non-negative integer that reflects the severity level of the alarm.</p>
    #[serde(rename = "severity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<i64>,
    /// <p><p>The status of the alarm model. The status can be one of the following values:</p> <ul> <li> <p> <code>ACTIVE</code> - The alarm model is active and it&#39;s ready to evaluate data.</p> </li> <li> <p> <code>ACTIVATING</code> - AWS IoT Events is activating your alarm model. Activating an alarm model can take up to a few minutes.</p> </li> <li> <p> <code>INACTIVE</code> - The alarm model is inactive, so it isn&#39;t ready to evaluate data. Check your alarm model information and update the alarm model.</p> </li> <li> <p> <code>FAILED</code> - You couldn&#39;t create or update the alarm model. Check your alarm model information and try again.</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p> Contains information about the status of the alarm model. </p>
    #[serde(rename = "statusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDetectorModelAnalysisRequest {
    /// <p>The ID of the analysis result that you want to retrieve.</p>
    #[serde(rename = "analysisId")]
    pub analysis_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeDetectorModelAnalysisResponse {
    /// <p><p>The status of the analysis activity. The status can be one of the following values:</p> <ul> <li> <p> <code>RUNNING</code> - AWS IoT Events is analyzing your detector model. This process can take several minutes to complete.</p> </li> <li> <p> <code>COMPLETE</code> - AWS IoT Events finished analyzing your detector model.</p> </li> <li> <p> <code>FAILED</code> - AWS IoT Events couldn&#39;t analyze your detector model. Try again later.</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeDetectorModelResponse {
    /// <p>Information about the detector model.</p>
    #[serde(rename = "detectorModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_model: Option<DetectorModel>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeInputRequest {
    /// <p>The name of the input.</p>
    #[serde(rename = "inputName")]
    pub input_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeInputResponse {
    /// <p>Information about the input.</p>
    #[serde(rename = "input")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<Input>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeLoggingOptionsRequest {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeLoggingOptionsResponse {
    /// <p>The current settings of the AWS IoT Events logging options.</p>
    #[serde(rename = "loggingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_options: Option<LoggingOptions>,
}

/// <p>The detector model and the specific detectors (instances) for which the logging level is given.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
    /// <p>The value used to identify a detector instance. When a device or system sends input, a new detector instance with a unique key value is created. AWS IoT Events can continue to route input to its corresponding detector instance based on this identifying information. </p> <p>This parameter uses a JSON-path expression to select the attribute-value pair in the message payload that is used for identification. To route the message to the correct detector instance, the device must send a message payload that contains the same attribute-value.</p>
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DetectorModelDefinition {
    /// <p>The state that is entered at the creation of each detector (instance).</p>
    #[serde(rename = "initialStateName")]
    pub initial_state_name: String,
    /// <p>Information about the states of the detector.</p>
    #[serde(rename = "states")]
    pub states: Vec<State>,
}

/// <p>Information about the detector model.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

/// <p>Defines an action to write to the Amazon DynamoDB table that you created. The standard action payload contains all the information about the detector model instance and the event that triggered the action. You can customize the <a href="https://docs.aws.amazon.com/iotevents/latest/apireference/API_Payload.html">payload</a>. One column of the DynamoDB table receives all attribute-value pairs in the payload that you specify.</p> <p>You must use expressions for all parameters in <code>DynamoDBAction</code>. The expressions accept literals, operators, functions, references, and substitution templates.</p> <p class="title"> <b>Examples</b> </p> <ul> <li> <p>For literal values, the expressions must contain single quotes. For example, the value for the <code>hashKeyType</code> parameter can be <code>'STRING'</code>.</p> </li> <li> <p>For references, you must specify either variables or input values. For example, the value for the <code>hashKeyField</code> parameter can be <code>$input.GreenhouseInput.name</code>.</p> </li> <li> <p>For a substitution template, you must use <code>${}</code>, and the template must be in single quotes. A substitution template can also contain a combination of literals, operators, functions, references, and substitution templates.</p> <p>In the following example, the value for the <code>hashKeyValue</code> parameter uses a substitution template. </p> <p> <code>'${$input.GreenhouseInput.temperature * 6 / 5 + 32} in Fahrenheit'</code> </p> </li> <li> <p>For a string concatenation, you must use <code>+</code>. A string concatenation can also contain a combination of literals, operators, functions, references, and substitution templates.</p> <p>In the following example, the value for the <code>tableName</code> parameter uses a string concatenation. </p> <p> <code>'GreenhouseTemperatureTable ' + $input.GreenhouseInput.date</code> </p> </li> </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/iotevents/latest/developerguide/iotevents-expressions.html">Expressions</a> in the <i>AWS IoT Events Developer Guide</i>.</p> <p>If the defined payload type is a string, <code>DynamoDBAction</code> writes non-JSON data to the DynamoDB table as binary data. The DynamoDB console displays the data as Base64-encoded text. The value for the <code>payloadField</code> parameter is <code>&lt;payload-field&gt;_raw</code>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DynamoDBAction {
    /// <p>The name of the hash key (also called the partition key). The <code>hashKeyField</code> value must match the partition key of the target DynamoDB table.</p>
    #[serde(rename = "hashKeyField")]
    pub hash_key_field: String,
    /// <p>The data type for the hash key (also called the partition key). You can specify the following values:</p> <ul> <li> <p> <code>'STRING'</code> - The hash key is a string.</p> </li> <li> <p> <code>'NUMBER'</code> - The hash key is a number.</p> </li> </ul> <p>If you don't specify <code>hashKeyType</code>, the default value is <code>'STRING'</code>.</p>
    #[serde(rename = "hashKeyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_key_type: Option<String>,
    /// <p>The value of the hash key (also called the partition key).</p>
    #[serde(rename = "hashKeyValue")]
    pub hash_key_value: String,
    /// <p>The type of operation to perform. You can specify the following values: </p> <ul> <li> <p> <code>'INSERT'</code> - Insert data as a new item into the DynamoDB table. This item uses the specified hash key as a partition key. If you specified a range key, the item uses the range key as a sort key.</p> </li> <li> <p> <code>'UPDATE'</code> - Update an existing item of the DynamoDB table with new data. This item's partition key must match the specified hash key. If you specified a range key, the range key must match the item's sort key.</p> </li> <li> <p> <code>'DELETE'</code> - Delete an existing item of the DynamoDB table. This item's partition key must match the specified hash key. If you specified a range key, the range key must match the item's sort key.</p> </li> </ul> <p>If you don't specify this parameter, AWS IoT Events triggers the <code>'INSERT'</code> operation.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(rename = "payload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Payload>,
    /// <p>The name of the DynamoDB column that receives the action payload.</p> <p>If you don't specify this parameter, the name of the DynamoDB column is <code>payload</code>.</p>
    #[serde(rename = "payloadField")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_field: Option<String>,
    /// <p>The name of the range key (also called the sort key). The <code>rangeKeyField</code> value must match the sort key of the target DynamoDB table. </p>
    #[serde(rename = "rangeKeyField")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_key_field: Option<String>,
    /// <p>The data type for the range key (also called the sort key), You can specify the following values:</p> <ul> <li> <p> <code>'STRING'</code> - The range key is a string.</p> </li> <li> <p> <code>'NUMBER'</code> - The range key is number.</p> </li> </ul> <p>If you don't specify <code>rangeKeyField</code>, the default value is <code>'STRING'</code>.</p>
    #[serde(rename = "rangeKeyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_key_type: Option<String>,
    /// <p>The value of the range key (also called the sort key).</p>
    #[serde(rename = "rangeKeyValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_key_value: Option<String>,
    /// <p>The name of the DynamoDB table. The <code>tableName</code> value must match the table name of the target DynamoDB table. </p>
    #[serde(rename = "tableName")]
    pub table_name: String,
}

/// <p>Defines an action to write to the Amazon DynamoDB table that you created. The default action payload contains all the information about the detector model instance and the event that triggered the action. You can customize the <a href="https://docs.aws.amazon.com/iotevents/latest/apireference/API_Payload.html">payload</a>. A separate column of the DynamoDB table receives one attribute-value pair in the payload that you specify.</p> <p>You must use expressions for all parameters in <code>DynamoDBv2Action</code>. The expressions accept literals, operators, functions, references, and substitution templates.</p> <p class="title"> <b>Examples</b> </p> <ul> <li> <p>For literal values, the expressions must contain single quotes. For example, the value for the <code>tableName</code> parameter can be <code>'GreenhouseTemperatureTable'</code>.</p> </li> <li> <p>For references, you must specify either variables or input values. For example, the value for the <code>tableName</code> parameter can be <code>$variable.ddbtableName</code>.</p> </li> <li> <p>For a substitution template, you must use <code>${}</code>, and the template must be in single quotes. A substitution template can also contain a combination of literals, operators, functions, references, and substitution templates.</p> <p>In the following example, the value for the <code>contentExpression</code> parameter in <code>Payload</code> uses a substitution template. </p> <p> <code>'{\"sensorID\": \"${$input.GreenhouseInput.sensor_id}\", \"temperature\": \"${$input.GreenhouseInput.temperature * 9 / 5 + 32}\"}'</code> </p> </li> <li> <p>For a string concatenation, you must use <code>+</code>. A string concatenation can also contain a combination of literals, operators, functions, references, and substitution templates.</p> <p>In the following example, the value for the <code>tableName</code> parameter uses a string concatenation. </p> <p> <code>'GreenhouseTemperatureTable ' + $input.GreenhouseInput.date</code> </p> </li> </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/iotevents/latest/developerguide/iotevents-expressions.html">Expressions</a> in the <i>AWS IoT Events Developer Guide</i>.</p> <p>The value for the <code>type</code> parameter in <code>Payload</code> must be <code>JSON</code>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DynamoDBv2Action {
    #[serde(rename = "payload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Payload>,
    /// <p>The name of the DynamoDB table.</p>
    #[serde(rename = "tableName")]
    pub table_name: String,
}

/// <p>Contains the configuration information of email notifications.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct EmailConfiguration {
    /// <p>Contains the subject and message of an email.</p>
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<EmailContent>,
    /// <p><p>The email address that sends emails.</p> <important> <p>If you use the AWS IoT Events managed AWS Lambda function to manage your emails, you must <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/verify-email-addresses.html">verify the email address that sends emails in Amazon SES</a>.</p> </important></p>
    #[serde(rename = "from")]
    pub from: String,
    /// <p><p>Contains the information of one or more recipients who receive the emails.</p> <important> <p>You must <a href="https://docs.aws.amazon.com/singlesignon/latest/userguide/addusers.html">add the users that receive emails to your AWS SSO store</a>.</p> </important></p>
    #[serde(rename = "recipients")]
    pub recipients: EmailRecipients,
}

/// <p>Contains the subject and message of an email.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct EmailContent {
    /// <p>The message that you want to send. The message can be up to 200 characters.</p>
    #[serde(rename = "additionalMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_message: Option<String>,
    /// <p>The subject of the email.</p>
    #[serde(rename = "subject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
}

/// <p><p>Contains the information of one or more recipients who receive the emails.</p> <important> <p>You must <a href="https://docs.aws.amazon.com/singlesignon/latest/userguide/addusers.html">add the users that receive emails to your AWS SSO store</a>.</p> </important></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct EmailRecipients {
    /// <p>Specifies one or more recipients who receive the email.</p>
    #[serde(rename = "to")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<Vec<RecipientDetail>>,
}

/// <p>Specifies the <code>actions</code> to be performed when the <code>condition</code> evaluates to TRUE.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Event {
    /// <p>The actions to be performed.</p>
    #[serde(rename = "actions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<Action>>,
    /// <p>Optional. The Boolean expression that, when TRUE, causes the <code>actions</code> to be performed. If not present, the actions are performed (=TRUE). If the expression result is not a Boolean value, the actions are not performed (=FALSE).</p>
    #[serde(rename = "condition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    /// <p>The name of the event.</p>
    #[serde(rename = "eventName")]
    pub event_name: String,
}

/// <p>Sends information about the detector model instance and the event that triggered the action to an Amazon Kinesis Data Firehose delivery stream.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FirehoseAction {
    /// <p>The name of the Kinesis Data Firehose delivery stream where the data is written.</p>
    #[serde(rename = "deliveryStreamName")]
    pub delivery_stream_name: String,
    /// <p>You can configure the action payload when you send a message to an Amazon Kinesis Data Firehose delivery stream.</p>
    #[serde(rename = "payload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Payload>,
    /// <p>A character separator that is used to separate records written to the Kinesis Data Firehose delivery stream. Valid values are: '\n' (newline), '\t' (tab), '\r\n' (Windows newline), ',' (comma).</p>
    #[serde(rename = "separator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDetectorModelAnalysisResultsRequest {
    /// <p>The ID of the analysis result that you want to retrieve.</p>
    #[serde(rename = "analysisId")]
    pub analysis_id: String,
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
pub struct GetDetectorModelAnalysisResultsResponse {
    /// <p>Contains information about one or more analysis results.</p>
    #[serde(rename = "analysisResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_results: Option<Vec<AnalysisResult>>,
    /// <p>The token that you can use to return the next set of results, or <code>null</code> if there are no more results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Specifies the default alarm state. The configuration applies to all alarms that were created based on this alarm model.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct InitializationConfiguration {
    /// <p>The value must be <code>TRUE</code> or <code>FALSE</code>. If <code>FALSE</code>, all alarm instances created based on the alarm model are activated. The default value is <code>TRUE</code>.</p>
    #[serde(rename = "disabledOnInitialization")]
    pub disabled_on_initialization: bool,
}

/// <p>Information about the input.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct InputDefinition {
    /// <p>The attributes from the JSON payload that are made available by the input. Inputs are derived from messages sent to the AWS IoT Events system using <code>BatchPutMessage</code>. Each such message contains a JSON payload, and those attributes (and their paired values) specified here are available for use in the <code>condition</code> expressions used by detectors that monitor this input. </p>
    #[serde(rename = "attributes")]
    pub attributes: Vec<Attribute>,
}

/// <p> The identifer of the input. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InputIdentifier {
    /// <p> The identifier of the input routed to AWS IoT Events. </p>
    #[serde(rename = "iotEventsInputIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iot_events_input_identifier: Option<IotEventsInputIdentifier>,
    /// <p> The identifer of the input routed from AWS IoT SiteWise. </p>
    #[serde(rename = "iotSiteWiseInputIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iot_site_wise_input_identifier: Option<IotSiteWiseInputIdentifier>,
}

/// <p>Information about the input.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct IotEventsAction {
    /// <p>The name of the AWS IoT Events input where the data is sent.</p>
    #[serde(rename = "inputName")]
    pub input_name: String,
    /// <p>You can configure the action payload when you send a message to an AWS IoT Events input.</p>
    #[serde(rename = "payload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Payload>,
}

/// <p> The identifier of the input routed to AWS IoT Events. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct IotEventsInputIdentifier {
    /// <p> The name of the input routed to AWS IoT Events. </p>
    #[serde(rename = "inputName")]
    pub input_name: String,
}

/// <p>Sends information about the detector model instance and the event that triggered the action to a specified asset property in AWS IoT SiteWise.</p> <p>You must use expressions for all parameters in <code>IotSiteWiseAction</code>. The expressions accept literals, operators, functions, references, and substitutions templates.</p> <p class="title"> <b>Examples</b> </p> <ul> <li> <p>For literal values, the expressions must contain single quotes. For example, the value for the <code>propertyAlias</code> parameter can be <code>'/company/windfarm/3/turbine/7/temperature'</code>.</p> </li> <li> <p>For references, you must specify either variables or input values. For example, the value for the <code>assetId</code> parameter can be <code>$input.TurbineInput.assetId1</code>.</p> </li> <li> <p>For a substitution template, you must use <code>${}</code>, and the template must be in single quotes. A substitution template can also contain a combination of literals, operators, functions, references, and substitution templates.</p> <p>In the following example, the value for the <code>propertyAlias</code> parameter uses a substitution template. </p> <p> <code>'company/windfarm/${$input.TemperatureInput.sensorData.windfarmID}/turbine/ ${$input.TemperatureInput.sensorData.turbineID}/temperature'</code> </p> </li> </ul> <p>You must specify either <code>propertyAlias</code> or both <code>assetId</code> and <code>propertyId</code> to identify the target asset property in AWS IoT SiteWise.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/iotevents/latest/developerguide/iotevents-expressions.html">Expressions</a> in the <i>AWS IoT Events Developer Guide</i>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct IotSiteWiseAction {
    /// <p>The ID of the asset that has the specified property.</p>
    #[serde(rename = "assetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    /// <p>A unique identifier for this entry. You can use the entry ID to track which data entry causes an error in case of failure. The default is a new unique identifier.</p>
    #[serde(rename = "entryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_id: Option<String>,
    /// <p>The alias of the asset property.</p>
    #[serde(rename = "propertyAlias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_alias: Option<String>,
    /// <p>The ID of the asset property.</p>
    #[serde(rename = "propertyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_id: Option<String>,
    /// <p>The value to send to the asset property. This value contains timestamp, quality, and value (TQV) information. </p>
    #[serde(rename = "propertyValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_value: Option<AssetPropertyValue>,
}

/// <p> The asset model property identifer of the input routed from AWS IoT SiteWise. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct IotSiteWiseAssetModelPropertyIdentifier {
    /// <p> The ID of the AWS IoT SiteWise asset model. </p>
    #[serde(rename = "assetModelId")]
    pub asset_model_id: String,
    /// <p> The ID of the AWS IoT SiteWise asset property. </p>
    #[serde(rename = "propertyId")]
    pub property_id: String,
}

/// <p> The identifer of the input routed from AWS IoT SiteWise. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct IotSiteWiseInputIdentifier {
    /// <p> The identifier of the AWS IoT SiteWise asset model property. </p>
    #[serde(rename = "iotSiteWiseAssetModelPropertyIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iot_site_wise_asset_model_property_identifier:
        Option<IotSiteWiseAssetModelPropertyIdentifier>,
}

/// <p>Information required to publish the MQTT message through the AWS IoT message broker.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct IotTopicPublishAction {
    /// <p>The MQTT topic of the message. You can use a string expression that includes variables (<code>$variable.&lt;variable-name&gt;</code>) and input values (<code>$input.&lt;input-name&gt;.&lt;path-to-datum&gt;</code>) as the topic string.</p>
    #[serde(rename = "mqttTopic")]
    pub mqtt_topic: String,
    /// <p>You can configure the action payload when you publish a message to an AWS IoT Core topic.</p>
    #[serde(rename = "payload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Payload>,
}

/// <p>Calls a Lambda function, passing in information about the detector model instance and the event that triggered the action.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct LambdaAction {
    /// <p>The ARN of the Lambda function that is executed.</p>
    #[serde(rename = "functionArn")]
    pub function_arn: String,
    /// <p>You can configure the action payload when you send a message to a Lambda function.</p>
    #[serde(rename = "payload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Payload>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListAlarmModelVersionsRequest {
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
pub struct ListAlarmModelVersionsResponse {
    /// <p>A list that summarizes each alarm model version.</p>
    #[serde(rename = "alarmModelVersionSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_model_version_summaries: Option<Vec<AlarmModelVersionSummary>>,
    /// <p>The token that you can use to return the next set of results, or <code>null</code> if there are no more results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListAlarmModelsRequest {
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
pub struct ListAlarmModelsResponse {
    /// <p>A list that summarizes each alarm model.</p>
    #[serde(rename = "alarmModelSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_model_summaries: Option<Vec<AlarmModelSummary>>,
    /// <p>The token that you can use to return the next set of results, or <code>null</code> if there are no more results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDetectorModelVersionsRequest {
    /// <p>The name of the detector model whose versions are returned.</p>
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
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDetectorModelVersionsResponse {
    /// <p>Summary information about the detector model versions.</p>
    #[serde(rename = "detectorModelVersionSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_model_version_summaries: Option<Vec<DetectorModelVersionSummary>>,
    /// <p>The token that you can use to return the next set of results, or <code>null</code> if there are no more results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDetectorModelsRequest {
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
pub struct ListDetectorModelsResponse {
    /// <p>Summary information about the detector models.</p>
    #[serde(rename = "detectorModelSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_model_summaries: Option<Vec<DetectorModelSummary>>,
    /// <p>The token that you can use to return the next set of results, or <code>null</code> if there are no more results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListInputRoutingsRequest {
    /// <p> The identifer of the routed input. </p>
    #[serde(rename = "inputIdentifier")]
    pub input_identifier: InputIdentifier,
    /// <p> The maximum number of results to be returned per request. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p> The token that you can use to return the next set of results. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListInputRoutingsResponse {
    /// <p> The token that you can use to return the next set of results, or <code>null</code> if there are no more results. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p> Summary information about the routed resources. </p>
    #[serde(rename = "routedResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routed_resources: Option<Vec<RoutedResource>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListInputsRequest {
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
pub struct ListInputsResponse {
    /// <p>Summary information about the inputs.</p>
    #[serde(rename = "inputSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_summaries: Option<Vec<InputSummary>>,
    /// <p>The token that you can use to return the next set of results, or <code>null</code> if there are no more results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The ARN of the resource.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>The list of tags assigned to the resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>The values of the AWS IoT Events logging options.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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

/// <p>Contains the notification settings of an alarm model. The settings apply to all alarms that were created based on this alarm model.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct NotificationAction {
    /// <p>Specifies an AWS Lambda function to manage alarm notifications. You can create one or use the <a href="https://docs.aws.amazon.com/iotevents/latest/developerguide/lambda-support.html">AWS Lambda function provided by AWS IoT Events</a>.</p>
    #[serde(rename = "action")]
    pub action: NotificationTargetActions,
    /// <p>Contains the configuration information of email notifications.</p>
    #[serde(rename = "emailConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_configurations: Option<Vec<EmailConfiguration>>,
    /// <p>Contains the configuration information of SMS notifications.</p>
    #[serde(rename = "smsConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_configurations: Option<Vec<SMSConfiguration>>,
}

/// <p>Specifies an AWS Lambda function to manage alarm notifications. You can create one or use the <a href="https://docs.aws.amazon.com/iotevents/latest/developerguide/lambda-support.html">AWS Lambda function provided by AWS IoT Events</a>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct NotificationTargetActions {
    #[serde(rename = "lambdaAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_action: Option<LambdaAction>,
}

/// <p>When entering this state, perform these <code>actions</code> if the <code>condition</code> is TRUE.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct OnEnterLifecycle {
    /// <p>Specifies the actions that are performed when the state is entered and the <code>condition</code> is <code>TRUE</code>.</p>
    #[serde(rename = "events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<Event>>,
}

/// <p>When exiting this state, perform these <code>actions</code> if the specified <code>condition</code> is <code>TRUE</code>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct OnExitLifecycle {
    /// <p>Specifies the <code>actions</code> that are performed when the state is exited and the <code>condition</code> is <code>TRUE</code>.</p>
    #[serde(rename = "events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<Event>>,
}

/// <p>Specifies the actions performed when the <code>condition</code> evaluates to TRUE.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct OnInputLifecycle {
    /// <p>Specifies the actions performed when the <code>condition</code> evaluates to TRUE.</p>
    #[serde(rename = "events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<Event>>,
    /// <p>Specifies the actions performed, and the next state entered, when a <code>condition</code> evaluates to TRUE.</p>
    #[serde(rename = "transitionEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition_events: Option<Vec<TransitionEvent>>,
}

/// <p>Information needed to configure the payload.</p> <p>By default, AWS IoT Events generates a standard payload in JSON for any action. This action payload contains all attribute-value pairs that have the information about the detector model instance and the event triggered the action. To configure the action payload, you can use <code>contentExpression</code>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Payload {
    /// <p>The content of the payload. You can use a string expression that includes quoted strings (<code>'&lt;string&gt;'</code>), variables (<code>$variable.&lt;variable-name&gt;</code>), input values (<code>$input.&lt;input-name&gt;.&lt;path-to-datum&gt;</code>), string concatenations, and quoted strings that contain <code>${}</code> as the content. The recommended maximum size of a content expression is 1 KB.</p>
    #[serde(rename = "contentExpression")]
    pub content_expression: String,
    /// <p>The value of the payload type can be either <code>STRING</code> or <code>JSON</code>.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutLoggingOptionsRequest {
    /// <p>The new values of the AWS IoT Events logging options.</p>
    #[serde(rename = "loggingOptions")]
    pub logging_options: LoggingOptions,
}

/// <p>The information that identifies the recipient.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RecipientDetail {
    /// <p>The AWS Single Sign-On (AWS SSO) authentication information.</p>
    #[serde(rename = "ssoIdentity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sso_identity: Option<SSOIdentity>,
}

/// <p>Information required to reset the timer. The timer is reset to the previously evaluated result of the duration. The duration expression isn't reevaluated when you reset the timer.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ResetTimerAction {
    /// <p>The name of the timer to reset.</p>
    #[serde(rename = "timerName")]
    pub timer_name: String,
}

/// <p> Contains information about the routed resource. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RoutedResource {
    /// <p> The ARN of the routed resource. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>AWS General Reference</i>. </p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p> The name of the routed resource. </p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Contains the configuration information of SMS notifications.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SMSConfiguration {
    /// <p>The message that you want to send. The message can be up to 200 characters.</p>
    #[serde(rename = "additionalMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_message: Option<String>,
    /// <p><p>Specifies one or more recipients who receive the message.</p> <important> <p>You must <a href="https://docs.aws.amazon.com/singlesignon/latest/userguide/addusers.html">add the users that receive SMS messages to your AWS SSO store</a>.</p> </important></p>
    #[serde(rename = "recipients")]
    pub recipients: Vec<RecipientDetail>,
    /// <p>The sender ID.</p>
    #[serde(rename = "senderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<String>,
}

/// <p>Information required to publish the Amazon SNS message.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SNSTopicPublishAction {
    /// <p>You can configure the action payload when you send a message as an Amazon SNS push notification.</p>
    #[serde(rename = "payload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Payload>,
    /// <p>The ARN of the Amazon SNS target where the message is sent.</p>
    #[serde(rename = "targetArn")]
    pub target_arn: String,
}

/// <p>Contains information about your identity source in AWS Single Sign-On. For more information, see the <a href="https://docs.aws.amazon.com/singlesignon/latest/userguide/what-is.html">AWS Single Sign-On User Guide</a>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SSOIdentity {
    /// <p>The ID of the AWS SSO identity store.</p>
    #[serde(rename = "identityStoreId")]
    pub identity_store_id: String,
    /// <p>The user ID.</p>
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// <p>Information needed to set the timer.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SetTimerAction {
    /// <p>The duration of the timer, in seconds. You can use a string expression that includes numbers, variables (<code>$variable.&lt;variable-name&gt;</code>), and input values (<code>$input.&lt;input-name&gt;.&lt;path-to-datum&gt;</code>) as the duration. The range of the duration is 1-31622400 seconds. To ensure accuracy, the minimum duration is 60 seconds. The evaluated result of the duration is rounded down to the nearest whole number. </p>
    #[serde(rename = "durationExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_expression: Option<String>,
    /// <p>The name of the timer.</p>
    #[serde(rename = "timerName")]
    pub timer_name: String,
}

/// <p>Information about the variable and its new value.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SetVariableAction {
    /// <p>The new value of the variable.</p>
    #[serde(rename = "value")]
    pub value: String,
    /// <p>The name of the variable.</p>
    #[serde(rename = "variableName")]
    pub variable_name: String,
}

/// <p>A rule that compares an input property value to a threshold value with a comparison operator.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SimpleRule {
    /// <p>The comparison operator.</p>
    #[serde(rename = "comparisonOperator")]
    pub comparison_operator: String,
    /// <p>The value on the left side of the comparison operator. You can specify an AWS IoT Events input attribute as an input property.</p>
    #[serde(rename = "inputProperty")]
    pub input_property: String,
    /// <p>The value on the right side of the comparison operator. You can enter a number or specify an AWS IoT Events input attribute.</p>
    #[serde(rename = "threshold")]
    pub threshold: String,
}

/// <p>Sends information about the detector model instance and the event that triggered the action to an Amazon SQS queue.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SqsAction {
    /// <p>You can configure the action payload when you send a message to an Amazon SQS queue.</p>
    #[serde(rename = "payload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Payload>,
    /// <p>The URL of the SQS queue where the data is written.</p>
    #[serde(rename = "queueUrl")]
    pub queue_url: String,
    /// <p>Set this to TRUE if you want the data to be base-64 encoded before it is written to the queue. Otherwise, set this to FALSE.</p>
    #[serde(rename = "useBase64")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_base_64: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartDetectorModelAnalysisRequest {
    #[serde(rename = "detectorModelDefinition")]
    pub detector_model_definition: DetectorModelDefinition,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartDetectorModelAnalysisResponse {
    /// <p>The ID that you can use to retrieve the analysis result.</p>
    #[serde(rename = "analysisId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_id: Option<String>,
}

/// <p>Information that defines a state of a detector.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct State {
    /// <p>When entering this state, perform these <code>actions</code> if the <code>condition</code> is TRUE.</p>
    #[serde(rename = "onEnter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_enter: Option<OnEnterLifecycle>,
    /// <p>When exiting this state, perform these <code>actions</code> if the specified <code>condition</code> is <code>TRUE</code>.</p>
    #[serde(rename = "onExit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_exit: Option<OnExitLifecycle>,
    /// <p>When an input is received and the <code>condition</code> is TRUE, perform the specified <code>actions</code>.</p>
    #[serde(rename = "onInput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_input: Option<OnInputLifecycle>,
    /// <p>The name of the state.</p>
    #[serde(rename = "stateName")]
    pub state_name: String,
}

/// <p>Metadata that can be used to manage the resource.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tag {
    /// <p>The tag's key.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>The tag's value.</p>
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The ARN of the resource.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The new or modified tags for the resource.</p>
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

/// <p>Specifies the actions performed and the next state entered when a <code>condition</code> evaluates to TRUE.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TransitionEvent {
    /// <p>The actions to be performed.</p>
    #[serde(rename = "actions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<Action>>,
    /// <p>Required. A Boolean expression that when TRUE causes the actions to be performed and the <code>nextState</code> to be entered.</p>
    #[serde(rename = "condition")]
    pub condition: String,
    /// <p>The name of the transition event.</p>
    #[serde(rename = "eventName")]
    pub event_name: String,
    /// <p>The next state to enter.</p>
    #[serde(rename = "nextState")]
    pub next_state: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The ARN of the resource.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>A list of the keys of the tags to be removed from the resource.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateAlarmModelRequest {
    /// <p>Contains the configuration information of alarm state changes.</p>
    #[serde(rename = "alarmCapabilities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_capabilities: Option<AlarmCapabilities>,
    /// <p>Contains information about one or more alarm actions.</p>
    #[serde(rename = "alarmEventActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_event_actions: Option<AlarmEventActions>,
    /// <p>The description of the alarm model.</p>
    #[serde(rename = "alarmModelDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_model_description: Option<String>,
    /// <p>The name of the alarm model.</p>
    #[serde(rename = "alarmModelName")]
    pub alarm_model_name: String,
    /// <p>Contains information about one or more notification actions.</p>
    #[serde(rename = "alarmNotification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_notification: Option<AlarmNotification>,
    /// <p>Defines when your alarm is invoked.</p>
    #[serde(rename = "alarmRule")]
    pub alarm_rule: AlarmRule,
    /// <p>The ARN of the IAM role that allows the alarm to perform actions and access AWS resources. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>AWS General Reference</i>.</p>
    #[serde(rename = "roleArn")]
    pub role_arn: String,
    /// <p>A non-negative integer that reflects the severity level of the alarm.</p>
    #[serde(rename = "severity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateAlarmModelResponse {
    /// <p>The ARN of the alarm model. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>AWS General Reference</i>.</p>
    #[serde(rename = "alarmModelArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_model_arn: Option<String>,
    /// <p>The version of the alarm model.</p>
    #[serde(rename = "alarmModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_model_version: Option<String>,
    /// <p>The time the alarm model was created, in the Unix epoch format.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The time the alarm model was last updated, in the Unix epoch format.</p>
    #[serde(rename = "lastUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    /// <p><p>The status of the alarm model. The status can be one of the following values:</p> <ul> <li> <p> <code>ACTIVE</code> - The alarm model is active and it&#39;s ready to evaluate data.</p> </li> <li> <p> <code>ACTIVATING</code> - AWS IoT Events is activating your alarm model. Activating an alarm model can take up to a few minutes.</p> </li> <li> <p> <code>INACTIVE</code> - The alarm model is inactive, so it isn&#39;t ready to evaluate data. Check your alarm model information and update the alarm model.</p> </li> <li> <p> <code>FAILED</code> - You couldn&#39;t create or update the alarm model. Check your alarm model information and try again.</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDetectorModelResponse {
    /// <p>Information about how the detector model is configured.</p>
    #[serde(rename = "detectorModelConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_model_configuration: Option<DetectorModelConfiguration>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateInputResponse {
    /// <p>Information about the configuration of the input.</p>
    #[serde(rename = "inputConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_configuration: Option<InputConfiguration>,
}

/// Errors returned by CreateAlarmModel
#[derive(Debug, PartialEq)]
pub enum CreateAlarmModelError {
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

impl CreateAlarmModelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateAlarmModelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(CreateAlarmModelError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateAlarmModelError::InvalidRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateAlarmModelError::LimitExceeded(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateAlarmModelError::ResourceAlreadyExists(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(CreateAlarmModelError::ResourceInUse(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateAlarmModelError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateAlarmModelError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateAlarmModelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateAlarmModelError::InternalFailure(ref cause) => write!(f, "{}", cause),
            CreateAlarmModelError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateAlarmModelError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateAlarmModelError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateAlarmModelError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateAlarmModelError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateAlarmModelError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateAlarmModelError {}
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
/// Errors returned by DeleteAlarmModel
#[derive(Debug, PartialEq)]
pub enum DeleteAlarmModelError {
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

impl DeleteAlarmModelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAlarmModelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteAlarmModelError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteAlarmModelError::InvalidRequest(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteAlarmModelError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteAlarmModelError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteAlarmModelError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteAlarmModelError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteAlarmModelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteAlarmModelError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteAlarmModelError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteAlarmModelError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteAlarmModelError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteAlarmModelError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteAlarmModelError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteAlarmModelError {}
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
/// Errors returned by DescribeAlarmModel
#[derive(Debug, PartialEq)]
pub enum DescribeAlarmModelError {
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

impl DescribeAlarmModelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAlarmModelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(DescribeAlarmModelError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeAlarmModelError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeAlarmModelError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeAlarmModelError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeAlarmModelError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeAlarmModelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeAlarmModelError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DescribeAlarmModelError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeAlarmModelError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeAlarmModelError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DescribeAlarmModelError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeAlarmModelError {}
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
/// Errors returned by DescribeDetectorModelAnalysis
#[derive(Debug, PartialEq)]
pub enum DescribeDetectorModelAnalysisError {
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

impl DescribeDetectorModelAnalysisError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeDetectorModelAnalysisError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(
                        DescribeDetectorModelAnalysisError::InternalFailure(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        DescribeDetectorModelAnalysisError::InvalidRequest(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeDetectorModelAnalysisError::ResourceNotFound(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        DescribeDetectorModelAnalysisError::ServiceUnavailable(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeDetectorModelAnalysisError::Throttling(
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
impl fmt::Display for DescribeDetectorModelAnalysisError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDetectorModelAnalysisError::InternalFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeDetectorModelAnalysisError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeDetectorModelAnalysisError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeDetectorModelAnalysisError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeDetectorModelAnalysisError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeDetectorModelAnalysisError {}
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
/// Errors returned by GetDetectorModelAnalysisResults
#[derive(Debug, PartialEq)]
pub enum GetDetectorModelAnalysisResultsError {
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

impl GetDetectorModelAnalysisResultsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetDetectorModelAnalysisResultsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(
                        GetDetectorModelAnalysisResultsError::InternalFailure(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        GetDetectorModelAnalysisResultsError::InvalidRequest(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        GetDetectorModelAnalysisResultsError::ResourceNotFound(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        GetDetectorModelAnalysisResultsError::ServiceUnavailable(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetDetectorModelAnalysisResultsError::Throttling(
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
impl fmt::Display for GetDetectorModelAnalysisResultsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDetectorModelAnalysisResultsError::InternalFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            GetDetectorModelAnalysisResultsError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            GetDetectorModelAnalysisResultsError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            GetDetectorModelAnalysisResultsError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            GetDetectorModelAnalysisResultsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDetectorModelAnalysisResultsError {}
/// Errors returned by ListAlarmModelVersions
#[derive(Debug, PartialEq)]
pub enum ListAlarmModelVersionsError {
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

impl ListAlarmModelVersionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAlarmModelVersionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(ListAlarmModelVersionsError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListAlarmModelVersionsError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListAlarmModelVersionsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListAlarmModelVersionsError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListAlarmModelVersionsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListAlarmModelVersionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListAlarmModelVersionsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListAlarmModelVersionsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListAlarmModelVersionsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListAlarmModelVersionsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListAlarmModelVersionsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListAlarmModelVersionsError {}
/// Errors returned by ListAlarmModels
#[derive(Debug, PartialEq)]
pub enum ListAlarmModelsError {
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>The request was invalid.</p>
    InvalidRequest(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request could not be completed due to throttling.</p>
    Throttling(String),
}

impl ListAlarmModelsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAlarmModelsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(ListAlarmModelsError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListAlarmModelsError::InvalidRequest(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListAlarmModelsError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListAlarmModelsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListAlarmModelsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListAlarmModelsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListAlarmModelsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListAlarmModelsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListAlarmModelsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListAlarmModelsError {}
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
/// Errors returned by ListInputRoutings
#[derive(Debug, PartialEq)]
pub enum ListInputRoutingsError {
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

impl ListInputRoutingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListInputRoutingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(ListInputRoutingsError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListInputRoutingsError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListInputRoutingsError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListInputRoutingsError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListInputRoutingsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListInputRoutingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListInputRoutingsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListInputRoutingsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListInputRoutingsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListInputRoutingsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListInputRoutingsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListInputRoutingsError {}
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
/// Errors returned by StartDetectorModelAnalysis
#[derive(Debug, PartialEq)]
pub enum StartDetectorModelAnalysisError {
    /// <p>An internal failure occurred.</p>
    InternalFailure(String),
    /// <p>The request was invalid.</p>
    InvalidRequest(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request could not be completed due to throttling.</p>
    Throttling(String),
}

impl StartDetectorModelAnalysisError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<StartDetectorModelAnalysisError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(StartDetectorModelAnalysisError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StartDetectorModelAnalysisError::InvalidRequest(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(StartDetectorModelAnalysisError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        StartDetectorModelAnalysisError::ServiceUnavailable(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(StartDetectorModelAnalysisError::Throttling(
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
impl fmt::Display for StartDetectorModelAnalysisError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartDetectorModelAnalysisError::InternalFailure(ref cause) => write!(f, "{}", cause),
            StartDetectorModelAnalysisError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StartDetectorModelAnalysisError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            StartDetectorModelAnalysisError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            StartDetectorModelAnalysisError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartDetectorModelAnalysisError {}
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
/// Errors returned by UpdateAlarmModel
#[derive(Debug, PartialEq)]
pub enum UpdateAlarmModelError {
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

impl UpdateAlarmModelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateAlarmModelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateAlarmModelError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateAlarmModelError::InvalidRequest(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(UpdateAlarmModelError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateAlarmModelError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateAlarmModelError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateAlarmModelError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateAlarmModelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateAlarmModelError::InternalFailure(ref cause) => write!(f, "{}", cause),
            UpdateAlarmModelError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateAlarmModelError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            UpdateAlarmModelError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateAlarmModelError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateAlarmModelError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateAlarmModelError {}
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
#[async_trait]
pub trait IotEvents {
    /// <p>Creates an alarm model to monitor an AWS IoT Events input attribute. You can use the alarm to get notified when the value is outside a specified range. For more information, see <a href="https://docs.aws.amazon.com/iotevents/latest/developerguide/create-alarms.html">Create an alarm model</a> in the <i>AWS IoT Events Developer Guide</i>.</p>
    async fn create_alarm_model(
        &self,
        input: CreateAlarmModelRequest,
    ) -> Result<CreateAlarmModelResponse, RusotoError<CreateAlarmModelError>>;

    /// <p>Creates a detector model.</p>
    async fn create_detector_model(
        &self,
        input: CreateDetectorModelRequest,
    ) -> Result<CreateDetectorModelResponse, RusotoError<CreateDetectorModelError>>;

    /// <p>Creates an input.</p>
    async fn create_input(
        &self,
        input: CreateInputRequest,
    ) -> Result<CreateInputResponse, RusotoError<CreateInputError>>;

    /// <p>Deletes an alarm model. Any alarm instances that were created based on this alarm model are also deleted. This action can't be undone.</p>
    async fn delete_alarm_model(
        &self,
        input: DeleteAlarmModelRequest,
    ) -> Result<DeleteAlarmModelResponse, RusotoError<DeleteAlarmModelError>>;

    /// <p>Deletes a detector model. Any active instances of the detector model are also deleted.</p>
    async fn delete_detector_model(
        &self,
        input: DeleteDetectorModelRequest,
    ) -> Result<DeleteDetectorModelResponse, RusotoError<DeleteDetectorModelError>>;

    /// <p>Deletes an input.</p>
    async fn delete_input(
        &self,
        input: DeleteInputRequest,
    ) -> Result<DeleteInputResponse, RusotoError<DeleteInputError>>;

    /// <p>Retrieves information about an alarm model. If you don't specify a value for the <code>alarmModelVersion</code> parameter, the latest version is returned.</p>
    async fn describe_alarm_model(
        &self,
        input: DescribeAlarmModelRequest,
    ) -> Result<DescribeAlarmModelResponse, RusotoError<DescribeAlarmModelError>>;

    /// <p>Describes a detector model. If the <code>version</code> parameter is not specified, information about the latest version is returned.</p>
    async fn describe_detector_model(
        &self,
        input: DescribeDetectorModelRequest,
    ) -> Result<DescribeDetectorModelResponse, RusotoError<DescribeDetectorModelError>>;

    /// <p><p>Retrieves runtime information about a detector model analysis.</p> <note> <p>After AWS IoT Events starts analyzing your detector model, you have up to 24 hours to retrieve the analysis results.</p> </note></p>
    async fn describe_detector_model_analysis(
        &self,
        input: DescribeDetectorModelAnalysisRequest,
    ) -> Result<
        DescribeDetectorModelAnalysisResponse,
        RusotoError<DescribeDetectorModelAnalysisError>,
    >;

    /// <p>Describes an input.</p>
    async fn describe_input(
        &self,
        input: DescribeInputRequest,
    ) -> Result<DescribeInputResponse, RusotoError<DescribeInputError>>;

    /// <p>Retrieves the current settings of the AWS IoT Events logging options.</p>
    async fn describe_logging_options(
        &self,
    ) -> Result<DescribeLoggingOptionsResponse, RusotoError<DescribeLoggingOptionsError>>;

    /// <p><p>Retrieves one or more analysis results of the detector model.</p> <note> <p>After AWS IoT Events starts analyzing your detector model, you have up to 24 hours to retrieve the analysis results.</p> </note></p>
    async fn get_detector_model_analysis_results(
        &self,
        input: GetDetectorModelAnalysisResultsRequest,
    ) -> Result<
        GetDetectorModelAnalysisResultsResponse,
        RusotoError<GetDetectorModelAnalysisResultsError>,
    >;

    /// <p>Lists all the versions of an alarm model. The operation returns only the metadata associated with each alarm model version.</p>
    async fn list_alarm_model_versions(
        &self,
        input: ListAlarmModelVersionsRequest,
    ) -> Result<ListAlarmModelVersionsResponse, RusotoError<ListAlarmModelVersionsError>>;

    /// <p>Lists the alarm models that you created. The operation returns only the metadata associated with each alarm model.</p>
    async fn list_alarm_models(
        &self,
        input: ListAlarmModelsRequest,
    ) -> Result<ListAlarmModelsResponse, RusotoError<ListAlarmModelsError>>;

    /// <p>Lists all the versions of a detector model. Only the metadata associated with each detector model version is returned.</p>
    async fn list_detector_model_versions(
        &self,
        input: ListDetectorModelVersionsRequest,
    ) -> Result<ListDetectorModelVersionsResponse, RusotoError<ListDetectorModelVersionsError>>;

    /// <p>Lists the detector models you have created. Only the metadata associated with each detector model is returned.</p>
    async fn list_detector_models(
        &self,
        input: ListDetectorModelsRequest,
    ) -> Result<ListDetectorModelsResponse, RusotoError<ListDetectorModelsError>>;

    /// <p> Lists one or more input routings. </p>
    async fn list_input_routings(
        &self,
        input: ListInputRoutingsRequest,
    ) -> Result<ListInputRoutingsResponse, RusotoError<ListInputRoutingsError>>;

    /// <p>Lists the inputs you have created.</p>
    async fn list_inputs(
        &self,
        input: ListInputsRequest,
    ) -> Result<ListInputsResponse, RusotoError<ListInputsError>>;

    /// <p>Lists the tags (metadata) you have assigned to the resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Sets or updates the AWS IoT Events logging options.</p> <p>If you update the value of any <code>loggingOptions</code> field, it takes up to one minute for the change to take effect. If you change the policy attached to the role you specified in the <code>roleArn</code> field (for example, to correct an invalid policy), it takes up to five minutes for that change to take effect.</p>
    async fn put_logging_options(
        &self,
        input: PutLoggingOptionsRequest,
    ) -> Result<(), RusotoError<PutLoggingOptionsError>>;

    /// <p>Performs an analysis of your detector model. For more information, see <a href="https://docs.aws.amazon.com/iotevents/latest/developerguide/iotevents-analyze-api.html">Troubleshooting a detector model</a> in the <i>AWS IoT Events Developer Guide</i>.</p>
    async fn start_detector_model_analysis(
        &self,
        input: StartDetectorModelAnalysisRequest,
    ) -> Result<StartDetectorModelAnalysisResponse, RusotoError<StartDetectorModelAnalysisError>>;

    /// <p>Adds to or modifies the tags of the given resource. Tags are metadata that can be used to manage a resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Removes the given tags (metadata) from the resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p>Updates an alarm model. Any alarms that were created based on the previous version are deleted and then created again as new data arrives.</p>
    async fn update_alarm_model(
        &self,
        input: UpdateAlarmModelRequest,
    ) -> Result<UpdateAlarmModelResponse, RusotoError<UpdateAlarmModelError>>;

    /// <p>Updates a detector model. Detectors (instances) spawned by the previous version are deleted and then re-created as new inputs arrive.</p>
    async fn update_detector_model(
        &self,
        input: UpdateDetectorModelRequest,
    ) -> Result<UpdateDetectorModelResponse, RusotoError<UpdateDetectorModelError>>;

    /// <p>Updates an input.</p>
    async fn update_input(
        &self,
        input: UpdateInputRequest,
    ) -> Result<UpdateInputResponse, RusotoError<UpdateInputError>>;
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

#[async_trait]
impl IotEvents for IotEventsClient {
    /// <p>Creates an alarm model to monitor an AWS IoT Events input attribute. You can use the alarm to get notified when the value is outside a specified range. For more information, see <a href="https://docs.aws.amazon.com/iotevents/latest/developerguide/create-alarms.html">Create an alarm model</a> in the <i>AWS IoT Events Developer Guide</i>.</p>
    #[allow(unused_mut)]
    async fn create_alarm_model(
        &self,
        input: CreateAlarmModelRequest,
    ) -> Result<CreateAlarmModelResponse, RusotoError<CreateAlarmModelError>> {
        let request_uri = "/alarm-models";

        let mut request = SignedRequest::new("POST", "iotevents", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateAlarmModelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateAlarmModelError::from_response(response))
        }
    }

    /// <p>Creates a detector model.</p>
    #[allow(unused_mut)]
    async fn create_detector_model(
        &self,
        input: CreateDetectorModelRequest,
    ) -> Result<CreateDetectorModelResponse, RusotoError<CreateDetectorModelError>> {
        let request_uri = "/detector-models";

        let mut request = SignedRequest::new("POST", "iotevents", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateDetectorModelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateDetectorModelError::from_response(response))
        }
    }

    /// <p>Creates an input.</p>
    #[allow(unused_mut)]
    async fn create_input(
        &self,
        input: CreateInputRequest,
    ) -> Result<CreateInputResponse, RusotoError<CreateInputError>> {
        let request_uri = "/inputs";

        let mut request = SignedRequest::new("POST", "iotevents", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateInputResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateInputError::from_response(response))
        }
    }

    /// <p>Deletes an alarm model. Any alarm instances that were created based on this alarm model are also deleted. This action can't be undone.</p>
    #[allow(unused_mut)]
    async fn delete_alarm_model(
        &self,
        input: DeleteAlarmModelRequest,
    ) -> Result<DeleteAlarmModelResponse, RusotoError<DeleteAlarmModelError>> {
        let request_uri = format!(
            "/alarm-models/{alarm_model_name}",
            alarm_model_name = input.alarm_model_name
        );

        let mut request = SignedRequest::new("DELETE", "iotevents", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteAlarmModelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteAlarmModelError::from_response(response))
        }
    }

    /// <p>Deletes a detector model. Any active instances of the detector model are also deleted.</p>
    #[allow(unused_mut)]
    async fn delete_detector_model(
        &self,
        input: DeleteDetectorModelRequest,
    ) -> Result<DeleteDetectorModelResponse, RusotoError<DeleteDetectorModelError>> {
        let request_uri = format!(
            "/detector-models/{detector_model_name}",
            detector_model_name = input.detector_model_name
        );

        let mut request = SignedRequest::new("DELETE", "iotevents", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteDetectorModelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteDetectorModelError::from_response(response))
        }
    }

    /// <p>Deletes an input.</p>
    #[allow(unused_mut)]
    async fn delete_input(
        &self,
        input: DeleteInputRequest,
    ) -> Result<DeleteInputResponse, RusotoError<DeleteInputError>> {
        let request_uri = format!("/inputs/{input_name}", input_name = input.input_name);

        let mut request = SignedRequest::new("DELETE", "iotevents", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteInputResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteInputError::from_response(response))
        }
    }

    /// <p>Retrieves information about an alarm model. If you don't specify a value for the <code>alarmModelVersion</code> parameter, the latest version is returned.</p>
    #[allow(unused_mut)]
    async fn describe_alarm_model(
        &self,
        input: DescribeAlarmModelRequest,
    ) -> Result<DescribeAlarmModelResponse, RusotoError<DescribeAlarmModelError>> {
        let request_uri = format!(
            "/alarm-models/{alarm_model_name}",
            alarm_model_name = input.alarm_model_name
        );

        let mut request = SignedRequest::new("GET", "iotevents", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.alarm_model_version {
            params.put("version", x);
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
                .deserialize::<DescribeAlarmModelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeAlarmModelError::from_response(response))
        }
    }

    /// <p>Describes a detector model. If the <code>version</code> parameter is not specified, information about the latest version is returned.</p>
    #[allow(unused_mut)]
    async fn describe_detector_model(
        &self,
        input: DescribeDetectorModelRequest,
    ) -> Result<DescribeDetectorModelResponse, RusotoError<DescribeDetectorModelError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeDetectorModelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeDetectorModelError::from_response(response))
        }
    }

    /// <p><p>Retrieves runtime information about a detector model analysis.</p> <note> <p>After AWS IoT Events starts analyzing your detector model, you have up to 24 hours to retrieve the analysis results.</p> </note></p>
    #[allow(unused_mut)]
    async fn describe_detector_model_analysis(
        &self,
        input: DescribeDetectorModelAnalysisRequest,
    ) -> Result<
        DescribeDetectorModelAnalysisResponse,
        RusotoError<DescribeDetectorModelAnalysisError>,
    > {
        let request_uri = format!(
            "/analysis/detector-models/{analysis_id}",
            analysis_id = input.analysis_id
        );

        let mut request = SignedRequest::new("GET", "iotevents", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeDetectorModelAnalysisResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeDetectorModelAnalysisError::from_response(response))
        }
    }

    /// <p>Describes an input.</p>
    #[allow(unused_mut)]
    async fn describe_input(
        &self,
        input: DescribeInputRequest,
    ) -> Result<DescribeInputResponse, RusotoError<DescribeInputError>> {
        let request_uri = format!("/inputs/{input_name}", input_name = input.input_name);

        let mut request = SignedRequest::new("GET", "iotevents", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeInputResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeInputError::from_response(response))
        }
    }

    /// <p>Retrieves the current settings of the AWS IoT Events logging options.</p>
    #[allow(unused_mut)]
    async fn describe_logging_options(
        &self,
    ) -> Result<DescribeLoggingOptionsResponse, RusotoError<DescribeLoggingOptionsError>> {
        let request_uri = "/logging";

        let mut request = SignedRequest::new("GET", "iotevents", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeLoggingOptionsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeLoggingOptionsError::from_response(response))
        }
    }

    /// <p><p>Retrieves one or more analysis results of the detector model.</p> <note> <p>After AWS IoT Events starts analyzing your detector model, you have up to 24 hours to retrieve the analysis results.</p> </note></p>
    #[allow(unused_mut)]
    async fn get_detector_model_analysis_results(
        &self,
        input: GetDetectorModelAnalysisResultsRequest,
    ) -> Result<
        GetDetectorModelAnalysisResultsResponse,
        RusotoError<GetDetectorModelAnalysisResultsError>,
    > {
        let request_uri = format!(
            "/analysis/detector-models/{analysis_id}/results",
            analysis_id = input.analysis_id
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetDetectorModelAnalysisResultsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetDetectorModelAnalysisResultsError::from_response(
                response,
            ))
        }
    }

    /// <p>Lists all the versions of an alarm model. The operation returns only the metadata associated with each alarm model version.</p>
    #[allow(unused_mut)]
    async fn list_alarm_model_versions(
        &self,
        input: ListAlarmModelVersionsRequest,
    ) -> Result<ListAlarmModelVersionsResponse, RusotoError<ListAlarmModelVersionsError>> {
        let request_uri = format!(
            "/alarm-models/{alarm_model_name}/versions",
            alarm_model_name = input.alarm_model_name
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListAlarmModelVersionsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListAlarmModelVersionsError::from_response(response))
        }
    }

    /// <p>Lists the alarm models that you created. The operation returns only the metadata associated with each alarm model.</p>
    #[allow(unused_mut)]
    async fn list_alarm_models(
        &self,
        input: ListAlarmModelsRequest,
    ) -> Result<ListAlarmModelsResponse, RusotoError<ListAlarmModelsError>> {
        let request_uri = "/alarm-models";

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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListAlarmModelsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListAlarmModelsError::from_response(response))
        }
    }

    /// <p>Lists all the versions of a detector model. Only the metadata associated with each detector model version is returned.</p>
    #[allow(unused_mut)]
    async fn list_detector_model_versions(
        &self,
        input: ListDetectorModelVersionsRequest,
    ) -> Result<ListDetectorModelVersionsResponse, RusotoError<ListDetectorModelVersionsError>>
    {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListDetectorModelVersionsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListDetectorModelVersionsError::from_response(response))
        }
    }

    /// <p>Lists the detector models you have created. Only the metadata associated with each detector model is returned.</p>
    #[allow(unused_mut)]
    async fn list_detector_models(
        &self,
        input: ListDetectorModelsRequest,
    ) -> Result<ListDetectorModelsResponse, RusotoError<ListDetectorModelsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListDetectorModelsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListDetectorModelsError::from_response(response))
        }
    }

    /// <p> Lists one or more input routings. </p>
    #[allow(unused_mut)]
    async fn list_input_routings(
        &self,
        input: ListInputRoutingsRequest,
    ) -> Result<ListInputRoutingsResponse, RusotoError<ListInputRoutingsError>> {
        let request_uri = "/input-routings";

        let mut request = SignedRequest::new("POST", "iotevents", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListInputRoutingsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListInputRoutingsError::from_response(response))
        }
    }

    /// <p>Lists the inputs you have created.</p>
    #[allow(unused_mut)]
    async fn list_inputs(
        &self,
        input: ListInputsRequest,
    ) -> Result<ListInputsResponse, RusotoError<ListInputsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListInputsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListInputsError::from_response(response))
        }
    }

    /// <p>Lists the tags (metadata) you have assigned to the resource.</p>
    #[allow(unused_mut)]
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let request_uri = "/tags";

        let mut request = SignedRequest::new("GET", "iotevents", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("resourceArn", &input.resource_arn);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTagsForResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p>Sets or updates the AWS IoT Events logging options.</p> <p>If you update the value of any <code>loggingOptions</code> field, it takes up to one minute for the change to take effect. If you change the policy attached to the role you specified in the <code>roleArn</code> field (for example, to correct an invalid policy), it takes up to five minutes for that change to take effect.</p>
    #[allow(unused_mut)]
    async fn put_logging_options(
        &self,
        input: PutLoggingOptionsRequest,
    ) -> Result<(), RusotoError<PutLoggingOptionsError>> {
        let request_uri = "/logging";

        let mut request = SignedRequest::new("PUT", "iotevents", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutLoggingOptionsError::from_response(response))
        }
    }

    /// <p>Performs an analysis of your detector model. For more information, see <a href="https://docs.aws.amazon.com/iotevents/latest/developerguide/iotevents-analyze-api.html">Troubleshooting a detector model</a> in the <i>AWS IoT Events Developer Guide</i>.</p>
    #[allow(unused_mut)]
    async fn start_detector_model_analysis(
        &self,
        input: StartDetectorModelAnalysisRequest,
    ) -> Result<StartDetectorModelAnalysisResponse, RusotoError<StartDetectorModelAnalysisError>>
    {
        let request_uri = "/analysis/detector-models/";

        let mut request = SignedRequest::new("POST", "iotevents", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<StartDetectorModelAnalysisResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StartDetectorModelAnalysisError::from_response(response))
        }
    }

    /// <p>Adds to or modifies the tags of the given resource. Tags are metadata that can be used to manage a resource.</p>
    #[allow(unused_mut)]
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let request_uri = "/tags";

        let mut request = SignedRequest::new("POST", "iotevents", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        params.put("resourceArn", &input.resource_arn);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<TagResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Removes the given tags (metadata) from the resource.</p>
    #[allow(unused_mut)]
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let request_uri = "/tags";

        let mut request = SignedRequest::new("DELETE", "iotevents", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("resourceArn", &input.resource_arn);
        for item in input.tag_keys.iter() {
            params.put("tagKeys", item);
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
                .deserialize::<UntagResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p>Updates an alarm model. Any alarms that were created based on the previous version are deleted and then created again as new data arrives.</p>
    #[allow(unused_mut)]
    async fn update_alarm_model(
        &self,
        input: UpdateAlarmModelRequest,
    ) -> Result<UpdateAlarmModelResponse, RusotoError<UpdateAlarmModelError>> {
        let request_uri = format!(
            "/alarm-models/{alarm_model_name}",
            alarm_model_name = input.alarm_model_name
        );

        let mut request = SignedRequest::new("POST", "iotevents", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateAlarmModelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateAlarmModelError::from_response(response))
        }
    }

    /// <p>Updates a detector model. Detectors (instances) spawned by the previous version are deleted and then re-created as new inputs arrive.</p>
    #[allow(unused_mut)]
    async fn update_detector_model(
        &self,
        input: UpdateDetectorModelRequest,
    ) -> Result<UpdateDetectorModelResponse, RusotoError<UpdateDetectorModelError>> {
        let request_uri = format!(
            "/detector-models/{detector_model_name}",
            detector_model_name = input.detector_model_name
        );

        let mut request = SignedRequest::new("POST", "iotevents", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateDetectorModelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateDetectorModelError::from_response(response))
        }
    }

    /// <p>Updates an input.</p>
    #[allow(unused_mut)]
    async fn update_input(
        &self,
        input: UpdateInputRequest,
    ) -> Result<UpdateInputResponse, RusotoError<UpdateInputError>> {
        let request_uri = format!("/inputs/{input_name}", input_name = input.input_name);

        let mut request = SignedRequest::new("PUT", "iotevents", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateInputResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateInputError::from_response(response))
        }
    }
}
