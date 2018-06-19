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
/// <p>A collection of accounts and regions.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccountAggregationSource {
    /// <p>The 12-digit account ID of the account being aggregated. </p>
    #[serde(rename = "AccountIds")]
    pub account_ids: Vec<String>,
    /// <p>If true, aggreagate existing AWS Config regions and future regions.</p>
    #[serde(rename = "AllAwsRegions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_aws_regions: Option<bool>,
    /// <p>The source regions being aggregated.</p>
    #[serde(rename = "AwsRegions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_regions: Option<Vec<String>>,
}

/// <p>Indicates whether an AWS Config rule is compliant based on account ID, region, compliance, and rule name.</p> <p>A rule is compliant if all of the resources that the rule evaluated comply with it. It is noncompliant if any of these resources do not comply.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AggregateComplianceByConfigRule {
    /// <p>The 12-digit account ID of the source account.</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The source region from where the data is aggregated.</p>
    #[serde(rename = "AwsRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
    /// <p>Indicates whether an AWS resource or AWS Config rule is compliant and provides the number of contributors that affect the compliance.</p>
    #[serde(rename = "Compliance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance: Option<Compliance>,
    /// <p>The name of the AWS Config rule.</p>
    #[serde(rename = "ConfigRuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_name: Option<String>,
}

/// <p>Returns the number of compliant and noncompliant rules for one or more accounts and regions in an aggregator.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AggregateComplianceCount {
    /// <p>The number of compliant and noncompliant AWS Config rules.</p>
    #[serde(rename = "ComplianceSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_summary: Option<ComplianceSummary>,
    /// <p>The 12-digit account ID or region based on the GroupByKey value.</p>
    #[serde(rename = "GroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}

/// <p>The details of an AWS Config evaluation for an account ID and region in an aggregator. Provides the AWS resource that was evaluated, the compliance of the resource, related time stamps, and supplementary information. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AggregateEvaluationResult {
    /// <p>The 12-digit account ID of the source account.</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>Supplementary information about how the agrregate evaluation determined the compliance.</p>
    #[serde(rename = "Annotation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation: Option<String>,
    /// <p>The source region from where the data is aggregated.</p>
    #[serde(rename = "AwsRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
    /// <p>The resource compliance status.</p> <p>For the <code>AggregationEvaluationResult</code> data type, AWS Config supports only the <code>COMPLIANT</code> and <code>NON_COMPLIANT</code>. AWS Config does not support the <code>NOT_APPLICABLE</code> and <code>INSUFFICIENT_DATA</code> value.</p>
    #[serde(rename = "ComplianceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_type: Option<String>,
    /// <p>The time when the AWS Config rule evaluated the AWS resource.</p>
    #[serde(rename = "ConfigRuleInvokedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_invoked_time: Option<f64>,
    /// <p>Uniquely identifies the evaluation result.</p>
    #[serde(rename = "EvaluationResultIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_result_identifier: Option<EvaluationResultIdentifier>,
    /// <p>The time when AWS Config recorded the aggregate evaluation result.</p>
    #[serde(rename = "ResultRecordedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_recorded_time: Option<f64>,
}

/// <p>The current sync status between the source and the aggregator account.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AggregatedSourceStatus {
    /// <p>The region authorized to collect aggregated data.</p>
    #[serde(rename = "AwsRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
    /// <p>The error code that AWS Config returned when the source account aggregation last failed.</p>
    #[serde(rename = "LastErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_code: Option<String>,
    /// <p>The message indicating that the source account aggregation failed due to an error.</p>
    #[serde(rename = "LastErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_message: Option<String>,
    /// <p><p>Filters the last updated status type.</p> <ul> <li> <p>Valid value FAILED indicates errors while moving data.</p> </li> <li> <p>Valid value SUCCEEDED indicates the data was successfully moved.</p> </li> <li> <p>Valid value OUTDATED indicates the data is not the most recent.</p> </li> </ul></p>
    #[serde(rename = "LastUpdateStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_status: Option<String>,
    /// <p>The time of the last update.</p>
    #[serde(rename = "LastUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    /// <p>The source account ID or an organization.</p>
    #[serde(rename = "SourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    /// <p>The source account or an organization.</p>
    #[serde(rename = "SourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
}

/// <p>An object that represents the authorizations granted to aggregator accounts and regions.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AggregationAuthorization {
    /// <p>The Amazon Resource Name (ARN) of the aggregation object.</p>
    #[serde(rename = "AggregationAuthorizationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_authorization_arn: Option<String>,
    /// <p>The 12-digit account ID of the account authorized to aggregate data.</p>
    #[serde(rename = "AuthorizedAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_account_id: Option<String>,
    /// <p>The region authorized to collect aggregated data.</p>
    #[serde(rename = "AuthorizedAwsRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_aws_region: Option<String>,
    /// <p>The time stamp when the aggregation authorization was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
}

/// <p>The detailed configuration of a specified resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BaseConfigurationItem {
    /// <p>The 12 digit AWS account ID associated with the resource.</p>
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The Availability Zone associated with the resource.</p>
    #[serde(rename = "availabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>The region where the resource resides.</p>
    #[serde(rename = "awsRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
    /// <p>The description of the resource configuration.</p>
    #[serde(rename = "configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<String>,
    /// <p>The time when the configuration recording was initiated.</p>
    #[serde(rename = "configurationItemCaptureTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_item_capture_time: Option<f64>,
    /// <p>The configuration item status.</p>
    #[serde(rename = "configurationItemStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_item_status: Option<String>,
    /// <p>An identifier that indicates the ordering of the configuration items of a resource.</p>
    #[serde(rename = "configurationStateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_state_id: Option<String>,
    /// <p>The time stamp when the resource was created.</p>
    #[serde(rename = "resourceCreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_creation_time: Option<f64>,
    /// <p>The ID of the resource (for example., sg-xxxxxx).</p>
    #[serde(rename = "resourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>The custom name of the resource, if available.</p>
    #[serde(rename = "resourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    /// <p>The type of AWS resource.</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>Configuration attributes that AWS Config returns for certain resource types to supplement the information returned for the configuration parameter.</p>
    #[serde(rename = "supplementaryConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplementary_configuration: Option<::std::collections::HashMap<String, String>>,
    /// <p>The version number of the resource configuration.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchGetResourceConfigRequest {
    /// <p>A list of resource keys to be processed with the current request. Each element in the list consists of the resource type and resource ID.</p>
    #[serde(rename = "resourceKeys")]
    pub resource_keys: Vec<ResourceKey>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchGetResourceConfigResponse {
    /// <p>A list that contains the current configuration of one or more resources.</p>
    #[serde(rename = "baseConfigurationItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_configuration_items: Option<Vec<BaseConfigurationItem>>,
    /// <p>A list of resource keys that were not processed with the current response. The unprocessesResourceKeys value is in the same form as ResourceKeys, so the value can be directly provided to a subsequent BatchGetResourceConfig operation. If there are no unprocessed resource keys, the response contains an empty unprocessedResourceKeys list. </p>
    #[serde(rename = "unprocessedResourceKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_resource_keys: Option<Vec<ResourceKey>>,
}

/// <p>Indicates whether an AWS resource or AWS Config rule is compliant and provides the number of contributors that affect the compliance.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Compliance {
    /// <p>The number of AWS resources or AWS Config rules that cause a result of <code>NON_COMPLIANT</code>, up to a maximum number.</p>
    #[serde(rename = "ComplianceContributorCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_contributor_count: Option<ComplianceContributorCount>,
    /// <p>Indicates whether an AWS resource or AWS Config rule is compliant.</p> <p>A resource is compliant if it complies with all of the AWS Config rules that evaluate it. A resource is noncompliant if it does not comply with one or more of these rules.</p> <p>A rule is compliant if all of the resources that the rule evaluates comply with it. A rule is noncompliant if any of these resources do not comply.</p> <p>AWS Config returns the <code>INSUFFICIENT_DATA</code> value when no evaluation results are available for the AWS resource or AWS Config rule.</p> <p>For the <code>Compliance</code> data type, AWS Config supports only <code>COMPLIANT</code>, <code>NON_COMPLIANT</code>, and <code>INSUFFICIENT_DATA</code> values. AWS Config does not support the <code>NOT_APPLICABLE</code> value for the <code>Compliance</code> data type.</p>
    #[serde(rename = "ComplianceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_type: Option<String>,
}

/// <p>Indicates whether an AWS Config rule is compliant. A rule is compliant if all of the resources that the rule evaluated comply with it. A rule is noncompliant if any of these resources do not comply.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ComplianceByConfigRule {
    /// <p>Indicates whether the AWS Config rule is compliant.</p>
    #[serde(rename = "Compliance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance: Option<Compliance>,
    /// <p>The name of the AWS Config rule.</p>
    #[serde(rename = "ConfigRuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_name: Option<String>,
}

/// <p>Indicates whether an AWS resource that is evaluated according to one or more AWS Config rules is compliant. A resource is compliant if it complies with all of the rules that evaluate it. A resource is noncompliant if it does not comply with one or more of these rules.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ComplianceByResource {
    /// <p>Indicates whether the AWS resource complies with all of the AWS Config rules that evaluated it.</p>
    #[serde(rename = "Compliance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance: Option<Compliance>,
    /// <p>The ID of the AWS resource that was evaluated.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>The type of the AWS resource that was evaluated.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

/// <p>The number of AWS resources or AWS Config rules responsible for the current compliance of the item, up to a maximum number.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ComplianceContributorCount {
    /// <p>Indicates whether the maximum count is reached.</p>
    #[serde(rename = "CapExceeded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cap_exceeded: Option<bool>,
    /// <p>The number of AWS resources or AWS Config rules responsible for the current compliance of the item.</p>
    #[serde(rename = "CappedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capped_count: Option<i64>,
}

/// <p>The number of AWS Config rules or AWS resources that are compliant and noncompliant.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ComplianceSummary {
    /// <p>The time that AWS Config created the compliance summary.</p>
    #[serde(rename = "ComplianceSummaryTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_summary_timestamp: Option<f64>,
    /// <p>The number of AWS Config rules or AWS resources that are compliant, up to a maximum of 25 for rules and 100 for resources.</p>
    #[serde(rename = "CompliantResourceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliant_resource_count: Option<ComplianceContributorCount>,
    /// <p>The number of AWS Config rules or AWS resources that are noncompliant, up to a maximum of 25 for rules and 100 for resources.</p>
    #[serde(rename = "NonCompliantResourceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_compliant_resource_count: Option<ComplianceContributorCount>,
}

/// <p>The number of AWS resources of a specific type that are compliant or noncompliant, up to a maximum of 100 for each.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ComplianceSummaryByResourceType {
    /// <p>The number of AWS resources that are compliant or noncompliant, up to a maximum of 100 for each.</p>
    #[serde(rename = "ComplianceSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_summary: Option<ComplianceSummary>,
    /// <p>The type of AWS resource.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

/// <p>Provides status of the delivery of the snapshot or the configuration history to the specified Amazon S3 bucket. Also provides the status of notifications about the Amazon S3 delivery to the specified Amazon SNS topic.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ConfigExportDeliveryInfo {
    /// <p>The time of the last attempted delivery.</p>
    #[serde(rename = "lastAttemptTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_attempt_time: Option<f64>,
    /// <p>The error code from the last attempted delivery.</p>
    #[serde(rename = "lastErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_code: Option<String>,
    /// <p>The error message from the last attempted delivery.</p>
    #[serde(rename = "lastErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_message: Option<String>,
    /// <p>Status of the last attempted delivery.</p>
    #[serde(rename = "lastStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status: Option<String>,
    /// <p>The time of the last successful delivery.</p>
    #[serde(rename = "lastSuccessfulTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_time: Option<f64>,
    /// <p>The time that the next delivery occurs.</p>
    #[serde(rename = "nextDeliveryTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_delivery_time: Option<f64>,
}

/// <p>An AWS Config rule represents an AWS Lambda function that you create for a custom rule or a predefined function for an AWS managed rule. The function evaluates configuration items to assess whether your AWS resources comply with your desired configurations. This function can run when AWS Config detects a configuration change to an AWS resource and at a periodic frequency that you choose (for example, every 24 hours).</p> <note> <p>You can use the AWS CLI and AWS SDKs if you want to create a rule that triggers evaluations for your resources when AWS Config delivers the configuration snapshot. For more information, see <a>ConfigSnapshotDeliveryProperties</a>.</p> </note> <p>For more information about developing and using AWS Config rules, see <a href="http://docs.aws.amazon.com/config/latest/developerguide/evaluate-config.html">Evaluating AWS Resource Configurations with AWS Config</a> in the <i>AWS Config Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigRule {
    /// <p>The Amazon Resource Name (ARN) of the AWS Config rule.</p>
    #[serde(rename = "ConfigRuleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_arn: Option<String>,
    /// <p>The ID of the AWS Config rule.</p>
    #[serde(rename = "ConfigRuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_id: Option<String>,
    /// <p>The name that you assign to the AWS Config rule. The name is required if you are adding a new rule.</p>
    #[serde(rename = "ConfigRuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_name: Option<String>,
    /// <p>Indicates whether the AWS Config rule is active or is currently being deleted by AWS Config. It can also indicate the evaluation status for the AWS Config rule.</p> <p>AWS Config sets the state of the rule to <code>EVALUATING</code> temporarily after you use the <code>StartConfigRulesEvaluation</code> request to evaluate your resources against the AWS Config rule.</p> <p>AWS Config sets the state of the rule to <code>DELETING_RESULTS</code> temporarily after you use the <code>DeleteEvaluationResults</code> request to delete the current evaluation results for the AWS Config rule.</p> <p>AWS Config temporarily sets the state of a rule to <code>DELETING</code> after you use the <code>DeleteConfigRule</code> request to delete the rule. After AWS Config deletes the rule, the rule and all of its evaluations are erased and are no longer available.</p>
    #[serde(rename = "ConfigRuleState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_state: Option<String>,
    /// <p>The description that you provide for the AWS Config rule.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A string, in JSON format, that is passed to the AWS Config rule Lambda function.</p>
    #[serde(rename = "InputParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_parameters: Option<String>,
    /// <p><p>The maximum frequency with which AWS Config runs evaluations for a rule. You can specify a value for <code>MaximumExecutionFrequency</code> when:</p> <ul> <li> <p>You are using an AWS managed rule that is triggered at a periodic frequency.</p> </li> <li> <p>Your custom rule is triggered when AWS Config delivers the configuration snapshot. For more information, see <a>ConfigSnapshotDeliveryProperties</a>.</p> </li> </ul> <note> <p>By default, rules with a periodic trigger are evaluated every 24 hours. To change the frequency, specify a valid value for the <code>MaximumExecutionFrequency</code> parameter.</p> </note></p>
    #[serde(rename = "MaximumExecutionFrequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_execution_frequency: Option<String>,
    /// <p>Defines which resources can trigger an evaluation for the rule. The scope can include one or more resource types, a combination of one resource type and one resource ID, or a combination of a tag key and value. Specify a scope to constrain the resources that can trigger an evaluation for the rule. If you do not specify a scope, evaluations are triggered when any resource in the recording group changes.</p>
    #[serde(rename = "Scope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<Scope>,
    /// <p>Provides the rule owner (AWS or customer), the rule identifier, and the notifications that cause the function to evaluate your AWS resources.</p>
    #[serde(rename = "Source")]
    pub source: Source,
}

/// <p>Filters the compliance results based on account ID, region, compliance type, and rule name.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ConfigRuleComplianceFilters {
    /// <p>The 12-digit account ID of the source account. </p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The source region where the data is aggregated. </p>
    #[serde(rename = "AwsRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
    /// <p>The rule compliance status.</p> <p>For the <code>ConfigRuleComplianceFilters</code> data type, AWS Config supports only <code>COMPLIANT</code> and <code>NON_COMPLIANT</code>. AWS Config does not support the <code>NOT_APPLICABLE</code> and the <code>INSUFFICIENT_DATA</code> values.</p>
    #[serde(rename = "ComplianceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_type: Option<String>,
    /// <p>The name of the AWS Config rule.</p>
    #[serde(rename = "ConfigRuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_name: Option<String>,
}

/// <p>Filters the results based on the account IDs and regions.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ConfigRuleComplianceSummaryFilters {
    /// <p>The 12-digit account ID of the source account.</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The source region where the data is aggregated.</p>
    #[serde(rename = "AwsRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
}

/// <p>Status information for your AWS managed Config rules. The status includes information such as the last time the rule ran, the last time it failed, and the related error for the last failure.</p> <p>This action does not return status information about custom AWS Config rules.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ConfigRuleEvaluationStatus {
    /// <p>The Amazon Resource Name (ARN) of the AWS Config rule.</p>
    #[serde(rename = "ConfigRuleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_arn: Option<String>,
    /// <p>The ID of the AWS Config rule.</p>
    #[serde(rename = "ConfigRuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_id: Option<String>,
    /// <p>The name of the AWS Config rule.</p>
    #[serde(rename = "ConfigRuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_name: Option<String>,
    /// <p>The time that you first activated the AWS Config rule.</p>
    #[serde(rename = "FirstActivatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_activated_time: Option<f64>,
    /// <p><p>Indicates whether AWS Config has evaluated your resources against the rule at least once.</p> <ul> <li> <p> <code>true</code> - AWS Config has evaluated your AWS resources against the rule at least once.</p> </li> <li> <p> <code>false</code> - AWS Config has not once finished evaluating your AWS resources against the rule.</p> </li> </ul></p>
    #[serde(rename = "FirstEvaluationStarted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_evaluation_started: Option<bool>,
    /// <p>The error code that AWS Config returned when the rule last failed.</p>
    #[serde(rename = "LastErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_code: Option<String>,
    /// <p>The error message that AWS Config returned when the rule last failed.</p>
    #[serde(rename = "LastErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_message: Option<String>,
    /// <p>The time that AWS Config last failed to evaluate your AWS resources against the rule.</p>
    #[serde(rename = "LastFailedEvaluationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_failed_evaluation_time: Option<f64>,
    /// <p>The time that AWS Config last failed to invoke the AWS Config rule to evaluate your AWS resources.</p>
    #[serde(rename = "LastFailedInvocationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_failed_invocation_time: Option<f64>,
    /// <p>The time that AWS Config last successfully evaluated your AWS resources against the rule.</p>
    #[serde(rename = "LastSuccessfulEvaluationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_evaluation_time: Option<f64>,
    /// <p>The time that AWS Config last successfully invoked the AWS Config rule to evaluate your AWS resources.</p>
    #[serde(rename = "LastSuccessfulInvocationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_invocation_time: Option<f64>,
}

/// <p>Provides options for how often AWS Config delivers configuration snapshots to the Amazon S3 bucket in your delivery channel.</p> <note> <p>If you want to create a rule that triggers evaluations for your resources when AWS Config delivers the configuration snapshot, see the following:</p> </note> <p>The frequency for a rule that triggers evaluations for your resources when AWS Config delivers the configuration snapshot is set by one of two values, depending on which is less frequent:</p> <ul> <li> <p>The value for the <code>deliveryFrequency</code> parameter within the delivery channel configuration, which sets how often AWS Config delivers configuration snapshots. This value also sets how often AWS Config invokes evaluations for AWS Config rules.</p> </li> <li> <p>The value for the <code>MaximumExecutionFrequency</code> parameter, which sets the maximum frequency with which AWS Config invokes evaluations for the rule. For more information, see <a>ConfigRule</a>.</p> </li> </ul> <p>If the <code>deliveryFrequency</code> value is less frequent than the <code>MaximumExecutionFrequency</code> value for a rule, AWS Config invokes the rule only as often as the <code>deliveryFrequency</code> value.</p> <ol> <li> <p>For example, you want your rule to run evaluations when AWS Config delivers the configuration snapshot.</p> </li> <li> <p>You specify the <code>MaximumExecutionFrequency</code> value for <code>Six_Hours</code>. </p> </li> <li> <p>You then specify the delivery channel <code>deliveryFrequency</code> value for <code>TwentyFour_Hours</code>.</p> </li> <li> <p>Because the value for <code>deliveryFrequency</code> is less frequent than <code>MaximumExecutionFrequency</code>, AWS Config invokes evaluations for the rule every 24 hours. </p> </li> </ol> <p>You should set the <code>MaximumExecutionFrequency</code> value to be at least as frequent as the <code>deliveryFrequency</code> value. You can view the <code>deliveryFrequency</code> value by using the <code>DescribeDeliveryChannnels</code> action.</p> <p>To update the <code>deliveryFrequency</code> with which AWS Config delivers your configuration snapshots, use the <code>PutDeliveryChannel</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigSnapshotDeliveryProperties {
    /// <p>The frequency with which AWS Config delivers configuration snapshots.</p>
    #[serde(rename = "deliveryFrequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_frequency: Option<String>,
}

/// <p>A list that contains the status of the delivery of the configuration stream notification to the Amazon SNS topic.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ConfigStreamDeliveryInfo {
    /// <p>The error code from the last attempted delivery.</p>
    #[serde(rename = "lastErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_code: Option<String>,
    /// <p>The error message from the last attempted delivery.</p>
    #[serde(rename = "lastErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_message: Option<String>,
    /// <p>Status of the last attempted delivery.</p> <p> <b>Note</b> Providing an SNS topic on a <a href="http://docs.aws.amazon.com/config/latest/APIReference/API_DeliveryChannel.html">DeliveryChannel</a> for AWS Config is optional. If the SNS delivery is turned off, the last status will be <b>Not_Applicable</b>.</p>
    #[serde(rename = "lastStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status: Option<String>,
    /// <p>The time from the last status change.</p>
    #[serde(rename = "lastStatusChangeTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status_change_time: Option<f64>,
}

