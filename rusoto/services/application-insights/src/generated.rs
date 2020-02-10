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
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
use std::pin::Pin;
/// <p>Describes a standalone resource or similarly grouped resources that the application is made up of.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ApplicationComponent {
    /// <p>The name of the component.</p>
    #[serde(rename = "ComponentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_name: Option<String>,
    /// <p>Indicates whether the application component is monitored. </p>
    #[serde(rename = "Monitor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor: Option<bool>,
    /// <p>The resource type. Supported resource types include EC2 instances, Auto Scaling group, Classic ELB, Application ELB, and SQS Queue.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The stack tier of the application component.</p>
    #[serde(rename = "Tier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}

/// <p>Describes the status of the application.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ApplicationInfo {
    /// <p>The lifecycle of the application. </p>
    #[serde(rename = "LifeCycle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub life_cycle: Option<String>,
    /// <p> Indicates whether Application Insights will create opsItems for any problem detected by Application Insights for an application. </p>
    #[serde(rename = "OpsCenterEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_center_enabled: Option<bool>,
    /// <p> The SNS topic provided to Application Insights that is associated to the created opsItems to receive SNS notifications for opsItem updates. </p>
    #[serde(rename = "OpsItemSNSTopicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_item_sns_topic_arn: Option<String>,
    /// <p><p>The issues on the user side that block Application Insights from successfully monitoring an application. Example remarks include:</p> <ul> <li> <p>“Configuring application, detected 1 Errors, 3 Warnings”</p> </li> <li> <p>“Configuring application, detected 1 Unconfigured Components”</p> </li> </ul></p>
    #[serde(rename = "Remarks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<String>,
    /// <p>The name of the resource group used for the application.</p>
    #[serde(rename = "ResourceGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
}

