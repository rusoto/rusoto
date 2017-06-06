#[allow(warnings)]
use hyper::Client;
use hyper::status::StatusCode;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::region;

use std::fmt;
use std::error::Error;
use rusoto_core::request::HttpDispatchError;
use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};

use serde_json;
use rusoto_core::signature::SignedRequest;
use serde_json::Value as SerdeJsonValue;
use serde_json::from_str;
pub type ARN = String;
pub type AccountId = String;
pub type AllSupported = bool;
pub type AvailabilityZone = String;
pub type AwsRegion = String;
pub type Boolean = bool;
pub type ChannelName = String;
pub type ChronologicalOrder = String;
#[doc="<p>Indicates whether an AWS resource or AWS Config rule is compliant and provides the number of contributors that affect the compliance.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Compliance {
    #[doc="<p>The number of AWS resources or AWS Config rules that cause a result of <code>NON_COMPLIANT</code>, up to a maximum number.</p>"]
    #[serde(rename="ComplianceContributorCount")]
    pub compliance_contributor_count: Option<ComplianceContributorCount>,
    #[doc="<p>Indicates whether an AWS resource or AWS Config rule is compliant.</p> <p>A resource is compliant if it complies with all of the AWS Config rules that evaluate it, and it is noncompliant if it does not comply with one or more of these rules.</p> <p>A rule is compliant if all of the resources that the rule evaluates comply with it, and it is noncompliant if any of these resources do not comply.</p> <p>AWS Config returns the <code>INSUFFICIENT_DATA</code> value when no evaluation results are available for the AWS resource or Config rule.</p> <p>For the <code>Compliance</code> data type, AWS Config supports only <code>COMPLIANT</code>, <code>NON_COMPLIANT</code>, and <code>INSUFFICIENT_DATA</code> values. AWS Config does not support the <code>NOT_APPLICABLE</code> value for the <code>Compliance</code> data type.</p>"]
    #[serde(rename="ComplianceType")]
    pub compliance_type: Option<ComplianceType>,
}

#[doc="<p>Indicates whether an AWS Config rule is compliant. A rule is compliant if all of the resources that the rule evaluated comply with it, and it is noncompliant if any of these resources do not comply.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ComplianceByConfigRule {
    #[doc="<p>Indicates whether the AWS Config rule is compliant.</p>"]
    #[serde(rename="Compliance")]
    pub compliance: Option<Compliance>,
    #[doc="<p>The name of the AWS Config rule.</p>"]
    #[serde(rename="ConfigRuleName")]
    pub config_rule_name: Option<StringWithCharLimit64>,
}

pub type ComplianceByConfigRules = Vec<ComplianceByConfigRule>;
#[doc="<p>Indicates whether an AWS resource that is evaluated according to one or more AWS Config rules is compliant. A resource is compliant if it complies with all of the rules that evaluate it, and it is noncompliant if it does not comply with one or more of these rules.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ComplianceByResource {
    #[doc="<p>Indicates whether the AWS resource complies with all of the AWS Config rules that evaluated it.</p>"]
    #[serde(rename="Compliance")]
    pub compliance: Option<Compliance>,
    #[doc="<p>The ID of the AWS resource that was evaluated.</p>"]
    #[serde(rename="ResourceId")]
    pub resource_id: Option<StringWithCharLimit256>,
    #[doc="<p>The type of the AWS resource that was evaluated.</p>"]
    #[serde(rename="ResourceType")]
    pub resource_type: Option<StringWithCharLimit256>,
}

pub type ComplianceByResources = Vec<ComplianceByResource>;
#[doc="<p>The number of AWS resources or AWS Config rules responsible for the current compliance of the item, up to a maximum number.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ComplianceContributorCount {
    #[doc="<p>Indicates whether the maximum count is reached.</p>"]
    #[serde(rename="CapExceeded")]
    #[serde(skip_serializing_if="::std::option::Option::is_none")]
    pub cap_exceeded: Option<Boolean>,
    #[doc="<p>The number of AWS resources or AWS Config rules responsible for the current compliance of the item.</p>"]
    #[serde(rename="CappedCount")]
    pub capped_count: Option<Integer>,
}

pub type ComplianceResourceTypes = Vec<StringWithCharLimit256>;
pub type ComplianceSummariesByResourceType = Vec<ComplianceSummaryByResourceType>;
#[doc="<p>The number of AWS Config rules or AWS resources that are compliant and noncompliant.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ComplianceSummary {
    #[doc="<p>The time that AWS Config created the compliance summary.</p>"]
    #[serde(rename="ComplianceSummaryTimestamp")]
    pub compliance_summary_timestamp: Option<Date>,
    #[doc="<p>The number of AWS Config rules or AWS resources that are compliant, up to a maximum of 25 for rules and 100 for resources.</p>"]
    #[serde(rename="CompliantResourceCount")]
    pub compliant_resource_count: Option<ComplianceContributorCount>,
    #[doc="<p>The number of AWS Config rules or AWS resources that are noncompliant, up to a maximum of 25 for rules and 100 for resources.</p>"]
    #[serde(rename="NonCompliantResourceCount")]
    pub non_compliant_resource_count: Option<ComplianceContributorCount>,
}

#[doc="<p>The number of AWS resources of a specific type that are compliant or noncompliant, up to a maximum of 100 for each compliance.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ComplianceSummaryByResourceType {
    #[doc="<p>The number of AWS resources that are compliant or noncompliant, up to a maximum of 100 for each compliance.</p>"]
    #[serde(rename="ComplianceSummary")]
    pub compliance_summary: Option<ComplianceSummary>,
    #[doc="<p>The type of AWS resource.</p>"]
    #[serde(rename="ResourceType")]
    pub resource_type: Option<StringWithCharLimit256>,
}

pub type ComplianceType = String;
pub type ComplianceTypes = Vec<ComplianceType>;
#[doc="<p>A list that contains the status of the delivery of either the snapshot or the configuration history to the specified Amazon S3 bucket.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ConfigExportDeliveryInfo {
    #[doc="<p>The time of the last attempted delivery.</p>"]
    #[serde(rename="lastAttemptTime")]
    pub last_attempt_time: Option<Date>,
    #[doc="<p>The error code from the last attempted delivery.</p>"]
    #[serde(rename="lastErrorCode")]
    pub last_error_code: Option<String>,
    #[doc="<p>The error message from the last attempted delivery.</p>"]
    #[serde(rename="lastErrorMessage")]
    pub last_error_message: Option<String>,
    #[doc="<p>Status of the last attempted delivery.</p>"]
    #[serde(rename="lastStatus")]
    pub last_status: Option<DeliveryStatus>,
    #[doc="<p>The time of the last successful delivery.</p>"]
    #[serde(rename="lastSuccessfulTime")]
    pub last_successful_time: Option<Date>,
    #[doc="<p>The time that the next delivery occurs.</p>"]
    #[serde(rename="nextDeliveryTime")]
    pub next_delivery_time: Option<Date>,
}

#[doc="<p>An AWS Config rule represents an AWS Lambda function that you create for a custom rule or a predefined function for an AWS managed rule. The function evaluates configuration items to assess whether your AWS resources comply with your desired configurations. This function can run when AWS Config detects a configuration change to an AWS resource and at a periodic frequency that you choose (for example, every 24 hours).</p> <note> <p>You can use the AWS CLI and AWS SDKs if you want to create a rule that triggers evaluations for your resources when AWS Config delivers the configuration snapshot. For more information, see <a>ConfigSnapshotDeliveryProperties</a>.</p> </note> <p>For more information about developing and using AWS Config rules, see <a href=\"http://docs.aws.amazon.com/config/latest/developerguide/evaluate-config.html\">Evaluating AWS Resource Configurations with AWS Config</a> in the <i>AWS Config Developer Guide</i>.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct ConfigRule {
    #[doc="<p>The Amazon Resource Name (ARN) of the AWS Config rule.</p>"]
    #[serde(rename="ConfigRuleArn")]
    pub config_rule_arn: Option<String>,
    #[doc="<p>The ID of the AWS Config rule.</p>"]
    #[serde(rename="ConfigRuleId")]
    pub config_rule_id: Option<String>,
    #[doc="<p>The name that you assign to the AWS Config rule. The name is required if you are adding a new rule.</p>"]
    #[serde(rename="ConfigRuleName")]
    pub config_rule_name: Option<StringWithCharLimit64>,
    #[doc="<p>Indicates whether the AWS Config rule is active or is currently being deleted by AWS Config. It can also indicate the evaluation status for the Config rule.</p> <p>AWS Config sets the state of the rule to <code>EVALUATING</code> temporarily after you use the <code>StartConfigRulesEvaluation</code> request to evaluate your resources against the Config rule.</p> <p>AWS Config sets the state of the rule to <code>DELETING_RESULTS</code> temporarily after you use the <code>DeleteEvaluationResults</code> request to delete the current evaluation results for the Config rule.</p> <p>AWS Config sets the state of a rule to <code>DELETING</code> temporarily after you use the <code>DeleteConfigRule</code> request to delete the rule. After AWS Config deletes the rule, the rule and all of its evaluations are erased and are no longer available.</p>"]
    #[serde(rename="ConfigRuleState")]
    pub config_rule_state: Option<ConfigRuleState>,
    #[doc="<p>The description that you provide for the AWS Config rule.</p>"]
    #[serde(rename="Description")]
    pub description: Option<EmptiableStringWithCharLimit256>,
    #[doc="<p>A string in JSON format that is passed to the AWS Config rule Lambda function.</p>"]
    #[serde(rename="InputParameters")]
    pub input_parameters: Option<StringWithCharLimit1024>,
    #[doc="<p>The maximum frequency with which AWS Config runs evaluations for a rule. You can specify a value for <code>MaximumExecutionFrequency</code> when:</p> <ul> <li> <p>You are using an AWS managed rule that is triggered at a periodic frequency.</p> </li> <li> <p>Your custom rule is triggered when AWS Config delivers the configuration snapshot. For more information, see <a>ConfigSnapshotDeliveryProperties</a>.</p> </li> </ul> <note> <p>By default, rules with a periodic trigger are evaluated every 24 hours. To change the frequency, specify a valid value for the <code>MaximumExecutionFrequency</code> parameter.</p> </note>"]
    #[serde(rename="MaximumExecutionFrequency")]
    pub maximum_execution_frequency: Option<MaximumExecutionFrequency>,
    #[doc="<p>Defines which resources can trigger an evaluation for the rule. The scope can include one or more resource types, a combination of one resource type and one resource ID, or a combination of a tag key and value. Specify a scope to constrain the resources that can trigger an evaluation for the rule. If you do not specify a scope, evaluations are triggered when any resource in the recording group changes.</p>"]
    #[serde(rename="Scope")]
    pub scope: Option<Scope>,
    #[doc="<p>Provides the rule owner (AWS or customer), the rule identifier, and the notifications that cause the function to evaluate your AWS resources.</p>"]
    #[serde(rename="Source")]
    pub source: Source,
}

#[doc="<p>Status information for your AWS managed Config rules. The status includes information such as the last time the rule ran, the last time it failed, and the related error for the last failure.</p> <p>This action does not return status information about custom Config rules.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ConfigRuleEvaluationStatus {
    #[doc="<p>The Amazon Resource Name (ARN) of the AWS Config rule.</p>"]
    #[serde(rename="ConfigRuleArn")]
    pub config_rule_arn: Option<String>,
    #[doc="<p>The ID of the AWS Config rule.</p>"]
    #[serde(rename="ConfigRuleId")]
    pub config_rule_id: Option<String>,
    #[doc="<p>The name of the AWS Config rule.</p>"]
    #[serde(rename="ConfigRuleName")]
    pub config_rule_name: Option<StringWithCharLimit64>,
    #[doc="<p>The time that you first activated the AWS Config rule.</p>"]
    #[serde(rename="FirstActivatedTime")]
    pub first_activated_time: Option<Date>,
    #[doc="<p>Indicates whether AWS Config has evaluated your resources against the rule at least once.</p> <ul> <li> <p> <code>true</code> - AWS Config has evaluated your AWS resources against the rule at least once.</p> </li> <li> <p> <code>false</code> - AWS Config has not once finished evaluating your AWS resources against the rule.</p> </li> </ul>"]
    #[serde(rename="FirstEvaluationStarted")]
    #[serde(skip_serializing_if="::std::option::Option::is_none")]
    pub first_evaluation_started: Option<Boolean>,
    #[doc="<p>The error code that AWS Config returned when the rule last failed.</p>"]
    #[serde(rename="LastErrorCode")]
    pub last_error_code: Option<String>,
    #[doc="<p>The error message that AWS Config returned when the rule last failed.</p>"]
    #[serde(rename="LastErrorMessage")]
    pub last_error_message: Option<String>,
    #[doc="<p>The time that AWS Config last failed to evaluate your AWS resources against the rule.</p>"]
    #[serde(rename="LastFailedEvaluationTime")]
    pub last_failed_evaluation_time: Option<Date>,
    #[doc="<p>The time that AWS Config last failed to invoke the AWS Config rule to evaluate your AWS resources.</p>"]
    #[serde(rename="LastFailedInvocationTime")]
    pub last_failed_invocation_time: Option<Date>,
    #[doc="<p>The time that AWS Config last successfully evaluated your AWS resources against the rule.</p>"]
    #[serde(rename="LastSuccessfulEvaluationTime")]
    pub last_successful_evaluation_time: Option<Date>,
    #[doc="<p>The time that AWS Config last successfully invoked the AWS Config rule to evaluate your AWS resources.</p>"]
    #[serde(rename="LastSuccessfulInvocationTime")]
    pub last_successful_invocation_time: Option<Date>,
}

pub type ConfigRuleEvaluationStatusList = Vec<ConfigRuleEvaluationStatus>;
pub type ConfigRuleNames = Vec<StringWithCharLimit64>;
pub type ConfigRuleState = String;
pub type ConfigRules = Vec<ConfigRule>;
#[doc="<p>Provides options for how often AWS Config delivers configuration snapshots to the Amazon S3 bucket in your delivery channel.</p> <note> <p>If you want to create a rule that triggers evaluations for your resources when AWS Config delivers the configuration snapshot, see the following:</p> </note> <p>The frequency for a rule that triggers evaluations for your resources when AWS Config delivers the configuration snapshot is set by one of two values, depending on which is less frequent:</p> <ul> <li> <p>The value for the <code>deliveryFrequency</code> parameter within the delivery channel configuration, which sets how often AWS Config delivers configuration snapshots. This value also sets how often AWS Config invokes evaluations for Config rules.</p> </li> <li> <p>The value for the <code>MaximumExecutionFrequency</code> parameter, which sets the maximum frequency with which AWS Config invokes evaluations for the rule. For more information, see <a>ConfigRule</a>.</p> </li> </ul> <p>If the <code>deliveryFrequency</code> value is less frequent than the <code>MaximumExecutionFrequency</code> value for a rule, AWS Config invokes the rule only as often as the <code>deliveryFrequency</code> value.</p> <ol> <li> <p>For example, you want your rule to run evaluations when AWS Config delivers the configuration snapshot.</p> </li> <li> <p>You specify the <code>MaximumExecutionFrequency</code> value for <code>Six_Hours</code>. </p> </li> <li> <p>You then specify the delivery channel <code>deliveryFrequency</code> value for <code>TwentyFour_Hours</code>.</p> </li> <li> <p>Because the value for <code>deliveryFrequency</code> is less frequent than <code>MaximumExecutionFrequency</code>, AWS Config invokes evaluations for the rule every 24 hours. </p> </li> </ol> <p>You should set the <code>MaximumExecutionFrequency</code> value to be at least as frequent as the <code>deliveryFrequency</code> value. You can view the <code>deliveryFrequency</code> value by using the <code>DescribeDeliveryChannnels</code> action.</p> <p>To update the <code>deliveryFrequency</code> with which AWS Config delivers your configuration snapshots, use the <code>PutDeliveryChannel</code> action.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct ConfigSnapshotDeliveryProperties {
    #[doc="<p>The frequency with which AWS Config delivers configuration snapshots.</p>"]
    #[serde(rename="deliveryFrequency")]
    pub delivery_frequency: Option<MaximumExecutionFrequency>,
}

#[doc="<p>A list that contains the status of the delivery of the configuration stream notification to the Amazon SNS topic.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ConfigStreamDeliveryInfo {
    #[doc="<p>The error code from the last attempted delivery.</p>"]
    #[serde(rename="lastErrorCode")]
    pub last_error_code: Option<String>,
    #[doc="<p>The error message from the last attempted delivery.</p>"]
    #[serde(rename="lastErrorMessage")]
    pub last_error_message: Option<String>,
    #[doc="<p>Status of the last attempted delivery.</p> <p> <b>Note</b> Providing an SNS topic on a <a href=\"http://docs.aws.amazon.com/config/latest/APIReference/API_DeliveryChannel.html\">DeliveryChannel</a> for AWS Config is optional. If the SNS delivery is turned off, the last status will be <b>Not_Applicable</b>.</p>"]
    #[serde(rename="lastStatus")]
    pub last_status: Option<DeliveryStatus>,
    #[doc="<p>The time from the last status change.</p>"]
    #[serde(rename="lastStatusChangeTime")]
    pub last_status_change_time: Option<Date>,
}

pub type Configuration = String;
#[doc="<p>A list that contains detailed configurations of a specified resource.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ConfigurationItem {
    #[doc="<p>The 12 digit AWS account ID associated with the resource.</p>"]
    #[serde(rename="accountId")]
    pub account_id: Option<AccountId>,
    #[doc="<p>The Amazon Resource Name (ARN) of the resource.</p>"]
    #[serde(rename="arn")]
    pub arn: Option<ARN>,
    #[doc="<p>The Availability Zone associated with the resource.</p>"]
    #[serde(rename="availabilityZone")]
    pub availability_zone: Option<AvailabilityZone>,
    #[doc="<p>The region where the resource resides.</p>"]
    #[serde(rename="awsRegion")]
    pub aws_region: Option<AwsRegion>,
    #[doc="<p>The description of the resource configuration.</p>"]
    #[serde(rename="configuration")]
    pub configuration: Option<Configuration>,
    #[doc="<p>The time when the configuration recording was initiated.</p>"]
    #[serde(rename="configurationItemCaptureTime")]
    pub configuration_item_capture_time: Option<ConfigurationItemCaptureTime>,
    #[doc="<p>Unique MD5 hash that represents the configuration item's state.</p> <p>You can use MD5 hash to compare the states of two or more configuration items that are associated with the same resource.</p>"]
    #[serde(rename="configurationItemMD5Hash")]
    pub configuration_item_md5_hash: Option<ConfigurationItemMD5Hash>,
    #[doc="<p>The configuration item status.</p>"]
    #[serde(rename="configurationItemStatus")]
    pub configuration_item_status: Option<ConfigurationItemStatus>,
    #[doc="<p>An identifier that indicates the ordering of the configuration items of a resource.</p>"]
    #[serde(rename="configurationStateId")]
    pub configuration_state_id: Option<ConfigurationStateId>,
    #[doc="<p>A list of CloudTrail event IDs.</p> <p>A populated field indicates that the current configuration was initiated by the events recorded in the CloudTrail log. For more information about CloudTrail, see <a href=\"http://docs.aws.amazon.com/awscloudtrail/latest/userguide/what_is_cloud_trail_top_level.html\">What is AWS CloudTrail?</a>.</p> <p>An empty field indicates that the current configuration was not initiated by any event.</p>"]
    #[serde(rename="relatedEvents")]
    pub related_events: Option<RelatedEventList>,
    #[doc="<p>A list of related AWS resources.</p>"]
    #[serde(rename="relationships")]
    pub relationships: Option<RelationshipList>,
    #[doc="<p>The time stamp when the resource was created.</p>"]
    #[serde(rename="resourceCreationTime")]
    pub resource_creation_time: Option<ResourceCreationTime>,
    #[doc="<p>The ID of the resource (for example., <code>sg-xxxxxx</code>).</p>"]
    #[serde(rename="resourceId")]
    pub resource_id: Option<ResourceId>,
    #[doc="<p>The custom name of the resource, if available.</p>"]
    #[serde(rename="resourceName")]
    pub resource_name: Option<ResourceName>,
    #[doc="<p>The type of AWS resource.</p>"]
    #[serde(rename="resourceType")]
    pub resource_type: Option<ResourceType>,
    #[doc="<p>Configuration attributes that AWS Config returns for certain resource types to supplement the information returned for the <code>configuration</code> parameter.</p>"]
    #[serde(rename="supplementaryConfiguration")]
    pub supplementary_configuration: Option<SupplementaryConfiguration>,
    #[doc="<p>A mapping of key value tags associated with the resource.</p>"]
    #[serde(rename="tags")]
    pub tags: Option<Tags>,
    #[doc="<p>The version number of the resource configuration.</p>"]
    #[serde(rename="version")]
    pub version: Option<Version>,
}