/// <p>The details about the configuration aggregator, including information about source accounts, regions, and metadata of the aggregator. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ConfigurationAggregator {
    /// <p>Provides a list of source accounts and regions to be aggregated.</p>
    #[serde(rename = "AccountAggregationSources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_aggregation_sources: Option<Vec<AccountAggregationSource>>,
    /// <p>The Amazon Resource Name (ARN) of the aggregator.</p>
    #[serde(rename = "ConfigurationAggregatorArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_aggregator_arn: Option<String>,
    /// <p>The name of the aggregator.</p>
    #[serde(rename = "ConfigurationAggregatorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_aggregator_name: Option<String>,
    /// <p>The time stamp when the configuration aggregator was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The time of the last update.</p>
    #[serde(rename = "LastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    /// <p>Provides an organization and list of regions to be aggregated.</p>
    #[serde(rename = "OrganizationAggregationSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_aggregation_source: Option<OrganizationAggregationSource>,
}

/// <p>A list that contains detailed configurations of a specified resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ConfigurationItem {
    /// <p>The 12-digit AWS account ID associated with the resource.</p>
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The Availability Zone associated with the resource.</p>
    #[serde(rename = "availabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>The region where the resource resides.</p>
    #[serde(rename = "awsRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
    /// <p>The description of the resource configuration.</p>
    #[serde(rename = "configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<String>,
    /// <p>The time when the configuration recording was initiated.</p>
    #[serde(rename = "configurationItemCaptureTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_item_capture_time: Option<f64>,
    /// <p>Unique MD5 hash that represents the configuration item's state.</p> <p>You can use MD5 hash to compare the states of two or more configuration items that are associated with the same resource.</p>
    #[serde(rename = "configurationItemMD5Hash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_item_md5_hash: Option<String>,
    /// <p>The configuration item status.</p>
    #[serde(rename = "configurationItemStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_item_status: Option<String>,
    /// <p>An identifier that indicates the ordering of the configuration items of a resource.</p>
    #[serde(rename = "configurationStateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_state_id: Option<String>,
    /// <p>A list of CloudTrail event IDs.</p> <p>A populated field indicates that the current configuration was initiated by the events recorded in the CloudTrail log. For more information about CloudTrail, see <a href="http://docs.aws.amazon.com/awscloudtrail/latest/userguide/what_is_cloud_trail_top_level.html">What Is AWS CloudTrail</a>.</p> <p>An empty field indicates that the current configuration was not initiated by any event.</p>
    #[serde(rename = "relatedEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_events: Option<Vec<String>>,
    /// <p>A list of related AWS resources.</p>
    #[serde(rename = "relationships")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationships: Option<Vec<Relationship>>,
    /// <p>The time stamp when the resource was created.</p>
    #[serde(rename = "resourceCreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_creation_time: Option<f64>,
    /// <p>The ID of the resource (for example, <code>sg-xxxxxx</code>).</p>
    #[serde(rename = "resourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>The custom name of the resource, if available.</p>
    #[serde(rename = "resourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    /// <p>The type of AWS resource.</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>Configuration attributes that AWS Config returns for certain resource types to supplement the information returned for the <code>configuration</code> parameter.</p>
    #[serde(rename = "supplementaryConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplementary_configuration: Option<::std::collections::HashMap<String, String>>,
    /// <p>A mapping of key value tags associated with the resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The version number of the resource configuration.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>An object that represents the recording of configuration changes of an AWS resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationRecorder {
    /// <p>The name of the recorder. By default, AWS Config automatically assigns the name "default" when creating the configuration recorder. You cannot change the assigned name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Specifies the types of AWS resources for which AWS Config records configuration changes.</p>
    #[serde(rename = "recordingGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_group: Option<RecordingGroup>,
    /// <p>Amazon Resource Name (ARN) of the IAM role used to describe the AWS resources associated with the account.</p>
    #[serde(rename = "roleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

/// <p>The current status of the configuration recorder.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ConfigurationRecorderStatus {
    /// <p>The error code indicating that the recording failed.</p>
    #[serde(rename = "lastErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_code: Option<String>,
    /// <p>The message indicating that the recording failed due to an error.</p>
    #[serde(rename = "lastErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_message: Option<String>,
    /// <p>The time the recorder was last started.</p>
    #[serde(rename = "lastStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_start_time: Option<f64>,
    /// <p>The last (previous) status of the recorder.</p>
    #[serde(rename = "lastStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status: Option<String>,
    /// <p>The time when the status was last changed.</p>
    #[serde(rename = "lastStatusChangeTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status_change_time: Option<f64>,
    /// <p>The time the recorder was last stopped.</p>
    #[serde(rename = "lastStopTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_stop_time: Option<f64>,
    /// <p>The name of the configuration recorder.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Specifies whether or not the recorder is currently recording.</p>
    #[serde(rename = "recording")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteAggregationAuthorizationRequest {
    /// <p>The 12-digit account ID of the account authorized to aggregate data.</p>
    #[serde(rename = "AuthorizedAccountId")]
    pub authorized_account_id: String,
    /// <p>The region authorized to collect aggregated data.</p>
    #[serde(rename = "AuthorizedAwsRegion")]
    pub authorized_aws_region: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteConfigRuleRequest {
    /// <p>The name of the AWS Config rule that you want to delete.</p>
    #[serde(rename = "ConfigRuleName")]
    pub config_rule_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteConfigurationAggregatorRequest {
    /// <p>The name of the configuration aggregator.</p>
    #[serde(rename = "ConfigurationAggregatorName")]
    pub configuration_aggregator_name: String,
}

/// <p>The request object for the <code>DeleteConfigurationRecorder</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteConfigurationRecorderRequest {
    /// <p>The name of the configuration recorder to be deleted. You can retrieve the name of your configuration recorder by using the <code>DescribeConfigurationRecorders</code> action.</p>
    #[serde(rename = "ConfigurationRecorderName")]
    pub configuration_recorder_name: String,
}

/// <p>The input for the <a>DeleteDeliveryChannel</a> action. The action accepts the following data, in JSON format. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDeliveryChannelRequest {
    /// <p>The name of the delivery channel to delete.</p>
    #[serde(rename = "DeliveryChannelName")]
    pub delivery_channel_name: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteEvaluationResultsRequest {
    /// <p>The name of the AWS Config rule for which you want to delete the evaluation results.</p>
    #[serde(rename = "ConfigRuleName")]
    pub config_rule_name: String,
}

/// <p>The output when you delete the evaluation results for the specified AWS Config rule.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteEvaluationResultsResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeletePendingAggregationRequestRequest {
    /// <p>The 12-digit account ID of the account requesting to aggregate data.</p>
    #[serde(rename = "RequesterAccountId")]
    pub requester_account_id: String,
    /// <p>The region requesting to aggregate data.</p>
    #[serde(rename = "RequesterAwsRegion")]
    pub requester_aws_region: String,
}

/// <p>The input for the <a>DeliverConfigSnapshot</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeliverConfigSnapshotRequest {
    /// <p>The name of the delivery channel through which the snapshot is delivered.</p>
    #[serde(rename = "deliveryChannelName")]
    pub delivery_channel_name: String,
}

/// <p>The output for the <a>DeliverConfigSnapshot</a> action, in JSON format.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeliverConfigSnapshotResponse {
    /// <p>The ID of the snapshot that is being created.</p>
    #[serde(rename = "configSnapshotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_snapshot_id: Option<String>,
}

/// <p>The channel through which AWS Config delivers notifications and updated configuration states.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeliveryChannel {
    /// <p>The options for how often AWS Config delivers configuration snapshots to the Amazon S3 bucket.</p>
    #[serde(rename = "configSnapshotDeliveryProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_snapshot_delivery_properties: Option<ConfigSnapshotDeliveryProperties>,
    /// <p>The name of the delivery channel. By default, AWS Config assigns the name "default" when creating the delivery channel. To change the delivery channel name, you must use the DeleteDeliveryChannel action to delete your current delivery channel, and then you must use the PutDeliveryChannel command to create a delivery channel that has the desired name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The name of the Amazon S3 bucket to which AWS Config delivers configuration snapshots and configuration history files.</p> <p>If you specify a bucket that belongs to another AWS account, that bucket must have policies that grant access permissions to AWS Config. For more information, see <a href="http://docs.aws.amazon.com/config/latest/developerguide/s3-bucket-policy.html">Permissions for the Amazon S3 Bucket</a> in the AWS Config Developer Guide.</p>
    #[serde(rename = "s3BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_bucket_name: Option<String>,
    /// <p>The prefix for the specified Amazon S3 bucket.</p>
    #[serde(rename = "s3KeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_key_prefix: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon SNS topic to which AWS Config sends notifications about configuration changes.</p> <p>If you choose a topic from another account, the topic must have policies that grant access permissions to AWS Config. For more information, see <a href="http://docs.aws.amazon.com/config/latest/developerguide/sns-topic-policy.html">Permissions for the Amazon SNS Topic</a> in the AWS Config Developer Guide.</p>
    #[serde(rename = "snsTopicARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
}

/// <p>The status of a specified delivery channel.</p> <p>Valid values: <code>Success</code> | <code>Failure</code> </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeliveryChannelStatus {
    /// <p>A list that contains the status of the delivery of the configuration history to the specified Amazon S3 bucket.</p>
    #[serde(rename = "configHistoryDeliveryInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_history_delivery_info: Option<ConfigExportDeliveryInfo>,
    /// <p>A list containing the status of the delivery of the snapshot to the specified Amazon S3 bucket.</p>
    #[serde(rename = "configSnapshotDeliveryInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_snapshot_delivery_info: Option<ConfigExportDeliveryInfo>,
    /// <p>A list containing the status of the delivery of the configuration stream notification to the specified Amazon SNS topic.</p>
    #[serde(rename = "configStreamDeliveryInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_stream_delivery_info: Option<ConfigStreamDeliveryInfo>,
    /// <p>The name of the delivery channel.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeAggregateComplianceByConfigRulesRequest {
    /// <p>The name of the configuration aggregator.</p>
    #[serde(rename = "ConfigurationAggregatorName")]
    pub configuration_aggregator_name: String,
    /// <p>Filters the results by ConfigRuleComplianceFilters object. </p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<ConfigRuleComplianceFilters>,
    /// <p>The maximum number of evaluation results returned on each page. The default is maximum. If you specify 0, AWS Config uses the default.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The nextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeAggregateComplianceByConfigRulesResponse {
    /// <p>Returns a list of AggregateComplianceByConfigRule object.</p>
    #[serde(rename = "AggregateComplianceByConfigRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_compliance_by_config_rules: Option<Vec<AggregateComplianceByConfigRule>>,
    /// <p>The nextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeAggregationAuthorizationsRequest {
    /// <p>The maximum number of AggregationAuthorizations returned on each page. The default is maximum. If you specify 0, AWS Config uses the default.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The nextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeAggregationAuthorizationsResponse {
    /// <p>Returns a list of authorizations granted to various aggregator accounts and regions.</p>
    #[serde(rename = "AggregationAuthorizations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_authorizations: Option<Vec<AggregationAuthorization>>,
    /// <p>The nextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeComplianceByConfigRuleRequest {
    /// <p>Filters the results by compliance.</p> <p>The allowed values are <code>COMPLIANT</code>, <code>NON_COMPLIANT</code>, and <code>INSUFFICIENT_DATA</code>.</p>
    #[serde(rename = "ComplianceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_types: Option<Vec<String>>,
    /// <p>Specify one or more AWS Config rule names to filter the results by rule.</p>
    #[serde(rename = "ConfigRuleNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_names: Option<Vec<String>>,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeComplianceByConfigRuleResponse {
    /// <p>Indicates whether each of the specified AWS Config rules is compliant.</p>
    #[serde(rename = "ComplianceByConfigRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_by_config_rules: Option<Vec<ComplianceByConfigRule>>,
    /// <p>The string that you use in a subsequent request to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeComplianceByResourceRequest {
    /// <p>Filters the results by compliance.</p> <p>The allowed values are <code>COMPLIANT</code> and <code>NON_COMPLIANT</code>.</p>
    #[serde(rename = "ComplianceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_types: Option<Vec<String>>,
    /// <p>The maximum number of evaluation results returned on each page. The default is 10. You cannot specify a number greater than 100. If you specify 0, AWS Config uses the default.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the AWS resource for which you want compliance information. You can specify only one resource ID. If you specify a resource ID, you must also specify a type for <code>ResourceType</code>.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>The types of AWS resources for which you want compliance information (for example, <code>AWS::EC2::Instance</code>). For this action, you can specify that the resource type is an AWS account by specifying <code>AWS::::Account</code>.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeComplianceByResourceResponse {
    /// <p>Indicates whether the specified AWS resource complies with all of the AWS Config rules that evaluate it.</p>
    #[serde(rename = "ComplianceByResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_by_resources: Option<Vec<ComplianceByResource>>,
    /// <p>The string that you use in a subsequent request to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeConfigRuleEvaluationStatusRequest {
    /// <p>The name of the AWS managed Config rules for which you want status information. If you do not specify any names, AWS Config returns status information for all AWS managed Config rules that you use.</p>
    #[serde(rename = "ConfigRuleNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_names: Option<Vec<String>>,
    /// <p>The number of rule evaluation results that you want returned.</p> <p>This parameter is required if the rule limit for your account is more than the default of 50 rules.</p> <p>For information about requesting a rule limit increase, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_config">AWS Config Limits</a> in the <i>AWS General Reference Guide</i>.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeConfigRuleEvaluationStatusResponse {
    /// <p>Status information about your AWS managed Config rules.</p>
    #[serde(rename = "ConfigRulesEvaluationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rules_evaluation_status: Option<Vec<ConfigRuleEvaluationStatus>>,
    /// <p>The string that you use in a subsequent request to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeConfigRulesRequest {
    /// <p>The names of the AWS Config rules for which you want details. If you do not specify any names, AWS Config returns details for all your rules.</p>
    #[serde(rename = "ConfigRuleNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_names: Option<Vec<String>>,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeConfigRulesResponse {
    /// <p>The details about your AWS Config rules.</p>
    #[serde(rename = "ConfigRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rules: Option<Vec<ConfigRule>>,
    /// <p>The string that you use in a subsequent request to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeConfigurationAggregatorSourcesStatusRequest {
    /// <p>The name of the configuration aggregator.</p>
    #[serde(rename = "ConfigurationAggregatorName")]
    pub configuration_aggregator_name: String,
    /// <p>The maximum number of AggregatorSourceStatus returned on each page. The default is maximum. If you specify 0, AWS Config uses the default.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The nextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>Filters the status type.</p> <ul> <li> <p>Valid value FAILED indicates errors while moving data.</p> </li> <li> <p>Valid value SUCCEEDED indicates the data was successfully moved.</p> </li> <li> <p>Valid value OUTDATED indicates the data is not the most recent.</p> </li> </ul></p>
    #[serde(rename = "UpdateStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_status: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeConfigurationAggregatorSourcesStatusResponse {
    /// <p>Retuns an AggregatedSourceStatus object. </p>
    #[serde(rename = "AggregatedSourceStatusList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregated_source_status_list: Option<Vec<AggregatedSourceStatus>>,
    /// <p>The nextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeConfigurationAggregatorsRequest {
    /// <p>The name of the configuration aggregators.</p>
    #[serde(rename = "ConfigurationAggregatorNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_aggregator_names: Option<Vec<String>>,
    /// <p>The maximum number of configuration aggregators returned on each page. The default is maximum. If you specify 0, AWS Config uses the default.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The nextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeConfigurationAggregatorsResponse {
    /// <p>Returns a ConfigurationAggregators object.</p>
    #[serde(rename = "ConfigurationAggregators")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_aggregators: Option<Vec<ConfigurationAggregator>>,
    /// <p>The nextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>The input for the <a>DescribeConfigurationRecorderStatus</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeConfigurationRecorderStatusRequest {
    /// <p>The name(s) of the configuration recorder. If the name is not specified, the action returns the current status of all the configuration recorders associated with the account.</p>
    #[serde(rename = "ConfigurationRecorderNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_recorder_names: Option<Vec<String>>,
}

/// <p>The output for the <a>DescribeConfigurationRecorderStatus</a> action, in JSON format.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeConfigurationRecorderStatusResponse {
    /// <p>A list that contains status of the specified recorders.</p>
    #[serde(rename = "ConfigurationRecordersStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_recorders_status: Option<Vec<ConfigurationRecorderStatus>>,
}

/// <p>The input for the <a>DescribeConfigurationRecorders</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeConfigurationRecordersRequest {
    /// <p>A list of configuration recorder names.</p>
    #[serde(rename = "ConfigurationRecorderNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_recorder_names: Option<Vec<String>>,
}

/// <p>The output for the <a>DescribeConfigurationRecorders</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeConfigurationRecordersResponse {
    /// <p>A list that contains the descriptions of the specified configuration recorders.</p>
    #[serde(rename = "ConfigurationRecorders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_recorders: Option<Vec<ConfigurationRecorder>>,
}

/// <p>The input for the <a>DeliveryChannelStatus</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeDeliveryChannelStatusRequest {
    /// <p>A list of delivery channel names.</p>
    #[serde(rename = "DeliveryChannelNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_channel_names: Option<Vec<String>>,
}

/// <p>The output for the <a>DescribeDeliveryChannelStatus</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeDeliveryChannelStatusResponse {
    /// <p>A list that contains the status of a specified delivery channel.</p>
    #[serde(rename = "DeliveryChannelsStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_channels_status: Option<Vec<DeliveryChannelStatus>>,
}

/// <p>The input for the <a>DescribeDeliveryChannels</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeDeliveryChannelsRequest {
    /// <p>A list of delivery channel names.</p>
    #[serde(rename = "DeliveryChannelNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_channel_names: Option<Vec<String>>,
}

/// <p>The output for the <a>DescribeDeliveryChannels</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeDeliveryChannelsResponse {
    /// <p>A list that contains the descriptions of the specified delivery channel.</p>
    #[serde(rename = "DeliveryChannels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_channels: Option<Vec<DeliveryChannel>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribePendingAggregationRequestsRequest {
    /// <p>The maximum number of evaluation results returned on each page. The default is maximum. If you specify 0, AWS Config uses the default.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The nextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribePendingAggregationRequestsResponse {
    /// <p>The nextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Returns a PendingAggregationRequests object.</p>
    #[serde(rename = "PendingAggregationRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_aggregation_requests: Option<Vec<PendingAggregationRequest>>,
}

/// <p>Identifies an AWS resource and indicates whether it complies with the AWS Config rule that it was evaluated against.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Evaluation {
    /// <p>Supplementary information about how the evaluation determined the compliance.</p>
    #[serde(rename = "Annotation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation: Option<String>,
    /// <p>The ID of the AWS resource that was evaluated.</p>
    #[serde(rename = "ComplianceResourceId")]
    pub compliance_resource_id: String,
    /// <p>The type of AWS resource that was evaluated.</p>
    #[serde(rename = "ComplianceResourceType")]
    pub compliance_resource_type: String,
    /// <p>Indicates whether the AWS resource complies with the AWS Config rule that it was evaluated against.</p> <p>For the <code>Evaluation</code> data type, AWS Config supports only the <code>COMPLIANT</code>, <code>NON_COMPLIANT</code>, and <code>NOT_APPLICABLE</code> values. AWS Config does not support the <code>INSUFFICIENT_DATA</code> value for this data type.</p> <p>Similarly, AWS Config does not accept <code>INSUFFICIENT_DATA</code> as the value for <code>ComplianceType</code> from a <code>PutEvaluations</code> request. For example, an AWS Lambda function for a custom AWS Config rule cannot pass an <code>INSUFFICIENT_DATA</code> value to AWS Config.</p>
    #[serde(rename = "ComplianceType")]
    pub compliance_type: String,
    /// <p>The time of the event in AWS Config that triggered the evaluation. For event-based evaluations, the time indicates when AWS Config created the configuration item that triggered the evaluation. For periodic evaluations, the time indicates when AWS Config triggered the evaluation at the frequency that you specified (for example, every 24 hours).</p>
    #[serde(rename = "OrderingTimestamp")]
    pub ordering_timestamp: f64,
}

/// <p>The details of an AWS Config evaluation. Provides the AWS resource that was evaluated, the compliance of the resource, related time stamps, and supplementary information.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct EvaluationResult {
    /// <p>Supplementary information about how the evaluation determined the compliance.</p>
    #[serde(rename = "Annotation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation: Option<String>,
    /// <p>Indicates whether the AWS resource complies with the AWS Config rule that evaluated it.</p> <p>For the <code>EvaluationResult</code> data type, AWS Config supports only the <code>COMPLIANT</code>, <code>NON_COMPLIANT</code>, and <code>NOT_APPLICABLE</code> values. AWS Config does not support the <code>INSUFFICIENT_DATA</code> value for the <code>EvaluationResult</code> data type.</p>
    #[serde(rename = "ComplianceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_type: Option<String>,
    /// <p>The time when the AWS Config rule evaluated the AWS resource.</p>
    #[serde(rename = "ConfigRuleInvokedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_invoked_time: Option<f64>,
    /// <p>Uniquely identifies the evaluation result.</p>
    #[serde(rename = "EvaluationResultIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_result_identifier: Option<EvaluationResultIdentifier>,
    /// <p>The time when AWS Config recorded the evaluation result.</p>
    #[serde(rename = "ResultRecordedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_recorded_time: Option<f64>,
    /// <p>An encrypted token that associates an evaluation with an AWS Config rule. The token identifies the rule, the AWS resource being evaluated, and the event that triggered the evaluation.</p>
    #[serde(rename = "ResultToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_token: Option<String>,
}