/// <p> The event information. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConfigurationEvent {
    /// <p> The details of the event in plain text. </p>
    #[serde(rename = "EventDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_detail: Option<String>,
    /// <p> The name of the resource Application Insights attempted to configure. </p>
    #[serde(rename = "EventResourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_resource_name: Option<String>,
    /// <p> The resource type that Application Insights attempted to configure, for example, CLOUDWATCH_ALARM. </p>
    #[serde(rename = "EventResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_resource_type: Option<String>,
    /// <p> The status of the configuration update event. Possible values include INFO, WARN, and ERROR. </p>
    #[serde(rename = "EventStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_status: Option<String>,
    /// <p> The timestamp of the event. </p>
    #[serde(rename = "EventTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_time: Option<f64>,
    /// <p> The resource monitored by Application Insights. </p>
    #[serde(rename = "MonitoredResourceARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitored_resource_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateApplicationRequest {
    /// <p> When set to <code>true</code>, creates opsItems for any problems detected on an application. </p>
    #[serde(rename = "OpsCenterEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_center_enabled: Option<bool>,
    /// <p> The SNS topic provided to Application Insights that is associated to the created opsItem. Allows you to receive notifications for updates to the opsItem. </p>
    #[serde(rename = "OpsItemSNSTopicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_item_sns_topic_arn: Option<String>,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "ResourceGroupName")]
    pub resource_group_name: String,
    /// <p>List of tags to add to the application. tag key (<code>Key</code>) and an associated tag value (<code>Value</code>). The maximum length of a tag key is 128 characters. The maximum length of a tag value is 256 characters.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateApplicationResponse {
    /// <p>Information about the application.</p>
    #[serde(rename = "ApplicationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_info: Option<ApplicationInfo>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateComponentRequest {
    /// <p>The name of the component.</p>
    #[serde(rename = "ComponentName")]
    pub component_name: String,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "ResourceGroupName")]
    pub resource_group_name: String,
    /// <p>The list of resource ARNs that belong to the component.</p>
    #[serde(rename = "ResourceList")]
    pub resource_list: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateComponentResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateLogPatternRequest {
    /// <p>The log pattern.</p>
    #[serde(rename = "Pattern")]
    pub pattern: String,
    /// <p>The name of the log pattern.</p>
    #[serde(rename = "PatternName")]
    pub pattern_name: String,
    /// <p>The name of the log pattern set.</p>
    #[serde(rename = "PatternSetName")]
    pub pattern_set_name: String,
    /// <p>Rank of the log pattern.</p>
    #[serde(rename = "Rank")]
    pub rank: i64,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "ResourceGroupName")]
    pub resource_group_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateLogPatternResponse {
    /// <p>The successfully created log pattern.</p>
    #[serde(rename = "LogPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_pattern: Option<LogPattern>,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "ResourceGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteApplicationRequest {
    /// <p>The name of the resource group.</p>
    #[serde(rename = "ResourceGroupName")]
    pub resource_group_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteApplicationResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteComponentRequest {
    /// <p>The name of the component.</p>
    #[serde(rename = "ComponentName")]
    pub component_name: String,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "ResourceGroupName")]
    pub resource_group_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteComponentResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteLogPatternRequest {
    /// <p>The name of the log pattern.</p>
    #[serde(rename = "PatternName")]
    pub pattern_name: String,
    /// <p>The name of the log pattern set.</p>
    #[serde(rename = "PatternSetName")]
    pub pattern_set_name: String,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "ResourceGroupName")]
    pub resource_group_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteLogPatternResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeApplicationRequest {
    /// <p>The name of the resource group.</p>
    #[serde(rename = "ResourceGroupName")]
    pub resource_group_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeApplicationResponse {
    /// <p>Information about the application.</p>
    #[serde(rename = "ApplicationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_info: Option<ApplicationInfo>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeComponentConfigurationRecommendationRequest {
    /// <p>The name of the component.</p>
    #[serde(rename = "ComponentName")]
    pub component_name: String,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "ResourceGroupName")]
    pub resource_group_name: String,
    /// <p>The tier of the application component. Supported tiers include <code>DOT_NET_CORE</code>, <code>DOT_NET_WORKER</code>, <code>DOT_NET_WEB</code>, <code>SQL_SERVER</code>, and <code>DEFAULT</code>.</p>
    #[serde(rename = "Tier")]
    pub tier: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeComponentConfigurationRecommendationResponse {
    /// <p>The recommended configuration settings of the component. The value is the escaped JSON of the configuration.</p>
    #[serde(rename = "ComponentConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_configuration: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeComponentConfigurationRequest {
    /// <p>The name of the component.</p>
    #[serde(rename = "ComponentName")]
    pub component_name: String,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "ResourceGroupName")]
    pub resource_group_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeComponentConfigurationResponse {
    /// <p>The configuration settings of the component. The value is the escaped JSON of the configuration.</p>
    #[serde(rename = "ComponentConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_configuration: Option<String>,
    /// <p>Indicates whether the application component is monitored.</p>
    #[serde(rename = "Monitor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor: Option<bool>,
    /// <p>The tier of the application component. Supported tiers include <code>DOT_NET_CORE</code>, <code>DOT_NET_WORKER</code>, <code>DOT_NET_WEB</code>, <code>SQL_SERVER</code>, and <code>DEFAULT</code> </p>
    #[serde(rename = "Tier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeComponentRequest {
    /// <p>The name of the component.</p>
    #[serde(rename = "ComponentName")]
    pub component_name: String,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "ResourceGroupName")]
    pub resource_group_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeComponentResponse {
    #[serde(rename = "ApplicationComponent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_component: Option<ApplicationComponent>,
    /// <p>The list of resource ARNs that belong to the component.</p>
    #[serde(rename = "ResourceList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_list: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeLogPatternRequest {
    /// <p>The name of the log pattern.</p>
    #[serde(rename = "PatternName")]
    pub pattern_name: String,
    /// <p>The name of the log pattern set.</p>
    #[serde(rename = "PatternSetName")]
    pub pattern_set_name: String,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "ResourceGroupName")]
    pub resource_group_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeLogPatternResponse {
    /// <p>The successfully created log pattern.</p>
    #[serde(rename = "LogPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_pattern: Option<LogPattern>,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "ResourceGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeObservationRequest {
    /// <p>The ID of the observation.</p>
    #[serde(rename = "ObservationId")]
    pub observation_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeObservationResponse {
    /// <p>Information about the observation.</p>
    #[serde(rename = "Observation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observation: Option<Observation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeProblemObservationsRequest {
    /// <p>The ID of the problem.</p>
    #[serde(rename = "ProblemId")]
    pub problem_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeProblemObservationsResponse {
    /// <p>Observations related to the problem.</p>
    #[serde(rename = "RelatedObservations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_observations: Option<RelatedObservations>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeProblemRequest {
    /// <p>The ID of the problem.</p>
    #[serde(rename = "ProblemId")]
    pub problem_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeProblemResponse {
    /// <p>Information about the problem. </p>
    #[serde(rename = "Problem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub problem: Option<Problem>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListApplicationsRequest {
    /// <p>The maximum number of results to return in a single call. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to request the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListApplicationsResponse {
    /// <p>The list of applications.</p>
    #[serde(rename = "ApplicationInfoList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_info_list: Option<Vec<ApplicationInfo>>,
    /// <p>The token used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListComponentsRequest {
    /// <p>The maximum number of results to return in a single call. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to request the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "ResourceGroupName")]
    pub resource_group_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListComponentsResponse {
    /// <p>The list of application components.</p>
    #[serde(rename = "ApplicationComponentList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_component_list: Option<Vec<ApplicationComponent>>,
    /// <p>The token to request the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListConfigurationHistoryRequest {
    /// <p>The end time of the event.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The status of the configuration update event. Possible values include INFO, WARN, and ERROR.</p>
    #[serde(rename = "EventStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_status: Option<String>,
    /// <p> The maximum number of results returned by <code>ListConfigurationHistory</code> in paginated output. When this parameter is used, <code>ListConfigurationHistory</code> returns only <code>MaxResults</code> in a single page along with a <code>NextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListConfigurationHistory</code> request with the returned <code>NextToken</code> value. If this parameter is not used, then <code>ListConfigurationHistory</code> returns all results. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The <code>NextToken</code> value returned from a previous paginated <code>ListConfigurationHistory</code> request where <code>MaxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>NextToken</code> value. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Resource group to which the application belongs. </p>
    #[serde(rename = "ResourceGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
    /// <p>The start time of the event. </p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListConfigurationHistoryResponse {
    /// <p> The list of configuration events and their corresponding details. </p>
    #[serde(rename = "EventList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_list: Option<Vec<ConfigurationEvent>>,
    /// <p>The <code>NextToken</code> value to include in a future <code>ListConfigurationHistory</code> request. When the results of a <code>ListConfigurationHistory</code> request exceed <code>MaxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListLogPatternSetsRequest {
    /// <p>The maximum number of results to return in a single call. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to request the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "ResourceGroupName")]
    pub resource_group_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListLogPatternSetsResponse {
    /// <p>The list of log pattern sets.</p>
    #[serde(rename = "LogPatternSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_pattern_sets: Option<Vec<String>>,
    /// <p>The token used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "ResourceGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListLogPatternsRequest {
    /// <p>The maximum number of results to return in a single call. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to request the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the log pattern set.</p>
    #[serde(rename = "PatternSetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern_set_name: Option<String>,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "ResourceGroupName")]
    pub resource_group_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListLogPatternsResponse {
    /// <p>The list of log patterns.</p>
    #[serde(rename = "LogPatterns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_patterns: Option<Vec<LogPattern>>,
    /// <p>The token used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "ResourceGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListProblemsRequest {
    /// <p>The time when the problem ended, in epoch seconds. If not specified, problems within the past seven days are returned.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The maximum number of results to return in a single call. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to request the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "ResourceGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
    /// <p>The time when the problem was detected, in epoch seconds. If you don't specify a time frame for the request, problems within the past seven days are returned.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListProblemsResponse {
    /// <p>The token used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of problems. </p>
    #[serde(rename = "ProblemList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub problem_list: Option<Vec<Problem>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the application that you want to retrieve tag information for.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>An array that lists all the tags that are associated with the application. Each tag consists of a required tag key (<code>Key</code>) and an associated tag value (<code>Value</code>).</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>An object that defines the log patterns that belongs to a <code>LogPatternSet</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LogPattern {
    /// <p>A regular expression that defines the log pattern. A log pattern can contains at many as 50 characters, and it cannot be empty.</p>
    #[serde(rename = "Pattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    /// <p>The name of the log pattern. A log pattern name can contains at many as 50 characters, and it cannot be empty. The characters can be Unicode letters, digits or one of the following symbols: period, dash, underscore.</p>
    #[serde(rename = "PatternName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern_name: Option<String>,
    /// <p>The name of the log pattern. A log pattern name can contains at many as 30 characters, and it cannot be empty. The characters can be Unicode letters, digits or one of the following symbols: period, dash, underscore.</p>
    #[serde(rename = "PatternSetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern_set_name: Option<String>,
    /// <p>Rank of the log pattern.</p>
    #[serde(rename = "Rank")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i64>,
}

/// <p>Describes an anomaly or error with the application.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Observation {
    /// <p>The time when the observation ended, in epoch seconds.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The ID of the observation type.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The timestamp in the CloudWatch Logs that specifies when the matched line occurred.</p>
    #[serde(rename = "LineTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_time: Option<f64>,
    /// <p>The log filter of the observation.</p>
    #[serde(rename = "LogFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_filter: Option<String>,
    /// <p>The log group name.</p>
    #[serde(rename = "LogGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group: Option<String>,
    /// <p>The log text of the observation.</p>
    #[serde(rename = "LogText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_text: Option<String>,
    /// <p>The name of the observation metric.</p>
    #[serde(rename = "MetricName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    /// <p>The namespace of the observation metric.</p>
    #[serde(rename = "MetricNamespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_namespace: Option<String>,
    /// <p>The source resource ARN of the observation.</p>
    #[serde(rename = "SourceARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
    /// <p>The source type of the observation.</p>
    #[serde(rename = "SourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    /// <p>The time when the observation was first detected, in epoch seconds.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The unit of the source observation metric.</p>
    #[serde(rename = "Unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// <p>The value of the source observation metric.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

/// <p>Describes a problem that is detected by correlating observations.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Problem {
    /// <p>The resource affected by the problem.</p>
    #[serde(rename = "AffectedResource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affected_resource: Option<String>,
    /// <p>The time when the problem ended, in epoch seconds.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>Feedback provided by the user about the problem.</p>
    #[serde(rename = "Feedback")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback: Option<::std::collections::HashMap<String, String>>,
    /// <p>The ID of the problem.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>A detailed analysis of the problem using machine learning.</p>
    #[serde(rename = "Insights")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insights: Option<String>,
    /// <p>The name of the resource group affected by the problem.</p>
    #[serde(rename = "ResourceGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
    /// <p>A measure of the level of impact of the problem.</p>
    #[serde(rename = "SeverityLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_level: Option<String>,
    /// <p>The time when the problem started, in epoch seconds.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The status of the problem.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The name of the problem.</p>
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// <p>Describes observations related to the problem.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RelatedObservations {
    /// <p>The list of observations related to the problem.</p>
    #[serde(rename = "ObservationList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observation_list: Option<Vec<Observation>>,
}

/// <p><p>An object that defines the tags associated with an application. A <i>tag</i> is a label that you optionally define and associate with an application. Tags can help you categorize and manage resources in different ways, such as by purpose, owner, environment, or other criteria.</p> <p>Each tag consists of a required <i>tag key</i> and an associated <i>tag value</i>, both of which you define. A tag key is a general label that acts as a category for a more specific tag value. A tag value acts as a descriptor within a tag key. A tag key can contain as many as 128 characters. A tag value can contain as many as 256 characters. The characters can be Unicode letters, digits, white space, or one of the following symbols: _ . : / = + -. The following additional restrictions apply to tags:</p> <ul> <li> <p>Tag keys and values are case sensitive.</p> </li> <li> <p>For each associated resource, each tag key must be unique and it can have only one value.</p> </li> <li> <p>The <code>aws:</code> prefix is reserved for use by AWS; you can’t use it in any tag keys or values that you define. In addition, you can&#39;t edit or remove tag keys or values that use this prefix. </p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>One part of a key-value pair that defines a tag. The maximum length of a tag key is 128 characters. The minimum length is 1 character.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The optional part of a key-value pair that defines a tag. The maximum length of a tag value is 256 characters. The minimum length is 0 characters. If you don't want an application to have a specific tag value, don't specify a value for this parameter.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the application that you want to add one or more tags to.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    /// <p>A list of tags that to add to the application. A tag consists of a required tag key (<code>Key</code>) and an associated tag value (<code>Value</code>). The maximum length of a tag key is 128 characters. The maximum length of a tag value is 256 characters.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the application that you want to remove one or more tags from.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    /// <p>The tags (tag keys) that you want to remove from the resource. When you specify a tag key, the action removes both that key and its associated tag value.</p> <p>To remove more than one tag from the application, append the <code>TagKeys</code> parameter and argument for each additional tag to remove, separated by an ampersand. </p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateApplicationRequest {
    /// <p> When set to <code>true</code>, creates opsItems for any problems detected on an application. </p>
    #[serde(rename = "OpsCenterEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_center_enabled: Option<bool>,
    /// <p> The SNS topic provided to Application Insights that is associated to the created opsItem. Allows you to receive notifications for updates to the opsItem.</p>
    #[serde(rename = "OpsItemSNSTopicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_item_sns_topic_arn: Option<String>,
    /// <p> Disassociates the SNS topic from the opsItem created for detected problems.</p>
    #[serde(rename = "RemoveSNSTopic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_sns_topic: Option<bool>,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "ResourceGroupName")]
    pub resource_group_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateApplicationResponse {
    /// <p>Information about the application. </p>
    #[serde(rename = "ApplicationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_info: Option<ApplicationInfo>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateComponentConfigurationRequest {
    /// <p>The configuration settings of the component. The value is the escaped JSON of the configuration. For more information about the JSON format, see <a href="https://docs.aws.amazon.com/sdk-for-javascript/v2/developer-guide/working-with-json.html">Working with JSON</a>. You can send a request to <code>DescribeComponentConfigurationRecommendation</code> to see the recommended configuration for a component. For the complete format of the component configuration file, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/component-config.html">Component Configuration</a>.</p>
    #[serde(rename = "ComponentConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_configuration: Option<String>,
    /// <p>The name of the component.</p>
    #[serde(rename = "ComponentName")]
    pub component_name: String,
    /// <p>Indicates whether the application component is monitored.</p>
    #[serde(rename = "Monitor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor: Option<bool>,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "ResourceGroupName")]
    pub resource_group_name: String,
    /// <p>The tier of the application component. Supported tiers include <code>DOT_NET_WORKER</code>, <code>DOT_NET_WEB</code>, <code>DOT_NET_CORE</code>, <code>SQL_SERVER</code>, and <code>DEFAULT</code>.</p>
    #[serde(rename = "Tier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateComponentConfigurationResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateComponentRequest {
    /// <p>The name of the component.</p>
    #[serde(rename = "ComponentName")]
    pub component_name: String,
    /// <p>The new name of the component.</p>
    #[serde(rename = "NewComponentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_component_name: Option<String>,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "ResourceGroupName")]
    pub resource_group_name: String,
    /// <p>The list of resource ARNs that belong to the component.</p>
    #[serde(rename = "ResourceList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_list: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateComponentResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateLogPatternRequest {
    /// <p>The log pattern.</p>
    #[serde(rename = "Pattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    /// <p>The name of the log pattern.</p>
    #[serde(rename = "PatternName")]
    pub pattern_name: String,
    /// <p>The name of the log pattern set.</p>
    #[serde(rename = "PatternSetName")]
    pub pattern_set_name: String,
    /// <p>Rank of the log pattern.</p>
    #[serde(rename = "Rank")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i64>,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "ResourceGroupName")]
    pub resource_group_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateLogPatternResponse {
    /// <p>The successfully created log pattern.</p>
    #[serde(rename = "LogPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_pattern: Option<LogPattern>,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "ResourceGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
}

/// Errors returned by CreateApplication
#[derive(Debug, PartialEq)]
pub enum CreateApplicationError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource is already created or in use.</p>
    ResourceInUse(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
    /// <p>Tags are already registered for the specified application ARN.</p>
    TagsAlreadyExist(String),
}

impl CreateApplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateApplicationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(CreateApplicationError::InternalServer(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(CreateApplicationError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateApplicationError::ResourceNotFound(err.msg))
                }
                "TagsAlreadyExistException" => {
                    return RusotoError::Service(CreateApplicationError::TagsAlreadyExist(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateApplicationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateApplicationError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateApplicationError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateApplicationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateApplicationError::TagsAlreadyExist(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateApplicationError {}
/// Errors returned by CreateComponent
#[derive(Debug, PartialEq)]
pub enum CreateComponentError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource is already created or in use.</p>
    ResourceInUse(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl CreateComponentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateComponentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(CreateComponentError::InternalServer(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(CreateComponentError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateComponentError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateComponentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateComponentError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateComponentError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateComponentError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateComponentError {}
/// Errors returned by CreateLogPattern
#[derive(Debug, PartialEq)]
pub enum CreateLogPatternError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource is already created or in use.</p>
    ResourceInUse(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl CreateLogPatternError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateLogPatternError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(CreateLogPatternError::InternalServer(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(CreateLogPatternError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateLogPatternError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateLogPatternError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateLogPatternError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateLogPatternError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateLogPatternError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateLogPatternError {}
/// Errors returned by DeleteApplication
#[derive(Debug, PartialEq)]
pub enum DeleteApplicationError {
    /// <p>The request is not understood by the server.</p>
    BadRequest(String),
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl DeleteApplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteApplicationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteApplicationError::BadRequest(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DeleteApplicationError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteApplicationError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteApplicationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteApplicationError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteApplicationError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteApplicationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteApplicationError {}
/// Errors returned by DeleteComponent
#[derive(Debug, PartialEq)]
pub enum DeleteComponentError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl DeleteComponentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteComponentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DeleteComponentError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteComponentError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteComponentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteComponentError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteComponentError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteComponentError {}
/// Errors returned by DeleteLogPattern
#[derive(Debug, PartialEq)]
pub enum DeleteLogPatternError {
    /// <p>The request is not understood by the server.</p>
    BadRequest(String),
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl DeleteLogPatternError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteLogPatternError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteLogPatternError::BadRequest(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DeleteLogPatternError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteLogPatternError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteLogPatternError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteLogPatternError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteLogPatternError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteLogPatternError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteLogPatternError {}
/// Errors returned by DescribeApplication
#[derive(Debug, PartialEq)]
pub enum DescribeApplicationError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl DescribeApplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeApplicationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeApplicationError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeApplicationError::ResourceNotFound(
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
impl fmt::Display for DescribeApplicationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeApplicationError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeApplicationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeApplicationError {}
/// Errors returned by DescribeComponent
#[derive(Debug, PartialEq)]
pub enum DescribeComponentError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl DescribeComponentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeComponentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeComponentError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeComponentError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeComponentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeComponentError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeComponentError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeComponentError {}
/// Errors returned by DescribeComponentConfiguration
#[derive(Debug, PartialEq)]
pub enum DescribeComponentConfigurationError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl DescribeComponentConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeComponentConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(
                        DescribeComponentConfigurationError::InternalServer(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeComponentConfigurationError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeComponentConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeComponentConfigurationError::InternalServer(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeComponentConfigurationError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeComponentConfigurationError {}
/// Errors returned by DescribeComponentConfigurationRecommendation
#[derive(Debug, PartialEq)]
pub enum DescribeComponentConfigurationRecommendationError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl DescribeComponentConfigurationRecommendationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeComponentConfigurationRecommendationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(
                        DescribeComponentConfigurationRecommendationError::InternalServer(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeComponentConfigurationRecommendationError::ResourceNotFound(
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
impl fmt::Display for DescribeComponentConfigurationRecommendationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeComponentConfigurationRecommendationError::InternalServer(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeComponentConfigurationRecommendationError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeComponentConfigurationRecommendationError {}
/// Errors returned by DescribeLogPattern
#[derive(Debug, PartialEq)]
pub enum DescribeLogPatternError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl DescribeLogPatternError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeLogPatternError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeLogPatternError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeLogPatternError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeLogPatternError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeLogPatternError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeLogPatternError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeLogPatternError {}
/// Errors returned by DescribeObservation
#[derive(Debug, PartialEq)]
pub enum DescribeObservationError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl DescribeObservationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeObservationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeObservationError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeObservationError::ResourceNotFound(
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
impl fmt::Display for DescribeObservationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeObservationError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeObservationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeObservationError {}
/// Errors returned by DescribeProblem
#[derive(Debug, PartialEq)]
pub enum DescribeProblemError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl DescribeProblemError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeProblemError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeProblemError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeProblemError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeProblemError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeProblemError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeProblemError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeProblemError {}
/// Errors returned by DescribeProblemObservations
#[derive(Debug, PartialEq)]
pub enum DescribeProblemObservationsError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl DescribeProblemObservationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeProblemObservationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeProblemObservationsError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeProblemObservationsError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeProblemObservationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeProblemObservationsError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeProblemObservationsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeProblemObservationsError {}
/// Errors returned by ListApplications
#[derive(Debug, PartialEq)]
pub enum ListApplicationsError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
}

impl ListApplicationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListApplicationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListApplicationsError::InternalServer(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListApplicationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListApplicationsError::InternalServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListApplicationsError {}
/// Errors returned by ListComponents
#[derive(Debug, PartialEq)]
pub enum ListComponentsError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl ListComponentsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListComponentsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListComponentsError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListComponentsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListComponentsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListComponentsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListComponentsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListComponentsError {}
/// Errors returned by ListConfigurationHistory
#[derive(Debug, PartialEq)]
pub enum ListConfigurationHistoryError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl ListConfigurationHistoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListConfigurationHistoryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListConfigurationHistoryError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListConfigurationHistoryError::ResourceNotFound(
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
impl fmt::Display for ListConfigurationHistoryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListConfigurationHistoryError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListConfigurationHistoryError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListConfigurationHistoryError {}
/// Errors returned by ListLogPatternSets
#[derive(Debug, PartialEq)]
pub enum ListLogPatternSetsError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl ListLogPatternSetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListLogPatternSetsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListLogPatternSetsError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListLogPatternSetsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListLogPatternSetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListLogPatternSetsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListLogPatternSetsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListLogPatternSetsError {}
/// Errors returned by ListLogPatterns
#[derive(Debug, PartialEq)]
pub enum ListLogPatternsError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl ListLogPatternsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListLogPatternsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListLogPatternsError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListLogPatternsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListLogPatternsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListLogPatternsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListLogPatternsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListLogPatternsError {}
/// Errors returned by ListProblems
#[derive(Debug, PartialEq)]
pub enum ListProblemsError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl ListProblemsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListProblemsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListProblemsError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListProblemsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListProblemsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListProblemsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListProblemsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListProblemsError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
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
            ListTagsForResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
    /// <p>The number of the provided tags is beyond the limit, or the number of total tags you are trying to attach to the specified resource exceeds the limit.</p>
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
    /// <p>The resource does not exist in the customer account.</p>
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
/// Errors returned by UpdateApplication
#[derive(Debug, PartialEq)]
pub enum UpdateApplicationError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl UpdateApplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateApplicationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(UpdateApplicationError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateApplicationError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateApplicationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateApplicationError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateApplicationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateApplicationError {}
/// Errors returned by UpdateComponent
#[derive(Debug, PartialEq)]
pub enum UpdateComponentError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource is already created or in use.</p>
    ResourceInUse(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl UpdateComponentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateComponentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(UpdateComponentError::InternalServer(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(UpdateComponentError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateComponentError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateComponentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateComponentError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateComponentError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            UpdateComponentError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateComponentError {}
/// Errors returned by UpdateComponentConfiguration
#[derive(Debug, PartialEq)]
pub enum UpdateComponentConfigurationError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl UpdateComponentConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateComponentConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(UpdateComponentConfigurationError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        UpdateComponentConfigurationError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateComponentConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateComponentConfigurationError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateComponentConfigurationError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateComponentConfigurationError {}
/// Errors returned by UpdateLogPattern
#[derive(Debug, PartialEq)]
pub enum UpdateLogPatternError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource is already created or in use.</p>
    ResourceInUse(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl UpdateLogPatternError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateLogPatternError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(UpdateLogPatternError::InternalServer(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(UpdateLogPatternError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateLogPatternError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateLogPatternError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateLogPatternError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateLogPatternError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            UpdateLogPatternError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateLogPatternError {}
/// Trait representing the capabilities of the Application Insights API. Application Insights clients implement this trait.
pub trait ApplicationInsights {
    /// <p>Adds an application that is created from a resource group.</p>
    fn create_application(
        &self,
        input: CreateApplicationRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<CreateApplicationResponse, RusotoError<CreateApplicationError>>,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Creates a custom component by grouping similar standalone instances to monitor.</p>
    fn create_component(
        &self,
        input: CreateComponentRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<CreateComponentResponse, RusotoError<CreateComponentError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Adds an log pattern to a <code>LogPatternSet</code>.</p>
    fn create_log_pattern(
        &self,
        input: CreateLogPatternRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<CreateLogPatternResponse, RusotoError<CreateLogPatternError>>,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Removes the specified application from monitoring. Does not delete the application.</p>
    fn delete_application(
        &self,
        input: DeleteApplicationRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<DeleteApplicationResponse, RusotoError<DeleteApplicationError>>,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Ungroups a custom component. When you ungroup custom components, all applicable monitors that are set up for the component are removed and the instances revert to their standalone status.</p>
    fn delete_component(
        &self,
        input: DeleteComponentRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<DeleteComponentResponse, RusotoError<DeleteComponentError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Removes the specified log pattern from a <code>LogPatternSet</code>.</p>
    fn delete_log_pattern(
        &self,
        input: DeleteLogPatternRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<DeleteLogPatternResponse, RusotoError<DeleteLogPatternError>>,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Describes the application.</p>
    fn describe_application(
        &self,
        input: DescribeApplicationRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeApplicationResponse,
                        RusotoError<DescribeApplicationError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Describes a component and lists the resources that are grouped together in a component.</p>
    fn describe_component(
        &self,
        input: DescribeComponentRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<DescribeComponentResponse, RusotoError<DescribeComponentError>>,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Describes the monitoring configuration of the component.</p>
    fn describe_component_configuration(
        &self,
        input: DescribeComponentConfigurationRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeComponentConfigurationResponse,
                        RusotoError<DescribeComponentConfigurationError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Describes the recommended monitoring configuration of the component.</p>
    fn describe_component_configuration_recommendation(
        &self,
        input: DescribeComponentConfigurationRecommendationRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeComponentConfigurationRecommendationResponse,
                        RusotoError<DescribeComponentConfigurationRecommendationError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Describe a specific log pattern from a <code>LogPatternSet</code>.</p>
    fn describe_log_pattern(
        &self,
        input: DescribeLogPatternRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeLogPatternResponse,
                        RusotoError<DescribeLogPatternError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Describes an anomaly or error with the application.</p>
    fn describe_observation(
        &self,
        input: DescribeObservationRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeObservationResponse,
                        RusotoError<DescribeObservationError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Describes an application problem.</p>
    fn describe_problem(
        &self,
        input: DescribeProblemRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<DescribeProblemResponse, RusotoError<DescribeProblemError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Describes the anomalies or errors associated with the problem.</p>
    fn describe_problem_observations(
        &self,
        input: DescribeProblemObservationsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeProblemObservationsResponse,
                        RusotoError<DescribeProblemObservationsError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Lists the IDs of the applications that you are monitoring. </p>
    fn list_applications(
        &self,
        input: ListApplicationsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<ListApplicationsResponse, RusotoError<ListApplicationsError>>,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Lists the auto-grouped, standalone, and custom components of the application.</p>
    fn list_components(
        &self,
        input: ListComponentsRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<ListComponentsResponse, RusotoError<ListComponentsError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p><p> Lists the INFO, WARN, and ERROR events for periodic configuration updates performed by Application Insights. Examples of events represented are: </p> <ul> <li> <p>INFO: creating a new alarm or updating an alarm threshold.</p> </li> <li> <p>WARN: alarm not created due to insufficient data points used to predict thresholds.</p> </li> <li> <p>ERROR: alarm not created due to permission errors or exceeding quotas. </p> </li> </ul></p>
    fn list_configuration_history(
        &self,
        input: ListConfigurationHistoryRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListConfigurationHistoryResponse,
                        RusotoError<ListConfigurationHistoryError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Lists the log pattern sets in the specific application.</p>
    fn list_log_pattern_sets(
        &self,
        input: ListLogPatternSetsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListLogPatternSetsResponse,
                        RusotoError<ListLogPatternSetsError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Lists the log patterns in the specific log <code>LogPatternSet</code>.</p>
    fn list_log_patterns(
        &self,
        input: ListLogPatternsRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<ListLogPatternsResponse, RusotoError<ListLogPatternsError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Lists the problems with your application.</p>
    fn list_problems(
        &self,
        input: ListProblemsRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<ListProblemsResponse, RusotoError<ListProblemsError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Retrieve a list of the tags (keys and values) that are associated with a specified application. A <i>tag</i> is a label that you optionally define and associate with an application. Each tag consists of a required <i>tag key</i> and an optional associated <i>tag value</i>. A tag key is a general label that acts as a category for more specific tag values. A tag value acts as a descriptor within a tag key.</p>
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

    /// <p>Add one or more tags (keys and values) to a specified application. A <i>tag</i> is a label that you optionally define and associate with an application. Tags can help you categorize and manage application in different ways, such as by purpose, owner, environment, or other criteria. </p> <p>Each tag consists of a required <i>tag key</i> and an associated <i>tag value</i>, both of which you define. A tag key is a general label that acts as a category for more specific tag values. A tag value acts as a descriptor within a tag key.</p>
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

    /// <p>Remove one or more tags (keys and values) from a specified application.</p>
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

    /// <p>Updates the application.</p>
    fn update_application(
        &self,
        input: UpdateApplicationRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<UpdateApplicationResponse, RusotoError<UpdateApplicationError>>,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Updates the custom component name and/or the list of resources that make up the component.</p>
    fn update_component(
        &self,
        input: UpdateComponentRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<UpdateComponentResponse, RusotoError<UpdateComponentError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Updates the monitoring configurations for the component. The configuration input parameter is an escaped JSON of the configuration and should match the schema of what is returned by <code>DescribeComponentConfigurationRecommendation</code>. </p>
    fn update_component_configuration(
        &self,
        input: UpdateComponentConfigurationRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        UpdateComponentConfigurationResponse,
                        RusotoError<UpdateComponentConfigurationError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Adds a log pattern to a <code>LogPatternSet</code>.</p>
    fn update_log_pattern(
        &self,
        input: UpdateLogPatternRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<UpdateLogPatternResponse, RusotoError<UpdateLogPatternError>>,
                > + Send
                + 'static,
        >,
    >;
}
/// A client for the Application Insights API.
#[derive(Clone)]
pub struct ApplicationInsightsClient {
    client: Client,
    region: region::Region,
}

impl ApplicationInsightsClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ApplicationInsightsClient {
        ApplicationInsightsClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ApplicationInsightsClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        ApplicationInsightsClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> ApplicationInsightsClient {
        ApplicationInsightsClient { client, region }
    }
}

impl ApplicationInsights for ApplicationInsightsClient {
    /// <p>Adds an application that is created from a resource group.</p>
    fn create_application(
        &self,
        input: CreateApplicationRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<CreateApplicationResponse, RusotoError<CreateApplicationError>>,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "applicationinsights", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "EC2WindowsBarleyService.CreateApplication");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<CreateApplicationResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(CreateApplicationError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Creates a custom component by grouping similar standalone instances to monitor.</p>
    fn create_component(
        &self,
        input: CreateComponentRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<CreateComponentResponse, RusotoError<CreateComponentError>>>
                + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "applicationinsights", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "EC2WindowsBarleyService.CreateComponent");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<CreateComponentResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(CreateComponentError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Adds an log pattern to a <code>LogPatternSet</code>.</p>
    fn create_log_pattern(
        &self,
        input: CreateLogPatternRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<CreateLogPatternResponse, RusotoError<CreateLogPatternError>>,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "applicationinsights", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "EC2WindowsBarleyService.CreateLogPattern");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<CreateLogPatternResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(CreateLogPatternError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Removes the specified application from monitoring. Does not delete the application.</p>
    fn delete_application(
        &self,
        input: DeleteApplicationRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<DeleteApplicationResponse, RusotoError<DeleteApplicationError>>,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "applicationinsights", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "EC2WindowsBarleyService.DeleteApplication");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<DeleteApplicationResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(DeleteApplicationError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Ungroups a custom component. When you ungroup custom components, all applicable monitors that are set up for the component are removed and the instances revert to their standalone status.</p>
    fn delete_component(
        &self,
        input: DeleteComponentRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<DeleteComponentResponse, RusotoError<DeleteComponentError>>>
                + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "applicationinsights", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "EC2WindowsBarleyService.DeleteComponent");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<DeleteComponentResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(DeleteComponentError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Removes the specified log pattern from a <code>LogPatternSet</code>.</p>
    fn delete_log_pattern(
        &self,
        input: DeleteLogPatternRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<DeleteLogPatternResponse, RusotoError<DeleteLogPatternError>>,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "applicationinsights", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "EC2WindowsBarleyService.DeleteLogPattern");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<DeleteLogPatternResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(DeleteLogPatternError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Describes the application.</p>
    fn describe_application(
        &self,
        input: DescribeApplicationRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeApplicationResponse,
                        RusotoError<DescribeApplicationError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "applicationinsights", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "EC2WindowsBarleyService.DescribeApplication",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<DescribeApplicationResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(DescribeApplicationError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Describes a component and lists the resources that are grouped together in a component.</p>
    fn describe_component(
        &self,
        input: DescribeComponentRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<DescribeComponentResponse, RusotoError<DescribeComponentError>>,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "applicationinsights", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "EC2WindowsBarleyService.DescribeComponent");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<DescribeComponentResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(DescribeComponentError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Describes the monitoring configuration of the component.</p>
    fn describe_component_configuration(
        &self,
        input: DescribeComponentConfigurationRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeComponentConfigurationResponse,
                        RusotoError<DescribeComponentConfigurationError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "applicationinsights", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "EC2WindowsBarleyService.DescribeComponentConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<DescribeComponentConfigurationResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(DescribeComponentConfigurationError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Describes the recommended monitoring configuration of the component.</p>
    fn describe_component_configuration_recommendation(
        &self,
        input: DescribeComponentConfigurationRecommendationRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeComponentConfigurationRecommendationResponse,
                        RusotoError<DescribeComponentConfigurationRecommendationError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "applicationinsights", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "EC2WindowsBarleyService.DescribeComponentConfigurationRecommendation",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<DescribeComponentConfigurationRecommendationResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(DescribeComponentConfigurationRecommendationError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Describe a specific log pattern from a <code>LogPatternSet</code>.</p>
    fn describe_log_pattern(
        &self,
        input: DescribeLogPatternRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeLogPatternResponse,
                        RusotoError<DescribeLogPatternError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "applicationinsights", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "EC2WindowsBarleyService.DescribeLogPattern");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<DescribeLogPatternResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(DescribeLogPatternError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Describes an anomaly or error with the application.</p>
    fn describe_observation(
        &self,
        input: DescribeObservationRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeObservationResponse,
                        RusotoError<DescribeObservationError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "applicationinsights", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "EC2WindowsBarleyService.DescribeObservation",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<DescribeObservationResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(DescribeObservationError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Describes an application problem.</p>
    fn describe_problem(
        &self,
        input: DescribeProblemRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<DescribeProblemResponse, RusotoError<DescribeProblemError>>>
                + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "applicationinsights", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "EC2WindowsBarleyService.DescribeProblem");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<DescribeProblemResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(DescribeProblemError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Describes the anomalies or errors associated with the problem.</p>
    fn describe_problem_observations(
        &self,
        input: DescribeProblemObservationsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeProblemObservationsResponse,
                        RusotoError<DescribeProblemObservationsError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "applicationinsights", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "EC2WindowsBarleyService.DescribeProblemObservations",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<DescribeProblemObservationsResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(DescribeProblemObservationsError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Lists the IDs of the applications that you are monitoring. </p>
    fn list_applications(
        &self,
        input: ListApplicationsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<ListApplicationsResponse, RusotoError<ListApplicationsError>>,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "applicationinsights", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "EC2WindowsBarleyService.ListApplications");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<ListApplicationsResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(ListApplicationsError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Lists the auto-grouped, standalone, and custom components of the application.</p>
    fn list_components(
        &self,
        input: ListComponentsRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<ListComponentsResponse, RusotoError<ListComponentsError>>>
                + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "applicationinsights", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "EC2WindowsBarleyService.ListComponents");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<ListComponentsResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(ListComponentsError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p><p> Lists the INFO, WARN, and ERROR events for periodic configuration updates performed by Application Insights. Examples of events represented are: </p> <ul> <li> <p>INFO: creating a new alarm or updating an alarm threshold.</p> </li> <li> <p>WARN: alarm not created due to insufficient data points used to predict thresholds.</p> </li> <li> <p>ERROR: alarm not created due to permission errors or exceeding quotas. </p> </li> </ul></p>
    fn list_configuration_history(
        &self,
        input: ListConfigurationHistoryRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListConfigurationHistoryResponse,
                        RusotoError<ListConfigurationHistoryError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "applicationinsights", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "EC2WindowsBarleyService.ListConfigurationHistory",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<ListConfigurationHistoryResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(ListConfigurationHistoryError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Lists the log pattern sets in the specific application.</p>
    fn list_log_pattern_sets(
        &self,
        input: ListLogPatternSetsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListLogPatternSetsResponse,
                        RusotoError<ListLogPatternSetsError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "applicationinsights", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "EC2WindowsBarleyService.ListLogPatternSets");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<ListLogPatternSetsResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(ListLogPatternSetsError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Lists the log patterns in the specific log <code>LogPatternSet</code>.</p>
    fn list_log_patterns(
        &self,
        input: ListLogPatternsRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<ListLogPatternsResponse, RusotoError<ListLogPatternsError>>>
                + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "applicationinsights", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "EC2WindowsBarleyService.ListLogPatterns");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<ListLogPatternsResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(ListLogPatternsError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Lists the problems with your application.</p>
    fn list_problems(
        &self,
        input: ListProblemsRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<ListProblemsResponse, RusotoError<ListProblemsError>>>
                + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "applicationinsights", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "EC2WindowsBarleyService.ListProblems");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<ListProblemsResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(ListProblemsError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Retrieve a list of the tags (keys and values) that are associated with a specified application. A <i>tag</i> is a label that you optionally define and associate with an application. Each tag consists of a required <i>tag key</i> and an optional associated <i>tag value</i>. A tag key is a general label that acts as a category for more specific tag values. A tag value acts as a descriptor within a tag key.</p>
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
        let mut request = SignedRequest::new("POST", "applicationinsights", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "EC2WindowsBarleyService.ListTagsForResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
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
        .boxed()
    }

    /// <p>Add one or more tags (keys and values) to a specified application. A <i>tag</i> is a label that you optionally define and associate with an application. Tags can help you categorize and manage application in different ways, such as by purpose, owner, environment, or other criteria. </p> <p>Each tag consists of a required <i>tag key</i> and an associated <i>tag value</i>, both of which you define. A tag key is a general label that acts as a category for more specific tag values. A tag value acts as a descriptor within a tag key.</p>
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
        let mut request = SignedRequest::new("POST", "applicationinsights", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "EC2WindowsBarleyService.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response).deserialize::<TagResourceResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(TagResourceError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Remove one or more tags (keys and values) from a specified application.</p>
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
        let mut request = SignedRequest::new("POST", "applicationinsights", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "EC2WindowsBarleyService.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<UntagResourceResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(UntagResourceError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Updates the application.</p>
    fn update_application(
        &self,
        input: UpdateApplicationRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<UpdateApplicationResponse, RusotoError<UpdateApplicationError>>,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "applicationinsights", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "EC2WindowsBarleyService.UpdateApplication");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<UpdateApplicationResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(UpdateApplicationError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Updates the custom component name and/or the list of resources that make up the component.</p>
    fn update_component(
        &self,
        input: UpdateComponentRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<UpdateComponentResponse, RusotoError<UpdateComponentError>>>
                + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "applicationinsights", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "EC2WindowsBarleyService.UpdateComponent");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<UpdateComponentResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(UpdateComponentError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Updates the monitoring configurations for the component. The configuration input parameter is an escaped JSON of the configuration and should match the schema of what is returned by <code>DescribeComponentConfigurationRecommendation</code>. </p>
    fn update_component_configuration(
        &self,
        input: UpdateComponentConfigurationRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        UpdateComponentConfigurationResponse,
                        RusotoError<UpdateComponentConfigurationError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "applicationinsights", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "EC2WindowsBarleyService.UpdateComponentConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<UpdateComponentConfigurationResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(UpdateComponentConfigurationError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Adds a log pattern to a <code>LogPatternSet</code>.</p>
    fn update_log_pattern(
        &self,
        input: UpdateLogPatternRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<UpdateLogPatternResponse, RusotoError<UpdateLogPatternError>>,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "applicationinsights", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "EC2WindowsBarleyService.UpdateLogPattern");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<UpdateLogPatternResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(UpdateLogPatternError::from_response(response))
            }
        }
        .boxed()
    }
}
