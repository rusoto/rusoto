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
/// <p>Represents the output of the <code>CreateBudget</code> operation. The content consists of the detailed metadata and data file information, and the current status of the <code>budget</code> object.</p> <p>This is the ARN pattern for a budget: </p> <p> <code>arn:aws:budgetservice::AccountId:budget/budgetName</code> </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Budget {
    /// <p>The total amount of cost, usage, RI utilization, RI coverage, Savings Plans utilization, or Savings Plans coverage that you want to track with your budget.</p> <p> <code>BudgetLimit</code> is required for cost or usage budgets, but optional for RI or Savings Plans utilization or coverage budgets. RI and Savings Plans utilization or coverage budgets default to <code>100</code>, which is the only valid value for RI or Savings Plans utilization or coverage budgets. You can't use <code>BudgetLimit</code> with <code>PlannedBudgetLimits</code> for <code>CreateBudget</code> and <code>UpdateBudget</code> actions. </p>
    #[serde(rename = "BudgetLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_limit: Option<Spend>,
    /// <p>The name of a budget. The name must be unique within an account. The <code>:</code> and <code>&bsol;</code> characters aren't allowed in <code>BudgetName</code>.</p>
    #[serde(rename = "BudgetName")]
    pub budget_name: String,
    /// <p>Whether this budget tracks costs, usage, RI utilization, RI coverage, Savings Plans utilization, or Savings Plans coverage.</p>
    #[serde(rename = "BudgetType")]
    pub budget_type: String,
    /// <p>The actual and forecasted cost or usage that the budget tracks.</p>
    #[serde(rename = "CalculatedSpend")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculated_spend: Option<CalculatedSpend>,
    /// <p><p>The cost filters, such as service or tag, that are applied to a budget.</p> <p>AWS Budgets supports the following services as a filter for RI budgets:</p> <ul> <li> <p>Amazon Elastic Compute Cloud - Compute</p> </li> <li> <p>Amazon Redshift</p> </li> <li> <p>Amazon Relational Database Service</p> </li> <li> <p>Amazon ElastiCache</p> </li> <li> <p>Amazon Elasticsearch Service</p> </li> </ul></p>
    #[serde(rename = "CostFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_filters: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The types of costs that are included in this <code>COST</code> budget.</p> <p> <code>USAGE</code>, <code>RI_UTILIZATION</code>, <code>RI_COVERAGE</code>, <code>Savings_Plans_Utilization</code>, and <code>Savings_Plans_Coverage</code> budgets do not have <code>CostTypes</code>.</p>
    #[serde(rename = "CostTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_types: Option<CostTypes>,
    /// <p>The last time that you updated this budget.</p>
    #[serde(rename = "LastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    /// <p>A map containing multiple <code>BudgetLimit</code>, including current or future limits.</p> <p> <code>PlannedBudgetLimits</code> is available for cost or usage budget and supports monthly and quarterly <code>TimeUnit</code>. </p> <p>For monthly budgets, provide 12 months of <code>PlannedBudgetLimits</code> values. This must start from the current month and include the next 11 months. The <code>key</code> is the start of the month, <code>UTC</code> in epoch seconds. </p> <p>For quarterly budgets, provide 4 quarters of <code>PlannedBudgetLimits</code> value entries in standard calendar quarter increments. This must start from the current quarter and include the next 3 quarters. The <code>key</code> is the start of the quarter, <code>UTC</code> in epoch seconds. </p> <p>If the planned budget expires before 12 months for monthly or 4 quarters for quarterly, provide the <code>PlannedBudgetLimits</code> values only for the remaining periods.</p> <p>If the budget begins at a date in the future, provide <code>PlannedBudgetLimits</code> values from the start date of the budget. </p> <p>After all of the <code>BudgetLimit</code> values in <code>PlannedBudgetLimits</code> are used, the budget continues to use the last limit as the <code>BudgetLimit</code>. At that point, the planned budget provides the same experience as a fixed budget. </p> <p> <code>DescribeBudget</code> and <code>DescribeBudgets</code> response along with <code>PlannedBudgetLimits</code> will also contain <code>BudgetLimit</code> representing the current month or quarter limit present in <code>PlannedBudgetLimits</code>. This only applies to budgets created with <code>PlannedBudgetLimits</code>. Budgets created without <code>PlannedBudgetLimits</code> will only contain <code>BudgetLimit</code>, and no <code>PlannedBudgetLimits</code>.</p>
    #[serde(rename = "PlannedBudgetLimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planned_budget_limits: Option<::std::collections::HashMap<String, Spend>>,
    /// <p>The period of time that is covered by a budget. The period has a start date and an end date. The start date must come before the end date. The end date must come before <code>06/15/87 00:00 UTC</code>. </p> <p>If you create your budget and don't specify a start date, AWS defaults to the start of your chosen time period (DAILY, MONTHLY, QUARTERLY, or ANNUALLY). For example, if you created your budget on January 24, 2018, chose <code>DAILY</code>, and didn't set a start date, AWS set your start date to <code>01/24/18 00:00 UTC</code>. If you chose <code>MONTHLY</code>, AWS set your start date to <code>01/01/18 00:00 UTC</code>. If you didn't specify an end date, AWS set your end date to <code>06/15/87 00:00 UTC</code>. The defaults are the same for the AWS Billing and Cost Management console and the API. </p> <p>You can change either date with the <code>UpdateBudget</code> operation.</p> <p>After the end date, AWS deletes the budget and all associated notifications and subscribers.</p>
    #[serde(rename = "TimePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_period: Option<TimePeriod>,
    /// <p>The length of time until a budget resets the actual and forecasted spend. <code>DAILY</code> is available only for <code>RI_UTILIZATION</code>, <code>RI_COVERAGE</code>, <code>Savings_Plans_Utilization</code>, and <code>Savings_Plans_Coverage</code> budgets.</p>
    #[serde(rename = "TimeUnit")]
    pub time_unit: String,
}