pub type ConfigurationItemCaptureTime = f64;
pub type ConfigurationItemList = Vec<ConfigurationItem>;
pub type ConfigurationItemMD5Hash = String;
pub type ConfigurationItemStatus = String;
#[doc="<p>An object that represents the recording of configuration changes of an AWS resource.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct ConfigurationRecorder {
    #[doc="<p>The name of the recorder. By default, AWS Config automatically assigns the name \"default\" when creating the configuration recorder. You cannot change the assigned name.</p>"]
    #[serde(rename="name")]
    pub name: Option<RecorderName>,
    #[doc="<p>Specifies the types of AWS resource for which AWS Config records configuration changes.</p>"]
    #[serde(rename="recordingGroup")]
    pub recording_group: Option<RecordingGroup>,
    #[doc="<p>Amazon Resource Name (ARN) of the IAM role used to describe the AWS resources associated with the account.</p>"]
    #[serde(rename="roleARN")]
    pub role_arn: Option<String>,
}

pub type ConfigurationRecorderList = Vec<ConfigurationRecorder>;
pub type ConfigurationRecorderNameList = Vec<RecorderName>;
#[doc="<p>The current status of the configuration recorder.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ConfigurationRecorderStatus {
    #[doc="<p>The error code indicating that the recording failed.</p>"]
    #[serde(rename="lastErrorCode")]
    pub last_error_code: Option<String>,
    #[doc="<p>The message indicating that the recording failed due to an error.</p>"]
    #[serde(rename="lastErrorMessage")]
    pub last_error_message: Option<String>,
    #[doc="<p>The time the recorder was last started.</p>"]
    #[serde(rename="lastStartTime")]
    pub last_start_time: Option<Date>,
    #[doc="<p>The last (previous) status of the recorder.</p>"]
    #[serde(rename="lastStatus")]
    pub last_status: Option<RecorderStatus>,
    #[doc="<p>The time when the status was last changed.</p>"]
    #[serde(rename="lastStatusChangeTime")]
    pub last_status_change_time: Option<Date>,
    #[doc="<p>The time the recorder was last stopped.</p>"]
    #[serde(rename="lastStopTime")]
    pub last_stop_time: Option<Date>,
    #[doc="<p>The name of the configuration recorder.</p>"]
    #[serde(rename="name")]
    pub name: Option<String>,
    #[doc="<p>Specifies whether the recorder is currently recording or not.</p>"]
    #[serde(rename="recording")]
    #[serde(skip_serializing_if="::std::option::Option::is_none")]
    pub recording: Option<Boolean>,
}

pub type ConfigurationRecorderStatusList = Vec<ConfigurationRecorderStatus>;
pub type ConfigurationStateId = String;
pub type Date = f64;
#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteConfigRuleRequest {
    #[doc="<p>The name of the AWS Config rule that you want to delete.</p>"]
    #[serde(rename="ConfigRuleName")]
    pub config_rule_name: StringWithCharLimit64,
}

#[doc="<p>The request object for the <code>DeleteConfigurationRecorder</code> action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteConfigurationRecorderRequest {
    #[doc="<p>The name of the configuration recorder to be deleted. You can retrieve the name of your configuration recorder by using the <code>DescribeConfigurationRecorders</code> action.</p>"]
    #[serde(rename="ConfigurationRecorderName")]
    pub configuration_recorder_name: RecorderName,
}

#[doc="<p>The input for the <a>DeleteDeliveryChannel</a> action. The action accepts the following data in JSON format. </p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteDeliveryChannelRequest {
    #[doc="<p>The name of the delivery channel to delete.</p>"]
    #[serde(rename="DeliveryChannelName")]
    pub delivery_channel_name: ChannelName,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteEvaluationResultsRequest {
    #[doc="<p>The name of the Config rule for which you want to delete the evaluation results.</p>"]
    #[serde(rename="ConfigRuleName")]
    pub config_rule_name: StringWithCharLimit64,
}

#[doc="<p>The output when you delete the evaluation results for the specified Config rule.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteEvaluationResultsResponse;

#[doc="<p>The input for the <a>DeliverConfigSnapshot</a> action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeliverConfigSnapshotRequest {
    #[doc="<p>The name of the delivery channel through which the snapshot is delivered.</p>"]
    #[serde(rename="deliveryChannelName")]
    pub delivery_channel_name: ChannelName,
}

#[doc="<p>The output for the <a>DeliverConfigSnapshot</a> action in JSON format.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeliverConfigSnapshotResponse {
    #[doc="<p>The ID of the snapshot that is being created.</p>"]
    #[serde(rename="configSnapshotId")]
    pub config_snapshot_id: Option<String>,
}

#[doc="<p>The channel through which AWS Config delivers notifications and updated configuration states.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct DeliveryChannel {
    #[doc="<p>The options for how often AWS Config delivers configuration snapshots to the Amazon S3 bucket.</p>"]
    #[serde(rename="configSnapshotDeliveryProperties")]
    pub config_snapshot_delivery_properties: Option<ConfigSnapshotDeliveryProperties>,
    #[doc="<p>The name of the delivery channel. By default, AWS Config assigns the name \"default\" when creating the delivery channel. To change the delivery channel name, you must use the DeleteDeliveryChannel action to delete your current delivery channel, and then you must use the PutDeliveryChannel command to create a delivery channel that has the desired name.</p>"]
    #[serde(rename="name")]
    pub name: Option<ChannelName>,
    #[doc="<p>The name of the Amazon S3 bucket to which AWS Config delivers configuration snapshots and configuration history files.</p> <p>If you specify a bucket that belongs to another AWS account, that bucket must have policies that grant access permissions to AWS Config. For more information, see <a href=\"http://docs.aws.amazon.com/config/latest/developerguide/s3-bucket-policy.html\">Permissions for the Amazon S3 Bucket</a> in the AWS Config Developer Guide.</p>"]
    #[serde(rename="s3BucketName")]
    pub s_3_bucket_name: Option<String>,
    #[doc="<p>The prefix for the specified Amazon S3 bucket.</p>"]
    #[serde(rename="s3KeyPrefix")]
    pub s_3_key_prefix: Option<String>,
    #[doc="<p>The Amazon Resource Name (ARN) of the Amazon SNS topic to which AWS Config sends notifications about configuration changes.</p> <p>If you choose a topic from another account, the topic must have policies that grant access permissions to AWS Config. For more information, see <a href=\"http://docs.aws.amazon.com/config/latest/developerguide/sns-topic-policy.html\">Permissions for the Amazon SNS Topic</a> in the AWS Config Developer Guide.</p>"]
    #[serde(rename="snsTopicARN")]
    pub sns_topic_arn: Option<String>,
}

pub type DeliveryChannelList = Vec<DeliveryChannel>;
pub type DeliveryChannelNameList = Vec<ChannelName>;
#[doc="<p>The status of a specified delivery channel.</p> <p>Valid values: <code>Success</code> | <code>Failure</code> </p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeliveryChannelStatus {
    #[doc="<p>A list that contains the status of the delivery of the configuration history to the specified Amazon S3 bucket.</p>"]
    #[serde(rename="configHistoryDeliveryInfo")]
    pub config_history_delivery_info: Option<ConfigExportDeliveryInfo>,
    #[doc="<p>A list containing the status of the delivery of the snapshot to the specified Amazon S3 bucket.</p>"]
    #[serde(rename="configSnapshotDeliveryInfo")]
    pub config_snapshot_delivery_info: Option<ConfigExportDeliveryInfo>,
    #[doc="<p>A list containing the status of the delivery of the configuration stream notification to the specified Amazon SNS topic.</p>"]
    #[serde(rename="configStreamDeliveryInfo")]
    pub config_stream_delivery_info: Option<ConfigStreamDeliveryInfo>,
    #[doc="<p>The name of the delivery channel.</p>"]
    #[serde(rename="name")]
    pub name: Option<String>,
}

pub type DeliveryChannelStatusList = Vec<DeliveryChannelStatus>;
pub type DeliveryStatus = String;
#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeComplianceByConfigRuleRequest {
    #[doc="<p>Filters the results by compliance.</p> <p>The allowed values are <code>COMPLIANT</code>, <code>NON_COMPLIANT</code>, and <code>INSUFFICIENT_DATA</code>.</p>"]
    #[serde(rename="ComplianceTypes")]
    pub compliance_types: Option<ComplianceTypes>,
    #[doc="<p>Specify one or more AWS Config rule names to filter the results by rule.</p>"]
    #[serde(rename="ConfigRuleNames")]
    pub config_rule_names: Option<ConfigRuleNames>,
    #[doc="<p>The <code>NextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>"]
    #[serde(rename="NextToken")]
    pub next_token: Option<String>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeComplianceByConfigRuleResponse {
    #[doc="<p>Indicates whether each of the specified AWS Config rules is compliant.</p>"]
    #[serde(rename="ComplianceByConfigRules")]
    pub compliance_by_config_rules: Option<ComplianceByConfigRules>,
    #[doc="<p>The string that you use in a subsequent request to get the next page of results in a paginated response.</p>"]
    #[serde(rename="NextToken")]
    pub next_token: Option<String>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeComplianceByResourceRequest {
    #[doc="<p>Filters the results by compliance.</p> <p>The allowed values are <code>COMPLIANT</code>, <code>NON_COMPLIANT</code>, and <code>INSUFFICIENT_DATA</code>.</p>"]
    #[serde(rename="ComplianceTypes")]
    pub compliance_types: Option<ComplianceTypes>,
    #[doc="<p>The maximum number of evaluation results returned on each page. The default is 10. You cannot specify a limit greater than 100. If you specify 0, AWS Config uses the default.</p>"]
    #[serde(rename="Limit")]
    pub limit: Option<Limit>,
    #[doc="<p>The <code>NextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>"]
    #[serde(rename="NextToken")]
    pub next_token: Option<NextToken>,
    #[doc="<p>The ID of the AWS resource for which you want compliance information. You can specify only one resource ID. If you specify a resource ID, you must also specify a type for <code>ResourceType</code>.</p>"]
    #[serde(rename="ResourceId")]
    pub resource_id: Option<StringWithCharLimit256>,
    #[doc="<p>The types of AWS resources for which you want compliance information; for example, <code>AWS::EC2::Instance</code>. For this action, you can specify that the resource type is an AWS account by specifying <code>AWS::::Account</code>.</p>"]
    #[serde(rename="ResourceType")]
    pub resource_type: Option<StringWithCharLimit256>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeComplianceByResourceResponse {
    #[doc="<p>Indicates whether the specified AWS resource complies with all of the AWS Config rules that evaluate it.</p>"]
    #[serde(rename="ComplianceByResources")]
    pub compliance_by_resources: Option<ComplianceByResources>,
    #[doc="<p>The string that you use in a subsequent request to get the next page of results in a paginated response.</p>"]
    #[serde(rename="NextToken")]
    pub next_token: Option<NextToken>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeConfigRuleEvaluationStatusRequest {
    #[doc="<p>The name of the AWS managed Config rules for which you want status information. If you do not specify any names, AWS Config returns status information for all AWS managed Config rules that you use.</p>"]
    #[serde(rename="ConfigRuleNames")]
    pub config_rule_names: Option<ConfigRuleNames>,
    #[doc="<p>The number of rule evaluation results that you want returned.</p> <p>This parameter is required if the rule limit for your account is more than the default of 50 rules.</p> <p>For more information about requesting a rule limit increase, see <a href=\"http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_config\">AWS Config Limits</a> in the <i>AWS General Reference Guide</i>.</p>"]
    #[serde(rename="Limit")]
    pub limit: Option<RuleLimit>,
    #[doc="<p>The <code>NextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>"]
    #[serde(rename="NextToken")]
    pub next_token: Option<String>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeConfigRuleEvaluationStatusResponse {
    #[doc="<p>Status information about your AWS managed Config rules.</p>"]
    #[serde(rename="ConfigRulesEvaluationStatus")]
    pub config_rules_evaluation_status: Option<ConfigRuleEvaluationStatusList>,
    #[doc="<p>The string that you use in a subsequent request to get the next page of results in a paginated response.</p>"]
    #[serde(rename="NextToken")]
    pub next_token: Option<String>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeConfigRulesRequest {
    #[doc="<p>The names of the AWS Config rules for which you want details. If you do not specify any names, AWS Config returns details for all your rules.</p>"]
    #[serde(rename="ConfigRuleNames")]
    pub config_rule_names: Option<ConfigRuleNames>,
    #[doc="<p>The <code>NextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>"]
    #[serde(rename="NextToken")]
    pub next_token: Option<String>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeConfigRulesResponse {
    #[doc="<p>The details about your AWS Config rules.</p>"]
    #[serde(rename="ConfigRules")]
    pub config_rules: Option<ConfigRules>,
    #[doc="<p>The string that you use in a subsequent request to get the next page of results in a paginated response.</p>"]
    #[serde(rename="NextToken")]
    pub next_token: Option<String>,
}

#[doc="<p>The input for the <a>DescribeConfigurationRecorderStatus</a> action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeConfigurationRecorderStatusRequest {
    #[doc="<p>The name(s) of the configuration recorder. If the name is not specified, the action returns the current status of all the configuration recorders associated with the account.</p>"]
    #[serde(rename="ConfigurationRecorderNames")]
    pub configuration_recorder_names: Option<ConfigurationRecorderNameList>,
}

#[doc="<p>The output for the <a>DescribeConfigurationRecorderStatus</a> action in JSON format.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeConfigurationRecorderStatusResponse {
    #[doc="<p>A list that contains status of the specified recorders.</p>"]
    #[serde(rename="ConfigurationRecordersStatus")]
    pub configuration_recorders_status: Option<ConfigurationRecorderStatusList>,
}

#[doc="<p>The input for the <a>DescribeConfigurationRecorders</a> action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeConfigurationRecordersRequest {
    #[doc="<p>A list of configuration recorder names.</p>"]
    #[serde(rename="ConfigurationRecorderNames")]
    pub configuration_recorder_names: Option<ConfigurationRecorderNameList>,
}

#[doc="<p>The output for the <a>DescribeConfigurationRecorders</a> action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeConfigurationRecordersResponse {
    #[doc="<p>A list that contains the descriptions of the specified configuration recorders.</p>"]
    #[serde(rename="ConfigurationRecorders")]
    pub configuration_recorders: Option<ConfigurationRecorderList>,
}

#[doc="<p>The input for the <a>DeliveryChannelStatus</a> action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeDeliveryChannelStatusRequest {
    #[doc="<p>A list of delivery channel names.</p>"]
    #[serde(rename="DeliveryChannelNames")]
    pub delivery_channel_names: Option<DeliveryChannelNameList>,
}

#[doc="<p>The output for the <a>DescribeDeliveryChannelStatus</a> action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeDeliveryChannelStatusResponse {
    #[doc="<p>A list that contains the status of a specified delivery channel.</p>"]
    #[serde(rename="DeliveryChannelsStatus")]
    pub delivery_channels_status: Option<DeliveryChannelStatusList>,
}

#[doc="<p>The input for the <a>DescribeDeliveryChannels</a> action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeDeliveryChannelsRequest {
    #[doc="<p>A list of delivery channel names.</p>"]
    #[serde(rename="DeliveryChannelNames")]
    pub delivery_channel_names: Option<DeliveryChannelNameList>,
}

#[doc="<p>The output for the <a>DescribeDeliveryChannels</a> action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeDeliveryChannelsResponse {
    #[doc="<p>A list that contains the descriptions of the specified delivery channel.</p>"]
    #[serde(rename="DeliveryChannels")]
    pub delivery_channels: Option<DeliveryChannelList>,
}

pub type EarlierTime = f64;
pub type EmptiableStringWithCharLimit256 = String;
#[doc="<p>Identifies an AWS resource and indicates whether it complies with the AWS Config rule that it was evaluated against.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct Evaluation {
    #[doc="<p>Supplementary information about how the evaluation determined the compliance.</p>"]
    #[serde(rename="Annotation")]
    pub annotation: Option<StringWithCharLimit256>,
    #[doc="<p>The ID of the AWS resource that was evaluated.</p>"]
    #[serde(rename="ComplianceResourceId")]
    pub compliance_resource_id: StringWithCharLimit256,
    #[doc="<p>The type of AWS resource that was evaluated.</p>"]
    #[serde(rename="ComplianceResourceType")]
    pub compliance_resource_type: StringWithCharLimit256,
    #[doc="<p>Indicates whether the AWS resource complies with the AWS Config rule that it was evaluated against.</p> <p>For the <code>Evaluation</code> data type, AWS Config supports only the <code>COMPLIANT</code>, <code>NON_COMPLIANT</code>, and <code>NOT_APPLICABLE</code> values. AWS Config does not support the <code>INSUFFICIENT_DATA</code> value for this data type.</p> <p>Similarly, AWS Config does not accept <code>INSUFFICIENT_DATA</code> as the value for <code>ComplianceType</code> from a <code>PutEvaluations</code> request. For example, an AWS Lambda function for a custom Config rule cannot pass an <code>INSUFFICIENT_DATA</code> value to AWS Config.</p>"]
    #[serde(rename="ComplianceType")]
    pub compliance_type: ComplianceType,
    #[doc="<p>The time of the event in AWS Config that triggered the evaluation. For event-based evaluations, the time indicates when AWS Config created the configuration item that triggered the evaluation. For periodic evaluations, the time indicates when AWS Config triggered the evaluation at the frequency that you specified (for example, every 24 hours).</p>"]
    #[serde(rename="OrderingTimestamp")]
    pub ordering_timestamp: OrderingTimestamp,
}

