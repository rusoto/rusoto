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
/// <p>Describes the configuration of an Auto Scaling group.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AutoScalingGroupConfiguration {
    /// <p>The desired capacity, or number of instances, for the Auto Scaling group.</p>
    #[serde(rename = "desiredCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_capacity: Option<i64>,
    /// <p>The instance type for the Auto Scaling group.</p>
    #[serde(rename = "instanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p>The maximum size, or maximum number of instances, for the Auto Scaling group.</p>
    #[serde(rename = "maxSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size: Option<i64>,
    /// <p>The minimum size, or minimum number of instances, for the Auto Scaling group.</p>
    #[serde(rename = "minSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_size: Option<i64>,
}

/// <p>Describes an Auto Scaling group recommendation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AutoScalingGroupRecommendation {
    /// <p>The AWS account ID of the Auto Scaling group.</p>
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the Auto Scaling group.</p>
    #[serde(rename = "autoScalingGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_group_arn: Option<String>,
    /// <p>The name of the Auto Scaling group.</p>
    #[serde(rename = "autoScalingGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_group_name: Option<String>,
    /// <p>An array of objects that describe the current configuration of the Auto Scaling group.</p>
    #[serde(rename = "currentConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_configuration: Option<AutoScalingGroupConfiguration>,
    /// <p><p>The finding classification for the Auto Scaling group.</p> <p>Findings for Auto Scaling groups include:</p> <ul> <li> <p> <b> <code>NotOptimized</code> </b>—An Auto Scaling group is considered not optimized when AWS Compute Optimizer identifies a recommendation that can provide better performance for your workload.</p> </li> <li> <p> <b> <code>Optimized</code> </b>—An Auto Scaling group is considered optimized when Compute Optimizer determines that the group is correctly provisioned to run your workload based on the chosen instance type. For optimized resources, Compute Optimizer might recommend a new generation instance type.</p> </li> </ul> <note> <p>The values that are returned might be <code>NOT_OPTIMIZED</code> or <code>OPTIMIZED</code>.</p> </note></p>
    #[serde(rename = "finding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding: Option<String>,
    /// <p>The time stamp of when the Auto Scaling group recommendation was last refreshed.</p>
    #[serde(rename = "lastRefreshTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_refresh_timestamp: Option<f64>,
    /// <p>The number of days for which utilization metrics were analyzed for the Auto Scaling group.</p>
    #[serde(rename = "lookBackPeriodInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub look_back_period_in_days: Option<f64>,
    /// <p>An array of objects that describe the recommendation options for the Auto Scaling group.</p>
    #[serde(rename = "recommendationOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_options: Option<Vec<AutoScalingGroupRecommendationOption>>,
    /// <p>An array of objects that describe the utilization metrics of the Auto Scaling group.</p>
    #[serde(rename = "utilizationMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utilization_metrics: Option<Vec<UtilizationMetric>>,
}

/// <p>Describes a recommendation option for an Auto Scaling group.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AutoScalingGroupRecommendationOption {
    /// <p>An array of objects that describe an Auto Scaling group configuration.</p>
    #[serde(rename = "configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<AutoScalingGroupConfiguration>,
    /// <p>The performance risk of the Auto Scaling group configuration recommendation.</p> <p>Performance risk is the likelihood of the recommended instance type not meeting the performance requirement of your workload.</p> <p>The lowest performance risk is categorized as <code>0</code>, and the highest as <code>5</code>.</p>
    #[serde(rename = "performanceRisk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_risk: Option<f64>,
    /// <p>An array of objects that describe the projected utilization metrics of the Auto Scaling group recommendation option.</p>
    #[serde(rename = "projectedUtilizationMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projected_utilization_metrics: Option<Vec<UtilizationMetric>>,
    /// <p>The rank of the Auto Scaling group recommendation option.</p> <p>The top recommendation option is ranked as <code>1</code>.</p>
    #[serde(rename = "rank")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i64>,
}

