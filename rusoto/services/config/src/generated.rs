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
/// <p>A collection of accounts and regions.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccountAggregationSource {
    /// <p>The 12-digit account ID of the account being aggregated. </p>
    #[serde(rename = "AccountIds")]
    pub account_ids: Vec<String>,
    /// <p>If true, aggregate existing AWS Config regions and future regions.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>The details that identify a resource that is collected by AWS Config aggregator, including the resource type, ID, (if available) the custom resource name, the source account, and source region.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AggregateResourceIdentifier {
    /// <p>The ID of the AWS resource.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>The name of the AWS resource.</p>
    #[serde(rename = "ResourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    /// <p>The type of the AWS resource.</p>
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// <p>The 12-digit account ID of the source account.</p>
    #[serde(rename = "SourceAccountId")]
    pub source_account_id: String,
    /// <p>The source region where data is aggregated.</p>
    #[serde(rename = "SourceRegion")]
    pub source_region: String,
}

/// <p>The current sync status between the source and the aggregator account.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BaseConfigurationItem {
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchGetAggregateResourceConfigRequest {
    /// <p>The name of the configuration aggregator.</p>
    #[serde(rename = "ConfigurationAggregatorName")]
    pub configuration_aggregator_name: String,
    /// <p>A list of aggregate ResourceIdentifiers objects. </p>
    #[serde(rename = "ResourceIdentifiers")]
    pub resource_identifiers: Vec<AggregateResourceIdentifier>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchGetAggregateResourceConfigResponse {
    /// <p>A list that contains the current configuration of one or more resources.</p>
    #[serde(rename = "BaseConfigurationItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_configuration_items: Option<Vec<BaseConfigurationItem>>,
    /// <p>A list of resource identifiers that were not processed with current scope. The list is empty if all the resources are processed.</p>
    #[serde(rename = "UnprocessedResourceIdentifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_resource_identifiers: Option<Vec<AggregateResourceIdentifier>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchGetResourceConfigRequest {
    /// <p>A list of resource keys to be processed with the current request. Each element in the list consists of the resource type and resource ID.</p>
    #[serde(rename = "resourceKeys")]
    pub resource_keys: Vec<ResourceKey>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>An AWS Config rule represents an AWS Lambda function that you create for a custom rule or a predefined function for an AWS managed rule. The function evaluates configuration items to assess whether your AWS resources comply with your desired configurations. This function can run when AWS Config detects a configuration change to an AWS resource and at a periodic frequency that you choose (for example, every 24 hours).</p> <note> <p>You can use the AWS CLI and AWS SDKs if you want to create a rule that triggers evaluations for your resources when AWS Config delivers the configuration snapshot. For more information, see <a>ConfigSnapshotDeliveryProperties</a>.</p> </note> <p>For more information about developing and using AWS Config rules, see <a href="https://docs.aws.amazon.com/config/latest/developerguide/evaluate-config.html">Evaluating AWS Resource Configurations with AWS Config</a> in the <i>AWS Config Developer Guide</i>.</p>
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
    /// <p><p>Service principal name of the service that created the rule.</p> <note> <p>The field is populated only if the service linked rule is created by a service. The field is empty if you create your own rule.</p> </note></p>
    #[serde(rename = "CreatedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>Provides options for how often AWS Config delivers configuration snapshots to the Amazon S3 bucket in your delivery channel.</p> <p>The frequency for a rule that triggers evaluations for your resources when AWS Config delivers the configuration snapshot is set by one of two values, depending on which is less frequent:</p> <ul> <li> <p>The value for the <code>deliveryFrequency</code> parameter within the delivery channel configuration, which sets how often AWS Config delivers configuration snapshots. This value also sets how often AWS Config invokes evaluations for AWS Config rules.</p> </li> <li> <p>The value for the <code>MaximumExecutionFrequency</code> parameter, which sets the maximum frequency with which AWS Config invokes evaluations for the rule. For more information, see <a>ConfigRule</a>.</p> </li> </ul> <p>If the <code>deliveryFrequency</code> value is less frequent than the <code>MaximumExecutionFrequency</code> value for a rule, AWS Config invokes the rule only as often as the <code>deliveryFrequency</code> value.</p> <ol> <li> <p>For example, you want your rule to run evaluations when AWS Config delivers the configuration snapshot.</p> </li> <li> <p>You specify the <code>MaximumExecutionFrequency</code> value for <code>Six_Hours</code>. </p> </li> <li> <p>You then specify the delivery channel <code>deliveryFrequency</code> value for <code>TwentyFour_Hours</code>.</p> </li> <li> <p>Because the value for <code>deliveryFrequency</code> is less frequent than <code>MaximumExecutionFrequency</code>, AWS Config invokes evaluations for the rule every 24 hours. </p> </li> </ol> <p>You should set the <code>MaximumExecutionFrequency</code> value to be at least as frequent as the <code>deliveryFrequency</code> value. You can view the <code>deliveryFrequency</code> value by using the <code>DescribeDeliveryChannnels</code> action.</p> <p>To update the <code>deliveryFrequency</code> with which AWS Config delivers your configuration snapshots, use the <code>PutDeliveryChannel</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigSnapshotDeliveryProperties {
    /// <p>The frequency with which AWS Config delivers configuration snapshots.</p>
    #[serde(rename = "deliveryFrequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_frequency: Option<String>,
}

/// <p>A list that contains the status of the delivery of the configuration stream notification to the Amazon SNS topic.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConfigStreamDeliveryInfo {
    /// <p>The error code from the last attempted delivery.</p>
    #[serde(rename = "lastErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_code: Option<String>,
    /// <p>The error message from the last attempted delivery.</p>
    #[serde(rename = "lastErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_message: Option<String>,
    /// <p>Status of the last attempted delivery.</p> <p> <b>Note</b> Providing an SNS topic on a <a href="https://docs.aws.amazon.com/config/latest/APIReference/API_DeliveryChannel.html">DeliveryChannel</a> for AWS Config is optional. If the SNS delivery is turned off, the last status will be <b>Not_Applicable</b>.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConfigurationItem {
    /// <p>The 12-digit AWS account ID associated with the resource.</p>
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>accoun</p>
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
    /// <p>A list of CloudTrail event IDs.</p> <p>A populated field indicates that the current configuration was initiated by the events recorded in the CloudTrail log. For more information about CloudTrail, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/what_is_cloud_trail_top_level.html">What Is AWS CloudTrail</a>.</p> <p>An empty field indicates that the current configuration was not initiated by any event. As of Version 1.3, the relatedEvents field is empty. You can access the <a href="https://docs.aws.amazon.com/awscloudtrail/latest/APIReference/API_LookupEvents.html">LookupEvents API</a> in the <i>AWS CloudTrail API Reference</i> to retrieve the events for the resource.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>Filters the conformance pack by compliance types and AWS Config rule names.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ConformancePackComplianceFilters {
    /// <p>Filters the results by compliance.</p> <p>The allowed values are <code>COMPLIANT</code> and <code>NON_COMPLIANT</code>.</p>
    #[serde(rename = "ComplianceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_type: Option<String>,
    /// <p>Filters the results by AWS Config rule names.</p>
    #[serde(rename = "ConfigRuleNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_names: Option<Vec<String>>,
}

/// <p>Summary includes the name and status of the conformance pack.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConformancePackComplianceSummary {
    /// <p>The status of the conformance pack. The allowed values are COMPLIANT and NON_COMPLIANT. </p>
    #[serde(rename = "ConformancePackComplianceStatus")]
    pub conformance_pack_compliance_status: String,
    /// <p>The name of the conformance pack name.</p>
    #[serde(rename = "ConformancePackName")]
    pub conformance_pack_name: String,
}

/// <p>Returns details of a conformance pack. A conformance pack is a collection of AWS Config rules and remediation actions that can be easily deployed in an account and a region.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConformancePackDetail {
    /// <p>Amazon Resource Name (ARN) of the conformance pack.</p>
    #[serde(rename = "ConformancePackArn")]
    pub conformance_pack_arn: String,
    /// <p>ID of the conformance pack.</p>
    #[serde(rename = "ConformancePackId")]
    pub conformance_pack_id: String,
    /// <p>A list of <code>ConformancePackInputParameter</code> objects.</p>
    #[serde(rename = "ConformancePackInputParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conformance_pack_input_parameters: Option<Vec<ConformancePackInputParameter>>,
    /// <p>Name of the conformance pack.</p>
    #[serde(rename = "ConformancePackName")]
    pub conformance_pack_name: String,
    /// <p>AWS service that created the conformance pack.</p>
    #[serde(rename = "CreatedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// <p>Conformance pack template that is used to create a pack. The delivery bucket name should start with awsconfigconforms. For example: "Resource": "arn:aws:s3:::your_bucket_name/*".</p>
    #[serde(rename = "DeliveryS3Bucket")]
    pub delivery_s3_bucket: String,
    /// <p>The prefix for the Amazon S3 bucket.</p>
    #[serde(rename = "DeliveryS3KeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_s3_key_prefix: Option<String>,
    /// <p>Last time when conformation pack update was requested. </p>
    #[serde(rename = "LastUpdateRequestedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_requested_time: Option<f64>,
}

/// <p>Filters a conformance pack by AWS Config rule names, compliance types, AWS resource types, and resource IDs.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ConformancePackEvaluationFilters {
    /// <p>Filters the results by compliance.</p> <p>The allowed values are <code>COMPLIANT</code> and <code>NON_COMPLIANT</code>.</p>
    #[serde(rename = "ComplianceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_type: Option<String>,
    /// <p>Filters the results by AWS Config rule names.</p>
    #[serde(rename = "ConfigRuleNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_names: Option<Vec<String>>,
    /// <p><p>Filters the results by resource IDs.</p> <note> <p>This is valid only when you provide resource type. If there is no resource type, you will see an error.</p> </note></p>
    #[serde(rename = "ResourceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_ids: Option<Vec<String>>,
    /// <p>Filters the results by the resource type (for example, <code>"AWS::EC2::Instance"</code>). </p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

/// <p>The details of a conformance pack evaluation. Provides AWS Config rule and AWS resource type that was evaluated, the compliance of the conformance pack, related time stamps, and supplementary information. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConformancePackEvaluationResult {
    /// <p>Supplementary information about how the evaluation determined the compliance. </p>
    #[serde(rename = "Annotation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation: Option<String>,
    /// <p>The compliance type. The allowed values are <code>COMPLIANT</code> and <code>NON_COMPLIANT</code>. </p>
    #[serde(rename = "ComplianceType")]
    pub compliance_type: String,
    /// <p>The time when AWS Config rule evaluated AWS resource.</p>
    #[serde(rename = "ConfigRuleInvokedTime")]
    pub config_rule_invoked_time: f64,
    #[serde(rename = "EvaluationResultIdentifier")]
    pub evaluation_result_identifier: EvaluationResultIdentifier,
    /// <p>The time when AWS Config recorded the evaluation result. </p>
    #[serde(rename = "ResultRecordedTime")]
    pub result_recorded_time: f64,
}

/// <p>Input parameters in the form of key-value pairs for the conformance pack, both of which you define. Keys can have a maximum character length of 128 characters, and values can have a maximum length of 256 characters.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConformancePackInputParameter {
    /// <p>One part of a key-value pair.</p>
    #[serde(rename = "ParameterName")]
    pub parameter_name: String,
    /// <p>Another part of the key-value pair. </p>
    #[serde(rename = "ParameterValue")]
    pub parameter_value: String,
}

/// <p>Compliance information of one or more AWS Config rules within a conformance pack. You can filter using AWS Config rule names and compliance types.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConformancePackRuleCompliance {
    /// <p>Compliance of the AWS Config rule</p> <p>The allowed values are <code>COMPLIANT</code> and <code>NON_COMPLIANT</code>.</p>
    #[serde(rename = "ComplianceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_type: Option<String>,
    /// <p>Name of the config rule.</p>
    #[serde(rename = "ConfigRuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_name: Option<String>,
}

/// <p>Status details of a conformance pack.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConformancePackStatusDetail {
    /// <p>Amazon Resource Name (ARN) of comformance pack.</p>
    #[serde(rename = "ConformancePackArn")]
    pub conformance_pack_arn: String,
    /// <p>ID of the conformance pack.</p>
    #[serde(rename = "ConformancePackId")]
    pub conformance_pack_id: String,
    /// <p>Name of the conformance pack.</p>
    #[serde(rename = "ConformancePackName")]
    pub conformance_pack_name: String,
    /// <p><p>Indicates deployment status of conformance pack.</p> <p>AWS Config sets the state of the conformance pack to:</p> <ul> <li> <p>CREATE<em>IN</em>PROGRESS when a conformance pack creation is in progress for an account.</p> </li> <li> <p>CREATE<em>COMPLETE when a conformance pack has been successfully created in your account.</p> </li> <li> <p>CREATE</em>FAILED when a conformance pack creation failed in your account.</p> </li> <li> <p>DELETE<em>IN</em>PROGRESS when a conformance pack deletion is in progress. </p> </li> <li> <p>DELETE_FAILED when a conformance pack deletion failed in your account.</p> </li> </ul></p>
    #[serde(rename = "ConformancePackState")]
    pub conformance_pack_state: String,
    /// <p>The reason of conformance pack creation failure.</p>
    #[serde(rename = "ConformancePackStatusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conformance_pack_status_reason: Option<String>,
    /// <p>Last time when conformation pack creation and update was successful.</p>
    #[serde(rename = "LastUpdateCompletedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_completed_time: Option<f64>,
    /// <p>Last time when conformation pack creation and update was requested.</p>
    #[serde(rename = "LastUpdateRequestedTime")]
    pub last_update_requested_time: f64,
    /// <p>Amazon Resource Name (ARN) of AWS CloudFormation stack. </p>
    #[serde(rename = "StackArn")]
    pub stack_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteConfigRuleRequest {
    /// <p>The name of the AWS Config rule that you want to delete.</p>
    #[serde(rename = "ConfigRuleName")]
    pub config_rule_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteConfigurationAggregatorRequest {
    /// <p>The name of the configuration aggregator.</p>
    #[serde(rename = "ConfigurationAggregatorName")]
    pub configuration_aggregator_name: String,
}

/// <p>The request object for the <code>DeleteConfigurationRecorder</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteConfigurationRecorderRequest {
    /// <p>The name of the configuration recorder to be deleted. You can retrieve the name of your configuration recorder by using the <code>DescribeConfigurationRecorders</code> action.</p>
    #[serde(rename = "ConfigurationRecorderName")]
    pub configuration_recorder_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteConformancePackRequest {
    /// <p>Name of the conformance pack you want to delete.</p>
    #[serde(rename = "ConformancePackName")]
    pub conformance_pack_name: String,
}

/// <p>The input for the <a>DeleteDeliveryChannel</a> action. The action accepts the following data, in JSON format. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDeliveryChannelRequest {
    /// <p>The name of the delivery channel to delete.</p>
    #[serde(rename = "DeliveryChannelName")]
    pub delivery_channel_name: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteEvaluationResultsRequest {
    /// <p>The name of the AWS Config rule for which you want to delete the evaluation results.</p>
    #[serde(rename = "ConfigRuleName")]
    pub config_rule_name: String,
}

/// <p>The output when you delete the evaluation results for the specified AWS Config rule.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteEvaluationResultsResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteOrganizationConfigRuleRequest {
    /// <p>The name of organization config rule that you want to delete.</p>
    #[serde(rename = "OrganizationConfigRuleName")]
    pub organization_config_rule_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteOrganizationConformancePackRequest {
    /// <p>The name of organization conformance pack that you want to delete.</p>
    #[serde(rename = "OrganizationConformancePackName")]
    pub organization_conformance_pack_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeletePendingAggregationRequestRequest {
    /// <p>The 12-digit account ID of the account requesting to aggregate data.</p>
    #[serde(rename = "RequesterAccountId")]
    pub requester_account_id: String,
    /// <p>The region requesting to aggregate data.</p>
    #[serde(rename = "RequesterAwsRegion")]
    pub requester_aws_region: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteRemediationConfigurationRequest {
    /// <p>The name of the AWS Config rule for which you want to delete remediation configuration.</p>
    #[serde(rename = "ConfigRuleName")]
    pub config_rule_name: String,
    /// <p>The type of a resource.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteRemediationConfigurationResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteRemediationExceptionsRequest {
    /// <p>The name of the AWS Config rule for which you want to delete remediation exception configuration.</p>
    #[serde(rename = "ConfigRuleName")]
    pub config_rule_name: String,
    /// <p>An exception list of resource exception keys to be processed with the current request. AWS Config adds exception for each resource key. For example, AWS Config adds 3 exceptions for 3 resource keys. </p>
    #[serde(rename = "ResourceKeys")]
    pub resource_keys: Vec<RemediationExceptionResourceKey>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteRemediationExceptionsResponse {
    /// <p>Returns a list of failed delete remediation exceptions batch objects. Each object in the batch consists of a list of failed items and failure messages.</p>
    #[serde(rename = "FailedBatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_batches: Option<Vec<FailedDeleteRemediationExceptionsBatch>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteResourceConfigRequest {
    /// <p>Unique identifier of the resource.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>The type of the resource.</p>
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteRetentionConfigurationRequest {
    /// <p>The name of the retention configuration to delete.</p>
    #[serde(rename = "RetentionConfigurationName")]
    pub retention_configuration_name: String,
}

/// <p>The input for the <a>DeliverConfigSnapshot</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeliverConfigSnapshotRequest {
    /// <p>The name of the delivery channel through which the snapshot is delivered.</p>
    #[serde(rename = "deliveryChannelName")]
    pub delivery_channel_name: String,
}

/// <p>The output for the <a>DeliverConfigSnapshot</a> action, in JSON format.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The name of the Amazon S3 bucket to which AWS Config delivers configuration snapshots and configuration history files.</p> <p>If you specify a bucket that belongs to another AWS account, that bucket must have policies that grant access permissions to AWS Config. For more information, see <a href="https://docs.aws.amazon.com/config/latest/developerguide/s3-bucket-policy.html">Permissions for the Amazon S3 Bucket</a> in the AWS Config Developer Guide.</p>
    #[serde(rename = "s3BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_bucket_name: Option<String>,
    /// <p>The prefix for the specified Amazon S3 bucket.</p>
    #[serde(rename = "s3KeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_key_prefix: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon SNS topic to which AWS Config sends notifications about configuration changes.</p> <p>If you choose a topic from another account, the topic must have policies that grant access permissions to AWS Config. For more information, see <a href="https://docs.aws.amazon.com/config/latest/developerguide/sns-topic-policy.html">Permissions for the Amazon SNS Topic</a> in the AWS Config Developer Guide.</p>
    #[serde(rename = "snsTopicARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
}

/// <p>The status of a specified delivery channel.</p> <p>Valid values: <code>Success</code> | <code>Failure</code> </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAggregateComplianceByConfigRulesResponse {
    /// <p>Returns a list of AggregateComplianceByConfigRule object.</p>
    #[serde(rename = "AggregateComplianceByConfigRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_compliance_by_config_rules: Option<Vec<AggregateComplianceByConfigRule>>,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeAggregationAuthorizationsRequest {
    /// <p>The maximum number of AggregationAuthorizations returned on each page. The default is maximum. If you specify 0, AWS Config uses the default.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAggregationAuthorizationsResponse {
    /// <p>Returns a list of authorizations granted to various aggregator accounts and regions.</p>
    #[serde(rename = "AggregationAuthorizations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_authorizations: Option<Vec<AggregationAuthorization>>,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeComplianceByConfigRuleRequest {
    /// <p>Filters the results by compliance.</p> <p>The allowed values are <code>COMPLIANT</code> and <code>NON_COMPLIANT</code>.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeComplianceByResourceRequest {
    /// <p>Filters the results by compliance.</p> <p>The allowed values are <code>COMPLIANT</code>, <code>NON_COMPLIANT</code>, and <code>INSUFFICIENT_DATA</code>.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeConfigRuleEvaluationStatusRequest {
    /// <p>The name of the AWS managed Config rules for which you want status information. If you do not specify any names, AWS Config returns status information for all AWS managed Config rules that you use.</p>
    #[serde(rename = "ConfigRuleNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_names: Option<Vec<String>>,
    /// <p>The number of rule evaluation results that you want returned.</p> <p>This parameter is required if the rule limit for your account is more than the default of 150 rules.</p> <p>For information about requesting a rule limit increase, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_config">AWS Config Limits</a> in the <i>AWS General Reference Guide</i>.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeConfigurationAggregatorSourcesStatusRequest {
    /// <p>The name of the configuration aggregator.</p>
    #[serde(rename = "ConfigurationAggregatorName")]
    pub configuration_aggregator_name: String,
    /// <p>The maximum number of AggregatorSourceStatus returned on each page. The default is maximum. If you specify 0, AWS Config uses the default.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>Filters the status type.</p> <ul> <li> <p>Valid value FAILED indicates errors while moving data.</p> </li> <li> <p>Valid value SUCCEEDED indicates the data was successfully moved.</p> </li> <li> <p>Valid value OUTDATED indicates the data is not the most recent.</p> </li> </ul></p>
    #[serde(rename = "UpdateStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_status: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeConfigurationAggregatorSourcesStatusResponse {
    /// <p>Returns an AggregatedSourceStatus object. </p>
    #[serde(rename = "AggregatedSourceStatusList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregated_source_status_list: Option<Vec<AggregatedSourceStatus>>,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeConfigurationAggregatorsRequest {
    /// <p>The name of the configuration aggregators.</p>
    #[serde(rename = "ConfigurationAggregatorNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_aggregator_names: Option<Vec<String>>,
    /// <p>The maximum number of configuration aggregators returned on each page. The default is maximum. If you specify 0, AWS Config uses the default.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeConfigurationAggregatorsResponse {
    /// <p>Returns a ConfigurationAggregators object.</p>
    #[serde(rename = "ConfigurationAggregators")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_aggregators: Option<Vec<ConfigurationAggregator>>,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>The input for the <a>DescribeConfigurationRecorderStatus</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeConfigurationRecorderStatusRequest {
    /// <p>The name(s) of the configuration recorder. If the name is not specified, the action returns the current status of all the configuration recorders associated with the account.</p>
    #[serde(rename = "ConfigurationRecorderNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_recorder_names: Option<Vec<String>>,
}

/// <p>The output for the <a>DescribeConfigurationRecorderStatus</a> action, in JSON format.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeConfigurationRecorderStatusResponse {
    /// <p>A list that contains status of the specified recorders.</p>
    #[serde(rename = "ConfigurationRecordersStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_recorders_status: Option<Vec<ConfigurationRecorderStatus>>,
}

/// <p>The input for the <a>DescribeConfigurationRecorders</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeConfigurationRecordersRequest {
    /// <p>A list of configuration recorder names.</p>
    #[serde(rename = "ConfigurationRecorderNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_recorder_names: Option<Vec<String>>,
}

/// <p>The output for the <a>DescribeConfigurationRecorders</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeConfigurationRecordersResponse {
    /// <p>A list that contains the descriptions of the specified configuration recorders.</p>
    #[serde(rename = "ConfigurationRecorders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_recorders: Option<Vec<ConfigurationRecorder>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeConformancePackComplianceRequest {
    /// <p>Name of the conformance pack.</p>
    #[serde(rename = "ConformancePackName")]
    pub conformance_pack_name: String,
    /// <p>A <code>ConformancePackComplianceFilters</code> object.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<ConformancePackComplianceFilters>,
    /// <p>The maximum number of AWS Config rules within a conformance pack are returned on each page.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <code>nextToken</code> string returned in a previous request that you use to request the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeConformancePackComplianceResponse {
    /// <p>Name of the conformance pack.</p>
    #[serde(rename = "ConformancePackName")]
    pub conformance_pack_name: String,
    /// <p>Returns a list of <code>ConformancePackRuleCompliance</code> objects.</p>
    #[serde(rename = "ConformancePackRuleComplianceList")]
    pub conformance_pack_rule_compliance_list: Vec<ConformancePackRuleCompliance>,
    /// <p>The <code>nextToken</code> string returned in a previous request that you use to request the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeConformancePackStatusRequest {
    /// <p>Comma-separated list of conformance pack names.</p>
    #[serde(rename = "ConformancePackNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conformance_pack_names: Option<Vec<String>>,
    /// <p>The maximum number of conformance packs status returned on each page.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <code>nextToken</code> string returned in a previous request that you use to request the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeConformancePackStatusResponse {
    /// <p>A list of <code>ConformancePackStatusDetail</code> objects.</p>
    #[serde(rename = "ConformancePackStatusDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conformance_pack_status_details: Option<Vec<ConformancePackStatusDetail>>,
    /// <p>The <code>nextToken</code> string returned in a previous request that you use to request the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeConformancePacksRequest {
    /// <p>Comma-separated list of conformance pack names for which you want details. If you do not specify any names, AWS Config returns details for all your conformance packs. </p>
    #[serde(rename = "ConformancePackNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conformance_pack_names: Option<Vec<String>>,
    /// <p>The maximum number of conformance packs returned on each page.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <code>nextToken</code> string returned in a previous request that you use to request the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeConformancePacksResponse {
    /// <p>Returns a list of <code>ConformancePackDetail</code> objects.</p>
    #[serde(rename = "ConformancePackDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conformance_pack_details: Option<Vec<ConformancePackDetail>>,
    /// <p>The <code>nextToken</code> string returned in a previous request that you use to request the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>The input for the <a>DeliveryChannelStatus</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDeliveryChannelStatusRequest {
    /// <p>A list of delivery channel names.</p>
    #[serde(rename = "DeliveryChannelNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_channel_names: Option<Vec<String>>,
}

/// <p>The output for the <a>DescribeDeliveryChannelStatus</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeDeliveryChannelStatusResponse {
    /// <p>A list that contains the status of a specified delivery channel.</p>
    #[serde(rename = "DeliveryChannelsStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_channels_status: Option<Vec<DeliveryChannelStatus>>,
}

/// <p>The input for the <a>DescribeDeliveryChannels</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDeliveryChannelsRequest {
    /// <p>A list of delivery channel names.</p>
    #[serde(rename = "DeliveryChannelNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_channel_names: Option<Vec<String>>,
}

/// <p>The output for the <a>DescribeDeliveryChannels</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeDeliveryChannelsResponse {
    /// <p>A list that contains the descriptions of the specified delivery channel.</p>
    #[serde(rename = "DeliveryChannels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_channels: Option<Vec<DeliveryChannel>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeOrganizationConfigRuleStatusesRequest {
    /// <p>The maximum number of <code>OrganizationConfigRuleStatuses</code> returned on each page. If you do no specify a number, AWS Config uses the default. The default is 100.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The names of organization config rules for which you want status details. If you do not specify any names, AWS Config returns details for all your organization AWS Confg rules.</p>
    #[serde(rename = "OrganizationConfigRuleNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_config_rule_names: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeOrganizationConfigRuleStatusesResponse {
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of <code>OrganizationConfigRuleStatus</code> objects.</p>
    #[serde(rename = "OrganizationConfigRuleStatuses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_config_rule_statuses: Option<Vec<OrganizationConfigRuleStatus>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeOrganizationConfigRulesRequest {
    /// <p>The maximum number of organization config rules returned on each page. If you do no specify a number, AWS Config uses the default. The default is 100.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The names of organization config rules for which you want details. If you do not specify any names, AWS Config returns details for all your organization config rules.</p>
    #[serde(rename = "OrganizationConfigRuleNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_config_rule_names: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeOrganizationConfigRulesResponse {
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Returns a list of <code>OrganizationConfigRule</code> objects.</p>
    #[serde(rename = "OrganizationConfigRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_config_rules: Option<Vec<OrganizationConfigRule>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeOrganizationConformancePackStatusesRequest {
    /// <p>The maximum number of OrganizationConformancePackStatuses returned on each page. If you do no specify a number, AWS Config uses the default. The default is 100. </p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The nextToken string returned on a previous page that you use to get the next page of results in a paginated response. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The names of organization conformance packs for which you want status details. If you do not specify any names, AWS Config returns details for all your organization conformance packs. </p>
    #[serde(rename = "OrganizationConformancePackNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_conformance_pack_names: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeOrganizationConformancePackStatusesResponse {
    /// <p>The nextToken string returned on a previous page that you use to get the next page of results in a paginated response. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of <code>OrganizationConformancePackStatus</code> objects. </p>
    #[serde(rename = "OrganizationConformancePackStatuses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_conformance_pack_statuses: Option<Vec<OrganizationConformancePackStatus>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeOrganizationConformancePacksRequest {
    /// <p>The maximum number of organization config packs returned on each page. If you do no specify a number, AWS Config uses the default. The default is 100.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The nextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name that you assign to an organization conformance pack.</p>
    #[serde(rename = "OrganizationConformancePackNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_conformance_pack_names: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeOrganizationConformancePacksResponse {
    /// <p>The nextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Returns a list of OrganizationConformancePacks objects.</p>
    #[serde(rename = "OrganizationConformancePacks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_conformance_packs: Option<Vec<OrganizationConformancePack>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribePendingAggregationRequestsRequest {
    /// <p>The maximum number of evaluation results returned on each page. The default is maximum. If you specify 0, AWS Config uses the default.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribePendingAggregationRequestsResponse {
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Returns a PendingAggregationRequests object.</p>
    #[serde(rename = "PendingAggregationRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_aggregation_requests: Option<Vec<PendingAggregationRequest>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeRemediationConfigurationsRequest {
    /// <p>A list of AWS Config rule names of remediation configurations for which you want details. </p>
    #[serde(rename = "ConfigRuleNames")]
    pub config_rule_names: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeRemediationConfigurationsResponse {
    /// <p>Returns a remediation configuration object.</p>
    #[serde(rename = "RemediationConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediation_configurations: Option<Vec<RemediationConfiguration>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeRemediationExceptionsRequest {
    /// <p>The name of the AWS Config rule.</p>
    #[serde(rename = "ConfigRuleName")]
    pub config_rule_name: String,
    /// <p>The maximum number of RemediationExceptionResourceKey returned on each page. The default is 25. If you specify 0, AWS Config uses the default.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <code>nextToken</code> string returned in a previous request that you use to request the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An exception list of resource exception keys to be processed with the current request. AWS Config adds exception for each resource key. For example, AWS Config adds 3 exceptions for 3 resource keys. </p>
    #[serde(rename = "ResourceKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_keys: Option<Vec<RemediationExceptionResourceKey>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeRemediationExceptionsResponse {
    /// <p>The <code>nextToken</code> string returned in a previous request that you use to request the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Returns a list of remediation exception objects.</p>
    #[serde(rename = "RemediationExceptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediation_exceptions: Option<Vec<RemediationException>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeRemediationExecutionStatusRequest {
    /// <p>A list of AWS Config rule names.</p>
    #[serde(rename = "ConfigRuleName")]
    pub config_rule_name: String,
    /// <p>The maximum number of RemediationExecutionStatuses returned on each page. The default is maximum. If you specify 0, AWS Config uses the default. </p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of resource keys to be processed with the current request. Each element in the list consists of the resource type and resource ID. </p>
    #[serde(rename = "ResourceKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_keys: Option<Vec<ResourceKey>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeRemediationExecutionStatusResponse {
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Returns a list of remediation execution statuses objects.</p>
    #[serde(rename = "RemediationExecutionStatuses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediation_execution_statuses: Option<Vec<RemediationExecutionStatus>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeRetentionConfigurationsRequest {
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>A list of names of retention configurations for which you want details. If you do not specify a name, AWS Config returns details for all the retention configurations for that account.</p> <note> <p>Currently, AWS Config supports only one retention configuration per region in your account.</p> </note></p>
    #[serde(rename = "RetentionConfigurationNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_configuration_names: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeRetentionConfigurationsResponse {
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Returns a retention configuration object.</p>
    #[serde(rename = "RetentionConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_configurations: Option<Vec<RetentionConfiguration>>,
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>The controls that AWS Config uses for executing remediations.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionControls {
    /// <p>A SsmControls object.</p>
    #[serde(rename = "SsmControls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssm_controls: Option<SsmControls>,
}

/// <p>List of each of the failed delete remediation exceptions with specific reasons.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FailedDeleteRemediationExceptionsBatch {
    /// <p>Returns remediation exception resource key object of the failed items.</p>
    #[serde(rename = "FailedItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_items: Option<Vec<RemediationExceptionResourceKey>>,
    /// <p>Returns a failure message for delete remediation exception. For example, AWS Config creates an exception due to an internal error.</p>
    #[serde(rename = "FailureMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
}

/// <p>List of each of the failed remediations with specific reasons.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FailedRemediationBatch {
    /// <p>Returns remediation configurations of the failed items.</p>
    #[serde(rename = "FailedItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_items: Option<Vec<RemediationConfiguration>>,
    /// <p>Returns a failure message. For example, the resource is already compliant.</p>
    #[serde(rename = "FailureMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
}

/// <p>List of each of the failed remediation exceptions with specific reasons.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FailedRemediationExceptionBatch {
    /// <p>Returns remediation exception resource key object of the failed items.</p>
    #[serde(rename = "FailedItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_items: Option<Vec<RemediationException>>,
    /// <p>Returns a failure message. For example, the auto-remediation has failed.</p>
    #[serde(rename = "FailureMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
}