#[doc="<p>The details of an AWS Config evaluation. Provides the AWS resource that was evaluated, the compliance of the resource, related timestamps, and supplementary information.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct EvaluationResult {
    #[doc="<p>Supplementary information about how the evaluation determined the compliance.</p>"]
    #[serde(rename="Annotation")]
    pub annotation: Option<StringWithCharLimit256>,
    #[doc="<p>Indicates whether the AWS resource complies with the AWS Config rule that evaluated it.</p> <p>For the <code>EvaluationResult</code> data type, AWS Config supports only the <code>COMPLIANT</code>, <code>NON_COMPLIANT</code>, and <code>NOT_APPLICABLE</code> values. AWS Config does not support the <code>INSUFFICIENT_DATA</code> value for the <code>EvaluationResult</code> data type.</p>"]
    #[serde(rename="ComplianceType")]
    pub compliance_type: Option<ComplianceType>,
    #[doc="<p>The time when the AWS Config rule evaluated the AWS resource.</p>"]
    #[serde(rename="ConfigRuleInvokedTime")]
    pub config_rule_invoked_time: Option<Date>,
    #[doc="<p>Uniquely identifies the evaluation result.</p>"]
    #[serde(rename="EvaluationResultIdentifier")]
    pub evaluation_result_identifier: Option<EvaluationResultIdentifier>,
    #[doc="<p>The time when AWS Config recorded the evaluation result.</p>"]
    #[serde(rename="ResultRecordedTime")]
    pub result_recorded_time: Option<Date>,
    #[doc="<p>An encrypted token that associates an evaluation with an AWS Config rule. The token identifies the rule, the AWS resource being evaluated, and the event that triggered the evaluation.</p>"]
    #[serde(rename="ResultToken")]
    pub result_token: Option<String>,
}

#[doc="<p>Uniquely identifies an evaluation result.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct EvaluationResultIdentifier {
    #[doc="<p>Identifies an AWS Config rule used to evaluate an AWS resource, and provides the type and ID of the evaluated resource.</p>"]
    #[serde(rename="EvaluationResultQualifier")]
    pub evaluation_result_qualifier: Option<EvaluationResultQualifier>,
    #[doc="<p>The time of the event that triggered the evaluation of your AWS resources. The time can indicate when AWS Config delivered a configuration item change notification, or it can indicate when AWS Config delivered the configuration snapshot, depending on which event triggered the evaluation.</p>"]
    #[serde(rename="OrderingTimestamp")]
    pub ordering_timestamp: Option<Date>,
}

#[doc="<p>Identifies an AWS Config rule that evaluated an AWS resource, and provides the type and ID of the resource that the rule evaluated.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct EvaluationResultQualifier {
    #[doc="<p>The name of the AWS Config rule that was used in the evaluation.</p>"]
    #[serde(rename="ConfigRuleName")]
    pub config_rule_name: Option<StringWithCharLimit64>,
    #[doc="<p>The ID of the evaluated AWS resource.</p>"]
    #[serde(rename="ResourceId")]
    pub resource_id: Option<StringWithCharLimit256>,
    #[doc="<p>The type of AWS resource that was evaluated.</p>"]
    #[serde(rename="ResourceType")]
    pub resource_type: Option<StringWithCharLimit256>,
}

pub type EvaluationResults = Vec<EvaluationResult>;
pub type Evaluations = Vec<Evaluation>;
pub type EventSource = String;
#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct GetComplianceDetailsByConfigRuleRequest {
    #[doc="<p>Filters the results by compliance.</p> <p>The allowed values are <code>COMPLIANT</code>, <code>NON_COMPLIANT</code>, and <code>NOT_APPLICABLE</code>.</p>"]
    #[serde(rename="ComplianceTypes")]
    pub compliance_types: Option<ComplianceTypes>,
    #[doc="<p>The name of the AWS Config rule for which you want compliance information.</p>"]
    #[serde(rename="ConfigRuleName")]
    pub config_rule_name: StringWithCharLimit64,
    #[doc="<p>The maximum number of evaluation results returned on each page. The default is 10. You cannot specify a limit greater than 100. If you specify 0, AWS Config uses the default.</p>"]
    #[serde(rename="Limit")]
    pub limit: Option<Limit>,
    #[doc="<p>The <code>NextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>"]
    #[serde(rename="NextToken")]
    pub next_token: Option<NextToken>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetComplianceDetailsByConfigRuleResponse {
    #[doc="<p>Indicates whether the AWS resource complies with the specified AWS Config rule.</p>"]
    #[serde(rename="EvaluationResults")]
    pub evaluation_results: Option<EvaluationResults>,
    #[doc="<p>The string that you use in a subsequent request to get the next page of results in a paginated response.</p>"]
    #[serde(rename="NextToken")]
    pub next_token: Option<NextToken>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct GetComplianceDetailsByResourceRequest {
    #[doc="<p>Filters the results by compliance.</p> <p>The allowed values are <code>COMPLIANT</code>, <code>NON_COMPLIANT</code>, and <code>NOT_APPLICABLE</code>.</p>"]
    #[serde(rename="ComplianceTypes")]
    pub compliance_types: Option<ComplianceTypes>,
    #[doc="<p>The <code>NextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>"]
    #[serde(rename="NextToken")]
    pub next_token: Option<String>,
    #[doc="<p>The ID of the AWS resource for which you want compliance information.</p>"]
    #[serde(rename="ResourceId")]
    pub resource_id: StringWithCharLimit256,
    #[doc="<p>The type of the AWS resource for which you want compliance information.</p>"]
    #[serde(rename="ResourceType")]
    pub resource_type: StringWithCharLimit256,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetComplianceDetailsByResourceResponse {
    #[doc="<p>Indicates whether the specified AWS resource complies each AWS Config rule.</p>"]
    #[serde(rename="EvaluationResults")]
    pub evaluation_results: Option<EvaluationResults>,
    #[doc="<p>The string that you use in a subsequent request to get the next page of results in a paginated response.</p>"]
    #[serde(rename="NextToken")]
    pub next_token: Option<String>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetComplianceSummaryByConfigRuleResponse {
    #[doc="<p>The number of AWS Config rules that are compliant and the number that are noncompliant, up to a maximum of 25 for each.</p>"]
    #[serde(rename="ComplianceSummary")]
    pub compliance_summary: Option<ComplianceSummary>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct GetComplianceSummaryByResourceTypeRequest {
    #[doc="<p>Specify one or more resource types to get the number of resources that are compliant and the number that are noncompliant for each resource type.</p> <p>For this request, you can specify an AWS resource type such as <code>AWS::EC2::Instance</code>, and you can specify that the resource type is an AWS account by specifying <code>AWS::::Account</code>.</p>"]
    #[serde(rename="ResourceTypes")]
    pub resource_types: Option<ResourceTypes>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetComplianceSummaryByResourceTypeResponse {
    #[doc="<p>The number of resources that are compliant and the number that are noncompliant. If one or more resource types were provided with the request, the numbers are returned for each resource type. The maximum number returned is 100.</p>"]
    #[serde(rename="ComplianceSummariesByResourceType")]
    pub compliance_summaries_by_resource_type: Option<ComplianceSummariesByResourceType>,
}

#[doc="<p>The input for the <a>GetResourceConfigHistory</a> action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct GetResourceConfigHistoryRequest {
    #[doc="<p>The chronological order for configuration items listed. By default the results are listed in reverse chronological order.</p>"]
    #[serde(rename="chronologicalOrder")]
    pub chronological_order: Option<ChronologicalOrder>,
    #[doc="<p>The time stamp that indicates an earlier time. If not specified, the action returns paginated results that contain configuration items that start from when the first configuration item was recorded.</p>"]
    #[serde(rename="earlierTime")]
    pub earlier_time: Option<EarlierTime>,
    #[doc="<p>The time stamp that indicates a later time. If not specified, current time is taken.</p>"]
    #[serde(rename="laterTime")]
    pub later_time: Option<LaterTime>,
    #[doc="<p>The maximum number of configuration items returned on each page. The default is 10. You cannot specify a limit greater than 100. If you specify 0, AWS Config uses the default.</p>"]
    #[serde(rename="limit")]
    pub limit: Option<Limit>,
    #[doc="<p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>"]
    #[serde(rename="nextToken")]
    pub next_token: Option<NextToken>,
    #[doc="<p>The ID of the resource (for example., <code>sg-xxxxxx</code>).</p>"]
    #[serde(rename="resourceId")]
    pub resource_id: ResourceId,
    #[doc="<p>The resource type.</p>"]
    #[serde(rename="resourceType")]
    pub resource_type: ResourceType,
}

#[doc="<p>The output for the <a>GetResourceConfigHistory</a> action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetResourceConfigHistoryResponse {
    #[doc="<p>A list that contains the configuration history of one or more resources.</p>"]
    #[serde(rename="configurationItems")]
    pub configuration_items: Option<ConfigurationItemList>,
    #[doc="<p>The string that you use in a subsequent request to get the next page of results in a paginated response.</p>"]
    #[serde(rename="nextToken")]
    pub next_token: Option<NextToken>,
}

pub type IncludeGlobalResourceTypes = bool;
pub type Integer = i64;
pub type LaterTime = f64;
pub type Limit = i64;
#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListDiscoveredResourcesRequest {
    #[doc="<p>Specifies whether AWS Config includes deleted resources in the results. By default, deleted resources are not included.</p>"]
    #[serde(rename="includeDeletedResources")]
    #[serde(skip_serializing_if="::std::option::Option::is_none")]
    pub include_deleted_resources: Option<Boolean>,
    #[doc="<p>The maximum number of resource identifiers returned on each page. The default is 100. You cannot specify a limit greater than 100. If you specify 0, AWS Config uses the default.</p>"]
    #[serde(rename="limit")]
    pub limit: Option<Limit>,
    #[doc="<p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>"]
    #[serde(rename="nextToken")]
    pub next_token: Option<NextToken>,
    #[doc="<p>The IDs of only those resources that you want AWS Config to list in the response. If you do not specify this parameter, AWS Config lists all resources of the specified type that it has discovered.</p>"]
    #[serde(rename="resourceIds")]
    pub resource_ids: Option<ResourceIdList>,
    #[doc="<p>The custom name of only those resources that you want AWS Config to list in the response. If you do not specify this parameter, AWS Config lists all resources of the specified type that it has discovered.</p>"]
    #[serde(rename="resourceName")]
    pub resource_name: Option<ResourceName>,
    #[doc="<p>The type of resources that you want AWS Config to list in the response.</p>"]
    #[serde(rename="resourceType")]
    pub resource_type: ResourceType,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListDiscoveredResourcesResponse {
    #[doc="<p>The string that you use in a subsequent request to get the next page of results in a paginated response.</p>"]
    #[serde(rename="nextToken")]
    pub next_token: Option<NextToken>,
    #[doc="<p>The details that identify a resource that is discovered by AWS Config, including the resource type, ID, and (if available) the custom resource name.</p>"]
    #[serde(rename="resourceIdentifiers")]
    pub resource_identifiers: Option<ResourceIdentifierList>,
}

pub type MaximumExecutionFrequency = String;
pub type MessageType = String;
pub type Name = String;
pub type NextToken = String;
pub type OrderingTimestamp = f64;
pub type Owner = String;
#[derive(Default,Debug,Clone,Serialize)]
pub struct PutConfigRuleRequest {
    #[doc="<p>The rule that you want to add to your account.</p>"]
    #[serde(rename="ConfigRule")]
    pub config_rule: ConfigRule,
}

#[doc="<p>The input for the <a>PutConfigurationRecorder</a> action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct PutConfigurationRecorderRequest {
    #[doc="<p>The configuration recorder object that records each configuration change made to the resources.</p>"]
    #[serde(rename="ConfigurationRecorder")]
    pub configuration_recorder: ConfigurationRecorder,
}

#[doc="<p>The input for the <a>PutDeliveryChannel</a> action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct PutDeliveryChannelRequest {
    #[doc="<p>The configuration delivery channel object that delivers the configuration information to an Amazon S3 bucket, and to an Amazon SNS topic.</p>"]
    #[serde(rename="DeliveryChannel")]
    pub delivery_channel: DeliveryChannel,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct PutEvaluationsRequest {
    #[doc="<p>The assessments that the AWS Lambda function performs. Each evaluation identifies an AWS resource and indicates whether it complies with the AWS Config rule that invokes the AWS Lambda function.</p>"]
    #[serde(rename="Evaluations")]
    pub evaluations: Option<Evaluations>,
    #[doc="<p>An encrypted token that associates an evaluation with an AWS Config rule. Identifies the rule and the event that triggered the evaluation</p>"]
    #[serde(rename="ResultToken")]
    pub result_token: String,
    #[doc="<p>Use this parameter to specify a test run for <code>PutEvaluations</code>. You can verify whether your AWS Lambda function will deliver evaluation results to AWS Config. No updates occur to your existing evaluations, and evaluation results are not sent to AWS Config.</p> <note> <p>When <code>TestMode</code> is <code>true</code>, <code>PutEvaluations</code> doesn't require a valid value for the <code>ResultToken</code> parameter, but the value cannot be null.</p> </note>"]
    #[serde(rename="TestMode")]
    #[serde(skip_serializing_if="::std::option::Option::is_none")]
    pub test_mode: Option<Boolean>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct PutEvaluationsResponse {
    #[doc="<p>Requests that failed because of a client or server error.</p>"]
    #[serde(rename="FailedEvaluations")]
    pub failed_evaluations: Option<Evaluations>,
}

pub type RecorderName = String;
pub type RecorderStatus = String;
#[doc="<p>Specifies the types of AWS resource for which AWS Config records configuration changes.</p> <p>In the recording group, you specify whether all supported types or specific types of resources are recorded.</p> <p>By default, AWS Config records configuration changes for all supported types of regional resources that AWS Config discovers in the region in which it is running. Regional resources are tied to a region and can be used only in that region. Examples of regional resources are EC2 instances and EBS volumes.</p> <p>You can also have AWS Config record configuration changes for supported types of global resources (for example, IAM resources). Global resources are not tied to an individual region and can be used in all regions.</p> <important> <p>The configuration details for any global resource are the same in all regions. If you customize AWS Config in multiple regions to record global resources, it will create multiple configuration items each time a global resource changes: one configuration item for each region. These configuration items will contain identical data. To prevent duplicate configuration items, you should consider customizing AWS Config in only one region to record global resources, unless you want the configuration items to be available in multiple regions.</p> </important> <p>If you don't want AWS Config to record all resources, you can specify which types of resources it will record with the <code>resourceTypes</code> parameter.</p> <p>For a list of supported resource types, see <a href=\"http://docs.aws.amazon.com/config/latest/developerguide/resource-config-reference.html#supported-resources\">Supported resource types</a>.</p> <p>For more information, see <a href=\"http://docs.aws.amazon.com/config/latest/developerguide/select-resources.html\">Selecting Which Resources AWS Config Records</a>.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct RecordingGroup {
    #[doc="<p>Specifies whether AWS Config records configuration changes for every supported type of regional resource.</p> <p>If you set this option to <code>true</code>, when AWS Config adds support for a new type of regional resource, it automatically starts recording resources of that type.</p> <p>If you set this option to <code>true</code>, you cannot enumerate a list of <code>resourceTypes</code>.</p>"]
    #[serde(rename="allSupported")]
    #[serde(skip_serializing_if="::std::option::Option::is_none")]
    pub all_supported: Option<AllSupported>,
    #[doc="<p>Specifies whether AWS Config includes all supported types of global resources (for example, IAM resources) with the resources that it records.</p> <p>Before you can set this option to <code>true</code>, you must set the <code>allSupported</code> option to <code>true</code>.</p> <p>If you set this option to <code>true</code>, when AWS Config adds support for a new type of global resource, it automatically starts recording resources of that type.</p> <p>The configuration details for any global resource are the same in all regions. To prevent duplicate configuration items, you should consider customizing AWS Config in only one region to record global resources.</p>"]
    #[serde(rename="includeGlobalResourceTypes")]
    #[serde(skip_serializing_if="::std::option::Option::is_none")]
    pub include_global_resource_types: Option<IncludeGlobalResourceTypes>,
    #[doc="<p>A comma-separated list that specifies the types of AWS resources for which AWS Config records configuration changes (for example, <code>AWS::EC2::Instance</code> or <code>AWS::CloudTrail::Trail</code>).</p> <p>Before you can set this option to <code>true</code>, you must set the <code>allSupported</code> option to <code>false</code>.</p> <p>If you set this option to <code>true</code>, when AWS Config adds support for a new type of resource, it will not record resources of that type unless you manually add that type to your recording group.</p> <p>For a list of valid <code>resourceTypes</code> values, see the <b>resourceType Value</b> column in <a href=\"http://docs.aws.amazon.com/config/latest/developerguide/resource-config-reference.html#supported-resources\">Supported AWS Resource Types</a>.</p>"]
    #[serde(rename="resourceTypes")]
    pub resource_types: Option<ResourceTypeList>,
}

pub type ReevaluateConfigRuleNames = Vec<StringWithCharLimit64>;
pub type RelatedEvent = String;
pub type RelatedEventList = Vec<RelatedEvent>;
#[doc="<p>The relationship of the related resource to the main resource.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Relationship {
    #[doc="<p>The type of relationship with the related resource.</p>"]
    #[serde(rename="relationshipName")]
    pub relationship_name: Option<RelationshipName>,
    #[doc="<p>The ID of the related resource (for example, <code>sg-xxxxxx</code>).</p>"]
    #[serde(rename="resourceId")]
    pub resource_id: Option<ResourceId>,
    #[doc="<p>The custom name of the related resource, if available.</p>"]
    #[serde(rename="resourceName")]
    pub resource_name: Option<ResourceName>,
    #[doc="<p>The resource type of the related resource.</p>"]
    #[serde(rename="resourceType")]
    pub resource_type: Option<ResourceType>,
}

pub type RelationshipList = Vec<Relationship>;
pub type RelationshipName = String;
pub type ResourceCreationTime = f64;
pub type ResourceDeletionTime = f64;
pub type ResourceId = String;
pub type ResourceIdList = Vec<ResourceId>;
#[doc="<p>The details that identify a resource that is discovered by AWS Config, including the resource type, ID, and (if available) the custom resource name.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ResourceIdentifier {
    #[doc="<p>The time that the resource was deleted.</p>"]
    #[serde(rename="resourceDeletionTime")]
    pub resource_deletion_time: Option<ResourceDeletionTime>,
    #[doc="<p>The ID of the resource (for example., <code>sg-xxxxxx</code>).</p>"]
    #[serde(rename="resourceId")]
    pub resource_id: Option<ResourceId>,
    #[doc="<p>The custom name of the resource (if available).</p>"]
    #[serde(rename="resourceName")]
    pub resource_name: Option<ResourceName>,
    #[doc="<p>The type of resource.</p>"]
    #[serde(rename="resourceType")]
    pub resource_type: Option<ResourceType>,
}

pub type ResourceIdentifierList = Vec<ResourceIdentifier>;
pub type ResourceName = String;
pub type ResourceType = String;
pub type ResourceTypeList = Vec<ResourceType>;
pub type ResourceTypes = Vec<StringWithCharLimit256>;
pub type RuleLimit = i64;
#[doc="<p>Defines which resources trigger an evaluation for an AWS Config rule. The scope can include one or more resource types, a combination of a tag key and value, or a combination of one resource type and one resource ID. Specify a scope to constrain which resources trigger an evaluation for a rule. Otherwise, evaluations for the rule are triggered when any resource in your recording group changes in configuration.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct Scope {
    #[doc="<p>The IDs of the only AWS resource that you want to trigger an evaluation for the rule. If you specify a resource ID, you must specify one resource type for <code>ComplianceResourceTypes</code>.</p>"]
    #[serde(rename="ComplianceResourceId")]
    pub compliance_resource_id: Option<StringWithCharLimit256>,
    #[doc="<p>The resource types of only those AWS resources that you want to trigger an evaluation for the rule. You can only specify one type if you also specify a resource ID for <code>ComplianceResourceId</code>.</p>"]
    #[serde(rename="ComplianceResourceTypes")]
    pub compliance_resource_types: Option<ComplianceResourceTypes>,
    #[doc="<p>The tag key that is applied to only those AWS resources that you want you want to trigger an evaluation for the rule.</p>"]
    #[serde(rename="TagKey")]
    pub tag_key: Option<StringWithCharLimit128>,
    #[doc="<p>The tag value applied to only those AWS resources that you want to trigger an evaluation for the rule. If you specify a value for <code>TagValue</code>, you must also specify a value for <code>TagKey</code>.</p>"]
    #[serde(rename="TagValue")]
    pub tag_value: Option<StringWithCharLimit256>,
}

