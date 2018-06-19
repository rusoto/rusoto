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
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AddAttributesToFindingsRequest {
    /// <p>The array of attributes that you want to assign to specified findings.</p>
    #[serde(rename = "attributes")]
    pub attributes: Vec<Attribute>,
    /// <p>The ARNs that specify the findings that you want to assign attributes to.</p>
    #[serde(rename = "findingArns")]
    pub finding_arns: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AddAttributesToFindingsResponse {
    /// <p>Attribute details that cannot be described. An error code is provided for each failed item.</p>
    #[serde(rename = "failedItems")]
    pub failed_items: ::std::collections::HashMap<String, FailedItemDetails>,
}

/// <p>Used in the exception error that is thrown if you start an assessment run for an assessment target that includes an EC2 instance that is already participating in another started assessment run.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AgentAlreadyRunningAssessment {
    /// <p>ID of the agent that is running on an EC2 instance that is already participating in another started assessment run.</p>
    pub agent_id: String,
    /// <p>The ARN of the assessment run that has already been started.</p>
    pub assessment_run_arn: String,
}

/// <p>Contains information about an Amazon Inspector agent. This data type is used as a request parameter in the <a>ListAssessmentRunAgents</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AgentFilter {
    /// <p>The detailed health state of the agent. Values can be set to <b>IDLE</b>, <b>RUNNING</b>, <b>SHUTDOWN</b>, <b>UNHEALTHY</b>, <b>THROTTLED</b>, and <b>UNKNOWN</b>. </p>
    #[serde(rename = "agentHealthCodes")]
    pub agent_health_codes: Vec<String>,
    /// <p>The current health state of the agent. Values can be set to <b>HEALTHY</b> or <b>UNHEALTHY</b>.</p>
    #[serde(rename = "agentHealths")]
    pub agent_healths: Vec<String>,
}

/// <p>Used as a response element in the <a>PreviewAgents</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AgentPreview {
    /// <p>The health status of the Amazon Inspector Agent.</p>
    #[serde(rename = "agentHealth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_health: Option<String>,
    /// <p>The ID of the EC2 instance where the agent is installed.</p>
    #[serde(rename = "agentId")]
    pub agent_id: String,
    /// <p>The version of the Amazon Inspector Agent.</p>
    #[serde(rename = "agentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    /// <p>The Auto Scaling group for the EC2 instance where the agent is installed.</p>
    #[serde(rename = "autoScalingGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_group: Option<String>,
    /// <p>The hostname of the EC2 instance on which the Amazon Inspector Agent is installed.</p>
    #[serde(rename = "hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// <p>The IP address of the EC2 instance on which the Amazon Inspector Agent is installed.</p>
    #[serde(rename = "ipv4Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv_4_address: Option<String>,
    /// <p>The kernel version of the operating system running on the EC2 instance on which the Amazon Inspector Agent is installed.</p>
    #[serde(rename = "kernelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_version: Option<String>,
    /// <p>The operating system running on the EC2 instance on which the Amazon Inspector Agent is installed.</p>
    #[serde(rename = "operatingSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
}

/// <p>A snapshot of an Amazon Inspector assessment run that contains the findings of the assessment run .</p> <p>Used as the response element in the <a>DescribeAssessmentRuns</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AssessmentRun {
    /// <p>The ARN of the assessment run.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>The ARN of the assessment template that is associated with the assessment run.</p>
    #[serde(rename = "assessmentTemplateArn")]
    pub assessment_template_arn: String,
    /// <p>The assessment run completion time that corresponds to the rules packages evaluation completion time or failure.</p>
    #[serde(rename = "completedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<f64>,
    /// <p>The time when <a>StartAssessmentRun</a> was called.</p>
    #[serde(rename = "createdAt")]
    pub created_at: f64,
    /// <p>A Boolean value (true or false) that specifies whether the process of collecting data from the agents is completed.</p>
    #[serde(rename = "dataCollected")]
    pub data_collected: bool,
    /// <p>The duration of the assessment run.</p>
    #[serde(rename = "durationInSeconds")]
    pub duration_in_seconds: i64,
    /// <p>Provides a total count of generated findings per severity.</p>
    #[serde(rename = "findingCounts")]
    pub finding_counts: ::std::collections::HashMap<String, i64>,
    /// <p>The auto-generated name for the assessment run.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>A list of notifications for the event subscriptions. A notification about a particular generated finding is added to this list only once.</p>
    #[serde(rename = "notifications")]
    pub notifications: Vec<AssessmentRunNotification>,
    /// <p>The rules packages selected for the assessment run.</p>
    #[serde(rename = "rulesPackageArns")]
    pub rules_package_arns: Vec<String>,
    /// <p>The time when <a>StartAssessmentRun</a> was called.</p>
    #[serde(rename = "startedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<f64>,
    /// <p>The state of the assessment run.</p>
    #[serde(rename = "state")]
    pub state: String,
    /// <p>The last time when the assessment run's state changed.</p>
    #[serde(rename = "stateChangedAt")]
    pub state_changed_at: f64,
    /// <p>A list of the assessment run state changes.</p>
    #[serde(rename = "stateChanges")]
    pub state_changes: Vec<AssessmentRunStateChange>,
    /// <p>The user-defined attributes that are assigned to every generated finding.</p>
    #[serde(rename = "userAttributesForFindings")]
    pub user_attributes_for_findings: Vec<Attribute>,
}

/// <p>Contains information about an Amazon Inspector agent. This data type is used as a response element in the <a>ListAssessmentRunAgents</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AssessmentRunAgent {
    /// <p>The current health state of the agent.</p>
    #[serde(rename = "agentHealth")]
    pub agent_health: String,
    /// <p>The detailed health state of the agent.</p>
    #[serde(rename = "agentHealthCode")]
    pub agent_health_code: String,
    /// <p>The description for the agent health code.</p>
    #[serde(rename = "agentHealthDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_health_details: Option<String>,
    /// <p>The AWS account of the EC2 instance where the agent is installed.</p>
    #[serde(rename = "agentId")]
    pub agent_id: String,
    /// <p>The ARN of the assessment run that is associated with the agent.</p>
    #[serde(rename = "assessmentRunArn")]
    pub assessment_run_arn: String,
    /// <p>The Auto Scaling group of the EC2 instance that is specified by the agent ID.</p>
    #[serde(rename = "autoScalingGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_group: Option<String>,
    /// <p>The Amazon Inspector application data metrics that are collected by the agent.</p>
    #[serde(rename = "telemetryMetadata")]
    pub telemetry_metadata: Vec<TelemetryMetadata>,
}

/// <p>Used as the request parameter in the <a>ListAssessmentRuns</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssessmentRunFilter {
    /// <p>For a record to match a filter, the value that is specified for this data type property must inclusively match any value between the specified minimum and maximum values of the <b>completedAt</b> property of the <a>AssessmentRun</a> data type.</p>
    #[serde(rename = "completionTimeRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_time_range: Option<TimestampRange>,
    /// <p>For a record to match a filter, the value that is specified for this data type property must inclusively match any value between the specified minimum and maximum values of the <b>durationInSeconds</b> property of the <a>AssessmentRun</a> data type.</p>
    #[serde(rename = "durationRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_range: Option<DurationRange>,
    /// <p>For a record to match a filter, an explicit value or a string containing a wildcard that is specified for this data type property must match the value of the <b>assessmentRunName</b> property of the <a>AssessmentRun</a> data type.</p>
    #[serde(rename = "namePattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_pattern: Option<String>,
    /// <p>For a record to match a filter, the value that is specified for this data type property must be contained in the list of values of the <b>rulesPackages</b> property of the <a>AssessmentRun</a> data type.</p>
    #[serde(rename = "rulesPackageArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules_package_arns: Option<Vec<String>>,
    /// <p>For a record to match a filter, the value that is specified for this data type property must inclusively match any value between the specified minimum and maximum values of the <b>startTime</b> property of the <a>AssessmentRun</a> data type.</p>
    #[serde(rename = "startTimeRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time_range: Option<TimestampRange>,
    /// <p>For a record to match a filter, the value that is specified for this data type property must match the <b>stateChangedAt</b> property of the <a>AssessmentRun</a> data type.</p>
    #[serde(rename = "stateChangeTimeRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_change_time_range: Option<TimestampRange>,
    /// <p>For a record to match a filter, one of the values specified for this data type property must be the exact match of the value of the <b>assessmentRunState</b> property of the <a>AssessmentRun</a> data type.</p>
    #[serde(rename = "states")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<String>>,
}

/// <p>Used as one of the elements of the <a>AssessmentRun</a> data type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AssessmentRunNotification {
    /// <p>The date of the notification.</p>
    #[serde(rename = "date")]
    pub date: f64,
    /// <p>The Boolean value that specifies whether the notification represents an error.</p>
    #[serde(rename = "error")]
    pub error: bool,
    /// <p>The event for which a notification is sent.</p>
    #[serde(rename = "event")]
    pub event: String,
    /// <p>The message included in the notification.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The status code of the SNS notification.</p>
    #[serde(rename = "snsPublishStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_publish_status_code: Option<String>,
    /// <p>The SNS topic to which the SNS notification is sent.</p>
    #[serde(rename = "snsTopicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
}

/// <p>Used as one of the elements of the <a>AssessmentRun</a> data type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AssessmentRunStateChange {
    /// <p>The assessment run state.</p>
    #[serde(rename = "state")]
    pub state: String,
    /// <p>The last time the assessment run state changed.</p>
    #[serde(rename = "stateChangedAt")]
    pub state_changed_at: f64,
}

/// <p>Contains information about an Amazon Inspector application. This data type is used as the response element in the <a>DescribeAssessmentTargets</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AssessmentTarget {
    /// <p>The ARN that specifies the Amazon Inspector assessment target.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>The time at which the assessment target is created.</p>
    #[serde(rename = "createdAt")]
    pub created_at: f64,
    /// <p>The name of the Amazon Inspector assessment target.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The ARN that specifies the resource group that is associated with the assessment target.</p>
    #[serde(rename = "resourceGroupArn")]
    pub resource_group_arn: String,
    /// <p>The time at which <a>UpdateAssessmentTarget</a> is called.</p>
    #[serde(rename = "updatedAt")]
    pub updated_at: f64,
}

/// <p>Used as the request parameter in the <a>ListAssessmentTargets</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssessmentTargetFilter {
    /// <p>For a record to match a filter, an explicit value or a string that contains a wildcard that is specified for this data type property must match the value of the <b>assessmentTargetName</b> property of the <a>AssessmentTarget</a> data type.</p>
    #[serde(rename = "assessmentTargetNamePattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_target_name_pattern: Option<String>,
}

/// <p>Contains information about an Amazon Inspector assessment template. This data type is used as the response element in the <a>DescribeAssessmentTemplates</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AssessmentTemplate {
    /// <p>The ARN of the assessment template.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>The number of existing assessment runs associated with this assessment template. This value can be zero or a positive integer.</p>
    #[serde(rename = "assessmentRunCount")]
    pub assessment_run_count: i64,
    /// <p>The ARN of the assessment target that corresponds to this assessment template.</p>
    #[serde(rename = "assessmentTargetArn")]
    pub assessment_target_arn: String,
    /// <p>The time at which the assessment template is created.</p>
    #[serde(rename = "createdAt")]
    pub created_at: f64,
    /// <p>The duration in seconds specified for this assessment tempate. The default value is 3600 seconds (one hour). The maximum value is 86400 seconds (one day).</p>
    #[serde(rename = "durationInSeconds")]
    pub duration_in_seconds: i64,
    /// <p>The Amazon Resource Name (ARN) of the most recent assessment run associated with this assessment template. This value exists only when the value of assessmentRunCount is greater than zero.</p>
    #[serde(rename = "lastAssessmentRunArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_assessment_run_arn: Option<String>,
    /// <p>The name of the assessment template.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The rules packages that are specified for this assessment template.</p>
    #[serde(rename = "rulesPackageArns")]
    pub rules_package_arns: Vec<String>,
    /// <p>The user-defined attributes that are assigned to every generated finding from the assessment run that uses this assessment template.</p>
    #[serde(rename = "userAttributesForFindings")]
    pub user_attributes_for_findings: Vec<Attribute>,
}

/// <p>Used as the request parameter in the <a>ListAssessmentTemplates</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssessmentTemplateFilter {
    /// <p>For a record to match a filter, the value specified for this data type property must inclusively match any value between the specified minimum and maximum values of the <b>durationInSeconds</b> property of the <a>AssessmentTemplate</a> data type.</p>
    #[serde(rename = "durationRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_range: Option<DurationRange>,
    /// <p>For a record to match a filter, an explicit value or a string that contains a wildcard that is specified for this data type property must match the value of the <b>assessmentTemplateName</b> property of the <a>AssessmentTemplate</a> data type.</p>
    #[serde(rename = "namePattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_pattern: Option<String>,
    /// <p>For a record to match a filter, the values that are specified for this data type property must be contained in the list of values of the <b>rulesPackageArns</b> property of the <a>AssessmentTemplate</a> data type.</p>
    #[serde(rename = "rulesPackageArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules_package_arns: Option<Vec<String>>,
}

/// <p>A collection of attributes of the host from which the finding is generated.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AssetAttributes {
    /// <p>The ID of the agent that is installed on the EC2 instance where the finding is generated.</p>
    #[serde(rename = "agentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    /// <p>The ID of the Amazon Machine Image (AMI) that is installed on the EC2 instance where the finding is generated.</p>
    #[serde(rename = "amiId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ami_id: Option<String>,
    /// <p>The Auto Scaling group of the EC2 instance where the finding is generated.</p>
    #[serde(rename = "autoScalingGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_group: Option<String>,
    /// <p>The hostname of the EC2 instance where the finding is generated.</p>
    #[serde(rename = "hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// <p>The list of IP v4 addresses of the EC2 instance where the finding is generated.</p>
    #[serde(rename = "ipv4Addresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv_4_addresses: Option<Vec<String>>,
    /// <p>The schema version of this data type.</p>
    #[serde(rename = "schemaVersion")]
    pub schema_version: i64,
}