/// <p>Details about the fields such as name of the field.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FieldInfo {
    /// <p>Name of the field.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAggregateComplianceDetailsByConfigRuleResponse {
    /// <p>Returns an AggregateEvaluationResults object.</p>
    #[serde(rename = "AggregateEvaluationResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_evaluation_results: Option<Vec<AggregateEvaluationResult>>,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAggregateConfigRuleComplianceSummaryResponse {
    /// <p>Returns a list of AggregateComplianceCounts object.</p>
    #[serde(rename = "AggregateComplianceCounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_compliance_counts: Option<Vec<AggregateComplianceCount>>,
    /// <p>Groups the result based on ACCOUNT_ID or AWS_REGION.</p>
    #[serde(rename = "GroupByKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by_key: Option<String>,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAggregateDiscoveredResourceCountsRequest {
    /// <p>The name of the configuration aggregator.</p>
    #[serde(rename = "ConfigurationAggregatorName")]
    pub configuration_aggregator_name: String,
    /// <p>Filters the results based on the <code>ResourceCountFilters</code> object.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<ResourceCountFilters>,
    /// <p>The key to group the resource counts.</p>
    #[serde(rename = "GroupByKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by_key: Option<String>,
    /// <p>The maximum number of <a>GroupedResourceCount</a> objects returned on each page. The default is 1000. You cannot specify a number greater than 1000. If you specify 0, AWS Config uses the default.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAggregateDiscoveredResourceCountsResponse {
    /// <p>The key passed into the request object. If <code>GroupByKey</code> is not provided, the result will be empty.</p>
    #[serde(rename = "GroupByKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by_key: Option<String>,
    /// <p>Returns a list of GroupedResourceCount objects.</p>
    #[serde(rename = "GroupedResourceCounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouped_resource_counts: Option<Vec<GroupedResourceCount>>,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The total number of resources that are present in an aggregator with the filters that you provide.</p>
    #[serde(rename = "TotalDiscoveredResources")]
    pub total_discovered_resources: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAggregateResourceConfigRequest {
    /// <p>The name of the configuration aggregator.</p>
    #[serde(rename = "ConfigurationAggregatorName")]
    pub configuration_aggregator_name: String,
    /// <p>An object that identifies aggregate resource.</p>
    #[serde(rename = "ResourceIdentifier")]
    pub resource_identifier: AggregateResourceIdentifier,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAggregateResourceConfigResponse {
    /// <p>Returns a <code>ConfigurationItem</code> object.</p>
    #[serde(rename = "ConfigurationItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_item: Option<ConfigurationItem>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetComplianceSummaryByConfigRuleResponse {
    /// <p>The number of AWS Config rules that are compliant and the number that are noncompliant, up to a maximum of 25 for each.</p>
    #[serde(rename = "ComplianceSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_summary: Option<ComplianceSummary>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetComplianceSummaryByResourceTypeRequest {
    /// <p>Specify one or more resource types to get the number of resources that are compliant and the number that are noncompliant for each resource type.</p> <p>For this request, you can specify an AWS resource type such as <code>AWS::EC2::Instance</code>. You can specify that the resource type is an AWS account by specifying <code>AWS::::Account</code>.</p>
    #[serde(rename = "ResourceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types: Option<Vec<String>>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetComplianceSummaryByResourceTypeResponse {
    /// <p>The number of resources that are compliant and the number that are noncompliant. If one or more resource types were provided with the request, the numbers are returned for each resource type. The maximum number returned is 100.</p>
    #[serde(rename = "ComplianceSummariesByResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_summaries_by_resource_type: Option<Vec<ComplianceSummaryByResourceType>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetConformancePackComplianceDetailsRequest {
    /// <p>Name of the conformance pack.</p>
    #[serde(rename = "ConformancePackName")]
    pub conformance_pack_name: String,
    /// <p>A <code>ConformancePackEvaluationFilters</code> object.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<ConformancePackEvaluationFilters>,
    /// <p>The maximum number of evaluation results returned on each page. If you do no specify a number, AWS Config uses the default. The default is 100.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <code>nextToken</code> string returned in a previous request that you use to request the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetConformancePackComplianceDetailsResponse {
    /// <p>Name of the conformance pack.</p>
    #[serde(rename = "ConformancePackName")]
    pub conformance_pack_name: String,
    /// <p>Returns a list of <code>ConformancePackEvaluationResult</code> objects.</p>
    #[serde(rename = "ConformancePackRuleEvaluationResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conformance_pack_rule_evaluation_results: Option<Vec<ConformancePackEvaluationResult>>,
    /// <p>The <code>nextToken</code> string returned in a previous request that you use to request the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetConformancePackComplianceSummaryRequest {
    /// <p>Names of conformance packs.</p>
    #[serde(rename = "ConformancePackNames")]
    pub conformance_pack_names: Vec<String>,
    /// <p>The maximum number of conformance packs returned on each page.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The nextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetConformancePackComplianceSummaryResponse {
    /// <p>A list of <code>ConformancePackComplianceSummary</code> objects. </p>
    #[serde(rename = "ConformancePackComplianceSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conformance_pack_compliance_summary_list: Option<Vec<ConformancePackComplianceSummary>>,
    /// <p>The nextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetOrganizationConfigRuleDetailedStatusRequest {
    /// <p>A <code>StatusDetailFilters</code> object.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<StatusDetailFilters>,
    /// <p>The maximum number of <code>OrganizationConfigRuleDetailedStatus</code> returned on each page. If you do not specify a number, AWS Config uses the default. The default is 100.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of organization config rule for which you want status details for member accounts.</p>
    #[serde(rename = "OrganizationConfigRuleName")]
    pub organization_config_rule_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetOrganizationConfigRuleDetailedStatusResponse {
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of <code>MemberAccountStatus</code> objects.</p>
    #[serde(rename = "OrganizationConfigRuleDetailedStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_config_rule_detailed_status: Option<Vec<MemberAccountStatus>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetOrganizationConformancePackDetailedStatusRequest {
    /// <p>An <code>OrganizationResourceDetailedStatusFilters</code> object.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<OrganizationResourceDetailedStatusFilters>,
    /// <p>The maximum number of <code>OrganizationConformancePackDetailedStatuses</code> returned on each page. If you do not specify a number, AWS Config uses the default. The default is 100. </p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The nextToken string returned on a previous page that you use to get the next page of results in a paginated response. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of organization conformance pack for which you want status details for member accounts.</p>
    #[serde(rename = "OrganizationConformancePackName")]
    pub organization_conformance_pack_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetOrganizationConformancePackDetailedStatusResponse {
    /// <p>The nextToken string returned on a previous page that you use to get the next page of results in a paginated response. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of <code>OrganizationConformancePackDetailedStatus</code> objects. </p>
    #[serde(rename = "OrganizationConformancePackDetailedStatuses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_conformance_pack_detailed_statuses:
        Option<Vec<OrganizationConformancePackDetailedStatus>>,
}

/// <p>The input for the <a>GetResourceConfigHistory</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>The count of resources that are grouped by the group name.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GroupedResourceCount {
    /// <p>The name of the group that can be region, account ID, or resource type. For example, region1, region2 if the region was chosen as <code>GroupByKey</code>.</p>
    #[serde(rename = "GroupName")]
    pub group_name: String,
    /// <p>The number of resources in the group.</p>
    #[serde(rename = "ResourceCount")]
    pub resource_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListAggregateDiscoveredResourcesRequest {
    /// <p>The name of the configuration aggregator. </p>
    #[serde(rename = "ConfigurationAggregatorName")]
    pub configuration_aggregator_name: String,
    /// <p>Filters the results based on the <code>ResourceFilters</code> object.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<ResourceFilters>,
    /// <p>The maximum number of resource identifiers returned on each page. The default is 100. You cannot specify a number greater than 100. If you specify 0, AWS Config uses the default.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The type of resources that you want AWS Config to list in the response.</p>
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListAggregateDiscoveredResourcesResponse {
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Returns a list of <code>ResourceIdentifiers</code> objects.</p>
    #[serde(rename = "ResourceIdentifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifiers: Option<Vec<AggregateResourceIdentifier>>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The maximum number of tags returned on each page. The limit maximum is 50. You cannot specify a number greater than 50. If you specify 0, AWS Config uses the default. </p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) that identifies the resource for which to list the tags. Currently, the supported resources are <code>ConfigRule</code>, <code>ConfigurationAggregator</code> and <code>AggregatorAuthorization</code>.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The tags for the resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Organization config rule creation or deletion status in each member account. This includes the name of the rule, the status, error code and error message when the rule creation or deletion failed.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MemberAccountStatus {
    /// <p>The 12-digit account ID of a member account.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The name of config rule deployed in the member account.</p>
    #[serde(rename = "ConfigRuleName")]
    pub config_rule_name: String,
    /// <p>An error code that is returned when config rule creation or deletion failed in the member account.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>An error message indicating that config rule account creation or deletion has failed due to an error in the member account.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The timestamp of the last status update.</p>
    #[serde(rename = "LastUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    /// <p><p>Indicates deployment status for config rule in the member account. When master account calls <code>PutOrganizationConfigRule</code> action for the first time, config rule status is created in the member account. When master account calls <code>PutOrganizationConfigRule</code> action for the second time, config rule status is updated in the member account. Config rule status is deleted when the master account deletes <code>OrganizationConfigRule</code> and disables service access for <code>config-multiaccountsetup.amazonaws.com</code>. </p> <p> AWS Config sets the state of the rule to:</p> <ul> <li> <p> <code>CREATE<em>SUCCESSFUL</code> when config rule has been created in the member account. </p> </li> <li> <p> <code>CREATE</em>IN<em>PROGRESS</code> when config rule is being created in the member account.</p> </li> <li> <p> <code>CREATE</em>FAILED</code> when config rule creation has failed in the member account.</p> </li> <li> <p> <code>DELETE<em>FAILED</code> when config rule deletion has failed in the member account.</p> </li> <li> <p> <code>DELETE</em>IN<em>PROGRESS</code> when config rule is being deleted in the member account.</p> </li> <li> <p> <code>DELETE</em>SUCCESSFUL</code> when config rule has been deleted in the member account. </p> </li> <li> <p> <code>UPDATE<em>SUCCESSFUL</code> when config rule has been updated in the member account.</p> </li> <li> <p> <code>UPDATE</em>IN<em>PROGRESS</code> when config rule is being updated in the member account.</p> </li> <li> <p> <code>UPDATE</em>FAILED</code> when config rule deletion has failed in the member account.</p> </li> </ul></p>
    #[serde(rename = "MemberAccountRuleStatus")]
    pub member_account_rule_status: String,
}

/// <p>This object contains regions to set up the aggregator and an IAM role to retrieve organization details.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrganizationAggregationSource {
    /// <p>If true, aggregate existing AWS Config regions and future regions.</p>
    #[serde(rename = "AllAwsRegions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_aws_regions: Option<bool>,
    /// <p>The source regions being aggregated.</p>
    #[serde(rename = "AwsRegions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_regions: Option<Vec<String>>,
    /// <p>ARN of the IAM role used to retrieve AWS Organization details associated with the aggregator account.</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
}

/// <p>An organization config rule that has information about config rules that AWS Config creates in member accounts.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OrganizationConfigRule {
    /// <p>A comma-separated list of accounts excluded from organization config rule.</p>
    #[serde(rename = "ExcludedAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_accounts: Option<Vec<String>>,
    /// <p>The timestamp of the last update.</p>
    #[serde(rename = "LastUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    /// <p>Amazon Resource Name (ARN) of organization config rule.</p>
    #[serde(rename = "OrganizationConfigRuleArn")]
    pub organization_config_rule_arn: String,
    /// <p>The name that you assign to organization config rule.</p>
    #[serde(rename = "OrganizationConfigRuleName")]
    pub organization_config_rule_name: String,
    /// <p>An <code>OrganizationCustomRuleMetadata</code> object.</p>
    #[serde(rename = "OrganizationCustomRuleMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_custom_rule_metadata: Option<OrganizationCustomRuleMetadata>,
    /// <p>An <code>OrganizationManagedRuleMetadata</code> object.</p>
    #[serde(rename = "OrganizationManagedRuleMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_managed_rule_metadata: Option<OrganizationManagedRuleMetadata>,
}

/// <p>Returns the status for an organization config rule in an organization.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OrganizationConfigRuleStatus {
    /// <p>An error code that is returned when organization config rule creation or deletion has failed.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>An error message indicating that organization config rule creation or deletion failed due to an error.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The timestamp of the last update.</p>
    #[serde(rename = "LastUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    /// <p>The name that you assign to organization config rule.</p>
    #[serde(rename = "OrganizationConfigRuleName")]
    pub organization_config_rule_name: String,
    /// <p><p>Indicates deployment status of an organization config rule. When master account calls PutOrganizationConfigRule action for the first time, config rule status is created in all the member accounts. When master account calls PutOrganizationConfigRule action for the second time, config rule status is updated in all the member accounts. Additionally, config rule status is updated when one or more member accounts join or leave an organization. Config rule status is deleted when the master account deletes OrganizationConfigRule in all the member accounts and disables service access for <code>config-multiaccountsetup.amazonaws.com</code>.</p> <p>AWS Config sets the state of the rule to:</p> <ul> <li> <p> <code>CREATE<em>SUCCESSFUL</code> when an organization config rule has been successfully created in all the member accounts. </p> </li> <li> <p> <code>CREATE</em>IN<em>PROGRESS</code> when an organization config rule creation is in progress.</p> </li> <li> <p> <code>CREATE</em>FAILED</code> when an organization config rule creation failed in one or more member accounts within that organization.</p> </li> <li> <p> <code>DELETE<em>FAILED</code> when an organization config rule deletion failed in one or more member accounts within that organization.</p> </li> <li> <p> <code>DELETE</em>IN<em>PROGRESS</code> when an organization config rule deletion is in progress.</p> </li> <li> <p> <code>DELETE</em>SUCCESSFUL</code> when an organization config rule has been successfully deleted from all the member accounts.</p> </li> <li> <p> <code>UPDATE<em>SUCCESSFUL</code> when an organization config rule has been successfully updated in all the member accounts.</p> </li> <li> <p> <code>UPDATE</em>IN<em>PROGRESS</code> when an organization config rule update is in progress.</p> </li> <li> <p> <code>UPDATE</em>FAILED</code> when an organization config rule update failed in one or more member accounts within that organization.</p> </li> </ul></p>
    #[serde(rename = "OrganizationRuleStatus")]
    pub organization_rule_status: String,
}

/// <p>An organization conformance pack that has information about conformance packs that AWS Config creates in member accounts. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OrganizationConformancePack {
    /// <p>A list of <code>ConformancePackInputParameter</code> objects.</p>
    #[serde(rename = "ConformancePackInputParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conformance_pack_input_parameters: Option<Vec<ConformancePackInputParameter>>,
    /// <p>Location of an Amazon S3 bucket where AWS Config can deliver evaluation results and conformance pack template that is used to create a pack. </p>
    #[serde(rename = "DeliveryS3Bucket")]
    pub delivery_s3_bucket: String,
    /// <p>Any folder structure you want to add to an Amazon S3 bucket.</p>
    #[serde(rename = "DeliveryS3KeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_s3_key_prefix: Option<String>,
    /// <p>A comma-separated list of accounts excluded from organization conformance pack.</p>
    #[serde(rename = "ExcludedAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_accounts: Option<Vec<String>>,
    /// <p>Last time when organization conformation pack was updated.</p>
    #[serde(rename = "LastUpdateTime")]
    pub last_update_time: f64,
    /// <p>Amazon Resource Name (ARN) of organization conformance pack.</p>
    #[serde(rename = "OrganizationConformancePackArn")]
    pub organization_conformance_pack_arn: String,
    /// <p>The name you assign to an organization conformance pack.</p>
    #[serde(rename = "OrganizationConformancePackName")]
    pub organization_conformance_pack_name: String,
}

/// <p>Organization conformance pack creation or deletion status in each member account. This includes the name of the conformance pack, the status, error code and error message when the conformance pack creation or deletion failed. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OrganizationConformancePackDetailedStatus {
    /// <p>The 12-digit account ID of a member account.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The name of conformance pack deployed in the member account.</p>
    #[serde(rename = "ConformancePackName")]
    pub conformance_pack_name: String,
    /// <p>An error code that is returned when conformance pack creation or deletion failed in the member account. </p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>An error message indicating that conformance pack account creation or deletion has failed due to an error in the member account. </p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The timestamp of the last status update.</p>
    #[serde(rename = "LastUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    /// <p><p>Indicates deployment status for conformance pack in a member account. When master account calls <code>PutOrganizationConformancePack</code> action for the first time, conformance pack status is created in the member account. When master account calls <code>PutOrganizationConformancePack</code> action for the second time, conformance pack status is updated in the member account. Conformance pack status is deleted when the master account deletes <code>OrganizationConformancePack</code> and disables service access for <code>config-multiaccountsetup.amazonaws.com</code>. </p> <p> AWS Config sets the state of the conformance pack to:</p> <ul> <li> <p> <code>CREATE<em>SUCCESSFUL</code> when conformance pack has been created in the member account. </p> </li> <li> <p> <code>CREATE</em>IN<em>PROGRESS</code> when conformance pack is being created in the member account.</p> </li> <li> <p> <code>CREATE</em>FAILED</code> when conformance pack creation has failed in the member account.</p> </li> <li> <p> <code>DELETE<em>FAILED</code> when conformance pack deletion has failed in the member account.</p> </li> <li> <p> <code>DELETE</em>IN<em>PROGRESS</code> when conformance pack is being deleted in the member account.</p> </li> <li> <p> <code>DELETE</em>SUCCESSFUL</code> when conformance pack has been deleted in the member account. </p> </li> <li> <p> <code>UPDATE<em>SUCCESSFUL</code> when conformance pack has been updated in the member account.</p> </li> <li> <p> <code>UPDATE</em>IN<em>PROGRESS</code> when conformance pack is being updated in the member account.</p> </li> <li> <p> <code>UPDATE</em>FAILED</code> when conformance pack deletion has failed in the member account.</p> </li> </ul></p>
    #[serde(rename = "Status")]
    pub status: String,
}

/// <p>Returns the status for an organization conformance pack in an organization.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OrganizationConformancePackStatus {
    /// <p>An error code that is returned when organization conformance pack creation or deletion has failed in a member account. </p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>An error message indicating that organization conformance pack creation or deletion failed due to an error. </p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The timestamp of the last update.</p>
    #[serde(rename = "LastUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    /// <p>The name that you assign to organization conformance pack.</p>
    #[serde(rename = "OrganizationConformancePackName")]
    pub organization_conformance_pack_name: String,
    /// <p><p>Indicates deployment status of an organization conformance pack. When master account calls PutOrganizationConformancePack for the first time, conformance pack status is created in all the member accounts. When master account calls PutOrganizationConformancePack for the second time, conformance pack status is updated in all the member accounts. Additionally, conformance pack status is updated when one or more member accounts join or leave an organization. Conformance pack status is deleted when the master account deletes OrganizationConformancePack in all the member accounts and disables service access for <code>config-multiaccountsetup.amazonaws.com</code>.</p> <p>AWS Config sets the state of the conformance pack to:</p> <ul> <li> <p> <code>CREATE<em>SUCCESSFUL</code> when an organization conformance pack has been successfully created in all the member accounts. </p> </li> <li> <p> <code>CREATE</em>IN<em>PROGRESS</code> when an organization conformance pack creation is in progress.</p> </li> <li> <p> <code>CREATE</em>FAILED</code> when an organization conformance pack creation failed in one or more member accounts within that organization.</p> </li> <li> <p> <code>DELETE<em>FAILED</code> when an organization conformance pack deletion failed in one or more member accounts within that organization.</p> </li> <li> <p> <code>DELETE</em>IN<em>PROGRESS</code> when an organization conformance pack deletion is in progress.</p> </li> <li> <p> <code>DELETE</em>SUCCESSFUL</code> when an organization conformance pack has been successfully deleted from all the member accounts.</p> </li> <li> <p> <code>UPDATE<em>SUCCESSFUL</code> when an organization conformance pack has been successfully updated in all the member accounts.</p> </li> <li> <p> <code>UPDATE</em>IN<em>PROGRESS</code> when an organization conformance pack update is in progress.</p> </li> <li> <p> <code>UPDATE</em>FAILED</code> when an organization conformance pack update failed in one or more member accounts within that organization.</p> </li> </ul></p>
    #[serde(rename = "Status")]
    pub status: String,
}

/// <p>An object that specifies organization custom rule metadata such as resource type, resource ID of AWS resource, Lamdba function ARN, and organization trigger types that trigger AWS Config to evaluate your AWS resources against a rule. It also provides the frequency with which you want AWS Config to run evaluations for the rule if the trigger type is periodic.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrganizationCustomRuleMetadata {
    /// <p>The description that you provide for organization config rule.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A string, in JSON format, that is passed to organization config rule Lambda function.</p>
    #[serde(rename = "InputParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_parameters: Option<String>,
    /// <p>The lambda function ARN.</p>
    #[serde(rename = "LambdaFunctionArn")]
    pub lambda_function_arn: String,
    /// <p><p>The maximum frequency with which AWS Config runs evaluations for a rule. Your custom rule is triggered when AWS Config delivers the configuration snapshot. For more information, see <a>ConfigSnapshotDeliveryProperties</a>.</p> <note> <p>By default, rules with a periodic trigger are evaluated every 24 hours. To change the frequency, specify a valid value for the <code>MaximumExecutionFrequency</code> parameter.</p> </note></p>
    #[serde(rename = "MaximumExecutionFrequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_execution_frequency: Option<String>,
    /// <p><p>The type of notification that triggers AWS Config to run an evaluation for a rule. You can specify the following notification types:</p> <ul> <li> <p> <code>ConfigurationItemChangeNotification</code> - Triggers an evaluation when AWS Config delivers a configuration item as a result of a resource change.</p> </li> <li> <p> <code>OversizedConfigurationItemChangeNotification</code> - Triggers an evaluation when AWS Config delivers an oversized configuration item. AWS Config may generate this notification type when a resource changes and the notification exceeds the maximum size allowed by Amazon SNS.</p> </li> <li> <p> <code>ScheduledNotification</code> - Triggers a periodic evaluation at the frequency specified for <code>MaximumExecutionFrequency</code>.</p> </li> </ul></p>
    #[serde(rename = "OrganizationConfigRuleTriggerTypes")]
    pub organization_config_rule_trigger_types: Vec<String>,
    /// <p>The ID of the AWS resource that was evaluated.</p>
    #[serde(rename = "ResourceIdScope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id_scope: Option<String>,
    /// <p>The type of the AWS resource that was evaluated.</p>
    #[serde(rename = "ResourceTypesScope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types_scope: Option<Vec<String>>,
    /// <p>One part of a key-value pair that make up a tag. A key is a general label that acts like a category for more specific tag values. </p>
    #[serde(rename = "TagKeyScope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key_scope: Option<String>,
    /// <p>The optional part of a key-value pair that make up a tag. A value acts as a descriptor within a tag category (key). </p>
    #[serde(rename = "TagValueScope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value_scope: Option<String>,
}

/// <p>An object that specifies organization managed rule metadata such as resource type and ID of AWS resource along with the rule identifier. It also provides the frequency with which you want AWS Config to run evaluations for the rule if the trigger type is periodic.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrganizationManagedRuleMetadata {
    /// <p>The description that you provide for organization config rule.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A string, in JSON format, that is passed to organization config rule Lambda function.</p>
    #[serde(rename = "InputParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_parameters: Option<String>,
    /// <p><p>The maximum frequency with which AWS Config runs evaluations for a rule. You are using an AWS managed rule that is triggered at a periodic frequency.</p> <note> <p>By default, rules with a periodic trigger are evaluated every 24 hours. To change the frequency, specify a valid value for the <code>MaximumExecutionFrequency</code> parameter.</p> </note></p>
    #[serde(rename = "MaximumExecutionFrequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_execution_frequency: Option<String>,
    /// <p>The ID of the AWS resource that was evaluated.</p>
    #[serde(rename = "ResourceIdScope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id_scope: Option<String>,
    /// <p>The type of the AWS resource that was evaluated.</p>
    #[serde(rename = "ResourceTypesScope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types_scope: Option<Vec<String>>,
    /// <p>For organization config managed rules, a predefined identifier from a list. For example, <code>IAM_PASSWORD_POLICY</code> is a managed rule. To reference a managed rule, see <a href="https://docs.aws.amazon.com/config/latest/developerguide/evaluate-config_use-managed-rules.html">Using AWS Managed Config Rules</a>.</p>
    #[serde(rename = "RuleIdentifier")]
    pub rule_identifier: String,
    /// <p>One part of a key-value pair that make up a tag. A key is a general label that acts like a category for more specific tag values. </p>
    #[serde(rename = "TagKeyScope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key_scope: Option<String>,
    /// <p>The optional part of a key-value pair that make up a tag. A value acts as a descriptor within a tag category (key).</p>
    #[serde(rename = "TagValueScope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value_scope: Option<String>,
}

/// <p>Status filter object to filter results based on specific member account ID or status type for an organization conformance pack.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct OrganizationResourceDetailedStatusFilters {
    /// <p>The 12-digit account ID of the member account within an organization.</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p><p>Indicates deployment status for conformance pack in a member account. When master account calls <code>PutOrganizationConformancePack</code> action for the first time, conformance pack status is created in the member account. When master account calls <code>PutOrganizationConformancePack</code> action for the second time, conformance pack status is updated in the member account. Conformance pack status is deleted when the master account deletes <code>OrganizationConformancePack</code> and disables service access for <code>config-multiaccountsetup.amazonaws.com</code>. </p> <p> AWS Config sets the state of the conformance pack to:</p> <ul> <li> <p> <code>CREATE<em>SUCCESSFUL</code> when conformance pack has been created in the member account. </p> </li> <li> <p> <code>CREATE</em>IN<em>PROGRESS</code> when conformance pack is being created in the member account.</p> </li> <li> <p> <code>CREATE</em>FAILED</code> when conformance pack creation has failed in the member account.</p> </li> <li> <p> <code>DELETE<em>FAILED</code> when conformance pack deletion has failed in the member account.</p> </li> <li> <p> <code>DELETE</em>IN<em>PROGRESS</code> when conformance pack is being deleted in the member account.</p> </li> <li> <p> <code>DELETE</em>SUCCESSFUL</code> when conformance pack has been deleted in the member account. </p> </li> <li> <p> <code>UPDATE<em>SUCCESSFUL</code> when conformance pack has been updated in the member account.</p> </li> <li> <p> <code>UPDATE</em>IN<em>PROGRESS</code> when conformance pack is being updated in the member account.</p> </li> <li> <p> <code>UPDATE</em>FAILED</code> when conformance pack deletion has failed in the member account.</p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>An object that represents the account ID and region of an aggregator account that is requesting authorization but is not yet authorized.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutAggregationAuthorizationRequest {
    /// <p>The 12-digit account ID of the account authorized to aggregate data.</p>
    #[serde(rename = "AuthorizedAccountId")]
    pub authorized_account_id: String,
    /// <p>The region authorized to collect aggregated data.</p>
    #[serde(rename = "AuthorizedAwsRegion")]
    pub authorized_aws_region: String,
    /// <p>An array of tag object.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutAggregationAuthorizationResponse {
    /// <p>Returns an AggregationAuthorization object. </p>
    #[serde(rename = "AggregationAuthorization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_authorization: Option<AggregationAuthorization>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutConfigRuleRequest {
    /// <p>The rule that you want to add to your account.</p>
    #[serde(rename = "ConfigRule")]
    pub config_rule: ConfigRule,
    /// <p>An array of tag object.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>An array of tag object.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutConfigurationAggregatorResponse {
    /// <p>Returns a ConfigurationAggregator object.</p>
    #[serde(rename = "ConfigurationAggregator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_aggregator: Option<ConfigurationAggregator>,
}

/// <p>The input for the <a>PutConfigurationRecorder</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutConfigurationRecorderRequest {
    /// <p>The configuration recorder object that records each configuration change made to the resources.</p>
    #[serde(rename = "ConfigurationRecorder")]
    pub configuration_recorder: ConfigurationRecorder,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutConformancePackRequest {
    /// <p>A list of <code>ConformancePackInputParameter</code> objects.</p>
    #[serde(rename = "ConformancePackInputParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conformance_pack_input_parameters: Option<Vec<ConformancePackInputParameter>>,
    /// <p>Name of the conformance pack you want to create.</p>
    #[serde(rename = "ConformancePackName")]
    pub conformance_pack_name: String,
    /// <p>AWS Config stores intermediate files while processing conformance pack template.</p>
    #[serde(rename = "DeliveryS3Bucket")]
    pub delivery_s3_bucket: String,
    /// <p>The prefix for the Amazon S3 bucket. </p>
    #[serde(rename = "DeliveryS3KeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_s3_key_prefix: Option<String>,
    /// <p><p>A string containing full conformance pack template body. Structure containing the template body with a minimum length of 1 byte and a maximum length of 51,200 bytes.</p> <note> <p>You can only use a YAML template with one resource type, that is, config rule and a remediation action. </p> </note></p>
    #[serde(rename = "TemplateBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_body: Option<String>,
    /// <p><p>Location of file containing the template body (<code>s3://bucketname/prefix</code>). The uri must point to the conformance pack template (max size: 300 KB) that is located in an Amazon S3 bucket in the same region as the conformance pack. </p> <note> <p>You must have access to read Amazon S3 bucket.</p> </note></p>
    #[serde(rename = "TemplateS3Uri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_s3_uri: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutConformancePackResponse {
    /// <p>ARN of the conformance pack.</p>
    #[serde(rename = "ConformancePackArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conformance_pack_arn: Option<String>,
}

/// <p>The input for the <a>PutDeliveryChannel</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutDeliveryChannelRequest {
    /// <p>The configuration delivery channel object that delivers the configuration information to an Amazon S3 bucket and to an Amazon SNS topic.</p>
    #[serde(rename = "DeliveryChannel")]
    pub delivery_channel: DeliveryChannel,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutEvaluationsResponse {
    /// <p>Requests that failed because of a client or server error.</p>
    #[serde(rename = "FailedEvaluations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_evaluations: Option<Vec<Evaluation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutOrganizationConfigRuleRequest {
    /// <p>A comma-separated list of accounts that you want to exclude from an organization config rule.</p>
    #[serde(rename = "ExcludedAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_accounts: Option<Vec<String>>,
    /// <p>The name that you assign to an organization config rule.</p>
    #[serde(rename = "OrganizationConfigRuleName")]
    pub organization_config_rule_name: String,
    /// <p>An <code>OrganizationCustomRuleMetadata</code> object.</p>
    #[serde(rename = "OrganizationCustomRuleMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_custom_rule_metadata: Option<OrganizationCustomRuleMetadata>,
    /// <p>An <code>OrganizationManagedRuleMetadata</code> object. </p>
    #[serde(rename = "OrganizationManagedRuleMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_managed_rule_metadata: Option<OrganizationManagedRuleMetadata>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutOrganizationConfigRuleResponse {
    /// <p>The Amazon Resource Name (ARN) of an organization config rule.</p>
    #[serde(rename = "OrganizationConfigRuleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_config_rule_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutOrganizationConformancePackRequest {
    /// <p>A list of <code>ConformancePackInputParameter</code> objects.</p>
    #[serde(rename = "ConformancePackInputParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conformance_pack_input_parameters: Option<Vec<ConformancePackInputParameter>>,
    /// <p>Location of an Amazon S3 bucket where AWS Config can deliver evaluation results. AWS Config stores intermediate files while processing conformance pack template. </p> <p>The delivery bucket name should start with awsconfigconforms. For example: "Resource": "arn:aws:s3:::your_bucket_name/*". For more information, see <a href="https://docs.aws.amazon.com/config/latest/developerguide/conformance-pack-organization-apis.html">Permissions for cross account bucket access</a>.</p>
    #[serde(rename = "DeliveryS3Bucket")]
    pub delivery_s3_bucket: String,
    /// <p>The prefix for the Amazon S3 bucket.</p>
    #[serde(rename = "DeliveryS3KeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_s3_key_prefix: Option<String>,
    /// <p>A list of AWS accounts to be excluded from an organization conformance pack while deploying a conformance pack.</p>
    #[serde(rename = "ExcludedAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_accounts: Option<Vec<String>>,
    /// <p>Name of the organization conformance pack you want to create.</p>
    #[serde(rename = "OrganizationConformancePackName")]
    pub organization_conformance_pack_name: String,
    /// <p>A string containing full conformance pack template body. Structure containing the template body with a minimum length of 1 byte and a maximum length of 51,200 bytes.</p>
    #[serde(rename = "TemplateBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_body: Option<String>,
    /// <p><p>Location of file containing the template body. The uri must point to the conformance pack template (max size: 300 KB).</p> <note> <p>You must have access to read Amazon S3 bucket.</p> </note></p>
    #[serde(rename = "TemplateS3Uri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_s3_uri: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutOrganizationConformancePackResponse {
    /// <p>ARN of the organization conformance pack.</p>
    #[serde(rename = "OrganizationConformancePackArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_conformance_pack_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutRemediationConfigurationsRequest {
    /// <p>A list of remediation configuration objects.</p>
    #[serde(rename = "RemediationConfigurations")]
    pub remediation_configurations: Vec<RemediationConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutRemediationConfigurationsResponse {
    /// <p>Returns a list of failed remediation batch objects.</p>
    #[serde(rename = "FailedBatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_batches: Option<Vec<FailedRemediationBatch>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutRemediationExceptionsRequest {
    /// <p>The name of the AWS Config rule for which you want to create remediation exception.</p>
    #[serde(rename = "ConfigRuleName")]
    pub config_rule_name: String,
    /// <p>The exception is automatically deleted after the expiration date.</p>
    #[serde(rename = "ExpirationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<f64>,
    /// <p>The message contains an explanation of the exception.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>An exception list of resource exception keys to be processed with the current request. AWS Config adds exception for each resource key. For example, AWS Config adds 3 exceptions for 3 resource keys. </p>
    #[serde(rename = "ResourceKeys")]
    pub resource_keys: Vec<RemediationExceptionResourceKey>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutRemediationExceptionsResponse {
    /// <p>Returns a list of failed remediation exceptions batch objects. Each object in the batch consists of a list of failed items and failure messages.</p>
    #[serde(rename = "FailedBatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_batches: Option<Vec<FailedRemediationExceptionBatch>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutResourceConfigRequest {
    /// <p><p>The configuration object of the resource in valid JSON format. It must match the schema registered with AWS CloudFormation.</p> <note> <p>The configuration JSON must not exceed 64 KB.</p> </note></p>
    #[serde(rename = "Configuration")]
    pub configuration: String,
    /// <p>Unique identifier of the resource.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>Name of the resource.</p>
    #[serde(rename = "ResourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    /// <p><p>The type of the resource. The custom resource type must be registered with AWS CloudFormation. </p> <note> <p>You cannot use the organization names “aws”, “amzn”, “amazon”, “alexa”, “custom” with custom resource types. It is the first part of the ResourceType up to the first ::.</p> </note></p>
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// <p>Version of the schema registered for the ResourceType in AWS CloudFormation.</p>
    #[serde(rename = "SchemaVersionId")]
    pub schema_version_id: String,
    /// <p>Tags associated with the resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutRetentionConfigurationRequest {
    /// <p><p>Number of days AWS Config stores your historical information.</p> <note> <p>Currently, only applicable to the configuration item history.</p> </note></p>
    #[serde(rename = "RetentionPeriodInDays")]
    pub retention_period_in_days: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutRetentionConfigurationResponse {
    /// <p>Returns a retention configuration object.</p>
    #[serde(rename = "RetentionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_configuration: Option<RetentionConfiguration>,
}

/// <p>Details about the query.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct QueryInfo {
    /// <p>Returns a <code>FieldInfo</code> object.</p>
    #[serde(rename = "SelectFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select_fields: Option<Vec<FieldInfo>>,
}

/// <p>Specifies the types of AWS resource for which AWS Config records configuration changes.</p> <p>In the recording group, you specify whether all supported types or specific types of resources are recorded.</p> <p>By default, AWS Config records configuration changes for all supported types of regional resources that AWS Config discovers in the region in which it is running. Regional resources are tied to a region and can be used only in that region. Examples of regional resources are EC2 instances and EBS volumes.</p> <p>You can also have AWS Config record configuration changes for supported types of global resources (for example, IAM resources). Global resources are not tied to an individual region and can be used in all regions.</p> <important> <p>The configuration details for any global resource are the same in all regions. If you customize AWS Config in multiple regions to record global resources, it will create multiple configuration items each time a global resource changes: one configuration item for each region. These configuration items will contain identical data. To prevent duplicate configuration items, you should consider customizing AWS Config in only one region to record global resources, unless you want the configuration items to be available in multiple regions.</p> </important> <p>If you don't want AWS Config to record all resources, you can specify which types of resources it will record with the <code>resourceTypes</code> parameter.</p> <p>For a list of supported resource types, see <a href="https://docs.aws.amazon.com/config/latest/developerguide/resource-config-reference.html#supported-resources">Supported Resource Types</a>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/config/latest/developerguide/select-resources.html">Selecting Which Resources AWS Config Records</a>.</p>
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
    /// <p>A comma-separated list that specifies the types of AWS resources for which AWS Config records configuration changes (for example, <code>AWS::EC2::Instance</code> or <code>AWS::CloudTrail::Trail</code>).</p> <p>Before you can set this option to <code>true</code>, you must set the <code>allSupported</code> option to <code>false</code>.</p> <p>If you set this option to <code>true</code>, when AWS Config adds support for a new type of resource, it will not record resources of that type unless you manually add that type to your recording group.</p> <p>For a list of valid <code>resourceTypes</code> values, see the <b>resourceType Value</b> column in <a href="https://docs.aws.amazon.com/config/latest/developerguide/resource-config-reference.html#supported-resources">Supported AWS Resource Types</a>.</p>
    #[serde(rename = "resourceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types: Option<Vec<String>>,
}

/// <p>The relationship of the related resource to the main resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>An object that represents the details about the remediation configuration that includes the remediation action, parameters, and data to execute the action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemediationConfiguration {
    /// <p>Amazon Resource Name (ARN) of remediation configuration.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The remediation is triggered automatically.</p>
    #[serde(rename = "Automatic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic: Option<bool>,
    /// <p>The name of the AWS Config rule.</p>
    #[serde(rename = "ConfigRuleName")]
    pub config_rule_name: String,
    /// <p>Name of the service that owns the service linked rule, if applicable.</p>
    #[serde(rename = "CreatedByService")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_service: Option<String>,
    /// <p>An ExecutionControls object.</p>
    #[serde(rename = "ExecutionControls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_controls: Option<ExecutionControls>,
    /// <p>The maximum number of failed attempts for auto-remediation. If you do not select a number, the default is 5.</p> <p>For example, if you specify MaximumAutomaticAttempts as 5 with RetryAttemptsSeconds as 50 seconds, AWS Config throws an exception after the 5th failed attempt within 50 seconds.</p>
    #[serde(rename = "MaximumAutomaticAttempts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_automatic_attempts: Option<i64>,
    /// <p>An object of the RemediationParameterValue.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, RemediationParameterValue>>,
    /// <p>The type of a resource. </p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>Maximum time in seconds that AWS Config runs auto-remediation. If you do not select a number, the default is 60 seconds. </p> <p>For example, if you specify RetryAttemptsSeconds as 50 seconds and MaximumAutomaticAttempts as 5, AWS Config will run auto-remediations 5 times within 50 seconds before throwing an exception. </p>
    #[serde(rename = "RetryAttemptSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_attempt_seconds: Option<i64>,
    /// <p>Target ID is the name of the public document.</p>
    #[serde(rename = "TargetId")]
    pub target_id: String,
    /// <p>The type of the target. Target executes remediation. For example, SSM document.</p>
    #[serde(rename = "TargetType")]
    pub target_type: String,
    /// <p>Version of the target. For example, version of the SSM document.</p>
    #[serde(rename = "TargetVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_version: Option<String>,
}