#[doc="<p>Provides the AWS Config rule owner (AWS or customer), the rule identifier, and the events that trigger the evaluation of your AWS resources.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct Source {
    #[doc="<p>Indicates whether AWS or the customer owns and manages the AWS Config rule.</p>"]
    #[serde(rename="Owner")]
    pub owner: Owner,
    #[doc="<p>Provides the source and type of the event that causes AWS Config to evaluate your AWS resources.</p>"]
    #[serde(rename="SourceDetails")]
    pub source_details: Option<SourceDetails>,
    #[doc="<p>For AWS Config managed rules, a predefined identifier from a list. For example, <code>IAM_PASSWORD_POLICY</code> is a managed rule. To reference a managed rule, see <a href=\"http://docs.aws.amazon.com/config/latest/developerguide/evaluate-config_use-managed-rules.html\">Using AWS Managed Config Rules</a>.</p> <p>For custom rules, the identifier is the Amazon Resource Name (ARN) of the rule's AWS Lambda function, such as <code>arn:aws:lambda:us-east-1:123456789012:function:custom_rule_name</code>.</p>"]
    #[serde(rename="SourceIdentifier")]
    pub source_identifier: StringWithCharLimit256,
}

#[doc="<p>Provides the source and the message types that trigger AWS Config to evaluate your AWS resources against a rule. It also provides the frequency with which you want AWS Config to run evaluations for the rule if the trigger type is periodic. You can specify the parameter values for <code>SourceDetail</code> only for custom rules. </p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct SourceDetail {
    #[doc="<p>The source of the event, such as an AWS service, that triggers AWS Config to evaluate your AWS resources.</p>"]
    #[serde(rename="EventSource")]
    pub event_source: Option<EventSource>,
    #[doc="<p>The frequency that you want AWS Config to run evaluations for a custom rule with a periodic trigger. If you specify a value for <code>MaximumExecutionFrequency</code>, then <code>MessageType</code> must use the <code>ScheduledNotification</code> value.</p> <note> <p>By default, rules with a periodic trigger are evaluated every 24 hours. To change the frequency, specify a valid value for the <code>MaximumExecutionFrequency</code> parameter.</p> </note>"]
    #[serde(rename="MaximumExecutionFrequency")]
    pub maximum_execution_frequency: Option<MaximumExecutionFrequency>,
    #[doc="<p>The type of notification that triggers AWS Config to run an evaluation for a rule. You can specify the following notification types:</p> <ul> <li> <p> <code>ConfigurationItemChangeNotification</code> - Triggers an evaluation when AWS Config delivers a configuration item as a result of a resource change.</p> </li> <li> <p> <code>OversizedConfigurationItemChangeNotification</code> - Triggers an evaluation when AWS Config delivers an oversized configuration item. AWS Config may generate this notification type when a resource changes and the notification exceeds the maximum size allowed by Amazon SNS.</p> </li> <li> <p> <code>ScheduledNotification</code> - Triggers a periodic evaluation at the frequency specified for <code>MaximumExecutionFrequency</code>.</p> </li> <li> <p> <code>ConfigurationSnapshotDeliveryCompleted</code> - Triggers a periodic evaluation when AWS Config delivers a configuration snapshot.</p> </li> </ul> <p>If you want your custom rule to be triggered by configuration changes, specify both <code>ConfigurationItemChangeNotification</code> and <code>OversizedConfigurationItemChangeNotification</code>. </p>"]
    #[serde(rename="MessageType")]
    pub message_type: Option<MessageType>,
}

pub type SourceDetails = Vec<SourceDetail>;
#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct StartConfigRulesEvaluationRequest {
    #[doc="<p>The list of names of Config rules that you want to run evaluations for.</p>"]
    #[serde(rename="ConfigRuleNames")]
    pub config_rule_names: Option<ReevaluateConfigRuleNames>,
}

#[doc="<p>The output when you start the evaluation for the specified Config rule.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct StartConfigRulesEvaluationResponse;

#[doc="<p>The input for the <a>StartConfigurationRecorder</a> action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct StartConfigurationRecorderRequest {
    #[doc="<p>The name of the recorder object that records each configuration change made to the resources.</p>"]
    #[serde(rename="ConfigurationRecorderName")]
    pub configuration_recorder_name: RecorderName,
}

#[doc="<p>The input for the <a>StopConfigurationRecorder</a> action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct StopConfigurationRecorderRequest {
    #[doc="<p>The name of the recorder object that records each configuration change made to the resources.</p>"]
    #[serde(rename="ConfigurationRecorderName")]
    pub configuration_recorder_name: RecorderName,
}

pub type StringWithCharLimit1024 = String;
pub type StringWithCharLimit128 = String;
pub type StringWithCharLimit256 = String;
pub type StringWithCharLimit64 = String;
pub type SupplementaryConfiguration = ::std::collections::HashMap<SupplementaryConfigurationName,
                                                                  SupplementaryConfigurationValue>;