/// <p>Uniquely identifies an evaluation result.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct EvaluationResultIdentifier {
    /// <p>Identifies an AWS Config rule used to evaluate an AWS resource, and provides the type and ID of the evaluated resource.</p>
    #[serde(rename = "EvaluationResultQualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_result_qualifier: Option<EvaluationResultQualifier>,
    /// <p>The time of the event that triggered the evaluation of your AWS resources. The time can indicate when AWS Config delivered a configuration item change notification, or it can indicate when AWS Config delivered the configuration snapshot, depending on which event triggered the evaluation.</p>
    #[serde(rename = "OrderingTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordering_timestamp: Option<f64>,
}

/// <p>Identifies an AWS Config rule that evaluated an AWS resource, and provides the type and ID of the resource that the rule evaluated.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct EvaluationResultQualifier {
    /// <p>The name of the AWS Config rule that was used in the evaluation.</p>
    #[serde(rename = "ConfigRuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_name: Option<String>,
    /// <p>The ID of the evaluated AWS resource.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>The type of AWS resource that was evaluated.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAggregateComplianceDetailsByConfigRuleRequest {
    /// <p>The 12-digit account ID of the source account.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The source region from where the data is aggregated.</p>
    #[serde(rename = "AwsRegion")]
    pub aws_region: String,
    /// <p><p>The resource compliance status.</p> <note> <p>For the <code>GetAggregateComplianceDetailsByConfigRuleRequest</code> data type, AWS Config supports only the <code>COMPLIANT</code> and <code>NON<em>COMPLIANT</code>. AWS Config does not support the <code>NOT</em>APPLICABLE</code> and <code>INSUFFICIENT_DATA</code> values.</p> </note></p>
    #[serde(rename = "ComplianceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_type: Option<String>,
    /// <p>The name of the AWS Config rule for which you want compliance information.</p>
    #[serde(rename = "ConfigRuleName")]
    pub config_rule_name: String,
    /// <p>The name of the configuration aggregator.</p>
    #[serde(rename = "ConfigurationAggregatorName")]
    pub configuration_aggregator_name: String,
    /// <p>The maximum number of evaluation results returned on each page. The default is 50. You cannot specify a number greater than 100. If you specify 0, AWS Config uses the default.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The nextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetAggregateComplianceDetailsByConfigRuleResponse {
    /// <p>Returns an AggregateEvaluationResults object.</p>
    #[serde(rename = "AggregateEvaluationResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_evaluation_results: Option<Vec<AggregateEvaluationResult>>,
    /// <p>The nextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAggregateConfigRuleComplianceSummaryRequest {
    /// <p>The name of the configuration aggregator.</p>
    #[serde(rename = "ConfigurationAggregatorName")]
    pub configuration_aggregator_name: String,
    /// <p>Filters the results based on the ConfigRuleComplianceSummaryFilters object.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<ConfigRuleComplianceSummaryFilters>,
    /// <p>Groups the result based on ACCOUNT_ID or AWS_REGION.</p>
    #[serde(rename = "GroupByKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by_key: Option<String>,
    /// <p>The maximum number of evaluation results returned on each page. The default is 1000. You cannot specify a number greater than 1000. If you specify 0, AWS Config uses the default.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The nextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetAggregateConfigRuleComplianceSummaryResponse {
    /// <p>Returns a list of AggregateComplianceCounts object.</p>
    #[serde(rename = "AggregateComplianceCounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_compliance_counts: Option<Vec<AggregateComplianceCount>>,
    /// <p>Groups the result based on ACCOUNT_ID or AWS_REGION.</p>
    #[serde(rename = "GroupByKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by_key: Option<String>,
    /// <p>The nextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetComplianceDetailsByConfigRuleRequest {
    /// <p>Filters the results by compliance.</p> <p>The allowed values are <code>COMPLIANT</code>, <code>NON_COMPLIANT</code>, and <code>NOT_APPLICABLE</code>.</p>
    #[serde(rename = "ComplianceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_types: Option<Vec<String>>,
    /// <p>The name of the AWS Config rule for which you want compliance information.</p>
    #[serde(rename = "ConfigRuleName")]
    pub config_rule_name: String,
    /// <p>The maximum number of evaluation results returned on each page. The default is 10. You cannot specify a number greater than 100. If you specify 0, AWS Config uses the default.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetComplianceDetailsByConfigRuleResponse {
    /// <p>Indicates whether the AWS resource complies with the specified AWS Config rule.</p>
    #[serde(rename = "EvaluationResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_results: Option<Vec<EvaluationResult>>,
    /// <p>The string that you use in a subsequent request to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetComplianceDetailsByResourceRequest {
    /// <p>Filters the results by compliance.</p> <p>The allowed values are <code>COMPLIANT</code>, <code>NON_COMPLIANT</code>, and <code>NOT_APPLICABLE</code>.</p>
    #[serde(rename = "ComplianceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_types: Option<Vec<String>>,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the AWS resource for which you want compliance information.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>The type of the AWS resource for which you want compliance information.</p>
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetComplianceDetailsByResourceResponse {
    /// <p>Indicates whether the specified AWS resource complies each AWS Config rule.</p>
    #[serde(rename = "EvaluationResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_results: Option<Vec<EvaluationResult>>,
    /// <p>The string that you use in a subsequent request to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetComplianceSummaryByConfigRuleResponse {
    /// <p>The number of AWS Config rules that are compliant and the number that are noncompliant, up to a maximum of 25 for each.</p>
    #[serde(rename = "ComplianceSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_summary: Option<ComplianceSummary>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetComplianceSummaryByResourceTypeRequest {
    /// <p>Specify one or more resource types to get the number of resources that are compliant and the number that are noncompliant for each resource type.</p> <p>For this request, you can specify an AWS resource type such as <code>AWS::EC2::Instance</code>. You can specify that the resource type is an AWS account by specifying <code>AWS::::Account</code>.</p>
    #[serde(rename = "ResourceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types: Option<Vec<String>>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetComplianceSummaryByResourceTypeResponse {
    /// <p>The number of resources that are compliant and the number that are noncompliant. If one or more resource types were provided with the request, the numbers are returned for each resource type. The maximum number returned is 100.</p>
    #[serde(rename = "ComplianceSummariesByResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_summaries_by_resource_type: Option<Vec<ComplianceSummaryByResourceType>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDiscoveredResourceCountsRequest {
    /// <p>The maximum number of <a>ResourceCount</a> objects returned on each page. The default is 100. You cannot specify a number greater than 100. If you specify 0, AWS Config uses the default.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>The comma-separated list that specifies the resource types that you want AWS Config to return (for example, <code>&quot;AWS::EC2::Instance&quot;</code>, <code>&quot;AWS::IAM::User&quot;</code>).</p> <p>If a value for <code>resourceTypes</code> is not specified, AWS Config returns all resource types that AWS Config is recording in the region for your account.</p> <note> <p>If the configuration recorder is turned off, AWS Config returns an empty list of <a>ResourceCount</a> objects. If the configuration recorder is not recording a specific resource type (for example, S3 buckets), that resource type is not returned in the list of <a>ResourceCount</a> objects.</p> </note></p>
    #[serde(rename = "resourceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetDiscoveredResourceCountsResponse {
    /// <p>The string that you use in a subsequent request to get the next page of results in a paginated response.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of <code>ResourceCount</code> objects. Each object is listed in descending order by the number of resources.</p>
    #[serde(rename = "resourceCounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_counts: Option<Vec<ResourceCount>>,
    /// <p><p>The total number of resources that AWS Config is recording in the region for your account. If you specify resource types in the request, AWS Config returns only the total number of resources for those resource types.</p> <p class="title"> <b>Example</b> </p> <ol> <li> <p>AWS Config is recording three resource types in the US East (Ohio) Region for your account: 25 EC2 instances, 20 IAM users, and 15 S3 buckets, for a total of 60 resources.</p> </li> <li> <p>You make a call to the <code>GetDiscoveredResourceCounts</code> action and specify the resource type, <code>&quot;AWS::EC2::Instances&quot;</code>, in the request.</p> </li> <li> <p>AWS Config returns 25 for <code>totalDiscoveredResources</code>.</p> </li> </ol></p>
    #[serde(rename = "totalDiscoveredResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_discovered_resources: Option<i64>,
}

/// <p>The input for the <a>GetResourceConfigHistory</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetResourceConfigHistoryRequest {
    /// <p>The chronological order for configuration items listed. By default, the results are listed in reverse chronological order.</p>
    #[serde(rename = "chronologicalOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chronological_order: Option<String>,
    /// <p>The time stamp that indicates an earlier time. If not specified, the action returns paginated results that contain configuration items that start when the first configuration item was recorded.</p>
    #[serde(rename = "earlierTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earlier_time: Option<f64>,
    /// <p>The time stamp that indicates a later time. If not specified, current time is taken.</p>
    #[serde(rename = "laterTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub later_time: Option<f64>,
    /// <p>The maximum number of configuration items returned on each page. The default is 10. You cannot specify a number greater than 100. If you specify 0, AWS Config uses the default.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the resource (for example., <code>sg-xxxxxx</code>).</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>The resource type.</p>
    #[serde(rename = "resourceType")]
    pub resource_type: String,
}

/// <p>The output for the <a>GetResourceConfigHistory</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetResourceConfigHistoryResponse {
    /// <p>A list that contains the configuration history of one or more resources.</p>
    #[serde(rename = "configurationItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_items: Option<Vec<ConfigurationItem>>,
    /// <p>The string that you use in a subsequent request to get the next page of results in a paginated response.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListDiscoveredResourcesRequest {
    /// <p>Specifies whether AWS Config includes deleted resources in the results. By default, deleted resources are not included.</p>
    #[serde(rename = "includeDeletedResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_deleted_resources: Option<bool>,
    /// <p>The maximum number of resource identifiers returned on each page. The default is 100. You cannot specify a number greater than 100. If you specify 0, AWS Config uses the default.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The IDs of only those resources that you want AWS Config to list in the response. If you do not specify this parameter, AWS Config lists all resources of the specified type that it has discovered.</p>
    #[serde(rename = "resourceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_ids: Option<Vec<String>>,
    /// <p>The custom name of only those resources that you want AWS Config to list in the response. If you do not specify this parameter, AWS Config lists all resources of the specified type that it has discovered.</p>
    #[serde(rename = "resourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    /// <p>The type of resources that you want AWS Config to list in the response.</p>
    #[serde(rename = "resourceType")]
    pub resource_type: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListDiscoveredResourcesResponse {
    /// <p>The string that you use in a subsequent request to get the next page of results in a paginated response.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The details that identify a resource that is discovered by AWS Config, including the resource type, ID, and (if available) the custom resource name.</p>
    #[serde(rename = "resourceIdentifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifiers: Option<Vec<ResourceIdentifier>>,
}

/// <p>This object contains regions to setup the aggregator and an IAM role to retrieve organization details.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrganizationAggregationSource {
    /// <p>If true, aggreagate existing AWS Config regions and future regions.</p>
    #[serde(rename = "AllAwsRegions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_aws_regions: Option<bool>,
    /// <p>The source regions being aggregated.</p>
    #[serde(rename = "AwsRegions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_regions: Option<Vec<String>>,
    /// <p>ARN of the IAM role used to retreive AWS Organization details associated with the aggregator account.</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
}

/// <p>An object that represents the account ID and region of an aggregator account that is requesting authorization but is not yet authorized.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PendingAggregationRequest {
    /// <p>The 12-digit account ID of the account requesting to aggregate data.</p>
    #[serde(rename = "RequesterAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester_account_id: Option<String>,
    /// <p>The region requesting to aggregate data. </p>
    #[serde(rename = "RequesterAwsRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester_aws_region: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutAggregationAuthorizationRequest {
    /// <p>The 12-digit account ID of the account authorized to aggregate data.</p>
    #[serde(rename = "AuthorizedAccountId")]
    pub authorized_account_id: String,
    /// <p>The region authorized to collect aggregated data.</p>
    #[serde(rename = "AuthorizedAwsRegion")]
    pub authorized_aws_region: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PutAggregationAuthorizationResponse {
    /// <p>Returns an AggregationAuthorization object. </p>
    #[serde(rename = "AggregationAuthorization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_authorization: Option<AggregationAuthorization>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutConfigRuleRequest {
    /// <p>The rule that you want to add to your account.</p>
    #[serde(rename = "ConfigRule")]
    pub config_rule: ConfigRule,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutConfigurationAggregatorRequest {
    /// <p>A list of AccountAggregationSource object. </p>
    #[serde(rename = "AccountAggregationSources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_aggregation_sources: Option<Vec<AccountAggregationSource>>,
    /// <p>The name of the configuration aggregator.</p>
    #[serde(rename = "ConfigurationAggregatorName")]
    pub configuration_aggregator_name: String,
    /// <p>An OrganizationAggregationSource object.</p>
    #[serde(rename = "OrganizationAggregationSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_aggregation_source: Option<OrganizationAggregationSource>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PutConfigurationAggregatorResponse {
    /// <p>Returns a ConfigurationAggregator object.</p>
    #[serde(rename = "ConfigurationAggregator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_aggregator: Option<ConfigurationAggregator>,
}

/// <p>The input for the <a>PutConfigurationRecorder</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutConfigurationRecorderRequest {
    /// <p>The configuration recorder object that records each configuration change made to the resources.</p>
    #[serde(rename = "ConfigurationRecorder")]
    pub configuration_recorder: ConfigurationRecorder,
}

/// <p>The input for the <a>PutDeliveryChannel</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutDeliveryChannelRequest {
    /// <p>The configuration delivery channel object that delivers the configuration information to an Amazon S3 bucket and to an Amazon SNS topic.</p>
    #[serde(rename = "DeliveryChannel")]
    pub delivery_channel: DeliveryChannel,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutEvaluationsRequest {
    /// <p>The assessments that the AWS Lambda function performs. Each evaluation identifies an AWS resource and indicates whether it complies with the AWS Config rule that invokes the AWS Lambda function.</p>
    #[serde(rename = "Evaluations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluations: Option<Vec<Evaluation>>,
    /// <p>An encrypted token that associates an evaluation with an AWS Config rule. Identifies the rule and the event that triggered the evaluation.</p>
    #[serde(rename = "ResultToken")]
    pub result_token: String,
    /// <p><p>Use this parameter to specify a test run for <code>PutEvaluations</code>. You can verify whether your AWS Lambda function will deliver evaluation results to AWS Config. No updates occur to your existing evaluations, and evaluation results are not sent to AWS Config.</p> <note> <p>When <code>TestMode</code> is <code>true</code>, <code>PutEvaluations</code> doesn&#39;t require a valid value for the <code>ResultToken</code> parameter, but the value cannot be null.</p> </note></p>
    #[serde(rename = "TestMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_mode: Option<bool>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PutEvaluationsResponse {
    /// <p>Requests that failed because of a client or server error.</p>
    #[serde(rename = "FailedEvaluations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_evaluations: Option<Vec<Evaluation>>,
}

/// <p>Specifies the types of AWS resource for which AWS Config records configuration changes.</p> <p>In the recording group, you specify whether all supported types or specific types of resources are recorded.</p> <p>By default, AWS Config records configuration changes for all supported types of regional resources that AWS Config discovers in the region in which it is running. Regional resources are tied to a region and can be used only in that region. Examples of regional resources are EC2 instances and EBS volumes.</p> <p>You can also have AWS Config record configuration changes for supported types of global resources (for example, IAM resources). Global resources are not tied to an individual region and can be used in all regions.</p> <important> <p>The configuration details for any global resource are the same in all regions. If you customize AWS Config in multiple regions to record global resources, it will create multiple configuration items each time a global resource changes: one configuration item for each region. These configuration items will contain identical data. To prevent duplicate configuration items, you should consider customizing AWS Config in only one region to record global resources, unless you want the configuration items to be available in multiple regions.</p> </important> <p>If you don't want AWS Config to record all resources, you can specify which types of resources it will record with the <code>resourceTypes</code> parameter.</p> <p>For a list of supported resource types, see <a href="http://docs.aws.amazon.com/config/latest/developerguide/resource-config-reference.html#supported-resources">Supported Resource Types</a>.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/config/latest/developerguide/select-resources.html">Selecting Which Resources AWS Config Records</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecordingGroup {
    /// <p>Specifies whether AWS Config records configuration changes for every supported type of regional resource.</p> <p>If you set this option to <code>true</code>, when AWS Config adds support for a new type of regional resource, it starts recording resources of that type automatically.</p> <p>If you set this option to <code>true</code>, you cannot enumerate a list of <code>resourceTypes</code>.</p>
    #[serde(rename = "allSupported")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_supported: Option<bool>,
    /// <p>Specifies whether AWS Config includes all supported types of global resources (for example, IAM resources) with the resources that it records.</p> <p>Before you can set this option to <code>true</code>, you must set the <code>allSupported</code> option to <code>true</code>.</p> <p>If you set this option to <code>true</code>, when AWS Config adds support for a new type of global resource, it starts recording resources of that type automatically.</p> <p>The configuration details for any global resource are the same in all regions. To prevent duplicate configuration items, you should consider customizing AWS Config in only one region to record global resources.</p>
    #[serde(rename = "includeGlobalResourceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_global_resource_types: Option<bool>,
    /// <p>A comma-separated list that specifies the types of AWS resources for which AWS Config records configuration changes (for example, <code>AWS::EC2::Instance</code> or <code>AWS::CloudTrail::Trail</code>).</p> <p>Before you can set this option to <code>true</code>, you must set the <code>allSupported</code> option to <code>false</code>.</p> <p>If you set this option to <code>true</code>, when AWS Config adds support for a new type of resource, it will not record resources of that type unless you manually add that type to your recording group.</p> <p>For a list of valid <code>resourceTypes</code> values, see the <b>resourceType Value</b> column in <a href="http://docs.aws.amazon.com/config/latest/developerguide/resource-config-reference.html#supported-resources">Supported AWS Resource Types</a>.</p>
    #[serde(rename = "resourceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types: Option<Vec<String>>,
}

/// <p>The relationship of the related resource to the main resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Relationship {
    /// <p>The type of relationship with the related resource.</p>
    #[serde(rename = "relationshipName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_name: Option<String>,
    /// <p>The ID of the related resource (for example, <code>sg-xxxxxx</code>).</p>
    #[serde(rename = "resourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>The custom name of the related resource, if available.</p>
    #[serde(rename = "resourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    /// <p>The resource type of the related resource.</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

/// <p>An object that contains the resource type and the number of resources.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ResourceCount {
    /// <p>The number of resources.</p>
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// <p>The resource type (for example, <code>"AWS::EC2::Instance"</code>).</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

/// <p>The details that identify a resource that is discovered by AWS Config, including the resource type, ID, and (if available) the custom resource name.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ResourceIdentifier {
    /// <p>The time that the resource was deleted.</p>
    #[serde(rename = "resourceDeletionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_deletion_time: Option<f64>,
    /// <p>The ID of the resource (for example, <code>sg-xxxxxx</code>).</p>
    #[serde(rename = "resourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>The custom name of the resource (if available).</p>
    #[serde(rename = "resourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    /// <p>The type of resource.</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

/// <p>The details that identify a resource within AWS Config, including the resource type and resource ID.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceKey {
    /// <p>The ID of the resource (for example., sg-xxxxxx). </p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>The resource type.</p>
    #[serde(rename = "resourceType")]
    pub resource_type: String,
}

/// <p>Defines which resources trigger an evaluation for an AWS Config rule. The scope can include one or more resource types, a combination of a tag key and value, or a combination of one resource type and one resource ID. Specify a scope to constrain which resources trigger an evaluation for a rule. Otherwise, evaluations for the rule are triggered when any resource in your recording group changes in configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scope {
    /// <p>The ID of the only AWS resource that you want to trigger an evaluation for the rule. If you specify a resource ID, you must specify one resource type for <code>ComplianceResourceTypes</code>.</p>
    #[serde(rename = "ComplianceResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_resource_id: Option<String>,
    /// <p>The resource types of only those AWS resources that you want to trigger an evaluation for the rule. You can only specify one type if you also specify a resource ID for <code>ComplianceResourceId</code>.</p>
    #[serde(rename = "ComplianceResourceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_resource_types: Option<Vec<String>>,
    /// <p>The tag key that is applied to only those AWS resources that you want to trigger an evaluation for the rule.</p>
    #[serde(rename = "TagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    /// <p>The tag value applied to only those AWS resources that you want to trigger an evaluation for the rule. If you specify a value for <code>TagValue</code>, you must also specify a value for <code>TagKey</code>.</p>
    #[serde(rename = "TagValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
}

/// <p>Provides the AWS Config rule owner (AWS or customer), the rule identifier, and the events that trigger the evaluation of your AWS resources.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Source {
    /// <p>Indicates whether AWS or the customer owns and manages the AWS Config rule.</p>
    #[serde(rename = "Owner")]
    pub owner: String,
    /// <p>Provides the source and type of the event that causes AWS Config to evaluate your AWS resources.</p>
    #[serde(rename = "SourceDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_details: Option<Vec<SourceDetail>>,
    /// <p>For AWS Config managed rules, a predefined identifier from a list. For example, <code>IAM_PASSWORD_POLICY</code> is a managed rule. To reference a managed rule, see <a href="http://docs.aws.amazon.com/config/latest/developerguide/evaluate-config_use-managed-rules.html">Using AWS Managed Config Rules</a>.</p> <p>For custom rules, the identifier is the Amazon Resource Name (ARN) of the rule's AWS Lambda function, such as <code>arn:aws:lambda:us-east-2:123456789012:function:custom_rule_name</code>.</p>
    #[serde(rename = "SourceIdentifier")]
    pub source_identifier: String,
}