/// <p>An object that represents the details about the remediation exception. The details include the rule name, an explanation of an exception, the time when the exception will be deleted, the resource ID, and resource type. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RemediationException {
    /// <p>The name of the AWS Config rule.</p>
    #[serde(rename = "ConfigRuleName")]
    pub config_rule_name: String,
    /// <p>The time when the remediation exception will be deleted.</p>
    #[serde(rename = "ExpirationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<f64>,
    /// <p>An explanation of an remediation exception.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The ID of the resource (for example., sg-xxxxxx).</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>The type of a resource.</p>
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
}

/// <p>The details that identify a resource within AWS Config, including the resource type and resource ID. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemediationExceptionResourceKey {
    /// <p>The ID of the resource (for example., sg-xxxxxx).</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>The type of a resource.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

/// <p>Provides details of the current status of the invoked remediation action for that resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RemediationExecutionStatus {
    /// <p>Start time when the remediation was executed.</p>
    #[serde(rename = "InvocationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_time: Option<f64>,
    /// <p>The time when the remediation execution was last updated.</p>
    #[serde(rename = "LastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "ResourceKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_key: Option<ResourceKey>,
    /// <p>ENUM of the values.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>Details of every step.</p>
    #[serde(rename = "StepDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_details: Option<Vec<RemediationExecutionStep>>,
}

/// <p>Name of the step from the SSM document.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RemediationExecutionStep {
    /// <p>An error message if the step was interrupted during execution.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The details of the step.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The time when the step started.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The valid status of the step.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The time when the step stopped.</p>
    #[serde(rename = "StopTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_time: Option<f64>,
}

/// <p>The value is either a dynamic (resource) value or a static value. You must select either a dynamic value or a static value.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemediationParameterValue {
    /// <p>The value is dynamic and changes at run-time.</p>
    #[serde(rename = "ResourceValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_value: Option<ResourceValue>,
    /// <p>The value is static and does not change at run-time.</p>
    #[serde(rename = "StaticValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_value: Option<StaticValue>,
}

/// <p>An object that contains the resource type and the number of resources.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>Filters the resource count based on account ID, region, and resource type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ResourceCountFilters {
    /// <p>The 12-digit ID of the account.</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The region where the account is located.</p>
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>The type of the AWS resource.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

/// <p>Filters the results by resource account ID, region, resource ID, and resource name.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ResourceFilters {
    /// <p>The 12-digit source account ID.</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The source region.</p>
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>The ID of the resource.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>The name of the resource.</p>
    #[serde(rename = "ResourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
}

/// <p>The details that identify a resource that is discovered by AWS Config, including the resource type, ID, and (if available) the custom resource name.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>The dynamic value of the resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceValue {
    /// <p>The value is a resource ID.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

/// <p>An object with the name of the retention configuration and the retention period in days. The object stores the configuration for data retention in AWS Config.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RetentionConfiguration {
    /// <p>The name of the retention configuration object.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p><p>Number of days AWS Config stores your historical information.</p> <note> <p>Currently, only applicable to the configuration item history.</p> </note></p>
    #[serde(rename = "RetentionPeriodInDays")]
    pub retention_period_in_days: i64,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SelectResourceConfigRequest {
    /// <p>The SQL query <code>SELECT</code> command.</p>
    #[serde(rename = "Expression")]
    pub expression: String,
    /// <p>The maximum number of query results returned on each page. </p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <code>nextToken</code> string returned in a previous request that you use to request the next page of results in a paginated response. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SelectResourceConfigResponse {
    /// <p>The <code>nextToken</code> string returned in a previous request that you use to request the next page of results in a paginated response. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Returns the <code>QueryInfo</code> object.</p>
    #[serde(rename = "QueryInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_info: Option<QueryInfo>,
    /// <p>Returns the results for the SQL query.</p>
    #[serde(rename = "Results")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<String>>,
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
    /// <p>For AWS Config managed rules, a predefined identifier from a list. For example, <code>IAM_PASSWORD_POLICY</code> is a managed rule. To reference a managed rule, see <a href="https://docs.aws.amazon.com/config/latest/developerguide/evaluate-config_use-managed-rules.html">Using AWS Managed Config Rules</a>.</p> <p>For custom rules, the identifier is the Amazon Resource Name (ARN) of the rule's AWS Lambda function, such as <code>arn:aws:lambda:us-east-2:123456789012:function:custom_rule_name</code>.</p>
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
    /// <p>The type of notification that triggers AWS Config to run an evaluation for a rule. You can specify the following notification types:</p> <ul> <li> <p> <code>ConfigurationItemChangeNotification</code> - Triggers an evaluation when AWS Config delivers a configuration item as a result of a resource change.</p> </li> <li> <p> <code>OversizedConfigurationItemChangeNotification</code> - Triggers an evaluation when AWS Config delivers an oversized configuration item. AWS Config may generate this notification type when a resource changes and the notification exceeds the maximum size allowed by Amazon SNS.</p> </li> <li> <p> <code>ScheduledNotification</code> - Triggers a periodic evaluation at the frequency specified for <code>MaximumExecutionFrequency</code>.</p> </li> <li> <p> <code>ConfigurationSnapshotDeliveryCompleted</code> - Triggers a periodic evaluation when AWS Config delivers a configuration snapshot.</p> </li> </ul> <p>If you want your custom rule to be triggered by configuration changes, specify two SourceDetail objects, one for <code>ConfigurationItemChangeNotification</code> and one for <code>OversizedConfigurationItemChangeNotification</code>.</p>
    #[serde(rename = "MessageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
}

/// <p>AWS Systems Manager (SSM) specific remediation controls.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SsmControls {
    /// <p>The maximum percentage of remediation actions allowed to run in parallel on the non-compliant resources for that specific rule. You can specify a percentage, such as 10%. The default value is 10. </p>
    #[serde(rename = "ConcurrentExecutionRatePercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrent_execution_rate_percentage: Option<i64>,
    /// <p>The percentage of errors that are allowed before SSM stops running automations on non-compliant resources for that specific rule. You can specify a percentage of errors, for example 10%. If you do not specifiy a percentage, the default is 50%. For example, if you set the ErrorPercentage to 40% for 10 non-compliant resources, then SSM stops running the automations when the fifth error is received. </p>
    #[serde(rename = "ErrorPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_percentage: Option<i64>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartConfigRulesEvaluationRequest {
    /// <p>The list of names of AWS Config rules that you want to run evaluations for.</p>
    #[serde(rename = "ConfigRuleNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_names: Option<Vec<String>>,
}

/// <p>The output when you start the evaluation for the specified AWS Config rule.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartConfigRulesEvaluationResponse {}

/// <p>The input for the <a>StartConfigurationRecorder</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartConfigurationRecorderRequest {
    /// <p>The name of the recorder object that records each configuration change made to the resources.</p>
    #[serde(rename = "ConfigurationRecorderName")]
    pub configuration_recorder_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartRemediationExecutionRequest {
    /// <p>The list of names of AWS Config rules that you want to run remediation execution for.</p>
    #[serde(rename = "ConfigRuleName")]
    pub config_rule_name: String,
    /// <p>A list of resource keys to be processed with the current request. Each element in the list consists of the resource type and resource ID. </p>
    #[serde(rename = "ResourceKeys")]
    pub resource_keys: Vec<ResourceKey>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartRemediationExecutionResponse {
    /// <p>For resources that have failed to start execution, the API returns a resource key object.</p>
    #[serde(rename = "FailedItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_items: Option<Vec<ResourceKey>>,
    /// <p>Returns a failure message. For example, the resource is already compliant.</p>
    #[serde(rename = "FailureMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
}

/// <p>The static value of the resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StaticValue {
    /// <p>A list of values. For example, the ARN of the assumed role. </p>
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}

/// <p>Status filter object to filter results based on specific member account ID or status type for an organization config rule. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StatusDetailFilters {
    /// <p>The 12-digit account ID of the member account within an organization.</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p><p>Indicates deployment status for config rule in the member account. When master account calls <code>PutOrganizationConfigRule</code> action for the first time, config rule status is created in the member account. When master account calls <code>PutOrganizationConfigRule</code> action for the second time, config rule status is updated in the member account. Config rule status is deleted when the master account deletes <code>OrganizationConfigRule</code> and disables service access for <code>config-multiaccountsetup.amazonaws.com</code>. </p> <p>AWS Config sets the state of the rule to:</p> <ul> <li> <p> <code>CREATE<em>SUCCESSFUL</code> when config rule has been created in the member account.</p> </li> <li> <p> <code>CREATE</em>IN<em>PROGRESS</code> when config rule is being created in the member account.</p> </li> <li> <p> <code>CREATE</em>FAILED</code> when config rule creation has failed in the member account.</p> </li> <li> <p> <code>DELETE<em>FAILED</code> when config rule deletion has failed in the member account.</p> </li> <li> <p> <code>DELETE</em>IN<em>PROGRESS</code> when config rule is being deleted in the member account.</p> </li> <li> <p> <code>DELETE</em>SUCCESSFUL</code> when config rule has been deleted in the member account.</p> </li> <li> <p> <code>UPDATE<em>SUCCESSFUL</code> when config rule has been updated in the member account.</p> </li> <li> <p> <code>UPDATE</em>IN<em>PROGRESS</code> when config rule is being updated in the member account.</p> </li> <li> <p> <code>UPDATE</em>FAILED</code> when config rule deletion has failed in the member account.</p> </li> </ul></p>
    #[serde(rename = "MemberAccountRuleStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_account_rule_status: Option<String>,
}

/// <p>The input for the <a>StopConfigurationRecorder</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopConfigurationRecorderRequest {
    /// <p>The name of the recorder object that records each configuration change made to the resources.</p>
    #[serde(rename = "ConfigurationRecorderName")]
    pub configuration_recorder_name: String,
}

/// <p>The tags for the resource. The metadata that you apply to a resource to help you categorize and organize them. Each tag consists of a key and an optional value, both of which you define. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>One part of a key-value pair that make up a tag. A key is a general label that acts like a category for more specific tag values.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The optional part of a key-value pair that make up a tag. A value acts as a descriptor within a tag category (key).</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) that identifies the resource for which to list the tags. Currently, the supported resources are <code>ConfigRule</code>, <code>ConfigurationAggregator</code> and <code>AggregatorAuthorization</code>.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>An array of tag object.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) that identifies the resource for which to list the tags. Currently, the supported resources are <code>ConfigRule</code>, <code>ConfigurationAggregator</code> and <code>AggregatorAuthorization</code>.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The keys of the tags to be removed.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

/// Errors returned by BatchGetAggregateResourceConfig
#[derive(Debug, PartialEq)]
pub enum BatchGetAggregateResourceConfigError {
    /// <p>You have specified a configuration aggregator that does not exist.</p>
    NoSuchConfigurationAggregator(String),
}