/// <p>A history of the state of a budget at the end of the budget's specified time period.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BudgetPerformanceHistory {
    #[serde(rename = "BudgetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_name: Option<String>,
    #[serde(rename = "BudgetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_type: Option<String>,
    /// <p>A list of amounts of cost or usage that you created budgets for, compared to your actual costs or usage.</p>
    #[serde(rename = "BudgetedAndActualAmountsList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budgeted_and_actual_amounts_list: Option<Vec<BudgetedAndActualAmounts>>,
    /// <p>The history of the cost filters for a budget during the specified time period.</p>
    #[serde(rename = "CostFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_filters: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The history of the cost types for a budget during the specified time period.</p>
    #[serde(rename = "CostTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_types: Option<CostTypes>,
    #[serde(rename = "TimeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_unit: Option<String>,
}

/// <p>The amount of cost or usage that you created the budget for, compared to your actual costs or usage.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BudgetedAndActualAmounts {
    /// <p>Your actual costs or usage for a budget period.</p>
    #[serde(rename = "ActualAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_amount: Option<Spend>,
    /// <p>The amount of cost or usage that you created the budget for.</p>
    #[serde(rename = "BudgetedAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budgeted_amount: Option<Spend>,
    /// <p>The time period covered by this budget comparison.</p>
    #[serde(rename = "TimePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_period: Option<TimePeriod>,
}

/// <p>The spend objects that are associated with this budget. The <code>actualSpend</code> tracks how much you've used, cost, usage, or RI units, and the <code>forecastedSpend</code> tracks how much you are predicted to spend if your current usage remains steady.</p> <p>For example, if it is the 20th of the month and you have spent <code>50</code> dollars on Amazon EC2, your <code>actualSpend</code> is <code>50 USD</code>, and your <code>forecastedSpend</code> is <code>75 USD</code>.</p>
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

/// <p>The types of cost that are included in a <code>COST</code> budget, such as tax and subscriptions.</p> <p> <code>USAGE</code>, <code>RI_UTILIZATION</code>, and <code>RI_COVERAGE</code> budgets do not have <code>CostTypes</code>.</p>
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
    /// <p>Specifies whether a budget uses a blended rate.</p> <p>The default value is <code>false</code>.</p>
    #[serde(rename = "UseBlended")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_blended: Option<bool>,
}

/// <p> Request of CreateBudget </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateBudgetRequest {
    /// <p>The <code>accountId</code> that is associated with the budget.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The budget object that you want to create.</p>
    #[serde(rename = "Budget")]
    pub budget: Budget,
    /// <p>A notification that you want to associate with a budget. A budget can have up to five notifications, and each notification can have one SNS subscriber and up to 10 email subscribers. If you include notifications and subscribers in your <code>CreateBudget</code> call, AWS creates the notifications and subscribers for you.</p>
    #[serde(rename = "NotificationsWithSubscribers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications_with_subscribers: Option<Vec<NotificationWithSubscribers>>,
}

/// <p> Response of CreateBudget </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateBudgetResponse {}

/// <p> Request of CreateNotification </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateNotificationRequest {
    /// <p>The <code>accountId</code> that is associated with the budget that you want to create a notification for.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The name of the budget that you want AWS to notify you about. Budget names must be unique within an account.</p>
    #[serde(rename = "BudgetName")]
    pub budget_name: String,
    /// <p>The notification that you want to create.</p>
    #[serde(rename = "Notification")]
    pub notification: Notification,
    /// <p>A list of subscribers that you want to associate with the notification. Each notification can have one SNS subscriber and up to 10 email subscribers.</p>
    #[serde(rename = "Subscribers")]
    pub subscribers: Vec<Subscriber>,
}

/// <p> Response of CreateNotification </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateNotificationResponse {}

/// <p> Request of CreateSubscriber </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateSubscriberRequest {
    /// <p>The <code>accountId</code> that is associated with the budget that you want to create a subscriber for.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateSubscriberResponse {}

/// <p> Request of DeleteBudget </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteBudgetResponse {}

/// <p> Request of DeleteNotification </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteNotificationResponse {}

/// <p> Request of DeleteSubscriber </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteSubscriberResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeBudgetPerformanceHistoryRequest {
    #[serde(rename = "AccountId")]
    pub account_id: String,
    #[serde(rename = "BudgetName")]
    pub budget_name: String,
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Retrieves how often the budget went into an <code>ALARM</code> state for the specified time period.</p>
    #[serde(rename = "TimePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_period: Option<TimePeriod>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeBudgetPerformanceHistoryResponse {
    /// <p>The history of how often the budget has gone into an <code>ALARM</code> state.</p> <p>For <code>DAILY</code> budgets, the history saves the state of the budget for the last 60 days. For <code>MONTHLY</code> budgets, the history saves the state of the budget for the current month plus the last 12 months. For <code>QUARTERLY</code> budgets, the history saves the state of the budget for the last four quarters.</p>
    #[serde(rename = "BudgetPerformanceHistory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_performance_history: Option<BudgetPerformanceHistory>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p> Request of DescribeBudget </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeBudgetResponse {
    /// <p>The description of the budget.</p>
    #[serde(rename = "Budget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget: Option<Budget>,
}

/// <p> Request of DescribeBudgets </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeBudgetsRequest {
    /// <p>The <code>accountId</code> that is associated with the budgets that you want descriptions of.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>An optional integer that represents how many entries a paginated response contains. The maximum is 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token that you include in your request to indicate the next set of results that you want to retrieve.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p> Response of DescribeBudgets </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeBudgetsResponse {
    /// <p>A list of budgets.</p>
    #[serde(rename = "Budgets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budgets: Option<Vec<Budget>>,
    /// <p>The pagination token in the service response that indicates the next set of results that you can retrieve.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p> Request of DescribeNotificationsForBudget </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeNotificationsForBudgetRequest {
    /// <p>The <code>accountId</code> that is associated with the budget whose notifications you want descriptions of.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The name of the budget whose notifications you want descriptions of.</p>
    #[serde(rename = "BudgetName")]
    pub budget_name: String,
    /// <p>An optional integer that represents how many entries a paginated response contains. The maximum is 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token that you include in your request to indicate the next set of results that you want to retrieve.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p> Response of GetNotificationsForBudget </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeNotificationsForBudgetResponse {
    /// <p>The pagination token in the service response that indicates the next set of results that you can retrieve.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of notifications that are associated with a budget.</p>
    #[serde(rename = "Notifications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications: Option<Vec<Notification>>,
}

