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
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateNotificationRuleRequest {
    /// <p><p>A unique, client-generated idempotency token that, when provided in a request, ensures the request cannot be repeated with a changed parameter. If a request with the same parameters is received and a token is included, the request returns information about the initial request that used that token.</p> <note> <p>The AWS SDKs prepopulate client request tokens. If you are using an AWS SDK, an idempotency token is created for you.</p> </note></p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The level of detail to include in the notifications for this resource. BASIC will include only the contents of the event as it would appear in AWS CloudWatch. FULL will include any supplemental information provided by AWS CodeStar Notifications and/or the service for the resource for which the notification is created.</p>
    #[serde(rename = "DetailType")]
    pub detail_type: String,
    /// <p>A list of event types associated with this notification rule. For a list of allowed events, see <a>EventTypeSummary</a>.</p>
    #[serde(rename = "EventTypeIds")]
    pub event_type_ids: Vec<String>,
    /// <p>The name for the notification rule. Notifictaion rule names must be unique in your AWS account.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The Amazon Resource Name (ARN) of the resource to associate with the notification rule. Supported resources include pipelines in AWS CodePipeline, repositories in AWS CodeCommit, and build projects in AWS CodeBuild.</p>
    #[serde(rename = "Resource")]
    pub resource: String,
    /// <p>The status of the notification rule. The default value is ENABLED. If the status is set to DISABLED, notifications aren't sent for the notification rule.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A list of tags to apply to this notification rule. Key names cannot start with "aws". </p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>A list of Amazon Resource Names (ARNs) of SNS topics to associate with the notification rule.</p>
    #[serde(rename = "Targets")]
    pub targets: Vec<Target>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateNotificationRuleResult {
    /// <p>The Amazon Resource Name (ARN) of the notification rule.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteNotificationRuleRequest {
    /// <p>The Amazon Resource Name (ARN) of the notification rule you want to delete.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteNotificationRuleResult {
    /// <p>The Amazon Resource Name (ARN) of the deleted notification rule.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteTargetRequest {
    /// <p>A Boolean value that can be used to delete all associations with this SNS topic. The default value is FALSE. If set to TRUE, all associations between that target and every notification rule in your AWS account are deleted.</p>
    #[serde(rename = "ForceUnsubscribeAll")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_unsubscribe_all: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) of the SNS topic to delete.</p>
    #[serde(rename = "TargetAddress")]
    pub target_address: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteTargetResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeNotificationRuleRequest {
    /// <p>The Amazon Resource Name (ARN) of the notification rule.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeNotificationRuleResult {
    /// <p>The Amazon Resource Name (ARN) of the notification rule.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>The name or email alias of the person who created the notification rule.</p>
    #[serde(rename = "CreatedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// <p>The date and time the notification rule was created, in timestamp format.</p>
    #[serde(rename = "CreatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    /// <p>The level of detail included in the notifications for this resource. BASIC will include only the contents of the event as it would appear in AWS CloudWatch. FULL will include any supplemental information provided by AWS CodeStar Notifications and/or the service for the resource for which the notification is created.</p>
    #[serde(rename = "DetailType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_type: Option<String>,
    /// <p>A list of the event types associated with the notification rule.</p>
    #[serde(rename = "EventTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_types: Option<Vec<EventTypeSummary>>,
    /// <p>The date and time the notification rule was most recently updated, in timestamp format.</p>
    #[serde(rename = "LastModifiedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_timestamp: Option<f64>,
    /// <p>The name of the notification rule.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the resource associated with the notification rule.</p>
    #[serde(rename = "Resource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    /// <p>The status of the notification rule. Valid statuses are on (sending notifications) or off (not sending notifications).</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The tags associated with the notification rule.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>A list of the SNS topics associated with the notification rule.</p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<TargetSummary>>,
}

/// <p>Returns information about an event that has triggered a notification rule.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EventTypeSummary {
    /// <p>The system-generated ID of the event.</p>
    #[serde(rename = "EventTypeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type_id: Option<String>,
    /// <p>The name of the event.</p>
    #[serde(rename = "EventTypeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type_name: Option<String>,
    /// <p>The resource type of the event.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The name of the service for which the event applies.</p>
    #[serde(rename = "ServiceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
}

/// <p>Information about a filter to apply to the list of returned event types. You can filter by resource type or service name.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListEventTypesFilter {
    /// <p>The system-generated name of the filter type you want to filter by.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The name of the resource type (for example, pipeline) or service name (for example, CodePipeline) that you want to filter by.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListEventTypesRequest {
    /// <p>The filters to use to return information by service or resource type.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ListEventTypesFilter>>,
    /// <p>A non-negative integer used to limit the number of returned results. The default number is 50. The maximum number of results that can be returned is 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An enumeration token that, when provided in a request, returns the next batch of the results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListEventTypesResult {
    /// <p>Information about each event, including service name, resource type, event ID, and event name.</p>
    #[serde(rename = "EventTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_types: Option<Vec<EventTypeSummary>>,
    /// <p>An enumeration token that can be used in a request to return the next batch of the results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Information about a filter to apply to the list of returned notification rules. You can filter by event type, owner, resource, or target.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListNotificationRulesFilter {
    /// <p>The name of the attribute you want to use to filter the returned notification rules.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The value of the attribute you want to use to filter the returned notification rules. For example, if you specify filtering by <i>RESOURCE</i> in Name, you might specify the ARN of a pipeline in AWS CodePipeline for the value.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListNotificationRulesRequest {
    /// <p><p>The filters to use to return information by service or resource type. For valid values, see <a>ListNotificationRulesFilter</a>.</p> <note> <p>A filter with the same name can appear more than once when used with OR statements. Filters with different names should be applied with AND statements.</p> </note></p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ListNotificationRulesFilter>>,
    /// <p>A non-negative integer used to limit the number of returned results. The maximum number of results that can be returned is 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An enumeration token that, when provided in a request, returns the next batch of the results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListNotificationRulesResult {
    /// <p>An enumeration token that can be used in a request to return the next batch of the results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of notification rules for the AWS account, by Amazon Resource Name (ARN) and ID. </p>
    #[serde(rename = "NotificationRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_rules: Option<Vec<NotificationRuleSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The Amazon Resource Name (ARN) for the notification rule.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResult {
    /// <p>The tags associated with the notification rule.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Information about a filter to apply to the list of returned targets. You can filter by target type, address, or status. For example, to filter results to notification rules that have active Amazon SNS topics as targets, you could specify a ListTargetsFilter Name as TargetType and a Value of SNS, and a Name of TARGET_STATUS and a Value of ACTIVE.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTargetsFilter {
    /// <p>The name of the attribute you want to use to filter the returned targets.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The value of the attribute you want to use to filter the returned targets. For example, if you specify <i>SNS</i> for the Target type, you could specify an Amazon Resource Name (ARN) for a topic as the value.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTargetsRequest {
    /// <p><p>The filters to use to return information by service or resource type. Valid filters include target type, target address, and target status.</p> <note> <p>A filter with the same name can appear more than once when used with OR statements. Filters with different names should be applied with AND statements.</p> </note></p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ListTargetsFilter>>,
    /// <p>A non-negative integer used to limit the number of returned results. The maximum number of results that can be returned is 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An enumeration token that, when provided in a request, returns the next batch of the results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTargetsResult {
    /// <p>An enumeration token that can be used in a request to return the next batch of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of notification rule targets. </p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<TargetSummary>>,
}