impl BatchGetAggregateResourceConfigError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<BatchGetAggregateResourceConfigError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NoSuchConfigurationAggregatorException" => {
                    return RusotoError::Service(
                        BatchGetAggregateResourceConfigError::NoSuchConfigurationAggregator(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchGetAggregateResourceConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchGetAggregateResourceConfigError::NoSuchConfigurationAggregator(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for BatchGetAggregateResourceConfigError {}
/// Errors returned by BatchGetResourceConfig
#[derive(Debug, PartialEq)]
pub enum BatchGetResourceConfigError {
    /// <p>There are no configuration recorders available to provide the role needed to describe your resources. Create a configuration recorder.</p>
    NoAvailableConfigurationRecorder(String),
}

impl BatchGetResourceConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchGetResourceConfigError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NoAvailableConfigurationRecorderException" => {
                    return RusotoError::Service(
                        BatchGetResourceConfigError::NoAvailableConfigurationRecorder(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchGetResourceConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchGetResourceConfigError::NoAvailableConfigurationRecorder(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for BatchGetResourceConfigError {}
/// Errors returned by DeleteAggregationAuthorization
#[derive(Debug, PartialEq)]
pub enum DeleteAggregationAuthorizationError {
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
}

impl DeleteAggregationAuthorizationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteAggregationAuthorizationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DeleteAggregationAuthorizationError::InvalidParameterValue(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteAggregationAuthorizationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteAggregationAuthorizationError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteAggregationAuthorizationError {}
/// Errors returned by DeleteConfigRule
#[derive(Debug, PartialEq)]
pub enum DeleteConfigRuleError {
    /// <p>One or more AWS Config rules in the request are invalid. Verify that the rule names are correct and try again.</p>
    NoSuchConfigRule(String),
    /// <p><p>You see this exception in the following cases: </p> <ul> <li> <p>For DeleteConfigRule, AWS Config is deleting this rule. Try your request again later.</p> </li> <li> <p>For DeleteConfigRule, the rule is deleting your evaluation results. Try your request again later.</p> </li> <li> <p>For DeleteConfigRule, a remediation action is associated with the rule and AWS Config cannot delete this rule. Delete the remediation action associated with the rule before deleting the rule and try your request again later.</p> </li> <li> <p>For PutConfigOrganizationRule, organization config rule deletion is in progress. Try your request again later.</p> </li> <li> <p>For DeleteOrganizationConfigRule, organization config rule creation is in progress. Try your request again later.</p> </li> <li> <p>For PutConformancePack and PutOrganizationConformancePack, a conformance pack creation, update, and deletion is in progress. Try your request again later.</p> </li> <li> <p>For DeleteConformancePack, a conformance pack creation, update, and deletion is in progress. Try your request again later.</p> </li> </ul></p>
    ResourceInUse(String),
}

impl DeleteConfigRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteConfigRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NoSuchConfigRuleException" => {
                    return RusotoError::Service(DeleteConfigRuleError::NoSuchConfigRule(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteConfigRuleError::ResourceInUse(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteConfigRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteConfigRuleError::NoSuchConfigRule(ref cause) => write!(f, "{}", cause),
            DeleteConfigRuleError::ResourceInUse(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteConfigRuleError {}
/// Errors returned by DeleteConfigurationAggregator
#[derive(Debug, PartialEq)]
pub enum DeleteConfigurationAggregatorError {
    /// <p>You have specified a configuration aggregator that does not exist.</p>
    NoSuchConfigurationAggregator(String),
}

impl DeleteConfigurationAggregatorError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteConfigurationAggregatorError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NoSuchConfigurationAggregatorException" => {
                    return RusotoError::Service(
                        DeleteConfigurationAggregatorError::NoSuchConfigurationAggregator(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteConfigurationAggregatorError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteConfigurationAggregatorError::NoSuchConfigurationAggregator(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteConfigurationAggregatorError {}
/// Errors returned by DeleteConfigurationRecorder
#[derive(Debug, PartialEq)]
pub enum DeleteConfigurationRecorderError {
    /// <p>You have specified a configuration recorder that does not exist.</p>
    NoSuchConfigurationRecorder(String),
}

impl DeleteConfigurationRecorderError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteConfigurationRecorderError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NoSuchConfigurationRecorderException" => {
                    return RusotoError::Service(
                        DeleteConfigurationRecorderError::NoSuchConfigurationRecorder(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteConfigurationRecorderError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteConfigurationRecorderError::NoSuchConfigurationRecorder(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteConfigurationRecorderError {}
/// Errors returned by DeleteConformancePack
#[derive(Debug, PartialEq)]
pub enum DeleteConformancePackError {
    /// <p>You specified one or more conformance packs that do not exist.</p>
    NoSuchConformancePack(String),
    /// <p><p>You see this exception in the following cases: </p> <ul> <li> <p>For DeleteConfigRule, AWS Config is deleting this rule. Try your request again later.</p> </li> <li> <p>For DeleteConfigRule, the rule is deleting your evaluation results. Try your request again later.</p> </li> <li> <p>For DeleteConfigRule, a remediation action is associated with the rule and AWS Config cannot delete this rule. Delete the remediation action associated with the rule before deleting the rule and try your request again later.</p> </li> <li> <p>For PutConfigOrganizationRule, organization config rule deletion is in progress. Try your request again later.</p> </li> <li> <p>For DeleteOrganizationConfigRule, organization config rule creation is in progress. Try your request again later.</p> </li> <li> <p>For PutConformancePack and PutOrganizationConformancePack, a conformance pack creation, update, and deletion is in progress. Try your request again later.</p> </li> <li> <p>For DeleteConformancePack, a conformance pack creation, update, and deletion is in progress. Try your request again later.</p> </li> </ul></p>
    ResourceInUse(String),
}

impl DeleteConformancePackError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteConformancePackError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NoSuchConformancePackException" => {
                    return RusotoError::Service(DeleteConformancePackError::NoSuchConformancePack(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteConformancePackError::ResourceInUse(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteConformancePackError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteConformancePackError::NoSuchConformancePack(ref cause) => write!(f, "{}", cause),
            DeleteConformancePackError::ResourceInUse(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteConformancePackError {}
/// Errors returned by DeleteDeliveryChannel
#[derive(Debug, PartialEq)]
pub enum DeleteDeliveryChannelError {
    /// <p>You cannot delete the delivery channel you specified because the configuration recorder is running.</p>
    LastDeliveryChannelDeleteFailed(String),
    /// <p>You have specified a delivery channel that does not exist.</p>
    NoSuchDeliveryChannel(String),
}

impl DeleteDeliveryChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDeliveryChannelError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "LastDeliveryChannelDeleteFailedException" => {
                    return RusotoError::Service(
                        DeleteDeliveryChannelError::LastDeliveryChannelDeleteFailed(err.msg),
                    )
                }
                "NoSuchDeliveryChannelException" => {
                    return RusotoError::Service(DeleteDeliveryChannelError::NoSuchDeliveryChannel(
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
impl fmt::Display for DeleteDeliveryChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDeliveryChannelError::LastDeliveryChannelDeleteFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteDeliveryChannelError::NoSuchDeliveryChannel(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDeliveryChannelError {}
/// Errors returned by DeleteEvaluationResults
#[derive(Debug, PartialEq)]
pub enum DeleteEvaluationResultsError {
    /// <p>One or more AWS Config rules in the request are invalid. Verify that the rule names are correct and try again.</p>
    NoSuchConfigRule(String),
    /// <p><p>You see this exception in the following cases: </p> <ul> <li> <p>For DeleteConfigRule, AWS Config is deleting this rule. Try your request again later.</p> </li> <li> <p>For DeleteConfigRule, the rule is deleting your evaluation results. Try your request again later.</p> </li> <li> <p>For DeleteConfigRule, a remediation action is associated with the rule and AWS Config cannot delete this rule. Delete the remediation action associated with the rule before deleting the rule and try your request again later.</p> </li> <li> <p>For PutConfigOrganizationRule, organization config rule deletion is in progress. Try your request again later.</p> </li> <li> <p>For DeleteOrganizationConfigRule, organization config rule creation is in progress. Try your request again later.</p> </li> <li> <p>For PutConformancePack and PutOrganizationConformancePack, a conformance pack creation, update, and deletion is in progress. Try your request again later.</p> </li> <li> <p>For DeleteConformancePack, a conformance pack creation, update, and deletion is in progress. Try your request again later.</p> </li> </ul></p>
    ResourceInUse(String),
}

impl DeleteEvaluationResultsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteEvaluationResultsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NoSuchConfigRuleException" => {
                    return RusotoError::Service(DeleteEvaluationResultsError::NoSuchConfigRule(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteEvaluationResultsError::ResourceInUse(
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
impl fmt::Display for DeleteEvaluationResultsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteEvaluationResultsError::NoSuchConfigRule(ref cause) => write!(f, "{}", cause),
            DeleteEvaluationResultsError::ResourceInUse(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteEvaluationResultsError {}
/// Errors returned by DeleteOrganizationConfigRule
#[derive(Debug, PartialEq)]
pub enum DeleteOrganizationConfigRuleError {
    /// <p>You specified one or more organization config rules that do not exist.</p>
    NoSuchOrganizationConfigRule(String),
    /// <p>For PutConfigAggregator API, no permission to call EnableAWSServiceAccess API.</p> <p>For all OrganizationConfigRule and OrganizationConformancePack APIs, AWS Config throws an exception if APIs are called from member accounts. All APIs must be called from organization master account.</p>
    OrganizationAccessDenied(String),
    /// <p><p>You see this exception in the following cases: </p> <ul> <li> <p>For DeleteConfigRule, AWS Config is deleting this rule. Try your request again later.</p> </li> <li> <p>For DeleteConfigRule, the rule is deleting your evaluation results. Try your request again later.</p> </li> <li> <p>For DeleteConfigRule, a remediation action is associated with the rule and AWS Config cannot delete this rule. Delete the remediation action associated with the rule before deleting the rule and try your request again later.</p> </li> <li> <p>For PutConfigOrganizationRule, organization config rule deletion is in progress. Try your request again later.</p> </li> <li> <p>For DeleteOrganizationConfigRule, organization config rule creation is in progress. Try your request again later.</p> </li> <li> <p>For PutConformancePack and PutOrganizationConformancePack, a conformance pack creation, update, and deletion is in progress. Try your request again later.</p> </li> <li> <p>For DeleteConformancePack, a conformance pack creation, update, and deletion is in progress. Try your request again later.</p> </li> </ul></p>
    ResourceInUse(String),
}

impl DeleteOrganizationConfigRuleError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteOrganizationConfigRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NoSuchOrganizationConfigRuleException" => {
                    return RusotoError::Service(
                        DeleteOrganizationConfigRuleError::NoSuchOrganizationConfigRule(err.msg),
                    )
                }
                "OrganizationAccessDeniedException" => {
                    return RusotoError::Service(
                        DeleteOrganizationConfigRuleError::OrganizationAccessDenied(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteOrganizationConfigRuleError::ResourceInUse(
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
impl fmt::Display for DeleteOrganizationConfigRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteOrganizationConfigRuleError::NoSuchOrganizationConfigRule(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteOrganizationConfigRuleError::OrganizationAccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteOrganizationConfigRuleError::ResourceInUse(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteOrganizationConfigRuleError {}
/// Errors returned by DeleteOrganizationConformancePack
#[derive(Debug, PartialEq)]
pub enum DeleteOrganizationConformancePackError {
    /// <p>AWS Config organization conformance pack that you passed in the filter does not exist.</p> <p>For DeleteOrganizationConformancePack, you tried to delete an organization conformance pack that does not exist.</p>
    NoSuchOrganizationConformancePack(String),
    /// <p>For PutConfigAggregator API, no permission to call EnableAWSServiceAccess API.</p> <p>For all OrganizationConfigRule and OrganizationConformancePack APIs, AWS Config throws an exception if APIs are called from member accounts. All APIs must be called from organization master account.</p>
    OrganizationAccessDenied(String),
    /// <p><p>You see this exception in the following cases: </p> <ul> <li> <p>For DeleteConfigRule, AWS Config is deleting this rule. Try your request again later.</p> </li> <li> <p>For DeleteConfigRule, the rule is deleting your evaluation results. Try your request again later.</p> </li> <li> <p>For DeleteConfigRule, a remediation action is associated with the rule and AWS Config cannot delete this rule. Delete the remediation action associated with the rule before deleting the rule and try your request again later.</p> </li> <li> <p>For PutConfigOrganizationRule, organization config rule deletion is in progress. Try your request again later.</p> </li> <li> <p>For DeleteOrganizationConfigRule, organization config rule creation is in progress. Try your request again later.</p> </li> <li> <p>For PutConformancePack and PutOrganizationConformancePack, a conformance pack creation, update, and deletion is in progress. Try your request again later.</p> </li> <li> <p>For DeleteConformancePack, a conformance pack creation, update, and deletion is in progress. Try your request again later.</p> </li> </ul></p>
    ResourceInUse(String),
}

impl DeleteOrganizationConformancePackError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteOrganizationConformancePackError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NoSuchOrganizationConformancePackException" => {
                    return RusotoError::Service(
                        DeleteOrganizationConformancePackError::NoSuchOrganizationConformancePack(
                            err.msg,
                        ),
                    )
                }
                "OrganizationAccessDeniedException" => {
                    return RusotoError::Service(
                        DeleteOrganizationConformancePackError::OrganizationAccessDenied(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(
                        DeleteOrganizationConformancePackError::ResourceInUse(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteOrganizationConformancePackError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteOrganizationConformancePackError::NoSuchOrganizationConformancePack(
                ref cause,
            ) => write!(f, "{}", cause),
            DeleteOrganizationConformancePackError::OrganizationAccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteOrganizationConformancePackError::ResourceInUse(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteOrganizationConformancePackError {}
/// Errors returned by DeletePendingAggregationRequest
#[derive(Debug, PartialEq)]
pub enum DeletePendingAggregationRequestError {
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
}

impl DeletePendingAggregationRequestError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeletePendingAggregationRequestError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DeletePendingAggregationRequestError::InvalidParameterValue(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeletePendingAggregationRequestError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeletePendingAggregationRequestError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeletePendingAggregationRequestError {}
/// Errors returned by DeleteRemediationConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteRemediationConfigurationError {
    /// <p>You specified an AWS Config rule without a remediation configuration.</p>
    NoSuchRemediationConfiguration(String),
    /// <p>Remediation action is in progress. You can either cancel execution in AWS Systems Manager or wait and try again later. </p>
    RemediationInProgress(String),
}

impl DeleteRemediationConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteRemediationConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NoSuchRemediationConfigurationException" => {
                    return RusotoError::Service(
                        DeleteRemediationConfigurationError::NoSuchRemediationConfiguration(
                            err.msg,
                        ),
                    )
                }
                "RemediationInProgressException" => {
                    return RusotoError::Service(
                        DeleteRemediationConfigurationError::RemediationInProgress(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteRemediationConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteRemediationConfigurationError::NoSuchRemediationConfiguration(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteRemediationConfigurationError::RemediationInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteRemediationConfigurationError {}
/// Errors returned by DeleteRemediationExceptions
#[derive(Debug, PartialEq)]
pub enum DeleteRemediationExceptionsError {
    /// <p>You tried to delete a remediation exception that does not exist.</p>
    NoSuchRemediation(String),
}

impl DeleteRemediationExceptionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteRemediationExceptionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NoSuchRemediationExceptionException" => {
                    return RusotoError::Service(
                        DeleteRemediationExceptionsError::NoSuchRemediation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteRemediationExceptionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteRemediationExceptionsError::NoSuchRemediation(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteRemediationExceptionsError {}
/// Errors returned by DeleteResourceConfig
#[derive(Debug, PartialEq)]
pub enum DeleteResourceConfigError {
    /// <p>There is no configuration recorder running.</p>
    NoRunningConfigurationRecorder(String),
}

impl DeleteResourceConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteResourceConfigError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NoRunningConfigurationRecorderException" => {
                    return RusotoError::Service(
                        DeleteResourceConfigError::NoRunningConfigurationRecorder(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteResourceConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteResourceConfigError::NoRunningConfigurationRecorder(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteResourceConfigError {}
/// Errors returned by DeleteRetentionConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteRetentionConfigurationError {
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
    /// <p>You have specified a retention configuration that does not exist.</p>
    NoSuchRetentionConfiguration(String),
}

impl DeleteRetentionConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteRetentionConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DeleteRetentionConfigurationError::InvalidParameterValue(err.msg),
                    )
                }
                "NoSuchRetentionConfigurationException" => {
                    return RusotoError::Service(
                        DeleteRetentionConfigurationError::NoSuchRetentionConfiguration(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteRetentionConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteRetentionConfigurationError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteRetentionConfigurationError::NoSuchRetentionConfiguration(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteRetentionConfigurationError {}
/// Errors returned by DeliverConfigSnapshot
#[derive(Debug, PartialEq)]
pub enum DeliverConfigSnapshotError {
    /// <p>There are no configuration recorders available to provide the role needed to describe your resources. Create a configuration recorder.</p>
    NoAvailableConfigurationRecorder(String),
    /// <p>There is no configuration recorder running.</p>
    NoRunningConfigurationRecorder(String),
    /// <p>You have specified a delivery channel that does not exist.</p>
    NoSuchDeliveryChannel(String),
}

impl DeliverConfigSnapshotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeliverConfigSnapshotError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NoAvailableConfigurationRecorderException" => {
                    return RusotoError::Service(
                        DeliverConfigSnapshotError::NoAvailableConfigurationRecorder(err.msg),
                    )
                }
                "NoRunningConfigurationRecorderException" => {
                    return RusotoError::Service(
                        DeliverConfigSnapshotError::NoRunningConfigurationRecorder(err.msg),
                    )
                }
                "NoSuchDeliveryChannelException" => {
                    return RusotoError::Service(DeliverConfigSnapshotError::NoSuchDeliveryChannel(
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
impl fmt::Display for DeliverConfigSnapshotError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeliverConfigSnapshotError::NoAvailableConfigurationRecorder(ref cause) => {
                write!(f, "{}", cause)
            }
            DeliverConfigSnapshotError::NoRunningConfigurationRecorder(ref cause) => {
                write!(f, "{}", cause)
            }
            DeliverConfigSnapshotError::NoSuchDeliveryChannel(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeliverConfigSnapshotError {}
/// Errors returned by DescribeAggregateComplianceByConfigRules
#[derive(Debug, PartialEq)]
pub enum DescribeAggregateComplianceByConfigRulesError {
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>You have specified a configuration aggregator that does not exist.</p>
    NoSuchConfigurationAggregator(String),
}

impl DescribeAggregateComplianceByConfigRulesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeAggregateComplianceByConfigRulesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidLimitException" => {
                    return RusotoError::Service(
                        DescribeAggregateComplianceByConfigRulesError::InvalidLimit(err.msg),
                    )
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        DescribeAggregateComplianceByConfigRulesError::InvalidNextToken(err.msg),
                    )
                }
                "NoSuchConfigurationAggregatorException" => return RusotoError::Service(
                    DescribeAggregateComplianceByConfigRulesError::NoSuchConfigurationAggregator(
                        err.msg,
                    ),
                ),
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeAggregateComplianceByConfigRulesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeAggregateComplianceByConfigRulesError::InvalidLimit(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeAggregateComplianceByConfigRulesError::InvalidNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeAggregateComplianceByConfigRulesError::NoSuchConfigurationAggregator(
                ref cause,
            ) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeAggregateComplianceByConfigRulesError {}
/// Errors returned by DescribeAggregationAuthorizations
#[derive(Debug, PartialEq)]
pub enum DescribeAggregationAuthorizationsError {
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
}

impl DescribeAggregationAuthorizationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeAggregationAuthorizationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidLimitException" => {
                    return RusotoError::Service(
                        DescribeAggregationAuthorizationsError::InvalidLimit(err.msg),
                    )
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        DescribeAggregationAuthorizationsError::InvalidNextToken(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DescribeAggregationAuthorizationsError::InvalidParameterValue(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeAggregationAuthorizationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeAggregationAuthorizationsError::InvalidLimit(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeAggregationAuthorizationsError::InvalidNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeAggregationAuthorizationsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeAggregationAuthorizationsError {}
/// Errors returned by DescribeComplianceByConfigRule
#[derive(Debug, PartialEq)]
pub enum DescribeComplianceByConfigRuleError {
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
    /// <p>One or more AWS Config rules in the request are invalid. Verify that the rule names are correct and try again.</p>
    NoSuchConfigRule(String),
}

impl DescribeComplianceByConfigRuleError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeComplianceByConfigRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        DescribeComplianceByConfigRuleError::InvalidNextToken(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DescribeComplianceByConfigRuleError::InvalidParameterValue(err.msg),
                    )
                }
                "NoSuchConfigRuleException" => {
                    return RusotoError::Service(
                        DescribeComplianceByConfigRuleError::NoSuchConfigRule(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeComplianceByConfigRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeComplianceByConfigRuleError::InvalidNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeComplianceByConfigRuleError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeComplianceByConfigRuleError::NoSuchConfigRule(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeComplianceByConfigRuleError {}
/// Errors returned by DescribeComplianceByResource
#[derive(Debug, PartialEq)]
pub enum DescribeComplianceByResourceError {
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
}

impl DescribeComplianceByResourceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeComplianceByResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        DescribeComplianceByResourceError::InvalidNextToken(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DescribeComplianceByResourceError::InvalidParameterValue(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeComplianceByResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeComplianceByResourceError::InvalidNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeComplianceByResourceError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeComplianceByResourceError {}
/// Errors returned by DescribeConfigRuleEvaluationStatus
#[derive(Debug, PartialEq)]
pub enum DescribeConfigRuleEvaluationStatusError {
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
    /// <p>One or more AWS Config rules in the request are invalid. Verify that the rule names are correct and try again.</p>
    NoSuchConfigRule(String),
}

impl DescribeConfigRuleEvaluationStatusError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeConfigRuleEvaluationStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        DescribeConfigRuleEvaluationStatusError::InvalidNextToken(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DescribeConfigRuleEvaluationStatusError::InvalidParameterValue(err.msg),
                    )
                }
                "NoSuchConfigRuleException" => {
                    return RusotoError::Service(
                        DescribeConfigRuleEvaluationStatusError::NoSuchConfigRule(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeConfigRuleEvaluationStatusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeConfigRuleEvaluationStatusError::InvalidNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeConfigRuleEvaluationStatusError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeConfigRuleEvaluationStatusError::NoSuchConfigRule(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeConfigRuleEvaluationStatusError {}
/// Errors returned by DescribeConfigRules
#[derive(Debug, PartialEq)]
pub enum DescribeConfigRulesError {
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>One or more AWS Config rules in the request are invalid. Verify that the rule names are correct and try again.</p>
    NoSuchConfigRule(String),
}

impl DescribeConfigRulesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeConfigRulesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(DescribeConfigRulesError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "NoSuchConfigRuleException" => {
                    return RusotoError::Service(DescribeConfigRulesError::NoSuchConfigRule(
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
impl fmt::Display for DescribeConfigRulesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeConfigRulesError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            DescribeConfigRulesError::NoSuchConfigRule(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeConfigRulesError {}
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
}

impl DescribeConfigurationAggregatorSourcesStatusError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeConfigurationAggregatorSourcesStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                                "InvalidLimitException" => return RusotoError::Service(DescribeConfigurationAggregatorSourcesStatusError::InvalidLimit(err.msg)),
"InvalidNextTokenException" => return RusotoError::Service(DescribeConfigurationAggregatorSourcesStatusError::InvalidNextToken(err.msg)),
"InvalidParameterValueException" => return RusotoError::Service(DescribeConfigurationAggregatorSourcesStatusError::InvalidParameterValue(err.msg)),
"NoSuchConfigurationAggregatorException" => return RusotoError::Service(DescribeConfigurationAggregatorSourcesStatusError::NoSuchConfigurationAggregator(err.msg)),
"ValidationException" => return RusotoError::Validation(err.msg),
_ => {}
                            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeConfigurationAggregatorSourcesStatusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeConfigurationAggregatorSourcesStatusError::InvalidLimit(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeConfigurationAggregatorSourcesStatusError::InvalidNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeConfigurationAggregatorSourcesStatusError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeConfigurationAggregatorSourcesStatusError::NoSuchConfigurationAggregator(
                ref cause,
            ) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeConfigurationAggregatorSourcesStatusError {}
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
}

impl DescribeConfigurationAggregatorsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeConfigurationAggregatorsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidLimitException" => {
                    return RusotoError::Service(
                        DescribeConfigurationAggregatorsError::InvalidLimit(err.msg),
                    )
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        DescribeConfigurationAggregatorsError::InvalidNextToken(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DescribeConfigurationAggregatorsError::InvalidParameterValue(err.msg),
                    )
                }
                "NoSuchConfigurationAggregatorException" => {
                    return RusotoError::Service(
                        DescribeConfigurationAggregatorsError::NoSuchConfigurationAggregator(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeConfigurationAggregatorsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeConfigurationAggregatorsError::InvalidLimit(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeConfigurationAggregatorsError::InvalidNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeConfigurationAggregatorsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeConfigurationAggregatorsError::NoSuchConfigurationAggregator(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeConfigurationAggregatorsError {}
/// Errors returned by DescribeConfigurationRecorderStatus
#[derive(Debug, PartialEq)]
pub enum DescribeConfigurationRecorderStatusError {
    /// <p>You have specified a configuration recorder that does not exist.</p>
    NoSuchConfigurationRecorder(String),
}

impl DescribeConfigurationRecorderStatusError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeConfigurationRecorderStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NoSuchConfigurationRecorderException" => {
                    return RusotoError::Service(
                        DescribeConfigurationRecorderStatusError::NoSuchConfigurationRecorder(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeConfigurationRecorderStatusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeConfigurationRecorderStatusError::NoSuchConfigurationRecorder(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeConfigurationRecorderStatusError {}
/// Errors returned by DescribeConfigurationRecorders
#[derive(Debug, PartialEq)]
pub enum DescribeConfigurationRecordersError {
    /// <p>You have specified a configuration recorder that does not exist.</p>
    NoSuchConfigurationRecorder(String),
}

impl DescribeConfigurationRecordersError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeConfigurationRecordersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NoSuchConfigurationRecorderException" => {
                    return RusotoError::Service(
                        DescribeConfigurationRecordersError::NoSuchConfigurationRecorder(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeConfigurationRecordersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeConfigurationRecordersError::NoSuchConfigurationRecorder(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeConfigurationRecordersError {}
/// Errors returned by DescribeConformancePackCompliance
#[derive(Debug, PartialEq)]
pub enum DescribeConformancePackComplianceError {
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
    /// <p>AWS Config rule that you passed in the filter does not exist.</p>
    NoSuchConfigRuleInConformancePack(String),
    /// <p>You specified one or more conformance packs that do not exist.</p>
    NoSuchConformancePack(String),
}

impl DescribeConformancePackComplianceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeConformancePackComplianceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidLimitException" => {
                    return RusotoError::Service(
                        DescribeConformancePackComplianceError::InvalidLimit(err.msg),
                    )
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        DescribeConformancePackComplianceError::InvalidNextToken(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DescribeConformancePackComplianceError::InvalidParameterValue(err.msg),
                    )
                }
                "NoSuchConfigRuleInConformancePackException" => {
                    return RusotoError::Service(
                        DescribeConformancePackComplianceError::NoSuchConfigRuleInConformancePack(
                            err.msg,
                        ),
                    )
                }
                "NoSuchConformancePackException" => {
                    return RusotoError::Service(
                        DescribeConformancePackComplianceError::NoSuchConformancePack(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeConformancePackComplianceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeConformancePackComplianceError::InvalidLimit(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeConformancePackComplianceError::InvalidNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeConformancePackComplianceError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeConformancePackComplianceError::NoSuchConfigRuleInConformancePack(
                ref cause,
            ) => write!(f, "{}", cause),
            DescribeConformancePackComplianceError::NoSuchConformancePack(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeConformancePackComplianceError {}
/// Errors returned by DescribeConformancePackStatus
#[derive(Debug, PartialEq)]
pub enum DescribeConformancePackStatusError {
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
}

impl DescribeConformancePackStatusError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeConformancePackStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidLimitException" => {
                    return RusotoError::Service(DescribeConformancePackStatusError::InvalidLimit(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        DescribeConformancePackStatusError::InvalidNextToken(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeConformancePackStatusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeConformancePackStatusError::InvalidLimit(ref cause) => write!(f, "{}", cause),
            DescribeConformancePackStatusError::InvalidNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeConformancePackStatusError {}
/// Errors returned by DescribeConformancePacks
#[derive(Debug, PartialEq)]
pub enum DescribeConformancePacksError {
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>You specified one or more conformance packs that do not exist.</p>
    NoSuchConformancePack(String),
}

impl DescribeConformancePacksError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeConformancePacksError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidLimitException" => {
                    return RusotoError::Service(DescribeConformancePacksError::InvalidLimit(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(DescribeConformancePacksError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "NoSuchConformancePackException" => {
                    return RusotoError::Service(
                        DescribeConformancePacksError::NoSuchConformancePack(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeConformancePacksError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeConformancePacksError::InvalidLimit(ref cause) => write!(f, "{}", cause),
            DescribeConformancePacksError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            DescribeConformancePacksError::NoSuchConformancePack(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeConformancePacksError {}
/// Errors returned by DescribeDeliveryChannelStatus
#[derive(Debug, PartialEq)]
pub enum DescribeDeliveryChannelStatusError {
    /// <p>You have specified a delivery channel that does not exist.</p>
    NoSuchDeliveryChannel(String),
}

impl DescribeDeliveryChannelStatusError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeDeliveryChannelStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NoSuchDeliveryChannelException" => {
                    return RusotoError::Service(
                        DescribeDeliveryChannelStatusError::NoSuchDeliveryChannel(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeDeliveryChannelStatusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDeliveryChannelStatusError::NoSuchDeliveryChannel(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeDeliveryChannelStatusError {}
/// Errors returned by DescribeDeliveryChannels
#[derive(Debug, PartialEq)]
pub enum DescribeDeliveryChannelsError {
    /// <p>You have specified a delivery channel that does not exist.</p>
    NoSuchDeliveryChannel(String),
}

impl DescribeDeliveryChannelsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeDeliveryChannelsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NoSuchDeliveryChannelException" => {
                    return RusotoError::Service(
                        DescribeDeliveryChannelsError::NoSuchDeliveryChannel(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeDeliveryChannelsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDeliveryChannelsError::NoSuchDeliveryChannel(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeDeliveryChannelsError {}
/// Errors returned by DescribeOrganizationConfigRuleStatuses
#[derive(Debug, PartialEq)]
pub enum DescribeOrganizationConfigRuleStatusesError {
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>You specified one or more organization config rules that do not exist.</p>
    NoSuchOrganizationConfigRule(String),
    /// <p>For PutConfigAggregator API, no permission to call EnableAWSServiceAccess API.</p> <p>For all OrganizationConfigRule and OrganizationConformancePack APIs, AWS Config throws an exception if APIs are called from member accounts. All APIs must be called from organization master account.</p>
    OrganizationAccessDenied(String),
}

impl DescribeOrganizationConfigRuleStatusesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeOrganizationConfigRuleStatusesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidLimitException" => {
                    return RusotoError::Service(
                        DescribeOrganizationConfigRuleStatusesError::InvalidLimit(err.msg),
                    )
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        DescribeOrganizationConfigRuleStatusesError::InvalidNextToken(err.msg),
                    )
                }
                "NoSuchOrganizationConfigRuleException" => {
                    return RusotoError::Service(
                        DescribeOrganizationConfigRuleStatusesError::NoSuchOrganizationConfigRule(
                            err.msg,
                        ),
                    )
                }
                "OrganizationAccessDeniedException" => {
                    return RusotoError::Service(
                        DescribeOrganizationConfigRuleStatusesError::OrganizationAccessDenied(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeOrganizationConfigRuleStatusesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeOrganizationConfigRuleStatusesError::InvalidLimit(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeOrganizationConfigRuleStatusesError::InvalidNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeOrganizationConfigRuleStatusesError::NoSuchOrganizationConfigRule(
                ref cause,
            ) => write!(f, "{}", cause),
            DescribeOrganizationConfigRuleStatusesError::OrganizationAccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeOrganizationConfigRuleStatusesError {}
/// Errors returned by DescribeOrganizationConfigRules
#[derive(Debug, PartialEq)]
pub enum DescribeOrganizationConfigRulesError {
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>You specified one or more organization config rules that do not exist.</p>
    NoSuchOrganizationConfigRule(String),
    /// <p>For PutConfigAggregator API, no permission to call EnableAWSServiceAccess API.</p> <p>For all OrganizationConfigRule and OrganizationConformancePack APIs, AWS Config throws an exception if APIs are called from member accounts. All APIs must be called from organization master account.</p>
    OrganizationAccessDenied(String),
}

impl DescribeOrganizationConfigRulesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeOrganizationConfigRulesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidLimitException" => {
                    return RusotoError::Service(
                        DescribeOrganizationConfigRulesError::InvalidLimit(err.msg),
                    )
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        DescribeOrganizationConfigRulesError::InvalidNextToken(err.msg),
                    )
                }
                "NoSuchOrganizationConfigRuleException" => {
                    return RusotoError::Service(
                        DescribeOrganizationConfigRulesError::NoSuchOrganizationConfigRule(err.msg),
                    )
                }
                "OrganizationAccessDeniedException" => {
                    return RusotoError::Service(
                        DescribeOrganizationConfigRulesError::OrganizationAccessDenied(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeOrganizationConfigRulesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeOrganizationConfigRulesError::InvalidLimit(ref cause) => write!(f, "{}", cause),
            DescribeOrganizationConfigRulesError::InvalidNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeOrganizationConfigRulesError::NoSuchOrganizationConfigRule(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeOrganizationConfigRulesError::OrganizationAccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeOrganizationConfigRulesError {}
/// Errors returned by DescribeOrganizationConformancePackStatuses
#[derive(Debug, PartialEq)]
pub enum DescribeOrganizationConformancePackStatusesError {
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>AWS Config organization conformance pack that you passed in the filter does not exist.</p> <p>For DeleteOrganizationConformancePack, you tried to delete an organization conformance pack that does not exist.</p>
    NoSuchOrganizationConformancePack(String),
    /// <p>For PutConfigAggregator API, no permission to call EnableAWSServiceAccess API.</p> <p>For all OrganizationConfigRule and OrganizationConformancePack APIs, AWS Config throws an exception if APIs are called from member accounts. All APIs must be called from organization master account.</p>
    OrganizationAccessDenied(String),
}

impl DescribeOrganizationConformancePackStatusesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeOrganizationConformancePackStatusesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                                "InvalidLimitException" => return RusotoError::Service(DescribeOrganizationConformancePackStatusesError::InvalidLimit(err.msg)),
"InvalidNextTokenException" => return RusotoError::Service(DescribeOrganizationConformancePackStatusesError::InvalidNextToken(err.msg)),
"NoSuchOrganizationConformancePackException" => return RusotoError::Service(DescribeOrganizationConformancePackStatusesError::NoSuchOrganizationConformancePack(err.msg)),
"OrganizationAccessDeniedException" => return RusotoError::Service(DescribeOrganizationConformancePackStatusesError::OrganizationAccessDenied(err.msg)),
"ValidationException" => return RusotoError::Validation(err.msg),
_ => {}
                            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeOrganizationConformancePackStatusesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeOrganizationConformancePackStatusesError::InvalidLimit(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeOrganizationConformancePackStatusesError::InvalidNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeOrganizationConformancePackStatusesError::NoSuchOrganizationConformancePack(
                ref cause,
            ) => write!(f, "{}", cause),
            DescribeOrganizationConformancePackStatusesError::OrganizationAccessDenied(
                ref cause,
            ) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeOrganizationConformancePackStatusesError {}
/// Errors returned by DescribeOrganizationConformancePacks
#[derive(Debug, PartialEq)]
pub enum DescribeOrganizationConformancePacksError {
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>AWS Config organization conformance pack that you passed in the filter does not exist.</p> <p>For DeleteOrganizationConformancePack, you tried to delete an organization conformance pack that does not exist.</p>
    NoSuchOrganizationConformancePack(String),
    /// <p>For PutConfigAggregator API, no permission to call EnableAWSServiceAccess API.</p> <p>For all OrganizationConfigRule and OrganizationConformancePack APIs, AWS Config throws an exception if APIs are called from member accounts. All APIs must be called from organization master account.</p>
    OrganizationAccessDenied(String),
}

impl DescribeOrganizationConformancePacksError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeOrganizationConformancePacksError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidLimitException" => {
                    return RusotoError::Service(
                        DescribeOrganizationConformancePacksError::InvalidLimit(err.msg),
                    )
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        DescribeOrganizationConformancePacksError::InvalidNextToken(err.msg),
                    )
                }
                "NoSuchOrganizationConformancePackException" => return RusotoError::Service(
                    DescribeOrganizationConformancePacksError::NoSuchOrganizationConformancePack(
                        err.msg,
                    ),
                ),
                "OrganizationAccessDeniedException" => {
                    return RusotoError::Service(
                        DescribeOrganizationConformancePacksError::OrganizationAccessDenied(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeOrganizationConformancePacksError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeOrganizationConformancePacksError::InvalidLimit(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeOrganizationConformancePacksError::InvalidNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeOrganizationConformancePacksError::NoSuchOrganizationConformancePack(
                ref cause,
            ) => write!(f, "{}", cause),
            DescribeOrganizationConformancePacksError::OrganizationAccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeOrganizationConformancePacksError {}
/// Errors returned by DescribePendingAggregationRequests
#[derive(Debug, PartialEq)]
pub enum DescribePendingAggregationRequestsError {
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
}

impl DescribePendingAggregationRequestsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribePendingAggregationRequestsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidLimitException" => {
                    return RusotoError::Service(
                        DescribePendingAggregationRequestsError::InvalidLimit(err.msg),
                    )
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        DescribePendingAggregationRequestsError::InvalidNextToken(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DescribePendingAggregationRequestsError::InvalidParameterValue(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribePendingAggregationRequestsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribePendingAggregationRequestsError::InvalidLimit(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribePendingAggregationRequestsError::InvalidNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribePendingAggregationRequestsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribePendingAggregationRequestsError {}
/// Errors returned by DescribeRemediationConfigurations
#[derive(Debug, PartialEq)]
pub enum DescribeRemediationConfigurationsError {}

impl DescribeRemediationConfigurationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeRemediationConfigurationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeRemediationConfigurationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DescribeRemediationConfigurationsError {}
/// Errors returned by DescribeRemediationExceptions
#[derive(Debug, PartialEq)]
pub enum DescribeRemediationExceptionsError {
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
}

impl DescribeRemediationExceptionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeRemediationExceptionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        DescribeRemediationExceptionsError::InvalidNextToken(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DescribeRemediationExceptionsError::InvalidParameterValue(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeRemediationExceptionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeRemediationExceptionsError::InvalidNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeRemediationExceptionsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeRemediationExceptionsError {}
/// Errors returned by DescribeRemediationExecutionStatus
#[derive(Debug, PartialEq)]
pub enum DescribeRemediationExecutionStatusError {
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>You specified an AWS Config rule without a remediation configuration.</p>
    NoSuchRemediationConfiguration(String),
}

impl DescribeRemediationExecutionStatusError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeRemediationExecutionStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        DescribeRemediationExecutionStatusError::InvalidNextToken(err.msg),
                    )
                }
                "NoSuchRemediationConfigurationException" => {
                    return RusotoError::Service(
                        DescribeRemediationExecutionStatusError::NoSuchRemediationConfiguration(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeRemediationExecutionStatusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeRemediationExecutionStatusError::InvalidNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeRemediationExecutionStatusError::NoSuchRemediationConfiguration(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeRemediationExecutionStatusError {}
/// Errors returned by DescribeRetentionConfigurations
#[derive(Debug, PartialEq)]
pub enum DescribeRetentionConfigurationsError {
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
    /// <p>You have specified a retention configuration that does not exist.</p>
    NoSuchRetentionConfiguration(String),
}

impl DescribeRetentionConfigurationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeRetentionConfigurationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        DescribeRetentionConfigurationsError::InvalidNextToken(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DescribeRetentionConfigurationsError::InvalidParameterValue(err.msg),
                    )
                }
                "NoSuchRetentionConfigurationException" => {
                    return RusotoError::Service(
                        DescribeRetentionConfigurationsError::NoSuchRetentionConfiguration(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeRetentionConfigurationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeRetentionConfigurationsError::InvalidNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeRetentionConfigurationsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeRetentionConfigurationsError::NoSuchRetentionConfiguration(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeRetentionConfigurationsError {}
/// Errors returned by GetAggregateComplianceDetailsByConfigRule
#[derive(Debug, PartialEq)]
pub enum GetAggregateComplianceDetailsByConfigRuleError {
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>You have specified a configuration aggregator that does not exist.</p>
    NoSuchConfigurationAggregator(String),
}

impl GetAggregateComplianceDetailsByConfigRuleError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetAggregateComplianceDetailsByConfigRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidLimitException" => {
                    return RusotoError::Service(
                        GetAggregateComplianceDetailsByConfigRuleError::InvalidLimit(err.msg),
                    )
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        GetAggregateComplianceDetailsByConfigRuleError::InvalidNextToken(err.msg),
                    )
                }
                "NoSuchConfigurationAggregatorException" => return RusotoError::Service(
                    GetAggregateComplianceDetailsByConfigRuleError::NoSuchConfigurationAggregator(
                        err.msg,
                    ),
                ),
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetAggregateComplianceDetailsByConfigRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAggregateComplianceDetailsByConfigRuleError::InvalidLimit(ref cause) => {
                write!(f, "{}", cause)
            }
            GetAggregateComplianceDetailsByConfigRuleError::InvalidNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
            GetAggregateComplianceDetailsByConfigRuleError::NoSuchConfigurationAggregator(
                ref cause,
            ) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetAggregateComplianceDetailsByConfigRuleError {}
/// Errors returned by GetAggregateConfigRuleComplianceSummary
#[derive(Debug, PartialEq)]
pub enum GetAggregateConfigRuleComplianceSummaryError {
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>You have specified a configuration aggregator that does not exist.</p>
    NoSuchConfigurationAggregator(String),
}

impl GetAggregateConfigRuleComplianceSummaryError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetAggregateConfigRuleComplianceSummaryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidLimitException" => {
                    return RusotoError::Service(
                        GetAggregateConfigRuleComplianceSummaryError::InvalidLimit(err.msg),
                    )
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        GetAggregateConfigRuleComplianceSummaryError::InvalidNextToken(err.msg),
                    )
                }
                "NoSuchConfigurationAggregatorException" => {
                    return RusotoError::Service(
                        GetAggregateConfigRuleComplianceSummaryError::NoSuchConfigurationAggregator(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetAggregateConfigRuleComplianceSummaryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAggregateConfigRuleComplianceSummaryError::InvalidLimit(ref cause) => {
                write!(f, "{}", cause)
            }
            GetAggregateConfigRuleComplianceSummaryError::InvalidNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
            GetAggregateConfigRuleComplianceSummaryError::NoSuchConfigurationAggregator(
                ref cause,
            ) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetAggregateConfigRuleComplianceSummaryError {}
/// Errors returned by GetAggregateDiscoveredResourceCounts
#[derive(Debug, PartialEq)]
pub enum GetAggregateDiscoveredResourceCountsError {
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>You have specified a configuration aggregator that does not exist.</p>
    NoSuchConfigurationAggregator(String),
}

impl GetAggregateDiscoveredResourceCountsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetAggregateDiscoveredResourceCountsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidLimitException" => {
                    return RusotoError::Service(
                        GetAggregateDiscoveredResourceCountsError::InvalidLimit(err.msg),
                    )
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        GetAggregateDiscoveredResourceCountsError::InvalidNextToken(err.msg),
                    )
                }
                "NoSuchConfigurationAggregatorException" => {
                    return RusotoError::Service(
                        GetAggregateDiscoveredResourceCountsError::NoSuchConfigurationAggregator(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetAggregateDiscoveredResourceCountsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAggregateDiscoveredResourceCountsError::InvalidLimit(ref cause) => {
                write!(f, "{}", cause)
            }
            GetAggregateDiscoveredResourceCountsError::InvalidNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
            GetAggregateDiscoveredResourceCountsError::NoSuchConfigurationAggregator(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetAggregateDiscoveredResourceCountsError {}
/// Errors returned by GetAggregateResourceConfig
#[derive(Debug, PartialEq)]
pub enum GetAggregateResourceConfigError {
    /// <p>You have specified a configuration aggregator that does not exist.</p>
    NoSuchConfigurationAggregator(String),
    /// <p>The configuration item size is outside the allowable range.</p>
    OversizedConfigurationItem(String),
    /// <p>You have specified a resource that is either unknown or has not been discovered.</p>
    ResourceNotDiscovered(String),
}

impl GetAggregateResourceConfigError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetAggregateResourceConfigError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NoSuchConfigurationAggregatorException" => {
                    return RusotoError::Service(
                        GetAggregateResourceConfigError::NoSuchConfigurationAggregator(err.msg),
                    )
                }
                "OversizedConfigurationItemException" => {
                    return RusotoError::Service(
                        GetAggregateResourceConfigError::OversizedConfigurationItem(err.msg),
                    )
                }
                "ResourceNotDiscoveredException" => {
                    return RusotoError::Service(
                        GetAggregateResourceConfigError::ResourceNotDiscovered(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetAggregateResourceConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAggregateResourceConfigError::NoSuchConfigurationAggregator(ref cause) => {
                write!(f, "{}", cause)
            }
            GetAggregateResourceConfigError::OversizedConfigurationItem(ref cause) => {
                write!(f, "{}", cause)
            }
            GetAggregateResourceConfigError::ResourceNotDiscovered(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetAggregateResourceConfigError {}
/// Errors returned by GetComplianceDetailsByConfigRule
#[derive(Debug, PartialEq)]
pub enum GetComplianceDetailsByConfigRuleError {
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
    /// <p>One or more AWS Config rules in the request are invalid. Verify that the rule names are correct and try again.</p>
    NoSuchConfigRule(String),
}

impl GetComplianceDetailsByConfigRuleError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetComplianceDetailsByConfigRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        GetComplianceDetailsByConfigRuleError::InvalidNextToken(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        GetComplianceDetailsByConfigRuleError::InvalidParameterValue(err.msg),
                    )
                }
                "NoSuchConfigRuleException" => {
                    return RusotoError::Service(
                        GetComplianceDetailsByConfigRuleError::NoSuchConfigRule(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetComplianceDetailsByConfigRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetComplianceDetailsByConfigRuleError::InvalidNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
            GetComplianceDetailsByConfigRuleError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            GetComplianceDetailsByConfigRuleError::NoSuchConfigRule(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetComplianceDetailsByConfigRuleError {}
/// Errors returned by GetComplianceDetailsByResource
#[derive(Debug, PartialEq)]
pub enum GetComplianceDetailsByResourceError {
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
}

impl GetComplianceDetailsByResourceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetComplianceDetailsByResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        GetComplianceDetailsByResourceError::InvalidParameterValue(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetComplianceDetailsByResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetComplianceDetailsByResourceError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetComplianceDetailsByResourceError {}
/// Errors returned by GetComplianceSummaryByConfigRule
#[derive(Debug, PartialEq)]
pub enum GetComplianceSummaryByConfigRuleError {}

impl GetComplianceSummaryByConfigRuleError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetComplianceSummaryByConfigRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetComplianceSummaryByConfigRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for GetComplianceSummaryByConfigRuleError {}
/// Errors returned by GetComplianceSummaryByResourceType
#[derive(Debug, PartialEq)]
pub enum GetComplianceSummaryByResourceTypeError {
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
}

impl GetComplianceSummaryByResourceTypeError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetComplianceSummaryByResourceTypeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        GetComplianceSummaryByResourceTypeError::InvalidParameterValue(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetComplianceSummaryByResourceTypeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetComplianceSummaryByResourceTypeError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetComplianceSummaryByResourceTypeError {}
/// Errors returned by GetConformancePackComplianceDetails
#[derive(Debug, PartialEq)]
pub enum GetConformancePackComplianceDetailsError {
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
    /// <p>AWS Config rule that you passed in the filter does not exist.</p>
    NoSuchConfigRuleInConformancePack(String),
    /// <p>You specified one or more conformance packs that do not exist.</p>
    NoSuchConformancePack(String),
}

impl GetConformancePackComplianceDetailsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetConformancePackComplianceDetailsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidLimitException" => {
                    return RusotoError::Service(
                        GetConformancePackComplianceDetailsError::InvalidLimit(err.msg),
                    )
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        GetConformancePackComplianceDetailsError::InvalidNextToken(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        GetConformancePackComplianceDetailsError::InvalidParameterValue(err.msg),
                    )
                }
                "NoSuchConfigRuleInConformancePackException" => {
                    return RusotoError::Service(
                        GetConformancePackComplianceDetailsError::NoSuchConfigRuleInConformancePack(
                            err.msg,
                        ),
                    )
                }
                "NoSuchConformancePackException" => {
                    return RusotoError::Service(
                        GetConformancePackComplianceDetailsError::NoSuchConformancePack(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetConformancePackComplianceDetailsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetConformancePackComplianceDetailsError::InvalidLimit(ref cause) => {
                write!(f, "{}", cause)
            }
            GetConformancePackComplianceDetailsError::InvalidNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
            GetConformancePackComplianceDetailsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            GetConformancePackComplianceDetailsError::NoSuchConfigRuleInConformancePack(
                ref cause,
            ) => write!(f, "{}", cause),
            GetConformancePackComplianceDetailsError::NoSuchConformancePack(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetConformancePackComplianceDetailsError {}
/// Errors returned by GetConformancePackComplianceSummary
#[derive(Debug, PartialEq)]
pub enum GetConformancePackComplianceSummaryError {
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>You specified one or more conformance packs that do not exist.</p>
    NoSuchConformancePack(String),
}

impl GetConformancePackComplianceSummaryError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetConformancePackComplianceSummaryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidLimitException" => {
                    return RusotoError::Service(
                        GetConformancePackComplianceSummaryError::InvalidLimit(err.msg),
                    )
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        GetConformancePackComplianceSummaryError::InvalidNextToken(err.msg),
                    )
                }
                "NoSuchConformancePackException" => {
                    return RusotoError::Service(
                        GetConformancePackComplianceSummaryError::NoSuchConformancePack(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetConformancePackComplianceSummaryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetConformancePackComplianceSummaryError::InvalidLimit(ref cause) => {
                write!(f, "{}", cause)
            }
            GetConformancePackComplianceSummaryError::InvalidNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
            GetConformancePackComplianceSummaryError::NoSuchConformancePack(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetConformancePackComplianceSummaryError {}
/// Errors returned by GetDiscoveredResourceCounts
#[derive(Debug, PartialEq)]
pub enum GetDiscoveredResourceCountsError {
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
}

impl GetDiscoveredResourceCountsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetDiscoveredResourceCountsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidLimitException" => {
                    return RusotoError::Service(GetDiscoveredResourceCountsError::InvalidLimit(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        GetDiscoveredResourceCountsError::InvalidNextToken(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDiscoveredResourceCountsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDiscoveredResourceCountsError::InvalidLimit(ref cause) => write!(f, "{}", cause),
            GetDiscoveredResourceCountsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDiscoveredResourceCountsError {}
/// Errors returned by GetOrganizationConfigRuleDetailedStatus
#[derive(Debug, PartialEq)]
pub enum GetOrganizationConfigRuleDetailedStatusError {
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>You specified one or more organization config rules that do not exist.</p>
    NoSuchOrganizationConfigRule(String),
    /// <p>For PutConfigAggregator API, no permission to call EnableAWSServiceAccess API.</p> <p>For all OrganizationConfigRule and OrganizationConformancePack APIs, AWS Config throws an exception if APIs are called from member accounts. All APIs must be called from organization master account.</p>
    OrganizationAccessDenied(String),
}

impl GetOrganizationConfigRuleDetailedStatusError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetOrganizationConfigRuleDetailedStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidLimitException" => {
                    return RusotoError::Service(
                        GetOrganizationConfigRuleDetailedStatusError::InvalidLimit(err.msg),
                    )
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        GetOrganizationConfigRuleDetailedStatusError::InvalidNextToken(err.msg),
                    )
                }
                "NoSuchOrganizationConfigRuleException" => {
                    return RusotoError::Service(
                        GetOrganizationConfigRuleDetailedStatusError::NoSuchOrganizationConfigRule(
                            err.msg,
                        ),
                    )
                }
                "OrganizationAccessDeniedException" => {
                    return RusotoError::Service(
                        GetOrganizationConfigRuleDetailedStatusError::OrganizationAccessDenied(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetOrganizationConfigRuleDetailedStatusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetOrganizationConfigRuleDetailedStatusError::InvalidLimit(ref cause) => {
                write!(f, "{}", cause)
            }
            GetOrganizationConfigRuleDetailedStatusError::InvalidNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
            GetOrganizationConfigRuleDetailedStatusError::NoSuchOrganizationConfigRule(
                ref cause,
            ) => write!(f, "{}", cause),
            GetOrganizationConfigRuleDetailedStatusError::OrganizationAccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetOrganizationConfigRuleDetailedStatusError {}
/// Errors returned by GetOrganizationConformancePackDetailedStatus
#[derive(Debug, PartialEq)]
pub enum GetOrganizationConformancePackDetailedStatusError {
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>AWS Config organization conformance pack that you passed in the filter does not exist.</p> <p>For DeleteOrganizationConformancePack, you tried to delete an organization conformance pack that does not exist.</p>
    NoSuchOrganizationConformancePack(String),
    /// <p>For PutConfigAggregator API, no permission to call EnableAWSServiceAccess API.</p> <p>For all OrganizationConfigRule and OrganizationConformancePack APIs, AWS Config throws an exception if APIs are called from member accounts. All APIs must be called from organization master account.</p>
    OrganizationAccessDenied(String),
}

impl GetOrganizationConformancePackDetailedStatusError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetOrganizationConformancePackDetailedStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                                "InvalidLimitException" => return RusotoError::Service(GetOrganizationConformancePackDetailedStatusError::InvalidLimit(err.msg)),
"InvalidNextTokenException" => return RusotoError::Service(GetOrganizationConformancePackDetailedStatusError::InvalidNextToken(err.msg)),
"NoSuchOrganizationConformancePackException" => return RusotoError::Service(GetOrganizationConformancePackDetailedStatusError::NoSuchOrganizationConformancePack(err.msg)),
"OrganizationAccessDeniedException" => return RusotoError::Service(GetOrganizationConformancePackDetailedStatusError::OrganizationAccessDenied(err.msg)),
"ValidationException" => return RusotoError::Validation(err.msg),
_ => {}
                            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetOrganizationConformancePackDetailedStatusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
                            GetOrganizationConformancePackDetailedStatusError::InvalidLimit(ref cause) => write!(f, "{}", cause),
GetOrganizationConformancePackDetailedStatusError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
GetOrganizationConformancePackDetailedStatusError::NoSuchOrganizationConformancePack(ref cause) => write!(f, "{}", cause),
GetOrganizationConformancePackDetailedStatusError::OrganizationAccessDenied(ref cause) => write!(f, "{}", cause)
                        }
    }
}
impl Error for GetOrganizationConformancePackDetailedStatusError {}
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
}

impl GetResourceConfigHistoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetResourceConfigHistoryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidLimitException" => {
                    return RusotoError::Service(GetResourceConfigHistoryError::InvalidLimit(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(GetResourceConfigHistoryError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "InvalidTimeRangeException" => {
                    return RusotoError::Service(GetResourceConfigHistoryError::InvalidTimeRange(
                        err.msg,
                    ))
                }
                "NoAvailableConfigurationRecorderException" => {
                    return RusotoError::Service(
                        GetResourceConfigHistoryError::NoAvailableConfigurationRecorder(err.msg),
                    )
                }
                "ResourceNotDiscoveredException" => {
                    return RusotoError::Service(
                        GetResourceConfigHistoryError::ResourceNotDiscovered(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetResourceConfigHistoryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetResourceConfigHistoryError::InvalidLimit(ref cause) => write!(f, "{}", cause),
            GetResourceConfigHistoryError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            GetResourceConfigHistoryError::InvalidTimeRange(ref cause) => write!(f, "{}", cause),
            GetResourceConfigHistoryError::NoAvailableConfigurationRecorder(ref cause) => {
                write!(f, "{}", cause)
            }
            GetResourceConfigHistoryError::ResourceNotDiscovered(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetResourceConfigHistoryError {}
/// Errors returned by ListAggregateDiscoveredResources
#[derive(Debug, PartialEq)]
pub enum ListAggregateDiscoveredResourcesError {
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>You have specified a configuration aggregator that does not exist.</p>
    NoSuchConfigurationAggregator(String),
}

impl ListAggregateDiscoveredResourcesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListAggregateDiscoveredResourcesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidLimitException" => {
                    return RusotoError::Service(
                        ListAggregateDiscoveredResourcesError::InvalidLimit(err.msg),
                    )
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        ListAggregateDiscoveredResourcesError::InvalidNextToken(err.msg),
                    )
                }
                "NoSuchConfigurationAggregatorException" => {
                    return RusotoError::Service(
                        ListAggregateDiscoveredResourcesError::NoSuchConfigurationAggregator(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListAggregateDiscoveredResourcesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListAggregateDiscoveredResourcesError::InvalidLimit(ref cause) => {
                write!(f, "{}", cause)
            }
            ListAggregateDiscoveredResourcesError::InvalidNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
            ListAggregateDiscoveredResourcesError::NoSuchConfigurationAggregator(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListAggregateDiscoveredResourcesError {}
/// Errors returned by ListDiscoveredResources
#[derive(Debug, PartialEq)]
pub enum ListDiscoveredResourcesError {
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>There are no configuration recorders available to provide the role needed to describe your resources. Create a configuration recorder.</p>
    NoAvailableConfigurationRecorder(String),
}

impl ListDiscoveredResourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDiscoveredResourcesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidLimitException" => {
                    return RusotoError::Service(ListDiscoveredResourcesError::InvalidLimit(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListDiscoveredResourcesError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "NoAvailableConfigurationRecorderException" => {
                    return RusotoError::Service(
                        ListDiscoveredResourcesError::NoAvailableConfigurationRecorder(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListDiscoveredResourcesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDiscoveredResourcesError::InvalidLimit(ref cause) => write!(f, "{}", cause),
            ListDiscoveredResourcesError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListDiscoveredResourcesError::NoAvailableConfigurationRecorder(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListDiscoveredResourcesError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>You have specified a resource that does not exist.</p>
    ResourceNotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidLimitException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidLimit(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
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
            ListTagsForResourceError::InvalidLimit(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by PutAggregationAuthorization
#[derive(Debug, PartialEq)]
pub enum PutAggregationAuthorizationError {
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
}

impl PutAggregationAuthorizationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutAggregationAuthorizationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        PutAggregationAuthorizationError::InvalidParameterValue(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutAggregationAuthorizationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutAggregationAuthorizationError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutAggregationAuthorizationError {}
/// Errors returned by PutConfigRule
#[derive(Debug, PartialEq)]
pub enum PutConfigRuleError {
    /// <p><p>Indicates one of the following errors:</p> <ul> <li> <p>For PutConfigRule, the rule cannot be created because the IAM role assigned to AWS Config lacks permissions to perform the config:Put* action.</p> </li> <li> <p>For PutConfigRule, the AWS Lambda function cannot be invoked. Check the function ARN, and check the function&#39;s permissions.</p> </li> <li> <p>For PutOrganizationConfigRule, organization config rule cannot be created because you do not have permissions to call IAM <code>GetRole</code> action or create a service linked role.</p> </li> <li> <p>For PutConformancePack and PutOrganizationConformancePack, a conformance pack cannot be created because you do not have permissions: </p> <ul> <li> <p>To call IAM <code>GetRole</code> action or create a service linked role.</p> </li> <li> <p>To read Amazon S3 bucket.</p> </li> </ul> </li> </ul></p>
    InsufficientPermissions(String),
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
    /// <p>Failed to add the AWS Config rule because the account already contains the maximum number of 150 rules. Consider deleting any deactivated rules before you add new rules.</p>
    MaxNumberOfConfigRulesExceeded(String),
    /// <p>There are no configuration recorders available to provide the role needed to describe your resources. Create a configuration recorder.</p>
    NoAvailableConfigurationRecorder(String),
    /// <p><p>You see this exception in the following cases: </p> <ul> <li> <p>For DeleteConfigRule, AWS Config is deleting this rule. Try your request again later.</p> </li> <li> <p>For DeleteConfigRule, the rule is deleting your evaluation results. Try your request again later.</p> </li> <li> <p>For DeleteConfigRule, a remediation action is associated with the rule and AWS Config cannot delete this rule. Delete the remediation action associated with the rule before deleting the rule and try your request again later.</p> </li> <li> <p>For PutConfigOrganizationRule, organization config rule deletion is in progress. Try your request again later.</p> </li> <li> <p>For DeleteOrganizationConfigRule, organization config rule creation is in progress. Try your request again later.</p> </li> <li> <p>For PutConformancePack and PutOrganizationConformancePack, a conformance pack creation, update, and deletion is in progress. Try your request again later.</p> </li> <li> <p>For DeleteConformancePack, a conformance pack creation, update, and deletion is in progress. Try your request again later.</p> </li> </ul></p>
    ResourceInUse(String),
}

impl PutConfigRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutConfigRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InsufficientPermissionsException" => {
                    return RusotoError::Service(PutConfigRuleError::InsufficientPermissions(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(PutConfigRuleError::InvalidParameterValue(err.msg))
                }
                "MaxNumberOfConfigRulesExceededException" => {
                    return RusotoError::Service(
                        PutConfigRuleError::MaxNumberOfConfigRulesExceeded(err.msg),
                    )
                }
                "NoAvailableConfigurationRecorderException" => {
                    return RusotoError::Service(
                        PutConfigRuleError::NoAvailableConfigurationRecorder(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(PutConfigRuleError::ResourceInUse(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutConfigRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutConfigRuleError::InsufficientPermissions(ref cause) => write!(f, "{}", cause),
            PutConfigRuleError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            PutConfigRuleError::MaxNumberOfConfigRulesExceeded(ref cause) => write!(f, "{}", cause),
            PutConfigRuleError::NoAvailableConfigurationRecorder(ref cause) => {
                write!(f, "{}", cause)
            }
            PutConfigRuleError::ResourceInUse(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutConfigRuleError {}
/// Errors returned by PutConfigurationAggregator
#[derive(Debug, PartialEq)]
pub enum PutConfigurationAggregatorError {
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
    /// <p>You have provided a null or empty role ARN.</p>
    InvalidRole(String),
    /// <p>For <code>StartConfigRulesEvaluation</code> API, this exception is thrown if an evaluation is in progress or if you call the <a>StartConfigRulesEvaluation</a> API more than once per minute.</p> <p>For <code>PutConfigurationAggregator</code> API, this exception is thrown if the number of accounts and aggregators exceeds the limit.</p>
    LimitExceeded(String),
    /// <p>Organization is no longer available.</p>
    NoAvailableOrganization(String),
    /// <p>For PutConfigAggregator API, no permission to call EnableAWSServiceAccess API.</p> <p>For all OrganizationConfigRule and OrganizationConformancePack APIs, AWS Config throws an exception if APIs are called from member accounts. All APIs must be called from organization master account.</p>
    OrganizationAccessDenied(String),
    /// <p>AWS Config resource cannot be created because your organization does not have all features enabled.</p>
    OrganizationAllFeaturesNotEnabled(String),
}

impl PutConfigurationAggregatorError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutConfigurationAggregatorError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        PutConfigurationAggregatorError::InvalidParameterValue(err.msg),
                    )
                }
                "InvalidRoleException" => {
                    return RusotoError::Service(PutConfigurationAggregatorError::InvalidRole(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(PutConfigurationAggregatorError::LimitExceeded(
                        err.msg,
                    ))
                }
                "NoAvailableOrganizationException" => {
                    return RusotoError::Service(
                        PutConfigurationAggregatorError::NoAvailableOrganization(err.msg),
                    )
                }
                "OrganizationAccessDeniedException" => {
                    return RusotoError::Service(
                        PutConfigurationAggregatorError::OrganizationAccessDenied(err.msg),
                    )
                }
                "OrganizationAllFeaturesNotEnabledException" => {
                    return RusotoError::Service(
                        PutConfigurationAggregatorError::OrganizationAllFeaturesNotEnabled(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutConfigurationAggregatorError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutConfigurationAggregatorError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            PutConfigurationAggregatorError::InvalidRole(ref cause) => write!(f, "{}", cause),
            PutConfigurationAggregatorError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            PutConfigurationAggregatorError::NoAvailableOrganization(ref cause) => {
                write!(f, "{}", cause)
            }
            PutConfigurationAggregatorError::OrganizationAccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            PutConfigurationAggregatorError::OrganizationAllFeaturesNotEnabled(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutConfigurationAggregatorError {}
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
}

impl PutConfigurationRecorderError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutConfigurationRecorderError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidConfigurationRecorderNameException" => {
                    return RusotoError::Service(
                        PutConfigurationRecorderError::InvalidConfigurationRecorderName(err.msg),
                    )
                }
                "InvalidRecordingGroupException" => {
                    return RusotoError::Service(
                        PutConfigurationRecorderError::InvalidRecordingGroup(err.msg),
                    )
                }
                "InvalidRoleException" => {
                    return RusotoError::Service(PutConfigurationRecorderError::InvalidRole(
                        err.msg,
                    ))
                }
                "MaxNumberOfConfigurationRecordersExceededException" => {
                    return RusotoError::Service(
                        PutConfigurationRecorderError::MaxNumberOfConfigurationRecordersExceeded(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutConfigurationRecorderError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutConfigurationRecorderError::InvalidConfigurationRecorderName(ref cause) => {
                write!(f, "{}", cause)
            }
            PutConfigurationRecorderError::InvalidRecordingGroup(ref cause) => {
                write!(f, "{}", cause)
            }
            PutConfigurationRecorderError::InvalidRole(ref cause) => write!(f, "{}", cause),
            PutConfigurationRecorderError::MaxNumberOfConfigurationRecordersExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutConfigurationRecorderError {}
/// Errors returned by PutConformancePack
#[derive(Debug, PartialEq)]
pub enum PutConformancePackError {
    /// <p>You have specified a template that is not valid or supported.</p>
    ConformancePackTemplateValidation(String),
    /// <p><p>Indicates one of the following errors:</p> <ul> <li> <p>For PutConfigRule, the rule cannot be created because the IAM role assigned to AWS Config lacks permissions to perform the config:Put* action.</p> </li> <li> <p>For PutConfigRule, the AWS Lambda function cannot be invoked. Check the function ARN, and check the function&#39;s permissions.</p> </li> <li> <p>For PutOrganizationConfigRule, organization config rule cannot be created because you do not have permissions to call IAM <code>GetRole</code> action or create a service linked role.</p> </li> <li> <p>For PutConformancePack and PutOrganizationConformancePack, a conformance pack cannot be created because you do not have permissions: </p> <ul> <li> <p>To call IAM <code>GetRole</code> action or create a service linked role.</p> </li> <li> <p>To read Amazon S3 bucket.</p> </li> </ul> </li> </ul></p>
    InsufficientPermissions(String),
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
    /// <p>You have reached the limit (6) of the number of conformance packs in an account (6 conformance pack with 25 AWS Config rules per pack).</p>
    MaxNumberOfConformancePacksExceeded(String),
    /// <p><p>You see this exception in the following cases: </p> <ul> <li> <p>For DeleteConfigRule, AWS Config is deleting this rule. Try your request again later.</p> </li> <li> <p>For DeleteConfigRule, the rule is deleting your evaluation results. Try your request again later.</p> </li> <li> <p>For DeleteConfigRule, a remediation action is associated with the rule and AWS Config cannot delete this rule. Delete the remediation action associated with the rule before deleting the rule and try your request again later.</p> </li> <li> <p>For PutConfigOrganizationRule, organization config rule deletion is in progress. Try your request again later.</p> </li> <li> <p>For DeleteOrganizationConfigRule, organization config rule creation is in progress. Try your request again later.</p> </li> <li> <p>For PutConformancePack and PutOrganizationConformancePack, a conformance pack creation, update, and deletion is in progress. Try your request again later.</p> </li> <li> <p>For DeleteConformancePack, a conformance pack creation, update, and deletion is in progress. Try your request again later.</p> </li> </ul></p>
    ResourceInUse(String),
}

impl PutConformancePackError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutConformancePackError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConformancePackTemplateValidationException" => {
                    return RusotoError::Service(
                        PutConformancePackError::ConformancePackTemplateValidation(err.msg),
                    )
                }
                "InsufficientPermissionsException" => {
                    return RusotoError::Service(PutConformancePackError::InsufficientPermissions(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(PutConformancePackError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "MaxNumberOfConformancePacksExceededException" => {
                    return RusotoError::Service(
                        PutConformancePackError::MaxNumberOfConformancePacksExceeded(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(PutConformancePackError::ResourceInUse(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutConformancePackError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutConformancePackError::ConformancePackTemplateValidation(ref cause) => {
                write!(f, "{}", cause)
            }
            PutConformancePackError::InsufficientPermissions(ref cause) => write!(f, "{}", cause),
            PutConformancePackError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            PutConformancePackError::MaxNumberOfConformancePacksExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            PutConformancePackError::ResourceInUse(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutConformancePackError {}
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
}

impl PutDeliveryChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutDeliveryChannelError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InsufficientDeliveryPolicyException" => {
                    return RusotoError::Service(
                        PutDeliveryChannelError::InsufficientDeliveryPolicy(err.msg),
                    )
                }
                "InvalidDeliveryChannelNameException" => {
                    return RusotoError::Service(
                        PutDeliveryChannelError::InvalidDeliveryChannelName(err.msg),
                    )
                }
                "InvalidS3KeyPrefixException" => {
                    return RusotoError::Service(PutDeliveryChannelError::InvalidS3KeyPrefix(
                        err.msg,
                    ))
                }
                "InvalidSNSTopicARNException" => {
                    return RusotoError::Service(PutDeliveryChannelError::InvalidSNSTopicARN(
                        err.msg,
                    ))
                }
                "MaxNumberOfDeliveryChannelsExceededException" => {
                    return RusotoError::Service(
                        PutDeliveryChannelError::MaxNumberOfDeliveryChannelsExceeded(err.msg),
                    )
                }
                "NoAvailableConfigurationRecorderException" => {
                    return RusotoError::Service(
                        PutDeliveryChannelError::NoAvailableConfigurationRecorder(err.msg),
                    )
                }
                "NoSuchBucketException" => {
                    return RusotoError::Service(PutDeliveryChannelError::NoSuchBucket(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutDeliveryChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutDeliveryChannelError::InsufficientDeliveryPolicy(ref cause) => {
                write!(f, "{}", cause)
            }
            PutDeliveryChannelError::InvalidDeliveryChannelName(ref cause) => {
                write!(f, "{}", cause)
            }
            PutDeliveryChannelError::InvalidS3KeyPrefix(ref cause) => write!(f, "{}", cause),
            PutDeliveryChannelError::InvalidSNSTopicARN(ref cause) => write!(f, "{}", cause),
            PutDeliveryChannelError::MaxNumberOfDeliveryChannelsExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            PutDeliveryChannelError::NoAvailableConfigurationRecorder(ref cause) => {
                write!(f, "{}", cause)
            }
            PutDeliveryChannelError::NoSuchBucket(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutDeliveryChannelError {}
/// Errors returned by PutEvaluations
#[derive(Debug, PartialEq)]
pub enum PutEvaluationsError {
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
    /// <p>The specified <code>ResultToken</code> is invalid.</p>
    InvalidResultToken(String),
    /// <p>One or more AWS Config rules in the request are invalid. Verify that the rule names are correct and try again.</p>
    NoSuchConfigRule(String),
}

impl PutEvaluationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutEvaluationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(PutEvaluationsError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "InvalidResultTokenException" => {
                    return RusotoError::Service(PutEvaluationsError::InvalidResultToken(err.msg))
                }
                "NoSuchConfigRuleException" => {
                    return RusotoError::Service(PutEvaluationsError::NoSuchConfigRule(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutEvaluationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutEvaluationsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            PutEvaluationsError::InvalidResultToken(ref cause) => write!(f, "{}", cause),
            PutEvaluationsError::NoSuchConfigRule(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutEvaluationsError {}
/// Errors returned by PutOrganizationConfigRule
#[derive(Debug, PartialEq)]
pub enum PutOrganizationConfigRuleError {
    /// <p><p>Indicates one of the following errors:</p> <ul> <li> <p>For PutConfigRule, the rule cannot be created because the IAM role assigned to AWS Config lacks permissions to perform the config:Put* action.</p> </li> <li> <p>For PutConfigRule, the AWS Lambda function cannot be invoked. Check the function ARN, and check the function&#39;s permissions.</p> </li> <li> <p>For PutOrganizationConfigRule, organization config rule cannot be created because you do not have permissions to call IAM <code>GetRole</code> action or create a service linked role.</p> </li> <li> <p>For PutConformancePack and PutOrganizationConformancePack, a conformance pack cannot be created because you do not have permissions: </p> <ul> <li> <p>To call IAM <code>GetRole</code> action or create a service linked role.</p> </li> <li> <p>To read Amazon S3 bucket.</p> </li> </ul> </li> </ul></p>
    InsufficientPermissions(String),
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
    /// <p>You have reached the limit of the number of organization config rules you can create.</p>
    MaxNumberOfOrganizationConfigRulesExceeded(String),
    /// <p>Organization is no longer available.</p>
    NoAvailableOrganization(String),
    /// <p>For PutConfigAggregator API, no permission to call EnableAWSServiceAccess API.</p> <p>For all OrganizationConfigRule and OrganizationConformancePack APIs, AWS Config throws an exception if APIs are called from member accounts. All APIs must be called from organization master account.</p>
    OrganizationAccessDenied(String),
    /// <p>AWS Config resource cannot be created because your organization does not have all features enabled.</p>
    OrganizationAllFeaturesNotEnabled(String),
    /// <p><p>You see this exception in the following cases: </p> <ul> <li> <p>For DeleteConfigRule, AWS Config is deleting this rule. Try your request again later.</p> </li> <li> <p>For DeleteConfigRule, the rule is deleting your evaluation results. Try your request again later.</p> </li> <li> <p>For DeleteConfigRule, a remediation action is associated with the rule and AWS Config cannot delete this rule. Delete the remediation action associated with the rule before deleting the rule and try your request again later.</p> </li> <li> <p>For PutConfigOrganizationRule, organization config rule deletion is in progress. Try your request again later.</p> </li> <li> <p>For DeleteOrganizationConfigRule, organization config rule creation is in progress. Try your request again later.</p> </li> <li> <p>For PutConformancePack and PutOrganizationConformancePack, a conformance pack creation, update, and deletion is in progress. Try your request again later.</p> </li> <li> <p>For DeleteConformancePack, a conformance pack creation, update, and deletion is in progress. Try your request again later.</p> </li> </ul></p>
    ResourceInUse(String),
}

impl PutOrganizationConfigRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutOrganizationConfigRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InsufficientPermissionsException" => {
                    return RusotoError::Service(
                        PutOrganizationConfigRuleError::InsufficientPermissions(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        PutOrganizationConfigRuleError::InvalidParameterValue(err.msg),
                    )
                }
                "MaxNumberOfOrganizationConfigRulesExceededException" => {
                    return RusotoError::Service(
                        PutOrganizationConfigRuleError::MaxNumberOfOrganizationConfigRulesExceeded(
                            err.msg,
                        ),
                    )
                }
                "NoAvailableOrganizationException" => {
                    return RusotoError::Service(
                        PutOrganizationConfigRuleError::NoAvailableOrganization(err.msg),
                    )
                }
                "OrganizationAccessDeniedException" => {
                    return RusotoError::Service(
                        PutOrganizationConfigRuleError::OrganizationAccessDenied(err.msg),
                    )
                }
                "OrganizationAllFeaturesNotEnabledException" => {
                    return RusotoError::Service(
                        PutOrganizationConfigRuleError::OrganizationAllFeaturesNotEnabled(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(PutOrganizationConfigRuleError::ResourceInUse(
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
impl fmt::Display for PutOrganizationConfigRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutOrganizationConfigRuleError::InsufficientPermissions(ref cause) => {
                write!(f, "{}", cause)
            }
            PutOrganizationConfigRuleError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            PutOrganizationConfigRuleError::MaxNumberOfOrganizationConfigRulesExceeded(
                ref cause,
            ) => write!(f, "{}", cause),
            PutOrganizationConfigRuleError::NoAvailableOrganization(ref cause) => {
                write!(f, "{}", cause)
            }
            PutOrganizationConfigRuleError::OrganizationAccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            PutOrganizationConfigRuleError::OrganizationAllFeaturesNotEnabled(ref cause) => {
                write!(f, "{}", cause)
            }
            PutOrganizationConfigRuleError::ResourceInUse(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutOrganizationConfigRuleError {}
/// Errors returned by PutOrganizationConformancePack
#[derive(Debug, PartialEq)]
pub enum PutOrganizationConformancePackError {
    /// <p><p>Indicates one of the following errors:</p> <ul> <li> <p>For PutConfigRule, the rule cannot be created because the IAM role assigned to AWS Config lacks permissions to perform the config:Put* action.</p> </li> <li> <p>For PutConfigRule, the AWS Lambda function cannot be invoked. Check the function ARN, and check the function&#39;s permissions.</p> </li> <li> <p>For PutOrganizationConfigRule, organization config rule cannot be created because you do not have permissions to call IAM <code>GetRole</code> action or create a service linked role.</p> </li> <li> <p>For PutConformancePack and PutOrganizationConformancePack, a conformance pack cannot be created because you do not have permissions: </p> <ul> <li> <p>To call IAM <code>GetRole</code> action or create a service linked role.</p> </li> <li> <p>To read Amazon S3 bucket.</p> </li> </ul> </li> </ul></p>
    InsufficientPermissions(String),
    /// <p>You have reached the limit (6) of the number of organization conformance packs in an account (6 conformance pack with 25 AWS Config rules per pack per account).</p>
    MaxNumberOfOrganizationConformancePacksExceeded(String),
    /// <p>Organization is no longer available.</p>
    NoAvailableOrganization(String),
    /// <p>For PutConfigAggregator API, no permission to call EnableAWSServiceAccess API.</p> <p>For all OrganizationConfigRule and OrganizationConformancePack APIs, AWS Config throws an exception if APIs are called from member accounts. All APIs must be called from organization master account.</p>
    OrganizationAccessDenied(String),
    /// <p>AWS Config resource cannot be created because your organization does not have all features enabled.</p>
    OrganizationAllFeaturesNotEnabled(String),
    /// <p>You have specified a template that is not valid or supported.</p>
    OrganizationConformancePackTemplateValidation(String),
    /// <p><p>You see this exception in the following cases: </p> <ul> <li> <p>For DeleteConfigRule, AWS Config is deleting this rule. Try your request again later.</p> </li> <li> <p>For DeleteConfigRule, the rule is deleting your evaluation results. Try your request again later.</p> </li> <li> <p>For DeleteConfigRule, a remediation action is associated with the rule and AWS Config cannot delete this rule. Delete the remediation action associated with the rule before deleting the rule and try your request again later.</p> </li> <li> <p>For PutConfigOrganizationRule, organization config rule deletion is in progress. Try your request again later.</p> </li> <li> <p>For DeleteOrganizationConfigRule, organization config rule creation is in progress. Try your request again later.</p> </li> <li> <p>For PutConformancePack and PutOrganizationConformancePack, a conformance pack creation, update, and deletion is in progress. Try your request again later.</p> </li> <li> <p>For DeleteConformancePack, a conformance pack creation, update, and deletion is in progress. Try your request again later.</p> </li> </ul></p>
    ResourceInUse(String),
}

impl PutOrganizationConformancePackError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutOrganizationConformancePackError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                                "InsufficientPermissionsException" => return RusotoError::Service(PutOrganizationConformancePackError::InsufficientPermissions(err.msg)),
"MaxNumberOfOrganizationConformancePacksExceededException" => return RusotoError::Service(PutOrganizationConformancePackError::MaxNumberOfOrganizationConformancePacksExceeded(err.msg)),
"NoAvailableOrganizationException" => return RusotoError::Service(PutOrganizationConformancePackError::NoAvailableOrganization(err.msg)),
"OrganizationAccessDeniedException" => return RusotoError::Service(PutOrganizationConformancePackError::OrganizationAccessDenied(err.msg)),
"OrganizationAllFeaturesNotEnabledException" => return RusotoError::Service(PutOrganizationConformancePackError::OrganizationAllFeaturesNotEnabled(err.msg)),
"OrganizationConformancePackTemplateValidationException" => return RusotoError::Service(PutOrganizationConformancePackError::OrganizationConformancePackTemplateValidation(err.msg)),
"ResourceInUseException" => return RusotoError::Service(PutOrganizationConformancePackError::ResourceInUse(err.msg)),
"ValidationException" => return RusotoError::Validation(err.msg),
_ => {}
                            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutOrganizationConformancePackError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
                            PutOrganizationConformancePackError::InsufficientPermissions(ref cause) => write!(f, "{}", cause),
PutOrganizationConformancePackError::MaxNumberOfOrganizationConformancePacksExceeded(ref cause) => write!(f, "{}", cause),
PutOrganizationConformancePackError::NoAvailableOrganization(ref cause) => write!(f, "{}", cause),
PutOrganizationConformancePackError::OrganizationAccessDenied(ref cause) => write!(f, "{}", cause),
PutOrganizationConformancePackError::OrganizationAllFeaturesNotEnabled(ref cause) => write!(f, "{}", cause),
PutOrganizationConformancePackError::OrganizationConformancePackTemplateValidation(ref cause) => write!(f, "{}", cause),
PutOrganizationConformancePackError::ResourceInUse(ref cause) => write!(f, "{}", cause)
                        }
    }
}
impl Error for PutOrganizationConformancePackError {}
/// Errors returned by PutRemediationConfigurations
#[derive(Debug, PartialEq)]
pub enum PutRemediationConfigurationsError {
    /// <p><p>Indicates one of the following errors:</p> <ul> <li> <p>For PutConfigRule, the rule cannot be created because the IAM role assigned to AWS Config lacks permissions to perform the config:Put* action.</p> </li> <li> <p>For PutConfigRule, the AWS Lambda function cannot be invoked. Check the function ARN, and check the function&#39;s permissions.</p> </li> <li> <p>For PutOrganizationConfigRule, organization config rule cannot be created because you do not have permissions to call IAM <code>GetRole</code> action or create a service linked role.</p> </li> <li> <p>For PutConformancePack and PutOrganizationConformancePack, a conformance pack cannot be created because you do not have permissions: </p> <ul> <li> <p>To call IAM <code>GetRole</code> action or create a service linked role.</p> </li> <li> <p>To read Amazon S3 bucket.</p> </li> </ul> </li> </ul></p>
    InsufficientPermissions(String),
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
}

impl PutRemediationConfigurationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutRemediationConfigurationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InsufficientPermissionsException" => {
                    return RusotoError::Service(
                        PutRemediationConfigurationsError::InsufficientPermissions(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        PutRemediationConfigurationsError::InvalidParameterValue(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutRemediationConfigurationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutRemediationConfigurationsError::InsufficientPermissions(ref cause) => {
                write!(f, "{}", cause)
            }
            PutRemediationConfigurationsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutRemediationConfigurationsError {}
/// Errors returned by PutRemediationExceptions
#[derive(Debug, PartialEq)]
pub enum PutRemediationExceptionsError {
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
}

impl PutRemediationExceptionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutRemediationExceptionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        PutRemediationExceptionsError::InvalidParameterValue(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutRemediationExceptionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutRemediationExceptionsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutRemediationExceptionsError {}
/// Errors returned by PutResourceConfig
#[derive(Debug, PartialEq)]
pub enum PutResourceConfigError {
    /// <p><p>Indicates one of the following errors:</p> <ul> <li> <p>For PutConfigRule, the rule cannot be created because the IAM role assigned to AWS Config lacks permissions to perform the config:Put* action.</p> </li> <li> <p>For PutConfigRule, the AWS Lambda function cannot be invoked. Check the function ARN, and check the function&#39;s permissions.</p> </li> <li> <p>For PutOrganizationConfigRule, organization config rule cannot be created because you do not have permissions to call IAM <code>GetRole</code> action or create a service linked role.</p> </li> <li> <p>For PutConformancePack and PutOrganizationConformancePack, a conformance pack cannot be created because you do not have permissions: </p> <ul> <li> <p>To call IAM <code>GetRole</code> action or create a service linked role.</p> </li> <li> <p>To read Amazon S3 bucket.</p> </li> </ul> </li> </ul></p>
    InsufficientPermissions(String),
    /// <p>You have reached the limit (100,000) of active custom resource types in your account. Delete unused resources using <code>DeleteResourceConfig</code>.</p>
    MaxActiveResourcesExceeded(String),
    /// <p>There is no configuration recorder running.</p>
    NoRunningConfigurationRecorder(String),
}

impl PutResourceConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutResourceConfigError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InsufficientPermissionsException" => {
                    return RusotoError::Service(PutResourceConfigError::InsufficientPermissions(
                        err.msg,
                    ))
                }
                "MaxActiveResourcesExceededException" => {
                    return RusotoError::Service(
                        PutResourceConfigError::MaxActiveResourcesExceeded(err.msg),
                    )
                }
                "NoRunningConfigurationRecorderException" => {
                    return RusotoError::Service(
                        PutResourceConfigError::NoRunningConfigurationRecorder(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutResourceConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutResourceConfigError::InsufficientPermissions(ref cause) => write!(f, "{}", cause),
            PutResourceConfigError::MaxActiveResourcesExceeded(ref cause) => write!(f, "{}", cause),
            PutResourceConfigError::NoRunningConfigurationRecorder(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutResourceConfigError {}
/// Errors returned by PutRetentionConfiguration
#[derive(Debug, PartialEq)]
pub enum PutRetentionConfigurationError {
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
    /// <p>Failed to add the retention configuration because a retention configuration with that name already exists.</p>
    MaxNumberOfRetentionConfigurationsExceeded(String),
}

impl PutRetentionConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutRetentionConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        PutRetentionConfigurationError::InvalidParameterValue(err.msg),
                    )
                }
                "MaxNumberOfRetentionConfigurationsExceededException" => {
                    return RusotoError::Service(
                        PutRetentionConfigurationError::MaxNumberOfRetentionConfigurationsExceeded(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutRetentionConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutRetentionConfigurationError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            PutRetentionConfigurationError::MaxNumberOfRetentionConfigurationsExceeded(
                ref cause,
            ) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutRetentionConfigurationError {}
/// Errors returned by SelectResourceConfig
#[derive(Debug, PartialEq)]
pub enum SelectResourceConfigError {
    /// <p>The syntax of the query is incorrect.</p>
    InvalidExpression(String),
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
}

impl SelectResourceConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SelectResourceConfigError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidExpressionException" => {
                    return RusotoError::Service(SelectResourceConfigError::InvalidExpression(
                        err.msg,
                    ))
                }
                "InvalidLimitException" => {
                    return RusotoError::Service(SelectResourceConfigError::InvalidLimit(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(SelectResourceConfigError::InvalidNextToken(
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
impl fmt::Display for SelectResourceConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SelectResourceConfigError::InvalidExpression(ref cause) => write!(f, "{}", cause),
            SelectResourceConfigError::InvalidLimit(ref cause) => write!(f, "{}", cause),
            SelectResourceConfigError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SelectResourceConfigError {}
/// Errors returned by StartConfigRulesEvaluation
#[derive(Debug, PartialEq)]
pub enum StartConfigRulesEvaluationError {
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
    /// <p>For <code>StartConfigRulesEvaluation</code> API, this exception is thrown if an evaluation is in progress or if you call the <a>StartConfigRulesEvaluation</a> API more than once per minute.</p> <p>For <code>PutConfigurationAggregator</code> API, this exception is thrown if the number of accounts and aggregators exceeds the limit.</p>
    LimitExceeded(String),
    /// <p>One or more AWS Config rules in the request are invalid. Verify that the rule names are correct and try again.</p>
    NoSuchConfigRule(String),
    /// <p><p>You see this exception in the following cases: </p> <ul> <li> <p>For DeleteConfigRule, AWS Config is deleting this rule. Try your request again later.</p> </li> <li> <p>For DeleteConfigRule, the rule is deleting your evaluation results. Try your request again later.</p> </li> <li> <p>For DeleteConfigRule, a remediation action is associated with the rule and AWS Config cannot delete this rule. Delete the remediation action associated with the rule before deleting the rule and try your request again later.</p> </li> <li> <p>For PutConfigOrganizationRule, organization config rule deletion is in progress. Try your request again later.</p> </li> <li> <p>For DeleteOrganizationConfigRule, organization config rule creation is in progress. Try your request again later.</p> </li> <li> <p>For PutConformancePack and PutOrganizationConformancePack, a conformance pack creation, update, and deletion is in progress. Try your request again later.</p> </li> <li> <p>For DeleteConformancePack, a conformance pack creation, update, and deletion is in progress. Try your request again later.</p> </li> </ul></p>
    ResourceInUse(String),
}

impl StartConfigRulesEvaluationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<StartConfigRulesEvaluationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        StartConfigRulesEvaluationError::InvalidParameterValue(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(StartConfigRulesEvaluationError::LimitExceeded(
                        err.msg,
                    ))
                }
                "NoSuchConfigRuleException" => {
                    return RusotoError::Service(StartConfigRulesEvaluationError::NoSuchConfigRule(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(StartConfigRulesEvaluationError::ResourceInUse(
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
impl fmt::Display for StartConfigRulesEvaluationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartConfigRulesEvaluationError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            StartConfigRulesEvaluationError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            StartConfigRulesEvaluationError::NoSuchConfigRule(ref cause) => write!(f, "{}", cause),
            StartConfigRulesEvaluationError::ResourceInUse(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartConfigRulesEvaluationError {}
/// Errors returned by StartConfigurationRecorder
#[derive(Debug, PartialEq)]
pub enum StartConfigurationRecorderError {
    /// <p>There is no delivery channel available to record configurations.</p>
    NoAvailableDeliveryChannel(String),
    /// <p>You have specified a configuration recorder that does not exist.</p>
    NoSuchConfigurationRecorder(String),
}

impl StartConfigurationRecorderError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<StartConfigurationRecorderError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NoAvailableDeliveryChannelException" => {
                    return RusotoError::Service(
                        StartConfigurationRecorderError::NoAvailableDeliveryChannel(err.msg),
                    )
                }
                "NoSuchConfigurationRecorderException" => {
                    return RusotoError::Service(
                        StartConfigurationRecorderError::NoSuchConfigurationRecorder(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartConfigurationRecorderError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartConfigurationRecorderError::NoAvailableDeliveryChannel(ref cause) => {
                write!(f, "{}", cause)
            }
            StartConfigurationRecorderError::NoSuchConfigurationRecorder(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for StartConfigurationRecorderError {}
/// Errors returned by StartRemediationExecution
#[derive(Debug, PartialEq)]
pub enum StartRemediationExecutionError {
    /// <p><p>Indicates one of the following errors:</p> <ul> <li> <p>For PutConfigRule, the rule cannot be created because the IAM role assigned to AWS Config lacks permissions to perform the config:Put* action.</p> </li> <li> <p>For PutConfigRule, the AWS Lambda function cannot be invoked. Check the function ARN, and check the function&#39;s permissions.</p> </li> <li> <p>For PutOrganizationConfigRule, organization config rule cannot be created because you do not have permissions to call IAM <code>GetRole</code> action or create a service linked role.</p> </li> <li> <p>For PutConformancePack and PutOrganizationConformancePack, a conformance pack cannot be created because you do not have permissions: </p> <ul> <li> <p>To call IAM <code>GetRole</code> action or create a service linked role.</p> </li> <li> <p>To read Amazon S3 bucket.</p> </li> </ul> </li> </ul></p>
    InsufficientPermissions(String),
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
    /// <p>You specified an AWS Config rule without a remediation configuration.</p>
    NoSuchRemediationConfiguration(String),
}

impl StartRemediationExecutionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartRemediationExecutionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InsufficientPermissionsException" => {
                    return RusotoError::Service(
                        StartRemediationExecutionError::InsufficientPermissions(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        StartRemediationExecutionError::InvalidParameterValue(err.msg),
                    )
                }
                "NoSuchRemediationConfigurationException" => {
                    return RusotoError::Service(
                        StartRemediationExecutionError::NoSuchRemediationConfiguration(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartRemediationExecutionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartRemediationExecutionError::InsufficientPermissions(ref cause) => {
                write!(f, "{}", cause)
            }
            StartRemediationExecutionError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            StartRemediationExecutionError::NoSuchRemediationConfiguration(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for StartRemediationExecutionError {}
/// Errors returned by StopConfigurationRecorder
#[derive(Debug, PartialEq)]
pub enum StopConfigurationRecorderError {
    /// <p>You have specified a configuration recorder that does not exist.</p>
    NoSuchConfigurationRecorder(String),
}

impl StopConfigurationRecorderError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopConfigurationRecorderError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NoSuchConfigurationRecorderException" => {
                    return RusotoError::Service(
                        StopConfigurationRecorderError::NoSuchConfigurationRecorder(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StopConfigurationRecorderError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopConfigurationRecorderError::NoSuchConfigurationRecorder(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for StopConfigurationRecorderError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>You have specified a resource that does not exist.</p>
    ResourceNotFound(String),
    /// <p>You have reached the limit of the number of tags you can use. You have more than 50 tags.</p>
    TooManyTags(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
                }
                "TooManyTagsException" => {
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
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::TooManyTags(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>You have specified a resource that does not exist.</p>
    ResourceNotFound(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
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
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Trait representing the capabilities of the Config Service API. Config Service clients implement this trait.
#[async_trait]
pub trait ConfigService {
    /// <p><p>Returns the current configuration items for resources that are present in your AWS Config aggregator. The operation also returns a list of resources that are not processed in the current request. If there are no unprocessed resources, the operation returns an empty <code>unprocessedResourceIdentifiers</code> list. </p> <note> <ul> <li> <p>The API does not return results for deleted resources.</p> </li> <li> <p> The API does not return tags and relationships.</p> </li> </ul> </note></p>
    async fn batch_get_aggregate_resource_config(
        &self,
        input: BatchGetAggregateResourceConfigRequest,
    ) -> Result<
        BatchGetAggregateResourceConfigResponse,
        RusotoError<BatchGetAggregateResourceConfigError>,
    >;

    /// <p><p>Returns the current configuration for one or more requested resources. The operation also returns a list of resources that are not processed in the current request. If there are no unprocessed resources, the operation returns an empty unprocessedResourceKeys list. </p> <note> <ul> <li> <p>The API does not return results for deleted resources.</p> </li> <li> <p> The API does not return any tags for the requested resources. This information is filtered out of the supplementaryConfiguration section of the API response.</p> </li> </ul> </note></p>
    async fn batch_get_resource_config(
        &self,
        input: BatchGetResourceConfigRequest,
    ) -> Result<BatchGetResourceConfigResponse, RusotoError<BatchGetResourceConfigError>>;

    /// <p>Deletes the authorization granted to the specified configuration aggregator account in a specified region.</p>
    async fn delete_aggregation_authorization(
        &self,
        input: DeleteAggregationAuthorizationRequest,
    ) -> Result<(), RusotoError<DeleteAggregationAuthorizationError>>;

    /// <p>Deletes the specified AWS Config rule and all of its evaluation results.</p> <p>AWS Config sets the state of a rule to <code>DELETING</code> until the deletion is complete. You cannot update a rule while it is in this state. If you make a <code>PutConfigRule</code> or <code>DeleteConfigRule</code> request for the rule, you will receive a <code>ResourceInUseException</code>.</p> <p>You can check the state of a rule by using the <code>DescribeConfigRules</code> request.</p>
    async fn delete_config_rule(
        &self,
        input: DeleteConfigRuleRequest,
    ) -> Result<(), RusotoError<DeleteConfigRuleError>>;

    /// <p>Deletes the specified configuration aggregator and the aggregated data associated with the aggregator.</p>
    async fn delete_configuration_aggregator(
        &self,
        input: DeleteConfigurationAggregatorRequest,
    ) -> Result<(), RusotoError<DeleteConfigurationAggregatorError>>;

    /// <p>Deletes the configuration recorder.</p> <p>After the configuration recorder is deleted, AWS Config will not record resource configuration changes until you create a new configuration recorder.</p> <p>This action does not delete the configuration information that was previously recorded. You will be able to access the previously recorded information by using the <code>GetResourceConfigHistory</code> action, but you will not be able to access this information in the AWS Config console until you create a new configuration recorder.</p>
    async fn delete_configuration_recorder(
        &self,
        input: DeleteConfigurationRecorderRequest,
    ) -> Result<(), RusotoError<DeleteConfigurationRecorderError>>;

    /// <p>Deletes the specified conformance pack and all the AWS Config rules, remediation actions, and all evaluation results within that conformance pack.</p> <p>AWS Config sets the conformance pack to <code>DELETE_IN_PROGRESS</code> until the deletion is complete. You cannot update a conformance pack while it is in this state.</p>
    async fn delete_conformance_pack(
        &self,
        input: DeleteConformancePackRequest,
    ) -> Result<(), RusotoError<DeleteConformancePackError>>;

    /// <p>Deletes the delivery channel.</p> <p>Before you can delete the delivery channel, you must stop the configuration recorder by using the <a>StopConfigurationRecorder</a> action.</p>
    async fn delete_delivery_channel(
        &self,
        input: DeleteDeliveryChannelRequest,
    ) -> Result<(), RusotoError<DeleteDeliveryChannelError>>;

    /// <p>Deletes the evaluation results for the specified AWS Config rule. You can specify one AWS Config rule per request. After you delete the evaluation results, you can call the <a>StartConfigRulesEvaluation</a> API to start evaluating your AWS resources against the rule.</p>
    async fn delete_evaluation_results(
        &self,
        input: DeleteEvaluationResultsRequest,
    ) -> Result<DeleteEvaluationResultsResponse, RusotoError<DeleteEvaluationResultsError>>;

    /// <p>Deletes the specified organization config rule and all of its evaluation results from all member accounts in that organization. Only a master account can delete an organization config rule.</p> <p>AWS Config sets the state of a rule to DELETE_IN_PROGRESS until the deletion is complete. You cannot update a rule while it is in this state.</p>
    async fn delete_organization_config_rule(
        &self,
        input: DeleteOrganizationConfigRuleRequest,
    ) -> Result<(), RusotoError<DeleteOrganizationConfigRuleError>>;

    /// <p>Deletes the specified organization conformance pack and all of the config rules and remediation actions from all member accounts in that organization. Only a master account can delete an organization conformance pack.</p> <p>AWS Config sets the state of a conformance pack to DELETE_IN_PROGRESS until the deletion is complete. You cannot update a conformance pack while it is in this state. </p>
    async fn delete_organization_conformance_pack(
        &self,
        input: DeleteOrganizationConformancePackRequest,
    ) -> Result<(), RusotoError<DeleteOrganizationConformancePackError>>;

    /// <p>Deletes pending authorization requests for a specified aggregator account in a specified region.</p>
    async fn delete_pending_aggregation_request(
        &self,
        input: DeletePendingAggregationRequestRequest,
    ) -> Result<(), RusotoError<DeletePendingAggregationRequestError>>;

    /// <p>Deletes the remediation configuration.</p>
    async fn delete_remediation_configuration(
        &self,
        input: DeleteRemediationConfigurationRequest,
    ) -> Result<
        DeleteRemediationConfigurationResponse,
        RusotoError<DeleteRemediationConfigurationError>,
    >;

    /// <p>Deletes one or more remediation exceptions mentioned in the resource keys.</p>
    async fn delete_remediation_exceptions(
        &self,
        input: DeleteRemediationExceptionsRequest,
    ) -> Result<DeleteRemediationExceptionsResponse, RusotoError<DeleteRemediationExceptionsError>>;

    /// <p>Records the configuration state for a custom resource that has been deleted. This API records a new ConfigurationItem with a ResourceDeleted status. You can retrieve the ConfigurationItems recorded for this resource in your AWS Config History. </p>
    async fn delete_resource_config(
        &self,
        input: DeleteResourceConfigRequest,
    ) -> Result<(), RusotoError<DeleteResourceConfigError>>;

    /// <p>Deletes the retention configuration.</p>
    async fn delete_retention_configuration(
        &self,
        input: DeleteRetentionConfigurationRequest,
    ) -> Result<(), RusotoError<DeleteRetentionConfigurationError>>;

    /// <p><p>Schedules delivery of a configuration snapshot to the Amazon S3 bucket in the specified delivery channel. After the delivery has started, AWS Config sends the following notifications using an Amazon SNS topic that you have specified.</p> <ul> <li> <p>Notification of the start of the delivery.</p> </li> <li> <p>Notification of the completion of the delivery, if the delivery was successfully completed.</p> </li> <li> <p>Notification of delivery failure, if the delivery failed.</p> </li> </ul></p>
    async fn deliver_config_snapshot(
        &self,
        input: DeliverConfigSnapshotRequest,
    ) -> Result<DeliverConfigSnapshotResponse, RusotoError<DeliverConfigSnapshotError>>;

    /// <p><p>Returns a list of compliant and noncompliant rules with the number of resources for compliant and noncompliant rules. </p> <note> <p>The results can return an empty result page, but if you have a <code>nextToken</code>, the results are displayed on the next page.</p> </note></p>
    async fn describe_aggregate_compliance_by_config_rules(
        &self,
        input: DescribeAggregateComplianceByConfigRulesRequest,
    ) -> Result<
        DescribeAggregateComplianceByConfigRulesResponse,
        RusotoError<DescribeAggregateComplianceByConfigRulesError>,
    >;

    /// <p>Returns a list of authorizations granted to various aggregator accounts and regions.</p>
    async fn describe_aggregation_authorizations(
        &self,
        input: DescribeAggregationAuthorizationsRequest,
    ) -> Result<
        DescribeAggregationAuthorizationsResponse,
        RusotoError<DescribeAggregationAuthorizationsError>,
    >;

    /// <p><p>Indicates whether the specified AWS Config rules are compliant. If a rule is noncompliant, this action returns the number of AWS resources that do not comply with the rule.</p> <p>A rule is compliant if all of the evaluated resources comply with it. It is noncompliant if any of these resources do not comply.</p> <p>If AWS Config has no current evaluation results for the rule, it returns <code>INSUFFICIENT<em>DATA</code>. This result might indicate one of the following conditions:</p> <ul> <li> <p>AWS Config has never invoked an evaluation for the rule. To check whether it has, use the <code>DescribeConfigRuleEvaluationStatus</code> action to get the <code>LastSuccessfulInvocationTime</code> and <code>LastFailedInvocationTime</code>.</p> </li> <li> <p>The rule&#39;s AWS Lambda function is failing to send evaluation results to AWS Config. Verify that the role you assigned to your configuration recorder includes the <code>config:PutEvaluations</code> permission. If the rule is a custom rule, verify that the AWS Lambda execution role includes the <code>config:PutEvaluations</code> permission.</p> </li> <li> <p>The rule&#39;s AWS Lambda function has returned <code>NOT</em>APPLICABLE</code> for all evaluation results. This can occur if the resources were deleted or removed from the rule&#39;s scope.</p> </li> </ul></p>
    async fn describe_compliance_by_config_rule(
        &self,
        input: DescribeComplianceByConfigRuleRequest,
    ) -> Result<
        DescribeComplianceByConfigRuleResponse,
        RusotoError<DescribeComplianceByConfigRuleError>,
    >;

    /// <p><p>Indicates whether the specified AWS resources are compliant. If a resource is noncompliant, this action returns the number of AWS Config rules that the resource does not comply with.</p> <p>A resource is compliant if it complies with all the AWS Config rules that evaluate it. It is noncompliant if it does not comply with one or more of these rules.</p> <p>If AWS Config has no current evaluation results for the resource, it returns <code>INSUFFICIENT<em>DATA</code>. This result might indicate one of the following conditions about the rules that evaluate the resource:</p> <ul> <li> <p>AWS Config has never invoked an evaluation for the rule. To check whether it has, use the <code>DescribeConfigRuleEvaluationStatus</code> action to get the <code>LastSuccessfulInvocationTime</code> and <code>LastFailedInvocationTime</code>.</p> </li> <li> <p>The rule&#39;s AWS Lambda function is failing to send evaluation results to AWS Config. Verify that the role that you assigned to your configuration recorder includes the <code>config:PutEvaluations</code> permission. If the rule is a custom rule, verify that the AWS Lambda execution role includes the <code>config:PutEvaluations</code> permission.</p> </li> <li> <p>The rule&#39;s AWS Lambda function has returned <code>NOT</em>APPLICABLE</code> for all evaluation results. This can occur if the resources were deleted or removed from the rule&#39;s scope.</p> </li> </ul></p>
    async fn describe_compliance_by_resource(
        &self,
        input: DescribeComplianceByResourceRequest,
    ) -> Result<DescribeComplianceByResourceResponse, RusotoError<DescribeComplianceByResourceError>>;

    /// <p>Returns status information for each of your AWS managed Config rules. The status includes information such as the last time AWS Config invoked the rule, the last time AWS Config failed to invoke the rule, and the related error for the last failure.</p>
    async fn describe_config_rule_evaluation_status(
        &self,
        input: DescribeConfigRuleEvaluationStatusRequest,
    ) -> Result<
        DescribeConfigRuleEvaluationStatusResponse,
        RusotoError<DescribeConfigRuleEvaluationStatusError>,
    >;

    /// <p>Returns details about your AWS Config rules.</p>
    async fn describe_config_rules(
        &self,
        input: DescribeConfigRulesRequest,
    ) -> Result<DescribeConfigRulesResponse, RusotoError<DescribeConfigRulesError>>;

    /// <p>Returns status information for sources within an aggregator. The status includes information about the last time AWS Config verified authorization between the source account and an aggregator account. In case of a failure, the status contains the related error code or message. </p>
    async fn describe_configuration_aggregator_sources_status(
        &self,
        input: DescribeConfigurationAggregatorSourcesStatusRequest,
    ) -> Result<
        DescribeConfigurationAggregatorSourcesStatusResponse,
        RusotoError<DescribeConfigurationAggregatorSourcesStatusError>,
    >;

    /// <p>Returns the details of one or more configuration aggregators. If the configuration aggregator is not specified, this action returns the details for all the configuration aggregators associated with the account. </p>
    async fn describe_configuration_aggregators(
        &self,
        input: DescribeConfigurationAggregatorsRequest,
    ) -> Result<
        DescribeConfigurationAggregatorsResponse,
        RusotoError<DescribeConfigurationAggregatorsError>,
    >;

    /// <p><p>Returns the current status of the specified configuration recorder. If a configuration recorder is not specified, this action returns the status of all configuration recorders associated with the account.</p> <note> <p>Currently, you can specify only one configuration recorder per region in your account.</p> </note></p>
    async fn describe_configuration_recorder_status(
        &self,
        input: DescribeConfigurationRecorderStatusRequest,
    ) -> Result<
        DescribeConfigurationRecorderStatusResponse,
        RusotoError<DescribeConfigurationRecorderStatusError>,
    >;

    /// <p><p>Returns the details for the specified configuration recorders. If the configuration recorder is not specified, this action returns the details for all configuration recorders associated with the account.</p> <note> <p>Currently, you can specify only one configuration recorder per region in your account.</p> </note></p>
    async fn describe_configuration_recorders(
        &self,
        input: DescribeConfigurationRecordersRequest,
    ) -> Result<
        DescribeConfigurationRecordersResponse,
        RusotoError<DescribeConfigurationRecordersError>,
    >;

    /// <p><p>Returns compliance details for each rule in that conformance pack.</p> <note> <p>You must provide exact rule names.</p> </note></p>
    async fn describe_conformance_pack_compliance(
        &self,
        input: DescribeConformancePackComplianceRequest,
    ) -> Result<
        DescribeConformancePackComplianceResponse,
        RusotoError<DescribeConformancePackComplianceError>,
    >;

    /// <p><p>Provides one or more conformance packs deployment status.</p> <note> <p>If there are no conformance packs then you will see an empty result.</p> </note></p>
    async fn describe_conformance_pack_status(
        &self,
        input: DescribeConformancePackStatusRequest,
    ) -> Result<
        DescribeConformancePackStatusResponse,
        RusotoError<DescribeConformancePackStatusError>,
    >;

    /// <p>Returns a list of one or more conformance packs.</p>
    async fn describe_conformance_packs(
        &self,
        input: DescribeConformancePacksRequest,
    ) -> Result<DescribeConformancePacksResponse, RusotoError<DescribeConformancePacksError>>;

    /// <p><p>Returns the current status of the specified delivery channel. If a delivery channel is not specified, this action returns the current status of all delivery channels associated with the account.</p> <note> <p>Currently, you can specify only one delivery channel per region in your account.</p> </note></p>
    async fn describe_delivery_channel_status(
        &self,
        input: DescribeDeliveryChannelStatusRequest,
    ) -> Result<
        DescribeDeliveryChannelStatusResponse,
        RusotoError<DescribeDeliveryChannelStatusError>,
    >;

    /// <p><p>Returns details about the specified delivery channel. If a delivery channel is not specified, this action returns the details of all delivery channels associated with the account.</p> <note> <p>Currently, you can specify only one delivery channel per region in your account.</p> </note></p>
    async fn describe_delivery_channels(
        &self,
        input: DescribeDeliveryChannelsRequest,
    ) -> Result<DescribeDeliveryChannelsResponse, RusotoError<DescribeDeliveryChannelsError>>;

    /// <p><p>Provides organization config rule deployment status for an organization.</p> <note> <p>The status is not considered successful until organization config rule is successfully deployed in all the member accounts with an exception of excluded accounts.</p> <p>When you specify the limit and the next token, you receive a paginated response. Limit and next token are not applicable if you specify organization config rule names. It is only applicable, when you request all the organization config rules.</p> <p>Only a master account can call this API.</p> </note></p>
    async fn describe_organization_config_rule_statuses(
        &self,
        input: DescribeOrganizationConfigRuleStatusesRequest,
    ) -> Result<
        DescribeOrganizationConfigRuleStatusesResponse,
        RusotoError<DescribeOrganizationConfigRuleStatusesError>,
    >;

    /// <p><p>Returns a list of organization config rules.</p> <note> <p>When you specify the limit and the next token, you receive a paginated response. Limit and next token are not applicable if you specify organization config rule names. It is only applicable, when you request all the organization config rules.</p> <p>Only a master account can call this API.</p> </note></p>
    async fn describe_organization_config_rules(
        &self,
        input: DescribeOrganizationConfigRulesRequest,
    ) -> Result<
        DescribeOrganizationConfigRulesResponse,
        RusotoError<DescribeOrganizationConfigRulesError>,
    >;

    /// <p><p>Provides organization conformance pack deployment status for an organization.</p> <note> <p>The status is not considered successful until organization conformance pack is successfully deployed in all the member accounts with an exception of excluded accounts.</p> <p>When you specify the limit and the next token, you receive a paginated response. Limit and next token are not applicable if you specify organization conformance pack names. They are only applicable, when you request all the organization conformance packs.</p> <p>Only a master account can call this API.</p> </note></p>
    async fn describe_organization_conformance_pack_statuses(
        &self,
        input: DescribeOrganizationConformancePackStatusesRequest,
    ) -> Result<
        DescribeOrganizationConformancePackStatusesResponse,
        RusotoError<DescribeOrganizationConformancePackStatusesError>,
    >;

    /// <p><p>Returns a list of organization conformance packs.</p> <note> <p>When you specify the limit and the next token, you receive a paginated response. </p> <p>Limit and next token are not applicable if you specify organization conformance packs names. They are only applicable, when you request all the organization conformance packs. </p> <p>Only a master account can call this API.</p> </note></p>
    async fn describe_organization_conformance_packs(
        &self,
        input: DescribeOrganizationConformancePacksRequest,
    ) -> Result<
        DescribeOrganizationConformancePacksResponse,
        RusotoError<DescribeOrganizationConformancePacksError>,
    >;

    /// <p>Returns a list of all pending aggregation requests.</p>
    async fn describe_pending_aggregation_requests(
        &self,
        input: DescribePendingAggregationRequestsRequest,
    ) -> Result<
        DescribePendingAggregationRequestsResponse,
        RusotoError<DescribePendingAggregationRequestsError>,
    >;

    /// <p>Returns the details of one or more remediation configurations.</p>
    async fn describe_remediation_configurations(
        &self,
        input: DescribeRemediationConfigurationsRequest,
    ) -> Result<
        DescribeRemediationConfigurationsResponse,
        RusotoError<DescribeRemediationConfigurationsError>,
    >;

    /// <p><p>Returns the details of one or more remediation exceptions. A detailed view of a remediation exception for a set of resources that includes an explanation of an exception and the time when the exception will be deleted. When you specify the limit and the next token, you receive a paginated response. </p> <note> <p>When you specify the limit and the next token, you receive a paginated response. </p> <p>Limit and next token are not applicable if you request resources in batch. It is only applicable, when you request all resources.</p> </note></p>
    async fn describe_remediation_exceptions(
        &self,
        input: DescribeRemediationExceptionsRequest,
    ) -> Result<
        DescribeRemediationExceptionsResponse,
        RusotoError<DescribeRemediationExceptionsError>,
    >;

    /// <p>Provides a detailed view of a Remediation Execution for a set of resources including state, timestamps for when steps for the remediation execution occur, and any error messages for steps that have failed. When you specify the limit and the next token, you receive a paginated response.</p>
    async fn describe_remediation_execution_status(
        &self,
        input: DescribeRemediationExecutionStatusRequest,
    ) -> Result<
        DescribeRemediationExecutionStatusResponse,
        RusotoError<DescribeRemediationExecutionStatusError>,
    >;

    /// <p><p>Returns the details of one or more retention configurations. If the retention configuration name is not specified, this action returns the details for all the retention configurations for that account.</p> <note> <p>Currently, AWS Config supports only one retention configuration per region in your account.</p> </note></p>
    async fn describe_retention_configurations(
        &self,
        input: DescribeRetentionConfigurationsRequest,
    ) -> Result<
        DescribeRetentionConfigurationsResponse,
        RusotoError<DescribeRetentionConfigurationsError>,
    >;

    /// <p><p>Returns the evaluation results for the specified AWS Config rule for a specific resource in a rule. The results indicate which AWS resources were evaluated by the rule, when each resource was last evaluated, and whether each resource complies with the rule. </p> <note> <p>The results can return an empty result page. But if you have a <code>nextToken</code>, the results are displayed on the next page.</p> </note></p>
    async fn get_aggregate_compliance_details_by_config_rule(
        &self,
        input: GetAggregateComplianceDetailsByConfigRuleRequest,
    ) -> Result<
        GetAggregateComplianceDetailsByConfigRuleResponse,
        RusotoError<GetAggregateComplianceDetailsByConfigRuleError>,
    >;

    /// <p><p>Returns the number of compliant and noncompliant rules for one or more accounts and regions in an aggregator.</p> <note> <p>The results can return an empty result page, but if you have a nextToken, the results are displayed on the next page.</p> </note></p>
    async fn get_aggregate_config_rule_compliance_summary(
        &self,
        input: GetAggregateConfigRuleComplianceSummaryRequest,
    ) -> Result<
        GetAggregateConfigRuleComplianceSummaryResponse,
        RusotoError<GetAggregateConfigRuleComplianceSummaryError>,
    >;

    /// <p>Returns the resource counts across accounts and regions that are present in your AWS Config aggregator. You can request the resource counts by providing filters and GroupByKey.</p> <p>For example, if the input contains accountID 12345678910 and region us-east-1 in filters, the API returns the count of resources in account ID 12345678910 and region us-east-1. If the input contains ACCOUNT_ID as a GroupByKey, the API returns resource counts for all source accounts that are present in your aggregator.</p>
    async fn get_aggregate_discovered_resource_counts(
        &self,
        input: GetAggregateDiscoveredResourceCountsRequest,
    ) -> Result<
        GetAggregateDiscoveredResourceCountsResponse,
        RusotoError<GetAggregateDiscoveredResourceCountsError>,
    >;

    /// <p>Returns configuration item that is aggregated for your specific resource in a specific source account and region.</p>
    async fn get_aggregate_resource_config(
        &self,
        input: GetAggregateResourceConfigRequest,
    ) -> Result<GetAggregateResourceConfigResponse, RusotoError<GetAggregateResourceConfigError>>;

    /// <p>Returns the evaluation results for the specified AWS Config rule. The results indicate which AWS resources were evaluated by the rule, when each resource was last evaluated, and whether each resource complies with the rule.</p>
    async fn get_compliance_details_by_config_rule(
        &self,
        input: GetComplianceDetailsByConfigRuleRequest,
    ) -> Result<
        GetComplianceDetailsByConfigRuleResponse,
        RusotoError<GetComplianceDetailsByConfigRuleError>,
    >;

    /// <p>Returns the evaluation results for the specified AWS resource. The results indicate which AWS Config rules were used to evaluate the resource, when each rule was last used, and whether the resource complies with each rule.</p>
    async fn get_compliance_details_by_resource(
        &self,
        input: GetComplianceDetailsByResourceRequest,
    ) -> Result<
        GetComplianceDetailsByResourceResponse,
        RusotoError<GetComplianceDetailsByResourceError>,
    >;

    /// <p>Returns the number of AWS Config rules that are compliant and noncompliant, up to a maximum of 25 for each.</p>
    async fn get_compliance_summary_by_config_rule(
        &self,
    ) -> Result<
        GetComplianceSummaryByConfigRuleResponse,
        RusotoError<GetComplianceSummaryByConfigRuleError>,
    >;

    /// <p>Returns the number of resources that are compliant and the number that are noncompliant. You can specify one or more resource types to get these numbers for each resource type. The maximum number returned is 100.</p>
    async fn get_compliance_summary_by_resource_type(
        &self,
        input: GetComplianceSummaryByResourceTypeRequest,
    ) -> Result<
        GetComplianceSummaryByResourceTypeResponse,
        RusotoError<GetComplianceSummaryByResourceTypeError>,
    >;

    /// <p>Returns compliance details of a conformance pack for all AWS resources that are monitered by conformance pack.</p>
    async fn get_conformance_pack_compliance_details(
        &self,
        input: GetConformancePackComplianceDetailsRequest,
    ) -> Result<
        GetConformancePackComplianceDetailsResponse,
        RusotoError<GetConformancePackComplianceDetailsError>,
    >;

    /// <p>Returns compliance details for the conformance pack based on the cumulative compliance results of all the rules in that conformance pack.</p>
    async fn get_conformance_pack_compliance_summary(
        &self,
        input: GetConformancePackComplianceSummaryRequest,
    ) -> Result<
        GetConformancePackComplianceSummaryResponse,
        RusotoError<GetConformancePackComplianceSummaryError>,
    >;

    /// <p><p>Returns the resource types, the number of each resource type, and the total number of resources that AWS Config is recording in this region for your AWS account. </p> <p class="title"> <b>Example</b> </p> <ol> <li> <p>AWS Config is recording three resource types in the US East (Ohio) Region for your account: 25 EC2 instances, 20 IAM users, and 15 S3 buckets.</p> </li> <li> <p>You make a call to the <code>GetDiscoveredResourceCounts</code> action and specify that you want all resource types. </p> </li> <li> <p>AWS Config returns the following:</p> <ul> <li> <p>The resource types (EC2 instances, IAM users, and S3 buckets).</p> </li> <li> <p>The number of each resource type (25, 20, and 15).</p> </li> <li> <p>The total number of all resources (60).</p> </li> </ul> </li> </ol> <p>The response is paginated. By default, AWS Config lists 100 <a>ResourceCount</a> objects on each page. You can customize this number with the <code>limit</code> parameter. The response includes a <code>nextToken</code> string. To get the next page of results, run the request again and specify the string for the <code>nextToken</code> parameter.</p> <note> <p>If you make a call to the <a>GetDiscoveredResourceCounts</a> action, you might not immediately receive resource counts in the following situations:</p> <ul> <li> <p>You are a new AWS Config customer.</p> </li> <li> <p>You just enabled resource recording.</p> </li> </ul> <p>It might take a few minutes for AWS Config to record and count your resources. Wait a few minutes and then retry the <a>GetDiscoveredResourceCounts</a> action. </p> </note></p>
    async fn get_discovered_resource_counts(
        &self,
        input: GetDiscoveredResourceCountsRequest,
    ) -> Result<GetDiscoveredResourceCountsResponse, RusotoError<GetDiscoveredResourceCountsError>>;

    /// <p><p>Returns detailed status for each member account within an organization for a given organization config rule.</p> <note> <p>Only a master account can call this API.</p> </note></p>
    async fn get_organization_config_rule_detailed_status(
        &self,
        input: GetOrganizationConfigRuleDetailedStatusRequest,
    ) -> Result<
        GetOrganizationConfigRuleDetailedStatusResponse,
        RusotoError<GetOrganizationConfigRuleDetailedStatusError>,
    >;

    /// <p>Returns detailed status for each member account within an organization for a given organization conformance pack.</p> <p>Only a master account can call this API.</p>
    async fn get_organization_conformance_pack_detailed_status(
        &self,
        input: GetOrganizationConformancePackDetailedStatusRequest,
    ) -> Result<
        GetOrganizationConformancePackDetailedStatusResponse,
        RusotoError<GetOrganizationConformancePackDetailedStatusError>,
    >;

    /// <p><p>Returns a list of configuration items for the specified resource. The list contains details about each state of the resource during the specified time interval. If you specified a retention period to retain your <code>ConfigurationItems</code> between a minimum of 30 days and a maximum of 7 years (2557 days), AWS Config returns the <code>ConfigurationItems</code> for the specified retention period. </p> <p>The response is paginated. By default, AWS Config returns a limit of 10 configuration items per page. You can customize this number with the <code>limit</code> parameter. The response includes a <code>nextToken</code> string. To get the next page of results, run the request again and specify the string for the <code>nextToken</code> parameter.</p> <note> <p>Each call to the API is limited to span a duration of seven days. It is likely that the number of records returned is smaller than the specified <code>limit</code>. In such cases, you can make another call, using the <code>nextToken</code>.</p> </note></p>
    async fn get_resource_config_history(
        &self,
        input: GetResourceConfigHistoryRequest,
    ) -> Result<GetResourceConfigHistoryResponse, RusotoError<GetResourceConfigHistoryError>>;

    /// <p>Accepts a resource type and returns a list of resource identifiers that are aggregated for a specific resource type across accounts and regions. A resource identifier includes the resource type, ID, (if available) the custom resource name, source account, and source region. You can narrow the results to include only resources that have specific resource IDs, or a resource name, or source account ID, or source region.</p> <p>For example, if the input consists of accountID 12345678910 and the region is us-east-1 for resource type <code>AWS::EC2::Instance</code> then the API returns all the EC2 instance identifiers of accountID 12345678910 and region us-east-1.</p>
    async fn list_aggregate_discovered_resources(
        &self,
        input: ListAggregateDiscoveredResourcesRequest,
    ) -> Result<
        ListAggregateDiscoveredResourcesResponse,
        RusotoError<ListAggregateDiscoveredResourcesError>,
    >;

    /// <p>Accepts a resource type and returns a list of resource identifiers for the resources of that type. A resource identifier includes the resource type, ID, and (if available) the custom resource name. The results consist of resources that AWS Config has discovered, including those that AWS Config is not currently recording. You can narrow the results to include only resources that have specific resource IDs or a resource name.</p> <note> <p>You can specify either resource IDs or a resource name, but not both, in the same request.</p> </note> <p>The response is paginated. By default, AWS Config lists 100 resource identifiers on each page. You can customize this number with the <code>limit</code> parameter. The response includes a <code>nextToken</code> string. To get the next page of results, run the request again and specify the string for the <code>nextToken</code> parameter.</p>
    async fn list_discovered_resources(
        &self,
        input: ListDiscoveredResourcesRequest,
    ) -> Result<ListDiscoveredResourcesResponse, RusotoError<ListDiscoveredResourcesError>>;

    /// <p>List the tags for AWS Config resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Authorizes the aggregator account and region to collect data from the source account and region. </p>
    async fn put_aggregation_authorization(
        &self,
        input: PutAggregationAuthorizationRequest,
    ) -> Result<PutAggregationAuthorizationResponse, RusotoError<PutAggregationAuthorizationError>>;

    /// <p>Adds or updates an AWS Config rule for evaluating whether your AWS resources comply with your desired configurations.</p> <p>You can use this action for custom AWS Config rules and AWS managed Config rules. A custom AWS Config rule is a rule that you develop and maintain. An AWS managed Config rule is a customizable, predefined rule that AWS Config provides.</p> <p>If you are adding a new custom AWS Config rule, you must first create the AWS Lambda function that the rule invokes to evaluate your resources. When you use the <code>PutConfigRule</code> action to add the rule to AWS Config, you must specify the Amazon Resource Name (ARN) that AWS Lambda assigns to the function. Specify the ARN for the <code>SourceIdentifier</code> key. This key is part of the <code>Source</code> object, which is part of the <code>ConfigRule</code> object. </p> <p>If you are adding an AWS managed Config rule, specify the rule's identifier for the <code>SourceIdentifier</code> key. To reference AWS managed Config rule identifiers, see <a href="https://docs.aws.amazon.com/config/latest/developerguide/evaluate-config_use-managed-rules.html">About AWS Managed Config Rules</a>.</p> <p>For any new rule that you add, specify the <code>ConfigRuleName</code> in the <code>ConfigRule</code> object. Do not specify the <code>ConfigRuleArn</code> or the <code>ConfigRuleId</code>. These values are generated by AWS Config for new rules.</p> <p>If you are updating a rule that you added previously, you can specify the rule by <code>ConfigRuleName</code>, <code>ConfigRuleId</code>, or <code>ConfigRuleArn</code> in the <code>ConfigRule</code> data type that you use in this request.</p> <p>The maximum number of rules that AWS Config supports is 150.</p> <p>For information about requesting a rule limit increase, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_config">AWS Config Limits</a> in the <i>AWS General Reference Guide</i>.</p> <p>For more information about developing and using AWS Config rules, see <a href="https://docs.aws.amazon.com/config/latest/developerguide/evaluate-config.html">Evaluating AWS Resource Configurations with AWS Config</a> in the <i>AWS Config Developer Guide</i>.</p>
    async fn put_config_rule(
        &self,
        input: PutConfigRuleRequest,
    ) -> Result<(), RusotoError<PutConfigRuleError>>;

    /// <p><p>Creates and updates the configuration aggregator with the selected source accounts and regions. The source account can be individual account(s) or an organization.</p> <note> <p>AWS Config should be enabled in source accounts and regions you want to aggregate.</p> <p>If your source type is an organization, you must be signed in to the master account and all features must be enabled in your organization. AWS Config calls <code>EnableAwsServiceAccess</code> API to enable integration between AWS Config and AWS Organizations. </p> </note></p>
    async fn put_configuration_aggregator(
        &self,
        input: PutConfigurationAggregatorRequest,
    ) -> Result<PutConfigurationAggregatorResponse, RusotoError<PutConfigurationAggregatorError>>;

    /// <p><p>Creates a new configuration recorder to record the selected resource configurations.</p> <p>You can use this action to change the role <code>roleARN</code> or the <code>recordingGroup</code> of an existing recorder. To change the role, call the action on the existing configuration recorder and specify a role.</p> <note> <p>Currently, you can specify only one configuration recorder per region in your account.</p> <p>If <code>ConfigurationRecorder</code> does not have the <b>recordingGroup</b> parameter specified, the default is to record all supported resource types.</p> </note></p>
    async fn put_configuration_recorder(
        &self,
        input: PutConfigurationRecorderRequest,
    ) -> Result<(), RusotoError<PutConfigurationRecorderError>>;

    /// <p><p>Creates or updates a conformance pack. A conformance pack is a collection of AWS Config rules that can be easily deployed in an account and a region and across AWS Organization.</p> <p>This API creates a service linked role <code>AWSServiceRoleForConfigConforms</code> in your account. The service linked role is created only when the role does not exist in your account. AWS Config verifies the existence of role with <code>GetRole</code> action.</p> <note> <p>You must specify either the <code>TemplateS3Uri</code> or the <code>TemplateBody</code> parameter, but not both. If you provide both AWS Config uses the <code>TemplateS3Uri</code> parameter and ignores the <code>TemplateBody</code> parameter.</p> </note></p>
    async fn put_conformance_pack(
        &self,
        input: PutConformancePackRequest,
    ) -> Result<PutConformancePackResponse, RusotoError<PutConformancePackError>>;

    /// <p><p>Creates a delivery channel object to deliver configuration information to an Amazon S3 bucket and Amazon SNS topic.</p> <p>Before you can create a delivery channel, you must create a configuration recorder.</p> <p>You can use this action to change the Amazon S3 bucket or an Amazon SNS topic of the existing delivery channel. To change the Amazon S3 bucket or an Amazon SNS topic, call this action and specify the changed values for the S3 bucket and the SNS topic. If you specify a different value for either the S3 bucket or the SNS topic, this action will keep the existing value for the parameter that is not changed.</p> <note> <p>You can have only one delivery channel per region in your account.</p> </note></p>
    async fn put_delivery_channel(
        &self,
        input: PutDeliveryChannelRequest,
    ) -> Result<(), RusotoError<PutDeliveryChannelError>>;

    /// <p>Used by an AWS Lambda function to deliver evaluation results to AWS Config. This action is required in every AWS Lambda function that is invoked by an AWS Config rule.</p>
    async fn put_evaluations(
        &self,
        input: PutEvaluationsRequest,
    ) -> Result<PutEvaluationsResponse, RusotoError<PutEvaluationsError>>;

    /// <p><p>Adds or updates organization config rule for your entire organization evaluating whether your AWS resources comply with your desired configurations. Only a master account can create or update an organization config rule.</p> <p>This API enables organization service access through the <code>EnableAWSServiceAccess</code> action and creates a service linked role <code>AWSServiceRoleForConfigMultiAccountSetup</code> in the master account of your organization. The service linked role is created only when the role does not exist in the master account. AWS Config verifies the existence of role with <code>GetRole</code> action.</p> <p>You can use this action to create both custom AWS Config rules and AWS managed Config rules. If you are adding a new custom AWS Config rule, you must first create AWS Lambda function in the master account that the rule invokes to evaluate your resources. When you use the <code>PutOrganizationConfigRule</code> action to add the rule to AWS Config, you must specify the Amazon Resource Name (ARN) that AWS Lambda assigns to the function. If you are adding an AWS managed Config rule, specify the rule&#39;s identifier for the <code>RuleIdentifier</code> key.</p> <p>The maximum number of organization config rules that AWS Config supports is 150.</p> <note> <p>Specify either <code>OrganizationCustomRuleMetadata</code> or <code>OrganizationManagedRuleMetadata</code>.</p> </note></p>
    async fn put_organization_config_rule(
        &self,
        input: PutOrganizationConfigRuleRequest,
    ) -> Result<PutOrganizationConfigRuleResponse, RusotoError<PutOrganizationConfigRuleError>>;

    /// <p><p>Deploys conformance packs across member accounts in an AWS Organization.</p> <p>This API enables organization service access for <code>config-multiaccountsetup.amazonaws.com</code> through the <code>EnableAWSServiceAccess</code> action and creates a service linked role <code>AWSServiceRoleForConfigMultiAccountSetup</code> in the master account of your organization. The service linked role is created only when the role does not exist in the master account. AWS Config verifies the existence of role with GetRole action.</p> <note> <p>You must specify either the <code>TemplateS3Uri</code> or the <code>TemplateBody</code> parameter, but not both. If you provide both AWS Config uses the <code>TemplateS3Uri</code> parameter and ignores the <code>TemplateBody</code> parameter.</p> <p>AWS Config sets the state of a conformance pack to CREATE<em>IN</em>PROGRESS and UPDATE<em>IN</em>PROGRESS until the confomance pack is created or updated. You cannot update a conformance pack while it is in this state.</p> <p>You can create 6 conformance packs with 25 AWS Config rules in each pack.</p> </note></p>
    async fn put_organization_conformance_pack(
        &self,
        input: PutOrganizationConformancePackRequest,
    ) -> Result<
        PutOrganizationConformancePackResponse,
        RusotoError<PutOrganizationConformancePackError>,
    >;

    /// <p>Adds or updates the remediation configuration with a specific AWS Config rule with the selected target or action. The API creates the <code>RemediationConfiguration</code> object for the AWS Config rule. The AWS Config rule must already exist for you to add a remediation configuration. The target (SSM document) must exist and have permissions to use the target. </p>
    async fn put_remediation_configurations(
        &self,
        input: PutRemediationConfigurationsRequest,
    ) -> Result<PutRemediationConfigurationsResponse, RusotoError<PutRemediationConfigurationsError>>;

    /// <p>A remediation exception is when a specific resource is no longer considered for auto-remediation. This API adds a new exception or updates an exisiting exception for a specific resource with a specific AWS Config rule. </p>
    async fn put_remediation_exceptions(
        &self,
        input: PutRemediationExceptionsRequest,
    ) -> Result<PutRemediationExceptionsResponse, RusotoError<PutRemediationExceptionsError>>;

    /// <p><p>Records the configuration state for the resource provided in the request. The configuration state of a resource is represented in AWS Config as Configuration Items. Once this API records the configuration item, you can retrieve the list of configuration items for the custom resource type using existing AWS Config APIs. </p> <note> <p>The custom resource type must be registered with AWS CloudFormation. This API accepts the configuration item registered with AWS CloudFormation.</p> <p>When you call this API, AWS Config only stores configuration state of the resource provided in the request. This API does not change or remediate the configuration of the resource. </p> </note></p>
    async fn put_resource_config(
        &self,
        input: PutResourceConfigRequest,
    ) -> Result<(), RusotoError<PutResourceConfigError>>;

    /// <p><p>Creates and updates the retention configuration with details about retention period (number of days) that AWS Config stores your historical information. The API creates the <code>RetentionConfiguration</code> object and names the object as <b>default</b>. When you have a <code>RetentionConfiguration</code> object named <b>default</b>, calling the API modifies the default object. </p> <note> <p>Currently, AWS Config supports only one retention configuration per region in your account.</p> </note></p>
    async fn put_retention_configuration(
        &self,
        input: PutRetentionConfigurationRequest,
    ) -> Result<PutRetentionConfigurationResponse, RusotoError<PutRetentionConfigurationError>>;

    /// <p>Accepts a structured query language (SQL) <code>SELECT</code> command, performs the corresponding search, and returns resource configurations matching the properties.</p> <p>For more information about query components, see the <a href="https://docs.aws.amazon.com/config/latest/developerguide/query-components.html"> <b>Query Components</b> </a> section in the AWS Config Developer Guide.</p>
    async fn select_resource_config(
        &self,
        input: SelectResourceConfigRequest,
    ) -> Result<SelectResourceConfigResponse, RusotoError<SelectResourceConfigError>>;

    /// <p><p>Runs an on-demand evaluation for the specified AWS Config rules against the last known configuration state of the resources. Use <code>StartConfigRulesEvaluation</code> when you want to test that a rule you updated is working as expected. <code>StartConfigRulesEvaluation</code> does not re-record the latest configuration state for your resources. It re-runs an evaluation against the last known state of your resources. </p> <p>You can specify up to 25 AWS Config rules per request. </p> <p>An existing <code>StartConfigRulesEvaluation</code> call for the specified rules must complete before you can call the API again. If you chose to have AWS Config stream to an Amazon SNS topic, you will receive a <code>ConfigRuleEvaluationStarted</code> notification when the evaluation starts.</p> <note> <p>You don&#39;t need to call the <code>StartConfigRulesEvaluation</code> API to run an evaluation for a new rule. When you create a rule, AWS Config evaluates your resources against the rule automatically. </p> </note> <p>The <code>StartConfigRulesEvaluation</code> API is useful if you want to run on-demand evaluations, such as the following example:</p> <ol> <li> <p>You have a custom rule that evaluates your IAM resources every 24 hours.</p> </li> <li> <p>You update your Lambda function to add additional conditions to your rule.</p> </li> <li> <p>Instead of waiting for the next periodic evaluation, you call the <code>StartConfigRulesEvaluation</code> API.</p> </li> <li> <p>AWS Config invokes your Lambda function and evaluates your IAM resources.</p> </li> <li> <p>Your custom rule will still run periodic evaluations every 24 hours.</p> </li> </ol></p>
    async fn start_config_rules_evaluation(
        &self,
        input: StartConfigRulesEvaluationRequest,
    ) -> Result<StartConfigRulesEvaluationResponse, RusotoError<StartConfigRulesEvaluationError>>;

    /// <p>Starts recording configurations of the AWS resources you have selected to record in your AWS account.</p> <p>You must have created at least one delivery channel to successfully start the configuration recorder.</p>
    async fn start_configuration_recorder(
        &self,
        input: StartConfigurationRecorderRequest,
    ) -> Result<(), RusotoError<StartConfigurationRecorderError>>;

    /// <p>Runs an on-demand remediation for the specified AWS Config rules against the last known remediation configuration. It runs an execution against the current state of your resources. Remediation execution is asynchronous.</p> <p>You can specify up to 100 resource keys per request. An existing StartRemediationExecution call for the specified resource keys must complete before you can call the API again.</p>
    async fn start_remediation_execution(
        &self,
        input: StartRemediationExecutionRequest,
    ) -> Result<StartRemediationExecutionResponse, RusotoError<StartRemediationExecutionError>>;

    /// <p>Stops recording configurations of the AWS resources you have selected to record in your AWS account.</p>
    async fn stop_configuration_recorder(
        &self,
        input: StopConfigurationRecorderRequest,
    ) -> Result<(), RusotoError<StopConfigurationRecorderError>>;

    /// <p>Associates the specified tags to a resource with the specified resourceArn. If existing tags on a resource are not specified in the request parameters, they are not changed. When a resource is deleted, the tags associated with that resource are deleted as well.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<(), RusotoError<TagResourceError>>;

    /// <p>Deletes specified tags from a resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<(), RusotoError<UntagResourceError>>;
}
/// A client for the Config Service API.
#[derive(Clone)]
pub struct ConfigServiceClient {
    client: Client,
    region: region::Region,
}

impl ConfigServiceClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ConfigServiceClient {
        ConfigServiceClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ConfigServiceClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        ConfigServiceClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> ConfigServiceClient {
        ConfigServiceClient { client, region }
    }
}

#[async_trait]
impl ConfigService for ConfigServiceClient {
    /// <p><p>Returns the current configuration items for resources that are present in your AWS Config aggregator. The operation also returns a list of resources that are not processed in the current request. If there are no unprocessed resources, the operation returns an empty <code>unprocessedResourceIdentifiers</code> list. </p> <note> <ul> <li> <p>The API does not return results for deleted resources.</p> </li> <li> <p> The API does not return tags and relationships.</p> </li> </ul> </note></p>
    async fn batch_get_aggregate_resource_config(
        &self,
        input: BatchGetAggregateResourceConfigRequest,
    ) -> Result<
        BatchGetAggregateResourceConfigResponse,
        RusotoError<BatchGetAggregateResourceConfigError>,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.BatchGetAggregateResourceConfig",
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
                .deserialize::<BatchGetAggregateResourceConfigResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(BatchGetAggregateResourceConfigError::from_response(
                response,
            ))
        }
    }

    /// <p><p>Returns the current configuration for one or more requested resources. The operation also returns a list of resources that are not processed in the current request. If there are no unprocessed resources, the operation returns an empty unprocessedResourceKeys list. </p> <note> <ul> <li> <p>The API does not return results for deleted resources.</p> </li> <li> <p> The API does not return any tags for the requested resources. This information is filtered out of the supplementaryConfiguration section of the API response.</p> </li> </ul> </note></p>
    async fn batch_get_resource_config(
        &self,
        input: BatchGetResourceConfigRequest,
    ) -> Result<BatchGetResourceConfigResponse, RusotoError<BatchGetResourceConfigError>> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StarlingDoveService.BatchGetResourceConfig");
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
                .deserialize::<BatchGetResourceConfigResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(BatchGetResourceConfigError::from_response(response))
        }
    }

    /// <p>Deletes the authorization granted to the specified configuration aggregator account in a specified region.</p>
    async fn delete_aggregation_authorization(
        &self,
        input: DeleteAggregationAuthorizationRequest,
    ) -> Result<(), RusotoError<DeleteAggregationAuthorizationError>> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DeleteAggregationAuthorization",
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
            Err(DeleteAggregationAuthorizationError::from_response(response))
        }
    }

    /// <p>Deletes the specified AWS Config rule and all of its evaluation results.</p> <p>AWS Config sets the state of a rule to <code>DELETING</code> until the deletion is complete. You cannot update a rule while it is in this state. If you make a <code>PutConfigRule</code> or <code>DeleteConfigRule</code> request for the rule, you will receive a <code>ResourceInUseException</code>.</p> <p>You can check the state of a rule by using the <code>DescribeConfigRules</code> request.</p>
    async fn delete_config_rule(
        &self,
        input: DeleteConfigRuleRequest,
    ) -> Result<(), RusotoError<DeleteConfigRuleError>> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StarlingDoveService.DeleteConfigRule");
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
            Err(DeleteConfigRuleError::from_response(response))
        }
    }

    /// <p>Deletes the specified configuration aggregator and the aggregated data associated with the aggregator.</p>
    async fn delete_configuration_aggregator(
        &self,
        input: DeleteConfigurationAggregatorRequest,
    ) -> Result<(), RusotoError<DeleteConfigurationAggregatorError>> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DeleteConfigurationAggregator",
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
            Err(DeleteConfigurationAggregatorError::from_response(response))
        }
    }

    /// <p>Deletes the configuration recorder.</p> <p>After the configuration recorder is deleted, AWS Config will not record resource configuration changes until you create a new configuration recorder.</p> <p>This action does not delete the configuration information that was previously recorded. You will be able to access the previously recorded information by using the <code>GetResourceConfigHistory</code> action, but you will not be able to access this information in the AWS Config console until you create a new configuration recorder.</p>
    async fn delete_configuration_recorder(
        &self,
        input: DeleteConfigurationRecorderRequest,
    ) -> Result<(), RusotoError<DeleteConfigurationRecorderError>> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DeleteConfigurationRecorder",
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
            Err(DeleteConfigurationRecorderError::from_response(response))
        }
    }

    /// <p>Deletes the specified conformance pack and all the AWS Config rules, remediation actions, and all evaluation results within that conformance pack.</p> <p>AWS Config sets the conformance pack to <code>DELETE_IN_PROGRESS</code> until the deletion is complete. You cannot update a conformance pack while it is in this state.</p>
    async fn delete_conformance_pack(
        &self,
        input: DeleteConformancePackRequest,
    ) -> Result<(), RusotoError<DeleteConformancePackError>> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StarlingDoveService.DeleteConformancePack");
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
            Err(DeleteConformancePackError::from_response(response))
        }
    }

    /// <p>Deletes the delivery channel.</p> <p>Before you can delete the delivery channel, you must stop the configuration recorder by using the <a>StopConfigurationRecorder</a> action.</p>
    async fn delete_delivery_channel(
        &self,
        input: DeleteDeliveryChannelRequest,
    ) -> Result<(), RusotoError<DeleteDeliveryChannelError>> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StarlingDoveService.DeleteDeliveryChannel");
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
            Err(DeleteDeliveryChannelError::from_response(response))
        }
    }

    /// <p>Deletes the evaluation results for the specified AWS Config rule. You can specify one AWS Config rule per request. After you delete the evaluation results, you can call the <a>StartConfigRulesEvaluation</a> API to start evaluating your AWS resources against the rule.</p>
    async fn delete_evaluation_results(
        &self,
        input: DeleteEvaluationResultsRequest,
    ) -> Result<DeleteEvaluationResultsResponse, RusotoError<DeleteEvaluationResultsError>> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DeleteEvaluationResults",
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
                .deserialize::<DeleteEvaluationResultsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteEvaluationResultsError::from_response(response))
        }
    }

    /// <p>Deletes the specified organization config rule and all of its evaluation results from all member accounts in that organization. Only a master account can delete an organization config rule.</p> <p>AWS Config sets the state of a rule to DELETE_IN_PROGRESS until the deletion is complete. You cannot update a rule while it is in this state.</p>
    async fn delete_organization_config_rule(
        &self,
        input: DeleteOrganizationConfigRuleRequest,
    ) -> Result<(), RusotoError<DeleteOrganizationConfigRuleError>> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DeleteOrganizationConfigRule",
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
            Err(DeleteOrganizationConfigRuleError::from_response(response))
        }
    }

    /// <p>Deletes the specified organization conformance pack and all of the config rules and remediation actions from all member accounts in that organization. Only a master account can delete an organization conformance pack.</p> <p>AWS Config sets the state of a conformance pack to DELETE_IN_PROGRESS until the deletion is complete. You cannot update a conformance pack while it is in this state. </p>
    async fn delete_organization_conformance_pack(
        &self,
        input: DeleteOrganizationConformancePackRequest,
    ) -> Result<(), RusotoError<DeleteOrganizationConformancePackError>> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DeleteOrganizationConformancePack",
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
            Err(DeleteOrganizationConformancePackError::from_response(
                response,
            ))
        }
    }

    /// <p>Deletes pending authorization requests for a specified aggregator account in a specified region.</p>
    async fn delete_pending_aggregation_request(
        &self,
        input: DeletePendingAggregationRequestRequest,
    ) -> Result<(), RusotoError<DeletePendingAggregationRequestError>> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DeletePendingAggregationRequest",
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
            Err(DeletePendingAggregationRequestError::from_response(
                response,
            ))
        }
    }

    /// <p>Deletes the remediation configuration.</p>
    async fn delete_remediation_configuration(
        &self,
        input: DeleteRemediationConfigurationRequest,
    ) -> Result<
        DeleteRemediationConfigurationResponse,
        RusotoError<DeleteRemediationConfigurationError>,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DeleteRemediationConfiguration",
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
                .deserialize::<DeleteRemediationConfigurationResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteRemediationConfigurationError::from_response(response))
        }
    }

    /// <p>Deletes one or more remediation exceptions mentioned in the resource keys.</p>
    async fn delete_remediation_exceptions(
        &self,
        input: DeleteRemediationExceptionsRequest,
    ) -> Result<DeleteRemediationExceptionsResponse, RusotoError<DeleteRemediationExceptionsError>>
    {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DeleteRemediationExceptions",
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
                .deserialize::<DeleteRemediationExceptionsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteRemediationExceptionsError::from_response(response))
        }
    }

    /// <p>Records the configuration state for a custom resource that has been deleted. This API records a new ConfigurationItem with a ResourceDeleted status. You can retrieve the ConfigurationItems recorded for this resource in your AWS Config History. </p>
    async fn delete_resource_config(
        &self,
        input: DeleteResourceConfigRequest,
    ) -> Result<(), RusotoError<DeleteResourceConfigError>> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StarlingDoveService.DeleteResourceConfig");
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
            Err(DeleteResourceConfigError::from_response(response))
        }
    }

    /// <p>Deletes the retention configuration.</p>
    async fn delete_retention_configuration(
        &self,
        input: DeleteRetentionConfigurationRequest,
    ) -> Result<(), RusotoError<DeleteRetentionConfigurationError>> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DeleteRetentionConfiguration",
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
            Err(DeleteRetentionConfigurationError::from_response(response))
        }
    }

    /// <p><p>Schedules delivery of a configuration snapshot to the Amazon S3 bucket in the specified delivery channel. After the delivery has started, AWS Config sends the following notifications using an Amazon SNS topic that you have specified.</p> <ul> <li> <p>Notification of the start of the delivery.</p> </li> <li> <p>Notification of the completion of the delivery, if the delivery was successfully completed.</p> </li> <li> <p>Notification of delivery failure, if the delivery failed.</p> </li> </ul></p>
    async fn deliver_config_snapshot(
        &self,
        input: DeliverConfigSnapshotRequest,
    ) -> Result<DeliverConfigSnapshotResponse, RusotoError<DeliverConfigSnapshotError>> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StarlingDoveService.DeliverConfigSnapshot");
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
                .deserialize::<DeliverConfigSnapshotResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeliverConfigSnapshotError::from_response(response))
        }
    }

    /// <p><p>Returns a list of compliant and noncompliant rules with the number of resources for compliant and noncompliant rules. </p> <note> <p>The results can return an empty result page, but if you have a <code>nextToken</code>, the results are displayed on the next page.</p> </note></p>
    async fn describe_aggregate_compliance_by_config_rules(
        &self,
        input: DescribeAggregateComplianceByConfigRulesRequest,
    ) -> Result<
        DescribeAggregateComplianceByConfigRulesResponse,
        RusotoError<DescribeAggregateComplianceByConfigRulesError>,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeAggregateComplianceByConfigRules",
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
                .deserialize::<DescribeAggregateComplianceByConfigRulesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeAggregateComplianceByConfigRulesError::from_response(response))
        }
    }

    /// <p>Returns a list of authorizations granted to various aggregator accounts and regions.</p>
    async fn describe_aggregation_authorizations(
        &self,
        input: DescribeAggregationAuthorizationsRequest,
    ) -> Result<
        DescribeAggregationAuthorizationsResponse,
        RusotoError<DescribeAggregationAuthorizationsError>,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeAggregationAuthorizations",
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
                .deserialize::<DescribeAggregationAuthorizationsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeAggregationAuthorizationsError::from_response(
                response,
            ))
        }
    }

    /// <p><p>Indicates whether the specified AWS Config rules are compliant. If a rule is noncompliant, this action returns the number of AWS resources that do not comply with the rule.</p> <p>A rule is compliant if all of the evaluated resources comply with it. It is noncompliant if any of these resources do not comply.</p> <p>If AWS Config has no current evaluation results for the rule, it returns <code>INSUFFICIENT<em>DATA</code>. This result might indicate one of the following conditions:</p> <ul> <li> <p>AWS Config has never invoked an evaluation for the rule. To check whether it has, use the <code>DescribeConfigRuleEvaluationStatus</code> action to get the <code>LastSuccessfulInvocationTime</code> and <code>LastFailedInvocationTime</code>.</p> </li> <li> <p>The rule&#39;s AWS Lambda function is failing to send evaluation results to AWS Config. Verify that the role you assigned to your configuration recorder includes the <code>config:PutEvaluations</code> permission. If the rule is a custom rule, verify that the AWS Lambda execution role includes the <code>config:PutEvaluations</code> permission.</p> </li> <li> <p>The rule&#39;s AWS Lambda function has returned <code>NOT</em>APPLICABLE</code> for all evaluation results. This can occur if the resources were deleted or removed from the rule&#39;s scope.</p> </li> </ul></p>
    async fn describe_compliance_by_config_rule(
        &self,
        input: DescribeComplianceByConfigRuleRequest,
    ) -> Result<
        DescribeComplianceByConfigRuleResponse,
        RusotoError<DescribeComplianceByConfigRuleError>,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeComplianceByConfigRule",
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
                .deserialize::<DescribeComplianceByConfigRuleResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeComplianceByConfigRuleError::from_response(response))
        }
    }

    /// <p><p>Indicates whether the specified AWS resources are compliant. If a resource is noncompliant, this action returns the number of AWS Config rules that the resource does not comply with.</p> <p>A resource is compliant if it complies with all the AWS Config rules that evaluate it. It is noncompliant if it does not comply with one or more of these rules.</p> <p>If AWS Config has no current evaluation results for the resource, it returns <code>INSUFFICIENT<em>DATA</code>. This result might indicate one of the following conditions about the rules that evaluate the resource:</p> <ul> <li> <p>AWS Config has never invoked an evaluation for the rule. To check whether it has, use the <code>DescribeConfigRuleEvaluationStatus</code> action to get the <code>LastSuccessfulInvocationTime</code> and <code>LastFailedInvocationTime</code>.</p> </li> <li> <p>The rule&#39;s AWS Lambda function is failing to send evaluation results to AWS Config. Verify that the role that you assigned to your configuration recorder includes the <code>config:PutEvaluations</code> permission. If the rule is a custom rule, verify that the AWS Lambda execution role includes the <code>config:PutEvaluations</code> permission.</p> </li> <li> <p>The rule&#39;s AWS Lambda function has returned <code>NOT</em>APPLICABLE</code> for all evaluation results. This can occur if the resources were deleted or removed from the rule&#39;s scope.</p> </li> </ul></p>
    async fn describe_compliance_by_resource(
        &self,
        input: DescribeComplianceByResourceRequest,
    ) -> Result<DescribeComplianceByResourceResponse, RusotoError<DescribeComplianceByResourceError>>
    {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeComplianceByResource",
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
                .deserialize::<DescribeComplianceByResourceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeComplianceByResourceError::from_response(response))
        }
    }

    /// <p>Returns status information for each of your AWS managed Config rules. The status includes information such as the last time AWS Config invoked the rule, the last time AWS Config failed to invoke the rule, and the related error for the last failure.</p>
    async fn describe_config_rule_evaluation_status(
        &self,
        input: DescribeConfigRuleEvaluationStatusRequest,
    ) -> Result<
        DescribeConfigRuleEvaluationStatusResponse,
        RusotoError<DescribeConfigRuleEvaluationStatusError>,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeConfigRuleEvaluationStatus",
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
                .deserialize::<DescribeConfigRuleEvaluationStatusResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeConfigRuleEvaluationStatusError::from_response(
                response,
            ))
        }
    }

    /// <p>Returns details about your AWS Config rules.</p>
    async fn describe_config_rules(
        &self,
        input: DescribeConfigRulesRequest,
    ) -> Result<DescribeConfigRulesResponse, RusotoError<DescribeConfigRulesError>> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StarlingDoveService.DescribeConfigRules");
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
                .deserialize::<DescribeConfigRulesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeConfigRulesError::from_response(response))
        }
    }

    /// <p>Returns status information for sources within an aggregator. The status includes information about the last time AWS Config verified authorization between the source account and an aggregator account. In case of a failure, the status contains the related error code or message. </p>
    async fn describe_configuration_aggregator_sources_status(
        &self,
        input: DescribeConfigurationAggregatorSourcesStatusRequest,
    ) -> Result<
        DescribeConfigurationAggregatorSourcesStatusResponse,
        RusotoError<DescribeConfigurationAggregatorSourcesStatusError>,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeConfigurationAggregatorSourcesStatus",
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
                .deserialize::<DescribeConfigurationAggregatorSourcesStatusResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeConfigurationAggregatorSourcesStatusError::from_response(response))
        }
    }

    /// <p>Returns the details of one or more configuration aggregators. If the configuration aggregator is not specified, this action returns the details for all the configuration aggregators associated with the account. </p>
    async fn describe_configuration_aggregators(
        &self,
        input: DescribeConfigurationAggregatorsRequest,
    ) -> Result<
        DescribeConfigurationAggregatorsResponse,
        RusotoError<DescribeConfigurationAggregatorsError>,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeConfigurationAggregators",
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
                .deserialize::<DescribeConfigurationAggregatorsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeConfigurationAggregatorsError::from_response(
                response,
            ))
        }
    }

    /// <p><p>Returns the current status of the specified configuration recorder. If a configuration recorder is not specified, this action returns the status of all configuration recorders associated with the account.</p> <note> <p>Currently, you can specify only one configuration recorder per region in your account.</p> </note></p>
    async fn describe_configuration_recorder_status(
        &self,
        input: DescribeConfigurationRecorderStatusRequest,
    ) -> Result<
        DescribeConfigurationRecorderStatusResponse,
        RusotoError<DescribeConfigurationRecorderStatusError>,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeConfigurationRecorderStatus",
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
                .deserialize::<DescribeConfigurationRecorderStatusResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeConfigurationRecorderStatusError::from_response(
                response,
            ))
        }
    }

    /// <p><p>Returns the details for the specified configuration recorders. If the configuration recorder is not specified, this action returns the details for all configuration recorders associated with the account.</p> <note> <p>Currently, you can specify only one configuration recorder per region in your account.</p> </note></p>
    async fn describe_configuration_recorders(
        &self,
        input: DescribeConfigurationRecordersRequest,
    ) -> Result<
        DescribeConfigurationRecordersResponse,
        RusotoError<DescribeConfigurationRecordersError>,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeConfigurationRecorders",
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
                .deserialize::<DescribeConfigurationRecordersResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeConfigurationRecordersError::from_response(response))
        }
    }

    /// <p><p>Returns compliance details for each rule in that conformance pack.</p> <note> <p>You must provide exact rule names.</p> </note></p>
    async fn describe_conformance_pack_compliance(
        &self,
        input: DescribeConformancePackComplianceRequest,
    ) -> Result<
        DescribeConformancePackComplianceResponse,
        RusotoError<DescribeConformancePackComplianceError>,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeConformancePackCompliance",
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
                .deserialize::<DescribeConformancePackComplianceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeConformancePackComplianceError::from_response(
                response,
            ))
        }
    }

    /// <p><p>Provides one or more conformance packs deployment status.</p> <note> <p>If there are no conformance packs then you will see an empty result.</p> </note></p>
    async fn describe_conformance_pack_status(
        &self,
        input: DescribeConformancePackStatusRequest,
    ) -> Result<
        DescribeConformancePackStatusResponse,
        RusotoError<DescribeConformancePackStatusError>,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeConformancePackStatus",
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
                .deserialize::<DescribeConformancePackStatusResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeConformancePackStatusError::from_response(response))
        }
    }

    /// <p>Returns a list of one or more conformance packs.</p>
    async fn describe_conformance_packs(
        &self,
        input: DescribeConformancePacksRequest,
    ) -> Result<DescribeConformancePacksResponse, RusotoError<DescribeConformancePacksError>> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeConformancePacks",
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
                .deserialize::<DescribeConformancePacksResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeConformancePacksError::from_response(response))
        }
    }

    /// <p><p>Returns the current status of the specified delivery channel. If a delivery channel is not specified, this action returns the current status of all delivery channels associated with the account.</p> <note> <p>Currently, you can specify only one delivery channel per region in your account.</p> </note></p>
    async fn describe_delivery_channel_status(
        &self,
        input: DescribeDeliveryChannelStatusRequest,
    ) -> Result<
        DescribeDeliveryChannelStatusResponse,
        RusotoError<DescribeDeliveryChannelStatusError>,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeDeliveryChannelStatus",
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
                .deserialize::<DescribeDeliveryChannelStatusResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeDeliveryChannelStatusError::from_response(response))
        }
    }

    /// <p><p>Returns details about the specified delivery channel. If a delivery channel is not specified, this action returns the details of all delivery channels associated with the account.</p> <note> <p>Currently, you can specify only one delivery channel per region in your account.</p> </note></p>
    async fn describe_delivery_channels(
        &self,
        input: DescribeDeliveryChannelsRequest,
    ) -> Result<DescribeDeliveryChannelsResponse, RusotoError<DescribeDeliveryChannelsError>> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeDeliveryChannels",
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
                .deserialize::<DescribeDeliveryChannelsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeDeliveryChannelsError::from_response(response))
        }
    }

    /// <p><p>Provides organization config rule deployment status for an organization.</p> <note> <p>The status is not considered successful until organization config rule is successfully deployed in all the member accounts with an exception of excluded accounts.</p> <p>When you specify the limit and the next token, you receive a paginated response. Limit and next token are not applicable if you specify organization config rule names. It is only applicable, when you request all the organization config rules.</p> <p>Only a master account can call this API.</p> </note></p>
    async fn describe_organization_config_rule_statuses(
        &self,
        input: DescribeOrganizationConfigRuleStatusesRequest,
    ) -> Result<
        DescribeOrganizationConfigRuleStatusesResponse,
        RusotoError<DescribeOrganizationConfigRuleStatusesError>,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeOrganizationConfigRuleStatuses",
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
                .deserialize::<DescribeOrganizationConfigRuleStatusesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeOrganizationConfigRuleStatusesError::from_response(
                response,
            ))
        }
    }

    /// <p><p>Returns a list of organization config rules.</p> <note> <p>When you specify the limit and the next token, you receive a paginated response. Limit and next token are not applicable if you specify organization config rule names. It is only applicable, when you request all the organization config rules.</p> <p>Only a master account can call this API.</p> </note></p>
    async fn describe_organization_config_rules(
        &self,
        input: DescribeOrganizationConfigRulesRequest,
    ) -> Result<
        DescribeOrganizationConfigRulesResponse,
        RusotoError<DescribeOrganizationConfigRulesError>,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeOrganizationConfigRules",
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
                .deserialize::<DescribeOrganizationConfigRulesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeOrganizationConfigRulesError::from_response(
                response,
            ))
        }
    }

    /// <p><p>Provides organization conformance pack deployment status for an organization.</p> <note> <p>The status is not considered successful until organization conformance pack is successfully deployed in all the member accounts with an exception of excluded accounts.</p> <p>When you specify the limit and the next token, you receive a paginated response. Limit and next token are not applicable if you specify organization conformance pack names. They are only applicable, when you request all the organization conformance packs.</p> <p>Only a master account can call this API.</p> </note></p>
    async fn describe_organization_conformance_pack_statuses(
        &self,
        input: DescribeOrganizationConformancePackStatusesRequest,
    ) -> Result<
        DescribeOrganizationConformancePackStatusesResponse,
        RusotoError<DescribeOrganizationConformancePackStatusesError>,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeOrganizationConformancePackStatuses",
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
                .deserialize::<DescribeOrganizationConformancePackStatusesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeOrganizationConformancePackStatusesError::from_response(response))
        }
    }

    /// <p><p>Returns a list of organization conformance packs.</p> <note> <p>When you specify the limit and the next token, you receive a paginated response. </p> <p>Limit and next token are not applicable if you specify organization conformance packs names. They are only applicable, when you request all the organization conformance packs. </p> <p>Only a master account can call this API.</p> </note></p>
    async fn describe_organization_conformance_packs(
        &self,
        input: DescribeOrganizationConformancePacksRequest,
    ) -> Result<
        DescribeOrganizationConformancePacksResponse,
        RusotoError<DescribeOrganizationConformancePacksError>,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeOrganizationConformancePacks",
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
                .deserialize::<DescribeOrganizationConformancePacksResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeOrganizationConformancePacksError::from_response(
                response,
            ))
        }
    }

    /// <p>Returns a list of all pending aggregation requests.</p>
    async fn describe_pending_aggregation_requests(
        &self,
        input: DescribePendingAggregationRequestsRequest,
    ) -> Result<
        DescribePendingAggregationRequestsResponse,
        RusotoError<DescribePendingAggregationRequestsError>,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribePendingAggregationRequests",
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
                .deserialize::<DescribePendingAggregationRequestsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribePendingAggregationRequestsError::from_response(
                response,
            ))
        }
    }

    /// <p>Returns the details of one or more remediation configurations.</p>
    async fn describe_remediation_configurations(
        &self,
        input: DescribeRemediationConfigurationsRequest,
    ) -> Result<
        DescribeRemediationConfigurationsResponse,
        RusotoError<DescribeRemediationConfigurationsError>,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeRemediationConfigurations",
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
                .deserialize::<DescribeRemediationConfigurationsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeRemediationConfigurationsError::from_response(
                response,
            ))
        }
    }

    /// <p><p>Returns the details of one or more remediation exceptions. A detailed view of a remediation exception for a set of resources that includes an explanation of an exception and the time when the exception will be deleted. When you specify the limit and the next token, you receive a paginated response. </p> <note> <p>When you specify the limit and the next token, you receive a paginated response. </p> <p>Limit and next token are not applicable if you request resources in batch. It is only applicable, when you request all resources.</p> </note></p>
    async fn describe_remediation_exceptions(
        &self,
        input: DescribeRemediationExceptionsRequest,
    ) -> Result<
        DescribeRemediationExceptionsResponse,
        RusotoError<DescribeRemediationExceptionsError>,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeRemediationExceptions",
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
                .deserialize::<DescribeRemediationExceptionsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeRemediationExceptionsError::from_response(response))
        }
    }

    /// <p>Provides a detailed view of a Remediation Execution for a set of resources including state, timestamps for when steps for the remediation execution occur, and any error messages for steps that have failed. When you specify the limit and the next token, you receive a paginated response.</p>
    async fn describe_remediation_execution_status(
        &self,
        input: DescribeRemediationExecutionStatusRequest,
    ) -> Result<
        DescribeRemediationExecutionStatusResponse,
        RusotoError<DescribeRemediationExecutionStatusError>,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeRemediationExecutionStatus",
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
                .deserialize::<DescribeRemediationExecutionStatusResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeRemediationExecutionStatusError::from_response(
                response,
            ))
        }
    }

    /// <p><p>Returns the details of one or more retention configurations. If the retention configuration name is not specified, this action returns the details for all the retention configurations for that account.</p> <note> <p>Currently, AWS Config supports only one retention configuration per region in your account.</p> </note></p>
    async fn describe_retention_configurations(
        &self,
        input: DescribeRetentionConfigurationsRequest,
    ) -> Result<
        DescribeRetentionConfigurationsResponse,
        RusotoError<DescribeRetentionConfigurationsError>,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeRetentionConfigurations",
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
                .deserialize::<DescribeRetentionConfigurationsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeRetentionConfigurationsError::from_response(
                response,
            ))
        }
    }

    /// <p><p>Returns the evaluation results for the specified AWS Config rule for a specific resource in a rule. The results indicate which AWS resources were evaluated by the rule, when each resource was last evaluated, and whether each resource complies with the rule. </p> <note> <p>The results can return an empty result page. But if you have a <code>nextToken</code>, the results are displayed on the next page.</p> </note></p>
    async fn get_aggregate_compliance_details_by_config_rule(
        &self,
        input: GetAggregateComplianceDetailsByConfigRuleRequest,
    ) -> Result<
        GetAggregateComplianceDetailsByConfigRuleResponse,
        RusotoError<GetAggregateComplianceDetailsByConfigRuleError>,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.GetAggregateComplianceDetailsByConfigRule",
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
                .deserialize::<GetAggregateComplianceDetailsByConfigRuleResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetAggregateComplianceDetailsByConfigRuleError::from_response(response))
        }
    }

    /// <p><p>Returns the number of compliant and noncompliant rules for one or more accounts and regions in an aggregator.</p> <note> <p>The results can return an empty result page, but if you have a nextToken, the results are displayed on the next page.</p> </note></p>
    async fn get_aggregate_config_rule_compliance_summary(
        &self,
        input: GetAggregateConfigRuleComplianceSummaryRequest,
    ) -> Result<
        GetAggregateConfigRuleComplianceSummaryResponse,
        RusotoError<GetAggregateConfigRuleComplianceSummaryError>,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.GetAggregateConfigRuleComplianceSummary",
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
                .deserialize::<GetAggregateConfigRuleComplianceSummaryResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetAggregateConfigRuleComplianceSummaryError::from_response(
                response,
            ))
        }
    }

    /// <p>Returns the resource counts across accounts and regions that are present in your AWS Config aggregator. You can request the resource counts by providing filters and GroupByKey.</p> <p>For example, if the input contains accountID 12345678910 and region us-east-1 in filters, the API returns the count of resources in account ID 12345678910 and region us-east-1. If the input contains ACCOUNT_ID as a GroupByKey, the API returns resource counts for all source accounts that are present in your aggregator.</p>
    async fn get_aggregate_discovered_resource_counts(
        &self,
        input: GetAggregateDiscoveredResourceCountsRequest,
    ) -> Result<
        GetAggregateDiscoveredResourceCountsResponse,
        RusotoError<GetAggregateDiscoveredResourceCountsError>,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.GetAggregateDiscoveredResourceCounts",
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
                .deserialize::<GetAggregateDiscoveredResourceCountsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetAggregateDiscoveredResourceCountsError::from_response(
                response,
            ))
        }
    }

    /// <p>Returns configuration item that is aggregated for your specific resource in a specific source account and region.</p>
    async fn get_aggregate_resource_config(
        &self,
        input: GetAggregateResourceConfigRequest,
    ) -> Result<GetAggregateResourceConfigResponse, RusotoError<GetAggregateResourceConfigError>>
    {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.GetAggregateResourceConfig",
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
                .deserialize::<GetAggregateResourceConfigResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetAggregateResourceConfigError::from_response(response))
        }
    }

    /// <p>Returns the evaluation results for the specified AWS Config rule. The results indicate which AWS resources were evaluated by the rule, when each resource was last evaluated, and whether each resource complies with the rule.</p>
    async fn get_compliance_details_by_config_rule(
        &self,
        input: GetComplianceDetailsByConfigRuleRequest,
    ) -> Result<
        GetComplianceDetailsByConfigRuleResponse,
        RusotoError<GetComplianceDetailsByConfigRuleError>,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.GetComplianceDetailsByConfigRule",
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
                .deserialize::<GetComplianceDetailsByConfigRuleResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetComplianceDetailsByConfigRuleError::from_response(
                response,
            ))
        }
    }

    /// <p>Returns the evaluation results for the specified AWS resource. The results indicate which AWS Config rules were used to evaluate the resource, when each rule was last used, and whether the resource complies with each rule.</p>
    async fn get_compliance_details_by_resource(
        &self,
        input: GetComplianceDetailsByResourceRequest,
    ) -> Result<
        GetComplianceDetailsByResourceResponse,
        RusotoError<GetComplianceDetailsByResourceError>,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.GetComplianceDetailsByResource",
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
                .deserialize::<GetComplianceDetailsByResourceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetComplianceDetailsByResourceError::from_response(response))
        }
    }

    /// <p>Returns the number of AWS Config rules that are compliant and noncompliant, up to a maximum of 25 for each.</p>
    async fn get_compliance_summary_by_config_rule(
        &self,
    ) -> Result<
        GetComplianceSummaryByConfigRuleResponse,
        RusotoError<GetComplianceSummaryByConfigRuleError>,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.GetComplianceSummaryByConfigRule",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetComplianceSummaryByConfigRuleResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetComplianceSummaryByConfigRuleError::from_response(
                response,
            ))
        }
    }

    /// <p>Returns the number of resources that are compliant and the number that are noncompliant. You can specify one or more resource types to get these numbers for each resource type. The maximum number returned is 100.</p>
    async fn get_compliance_summary_by_resource_type(
        &self,
        input: GetComplianceSummaryByResourceTypeRequest,
    ) -> Result<
        GetComplianceSummaryByResourceTypeResponse,
        RusotoError<GetComplianceSummaryByResourceTypeError>,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.GetComplianceSummaryByResourceType",
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
                .deserialize::<GetComplianceSummaryByResourceTypeResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetComplianceSummaryByResourceTypeError::from_response(
                response,
            ))
        }
    }

    /// <p>Returns compliance details of a conformance pack for all AWS resources that are monitered by conformance pack.</p>
    async fn get_conformance_pack_compliance_details(
        &self,
        input: GetConformancePackComplianceDetailsRequest,
    ) -> Result<
        GetConformancePackComplianceDetailsResponse,
        RusotoError<GetConformancePackComplianceDetailsError>,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.GetConformancePackComplianceDetails",
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
                .deserialize::<GetConformancePackComplianceDetailsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetConformancePackComplianceDetailsError::from_response(
                response,
            ))
        }
    }

    /// <p>Returns compliance details for the conformance pack based on the cumulative compliance results of all the rules in that conformance pack.</p>
    async fn get_conformance_pack_compliance_summary(
        &self,
        input: GetConformancePackComplianceSummaryRequest,
    ) -> Result<
        GetConformancePackComplianceSummaryResponse,
        RusotoError<GetConformancePackComplianceSummaryError>,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.GetConformancePackComplianceSummary",
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
                .deserialize::<GetConformancePackComplianceSummaryResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetConformancePackComplianceSummaryError::from_response(
                response,
            ))
        }
    }

    /// <p><p>Returns the resource types, the number of each resource type, and the total number of resources that AWS Config is recording in this region for your AWS account. </p> <p class="title"> <b>Example</b> </p> <ol> <li> <p>AWS Config is recording three resource types in the US East (Ohio) Region for your account: 25 EC2 instances, 20 IAM users, and 15 S3 buckets.</p> </li> <li> <p>You make a call to the <code>GetDiscoveredResourceCounts</code> action and specify that you want all resource types. </p> </li> <li> <p>AWS Config returns the following:</p> <ul> <li> <p>The resource types (EC2 instances, IAM users, and S3 buckets).</p> </li> <li> <p>The number of each resource type (25, 20, and 15).</p> </li> <li> <p>The total number of all resources (60).</p> </li> </ul> </li> </ol> <p>The response is paginated. By default, AWS Config lists 100 <a>ResourceCount</a> objects on each page. You can customize this number with the <code>limit</code> parameter. The response includes a <code>nextToken</code> string. To get the next page of results, run the request again and specify the string for the <code>nextToken</code> parameter.</p> <note> <p>If you make a call to the <a>GetDiscoveredResourceCounts</a> action, you might not immediately receive resource counts in the following situations:</p> <ul> <li> <p>You are a new AWS Config customer.</p> </li> <li> <p>You just enabled resource recording.</p> </li> </ul> <p>It might take a few minutes for AWS Config to record and count your resources. Wait a few minutes and then retry the <a>GetDiscoveredResourceCounts</a> action. </p> </note></p>
    async fn get_discovered_resource_counts(
        &self,
        input: GetDiscoveredResourceCountsRequest,
    ) -> Result<GetDiscoveredResourceCountsResponse, RusotoError<GetDiscoveredResourceCountsError>>
    {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.GetDiscoveredResourceCounts",
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
                .deserialize::<GetDiscoveredResourceCountsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetDiscoveredResourceCountsError::from_response(response))
        }
    }

    /// <p><p>Returns detailed status for each member account within an organization for a given organization config rule.</p> <note> <p>Only a master account can call this API.</p> </note></p>
    async fn get_organization_config_rule_detailed_status(
        &self,
        input: GetOrganizationConfigRuleDetailedStatusRequest,
    ) -> Result<
        GetOrganizationConfigRuleDetailedStatusResponse,
        RusotoError<GetOrganizationConfigRuleDetailedStatusError>,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.GetOrganizationConfigRuleDetailedStatus",
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
                .deserialize::<GetOrganizationConfigRuleDetailedStatusResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetOrganizationConfigRuleDetailedStatusError::from_response(
                response,
            ))
        }
    }

    /// <p>Returns detailed status for each member account within an organization for a given organization conformance pack.</p> <p>Only a master account can call this API.</p>
    async fn get_organization_conformance_pack_detailed_status(
        &self,
        input: GetOrganizationConformancePackDetailedStatusRequest,
    ) -> Result<
        GetOrganizationConformancePackDetailedStatusResponse,
        RusotoError<GetOrganizationConformancePackDetailedStatusError>,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.GetOrganizationConformancePackDetailedStatus",
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
                .deserialize::<GetOrganizationConformancePackDetailedStatusResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetOrganizationConformancePackDetailedStatusError::from_response(response))
        }
    }

    /// <p><p>Returns a list of configuration items for the specified resource. The list contains details about each state of the resource during the specified time interval. If you specified a retention period to retain your <code>ConfigurationItems</code> between a minimum of 30 days and a maximum of 7 years (2557 days), AWS Config returns the <code>ConfigurationItems</code> for the specified retention period. </p> <p>The response is paginated. By default, AWS Config returns a limit of 10 configuration items per page. You can customize this number with the <code>limit</code> parameter. The response includes a <code>nextToken</code> string. To get the next page of results, run the request again and specify the string for the <code>nextToken</code> parameter.</p> <note> <p>Each call to the API is limited to span a duration of seven days. It is likely that the number of records returned is smaller than the specified <code>limit</code>. In such cases, you can make another call, using the <code>nextToken</code>.</p> </note></p>
    async fn get_resource_config_history(
        &self,
        input: GetResourceConfigHistoryRequest,
    ) -> Result<GetResourceConfigHistoryResponse, RusotoError<GetResourceConfigHistoryError>> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.GetResourceConfigHistory",
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
                .deserialize::<GetResourceConfigHistoryResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetResourceConfigHistoryError::from_response(response))
        }
    }

    /// <p>Accepts a resource type and returns a list of resource identifiers that are aggregated for a specific resource type across accounts and regions. A resource identifier includes the resource type, ID, (if available) the custom resource name, source account, and source region. You can narrow the results to include only resources that have specific resource IDs, or a resource name, or source account ID, or source region.</p> <p>For example, if the input consists of accountID 12345678910 and the region is us-east-1 for resource type <code>AWS::EC2::Instance</code> then the API returns all the EC2 instance identifiers of accountID 12345678910 and region us-east-1.</p>
    async fn list_aggregate_discovered_resources(
        &self,
        input: ListAggregateDiscoveredResourcesRequest,
    ) -> Result<
        ListAggregateDiscoveredResourcesResponse,
        RusotoError<ListAggregateDiscoveredResourcesError>,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.ListAggregateDiscoveredResources",
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
                .deserialize::<ListAggregateDiscoveredResourcesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListAggregateDiscoveredResourcesError::from_response(
                response,
            ))
        }
    }

    /// <p>Accepts a resource type and returns a list of resource identifiers for the resources of that type. A resource identifier includes the resource type, ID, and (if available) the custom resource name. The results consist of resources that AWS Config has discovered, including those that AWS Config is not currently recording. You can narrow the results to include only resources that have specific resource IDs or a resource name.</p> <note> <p>You can specify either resource IDs or a resource name, but not both, in the same request.</p> </note> <p>The response is paginated. By default, AWS Config lists 100 resource identifiers on each page. You can customize this number with the <code>limit</code> parameter. The response includes a <code>nextToken</code> string. To get the next page of results, run the request again and specify the string for the <code>nextToken</code> parameter.</p>
    async fn list_discovered_resources(
        &self,
        input: ListDiscoveredResourcesRequest,
    ) -> Result<ListDiscoveredResourcesResponse, RusotoError<ListDiscoveredResourcesError>> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.ListDiscoveredResources",
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
                .deserialize::<ListDiscoveredResourcesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListDiscoveredResourcesError::from_response(response))
        }
    }

    /// <p>List the tags for AWS Config resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StarlingDoveService.ListTagsForResource");
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
                .deserialize::<ListTagsForResourceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p>Authorizes the aggregator account and region to collect data from the source account and region. </p>
    async fn put_aggregation_authorization(
        &self,
        input: PutAggregationAuthorizationRequest,
    ) -> Result<PutAggregationAuthorizationResponse, RusotoError<PutAggregationAuthorizationError>>
    {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.PutAggregationAuthorization",
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
                .deserialize::<PutAggregationAuthorizationResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutAggregationAuthorizationError::from_response(response))
        }
    }

    /// <p>Adds or updates an AWS Config rule for evaluating whether your AWS resources comply with your desired configurations.</p> <p>You can use this action for custom AWS Config rules and AWS managed Config rules. A custom AWS Config rule is a rule that you develop and maintain. An AWS managed Config rule is a customizable, predefined rule that AWS Config provides.</p> <p>If you are adding a new custom AWS Config rule, you must first create the AWS Lambda function that the rule invokes to evaluate your resources. When you use the <code>PutConfigRule</code> action to add the rule to AWS Config, you must specify the Amazon Resource Name (ARN) that AWS Lambda assigns to the function. Specify the ARN for the <code>SourceIdentifier</code> key. This key is part of the <code>Source</code> object, which is part of the <code>ConfigRule</code> object. </p> <p>If you are adding an AWS managed Config rule, specify the rule's identifier for the <code>SourceIdentifier</code> key. To reference AWS managed Config rule identifiers, see <a href="https://docs.aws.amazon.com/config/latest/developerguide/evaluate-config_use-managed-rules.html">About AWS Managed Config Rules</a>.</p> <p>For any new rule that you add, specify the <code>ConfigRuleName</code> in the <code>ConfigRule</code> object. Do not specify the <code>ConfigRuleArn</code> or the <code>ConfigRuleId</code>. These values are generated by AWS Config for new rules.</p> <p>If you are updating a rule that you added previously, you can specify the rule by <code>ConfigRuleName</code>, <code>ConfigRuleId</code>, or <code>ConfigRuleArn</code> in the <code>ConfigRule</code> data type that you use in this request.</p> <p>The maximum number of rules that AWS Config supports is 150.</p> <p>For information about requesting a rule limit increase, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_config">AWS Config Limits</a> in the <i>AWS General Reference Guide</i>.</p> <p>For more information about developing and using AWS Config rules, see <a href="https://docs.aws.amazon.com/config/latest/developerguide/evaluate-config.html">Evaluating AWS Resource Configurations with AWS Config</a> in the <i>AWS Config Developer Guide</i>.</p>
    async fn put_config_rule(
        &self,
        input: PutConfigRuleRequest,
    ) -> Result<(), RusotoError<PutConfigRuleError>> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StarlingDoveService.PutConfigRule");
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
            Err(PutConfigRuleError::from_response(response))
        }
    }

    /// <p><p>Creates and updates the configuration aggregator with the selected source accounts and regions. The source account can be individual account(s) or an organization.</p> <note> <p>AWS Config should be enabled in source accounts and regions you want to aggregate.</p> <p>If your source type is an organization, you must be signed in to the master account and all features must be enabled in your organization. AWS Config calls <code>EnableAwsServiceAccess</code> API to enable integration between AWS Config and AWS Organizations. </p> </note></p>
    async fn put_configuration_aggregator(
        &self,
        input: PutConfigurationAggregatorRequest,
    ) -> Result<PutConfigurationAggregatorResponse, RusotoError<PutConfigurationAggregatorError>>
    {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.PutConfigurationAggregator",
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
                .deserialize::<PutConfigurationAggregatorResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutConfigurationAggregatorError::from_response(response))
        }
    }

    /// <p><p>Creates a new configuration recorder to record the selected resource configurations.</p> <p>You can use this action to change the role <code>roleARN</code> or the <code>recordingGroup</code> of an existing recorder. To change the role, call the action on the existing configuration recorder and specify a role.</p> <note> <p>Currently, you can specify only one configuration recorder per region in your account.</p> <p>If <code>ConfigurationRecorder</code> does not have the <b>recordingGroup</b> parameter specified, the default is to record all supported resource types.</p> </note></p>
    async fn put_configuration_recorder(
        &self,
        input: PutConfigurationRecorderRequest,
    ) -> Result<(), RusotoError<PutConfigurationRecorderError>> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.PutConfigurationRecorder",
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
            Err(PutConfigurationRecorderError::from_response(response))
        }
    }

    /// <p><p>Creates or updates a conformance pack. A conformance pack is a collection of AWS Config rules that can be easily deployed in an account and a region and across AWS Organization.</p> <p>This API creates a service linked role <code>AWSServiceRoleForConfigConforms</code> in your account. The service linked role is created only when the role does not exist in your account. AWS Config verifies the existence of role with <code>GetRole</code> action.</p> <note> <p>You must specify either the <code>TemplateS3Uri</code> or the <code>TemplateBody</code> parameter, but not both. If you provide both AWS Config uses the <code>TemplateS3Uri</code> parameter and ignores the <code>TemplateBody</code> parameter.</p> </note></p>
    async fn put_conformance_pack(
        &self,
        input: PutConformancePackRequest,
    ) -> Result<PutConformancePackResponse, RusotoError<PutConformancePackError>> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StarlingDoveService.PutConformancePack");
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
                .deserialize::<PutConformancePackResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutConformancePackError::from_response(response))
        }
    }

    /// <p><p>Creates a delivery channel object to deliver configuration information to an Amazon S3 bucket and Amazon SNS topic.</p> <p>Before you can create a delivery channel, you must create a configuration recorder.</p> <p>You can use this action to change the Amazon S3 bucket or an Amazon SNS topic of the existing delivery channel. To change the Amazon S3 bucket or an Amazon SNS topic, call this action and specify the changed values for the S3 bucket and the SNS topic. If you specify a different value for either the S3 bucket or the SNS topic, this action will keep the existing value for the parameter that is not changed.</p> <note> <p>You can have only one delivery channel per region in your account.</p> </note></p>
    async fn put_delivery_channel(
        &self,
        input: PutDeliveryChannelRequest,
    ) -> Result<(), RusotoError<PutDeliveryChannelError>> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StarlingDoveService.PutDeliveryChannel");
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
            Err(PutDeliveryChannelError::from_response(response))
        }
    }

    /// <p>Used by an AWS Lambda function to deliver evaluation results to AWS Config. This action is required in every AWS Lambda function that is invoked by an AWS Config rule.</p>
    async fn put_evaluations(
        &self,
        input: PutEvaluationsRequest,
    ) -> Result<PutEvaluationsResponse, RusotoError<PutEvaluationsError>> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StarlingDoveService.PutEvaluations");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<PutEvaluationsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutEvaluationsError::from_response(response))
        }
    }

    /// <p><p>Adds or updates organization config rule for your entire organization evaluating whether your AWS resources comply with your desired configurations. Only a master account can create or update an organization config rule.</p> <p>This API enables organization service access through the <code>EnableAWSServiceAccess</code> action and creates a service linked role <code>AWSServiceRoleForConfigMultiAccountSetup</code> in the master account of your organization. The service linked role is created only when the role does not exist in the master account. AWS Config verifies the existence of role with <code>GetRole</code> action.</p> <p>You can use this action to create both custom AWS Config rules and AWS managed Config rules. If you are adding a new custom AWS Config rule, you must first create AWS Lambda function in the master account that the rule invokes to evaluate your resources. When you use the <code>PutOrganizationConfigRule</code> action to add the rule to AWS Config, you must specify the Amazon Resource Name (ARN) that AWS Lambda assigns to the function. If you are adding an AWS managed Config rule, specify the rule&#39;s identifier for the <code>RuleIdentifier</code> key.</p> <p>The maximum number of organization config rules that AWS Config supports is 150.</p> <note> <p>Specify either <code>OrganizationCustomRuleMetadata</code> or <code>OrganizationManagedRuleMetadata</code>.</p> </note></p>
    async fn put_organization_config_rule(
        &self,
        input: PutOrganizationConfigRuleRequest,
    ) -> Result<PutOrganizationConfigRuleResponse, RusotoError<PutOrganizationConfigRuleError>>
    {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.PutOrganizationConfigRule",
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
                .deserialize::<PutOrganizationConfigRuleResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutOrganizationConfigRuleError::from_response(response))
        }
    }

    /// <p><p>Deploys conformance packs across member accounts in an AWS Organization.</p> <p>This API enables organization service access for <code>config-multiaccountsetup.amazonaws.com</code> through the <code>EnableAWSServiceAccess</code> action and creates a service linked role <code>AWSServiceRoleForConfigMultiAccountSetup</code> in the master account of your organization. The service linked role is created only when the role does not exist in the master account. AWS Config verifies the existence of role with GetRole action.</p> <note> <p>You must specify either the <code>TemplateS3Uri</code> or the <code>TemplateBody</code> parameter, but not both. If you provide both AWS Config uses the <code>TemplateS3Uri</code> parameter and ignores the <code>TemplateBody</code> parameter.</p> <p>AWS Config sets the state of a conformance pack to CREATE<em>IN</em>PROGRESS and UPDATE<em>IN</em>PROGRESS until the confomance pack is created or updated. You cannot update a conformance pack while it is in this state.</p> <p>You can create 6 conformance packs with 25 AWS Config rules in each pack.</p> </note></p>
    async fn put_organization_conformance_pack(
        &self,
        input: PutOrganizationConformancePackRequest,
    ) -> Result<
        PutOrganizationConformancePackResponse,
        RusotoError<PutOrganizationConformancePackError>,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.PutOrganizationConformancePack",
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
                .deserialize::<PutOrganizationConformancePackResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutOrganizationConformancePackError::from_response(response))
        }
    }

    /// <p>Adds or updates the remediation configuration with a specific AWS Config rule with the selected target or action. The API creates the <code>RemediationConfiguration</code> object for the AWS Config rule. The AWS Config rule must already exist for you to add a remediation configuration. The target (SSM document) must exist and have permissions to use the target. </p>
    async fn put_remediation_configurations(
        &self,
        input: PutRemediationConfigurationsRequest,
    ) -> Result<PutRemediationConfigurationsResponse, RusotoError<PutRemediationConfigurationsError>>
    {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.PutRemediationConfigurations",
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
                .deserialize::<PutRemediationConfigurationsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutRemediationConfigurationsError::from_response(response))
        }
    }

    /// <p>A remediation exception is when a specific resource is no longer considered for auto-remediation. This API adds a new exception or updates an exisiting exception for a specific resource with a specific AWS Config rule. </p>
    async fn put_remediation_exceptions(
        &self,
        input: PutRemediationExceptionsRequest,
    ) -> Result<PutRemediationExceptionsResponse, RusotoError<PutRemediationExceptionsError>> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.PutRemediationExceptions",
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
                .deserialize::<PutRemediationExceptionsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutRemediationExceptionsError::from_response(response))
        }
    }

    /// <p><p>Records the configuration state for the resource provided in the request. The configuration state of a resource is represented in AWS Config as Configuration Items. Once this API records the configuration item, you can retrieve the list of configuration items for the custom resource type using existing AWS Config APIs. </p> <note> <p>The custom resource type must be registered with AWS CloudFormation. This API accepts the configuration item registered with AWS CloudFormation.</p> <p>When you call this API, AWS Config only stores configuration state of the resource provided in the request. This API does not change or remediate the configuration of the resource. </p> </note></p>
    async fn put_resource_config(
        &self,
        input: PutResourceConfigRequest,
    ) -> Result<(), RusotoError<PutResourceConfigError>> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StarlingDoveService.PutResourceConfig");
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
            Err(PutResourceConfigError::from_response(response))
        }
    }

    /// <p><p>Creates and updates the retention configuration with details about retention period (number of days) that AWS Config stores your historical information. The API creates the <code>RetentionConfiguration</code> object and names the object as <b>default</b>. When you have a <code>RetentionConfiguration</code> object named <b>default</b>, calling the API modifies the default object. </p> <note> <p>Currently, AWS Config supports only one retention configuration per region in your account.</p> </note></p>
    async fn put_retention_configuration(
        &self,
        input: PutRetentionConfigurationRequest,
    ) -> Result<PutRetentionConfigurationResponse, RusotoError<PutRetentionConfigurationError>>
    {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.PutRetentionConfiguration",
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
                .deserialize::<PutRetentionConfigurationResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutRetentionConfigurationError::from_response(response))
        }
    }

    /// <p>Accepts a structured query language (SQL) <code>SELECT</code> command, performs the corresponding search, and returns resource configurations matching the properties.</p> <p>For more information about query components, see the <a href="https://docs.aws.amazon.com/config/latest/developerguide/query-components.html"> <b>Query Components</b> </a> section in the AWS Config Developer Guide.</p>
    async fn select_resource_config(
        &self,
        input: SelectResourceConfigRequest,
    ) -> Result<SelectResourceConfigResponse, RusotoError<SelectResourceConfigError>> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StarlingDoveService.SelectResourceConfig");
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
                .deserialize::<SelectResourceConfigResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(SelectResourceConfigError::from_response(response))
        }
    }

    /// <p><p>Runs an on-demand evaluation for the specified AWS Config rules against the last known configuration state of the resources. Use <code>StartConfigRulesEvaluation</code> when you want to test that a rule you updated is working as expected. <code>StartConfigRulesEvaluation</code> does not re-record the latest configuration state for your resources. It re-runs an evaluation against the last known state of your resources. </p> <p>You can specify up to 25 AWS Config rules per request. </p> <p>An existing <code>StartConfigRulesEvaluation</code> call for the specified rules must complete before you can call the API again. If you chose to have AWS Config stream to an Amazon SNS topic, you will receive a <code>ConfigRuleEvaluationStarted</code> notification when the evaluation starts.</p> <note> <p>You don&#39;t need to call the <code>StartConfigRulesEvaluation</code> API to run an evaluation for a new rule. When you create a rule, AWS Config evaluates your resources against the rule automatically. </p> </note> <p>The <code>StartConfigRulesEvaluation</code> API is useful if you want to run on-demand evaluations, such as the following example:</p> <ol> <li> <p>You have a custom rule that evaluates your IAM resources every 24 hours.</p> </li> <li> <p>You update your Lambda function to add additional conditions to your rule.</p> </li> <li> <p>Instead of waiting for the next periodic evaluation, you call the <code>StartConfigRulesEvaluation</code> API.</p> </li> <li> <p>AWS Config invokes your Lambda function and evaluates your IAM resources.</p> </li> <li> <p>Your custom rule will still run periodic evaluations every 24 hours.</p> </li> </ol></p>
    async fn start_config_rules_evaluation(
        &self,
        input: StartConfigRulesEvaluationRequest,
    ) -> Result<StartConfigRulesEvaluationResponse, RusotoError<StartConfigRulesEvaluationError>>
    {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.StartConfigRulesEvaluation",
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
                .deserialize::<StartConfigRulesEvaluationResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StartConfigRulesEvaluationError::from_response(response))
        }
    }

    /// <p>Starts recording configurations of the AWS resources you have selected to record in your AWS account.</p> <p>You must have created at least one delivery channel to successfully start the configuration recorder.</p>
    async fn start_configuration_recorder(
        &self,
        input: StartConfigurationRecorderRequest,
    ) -> Result<(), RusotoError<StartConfigurationRecorderError>> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.StartConfigurationRecorder",
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
            Err(StartConfigurationRecorderError::from_response(response))
        }
    }

    /// <p>Runs an on-demand remediation for the specified AWS Config rules against the last known remediation configuration. It runs an execution against the current state of your resources. Remediation execution is asynchronous.</p> <p>You can specify up to 100 resource keys per request. An existing StartRemediationExecution call for the specified resource keys must complete before you can call the API again.</p>
    async fn start_remediation_execution(
        &self,
        input: StartRemediationExecutionRequest,
    ) -> Result<StartRemediationExecutionResponse, RusotoError<StartRemediationExecutionError>>
    {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.StartRemediationExecution",
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
                .deserialize::<StartRemediationExecutionResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StartRemediationExecutionError::from_response(response))
        }
    }

    /// <p>Stops recording configurations of the AWS resources you have selected to record in your AWS account.</p>
    async fn stop_configuration_recorder(
        &self,
        input: StopConfigurationRecorderRequest,
    ) -> Result<(), RusotoError<StopConfigurationRecorderError>> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.StopConfigurationRecorder",
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
            Err(StopConfigurationRecorderError::from_response(response))
        }
    }

    /// <p>Associates the specified tags to a resource with the specified resourceArn. If existing tags on a resource are not specified in the request parameters, they are not changed. When a resource is deleted, the tags associated with that resource are deleted as well.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<(), RusotoError<TagResourceError>> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StarlingDoveService.TagResource");
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

    /// <p>Deletes specified tags from a resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<(), RusotoError<UntagResourceError>> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StarlingDoveService.UntagResource");
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