/// <p> Request of DescribeSubscribersForNotification </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeSubscribersForNotificationRequest {
    /// <p>The <code>accountId</code> that is associated with the budget whose subscribers you want descriptions of.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The name of the budget whose subscribers you want descriptions of.</p>
    #[serde(rename = "BudgetName")]
    pub budget_name: String,
    /// <p>An optional integer that represents how many entries a paginated response contains. The maximum is 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token that you include in your request to indicate the next set of results that you want to retrieve.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The notification whose subscribers you want to list.</p>
    #[serde(rename = "Notification")]
    pub notification: Notification,
}

/// <p> Response of DescribeSubscribersForNotification </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeSubscribersForNotificationResponse {
    /// <p>The pagination token in the service response that indicates the next set of results that you can retrieve.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of subscribers that are associated with a notification.</p>
    #[serde(rename = "Subscribers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers: Option<Vec<Subscriber>>,
}

/// <p><p>A notification that is associated with a budget. A budget can have up to five notifications. </p> <p>Each notification must have at least one subscriber. A notification can have one SNS subscriber and up to 10 email subscribers, for a total of 11 subscribers.</p> <p>For example, if you have a budget for 200 dollars and you want to be notified when you go over 160 dollars, create a notification with the following parameters:</p> <ul> <li> <p>A notificationType of <code>ACTUAL</code> </p> </li> <li> <p>A <code>thresholdType</code> of <code>PERCENTAGE</code> </p> </li> <li> <p>A <code>comparisonOperator</code> of <code>GREATER_THAN</code> </p> </li> <li> <p>A notification <code>threshold</code> of <code>80</code> </p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Notification {
    /// <p>The comparison that is used for this notification.</p>
    #[serde(rename = "ComparisonOperator")]
    pub comparison_operator: String,
    /// <p>Whether this notification is in alarm. If a budget notification is in the <code>ALARM</code> state, you have passed the set threshold for the budget.</p>
    #[serde(rename = "NotificationState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_state: Option<String>,
    /// <p>Whether the notification is for how much you have spent (<code>ACTUAL</code>) or for how much you're forecasted to spend (<code>FORECASTED</code>).</p>
    #[serde(rename = "NotificationType")]
    pub notification_type: String,
    /// <p>The threshold that is associated with a notification. Thresholds are always a percentage.</p>
    #[serde(rename = "Threshold")]
    pub threshold: f64,
    /// <p>The type of threshold for a notification. For <code>ABSOLUTE_VALUE</code> thresholds, AWS notifies you when you go over or are forecasted to go over your total cost threshold. For <code>PERCENTAGE</code> thresholds, AWS notifies you when you go over or are forecasted to go over a certain percentage of your forecasted spend. For example, if you have a budget for 200 dollars and you have a <code>PERCENTAGE</code> threshold of 80%, AWS notifies you when you go over 160 dollars.</p>
    #[serde(rename = "ThresholdType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_type: Option<String>,
}

/// <p>A notification with subscribers. A notification can have one SNS subscriber and up to 10 email subscribers, for a total of 11 subscribers.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct NotificationWithSubscribers {
    /// <p>The notification that is associated with a budget.</p>
    #[serde(rename = "Notification")]
    pub notification: Notification,
    /// <p>A list of subscribers who are subscribed to this notification.</p>
    #[serde(rename = "Subscribers")]
    pub subscribers: Vec<Subscriber>,
}

/// <p><p>The amount of cost or usage that is measured for a budget.</p> <p>For example, a <code>Spend</code> for <code>3 GB</code> of S3 usage would have the following parameters:</p> <ul> <li> <p>An <code>Amount</code> of <code>3</code> </p> </li> <li> <p>A <code>unit</code> of <code>GB</code> </p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Spend {
    /// <p>The cost or usage amount that is associated with a budget forecast, actual spend, or budget threshold.</p>
    #[serde(rename = "Amount")]
    pub amount: String,
    /// <p>The unit of measurement that is used for the budget forecast, actual spend, or budget threshold, such as dollars or GB.</p>
    #[serde(rename = "Unit")]
    pub unit: String,
}

/// <p><p>The subscriber to a budget notification. The subscriber consists of a subscription type and either an Amazon SNS topic or an email address.</p> <p>For example, an email subscriber would have the following parameters:</p> <ul> <li> <p>A <code>subscriptionType</code> of <code>EMAIL</code> </p> </li> <li> <p>An <code>address</code> of <code>example@example.com</code> </p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Subscriber {
    /// <p>The address that AWS sends budget notifications to, either an SNS topic or an email.</p> <p>When you create a subscriber, the value of <code>Address</code> can't contain line breaks.</p>
    #[serde(rename = "Address")]
    pub address: String,
    /// <p>The type of notification that AWS sends to a subscriber.</p>
    #[serde(rename = "SubscriptionType")]
    pub subscription_type: String,
}

/// <p>The period of time that is covered by a budget. The period has a start date and an end date. The start date must come before the end date. There are no restrictions on the end date. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimePeriod {
    /// <p>The end date for a budget. If you didn't specify an end date, AWS set your end date to <code>06/15/87 00:00 UTC</code>. The defaults are the same for the AWS Billing and Cost Management console and the API.</p> <p>After the end date, AWS deletes the budget and all associated notifications and subscribers. You can change your end date with the <code>UpdateBudget</code> operation.</p>
    #[serde(rename = "End")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<f64>,
    /// <p>The start date for a budget. If you created your budget and didn't specify a start date, AWS defaults to the start of your chosen time period (DAILY, MONTHLY, QUARTERLY, or ANNUALLY). For example, if you created your budget on January 24, 2018, chose <code>DAILY</code>, and didn't set a start date, AWS set your start date to <code>01/24/18 00:00 UTC</code>. If you chose <code>MONTHLY</code>, AWS set your start date to <code>01/01/18 00:00 UTC</code>. The defaults are the same for the AWS Billing and Cost Management console and the API.</p> <p>You can change your start date with the <code>UpdateBudget</code> operation.</p>
    #[serde(rename = "Start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}