/// <p>This data type is used as a request parameter in the <a>AddAttributesToFindings</a> and <a>CreateAssessmentTemplate</a> actions.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attribute {
    /// <p>The attribute key.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>The value assigned to the attribute key.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateAssessmentTargetRequest {
    /// <p>The user-defined name that identifies the assessment target that you want to create. The name must be unique within the AWS account.</p>
    #[serde(rename = "assessmentTargetName")]
    pub assessment_target_name: String,
    /// <p>The ARN that specifies the resource group that is used to create the assessment target.</p>
    #[serde(rename = "resourceGroupArn")]
    pub resource_group_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateAssessmentTargetResponse {
    /// <p>The ARN that specifies the assessment target that is created.</p>
    #[serde(rename = "assessmentTargetArn")]
    pub assessment_target_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateAssessmentTemplateRequest {
    /// <p>The ARN that specifies the assessment target for which you want to create the assessment template.</p>
    #[serde(rename = "assessmentTargetArn")]
    pub assessment_target_arn: String,
    /// <p>The user-defined name that identifies the assessment template that you want to create. You can create several assessment templates for an assessment target. The names of the assessment templates that correspond to a particular assessment target must be unique.</p>
    #[serde(rename = "assessmentTemplateName")]
    pub assessment_template_name: String,
    /// <p>The duration of the assessment run in seconds. The default value is 3600 seconds (one hour).</p>
    #[serde(rename = "durationInSeconds")]
    pub duration_in_seconds: i64,
    /// <p>The ARNs that specify the rules packages that you want to attach to the assessment template.</p>
    #[serde(rename = "rulesPackageArns")]
    pub rules_package_arns: Vec<String>,
    /// <p>The user-defined attributes that are assigned to every finding that is generated by the assessment run that uses this assessment template. An attribute is a key and value pair (an <a>Attribute</a> object). Within an assessment template, each key must be unique.</p>
    #[serde(rename = "userAttributesForFindings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_attributes_for_findings: Option<Vec<Attribute>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateAssessmentTemplateResponse {
    /// <p>The ARN that specifies the assessment template that is created.</p>
    #[serde(rename = "assessmentTemplateArn")]
    pub assessment_template_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateResourceGroupRequest {
    /// <p>A collection of keys and an array of possible values, '[{"key":"key1","values":["Value1","Value2"]},{"key":"Key2","values":["Value3"]}]'.</p> <p>For example,'[{"key":"Name","values":["TestEC2Instance"]}]'.</p>
    #[serde(rename = "resourceGroupTags")]
    pub resource_group_tags: Vec<ResourceGroupTag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateResourceGroupResponse {
    /// <p>The ARN that specifies the resource group that is created.</p>
    #[serde(rename = "resourceGroupArn")]
    pub resource_group_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteAssessmentRunRequest {
    /// <p>The ARN that specifies the assessment run that you want to delete.</p>
    #[serde(rename = "assessmentRunArn")]
    pub assessment_run_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteAssessmentTargetRequest {
    /// <p>The ARN that specifies the assessment target that you want to delete.</p>
    #[serde(rename = "assessmentTargetArn")]
    pub assessment_target_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteAssessmentTemplateRequest {
    /// <p>The ARN that specifies the assessment template that you want to delete.</p>
    #[serde(rename = "assessmentTemplateArn")]
    pub assessment_template_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeAssessmentRunsRequest {
    /// <p>The ARN that specifies the assessment run that you want to describe.</p>
    #[serde(rename = "assessmentRunArns")]
    pub assessment_run_arns: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeAssessmentRunsResponse {
    /// <p>Information about the assessment run.</p>
    #[serde(rename = "assessmentRuns")]
    pub assessment_runs: Vec<AssessmentRun>,
    /// <p>Assessment run details that cannot be described. An error code is provided for each failed item.</p>
    #[serde(rename = "failedItems")]
    pub failed_items: ::std::collections::HashMap<String, FailedItemDetails>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeAssessmentTargetsRequest {
    /// <p>The ARNs that specifies the assessment targets that you want to describe.</p>
    #[serde(rename = "assessmentTargetArns")]
    pub assessment_target_arns: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeAssessmentTargetsResponse {
    /// <p>Information about the assessment targets.</p>
    #[serde(rename = "assessmentTargets")]
    pub assessment_targets: Vec<AssessmentTarget>,
    /// <p>Assessment target details that cannot be described. An error code is provided for each failed item.</p>
    #[serde(rename = "failedItems")]
    pub failed_items: ::std::collections::HashMap<String, FailedItemDetails>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeAssessmentTemplatesRequest {
    #[serde(rename = "assessmentTemplateArns")]
    pub assessment_template_arns: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeAssessmentTemplatesResponse {
    /// <p>Information about the assessment templates.</p>
    #[serde(rename = "assessmentTemplates")]
    pub assessment_templates: Vec<AssessmentTemplate>,
    /// <p>Assessment template details that cannot be described. An error code is provided for each failed item.</p>
    #[serde(rename = "failedItems")]
    pub failed_items: ::std::collections::HashMap<String, FailedItemDetails>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeCrossAccountAccessRoleResponse {
    /// <p>The date when the cross-account access role was registered.</p>
    #[serde(rename = "registeredAt")]
    pub registered_at: f64,
    /// <p>The ARN that specifies the IAM role that Amazon Inspector uses to access your AWS account.</p>
    #[serde(rename = "roleArn")]
    pub role_arn: String,
    /// <p>A Boolean value that specifies whether the IAM role has the necessary policies attached to enable Amazon Inspector to access your AWS account.</p>
    #[serde(rename = "valid")]
    pub valid: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeFindingsRequest {
    /// <p>The ARN that specifies the finding that you want to describe.</p>
    #[serde(rename = "findingArns")]
    pub finding_arns: Vec<String>,
    /// <p>The locale into which you want to translate a finding description, recommendation, and the short description that identifies the finding.</p>
    #[serde(rename = "locale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeFindingsResponse {
    /// <p>Finding details that cannot be described. An error code is provided for each failed item.</p>
    #[serde(rename = "failedItems")]
    pub failed_items: ::std::collections::HashMap<String, FailedItemDetails>,
    /// <p>Information about the finding.</p>
    #[serde(rename = "findings")]
    pub findings: Vec<Finding>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeResourceGroupsRequest {
    /// <p>The ARN that specifies the resource group that you want to describe.</p>
    #[serde(rename = "resourceGroupArns")]
    pub resource_group_arns: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeResourceGroupsResponse {
    /// <p>Resource group details that cannot be described. An error code is provided for each failed item.</p>
    #[serde(rename = "failedItems")]
    pub failed_items: ::std::collections::HashMap<String, FailedItemDetails>,
    /// <p>Information about a resource group.</p>
    #[serde(rename = "resourceGroups")]
    pub resource_groups: Vec<ResourceGroup>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeRulesPackagesRequest {
    /// <p>The locale that you want to translate a rules package description into.</p>
    #[serde(rename = "locale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// <p>The ARN that specifies the rules package that you want to describe.</p>
    #[serde(rename = "rulesPackageArns")]
    pub rules_package_arns: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeRulesPackagesResponse {
    /// <p>Rules package details that cannot be described. An error code is provided for each failed item.</p>
    #[serde(rename = "failedItems")]
    pub failed_items: ::std::collections::HashMap<String, FailedItemDetails>,
    /// <p>Information about the rules package.</p>
    #[serde(rename = "rulesPackages")]
    pub rules_packages: Vec<RulesPackage>,
}

/// <p>This data type is used in the <a>AssessmentTemplateFilter</a> data type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DurationRange {
    /// <p>The maximum value of the duration range. Must be less than or equal to 604800 seconds (1 week).</p>
    #[serde(rename = "maxSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_seconds: Option<i64>,
    /// <p>The minimum value of the duration range. Must be greater than zero.</p>
    #[serde(rename = "minSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_seconds: Option<i64>,
}

/// <p>This data type is used in the <a>Subscription</a> data type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct EventSubscription {
    /// <p>The event for which Amazon Simple Notification Service (SNS) notifications are sent.</p>
    #[serde(rename = "event")]
    pub event: String,
    /// <p>The time at which <a>SubscribeToEvent</a> is called.</p>
    #[serde(rename = "subscribedAt")]
    pub subscribed_at: f64,
}

/// <p>Includes details about the failed items.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct FailedItemDetails {
    /// <p>The status code of a failed item.</p>
    #[serde(rename = "failureCode")]
    pub failure_code: String,
    /// <p>Indicates whether you can immediately retry a request for this item for a specified resource.</p>
    #[serde(rename = "retryable")]
    pub retryable: bool,
}

/// <p>Contains information about an Amazon Inspector finding. This data type is used as the response element in the <a>DescribeFindings</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Finding {
    /// <p>The ARN that specifies the finding.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>A collection of attributes of the host from which the finding is generated.</p>
    #[serde(rename = "assetAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_attributes: Option<AssetAttributes>,
    /// <p>The type of the host from which the finding is generated.</p>
    #[serde(rename = "assetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<String>,
    /// <p>The system-defined attributes for the finding.</p>
    #[serde(rename = "attributes")]
    pub attributes: Vec<Attribute>,
    /// <p>This data element is currently not used.</p>
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<i64>,
    /// <p>The time when the finding was generated.</p>
    #[serde(rename = "createdAt")]
    pub created_at: f64,
    /// <p>The description of the finding.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the finding.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>This data element is currently not used.</p>
    #[serde(rename = "indicatorOfCompromise")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indicator_of_compromise: Option<bool>,
    /// <p>The numeric value of the finding severity.</p>
    #[serde(rename = "numericSeverity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeric_severity: Option<f64>,
    /// <p>The recommendation for the finding.</p>
    #[serde(rename = "recommendation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation: Option<String>,
    /// <p>The schema version of this data type.</p>
    #[serde(rename = "schemaVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<i64>,
    /// <p>The data element is set to "Inspector".</p>
    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    /// <p>This data type is used in the <a>Finding</a> data type.</p>
    #[serde(rename = "serviceAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_attributes: Option<InspectorServiceAttributes>,
    /// <p>The finding severity. Values can be set to High, Medium, Low, and Informational.</p>
    #[serde(rename = "severity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    /// <p>The name of the finding.</p>
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// <p>The time when <a>AddAttributesToFindings</a> is called.</p>
    #[serde(rename = "updatedAt")]
    pub updated_at: f64,
    /// <p>The user-defined attributes that are assigned to the finding.</p>
    #[serde(rename = "userAttributes")]
    pub user_attributes: Vec<Attribute>,
}

/// <p>This data type is used as a request parameter in the <a>ListFindings</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct FindingFilter {
    /// <p>For a record to match a filter, one of the values that is specified for this data type property must be the exact match of the value of the <b>agentId</b> property of the <a>Finding</a> data type.</p>
    #[serde(rename = "agentIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ids: Option<Vec<String>>,
    /// <p>For a record to match a filter, the list of values that are specified for this data type property must be contained in the list of values of the <b>attributes</b> property of the <a>Finding</a> data type.</p>
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
    /// <p>For a record to match a filter, one of the values that is specified for this data type property must be the exact match of the value of the <b>autoScalingGroup</b> property of the <a>Finding</a> data type.</p>
    #[serde(rename = "autoScalingGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_groups: Option<Vec<String>>,
    /// <p>The time range during which the finding is generated.</p>
    #[serde(rename = "creationTimeRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_range: Option<TimestampRange>,
    /// <p>For a record to match a filter, one of the values that is specified for this data type property must be the exact match of the value of the <b>ruleName</b> property of the <a>Finding</a> data type.</p>
    #[serde(rename = "ruleNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_names: Option<Vec<String>>,
    /// <p>For a record to match a filter, one of the values that is specified for this data type property must be the exact match of the value of the <b>rulesPackageArn</b> property of the <a>Finding</a> data type.</p>
    #[serde(rename = "rulesPackageArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules_package_arns: Option<Vec<String>>,
    /// <p>For a record to match a filter, one of the values that is specified for this data type property must be the exact match of the value of the <b>severity</b> property of the <a>Finding</a> data type.</p>
    #[serde(rename = "severities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severities: Option<Vec<String>>,
    /// <p>For a record to match a filter, the value that is specified for this data type property must be contained in the list of values of the <b>userAttributes</b> property of the <a>Finding</a> data type.</p>
    #[serde(rename = "userAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_attributes: Option<Vec<Attribute>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAssessmentReportRequest {
    /// <p>The ARN that specifies the assessment run for which you want to generate a report.</p>
    #[serde(rename = "assessmentRunArn")]
    pub assessment_run_arn: String,
    /// <p>Specifies the file format (html or pdf) of the assessment report that you want to generate.</p>
    #[serde(rename = "reportFileFormat")]
    pub report_file_format: String,
    /// <p>Specifies the type of the assessment report that you want to generate. There are two types of assessment reports: a finding report and a full report. For more information, see <a href="http://docs.aws.amazon.com/inspector/latest/userguide/inspector_reports.html">Assessment Reports</a>. </p>
    #[serde(rename = "reportType")]
    pub report_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetAssessmentReportResponse {
    /// <p>Specifies the status of the request to generate an assessment report. </p>
    #[serde(rename = "status")]
    pub status: String,
    /// <p>Specifies the URL where you can find the generated assessment report. This parameter is only returned if the report is successfully generated.</p>
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetTelemetryMetadataRequest {
    /// <p>The ARN that specifies the assessment run that has the telemetry data that you want to obtain.</p>
    #[serde(rename = "assessmentRunArn")]
    pub assessment_run_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetTelemetryMetadataResponse {
    /// <p>Telemetry details.</p>
    #[serde(rename = "telemetryMetadata")]
    pub telemetry_metadata: Vec<TelemetryMetadata>,
}

/// <p>This data type is used in the <a>Finding</a> data type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct InspectorServiceAttributes {
    /// <p>The ARN of the assessment run during which the finding is generated.</p>
    #[serde(rename = "assessmentRunArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_run_arn: Option<String>,
    /// <p>The ARN of the rules package that is used to generate the finding.</p>
    #[serde(rename = "rulesPackageArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules_package_arn: Option<String>,
    /// <p>The schema version of this data type.</p>
    #[serde(rename = "schemaVersion")]
    pub schema_version: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListAssessmentRunAgentsRequest {
    /// <p>The ARN that specifies the assessment run whose agents you want to list.</p>
    #[serde(rename = "assessmentRunArn")]
    pub assessment_run_arn: String,
    /// <p>You can use this parameter to specify a subset of data to be included in the action's response.</p> <p>For a record to match a filter, all specified filter attributes must match. When multiple values are specified for a filter attribute, any of the values can match.</p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<AgentFilter>,
    /// <p>You can use this parameter to indicate the maximum number of items that you want in the response. The default value is 10. The maximum value is 500.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>You can use this parameter when paginating results. Set the value of this parameter to null on your first call to the <b>ListAssessmentRunAgents</b> action. Subsequent calls to the action fill <b>nextToken</b> in the request with the value of <b>NextToken</b> from the previous response to continue listing data.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListAssessmentRunAgentsResponse {
    /// <p>A list of ARNs that specifies the agents returned by the action.</p>
    #[serde(rename = "assessmentRunAgents")]
    pub assessment_run_agents: Vec<AssessmentRunAgent>,
    /// <p> When a response is generated, if there is more data to be listed, this parameter is present in the response and contains the value to use for the <b>nextToken</b> parameter in a subsequent pagination request. If there is no more data to be listed, this parameter is set to null.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListAssessmentRunsRequest {
    /// <p>The ARNs that specify the assessment templates whose assessment runs you want to list.</p>
    #[serde(rename = "assessmentTemplateArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_template_arns: Option<Vec<String>>,
    /// <p>You can use this parameter to specify a subset of data to be included in the action's response.</p> <p>For a record to match a filter, all specified filter attributes must match. When multiple values are specified for a filter attribute, any of the values can match.</p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<AssessmentRunFilter>,
    /// <p>You can use this parameter to indicate the maximum number of items that you want in the response. The default value is 10. The maximum value is 500.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>You can use this parameter when paginating results. Set the value of this parameter to null on your first call to the <b>ListAssessmentRuns</b> action. Subsequent calls to the action fill <b>nextToken</b> in the request with the value of <b>NextToken</b> from the previous response to continue listing data.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListAssessmentRunsResponse {
    /// <p>A list of ARNs that specifies the assessment runs that are returned by the action.</p>
    #[serde(rename = "assessmentRunArns")]
    pub assessment_run_arns: Vec<String>,
    /// <p> When a response is generated, if there is more data to be listed, this parameter is present in the response and contains the value to use for the <b>nextToken</b> parameter in a subsequent pagination request. If there is no more data to be listed, this parameter is set to null.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListAssessmentTargetsRequest {
    /// <p>You can use this parameter to specify a subset of data to be included in the action's response.</p> <p>For a record to match a filter, all specified filter attributes must match. When multiple values are specified for a filter attribute, any of the values can match.</p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<AssessmentTargetFilter>,
    /// <p>You can use this parameter to indicate the maximum number of items you want in the response. The default value is 10. The maximum value is 500.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>You can use this parameter when paginating results. Set the value of this parameter to null on your first call to the <b>ListAssessmentTargets</b> action. Subsequent calls to the action fill <b>nextToken</b> in the request with the value of <b>NextToken</b> from the previous response to continue listing data.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListAssessmentTargetsResponse {
    /// <p>A list of ARNs that specifies the assessment targets that are returned by the action.</p>
    #[serde(rename = "assessmentTargetArns")]
    pub assessment_target_arns: Vec<String>,
    /// <p> When a response is generated, if there is more data to be listed, this parameter is present in the response and contains the value to use for the <b>nextToken</b> parameter in a subsequent pagination request. If there is no more data to be listed, this parameter is set to null.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListAssessmentTemplatesRequest {
    /// <p>A list of ARNs that specifies the assessment targets whose assessment templates you want to list.</p>
    #[serde(rename = "assessmentTargetArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_target_arns: Option<Vec<String>>,
    /// <p>You can use this parameter to specify a subset of data to be included in the action's response.</p> <p>For a record to match a filter, all specified filter attributes must match. When multiple values are specified for a filter attribute, any of the values can match.</p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<AssessmentTemplateFilter>,
    /// <p>You can use this parameter to indicate the maximum number of items you want in the response. The default value is 10. The maximum value is 500.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>You can use this parameter when paginating results. Set the value of this parameter to null on your first call to the <b>ListAssessmentTemplates</b> action. Subsequent calls to the action fill <b>nextToken</b> in the request with the value of <b>NextToken</b> from the previous response to continue listing data.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListAssessmentTemplatesResponse {
    /// <p>A list of ARNs that specifies the assessment templates returned by the action.</p>
    #[serde(rename = "assessmentTemplateArns")]
    pub assessment_template_arns: Vec<String>,
    /// <p> When a response is generated, if there is more data to be listed, this parameter is present in the response and contains the value to use for the <b>nextToken</b> parameter in a subsequent pagination request. If there is no more data to be listed, this parameter is set to null.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListEventSubscriptionsRequest {
    /// <p>You can use this parameter to indicate the maximum number of items you want in the response. The default value is 10. The maximum value is 500.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>You can use this parameter when paginating results. Set the value of this parameter to null on your first call to the <b>ListEventSubscriptions</b> action. Subsequent calls to the action fill <b>nextToken</b> in the request with the value of <b>NextToken</b> from the previous response to continue listing data.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ARN of the assessment template for which you want to list the existing event subscriptions.</p>
    #[serde(rename = "resourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListEventSubscriptionsResponse {
    /// <p> When a response is generated, if there is more data to be listed, this parameter is present in the response and contains the value to use for the <b>nextToken</b> parameter in a subsequent pagination request. If there is no more data to be listed, this parameter is set to null.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Details of the returned event subscriptions.</p>
    #[serde(rename = "subscriptions")]
    pub subscriptions: Vec<Subscription>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListFindingsRequest {
    /// <p>The ARNs of the assessment runs that generate the findings that you want to list.</p>
    #[serde(rename = "assessmentRunArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_run_arns: Option<Vec<String>>,
    /// <p>You can use this parameter to specify a subset of data to be included in the action's response.</p> <p>For a record to match a filter, all specified filter attributes must match. When multiple values are specified for a filter attribute, any of the values can match.</p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<FindingFilter>,
    /// <p>You can use this parameter to indicate the maximum number of items you want in the response. The default value is 10. The maximum value is 500.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>You can use this parameter when paginating results. Set the value of this parameter to null on your first call to the <b>ListFindings</b> action. Subsequent calls to the action fill <b>nextToken</b> in the request with the value of <b>NextToken</b> from the previous response to continue listing data.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListFindingsResponse {
    /// <p>A list of ARNs that specifies the findings returned by the action.</p>
    #[serde(rename = "findingArns")]
    pub finding_arns: Vec<String>,
    /// <p> When a response is generated, if there is more data to be listed, this parameter is present in the response and contains the value to use for the <b>nextToken</b> parameter in a subsequent pagination request. If there is no more data to be listed, this parameter is set to null.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListRulesPackagesRequest {
    /// <p>You can use this parameter to indicate the maximum number of items you want in the response. The default value is 10. The maximum value is 500.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>You can use this parameter when paginating results. Set the value of this parameter to null on your first call to the <b>ListRulesPackages</b> action. Subsequent calls to the action fill <b>nextToken</b> in the request with the value of <b>NextToken</b> from the previous response to continue listing data.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListRulesPackagesResponse {
    /// <p> When a response is generated, if there is more data to be listed, this parameter is present in the response and contains the value to use for the <b>nextToken</b> parameter in a subsequent pagination request. If there is no more data to be listed, this parameter is set to null.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of ARNs that specifies the rules packages returned by the action.</p>
    #[serde(rename = "rulesPackageArns")]
    pub rules_package_arns: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTagsForResourceRequest {
    /// <p>The ARN that specifies the assessment template whose tags you want to list.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListTagsForResourceResponse {
    /// <p>A collection of key and value pairs.</p>
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PreviewAgentsRequest {
    /// <p>You can use this parameter to indicate the maximum number of items you want in the response. The default value is 10. The maximum value is 500.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>You can use this parameter when paginating results. Set the value of this parameter to null on your first call to the <b>PreviewAgents</b> action. Subsequent calls to the action fill <b>nextToken</b> in the request with the value of <b>NextToken</b> from the previous response to continue listing data.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ARN of the assessment target whose agents you want to preview.</p>
    #[serde(rename = "previewAgentsArn")]
    pub preview_agents_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PreviewAgentsResponse {
    /// <p>The resulting list of agents.</p>
    #[serde(rename = "agentPreviews")]
    pub agent_previews: Vec<AgentPreview>,
    /// <p> When a response is generated, if there is more data to be listed, this parameter is present in the response and contains the value to use for the <b>nextToken</b> parameter in a subsequent pagination request. If there is no more data to be listed, this parameter is set to null.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RegisterCrossAccountAccessRoleRequest {
    /// <p>The ARN of the IAM role that grants Amazon Inspector access to AWS Services needed to perform security assessments. </p>
    #[serde(rename = "roleArn")]
    pub role_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RemoveAttributesFromFindingsRequest {
    /// <p>The array of attribute keys that you want to remove from specified findings.</p>
    #[serde(rename = "attributeKeys")]
    pub attribute_keys: Vec<String>,
    /// <p>The ARNs that specify the findings that you want to remove attributes from.</p>
    #[serde(rename = "findingArns")]
    pub finding_arns: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RemoveAttributesFromFindingsResponse {
    /// <p>Attributes details that cannot be described. An error code is provided for each failed item.</p>
    #[serde(rename = "failedItems")]
    pub failed_items: ::std::collections::HashMap<String, FailedItemDetails>,
}

/// <p>Contains information about a resource group. The resource group defines a set of tags that, when queried, identify the AWS resources that make up the assessment target. This data type is used as the response element in the <a>DescribeResourceGroups</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ResourceGroup {
    /// <p>The ARN of the resource group.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>The time at which resource group is created.</p>
    #[serde(rename = "createdAt")]
    pub created_at: f64,
    /// <p>The tags (key and value pairs) of the resource group. This data type property is used in the <a>CreateResourceGroup</a> action.</p>
    #[serde(rename = "tags")]
    pub tags: Vec<ResourceGroupTag>,
}

/// <p>This data type is used as one of the elements of the <a>ResourceGroup</a> data type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceGroupTag {
    /// <p>A tag key.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>The value assigned to a tag key.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Contains information about an Amazon Inspector rules package. This data type is used as the response element in the <a>DescribeRulesPackages</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RulesPackage {
    /// <p>The ARN of the rules package.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>The description of the rules package.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the rules package.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The provider of the rules package.</p>
    #[serde(rename = "provider")]
    pub provider: String,
    /// <p>The version ID of the rules package.</p>
    #[serde(rename = "version")]
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SetTagsForResourceRequest {
    /// <p>The ARN of the assessment template that you want to set tags to.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>A collection of key and value pairs that you want to set to the assessment template.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartAssessmentRunRequest {
    /// <p>You can specify the name for the assessment run. The name must be unique for the assessment template whose ARN is used to start the assessment run.</p>
    #[serde(rename = "assessmentRunName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_run_name: Option<String>,
    /// <p>The ARN of the assessment template of the assessment run that you want to start.</p>
    #[serde(rename = "assessmentTemplateArn")]
    pub assessment_template_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StartAssessmentRunResponse {
    /// <p>The ARN of the assessment run that has been started.</p>
    #[serde(rename = "assessmentRunArn")]
    pub assessment_run_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopAssessmentRunRequest {
    /// <p>The ARN of the assessment run that you want to stop.</p>
    #[serde(rename = "assessmentRunArn")]
    pub assessment_run_arn: String,
    /// <p>An input option that can be set to either START_EVALUATION or SKIP_EVALUATION. START_EVALUATION (the default value), stops the AWS agent from collecting data and begins the results evaluation and the findings generation process. SKIP_EVALUATION cancels the assessment run immediately, after which no findings are generated.</p>
    #[serde(rename = "stopAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_action: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SubscribeToEventRequest {
    /// <p>The event for which you want to receive SNS notifications.</p>
    #[serde(rename = "event")]
    pub event: String,
    /// <p>The ARN of the assessment template that is used during the event for which you want to receive SNS notifications.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The ARN of the SNS topic to which the SNS notifications are sent.</p>
    #[serde(rename = "topicArn")]
    pub topic_arn: String,
}

/// <p>This data type is used as a response element in the <a>ListEventSubscriptions</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Subscription {
    /// <p>The list of existing event subscriptions.</p>
    #[serde(rename = "eventSubscriptions")]
    pub event_subscriptions: Vec<EventSubscription>,
    /// <p>The ARN of the assessment template that is used during the event for which the SNS notification is sent.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The ARN of the Amazon Simple Notification Service (SNS) topic to which the SNS notifications are sent.</p>
    #[serde(rename = "topicArn")]
    pub topic_arn: String,
}

/// <p>A key and value pair. This data type is used as a request parameter in the <a>SetTagsForResource</a> action and a response element in the <a>ListTagsForResource</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>A tag key.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>A value assigned to a tag key.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>The metadata about the Amazon Inspector application data metrics collected by the agent. This data type is used as the response element in the <a>GetTelemetryMetadata</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TelemetryMetadata {
    /// <p>The count of messages that the agent sends to the Amazon Inspector service.</p>
    #[serde(rename = "count")]
    pub count: i64,
    /// <p>The data size of messages that the agent sends to the Amazon Inspector service.</p>
    #[serde(rename = "dataSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_size: Option<i64>,
    /// <p>A specific type of behavioral data that is collected by the agent.</p>
    #[serde(rename = "messageType")]
    pub message_type: String,
}

/// <p>This data type is used in the <a>AssessmentRunFilter</a> data type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TimestampRange {
    /// <p>The minimum value of the timestamp range.</p>
    #[serde(rename = "beginDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_date: Option<f64>,
    /// <p>The maximum value of the timestamp range.</p>
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UnsubscribeFromEventRequest {
    /// <p>The event for which you want to stop receiving SNS notifications.</p>
    #[serde(rename = "event")]
    pub event: String,
    /// <p>The ARN of the assessment template that is used during the event for which you want to stop receiving SNS notifications.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The ARN of the SNS topic to which SNS notifications are sent.</p>
    #[serde(rename = "topicArn")]
    pub topic_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateAssessmentTargetRequest {
    /// <p>The ARN of the assessment target that you want to update.</p>
    #[serde(rename = "assessmentTargetArn")]
    pub assessment_target_arn: String,
    /// <p>The name of the assessment target that you want to update.</p>
    #[serde(rename = "assessmentTargetName")]
    pub assessment_target_name: String,
    /// <p>The ARN of the resource group that is used to specify the new resource group to associate with the assessment target.</p>
    #[serde(rename = "resourceGroupArn")]
    pub resource_group_arn: String,
}

/// Errors returned by AddAttributesToFindings
#[derive(Debug, PartialEq)]
pub enum AddAttributesToFindingsError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AddAttributesToFindingsError {
    pub fn from_body(body: &str) -> AddAttributesToFindingsError {
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
                    "AccessDeniedException" => {
                        AddAttributesToFindingsError::AccessDenied(String::from(error_message))
                    }
                    "InternalException" => {
                        AddAttributesToFindingsError::Internal(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        AddAttributesToFindingsError::InvalidInput(String::from(error_message))
                    }
                    "NoSuchEntityException" => {
                        AddAttributesToFindingsError::NoSuchEntity(String::from(error_message))
                    }
                    "ValidationException" => {
                        AddAttributesToFindingsError::Validation(error_message.to_string())
                    }
                    _ => AddAttributesToFindingsError::Unknown(String::from(body)),
                }
            }
            Err(_) => AddAttributesToFindingsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AddAttributesToFindingsError {
    fn from(err: serde_json::error::Error) -> AddAttributesToFindingsError {
        AddAttributesToFindingsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AddAttributesToFindingsError {
    fn from(err: CredentialsError) -> AddAttributesToFindingsError {
        AddAttributesToFindingsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AddAttributesToFindingsError {
    fn from(err: HttpDispatchError) -> AddAttributesToFindingsError {
        AddAttributesToFindingsError::HttpDispatch(err)
    }
}
impl From<io::Error> for AddAttributesToFindingsError {
    fn from(err: io::Error) -> AddAttributesToFindingsError {
        AddAttributesToFindingsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AddAttributesToFindingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddAttributesToFindingsError {
    fn description(&self) -> &str {
        match *self {
            AddAttributesToFindingsError::AccessDenied(ref cause) => cause,
            AddAttributesToFindingsError::Internal(ref cause) => cause,
            AddAttributesToFindingsError::InvalidInput(ref cause) => cause,
            AddAttributesToFindingsError::NoSuchEntity(ref cause) => cause,
            AddAttributesToFindingsError::Validation(ref cause) => cause,
            AddAttributesToFindingsError::Credentials(ref err) => err.description(),
            AddAttributesToFindingsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AddAttributesToFindingsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateAssessmentTarget
#[derive(Debug, PartialEq)]
pub enum CreateAssessmentTargetError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateAssessmentTargetError {
    pub fn from_body(body: &str) -> CreateAssessmentTargetError {
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
                    "AccessDeniedException" => {
                        CreateAssessmentTargetError::AccessDenied(String::from(error_message))
                    }
                    "InternalException" => {
                        CreateAssessmentTargetError::Internal(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        CreateAssessmentTargetError::InvalidInput(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateAssessmentTargetError::LimitExceeded(String::from(error_message))
                    }
                    "NoSuchEntityException" => {
                        CreateAssessmentTargetError::NoSuchEntity(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateAssessmentTargetError::Validation(error_message.to_string())
                    }
                    _ => CreateAssessmentTargetError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateAssessmentTargetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateAssessmentTargetError {
    fn from(err: serde_json::error::Error) -> CreateAssessmentTargetError {
        CreateAssessmentTargetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateAssessmentTargetError {
    fn from(err: CredentialsError) -> CreateAssessmentTargetError {
        CreateAssessmentTargetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateAssessmentTargetError {
    fn from(err: HttpDispatchError) -> CreateAssessmentTargetError {
        CreateAssessmentTargetError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateAssessmentTargetError {
    fn from(err: io::Error) -> CreateAssessmentTargetError {
        CreateAssessmentTargetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateAssessmentTargetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateAssessmentTargetError {
    fn description(&self) -> &str {
        match *self {
            CreateAssessmentTargetError::AccessDenied(ref cause) => cause,
            CreateAssessmentTargetError::Internal(ref cause) => cause,
            CreateAssessmentTargetError::InvalidInput(ref cause) => cause,
            CreateAssessmentTargetError::LimitExceeded(ref cause) => cause,
            CreateAssessmentTargetError::NoSuchEntity(ref cause) => cause,
            CreateAssessmentTargetError::Validation(ref cause) => cause,
            CreateAssessmentTargetError::Credentials(ref err) => err.description(),
            CreateAssessmentTargetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateAssessmentTargetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateAssessmentTemplate
#[derive(Debug, PartialEq)]
pub enum CreateAssessmentTemplateError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateAssessmentTemplateError {
    pub fn from_body(body: &str) -> CreateAssessmentTemplateError {
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
                    "AccessDeniedException" => {
                        CreateAssessmentTemplateError::AccessDenied(String::from(error_message))
                    }
                    "InternalException" => {
                        CreateAssessmentTemplateError::Internal(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        CreateAssessmentTemplateError::InvalidInput(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateAssessmentTemplateError::LimitExceeded(String::from(error_message))
                    }
                    "NoSuchEntityException" => {
                        CreateAssessmentTemplateError::NoSuchEntity(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateAssessmentTemplateError::Validation(error_message.to_string())
                    }
                    _ => CreateAssessmentTemplateError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateAssessmentTemplateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateAssessmentTemplateError {
    fn from(err: serde_json::error::Error) -> CreateAssessmentTemplateError {
        CreateAssessmentTemplateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateAssessmentTemplateError {
    fn from(err: CredentialsError) -> CreateAssessmentTemplateError {
        CreateAssessmentTemplateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateAssessmentTemplateError {
    fn from(err: HttpDispatchError) -> CreateAssessmentTemplateError {
        CreateAssessmentTemplateError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateAssessmentTemplateError {
    fn from(err: io::Error) -> CreateAssessmentTemplateError {
        CreateAssessmentTemplateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateAssessmentTemplateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateAssessmentTemplateError {
    fn description(&self) -> &str {
        match *self {
            CreateAssessmentTemplateError::AccessDenied(ref cause) => cause,
            CreateAssessmentTemplateError::Internal(ref cause) => cause,
            CreateAssessmentTemplateError::InvalidInput(ref cause) => cause,
            CreateAssessmentTemplateError::LimitExceeded(ref cause) => cause,
            CreateAssessmentTemplateError::NoSuchEntity(ref cause) => cause,
            CreateAssessmentTemplateError::Validation(ref cause) => cause,
            CreateAssessmentTemplateError::Credentials(ref err) => err.description(),
            CreateAssessmentTemplateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateAssessmentTemplateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateResourceGroup
#[derive(Debug, PartialEq)]
pub enum CreateResourceGroupError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account limits. The error code describes the limit exceeded.</p>
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

impl CreateResourceGroupError {
    pub fn from_body(body: &str) -> CreateResourceGroupError {
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
                    "AccessDeniedException" => {
                        CreateResourceGroupError::AccessDenied(String::from(error_message))
                    }
                    "InternalException" => {
                        CreateResourceGroupError::Internal(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        CreateResourceGroupError::InvalidInput(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateResourceGroupError::LimitExceeded(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateResourceGroupError::Validation(error_message.to_string())
                    }
                    _ => CreateResourceGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateResourceGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateResourceGroupError {
    fn from(err: serde_json::error::Error) -> CreateResourceGroupError {
        CreateResourceGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateResourceGroupError {
    fn from(err: CredentialsError) -> CreateResourceGroupError {
        CreateResourceGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateResourceGroupError {
    fn from(err: HttpDispatchError) -> CreateResourceGroupError {
        CreateResourceGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateResourceGroupError {
    fn from(err: io::Error) -> CreateResourceGroupError {
        CreateResourceGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateResourceGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateResourceGroupError {
    fn description(&self) -> &str {
        match *self {
            CreateResourceGroupError::AccessDenied(ref cause) => cause,
            CreateResourceGroupError::Internal(ref cause) => cause,
            CreateResourceGroupError::InvalidInput(ref cause) => cause,
            CreateResourceGroupError::LimitExceeded(ref cause) => cause,
            CreateResourceGroupError::Validation(ref cause) => cause,
            CreateResourceGroupError::Credentials(ref err) => err.description(),
            CreateResourceGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateResourceGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteAssessmentRun
#[derive(Debug, PartialEq)]
pub enum DeleteAssessmentRunError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>You cannot perform a specified action if an assessment run is currently in progress.</p>
    AssessmentRunInProgress(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteAssessmentRunError {
    pub fn from_body(body: &str) -> DeleteAssessmentRunError {
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
                    "AccessDeniedException" => {
                        DeleteAssessmentRunError::AccessDenied(String::from(error_message))
                    }
                    "AssessmentRunInProgressException" => {
                        DeleteAssessmentRunError::AssessmentRunInProgress(String::from(
                            error_message,
                        ))
                    }
                    "InternalException" => {
                        DeleteAssessmentRunError::Internal(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        DeleteAssessmentRunError::InvalidInput(String::from(error_message))
                    }
                    "NoSuchEntityException" => {
                        DeleteAssessmentRunError::NoSuchEntity(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteAssessmentRunError::Validation(error_message.to_string())
                    }
                    _ => DeleteAssessmentRunError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteAssessmentRunError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteAssessmentRunError {
    fn from(err: serde_json::error::Error) -> DeleteAssessmentRunError {
        DeleteAssessmentRunError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteAssessmentRunError {
    fn from(err: CredentialsError) -> DeleteAssessmentRunError {
        DeleteAssessmentRunError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteAssessmentRunError {
    fn from(err: HttpDispatchError) -> DeleteAssessmentRunError {
        DeleteAssessmentRunError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteAssessmentRunError {
    fn from(err: io::Error) -> DeleteAssessmentRunError {
        DeleteAssessmentRunError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteAssessmentRunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteAssessmentRunError {
    fn description(&self) -> &str {
        match *self {
            DeleteAssessmentRunError::AccessDenied(ref cause) => cause,
            DeleteAssessmentRunError::AssessmentRunInProgress(ref cause) => cause,
            DeleteAssessmentRunError::Internal(ref cause) => cause,
            DeleteAssessmentRunError::InvalidInput(ref cause) => cause,
            DeleteAssessmentRunError::NoSuchEntity(ref cause) => cause,
            DeleteAssessmentRunError::Validation(ref cause) => cause,
            DeleteAssessmentRunError::Credentials(ref err) => err.description(),
            DeleteAssessmentRunError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteAssessmentRunError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteAssessmentTarget
#[derive(Debug, PartialEq)]
pub enum DeleteAssessmentTargetError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>You cannot perform a specified action if an assessment run is currently in progress.</p>
    AssessmentRunInProgress(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteAssessmentTargetError {
    pub fn from_body(body: &str) -> DeleteAssessmentTargetError {
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
                    "AccessDeniedException" => {
                        DeleteAssessmentTargetError::AccessDenied(String::from(error_message))
                    }
                    "AssessmentRunInProgressException" => {
                        DeleteAssessmentTargetError::AssessmentRunInProgress(String::from(
                            error_message,
                        ))
                    }
                    "InternalException" => {
                        DeleteAssessmentTargetError::Internal(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        DeleteAssessmentTargetError::InvalidInput(String::from(error_message))
                    }
                    "NoSuchEntityException" => {
                        DeleteAssessmentTargetError::NoSuchEntity(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteAssessmentTargetError::Validation(error_message.to_string())
                    }
                    _ => DeleteAssessmentTargetError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteAssessmentTargetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteAssessmentTargetError {
    fn from(err: serde_json::error::Error) -> DeleteAssessmentTargetError {
        DeleteAssessmentTargetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteAssessmentTargetError {
    fn from(err: CredentialsError) -> DeleteAssessmentTargetError {
        DeleteAssessmentTargetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteAssessmentTargetError {
    fn from(err: HttpDispatchError) -> DeleteAssessmentTargetError {
        DeleteAssessmentTargetError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteAssessmentTargetError {
    fn from(err: io::Error) -> DeleteAssessmentTargetError {
        DeleteAssessmentTargetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteAssessmentTargetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteAssessmentTargetError {
    fn description(&self) -> &str {
        match *self {
            DeleteAssessmentTargetError::AccessDenied(ref cause) => cause,
            DeleteAssessmentTargetError::AssessmentRunInProgress(ref cause) => cause,
            DeleteAssessmentTargetError::Internal(ref cause) => cause,
            DeleteAssessmentTargetError::InvalidInput(ref cause) => cause,
            DeleteAssessmentTargetError::NoSuchEntity(ref cause) => cause,
            DeleteAssessmentTargetError::Validation(ref cause) => cause,
            DeleteAssessmentTargetError::Credentials(ref err) => err.description(),
            DeleteAssessmentTargetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteAssessmentTargetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteAssessmentTemplate
#[derive(Debug, PartialEq)]
pub enum DeleteAssessmentTemplateError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>You cannot perform a specified action if an assessment run is currently in progress.</p>
    AssessmentRunInProgress(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteAssessmentTemplateError {
    pub fn from_body(body: &str) -> DeleteAssessmentTemplateError {
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
                    "AccessDeniedException" => {
                        DeleteAssessmentTemplateError::AccessDenied(String::from(error_message))
                    }
                    "AssessmentRunInProgressException" => {
                        DeleteAssessmentTemplateError::AssessmentRunInProgress(String::from(
                            error_message,
                        ))
                    }
                    "InternalException" => {
                        DeleteAssessmentTemplateError::Internal(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        DeleteAssessmentTemplateError::InvalidInput(String::from(error_message))
                    }
                    "NoSuchEntityException" => {
                        DeleteAssessmentTemplateError::NoSuchEntity(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteAssessmentTemplateError::Validation(error_message.to_string())
                    }
                    _ => DeleteAssessmentTemplateError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteAssessmentTemplateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteAssessmentTemplateError {
    fn from(err: serde_json::error::Error) -> DeleteAssessmentTemplateError {
        DeleteAssessmentTemplateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteAssessmentTemplateError {
    fn from(err: CredentialsError) -> DeleteAssessmentTemplateError {
        DeleteAssessmentTemplateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteAssessmentTemplateError {
    fn from(err: HttpDispatchError) -> DeleteAssessmentTemplateError {
        DeleteAssessmentTemplateError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteAssessmentTemplateError {
    fn from(err: io::Error) -> DeleteAssessmentTemplateError {
        DeleteAssessmentTemplateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteAssessmentTemplateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteAssessmentTemplateError {
    fn description(&self) -> &str {
        match *self {
            DeleteAssessmentTemplateError::AccessDenied(ref cause) => cause,
            DeleteAssessmentTemplateError::AssessmentRunInProgress(ref cause) => cause,
            DeleteAssessmentTemplateError::Internal(ref cause) => cause,
            DeleteAssessmentTemplateError::InvalidInput(ref cause) => cause,
            DeleteAssessmentTemplateError::NoSuchEntity(ref cause) => cause,
            DeleteAssessmentTemplateError::Validation(ref cause) => cause,
            DeleteAssessmentTemplateError::Credentials(ref err) => err.description(),
            DeleteAssessmentTemplateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteAssessmentTemplateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAssessmentRuns
#[derive(Debug, PartialEq)]
pub enum DescribeAssessmentRunsError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeAssessmentRunsError {
    pub fn from_body(body: &str) -> DescribeAssessmentRunsError {
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
                        DescribeAssessmentRunsError::Internal(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        DescribeAssessmentRunsError::InvalidInput(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeAssessmentRunsError::Validation(error_message.to_string())
                    }
                    _ => DescribeAssessmentRunsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeAssessmentRunsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeAssessmentRunsError {
    fn from(err: serde_json::error::Error) -> DescribeAssessmentRunsError {
        DescribeAssessmentRunsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeAssessmentRunsError {
    fn from(err: CredentialsError) -> DescribeAssessmentRunsError {
        DescribeAssessmentRunsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeAssessmentRunsError {
    fn from(err: HttpDispatchError) -> DescribeAssessmentRunsError {
        DescribeAssessmentRunsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeAssessmentRunsError {
    fn from(err: io::Error) -> DescribeAssessmentRunsError {
        DescribeAssessmentRunsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeAssessmentRunsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAssessmentRunsError {
    fn description(&self) -> &str {
        match *self {
            DescribeAssessmentRunsError::Internal(ref cause) => cause,
            DescribeAssessmentRunsError::InvalidInput(ref cause) => cause,
            DescribeAssessmentRunsError::Validation(ref cause) => cause,
            DescribeAssessmentRunsError::Credentials(ref err) => err.description(),
            DescribeAssessmentRunsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeAssessmentRunsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAssessmentTargets
#[derive(Debug, PartialEq)]
pub enum DescribeAssessmentTargetsError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeAssessmentTargetsError {
    pub fn from_body(body: &str) -> DescribeAssessmentTargetsError {
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
                        DescribeAssessmentTargetsError::Internal(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        DescribeAssessmentTargetsError::InvalidInput(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeAssessmentTargetsError::Validation(error_message.to_string())
                    }
                    _ => DescribeAssessmentTargetsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeAssessmentTargetsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeAssessmentTargetsError {
    fn from(err: serde_json::error::Error) -> DescribeAssessmentTargetsError {
        DescribeAssessmentTargetsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeAssessmentTargetsError {
    fn from(err: CredentialsError) -> DescribeAssessmentTargetsError {
        DescribeAssessmentTargetsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeAssessmentTargetsError {
    fn from(err: HttpDispatchError) -> DescribeAssessmentTargetsError {
        DescribeAssessmentTargetsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeAssessmentTargetsError {
    fn from(err: io::Error) -> DescribeAssessmentTargetsError {
        DescribeAssessmentTargetsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeAssessmentTargetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAssessmentTargetsError {
    fn description(&self) -> &str {
        match *self {
            DescribeAssessmentTargetsError::Internal(ref cause) => cause,
            DescribeAssessmentTargetsError::InvalidInput(ref cause) => cause,
            DescribeAssessmentTargetsError::Validation(ref cause) => cause,
            DescribeAssessmentTargetsError::Credentials(ref err) => err.description(),
            DescribeAssessmentTargetsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeAssessmentTargetsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAssessmentTemplates
#[derive(Debug, PartialEq)]
pub enum DescribeAssessmentTemplatesError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeAssessmentTemplatesError {
    pub fn from_body(body: &str) -> DescribeAssessmentTemplatesError {
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
                        DescribeAssessmentTemplatesError::Internal(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        DescribeAssessmentTemplatesError::InvalidInput(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeAssessmentTemplatesError::Validation(error_message.to_string())
                    }
                    _ => DescribeAssessmentTemplatesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeAssessmentTemplatesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeAssessmentTemplatesError {
    fn from(err: serde_json::error::Error) -> DescribeAssessmentTemplatesError {
        DescribeAssessmentTemplatesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeAssessmentTemplatesError {
    fn from(err: CredentialsError) -> DescribeAssessmentTemplatesError {
        DescribeAssessmentTemplatesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeAssessmentTemplatesError {
    fn from(err: HttpDispatchError) -> DescribeAssessmentTemplatesError {
        DescribeAssessmentTemplatesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeAssessmentTemplatesError {
    fn from(err: io::Error) -> DescribeAssessmentTemplatesError {
        DescribeAssessmentTemplatesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeAssessmentTemplatesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAssessmentTemplatesError {
    fn description(&self) -> &str {
        match *self {
            DescribeAssessmentTemplatesError::Internal(ref cause) => cause,
            DescribeAssessmentTemplatesError::InvalidInput(ref cause) => cause,
            DescribeAssessmentTemplatesError::Validation(ref cause) => cause,
            DescribeAssessmentTemplatesError::Credentials(ref err) => err.description(),
            DescribeAssessmentTemplatesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeAssessmentTemplatesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeCrossAccountAccessRole
#[derive(Debug, PartialEq)]
pub enum DescribeCrossAccountAccessRoleError {
    /// <p>Internal server error.</p>
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

impl DescribeCrossAccountAccessRoleError {
    pub fn from_body(body: &str) -> DescribeCrossAccountAccessRoleError {
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
                        DescribeCrossAccountAccessRoleError::Internal(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeCrossAccountAccessRoleError::Validation(error_message.to_string())
                    }
                    _ => DescribeCrossAccountAccessRoleError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeCrossAccountAccessRoleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeCrossAccountAccessRoleError {
    fn from(err: serde_json::error::Error) -> DescribeCrossAccountAccessRoleError {
        DescribeCrossAccountAccessRoleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeCrossAccountAccessRoleError {
    fn from(err: CredentialsError) -> DescribeCrossAccountAccessRoleError {
        DescribeCrossAccountAccessRoleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeCrossAccountAccessRoleError {
    fn from(err: HttpDispatchError) -> DescribeCrossAccountAccessRoleError {
        DescribeCrossAccountAccessRoleError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeCrossAccountAccessRoleError {
    fn from(err: io::Error) -> DescribeCrossAccountAccessRoleError {
        DescribeCrossAccountAccessRoleError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeCrossAccountAccessRoleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeCrossAccountAccessRoleError {
    fn description(&self) -> &str {
        match *self {
            DescribeCrossAccountAccessRoleError::Internal(ref cause) => cause,
            DescribeCrossAccountAccessRoleError::Validation(ref cause) => cause,
            DescribeCrossAccountAccessRoleError::Credentials(ref err) => err.description(),
            DescribeCrossAccountAccessRoleError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeCrossAccountAccessRoleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeFindings
#[derive(Debug, PartialEq)]
pub enum DescribeFindingsError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeFindingsError {
    pub fn from_body(body: &str) -> DescribeFindingsError {
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
                        DescribeFindingsError::Internal(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        DescribeFindingsError::InvalidInput(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeFindingsError::Validation(error_message.to_string())
                    }
                    _ => DescribeFindingsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeFindingsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeFindingsError {
    fn from(err: serde_json::error::Error) -> DescribeFindingsError {
        DescribeFindingsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeFindingsError {
    fn from(err: CredentialsError) -> DescribeFindingsError {
        DescribeFindingsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeFindingsError {
    fn from(err: HttpDispatchError) -> DescribeFindingsError {
        DescribeFindingsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeFindingsError {
    fn from(err: io::Error) -> DescribeFindingsError {
        DescribeFindingsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeFindingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeFindingsError {
    fn description(&self) -> &str {
        match *self {
            DescribeFindingsError::Internal(ref cause) => cause,
            DescribeFindingsError::InvalidInput(ref cause) => cause,
            DescribeFindingsError::Validation(ref cause) => cause,
            DescribeFindingsError::Credentials(ref err) => err.description(),
            DescribeFindingsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeFindingsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeResourceGroups
#[derive(Debug, PartialEq)]
pub enum DescribeResourceGroupsError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeResourceGroupsError {
    pub fn from_body(body: &str) -> DescribeResourceGroupsError {
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
                        DescribeResourceGroupsError::Internal(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        DescribeResourceGroupsError::InvalidInput(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeResourceGroupsError::Validation(error_message.to_string())
                    }
                    _ => DescribeResourceGroupsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeResourceGroupsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeResourceGroupsError {
    fn from(err: serde_json::error::Error) -> DescribeResourceGroupsError {
        DescribeResourceGroupsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeResourceGroupsError {
    fn from(err: CredentialsError) -> DescribeResourceGroupsError {
        DescribeResourceGroupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeResourceGroupsError {
    fn from(err: HttpDispatchError) -> DescribeResourceGroupsError {
        DescribeResourceGroupsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeResourceGroupsError {
    fn from(err: io::Error) -> DescribeResourceGroupsError {
        DescribeResourceGroupsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeResourceGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeResourceGroupsError {
    fn description(&self) -> &str {
        match *self {
            DescribeResourceGroupsError::Internal(ref cause) => cause,
            DescribeResourceGroupsError::InvalidInput(ref cause) => cause,
            DescribeResourceGroupsError::Validation(ref cause) => cause,
            DescribeResourceGroupsError::Credentials(ref err) => err.description(),
            DescribeResourceGroupsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeResourceGroupsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeRulesPackages
#[derive(Debug, PartialEq)]
pub enum DescribeRulesPackagesError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeRulesPackagesError {
    pub fn from_body(body: &str) -> DescribeRulesPackagesError {
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
                        DescribeRulesPackagesError::Internal(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        DescribeRulesPackagesError::InvalidInput(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeRulesPackagesError::Validation(error_message.to_string())
                    }
                    _ => DescribeRulesPackagesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeRulesPackagesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeRulesPackagesError {
    fn from(err: serde_json::error::Error) -> DescribeRulesPackagesError {
        DescribeRulesPackagesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeRulesPackagesError {
    fn from(err: CredentialsError) -> DescribeRulesPackagesError {
        DescribeRulesPackagesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeRulesPackagesError {
    fn from(err: HttpDispatchError) -> DescribeRulesPackagesError {
        DescribeRulesPackagesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeRulesPackagesError {
    fn from(err: io::Error) -> DescribeRulesPackagesError {
        DescribeRulesPackagesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeRulesPackagesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeRulesPackagesError {
    fn description(&self) -> &str {
        match *self {
            DescribeRulesPackagesError::Internal(ref cause) => cause,
            DescribeRulesPackagesError::InvalidInput(ref cause) => cause,
            DescribeRulesPackagesError::Validation(ref cause) => cause,
            DescribeRulesPackagesError::Credentials(ref err) => err.description(),
            DescribeRulesPackagesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeRulesPackagesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetAssessmentReport
#[derive(Debug, PartialEq)]
pub enum GetAssessmentReportError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>You cannot perform a specified action if an assessment run is currently in progress.</p>
    AssessmentRunInProgress(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
    /// <p>Used by the <a>GetAssessmentReport</a> API. The request was rejected because you tried to generate a report for an assessment run that existed before reporting was supported in Amazon Inspector. You can only generate reports for assessment runs that took place or will take place after generating reports in Amazon Inspector became available.</p>
    UnsupportedFeature(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetAssessmentReportError {
    pub fn from_body(body: &str) -> GetAssessmentReportError {
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
                    "AccessDeniedException" => {
                        GetAssessmentReportError::AccessDenied(String::from(error_message))
                    }
                    "AssessmentRunInProgressException" => {
                        GetAssessmentReportError::AssessmentRunInProgress(String::from(
                            error_message,
                        ))
                    }
                    "InternalException" => {
                        GetAssessmentReportError::Internal(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetAssessmentReportError::InvalidInput(String::from(error_message))
                    }
                    "NoSuchEntityException" => {
                        GetAssessmentReportError::NoSuchEntity(String::from(error_message))
                    }
                    "UnsupportedFeatureException" => {
                        GetAssessmentReportError::UnsupportedFeature(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetAssessmentReportError::Validation(error_message.to_string())
                    }
                    _ => GetAssessmentReportError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetAssessmentReportError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetAssessmentReportError {
    fn from(err: serde_json::error::Error) -> GetAssessmentReportError {
        GetAssessmentReportError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetAssessmentReportError {
    fn from(err: CredentialsError) -> GetAssessmentReportError {
        GetAssessmentReportError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetAssessmentReportError {
    fn from(err: HttpDispatchError) -> GetAssessmentReportError {
        GetAssessmentReportError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetAssessmentReportError {
    fn from(err: io::Error) -> GetAssessmentReportError {
        GetAssessmentReportError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetAssessmentReportError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAssessmentReportError {
    fn description(&self) -> &str {
        match *self {
            GetAssessmentReportError::AccessDenied(ref cause) => cause,
            GetAssessmentReportError::AssessmentRunInProgress(ref cause) => cause,
            GetAssessmentReportError::Internal(ref cause) => cause,
            GetAssessmentReportError::InvalidInput(ref cause) => cause,
            GetAssessmentReportError::NoSuchEntity(ref cause) => cause,
            GetAssessmentReportError::UnsupportedFeature(ref cause) => cause,
            GetAssessmentReportError::Validation(ref cause) => cause,
            GetAssessmentReportError::Credentials(ref err) => err.description(),
            GetAssessmentReportError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetAssessmentReportError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetTelemetryMetadata
#[derive(Debug, PartialEq)]
pub enum GetTelemetryMetadataError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetTelemetryMetadataError {
    pub fn from_body(body: &str) -> GetTelemetryMetadataError {
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
                    "AccessDeniedException" => {
                        GetTelemetryMetadataError::AccessDenied(String::from(error_message))
                    }
                    "InternalException" => {
                        GetTelemetryMetadataError::Internal(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetTelemetryMetadataError::InvalidInput(String::from(error_message))
                    }
                    "NoSuchEntityException" => {
                        GetTelemetryMetadataError::NoSuchEntity(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetTelemetryMetadataError::Validation(error_message.to_string())
                    }
                    _ => GetTelemetryMetadataError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetTelemetryMetadataError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetTelemetryMetadataError {
    fn from(err: serde_json::error::Error) -> GetTelemetryMetadataError {
        GetTelemetryMetadataError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetTelemetryMetadataError {
    fn from(err: CredentialsError) -> GetTelemetryMetadataError {
        GetTelemetryMetadataError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetTelemetryMetadataError {
    fn from(err: HttpDispatchError) -> GetTelemetryMetadataError {
        GetTelemetryMetadataError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetTelemetryMetadataError {
    fn from(err: io::Error) -> GetTelemetryMetadataError {
        GetTelemetryMetadataError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetTelemetryMetadataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetTelemetryMetadataError {
    fn description(&self) -> &str {
        match *self {
            GetTelemetryMetadataError::AccessDenied(ref cause) => cause,
            GetTelemetryMetadataError::Internal(ref cause) => cause,
            GetTelemetryMetadataError::InvalidInput(ref cause) => cause,
            GetTelemetryMetadataError::NoSuchEntity(ref cause) => cause,
            GetTelemetryMetadataError::Validation(ref cause) => cause,
            GetTelemetryMetadataError::Credentials(ref err) => err.description(),
            GetTelemetryMetadataError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetTelemetryMetadataError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListAssessmentRunAgents
#[derive(Debug, PartialEq)]
pub enum ListAssessmentRunAgentsError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListAssessmentRunAgentsError {
    pub fn from_body(body: &str) -> ListAssessmentRunAgentsError {
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
                    "AccessDeniedException" => {
                        ListAssessmentRunAgentsError::AccessDenied(String::from(error_message))
                    }
                    "InternalException" => {
                        ListAssessmentRunAgentsError::Internal(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        ListAssessmentRunAgentsError::InvalidInput(String::from(error_message))
                    }
                    "NoSuchEntityException" => {
                        ListAssessmentRunAgentsError::NoSuchEntity(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListAssessmentRunAgentsError::Validation(error_message.to_string())
                    }
                    _ => ListAssessmentRunAgentsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListAssessmentRunAgentsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListAssessmentRunAgentsError {
    fn from(err: serde_json::error::Error) -> ListAssessmentRunAgentsError {
        ListAssessmentRunAgentsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListAssessmentRunAgentsError {
    fn from(err: CredentialsError) -> ListAssessmentRunAgentsError {
        ListAssessmentRunAgentsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListAssessmentRunAgentsError {
    fn from(err: HttpDispatchError) -> ListAssessmentRunAgentsError {
        ListAssessmentRunAgentsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListAssessmentRunAgentsError {
    fn from(err: io::Error) -> ListAssessmentRunAgentsError {
        ListAssessmentRunAgentsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListAssessmentRunAgentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAssessmentRunAgentsError {
    fn description(&self) -> &str {
        match *self {
            ListAssessmentRunAgentsError::AccessDenied(ref cause) => cause,
            ListAssessmentRunAgentsError::Internal(ref cause) => cause,
            ListAssessmentRunAgentsError::InvalidInput(ref cause) => cause,
            ListAssessmentRunAgentsError::NoSuchEntity(ref cause) => cause,
            ListAssessmentRunAgentsError::Validation(ref cause) => cause,
            ListAssessmentRunAgentsError::Credentials(ref err) => err.description(),
            ListAssessmentRunAgentsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListAssessmentRunAgentsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListAssessmentRuns
#[derive(Debug, PartialEq)]
pub enum ListAssessmentRunsError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListAssessmentRunsError {
    pub fn from_body(body: &str) -> ListAssessmentRunsError {
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
                    "AccessDeniedException" => {
                        ListAssessmentRunsError::AccessDenied(String::from(error_message))
                    }
                    "InternalException" => {
                        ListAssessmentRunsError::Internal(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        ListAssessmentRunsError::InvalidInput(String::from(error_message))
                    }
                    "NoSuchEntityException" => {
                        ListAssessmentRunsError::NoSuchEntity(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListAssessmentRunsError::Validation(error_message.to_string())
                    }
                    _ => ListAssessmentRunsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListAssessmentRunsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListAssessmentRunsError {
    fn from(err: serde_json::error::Error) -> ListAssessmentRunsError {
        ListAssessmentRunsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListAssessmentRunsError {
    fn from(err: CredentialsError) -> ListAssessmentRunsError {
        ListAssessmentRunsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListAssessmentRunsError {
    fn from(err: HttpDispatchError) -> ListAssessmentRunsError {
        ListAssessmentRunsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListAssessmentRunsError {
    fn from(err: io::Error) -> ListAssessmentRunsError {
        ListAssessmentRunsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListAssessmentRunsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAssessmentRunsError {
    fn description(&self) -> &str {
        match *self {
            ListAssessmentRunsError::AccessDenied(ref cause) => cause,
            ListAssessmentRunsError::Internal(ref cause) => cause,
            ListAssessmentRunsError::InvalidInput(ref cause) => cause,
            ListAssessmentRunsError::NoSuchEntity(ref cause) => cause,
            ListAssessmentRunsError::Validation(ref cause) => cause,
            ListAssessmentRunsError::Credentials(ref err) => err.description(),
            ListAssessmentRunsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListAssessmentRunsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListAssessmentTargets
#[derive(Debug, PartialEq)]
pub enum ListAssessmentTargetsError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListAssessmentTargetsError {
    pub fn from_body(body: &str) -> ListAssessmentTargetsError {
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
                    "AccessDeniedException" => {
                        ListAssessmentTargetsError::AccessDenied(String::from(error_message))
                    }
                    "InternalException" => {
                        ListAssessmentTargetsError::Internal(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        ListAssessmentTargetsError::InvalidInput(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListAssessmentTargetsError::Validation(error_message.to_string())
                    }
                    _ => ListAssessmentTargetsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListAssessmentTargetsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListAssessmentTargetsError {
    fn from(err: serde_json::error::Error) -> ListAssessmentTargetsError {
        ListAssessmentTargetsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListAssessmentTargetsError {
    fn from(err: CredentialsError) -> ListAssessmentTargetsError {
        ListAssessmentTargetsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListAssessmentTargetsError {
    fn from(err: HttpDispatchError) -> ListAssessmentTargetsError {
        ListAssessmentTargetsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListAssessmentTargetsError {
    fn from(err: io::Error) -> ListAssessmentTargetsError {
        ListAssessmentTargetsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListAssessmentTargetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAssessmentTargetsError {
    fn description(&self) -> &str {
        match *self {
            ListAssessmentTargetsError::AccessDenied(ref cause) => cause,
            ListAssessmentTargetsError::Internal(ref cause) => cause,
            ListAssessmentTargetsError::InvalidInput(ref cause) => cause,
            ListAssessmentTargetsError::Validation(ref cause) => cause,
            ListAssessmentTargetsError::Credentials(ref err) => err.description(),
            ListAssessmentTargetsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListAssessmentTargetsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListAssessmentTemplates
#[derive(Debug, PartialEq)]
pub enum ListAssessmentTemplatesError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListAssessmentTemplatesError {
    pub fn from_body(body: &str) -> ListAssessmentTemplatesError {
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
                    "AccessDeniedException" => {
                        ListAssessmentTemplatesError::AccessDenied(String::from(error_message))
                    }
                    "InternalException" => {
                        ListAssessmentTemplatesError::Internal(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        ListAssessmentTemplatesError::InvalidInput(String::from(error_message))
                    }
                    "NoSuchEntityException" => {
                        ListAssessmentTemplatesError::NoSuchEntity(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListAssessmentTemplatesError::Validation(error_message.to_string())
                    }
                    _ => ListAssessmentTemplatesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListAssessmentTemplatesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListAssessmentTemplatesError {
    fn from(err: serde_json::error::Error) -> ListAssessmentTemplatesError {
        ListAssessmentTemplatesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListAssessmentTemplatesError {
    fn from(err: CredentialsError) -> ListAssessmentTemplatesError {
        ListAssessmentTemplatesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListAssessmentTemplatesError {
    fn from(err: HttpDispatchError) -> ListAssessmentTemplatesError {
        ListAssessmentTemplatesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListAssessmentTemplatesError {
    fn from(err: io::Error) -> ListAssessmentTemplatesError {
        ListAssessmentTemplatesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListAssessmentTemplatesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAssessmentTemplatesError {
    fn description(&self) -> &str {
        match *self {
            ListAssessmentTemplatesError::AccessDenied(ref cause) => cause,
            ListAssessmentTemplatesError::Internal(ref cause) => cause,
            ListAssessmentTemplatesError::InvalidInput(ref cause) => cause,
            ListAssessmentTemplatesError::NoSuchEntity(ref cause) => cause,
            ListAssessmentTemplatesError::Validation(ref cause) => cause,
            ListAssessmentTemplatesError::Credentials(ref err) => err.description(),
            ListAssessmentTemplatesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListAssessmentTemplatesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListEventSubscriptions
#[derive(Debug, PartialEq)]
pub enum ListEventSubscriptionsError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListEventSubscriptionsError {
    pub fn from_body(body: &str) -> ListEventSubscriptionsError {
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
                    "AccessDeniedException" => {
                        ListEventSubscriptionsError::AccessDenied(String::from(error_message))
                    }
                    "InternalException" => {
                        ListEventSubscriptionsError::Internal(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        ListEventSubscriptionsError::InvalidInput(String::from(error_message))
                    }
                    "NoSuchEntityException" => {
                        ListEventSubscriptionsError::NoSuchEntity(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListEventSubscriptionsError::Validation(error_message.to_string())
                    }
                    _ => ListEventSubscriptionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListEventSubscriptionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListEventSubscriptionsError {
    fn from(err: serde_json::error::Error) -> ListEventSubscriptionsError {
        ListEventSubscriptionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListEventSubscriptionsError {
    fn from(err: CredentialsError) -> ListEventSubscriptionsError {
        ListEventSubscriptionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListEventSubscriptionsError {
    fn from(err: HttpDispatchError) -> ListEventSubscriptionsError {
        ListEventSubscriptionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListEventSubscriptionsError {
    fn from(err: io::Error) -> ListEventSubscriptionsError {
        ListEventSubscriptionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListEventSubscriptionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListEventSubscriptionsError {
    fn description(&self) -> &str {
        match *self {
            ListEventSubscriptionsError::AccessDenied(ref cause) => cause,
            ListEventSubscriptionsError::Internal(ref cause) => cause,
            ListEventSubscriptionsError::InvalidInput(ref cause) => cause,
            ListEventSubscriptionsError::NoSuchEntity(ref cause) => cause,
            ListEventSubscriptionsError::Validation(ref cause) => cause,
            ListEventSubscriptionsError::Credentials(ref err) => err.description(),
            ListEventSubscriptionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListEventSubscriptionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListFindings
#[derive(Debug, PartialEq)]
pub enum ListFindingsError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListFindingsError {
    pub fn from_body(body: &str) -> ListFindingsError {
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
                    "AccessDeniedException" => {
                        ListFindingsError::AccessDenied(String::from(error_message))
                    }
                    "InternalException" => ListFindingsError::Internal(String::from(error_message)),
                    "InvalidInputException" => {
                        ListFindingsError::InvalidInput(String::from(error_message))
                    }
                    "NoSuchEntityException" => {
                        ListFindingsError::NoSuchEntity(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListFindingsError::Validation(error_message.to_string())
                    }
                    _ => ListFindingsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListFindingsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListFindingsError {
    fn from(err: serde_json::error::Error) -> ListFindingsError {
        ListFindingsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListFindingsError {
    fn from(err: CredentialsError) -> ListFindingsError {
        ListFindingsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListFindingsError {
    fn from(err: HttpDispatchError) -> ListFindingsError {
        ListFindingsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListFindingsError {
    fn from(err: io::Error) -> ListFindingsError {
        ListFindingsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListFindingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListFindingsError {
    fn description(&self) -> &str {
        match *self {
            ListFindingsError::AccessDenied(ref cause) => cause,
            ListFindingsError::Internal(ref cause) => cause,
            ListFindingsError::InvalidInput(ref cause) => cause,
            ListFindingsError::NoSuchEntity(ref cause) => cause,
            ListFindingsError::Validation(ref cause) => cause,
            ListFindingsError::Credentials(ref err) => err.description(),
            ListFindingsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListFindingsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListRulesPackages
#[derive(Debug, PartialEq)]
pub enum ListRulesPackagesError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListRulesPackagesError {
    pub fn from_body(body: &str) -> ListRulesPackagesError {
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
                    "AccessDeniedException" => {
                        ListRulesPackagesError::AccessDenied(String::from(error_message))
                    }
                    "InternalException" => {
                        ListRulesPackagesError::Internal(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        ListRulesPackagesError::InvalidInput(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListRulesPackagesError::Validation(error_message.to_string())
                    }
                    _ => ListRulesPackagesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListRulesPackagesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListRulesPackagesError {
    fn from(err: serde_json::error::Error) -> ListRulesPackagesError {
        ListRulesPackagesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListRulesPackagesError {
    fn from(err: CredentialsError) -> ListRulesPackagesError {
        ListRulesPackagesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListRulesPackagesError {
    fn from(err: HttpDispatchError) -> ListRulesPackagesError {
        ListRulesPackagesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListRulesPackagesError {
    fn from(err: io::Error) -> ListRulesPackagesError {
        ListRulesPackagesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListRulesPackagesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListRulesPackagesError {
    fn description(&self) -> &str {
        match *self {
            ListRulesPackagesError::AccessDenied(ref cause) => cause,
            ListRulesPackagesError::Internal(ref cause) => cause,
            ListRulesPackagesError::InvalidInput(ref cause) => cause,
            ListRulesPackagesError::Validation(ref cause) => cause,
            ListRulesPackagesError::Credentials(ref err) => err.description(),
            ListRulesPackagesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListRulesPackagesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListTagsForResourceError {
    pub fn from_body(body: &str) -> ListTagsForResourceError {
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
                    "AccessDeniedException" => {
                        ListTagsForResourceError::AccessDenied(String::from(error_message))
                    }
                    "InternalException" => {
                        ListTagsForResourceError::Internal(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        ListTagsForResourceError::InvalidInput(String::from(error_message))
                    }
                    "NoSuchEntityException" => {
                        ListTagsForResourceError::NoSuchEntity(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListTagsForResourceError::Validation(error_message.to_string())
                    }
                    _ => ListTagsForResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListTagsForResourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListTagsForResourceError {
    fn from(err: serde_json::error::Error) -> ListTagsForResourceError {
        ListTagsForResourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListTagsForResourceError {
    fn from(err: CredentialsError) -> ListTagsForResourceError {
        ListTagsForResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTagsForResourceError {
    fn from(err: HttpDispatchError) -> ListTagsForResourceError {
        ListTagsForResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTagsForResourceError {
    fn from(err: io::Error) -> ListTagsForResourceError {
        ListTagsForResourceError::HttpDispatch(HttpDispatchError::from(err))
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
            ListTagsForResourceError::AccessDenied(ref cause) => cause,
            ListTagsForResourceError::Internal(ref cause) => cause,
            ListTagsForResourceError::InvalidInput(ref cause) => cause,
            ListTagsForResourceError::NoSuchEntity(ref cause) => cause,
            ListTagsForResourceError::Validation(ref cause) => cause,
            ListTagsForResourceError::Credentials(ref err) => err.description(),
            ListTagsForResourceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListTagsForResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PreviewAgents
#[derive(Debug, PartialEq)]
pub enum PreviewAgentsError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>Amazon Inspector cannot assume the cross-account role that it needs to list your EC2 instances during the assessment run.</p>
    InvalidCrossAccountRole(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PreviewAgentsError {
    pub fn from_body(body: &str) -> PreviewAgentsError {
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
                    "AccessDeniedException" => {
                        PreviewAgentsError::AccessDenied(String::from(error_message))
                    }
                    "InternalException" => {
                        PreviewAgentsError::Internal(String::from(error_message))
                    }
                    "InvalidCrossAccountRoleException" => {
                        PreviewAgentsError::InvalidCrossAccountRole(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        PreviewAgentsError::InvalidInput(String::from(error_message))
                    }
                    "NoSuchEntityException" => {
                        PreviewAgentsError::NoSuchEntity(String::from(error_message))
                    }
                    "ValidationException" => {
                        PreviewAgentsError::Validation(error_message.to_string())
                    }
                    _ => PreviewAgentsError::Unknown(String::from(body)),
                }
            }
            Err(_) => PreviewAgentsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PreviewAgentsError {
    fn from(err: serde_json::error::Error) -> PreviewAgentsError {
        PreviewAgentsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PreviewAgentsError {
    fn from(err: CredentialsError) -> PreviewAgentsError {
        PreviewAgentsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PreviewAgentsError {
    fn from(err: HttpDispatchError) -> PreviewAgentsError {
        PreviewAgentsError::HttpDispatch(err)
    }
}
impl From<io::Error> for PreviewAgentsError {
    fn from(err: io::Error) -> PreviewAgentsError {
        PreviewAgentsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PreviewAgentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PreviewAgentsError {
    fn description(&self) -> &str {
        match *self {
            PreviewAgentsError::AccessDenied(ref cause) => cause,
            PreviewAgentsError::Internal(ref cause) => cause,
            PreviewAgentsError::InvalidCrossAccountRole(ref cause) => cause,
            PreviewAgentsError::InvalidInput(ref cause) => cause,
            PreviewAgentsError::NoSuchEntity(ref cause) => cause,
            PreviewAgentsError::Validation(ref cause) => cause,
            PreviewAgentsError::Credentials(ref err) => err.description(),
            PreviewAgentsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PreviewAgentsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RegisterCrossAccountAccessRole
#[derive(Debug, PartialEq)]
pub enum RegisterCrossAccountAccessRoleError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>Amazon Inspector cannot assume the cross-account role that it needs to list your EC2 instances during the assessment run.</p>
    InvalidCrossAccountRole(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RegisterCrossAccountAccessRoleError {
    pub fn from_body(body: &str) -> RegisterCrossAccountAccessRoleError {
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
                    "AccessDeniedException" => RegisterCrossAccountAccessRoleError::AccessDenied(
                        String::from(error_message),
                    ),
                    "InternalException" => {
                        RegisterCrossAccountAccessRoleError::Internal(String::from(error_message))
                    }
                    "InvalidCrossAccountRoleException" => {
                        RegisterCrossAccountAccessRoleError::InvalidCrossAccountRole(String::from(
                            error_message,
                        ))
                    }
                    "InvalidInputException" => RegisterCrossAccountAccessRoleError::InvalidInput(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        RegisterCrossAccountAccessRoleError::Validation(error_message.to_string())
                    }
                    _ => RegisterCrossAccountAccessRoleError::Unknown(String::from(body)),
                }
            }
            Err(_) => RegisterCrossAccountAccessRoleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RegisterCrossAccountAccessRoleError {
    fn from(err: serde_json::error::Error) -> RegisterCrossAccountAccessRoleError {
        RegisterCrossAccountAccessRoleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RegisterCrossAccountAccessRoleError {
    fn from(err: CredentialsError) -> RegisterCrossAccountAccessRoleError {
        RegisterCrossAccountAccessRoleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RegisterCrossAccountAccessRoleError {
    fn from(err: HttpDispatchError) -> RegisterCrossAccountAccessRoleError {
        RegisterCrossAccountAccessRoleError::HttpDispatch(err)
    }
}
impl From<io::Error> for RegisterCrossAccountAccessRoleError {
    fn from(err: io::Error) -> RegisterCrossAccountAccessRoleError {
        RegisterCrossAccountAccessRoleError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RegisterCrossAccountAccessRoleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RegisterCrossAccountAccessRoleError {
    fn description(&self) -> &str {
        match *self {
            RegisterCrossAccountAccessRoleError::AccessDenied(ref cause) => cause,
            RegisterCrossAccountAccessRoleError::Internal(ref cause) => cause,
            RegisterCrossAccountAccessRoleError::InvalidCrossAccountRole(ref cause) => cause,
            RegisterCrossAccountAccessRoleError::InvalidInput(ref cause) => cause,
            RegisterCrossAccountAccessRoleError::Validation(ref cause) => cause,
            RegisterCrossAccountAccessRoleError::Credentials(ref err) => err.description(),
            RegisterCrossAccountAccessRoleError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RegisterCrossAccountAccessRoleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RemoveAttributesFromFindings
#[derive(Debug, PartialEq)]
pub enum RemoveAttributesFromFindingsError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RemoveAttributesFromFindingsError {
    pub fn from_body(body: &str) -> RemoveAttributesFromFindingsError {
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
                    "AccessDeniedException" => {
                        RemoveAttributesFromFindingsError::AccessDenied(String::from(error_message))
                    }
                    "InternalException" => {
                        RemoveAttributesFromFindingsError::Internal(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        RemoveAttributesFromFindingsError::InvalidInput(String::from(error_message))
                    }
                    "NoSuchEntityException" => {
                        RemoveAttributesFromFindingsError::NoSuchEntity(String::from(error_message))
                    }
                    "ValidationException" => {
                        RemoveAttributesFromFindingsError::Validation(error_message.to_string())
                    }
                    _ => RemoveAttributesFromFindingsError::Unknown(String::from(body)),
                }
            }
            Err(_) => RemoveAttributesFromFindingsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RemoveAttributesFromFindingsError {
    fn from(err: serde_json::error::Error) -> RemoveAttributesFromFindingsError {
        RemoveAttributesFromFindingsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RemoveAttributesFromFindingsError {
    fn from(err: CredentialsError) -> RemoveAttributesFromFindingsError {
        RemoveAttributesFromFindingsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RemoveAttributesFromFindingsError {
    fn from(err: HttpDispatchError) -> RemoveAttributesFromFindingsError {
        RemoveAttributesFromFindingsError::HttpDispatch(err)
    }
}
impl From<io::Error> for RemoveAttributesFromFindingsError {
    fn from(err: io::Error) -> RemoveAttributesFromFindingsError {
        RemoveAttributesFromFindingsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RemoveAttributesFromFindingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RemoveAttributesFromFindingsError {
    fn description(&self) -> &str {
        match *self {
            RemoveAttributesFromFindingsError::AccessDenied(ref cause) => cause,
            RemoveAttributesFromFindingsError::Internal(ref cause) => cause,
            RemoveAttributesFromFindingsError::InvalidInput(ref cause) => cause,
            RemoveAttributesFromFindingsError::NoSuchEntity(ref cause) => cause,
            RemoveAttributesFromFindingsError::Validation(ref cause) => cause,
            RemoveAttributesFromFindingsError::Credentials(ref err) => err.description(),
            RemoveAttributesFromFindingsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RemoveAttributesFromFindingsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetTagsForResource
#[derive(Debug, PartialEq)]
pub enum SetTagsForResourceError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SetTagsForResourceError {
    pub fn from_body(body: &str) -> SetTagsForResourceError {
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
                    "AccessDeniedException" => {
                        SetTagsForResourceError::AccessDenied(String::from(error_message))
                    }
                    "InternalException" => {
                        SetTagsForResourceError::Internal(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        SetTagsForResourceError::InvalidInput(String::from(error_message))
                    }
                    "NoSuchEntityException" => {
                        SetTagsForResourceError::NoSuchEntity(String::from(error_message))
                    }
                    "ValidationException" => {
                        SetTagsForResourceError::Validation(error_message.to_string())
                    }
                    _ => SetTagsForResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => SetTagsForResourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SetTagsForResourceError {
    fn from(err: serde_json::error::Error) -> SetTagsForResourceError {
        SetTagsForResourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SetTagsForResourceError {
    fn from(err: CredentialsError) -> SetTagsForResourceError {
        SetTagsForResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetTagsForResourceError {
    fn from(err: HttpDispatchError) -> SetTagsForResourceError {
        SetTagsForResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for SetTagsForResourceError {
    fn from(err: io::Error) -> SetTagsForResourceError {
        SetTagsForResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SetTagsForResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetTagsForResourceError {
    fn description(&self) -> &str {
        match *self {
            SetTagsForResourceError::AccessDenied(ref cause) => cause,
            SetTagsForResourceError::Internal(ref cause) => cause,
            SetTagsForResourceError::InvalidInput(ref cause) => cause,
            SetTagsForResourceError::NoSuchEntity(ref cause) => cause,
            SetTagsForResourceError::Validation(ref cause) => cause,
            SetTagsForResourceError::Credentials(ref err) => err.description(),
            SetTagsForResourceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SetTagsForResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StartAssessmentRun
#[derive(Debug, PartialEq)]
pub enum StartAssessmentRunError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>You started an assessment run, but one of the instances is already participating in another assessment run.</p>
    AgentsAlreadyRunningAssessment(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>Amazon Inspector cannot assume the cross-account role that it needs to list your EC2 instances during the assessment run.</p>
    InvalidCrossAccountRole(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl StartAssessmentRunError {
    pub fn from_body(body: &str) -> StartAssessmentRunError {
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
                    "AccessDeniedException" => {
                        StartAssessmentRunError::AccessDenied(String::from(error_message))
                    }
                    "AgentsAlreadyRunningAssessmentException" => {
                        StartAssessmentRunError::AgentsAlreadyRunningAssessment(String::from(
                            error_message,
                        ))
                    }
                    "InternalException" => {
                        StartAssessmentRunError::Internal(String::from(error_message))
                    }
                    "InvalidCrossAccountRoleException" => {
                        StartAssessmentRunError::InvalidCrossAccountRole(String::from(
                            error_message,
                        ))
                    }
                    "InvalidInputException" => {
                        StartAssessmentRunError::InvalidInput(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        StartAssessmentRunError::LimitExceeded(String::from(error_message))
                    }
                    "NoSuchEntityException" => {
                        StartAssessmentRunError::NoSuchEntity(String::from(error_message))
                    }
                    "ValidationException" => {
                        StartAssessmentRunError::Validation(error_message.to_string())
                    }
                    _ => StartAssessmentRunError::Unknown(String::from(body)),
                }
            }
            Err(_) => StartAssessmentRunError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StartAssessmentRunError {
    fn from(err: serde_json::error::Error) -> StartAssessmentRunError {
        StartAssessmentRunError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StartAssessmentRunError {
    fn from(err: CredentialsError) -> StartAssessmentRunError {
        StartAssessmentRunError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartAssessmentRunError {
    fn from(err: HttpDispatchError) -> StartAssessmentRunError {
        StartAssessmentRunError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartAssessmentRunError {
    fn from(err: io::Error) -> StartAssessmentRunError {
        StartAssessmentRunError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartAssessmentRunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartAssessmentRunError {
    fn description(&self) -> &str {
        match *self {
            StartAssessmentRunError::AccessDenied(ref cause) => cause,
            StartAssessmentRunError::AgentsAlreadyRunningAssessment(ref cause) => cause,
            StartAssessmentRunError::Internal(ref cause) => cause,
            StartAssessmentRunError::InvalidCrossAccountRole(ref cause) => cause,
            StartAssessmentRunError::InvalidInput(ref cause) => cause,
            StartAssessmentRunError::LimitExceeded(ref cause) => cause,
            StartAssessmentRunError::NoSuchEntity(ref cause) => cause,
            StartAssessmentRunError::Validation(ref cause) => cause,
            StartAssessmentRunError::Credentials(ref err) => err.description(),
            StartAssessmentRunError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StartAssessmentRunError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StopAssessmentRun
#[derive(Debug, PartialEq)]
pub enum StopAssessmentRunError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl StopAssessmentRunError {
    pub fn from_body(body: &str) -> StopAssessmentRunError {
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
                    "AccessDeniedException" => {
                        StopAssessmentRunError::AccessDenied(String::from(error_message))
                    }
                    "InternalException" => {
                        StopAssessmentRunError::Internal(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        StopAssessmentRunError::InvalidInput(String::from(error_message))
                    }
                    "NoSuchEntityException" => {
                        StopAssessmentRunError::NoSuchEntity(String::from(error_message))
                    }
                    "ValidationException" => {
                        StopAssessmentRunError::Validation(error_message.to_string())
                    }
                    _ => StopAssessmentRunError::Unknown(String::from(body)),
                }
            }
            Err(_) => StopAssessmentRunError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StopAssessmentRunError {
    fn from(err: serde_json::error::Error) -> StopAssessmentRunError {
        StopAssessmentRunError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StopAssessmentRunError {
    fn from(err: CredentialsError) -> StopAssessmentRunError {
        StopAssessmentRunError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopAssessmentRunError {
    fn from(err: HttpDispatchError) -> StopAssessmentRunError {
        StopAssessmentRunError::HttpDispatch(err)
    }
}
impl From<io::Error> for StopAssessmentRunError {
    fn from(err: io::Error) -> StopAssessmentRunError {
        StopAssessmentRunError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StopAssessmentRunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopAssessmentRunError {
    fn description(&self) -> &str {
        match *self {
            StopAssessmentRunError::AccessDenied(ref cause) => cause,
            StopAssessmentRunError::Internal(ref cause) => cause,
            StopAssessmentRunError::InvalidInput(ref cause) => cause,
            StopAssessmentRunError::NoSuchEntity(ref cause) => cause,
            StopAssessmentRunError::Validation(ref cause) => cause,
            StopAssessmentRunError::Credentials(ref err) => err.description(),
            StopAssessmentRunError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StopAssessmentRunError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SubscribeToEvent
#[derive(Debug, PartialEq)]
pub enum SubscribeToEventError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SubscribeToEventError {
    pub fn from_body(body: &str) -> SubscribeToEventError {
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
                    "AccessDeniedException" => {
                        SubscribeToEventError::AccessDenied(String::from(error_message))
                    }
                    "InternalException" => {
                        SubscribeToEventError::Internal(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        SubscribeToEventError::InvalidInput(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        SubscribeToEventError::LimitExceeded(String::from(error_message))
                    }
                    "NoSuchEntityException" => {
                        SubscribeToEventError::NoSuchEntity(String::from(error_message))
                    }
                    "ValidationException" => {
                        SubscribeToEventError::Validation(error_message.to_string())
                    }
                    _ => SubscribeToEventError::Unknown(String::from(body)),
                }
            }
            Err(_) => SubscribeToEventError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SubscribeToEventError {
    fn from(err: serde_json::error::Error) -> SubscribeToEventError {
        SubscribeToEventError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SubscribeToEventError {
    fn from(err: CredentialsError) -> SubscribeToEventError {
        SubscribeToEventError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SubscribeToEventError {
    fn from(err: HttpDispatchError) -> SubscribeToEventError {
        SubscribeToEventError::HttpDispatch(err)
    }
}
impl From<io::Error> for SubscribeToEventError {
    fn from(err: io::Error) -> SubscribeToEventError {
        SubscribeToEventError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SubscribeToEventError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SubscribeToEventError {
    fn description(&self) -> &str {
        match *self {
            SubscribeToEventError::AccessDenied(ref cause) => cause,
            SubscribeToEventError::Internal(ref cause) => cause,
            SubscribeToEventError::InvalidInput(ref cause) => cause,
            SubscribeToEventError::LimitExceeded(ref cause) => cause,
            SubscribeToEventError::NoSuchEntity(ref cause) => cause,
            SubscribeToEventError::Validation(ref cause) => cause,
            SubscribeToEventError::Credentials(ref err) => err.description(),
            SubscribeToEventError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            SubscribeToEventError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UnsubscribeFromEvent
#[derive(Debug, PartialEq)]
pub enum UnsubscribeFromEventError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UnsubscribeFromEventError {
    pub fn from_body(body: &str) -> UnsubscribeFromEventError {
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
                    "AccessDeniedException" => {
                        UnsubscribeFromEventError::AccessDenied(String::from(error_message))
                    }
                    "InternalException" => {
                        UnsubscribeFromEventError::Internal(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        UnsubscribeFromEventError::InvalidInput(String::from(error_message))
                    }
                    "NoSuchEntityException" => {
                        UnsubscribeFromEventError::NoSuchEntity(String::from(error_message))
                    }
                    "ValidationException" => {
                        UnsubscribeFromEventError::Validation(error_message.to_string())
                    }
                    _ => UnsubscribeFromEventError::Unknown(String::from(body)),
                }
            }
            Err(_) => UnsubscribeFromEventError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UnsubscribeFromEventError {
    fn from(err: serde_json::error::Error) -> UnsubscribeFromEventError {
        UnsubscribeFromEventError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UnsubscribeFromEventError {
    fn from(err: CredentialsError) -> UnsubscribeFromEventError {
        UnsubscribeFromEventError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UnsubscribeFromEventError {
    fn from(err: HttpDispatchError) -> UnsubscribeFromEventError {
        UnsubscribeFromEventError::HttpDispatch(err)
    }
}
impl From<io::Error> for UnsubscribeFromEventError {
    fn from(err: io::Error) -> UnsubscribeFromEventError {
        UnsubscribeFromEventError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UnsubscribeFromEventError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UnsubscribeFromEventError {
    fn description(&self) -> &str {
        match *self {
            UnsubscribeFromEventError::AccessDenied(ref cause) => cause,
            UnsubscribeFromEventError::Internal(ref cause) => cause,
            UnsubscribeFromEventError::InvalidInput(ref cause) => cause,
            UnsubscribeFromEventError::NoSuchEntity(ref cause) => cause,
            UnsubscribeFromEventError::Validation(ref cause) => cause,
            UnsubscribeFromEventError::Credentials(ref err) => err.description(),
            UnsubscribeFromEventError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UnsubscribeFromEventError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateAssessmentTarget
#[derive(Debug, PartialEq)]
pub enum UpdateAssessmentTargetError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateAssessmentTargetError {
    pub fn from_body(body: &str) -> UpdateAssessmentTargetError {
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
                    "AccessDeniedException" => {
                        UpdateAssessmentTargetError::AccessDenied(String::from(error_message))
                    }
                    "InternalException" => {
                        UpdateAssessmentTargetError::Internal(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        UpdateAssessmentTargetError::InvalidInput(String::from(error_message))
                    }
                    "NoSuchEntityException" => {
                        UpdateAssessmentTargetError::NoSuchEntity(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateAssessmentTargetError::Validation(error_message.to_string())
                    }
                    _ => UpdateAssessmentTargetError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateAssessmentTargetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateAssessmentTargetError {
    fn from(err: serde_json::error::Error) -> UpdateAssessmentTargetError {
        UpdateAssessmentTargetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateAssessmentTargetError {
    fn from(err: CredentialsError) -> UpdateAssessmentTargetError {
        UpdateAssessmentTargetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateAssessmentTargetError {
    fn from(err: HttpDispatchError) -> UpdateAssessmentTargetError {
        UpdateAssessmentTargetError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateAssessmentTargetError {
    fn from(err: io::Error) -> UpdateAssessmentTargetError {
        UpdateAssessmentTargetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateAssessmentTargetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateAssessmentTargetError {
    fn description(&self) -> &str {
        match *self {
            UpdateAssessmentTargetError::AccessDenied(ref cause) => cause,
            UpdateAssessmentTargetError::Internal(ref cause) => cause,
            UpdateAssessmentTargetError::InvalidInput(ref cause) => cause,
            UpdateAssessmentTargetError::NoSuchEntity(ref cause) => cause,
            UpdateAssessmentTargetError::Validation(ref cause) => cause,
            UpdateAssessmentTargetError::Credentials(ref err) => err.description(),
            UpdateAssessmentTargetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateAssessmentTargetError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon Inspector API. Amazon Inspector clients implement this trait.
pub trait Inspector {
    /// <p>Assigns attributes (key and value pairs) to the findings that are specified by the ARNs of the findings.</p>
    fn add_attributes_to_findings(
        &self,
        input: AddAttributesToFindingsRequest,
    ) -> RusotoFuture<AddAttributesToFindingsResponse, AddAttributesToFindingsError>;

    /// <p>Creates a new assessment target using the ARN of the resource group that is generated by <a>CreateResourceGroup</a>. If the <a href="https://docs.aws.amazon.com/inspector/latest/userguide/inspector_slr.html">service-linked role</a> isn’t already registered, also creates and registers a service-linked role to grant Amazon Inspector access to AWS Services needed to perform security assessments. You can create up to 50 assessment targets per AWS account. You can run up to 500 concurrent agents per AWS account. For more information, see <a href="http://docs.aws.amazon.com/inspector/latest/userguide/inspector_applications.html"> Amazon Inspector Assessment Targets</a>.</p>
    fn create_assessment_target(
        &self,
        input: CreateAssessmentTargetRequest,
    ) -> RusotoFuture<CreateAssessmentTargetResponse, CreateAssessmentTargetError>;

    /// <p>Creates an assessment template for the assessment target that is specified by the ARN of the assessment target. If the <a href="https://docs.aws.amazon.com/inspector/latest/userguide/inspector_slr.html">service-linked role</a> isn’t already registered, also creates and registers a service-linked role to grant Amazon Inspector access to AWS Services needed to perform security assessments.</p>
    fn create_assessment_template(
        &self,
        input: CreateAssessmentTemplateRequest,
    ) -> RusotoFuture<CreateAssessmentTemplateResponse, CreateAssessmentTemplateError>;

    /// <p>Creates a resource group using the specified set of tags (key and value pairs) that are used to select the EC2 instances to be included in an Amazon Inspector assessment target. The created resource group is then used to create an Amazon Inspector assessment target. For more information, see <a>CreateAssessmentTarget</a>.</p>
    fn create_resource_group(
        &self,
        input: CreateResourceGroupRequest,
    ) -> RusotoFuture<CreateResourceGroupResponse, CreateResourceGroupError>;

    /// <p>Deletes the assessment run that is specified by the ARN of the assessment run.</p>
    fn delete_assessment_run(
        &self,
        input: DeleteAssessmentRunRequest,
    ) -> RusotoFuture<(), DeleteAssessmentRunError>;

    /// <p>Deletes the assessment target that is specified by the ARN of the assessment target.</p>
    fn delete_assessment_target(
        &self,
        input: DeleteAssessmentTargetRequest,
    ) -> RusotoFuture<(), DeleteAssessmentTargetError>;

    /// <p>Deletes the assessment template that is specified by the ARN of the assessment template.</p>
    fn delete_assessment_template(
        &self,
        input: DeleteAssessmentTemplateRequest,
    ) -> RusotoFuture<(), DeleteAssessmentTemplateError>;

    /// <p>Describes the assessment runs that are specified by the ARNs of the assessment runs.</p>
    fn describe_assessment_runs(
        &self,
        input: DescribeAssessmentRunsRequest,
    ) -> RusotoFuture<DescribeAssessmentRunsResponse, DescribeAssessmentRunsError>;

    /// <p>Describes the assessment targets that are specified by the ARNs of the assessment targets.</p>
    fn describe_assessment_targets(
        &self,
        input: DescribeAssessmentTargetsRequest,
    ) -> RusotoFuture<DescribeAssessmentTargetsResponse, DescribeAssessmentTargetsError>;

    /// <p>Describes the assessment templates that are specified by the ARNs of the assessment templates.</p>
    fn describe_assessment_templates(
        &self,
        input: DescribeAssessmentTemplatesRequest,
    ) -> RusotoFuture<DescribeAssessmentTemplatesResponse, DescribeAssessmentTemplatesError>;

    /// <p>Describes the IAM role that enables Amazon Inspector to access your AWS account.</p>
    fn describe_cross_account_access_role(
        &self,
    ) -> RusotoFuture<DescribeCrossAccountAccessRoleResponse, DescribeCrossAccountAccessRoleError>;

    /// <p>Describes the findings that are specified by the ARNs of the findings.</p>
    fn describe_findings(
        &self,
        input: DescribeFindingsRequest,
    ) -> RusotoFuture<DescribeFindingsResponse, DescribeFindingsError>;

    /// <p>Describes the resource groups that are specified by the ARNs of the resource groups.</p>
    fn describe_resource_groups(
        &self,
        input: DescribeResourceGroupsRequest,
    ) -> RusotoFuture<DescribeResourceGroupsResponse, DescribeResourceGroupsError>;

    /// <p>Describes the rules packages that are specified by the ARNs of the rules packages.</p>
    fn describe_rules_packages(
        &self,
        input: DescribeRulesPackagesRequest,
    ) -> RusotoFuture<DescribeRulesPackagesResponse, DescribeRulesPackagesError>;

    /// <p>Produces an assessment report that includes detailed and comprehensive results of a specified assessment run. </p>
    fn get_assessment_report(
        &self,
        input: GetAssessmentReportRequest,
    ) -> RusotoFuture<GetAssessmentReportResponse, GetAssessmentReportError>;

    /// <p>Information about the data that is collected for the specified assessment run.</p>
    fn get_telemetry_metadata(
        &self,
        input: GetTelemetryMetadataRequest,
    ) -> RusotoFuture<GetTelemetryMetadataResponse, GetTelemetryMetadataError>;

    /// <p>Lists the agents of the assessment runs that are specified by the ARNs of the assessment runs.</p>
    fn list_assessment_run_agents(
        &self,
        input: ListAssessmentRunAgentsRequest,
    ) -> RusotoFuture<ListAssessmentRunAgentsResponse, ListAssessmentRunAgentsError>;

    /// <p>Lists the assessment runs that correspond to the assessment templates that are specified by the ARNs of the assessment templates.</p>
    fn list_assessment_runs(
        &self,
        input: ListAssessmentRunsRequest,
    ) -> RusotoFuture<ListAssessmentRunsResponse, ListAssessmentRunsError>;

    /// <p>Lists the ARNs of the assessment targets within this AWS account. For more information about assessment targets, see <a href="http://docs.aws.amazon.com/inspector/latest/userguide/inspector_applications.html">Amazon Inspector Assessment Targets</a>.</p>
    fn list_assessment_targets(
        &self,
        input: ListAssessmentTargetsRequest,
    ) -> RusotoFuture<ListAssessmentTargetsResponse, ListAssessmentTargetsError>;

    /// <p>Lists the assessment templates that correspond to the assessment targets that are specified by the ARNs of the assessment targets.</p>
    fn list_assessment_templates(
        &self,
        input: ListAssessmentTemplatesRequest,
    ) -> RusotoFuture<ListAssessmentTemplatesResponse, ListAssessmentTemplatesError>;

    /// <p>Lists all the event subscriptions for the assessment template that is specified by the ARN of the assessment template. For more information, see <a>SubscribeToEvent</a> and <a>UnsubscribeFromEvent</a>.</p>
    fn list_event_subscriptions(
        &self,
        input: ListEventSubscriptionsRequest,
    ) -> RusotoFuture<ListEventSubscriptionsResponse, ListEventSubscriptionsError>;

    /// <p>Lists findings that are generated by the assessment runs that are specified by the ARNs of the assessment runs.</p>
    fn list_findings(
        &self,
        input: ListFindingsRequest,
    ) -> RusotoFuture<ListFindingsResponse, ListFindingsError>;

    /// <p>Lists all available Amazon Inspector rules packages.</p>
    fn list_rules_packages(
        &self,
        input: ListRulesPackagesRequest,
    ) -> RusotoFuture<ListRulesPackagesResponse, ListRulesPackagesError>;

    /// <p>Lists all tags associated with an assessment template.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> RusotoFuture<ListTagsForResourceResponse, ListTagsForResourceError>;

    /// <p>Previews the agents installed on the EC2 instances that are part of the specified assessment target.</p>
    fn preview_agents(
        &self,
        input: PreviewAgentsRequest,
    ) -> RusotoFuture<PreviewAgentsResponse, PreviewAgentsError>;

    /// <p>Registers the IAM role that grants Amazon Inspector access to AWS Services needed to perform security assessments.</p>
    fn register_cross_account_access_role(
        &self,
        input: RegisterCrossAccountAccessRoleRequest,
    ) -> RusotoFuture<(), RegisterCrossAccountAccessRoleError>;

    /// <p>Removes entire attributes (key and value pairs) from the findings that are specified by the ARNs of the findings where an attribute with the specified key exists.</p>
    fn remove_attributes_from_findings(
        &self,
        input: RemoveAttributesFromFindingsRequest,
    ) -> RusotoFuture<RemoveAttributesFromFindingsResponse, RemoveAttributesFromFindingsError>;

    /// <p>Sets tags (key and value pairs) to the assessment template that is specified by the ARN of the assessment template.</p>
    fn set_tags_for_resource(
        &self,
        input: SetTagsForResourceRequest,
    ) -> RusotoFuture<(), SetTagsForResourceError>;

    /// <p>Starts the assessment run specified by the ARN of the assessment template. For this API to function properly, you must not exceed the limit of running up to 500 concurrent agents per AWS account.</p>
    fn start_assessment_run(
        &self,
        input: StartAssessmentRunRequest,
    ) -> RusotoFuture<StartAssessmentRunResponse, StartAssessmentRunError>;

    /// <p>Stops the assessment run that is specified by the ARN of the assessment run.</p>
    fn stop_assessment_run(
        &self,
        input: StopAssessmentRunRequest,
    ) -> RusotoFuture<(), StopAssessmentRunError>;

    /// <p>Enables the process of sending Amazon Simple Notification Service (SNS) notifications about a specified event to a specified SNS topic.</p>
    fn subscribe_to_event(
        &self,
        input: SubscribeToEventRequest,
    ) -> RusotoFuture<(), SubscribeToEventError>;

    /// <p>Disables the process of sending Amazon Simple Notification Service (SNS) notifications about a specified event to a specified SNS topic.</p>
    fn unsubscribe_from_event(
        &self,
        input: UnsubscribeFromEventRequest,
    ) -> RusotoFuture<(), UnsubscribeFromEventError>;

    /// <p>Updates the assessment target that is specified by the ARN of the assessment target.</p>
    fn update_assessment_target(
        &self,
        input: UpdateAssessmentTargetRequest,
    ) -> RusotoFuture<(), UpdateAssessmentTargetError>;
}
/// A client for the Amazon Inspector API.
pub struct InspectorClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl InspectorClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> InspectorClient {
        InspectorClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> InspectorClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        InspectorClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> Inspector for InspectorClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p>Assigns attributes (key and value pairs) to the findings that are specified by the ARNs of the findings.</p>
    fn add_attributes_to_findings(
        &self,
        input: AddAttributesToFindingsRequest,
    ) -> RusotoFuture<AddAttributesToFindingsResponse, AddAttributesToFindingsError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.AddAttributesToFindings");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<AddAttributesToFindingsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AddAttributesToFindingsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a new assessment target using the ARN of the resource group that is generated by <a>CreateResourceGroup</a>. If the <a href="https://docs.aws.amazon.com/inspector/latest/userguide/inspector_slr.html">service-linked role</a> isn’t already registered, also creates and registers a service-linked role to grant Amazon Inspector access to AWS Services needed to perform security assessments. You can create up to 50 assessment targets per AWS account. You can run up to 500 concurrent agents per AWS account. For more information, see <a href="http://docs.aws.amazon.com/inspector/latest/userguide/inspector_applications.html"> Amazon Inspector Assessment Targets</a>.</p>
    fn create_assessment_target(
        &self,
        input: CreateAssessmentTargetRequest,
    ) -> RusotoFuture<CreateAssessmentTargetResponse, CreateAssessmentTargetError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.CreateAssessmentTarget");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateAssessmentTargetResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateAssessmentTargetError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates an assessment template for the assessment target that is specified by the ARN of the assessment target. If the <a href="https://docs.aws.amazon.com/inspector/latest/userguide/inspector_slr.html">service-linked role</a> isn’t already registered, also creates and registers a service-linked role to grant Amazon Inspector access to AWS Services needed to perform security assessments.</p>
    fn create_assessment_template(
        &self,
        input: CreateAssessmentTemplateRequest,
    ) -> RusotoFuture<CreateAssessmentTemplateResponse, CreateAssessmentTemplateError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.CreateAssessmentTemplate");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateAssessmentTemplateResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateAssessmentTemplateError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a resource group using the specified set of tags (key and value pairs) that are used to select the EC2 instances to be included in an Amazon Inspector assessment target. The created resource group is then used to create an Amazon Inspector assessment target. For more information, see <a>CreateAssessmentTarget</a>.</p>
    fn create_resource_group(
        &self,
        input: CreateResourceGroupRequest,
    ) -> RusotoFuture<CreateResourceGroupResponse, CreateResourceGroupError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.CreateResourceGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateResourceGroupResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateResourceGroupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the assessment run that is specified by the ARN of the assessment run.</p>
    fn delete_assessment_run(
        &self,
        input: DeleteAssessmentRunRequest,
    ) -> RusotoFuture<(), DeleteAssessmentRunError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.DeleteAssessmentRun");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteAssessmentRunError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the assessment target that is specified by the ARN of the assessment target.</p>
    fn delete_assessment_target(
        &self,
        input: DeleteAssessmentTargetRequest,
    ) -> RusotoFuture<(), DeleteAssessmentTargetError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.DeleteAssessmentTarget");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteAssessmentTargetError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the assessment template that is specified by the ARN of the assessment template.</p>
    fn delete_assessment_template(
        &self,
        input: DeleteAssessmentTemplateRequest,
    ) -> RusotoFuture<(), DeleteAssessmentTemplateError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.DeleteAssessmentTemplate");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteAssessmentTemplateError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Describes the assessment runs that are specified by the ARNs of the assessment runs.</p>
    fn describe_assessment_runs(
        &self,
        input: DescribeAssessmentRunsRequest,
    ) -> RusotoFuture<DescribeAssessmentRunsResponse, DescribeAssessmentRunsError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.DescribeAssessmentRuns");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeAssessmentRunsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeAssessmentRunsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Describes the assessment targets that are specified by the ARNs of the assessment targets.</p>
    fn describe_assessment_targets(
        &self,
        input: DescribeAssessmentTargetsRequest,
    ) -> RusotoFuture<DescribeAssessmentTargetsResponse, DescribeAssessmentTargetsError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.DescribeAssessmentTargets");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeAssessmentTargetsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeAssessmentTargetsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Describes the assessment templates that are specified by the ARNs of the assessment templates.</p>
    fn describe_assessment_templates(
        &self,
        input: DescribeAssessmentTemplatesRequest,
    ) -> RusotoFuture<DescribeAssessmentTemplatesResponse, DescribeAssessmentTemplatesError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "InspectorService.DescribeAssessmentTemplates",
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

                    serde_json::from_str::<DescribeAssessmentTemplatesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeAssessmentTemplatesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Describes the IAM role that enables Amazon Inspector to access your AWS account.</p>
    fn describe_cross_account_access_role(
        &self,
    ) -> RusotoFuture<DescribeCrossAccountAccessRoleResponse, DescribeCrossAccountAccessRoleError>
    {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "InspectorService.DescribeCrossAccountAccessRole",
        );
        request.set_payload(Some(b"{}".to_vec()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeCrossAccountAccessRoleResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeCrossAccountAccessRoleError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Describes the findings that are specified by the ARNs of the findings.</p>
    fn describe_findings(
        &self,
        input: DescribeFindingsRequest,
    ) -> RusotoFuture<DescribeFindingsResponse, DescribeFindingsError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.DescribeFindings");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeFindingsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeFindingsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Describes the resource groups that are specified by the ARNs of the resource groups.</p>
    fn describe_resource_groups(
        &self,
        input: DescribeResourceGroupsRequest,
    ) -> RusotoFuture<DescribeResourceGroupsResponse, DescribeResourceGroupsError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.DescribeResourceGroups");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeResourceGroupsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeResourceGroupsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Describes the rules packages that are specified by the ARNs of the rules packages.</p>
    fn describe_rules_packages(
        &self,
        input: DescribeRulesPackagesRequest,
    ) -> RusotoFuture<DescribeRulesPackagesResponse, DescribeRulesPackagesError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.DescribeRulesPackages");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeRulesPackagesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeRulesPackagesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Produces an assessment report that includes detailed and comprehensive results of a specified assessment run. </p>
    fn get_assessment_report(
        &self,
        input: GetAssessmentReportRequest,
    ) -> RusotoFuture<GetAssessmentReportResponse, GetAssessmentReportError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.GetAssessmentReport");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetAssessmentReportResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetAssessmentReportError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Information about the data that is collected for the specified assessment run.</p>
    fn get_telemetry_metadata(
        &self,
        input: GetTelemetryMetadataRequest,
    ) -> RusotoFuture<GetTelemetryMetadataResponse, GetTelemetryMetadataError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.GetTelemetryMetadata");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetTelemetryMetadataResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetTelemetryMetadataError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the agents of the assessment runs that are specified by the ARNs of the assessment runs.</p>
    fn list_assessment_run_agents(
        &self,
        input: ListAssessmentRunAgentsRequest,
    ) -> RusotoFuture<ListAssessmentRunAgentsResponse, ListAssessmentRunAgentsError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.ListAssessmentRunAgents");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListAssessmentRunAgentsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListAssessmentRunAgentsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the assessment runs that correspond to the assessment templates that are specified by the ARNs of the assessment templates.</p>
    fn list_assessment_runs(
        &self,
        input: ListAssessmentRunsRequest,
    ) -> RusotoFuture<ListAssessmentRunsResponse, ListAssessmentRunsError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.ListAssessmentRuns");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListAssessmentRunsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListAssessmentRunsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the ARNs of the assessment targets within this AWS account. For more information about assessment targets, see <a href="http://docs.aws.amazon.com/inspector/latest/userguide/inspector_applications.html">Amazon Inspector Assessment Targets</a>.</p>
    fn list_assessment_targets(
        &self,
        input: ListAssessmentTargetsRequest,
    ) -> RusotoFuture<ListAssessmentTargetsResponse, ListAssessmentTargetsError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.ListAssessmentTargets");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListAssessmentTargetsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListAssessmentTargetsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the assessment templates that correspond to the assessment targets that are specified by the ARNs of the assessment targets.</p>
    fn list_assessment_templates(
        &self,
        input: ListAssessmentTemplatesRequest,
    ) -> RusotoFuture<ListAssessmentTemplatesResponse, ListAssessmentTemplatesError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.ListAssessmentTemplates");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListAssessmentTemplatesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListAssessmentTemplatesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists all the event subscriptions for the assessment template that is specified by the ARN of the assessment template. For more information, see <a>SubscribeToEvent</a> and <a>UnsubscribeFromEvent</a>.</p>
    fn list_event_subscriptions(
        &self,
        input: ListEventSubscriptionsRequest,
    ) -> RusotoFuture<ListEventSubscriptionsResponse, ListEventSubscriptionsError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.ListEventSubscriptions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListEventSubscriptionsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListEventSubscriptionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists findings that are generated by the assessment runs that are specified by the ARNs of the assessment runs.</p>
    fn list_findings(
        &self,
        input: ListFindingsRequest,
    ) -> RusotoFuture<ListFindingsResponse, ListFindingsError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.ListFindings");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListFindingsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListFindingsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists all available Amazon Inspector rules packages.</p>
    fn list_rules_packages(
        &self,
        input: ListRulesPackagesRequest,
    ) -> RusotoFuture<ListRulesPackagesResponse, ListRulesPackagesError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.ListRulesPackages");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListRulesPackagesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListRulesPackagesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists all tags associated with an assessment template.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> RusotoFuture<ListTagsForResourceResponse, ListTagsForResourceError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.ListTagsForResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListTagsForResourceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListTagsForResourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Previews the agents installed on the EC2 instances that are part of the specified assessment target.</p>
    fn preview_agents(
        &self,
        input: PreviewAgentsRequest,
    ) -> RusotoFuture<PreviewAgentsResponse, PreviewAgentsError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.PreviewAgents");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PreviewAgentsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PreviewAgentsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Registers the IAM role that grants Amazon Inspector access to AWS Services needed to perform security assessments.</p>
    fn register_cross_account_access_role(
        &self,
        input: RegisterCrossAccountAccessRoleRequest,
    ) -> RusotoFuture<(), RegisterCrossAccountAccessRoleError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "InspectorService.RegisterCrossAccountAccessRole",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RegisterCrossAccountAccessRoleError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Removes entire attributes (key and value pairs) from the findings that are specified by the ARNs of the findings where an attribute with the specified key exists.</p>
    fn remove_attributes_from_findings(
        &self,
        input: RemoveAttributesFromFindingsRequest,
    ) -> RusotoFuture<RemoveAttributesFromFindingsResponse, RemoveAttributesFromFindingsError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "InspectorService.RemoveAttributesFromFindings",
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

                    serde_json::from_str::<RemoveAttributesFromFindingsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RemoveAttributesFromFindingsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Sets tags (key and value pairs) to the assessment template that is specified by the ARN of the assessment template.</p>
    fn set_tags_for_resource(
        &self,
        input: SetTagsForResourceRequest,
    ) -> RusotoFuture<(), SetTagsForResourceError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.SetTagsForResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SetTagsForResourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Starts the assessment run specified by the ARN of the assessment template. For this API to function properly, you must not exceed the limit of running up to 500 concurrent agents per AWS account.</p>
    fn start_assessment_run(
        &self,
        input: StartAssessmentRunRequest,
    ) -> RusotoFuture<StartAssessmentRunResponse, StartAssessmentRunError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.StartAssessmentRun");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StartAssessmentRunResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(StartAssessmentRunError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Stops the assessment run that is specified by the ARN of the assessment run.</p>
    fn stop_assessment_run(
        &self,
        input: StopAssessmentRunRequest,
    ) -> RusotoFuture<(), StopAssessmentRunError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.StopAssessmentRun");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(StopAssessmentRunError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Enables the process of sending Amazon Simple Notification Service (SNS) notifications about a specified event to a specified SNS topic.</p>
    fn subscribe_to_event(
        &self,
        input: SubscribeToEventRequest,
    ) -> RusotoFuture<(), SubscribeToEventError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.SubscribeToEvent");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SubscribeToEventError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Disables the process of sending Amazon Simple Notification Service (SNS) notifications about a specified event to a specified SNS topic.</p>
    fn unsubscribe_from_event(
        &self,
        input: UnsubscribeFromEventRequest,
    ) -> RusotoFuture<(), UnsubscribeFromEventError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.UnsubscribeFromEvent");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UnsubscribeFromEventError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates the assessment target that is specified by the ARN of the assessment target.</p>
    fn update_assessment_target(
        &self,
        input: UpdateAssessmentTargetRequest,
    ) -> RusotoFuture<(), UpdateAssessmentTargetError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.UpdateAssessmentTarget");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateAssessmentTargetError::from_body(
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
