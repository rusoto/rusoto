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
/// <p>Represents the output of the <code>CreateBudget</code> operation. The content consists of the detailed metadata and data file information, and the current status of the <code>budget</code>.</p> <p>The ARN pattern for a budget is: <code>arn:aws:budgetservice::AccountId:budget/budgetName</code> </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Budget {
    /// <p>The total amount of cost, usage, or RI utilization that you want to track with your budget.</p> <p> <code>BudgetLimit</code> is required for cost or usage budgets, but optional for RI utilization budgets. RI utilization budgets default to the only valid value for RI utilization budgets, which is <code>100</code>.</p>
    #[serde(rename = "BudgetLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_limit: Option<Spend>,
    /// <p>The name of a budget. Unique within accounts. <code>:</code> and <code>&bsol;</code> characters are not allowed in the <code>BudgetName</code>.</p>
    #[serde(rename = "BudgetName")]
    pub budget_name: String,
    /// <p>Whether this budget tracks monetary costs, usage, or RI utilization.</p>
    #[serde(rename = "BudgetType")]
    pub budget_type: String,
    /// <p>The actual and forecasted cost or usage being tracked by a budget.</p>
    #[serde(rename = "CalculatedSpend")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculated_spend: Option<CalculatedSpend>,
    /// <p>The cost filters applied to a budget, such as service or region.</p>
    #[serde(rename = "CostFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_filters: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The types of costs included in this budget.</p>
    #[serde(rename = "CostTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_types: Option<CostTypes>,
    /// <p>The period of time covered by a budget. Has a start date and an end date. The start date must come before the end date. There are no restrictions on the end date. </p> <p>If you created your budget and didn't specify a start date, AWS defaults to the start of your chosen time period (i.e. DAILY, MONTHLY, QUARTERLY, ANNUALLY). For example, if you created your budget on January 24th 2018, chose <code>DAILY</code>, and didn't set a start date, AWS set your start date to <code>01/24/18 00:00 UTC</code>. If you chose <code>MONTHLY</code>, AWS set your start date to <code>01/01/18 00:00 UTC</code>. If you didn't specify an end date, AWS set your end date to <code>06/15/87 00:00 UTC</code>. The defaults are the same for the AWS Billing and Cost Management console and the API. </p> <p>You can change either date with the <code>UpdateBudget</code> operation.</p> <p>After the end date, AWS deletes the budget and all associated notifications and subscribers.</p>
    #[serde(rename = "TimePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_period: Option<TimePeriod>,
    /// <p>The length of time until a budget resets the actual and forecasted spend.</p>
    #[serde(rename = "TimeUnit")]
    pub time_unit: String,
}

/// <p>The spend objects associated with this budget. The <code>actualSpend</code> tracks how much you've used, cost, usage, or RI units, and the <code>forecastedSpend</code> tracks how much you are predicted to spend if your current usage remains steady.</p> <p>For example, if it is the 20th of the month and you have spent <code>50</code> dollars on Amazon EC2, your <code>actualSpend</code> is <code>50 USD</code>, and your <code>forecastedSpend</code> is <code>75 USD</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CalculatedSpend {
    /// <p>The amount of cost, usage, or RI units that you have used.</p>
    #[serde(rename = "ActualSpend")]
    pub actual_spend: Spend,
    /// <p>The amount of cost, usage, or RI units that you are forecasted to use.</p>
    #[serde(rename = "ForecastedSpend")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecasted_spend: Option<Spend>,
}

/// <p>The types of cost included in a budget, such as tax and subscriptions.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CostTypes {
    /// <p>Specifies whether a budget includes credits.</p> <p>The default value is <code>true</code>.</p>
    #[serde(rename = "IncludeCredit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_credit: Option<bool>,
    /// <p>Specifies whether a budget includes discounts.</p> <p>The default value is <code>true</code>.</p>
    #[serde(rename = "IncludeDiscount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_discount: Option<bool>,
    /// <p>Specifies whether a budget includes non-RI subscription costs.</p> <p>The default value is <code>true</code>.</p>
    #[serde(rename = "IncludeOtherSubscription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_other_subscription: Option<bool>,
    /// <p>Specifies whether a budget includes recurring fees such as monthly RI fees.</p> <p>The default value is <code>true</code>.</p>
    #[serde(rename = "IncludeRecurring")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_recurring: Option<bool>,
    /// <p>Specifies whether a budget includes refunds.</p> <p>The default value is <code>true</code>.</p>
    #[serde(rename = "IncludeRefund")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_refund: Option<bool>,
    /// <p>Specifies whether a budget includes subscriptions.</p> <p>The default value is <code>true</code>.</p>
    #[serde(rename = "IncludeSubscription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_subscription: Option<bool>,
    /// <p>Specifies whether a budget includes support subscription fees.</p> <p>The default value is <code>true</code>.</p>
    #[serde(rename = "IncludeSupport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_support: Option<bool>,
    /// <p>Specifies whether a budget includes taxes.</p> <p>The default value is <code>true</code>.</p>
    #[serde(rename = "IncludeTax")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_tax: Option<bool>,
    /// <p>Specifies whether a budget includes upfront RI costs.</p> <p>The default value is <code>true</code>.</p>
    #[serde(rename = "IncludeUpfront")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_upfront: Option<bool>,
    /// <p>Specifies whether a budget uses the amortized rate.</p> <p>The default value is <code>false</code>.</p>
    #[serde(rename = "UseAmortized")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_amortized: Option<bool>,
    /// <p>Specifies whether a budget uses blended rate.</p> <p>The default value is <code>false</code>.</p>
    #[serde(rename = "UseBlended")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_blended: Option<bool>,
}

/// <p> Request of CreateBudget </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateBudgetRequest {
    /// <p>The <code>accountId</code> that is associated with the budget.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The budget object that you want to create.</p>
    #[serde(rename = "Budget")]
    pub budget: Budget,
    /// <p>A notification that you want to associate with a budget. A budget can have up to five notifications, and each notification can have one SNS subscriber and up to ten email subscribers. If you include notifications and subscribers in your <code>CreateBudget</code> call, AWS creates the notifications and subscribers for you.</p>
    #[serde(rename = "NotificationsWithSubscribers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications_with_subscribers: Option<Vec<NotificationWithSubscribers>>,
}

/// <p> Response of CreateBudget </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateBudgetResponse {}