/// <p>Describes a filter that returns a more specific list of recommendations.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Filter {
    /// <p>The name of the filter.</p> <p>Specify <code>Finding</code> to filter the results to a specific findings classification.</p> <p>Specify <code>RecommendationSourceType</code> to filter the results to a specific resource type.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The value of the filter.</p> <p>If you specify the <code>name</code> parameter as <code>Finding</code>, and you're recommendations for an <i>instance</i>, then the valid values are <code>Underprovisioned</code>, <code>Overprovisioned</code>, <code>NotOptimized</code>, or <code>Optimized</code>.</p> <p>If you specify the <code>name</code> parameter as <code>Finding</code>, and you're recommendations for an <i>Auto Scaling group</i>, then the valid values are <code>Optimized</code>, or <code>NotOptimized</code>.</p> <p>If you specify the <code>name</code> parameter as <code>RecommendationSourceType</code>, then the valid values are <code>EC2Instance</code>, or <code>AutoScalingGroup</code>.</p>
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAutoScalingGroupRecommendationsRequest {
    /// <p>The AWS account IDs for which to return Auto Scaling group recommendations.</p> <p>Only one account ID can be specified per request.</p>
    #[serde(rename = "accountIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// <p>The Amazon Resource Name (ARN) of the Auto Scaling groups for which to return recommendations.</p>
    #[serde(rename = "autoScalingGroupArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_group_arns: Option<Vec<String>>,
    /// <p>An array of objects that describe a filter that returns a more specific list of Auto Scaling group recommendations.</p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>The maximum number of Auto Scaling group recommendations to return with a single call.</p> <p>To retrieve the remaining results, make another call with the returned <code>NextToken</code> value.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to advance to the next page of Auto Scaling group recommendations.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAutoScalingGroupRecommendationsResponse {
    /// <p>An array of objects that describe Auto Scaling group recommendations.</p>
    #[serde(rename = "autoScalingGroupRecommendations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_group_recommendations: Option<Vec<AutoScalingGroupRecommendation>>,
    /// <p>An array of objects that describe errors of the request.</p> <p>For example, an error is returned if you request recommendations for an unsupported Auto Scaling group.</p>
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<GetRecommendationError>>,
    /// <p>The token to use to advance to the next page of Auto Scaling group recommendations.</p> <p>This value is null when there are no more pages of Auto Scaling group recommendations to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetEC2InstanceRecommendationsRequest {
    /// <p>The AWS account IDs for which to return instance recommendations.</p> <p>Only one account ID can be specified per request.</p>
    #[serde(rename = "accountIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// <p>An array of objects that describe a filter that returns a more specific list of instance recommendations.</p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>The Amazon Resource Name (ARN) of the instances for which to return recommendations.</p>
    #[serde(rename = "instanceArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_arns: Option<Vec<String>>,
    /// <p>The maximum number of instance recommendations to return with a single call.</p> <p>To retrieve the remaining results, make another call with the returned <code>NextToken</code> value.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to advance to the next page of instance recommendations.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetEC2InstanceRecommendationsResponse {
    /// <p>An array of objects that describe errors of the request.</p> <p>For example, an error is returned if you request recommendations for an instance of an unsupported instance family.</p>
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<GetRecommendationError>>,
    /// <p>An array of objects that describe instance recommendations.</p>
    #[serde(rename = "instanceRecommendations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_recommendations: Option<Vec<InstanceRecommendation>>,
    /// <p>The token to use to advance to the next page of instance recommendations.</p> <p>This value is null when there are no more pages of instance recommendations to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetEC2RecommendationProjectedMetricsRequest {
    /// <p>The time stamp of the last projected metrics data point to return.</p>
    #[serde(rename = "endTime")]
    pub end_time: f64,
    /// <p>The Amazon Resource Name (ARN) of the instances for which to return recommendation projected metrics.</p>
    #[serde(rename = "instanceArn")]
    pub instance_arn: String,
    /// <p>The granularity, in seconds, of the projected metrics data points.</p>
    #[serde(rename = "period")]
    pub period: i64,
    /// <p>The time stamp of the first projected metrics data point to return.</p>
    #[serde(rename = "startTime")]
    pub start_time: f64,
    /// <p>The statistic of the projected metrics.</p>
    #[serde(rename = "stat")]
    pub stat: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetEC2RecommendationProjectedMetricsResponse {
    /// <p>An array of objects that describe a projected metrics.</p>
    #[serde(rename = "recommendedOptionProjectedMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_option_projected_metrics: Option<Vec<RecommendedOptionProjectedMetric>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetEnrollmentStatusRequest {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetEnrollmentStatusResponse {
    /// <p>Confirms the enrollment status of member accounts within the organization, if the account is a master account of an organization.</p>
    #[serde(rename = "memberAccountsEnrolled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_accounts_enrolled: Option<bool>,
    /// <p>The enrollment status of the account.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The reason for the enrollment status of the account.</p> <p>For example, an account might show a status of <code>Pending</code> because member accounts of an organization require more time to be enrolled in the service.</p>
    #[serde(rename = "statusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
}

/// <p>Describes an error experienced when getting recommendations.</p> <p>For example, an error is returned if you request recommendations for an unsupported Auto Scaling group, or if you request recommendations for an instance of an unsupported instance family.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRecommendationError {
    /// <p>The error code.</p>
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// <p>The ID of the error.</p>
    #[serde(rename = "identifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// <p>The message, or reason, for the error.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRecommendationSummariesRequest {
    /// <p>The AWS account IDs for which to return recommendation summaries.</p> <p>Only one account ID can be specified per request.</p>
    #[serde(rename = "accountIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// <p>The maximum number of recommendation summaries to return with a single call.</p> <p>To retrieve the remaining results, make another call with the returned <code>NextToken</code> value.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to advance to the next page of recommendation summaries.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRecommendationSummariesResponse {
    /// <p>The token to use to advance to the next page of recommendation summaries.</p> <p>This value is null when there are no more pages of recommendation summaries to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of objects that summarize a recommendation.</p>
    #[serde(rename = "recommendationSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_summaries: Option<Vec<RecommendationSummary>>,
}