/// <p> Request of UpdateBudget </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateBudgetResponse {}

/// <p> Request of UpdateNotification </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>The previous notification that is associated with a budget.</p>
    #[serde(rename = "OldNotification")]
    pub old_notification: Notification,
}

/// <p> Response of UpdateNotification </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateNotificationResponse {}

/// <p> Request of UpdateSubscriber </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateSubscriberRequest {
    /// <p>The <code>accountId</code> that is associated with the budget whose subscriber you want to update.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The name of the budget whose subscriber you want to update.</p>
    #[serde(rename = "BudgetName")]
    pub budget_name: String,
    /// <p>The updated subscriber that is associated with a budget notification.</p>
    #[serde(rename = "NewSubscriber")]
    pub new_subscriber: Subscriber,
    /// <p>The notification whose subscriber you want to update.</p>
    #[serde(rename = "Notification")]
    pub notification: Notification,
    /// <p>The previous subscriber that is associated with a budget notification.</p>
    #[serde(rename = "OldSubscriber")]
    pub old_subscriber: Subscriber,
}

/// <p> Response of UpdateSubscriber </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateSubscriberResponse {}

/// Errors returned by CreateBudget
#[derive(Debug, PartialEq)]
pub enum CreateBudgetError {
    /// <p>You are not authorized to use this operation with the given parameters.</p>
    AccessDenied(String),
    /// <p>You've exceeded the notification or subscriber limit.</p>
    CreationLimitExceeded(String),
    /// <p>The budget name already exists. Budget names must be unique within an account.</p>
    DuplicateRecord(String),
    /// <p>An error on the server occurred during the processing of your request. Try again later.</p>
    InternalError(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidParameter(String),
}

impl CreateBudgetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateBudgetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateBudgetError::AccessDenied(err.msg))
                }
                "CreationLimitExceededException" => {
                    return RusotoError::Service(CreateBudgetError::CreationLimitExceeded(err.msg))
                }
                "DuplicateRecordException" => {
                    return RusotoError::Service(CreateBudgetError::DuplicateRecord(err.msg))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(CreateBudgetError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateBudgetError::InvalidParameter(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateBudgetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateBudgetError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateBudgetError::CreationLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateBudgetError::DuplicateRecord(ref cause) => write!(f, "{}", cause),
            CreateBudgetError::InternalError(ref cause) => write!(f, "{}", cause),
            CreateBudgetError::InvalidParameter(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateBudgetError {}
/// Errors returned by CreateNotification
#[derive(Debug, PartialEq)]
pub enum CreateNotificationError {
    /// <p>You are not authorized to use this operation with the given parameters.</p>
    AccessDenied(String),
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
}

impl CreateNotificationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateNotificationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateNotificationError::AccessDenied(err.msg))
                }
                "CreationLimitExceededException" => {
                    return RusotoError::Service(CreateNotificationError::CreationLimitExceeded(
                        err.msg,
                    ))
                }
                "DuplicateRecordException" => {
                    return RusotoError::Service(CreateNotificationError::DuplicateRecord(err.msg))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(CreateNotificationError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateNotificationError::InvalidParameter(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateNotificationError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateNotificationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateNotificationError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateNotificationError::CreationLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateNotificationError::DuplicateRecord(ref cause) => write!(f, "{}", cause),
            CreateNotificationError::InternalError(ref cause) => write!(f, "{}", cause),
            CreateNotificationError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateNotificationError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateNotificationError {}
/// Errors returned by CreateSubscriber
#[derive(Debug, PartialEq)]
pub enum CreateSubscriberError {
    /// <p>You are not authorized to use this operation with the given parameters.</p>
    AccessDenied(String),
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
}

impl CreateSubscriberError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateSubscriberError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateSubscriberError::AccessDenied(err.msg))
                }
                "CreationLimitExceededException" => {
                    return RusotoError::Service(CreateSubscriberError::CreationLimitExceeded(
                        err.msg,
                    ))
                }
                "DuplicateRecordException" => {
                    return RusotoError::Service(CreateSubscriberError::DuplicateRecord(err.msg))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(CreateSubscriberError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateSubscriberError::InvalidParameter(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateSubscriberError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateSubscriberError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateSubscriberError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateSubscriberError::CreationLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateSubscriberError::DuplicateRecord(ref cause) => write!(f, "{}", cause),
            CreateSubscriberError::InternalError(ref cause) => write!(f, "{}", cause),
            CreateSubscriberError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateSubscriberError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateSubscriberError {}
/// Errors returned by DeleteBudget
#[derive(Debug, PartialEq)]
pub enum DeleteBudgetError {
    /// <p>You are not authorized to use this operation with the given parameters.</p>
    AccessDenied(String),
    /// <p>An error on the server occurred during the processing of your request. Try again later.</p>
    InternalError(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidParameter(String),
    /// <p>We can’t locate the resource that you specified.</p>
    NotFound(String),
}

impl DeleteBudgetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteBudgetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteBudgetError::AccessDenied(err.msg))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(DeleteBudgetError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteBudgetError::InvalidParameter(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteBudgetError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteBudgetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteBudgetError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteBudgetError::InternalError(ref cause) => write!(f, "{}", cause),
            DeleteBudgetError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteBudgetError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteBudgetError {}
/// Errors returned by DeleteNotification
#[derive(Debug, PartialEq)]
pub enum DeleteNotificationError {
    /// <p>You are not authorized to use this operation with the given parameters.</p>
    AccessDenied(String),
    /// <p>An error on the server occurred during the processing of your request. Try again later.</p>
    InternalError(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidParameter(String),
    /// <p>We can’t locate the resource that you specified.</p>
    NotFound(String),
}

impl DeleteNotificationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteNotificationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteNotificationError::AccessDenied(err.msg))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(DeleteNotificationError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteNotificationError::InvalidParameter(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteNotificationError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteNotificationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteNotificationError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteNotificationError::InternalError(ref cause) => write!(f, "{}", cause),
            DeleteNotificationError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteNotificationError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteNotificationError {}
/// Errors returned by DeleteSubscriber
#[derive(Debug, PartialEq)]
pub enum DeleteSubscriberError {
    /// <p>You are not authorized to use this operation with the given parameters.</p>
    AccessDenied(String),
    /// <p>An error on the server occurred during the processing of your request. Try again later.</p>
    InternalError(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidParameter(String),
    /// <p>We can’t locate the resource that you specified.</p>
    NotFound(String),
}

impl DeleteSubscriberError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteSubscriberError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteSubscriberError::AccessDenied(err.msg))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(DeleteSubscriberError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteSubscriberError::InvalidParameter(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteSubscriberError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteSubscriberError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteSubscriberError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteSubscriberError::InternalError(ref cause) => write!(f, "{}", cause),
            DeleteSubscriberError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteSubscriberError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteSubscriberError {}
/// Errors returned by DescribeBudget
#[derive(Debug, PartialEq)]
pub enum DescribeBudgetError {
    /// <p>You are not authorized to use this operation with the given parameters.</p>
    AccessDenied(String),
    /// <p>An error on the server occurred during the processing of your request. Try again later.</p>
    InternalError(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidParameter(String),
    /// <p>We can’t locate the resource that you specified.</p>
    NotFound(String),
}

impl DescribeBudgetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeBudgetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeBudgetError::AccessDenied(err.msg))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(DescribeBudgetError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeBudgetError::InvalidParameter(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeBudgetError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeBudgetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeBudgetError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeBudgetError::InternalError(ref cause) => write!(f, "{}", cause),
            DescribeBudgetError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeBudgetError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeBudgetError {}
/// Errors returned by DescribeBudgetPerformanceHistory
#[derive(Debug, PartialEq)]
pub enum DescribeBudgetPerformanceHistoryError {
    /// <p>You are not authorized to use this operation with the given parameters.</p>
    AccessDenied(String),
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
}

impl DescribeBudgetPerformanceHistoryError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeBudgetPerformanceHistoryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        DescribeBudgetPerformanceHistoryError::AccessDenied(err.msg),
                    )
                }
                "ExpiredNextTokenException" => {
                    return RusotoError::Service(
                        DescribeBudgetPerformanceHistoryError::ExpiredNextToken(err.msg),
                    )
                }
                "InternalErrorException" => {
                    return RusotoError::Service(
                        DescribeBudgetPerformanceHistoryError::InternalError(err.msg),
                    )
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        DescribeBudgetPerformanceHistoryError::InvalidNextToken(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        DescribeBudgetPerformanceHistoryError::InvalidParameter(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeBudgetPerformanceHistoryError::NotFound(
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
impl fmt::Display for DescribeBudgetPerformanceHistoryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeBudgetPerformanceHistoryError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeBudgetPerformanceHistoryError::ExpiredNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeBudgetPerformanceHistoryError::InternalError(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeBudgetPerformanceHistoryError::InvalidNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeBudgetPerformanceHistoryError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeBudgetPerformanceHistoryError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeBudgetPerformanceHistoryError {}
/// Errors returned by DescribeBudgets
#[derive(Debug, PartialEq)]
pub enum DescribeBudgetsError {
    /// <p>You are not authorized to use this operation with the given parameters.</p>
    AccessDenied(String),
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
}

impl DescribeBudgetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeBudgetsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeBudgetsError::AccessDenied(err.msg))
                }
                "ExpiredNextTokenException" => {
                    return RusotoError::Service(DescribeBudgetsError::ExpiredNextToken(err.msg))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(DescribeBudgetsError::InternalError(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(DescribeBudgetsError::InvalidNextToken(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeBudgetsError::InvalidParameter(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeBudgetsError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeBudgetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeBudgetsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeBudgetsError::ExpiredNextToken(ref cause) => write!(f, "{}", cause),
            DescribeBudgetsError::InternalError(ref cause) => write!(f, "{}", cause),
            DescribeBudgetsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            DescribeBudgetsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeBudgetsError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeBudgetsError {}
/// Errors returned by DescribeNotificationsForBudget
#[derive(Debug, PartialEq)]
pub enum DescribeNotificationsForBudgetError {
    /// <p>You are not authorized to use this operation with the given parameters.</p>
    AccessDenied(String),
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
}

impl DescribeNotificationsForBudgetError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeNotificationsForBudgetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeNotificationsForBudgetError::AccessDenied(
                        err.msg,
                    ))
                }
                "ExpiredNextTokenException" => {
                    return RusotoError::Service(
                        DescribeNotificationsForBudgetError::ExpiredNextToken(err.msg),
                    )
                }
                "InternalErrorException" => {
                    return RusotoError::Service(
                        DescribeNotificationsForBudgetError::InternalError(err.msg),
                    )
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        DescribeNotificationsForBudgetError::InvalidNextToken(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        DescribeNotificationsForBudgetError::InvalidParameter(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeNotificationsForBudgetError::NotFound(
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
impl fmt::Display for DescribeNotificationsForBudgetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeNotificationsForBudgetError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeNotificationsForBudgetError::ExpiredNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeNotificationsForBudgetError::InternalError(ref cause) => write!(f, "{}", cause),
            DescribeNotificationsForBudgetError::InvalidNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeNotificationsForBudgetError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeNotificationsForBudgetError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeNotificationsForBudgetError {}
/// Errors returned by DescribeSubscribersForNotification
#[derive(Debug, PartialEq)]
pub enum DescribeSubscribersForNotificationError {
    /// <p>You are not authorized to use this operation with the given parameters.</p>
    AccessDenied(String),
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
}

impl DescribeSubscribersForNotificationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeSubscribersForNotificationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        DescribeSubscribersForNotificationError::AccessDenied(err.msg),
                    )
                }
                "ExpiredNextTokenException" => {
                    return RusotoError::Service(
                        DescribeSubscribersForNotificationError::ExpiredNextToken(err.msg),
                    )
                }
                "InternalErrorException" => {
                    return RusotoError::Service(
                        DescribeSubscribersForNotificationError::InternalError(err.msg),
                    )
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        DescribeSubscribersForNotificationError::InvalidNextToken(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        DescribeSubscribersForNotificationError::InvalidParameter(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeSubscribersForNotificationError::NotFound(
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
impl fmt::Display for DescribeSubscribersForNotificationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeSubscribersForNotificationError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeSubscribersForNotificationError::ExpiredNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeSubscribersForNotificationError::InternalError(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeSubscribersForNotificationError::InvalidNextToken(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeSubscribersForNotificationError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeSubscribersForNotificationError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeSubscribersForNotificationError {}
/// Errors returned by UpdateBudget
#[derive(Debug, PartialEq)]
pub enum UpdateBudgetError {
    /// <p>You are not authorized to use this operation with the given parameters.</p>
    AccessDenied(String),
    /// <p>An error on the server occurred during the processing of your request. Try again later.</p>
    InternalError(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidParameter(String),
    /// <p>We can’t locate the resource that you specified.</p>
    NotFound(String),
}

impl UpdateBudgetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateBudgetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateBudgetError::AccessDenied(err.msg))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(UpdateBudgetError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateBudgetError::InvalidParameter(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateBudgetError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateBudgetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateBudgetError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateBudgetError::InternalError(ref cause) => write!(f, "{}", cause),
            UpdateBudgetError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateBudgetError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateBudgetError {}
/// Errors returned by UpdateNotification
#[derive(Debug, PartialEq)]
pub enum UpdateNotificationError {
    /// <p>You are not authorized to use this operation with the given parameters.</p>
    AccessDenied(String),
    /// <p>The budget name already exists. Budget names must be unique within an account.</p>
    DuplicateRecord(String),
    /// <p>An error on the server occurred during the processing of your request. Try again later.</p>
    InternalError(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidParameter(String),
    /// <p>We can’t locate the resource that you specified.</p>
    NotFound(String),
}

impl UpdateNotificationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateNotificationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateNotificationError::AccessDenied(err.msg))
                }
                "DuplicateRecordException" => {
                    return RusotoError::Service(UpdateNotificationError::DuplicateRecord(err.msg))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(UpdateNotificationError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateNotificationError::InvalidParameter(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateNotificationError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateNotificationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateNotificationError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateNotificationError::DuplicateRecord(ref cause) => write!(f, "{}", cause),
            UpdateNotificationError::InternalError(ref cause) => write!(f, "{}", cause),
            UpdateNotificationError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateNotificationError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateNotificationError {}
/// Errors returned by UpdateSubscriber
#[derive(Debug, PartialEq)]
pub enum UpdateSubscriberError {
    /// <p>You are not authorized to use this operation with the given parameters.</p>
    AccessDenied(String),
    /// <p>The budget name already exists. Budget names must be unique within an account.</p>
    DuplicateRecord(String),
    /// <p>An error on the server occurred during the processing of your request. Try again later.</p>
    InternalError(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidParameter(String),
    /// <p>We can’t locate the resource that you specified.</p>
    NotFound(String),
}

impl UpdateSubscriberError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateSubscriberError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateSubscriberError::AccessDenied(err.msg))
                }
                "DuplicateRecordException" => {
                    return RusotoError::Service(UpdateSubscriberError::DuplicateRecord(err.msg))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(UpdateSubscriberError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateSubscriberError::InvalidParameter(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateSubscriberError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateSubscriberError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateSubscriberError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateSubscriberError::DuplicateRecord(ref cause) => write!(f, "{}", cause),
            UpdateSubscriberError::InternalError(ref cause) => write!(f, "{}", cause),
            UpdateSubscriberError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateSubscriberError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateSubscriberError {}
/// Trait representing the capabilities of the AWSBudgets API. AWSBudgets clients implement this trait.
#[async_trait]
pub trait Budgets {
    /// <p><p>Creates a budget and, if included, notifications and subscribers. </p> <important> <p>Only one of <code>BudgetLimit</code> or <code>PlannedBudgetLimits</code> can be present in the syntax at one time. Use the syntax that matches your case. The Request Syntax section shows the <code>BudgetLimit</code> syntax. For <code>PlannedBudgetLimits</code>, see the <a href="https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_budgets_CreateBudget.html#API_CreateBudget_Examples">Examples</a> section. </p> </important></p>
    async fn create_budget(
        &self,
        input: CreateBudgetRequest,
    ) -> Result<CreateBudgetResponse, RusotoError<CreateBudgetError>>;

    /// <p>Creates a notification. You must create the budget before you create the associated notification.</p>
    async fn create_notification(
        &self,
        input: CreateNotificationRequest,
    ) -> Result<CreateNotificationResponse, RusotoError<CreateNotificationError>>;

    /// <p>Creates a subscriber. You must create the associated budget and notification before you create the subscriber.</p>
    async fn create_subscriber(
        &self,
        input: CreateSubscriberRequest,
    ) -> Result<CreateSubscriberResponse, RusotoError<CreateSubscriberError>>;

    /// <p><p>Deletes a budget. You can delete your budget at any time.</p> <important> <p>Deleting a budget also deletes the notifications and subscribers that are associated with that budget.</p> </important></p>
    async fn delete_budget(
        &self,
        input: DeleteBudgetRequest,
    ) -> Result<DeleteBudgetResponse, RusotoError<DeleteBudgetError>>;

    /// <p><p>Deletes a notification.</p> <important> <p>Deleting a notification also deletes the subscribers that are associated with the notification.</p> </important></p>
    async fn delete_notification(
        &self,
        input: DeleteNotificationRequest,
    ) -> Result<DeleteNotificationResponse, RusotoError<DeleteNotificationError>>;

    /// <p><p>Deletes a subscriber.</p> <important> <p>Deleting the last subscriber to a notification also deletes the notification.</p> </important></p>
    async fn delete_subscriber(
        &self,
        input: DeleteSubscriberRequest,
    ) -> Result<DeleteSubscriberResponse, RusotoError<DeleteSubscriberError>>;

    /// <p><p>Describes a budget.</p> <important> <p>The Request Syntax section shows the <code>BudgetLimit</code> syntax. For <code>PlannedBudgetLimits</code>, see the <a href="https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_budgets_DescribeBudget.html#API_DescribeBudget_Examples">Examples</a> section. </p> </important></p>
    async fn describe_budget(
        &self,
        input: DescribeBudgetRequest,
    ) -> Result<DescribeBudgetResponse, RusotoError<DescribeBudgetError>>;

    /// <p>Describes the history for <code>DAILY</code>, <code>MONTHLY</code>, and <code>QUARTERLY</code> budgets. Budget history isn't available for <code>ANNUAL</code> budgets.</p>
    async fn describe_budget_performance_history(
        &self,
        input: DescribeBudgetPerformanceHistoryRequest,
    ) -> Result<
        DescribeBudgetPerformanceHistoryResponse,
        RusotoError<DescribeBudgetPerformanceHistoryError>,
    >;

    /// <p><p>Lists the budgets that are associated with an account.</p> <important> <p>The Request Syntax section shows the <code>BudgetLimit</code> syntax. For <code>PlannedBudgetLimits</code>, see the <a href="https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_budgets_DescribeBudgets.html#API_DescribeBudgets_Examples">Examples</a> section. </p> </important></p>
    async fn describe_budgets(
        &self,
        input: DescribeBudgetsRequest,
    ) -> Result<DescribeBudgetsResponse, RusotoError<DescribeBudgetsError>>;

    /// <p>Lists the notifications that are associated with a budget.</p>
    async fn describe_notifications_for_budget(
        &self,
        input: DescribeNotificationsForBudgetRequest,
    ) -> Result<
        DescribeNotificationsForBudgetResponse,
        RusotoError<DescribeNotificationsForBudgetError>,
    >;

    /// <p>Lists the subscribers that are associated with a notification.</p>
    async fn describe_subscribers_for_notification(
        &self,
        input: DescribeSubscribersForNotificationRequest,
    ) -> Result<
        DescribeSubscribersForNotificationResponse,
        RusotoError<DescribeSubscribersForNotificationError>,
    >;

    /// <p><p>Updates a budget. You can change every part of a budget except for the <code>budgetName</code> and the <code>calculatedSpend</code>. When you modify a budget, the <code>calculatedSpend</code> drops to zero until AWS has new usage data to use for forecasting.</p> <important> <p>Only one of <code>BudgetLimit</code> or <code>PlannedBudgetLimits</code> can be present in the syntax at one time. Use the syntax that matches your case. The Request Syntax section shows the <code>BudgetLimit</code> syntax. For <code>PlannedBudgetLimits</code>, see the <a href="https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_budgets_UpdateBudget.html#API_UpdateBudget_Examples">Examples</a> section. </p> </important></p>
    async fn update_budget(
        &self,
        input: UpdateBudgetRequest,
    ) -> Result<UpdateBudgetResponse, RusotoError<UpdateBudgetError>>;

    /// <p>Updates a notification.</p>
    async fn update_notification(
        &self,
        input: UpdateNotificationRequest,
    ) -> Result<UpdateNotificationResponse, RusotoError<UpdateNotificationError>>;

    /// <p>Updates a subscriber.</p>
    async fn update_subscriber(
        &self,
        input: UpdateSubscriberRequest,
    ) -> Result<UpdateSubscriberResponse, RusotoError<UpdateSubscriberError>>;
}
/// A client for the AWSBudgets API.
#[derive(Clone)]
pub struct BudgetsClient {
    client: Client,
    region: region::Region,
}

impl BudgetsClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> BudgetsClient {
        BudgetsClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> BudgetsClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        BudgetsClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> BudgetsClient {
        BudgetsClient { client, region }
    }
}

#[async_trait]
impl Budgets for BudgetsClient {
    /// <p><p>Creates a budget and, if included, notifications and subscribers. </p> <important> <p>Only one of <code>BudgetLimit</code> or <code>PlannedBudgetLimits</code> can be present in the syntax at one time. Use the syntax that matches your case. The Request Syntax section shows the <code>BudgetLimit</code> syntax. For <code>PlannedBudgetLimits</code>, see the <a href="https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_budgets_CreateBudget.html#API_CreateBudget_Examples">Examples</a> section. </p> </important></p>
    async fn create_budget(
        &self,
        input: CreateBudgetRequest,
    ) -> Result<CreateBudgetResponse, RusotoError<CreateBudgetError>> {
        let mut request = SignedRequest::new("POST", "budgets", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSBudgetServiceGateway.CreateBudget");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateBudgetResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateBudgetError::from_response(response))
        }
    }

    /// <p>Creates a notification. You must create the budget before you create the associated notification.</p>
    async fn create_notification(
        &self,
        input: CreateNotificationRequest,
    ) -> Result<CreateNotificationResponse, RusotoError<CreateNotificationError>> {
        let mut request = SignedRequest::new("POST", "budgets", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSBudgetServiceGateway.CreateNotification");
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
                .deserialize::<CreateNotificationResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateNotificationError::from_response(response))
        }
    }

    /// <p>Creates a subscriber. You must create the associated budget and notification before you create the subscriber.</p>
    async fn create_subscriber(
        &self,
        input: CreateSubscriberRequest,
    ) -> Result<CreateSubscriberResponse, RusotoError<CreateSubscriberError>> {
        let mut request = SignedRequest::new("POST", "budgets", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSBudgetServiceGateway.CreateSubscriber");
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
                .deserialize::<CreateSubscriberResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateSubscriberError::from_response(response))
        }
    }

    /// <p><p>Deletes a budget. You can delete your budget at any time.</p> <important> <p>Deleting a budget also deletes the notifications and subscribers that are associated with that budget.</p> </important></p>
    async fn delete_budget(
        &self,
        input: DeleteBudgetRequest,
    ) -> Result<DeleteBudgetResponse, RusotoError<DeleteBudgetError>> {
        let mut request = SignedRequest::new("POST", "budgets", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSBudgetServiceGateway.DeleteBudget");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteBudgetResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteBudgetError::from_response(response))
        }
    }

    /// <p><p>Deletes a notification.</p> <important> <p>Deleting a notification also deletes the subscribers that are associated with the notification.</p> </important></p>
    async fn delete_notification(
        &self,
        input: DeleteNotificationRequest,
    ) -> Result<DeleteNotificationResponse, RusotoError<DeleteNotificationError>> {
        let mut request = SignedRequest::new("POST", "budgets", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSBudgetServiceGateway.DeleteNotification");
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
                .deserialize::<DeleteNotificationResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteNotificationError::from_response(response))
        }
    }

    /// <p><p>Deletes a subscriber.</p> <important> <p>Deleting the last subscriber to a notification also deletes the notification.</p> </important></p>
    async fn delete_subscriber(
        &self,
        input: DeleteSubscriberRequest,
    ) -> Result<DeleteSubscriberResponse, RusotoError<DeleteSubscriberError>> {
        let mut request = SignedRequest::new("POST", "budgets", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSBudgetServiceGateway.DeleteSubscriber");
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
                .deserialize::<DeleteSubscriberResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteSubscriberError::from_response(response))
        }
    }

    /// <p><p>Describes a budget.</p> <important> <p>The Request Syntax section shows the <code>BudgetLimit</code> syntax. For <code>PlannedBudgetLimits</code>, see the <a href="https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_budgets_DescribeBudget.html#API_DescribeBudget_Examples">Examples</a> section. </p> </important></p>
    async fn describe_budget(
        &self,
        input: DescribeBudgetRequest,
    ) -> Result<DescribeBudgetResponse, RusotoError<DescribeBudgetError>> {
        let mut request = SignedRequest::new("POST", "budgets", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSBudgetServiceGateway.DescribeBudget");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeBudgetResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeBudgetError::from_response(response))
        }
    }

    /// <p>Describes the history for <code>DAILY</code>, <code>MONTHLY</code>, and <code>QUARTERLY</code> budgets. Budget history isn't available for <code>ANNUAL</code> budgets.</p>
    async fn describe_budget_performance_history(
        &self,
        input: DescribeBudgetPerformanceHistoryRequest,
    ) -> Result<
        DescribeBudgetPerformanceHistoryResponse,
        RusotoError<DescribeBudgetPerformanceHistoryError>,
    > {
        let mut request = SignedRequest::new("POST", "budgets", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSBudgetServiceGateway.DescribeBudgetPerformanceHistory",
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
                .deserialize::<DescribeBudgetPerformanceHistoryResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeBudgetPerformanceHistoryError::from_response(
                response,
            ))
        }
    }

    /// <p><p>Lists the budgets that are associated with an account.</p> <important> <p>The Request Syntax section shows the <code>BudgetLimit</code> syntax. For <code>PlannedBudgetLimits</code>, see the <a href="https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_budgets_DescribeBudgets.html#API_DescribeBudgets_Examples">Examples</a> section. </p> </important></p>
    async fn describe_budgets(
        &self,
        input: DescribeBudgetsRequest,
    ) -> Result<DescribeBudgetsResponse, RusotoError<DescribeBudgetsError>> {
        let mut request = SignedRequest::new("POST", "budgets", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSBudgetServiceGateway.DescribeBudgets");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeBudgetsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeBudgetsError::from_response(response))
        }
    }

    /// <p>Lists the notifications that are associated with a budget.</p>
    async fn describe_notifications_for_budget(
        &self,
        input: DescribeNotificationsForBudgetRequest,
    ) -> Result<
        DescribeNotificationsForBudgetResponse,
        RusotoError<DescribeNotificationsForBudgetError>,
    > {
        let mut request = SignedRequest::new("POST", "budgets", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSBudgetServiceGateway.DescribeNotificationsForBudget",
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
                .deserialize::<DescribeNotificationsForBudgetResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeNotificationsForBudgetError::from_response(response))
        }
    }

    /// <p>Lists the subscribers that are associated with a notification.</p>
    async fn describe_subscribers_for_notification(
        &self,
        input: DescribeSubscribersForNotificationRequest,
    ) -> Result<
        DescribeSubscribersForNotificationResponse,
        RusotoError<DescribeSubscribersForNotificationError>,
    > {
        let mut request = SignedRequest::new("POST", "budgets", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSBudgetServiceGateway.DescribeSubscribersForNotification",
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
                .deserialize::<DescribeSubscribersForNotificationResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeSubscribersForNotificationError::from_response(
                response,
            ))
        }
    }

    /// <p><p>Updates a budget. You can change every part of a budget except for the <code>budgetName</code> and the <code>calculatedSpend</code>. When you modify a budget, the <code>calculatedSpend</code> drops to zero until AWS has new usage data to use for forecasting.</p> <important> <p>Only one of <code>BudgetLimit</code> or <code>PlannedBudgetLimits</code> can be present in the syntax at one time. Use the syntax that matches your case. The Request Syntax section shows the <code>BudgetLimit</code> syntax. For <code>PlannedBudgetLimits</code>, see the <a href="https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_budgets_UpdateBudget.html#API_UpdateBudget_Examples">Examples</a> section. </p> </important></p>
    async fn update_budget(
        &self,
        input: UpdateBudgetRequest,
    ) -> Result<UpdateBudgetResponse, RusotoError<UpdateBudgetError>> {
        let mut request = SignedRequest::new("POST", "budgets", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSBudgetServiceGateway.UpdateBudget");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UpdateBudgetResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateBudgetError::from_response(response))
        }
    }

    /// <p>Updates a notification.</p>
    async fn update_notification(
        &self,
        input: UpdateNotificationRequest,
    ) -> Result<UpdateNotificationResponse, RusotoError<UpdateNotificationError>> {
        let mut request = SignedRequest::new("POST", "budgets", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSBudgetServiceGateway.UpdateNotification");
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
                .deserialize::<UpdateNotificationResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateNotificationError::from_response(response))
        }
    }

    /// <p>Updates a subscriber.</p>
    async fn update_subscriber(
        &self,
        input: UpdateSubscriberRequest,
    ) -> Result<UpdateSubscriberResponse, RusotoError<UpdateSubscriberError>> {
        let mut request = SignedRequest::new("POST", "budgets", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSBudgetServiceGateway.UpdateSubscriber");
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
                .deserialize::<UpdateSubscriberResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateSubscriberError::from_response(response))
        }
    }
}