/// <p> Request of CreateNotification </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateNotificationRequest {
    /// <p>The <code>accountId</code> that is associated with the budget that you want to create a notification for.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The name of the budget that you want AWS to notified you about. Budget names must be unique within an account.</p>
    #[serde(rename = "BudgetName")]
    pub budget_name: String,
    /// <p>The notification that you want to create.</p>
    #[serde(rename = "Notification")]
    pub notification: Notification,
    /// <p>A list of subscribers that you want to associate with the notification. Each notification can have one SNS subscriber and up to ten email subscribers.</p>
    #[serde(rename = "Subscribers")]
    pub subscribers: Vec<Subscriber>,
}

/// <p> Response of CreateNotification </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateNotificationResponse {}

/// <p> Request of CreateSubscriber </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateSubscriberRequest {
    /// <p>The <code>accountId</code> associated with the budget that you want to create a subscriber for.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The name of the budget that you want to subscribe to. Budget names must be unique within an account.</p>
    #[serde(rename = "BudgetName")]
    pub budget_name: String,
    /// <p>The notification that you want to create a subscriber for.</p>
    #[serde(rename = "Notification")]
    pub notification: Notification,
    /// <p>The subscriber that you want to associate with a budget notification.</p>
    #[serde(rename = "Subscriber")]
    pub subscriber: Subscriber,
}

/// <p> Response of CreateSubscriber </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateSubscriberResponse {}

/// <p> Request of DeleteBudget </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteBudgetRequest {
    /// <p>The <code>accountId</code> that is associated with the budget that you want to delete.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The name of the budget that you want to delete.</p>
    #[serde(rename = "BudgetName")]
    pub budget_name: String,
}

/// <p> Response of DeleteBudget </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteBudgetResponse {}

/// <p> Request of DeleteNotification </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteNotificationRequest {
    /// <p>The <code>accountId</code> that is associated with the budget whose notification you want to delete.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The name of the budget whose notification you want to delete.</p>
    #[serde(rename = "BudgetName")]
    pub budget_name: String,
    /// <p>The notification that you want to delete.</p>
    #[serde(rename = "Notification")]
    pub notification: Notification,
}

/// <p> Response of DeleteNotification </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteNotificationResponse {}

/// <p> Request of DeleteSubscriber </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteSubscriberRequest {
    /// <p>The <code>accountId</code> that is associated with the budget whose subscriber you want to delete.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The name of the budget whose subscriber you want to delete.</p>
    #[serde(rename = "BudgetName")]
    pub budget_name: String,
    /// <p>The notification whose subscriber you want to delete.</p>
    #[serde(rename = "Notification")]
    pub notification: Notification,
    /// <p>The subscriber that you want to delete.</p>
    #[serde(rename = "Subscriber")]
    pub subscriber: Subscriber,
}

/// <p> Response of DeleteSubscriber </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteSubscriberResponse {}

/// <p> Request of DescribeBudget </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeBudgetRequest {
    /// <p>The <code>accountId</code> that is associated with the budget that you want a description of.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The name of the budget that you want a description of.</p>
    #[serde(rename = "BudgetName")]
    pub budget_name: String,
}

/// <p> Response of DescribeBudget </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeBudgetResponse {
    /// <p>The description of the budget.</p>
    #[serde(rename = "Budget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget: Option<Budget>,
}

/// <p> Request of DescribeBudgets </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeBudgetsRequest {
    /// <p>The <code>accountId</code> that is associated with the budgets that you want descriptions of.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>Optional integer. Specifies the maximum number of results to return in response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token that indicates the next set of results to retrieve.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p> Response of DescribeBudgets </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeBudgetsResponse {
    /// <p>A list of budgets.</p>
    #[serde(rename = "Budgets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budgets: Option<Vec<Budget>>,
    /// <p>The pagination token that indicates the next set of results that you can retrieve.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p> Request of DescribeNotificationsForBudget </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeNotificationsForBudgetRequest {
    /// <p>The <code>accountId</code> that is associated with the budget whose notifications you want descriptions of.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The name of the budget whose notifications you want descriptions of.</p>
    #[serde(rename = "BudgetName")]
    pub budget_name: String,
    /// <p>Optional integer. Specifies the maximum number of results to return in response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token that indicates the next set of results to retrieve.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p> Response of GetNotificationsForBudget </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeNotificationsForBudgetResponse {
    /// <p>The pagination token that indicates the next set of results that you can retrieve.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of notifications associated with a budget.</p>
    #[serde(rename = "Notifications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications: Option<Vec<Notification>>,
}

/// <p> Request of DescribeSubscribersForNotification </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeSubscribersForNotificationRequest {
    /// <p>The <code>accountId</code> that is associated with the budget whose subscribers you want descriptions of.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The name of the budget whose subscribers you want descriptions of.</p>
    #[serde(rename = "BudgetName")]
    pub budget_name: String,
    /// <p>Optional integer. Specifies the maximum number of results to return in response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token that indicates the next set of results to retrieve.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The notification whose subscribers you want to list.</p>
    #[serde(rename = "Notification")]
    pub notification: Notification,
}

/// <p> Response of DescribeSubscribersForNotification </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeSubscribersForNotificationResponse {
    /// <p>The pagination token that indicates the next set of results that you can retrieve.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of subscribers associated with a notification.</p>
    #[serde(rename = "Subscribers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers: Option<Vec<Subscriber>>,
}

/// <p><p>A notification associated with a budget. A budget can have up to five notifications. </p> <p>Each notification must have at least one subscriber. A notification can have one SNS subscriber and up to ten email subscribers, for a total of 11 subscribers.</p> <p>For example, if you have a budget for 200 dollars and you want to be notified when you go over 160 dollars, create a notification with the following parameters:</p> <ul> <li> <p>A notificationType of <code>ACTUAL</code> </p> </li> <li> <p>A comparisonOperator of <code>GREATER_THAN</code> </p> </li> <li> <p>A notification threshold of <code>80</code> </p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Notification {
    /// <p>The comparison used for this notification.</p>
    #[serde(rename = "ComparisonOperator")]
    pub comparison_operator: String,
    /// <p>Whether the notification is for how much you have spent (<code>ACTUAL</code>) or for how much you are forecasted to spend (<code>FORECASTED</code>).</p>
    #[serde(rename = "NotificationType")]
    pub notification_type: String,
    /// <p>The threshold associated with a notification. Thresholds are always a percentage.</p>
    #[serde(rename = "Threshold")]
    pub threshold: f64,
    /// <p>The type of threshold for a notification. For <code>ACTUAL</code> thresholds, AWS notifies you when you go over the threshold, and for <code>FORECASTED</code> thresholds AWS notifies you when you are forecasted to go over the threshold.</p>
    #[serde(rename = "ThresholdType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_type: Option<String>,
}