pub type SupplementaryConfigurationName = String;
pub type SupplementaryConfigurationValue = String;
pub type Tags = ::std::collections::HashMap<Name, Value>;
pub type Value = String;
pub type Version = String;
/// Errors returned by DeleteConfigRule
#[derive(Debug, PartialEq)]
pub enum DeleteConfigRuleError {
    ///<p>One or more AWS Config rules in the request are invalid. Verify that the rule names are correct and try again.</p>
    NoSuchConfigRule(String),
    ///<p>The rule is currently being deleted or the rule is deleting your evaluation results. Try your request again later.</p>
    ResourceInUse(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteConfigRuleError {
    pub fn from_body(body: &str) -> DeleteConfigRuleError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "NoSuchConfigRuleException" => {
                        DeleteConfigRuleError::NoSuchConfigRule(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        DeleteConfigRuleError::ResourceInUse(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteConfigRuleError::Validation(error_message.to_string())
                    }
                    _ => DeleteConfigRuleError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteConfigRuleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteConfigRuleError {
    fn from(err: serde_json::error::Error) -> DeleteConfigRuleError {
        DeleteConfigRuleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteConfigRuleError {
    fn from(err: CredentialsError) -> DeleteConfigRuleError {
        DeleteConfigRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteConfigRuleError {
    fn from(err: HttpDispatchError) -> DeleteConfigRuleError {
        DeleteConfigRuleError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteConfigRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteConfigRuleError {
    fn description(&self) -> &str {
        match *self {
            DeleteConfigRuleError::NoSuchConfigRule(ref cause) => cause,
            DeleteConfigRuleError::ResourceInUse(ref cause) => cause,
            DeleteConfigRuleError::Validation(ref cause) => cause,
            DeleteConfigRuleError::Credentials(ref err) => err.description(),
            DeleteConfigRuleError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteConfigRuleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteConfigurationRecorder
#[derive(Debug, PartialEq)]
pub enum DeleteConfigurationRecorderError {
    ///<p>You have specified a configuration recorder that does not exist.</p>
    NoSuchConfigurationRecorder(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteConfigurationRecorderError {
    pub fn from_body(body: &str) -> DeleteConfigurationRecorderError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "NoSuchConfigurationRecorderException" => DeleteConfigurationRecorderError::NoSuchConfigurationRecorder(String::from(error_message)),
                    "ValidationException" => {
                        DeleteConfigurationRecorderError::Validation(error_message.to_string())
                    }
                    _ => DeleteConfigurationRecorderError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteConfigurationRecorderError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteConfigurationRecorderError {
    fn from(err: serde_json::error::Error) -> DeleteConfigurationRecorderError {
        DeleteConfigurationRecorderError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteConfigurationRecorderError {
    fn from(err: CredentialsError) -> DeleteConfigurationRecorderError {
        DeleteConfigurationRecorderError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteConfigurationRecorderError {
    fn from(err: HttpDispatchError) -> DeleteConfigurationRecorderError {
        DeleteConfigurationRecorderError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteConfigurationRecorderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteConfigurationRecorderError {
    fn description(&self) -> &str {
        match *self {
            DeleteConfigurationRecorderError::NoSuchConfigurationRecorder(ref cause) => cause,
            DeleteConfigurationRecorderError::Validation(ref cause) => cause,
            DeleteConfigurationRecorderError::Credentials(ref err) => err.description(),
            DeleteConfigurationRecorderError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteConfigurationRecorderError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDeliveryChannel
#[derive(Debug, PartialEq)]
pub enum DeleteDeliveryChannelError {
    ///<p>You cannot delete the delivery channel you specified because the configuration recorder is running.</p>
    LastDeliveryChannelDeleteFailed(String),
    ///<p>You have specified a delivery channel that does not exist.</p>
    NoSuchDeliveryChannel(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteDeliveryChannelError {
    pub fn from_body(body: &str) -> DeleteDeliveryChannelError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "LastDeliveryChannelDeleteFailedException" => DeleteDeliveryChannelError::LastDeliveryChannelDeleteFailed(String::from(error_message)),
                    "NoSuchDeliveryChannelException" => DeleteDeliveryChannelError::NoSuchDeliveryChannel(String::from(error_message)),
                    "ValidationException" => {
                        DeleteDeliveryChannelError::Validation(error_message.to_string())
                    }
                    _ => DeleteDeliveryChannelError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteDeliveryChannelError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteDeliveryChannelError {
    fn from(err: serde_json::error::Error) -> DeleteDeliveryChannelError {
        DeleteDeliveryChannelError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteDeliveryChannelError {
    fn from(err: CredentialsError) -> DeleteDeliveryChannelError {
        DeleteDeliveryChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteDeliveryChannelError {
    fn from(err: HttpDispatchError) -> DeleteDeliveryChannelError {
        DeleteDeliveryChannelError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteDeliveryChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDeliveryChannelError {
    fn description(&self) -> &str {
        match *self {
            DeleteDeliveryChannelError::LastDeliveryChannelDeleteFailed(ref cause) => cause,
            DeleteDeliveryChannelError::NoSuchDeliveryChannel(ref cause) => cause,
            DeleteDeliveryChannelError::Validation(ref cause) => cause,
            DeleteDeliveryChannelError::Credentials(ref err) => err.description(),
            DeleteDeliveryChannelError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteDeliveryChannelError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteEvaluationResults
#[derive(Debug, PartialEq)]
pub enum DeleteEvaluationResultsError {
    ///<p>One or more AWS Config rules in the request are invalid. Verify that the rule names are correct and try again.</p>
    NoSuchConfigRule(String),
    ///<p>The rule is currently being deleted or the rule is deleting your evaluation results. Try your request again later.</p>
    ResourceInUse(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteEvaluationResultsError {
    pub fn from_body(body: &str) -> DeleteEvaluationResultsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "NoSuchConfigRuleException" => {
                        DeleteEvaluationResultsError::NoSuchConfigRule(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        DeleteEvaluationResultsError::ResourceInUse(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteEvaluationResultsError::Validation(error_message.to_string())
                    }
                    _ => DeleteEvaluationResultsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteEvaluationResultsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteEvaluationResultsError {
    fn from(err: serde_json::error::Error) -> DeleteEvaluationResultsError {
        DeleteEvaluationResultsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteEvaluationResultsError {
    fn from(err: CredentialsError) -> DeleteEvaluationResultsError {
        DeleteEvaluationResultsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteEvaluationResultsError {
    fn from(err: HttpDispatchError) -> DeleteEvaluationResultsError {
        DeleteEvaluationResultsError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteEvaluationResultsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteEvaluationResultsError {
    fn description(&self) -> &str {
        match *self {
            DeleteEvaluationResultsError::NoSuchConfigRule(ref cause) => cause,
            DeleteEvaluationResultsError::ResourceInUse(ref cause) => cause,
            DeleteEvaluationResultsError::Validation(ref cause) => cause,
            DeleteEvaluationResultsError::Credentials(ref err) => err.description(),
            DeleteEvaluationResultsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteEvaluationResultsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeliverConfigSnapshot
#[derive(Debug, PartialEq)]
pub enum DeliverConfigSnapshotError {
    ///<p>There are no configuration recorders available to provide the role needed to describe your resources. Create a configuration recorder.</p>
    NoAvailableConfigurationRecorder(String),
    ///<p>There is no configuration recorder running.</p>
    NoRunningConfigurationRecorder(String),
    ///<p>You have specified a delivery channel that does not exist.</p>
    NoSuchDeliveryChannel(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeliverConfigSnapshotError {
    pub fn from_body(body: &str) -> DeliverConfigSnapshotError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "NoAvailableConfigurationRecorderException" => DeliverConfigSnapshotError::NoAvailableConfigurationRecorder(String::from(error_message)),
                    "NoRunningConfigurationRecorderException" => DeliverConfigSnapshotError::NoRunningConfigurationRecorder(String::from(error_message)),
                    "NoSuchDeliveryChannelException" => DeliverConfigSnapshotError::NoSuchDeliveryChannel(String::from(error_message)),
                    "ValidationException" => {
                        DeliverConfigSnapshotError::Validation(error_message.to_string())
                    }
                    _ => DeliverConfigSnapshotError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeliverConfigSnapshotError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeliverConfigSnapshotError {
    fn from(err: serde_json::error::Error) -> DeliverConfigSnapshotError {
        DeliverConfigSnapshotError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeliverConfigSnapshotError {
    fn from(err: CredentialsError) -> DeliverConfigSnapshotError {
        DeliverConfigSnapshotError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeliverConfigSnapshotError {
    fn from(err: HttpDispatchError) -> DeliverConfigSnapshotError {
        DeliverConfigSnapshotError::HttpDispatch(err)
    }
}
impl fmt::Display for DeliverConfigSnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeliverConfigSnapshotError {
    fn description(&self) -> &str {
        match *self {
            DeliverConfigSnapshotError::NoAvailableConfigurationRecorder(ref cause) => cause,
            DeliverConfigSnapshotError::NoRunningConfigurationRecorder(ref cause) => cause,
            DeliverConfigSnapshotError::NoSuchDeliveryChannel(ref cause) => cause,
            DeliverConfigSnapshotError::Validation(ref cause) => cause,
            DeliverConfigSnapshotError::Credentials(ref err) => err.description(),
            DeliverConfigSnapshotError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeliverConfigSnapshotError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeComplianceByConfigRule
#[derive(Debug, PartialEq)]
pub enum DescribeComplianceByConfigRuleError {
    ///<p>The specified next token is invalid. Specify the <code>NextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    ///<p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
    ///<p>One or more AWS Config rules in the request are invalid. Verify that the rule names are correct and try again.</p>
    NoSuchConfigRule(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeComplianceByConfigRuleError {
    pub fn from_body(body: &str) -> DescribeComplianceByConfigRuleError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidNextTokenException" => DescribeComplianceByConfigRuleError::InvalidNextToken(String::from(error_message)),
                    "InvalidParameterValueException" => DescribeComplianceByConfigRuleError::InvalidParameterValue(String::from(error_message)),
                    "NoSuchConfigRuleException" => DescribeComplianceByConfigRuleError::NoSuchConfigRule(String::from(error_message)),
                    "ValidationException" => {
                        DescribeComplianceByConfigRuleError::Validation(error_message.to_string())
                    }
                    _ => DescribeComplianceByConfigRuleError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeComplianceByConfigRuleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeComplianceByConfigRuleError {
    fn from(err: serde_json::error::Error) -> DescribeComplianceByConfigRuleError {
        DescribeComplianceByConfigRuleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeComplianceByConfigRuleError {
    fn from(err: CredentialsError) -> DescribeComplianceByConfigRuleError {
        DescribeComplianceByConfigRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeComplianceByConfigRuleError {
    fn from(err: HttpDispatchError) -> DescribeComplianceByConfigRuleError {
        DescribeComplianceByConfigRuleError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeComplianceByConfigRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeComplianceByConfigRuleError {
    fn description(&self) -> &str {
        match *self {
            DescribeComplianceByConfigRuleError::InvalidNextToken(ref cause) => cause,
            DescribeComplianceByConfigRuleError::InvalidParameterValue(ref cause) => cause,
            DescribeComplianceByConfigRuleError::NoSuchConfigRule(ref cause) => cause,
            DescribeComplianceByConfigRuleError::Validation(ref cause) => cause,
            DescribeComplianceByConfigRuleError::Credentials(ref err) => err.description(),
            DescribeComplianceByConfigRuleError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeComplianceByConfigRuleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeComplianceByResource
#[derive(Debug, PartialEq)]
pub enum DescribeComplianceByResourceError {
    ///<p>The specified next token is invalid. Specify the <code>NextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    ///<p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeComplianceByResourceError {
    pub fn from_body(body: &str) -> DescribeComplianceByResourceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidNextTokenException" => DescribeComplianceByResourceError::InvalidNextToken(String::from(error_message)),
                    "InvalidParameterValueException" => DescribeComplianceByResourceError::InvalidParameterValue(String::from(error_message)),
                    "ValidationException" => {
                        DescribeComplianceByResourceError::Validation(error_message.to_string())
                    }
                    _ => DescribeComplianceByResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeComplianceByResourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeComplianceByResourceError {
    fn from(err: serde_json::error::Error) -> DescribeComplianceByResourceError {
        DescribeComplianceByResourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeComplianceByResourceError {
    fn from(err: CredentialsError) -> DescribeComplianceByResourceError {
        DescribeComplianceByResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeComplianceByResourceError {
    fn from(err: HttpDispatchError) -> DescribeComplianceByResourceError {
        DescribeComplianceByResourceError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeComplianceByResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeComplianceByResourceError {
    fn description(&self) -> &str {
        match *self {
            DescribeComplianceByResourceError::InvalidNextToken(ref cause) => cause,
            DescribeComplianceByResourceError::InvalidParameterValue(ref cause) => cause,
            DescribeComplianceByResourceError::Validation(ref cause) => cause,
            DescribeComplianceByResourceError::Credentials(ref err) => err.description(),
            DescribeComplianceByResourceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeComplianceByResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeConfigRuleEvaluationStatus
#[derive(Debug, PartialEq)]
pub enum DescribeConfigRuleEvaluationStatusError {
    ///<p>The specified next token is invalid. Specify the <code>NextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    ///<p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
    ///<p>One or more AWS Config rules in the request are invalid. Verify that the rule names are correct and try again.</p>
    NoSuchConfigRule(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeConfigRuleEvaluationStatusError {
    pub fn from_body(body: &str) -> DescribeConfigRuleEvaluationStatusError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidNextTokenException" => DescribeConfigRuleEvaluationStatusError::InvalidNextToken(String::from(error_message)),
                    "InvalidParameterValueException" => DescribeConfigRuleEvaluationStatusError::InvalidParameterValue(String::from(error_message)),
                    "NoSuchConfigRuleException" => DescribeConfigRuleEvaluationStatusError::NoSuchConfigRule(String::from(error_message)),
                    "ValidationException" => {
                        DescribeConfigRuleEvaluationStatusError::Validation(error_message
                                                                                .to_string())
                    }
                    _ => DescribeConfigRuleEvaluationStatusError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeConfigRuleEvaluationStatusError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeConfigRuleEvaluationStatusError {
    fn from(err: serde_json::error::Error) -> DescribeConfigRuleEvaluationStatusError {
        DescribeConfigRuleEvaluationStatusError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeConfigRuleEvaluationStatusError {
    fn from(err: CredentialsError) -> DescribeConfigRuleEvaluationStatusError {
        DescribeConfigRuleEvaluationStatusError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeConfigRuleEvaluationStatusError {
    fn from(err: HttpDispatchError) -> DescribeConfigRuleEvaluationStatusError {
        DescribeConfigRuleEvaluationStatusError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeConfigRuleEvaluationStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeConfigRuleEvaluationStatusError {
    fn description(&self) -> &str {
        match *self {
            DescribeConfigRuleEvaluationStatusError::InvalidNextToken(ref cause) => cause,
            DescribeConfigRuleEvaluationStatusError::InvalidParameterValue(ref cause) => cause,
            DescribeConfigRuleEvaluationStatusError::NoSuchConfigRule(ref cause) => cause,
            DescribeConfigRuleEvaluationStatusError::Validation(ref cause) => cause,
            DescribeConfigRuleEvaluationStatusError::Credentials(ref err) => err.description(),
            DescribeConfigRuleEvaluationStatusError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeConfigRuleEvaluationStatusError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeConfigRules
#[derive(Debug, PartialEq)]
pub enum DescribeConfigRulesError {
    ///<p>The specified next token is invalid. Specify the <code>NextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    ///<p>One or more AWS Config rules in the request are invalid. Verify that the rule names are correct and try again.</p>
    NoSuchConfigRule(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeConfigRulesError {
    pub fn from_body(body: &str) -> DescribeConfigRulesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidNextTokenException" => {
                        DescribeConfigRulesError::InvalidNextToken(String::from(error_message))
                    }
                    "NoSuchConfigRuleException" => {
                        DescribeConfigRulesError::NoSuchConfigRule(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeConfigRulesError::Validation(error_message.to_string())
                    }
                    _ => DescribeConfigRulesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeConfigRulesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeConfigRulesError {
    fn from(err: serde_json::error::Error) -> DescribeConfigRulesError {
        DescribeConfigRulesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeConfigRulesError {
    fn from(err: CredentialsError) -> DescribeConfigRulesError {
        DescribeConfigRulesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeConfigRulesError {
    fn from(err: HttpDispatchError) -> DescribeConfigRulesError {
        DescribeConfigRulesError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeConfigRulesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeConfigRulesError {
    fn description(&self) -> &str {
        match *self {
            DescribeConfigRulesError::InvalidNextToken(ref cause) => cause,
            DescribeConfigRulesError::NoSuchConfigRule(ref cause) => cause,
            DescribeConfigRulesError::Validation(ref cause) => cause,
            DescribeConfigRulesError::Credentials(ref err) => err.description(),
            DescribeConfigRulesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeConfigRulesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeConfigurationRecorderStatus
#[derive(Debug, PartialEq)]
pub enum DescribeConfigurationRecorderStatusError {
    ///<p>You have specified a configuration recorder that does not exist.</p>
    NoSuchConfigurationRecorder(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeConfigurationRecorderStatusError {
    pub fn from_body(body: &str) -> DescribeConfigurationRecorderStatusError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "NoSuchConfigurationRecorderException" => DescribeConfigurationRecorderStatusError::NoSuchConfigurationRecorder(String::from(error_message)),
                    "ValidationException" => {
                        DescribeConfigurationRecorderStatusError::Validation(error_message
                                                                                 .to_string())
                    }
                    _ => DescribeConfigurationRecorderStatusError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeConfigurationRecorderStatusError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeConfigurationRecorderStatusError {
    fn from(err: serde_json::error::Error) -> DescribeConfigurationRecorderStatusError {
        DescribeConfigurationRecorderStatusError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeConfigurationRecorderStatusError {
    fn from(err: CredentialsError) -> DescribeConfigurationRecorderStatusError {
        DescribeConfigurationRecorderStatusError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeConfigurationRecorderStatusError {
    fn from(err: HttpDispatchError) -> DescribeConfigurationRecorderStatusError {
        DescribeConfigurationRecorderStatusError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeConfigurationRecorderStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeConfigurationRecorderStatusError {
    fn description(&self) -> &str {
        match *self {
            DescribeConfigurationRecorderStatusError::NoSuchConfigurationRecorder(ref cause) => {
                cause
            }
            DescribeConfigurationRecorderStatusError::Validation(ref cause) => cause,
            DescribeConfigurationRecorderStatusError::Credentials(ref err) => err.description(),
            DescribeConfigurationRecorderStatusError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeConfigurationRecorderStatusError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeConfigurationRecorders
#[derive(Debug, PartialEq)]
pub enum DescribeConfigurationRecordersError {
    ///<p>You have specified a configuration recorder that does not exist.</p>
    NoSuchConfigurationRecorder(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeConfigurationRecordersError {
    pub fn from_body(body: &str) -> DescribeConfigurationRecordersError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "NoSuchConfigurationRecorderException" => DescribeConfigurationRecordersError::NoSuchConfigurationRecorder(String::from(error_message)),
                    "ValidationException" => {
                        DescribeConfigurationRecordersError::Validation(error_message.to_string())
                    }
                    _ => DescribeConfigurationRecordersError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeConfigurationRecordersError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeConfigurationRecordersError {
    fn from(err: serde_json::error::Error) -> DescribeConfigurationRecordersError {
        DescribeConfigurationRecordersError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeConfigurationRecordersError {
    fn from(err: CredentialsError) -> DescribeConfigurationRecordersError {
        DescribeConfigurationRecordersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeConfigurationRecordersError {
    fn from(err: HttpDispatchError) -> DescribeConfigurationRecordersError {
        DescribeConfigurationRecordersError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeConfigurationRecordersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeConfigurationRecordersError {
    fn description(&self) -> &str {
        match *self {
            DescribeConfigurationRecordersError::NoSuchConfigurationRecorder(ref cause) => cause,
            DescribeConfigurationRecordersError::Validation(ref cause) => cause,
            DescribeConfigurationRecordersError::Credentials(ref err) => err.description(),
            DescribeConfigurationRecordersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeConfigurationRecordersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeDeliveryChannelStatus
#[derive(Debug, PartialEq)]
pub enum DescribeDeliveryChannelStatusError {
    ///<p>You have specified a delivery channel that does not exist.</p>
    NoSuchDeliveryChannel(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeDeliveryChannelStatusError {
    pub fn from_body(body: &str) -> DescribeDeliveryChannelStatusError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "NoSuchDeliveryChannelException" => DescribeDeliveryChannelStatusError::NoSuchDeliveryChannel(String::from(error_message)),
                    "ValidationException" => {
                        DescribeDeliveryChannelStatusError::Validation(error_message.to_string())
                    }
                    _ => DescribeDeliveryChannelStatusError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeDeliveryChannelStatusError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeDeliveryChannelStatusError {
    fn from(err: serde_json::error::Error) -> DescribeDeliveryChannelStatusError {
        DescribeDeliveryChannelStatusError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeDeliveryChannelStatusError {
    fn from(err: CredentialsError) -> DescribeDeliveryChannelStatusError {
        DescribeDeliveryChannelStatusError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeDeliveryChannelStatusError {
    fn from(err: HttpDispatchError) -> DescribeDeliveryChannelStatusError {
        DescribeDeliveryChannelStatusError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeDeliveryChannelStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDeliveryChannelStatusError {
    fn description(&self) -> &str {
        match *self {
            DescribeDeliveryChannelStatusError::NoSuchDeliveryChannel(ref cause) => cause,
            DescribeDeliveryChannelStatusError::Validation(ref cause) => cause,
            DescribeDeliveryChannelStatusError::Credentials(ref err) => err.description(),
            DescribeDeliveryChannelStatusError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeDeliveryChannelStatusError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeDeliveryChannels
#[derive(Debug, PartialEq)]
pub enum DescribeDeliveryChannelsError {
    ///<p>You have specified a delivery channel that does not exist.</p>
    NoSuchDeliveryChannel(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeDeliveryChannelsError {
    pub fn from_body(body: &str) -> DescribeDeliveryChannelsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "NoSuchDeliveryChannelException" => DescribeDeliveryChannelsError::NoSuchDeliveryChannel(String::from(error_message)),
                    "ValidationException" => {
                        DescribeDeliveryChannelsError::Validation(error_message.to_string())
                    }
                    _ => DescribeDeliveryChannelsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeDeliveryChannelsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeDeliveryChannelsError {
    fn from(err: serde_json::error::Error) -> DescribeDeliveryChannelsError {
        DescribeDeliveryChannelsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeDeliveryChannelsError {
    fn from(err: CredentialsError) -> DescribeDeliveryChannelsError {
        DescribeDeliveryChannelsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeDeliveryChannelsError {
    fn from(err: HttpDispatchError) -> DescribeDeliveryChannelsError {
        DescribeDeliveryChannelsError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeDeliveryChannelsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDeliveryChannelsError {
    fn description(&self) -> &str {
        match *self {
            DescribeDeliveryChannelsError::NoSuchDeliveryChannel(ref cause) => cause,
            DescribeDeliveryChannelsError::Validation(ref cause) => cause,
            DescribeDeliveryChannelsError::Credentials(ref err) => err.description(),
            DescribeDeliveryChannelsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeDeliveryChannelsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetComplianceDetailsByConfigRule
#[derive(Debug, PartialEq)]
pub enum GetComplianceDetailsByConfigRuleError {
    ///<p>The specified next token is invalid. Specify the <code>NextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    ///<p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
    ///<p>One or more AWS Config rules in the request are invalid. Verify that the rule names are correct and try again.</p>
    NoSuchConfigRule(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetComplianceDetailsByConfigRuleError {
    pub fn from_body(body: &str) -> GetComplianceDetailsByConfigRuleError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidNextTokenException" => GetComplianceDetailsByConfigRuleError::InvalidNextToken(String::from(error_message)),
                    "InvalidParameterValueException" => GetComplianceDetailsByConfigRuleError::InvalidParameterValue(String::from(error_message)),
                    "NoSuchConfigRuleException" => GetComplianceDetailsByConfigRuleError::NoSuchConfigRule(String::from(error_message)),
                    "ValidationException" => {
                        GetComplianceDetailsByConfigRuleError::Validation(error_message.to_string())
                    }
                    _ => GetComplianceDetailsByConfigRuleError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetComplianceDetailsByConfigRuleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetComplianceDetailsByConfigRuleError {
    fn from(err: serde_json::error::Error) -> GetComplianceDetailsByConfigRuleError {
        GetComplianceDetailsByConfigRuleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetComplianceDetailsByConfigRuleError {
    fn from(err: CredentialsError) -> GetComplianceDetailsByConfigRuleError {
        GetComplianceDetailsByConfigRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetComplianceDetailsByConfigRuleError {
    fn from(err: HttpDispatchError) -> GetComplianceDetailsByConfigRuleError {
        GetComplianceDetailsByConfigRuleError::HttpDispatch(err)
    }
}
impl fmt::Display for GetComplianceDetailsByConfigRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetComplianceDetailsByConfigRuleError {
    fn description(&self) -> &str {
        match *self {
            GetComplianceDetailsByConfigRuleError::InvalidNextToken(ref cause) => cause,
            GetComplianceDetailsByConfigRuleError::InvalidParameterValue(ref cause) => cause,
            GetComplianceDetailsByConfigRuleError::NoSuchConfigRule(ref cause) => cause,
            GetComplianceDetailsByConfigRuleError::Validation(ref cause) => cause,
            GetComplianceDetailsByConfigRuleError::Credentials(ref err) => err.description(),
            GetComplianceDetailsByConfigRuleError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetComplianceDetailsByConfigRuleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetComplianceDetailsByResource
#[derive(Debug, PartialEq)]
pub enum GetComplianceDetailsByResourceError {
    ///<p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetComplianceDetailsByResourceError {
    pub fn from_body(body: &str) -> GetComplianceDetailsByResourceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterValueException" => GetComplianceDetailsByResourceError::InvalidParameterValue(String::from(error_message)),
                    "ValidationException" => {
                        GetComplianceDetailsByResourceError::Validation(error_message.to_string())
                    }
                    _ => GetComplianceDetailsByResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetComplianceDetailsByResourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetComplianceDetailsByResourceError {
    fn from(err: serde_json::error::Error) -> GetComplianceDetailsByResourceError {
        GetComplianceDetailsByResourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetComplianceDetailsByResourceError {
    fn from(err: CredentialsError) -> GetComplianceDetailsByResourceError {
        GetComplianceDetailsByResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetComplianceDetailsByResourceError {
    fn from(err: HttpDispatchError) -> GetComplianceDetailsByResourceError {
        GetComplianceDetailsByResourceError::HttpDispatch(err)
    }
}
impl fmt::Display for GetComplianceDetailsByResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetComplianceDetailsByResourceError {
    fn description(&self) -> &str {
        match *self {
            GetComplianceDetailsByResourceError::InvalidParameterValue(ref cause) => cause,
            GetComplianceDetailsByResourceError::Validation(ref cause) => cause,
            GetComplianceDetailsByResourceError::Credentials(ref err) => err.description(),
            GetComplianceDetailsByResourceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetComplianceDetailsByResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetComplianceSummaryByConfigRule
#[derive(Debug, PartialEq)]
pub enum GetComplianceSummaryByConfigRuleError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetComplianceSummaryByConfigRuleError {
    pub fn from_body(body: &str) -> GetComplianceSummaryByConfigRuleError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ValidationException" => {
                        GetComplianceSummaryByConfigRuleError::Validation(error_message.to_string())
                    }
                    _ => GetComplianceSummaryByConfigRuleError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetComplianceSummaryByConfigRuleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetComplianceSummaryByConfigRuleError {
    fn from(err: serde_json::error::Error) -> GetComplianceSummaryByConfigRuleError {
        GetComplianceSummaryByConfigRuleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetComplianceSummaryByConfigRuleError {
    fn from(err: CredentialsError) -> GetComplianceSummaryByConfigRuleError {
        GetComplianceSummaryByConfigRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetComplianceSummaryByConfigRuleError {
    fn from(err: HttpDispatchError) -> GetComplianceSummaryByConfigRuleError {
        GetComplianceSummaryByConfigRuleError::HttpDispatch(err)
    }
}
impl fmt::Display for GetComplianceSummaryByConfigRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetComplianceSummaryByConfigRuleError {
    fn description(&self) -> &str {
        match *self {
            GetComplianceSummaryByConfigRuleError::Validation(ref cause) => cause,
            GetComplianceSummaryByConfigRuleError::Credentials(ref err) => err.description(),
            GetComplianceSummaryByConfigRuleError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetComplianceSummaryByConfigRuleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetComplianceSummaryByResourceType
#[derive(Debug, PartialEq)]
pub enum GetComplianceSummaryByResourceTypeError {
    ///<p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetComplianceSummaryByResourceTypeError {
    pub fn from_body(body: &str) -> GetComplianceSummaryByResourceTypeError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterValueException" => GetComplianceSummaryByResourceTypeError::InvalidParameterValue(String::from(error_message)),
                    "ValidationException" => {
                        GetComplianceSummaryByResourceTypeError::Validation(error_message
                                                                                .to_string())
                    }
                    _ => GetComplianceSummaryByResourceTypeError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetComplianceSummaryByResourceTypeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetComplianceSummaryByResourceTypeError {
    fn from(err: serde_json::error::Error) -> GetComplianceSummaryByResourceTypeError {
        GetComplianceSummaryByResourceTypeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetComplianceSummaryByResourceTypeError {
    fn from(err: CredentialsError) -> GetComplianceSummaryByResourceTypeError {
        GetComplianceSummaryByResourceTypeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetComplianceSummaryByResourceTypeError {
    fn from(err: HttpDispatchError) -> GetComplianceSummaryByResourceTypeError {
        GetComplianceSummaryByResourceTypeError::HttpDispatch(err)
    }
}
impl fmt::Display for GetComplianceSummaryByResourceTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetComplianceSummaryByResourceTypeError {
    fn description(&self) -> &str {
        match *self {
            GetComplianceSummaryByResourceTypeError::InvalidParameterValue(ref cause) => cause,
            GetComplianceSummaryByResourceTypeError::Validation(ref cause) => cause,
            GetComplianceSummaryByResourceTypeError::Credentials(ref err) => err.description(),
            GetComplianceSummaryByResourceTypeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetComplianceSummaryByResourceTypeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetResourceConfigHistory
#[derive(Debug, PartialEq)]
pub enum GetResourceConfigHistoryError {
    ///<p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    ///<p>The specified next token is invalid. Specify the <code>NextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    ///<p>The specified time range is not valid. The earlier time is not chronologically before the later time.</p>
    InvalidTimeRange(String),
    ///<p>There are no configuration recorders available to provide the role needed to describe your resources. Create a configuration recorder.</p>
    NoAvailableConfigurationRecorder(String),
    ///<p>You have specified a resource that is either unknown or has not been discovered.</p>
    ResourceNotDiscovered(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetResourceConfigHistoryError {
    pub fn from_body(body: &str) -> GetResourceConfigHistoryError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidLimitException" => {
                        GetResourceConfigHistoryError::InvalidLimit(String::from(error_message))
                    }
                    "InvalidNextTokenException" => {
                        GetResourceConfigHistoryError::InvalidNextToken(String::from(error_message))
                    }
                    "InvalidTimeRangeException" => {
                        GetResourceConfigHistoryError::InvalidTimeRange(String::from(error_message))
                    }
                    "NoAvailableConfigurationRecorderException" => GetResourceConfigHistoryError::NoAvailableConfigurationRecorder(String::from(error_message)),
                    "ResourceNotDiscoveredException" => GetResourceConfigHistoryError::ResourceNotDiscovered(String::from(error_message)),
                    "ValidationException" => {
                        GetResourceConfigHistoryError::Validation(error_message.to_string())
                    }
                    _ => GetResourceConfigHistoryError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetResourceConfigHistoryError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetResourceConfigHistoryError {
    fn from(err: serde_json::error::Error) -> GetResourceConfigHistoryError {
        GetResourceConfigHistoryError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetResourceConfigHistoryError {
    fn from(err: CredentialsError) -> GetResourceConfigHistoryError {
        GetResourceConfigHistoryError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetResourceConfigHistoryError {
    fn from(err: HttpDispatchError) -> GetResourceConfigHistoryError {
        GetResourceConfigHistoryError::HttpDispatch(err)
    }
}
impl fmt::Display for GetResourceConfigHistoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetResourceConfigHistoryError {
    fn description(&self) -> &str {
        match *self {
            GetResourceConfigHistoryError::InvalidLimit(ref cause) => cause,
            GetResourceConfigHistoryError::InvalidNextToken(ref cause) => cause,
            GetResourceConfigHistoryError::InvalidTimeRange(ref cause) => cause,
            GetResourceConfigHistoryError::NoAvailableConfigurationRecorder(ref cause) => cause,
            GetResourceConfigHistoryError::ResourceNotDiscovered(ref cause) => cause,
            GetResourceConfigHistoryError::Validation(ref cause) => cause,
            GetResourceConfigHistoryError::Credentials(ref err) => err.description(),
            GetResourceConfigHistoryError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetResourceConfigHistoryError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListDiscoveredResources
#[derive(Debug, PartialEq)]
pub enum ListDiscoveredResourcesError {
    ///<p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    ///<p>The specified next token is invalid. Specify the <code>NextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    ///<p>There are no configuration recorders available to provide the role needed to describe your resources. Create a configuration recorder.</p>
    NoAvailableConfigurationRecorder(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListDiscoveredResourcesError {
    pub fn from_body(body: &str) -> ListDiscoveredResourcesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidLimitException" => {
                        ListDiscoveredResourcesError::InvalidLimit(String::from(error_message))
                    }
                    "InvalidNextTokenException" => {
                        ListDiscoveredResourcesError::InvalidNextToken(String::from(error_message))
                    }
                    "NoAvailableConfigurationRecorderException" => ListDiscoveredResourcesError::NoAvailableConfigurationRecorder(String::from(error_message)),
                    "ValidationException" => {
                        ListDiscoveredResourcesError::Validation(error_message.to_string())
                    }
                    _ => ListDiscoveredResourcesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListDiscoveredResourcesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListDiscoveredResourcesError {
    fn from(err: serde_json::error::Error) -> ListDiscoveredResourcesError {
        ListDiscoveredResourcesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListDiscoveredResourcesError {
    fn from(err: CredentialsError) -> ListDiscoveredResourcesError {
        ListDiscoveredResourcesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListDiscoveredResourcesError {
    fn from(err: HttpDispatchError) -> ListDiscoveredResourcesError {
        ListDiscoveredResourcesError::HttpDispatch(err)
    }
}
impl fmt::Display for ListDiscoveredResourcesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDiscoveredResourcesError {
    fn description(&self) -> &str {
        match *self {
            ListDiscoveredResourcesError::InvalidLimit(ref cause) => cause,
            ListDiscoveredResourcesError::InvalidNextToken(ref cause) => cause,
            ListDiscoveredResourcesError::NoAvailableConfigurationRecorder(ref cause) => cause,
            ListDiscoveredResourcesError::Validation(ref cause) => cause,
            ListDiscoveredResourcesError::Credentials(ref err) => err.description(),
            ListDiscoveredResourcesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListDiscoveredResourcesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutConfigRule
#[derive(Debug, PartialEq)]
pub enum PutConfigRuleError {
    ///<p>Indicates one of the following errors:</p> <ul> <li> <p>The rule cannot be created because the IAM role assigned to AWS Config lacks permissions to perform the config:Put* action.</p> </li> <li> <p>The AWS Lambda function cannot be invoked. Check the function ARN, and check the function's permissions.</p> </li> </ul>
    InsufficientPermissions(String),
    ///<p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
    ///<p>Failed to add the AWS Config rule because the account already contains the maximum number of 50 rules. Consider deleting any deactivated rules before adding new rules.</p>
    MaxNumberOfConfigRulesExceeded(String),
    ///<p>There are no configuration recorders available to provide the role needed to describe your resources. Create a configuration recorder.</p>
    NoAvailableConfigurationRecorder(String),
    ///<p>The rule is currently being deleted or the rule is deleting your evaluation results. Try your request again later.</p>
    ResourceInUse(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl PutConfigRuleError {
    pub fn from_body(body: &str) -> PutConfigRuleError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InsufficientPermissionsException" => {
                        PutConfigRuleError::InsufficientPermissions(String::from(error_message))
                    }
                    "InvalidParameterValueException" => {
                        PutConfigRuleError::InvalidParameterValue(String::from(error_message))
                    }
                    "MaxNumberOfConfigRulesExceededException" => PutConfigRuleError::MaxNumberOfConfigRulesExceeded(String::from(error_message)),
                    "NoAvailableConfigurationRecorderException" => PutConfigRuleError::NoAvailableConfigurationRecorder(String::from(error_message)),
                    "ResourceInUseException" => {
                        PutConfigRuleError::ResourceInUse(String::from(error_message))
                    }
                    "ValidationException" => {
                        PutConfigRuleError::Validation(error_message.to_string())
                    }
                    _ => PutConfigRuleError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutConfigRuleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutConfigRuleError {
    fn from(err: serde_json::error::Error) -> PutConfigRuleError {
        PutConfigRuleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutConfigRuleError {
    fn from(err: CredentialsError) -> PutConfigRuleError {
        PutConfigRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutConfigRuleError {
    fn from(err: HttpDispatchError) -> PutConfigRuleError {
        PutConfigRuleError::HttpDispatch(err)
    }
}
impl fmt::Display for PutConfigRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutConfigRuleError {
    fn description(&self) -> &str {
        match *self {
            PutConfigRuleError::InsufficientPermissions(ref cause) => cause,
            PutConfigRuleError::InvalidParameterValue(ref cause) => cause,
            PutConfigRuleError::MaxNumberOfConfigRulesExceeded(ref cause) => cause,
            PutConfigRuleError::NoAvailableConfigurationRecorder(ref cause) => cause,
            PutConfigRuleError::ResourceInUse(ref cause) => cause,
            PutConfigRuleError::Validation(ref cause) => cause,
            PutConfigRuleError::Credentials(ref err) => err.description(),
            PutConfigRuleError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutConfigRuleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutConfigurationRecorder
#[derive(Debug, PartialEq)]
pub enum PutConfigurationRecorderError {
    ///<p>You have provided a configuration recorder name that is not valid.</p>
    InvalidConfigurationRecorderName(String),
    ///<p>AWS Config throws an exception if the recording group does not contain a valid list of resource types. Invalid values could also be incorrectly formatted.</p>
    InvalidRecordingGroup(String),
    ///<p>You have provided a null or empty role ARN.</p>
    InvalidRole(String),
    ///<p>You have reached the limit on the number of recorders you can create.</p>
    MaxNumberOfConfigurationRecordersExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl PutConfigurationRecorderError {
    pub fn from_body(body: &str) -> PutConfigurationRecorderError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidConfigurationRecorderNameException" => PutConfigurationRecorderError::InvalidConfigurationRecorderName(String::from(error_message)),
                    "InvalidRecordingGroupException" => PutConfigurationRecorderError::InvalidRecordingGroup(String::from(error_message)),
                    "InvalidRoleException" => {
                        PutConfigurationRecorderError::InvalidRole(String::from(error_message))
                    }
                    "MaxNumberOfConfigurationRecordersExceededException" => PutConfigurationRecorderError::MaxNumberOfConfigurationRecordersExceeded(String::from(error_message)),
                    "ValidationException" => {
                        PutConfigurationRecorderError::Validation(error_message.to_string())
                    }
                    _ => PutConfigurationRecorderError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutConfigurationRecorderError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutConfigurationRecorderError {
    fn from(err: serde_json::error::Error) -> PutConfigurationRecorderError {
        PutConfigurationRecorderError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutConfigurationRecorderError {
    fn from(err: CredentialsError) -> PutConfigurationRecorderError {
        PutConfigurationRecorderError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutConfigurationRecorderError {
    fn from(err: HttpDispatchError) -> PutConfigurationRecorderError {
        PutConfigurationRecorderError::HttpDispatch(err)
    }
}
impl fmt::Display for PutConfigurationRecorderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutConfigurationRecorderError {
    fn description(&self) -> &str {
        match *self {
            PutConfigurationRecorderError::InvalidConfigurationRecorderName(ref cause) => cause,
            PutConfigurationRecorderError::InvalidRecordingGroup(ref cause) => cause,
            PutConfigurationRecorderError::InvalidRole(ref cause) => cause,
            PutConfigurationRecorderError::MaxNumberOfConfigurationRecordersExceeded(ref cause) => {
                cause
            }
            PutConfigurationRecorderError::Validation(ref cause) => cause,
            PutConfigurationRecorderError::Credentials(ref err) => err.description(),
            PutConfigurationRecorderError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutConfigurationRecorderError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutDeliveryChannel
#[derive(Debug, PartialEq)]
pub enum PutDeliveryChannelError {
    ///<p>Your Amazon S3 bucket policy does not permit AWS Config to write to it.</p>
    InsufficientDeliveryPolicy(String),
    ///<p>The specified delivery channel name is not valid.</p>
    InvalidDeliveryChannelName(String),
    ///<p>The specified Amazon S3 key prefix is not valid.</p>
    InvalidS3KeyPrefix(String),
    ///<p>The specified Amazon SNS topic does not exist.</p>
    InvalidSNSTopicARN(String),
    ///<p>You have reached the limit on the number of delivery channels you can create.</p>
    MaxNumberOfDeliveryChannelsExceeded(String),
    ///<p>There are no configuration recorders available to provide the role needed to describe your resources. Create a configuration recorder.</p>
    NoAvailableConfigurationRecorder(String),
    ///<p>The specified Amazon S3 bucket does not exist.</p>
    NoSuchBucket(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl PutDeliveryChannelError {
    pub fn from_body(body: &str) -> PutDeliveryChannelError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InsufficientDeliveryPolicyException" => PutDeliveryChannelError::InsufficientDeliveryPolicy(String::from(error_message)),
                    "InvalidDeliveryChannelNameException" => PutDeliveryChannelError::InvalidDeliveryChannelName(String::from(error_message)),
                    "InvalidS3KeyPrefixException" => {
                        PutDeliveryChannelError::InvalidS3KeyPrefix(String::from(error_message))
                    }
                    "InvalidSNSTopicARNException" => {
                        PutDeliveryChannelError::InvalidSNSTopicARN(String::from(error_message))
                    }
                    "MaxNumberOfDeliveryChannelsExceededException" => PutDeliveryChannelError::MaxNumberOfDeliveryChannelsExceeded(String::from(error_message)),
                    "NoAvailableConfigurationRecorderException" => PutDeliveryChannelError::NoAvailableConfigurationRecorder(String::from(error_message)),
                    "NoSuchBucketException" => {
                        PutDeliveryChannelError::NoSuchBucket(String::from(error_message))
                    }
                    "ValidationException" => {
                        PutDeliveryChannelError::Validation(error_message.to_string())
                    }
                    _ => PutDeliveryChannelError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutDeliveryChannelError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutDeliveryChannelError {
    fn from(err: serde_json::error::Error) -> PutDeliveryChannelError {
        PutDeliveryChannelError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutDeliveryChannelError {
    fn from(err: CredentialsError) -> PutDeliveryChannelError {
        PutDeliveryChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutDeliveryChannelError {
    fn from(err: HttpDispatchError) -> PutDeliveryChannelError {
        PutDeliveryChannelError::HttpDispatch(err)
    }
}
impl fmt::Display for PutDeliveryChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutDeliveryChannelError {
    fn description(&self) -> &str {
        match *self {
            PutDeliveryChannelError::InsufficientDeliveryPolicy(ref cause) => cause,
            PutDeliveryChannelError::InvalidDeliveryChannelName(ref cause) => cause,
            PutDeliveryChannelError::InvalidS3KeyPrefix(ref cause) => cause,
            PutDeliveryChannelError::InvalidSNSTopicARN(ref cause) => cause,
            PutDeliveryChannelError::MaxNumberOfDeliveryChannelsExceeded(ref cause) => cause,
            PutDeliveryChannelError::NoAvailableConfigurationRecorder(ref cause) => cause,
            PutDeliveryChannelError::NoSuchBucket(ref cause) => cause,
            PutDeliveryChannelError::Validation(ref cause) => cause,
            PutDeliveryChannelError::Credentials(ref err) => err.description(),
            PutDeliveryChannelError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutDeliveryChannelError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutEvaluations
#[derive(Debug, PartialEq)]
pub enum PutEvaluationsError {
    ///<p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
    ///<p>The specified <code>ResultToken</code> is invalid.</p>
    InvalidResultToken(String),
    ///<p>One or more AWS Config rules in the request are invalid. Verify that the rule names are correct and try again.</p>
    NoSuchConfigRule(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl PutEvaluationsError {
    pub fn from_body(body: &str) -> PutEvaluationsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterValueException" => {
                        PutEvaluationsError::InvalidParameterValue(String::from(error_message))
                    }
                    "InvalidResultTokenException" => {
                        PutEvaluationsError::InvalidResultToken(String::from(error_message))
                    }
                    "NoSuchConfigRuleException" => {
                        PutEvaluationsError::NoSuchConfigRule(String::from(error_message))
                    }
                    "ValidationException" => {
                        PutEvaluationsError::Validation(error_message.to_string())
                    }
                    _ => PutEvaluationsError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutEvaluationsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutEvaluationsError {
    fn from(err: serde_json::error::Error) -> PutEvaluationsError {
        PutEvaluationsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutEvaluationsError {
    fn from(err: CredentialsError) -> PutEvaluationsError {
        PutEvaluationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutEvaluationsError {
    fn from(err: HttpDispatchError) -> PutEvaluationsError {
        PutEvaluationsError::HttpDispatch(err)
    }
}
impl fmt::Display for PutEvaluationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutEvaluationsError {
    fn description(&self) -> &str {
        match *self {
            PutEvaluationsError::InvalidParameterValue(ref cause) => cause,
            PutEvaluationsError::InvalidResultToken(ref cause) => cause,
            PutEvaluationsError::NoSuchConfigRule(ref cause) => cause,
            PutEvaluationsError::Validation(ref cause) => cause,
            PutEvaluationsError::Credentials(ref err) => err.description(),
            PutEvaluationsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutEvaluationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StartConfigRulesEvaluation
#[derive(Debug, PartialEq)]
pub enum StartConfigRulesEvaluationError {
    ///<p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
    ///<p>This exception is thrown if an evaluation is in progress or if you call the <a>StartConfigRulesEvaluation</a> API more than once per minute.</p>
    LimitExceeded(String),
    ///<p>One or more AWS Config rules in the request are invalid. Verify that the rule names are correct and try again.</p>
    NoSuchConfigRule(String),
    ///<p>The rule is currently being deleted or the rule is deleting your evaluation results. Try your request again later.</p>
    ResourceInUse(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl StartConfigRulesEvaluationError {
    pub fn from_body(body: &str) -> StartConfigRulesEvaluationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterValueException" => StartConfigRulesEvaluationError::InvalidParameterValue(String::from(error_message)),
                    "LimitExceededException" => {
                        StartConfigRulesEvaluationError::LimitExceeded(String::from(error_message))
                    }
                    "NoSuchConfigRuleException" => StartConfigRulesEvaluationError::NoSuchConfigRule(String::from(error_message)),
                    "ResourceInUseException" => {
                        StartConfigRulesEvaluationError::ResourceInUse(String::from(error_message))
                    }
                    "ValidationException" => {
                        StartConfigRulesEvaluationError::Validation(error_message.to_string())
                    }
                    _ => StartConfigRulesEvaluationError::Unknown(String::from(body)),
                }
            }
            Err(_) => StartConfigRulesEvaluationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StartConfigRulesEvaluationError {
    fn from(err: serde_json::error::Error) -> StartConfigRulesEvaluationError {
        StartConfigRulesEvaluationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StartConfigRulesEvaluationError {
    fn from(err: CredentialsError) -> StartConfigRulesEvaluationError {
        StartConfigRulesEvaluationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartConfigRulesEvaluationError {
    fn from(err: HttpDispatchError) -> StartConfigRulesEvaluationError {
        StartConfigRulesEvaluationError::HttpDispatch(err)
    }
}
impl fmt::Display for StartConfigRulesEvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartConfigRulesEvaluationError {
    fn description(&self) -> &str {
        match *self {
            StartConfigRulesEvaluationError::InvalidParameterValue(ref cause) => cause,
            StartConfigRulesEvaluationError::LimitExceeded(ref cause) => cause,
            StartConfigRulesEvaluationError::NoSuchConfigRule(ref cause) => cause,
            StartConfigRulesEvaluationError::ResourceInUse(ref cause) => cause,
            StartConfigRulesEvaluationError::Validation(ref cause) => cause,
            StartConfigRulesEvaluationError::Credentials(ref err) => err.description(),
            StartConfigRulesEvaluationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StartConfigRulesEvaluationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StartConfigurationRecorder
#[derive(Debug, PartialEq)]
pub enum StartConfigurationRecorderError {
    ///<p>There is no delivery channel available to record configurations.</p>
    NoAvailableDeliveryChannel(String),
    ///<p>You have specified a configuration recorder that does not exist.</p>
    NoSuchConfigurationRecorder(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl StartConfigurationRecorderError {
    pub fn from_body(body: &str) -> StartConfigurationRecorderError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "NoAvailableDeliveryChannelException" => StartConfigurationRecorderError::NoAvailableDeliveryChannel(String::from(error_message)),
                    "NoSuchConfigurationRecorderException" => StartConfigurationRecorderError::NoSuchConfigurationRecorder(String::from(error_message)),
                    "ValidationException" => {
                        StartConfigurationRecorderError::Validation(error_message.to_string())
                    }
                    _ => StartConfigurationRecorderError::Unknown(String::from(body)),
                }
            }
            Err(_) => StartConfigurationRecorderError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StartConfigurationRecorderError {
    fn from(err: serde_json::error::Error) -> StartConfigurationRecorderError {
        StartConfigurationRecorderError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StartConfigurationRecorderError {
    fn from(err: CredentialsError) -> StartConfigurationRecorderError {
        StartConfigurationRecorderError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartConfigurationRecorderError {
    fn from(err: HttpDispatchError) -> StartConfigurationRecorderError {
        StartConfigurationRecorderError::HttpDispatch(err)
    }
}
impl fmt::Display for StartConfigurationRecorderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartConfigurationRecorderError {
    fn description(&self) -> &str {
        match *self {
            StartConfigurationRecorderError::NoAvailableDeliveryChannel(ref cause) => cause,
            StartConfigurationRecorderError::NoSuchConfigurationRecorder(ref cause) => cause,
            StartConfigurationRecorderError::Validation(ref cause) => cause,
            StartConfigurationRecorderError::Credentials(ref err) => err.description(),
            StartConfigurationRecorderError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StartConfigurationRecorderError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StopConfigurationRecorder
#[derive(Debug, PartialEq)]
pub enum StopConfigurationRecorderError {
    ///<p>You have specified a configuration recorder that does not exist.</p>
    NoSuchConfigurationRecorder(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl StopConfigurationRecorderError {
    pub fn from_body(body: &str) -> StopConfigurationRecorderError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "NoSuchConfigurationRecorderException" => StopConfigurationRecorderError::NoSuchConfigurationRecorder(String::from(error_message)),
                    "ValidationException" => {
                        StopConfigurationRecorderError::Validation(error_message.to_string())
                    }
                    _ => StopConfigurationRecorderError::Unknown(String::from(body)),
                }
            }
            Err(_) => StopConfigurationRecorderError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StopConfigurationRecorderError {
    fn from(err: serde_json::error::Error) -> StopConfigurationRecorderError {
        StopConfigurationRecorderError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StopConfigurationRecorderError {
    fn from(err: CredentialsError) -> StopConfigurationRecorderError {
        StopConfigurationRecorderError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopConfigurationRecorderError {
    fn from(err: HttpDispatchError) -> StopConfigurationRecorderError {
        StopConfigurationRecorderError::HttpDispatch(err)
    }
}
impl fmt::Display for StopConfigurationRecorderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopConfigurationRecorderError {
    fn description(&self) -> &str {
        match *self {
            StopConfigurationRecorderError::NoSuchConfigurationRecorder(ref cause) => cause,
            StopConfigurationRecorderError::Validation(ref cause) => cause,
            StopConfigurationRecorderError::Credentials(ref err) => err.description(),
            StopConfigurationRecorderError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StopConfigurationRecorderError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Config Service API. Config Service clients implement this trait.
pub trait ConfigService {
    #[doc="<p>Deletes the specified AWS Config rule and all of its evaluation results.</p> <p>AWS Config sets the state of a rule to <code>DELETING</code> until the deletion is complete. You cannot update a rule while it is in this state. If you make a <code>PutConfigRule</code> or <code>DeleteConfigRule</code> request for the rule, you will receive a <code>ResourceInUseException</code>.</p> <p>You can check the state of a rule by using the <code>DescribeConfigRules</code> request.</p>"]
    fn delete_config_rule(&self,
                          input: &DeleteConfigRuleRequest)
                          -> Result<(), DeleteConfigRuleError>;


    #[doc="<p>Deletes the configuration recorder.</p> <p>After the configuration recorder is deleted, AWS Config will not record resource configuration changes until you create a new configuration recorder.</p> <p>This action does not delete the configuration information that was previously recorded. You will be able to access the previously recorded information by using the <code>GetResourceConfigHistory</code> action, but you will not be able to access this information in the AWS Config console until you create a new configuration recorder.</p>"]
    fn delete_configuration_recorder(&self,
                                     input: &DeleteConfigurationRecorderRequest)
                                     -> Result<(), DeleteConfigurationRecorderError>;


    #[doc="<p>Deletes the delivery channel.</p> <p>Before you can delete the delivery channel, you must stop the configuration recorder by using the <a>StopConfigurationRecorder</a> action.</p>"]
    fn delete_delivery_channel(&self,
                               input: &DeleteDeliveryChannelRequest)
                               -> Result<(), DeleteDeliveryChannelError>;


    #[doc="<p>Deletes the evaluation results for the specified Config rule. You can specify one Config rule per request. After you delete the evaluation results, you can call the <a>StartConfigRulesEvaluation</a> API to start evaluating your AWS resources against the rule.</p>"]
    fn delete_evaluation_results
        (&self,
         input: &DeleteEvaluationResultsRequest)
         -> Result<DeleteEvaluationResultsResponse, DeleteEvaluationResultsError>;


    #[doc="<p>Schedules delivery of a configuration snapshot to the Amazon S3 bucket in the specified delivery channel. After the delivery has started, AWS Config sends following notifications using an Amazon SNS topic that you have specified.</p> <ul> <li> <p>Notification of starting the delivery.</p> </li> <li> <p>Notification of delivery completed, if the delivery was successfully completed.</p> </li> <li> <p>Notification of delivery failure, if the delivery failed to complete.</p> </li> </ul>"]
    fn deliver_config_snapshot
        (&self,
         input: &DeliverConfigSnapshotRequest)
         -> Result<DeliverConfigSnapshotResponse, DeliverConfigSnapshotError>;


    #[doc="<p>Indicates whether the specified AWS Config rules are compliant. If a rule is noncompliant, this action returns the number of AWS resources that do not comply with the rule.</p> <p>A rule is compliant if all of the evaluated resources comply with it, and it is noncompliant if any of these resources do not comply.</p> <p>If AWS Config has no current evaluation results for the rule, it returns <code>INSUFFICIENT_DATA</code>. This result might indicate one of the following conditions:</p> <ul> <li> <p>AWS Config has never invoked an evaluation for the rule. To check whether it has, use the <code>DescribeConfigRuleEvaluationStatus</code> action to get the <code>LastSuccessfulInvocationTime</code> and <code>LastFailedInvocationTime</code>.</p> </li> <li> <p>The rule's AWS Lambda function is failing to send evaluation results to AWS Config. Verify that the role that you assigned to your configuration recorder includes the <code>config:PutEvaluations</code> permission. If the rule is a custom rule, verify that the AWS Lambda execution role includes the <code>config:PutEvaluations</code> permission.</p> </li> <li> <p>The rule's AWS Lambda function has returned <code>NOT_APPLICABLE</code> for all evaluation results. This can occur if the resources were deleted or removed from the rule's scope.</p> </li> </ul>"]
    fn describe_compliance_by_config_rule
        (&self,
         input: &DescribeComplianceByConfigRuleRequest)
         -> Result<DescribeComplianceByConfigRuleResponse, DescribeComplianceByConfigRuleError>;


    #[doc="<p>Indicates whether the specified AWS resources are compliant. If a resource is noncompliant, this action returns the number of AWS Config rules that the resource does not comply with.</p> <p>A resource is compliant if it complies with all the AWS Config rules that evaluate it. It is noncompliant if it does not comply with one or more of these rules.</p> <p>If AWS Config has no current evaluation results for the resource, it returns <code>INSUFFICIENT_DATA</code>. This result might indicate one of the following conditions about the rules that evaluate the resource:</p> <ul> <li> <p>AWS Config has never invoked an evaluation for the rule. To check whether it has, use the <code>DescribeConfigRuleEvaluationStatus</code> action to get the <code>LastSuccessfulInvocationTime</code> and <code>LastFailedInvocationTime</code>.</p> </li> <li> <p>The rule's AWS Lambda function is failing to send evaluation results to AWS Config. Verify that the role that you assigned to your configuration recorder includes the <code>config:PutEvaluations</code> permission. If the rule is a custom rule, verify that the AWS Lambda execution role includes the <code>config:PutEvaluations</code> permission.</p> </li> <li> <p>The rule's AWS Lambda function has returned <code>NOT_APPLICABLE</code> for all evaluation results. This can occur if the resources were deleted or removed from the rule's scope.</p> </li> </ul>"]
    fn describe_compliance_by_resource
        (&self,
         input: &DescribeComplianceByResourceRequest)
         -> Result<DescribeComplianceByResourceResponse, DescribeComplianceByResourceError>;


    #[doc="<p>Returns status information for each of your AWS managed Config rules. The status includes information such as the last time AWS Config invoked the rule, the last time AWS Config failed to invoke the rule, and the related error for the last failure.</p>"]
    fn describe_config_rule_evaluation_status
        (&self,
         input: &DescribeConfigRuleEvaluationStatusRequest)
         -> Result<DescribeConfigRuleEvaluationStatusResponse,
                   DescribeConfigRuleEvaluationStatusError>;


    #[doc="<p>Returns details about your AWS Config rules.</p>"]
    fn describe_config_rules(&self,
                             input: &DescribeConfigRulesRequest)
                             -> Result<DescribeConfigRulesResponse, DescribeConfigRulesError>;


    #[doc="<p>Returns the current status of the specified configuration recorder. If a configuration recorder is not specified, this action returns the status of all configuration recorder associated with the account.</p> <note> <p>Currently, you can specify only one configuration recorder per region in your account.</p> </note>"]
    fn describe_configuration_recorder_status
        (&self,
         input: &DescribeConfigurationRecorderStatusRequest)
         -> Result<DescribeConfigurationRecorderStatusResponse,
                   DescribeConfigurationRecorderStatusError>;


    #[doc="<p>Returns the details for the specified configuration recorders. If the configuration recorder is not specified, this action returns the details for all configuration recorders associated with the account.</p> <note> <p>Currently, you can specify only one configuration recorder per region in your account.</p> </note>"]
    fn describe_configuration_recorders
        (&self,
         input: &DescribeConfigurationRecordersRequest)
         -> Result<DescribeConfigurationRecordersResponse, DescribeConfigurationRecordersError>;


    #[doc="<p>Returns the current status of the specified delivery channel. If a delivery channel is not specified, this action returns the current status of all delivery channels associated with the account.</p> <note> <p>Currently, you can specify only one delivery channel per region in your account.</p> </note>"]
    fn describe_delivery_channel_status
        (&self,
         input: &DescribeDeliveryChannelStatusRequest)
         -> Result<DescribeDeliveryChannelStatusResponse, DescribeDeliveryChannelStatusError>;


    #[doc="<p>Returns details about the specified delivery channel. If a delivery channel is not specified, this action returns the details of all delivery channels associated with the account.</p> <note> <p>Currently, you can specify only one delivery channel per region in your account.</p> </note>"]
    fn describe_delivery_channels
        (&self,
         input: &DescribeDeliveryChannelsRequest)
         -> Result<DescribeDeliveryChannelsResponse, DescribeDeliveryChannelsError>;


    #[doc="<p>Returns the evaluation results for the specified AWS Config rule. The results indicate which AWS resources were evaluated by the rule, when each resource was last evaluated, and whether each resource complies with the rule.</p>"]
    fn get_compliance_details_by_config_rule
        (&self,
         input: &GetComplianceDetailsByConfigRuleRequest)
         -> Result<GetComplianceDetailsByConfigRuleResponse, GetComplianceDetailsByConfigRuleError>;


    #[doc="<p>Returns the evaluation results for the specified AWS resource. The results indicate which AWS Config rules were used to evaluate the resource, when each rule was last used, and whether the resource complies with each rule.</p>"]
    fn get_compliance_details_by_resource
        (&self,
         input: &GetComplianceDetailsByResourceRequest)
         -> Result<GetComplianceDetailsByResourceResponse, GetComplianceDetailsByResourceError>;


    #[doc="<p>Returns the number of AWS Config rules that are compliant and noncompliant, up to a maximum of 25 for each.</p>"]
    fn get_compliance_summary_by_config_rule
        (&self)
         -> Result<GetComplianceSummaryByConfigRuleResponse, GetComplianceSummaryByConfigRuleError>;


    #[doc="<p>Returns the number of resources that are compliant and the number that are noncompliant. You can specify one or more resource types to get these numbers for each resource type. The maximum number returned is 100.</p>"]
    fn get_compliance_summary_by_resource_type
        (&self,
         input: &GetComplianceSummaryByResourceTypeRequest)
         -> Result<GetComplianceSummaryByResourceTypeResponse,
                   GetComplianceSummaryByResourceTypeError>;


    #[doc="<p>Returns a list of configuration items for the specified resource. The list contains details about each state of the resource during the specified time interval.</p> <p>The response is paginated, and by default, AWS Config returns a limit of 10 configuration items per page. You can customize this number with the <code>limit</code> parameter. The response includes a <code>nextToken</code> string, and to get the next page of results, run the request again and enter this string for the <code>nextToken</code> parameter.</p> <note> <p>Each call to the API is limited to span a duration of seven days. It is likely that the number of records returned is smaller than the specified <code>limit</code>. In such cases, you can make another call, using the <code>nextToken</code>.</p> </note>"]
    fn get_resource_config_history
        (&self,
         input: &GetResourceConfigHistoryRequest)
         -> Result<GetResourceConfigHistoryResponse, GetResourceConfigHistoryError>;


    #[doc="<p>Accepts a resource type and returns a list of resource identifiers for the resources of that type. A resource identifier includes the resource type, ID, and (if available) the custom resource name. The results consist of resources that AWS Config has discovered, including those that AWS Config is not currently recording. You can narrow the results to include only resources that have specific resource IDs or a resource name.</p> <note> <p>You can specify either resource IDs or a resource name but not both in the same request.</p> </note> <p>The response is paginated, and by default AWS Config lists 100 resource identifiers on each page. You can customize this number with the <code>limit</code> parameter. The response includes a <code>nextToken</code> string, and to get the next page of results, run the request again and enter this string for the <code>nextToken</code> parameter.</p>"]
    fn list_discovered_resources
        (&self,
         input: &ListDiscoveredResourcesRequest)
         -> Result<ListDiscoveredResourcesResponse, ListDiscoveredResourcesError>;


    #[doc="<p>Adds or updates an AWS Config rule for evaluating whether your AWS resources comply with your desired configurations.</p> <p>You can use this action for custom Config rules and AWS managed Config rules. A custom Config rule is a rule that you develop and maintain. An AWS managed Config rule is a customizable, predefined rule that AWS Config provides.</p> <p>If you are adding a new custom Config rule, you must first create the AWS Lambda function that the rule invokes to evaluate your resources. When you use the <code>PutConfigRule</code> action to add the rule to AWS Config, you must specify the Amazon Resource Name (ARN) that AWS Lambda assigns to the function. Specify the ARN for the <code>SourceIdentifier</code> key. This key is part of the <code>Source</code> object, which is part of the <code>ConfigRule</code> object. </p> <p>If you are adding an AWS managed Config rule, specify the rule's identifier for the <code>SourceIdentifier</code> key. To reference AWS managed Config rule identifiers, see <a href=\"http://docs.aws.amazon.com/config/latest/developerguide/evaluate-config_use-managed-rules.html\">About AWS Managed Config Rules</a>.</p> <p>For any new rule that you add, specify the <code>ConfigRuleName</code> in the <code>ConfigRule</code> object. Do not specify the <code>ConfigRuleArn</code> or the <code>ConfigRuleId</code>. These values are generated by AWS Config for new rules.</p> <p>If you are updating a rule that you added previously, you can specify the rule by <code>ConfigRuleName</code>, <code>ConfigRuleId</code>, or <code>ConfigRuleArn</code> in the <code>ConfigRule</code> data type that you use in this request.</p> <p>The maximum number of rules that AWS Config supports is 50.</p> <p>For more information about requesting a rule limit increase, see <a href=\"http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_config\">AWS Config Limits</a> in the <i>AWS General Reference Guide</i>.</p> <p>For more information about developing and using AWS Config rules, see <a href=\"http://docs.aws.amazon.com/config/latest/developerguide/evaluate-config.html\">Evaluating AWS Resource Configurations with AWS Config</a> in the <i>AWS Config Developer Guide</i>.</p>"]
    fn put_config_rule(&self, input: &PutConfigRuleRequest) -> Result<(), PutConfigRuleError>;


    #[doc="<p>Creates a new configuration recorder to record the selected resource configurations.</p> <p>You can use this action to change the role <code>roleARN</code> and/or the <code>recordingGroup</code> of an existing recorder. To change the role, call the action on the existing configuration recorder and specify a role.</p> <note> <p>Currently, you can specify only one configuration recorder per region in your account.</p> <p>If <code>ConfigurationRecorder</code> does not have the <b>recordingGroup</b> parameter specified, the default is to record all supported resource types.</p> </note>"]
    fn put_configuration_recorder(&self,
                                  input: &PutConfigurationRecorderRequest)
                                  -> Result<(), PutConfigurationRecorderError>;


    #[doc="<p>Creates a delivery channel object to deliver configuration information to an Amazon S3 bucket and Amazon SNS topic.</p> <p>Before you can create a delivery channel, you must create a configuration recorder.</p> <p>You can use this action to change the Amazon S3 bucket or an Amazon SNS topic of the existing delivery channel. To change the Amazon S3 bucket or an Amazon SNS topic, call this action and specify the changed values for the S3 bucket and the SNS topic. If you specify a different value for either the S3 bucket or the SNS topic, this action will keep the existing value for the parameter that is not changed.</p> <note> <p>You can have only one delivery channel per region in your account.</p> </note>"]
    fn put_delivery_channel(&self,
                            input: &PutDeliveryChannelRequest)
                            -> Result<(), PutDeliveryChannelError>;


    #[doc="<p>Used by an AWS Lambda function to deliver evaluation results to AWS Config. This action is required in every AWS Lambda function that is invoked by an AWS Config rule.</p>"]
    fn put_evaluations(&self,
                       input: &PutEvaluationsRequest)
                       -> Result<PutEvaluationsResponse, PutEvaluationsError>;


    #[doc="<p>Runs an on-demand evaluation for the specified Config rules against the last known configuration state of the resources. Use <code>StartConfigRulesEvaluation</code> when you want to test a rule that you updated is working as expected. <code>StartConfigRulesEvaluation</code> does not re-record the latest configuration state for your resources; it re-runs an evaluation against the last known state of your resources. </p> <p>You can specify up to 25 Config rules per request. </p> <p>An existing <code>StartConfigRulesEvaluation</code> call must complete for the specified rules before you can call the API again. If you chose to have AWS Config stream to an Amazon SNS topic, you will receive a <code>ConfigRuleEvaluationStarted</code> notification when the evaluation starts.</p> <note> <p>You don't need to call the <code>StartConfigRulesEvaluation</code> API to run an evaluation for a new rule. When you create a new rule, AWS Config automatically evaluates your resources against the rule. </p> </note> <p>The <code>StartConfigRulesEvaluation</code> API is useful if you want to run on-demand evaluations, such as the following example:</p> <ol> <li> <p>You have a custom rule that evaluates your IAM resources every 24 hours.</p> </li> <li> <p>You update your Lambda function to add additional conditions to your rule.</p> </li> <li> <p>Instead of waiting for the next periodic evaluation, you call the <code>StartConfigRulesEvaluation</code> API.</p> </li> <li> <p>AWS Config invokes your Lambda function and evaluates your IAM resources.</p> </li> <li> <p>Your custom rule will still run periodic evaluations every 24 hours.</p> </li> </ol>"]
    fn start_config_rules_evaluation
        (&self,
         input: &StartConfigRulesEvaluationRequest)
         -> Result<StartConfigRulesEvaluationResponse, StartConfigRulesEvaluationError>;


    #[doc="<p>Starts recording configurations of the AWS resources you have selected to record in your AWS account.</p> <p>You must have created at least one delivery channel to successfully start the configuration recorder.</p>"]
    fn start_configuration_recorder(&self,
                                    input: &StartConfigurationRecorderRequest)
                                    -> Result<(), StartConfigurationRecorderError>;


    #[doc="<p>Stops recording configurations of the AWS resources you have selected to record in your AWS account.</p>"]
    fn stop_configuration_recorder(&self,
                                   input: &StopConfigurationRecorderRequest)
                                   -> Result<(), StopConfigurationRecorderError>;
}
/// A client for the Config Service API.
pub struct ConfigServiceClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> ConfigServiceClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        ConfigServiceClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> ConfigService for ConfigServiceClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    #[doc="<p>Deletes the specified AWS Config rule and all of its evaluation results.</p> <p>AWS Config sets the state of a rule to <code>DELETING</code> until the deletion is complete. You cannot update a rule while it is in this state. If you make a <code>PutConfigRule</code> or <code>DeleteConfigRule</code> request for the rule, you will receive a <code>ResourceInUseException</code>.</p> <p>You can check the state of a rule by using the <code>DescribeConfigRules</code> request.</p>"]
    fn delete_config_rule(&self,
                          input: &DeleteConfigRuleRequest)
                          -> Result<(), DeleteConfigRuleError> {
        let mut request = SignedRequest::new("POST", "config", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StarlingDoveService.DeleteConfigRule");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => {
                Err(DeleteConfigRuleError::from_body(String::from_utf8_lossy(&response.body)
                                                         .as_ref()))
            }
        }
    }


    #[doc="<p>Deletes the configuration recorder.</p> <p>After the configuration recorder is deleted, AWS Config will not record resource configuration changes until you create a new configuration recorder.</p> <p>This action does not delete the configuration information that was previously recorded. You will be able to access the previously recorded information by using the <code>GetResourceConfigHistory</code> action, but you will not be able to access this information in the AWS Config console until you create a new configuration recorder.</p>"]
    fn delete_configuration_recorder(&self,
                                     input: &DeleteConfigurationRecorderRequest)
                                     -> Result<(), DeleteConfigurationRecorderError> {
        let mut request = SignedRequest::new("POST", "config", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "StarlingDoveService.DeleteConfigurationRecorder");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => Err(DeleteConfigurationRecorderError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Deletes the delivery channel.</p> <p>Before you can delete the delivery channel, you must stop the configuration recorder by using the <a>StopConfigurationRecorder</a> action.</p>"]
    fn delete_delivery_channel(&self,
                               input: &DeleteDeliveryChannelRequest)
                               -> Result<(), DeleteDeliveryChannelError> {
        let mut request = SignedRequest::new("POST", "config", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StarlingDoveService.DeleteDeliveryChannel");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => {
                Err(DeleteDeliveryChannelError::from_body(String::from_utf8_lossy(&response.body)
                                                              .as_ref()))
            }
        }
    }


    #[doc="<p>Deletes the evaluation results for the specified Config rule. You can specify one Config rule per request. After you delete the evaluation results, you can call the <a>StartConfigRulesEvaluation</a> API to start evaluating your AWS resources against the rule.</p>"]
    fn delete_evaluation_results
        (&self,
         input: &DeleteEvaluationResultsRequest)
         -> Result<DeleteEvaluationResultsResponse, DeleteEvaluationResultsError> {
        let mut request = SignedRequest::new("POST", "config", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "StarlingDoveService.DeleteEvaluationResults");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeleteEvaluationResultsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DeleteEvaluationResultsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Schedules delivery of a configuration snapshot to the Amazon S3 bucket in the specified delivery channel. After the delivery has started, AWS Config sends following notifications using an Amazon SNS topic that you have specified.</p> <ul> <li> <p>Notification of starting the delivery.</p> </li> <li> <p>Notification of delivery completed, if the delivery was successfully completed.</p> </li> <li> <p>Notification of delivery failure, if the delivery failed to complete.</p> </li> </ul>"]
    fn deliver_config_snapshot
        (&self,
         input: &DeliverConfigSnapshotRequest)
         -> Result<DeliverConfigSnapshotResponse, DeliverConfigSnapshotError> {
        let mut request = SignedRequest::new("POST", "config", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StarlingDoveService.DeliverConfigSnapshot");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeliverConfigSnapshotResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DeliverConfigSnapshotError::from_body(String::from_utf8_lossy(&response.body)
                                                              .as_ref()))
            }
        }
    }


    #[doc="<p>Indicates whether the specified AWS Config rules are compliant. If a rule is noncompliant, this action returns the number of AWS resources that do not comply with the rule.</p> <p>A rule is compliant if all of the evaluated resources comply with it, and it is noncompliant if any of these resources do not comply.</p> <p>If AWS Config has no current evaluation results for the rule, it returns <code>INSUFFICIENT_DATA</code>. This result might indicate one of the following conditions:</p> <ul> <li> <p>AWS Config has never invoked an evaluation for the rule. To check whether it has, use the <code>DescribeConfigRuleEvaluationStatus</code> action to get the <code>LastSuccessfulInvocationTime</code> and <code>LastFailedInvocationTime</code>.</p> </li> <li> <p>The rule's AWS Lambda function is failing to send evaluation results to AWS Config. Verify that the role that you assigned to your configuration recorder includes the <code>config:PutEvaluations</code> permission. If the rule is a custom rule, verify that the AWS Lambda execution role includes the <code>config:PutEvaluations</code> permission.</p> </li> <li> <p>The rule's AWS Lambda function has returned <code>NOT_APPLICABLE</code> for all evaluation results. This can occur if the resources were deleted or removed from the rule's scope.</p> </li> </ul>"]
    fn describe_compliance_by_config_rule
        (&self,
         input: &DescribeComplianceByConfigRuleRequest)
         -> Result<DescribeComplianceByConfigRuleResponse, DescribeComplianceByConfigRuleError> {
        let mut request = SignedRequest::new("POST", "config", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "StarlingDoveService.DescribeComplianceByConfigRule");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeComplianceByConfigRuleResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DescribeComplianceByConfigRuleError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Indicates whether the specified AWS resources are compliant. If a resource is noncompliant, this action returns the number of AWS Config rules that the resource does not comply with.</p> <p>A resource is compliant if it complies with all the AWS Config rules that evaluate it. It is noncompliant if it does not comply with one or more of these rules.</p> <p>If AWS Config has no current evaluation results for the resource, it returns <code>INSUFFICIENT_DATA</code>. This result might indicate one of the following conditions about the rules that evaluate the resource:</p> <ul> <li> <p>AWS Config has never invoked an evaluation for the rule. To check whether it has, use the <code>DescribeConfigRuleEvaluationStatus</code> action to get the <code>LastSuccessfulInvocationTime</code> and <code>LastFailedInvocationTime</code>.</p> </li> <li> <p>The rule's AWS Lambda function is failing to send evaluation results to AWS Config. Verify that the role that you assigned to your configuration recorder includes the <code>config:PutEvaluations</code> permission. If the rule is a custom rule, verify that the AWS Lambda execution role includes the <code>config:PutEvaluations</code> permission.</p> </li> <li> <p>The rule's AWS Lambda function has returned <code>NOT_APPLICABLE</code> for all evaluation results. This can occur if the resources were deleted or removed from the rule's scope.</p> </li> </ul>"]
    fn describe_compliance_by_resource
        (&self,
         input: &DescribeComplianceByResourceRequest)
         -> Result<DescribeComplianceByResourceResponse, DescribeComplianceByResourceError> {
        let mut request = SignedRequest::new("POST", "config", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "StarlingDoveService.DescribeComplianceByResource");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeComplianceByResourceResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DescribeComplianceByResourceError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Returns status information for each of your AWS managed Config rules. The status includes information such as the last time AWS Config invoked the rule, the last time AWS Config failed to invoke the rule, and the related error for the last failure.</p>"]
    fn describe_config_rule_evaluation_status
        (&self,
         input: &DescribeConfigRuleEvaluationStatusRequest)
         -> Result<DescribeConfigRuleEvaluationStatusResponse,
                   DescribeConfigRuleEvaluationStatusError> {
        let mut request = SignedRequest::new("POST", "config", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "StarlingDoveService.DescribeConfigRuleEvaluationStatus");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeConfigRuleEvaluationStatusResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DescribeConfigRuleEvaluationStatusError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Returns details about your AWS Config rules.</p>"]
    fn describe_config_rules(&self,
                             input: &DescribeConfigRulesRequest)
                             -> Result<DescribeConfigRulesResponse, DescribeConfigRulesError> {
        let mut request = SignedRequest::new("POST", "config", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StarlingDoveService.DescribeConfigRules");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeConfigRulesResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeConfigRulesError::from_body(String::from_utf8_lossy(&response.body)
                                                            .as_ref()))
            }
        }
    }


    #[doc="<p>Returns the current status of the specified configuration recorder. If a configuration recorder is not specified, this action returns the status of all configuration recorder associated with the account.</p> <note> <p>Currently, you can specify only one configuration recorder per region in your account.</p> </note>"]
    fn describe_configuration_recorder_status
        (&self,
         input: &DescribeConfigurationRecorderStatusRequest)
         -> Result<DescribeConfigurationRecorderStatusResponse,
                   DescribeConfigurationRecorderStatusError> {
        let mut request = SignedRequest::new("POST", "config", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "StarlingDoveService.DescribeConfigurationRecorderStatus");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeConfigurationRecorderStatusResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DescribeConfigurationRecorderStatusError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Returns the details for the specified configuration recorders. If the configuration recorder is not specified, this action returns the details for all configuration recorders associated with the account.</p> <note> <p>Currently, you can specify only one configuration recorder per region in your account.</p> </note>"]
    fn describe_configuration_recorders
        (&self,
         input: &DescribeConfigurationRecordersRequest)
         -> Result<DescribeConfigurationRecordersResponse, DescribeConfigurationRecordersError> {
        let mut request = SignedRequest::new("POST", "config", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "StarlingDoveService.DescribeConfigurationRecorders");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeConfigurationRecordersResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DescribeConfigurationRecordersError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Returns the current status of the specified delivery channel. If a delivery channel is not specified, this action returns the current status of all delivery channels associated with the account.</p> <note> <p>Currently, you can specify only one delivery channel per region in your account.</p> </note>"]
    fn describe_delivery_channel_status
        (&self,
         input: &DescribeDeliveryChannelStatusRequest)
         -> Result<DescribeDeliveryChannelStatusResponse, DescribeDeliveryChannelStatusError> {
        let mut request = SignedRequest::new("POST", "config", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "StarlingDoveService.DescribeDeliveryChannelStatus");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeDeliveryChannelStatusResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DescribeDeliveryChannelStatusError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Returns details about the specified delivery channel. If a delivery channel is not specified, this action returns the details of all delivery channels associated with the account.</p> <note> <p>Currently, you can specify only one delivery channel per region in your account.</p> </note>"]
    fn describe_delivery_channels
        (&self,
         input: &DescribeDeliveryChannelsRequest)
         -> Result<DescribeDeliveryChannelsResponse, DescribeDeliveryChannelsError> {
        let mut request = SignedRequest::new("POST", "config", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "StarlingDoveService.DescribeDeliveryChannels");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeDeliveryChannelsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DescribeDeliveryChannelsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Returns the evaluation results for the specified AWS Config rule. The results indicate which AWS resources were evaluated by the rule, when each resource was last evaluated, and whether each resource complies with the rule.</p>"]
    fn get_compliance_details_by_config_rule
        (&self,
         input: &GetComplianceDetailsByConfigRuleRequest)
         -> Result<GetComplianceDetailsByConfigRuleResponse, GetComplianceDetailsByConfigRuleError> {
        let mut request = SignedRequest::new("POST", "config", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "StarlingDoveService.GetComplianceDetailsByConfigRule");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetComplianceDetailsByConfigRuleResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(GetComplianceDetailsByConfigRuleError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Returns the evaluation results for the specified AWS resource. The results indicate which AWS Config rules were used to evaluate the resource, when each rule was last used, and whether the resource complies with each rule.</p>"]
    fn get_compliance_details_by_resource
        (&self,
         input: &GetComplianceDetailsByResourceRequest)
         -> Result<GetComplianceDetailsByResourceResponse, GetComplianceDetailsByResourceError> {
        let mut request = SignedRequest::new("POST", "config", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "StarlingDoveService.GetComplianceDetailsByResource");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetComplianceDetailsByResourceResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(GetComplianceDetailsByResourceError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Returns the number of AWS Config rules that are compliant and noncompliant, up to a maximum of 25 for each.</p>"]
    fn get_compliance_summary_by_config_rule
        (&self)
         -> Result<GetComplianceSummaryByConfigRuleResponse, GetComplianceSummaryByConfigRuleError> {
        let mut request = SignedRequest::new("POST", "config", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "StarlingDoveService.GetComplianceSummaryByConfigRule");

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetComplianceSummaryByConfigRuleResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(GetComplianceSummaryByConfigRuleError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Returns the number of resources that are compliant and the number that are noncompliant. You can specify one or more resource types to get these numbers for each resource type. The maximum number returned is 100.</p>"]
    fn get_compliance_summary_by_resource_type
        (&self,
         input: &GetComplianceSummaryByResourceTypeRequest)
         -> Result<GetComplianceSummaryByResourceTypeResponse,
                   GetComplianceSummaryByResourceTypeError> {
        let mut request = SignedRequest::new("POST", "config", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "StarlingDoveService.GetComplianceSummaryByResourceType");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetComplianceSummaryByResourceTypeResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(GetComplianceSummaryByResourceTypeError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Returns a list of configuration items for the specified resource. The list contains details about each state of the resource during the specified time interval.</p> <p>The response is paginated, and by default, AWS Config returns a limit of 10 configuration items per page. You can customize this number with the <code>limit</code> parameter. The response includes a <code>nextToken</code> string, and to get the next page of results, run the request again and enter this string for the <code>nextToken</code> parameter.</p> <note> <p>Each call to the API is limited to span a duration of seven days. It is likely that the number of records returned is smaller than the specified <code>limit</code>. In such cases, you can make another call, using the <code>nextToken</code>.</p> </note>"]
    fn get_resource_config_history
        (&self,
         input: &GetResourceConfigHistoryRequest)
         -> Result<GetResourceConfigHistoryResponse, GetResourceConfigHistoryError> {
        let mut request = SignedRequest::new("POST", "config", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "StarlingDoveService.GetResourceConfigHistory");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetResourceConfigHistoryResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(GetResourceConfigHistoryError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Accepts a resource type and returns a list of resource identifiers for the resources of that type. A resource identifier includes the resource type, ID, and (if available) the custom resource name. The results consist of resources that AWS Config has discovered, including those that AWS Config is not currently recording. You can narrow the results to include only resources that have specific resource IDs or a resource name.</p> <note> <p>You can specify either resource IDs or a resource name but not both in the same request.</p> </note> <p>The response is paginated, and by default AWS Config lists 100 resource identifiers on each page. You can customize this number with the <code>limit</code> parameter. The response includes a <code>nextToken</code> string, and to get the next page of results, run the request again and enter this string for the <code>nextToken</code> parameter.</p>"]
    fn list_discovered_resources
        (&self,
         input: &ListDiscoveredResourcesRequest)
         -> Result<ListDiscoveredResourcesResponse, ListDiscoveredResourcesError> {
        let mut request = SignedRequest::new("POST", "config", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "StarlingDoveService.ListDiscoveredResources");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListDiscoveredResourcesResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(ListDiscoveredResourcesError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Adds or updates an AWS Config rule for evaluating whether your AWS resources comply with your desired configurations.</p> <p>You can use this action for custom Config rules and AWS managed Config rules. A custom Config rule is a rule that you develop and maintain. An AWS managed Config rule is a customizable, predefined rule that AWS Config provides.</p> <p>If you are adding a new custom Config rule, you must first create the AWS Lambda function that the rule invokes to evaluate your resources. When you use the <code>PutConfigRule</code> action to add the rule to AWS Config, you must specify the Amazon Resource Name (ARN) that AWS Lambda assigns to the function. Specify the ARN for the <code>SourceIdentifier</code> key. This key is part of the <code>Source</code> object, which is part of the <code>ConfigRule</code> object. </p> <p>If you are adding an AWS managed Config rule, specify the rule's identifier for the <code>SourceIdentifier</code> key. To reference AWS managed Config rule identifiers, see <a href=\"http://docs.aws.amazon.com/config/latest/developerguide/evaluate-config_use-managed-rules.html\">About AWS Managed Config Rules</a>.</p> <p>For any new rule that you add, specify the <code>ConfigRuleName</code> in the <code>ConfigRule</code> object. Do not specify the <code>ConfigRuleArn</code> or the <code>ConfigRuleId</code>. These values are generated by AWS Config for new rules.</p> <p>If you are updating a rule that you added previously, you can specify the rule by <code>ConfigRuleName</code>, <code>ConfigRuleId</code>, or <code>ConfigRuleArn</code> in the <code>ConfigRule</code> data type that you use in this request.</p> <p>The maximum number of rules that AWS Config supports is 50.</p> <p>For more information about requesting a rule limit increase, see <a href=\"http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_config\">AWS Config Limits</a> in the <i>AWS General Reference Guide</i>.</p> <p>For more information about developing and using AWS Config rules, see <a href=\"http://docs.aws.amazon.com/config/latest/developerguide/evaluate-config.html\">Evaluating AWS Resource Configurations with AWS Config</a> in the <i>AWS Config Developer Guide</i>.</p>"]
    fn put_config_rule(&self, input: &PutConfigRuleRequest) -> Result<(), PutConfigRuleError> {
        let mut request = SignedRequest::new("POST", "config", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StarlingDoveService.PutConfigRule");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => {
                Err(PutConfigRuleError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Creates a new configuration recorder to record the selected resource configurations.</p> <p>You can use this action to change the role <code>roleARN</code> and/or the <code>recordingGroup</code> of an existing recorder. To change the role, call the action on the existing configuration recorder and specify a role.</p> <note> <p>Currently, you can specify only one configuration recorder per region in your account.</p> <p>If <code>ConfigurationRecorder</code> does not have the <b>recordingGroup</b> parameter specified, the default is to record all supported resource types.</p> </note>"]
    fn put_configuration_recorder(&self,
                                  input: &PutConfigurationRecorderRequest)
                                  -> Result<(), PutConfigurationRecorderError> {
        let mut request = SignedRequest::new("POST", "config", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "StarlingDoveService.PutConfigurationRecorder");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => Err(PutConfigurationRecorderError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Creates a delivery channel object to deliver configuration information to an Amazon S3 bucket and Amazon SNS topic.</p> <p>Before you can create a delivery channel, you must create a configuration recorder.</p> <p>You can use this action to change the Amazon S3 bucket or an Amazon SNS topic of the existing delivery channel. To change the Amazon S3 bucket or an Amazon SNS topic, call this action and specify the changed values for the S3 bucket and the SNS topic. If you specify a different value for either the S3 bucket or the SNS topic, this action will keep the existing value for the parameter that is not changed.</p> <note> <p>You can have only one delivery channel per region in your account.</p> </note>"]
    fn put_delivery_channel(&self,
                            input: &PutDeliveryChannelRequest)
                            -> Result<(), PutDeliveryChannelError> {
        let mut request = SignedRequest::new("POST", "config", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StarlingDoveService.PutDeliveryChannel");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => {
                Err(PutDeliveryChannelError::from_body(String::from_utf8_lossy(&response.body)
                                                           .as_ref()))
            }
        }
    }


    #[doc="<p>Used by an AWS Lambda function to deliver evaluation results to AWS Config. This action is required in every AWS Lambda function that is invoked by an AWS Config rule.</p>"]
    fn put_evaluations(&self,
                       input: &PutEvaluationsRequest)
                       -> Result<PutEvaluationsResponse, PutEvaluationsError> {
        let mut request = SignedRequest::new("POST", "config", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StarlingDoveService.PutEvaluations");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<PutEvaluationsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(PutEvaluationsError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Runs an on-demand evaluation for the specified Config rules against the last known configuration state of the resources. Use <code>StartConfigRulesEvaluation</code> when you want to test a rule that you updated is working as expected. <code>StartConfigRulesEvaluation</code> does not re-record the latest configuration state for your resources; it re-runs an evaluation against the last known state of your resources. </p> <p>You can specify up to 25 Config rules per request. </p> <p>An existing <code>StartConfigRulesEvaluation</code> call must complete for the specified rules before you can call the API again. If you chose to have AWS Config stream to an Amazon SNS topic, you will receive a <code>ConfigRuleEvaluationStarted</code> notification when the evaluation starts.</p> <note> <p>You don't need to call the <code>StartConfigRulesEvaluation</code> API to run an evaluation for a new rule. When you create a new rule, AWS Config automatically evaluates your resources against the rule. </p> </note> <p>The <code>StartConfigRulesEvaluation</code> API is useful if you want to run on-demand evaluations, such as the following example:</p> <ol> <li> <p>You have a custom rule that evaluates your IAM resources every 24 hours.</p> </li> <li> <p>You update your Lambda function to add additional conditions to your rule.</p> </li> <li> <p>Instead of waiting for the next periodic evaluation, you call the <code>StartConfigRulesEvaluation</code> API.</p> </li> <li> <p>AWS Config invokes your Lambda function and evaluates your IAM resources.</p> </li> <li> <p>Your custom rule will still run periodic evaluations every 24 hours.</p> </li> </ol>"]
    fn start_config_rules_evaluation
        (&self,
         input: &StartConfigRulesEvaluationRequest)
         -> Result<StartConfigRulesEvaluationResponse, StartConfigRulesEvaluationError> {
        let mut request = SignedRequest::new("POST", "config", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "StarlingDoveService.StartConfigRulesEvaluation");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<StartConfigRulesEvaluationResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(StartConfigRulesEvaluationError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Starts recording configurations of the AWS resources you have selected to record in your AWS account.</p> <p>You must have created at least one delivery channel to successfully start the configuration recorder.</p>"]
    fn start_configuration_recorder(&self,
                                    input: &StartConfigurationRecorderRequest)
                                    -> Result<(), StartConfigurationRecorderError> {
        let mut request = SignedRequest::new("POST", "config", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "StarlingDoveService.StartConfigurationRecorder");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => Err(StartConfigurationRecorderError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Stops recording configurations of the AWS resources you have selected to record in your AWS account.</p>"]
    fn stop_configuration_recorder(&self,
                                   input: &StopConfigurationRecorderRequest)
                                   -> Result<(), StopConfigurationRecorderError> {
        let mut request = SignedRequest::new("POST", "config", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "StarlingDoveService.StopConfigurationRecorder");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => Err(StopConfigurationRecorderError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }
}

#[cfg(test)]
mod protocol_tests {}