/// <p>Describes an Amazon EC2 instance recommendation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceRecommendation {
    /// <p>The AWS account ID of the instance recommendation.</p>
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The instance type of the current instance.</p>
    #[serde(rename = "currentInstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_instance_type: Option<String>,
    /// <p><p>The finding classification for the instance.</p> <p>Findings for instances include:</p> <ul> <li> <p> <b> <code>Underprovisioned</code> </b>—An instance is considered under-provisioned when at least one specification of your instance, such as CPU, memory, or network, does not meet the performance requirements of your workload. Under-provisioned instances may lead to poor application performance.</p> </li> <li> <p> <b> <code>Overprovisioned</code> </b>—An instance is considered over-provisioned when at least one specification of your instance, such as CPU, memory, or network, can be sized down while still meeting the performance requirements of your workload, and no specification is under-provisioned. Over-provisioned instances may lead to unnecessary infrastructure cost.</p> </li> <li> <p> <b> <code>Optimized</code> </b>—An instance is considered optimized when all specifications of your instance, such as CPU, memory, and network, meet the performance requirements of your workload and is not over provisioned. An optimized instance runs your workloads with optimal performance and infrastructure cost. For optimized resources, AWS Compute Optimizer might recommend a new generation instance type.</p> </li> </ul> <note> <p>The values that are returned might be <code>UNDER<em>PROVISIONED</code>, <code>OVER</em>PROVISIONED</code>, or <code>OPTIMIZED</code>.</p> </note></p>
    #[serde(rename = "finding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the current instance.</p>
    #[serde(rename = "instanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_arn: Option<String>,
    /// <p>The name of the current instance.</p>
    #[serde(rename = "instanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    /// <p>The time stamp of when the instance recommendation was last refreshed.</p>
    #[serde(rename = "lastRefreshTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_refresh_timestamp: Option<f64>,
    /// <p>The number of days for which utilization metrics were analyzed for the instance.</p>
    #[serde(rename = "lookBackPeriodInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub look_back_period_in_days: Option<f64>,
    /// <p>An array of objects that describe the recommendation options for the instance.</p>
    #[serde(rename = "recommendationOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_options: Option<Vec<InstanceRecommendationOption>>,
    /// <p>An array of objects that describe the source resource of the recommendation.</p>
    #[serde(rename = "recommendationSources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_sources: Option<Vec<RecommendationSource>>,
    /// <p>An array of objects that describe the utilization metrics of the instance.</p>
    #[serde(rename = "utilizationMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utilization_metrics: Option<Vec<UtilizationMetric>>,
}

/// <p>Describes a recommendation option for an Amazon EC2 instance.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceRecommendationOption {
    /// <p>The instance type of the instance recommendation.</p>
    #[serde(rename = "instanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p>The performance risk of the instance recommendation option.</p> <p>Performance risk is the likelihood of the recommended instance type not meeting the performance requirement of your workload.</p> <p>The lowest performance risk is categorized as <code>0</code>, and the highest as <code>5</code>.</p>
    #[serde(rename = "performanceRisk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_risk: Option<f64>,
    /// <p>An array of objects that describe the projected utilization metrics of the instance recommendation option.</p>
    #[serde(rename = "projectedUtilizationMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projected_utilization_metrics: Option<Vec<UtilizationMetric>>,
    /// <p>The rank of the instance recommendation option.</p> <p>The top recommendation option is ranked as <code>1</code>.</p>
    #[serde(rename = "rank")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i64>,
}

/// <p>Describes a projected utilization metric of a recommendation option, such as an Amazon EC2 instance.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProjectedMetric {
    /// <p><p>The name of the projected utilization metric.</p> <note> <p>Memory metrics are only returned for resources that have the unified CloudWatch agent installed on them. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/Install-CloudWatch-Agent.html">Enabling Memory Utilization with the CloudWatch Agent</a>.</p> </note></p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The time stamps of the projected utilization metric.</p>
    #[serde(rename = "timestamps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamps: Option<Vec<f64>>,
    /// <p>The values of the projected utilization metrics.</p>
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<f64>>,
}