/// <p>A notification with subscribers. A notification can have one SNS subscriber and up to ten email subscribers, for a total of 11 subscribers.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct NotificationWithSubscribers {
    /// <p>The notification associated with a budget.</p>
    #[serde(rename = "Notification")]
    pub notification: Notification,
    /// <p>A list of subscribers who are subscribed to this notification.</p>
    #[serde(rename = "Subscribers")]
    pub subscribers: Vec<Subscriber>,
}

/// <p><p>The amount of cost or usage being measured for a budget.</p> <p>For example, a <code>Spend</code> for <code>3 GB</code> of S3 usage would have the following parameters:</p> <ul> <li> <p>An <code>Amount</code> of <code>3</code> </p> </li> <li> <p>A <code>unit</code> of <code>GB</code> </p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Spend {
    /// <p>The cost or usage amount associated with a budget forecast, actual spend, or budget threshold.</p>
    #[serde(rename = "Amount")]
    pub amount: String,
    /// <p>The unit of measurement used for the budget forecast, actual spend, or budget threshold, such as dollars or GB.</p>
    #[serde(rename = "Unit")]
    pub unit: String,
}

/// <p><p>The subscriber to a budget notification. The subscriber consists of a subscription type and either an Amazon Simple Notification Service topic or an email address.</p> <p>For example, an email subscriber would have the following parameters:</p> <ul> <li> <p>A <code>subscriptionType</code> of <code>EMAIL</code> </p> </li> <li> <p>An <code>address</code> of <code>example@example.com</code> </p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Subscriber {
    /// <p>The address that AWS sends budget notifications to, either an SNS topic or an email.</p>
    #[serde(rename = "Address")]
    pub address: String,
    /// <p>The type of notification that AWS sends to a subscriber.</p>
    #[serde(rename = "SubscriptionType")]
    pub subscription_type: String,
}

/// <p>The period of time covered by a budget. Has a start date and an end date. The start date must come before the end date. There are no restrictions on the end date. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimePeriod {
    /// <p>The end date for a budget. If you didn't specify an end date, AWS set your end date to <code>06/15/87 00:00 UTC</code>. The defaults are the same for the AWS Billing and Cost Management console and the API.</p> <p>After the end date, AWS deletes the budget and all associated notifications and subscribers. You can change your end date with the <code>UpdateBudget</code> operation.</p>
    #[serde(rename = "End")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<f64>,
    /// <p>The start date for a budget. If you created your budget and didn't specify a start date, AWS defaults to the start of your chosen time period (i.e. DAILY, MONTHLY, QUARTERLY, ANNUALLY). For example, if you created your budget on January 24th 2018, chose <code>DAILY</code>, and didn't set a start date, AWS set your start date to <code>01/24/18 00:00 UTC</code>. If you chose <code>MONTHLY</code>, AWS set your start date to <code>01/01/18 00:00 UTC</code>. The defaults are the same for the AWS Billing and Cost Management console and the API.</p> <p>You can change your start date with the <code>UpdateBudget</code> operation.</p>
    #[serde(rename = "Start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}

/// <p> Request of UpdateBudget </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateBudgetRequest {
    /// <p>The <code>accountId</code> that is associated with the budget that you want to update.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The budget that you want to update your budget to.</p>
    #[serde(rename = "NewBudget")]
    pub new_budget: Budget,
}

/// <p> Response of UpdateBudget </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateBudgetResponse {}

/// <p> Request of UpdateNotification </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateNotificationRequest {
    /// <p>The <code>accountId</code> that is associated with the budget whose notification you want to update.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The name of the budget whose notification you want to update.</p>
    #[serde(rename = "BudgetName")]
    pub budget_name: String,
    /// <p>The updated notification to be associated with a budget.</p>
    #[serde(rename = "NewNotification")]
    pub new_notification: Notification,
    /// <p>The previous notification associated with a budget.</p>
    #[serde(rename = "OldNotification")]
    pub old_notification: Notification,
}

/// <p> Response of UpdateNotification </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateNotificationResponse {}

/// <p> Request of UpdateSubscriber </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateSubscriberRequest {
    /// <p>The <code>accountId</code> that is associated with the budget whose subscriber you want to update.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The name of the budget whose subscriber you want to update.</p>
    #[serde(rename = "BudgetName")]
    pub budget_name: String,
    /// <p>The updated subscriber associated with a budget notification.</p>
    #[serde(rename = "NewSubscriber")]
    pub new_subscriber: Subscriber,
    /// <p>The notification whose subscriber you want to update.</p>
    #[serde(rename = "Notification")]
    pub notification: Notification,
    /// <p>The previous subscriber associated with a budget notification.</p>
    #[serde(rename = "OldSubscriber")]
    pub old_subscriber: Subscriber,
}

/// <p> Response of UpdateSubscriber </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateSubscriberResponse {}