/// <p>Information about a specified notification rule.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NotificationRuleSummary {
    /// <p>The Amazon Resource Name (ARN) of the notification rule.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The unique ID of the notification rule.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SubscribeRequest {
    /// <p>The Amazon Resource Name (ARN) of the notification rule for which you want to create the association.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>An enumeration token that, when provided in a request, returns the next batch of the results.</p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "Target")]
    pub target: Target,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SubscribeResult {
    /// <p>The Amazon Resource Name (ARN) of the notification rule for which you have created assocations.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the notification rule to tag.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>The list of tags to associate with the resource. Tag key names cannot start with "aws".</p>
    #[serde(rename = "Tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResult {
    /// <p>The list of tags associated with the resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Information about the SNS topics associated with a notification rule.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Target {
    /// <p>The Amazon Resource Name (ARN) of the SNS topic.</p>
    #[serde(rename = "TargetAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_address: Option<String>,
    /// <p>The target type. Can be an Amazon SNS topic.</p>
    #[serde(rename = "TargetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
}

/// <p>Information about the targets specified for a notification rule.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TargetSummary {
    /// <p>The Amazon Resource Name (ARN) of the SNS topic.</p>
    #[serde(rename = "TargetAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_address: Option<String>,
    /// <p>The status of the target.</p>
    #[serde(rename = "TargetStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_status: Option<String>,
    /// <p>The type of the target (for example, SNS).</p>
    #[serde(rename = "TargetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UnsubscribeRequest {
    /// <p>The Amazon Resource Name (ARN) of the notification rule.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>The ARN of the SNS topic to unsubscribe from the notification rule.</p>
    #[serde(rename = "TargetAddress")]
    pub target_address: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UnsubscribeResult {
    /// <p>The Amazon Resource Name (ARN) of the the notification rule from which you have removed a subscription.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the notification rule from which to remove the tags.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>The key names of the tags to remove.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateNotificationRuleRequest {
    /// <p>The Amazon Resource Name (ARN) of the notification rule.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>The level of detail to include in the notifications for this resource. BASIC will include only the contents of the event as it would appear in AWS CloudWatch. FULL will include any supplemental information provided by AWS CodeStar Notifications and/or the service for the resource for which the notification is created.</p>
    #[serde(rename = "DetailType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_type: Option<String>,
    /// <p>A list of event types associated with this notification rule.</p>
    #[serde(rename = "EventTypeIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type_ids: Option<Vec<String>>,
    /// <p>The name of the notification rule.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The status of the notification rule. Valid statuses include enabled (sending notifications) or disabled (not sending notifications).</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The address and type of the targets to receive notifications from this notification rule.</p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateNotificationRuleResult {}

/// Errors returned by CreateNotificationRule
#[derive(Debug, PartialEq)]
pub enum CreateNotificationRuleError {
    /// <p>AWS CodeStar Notifications can't create the notification rule because you do not have sufficient permissions.</p>
    AccessDenied(String),
    /// <p>AWS CodeStar Notifications can't complete the request because the resource is being modified by another process. Wait a few minutes and try again.</p>
    ConcurrentModification(String),
    /// <p>Some or all of the configuration is incomplete, missing, or not valid.</p>
    Configuration(String),
    /// <p>One of the AWS CodeStar Notifications limits has been exceeded. Limits apply to accounts, notification rules, notifications, resources, and targets. For more information, see Limits.</p>
    LimitExceeded(String),
    /// <p>A resource with the same name or ID already exists. Notification rule names must be unique in your AWS account.</p>
    ResourceAlreadyExists(String),
}

impl CreateNotificationRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateNotificationRuleError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateNotificationRuleError::AccessDenied(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        CreateNotificationRuleError::ConcurrentModification(err.msg),
                    )
                }
                "ConfigurationException" => {
                    return RusotoError::Service(CreateNotificationRuleError::Configuration(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateNotificationRuleError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(
                        CreateNotificationRuleError::ResourceAlreadyExists(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateNotificationRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateNotificationRuleError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateNotificationRuleError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateNotificationRuleError::Configuration(ref cause) => write!(f, "{}", cause),
            CreateNotificationRuleError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateNotificationRuleError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateNotificationRuleError {}
/// Errors returned by DeleteNotificationRule
#[derive(Debug, PartialEq)]
pub enum DeleteNotificationRuleError {
    /// <p>AWS CodeStar Notifications can't complete the request because the resource is being modified by another process. Wait a few minutes and try again.</p>
    ConcurrentModification(String),
    /// <p>One of the AWS CodeStar Notifications limits has been exceeded. Limits apply to accounts, notification rules, notifications, resources, and targets. For more information, see Limits.</p>
    LimitExceeded(String),
}

impl DeleteNotificationRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteNotificationRuleError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        DeleteNotificationRuleError::ConcurrentModification(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteNotificationRuleError::LimitExceeded(
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
impl fmt::Display for DeleteNotificationRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteNotificationRuleError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteNotificationRuleError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteNotificationRuleError {}
/// Errors returned by DeleteTarget
#[derive(Debug, PartialEq)]
pub enum DeleteTargetError {}

impl DeleteTargetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteTargetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteTargetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DeleteTargetError {}
/// Errors returned by DescribeNotificationRule
#[derive(Debug, PartialEq)]
pub enum DescribeNotificationRuleError {
    /// <p>AWS CodeStar Notifications can't find a resource that matches the provided ARN. </p>
    ResourceNotFound(String),
}

impl DescribeNotificationRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeNotificationRuleError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeNotificationRuleError::ResourceNotFound(
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
impl fmt::Display for DescribeNotificationRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeNotificationRuleError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeNotificationRuleError {}
/// Errors returned by ListEventTypes
#[derive(Debug, PartialEq)]
pub enum ListEventTypesError {
    /// <p>The value for the enumeration token used in the request to return the next batch of the results is not valid. </p>
    InvalidNextToken(String),
}

impl ListEventTypesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListEventTypesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListEventTypesError::InvalidNextToken(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListEventTypesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListEventTypesError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListEventTypesError {}
/// Errors returned by ListNotificationRules
#[derive(Debug, PartialEq)]
pub enum ListNotificationRulesError {
    /// <p>The value for the enumeration token used in the request to return the next batch of the results is not valid. </p>
    InvalidNextToken(String),
}

impl ListNotificationRulesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListNotificationRulesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListNotificationRulesError::InvalidNextToken(
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
impl fmt::Display for ListNotificationRulesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListNotificationRulesError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListNotificationRulesError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>AWS CodeStar Notifications can't find a resource that matches the provided ARN. </p>
    ResourceNotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
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
/// Errors returned by ListTargets
#[derive(Debug, PartialEq)]
pub enum ListTargetsError {
    /// <p>The value for the enumeration token used in the request to return the next batch of the results is not valid. </p>
    InvalidNextToken(String),
}

impl ListTargetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTargetsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListTargetsError::InvalidNextToken(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTargetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTargetsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTargetsError {}
/// Errors returned by Subscribe
#[derive(Debug, PartialEq)]
pub enum SubscribeError {
    /// <p>AWS CodeStar Notifications can't find a resource that matches the provided ARN. </p>
    ResourceNotFound(String),
}

impl SubscribeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SubscribeError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(SubscribeError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SubscribeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SubscribeError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SubscribeError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>AWS CodeStar Notifications can't complete the request because the resource is being modified by another process. Wait a few minutes and try again.</p>
    ConcurrentModification(String),
    /// <p>AWS CodeStar Notifications can't find a resource that matches the provided ARN. </p>
    ResourceNotFound(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(TagResourceError::ConcurrentModification(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
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
            TagResourceError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by Unsubscribe
#[derive(Debug, PartialEq)]
pub enum UnsubscribeError {}

impl UnsubscribeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UnsubscribeError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UnsubscribeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for UnsubscribeError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>AWS CodeStar Notifications can't complete the request because the resource is being modified by another process. Wait a few minutes and try again.</p>
    ConcurrentModification(String),
    /// <p>AWS CodeStar Notifications can't find a resource that matches the provided ARN. </p>
    ResourceNotFound(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UntagResourceError::ConcurrentModification(
                        err.msg,
                    ))
                }
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
            UntagResourceError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateNotificationRule
#[derive(Debug, PartialEq)]
pub enum UpdateNotificationRuleError {
    /// <p>AWS CodeStar Notifications can't find a resource that matches the provided ARN. </p>
    ResourceNotFound(String),
}

impl UpdateNotificationRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateNotificationRuleError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateNotificationRuleError::ResourceNotFound(
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
impl fmt::Display for UpdateNotificationRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateNotificationRuleError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateNotificationRuleError {}
/// Trait representing the capabilities of the AWS CodeStar Notifications API. AWS CodeStar Notifications clients implement this trait.
#[async_trait]
pub trait CodeStarNotifications {
    /// <p>Creates a notification rule for a resource. The rule specifies the events you want notifications about and the targets (such as SNS topics) where you want to receive them.</p>
    async fn create_notification_rule(
        &self,
        input: CreateNotificationRuleRequest,
    ) -> Result<CreateNotificationRuleResult, RusotoError<CreateNotificationRuleError>>;

    /// <p>Deletes a notification rule for a resource.</p>
    async fn delete_notification_rule(
        &self,
        input: DeleteNotificationRuleRequest,
    ) -> Result<DeleteNotificationRuleResult, RusotoError<DeleteNotificationRuleError>>;

    /// <p>Deletes a specified target for notifications.</p>
    async fn delete_target(
        &self,
        input: DeleteTargetRequest,
    ) -> Result<DeleteTargetResult, RusotoError<DeleteTargetError>>;

    /// <p>Returns information about a specified notification rule.</p>
    async fn describe_notification_rule(
        &self,
        input: DescribeNotificationRuleRequest,
    ) -> Result<DescribeNotificationRuleResult, RusotoError<DescribeNotificationRuleError>>;

    /// <p>Returns information about the event types available for configuring notifications.</p>
    async fn list_event_types(
        &self,
        input: ListEventTypesRequest,
    ) -> Result<ListEventTypesResult, RusotoError<ListEventTypesError>>;

    /// <p>Returns a list of the notification rules for an AWS account.</p>
    async fn list_notification_rules(
        &self,
        input: ListNotificationRulesRequest,
    ) -> Result<ListNotificationRulesResult, RusotoError<ListNotificationRulesError>>;

    /// <p>Returns a list of the tags associated with a notification rule.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResult, RusotoError<ListTagsForResourceError>>;

    /// <p>Returns a list of the notification rule targets for an AWS account.</p>
    async fn list_targets(
        &self,
        input: ListTargetsRequest,
    ) -> Result<ListTargetsResult, RusotoError<ListTargetsError>>;

    /// <p>Creates an association between a notification rule and an SNS topic so that the associated target can receive notifications when the events described in the rule are triggered.</p>
    async fn subscribe(
        &self,
        input: SubscribeRequest,
    ) -> Result<SubscribeResult, RusotoError<SubscribeError>>;

    /// <p>Associates a set of provided tags with a notification rule.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResult, RusotoError<TagResourceError>>;

    /// <p>Removes an association between a notification rule and an Amazon SNS topic so that subscribers to that topic stop receiving notifications when the events described in the rule are triggered.</p>
    async fn unsubscribe(
        &self,
        input: UnsubscribeRequest,
    ) -> Result<UnsubscribeResult, RusotoError<UnsubscribeError>>;

    /// <p>Removes the association between one or more provided tags and a notification rule.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResult, RusotoError<UntagResourceError>>;

    /// <p><p>Updates a notification rule for a resource. You can change the events that trigger the notification rule, the status of the rule, and the targets that receive the notifications.</p> <note> <p>To add or remove tags for a notification rule, you must use <a>TagResource</a> and <a>UntagResource</a>.</p> </note></p>
    async fn update_notification_rule(
        &self,
        input: UpdateNotificationRuleRequest,
    ) -> Result<UpdateNotificationRuleResult, RusotoError<UpdateNotificationRuleError>>;
}
/// A client for the AWS CodeStar Notifications API.
#[derive(Clone)]
pub struct CodeStarNotificationsClient {
    client: Client,
    region: region::Region,
}

impl CodeStarNotificationsClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> CodeStarNotificationsClient {
        CodeStarNotificationsClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> CodeStarNotificationsClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        CodeStarNotificationsClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> CodeStarNotificationsClient {
        CodeStarNotificationsClient { client, region }
    }
}

#[async_trait]
impl CodeStarNotifications for CodeStarNotificationsClient {
    /// <p>Creates a notification rule for a resource. The rule specifies the events you want notifications about and the targets (such as SNS topics) where you want to receive them.</p>
    async fn create_notification_rule(
        &self,
        input: CreateNotificationRuleRequest,
    ) -> Result<CreateNotificationRuleResult, RusotoError<CreateNotificationRuleError>> {
        let request_uri = "/createNotificationRule";

        let mut request =
            SignedRequest::new("POST", "codestar-notifications", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateNotificationRuleResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateNotificationRuleError::from_response(response))
        }
    }

    /// <p>Deletes a notification rule for a resource.</p>
    async fn delete_notification_rule(
        &self,
        input: DeleteNotificationRuleRequest,
    ) -> Result<DeleteNotificationRuleResult, RusotoError<DeleteNotificationRuleError>> {
        let request_uri = "/deleteNotificationRule";

        let mut request =
            SignedRequest::new("POST", "codestar-notifications", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteNotificationRuleResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteNotificationRuleError::from_response(response))
        }
    }

    /// <p>Deletes a specified target for notifications.</p>
    async fn delete_target(
        &self,
        input: DeleteTargetRequest,
    ) -> Result<DeleteTargetResult, RusotoError<DeleteTargetError>> {
        let request_uri = "/deleteTarget";

        let mut request =
            SignedRequest::new("POST", "codestar-notifications", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteTargetResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteTargetError::from_response(response))
        }
    }

    /// <p>Returns information about a specified notification rule.</p>
    async fn describe_notification_rule(
        &self,
        input: DescribeNotificationRuleRequest,
    ) -> Result<DescribeNotificationRuleResult, RusotoError<DescribeNotificationRuleError>> {
        let request_uri = "/describeNotificationRule";

        let mut request =
            SignedRequest::new("POST", "codestar-notifications", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeNotificationRuleResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeNotificationRuleError::from_response(response))
        }
    }

    /// <p>Returns information about the event types available for configuring notifications.</p>
    async fn list_event_types(
        &self,
        input: ListEventTypesRequest,
    ) -> Result<ListEventTypesResult, RusotoError<ListEventTypesError>> {
        let request_uri = "/listEventTypes";

        let mut request =
            SignedRequest::new("POST", "codestar-notifications", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListEventTypesResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListEventTypesError::from_response(response))
        }
    }

    /// <p>Returns a list of the notification rules for an AWS account.</p>
    async fn list_notification_rules(
        &self,
        input: ListNotificationRulesRequest,
    ) -> Result<ListNotificationRulesResult, RusotoError<ListNotificationRulesError>> {
        let request_uri = "/listNotificationRules";

        let mut request =
            SignedRequest::new("POST", "codestar-notifications", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListNotificationRulesResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListNotificationRulesError::from_response(response))
        }
    }

    /// <p>Returns a list of the tags associated with a notification rule.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResult, RusotoError<ListTagsForResourceError>> {
        let request_uri = "/listTagsForResource";

        let mut request =
            SignedRequest::new("POST", "codestar-notifications", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTagsForResourceResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p>Returns a list of the notification rule targets for an AWS account.</p>
    async fn list_targets(
        &self,
        input: ListTargetsRequest,
    ) -> Result<ListTargetsResult, RusotoError<ListTargetsError>> {
        let request_uri = "/listTargets";

        let mut request =
            SignedRequest::new("POST", "codestar-notifications", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTargetsResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTargetsError::from_response(response))
        }
    }

    /// <p>Creates an association between a notification rule and an SNS topic so that the associated target can receive notifications when the events described in the rule are triggered.</p>
    async fn subscribe(
        &self,
        input: SubscribeRequest,
    ) -> Result<SubscribeResult, RusotoError<SubscribeError>> {
        let request_uri = "/subscribe";

        let mut request =
            SignedRequest::new("POST", "codestar-notifications", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<SubscribeResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(SubscribeError::from_response(response))
        }
    }

    /// <p>Associates a set of provided tags with a notification rule.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResult, RusotoError<TagResourceError>> {
        let request_uri = "/tagResource";

        let mut request =
            SignedRequest::new("POST", "codestar-notifications", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<TagResourceResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Removes an association between a notification rule and an Amazon SNS topic so that subscribers to that topic stop receiving notifications when the events described in the rule are triggered.</p>
    async fn unsubscribe(
        &self,
        input: UnsubscribeRequest,
    ) -> Result<UnsubscribeResult, RusotoError<UnsubscribeError>> {
        let request_uri = "/unsubscribe";

        let mut request =
            SignedRequest::new("POST", "codestar-notifications", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UnsubscribeResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UnsubscribeError::from_response(response))
        }
    }

    /// <p>Removes the association between one or more provided tags and a notification rule.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResult, RusotoError<UntagResourceError>> {
        let request_uri = "/untagResource";

        let mut request =
            SignedRequest::new("POST", "codestar-notifications", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UntagResourceResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p><p>Updates a notification rule for a resource. You can change the events that trigger the notification rule, the status of the rule, and the targets that receive the notifications.</p> <note> <p>To add or remove tags for a notification rule, you must use <a>TagResource</a> and <a>UntagResource</a>.</p> </note></p>
    async fn update_notification_rule(
        &self,
        input: UpdateNotificationRuleRequest,
    ) -> Result<UpdateNotificationRuleResult, RusotoError<UpdateNotificationRuleError>> {
        let request_uri = "/updateNotificationRule";

        let mut request =
            SignedRequest::new("POST", "codestar-notifications", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateNotificationRuleResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateNotificationRuleError::from_response(response))
        }
    }
}