/// <p>Describes the source of a recommendation, such as an Amazon EC2 instance or Auto Scaling group.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RecommendationSource {
    /// <p>The Amazon Resource Name (ARN) of the recommendation source.</p>
    #[serde(rename = "recommendationSourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_source_arn: Option<String>,
    /// <p>The resource type of the recommendation source.</p>
    #[serde(rename = "recommendationSourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_source_type: Option<String>,
}

/// <p>A summary of a recommendation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RecommendationSummary {
    /// <p>The AWS account ID of the recommendation summary.</p>
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The resource type of the recommendation.</p>
    #[serde(rename = "recommendationResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_resource_type: Option<String>,
    /// <p>An array of objects that describe a recommendation summary.</p>
    #[serde(rename = "summaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summaries: Option<Vec<Summary>>,
}

/// <p>Describes a projected utilization metric of a recommendation option.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RecommendedOptionProjectedMetric {
    /// <p>An array of objects that describe a projected utilization metric.</p>
    #[serde(rename = "projectedMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projected_metrics: Option<Vec<ProjectedMetric>>,
    /// <p>The rank of the recommendation option projected metric.</p> <p>The top recommendation option is ranked as <code>1</code>.</p> <p>The projected metric rank correlates to the recommendation option rank. For example, the projected metric ranked as <code>1</code> is related to the recommendation option that is also ranked as <code>1</code> in the same response.</p>
    #[serde(rename = "rank")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i64>,
    /// <p>The recommended instance type.</p>
    #[serde(rename = "recommendedInstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_instance_type: Option<String>,
}

/// <p>The summary of a recommendation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Summary {
    /// <p>The finding classification of the recommendation.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The value of the recommendation summary.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateEnrollmentStatusRequest {
    /// <p>Indicates whether to enroll member accounts within the organization, if the account is a master account of an organization.</p>
    #[serde(rename = "includeMemberAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_member_accounts: Option<bool>,
    /// <p>The new enrollment status of the account.</p> <p>Accepted options are <code>Active</code> or <code>Inactive</code>. You will get an error if <code>Pending</code> or <code>Failed</code> are specified.</p>
    #[serde(rename = "status")]
    pub status: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateEnrollmentStatusResponse {
    /// <p>The enrollment status of the account.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The reason for the enrollment status of the account. For example, an account might show a status of <code>Pending</code> because member accounts of an organization require more time to be enrolled in the service.</p>
    #[serde(rename = "statusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
}

/// <p>Describes a utilization metric of a resource, such as an Amazon EC2 instance.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UtilizationMetric {
    /// <p><p>The name of the utilization metric.</p> <note> <p>Memory metrics are only returned for resources that have the unified CloudWatch agent installed on them. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/Install-CloudWatch-Agent.html">Enabling Memory Utilization with the CloudWatch Agent</a>.</p> </note></p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The statistic of the utilization metric.</p>
    #[serde(rename = "statistic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistic: Option<String>,
    /// <p>The value of the utilization metric.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

/// Errors returned by GetAutoScalingGroupRecommendations
#[derive(Debug, PartialEq)]
pub enum GetAutoScalingGroupRecommendationsError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>The request processing has failed because of an unknown error, exception, or failure.</p>
    InternalServer(String),
    /// <p>An invalid or out-of-range value was supplied for the input parameter.</p>
    InvalidParameterValue(String),
    /// <p>The request must contain either a valid (registered) AWS access key ID or X.509 certificate.</p>
    MissingAuthenticationToken(String),
    /// <p>You must opt in to the service to perform this action.</p>
    OptInRequired(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The request has failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
    /// <p>The limit on the number of requests per second was exceeded.</p>
    Throttling(String),
}

impl GetAutoScalingGroupRecommendationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetAutoScalingGroupRecommendationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        GetAutoScalingGroupRecommendationsError::AccessDenied(err.msg),
                    )
                }
                "InternalServerException" => {
                    return RusotoError::Service(
                        GetAutoScalingGroupRecommendationsError::InternalServer(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        GetAutoScalingGroupRecommendationsError::InvalidParameterValue(err.msg),
                    )
                }
                "MissingAuthenticationToken" => {
                    return RusotoError::Service(
                        GetAutoScalingGroupRecommendationsError::MissingAuthenticationToken(
                            err.msg,
                        ),
                    )
                }
                "OptInRequiredException" => {
                    return RusotoError::Service(
                        GetAutoScalingGroupRecommendationsError::OptInRequired(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        GetAutoScalingGroupRecommendationsError::ResourceNotFound(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        GetAutoScalingGroupRecommendationsError::ServiceUnavailable(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(
                        GetAutoScalingGroupRecommendationsError::Throttling(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetAutoScalingGroupRecommendationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAutoScalingGroupRecommendationsError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            GetAutoScalingGroupRecommendationsError::InternalServer(ref cause) => {
                write!(f, "{}", cause)
            }
            GetAutoScalingGroupRecommendationsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            GetAutoScalingGroupRecommendationsError::MissingAuthenticationToken(ref cause) => {
                write!(f, "{}", cause)
            }
            GetAutoScalingGroupRecommendationsError::OptInRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            GetAutoScalingGroupRecommendationsError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            GetAutoScalingGroupRecommendationsError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            GetAutoScalingGroupRecommendationsError::Throttling(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetAutoScalingGroupRecommendationsError {}
/// Errors returned by GetEC2InstanceRecommendations
#[derive(Debug, PartialEq)]
pub enum GetEC2InstanceRecommendationsError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>The request processing has failed because of an unknown error, exception, or failure.</p>
    InternalServer(String),
    /// <p>An invalid or out-of-range value was supplied for the input parameter.</p>
    InvalidParameterValue(String),
    /// <p>The request must contain either a valid (registered) AWS access key ID or X.509 certificate.</p>
    MissingAuthenticationToken(String),
    /// <p>You must opt in to the service to perform this action.</p>
    OptInRequired(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The request has failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
    /// <p>The limit on the number of requests per second was exceeded.</p>
    Throttling(String),
}

impl GetEC2InstanceRecommendationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetEC2InstanceRecommendationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetEC2InstanceRecommendationsError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalServerException" => {
                    return RusotoError::Service(
                        GetEC2InstanceRecommendationsError::InternalServer(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        GetEC2InstanceRecommendationsError::InvalidParameterValue(err.msg),
                    )
                }
                "MissingAuthenticationToken" => {
                    return RusotoError::Service(
                        GetEC2InstanceRecommendationsError::MissingAuthenticationToken(err.msg),
                    )
                }
                "OptInRequiredException" => {
                    return RusotoError::Service(GetEC2InstanceRecommendationsError::OptInRequired(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        GetEC2InstanceRecommendationsError::ResourceNotFound(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        GetEC2InstanceRecommendationsError::ServiceUnavailable(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetEC2InstanceRecommendationsError::Throttling(
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
impl fmt::Display for GetEC2InstanceRecommendationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetEC2InstanceRecommendationsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetEC2InstanceRecommendationsError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetEC2InstanceRecommendationsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            GetEC2InstanceRecommendationsError::MissingAuthenticationToken(ref cause) => {
                write!(f, "{}", cause)
            }
            GetEC2InstanceRecommendationsError::OptInRequired(ref cause) => write!(f, "{}", cause),
            GetEC2InstanceRecommendationsError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            GetEC2InstanceRecommendationsError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            GetEC2InstanceRecommendationsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetEC2InstanceRecommendationsError {}
/// Errors returned by GetEC2RecommendationProjectedMetrics
#[derive(Debug, PartialEq)]
pub enum GetEC2RecommendationProjectedMetricsError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>The request processing has failed because of an unknown error, exception, or failure.</p>
    InternalServer(String),
    /// <p>An invalid or out-of-range value was supplied for the input parameter.</p>
    InvalidParameterValue(String),
    /// <p>The request must contain either a valid (registered) AWS access key ID or X.509 certificate.</p>
    MissingAuthenticationToken(String),
    /// <p>You must opt in to the service to perform this action.</p>
    OptInRequired(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The request has failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
    /// <p>The limit on the number of requests per second was exceeded.</p>
    Throttling(String),
}

impl GetEC2RecommendationProjectedMetricsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetEC2RecommendationProjectedMetricsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        GetEC2RecommendationProjectedMetricsError::AccessDenied(err.msg),
                    )
                }
                "InternalServerException" => {
                    return RusotoError::Service(
                        GetEC2RecommendationProjectedMetricsError::InternalServer(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        GetEC2RecommendationProjectedMetricsError::InvalidParameterValue(err.msg),
                    )
                }
                "MissingAuthenticationToken" => {
                    return RusotoError::Service(
                        GetEC2RecommendationProjectedMetricsError::MissingAuthenticationToken(
                            err.msg,
                        ),
                    )
                }
                "OptInRequiredException" => {
                    return RusotoError::Service(
                        GetEC2RecommendationProjectedMetricsError::OptInRequired(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        GetEC2RecommendationProjectedMetricsError::ResourceNotFound(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        GetEC2RecommendationProjectedMetricsError::ServiceUnavailable(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(
                        GetEC2RecommendationProjectedMetricsError::Throttling(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetEC2RecommendationProjectedMetricsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetEC2RecommendationProjectedMetricsError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            GetEC2RecommendationProjectedMetricsError::InternalServer(ref cause) => {
                write!(f, "{}", cause)
            }
            GetEC2RecommendationProjectedMetricsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            GetEC2RecommendationProjectedMetricsError::MissingAuthenticationToken(ref cause) => {
                write!(f, "{}", cause)
            }
            GetEC2RecommendationProjectedMetricsError::OptInRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            GetEC2RecommendationProjectedMetricsError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            GetEC2RecommendationProjectedMetricsError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            GetEC2RecommendationProjectedMetricsError::Throttling(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetEC2RecommendationProjectedMetricsError {}
/// Errors returned by GetEnrollmentStatus
#[derive(Debug, PartialEq)]
pub enum GetEnrollmentStatusError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>The request processing has failed because of an unknown error, exception, or failure.</p>
    InternalServer(String),
    /// <p>An invalid or out-of-range value was supplied for the input parameter.</p>
    InvalidParameterValue(String),
    /// <p>The request must contain either a valid (registered) AWS access key ID or X.509 certificate.</p>
    MissingAuthenticationToken(String),
    /// <p>The request has failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
    /// <p>The limit on the number of requests per second was exceeded.</p>
    Throttling(String),
}

impl GetEnrollmentStatusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetEnrollmentStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetEnrollmentStatusError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(GetEnrollmentStatusError::InternalServer(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(GetEnrollmentStatusError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "MissingAuthenticationToken" => {
                    return RusotoError::Service(
                        GetEnrollmentStatusError::MissingAuthenticationToken(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetEnrollmentStatusError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetEnrollmentStatusError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetEnrollmentStatusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetEnrollmentStatusError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetEnrollmentStatusError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetEnrollmentStatusError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            GetEnrollmentStatusError::MissingAuthenticationToken(ref cause) => {
                write!(f, "{}", cause)
            }
            GetEnrollmentStatusError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            GetEnrollmentStatusError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetEnrollmentStatusError {}
/// Errors returned by GetRecommendationSummaries
#[derive(Debug, PartialEq)]
pub enum GetRecommendationSummariesError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>The request processing has failed because of an unknown error, exception, or failure.</p>
    InternalServer(String),
    /// <p>An invalid or out-of-range value was supplied for the input parameter.</p>
    InvalidParameterValue(String),
    /// <p>The request must contain either a valid (registered) AWS access key ID or X.509 certificate.</p>
    MissingAuthenticationToken(String),
    /// <p>You must opt in to the service to perform this action.</p>
    OptInRequired(String),
    /// <p>The request has failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
    /// <p>The limit on the number of requests per second was exceeded.</p>
    Throttling(String),
}

impl GetRecommendationSummariesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetRecommendationSummariesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetRecommendationSummariesError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalServerException" => {
                    return RusotoError::Service(GetRecommendationSummariesError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        GetRecommendationSummariesError::InvalidParameterValue(err.msg),
                    )
                }
                "MissingAuthenticationToken" => {
                    return RusotoError::Service(
                        GetRecommendationSummariesError::MissingAuthenticationToken(err.msg),
                    )
                }
                "OptInRequiredException" => {
                    return RusotoError::Service(GetRecommendationSummariesError::OptInRequired(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        GetRecommendationSummariesError::ServiceUnavailable(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetRecommendationSummariesError::Throttling(
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
impl fmt::Display for GetRecommendationSummariesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRecommendationSummariesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetRecommendationSummariesError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetRecommendationSummariesError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            GetRecommendationSummariesError::MissingAuthenticationToken(ref cause) => {
                write!(f, "{}", cause)
            }
            GetRecommendationSummariesError::OptInRequired(ref cause) => write!(f, "{}", cause),
            GetRecommendationSummariesError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            GetRecommendationSummariesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetRecommendationSummariesError {}
/// Errors returned by UpdateEnrollmentStatus
#[derive(Debug, PartialEq)]
pub enum UpdateEnrollmentStatusError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>The request processing has failed because of an unknown error, exception, or failure.</p>
    InternalServer(String),
    /// <p>An invalid or out-of-range value was supplied for the input parameter.</p>
    InvalidParameterValue(String),
    /// <p>The request must contain either a valid (registered) AWS access key ID or X.509 certificate.</p>
    MissingAuthenticationToken(String),
    /// <p>The request has failed due to a temporary failure of the server.</p>
    ServiceUnavailable(String),
    /// <p>The limit on the number of requests per second was exceeded.</p>
    Throttling(String),
}

impl UpdateEnrollmentStatusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateEnrollmentStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateEnrollmentStatusError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(UpdateEnrollmentStatusError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        UpdateEnrollmentStatusError::InvalidParameterValue(err.msg),
                    )
                }
                "MissingAuthenticationToken" => {
                    return RusotoError::Service(
                        UpdateEnrollmentStatusError::MissingAuthenticationToken(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateEnrollmentStatusError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateEnrollmentStatusError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateEnrollmentStatusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateEnrollmentStatusError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateEnrollmentStatusError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateEnrollmentStatusError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            UpdateEnrollmentStatusError::MissingAuthenticationToken(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateEnrollmentStatusError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateEnrollmentStatusError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateEnrollmentStatusError {}
/// Trait representing the capabilities of the AWS Compute Optimizer API. AWS Compute Optimizer clients implement this trait.
#[async_trait]
pub trait ComputeOptimizer {
    /// <p>Returns Auto Scaling group recommendations.</p> <p>AWS Compute Optimizer currently generates recommendations for Auto Scaling groups that are configured to run instances of the M, C, R, T, and X instance families. The service does not generate recommendations for Auto Scaling groups that have a scaling policy attached to them, or that do not have the same values for desired, minimum, and maximum capacity. In order for Compute Optimizer to analyze your Auto Scaling groups, they must be of a fixed size. For more information, see the <a href="https://docs.aws.amazon.com/compute-optimizer/latest/ug/what-is.html">AWS Compute Optimizer User Guide</a>.</p>
    async fn get_auto_scaling_group_recommendations(
        &self,
        input: GetAutoScalingGroupRecommendationsRequest,
    ) -> Result<
        GetAutoScalingGroupRecommendationsResponse,
        RusotoError<GetAutoScalingGroupRecommendationsError>,
    >;

    /// <p>Returns Amazon EC2 instance recommendations.</p> <p>AWS Compute Optimizer currently generates recommendations for Amazon Elastic Compute Cloud (Amazon EC2) and Amazon EC2 Auto Scaling. It generates recommendations for M, C, R, T, and X instance families. For more information, see the <a href="https://docs.aws.amazon.com/compute-optimizer/latest/ug/what-is.html">AWS Compute Optimizer User Guide</a>.</p>
    async fn get_ec2_instance_recommendations(
        &self,
        input: GetEC2InstanceRecommendationsRequest,
    ) -> Result<
        GetEC2InstanceRecommendationsResponse,
        RusotoError<GetEC2InstanceRecommendationsError>,
    >;

    /// <p>Returns the projected utilization metrics of Amazon EC2 instance recommendations.</p>
    async fn get_ec2_recommendation_projected_metrics(
        &self,
        input: GetEC2RecommendationProjectedMetricsRequest,
    ) -> Result<
        GetEC2RecommendationProjectedMetricsResponse,
        RusotoError<GetEC2RecommendationProjectedMetricsError>,
    >;

    /// <p>Returns the enrollment (opt in) status of an account to the AWS Compute Optimizer service.</p> <p>If the account is a master account of an organization, this operation also confirms the enrollment status of member accounts within the organization.</p>
    async fn get_enrollment_status(
        &self,
    ) -> Result<GetEnrollmentStatusResponse, RusotoError<GetEnrollmentStatusError>>;

    /// <p>Returns the optimization findings for an account.</p> <p>For example, it returns the number of Amazon EC2 instances in an account that are under-provisioned, over-provisioned, or optimized. It also returns the number of Auto Scaling groups in an account that are not optimized, or optimized.</p>
    async fn get_recommendation_summaries(
        &self,
        input: GetRecommendationSummariesRequest,
    ) -> Result<GetRecommendationSummariesResponse, RusotoError<GetRecommendationSummariesError>>;

    /// <p>Updates the enrollment (opt in) status of an account to the AWS Compute Optimizer service.</p> <p>If the account is a master account of an organization, this operation can also enroll member accounts within the organization.</p>
    async fn update_enrollment_status(
        &self,
        input: UpdateEnrollmentStatusRequest,
    ) -> Result<UpdateEnrollmentStatusResponse, RusotoError<UpdateEnrollmentStatusError>>;
}
/// A client for the AWS Compute Optimizer API.
#[derive(Clone)]
pub struct ComputeOptimizerClient {
    client: Client,
    region: region::Region,
}

impl ComputeOptimizerClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ComputeOptimizerClient {
        ComputeOptimizerClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ComputeOptimizerClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        ComputeOptimizerClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> ComputeOptimizerClient {
        ComputeOptimizerClient { client, region }
    }
}

#[async_trait]
impl ComputeOptimizer for ComputeOptimizerClient {
    /// <p>Returns Auto Scaling group recommendations.</p> <p>AWS Compute Optimizer currently generates recommendations for Auto Scaling groups that are configured to run instances of the M, C, R, T, and X instance families. The service does not generate recommendations for Auto Scaling groups that have a scaling policy attached to them, or that do not have the same values for desired, minimum, and maximum capacity. In order for Compute Optimizer to analyze your Auto Scaling groups, they must be of a fixed size. For more information, see the <a href="https://docs.aws.amazon.com/compute-optimizer/latest/ug/what-is.html">AWS Compute Optimizer User Guide</a>.</p>
    async fn get_auto_scaling_group_recommendations(
        &self,
        input: GetAutoScalingGroupRecommendationsRequest,
    ) -> Result<
        GetAutoScalingGroupRecommendationsResponse,
        RusotoError<GetAutoScalingGroupRecommendationsError>,
    > {
        let mut request = SignedRequest::new("POST", "compute-optimizer", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "ComputeOptimizerService.GetAutoScalingGroupRecommendations",
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
                .deserialize::<GetAutoScalingGroupRecommendationsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetAutoScalingGroupRecommendationsError::from_response(
                response,
            ))
        }
    }

    /// <p>Returns Amazon EC2 instance recommendations.</p> <p>AWS Compute Optimizer currently generates recommendations for Amazon Elastic Compute Cloud (Amazon EC2) and Amazon EC2 Auto Scaling. It generates recommendations for M, C, R, T, and X instance families. For more information, see the <a href="https://docs.aws.amazon.com/compute-optimizer/latest/ug/what-is.html">AWS Compute Optimizer User Guide</a>.</p>
    async fn get_ec2_instance_recommendations(
        &self,
        input: GetEC2InstanceRecommendationsRequest,
    ) -> Result<
        GetEC2InstanceRecommendationsResponse,
        RusotoError<GetEC2InstanceRecommendationsError>,
    > {
        let mut request = SignedRequest::new("POST", "compute-optimizer", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "ComputeOptimizerService.GetEC2InstanceRecommendations",
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
                .deserialize::<GetEC2InstanceRecommendationsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetEC2InstanceRecommendationsError::from_response(response))
        }
    }

    /// <p>Returns the projected utilization metrics of Amazon EC2 instance recommendations.</p>
    async fn get_ec2_recommendation_projected_metrics(
        &self,
        input: GetEC2RecommendationProjectedMetricsRequest,
    ) -> Result<
        GetEC2RecommendationProjectedMetricsResponse,
        RusotoError<GetEC2RecommendationProjectedMetricsError>,
    > {
        let mut request = SignedRequest::new("POST", "compute-optimizer", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "ComputeOptimizerService.GetEC2RecommendationProjectedMetrics",
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
                .deserialize::<GetEC2RecommendationProjectedMetricsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetEC2RecommendationProjectedMetricsError::from_response(
                response,
            ))
        }
    }

    /// <p>Returns the enrollment (opt in) status of an account to the AWS Compute Optimizer service.</p> <p>If the account is a master account of an organization, this operation also confirms the enrollment status of member accounts within the organization.</p>
    async fn get_enrollment_status(
        &self,
    ) -> Result<GetEnrollmentStatusResponse, RusotoError<GetEnrollmentStatusError>> {
        let mut request = SignedRequest::new("POST", "compute-optimizer", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "ComputeOptimizerService.GetEnrollmentStatus",
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
                .deserialize::<GetEnrollmentStatusResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetEnrollmentStatusError::from_response(response))
        }
    }

    /// <p>Returns the optimization findings for an account.</p> <p>For example, it returns the number of Amazon EC2 instances in an account that are under-provisioned, over-provisioned, or optimized. It also returns the number of Auto Scaling groups in an account that are not optimized, or optimized.</p>
    async fn get_recommendation_summaries(
        &self,
        input: GetRecommendationSummariesRequest,
    ) -> Result<GetRecommendationSummariesResponse, RusotoError<GetRecommendationSummariesError>>
    {
        let mut request = SignedRequest::new("POST", "compute-optimizer", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "ComputeOptimizerService.GetRecommendationSummaries",
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
                .deserialize::<GetRecommendationSummariesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetRecommendationSummariesError::from_response(response))
        }
    }

    /// <p>Updates the enrollment (opt in) status of an account to the AWS Compute Optimizer service.</p> <p>If the account is a master account of an organization, this operation can also enroll member accounts within the organization.</p>
    async fn update_enrollment_status(
        &self,
        input: UpdateEnrollmentStatusRequest,
    ) -> Result<UpdateEnrollmentStatusResponse, RusotoError<UpdateEnrollmentStatusError>> {
        let mut request = SignedRequest::new("POST", "compute-optimizer", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "ComputeOptimizerService.UpdateEnrollmentStatus",
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
                .deserialize::<UpdateEnrollmentStatusResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateEnrollmentStatusError::from_response(response))
        }
    }
}