/// Errors returned by CreateBudget
#[derive(Debug, PartialEq)]
pub enum CreateBudgetError {
    /// <p>You've exceeded the notification or subscriber limit.</p>
    CreationLimitExceeded(String),
    /// <p>The budget name already exists. Budget names must be unique within an account.</p>
    DuplicateRecord(String),
    /// <p>An error on the server occurred during the processing of your request. Try again later.</p>
    InternalError(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidParameter(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateBudgetError {
    pub fn from_body(body: &str) -> CreateBudgetError {
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
                    "CreationLimitExceededException" => {
                        CreateBudgetError::CreationLimitExceeded(String::from(error_message))
                    }
                    "DuplicateRecordException" => {
                        CreateBudgetError::DuplicateRecord(String::from(error_message))
                    }
                    "InternalErrorException" => {
                        CreateBudgetError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        CreateBudgetError::InvalidParameter(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateBudgetError::Validation(error_message.to_string())
                    }
                    _ => CreateBudgetError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateBudgetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateBudgetError {
    fn from(err: serde_json::error::Error) -> CreateBudgetError {
        CreateBudgetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateBudgetError {
    fn from(err: CredentialsError) -> CreateBudgetError {
        CreateBudgetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateBudgetError {
    fn from(err: HttpDispatchError) -> CreateBudgetError {
        CreateBudgetError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateBudgetError {
    fn from(err: io::Error) -> CreateBudgetError {
        CreateBudgetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateBudgetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateBudgetError {
    fn description(&self) -> &str {
        match *self {
            CreateBudgetError::CreationLimitExceeded(ref cause) => cause,
            CreateBudgetError::DuplicateRecord(ref cause) => cause,
            CreateBudgetError::InternalError(ref cause) => cause,
            CreateBudgetError::InvalidParameter(ref cause) => cause,
            CreateBudgetError::Validation(ref cause) => cause,
            CreateBudgetError::Credentials(ref err) => err.description(),
            CreateBudgetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateBudgetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateNotification
#[derive(Debug, PartialEq)]
pub enum CreateNotificationError {
    /// <p>You've exceeded the notification or subscriber limit.</p>
    CreationLimitExceeded(String),
    /// <p>The budget name already exists. Budget names must be unique within an account.</p>
    DuplicateRecord(String),
    /// <p>An error on the server occurred during the processing of your request. Try again later.</p>
    InternalError(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidParameter(String),
    /// <p>We can’t locate the resource that you specified.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateNotificationError {
    pub fn from_body(body: &str) -> CreateNotificationError {
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
                    "CreationLimitExceededException" => {
                        CreateNotificationError::CreationLimitExceeded(String::from(error_message))
                    }
                    "DuplicateRecordException" => {
                        CreateNotificationError::DuplicateRecord(String::from(error_message))
                    }
                    "InternalErrorException" => {
                        CreateNotificationError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        CreateNotificationError::InvalidParameter(String::from(error_message))
                    }
                    "NotFoundException" => {
                        CreateNotificationError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateNotificationError::Validation(error_message.to_string())
                    }
                    _ => CreateNotificationError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateNotificationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateNotificationError {
    fn from(err: serde_json::error::Error) -> CreateNotificationError {
        CreateNotificationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateNotificationError {
    fn from(err: CredentialsError) -> CreateNotificationError {
        CreateNotificationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateNotificationError {
    fn from(err: HttpDispatchError) -> CreateNotificationError {
        CreateNotificationError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateNotificationError {
    fn from(err: io::Error) -> CreateNotificationError {
        CreateNotificationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateNotificationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateNotificationError {
    fn description(&self) -> &str {
        match *self {
            CreateNotificationError::CreationLimitExceeded(ref cause) => cause,
            CreateNotificationError::DuplicateRecord(ref cause) => cause,
            CreateNotificationError::InternalError(ref cause) => cause,
            CreateNotificationError::InvalidParameter(ref cause) => cause,
            CreateNotificationError::NotFound(ref cause) => cause,
            CreateNotificationError::Validation(ref cause) => cause,
            CreateNotificationError::Credentials(ref err) => err.description(),
            CreateNotificationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateNotificationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateSubscriber
#[derive(Debug, PartialEq)]
pub enum CreateSubscriberError {
    /// <p>You've exceeded the notification or subscriber limit.</p>
    CreationLimitExceeded(String),
    /// <p>The budget name already exists. Budget names must be unique within an account.</p>
    DuplicateRecord(String),
    /// <p>An error on the server occurred during the processing of your request. Try again later.</p>
    InternalError(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidParameter(String),
    /// <p>We can’t locate the resource that you specified.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateSubscriberError {
    pub fn from_body(body: &str) -> CreateSubscriberError {
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
                    "CreationLimitExceededException" => {
                        CreateSubscriberError::CreationLimitExceeded(String::from(error_message))
                    }
                    "DuplicateRecordException" => {
                        CreateSubscriberError::DuplicateRecord(String::from(error_message))
                    }
                    "InternalErrorException" => {
                        CreateSubscriberError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        CreateSubscriberError::InvalidParameter(String::from(error_message))
                    }
                    "NotFoundException" => {
                        CreateSubscriberError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateSubscriberError::Validation(error_message.to_string())
                    }
                    _ => CreateSubscriberError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateSubscriberError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateSubscriberError {
    fn from(err: serde_json::error::Error) -> CreateSubscriberError {
        CreateSubscriberError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateSubscriberError {
    fn from(err: CredentialsError) -> CreateSubscriberError {
        CreateSubscriberError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateSubscriberError {
    fn from(err: HttpDispatchError) -> CreateSubscriberError {
        CreateSubscriberError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateSubscriberError {
    fn from(err: io::Error) -> CreateSubscriberError {
        CreateSubscriberError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateSubscriberError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateSubscriberError {
    fn description(&self) -> &str {
        match *self {
            CreateSubscriberError::CreationLimitExceeded(ref cause) => cause,
            CreateSubscriberError::DuplicateRecord(ref cause) => cause,
            CreateSubscriberError::InternalError(ref cause) => cause,
            CreateSubscriberError::InvalidParameter(ref cause) => cause,
            CreateSubscriberError::NotFound(ref cause) => cause,
            CreateSubscriberError::Validation(ref cause) => cause,
            CreateSubscriberError::Credentials(ref err) => err.description(),
            CreateSubscriberError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateSubscriberError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteBudget
#[derive(Debug, PartialEq)]
pub enum DeleteBudgetError {
    /// <p>An error on the server occurred during the processing of your request. Try again later.</p>
    InternalError(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidParameter(String),
    /// <p>We can’t locate the resource that you specified.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteBudgetError {
    pub fn from_body(body: &str) -> DeleteBudgetError {
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
                    "InternalErrorException" => {
                        DeleteBudgetError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DeleteBudgetError::InvalidParameter(String::from(error_message))
                    }
                    "NotFoundException" => DeleteBudgetError::NotFound(String::from(error_message)),
                    "ValidationException" => {
                        DeleteBudgetError::Validation(error_message.to_string())
                    }
                    _ => DeleteBudgetError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteBudgetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteBudgetError {
    fn from(err: serde_json::error::Error) -> DeleteBudgetError {
        DeleteBudgetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteBudgetError {
    fn from(err: CredentialsError) -> DeleteBudgetError {
        DeleteBudgetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteBudgetError {
    fn from(err: HttpDispatchError) -> DeleteBudgetError {
        DeleteBudgetError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteBudgetError {
    fn from(err: io::Error) -> DeleteBudgetError {
        DeleteBudgetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteBudgetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBudgetError {
    fn description(&self) -> &str {
        match *self {
            DeleteBudgetError::InternalError(ref cause) => cause,
            DeleteBudgetError::InvalidParameter(ref cause) => cause,
            DeleteBudgetError::NotFound(ref cause) => cause,
            DeleteBudgetError::Validation(ref cause) => cause,
            DeleteBudgetError::Credentials(ref err) => err.description(),
            DeleteBudgetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteBudgetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteNotification
#[derive(Debug, PartialEq)]
pub enum DeleteNotificationError {
    /// <p>An error on the server occurred during the processing of your request. Try again later.</p>
    InternalError(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidParameter(String),
    /// <p>We can’t locate the resource that you specified.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteNotificationError {
    pub fn from_body(body: &str) -> DeleteNotificationError {
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
                    "InternalErrorException" => {
                        DeleteNotificationError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DeleteNotificationError::InvalidParameter(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteNotificationError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteNotificationError::Validation(error_message.to_string())
                    }
                    _ => DeleteNotificationError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteNotificationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteNotificationError {
    fn from(err: serde_json::error::Error) -> DeleteNotificationError {
        DeleteNotificationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteNotificationError {
    fn from(err: CredentialsError) -> DeleteNotificationError {
        DeleteNotificationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteNotificationError {
    fn from(err: HttpDispatchError) -> DeleteNotificationError {
        DeleteNotificationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteNotificationError {
    fn from(err: io::Error) -> DeleteNotificationError {
        DeleteNotificationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteNotificationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteNotificationError {
    fn description(&self) -> &str {
        match *self {
            DeleteNotificationError::InternalError(ref cause) => cause,
            DeleteNotificationError::InvalidParameter(ref cause) => cause,
            DeleteNotificationError::NotFound(ref cause) => cause,
            DeleteNotificationError::Validation(ref cause) => cause,
            DeleteNotificationError::Credentials(ref err) => err.description(),
            DeleteNotificationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteNotificationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteSubscriber
#[derive(Debug, PartialEq)]
pub enum DeleteSubscriberError {
    /// <p>An error on the server occurred during the processing of your request. Try again later.</p>
    InternalError(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidParameter(String),
    /// <p>We can’t locate the resource that you specified.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteSubscriberError {
    pub fn from_body(body: &str) -> DeleteSubscriberError {
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
                    "InternalErrorException" => {
                        DeleteSubscriberError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DeleteSubscriberError::InvalidParameter(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteSubscriberError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteSubscriberError::Validation(error_message.to_string())
                    }
                    _ => DeleteSubscriberError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteSubscriberError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteSubscriberError {
    fn from(err: serde_json::error::Error) -> DeleteSubscriberError {
        DeleteSubscriberError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteSubscriberError {
    fn from(err: CredentialsError) -> DeleteSubscriberError {
        DeleteSubscriberError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteSubscriberError {
    fn from(err: HttpDispatchError) -> DeleteSubscriberError {
        DeleteSubscriberError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteSubscriberError {
    fn from(err: io::Error) -> DeleteSubscriberError {
        DeleteSubscriberError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteSubscriberError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteSubscriberError {
    fn description(&self) -> &str {
        match *self {
            DeleteSubscriberError::InternalError(ref cause) => cause,
            DeleteSubscriberError::InvalidParameter(ref cause) => cause,
            DeleteSubscriberError::NotFound(ref cause) => cause,
            DeleteSubscriberError::Validation(ref cause) => cause,
            DeleteSubscriberError::Credentials(ref err) => err.description(),
            DeleteSubscriberError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteSubscriberError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeBudget
#[derive(Debug, PartialEq)]
pub enum DescribeBudgetError {
    /// <p>An error on the server occurred during the processing of your request. Try again later.</p>
    InternalError(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidParameter(String),
    /// <p>We can’t locate the resource that you specified.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeBudgetError {
    pub fn from_body(body: &str) -> DescribeBudgetError {
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
                    "InternalErrorException" => {
                        DescribeBudgetError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DescribeBudgetError::InvalidParameter(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DescribeBudgetError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeBudgetError::Validation(error_message.to_string())
                    }
                    _ => DescribeBudgetError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeBudgetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeBudgetError {
    fn from(err: serde_json::error::Error) -> DescribeBudgetError {
        DescribeBudgetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeBudgetError {
    fn from(err: CredentialsError) -> DescribeBudgetError {
        DescribeBudgetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeBudgetError {
    fn from(err: HttpDispatchError) -> DescribeBudgetError {
        DescribeBudgetError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeBudgetError {
    fn from(err: io::Error) -> DescribeBudgetError {
        DescribeBudgetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeBudgetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeBudgetError {
    fn description(&self) -> &str {
        match *self {
            DescribeBudgetError::InternalError(ref cause) => cause,
            DescribeBudgetError::InvalidParameter(ref cause) => cause,
            DescribeBudgetError::NotFound(ref cause) => cause,
            DescribeBudgetError::Validation(ref cause) => cause,
            DescribeBudgetError::Credentials(ref err) => err.description(),
            DescribeBudgetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeBudgetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeBudgets
#[derive(Debug, PartialEq)]
pub enum DescribeBudgetsError {
    /// <p>The pagination token expired.</p>
    ExpiredNextToken(String),
    /// <p>An error on the server occurred during the processing of your request. Try again later.</p>
    InternalError(String),
    /// <p>The pagination token is invalid.</p>
    InvalidNextToken(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidParameter(String),
    /// <p>We can’t locate the resource that you specified.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeBudgetsError {
    pub fn from_body(body: &str) -> DescribeBudgetsError {
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
                    "ExpiredNextTokenException" => {
                        DescribeBudgetsError::ExpiredNextToken(String::from(error_message))
                    }
                    "InternalErrorException" => {
                        DescribeBudgetsError::InternalError(String::from(error_message))
                    }
                    "InvalidNextTokenException" => {
                        DescribeBudgetsError::InvalidNextToken(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DescribeBudgetsError::InvalidParameter(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DescribeBudgetsError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeBudgetsError::Validation(error_message.to_string())
                    }
                    _ => DescribeBudgetsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeBudgetsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeBudgetsError {
    fn from(err: serde_json::error::Error) -> DescribeBudgetsError {
        DescribeBudgetsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeBudgetsError {
    fn from(err: CredentialsError) -> DescribeBudgetsError {
        DescribeBudgetsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeBudgetsError {
    fn from(err: HttpDispatchError) -> DescribeBudgetsError {
        DescribeBudgetsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeBudgetsError {
    fn from(err: io::Error) -> DescribeBudgetsError {
        DescribeBudgetsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeBudgetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeBudgetsError {
    fn description(&self) -> &str {
        match *self {
            DescribeBudgetsError::ExpiredNextToken(ref cause) => cause,
            DescribeBudgetsError::InternalError(ref cause) => cause,
            DescribeBudgetsError::InvalidNextToken(ref cause) => cause,
            DescribeBudgetsError::InvalidParameter(ref cause) => cause,
            DescribeBudgetsError::NotFound(ref cause) => cause,
            DescribeBudgetsError::Validation(ref cause) => cause,
            DescribeBudgetsError::Credentials(ref err) => err.description(),
            DescribeBudgetsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeBudgetsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeNotificationsForBudget
#[derive(Debug, PartialEq)]
pub enum DescribeNotificationsForBudgetError {
    /// <p>The pagination token expired.</p>
    ExpiredNextToken(String),
    /// <p>An error on the server occurred during the processing of your request. Try again later.</p>
    InternalError(String),
    /// <p>The pagination token is invalid.</p>
    InvalidNextToken(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidParameter(String),
    /// <p>We can’t locate the resource that you specified.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeNotificationsForBudgetError {
    pub fn from_body(body: &str) -> DescribeNotificationsForBudgetError {
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
                    "ExpiredNextTokenException" => {
                        DescribeNotificationsForBudgetError::ExpiredNextToken(String::from(
                            error_message,
                        ))
                    }
                    "InternalErrorException" => DescribeNotificationsForBudgetError::InternalError(
                        String::from(error_message),
                    ),
                    "InvalidNextTokenException" => {
                        DescribeNotificationsForBudgetError::InvalidNextToken(String::from(
                            error_message,
                        ))
                    }
                    "InvalidParameterException" => {
                        DescribeNotificationsForBudgetError::InvalidParameter(String::from(
                            error_message,
                        ))
                    }
                    "NotFoundException" => {
                        DescribeNotificationsForBudgetError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeNotificationsForBudgetError::Validation(error_message.to_string())
                    }
                    _ => DescribeNotificationsForBudgetError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeNotificationsForBudgetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeNotificationsForBudgetError {
    fn from(err: serde_json::error::Error) -> DescribeNotificationsForBudgetError {
        DescribeNotificationsForBudgetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeNotificationsForBudgetError {
    fn from(err: CredentialsError) -> DescribeNotificationsForBudgetError {
        DescribeNotificationsForBudgetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeNotificationsForBudgetError {
    fn from(err: HttpDispatchError) -> DescribeNotificationsForBudgetError {
        DescribeNotificationsForBudgetError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeNotificationsForBudgetError {
    fn from(err: io::Error) -> DescribeNotificationsForBudgetError {
        DescribeNotificationsForBudgetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeNotificationsForBudgetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeNotificationsForBudgetError {
    fn description(&self) -> &str {
        match *self {
            DescribeNotificationsForBudgetError::ExpiredNextToken(ref cause) => cause,
            DescribeNotificationsForBudgetError::InternalError(ref cause) => cause,
            DescribeNotificationsForBudgetError::InvalidNextToken(ref cause) => cause,
            DescribeNotificationsForBudgetError::InvalidParameter(ref cause) => cause,
            DescribeNotificationsForBudgetError::NotFound(ref cause) => cause,
            DescribeNotificationsForBudgetError::Validation(ref cause) => cause,
            DescribeNotificationsForBudgetError::Credentials(ref err) => err.description(),
            DescribeNotificationsForBudgetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeNotificationsForBudgetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeSubscribersForNotification
#[derive(Debug, PartialEq)]
pub enum DescribeSubscribersForNotificationError {
    /// <p>The pagination token expired.</p>
    ExpiredNextToken(String),
    /// <p>An error on the server occurred during the processing of your request. Try again later.</p>
    InternalError(String),
    /// <p>The pagination token is invalid.</p>
    InvalidNextToken(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidParameter(String),
    /// <p>We can’t locate the resource that you specified.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeSubscribersForNotificationError {
    pub fn from_body(body: &str) -> DescribeSubscribersForNotificationError {
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
                    "ExpiredNextTokenException" => {
                        DescribeSubscribersForNotificationError::ExpiredNextToken(String::from(
                            error_message,
                        ))
                    }
                    "InternalErrorException" => {
                        DescribeSubscribersForNotificationError::InternalError(String::from(
                            error_message,
                        ))
                    }
                    "InvalidNextTokenException" => {
                        DescribeSubscribersForNotificationError::InvalidNextToken(String::from(
                            error_message,
                        ))
                    }
                    "InvalidParameterException" => {
                        DescribeSubscribersForNotificationError::InvalidParameter(String::from(
                            error_message,
                        ))
                    }
                    "NotFoundException" => DescribeSubscribersForNotificationError::NotFound(
                        String::from(error_message),
                    ),
                    "ValidationException" => DescribeSubscribersForNotificationError::Validation(
                        error_message.to_string(),
                    ),
                    _ => DescribeSubscribersForNotificationError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeSubscribersForNotificationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeSubscribersForNotificationError {
    fn from(err: serde_json::error::Error) -> DescribeSubscribersForNotificationError {
        DescribeSubscribersForNotificationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeSubscribersForNotificationError {
    fn from(err: CredentialsError) -> DescribeSubscribersForNotificationError {
        DescribeSubscribersForNotificationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeSubscribersForNotificationError {
    fn from(err: HttpDispatchError) -> DescribeSubscribersForNotificationError {
        DescribeSubscribersForNotificationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeSubscribersForNotificationError {
    fn from(err: io::Error) -> DescribeSubscribersForNotificationError {
        DescribeSubscribersForNotificationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeSubscribersForNotificationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeSubscribersForNotificationError {
    fn description(&self) -> &str {
        match *self {
            DescribeSubscribersForNotificationError::ExpiredNextToken(ref cause) => cause,
            DescribeSubscribersForNotificationError::InternalError(ref cause) => cause,
            DescribeSubscribersForNotificationError::InvalidNextToken(ref cause) => cause,
            DescribeSubscribersForNotificationError::InvalidParameter(ref cause) => cause,
            DescribeSubscribersForNotificationError::NotFound(ref cause) => cause,
            DescribeSubscribersForNotificationError::Validation(ref cause) => cause,
            DescribeSubscribersForNotificationError::Credentials(ref err) => err.description(),
            DescribeSubscribersForNotificationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeSubscribersForNotificationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateBudget
#[derive(Debug, PartialEq)]
pub enum UpdateBudgetError {
    /// <p>An error on the server occurred during the processing of your request. Try again later.</p>
    InternalError(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidParameter(String),
    /// <p>We can’t locate the resource that you specified.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateBudgetError {
    pub fn from_body(body: &str) -> UpdateBudgetError {
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
                    "InternalErrorException" => {
                        UpdateBudgetError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        UpdateBudgetError::InvalidParameter(String::from(error_message))
                    }
                    "NotFoundException" => UpdateBudgetError::NotFound(String::from(error_message)),
                    "ValidationException" => {
                        UpdateBudgetError::Validation(error_message.to_string())
                    }
                    _ => UpdateBudgetError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateBudgetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateBudgetError {
    fn from(err: serde_json::error::Error) -> UpdateBudgetError {
        UpdateBudgetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateBudgetError {
    fn from(err: CredentialsError) -> UpdateBudgetError {
        UpdateBudgetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateBudgetError {
    fn from(err: HttpDispatchError) -> UpdateBudgetError {
        UpdateBudgetError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateBudgetError {
    fn from(err: io::Error) -> UpdateBudgetError {
        UpdateBudgetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateBudgetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateBudgetError {
    fn description(&self) -> &str {
        match *self {
            UpdateBudgetError::InternalError(ref cause) => cause,
            UpdateBudgetError::InvalidParameter(ref cause) => cause,
            UpdateBudgetError::NotFound(ref cause) => cause,
            UpdateBudgetError::Validation(ref cause) => cause,
            UpdateBudgetError::Credentials(ref err) => err.description(),
            UpdateBudgetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateBudgetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateNotification
#[derive(Debug, PartialEq)]
pub enum UpdateNotificationError {
    /// <p>The budget name already exists. Budget names must be unique within an account.</p>
    DuplicateRecord(String),
    /// <p>An error on the server occurred during the processing of your request. Try again later.</p>
    InternalError(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidParameter(String),
    /// <p>We can’t locate the resource that you specified.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateNotificationError {
    pub fn from_body(body: &str) -> UpdateNotificationError {
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
                    "DuplicateRecordException" => {
                        UpdateNotificationError::DuplicateRecord(String::from(error_message))
                    }
                    "InternalErrorException" => {
                        UpdateNotificationError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        UpdateNotificationError::InvalidParameter(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateNotificationError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateNotificationError::Validation(error_message.to_string())
                    }
                    _ => UpdateNotificationError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateNotificationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateNotificationError {
    fn from(err: serde_json::error::Error) -> UpdateNotificationError {
        UpdateNotificationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateNotificationError {
    fn from(err: CredentialsError) -> UpdateNotificationError {
        UpdateNotificationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateNotificationError {
    fn from(err: HttpDispatchError) -> UpdateNotificationError {
        UpdateNotificationError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateNotificationError {
    fn from(err: io::Error) -> UpdateNotificationError {
        UpdateNotificationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateNotificationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateNotificationError {
    fn description(&self) -> &str {
        match *self {
            UpdateNotificationError::DuplicateRecord(ref cause) => cause,
            UpdateNotificationError::InternalError(ref cause) => cause,
            UpdateNotificationError::InvalidParameter(ref cause) => cause,
            UpdateNotificationError::NotFound(ref cause) => cause,
            UpdateNotificationError::Validation(ref cause) => cause,
            UpdateNotificationError::Credentials(ref err) => err.description(),
            UpdateNotificationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateNotificationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateSubscriber
#[derive(Debug, PartialEq)]
pub enum UpdateSubscriberError {
    /// <p>The budget name already exists. Budget names must be unique within an account.</p>
    DuplicateRecord(String),
    /// <p>An error on the server occurred during the processing of your request. Try again later.</p>
    InternalError(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidParameter(String),
    /// <p>We can’t locate the resource that you specified.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateSubscriberError {
    pub fn from_body(body: &str) -> UpdateSubscriberError {
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
                    "DuplicateRecordException" => {
                        UpdateSubscriberError::DuplicateRecord(String::from(error_message))
                    }
                    "InternalErrorException" => {
                        UpdateSubscriberError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        UpdateSubscriberError::InvalidParameter(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateSubscriberError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateSubscriberError::Validation(error_message.to_string())
                    }
                    _ => UpdateSubscriberError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateSubscriberError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateSubscriberError {
    fn from(err: serde_json::error::Error) -> UpdateSubscriberError {
        UpdateSubscriberError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateSubscriberError {
    fn from(err: CredentialsError) -> UpdateSubscriberError {
        UpdateSubscriberError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateSubscriberError {
    fn from(err: HttpDispatchError) -> UpdateSubscriberError {
        UpdateSubscriberError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateSubscriberError {
    fn from(err: io::Error) -> UpdateSubscriberError {
        UpdateSubscriberError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateSubscriberError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateSubscriberError {
    fn description(&self) -> &str {
        match *self {
            UpdateSubscriberError::DuplicateRecord(ref cause) => cause,
            UpdateSubscriberError::InternalError(ref cause) => cause,
            UpdateSubscriberError::InvalidParameter(ref cause) => cause,
            UpdateSubscriberError::NotFound(ref cause) => cause,
            UpdateSubscriberError::Validation(ref cause) => cause,
            UpdateSubscriberError::Credentials(ref err) => err.description(),
            UpdateSubscriberError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateSubscriberError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWSBudgets API. AWSBudgets clients implement this trait.
pub trait Budgets {
    /// <p>Creates a budget and, if included, notifications and subscribers. </p>
    fn create_budget(
        &self,
        input: CreateBudgetRequest,
    ) -> RusotoFuture<CreateBudgetResponse, CreateBudgetError>;

    /// <p>Creates a notification. You must create the budget before you create the associated notification.</p>
    fn create_notification(
        &self,
        input: CreateNotificationRequest,
    ) -> RusotoFuture<CreateNotificationResponse, CreateNotificationError>;

    /// <p>Creates a subscriber. You must create the associated budget and notification before you create the subscriber.</p>
    fn create_subscriber(
        &self,
        input: CreateSubscriberRequest,
    ) -> RusotoFuture<CreateSubscriberResponse, CreateSubscriberError>;

    /// <p>Deletes a budget. You can delete your budget at any time.</p> <p> <b>Deleting a budget also deletes the notifications and subscribers associated with that budget.</b> </p>
    fn delete_budget(
        &self,
        input: DeleteBudgetRequest,
    ) -> RusotoFuture<DeleteBudgetResponse, DeleteBudgetError>;

    /// <p>Deletes a notification.</p> <p> <b>Deleting a notification also deletes the subscribers associated with the notification.</b> </p>
    fn delete_notification(
        &self,
        input: DeleteNotificationRequest,
    ) -> RusotoFuture<DeleteNotificationResponse, DeleteNotificationError>;

    /// <p>Deletes a subscriber.</p> <p> <b>Deleting the last subscriber to a notification also deletes the notification.</b> </p>
    fn delete_subscriber(
        &self,
        input: DeleteSubscriberRequest,
    ) -> RusotoFuture<DeleteSubscriberResponse, DeleteSubscriberError>;

    /// <p>Describes a budget.</p>
    fn describe_budget(
        &self,
        input: DescribeBudgetRequest,
    ) -> RusotoFuture<DescribeBudgetResponse, DescribeBudgetError>;

    /// <p>Lists the budgets associated with an account.</p>
    fn describe_budgets(
        &self,
        input: DescribeBudgetsRequest,
    ) -> RusotoFuture<DescribeBudgetsResponse, DescribeBudgetsError>;

    /// <p>Lists the notifications associated with a budget.</p>
    fn describe_notifications_for_budget(
        &self,
        input: DescribeNotificationsForBudgetRequest,
    ) -> RusotoFuture<DescribeNotificationsForBudgetResponse, DescribeNotificationsForBudgetError>;

    /// <p>Lists the subscribers associated with a notification.</p>
    fn describe_subscribers_for_notification(
        &self,
        input: DescribeSubscribersForNotificationRequest,
    ) -> RusotoFuture<
        DescribeSubscribersForNotificationResponse,
        DescribeSubscribersForNotificationError,
    >;

    /// <p>Updates a budget. You can change every part of a budget except for the <code>budgetName</code> and the <code>calculatedSpend</code>. When a budget is modified, the <code>calculatedSpend</code> drops to zero until AWS has new usage data to use for forecasting.</p>
    fn update_budget(
        &self,
        input: UpdateBudgetRequest,
    ) -> RusotoFuture<UpdateBudgetResponse, UpdateBudgetError>;

    /// <p>Updates a notification.</p>
    fn update_notification(
        &self,
        input: UpdateNotificationRequest,
    ) -> RusotoFuture<UpdateNotificationResponse, UpdateNotificationError>;

    /// <p>Updates a subscriber.</p>
    fn update_subscriber(
        &self,
        input: UpdateSubscriberRequest,
    ) -> RusotoFuture<UpdateSubscriberResponse, UpdateSubscriberError>;
}
/// A client for the AWSBudgets API.
pub struct BudgetsClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl BudgetsClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> BudgetsClient {
        BudgetsClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> BudgetsClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        BudgetsClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> Budgets for BudgetsClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p>Creates a budget and, if included, notifications and subscribers. </p>
    fn create_budget(
        &self,
        input: CreateBudgetRequest,
    ) -> RusotoFuture<CreateBudgetResponse, CreateBudgetError> {
        let mut request = SignedRequest::new("POST", "budgets", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSBudgetServiceGateway.CreateBudget");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateBudgetResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateBudgetError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a notification. You must create the budget before you create the associated notification.</p>
    fn create_notification(
        &self,
        input: CreateNotificationRequest,
    ) -> RusotoFuture<CreateNotificationResponse, CreateNotificationError> {
        let mut request = SignedRequest::new("POST", "budgets", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSBudgetServiceGateway.CreateNotification");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateNotificationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateNotificationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a subscriber. You must create the associated budget and notification before you create the subscriber.</p>
    fn create_subscriber(
        &self,
        input: CreateSubscriberRequest,
    ) -> RusotoFuture<CreateSubscriberResponse, CreateSubscriberError> {
        let mut request = SignedRequest::new("POST", "budgets", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSBudgetServiceGateway.CreateSubscriber");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateSubscriberResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateSubscriberError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes a budget. You can delete your budget at any time.</p> <p> <b>Deleting a budget also deletes the notifications and subscribers associated with that budget.</b> </p>
    fn delete_budget(
        &self,
        input: DeleteBudgetRequest,
    ) -> RusotoFuture<DeleteBudgetResponse, DeleteBudgetError> {
        let mut request = SignedRequest::new("POST", "budgets", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSBudgetServiceGateway.DeleteBudget");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteBudgetResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteBudgetError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes a notification.</p> <p> <b>Deleting a notification also deletes the subscribers associated with the notification.</b> </p>
    fn delete_notification(
        &self,
        input: DeleteNotificationRequest,
    ) -> RusotoFuture<DeleteNotificationResponse, DeleteNotificationError> {
        let mut request = SignedRequest::new("POST", "budgets", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSBudgetServiceGateway.DeleteNotification");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteNotificationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteNotificationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes a subscriber.</p> <p> <b>Deleting the last subscriber to a notification also deletes the notification.</b> </p>
    fn delete_subscriber(
        &self,
        input: DeleteSubscriberRequest,
    ) -> RusotoFuture<DeleteSubscriberResponse, DeleteSubscriberError> {
        let mut request = SignedRequest::new("POST", "budgets", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSBudgetServiceGateway.DeleteSubscriber");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteSubscriberResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteSubscriberError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Describes a budget.</p>
    fn describe_budget(
        &self,
        input: DescribeBudgetRequest,
    ) -> RusotoFuture<DescribeBudgetResponse, DescribeBudgetError> {
        let mut request = SignedRequest::new("POST", "budgets", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSBudgetServiceGateway.DescribeBudget");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeBudgetResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeBudgetError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the budgets associated with an account.</p>
    fn describe_budgets(
        &self,
        input: DescribeBudgetsRequest,
    ) -> RusotoFuture<DescribeBudgetsResponse, DescribeBudgetsError> {
        let mut request = SignedRequest::new("POST", "budgets", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSBudgetServiceGateway.DescribeBudgets");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeBudgetsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeBudgetsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the notifications associated with a budget.</p>
    fn describe_notifications_for_budget(
        &self,
        input: DescribeNotificationsForBudgetRequest,
    ) -> RusotoFuture<DescribeNotificationsForBudgetResponse, DescribeNotificationsForBudgetError>
    {
        let mut request = SignedRequest::new("POST", "budgets", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSBudgetServiceGateway.DescribeNotificationsForBudget",
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

                    serde_json::from_str::<DescribeNotificationsForBudgetResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeNotificationsForBudgetError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the subscribers associated with a notification.</p>
    fn describe_subscribers_for_notification(
        &self,
        input: DescribeSubscribersForNotificationRequest,
    ) -> RusotoFuture<
        DescribeSubscribersForNotificationResponse,
        DescribeSubscribersForNotificationError,
    > {
        let mut request = SignedRequest::new("POST", "budgets", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSBudgetServiceGateway.DescribeSubscribersForNotification",
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

                    serde_json::from_str::<DescribeSubscribersForNotificationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeSubscribersForNotificationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates a budget. You can change every part of a budget except for the <code>budgetName</code> and the <code>calculatedSpend</code>. When a budget is modified, the <code>calculatedSpend</code> drops to zero until AWS has new usage data to use for forecasting.</p>
    fn update_budget(
        &self,
        input: UpdateBudgetRequest,
    ) -> RusotoFuture<UpdateBudgetResponse, UpdateBudgetError> {
        let mut request = SignedRequest::new("POST", "budgets", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSBudgetServiceGateway.UpdateBudget");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateBudgetResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateBudgetError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates a notification.</p>
    fn update_notification(
        &self,
        input: UpdateNotificationRequest,
    ) -> RusotoFuture<UpdateNotificationResponse, UpdateNotificationError> {
        let mut request = SignedRequest::new("POST", "budgets", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSBudgetServiceGateway.UpdateNotification");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateNotificationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateNotificationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates a subscriber.</p>
    fn update_subscriber(
        &self,
        input: UpdateSubscriberRequest,
    ) -> RusotoFuture<UpdateSubscriberResponse, UpdateSubscriberError> {
        let mut request = SignedRequest::new("POST", "budgets", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSBudgetServiceGateway.UpdateSubscriber");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateSubscriberResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateSubscriberError::from_body(
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