/// <p>Provides the source and the message types that trigger AWS Config to evaluate your AWS resources against a rule. It also provides the frequency with which you want AWS Config to run evaluations for the rule if the trigger type is periodic. You can specify the parameter values for <code>SourceDetail</code> only for custom rules. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceDetail {
    /// <p>The source of the event, such as an AWS service, that triggers AWS Config to evaluate your AWS resources.</p>
    #[serde(rename = "EventSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source: Option<String>,
    /// <p><p>The frequency at which you want AWS Config to run evaluations for a custom rule with a periodic trigger. If you specify a value for <code>MaximumExecutionFrequency</code>, then <code>MessageType</code> must use the <code>ScheduledNotification</code> value.</p> <note> <p>By default, rules with a periodic trigger are evaluated every 24 hours. To change the frequency, specify a valid value for the <code>MaximumExecutionFrequency</code> parameter.</p> <p>Based on the valid value you choose, AWS Config runs evaluations once for each valid value. For example, if you choose <code>Three<em>Hours</code>, AWS Config runs evaluations once every three hours. In this case, <code>Three</em>Hours</code> is the frequency of this rule. </p> </note></p>
    #[serde(rename = "MaximumExecutionFrequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_execution_frequency: Option<String>,
    /// <p>The type of notification that triggers AWS Config to run an evaluation for a rule. You can specify the following notification types:</p> <ul> <li> <p> <code>ConfigurationItemChangeNotification</code> - Triggers an evaluation when AWS Config delivers a configuration item as a result of a resource change.</p> </li> <li> <p> <code>OversizedConfigurationItemChangeNotification</code> - Triggers an evaluation when AWS Config delivers an oversized configuration item. AWS Config may generate this notification type when a resource changes and the notification exceeds the maximum size allowed by Amazon SNS.</p> </li> <li> <p> <code>ScheduledNotification</code> - Triggers a periodic evaluation at the frequency specified for <code>MaximumExecutionFrequency</code>.</p> </li> <li> <p> <code>ConfigurationSnapshotDeliveryCompleted</code> - Triggers a periodic evaluation when AWS Config delivers a configuration snapshot.</p> </li> </ul> <p>If you want your custom rule to be triggered by configuration changes, specify both <code>ConfigurationItemChangeNotification</code> and <code>OversizedConfigurationItemChangeNotification</code>. </p>
    #[serde(rename = "MessageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartConfigRulesEvaluationRequest {
    /// <p>The list of names of AWS Config rules that you want to run evaluations for.</p>
    #[serde(rename = "ConfigRuleNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_names: Option<Vec<String>>,
}

/// <p>The output when you start the evaluation for the specified AWS Config rule.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StartConfigRulesEvaluationResponse {}

/// <p>The input for the <a>StartConfigurationRecorder</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartConfigurationRecorderRequest {
    /// <p>The name of the recorder object that records each configuration change made to the resources.</p>
    #[serde(rename = "ConfigurationRecorderName")]
    pub configuration_recorder_name: String,
}

/// <p>The input for the <a>StopConfigurationRecorder</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopConfigurationRecorderRequest {
    /// <p>The name of the recorder object that records each configuration change made to the resources.</p>
    #[serde(rename = "ConfigurationRecorderName")]
    pub configuration_recorder_name: String,
}

/// Errors returned by BatchGetResourceConfig
#[derive(Debug, PartialEq)]
pub enum BatchGetResourceConfigError {
    /// <p>There are no configuration recorders available to provide the role needed to describe your resources. Create a configuration recorder.</p>
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

impl BatchGetResourceConfigError {
    pub fn from_body(body: &str) -> BatchGetResourceConfigError {
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
                    "NoAvailableConfigurationRecorderException" => {
                        BatchGetResourceConfigError::NoAvailableConfigurationRecorder(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        BatchGetResourceConfigError::Validation(error_message.to_string())
                    }
                    _ => BatchGetResourceConfigError::Unknown(String::from(body)),
                }
            }
            Err(_) => BatchGetResourceConfigError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for BatchGetResourceConfigError {
    fn from(err: serde_json::error::Error) -> BatchGetResourceConfigError {
        BatchGetResourceConfigError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for BatchGetResourceConfigError {
    fn from(err: CredentialsError) -> BatchGetResourceConfigError {
        BatchGetResourceConfigError::Credentials(err)
    }
}
impl From<HttpDispatchError> for BatchGetResourceConfigError {
    fn from(err: HttpDispatchError) -> BatchGetResourceConfigError {
        BatchGetResourceConfigError::HttpDispatch(err)
    }
}
impl From<io::Error> for BatchGetResourceConfigError {
    fn from(err: io::Error) -> BatchGetResourceConfigError {
        BatchGetResourceConfigError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for BatchGetResourceConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchGetResourceConfigError {
    fn description(&self) -> &str {
        match *self {
            BatchGetResourceConfigError::NoAvailableConfigurationRecorder(ref cause) => cause,
            BatchGetResourceConfigError::Validation(ref cause) => cause,
            BatchGetResourceConfigError::Credentials(ref err) => err.description(),
            BatchGetResourceConfigError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            BatchGetResourceConfigError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteAggregationAuthorization
#[derive(Debug, PartialEq)]
pub enum DeleteAggregationAuthorizationError {
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
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

impl DeleteAggregationAuthorizationError {
    pub fn from_body(body: &str) -> DeleteAggregationAuthorizationError {
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
                    "InvalidParameterValueException" => {
                        DeleteAggregationAuthorizationError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DeleteAggregationAuthorizationError::Validation(error_message.to_string())
                    }
                    _ => DeleteAggregationAuthorizationError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteAggregationAuthorizationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteAggregationAuthorizationError {
    fn from(err: serde_json::error::Error) -> DeleteAggregationAuthorizationError {
        DeleteAggregationAuthorizationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteAggregationAuthorizationError {
    fn from(err: CredentialsError) -> DeleteAggregationAuthorizationError {
        DeleteAggregationAuthorizationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteAggregationAuthorizationError {
    fn from(err: HttpDispatchError) -> DeleteAggregationAuthorizationError {
        DeleteAggregationAuthorizationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteAggregationAuthorizationError {
    fn from(err: io::Error) -> DeleteAggregationAuthorizationError {
        DeleteAggregationAuthorizationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteAggregationAuthorizationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteAggregationAuthorizationError {
    fn description(&self) -> &str {
        match *self {
            DeleteAggregationAuthorizationError::InvalidParameterValue(ref cause) => cause,
            DeleteAggregationAuthorizationError::Validation(ref cause) => cause,
            DeleteAggregationAuthorizationError::Credentials(ref err) => err.description(),
            DeleteAggregationAuthorizationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteAggregationAuthorizationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteConfigRule
#[derive(Debug, PartialEq)]
pub enum DeleteConfigRuleError {
    /// <p>One or more AWS Config rules in the request are invalid. Verify that the rule names are correct and try again.</p>
    NoSuchConfigRule(String),
    /// <p>The rule is currently being deleted or the rule is deleting your evaluation results. Try your request again later.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for DeleteConfigRuleError {
    fn from(err: io::Error) -> DeleteConfigRuleError {
        DeleteConfigRuleError::HttpDispatch(HttpDispatchError::from(err))
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
/// Errors returned by DeleteConfigurationAggregator
#[derive(Debug, PartialEq)]
pub enum DeleteConfigurationAggregatorError {
    /// <p>You have specified a configuration aggregator that does not exist.</p>
    NoSuchConfigurationAggregator(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteConfigurationAggregatorError {
    pub fn from_body(body: &str) -> DeleteConfigurationAggregatorError {
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
                    "NoSuchConfigurationAggregatorException" => {
                        DeleteConfigurationAggregatorError::NoSuchConfigurationAggregator(
                            String::from(error_message),
                        )
                    }
                    "ValidationException" => {
                        DeleteConfigurationAggregatorError::Validation(error_message.to_string())
                    }
                    _ => DeleteConfigurationAggregatorError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteConfigurationAggregatorError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteConfigurationAggregatorError {
    fn from(err: serde_json::error::Error) -> DeleteConfigurationAggregatorError {
        DeleteConfigurationAggregatorError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteConfigurationAggregatorError {
    fn from(err: CredentialsError) -> DeleteConfigurationAggregatorError {
        DeleteConfigurationAggregatorError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteConfigurationAggregatorError {
    fn from(err: HttpDispatchError) -> DeleteConfigurationAggregatorError {
        DeleteConfigurationAggregatorError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteConfigurationAggregatorError {
    fn from(err: io::Error) -> DeleteConfigurationAggregatorError {
        DeleteConfigurationAggregatorError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteConfigurationAggregatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteConfigurationAggregatorError {
    fn description(&self) -> &str {
        match *self {
            DeleteConfigurationAggregatorError::NoSuchConfigurationAggregator(ref cause) => cause,
            DeleteConfigurationAggregatorError::Validation(ref cause) => cause,
            DeleteConfigurationAggregatorError::Credentials(ref err) => err.description(),
            DeleteConfigurationAggregatorError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteConfigurationAggregatorError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteConfigurationRecorder
#[derive(Debug, PartialEq)]
pub enum DeleteConfigurationRecorderError {
    /// <p>You have specified a configuration recorder that does not exist.</p>
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
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "NoSuchConfigurationRecorderException" => {
                        DeleteConfigurationRecorderError::NoSuchConfigurationRecorder(String::from(
                            error_message,
                        ))
                    }
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
impl From<io::Error> for DeleteConfigurationRecorderError {
    fn from(err: io::Error) -> DeleteConfigurationRecorderError {
        DeleteConfigurationRecorderError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>You cannot delete the delivery channel you specified because the configuration recorder is running.</p>
    LastDeliveryChannelDeleteFailed(String),
    /// <p>You have specified a delivery channel that does not exist.</p>
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
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "LastDeliveryChannelDeleteFailedException" => {
                        DeleteDeliveryChannelError::LastDeliveryChannelDeleteFailed(String::from(
                            error_message,
                        ))
                    }
                    "NoSuchDeliveryChannelException" => {
                        DeleteDeliveryChannelError::NoSuchDeliveryChannel(String::from(
                            error_message,
                        ))
                    }
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
impl From<io::Error> for DeleteDeliveryChannelError {
    fn from(err: io::Error) -> DeleteDeliveryChannelError {
        DeleteDeliveryChannelError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>One or more AWS Config rules in the request are invalid. Verify that the rule names are correct and try again.</p>
    NoSuchConfigRule(String),
    /// <p>The rule is currently being deleted or the rule is deleting your evaluation results. Try your request again later.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for DeleteEvaluationResultsError {
    fn from(err: io::Error) -> DeleteEvaluationResultsError {
        DeleteEvaluationResultsError::HttpDispatch(HttpDispatchError::from(err))
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
/// Errors returned by DeletePendingAggregationRequest
#[derive(Debug, PartialEq)]
pub enum DeletePendingAggregationRequestError {
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
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

impl DeletePendingAggregationRequestError {
    pub fn from_body(body: &str) -> DeletePendingAggregationRequestError {
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
                    "InvalidParameterValueException" => {
                        DeletePendingAggregationRequestError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DeletePendingAggregationRequestError::Validation(error_message.to_string())
                    }
                    _ => DeletePendingAggregationRequestError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeletePendingAggregationRequestError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeletePendingAggregationRequestError {
    fn from(err: serde_json::error::Error) -> DeletePendingAggregationRequestError {
        DeletePendingAggregationRequestError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeletePendingAggregationRequestError {
    fn from(err: CredentialsError) -> DeletePendingAggregationRequestError {
        DeletePendingAggregationRequestError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeletePendingAggregationRequestError {
    fn from(err: HttpDispatchError) -> DeletePendingAggregationRequestError {
        DeletePendingAggregationRequestError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeletePendingAggregationRequestError {
    fn from(err: io::Error) -> DeletePendingAggregationRequestError {
        DeletePendingAggregationRequestError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeletePendingAggregationRequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeletePendingAggregationRequestError {
    fn description(&self) -> &str {
        match *self {
            DeletePendingAggregationRequestError::InvalidParameterValue(ref cause) => cause,
            DeletePendingAggregationRequestError::Validation(ref cause) => cause,
            DeletePendingAggregationRequestError::Credentials(ref err) => err.description(),
            DeletePendingAggregationRequestError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeletePendingAggregationRequestError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeliverConfigSnapshot
#[derive(Debug, PartialEq)]
pub enum DeliverConfigSnapshotError {
    /// <p>There are no configuration recorders available to provide the role needed to describe your resources. Create a configuration recorder.</p>
    NoAvailableConfigurationRecorder(String),
    /// <p>There is no configuration recorder running.</p>
    NoRunningConfigurationRecorder(String),
    /// <p>You have specified a delivery channel that does not exist.</p>
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
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "NoAvailableConfigurationRecorderException" => {
                        DeliverConfigSnapshotError::NoAvailableConfigurationRecorder(String::from(
                            error_message,
                        ))
                    }
                    "NoRunningConfigurationRecorderException" => {
                        DeliverConfigSnapshotError::NoRunningConfigurationRecorder(String::from(
                            error_message,
                        ))
                    }
                    "NoSuchDeliveryChannelException" => {
                        DeliverConfigSnapshotError::NoSuchDeliveryChannel(String::from(
                            error_message,
                        ))
                    }
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
impl From<io::Error> for DeliverConfigSnapshotError {
    fn from(err: io::Error) -> DeliverConfigSnapshotError {
        DeliverConfigSnapshotError::HttpDispatch(HttpDispatchError::from(err))
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
/// Errors returned by DescribeAggregateComplianceByConfigRules
#[derive(Debug, PartialEq)]
pub enum DescribeAggregateComplianceByConfigRulesError {
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>You have specified a configuration aggregator that does not exist.</p>
    NoSuchConfigurationAggregator(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeAggregateComplianceByConfigRulesError {
    pub fn from_body(body: &str) -> DescribeAggregateComplianceByConfigRulesError {
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
                    "InvalidLimitException" => {
                        DescribeAggregateComplianceByConfigRulesError::InvalidLimit(String::from(
                            error_message,
                        ))
                    }
                    "InvalidNextTokenException" => {
                        DescribeAggregateComplianceByConfigRulesError::InvalidNextToken(
                            String::from(error_message),
                        )
                    }
                    "NoSuchConfigurationAggregatorException" => {
                        DescribeAggregateComplianceByConfigRulesError::NoSuchConfigurationAggregator(
                            String::from(error_message),
                        )
                    }
                    "ValidationException" => {
                        DescribeAggregateComplianceByConfigRulesError::Validation(
                            error_message.to_string(),
                        )
                    }
                    _ => DescribeAggregateComplianceByConfigRulesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeAggregateComplianceByConfigRulesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeAggregateComplianceByConfigRulesError {
    fn from(err: serde_json::error::Error) -> DescribeAggregateComplianceByConfigRulesError {
        DescribeAggregateComplianceByConfigRulesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeAggregateComplianceByConfigRulesError {
    fn from(err: CredentialsError) -> DescribeAggregateComplianceByConfigRulesError {
        DescribeAggregateComplianceByConfigRulesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeAggregateComplianceByConfigRulesError {
    fn from(err: HttpDispatchError) -> DescribeAggregateComplianceByConfigRulesError {
        DescribeAggregateComplianceByConfigRulesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeAggregateComplianceByConfigRulesError {
    fn from(err: io::Error) -> DescribeAggregateComplianceByConfigRulesError {
        DescribeAggregateComplianceByConfigRulesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeAggregateComplianceByConfigRulesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAggregateComplianceByConfigRulesError {
    fn description(&self) -> &str {
        match *self {
            DescribeAggregateComplianceByConfigRulesError::InvalidLimit(ref cause) => cause,
            DescribeAggregateComplianceByConfigRulesError::InvalidNextToken(ref cause) => cause,
            DescribeAggregateComplianceByConfigRulesError::NoSuchConfigurationAggregator(
                ref cause,
            ) => cause,
            DescribeAggregateComplianceByConfigRulesError::Validation(ref cause) => cause,
            DescribeAggregateComplianceByConfigRulesError::Credentials(ref err) => {
                err.description()
            }
            DescribeAggregateComplianceByConfigRulesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeAggregateComplianceByConfigRulesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAggregationAuthorizations
#[derive(Debug, PartialEq)]
pub enum DescribeAggregationAuthorizationsError {
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
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

impl DescribeAggregationAuthorizationsError {
    pub fn from_body(body: &str) -> DescribeAggregationAuthorizationsError {
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
                    "InvalidLimitException" => {
                        DescribeAggregationAuthorizationsError::InvalidLimit(String::from(
                            error_message,
                        ))
                    }
                    "InvalidNextTokenException" => {
                        DescribeAggregationAuthorizationsError::InvalidNextToken(String::from(
                            error_message,
                        ))
                    }
                    "InvalidParameterValueException" => {
                        DescribeAggregationAuthorizationsError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => DescribeAggregationAuthorizationsError::Validation(
                        error_message.to_string(),
                    ),
                    _ => DescribeAggregationAuthorizationsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeAggregationAuthorizationsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeAggregationAuthorizationsError {
    fn from(err: serde_json::error::Error) -> DescribeAggregationAuthorizationsError {
        DescribeAggregationAuthorizationsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeAggregationAuthorizationsError {
    fn from(err: CredentialsError) -> DescribeAggregationAuthorizationsError {
        DescribeAggregationAuthorizationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeAggregationAuthorizationsError {
    fn from(err: HttpDispatchError) -> DescribeAggregationAuthorizationsError {
        DescribeAggregationAuthorizationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeAggregationAuthorizationsError {
    fn from(err: io::Error) -> DescribeAggregationAuthorizationsError {
        DescribeAggregationAuthorizationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeAggregationAuthorizationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAggregationAuthorizationsError {
    fn description(&self) -> &str {
        match *self {
            DescribeAggregationAuthorizationsError::InvalidLimit(ref cause) => cause,
            DescribeAggregationAuthorizationsError::InvalidNextToken(ref cause) => cause,
            DescribeAggregationAuthorizationsError::InvalidParameterValue(ref cause) => cause,
            DescribeAggregationAuthorizationsError::Validation(ref cause) => cause,
            DescribeAggregationAuthorizationsError::Credentials(ref err) => err.description(),
            DescribeAggregationAuthorizationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeAggregationAuthorizationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeComplianceByConfigRule
#[derive(Debug, PartialEq)]
pub enum DescribeComplianceByConfigRuleError {
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
    /// <p>One or more AWS Config rules in the request are invalid. Verify that the rule names are correct and try again.</p>
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
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidNextTokenException" => {
                        DescribeComplianceByConfigRuleError::InvalidNextToken(String::from(
                            error_message,
                        ))
                    }
                    "InvalidParameterValueException" => {
                        DescribeComplianceByConfigRuleError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "NoSuchConfigRuleException" => {
                        DescribeComplianceByConfigRuleError::NoSuchConfigRule(String::from(
                            error_message,
                        ))
                    }
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
impl From<io::Error> for DescribeComplianceByConfigRuleError {
    fn from(err: io::Error) -> DescribeComplianceByConfigRuleError {
        DescribeComplianceByConfigRuleError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
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
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidNextTokenException" => {
                        DescribeComplianceByResourceError::InvalidNextToken(String::from(
                            error_message,
                        ))
                    }
                    "InvalidParameterValueException" => {
                        DescribeComplianceByResourceError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
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
impl From<io::Error> for DescribeComplianceByResourceError {
    fn from(err: io::Error) -> DescribeComplianceByResourceError {
        DescribeComplianceByResourceError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
    /// <p>One or more AWS Config rules in the request are invalid. Verify that the rule names are correct and try again.</p>
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
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidNextTokenException" => {
                        DescribeConfigRuleEvaluationStatusError::InvalidNextToken(String::from(
                            error_message,
                        ))
                    }
                    "InvalidParameterValueException" => {
                        DescribeConfigRuleEvaluationStatusError::InvalidParameterValue(
                            String::from(error_message),
                        )
                    }
                    "NoSuchConfigRuleException" => {
                        DescribeConfigRuleEvaluationStatusError::NoSuchConfigRule(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => DescribeConfigRuleEvaluationStatusError::Validation(
                        error_message.to_string(),
                    ),
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
impl From<io::Error> for DescribeConfigRuleEvaluationStatusError {
    fn from(err: io::Error) -> DescribeConfigRuleEvaluationStatusError {
        DescribeConfigRuleEvaluationStatusError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>One or more AWS Config rules in the request are invalid. Verify that the rule names are correct and try again.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for DescribeConfigRulesError {
    fn from(err: io::Error) -> DescribeConfigRulesError {
        DescribeConfigRulesError::HttpDispatch(HttpDispatchError::from(err))
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
/// Errors returned by DescribeConfigurationAggregatorSourcesStatus
#[derive(Debug, PartialEq)]
pub enum DescribeConfigurationAggregatorSourcesStatusError {
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
    /// <p>You have specified a configuration aggregator that does not exist.</p>
    NoSuchConfigurationAggregator(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeConfigurationAggregatorSourcesStatusError {
    pub fn from_body(body: &str) -> DescribeConfigurationAggregatorSourcesStatusError {
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
                                    "InvalidLimitException" => DescribeConfigurationAggregatorSourcesStatusError::InvalidLimit(String::from(error_message)),
"InvalidNextTokenException" => DescribeConfigurationAggregatorSourcesStatusError::InvalidNextToken(String::from(error_message)),
"InvalidParameterValueException" => DescribeConfigurationAggregatorSourcesStatusError::InvalidParameterValue(String::from(error_message)),
"NoSuchConfigurationAggregatorException" => DescribeConfigurationAggregatorSourcesStatusError::NoSuchConfigurationAggregator(String::from(error_message)),
"ValidationException" => DescribeConfigurationAggregatorSourcesStatusError::Validation(error_message.to_string()),
_ => DescribeConfigurationAggregatorSourcesStatusError::Unknown(String::from(body))
                                }
            }
            Err(_) => {
                DescribeConfigurationAggregatorSourcesStatusError::Unknown(String::from(body))
            }
        }
    }
}

impl From<serde_json::error::Error> for DescribeConfigurationAggregatorSourcesStatusError {
    fn from(err: serde_json::error::Error) -> DescribeConfigurationAggregatorSourcesStatusError {
        DescribeConfigurationAggregatorSourcesStatusError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeConfigurationAggregatorSourcesStatusError {
    fn from(err: CredentialsError) -> DescribeConfigurationAggregatorSourcesStatusError {
        DescribeConfigurationAggregatorSourcesStatusError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeConfigurationAggregatorSourcesStatusError {
    fn from(err: HttpDispatchError) -> DescribeConfigurationAggregatorSourcesStatusError {
        DescribeConfigurationAggregatorSourcesStatusError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeConfigurationAggregatorSourcesStatusError {
    fn from(err: io::Error) -> DescribeConfigurationAggregatorSourcesStatusError {
        DescribeConfigurationAggregatorSourcesStatusError::HttpDispatch(HttpDispatchError::from(
            err,
        ))
    }
}
impl fmt::Display for DescribeConfigurationAggregatorSourcesStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeConfigurationAggregatorSourcesStatusError {
    fn description(&self) -> &str {
        match *self {
            DescribeConfigurationAggregatorSourcesStatusError::InvalidLimit(ref cause) => cause,
            DescribeConfigurationAggregatorSourcesStatusError::InvalidNextToken(ref cause) => cause,
            DescribeConfigurationAggregatorSourcesStatusError::InvalidParameterValue(ref cause) => {
                cause
            }
            DescribeConfigurationAggregatorSourcesStatusError::NoSuchConfigurationAggregator(
                ref cause,
            ) => cause,
            DescribeConfigurationAggregatorSourcesStatusError::Validation(ref cause) => cause,
            DescribeConfigurationAggregatorSourcesStatusError::Credentials(ref err) => {
                err.description()
            }
            DescribeConfigurationAggregatorSourcesStatusError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeConfigurationAggregatorSourcesStatusError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeConfigurationAggregators
#[derive(Debug, PartialEq)]
pub enum DescribeConfigurationAggregatorsError {
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
    /// <p>You have specified a configuration aggregator that does not exist.</p>
    NoSuchConfigurationAggregator(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeConfigurationAggregatorsError {
    pub fn from_body(body: &str) -> DescribeConfigurationAggregatorsError {
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
                    "InvalidLimitException" => DescribeConfigurationAggregatorsError::InvalidLimit(
                        String::from(error_message),
                    ),
                    "InvalidNextTokenException" => {
                        DescribeConfigurationAggregatorsError::InvalidNextToken(String::from(
                            error_message,
                        ))
                    }
                    "InvalidParameterValueException" => {
                        DescribeConfigurationAggregatorsError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "NoSuchConfigurationAggregatorException" => {
                        DescribeConfigurationAggregatorsError::NoSuchConfigurationAggregator(
                            String::from(error_message),
                        )
                    }
                    "ValidationException" => {
                        DescribeConfigurationAggregatorsError::Validation(error_message.to_string())
                    }
                    _ => DescribeConfigurationAggregatorsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeConfigurationAggregatorsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeConfigurationAggregatorsError {
    fn from(err: serde_json::error::Error) -> DescribeConfigurationAggregatorsError {
        DescribeConfigurationAggregatorsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeConfigurationAggregatorsError {
    fn from(err: CredentialsError) -> DescribeConfigurationAggregatorsError {
        DescribeConfigurationAggregatorsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeConfigurationAggregatorsError {
    fn from(err: HttpDispatchError) -> DescribeConfigurationAggregatorsError {
        DescribeConfigurationAggregatorsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeConfigurationAggregatorsError {
    fn from(err: io::Error) -> DescribeConfigurationAggregatorsError {
        DescribeConfigurationAggregatorsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeConfigurationAggregatorsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeConfigurationAggregatorsError {
    fn description(&self) -> &str {
        match *self {
            DescribeConfigurationAggregatorsError::InvalidLimit(ref cause) => cause,
            DescribeConfigurationAggregatorsError::InvalidNextToken(ref cause) => cause,
            DescribeConfigurationAggregatorsError::InvalidParameterValue(ref cause) => cause,
            DescribeConfigurationAggregatorsError::NoSuchConfigurationAggregator(ref cause) => {
                cause
            }
            DescribeConfigurationAggregatorsError::Validation(ref cause) => cause,
            DescribeConfigurationAggregatorsError::Credentials(ref err) => err.description(),
            DescribeConfigurationAggregatorsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeConfigurationAggregatorsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeConfigurationRecorderStatus
#[derive(Debug, PartialEq)]
pub enum DescribeConfigurationRecorderStatusError {
    /// <p>You have specified a configuration recorder that does not exist.</p>
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
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "NoSuchConfigurationRecorderException" => {
                        DescribeConfigurationRecorderStatusError::NoSuchConfigurationRecorder(
                            String::from(error_message),
                        )
                    }
                    "ValidationException" => DescribeConfigurationRecorderStatusError::Validation(
                        error_message.to_string(),
                    ),
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
impl From<io::Error> for DescribeConfigurationRecorderStatusError {
    fn from(err: io::Error) -> DescribeConfigurationRecorderStatusError {
        DescribeConfigurationRecorderStatusError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>You have specified a configuration recorder that does not exist.</p>
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
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "NoSuchConfigurationRecorderException" => {
                        DescribeConfigurationRecordersError::NoSuchConfigurationRecorder(
                            String::from(error_message),
                        )
                    }
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
impl From<io::Error> for DescribeConfigurationRecordersError {
    fn from(err: io::Error) -> DescribeConfigurationRecordersError {
        DescribeConfigurationRecordersError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>You have specified a delivery channel that does not exist.</p>
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
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "NoSuchDeliveryChannelException" => {
                        DescribeDeliveryChannelStatusError::NoSuchDeliveryChannel(String::from(
                            error_message,
                        ))
                    }
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
impl From<io::Error> for DescribeDeliveryChannelStatusError {
    fn from(err: io::Error) -> DescribeDeliveryChannelStatusError {
        DescribeDeliveryChannelStatusError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>You have specified a delivery channel that does not exist.</p>
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
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "NoSuchDeliveryChannelException" => {
                        DescribeDeliveryChannelsError::NoSuchDeliveryChannel(String::from(
                            error_message,
                        ))
                    }
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
impl From<io::Error> for DescribeDeliveryChannelsError {
    fn from(err: io::Error) -> DescribeDeliveryChannelsError {
        DescribeDeliveryChannelsError::HttpDispatch(HttpDispatchError::from(err))
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
/// Errors returned by DescribePendingAggregationRequests
#[derive(Debug, PartialEq)]
pub enum DescribePendingAggregationRequestsError {
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
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

impl DescribePendingAggregationRequestsError {
    pub fn from_body(body: &str) -> DescribePendingAggregationRequestsError {
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
                    "InvalidLimitException" => {
                        DescribePendingAggregationRequestsError::InvalidLimit(String::from(
                            error_message,
                        ))
                    }
                    "InvalidNextTokenException" => {
                        DescribePendingAggregationRequestsError::InvalidNextToken(String::from(
                            error_message,
                        ))
                    }
                    "InvalidParameterValueException" => {
                        DescribePendingAggregationRequestsError::InvalidParameterValue(
                            String::from(error_message),
                        )
                    }
                    "ValidationException" => DescribePendingAggregationRequestsError::Validation(
                        error_message.to_string(),
                    ),
                    _ => DescribePendingAggregationRequestsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribePendingAggregationRequestsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribePendingAggregationRequestsError {
    fn from(err: serde_json::error::Error) -> DescribePendingAggregationRequestsError {
        DescribePendingAggregationRequestsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribePendingAggregationRequestsError {
    fn from(err: CredentialsError) -> DescribePendingAggregationRequestsError {
        DescribePendingAggregationRequestsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribePendingAggregationRequestsError {
    fn from(err: HttpDispatchError) -> DescribePendingAggregationRequestsError {
        DescribePendingAggregationRequestsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribePendingAggregationRequestsError {
    fn from(err: io::Error) -> DescribePendingAggregationRequestsError {
        DescribePendingAggregationRequestsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribePendingAggregationRequestsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribePendingAggregationRequestsError {
    fn description(&self) -> &str {
        match *self {
            DescribePendingAggregationRequestsError::InvalidLimit(ref cause) => cause,
            DescribePendingAggregationRequestsError::InvalidNextToken(ref cause) => cause,
            DescribePendingAggregationRequestsError::InvalidParameterValue(ref cause) => cause,
            DescribePendingAggregationRequestsError::Validation(ref cause) => cause,
            DescribePendingAggregationRequestsError::Credentials(ref err) => err.description(),
            DescribePendingAggregationRequestsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribePendingAggregationRequestsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetAggregateComplianceDetailsByConfigRule
#[derive(Debug, PartialEq)]
pub enum GetAggregateComplianceDetailsByConfigRuleError {
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>You have specified a configuration aggregator that does not exist.</p>
    NoSuchConfigurationAggregator(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetAggregateComplianceDetailsByConfigRuleError {
    pub fn from_body(body: &str) -> GetAggregateComplianceDetailsByConfigRuleError {
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
                                    "InvalidLimitException" => GetAggregateComplianceDetailsByConfigRuleError::InvalidLimit(String::from(error_message)),
"InvalidNextTokenException" => GetAggregateComplianceDetailsByConfigRuleError::InvalidNextToken(String::from(error_message)),
"NoSuchConfigurationAggregatorException" => GetAggregateComplianceDetailsByConfigRuleError::NoSuchConfigurationAggregator(String::from(error_message)),
"ValidationException" => GetAggregateComplianceDetailsByConfigRuleError::Validation(error_message.to_string()),
_ => GetAggregateComplianceDetailsByConfigRuleError::Unknown(String::from(body))
                                }
            }
            Err(_) => GetAggregateComplianceDetailsByConfigRuleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetAggregateComplianceDetailsByConfigRuleError {
    fn from(err: serde_json::error::Error) -> GetAggregateComplianceDetailsByConfigRuleError {
        GetAggregateComplianceDetailsByConfigRuleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetAggregateComplianceDetailsByConfigRuleError {
    fn from(err: CredentialsError) -> GetAggregateComplianceDetailsByConfigRuleError {
        GetAggregateComplianceDetailsByConfigRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetAggregateComplianceDetailsByConfigRuleError {
    fn from(err: HttpDispatchError) -> GetAggregateComplianceDetailsByConfigRuleError {
        GetAggregateComplianceDetailsByConfigRuleError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetAggregateComplianceDetailsByConfigRuleError {
    fn from(err: io::Error) -> GetAggregateComplianceDetailsByConfigRuleError {
        GetAggregateComplianceDetailsByConfigRuleError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetAggregateComplianceDetailsByConfigRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAggregateComplianceDetailsByConfigRuleError {
    fn description(&self) -> &str {
        match *self {
            GetAggregateComplianceDetailsByConfigRuleError::InvalidLimit(ref cause) => cause,
            GetAggregateComplianceDetailsByConfigRuleError::InvalidNextToken(ref cause) => cause,
            GetAggregateComplianceDetailsByConfigRuleError::NoSuchConfigurationAggregator(
                ref cause,
            ) => cause,
            GetAggregateComplianceDetailsByConfigRuleError::Validation(ref cause) => cause,
            GetAggregateComplianceDetailsByConfigRuleError::Credentials(ref err) => {
                err.description()
            }
            GetAggregateComplianceDetailsByConfigRuleError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetAggregateComplianceDetailsByConfigRuleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetAggregateConfigRuleComplianceSummary
#[derive(Debug, PartialEq)]
pub enum GetAggregateConfigRuleComplianceSummaryError {
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>You have specified a configuration aggregator that does not exist.</p>
    NoSuchConfigurationAggregator(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetAggregateConfigRuleComplianceSummaryError {
    pub fn from_body(body: &str) -> GetAggregateConfigRuleComplianceSummaryError {
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
                    "InvalidLimitException" => {
                        GetAggregateConfigRuleComplianceSummaryError::InvalidLimit(String::from(
                            error_message,
                        ))
                    }
                    "InvalidNextTokenException" => {
                        GetAggregateConfigRuleComplianceSummaryError::InvalidNextToken(
                            String::from(error_message),
                        )
                    }
                    "NoSuchConfigurationAggregatorException" => {
                        GetAggregateConfigRuleComplianceSummaryError::NoSuchConfigurationAggregator(
                            String::from(error_message),
                        )
                    }
                    "ValidationException" => {
                        GetAggregateConfigRuleComplianceSummaryError::Validation(
                            error_message.to_string(),
                        )
                    }
                    _ => GetAggregateConfigRuleComplianceSummaryError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetAggregateConfigRuleComplianceSummaryError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetAggregateConfigRuleComplianceSummaryError {
    fn from(err: serde_json::error::Error) -> GetAggregateConfigRuleComplianceSummaryError {
        GetAggregateConfigRuleComplianceSummaryError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetAggregateConfigRuleComplianceSummaryError {
    fn from(err: CredentialsError) -> GetAggregateConfigRuleComplianceSummaryError {
        GetAggregateConfigRuleComplianceSummaryError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetAggregateConfigRuleComplianceSummaryError {
    fn from(err: HttpDispatchError) -> GetAggregateConfigRuleComplianceSummaryError {
        GetAggregateConfigRuleComplianceSummaryError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetAggregateConfigRuleComplianceSummaryError {
    fn from(err: io::Error) -> GetAggregateConfigRuleComplianceSummaryError {
        GetAggregateConfigRuleComplianceSummaryError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetAggregateConfigRuleComplianceSummaryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAggregateConfigRuleComplianceSummaryError {
    fn description(&self) -> &str {
        match *self {
            GetAggregateConfigRuleComplianceSummaryError::InvalidLimit(ref cause) => cause,
            GetAggregateConfigRuleComplianceSummaryError::InvalidNextToken(ref cause) => cause,
            GetAggregateConfigRuleComplianceSummaryError::NoSuchConfigurationAggregator(
                ref cause,
            ) => cause,
            GetAggregateConfigRuleComplianceSummaryError::Validation(ref cause) => cause,
            GetAggregateConfigRuleComplianceSummaryError::Credentials(ref err) => err.description(),
            GetAggregateConfigRuleComplianceSummaryError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetAggregateConfigRuleComplianceSummaryError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetComplianceDetailsByConfigRule
#[derive(Debug, PartialEq)]
pub enum GetComplianceDetailsByConfigRuleError {
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
    /// <p>One or more AWS Config rules in the request are invalid. Verify that the rule names are correct and try again.</p>
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
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidNextTokenException" => {
                        GetComplianceDetailsByConfigRuleError::InvalidNextToken(String::from(
                            error_message,
                        ))
                    }
                    "InvalidParameterValueException" => {
                        GetComplianceDetailsByConfigRuleError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "NoSuchConfigRuleException" => {
                        GetComplianceDetailsByConfigRuleError::NoSuchConfigRule(String::from(
                            error_message,
                        ))
                    }
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
impl From<io::Error> for GetComplianceDetailsByConfigRuleError {
    fn from(err: io::Error) -> GetComplianceDetailsByConfigRuleError {
        GetComplianceDetailsByConfigRuleError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
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
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterValueException" => {
                        GetComplianceDetailsByResourceError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
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
impl From<io::Error> for GetComplianceDetailsByResourceError {
    fn from(err: io::Error) -> GetComplianceDetailsByResourceError {
        GetComplianceDetailsByResourceError::HttpDispatch(HttpDispatchError::from(err))
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for GetComplianceSummaryByConfigRuleError {
    fn from(err: io::Error) -> GetComplianceSummaryByConfigRuleError {
        GetComplianceSummaryByConfigRuleError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
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
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterValueException" => {
                        GetComplianceSummaryByResourceTypeError::InvalidParameterValue(
                            String::from(error_message),
                        )
                    }
                    "ValidationException" => GetComplianceSummaryByResourceTypeError::Validation(
                        error_message.to_string(),
                    ),
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
impl From<io::Error> for GetComplianceSummaryByResourceTypeError {
    fn from(err: io::Error) -> GetComplianceSummaryByResourceTypeError {
        GetComplianceSummaryByResourceTypeError::HttpDispatch(HttpDispatchError::from(err))
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
/// Errors returned by GetDiscoveredResourceCounts
#[derive(Debug, PartialEq)]
pub enum GetDiscoveredResourceCountsError {
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetDiscoveredResourceCountsError {
    pub fn from_body(body: &str) -> GetDiscoveredResourceCountsError {
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
                    "InvalidLimitException" => {
                        GetDiscoveredResourceCountsError::InvalidLimit(String::from(error_message))
                    }
                    "InvalidNextTokenException" => {
                        GetDiscoveredResourceCountsError::InvalidNextToken(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        GetDiscoveredResourceCountsError::Validation(error_message.to_string())
                    }
                    _ => GetDiscoveredResourceCountsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetDiscoveredResourceCountsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetDiscoveredResourceCountsError {
    fn from(err: serde_json::error::Error) -> GetDiscoveredResourceCountsError {
        GetDiscoveredResourceCountsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDiscoveredResourceCountsError {
    fn from(err: CredentialsError) -> GetDiscoveredResourceCountsError {
        GetDiscoveredResourceCountsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDiscoveredResourceCountsError {
    fn from(err: HttpDispatchError) -> GetDiscoveredResourceCountsError {
        GetDiscoveredResourceCountsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDiscoveredResourceCountsError {
    fn from(err: io::Error) -> GetDiscoveredResourceCountsError {
        GetDiscoveredResourceCountsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDiscoveredResourceCountsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDiscoveredResourceCountsError {
    fn description(&self) -> &str {
        match *self {
            GetDiscoveredResourceCountsError::InvalidLimit(ref cause) => cause,
            GetDiscoveredResourceCountsError::InvalidNextToken(ref cause) => cause,
            GetDiscoveredResourceCountsError::Validation(ref cause) => cause,
            GetDiscoveredResourceCountsError::Credentials(ref err) => err.description(),
            GetDiscoveredResourceCountsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetDiscoveredResourceCountsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetResourceConfigHistory
#[derive(Debug, PartialEq)]
pub enum GetResourceConfigHistoryError {
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>The specified time range is not valid. The earlier time is not chronologically before the later time.</p>
    InvalidTimeRange(String),
    /// <p>There are no configuration recorders available to provide the role needed to describe your resources. Create a configuration recorder.</p>
    NoAvailableConfigurationRecorder(String),
    /// <p>You have specified a resource that is either unknown or has not been discovered.</p>
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
                let raw_error_type = json
                    .get("__type")
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
                    "NoAvailableConfigurationRecorderException" => {
                        GetResourceConfigHistoryError::NoAvailableConfigurationRecorder(
                            String::from(error_message),
                        )
                    }
                    "ResourceNotDiscoveredException" => {
                        GetResourceConfigHistoryError::ResourceNotDiscovered(String::from(
                            error_message,
                        ))
                    }
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
impl From<io::Error> for GetResourceConfigHistoryError {
    fn from(err: io::Error) -> GetResourceConfigHistoryError {
        GetResourceConfigHistoryError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>There are no configuration recorders available to provide the role needed to describe your resources. Create a configuration recorder.</p>
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
                let raw_error_type = json
                    .get("__type")
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
                    "NoAvailableConfigurationRecorderException" => {
                        ListDiscoveredResourcesError::NoAvailableConfigurationRecorder(
                            String::from(error_message),
                        )
                    }
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
impl From<io::Error> for ListDiscoveredResourcesError {
    fn from(err: io::Error) -> ListDiscoveredResourcesError {
        ListDiscoveredResourcesError::HttpDispatch(HttpDispatchError::from(err))
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
/// Errors returned by PutAggregationAuthorization
#[derive(Debug, PartialEq)]
pub enum PutAggregationAuthorizationError {
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
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

impl PutAggregationAuthorizationError {
    pub fn from_body(body: &str) -> PutAggregationAuthorizationError {
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
                    "InvalidParameterValueException" => {
                        PutAggregationAuthorizationError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        PutAggregationAuthorizationError::Validation(error_message.to_string())
                    }
                    _ => PutAggregationAuthorizationError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutAggregationAuthorizationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutAggregationAuthorizationError {
    fn from(err: serde_json::error::Error) -> PutAggregationAuthorizationError {
        PutAggregationAuthorizationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutAggregationAuthorizationError {
    fn from(err: CredentialsError) -> PutAggregationAuthorizationError {
        PutAggregationAuthorizationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutAggregationAuthorizationError {
    fn from(err: HttpDispatchError) -> PutAggregationAuthorizationError {
        PutAggregationAuthorizationError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutAggregationAuthorizationError {
    fn from(err: io::Error) -> PutAggregationAuthorizationError {
        PutAggregationAuthorizationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutAggregationAuthorizationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutAggregationAuthorizationError {
    fn description(&self) -> &str {
        match *self {
            PutAggregationAuthorizationError::InvalidParameterValue(ref cause) => cause,
            PutAggregationAuthorizationError::Validation(ref cause) => cause,
            PutAggregationAuthorizationError::Credentials(ref err) => err.description(),
            PutAggregationAuthorizationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutAggregationAuthorizationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutConfigRule
#[derive(Debug, PartialEq)]
pub enum PutConfigRuleError {
    /// <p><p>Indicates one of the following errors:</p> <ul> <li> <p>The rule cannot be created because the IAM role assigned to AWS Config lacks permissions to perform the config:Put* action.</p> </li> <li> <p>The AWS Lambda function cannot be invoked. Check the function ARN, and check the function&#39;s permissions.</p> </li> </ul></p>
    InsufficientPermissions(String),
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
    /// <p>Failed to add the AWS Config rule because the account already contains the maximum number of 50 rules. Consider deleting any deactivated rules before you add new rules.</p>
    MaxNumberOfConfigRulesExceeded(String),
    /// <p>There are no configuration recorders available to provide the role needed to describe your resources. Create a configuration recorder.</p>
    NoAvailableConfigurationRecorder(String),
    /// <p>The rule is currently being deleted or the rule is deleting your evaluation results. Try your request again later.</p>
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
                let raw_error_type = json
                    .get("__type")
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
                    "MaxNumberOfConfigRulesExceededException" => {
                        PutConfigRuleError::MaxNumberOfConfigRulesExceeded(String::from(
                            error_message,
                        ))
                    }
                    "NoAvailableConfigurationRecorderException" => {
                        PutConfigRuleError::NoAvailableConfigurationRecorder(String::from(
                            error_message,
                        ))
                    }
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
impl From<io::Error> for PutConfigRuleError {
    fn from(err: io::Error) -> PutConfigRuleError {
        PutConfigRuleError::HttpDispatch(HttpDispatchError::from(err))
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
/// Errors returned by PutConfigurationAggregator
#[derive(Debug, PartialEq)]
pub enum PutConfigurationAggregatorError {
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
    /// <p>You have provided a null or empty role ARN.</p>
    InvalidRole(String),
    /// <p>This exception is thrown if an evaluation is in progress or if you call the <a>StartConfigRulesEvaluation</a> API more than once per minute.</p>
    LimitExceeded(String),
    /// <p>Organization does is no longer available.</p>
    NoAvailableOrganization(String),
    /// <p>No permission to call the EnableAWSServiceAccess API.</p>
    OrganizationAccessDenied(String),
    /// <p>The configuration aggregator cannot be created because organization does not have all features enabled.</p>
    OrganizationAllFeaturesNotEnabled(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutConfigurationAggregatorError {
    pub fn from_body(body: &str) -> PutConfigurationAggregatorError {
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
                    "InvalidParameterValueException" => {
                        PutConfigurationAggregatorError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "InvalidRoleException" => {
                        PutConfigurationAggregatorError::InvalidRole(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        PutConfigurationAggregatorError::LimitExceeded(String::from(error_message))
                    }
                    "NoAvailableOrganizationException" => {
                        PutConfigurationAggregatorError::NoAvailableOrganization(String::from(
                            error_message,
                        ))
                    }
                    "OrganizationAccessDeniedException" => {
                        PutConfigurationAggregatorError::OrganizationAccessDenied(String::from(
                            error_message,
                        ))
                    }
                    "OrganizationAllFeaturesNotEnabledException" => {
                        PutConfigurationAggregatorError::OrganizationAllFeaturesNotEnabled(
                            String::from(error_message),
                        )
                    }
                    "ValidationException" => {
                        PutConfigurationAggregatorError::Validation(error_message.to_string())
                    }
                    _ => PutConfigurationAggregatorError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutConfigurationAggregatorError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutConfigurationAggregatorError {
    fn from(err: serde_json::error::Error) -> PutConfigurationAggregatorError {
        PutConfigurationAggregatorError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutConfigurationAggregatorError {
    fn from(err: CredentialsError) -> PutConfigurationAggregatorError {
        PutConfigurationAggregatorError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutConfigurationAggregatorError {
    fn from(err: HttpDispatchError) -> PutConfigurationAggregatorError {
        PutConfigurationAggregatorError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutConfigurationAggregatorError {
    fn from(err: io::Error) -> PutConfigurationAggregatorError {
        PutConfigurationAggregatorError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutConfigurationAggregatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutConfigurationAggregatorError {
    fn description(&self) -> &str {
        match *self {
            PutConfigurationAggregatorError::InvalidParameterValue(ref cause) => cause,
            PutConfigurationAggregatorError::InvalidRole(ref cause) => cause,
            PutConfigurationAggregatorError::LimitExceeded(ref cause) => cause,
            PutConfigurationAggregatorError::NoAvailableOrganization(ref cause) => cause,
            PutConfigurationAggregatorError::OrganizationAccessDenied(ref cause) => cause,
            PutConfigurationAggregatorError::OrganizationAllFeaturesNotEnabled(ref cause) => cause,
            PutConfigurationAggregatorError::Validation(ref cause) => cause,
            PutConfigurationAggregatorError::Credentials(ref err) => err.description(),
            PutConfigurationAggregatorError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutConfigurationAggregatorError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutConfigurationRecorder
#[derive(Debug, PartialEq)]
pub enum PutConfigurationRecorderError {
    /// <p>You have provided a configuration recorder name that is not valid.</p>
    InvalidConfigurationRecorderName(String),
    /// <p>AWS Config throws an exception if the recording group does not contain a valid list of resource types. Invalid values might also be incorrectly formatted.</p>
    InvalidRecordingGroup(String),
    /// <p>You have provided a null or empty role ARN.</p>
    InvalidRole(String),
    /// <p>You have reached the limit of the number of recorders you can create.</p>
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
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidConfigurationRecorderNameException" => {
                        PutConfigurationRecorderError::InvalidConfigurationRecorderName(
                            String::from(error_message),
                        )
                    }
                    "InvalidRecordingGroupException" => {
                        PutConfigurationRecorderError::InvalidRecordingGroup(String::from(
                            error_message,
                        ))
                    }
                    "InvalidRoleException" => {
                        PutConfigurationRecorderError::InvalidRole(String::from(error_message))
                    }
                    "MaxNumberOfConfigurationRecordersExceededException" => {
                        PutConfigurationRecorderError::MaxNumberOfConfigurationRecordersExceeded(
                            String::from(error_message),
                        )
                    }
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
impl From<io::Error> for PutConfigurationRecorderError {
    fn from(err: io::Error) -> PutConfigurationRecorderError {
        PutConfigurationRecorderError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>Your Amazon S3 bucket policy does not permit AWS Config to write to it.</p>
    InsufficientDeliveryPolicy(String),
    /// <p>The specified delivery channel name is not valid.</p>
    InvalidDeliveryChannelName(String),
    /// <p>The specified Amazon S3 key prefix is not valid.</p>
    InvalidS3KeyPrefix(String),
    /// <p>The specified Amazon SNS topic does not exist.</p>
    InvalidSNSTopicARN(String),
    /// <p>You have reached the limit of the number of delivery channels you can create.</p>
    MaxNumberOfDeliveryChannelsExceeded(String),
    /// <p>There are no configuration recorders available to provide the role needed to describe your resources. Create a configuration recorder.</p>
    NoAvailableConfigurationRecorder(String),
    /// <p>The specified Amazon S3 bucket does not exist.</p>
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
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InsufficientDeliveryPolicyException" => {
                        PutDeliveryChannelError::InsufficientDeliveryPolicy(String::from(
                            error_message,
                        ))
                    }
                    "InvalidDeliveryChannelNameException" => {
                        PutDeliveryChannelError::InvalidDeliveryChannelName(String::from(
                            error_message,
                        ))
                    }
                    "InvalidS3KeyPrefixException" => {
                        PutDeliveryChannelError::InvalidS3KeyPrefix(String::from(error_message))
                    }
                    "InvalidSNSTopicARNException" => {
                        PutDeliveryChannelError::InvalidSNSTopicARN(String::from(error_message))
                    }
                    "MaxNumberOfDeliveryChannelsExceededException" => {
                        PutDeliveryChannelError::MaxNumberOfDeliveryChannelsExceeded(String::from(
                            error_message,
                        ))
                    }
                    "NoAvailableConfigurationRecorderException" => {
                        PutDeliveryChannelError::NoAvailableConfigurationRecorder(String::from(
                            error_message,
                        ))
                    }
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
impl From<io::Error> for PutDeliveryChannelError {
    fn from(err: io::Error) -> PutDeliveryChannelError {
        PutDeliveryChannelError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
    /// <p>The specified <code>ResultToken</code> is invalid.</p>
    InvalidResultToken(String),
    /// <p>One or more AWS Config rules in the request are invalid. Verify that the rule names are correct and try again.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for PutEvaluationsError {
    fn from(err: io::Error) -> PutEvaluationsError {
        PutEvaluationsError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
    /// <p>This exception is thrown if an evaluation is in progress or if you call the <a>StartConfigRulesEvaluation</a> API more than once per minute.</p>
    LimitExceeded(String),
    /// <p>One or more AWS Config rules in the request are invalid. Verify that the rule names are correct and try again.</p>
    NoSuchConfigRule(String),
    /// <p>The rule is currently being deleted or the rule is deleting your evaluation results. Try your request again later.</p>
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
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterValueException" => {
                        StartConfigRulesEvaluationError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "LimitExceededException" => {
                        StartConfigRulesEvaluationError::LimitExceeded(String::from(error_message))
                    }
                    "NoSuchConfigRuleException" => {
                        StartConfigRulesEvaluationError::NoSuchConfigRule(String::from(
                            error_message,
                        ))
                    }
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
impl From<io::Error> for StartConfigRulesEvaluationError {
    fn from(err: io::Error) -> StartConfigRulesEvaluationError {
        StartConfigRulesEvaluationError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>There is no delivery channel available to record configurations.</p>
    NoAvailableDeliveryChannel(String),
    /// <p>You have specified a configuration recorder that does not exist.</p>
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
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "NoAvailableDeliveryChannelException" => {
                        StartConfigurationRecorderError::NoAvailableDeliveryChannel(String::from(
                            error_message,
                        ))
                    }
                    "NoSuchConfigurationRecorderException" => {
                        StartConfigurationRecorderError::NoSuchConfigurationRecorder(String::from(
                            error_message,
                        ))
                    }
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
impl From<io::Error> for StartConfigurationRecorderError {
    fn from(err: io::Error) -> StartConfigurationRecorderError {
        StartConfigurationRecorderError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>You have specified a configuration recorder that does not exist.</p>
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
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "NoSuchConfigurationRecorderException" => {
                        StopConfigurationRecorderError::NoSuchConfigurationRecorder(String::from(
                            error_message,
                        ))
                    }
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
impl From<io::Error> for StopConfigurationRecorderError {
    fn from(err: io::Error) -> StopConfigurationRecorderError {
        StopConfigurationRecorderError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p><p>Returns the current configuration for one or more requested resources. The operation also returns a list of resources that are not processed in the current request. If there are no unprocessed resources, the operation returns an empty unprocessedResourceKeys list. </p> <note> <ul> <li> <p>The API does not return results for deleted resources.</p> </li> <li> <p> The API does not return any tags for the requested resources. This information is filtered out of the supplementaryConfiguration section of the API response.</p> </li> </ul> </note></p>
    fn batch_get_resource_config(
        &self,
        input: BatchGetResourceConfigRequest,
    ) -> RusotoFuture<BatchGetResourceConfigResponse, BatchGetResourceConfigError>;

    /// <p>Deletes the authorization granted to the specified configuration aggregator account in a specified region.</p>
    fn delete_aggregation_authorization(
        &self,
        input: DeleteAggregationAuthorizationRequest,
    ) -> RusotoFuture<(), DeleteAggregationAuthorizationError>;

    /// <p>Deletes the specified AWS Config rule and all of its evaluation results.</p> <p>AWS Config sets the state of a rule to <code>DELETING</code> until the deletion is complete. You cannot update a rule while it is in this state. If you make a <code>PutConfigRule</code> or <code>DeleteConfigRule</code> request for the rule, you will receive a <code>ResourceInUseException</code>.</p> <p>You can check the state of a rule by using the <code>DescribeConfigRules</code> request.</p>
    fn delete_config_rule(
        &self,
        input: DeleteConfigRuleRequest,
    ) -> RusotoFuture<(), DeleteConfigRuleError>;

    /// <p>Deletes the specified configuration aggregator and the aggregated data associated with the aggregator.</p>
    fn delete_configuration_aggregator(
        &self,
        input: DeleteConfigurationAggregatorRequest,
    ) -> RusotoFuture<(), DeleteConfigurationAggregatorError>;

    /// <p>Deletes the configuration recorder.</p> <p>After the configuration recorder is deleted, AWS Config will not record resource configuration changes until you create a new configuration recorder.</p> <p>This action does not delete the configuration information that was previously recorded. You will be able to access the previously recorded information by using the <code>GetResourceConfigHistory</code> action, but you will not be able to access this information in the AWS Config console until you create a new configuration recorder.</p>
    fn delete_configuration_recorder(
        &self,
        input: DeleteConfigurationRecorderRequest,
    ) -> RusotoFuture<(), DeleteConfigurationRecorderError>;

    /// <p>Deletes the delivery channel.</p> <p>Before you can delete the delivery channel, you must stop the configuration recorder by using the <a>StopConfigurationRecorder</a> action.</p>
    fn delete_delivery_channel(
        &self,
        input: DeleteDeliveryChannelRequest,
    ) -> RusotoFuture<(), DeleteDeliveryChannelError>;

    /// <p>Deletes the evaluation results for the specified AWS Config rule. You can specify one AWS Config rule per request. After you delete the evaluation results, you can call the <a>StartConfigRulesEvaluation</a> API to start evaluating your AWS resources against the rule.</p>
    fn delete_evaluation_results(
        &self,
        input: DeleteEvaluationResultsRequest,
    ) -> RusotoFuture<DeleteEvaluationResultsResponse, DeleteEvaluationResultsError>;

    /// <p>Deletes pending authorization requests for a specified aggregator account in a specified region.</p>
    fn delete_pending_aggregation_request(
        &self,
        input: DeletePendingAggregationRequestRequest,
    ) -> RusotoFuture<(), DeletePendingAggregationRequestError>;

    /// <p><p>Schedules delivery of a configuration snapshot to the Amazon S3 bucket in the specified delivery channel. After the delivery has started, AWS Config sends the following notifications using an Amazon SNS topic that you have specified.</p> <ul> <li> <p>Notification of the start of the delivery.</p> </li> <li> <p>Notification of the completion of the delivery, if the delivery was successfully completed.</p> </li> <li> <p>Notification of delivery failure, if the delivery failed.</p> </li> </ul></p>
    fn deliver_config_snapshot(
        &self,
        input: DeliverConfigSnapshotRequest,
    ) -> RusotoFuture<DeliverConfigSnapshotResponse, DeliverConfigSnapshotError>;

    /// <p><p>Returns a list of compliant and noncompliant rules with the number of resources for compliant and noncompliant rules. </p> <note> <p>The results can return an empty result page, but if you have a nextToken, the results are displayed on the next page.</p> </note></p>
    fn describe_aggregate_compliance_by_config_rules(
        &self,
        input: DescribeAggregateComplianceByConfigRulesRequest,
    ) -> RusotoFuture<
        DescribeAggregateComplianceByConfigRulesResponse,
        DescribeAggregateComplianceByConfigRulesError,
    >;

    /// <p>Returns a list of authorizations granted to various aggregator accounts and regions.</p>
    fn describe_aggregation_authorizations(
        &self,
        input: DescribeAggregationAuthorizationsRequest,
    ) -> RusotoFuture<
        DescribeAggregationAuthorizationsResponse,
        DescribeAggregationAuthorizationsError,
    >;

    /// <p><p>Indicates whether the specified AWS Config rules are compliant. If a rule is noncompliant, this action returns the number of AWS resources that do not comply with the rule.</p> <p>A rule is compliant if all of the evaluated resources comply with it. It is noncompliant if any of these resources do not comply.</p> <p>If AWS Config has no current evaluation results for the rule, it returns <code>INSUFFICIENT<em>DATA</code>. This result might indicate one of the following conditions:</p> <ul> <li> <p>AWS Config has never invoked an evaluation for the rule. To check whether it has, use the <code>DescribeConfigRuleEvaluationStatus</code> action to get the <code>LastSuccessfulInvocationTime</code> and <code>LastFailedInvocationTime</code>.</p> </li> <li> <p>The rule&#39;s AWS Lambda function is failing to send evaluation results to AWS Config. Verify that the role you assigned to your configuration recorder includes the <code>config:PutEvaluations</code> permission. If the rule is a custom rule, verify that the AWS Lambda execution role includes the <code>config:PutEvaluations</code> permission.</p> </li> <li> <p>The rule&#39;s AWS Lambda function has returned <code>NOT</em>APPLICABLE</code> for all evaluation results. This can occur if the resources were deleted or removed from the rule&#39;s scope.</p> </li> </ul></p>
    fn describe_compliance_by_config_rule(
        &self,
        input: DescribeComplianceByConfigRuleRequest,
    ) -> RusotoFuture<DescribeComplianceByConfigRuleResponse, DescribeComplianceByConfigRuleError>;

    /// <p><p>Indicates whether the specified AWS resources are compliant. If a resource is noncompliant, this action returns the number of AWS Config rules that the resource does not comply with.</p> <p>A resource is compliant if it complies with all the AWS Config rules that evaluate it. It is noncompliant if it does not comply with one or more of these rules.</p> <p>If AWS Config has no current evaluation results for the resource, it returns <code>INSUFFICIENT<em>DATA</code>. This result might indicate one of the following conditions about the rules that evaluate the resource:</p> <ul> <li> <p>AWS Config has never invoked an evaluation for the rule. To check whether it has, use the <code>DescribeConfigRuleEvaluationStatus</code> action to get the <code>LastSuccessfulInvocationTime</code> and <code>LastFailedInvocationTime</code>.</p> </li> <li> <p>The rule&#39;s AWS Lambda function is failing to send evaluation results to AWS Config. Verify that the role that you assigned to your configuration recorder includes the <code>config:PutEvaluations</code> permission. If the rule is a custom rule, verify that the AWS Lambda execution role includes the <code>config:PutEvaluations</code> permission.</p> </li> <li> <p>The rule&#39;s AWS Lambda function has returned <code>NOT</em>APPLICABLE</code> for all evaluation results. This can occur if the resources were deleted or removed from the rule&#39;s scope.</p> </li> </ul></p>
    fn describe_compliance_by_resource(
        &self,
        input: DescribeComplianceByResourceRequest,
    ) -> RusotoFuture<DescribeComplianceByResourceResponse, DescribeComplianceByResourceError>;

    /// <p>Returns status information for each of your AWS managed Config rules. The status includes information such as the last time AWS Config invoked the rule, the last time AWS Config failed to invoke the rule, and the related error for the last failure.</p>
    fn describe_config_rule_evaluation_status(
        &self,
        input: DescribeConfigRuleEvaluationStatusRequest,
    ) -> RusotoFuture<
        DescribeConfigRuleEvaluationStatusResponse,
        DescribeConfigRuleEvaluationStatusError,
    >;

    /// <p>Returns details about your AWS Config rules.</p>
    fn describe_config_rules(
        &self,
        input: DescribeConfigRulesRequest,
    ) -> RusotoFuture<DescribeConfigRulesResponse, DescribeConfigRulesError>;

    /// <p>Returns status information for sources within an aggregator. The status includes information about the last time AWS Config aggregated data from source accounts or AWS Config failed to aggregate data from source accounts with the related error code or message. </p>
    fn describe_configuration_aggregator_sources_status(
        &self,
        input: DescribeConfigurationAggregatorSourcesStatusRequest,
    ) -> RusotoFuture<
        DescribeConfigurationAggregatorSourcesStatusResponse,
        DescribeConfigurationAggregatorSourcesStatusError,
    >;

    /// <p>Returns the details of one or more configuration aggregators. If the configuration aggregator is not specified, this action returns the details for all the configuration aggregators associated with the account. </p>
    fn describe_configuration_aggregators(
        &self,
        input: DescribeConfigurationAggregatorsRequest,
    ) -> RusotoFuture<DescribeConfigurationAggregatorsResponse, DescribeConfigurationAggregatorsError>;

    /// <p><p>Returns the current status of the specified configuration recorder. If a configuration recorder is not specified, this action returns the status of all configuration recorders associated with the account.</p> <note> <p>Currently, you can specify only one configuration recorder per region in your account.</p> </note></p>
    fn describe_configuration_recorder_status(
        &self,
        input: DescribeConfigurationRecorderStatusRequest,
    ) -> RusotoFuture<
        DescribeConfigurationRecorderStatusResponse,
        DescribeConfigurationRecorderStatusError,
    >;

    /// <p><p>Returns the details for the specified configuration recorders. If the configuration recorder is not specified, this action returns the details for all configuration recorders associated with the account.</p> <note> <p>Currently, you can specify only one configuration recorder per region in your account.</p> </note></p>
    fn describe_configuration_recorders(
        &self,
        input: DescribeConfigurationRecordersRequest,
    ) -> RusotoFuture<DescribeConfigurationRecordersResponse, DescribeConfigurationRecordersError>;

    /// <p><p>Returns the current status of the specified delivery channel. If a delivery channel is not specified, this action returns the current status of all delivery channels associated with the account.</p> <note> <p>Currently, you can specify only one delivery channel per region in your account.</p> </note></p>
    fn describe_delivery_channel_status(
        &self,
        input: DescribeDeliveryChannelStatusRequest,
    ) -> RusotoFuture<DescribeDeliveryChannelStatusResponse, DescribeDeliveryChannelStatusError>;

    /// <p><p>Returns details about the specified delivery channel. If a delivery channel is not specified, this action returns the details of all delivery channels associated with the account.</p> <note> <p>Currently, you can specify only one delivery channel per region in your account.</p> </note></p>
    fn describe_delivery_channels(
        &self,
        input: DescribeDeliveryChannelsRequest,
    ) -> RusotoFuture<DescribeDeliveryChannelsResponse, DescribeDeliveryChannelsError>;

    /// <p>Returns a list of all pending aggregation requests.</p>
    fn describe_pending_aggregation_requests(
        &self,
        input: DescribePendingAggregationRequestsRequest,
    ) -> RusotoFuture<
        DescribePendingAggregationRequestsResponse,
        DescribePendingAggregationRequestsError,
    >;

    /// <p><p>Returns the evaluation results for the specified AWS Config rule for a specific resource in a rule. The results indicate which AWS resources were evaluated by the rule, when each resource was last evaluated, and whether each resource complies with the rule. </p> <note> <p>The results can return an empty result page. But if you have a nextToken, the results are displayed on the next page.</p> </note></p>
    fn get_aggregate_compliance_details_by_config_rule(
        &self,
        input: GetAggregateComplianceDetailsByConfigRuleRequest,
    ) -> RusotoFuture<
        GetAggregateComplianceDetailsByConfigRuleResponse,
        GetAggregateComplianceDetailsByConfigRuleError,
    >;

    /// <p><p>Returns the number of compliant and noncompliant rules for one or more accounts and regions in an aggregator.</p> <note> <p>The results can return an empty result page, but if you have a nextToken, the results are displayed on the next page.</p> </note></p>
    fn get_aggregate_config_rule_compliance_summary(
        &self,
        input: GetAggregateConfigRuleComplianceSummaryRequest,
    ) -> RusotoFuture<
        GetAggregateConfigRuleComplianceSummaryResponse,
        GetAggregateConfigRuleComplianceSummaryError,
    >;

    /// <p>Returns the evaluation results for the specified AWS Config rule. The results indicate which AWS resources were evaluated by the rule, when each resource was last evaluated, and whether each resource complies with the rule.</p>
    fn get_compliance_details_by_config_rule(
        &self,
        input: GetComplianceDetailsByConfigRuleRequest,
    ) -> RusotoFuture<GetComplianceDetailsByConfigRuleResponse, GetComplianceDetailsByConfigRuleError>;

    /// <p>Returns the evaluation results for the specified AWS resource. The results indicate which AWS Config rules were used to evaluate the resource, when each rule was last used, and whether the resource complies with each rule.</p>
    fn get_compliance_details_by_resource(
        &self,
        input: GetComplianceDetailsByResourceRequest,
    ) -> RusotoFuture<GetComplianceDetailsByResourceResponse, GetComplianceDetailsByResourceError>;

    /// <p>Returns the number of AWS Config rules that are compliant and noncompliant, up to a maximum of 25 for each.</p>
    fn get_compliance_summary_by_config_rule(
        &self,
    ) -> RusotoFuture<GetComplianceSummaryByConfigRuleResponse, GetComplianceSummaryByConfigRuleError>;

    /// <p>Returns the number of resources that are compliant and the number that are noncompliant. You can specify one or more resource types to get these numbers for each resource type. The maximum number returned is 100.</p>
    fn get_compliance_summary_by_resource_type(
        &self,
        input: GetComplianceSummaryByResourceTypeRequest,
    ) -> RusotoFuture<
        GetComplianceSummaryByResourceTypeResponse,
        GetComplianceSummaryByResourceTypeError,
    >;

    /// <p><p>Returns the resource types, the number of each resource type, and the total number of resources that AWS Config is recording in this region for your AWS account. </p> <p class="title"> <b>Example</b> </p> <ol> <li> <p>AWS Config is recording three resource types in the US East (Ohio) Region for your account: 25 EC2 instances, 20 IAM users, and 15 S3 buckets.</p> </li> <li> <p>You make a call to the <code>GetDiscoveredResourceCounts</code> action and specify that you want all resource types. </p> </li> <li> <p>AWS Config returns the following:</p> <ul> <li> <p>The resource types (EC2 instances, IAM users, and S3 buckets).</p> </li> <li> <p>The number of each resource type (25, 20, and 15).</p> </li> <li> <p>The total number of all resources (60).</p> </li> </ul> </li> </ol> <p>The response is paginated. By default, AWS Config lists 100 <a>ResourceCount</a> objects on each page. You can customize this number with the <code>limit</code> parameter. The response includes a <code>nextToken</code> string. To get the next page of results, run the request again and specify the string for the <code>nextToken</code> parameter.</p> <note> <p>If you make a call to the <a>GetDiscoveredResourceCounts</a> action, you might not immediately receive resource counts in the following situations:</p> <ul> <li> <p>You are a new AWS Config customer.</p> </li> <li> <p>You just enabled resource recording.</p> </li> </ul> <p>It might take a few minutes for AWS Config to record and count your resources. Wait a few minutes and then retry the <a>GetDiscoveredResourceCounts</a> action. </p> </note></p>
    fn get_discovered_resource_counts(
        &self,
        input: GetDiscoveredResourceCountsRequest,
    ) -> RusotoFuture<GetDiscoveredResourceCountsResponse, GetDiscoveredResourceCountsError>;

    /// <p><p>Returns a list of configuration items for the specified resource. The list contains details about each state of the resource during the specified time interval.</p> <p>The response is paginated. By default, AWS Config returns a limit of 10 configuration items per page. You can customize this number with the <code>limit</code> parameter. The response includes a <code>nextToken</code> string. To get the next page of results, run the request again and specify the string for the <code>nextToken</code> parameter.</p> <note> <p>Each call to the API is limited to span a duration of seven days. It is likely that the number of records returned is smaller than the specified <code>limit</code>. In such cases, you can make another call, using the <code>nextToken</code>.</p> </note></p>
    fn get_resource_config_history(
        &self,
        input: GetResourceConfigHistoryRequest,
    ) -> RusotoFuture<GetResourceConfigHistoryResponse, GetResourceConfigHistoryError>;

    /// <p>Accepts a resource type and returns a list of resource identifiers for the resources of that type. A resource identifier includes the resource type, ID, and (if available) the custom resource name. The results consist of resources that AWS Config has discovered, including those that AWS Config is not currently recording. You can narrow the results to include only resources that have specific resource IDs or a resource name.</p> <note> <p>You can specify either resource IDs or a resource name, but not both, in the same request.</p> </note> <p>The response is paginated. By default, AWS Config lists 100 resource identifiers on each page. You can customize this number with the <code>limit</code> parameter. The response includes a <code>nextToken</code> string. To get the next page of results, run the request again and specify the string for the <code>nextToken</code> parameter.</p>
    fn list_discovered_resources(
        &self,
        input: ListDiscoveredResourcesRequest,
    ) -> RusotoFuture<ListDiscoveredResourcesResponse, ListDiscoveredResourcesError>;

    /// <p>Authorizes the aggregator account and region to collect data from the source account and region. </p>
    fn put_aggregation_authorization(
        &self,
        input: PutAggregationAuthorizationRequest,
    ) -> RusotoFuture<PutAggregationAuthorizationResponse, PutAggregationAuthorizationError>;

    /// <p>Adds or updates an AWS Config rule for evaluating whether your AWS resources comply with your desired configurations.</p> <p>You can use this action for custom AWS Config rules and AWS managed Config rules. A custom AWS Config rule is a rule that you develop and maintain. An AWS managed Config rule is a customizable, predefined rule that AWS Config provides.</p> <p>If you are adding a new custom AWS Config rule, you must first create the AWS Lambda function that the rule invokes to evaluate your resources. When you use the <code>PutConfigRule</code> action to add the rule to AWS Config, you must specify the Amazon Resource Name (ARN) that AWS Lambda assigns to the function. Specify the ARN for the <code>SourceIdentifier</code> key. This key is part of the <code>Source</code> object, which is part of the <code>ConfigRule</code> object. </p> <p>If you are adding an AWS managed Config rule, specify the rule's identifier for the <code>SourceIdentifier</code> key. To reference AWS managed Config rule identifiers, see <a href="http://docs.aws.amazon.com/config/latest/developerguide/evaluate-config_use-managed-rules.html">About AWS Managed Config Rules</a>.</p> <p>For any new rule that you add, specify the <code>ConfigRuleName</code> in the <code>ConfigRule</code> object. Do not specify the <code>ConfigRuleArn</code> or the <code>ConfigRuleId</code>. These values are generated by AWS Config for new rules.</p> <p>If you are updating a rule that you added previously, you can specify the rule by <code>ConfigRuleName</code>, <code>ConfigRuleId</code>, or <code>ConfigRuleArn</code> in the <code>ConfigRule</code> data type that you use in this request.</p> <p>The maximum number of rules that AWS Config supports is 50.</p> <p>For information about requesting a rule limit increase, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_config">AWS Config Limits</a> in the <i>AWS General Reference Guide</i>.</p> <p>For more information about developing and using AWS Config rules, see <a href="http://docs.aws.amazon.com/config/latest/developerguide/evaluate-config.html">Evaluating AWS Resource Configurations with AWS Config</a> in the <i>AWS Config Developer Guide</i>.</p>
    fn put_config_rule(&self, input: PutConfigRuleRequest) -> RusotoFuture<(), PutConfigRuleError>;

    /// <p><p>Creates and updates the configuration aggregator with the selected source accounts and regions.</p> <note> <p>AWS Config should be enabled in accounts and regions you want to aggreagate.</p> </note></p>
    fn put_configuration_aggregator(
        &self,
        input: PutConfigurationAggregatorRequest,
    ) -> RusotoFuture<PutConfigurationAggregatorResponse, PutConfigurationAggregatorError>;

    /// <p><p>Creates a new configuration recorder to record the selected resource configurations.</p> <p>You can use this action to change the role <code>roleARN</code> or the <code>recordingGroup</code> of an existing recorder. To change the role, call the action on the existing configuration recorder and specify a role.</p> <note> <p>Currently, you can specify only one configuration recorder per region in your account.</p> <p>If <code>ConfigurationRecorder</code> does not have the <b>recordingGroup</b> parameter specified, the default is to record all supported resource types.</p> </note></p>
    fn put_configuration_recorder(
        &self,
        input: PutConfigurationRecorderRequest,
    ) -> RusotoFuture<(), PutConfigurationRecorderError>;

    /// <p><p>Creates a delivery channel object to deliver configuration information to an Amazon S3 bucket and Amazon SNS topic.</p> <p>Before you can create a delivery channel, you must create a configuration recorder.</p> <p>You can use this action to change the Amazon S3 bucket or an Amazon SNS topic of the existing delivery channel. To change the Amazon S3 bucket or an Amazon SNS topic, call this action and specify the changed values for the S3 bucket and the SNS topic. If you specify a different value for either the S3 bucket or the SNS topic, this action will keep the existing value for the parameter that is not changed.</p> <note> <p>You can have only one delivery channel per region in your account.</p> </note></p>
    fn put_delivery_channel(
        &self,
        input: PutDeliveryChannelRequest,
    ) -> RusotoFuture<(), PutDeliveryChannelError>;

    /// <p>Used by an AWS Lambda function to deliver evaluation results to AWS Config. This action is required in every AWS Lambda function that is invoked by an AWS Config rule.</p>
    fn put_evaluations(
        &self,
        input: PutEvaluationsRequest,
    ) -> RusotoFuture<PutEvaluationsResponse, PutEvaluationsError>;

    /// <p><p>Runs an on-demand evaluation for the specified AWS Config rules against the last known configuration state of the resources. Use <code>StartConfigRulesEvaluation</code> when you want to test that a rule you updated is working as expected. <code>StartConfigRulesEvaluation</code> does not re-record the latest configuration state for your resources. It re-runs an evaluation against the last known state of your resources. </p> <p>You can specify up to 25 AWS Config rules per request. </p> <p>An existing <code>StartConfigRulesEvaluation</code> call for the specified rules must complete before you can call the API again. If you chose to have AWS Config stream to an Amazon SNS topic, you will receive a <code>ConfigRuleEvaluationStarted</code> notification when the evaluation starts.</p> <note> <p>You don&#39;t need to call the <code>StartConfigRulesEvaluation</code> API to run an evaluation for a new rule. When you create a rule, AWS Config evaluates your resources against the rule automatically. </p> </note> <p>The <code>StartConfigRulesEvaluation</code> API is useful if you want to run on-demand evaluations, such as the following example:</p> <ol> <li> <p>You have a custom rule that evaluates your IAM resources every 24 hours.</p> </li> <li> <p>You update your Lambda function to add additional conditions to your rule.</p> </li> <li> <p>Instead of waiting for the next periodic evaluation, you call the <code>StartConfigRulesEvaluation</code> API.</p> </li> <li> <p>AWS Config invokes your Lambda function and evaluates your IAM resources.</p> </li> <li> <p>Your custom rule will still run periodic evaluations every 24 hours.</p> </li> </ol></p>
    fn start_config_rules_evaluation(
        &self,
        input: StartConfigRulesEvaluationRequest,
    ) -> RusotoFuture<StartConfigRulesEvaluationResponse, StartConfigRulesEvaluationError>;

    /// <p>Starts recording configurations of the AWS resources you have selected to record in your AWS account.</p> <p>You must have created at least one delivery channel to successfully start the configuration recorder.</p>
    fn start_configuration_recorder(
        &self,
        input: StartConfigurationRecorderRequest,
    ) -> RusotoFuture<(), StartConfigurationRecorderError>;

    /// <p>Stops recording configurations of the AWS resources you have selected to record in your AWS account.</p>
    fn stop_configuration_recorder(
        &self,
        input: StopConfigurationRecorderRequest,
    ) -> RusotoFuture<(), StopConfigurationRecorderError>;
}
/// A client for the Config Service API.
pub struct ConfigServiceClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl ConfigServiceClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> ConfigServiceClient {
        ConfigServiceClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> ConfigServiceClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        ConfigServiceClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> ConfigService for ConfigServiceClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p><p>Returns the current configuration for one or more requested resources. The operation also returns a list of resources that are not processed in the current request. If there are no unprocessed resources, the operation returns an empty unprocessedResourceKeys list. </p> <note> <ul> <li> <p>The API does not return results for deleted resources.</p> </li> <li> <p> The API does not return any tags for the requested resources. This information is filtered out of the supplementaryConfiguration section of the API response.</p> </li> </ul> </note></p>
    fn batch_get_resource_config(
        &self,
        input: BatchGetResourceConfigRequest,
    ) -> RusotoFuture<BatchGetResourceConfigResponse, BatchGetResourceConfigError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StarlingDoveService.BatchGetResourceConfig");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<BatchGetResourceConfigResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(BatchGetResourceConfigError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the authorization granted to the specified configuration aggregator account in a specified region.</p>
    fn delete_aggregation_authorization(
        &self,
        input: DeleteAggregationAuthorizationRequest,
    ) -> RusotoFuture<(), DeleteAggregationAuthorizationError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DeleteAggregationAuthorization",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteAggregationAuthorizationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified AWS Config rule and all of its evaluation results.</p> <p>AWS Config sets the state of a rule to <code>DELETING</code> until the deletion is complete. You cannot update a rule while it is in this state. If you make a <code>PutConfigRule</code> or <code>DeleteConfigRule</code> request for the rule, you will receive a <code>ResourceInUseException</code>.</p> <p>You can check the state of a rule by using the <code>DescribeConfigRules</code> request.</p>
    fn delete_config_rule(
        &self,
        input: DeleteConfigRuleRequest,
    ) -> RusotoFuture<(), DeleteConfigRuleError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StarlingDoveService.DeleteConfigRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteConfigRuleError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified configuration aggregator and the aggregated data associated with the aggregator.</p>
    fn delete_configuration_aggregator(
        &self,
        input: DeleteConfigurationAggregatorRequest,
    ) -> RusotoFuture<(), DeleteConfigurationAggregatorError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DeleteConfigurationAggregator",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteConfigurationAggregatorError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the configuration recorder.</p> <p>After the configuration recorder is deleted, AWS Config will not record resource configuration changes until you create a new configuration recorder.</p> <p>This action does not delete the configuration information that was previously recorded. You will be able to access the previously recorded information by using the <code>GetResourceConfigHistory</code> action, but you will not be able to access this information in the AWS Config console until you create a new configuration recorder.</p>
    fn delete_configuration_recorder(
        &self,
        input: DeleteConfigurationRecorderRequest,
    ) -> RusotoFuture<(), DeleteConfigurationRecorderError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DeleteConfigurationRecorder",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteConfigurationRecorderError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the delivery channel.</p> <p>Before you can delete the delivery channel, you must stop the configuration recorder by using the <a>StopConfigurationRecorder</a> action.</p>
    fn delete_delivery_channel(
        &self,
        input: DeleteDeliveryChannelRequest,
    ) -> RusotoFuture<(), DeleteDeliveryChannelError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StarlingDoveService.DeleteDeliveryChannel");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteDeliveryChannelError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the evaluation results for the specified AWS Config rule. You can specify one AWS Config rule per request. After you delete the evaluation results, you can call the <a>StartConfigRulesEvaluation</a> API to start evaluating your AWS resources against the rule.</p>
    fn delete_evaluation_results(
        &self,
        input: DeleteEvaluationResultsRequest,
    ) -> RusotoFuture<DeleteEvaluationResultsResponse, DeleteEvaluationResultsError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DeleteEvaluationResults",
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

                    serde_json::from_str::<DeleteEvaluationResultsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteEvaluationResultsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes pending authorization requests for a specified aggregator account in a specified region.</p>
    fn delete_pending_aggregation_request(
        &self,
        input: DeletePendingAggregationRequestRequest,
    ) -> RusotoFuture<(), DeletePendingAggregationRequestError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DeletePendingAggregationRequest",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeletePendingAggregationRequestError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Schedules delivery of a configuration snapshot to the Amazon S3 bucket in the specified delivery channel. After the delivery has started, AWS Config sends the following notifications using an Amazon SNS topic that you have specified.</p> <ul> <li> <p>Notification of the start of the delivery.</p> </li> <li> <p>Notification of the completion of the delivery, if the delivery was successfully completed.</p> </li> <li> <p>Notification of delivery failure, if the delivery failed.</p> </li> </ul></p>
    fn deliver_config_snapshot(
        &self,
        input: DeliverConfigSnapshotRequest,
    ) -> RusotoFuture<DeliverConfigSnapshotResponse, DeliverConfigSnapshotError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StarlingDoveService.DeliverConfigSnapshot");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeliverConfigSnapshotResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeliverConfigSnapshotError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Returns a list of compliant and noncompliant rules with the number of resources for compliant and noncompliant rules. </p> <note> <p>The results can return an empty result page, but if you have a nextToken, the results are displayed on the next page.</p> </note></p>
    fn describe_aggregate_compliance_by_config_rules(
        &self,
        input: DescribeAggregateComplianceByConfigRulesRequest,
    ) -> RusotoFuture<
        DescribeAggregateComplianceByConfigRulesResponse,
        DescribeAggregateComplianceByConfigRulesError,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeAggregateComplianceByConfigRules",
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

                    serde_json::from_str::<DescribeAggregateComplianceByConfigRulesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeAggregateComplianceByConfigRulesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a list of authorizations granted to various aggregator accounts and regions.</p>
    fn describe_aggregation_authorizations(
        &self,
        input: DescribeAggregationAuthorizationsRequest,
    ) -> RusotoFuture<
        DescribeAggregationAuthorizationsResponse,
        DescribeAggregationAuthorizationsError,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeAggregationAuthorizations",
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

                    serde_json::from_str::<DescribeAggregationAuthorizationsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeAggregationAuthorizationsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Indicates whether the specified AWS Config rules are compliant. If a rule is noncompliant, this action returns the number of AWS resources that do not comply with the rule.</p> <p>A rule is compliant if all of the evaluated resources comply with it. It is noncompliant if any of these resources do not comply.</p> <p>If AWS Config has no current evaluation results for the rule, it returns <code>INSUFFICIENT<em>DATA</code>. This result might indicate one of the following conditions:</p> <ul> <li> <p>AWS Config has never invoked an evaluation for the rule. To check whether it has, use the <code>DescribeConfigRuleEvaluationStatus</code> action to get the <code>LastSuccessfulInvocationTime</code> and <code>LastFailedInvocationTime</code>.</p> </li> <li> <p>The rule&#39;s AWS Lambda function is failing to send evaluation results to AWS Config. Verify that the role you assigned to your configuration recorder includes the <code>config:PutEvaluations</code> permission. If the rule is a custom rule, verify that the AWS Lambda execution role includes the <code>config:PutEvaluations</code> permission.</p> </li> <li> <p>The rule&#39;s AWS Lambda function has returned <code>NOT</em>APPLICABLE</code> for all evaluation results. This can occur if the resources were deleted or removed from the rule&#39;s scope.</p> </li> </ul></p>
    fn describe_compliance_by_config_rule(
        &self,
        input: DescribeComplianceByConfigRuleRequest,
    ) -> RusotoFuture<DescribeComplianceByConfigRuleResponse, DescribeComplianceByConfigRuleError>
    {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeComplianceByConfigRule",
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

                    serde_json::from_str::<DescribeComplianceByConfigRuleResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeComplianceByConfigRuleError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Indicates whether the specified AWS resources are compliant. If a resource is noncompliant, this action returns the number of AWS Config rules that the resource does not comply with.</p> <p>A resource is compliant if it complies with all the AWS Config rules that evaluate it. It is noncompliant if it does not comply with one or more of these rules.</p> <p>If AWS Config has no current evaluation results for the resource, it returns <code>INSUFFICIENT<em>DATA</code>. This result might indicate one of the following conditions about the rules that evaluate the resource:</p> <ul> <li> <p>AWS Config has never invoked an evaluation for the rule. To check whether it has, use the <code>DescribeConfigRuleEvaluationStatus</code> action to get the <code>LastSuccessfulInvocationTime</code> and <code>LastFailedInvocationTime</code>.</p> </li> <li> <p>The rule&#39;s AWS Lambda function is failing to send evaluation results to AWS Config. Verify that the role that you assigned to your configuration recorder includes the <code>config:PutEvaluations</code> permission. If the rule is a custom rule, verify that the AWS Lambda execution role includes the <code>config:PutEvaluations</code> permission.</p> </li> <li> <p>The rule&#39;s AWS Lambda function has returned <code>NOT</em>APPLICABLE</code> for all evaluation results. This can occur if the resources were deleted or removed from the rule&#39;s scope.</p> </li> </ul></p>
    fn describe_compliance_by_resource(
        &self,
        input: DescribeComplianceByResourceRequest,
    ) -> RusotoFuture<DescribeComplianceByResourceResponse, DescribeComplianceByResourceError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeComplianceByResource",
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

                    serde_json::from_str::<DescribeComplianceByResourceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeComplianceByResourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns status information for each of your AWS managed Config rules. The status includes information such as the last time AWS Config invoked the rule, the last time AWS Config failed to invoke the rule, and the related error for the last failure.</p>
    fn describe_config_rule_evaluation_status(
        &self,
        input: DescribeConfigRuleEvaluationStatusRequest,
    ) -> RusotoFuture<
        DescribeConfigRuleEvaluationStatusResponse,
        DescribeConfigRuleEvaluationStatusError,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeConfigRuleEvaluationStatus",
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

                    serde_json::from_str::<DescribeConfigRuleEvaluationStatusResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeConfigRuleEvaluationStatusError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns details about your AWS Config rules.</p>
    fn describe_config_rules(
        &self,
        input: DescribeConfigRulesRequest,
    ) -> RusotoFuture<DescribeConfigRulesResponse, DescribeConfigRulesError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StarlingDoveService.DescribeConfigRules");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeConfigRulesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeConfigRulesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns status information for sources within an aggregator. The status includes information about the last time AWS Config aggregated data from source accounts or AWS Config failed to aggregate data from source accounts with the related error code or message. </p>
    fn describe_configuration_aggregator_sources_status(
        &self,
        input: DescribeConfigurationAggregatorSourcesStatusRequest,
    ) -> RusotoFuture<
        DescribeConfigurationAggregatorSourcesStatusResponse,
        DescribeConfigurationAggregatorSourcesStatusError,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeConfigurationAggregatorSourcesStatus",
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

                    serde_json::from_str::<DescribeConfigurationAggregatorSourcesStatusResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(
                        DescribeConfigurationAggregatorSourcesStatusError::from_body(
                            String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                        ),
                    )
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns the details of one or more configuration aggregators. If the configuration aggregator is not specified, this action returns the details for all the configuration aggregators associated with the account. </p>
    fn describe_configuration_aggregators(
        &self,
        input: DescribeConfigurationAggregatorsRequest,
    ) -> RusotoFuture<DescribeConfigurationAggregatorsResponse, DescribeConfigurationAggregatorsError>
    {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeConfigurationAggregators",
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

                    serde_json::from_str::<DescribeConfigurationAggregatorsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeConfigurationAggregatorsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Returns the current status of the specified configuration recorder. If a configuration recorder is not specified, this action returns the status of all configuration recorders associated with the account.</p> <note> <p>Currently, you can specify only one configuration recorder per region in your account.</p> </note></p>
    fn describe_configuration_recorder_status(
        &self,
        input: DescribeConfigurationRecorderStatusRequest,
    ) -> RusotoFuture<
        DescribeConfigurationRecorderStatusResponse,
        DescribeConfigurationRecorderStatusError,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeConfigurationRecorderStatus",
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

                    serde_json::from_str::<DescribeConfigurationRecorderStatusResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeConfigurationRecorderStatusError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Returns the details for the specified configuration recorders. If the configuration recorder is not specified, this action returns the details for all configuration recorders associated with the account.</p> <note> <p>Currently, you can specify only one configuration recorder per region in your account.</p> </note></p>
    fn describe_configuration_recorders(
        &self,
        input: DescribeConfigurationRecordersRequest,
    ) -> RusotoFuture<DescribeConfigurationRecordersResponse, DescribeConfigurationRecordersError>
    {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeConfigurationRecorders",
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

                    serde_json::from_str::<DescribeConfigurationRecordersResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeConfigurationRecordersError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Returns the current status of the specified delivery channel. If a delivery channel is not specified, this action returns the current status of all delivery channels associated with the account.</p> <note> <p>Currently, you can specify only one delivery channel per region in your account.</p> </note></p>
    fn describe_delivery_channel_status(
        &self,
        input: DescribeDeliveryChannelStatusRequest,
    ) -> RusotoFuture<DescribeDeliveryChannelStatusResponse, DescribeDeliveryChannelStatusError>
    {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeDeliveryChannelStatus",
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

                    serde_json::from_str::<DescribeDeliveryChannelStatusResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeDeliveryChannelStatusError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Returns details about the specified delivery channel. If a delivery channel is not specified, this action returns the details of all delivery channels associated with the account.</p> <note> <p>Currently, you can specify only one delivery channel per region in your account.</p> </note></p>
    fn describe_delivery_channels(
        &self,
        input: DescribeDeliveryChannelsRequest,
    ) -> RusotoFuture<DescribeDeliveryChannelsResponse, DescribeDeliveryChannelsError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeDeliveryChannels",
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

                    serde_json::from_str::<DescribeDeliveryChannelsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeDeliveryChannelsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a list of all pending aggregation requests.</p>
    fn describe_pending_aggregation_requests(
        &self,
        input: DescribePendingAggregationRequestsRequest,
    ) -> RusotoFuture<
        DescribePendingAggregationRequestsResponse,
        DescribePendingAggregationRequestsError,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribePendingAggregationRequests",
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

                    serde_json::from_str::<DescribePendingAggregationRequestsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribePendingAggregationRequestsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Returns the evaluation results for the specified AWS Config rule for a specific resource in a rule. The results indicate which AWS resources were evaluated by the rule, when each resource was last evaluated, and whether each resource complies with the rule. </p> <note> <p>The results can return an empty result page. But if you have a nextToken, the results are displayed on the next page.</p> </note></p>
    fn get_aggregate_compliance_details_by_config_rule(
        &self,
        input: GetAggregateComplianceDetailsByConfigRuleRequest,
    ) -> RusotoFuture<
        GetAggregateComplianceDetailsByConfigRuleResponse,
        GetAggregateComplianceDetailsByConfigRuleError,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.GetAggregateComplianceDetailsByConfigRule",
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

                    serde_json::from_str::<GetAggregateComplianceDetailsByConfigRuleResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetAggregateComplianceDetailsByConfigRuleError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Returns the number of compliant and noncompliant rules for one or more accounts and regions in an aggregator.</p> <note> <p>The results can return an empty result page, but if you have a nextToken, the results are displayed on the next page.</p> </note></p>
    fn get_aggregate_config_rule_compliance_summary(
        &self,
        input: GetAggregateConfigRuleComplianceSummaryRequest,
    ) -> RusotoFuture<
        GetAggregateConfigRuleComplianceSummaryResponse,
        GetAggregateConfigRuleComplianceSummaryError,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.GetAggregateConfigRuleComplianceSummary",
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

                    serde_json::from_str::<GetAggregateConfigRuleComplianceSummaryResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetAggregateConfigRuleComplianceSummaryError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns the evaluation results for the specified AWS Config rule. The results indicate which AWS resources were evaluated by the rule, when each resource was last evaluated, and whether each resource complies with the rule.</p>
    fn get_compliance_details_by_config_rule(
        &self,
        input: GetComplianceDetailsByConfigRuleRequest,
    ) -> RusotoFuture<GetComplianceDetailsByConfigRuleResponse, GetComplianceDetailsByConfigRuleError>
    {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.GetComplianceDetailsByConfigRule",
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

                    serde_json::from_str::<GetComplianceDetailsByConfigRuleResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetComplianceDetailsByConfigRuleError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns the evaluation results for the specified AWS resource. The results indicate which AWS Config rules were used to evaluate the resource, when each rule was last used, and whether the resource complies with each rule.</p>
    fn get_compliance_details_by_resource(
        &self,
        input: GetComplianceDetailsByResourceRequest,
    ) -> RusotoFuture<GetComplianceDetailsByResourceResponse, GetComplianceDetailsByResourceError>
    {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.GetComplianceDetailsByResource",
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

                    serde_json::from_str::<GetComplianceDetailsByResourceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetComplianceDetailsByResourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns the number of AWS Config rules that are compliant and noncompliant, up to a maximum of 25 for each.</p>
    fn get_compliance_summary_by_config_rule(
        &self,
    ) -> RusotoFuture<GetComplianceSummaryByConfigRuleResponse, GetComplianceSummaryByConfigRuleError>
    {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.GetComplianceSummaryByConfigRule",
        );
        request.set_payload(Some(b"{}".to_vec()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetComplianceSummaryByConfigRuleResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetComplianceSummaryByConfigRuleError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns the number of resources that are compliant and the number that are noncompliant. You can specify one or more resource types to get these numbers for each resource type. The maximum number returned is 100.</p>
    fn get_compliance_summary_by_resource_type(
        &self,
        input: GetComplianceSummaryByResourceTypeRequest,
    ) -> RusotoFuture<
        GetComplianceSummaryByResourceTypeResponse,
        GetComplianceSummaryByResourceTypeError,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.GetComplianceSummaryByResourceType",
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

                    serde_json::from_str::<GetComplianceSummaryByResourceTypeResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetComplianceSummaryByResourceTypeError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Returns the resource types, the number of each resource type, and the total number of resources that AWS Config is recording in this region for your AWS account. </p> <p class="title"> <b>Example</b> </p> <ol> <li> <p>AWS Config is recording three resource types in the US East (Ohio) Region for your account: 25 EC2 instances, 20 IAM users, and 15 S3 buckets.</p> </li> <li> <p>You make a call to the <code>GetDiscoveredResourceCounts</code> action and specify that you want all resource types. </p> </li> <li> <p>AWS Config returns the following:</p> <ul> <li> <p>The resource types (EC2 instances, IAM users, and S3 buckets).</p> </li> <li> <p>The number of each resource type (25, 20, and 15).</p> </li> <li> <p>The total number of all resources (60).</p> </li> </ul> </li> </ol> <p>The response is paginated. By default, AWS Config lists 100 <a>ResourceCount</a> objects on each page. You can customize this number with the <code>limit</code> parameter. The response includes a <code>nextToken</code> string. To get the next page of results, run the request again and specify the string for the <code>nextToken</code> parameter.</p> <note> <p>If you make a call to the <a>GetDiscoveredResourceCounts</a> action, you might not immediately receive resource counts in the following situations:</p> <ul> <li> <p>You are a new AWS Config customer.</p> </li> <li> <p>You just enabled resource recording.</p> </li> </ul> <p>It might take a few minutes for AWS Config to record and count your resources. Wait a few minutes and then retry the <a>GetDiscoveredResourceCounts</a> action. </p> </note></p>
    fn get_discovered_resource_counts(
        &self,
        input: GetDiscoveredResourceCountsRequest,
    ) -> RusotoFuture<GetDiscoveredResourceCountsResponse, GetDiscoveredResourceCountsError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.GetDiscoveredResourceCounts",
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

                    serde_json::from_str::<GetDiscoveredResourceCountsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetDiscoveredResourceCountsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Returns a list of configuration items for the specified resource. The list contains details about each state of the resource during the specified time interval.</p> <p>The response is paginated. By default, AWS Config returns a limit of 10 configuration items per page. You can customize this number with the <code>limit</code> parameter. The response includes a <code>nextToken</code> string. To get the next page of results, run the request again and specify the string for the <code>nextToken</code> parameter.</p> <note> <p>Each call to the API is limited to span a duration of seven days. It is likely that the number of records returned is smaller than the specified <code>limit</code>. In such cases, you can make another call, using the <code>nextToken</code>.</p> </note></p>
    fn get_resource_config_history(
        &self,
        input: GetResourceConfigHistoryRequest,
    ) -> RusotoFuture<GetResourceConfigHistoryResponse, GetResourceConfigHistoryError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.GetResourceConfigHistory",
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

                    serde_json::from_str::<GetResourceConfigHistoryResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetResourceConfigHistoryError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Accepts a resource type and returns a list of resource identifiers for the resources of that type. A resource identifier includes the resource type, ID, and (if available) the custom resource name. The results consist of resources that AWS Config has discovered, including those that AWS Config is not currently recording. You can narrow the results to include only resources that have specific resource IDs or a resource name.</p> <note> <p>You can specify either resource IDs or a resource name, but not both, in the same request.</p> </note> <p>The response is paginated. By default, AWS Config lists 100 resource identifiers on each page. You can customize this number with the <code>limit</code> parameter. The response includes a <code>nextToken</code> string. To get the next page of results, run the request again and specify the string for the <code>nextToken</code> parameter.</p>
    fn list_discovered_resources(
        &self,
        input: ListDiscoveredResourcesRequest,
    ) -> RusotoFuture<ListDiscoveredResourcesResponse, ListDiscoveredResourcesError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.ListDiscoveredResources",
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

                    serde_json::from_str::<ListDiscoveredResourcesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListDiscoveredResourcesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Authorizes the aggregator account and region to collect data from the source account and region. </p>
    fn put_aggregation_authorization(
        &self,
        input: PutAggregationAuthorizationRequest,
    ) -> RusotoFuture<PutAggregationAuthorizationResponse, PutAggregationAuthorizationError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.PutAggregationAuthorization",
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

                    serde_json::from_str::<PutAggregationAuthorizationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PutAggregationAuthorizationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Adds or updates an AWS Config rule for evaluating whether your AWS resources comply with your desired configurations.</p> <p>You can use this action for custom AWS Config rules and AWS managed Config rules. A custom AWS Config rule is a rule that you develop and maintain. An AWS managed Config rule is a customizable, predefined rule that AWS Config provides.</p> <p>If you are adding a new custom AWS Config rule, you must first create the AWS Lambda function that the rule invokes to evaluate your resources. When you use the <code>PutConfigRule</code> action to add the rule to AWS Config, you must specify the Amazon Resource Name (ARN) that AWS Lambda assigns to the function. Specify the ARN for the <code>SourceIdentifier</code> key. This key is part of the <code>Source</code> object, which is part of the <code>ConfigRule</code> object. </p> <p>If you are adding an AWS managed Config rule, specify the rule's identifier for the <code>SourceIdentifier</code> key. To reference AWS managed Config rule identifiers, see <a href="http://docs.aws.amazon.com/config/latest/developerguide/evaluate-config_use-managed-rules.html">About AWS Managed Config Rules</a>.</p> <p>For any new rule that you add, specify the <code>ConfigRuleName</code> in the <code>ConfigRule</code> object. Do not specify the <code>ConfigRuleArn</code> or the <code>ConfigRuleId</code>. These values are generated by AWS Config for new rules.</p> <p>If you are updating a rule that you added previously, you can specify the rule by <code>ConfigRuleName</code>, <code>ConfigRuleId</code>, or <code>ConfigRuleArn</code> in the <code>ConfigRule</code> data type that you use in this request.</p> <p>The maximum number of rules that AWS Config supports is 50.</p> <p>For information about requesting a rule limit increase, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_config">AWS Config Limits</a> in the <i>AWS General Reference Guide</i>.</p> <p>For more information about developing and using AWS Config rules, see <a href="http://docs.aws.amazon.com/config/latest/developerguide/evaluate-config.html">Evaluating AWS Resource Configurations with AWS Config</a> in the <i>AWS Config Developer Guide</i>.</p>
    fn put_config_rule(&self, input: PutConfigRuleRequest) -> RusotoFuture<(), PutConfigRuleError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StarlingDoveService.PutConfigRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PutConfigRuleError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Creates and updates the configuration aggregator with the selected source accounts and regions.</p> <note> <p>AWS Config should be enabled in accounts and regions you want to aggreagate.</p> </note></p>
    fn put_configuration_aggregator(
        &self,
        input: PutConfigurationAggregatorRequest,
    ) -> RusotoFuture<PutConfigurationAggregatorResponse, PutConfigurationAggregatorError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.PutConfigurationAggregator",
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

                    serde_json::from_str::<PutConfigurationAggregatorResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PutConfigurationAggregatorError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Creates a new configuration recorder to record the selected resource configurations.</p> <p>You can use this action to change the role <code>roleARN</code> or the <code>recordingGroup</code> of an existing recorder. To change the role, call the action on the existing configuration recorder and specify a role.</p> <note> <p>Currently, you can specify only one configuration recorder per region in your account.</p> <p>If <code>ConfigurationRecorder</code> does not have the <b>recordingGroup</b> parameter specified, the default is to record all supported resource types.</p> </note></p>
    fn put_configuration_recorder(
        &self,
        input: PutConfigurationRecorderRequest,
    ) -> RusotoFuture<(), PutConfigurationRecorderError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.PutConfigurationRecorder",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PutConfigurationRecorderError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Creates a delivery channel object to deliver configuration information to an Amazon S3 bucket and Amazon SNS topic.</p> <p>Before you can create a delivery channel, you must create a configuration recorder.</p> <p>You can use this action to change the Amazon S3 bucket or an Amazon SNS topic of the existing delivery channel. To change the Amazon S3 bucket or an Amazon SNS topic, call this action and specify the changed values for the S3 bucket and the SNS topic. If you specify a different value for either the S3 bucket or the SNS topic, this action will keep the existing value for the parameter that is not changed.</p> <note> <p>You can have only one delivery channel per region in your account.</p> </note></p>
    fn put_delivery_channel(
        &self,
        input: PutDeliveryChannelRequest,
    ) -> RusotoFuture<(), PutDeliveryChannelError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StarlingDoveService.PutDeliveryChannel");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PutDeliveryChannelError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Used by an AWS Lambda function to deliver evaluation results to AWS Config. This action is required in every AWS Lambda function that is invoked by an AWS Config rule.</p>
    fn put_evaluations(
        &self,
        input: PutEvaluationsRequest,
    ) -> RusotoFuture<PutEvaluationsResponse, PutEvaluationsError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StarlingDoveService.PutEvaluations");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PutEvaluationsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PutEvaluationsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Runs an on-demand evaluation for the specified AWS Config rules against the last known configuration state of the resources. Use <code>StartConfigRulesEvaluation</code> when you want to test that a rule you updated is working as expected. <code>StartConfigRulesEvaluation</code> does not re-record the latest configuration state for your resources. It re-runs an evaluation against the last known state of your resources. </p> <p>You can specify up to 25 AWS Config rules per request. </p> <p>An existing <code>StartConfigRulesEvaluation</code> call for the specified rules must complete before you can call the API again. If you chose to have AWS Config stream to an Amazon SNS topic, you will receive a <code>ConfigRuleEvaluationStarted</code> notification when the evaluation starts.</p> <note> <p>You don&#39;t need to call the <code>StartConfigRulesEvaluation</code> API to run an evaluation for a new rule. When you create a rule, AWS Config evaluates your resources against the rule automatically. </p> </note> <p>The <code>StartConfigRulesEvaluation</code> API is useful if you want to run on-demand evaluations, such as the following example:</p> <ol> <li> <p>You have a custom rule that evaluates your IAM resources every 24 hours.</p> </li> <li> <p>You update your Lambda function to add additional conditions to your rule.</p> </li> <li> <p>Instead of waiting for the next periodic evaluation, you call the <code>StartConfigRulesEvaluation</code> API.</p> </li> <li> <p>AWS Config invokes your Lambda function and evaluates your IAM resources.</p> </li> <li> <p>Your custom rule will still run periodic evaluations every 24 hours.</p> </li> </ol></p>
    fn start_config_rules_evaluation(
        &self,
        input: StartConfigRulesEvaluationRequest,
    ) -> RusotoFuture<StartConfigRulesEvaluationResponse, StartConfigRulesEvaluationError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.StartConfigRulesEvaluation",
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

                    serde_json::from_str::<StartConfigRulesEvaluationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(StartConfigRulesEvaluationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Starts recording configurations of the AWS resources you have selected to record in your AWS account.</p> <p>You must have created at least one delivery channel to successfully start the configuration recorder.</p>
    fn start_configuration_recorder(
        &self,
        input: StartConfigurationRecorderRequest,
    ) -> RusotoFuture<(), StartConfigurationRecorderError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.StartConfigurationRecorder",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(StartConfigurationRecorderError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Stops recording configurations of the AWS resources you have selected to record in your AWS account.</p>
    fn stop_configuration_recorder(
        &self,
        input: StopConfigurationRecorderRequest,
    ) -> RusotoFuture<(), StopConfigurationRecorderError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.StopConfigurationRecorder",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(StopConfigurationRecorderError::from_body(
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
