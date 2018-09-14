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
use rusoto_core::region;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::{Client, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_str;
use serde_json::Value as SerdeJsonValue;
/// <p>The amount of instance usage that a reservation covered.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Coverage {
    /// <p>The amount of instance usage that a reservation covered, in hours.</p>
    #[serde(rename = "CoverageHours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage_hours: Option<CoverageHours>,
}

/// <p>Reservation coverage for a specified period, in hours.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CoverageByTime {
    /// <p>The groups of instances that are covered by a reservation.</p>
    #[serde(rename = "Groups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<ReservationCoverageGroup>>,
    /// <p>The period over which this coverage was used.</p>
    #[serde(rename = "TimePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_period: Option<DateInterval>,
    /// <p>The total reservation coverage, in hours.</p>
    #[serde(rename = "Total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<Coverage>,
}

/// <p>How long a running instance either used a reservation or was On-Demand.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CoverageHours {
    /// <p>The percentage of instance hours that are covered by a reservation.</p>
    #[serde(rename = "CoverageHoursPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage_hours_percentage: Option<String>,
    /// <p>The number of instance running hours that are covered by On-Demand Instances.</p>
    #[serde(rename = "OnDemandHours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_hours: Option<String>,
    /// <p>The number of instance running hours that are covered by reservations.</p>
    #[serde(rename = "ReservedHours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_hours: Option<String>,
    /// <p>The total instance usage, in hours.</p>
    #[serde(rename = "TotalRunningHours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_running_hours: Option<String>,
}

/// <p>The time period that you want the usage and costs for. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DateInterval {
    /// <p>The end of the time period that you want the usage and costs for. The end date is exclusive. For example, if <code>end</code> is <code>2017-05-01</code>, AWS retrieves cost and usage data from the start date up to, but not including, <code>2017-05-01</code>.</p>
    #[serde(rename = "End")]
    pub end: String,
    /// <p>The beginning of the time period that you want the usage and costs for. The start date is inclusive. For example, if <code>start</code> is <code>2017-01-01</code>, AWS retrieves cost and usage data starting at <code>2017-01-01</code> up to the end date.</p>
    #[serde(rename = "Start")]
    pub start: String,
}

/// <p>The metadata that you can use to filter and group your results. You can use <code>GetDimensionValues</code> to find specific values.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DimensionValues {
    /// <p>The names of the metadata types that you can use to filter and group your results. For example, <code>AZ</code> returns a list of Availability Zones.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The metadata values that you can use to filter and group your results. You can use <code>GetDimensionValues</code> to find specific values.</p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// <p>The metadata of a specific type that you can use to filter and group your results. You can use <code>GetDimensionValues</code> to find specific values.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DimensionValuesWithAttributes {
    /// <p>The attribute that applies to a specific <code>Dimension</code>.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>The value of a dimension with a specific attribute.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Details about the EC2 instances that AWS recommends that you purchase.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct EC2InstanceDetails {
    /// <p>The Availability Zone of the recommended reservation.</p>
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>Whether the recommendation is for a current generation instance. </p>
    #[serde(rename = "CurrentGeneration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_generation: Option<bool>,
    /// <p>The instance family of the recommended reservation.</p>
    #[serde(rename = "Family")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    /// <p>The type of instance that AWS recommends.</p>
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p>The platform of the recommended reservation. The platform is the specific combination of operating system, license model, and software on an instance.</p>
    #[serde(rename = "Platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>The AWS Region of the recommended reservation.</p>
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>Whether the recommended reservation is size flexible.</p>
    #[serde(rename = "SizeFlexEligible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_flex_eligible: Option<bool>,
    /// <p>Whether the recommended reservation is dedicated or shared.</p>
    #[serde(rename = "Tenancy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenancy: Option<String>,
}

/// <p>The EC2 hardware specifications that you want AWS to provide recommendations for.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EC2Specification {
    /// <p>Whether you want a recommendation for standard or convertible reservations.</p>
    #[serde(rename = "OfferingClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_class: Option<String>,
}

/// <p>Details about the ES instances that AWS recommends that you purchase.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ESInstanceDetails {
    /// <p>Whether the recommendation is for a current generation instance.</p>
    #[serde(rename = "CurrentGeneration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_generation: Option<bool>,
    /// <p>The class of instance that AWS recommends.</p>
    #[serde(rename = "InstanceClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_class: Option<String>,
    /// <p>The size of instance that AWS recommends.</p>
    #[serde(rename = "InstanceSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_size: Option<String>,
    /// <p>The AWS Region of the recommended reservation.</p>
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>Whether the recommended reservation is size flexible.</p>
    #[serde(rename = "SizeFlexEligible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_flex_eligible: Option<bool>,
}

/// <p>Details about the ElastiCache instances that AWS recommends that you purchase.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ElastiCacheInstanceDetails {
    /// <p>Whether the recommendation is for a current generation instance.</p>
    #[serde(rename = "CurrentGeneration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_generation: Option<bool>,
    /// <p>The instance family of the recommended reservation.</p>
    #[serde(rename = "Family")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    /// <p>The type of node that AWS recommends.</p>
    #[serde(rename = "NodeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    /// <p>The description of the recommended reservation.</p>
    #[serde(rename = "ProductDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,
    /// <p>The AWS Region of the recommended reservation.</p>
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>Whether the recommended reservation is size flexible.</p>
    #[serde(rename = "SizeFlexEligible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_flex_eligible: Option<bool>,
}

/// <p><p>Use <code>Expression</code> to filter by cost or by usage. There are two patterns: </p> <ul> <li> <p>Simple dimension values - You can set the dimension name and values for the filters that you plan to use. For example, you can filter for <code>INSTANCE<em>TYPE==m4.xlarge OR INSTANCE</em>TYPE==c4.large</code>. The <code>Expression</code> for that looks like this:</p> <p> <code>{ &quot;Dimensions&quot;: { &quot;Key&quot;: &quot;INSTANCE<em>TYPE&quot;, &quot;Values&quot;: [ &quot;m4.xlarge&quot;, “c4.large” ] } }</code> </p> <p>The list of dimension values are OR&#39;d together to retrieve cost or usage data. You can create <code>Expression</code> and <code>DimensionValues</code> objects using either <code>with<em></code> methods or <code>set</em></code> methods in multiple lines. </p> </li> <li> <p>Compound dimension values with logical operations - You can use multiple <code>Expression</code> types and the logical operators <code>AND/OR/NOT</code> to create a list of one or more <code>Expression</code> objects. This allows you to filter on more advanced options. For example, you can filter on <code>((INSTANCE</em>TYPE == m4.large OR INSTANCE<em>TYPE == m3.large) OR (TAG.Type == Type1)) AND (USAGE</em>TYPE != DataTransfer)</code>. The <code>Expression</code> for that looks like this:</p> <p> <code>{ &quot;And&quot;: [ {&quot;Or&quot;: [ {&quot;Dimensions&quot;: { &quot;Key&quot;: &quot;INSTANCE<em>TYPE&quot;, &quot;Values&quot;: [ &quot;m4.x.large&quot;, &quot;c4.large&quot; ] }}, {&quot;Tags&quot;: { &quot;Key&quot;: &quot;TagName&quot;, &quot;Values&quot;: [&quot;Value1&quot;] } } ]}, {&quot;Not&quot;: {&quot;Dimensions&quot;: { &quot;Key&quot;: &quot;USAGE</em>TYPE&quot;, &quot;Values&quot;: [&quot;DataTransfer&quot;] }}} ] } </code> </p> <note> <p>Because each <code>Expression</code> can have only one operator, the service returns an error if more than one is specified. The following example shows an <code>Expression</code> object that creates an error.</p> </note> <p> <code> { &quot;And&quot;: [ ... ], &quot;DimensionValues&quot;: { &quot;Dimension&quot;: &quot;USAGE_TYPE&quot;, &quot;Values&quot;: [ &quot;DataTransfer&quot; ] } } </code> </p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Expression {
    /// <p>Return results that match both <code>Dimension</code> objects.</p>
    #[serde(rename = "And")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and: Option<Vec<Expression>>,
    /// <p>The specific <code>Dimension</code> to use for <code>Expression</code>.</p>
    #[serde(rename = "Dimensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<DimensionValues>,
    /// <p>Return results that don't match a <code>Dimension</code> object.</p>
    #[serde(rename = "Not")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not: Option<Expression>,
    /// <p>Return results that match either <code>Dimension</code> object.</p>
    #[serde(rename = "Or")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or: Option<Vec<Expression>>,
    /// <p>The specific <code>Tag</code> to use for <code>Expression</code>.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagValues>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetCostAndUsageRequest {
    /// <p>Filters AWS costs by different dimensions. For example, you can specify <code>SERVICE</code> and <code>LINKED_ACCOUNT</code> and get the costs that are associated with that account's usage of that service. You can nest <code>Expression</code> objects to define any combination of dimension filters. For more information, see <a href="http://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_Expression.html">Expression</a>. </p>
    #[serde(rename = "Filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression>,
    /// <p>Sets the AWS cost granularity to <code>MONTHLY</code> or <code>DAILY</code>. If <code>Granularity</code> isn't set, the response object doesn't include the <code>Granularity</code>, either <code>MONTHLY</code> or <code>DAILY</code>. </p>
    #[serde(rename = "Granularity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<String>,
    /// <p>You can group AWS costs using up to two different groups, either dimensions, tag keys, or both.</p> <p>When you group by tag key, you get all tag values, including empty strings.</p> <p>Valid values are <code>AZ</code>, <code>INSTANCE_TYPE</code>, <code>LEGAL_ENTITY_NAME</code>, <code>LINKED_ACCOUNT</code>, <code>OPERATION</code>, <code>PLATFORM</code>, <code>PURCHASE_TYPE</code>, <code>SERVICE</code>, <code>TAGS</code>, <code>TENANCY</code>, and <code>USAGE_TYPE</code>.</p>
    #[serde(rename = "GroupBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<GroupDefinition>>,
    /// <p>Which metrics are returned in the query. For more information about blended and unblended rates, see <a href="https://aws.amazon.com/premiumsupport/knowledge-center/blended-rates-intro/">Why does the "blended" annotation appear on some line items in my bill?</a>. </p> <p>Valid values are <code>AmortizedCost</code>, <code>BlendedCost</code>, <code>UnblendedCost</code>, and <code>UsageQuantity</code>.</p> <note> <p>If you return the <code>UsageQuantity</code> metric, the service aggregates all usage numbers without taking into account the units. For example, if you aggregate <code>usageQuantity</code> across all of EC2, the results aren't meaningful because EC2 compute hours and data transfer are measured in different units (for example, hours vs. GB). To get more meaningful <code>UsageQuantity</code> metrics, filter by <code>UsageType</code> or <code>UsageTypeGroups</code>. </p> </note> <p> <code>Metrics</code> is required for <code>GetCostAndUsage</code> requests.</p>
    #[serde(rename = "Metrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<String>>,
    /// <p>The token to retrieve the next set of results. AWS provides the token when the response from a previous call has more results than the maximum page size.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>Sets the start and end dates for retrieving AWS costs. The start date is inclusive, but the end date is exclusive. For example, if <code>start</code> is <code>2017-01-01</code> and <code>end</code> is <code>2017-05-01</code>, then the cost and usage data is retrieved from <code>2017-01-01</code> up to and including <code>2017-04-30</code> but not including <code>2017-05-01</code>.</p>
    #[serde(rename = "TimePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_period: Option<DateInterval>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetCostAndUsageResponse {
    /// <p>The groups that are specified by the <code>Filter</code> or <code>GroupBy</code> parameters in the request.</p>
    #[serde(rename = "GroupDefinitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_definitions: Option<Vec<GroupDefinition>>,
    /// <p>The token for the next set of retrievable results. AWS provides the token when the response from a previous call has more results than the maximum page size.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>The time period that is covered by the results in the response.</p>
    #[serde(rename = "ResultsByTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results_by_time: Option<Vec<ResultByTime>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDimensionValuesRequest {
    /// <p><p>The context for the call to <code>GetDimensionValues</code>. This can be <code>RESERVATIONS</code> or <code>COST<em>AND</em>USAGE</code>. The default value is <code>COST<em>AND</em>USAGE</code>. If the context is set to <code>RESERVATIONS</code>, the resulting dimension values can be used in the <code>GetReservationUtilization</code> operation. If the context is set to <code>COST<em>AND</em>USAGE</code> the resulting dimension values can be used in the <code>GetCostAndUsage</code> operation.</p> <p>If you set the context to <code>COST<em>AND</em>USAGE</code>, you can use the following dimensions for searching:</p> <ul> <li> <p>AZ - The Availability Zone. An example is <code>us-east-1a</code>.</p> </li> <li> <p>DATABASE<em>ENGINE - The Amazon Relational Database Service database. Examples are Aurora or MySQL.</p> </li> <li> <p>INSTANCE</em>TYPE - The type of EC2 instance. An example is <code>m4.xlarge</code>.</p> </li> <li> <p>LEGAL<em>ENTITY</em>NAME - The name of the organization that sells you AWS services, such as Amazon Web Services.</p> </li> <li> <p>LINKED<em>ACCOUNT - The description in the attribute map that includes the full name of the member account. The value field contains the AWS ID of the member account.</p> </li> <li> <p>OPERATING</em>SYSTEM - The operating system. Examples are Windows or Linux.</p> </li> <li> <p>OPERATION - The action performed. Examples include <code>RunInstance</code> and <code>CreateBucket</code>.</p> </li> <li> <p>PLATFORM - The EC2 operating system. Examples are Windows or Linux.</p> </li> <li> <p>PURCHASE<em>TYPE - The reservation type of the purchase to which this usage is related. Examples include On-Demand Instances and Standard Reserved Instances.</p> </li> <li> <p>SERVICE - The AWS service such as Amazon DynamoDB.</p> </li> <li> <p>USAGE</em>TYPE - The type of usage. An example is DataTransfer-In-Bytes. The response for the <code>GetDimensionValues</code> operation includes a unit attribute. Examples include GB and Hrs.</p> </li> <li> <p>USAGE<em>TYPE</em>GROUP - The grouping of common usage types. An example is EC2: CloudWatch – Alarms. The response for this operation includes a unit attribute.</p> </li> <li> <p>RECORD<em>TYPE - The different types of charges such as RI fees, usage costs, tax refunds, and credits.</p> </li> </ul> <p>If you set the context to <code>RESERVATIONS</code>, you can use the following dimensions for searching:</p> <ul> <li> <p>AZ - The Availability Zone. An example is <code>us-east-1a</code>.</p> </li> <li> <p>CACHE</em>ENGINE - The Amazon ElastiCache operating system. Examples are Windows or Linux.</p> </li> <li> <p>DEPLOYMENT<em>OPTION - The scope of Amazon Relational Database Service deployments. Valid values are <code>SingleAZ</code> and <code>MultiAZ</code>.</p> </li> <li> <p>INSTANCE</em>TYPE - The type of EC2 instance. An example is <code>m4.xlarge</code>.</p> </li> <li> <p>LINKED_ACCOUNT - The description in the attribute map that includes the full name of the member account. The value field contains the AWS ID of the member account.</p> </li> <li> <p>PLATFORM - The EC2 operating system. Examples are Windows or Linux.</p> </li> <li> <p>REGION - The AWS Region.</p> </li> <li> <p>SCOPE (Utilization only) - The scope of a Reserved Instance (RI). Values are regional or a single Availability Zone.</p> </li> <li> <p>TAG (Coverage only) - The tags that are associated with a Reserved Instance (RI).</p> </li> <li> <p>TENANCY - The tenancy of a resource. Examples are shared or dedicated.</p> </li> </ul></p>
    #[serde(rename = "Context")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    /// <p>The name of the dimension. Each <code>Dimension</code> is available for different a <code>Context</code>. For more information, see <code>Context</code>.</p>
    #[serde(rename = "Dimension")]
    pub dimension: String,
    /// <p>The token to retrieve the next set of results. AWS provides the token when the response from a previous call has more results than the maximum page size.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>The value that you want to search the filter values for.</p>
    #[serde(rename = "SearchString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_string: Option<String>,
    /// <p>The start and end dates for retrieving the dimension values. The start date is inclusive, but the end date is exclusive. For example, if <code>start</code> is <code>2017-01-01</code> and <code>end</code> is <code>2017-05-01</code>, then the cost and usage data is retrieved from <code>2017-01-01</code> up to and including <code>2017-04-30</code> but not including <code>2017-05-01</code>.</p>
    #[serde(rename = "TimePeriod")]
    pub time_period: DateInterval,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetDimensionValuesResponse {
    /// <p><p>The filters that you used to filter your request. Some dimensions are available only for a specific context:</p> <p>If you set the context to <code>COST<em>AND</em>USAGE</code>, you can use the following dimensions for searching:</p> <ul> <li> <p>AZ - The Availability Zone. An example is <code>us-east-1a</code>.</p> </li> <li> <p>DATABASE<em>ENGINE - The Amazon Relational Database Service database. Examples are Aurora or MySQL.</p> </li> <li> <p>INSTANCE</em>TYPE - The type of EC2 instance. An example is <code>m4.xlarge</code>.</p> </li> <li> <p>LEGAL<em>ENTITY</em>NAME - The name of the organization that sells you AWS services, such as Amazon Web Services.</p> </li> <li> <p>LINKED<em>ACCOUNT - The description in the attribute map that includes the full name of the member account. The value field contains the AWS ID of the member account.</p> </li> <li> <p>OPERATING</em>SYSTEM - The operating system. Examples are Windows or Linux.</p> </li> <li> <p>OPERATION - The action performed. Examples include <code>RunInstance</code> and <code>CreateBucket</code>.</p> </li> <li> <p>PLATFORM - The EC2 operating system. Examples are Windows or Linux.</p> </li> <li> <p>PURCHASE<em>TYPE - The reservation type of the purchase to which this usage is related. Examples include On-Demand Instances and Standard Reserved Instances.</p> </li> <li> <p>SERVICE - The AWS service such as Amazon DynamoDB.</p> </li> <li> <p>USAGE</em>TYPE - The type of usage. An example is DataTransfer-In-Bytes. The response for the <code>GetDimensionValues</code> operation includes a unit attribute. Examples include GB and Hrs.</p> </li> <li> <p>USAGE<em>TYPE</em>GROUP - The grouping of common usage types. An example is EC2: CloudWatch – Alarms. The response for this operation includes a unit attribute.</p> </li> <li> <p>RECORD<em>TYPE - The different types of charges such as RI fees, usage costs, tax refunds, and credits.</p> </li> </ul> <p>If you set the context to <code>RESERVATIONS</code>, you can use the following dimensions for searching:</p> <ul> <li> <p>AZ - The Availability Zone. An example is <code>us-east-1a</code>.</p> </li> <li> <p>CACHE</em>ENGINE - The Amazon ElastiCache operating system. Examples are Windows or Linux.</p> </li> <li> <p>DEPLOYMENT<em>OPTION - The scope of Amazon Relational Database Service deployments. Valid values are <code>SingleAZ</code> and <code>MultiAZ</code>.</p> </li> <li> <p>INSTANCE</em>TYPE - The type of EC2 instance. An example is <code>m4.xlarge</code>.</p> </li> <li> <p>LINKED_ACCOUNT - The description in the attribute map that includes the full name of the member account. The value field contains the AWS ID of the member account.</p> </li> <li> <p>PLATFORM - The EC2 operating system. Examples are Windows or Linux.</p> </li> <li> <p>REGION - The AWS Region.</p> </li> <li> <p>SCOPE (Utilization only) - The scope of a Reserved Instance (RI). Values are regional or a single Availability Zone.</p> </li> <li> <p>TAG (Coverage only) - The tags that are associated with a Reserved Instance (RI).</p> </li> <li> <p>TENANCY - The tenancy of a resource. Examples are shared or dedicated.</p> </li> </ul></p>
    #[serde(rename = "DimensionValues")]
    pub dimension_values: Vec<DimensionValuesWithAttributes>,
    /// <p>The token for the next set of retrievable results. AWS provides the token when the response from a previous call has more results than the maximum page size.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>The number of results that AWS returned at one time.</p>
    #[serde(rename = "ReturnSize")]
    pub return_size: i64,
    /// <p>The total number of search results.</p>
    #[serde(rename = "TotalSize")]
    pub total_size: i64,
}

/// <p>You can use the following request parameters to query for how much of your instance usage is covered by a reservation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetReservationCoverageRequest {
    /// <p>Filters utilization data by dimensions. You can filter by the following dimensions:</p> <ul> <li> <p>AZ</p> </li> <li> <p>CACHE_ENGINE</p> </li> <li> <p>DATABASE_ENGINE</p> </li> <li> <p>DEPLOYMENT_OPTION</p> </li> <li> <p>INSTANCE_TYPE</p> </li> <li> <p>LINKED_ACCOUNT</p> </li> <li> <p>OPERATING_SYSTEM</p> </li> <li> <p>PLATFORM</p> </li> <li> <p>REGION</p> </li> <li> <p>SERVICE</p> </li> <li> <p>TAG</p> </li> <li> <p>TENANCY</p> </li> </ul> <p> <code>GetReservationCoverage</code> uses the same <code> <a href="http://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_Expression.html">Expression</a> </code> object as the other operations, but only <code>AND</code> is supported among each dimension. You can nest only one level deep. If there are multiple values for a dimension, they are OR'd together.</p> <p>If you don't provide a <code>SERVICE</code> filter, Cost Explorer defaults to EC2.</p>
    #[serde(rename = "Filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression>,
    /// <p>The granularity of the AWS cost data for the reservation. Valid values are <code>MONTHLY</code> and <code>DAILY</code>.</p> <p>If <code>GroupBy</code> is set, <code>Granularity</code> can't be set. If <code>Granularity</code> isn't set, the response object doesn't include <code>Granularity</code>, either <code>MONTHLY</code> or <code>DAILY</code>.</p>
    #[serde(rename = "Granularity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<String>,
    /// <p><p>You can group the data by the following attributes:</p> <ul> <li> <p>AZ</p> </li> <li> <p>CACHE<em>ENGINE</p> </li> <li> <p>DATABASE</em>ENGINE</p> </li> <li> <p>DEPLOYMENT<em>OPTION</p> </li> <li> <p>INSTANCE</em>TYPE</p> </li> <li> <p>LINKED<em>ACCOUNT</p> </li> <li> <p>OPERATING</em>SYSTEM</p> </li> <li> <p>PLATFORM</p> </li> <li> <p>REGION</p> </li> <li> <p>TAG</p> </li> <li> <p>TENANCY</p> </li> </ul></p>
    #[serde(rename = "GroupBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<GroupDefinition>>,
    /// <p>The token to retrieve the next set of results. AWS provides the token when the response from a previous call has more results than the maximum page size.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>The start and end dates of the period for which you want to retrieve data about reservation coverage. You can retrieve data for a maximum of 13 months: the last 12 months and the current month. The start date is inclusive, but the end date is exclusive. For example, if <code>start</code> is <code>2017-01-01</code> and <code>end</code> is <code>2017-05-01</code>, then the cost and usage data is retrieved from <code>2017-01-01</code> up to and including <code>2017-04-30</code> but not including <code>2017-05-01</code>. </p>
    #[serde(rename = "TimePeriod")]
    pub time_period: DateInterval,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetReservationCoverageResponse {
    /// <p>The amount of time that your reservations covered.</p>
    #[serde(rename = "CoveragesByTime")]
    pub coverages_by_time: Vec<CoverageByTime>,
    /// <p>The token for the next set of retrievable results. AWS provides the token when the response from a previous call has more results than the maximum page size.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>The total amount of instance usage that is covered by a reservation.</p>
    #[serde(rename = "Total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<Coverage>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetReservationPurchaseRecommendationRequest {
    /// <p>The account ID that is associated with the recommendation. </p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The account scope that you want recommendations for. <code>PAYER</code> means that AWS includes the master account and any member accounts when it calculates its recommendations. <code>LINKED</code> means that AWS includes only member accounts when it calculates its recommendations.</p> <p>Valid values are <code>PAYER</code> and <code>LINKED</code>.</p>
    #[serde(rename = "AccountScope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_scope: Option<String>,
    /// <p>The number of previous days that you want AWS to consider when it calculates your recommendations.</p>
    #[serde(rename = "LookbackPeriodInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookback_period_in_days: Option<String>,
    /// <p>The pagination token that indicates the next set of results that you want to retrieve.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>The number of recommendations that you want returned in a single response object.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The reservation purchase option that you want recommendations for.</p>
    #[serde(rename = "PaymentOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_option: Option<String>,
    /// <p>The specific service that you want recommendations for.</p>
    #[serde(rename = "Service")]
    pub service: String,
    /// <p>The hardware specifications for the service instances that you want recommendations for, such as standard or convertible EC2 instances.</p>
    #[serde(rename = "ServiceSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<ServiceSpecification>,
    /// <p>The reservation term that you want recommendations for.</p>
    #[serde(rename = "TermInYears")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term_in_years: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetReservationPurchaseRecommendationResponse {
    /// <p>Information about this specific recommendation call, such as the time stamp for when Cost Explorer generated this recommendation.</p>
    #[serde(rename = "Metadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ReservationPurchaseRecommendationMetadata>,
    /// <p>The pagination token for the next set of retrievable results.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>Recommendations for reservations to purchase.</p>
    #[serde(rename = "Recommendations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendations: Option<Vec<ReservationPurchaseRecommendation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetReservationUtilizationRequest {
    /// <p>Filters utilization data by dimensions. You can filter by the following dimensions:</p> <ul> <li> <p>AZ</p> </li> <li> <p>CACHE_ENGINE</p> </li> <li> <p>DATABASE_ENGINE</p> </li> <li> <p>DEPLOYMENT_OPTION</p> </li> <li> <p>INSTANCE_TYPE</p> </li> <li> <p>LINKED_ACCOUNT</p> </li> <li> <p>OPERATING_SYSTEM</p> </li> <li> <p>PLATFORM</p> </li> <li> <p>REGION</p> </li> <li> <p>SERVICE</p> </li> <li> <p>SCOPE</p> </li> <li> <p>TENANCY</p> </li> </ul> <p> <code>GetReservationUtilization</code> uses the same <code> <a href="http://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_Expression.html">Expression</a> </code> object as the other operations, but only <code>AND</code> is supported among each dimension, and nesting is supported up to only one level deep. If there are multiple values for a dimension, they are OR'd together.</p>
    #[serde(rename = "Filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression>,
    /// <p>If <code>GroupBy</code> is set, <code>Granularity</code> can't be set. If <code>Granularity</code> isn't set, the response object doesn't include <code>Granularity</code>, either <code>MONTHLY</code> or <code>DAILY</code>. If both <code>GroupBy</code> and <code>Granularity</code> aren't set, <code>GetReservationUtilization</code> defaults to <code>DAILY</code>.</p>
    #[serde(rename = "Granularity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<String>,
    /// <p>Groups only by <code>SUBSCRIPTION_ID</code>. Metadata is included.</p>
    #[serde(rename = "GroupBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<GroupDefinition>>,
    /// <p>The token to retrieve the next set of results. AWS provides the token when the response from a previous call has more results than the maximum page size.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>Sets the start and end dates for retrieving Reserved Instance (RI) utilization. The start date is inclusive, but the end date is exclusive. For example, if <code>start</code> is <code>2017-01-01</code> and <code>end</code> is <code>2017-05-01</code>, then the cost and usage data is retrieved from <code>2017-01-01</code> up to and including <code>2017-04-30</code> but not including <code>2017-05-01</code>. </p>
    #[serde(rename = "TimePeriod")]
    pub time_period: DateInterval,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetReservationUtilizationResponse {
    /// <p>The token for the next set of retrievable results. AWS provides the token when the response from a previous call has more results than the maximum page size.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>The total amount of time that you utilized your RIs.</p>
    #[serde(rename = "Total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<ReservationAggregates>,
    /// <p>The amount of time that you utilized your RIs.</p>
    #[serde(rename = "UtilizationsByTime")]
    pub utilizations_by_time: Vec<UtilizationByTime>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetTagsRequest {
    /// <p>The token to retrieve the next set of results. AWS provides the token when the response from a previous call has more results than the maximum page size.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>The value that you want to search for.</p>
    #[serde(rename = "SearchString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_string: Option<String>,
    /// <p>The key of the tag that you want to return values for.</p>
    #[serde(rename = "TagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    /// <p>The start and end dates for retrieving the dimension values. The start date is inclusive, but the end date is exclusive. For example, if <code>start</code> is <code>2017-01-01</code> and <code>end</code> is <code>2017-05-01</code>, then the cost and usage data is retrieved from <code>2017-01-01</code> up to and including <code>2017-04-30</code> but not including <code>2017-05-01</code>.</p>
    #[serde(rename = "TimePeriod")]
    pub time_period: DateInterval,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetTagsResponse {
    /// <p>The token for the next set of retrievable results. AWS provides the token when the response from a previous call has more results than the maximum page size.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>The number of query results that AWS returns at a time.</p>
    #[serde(rename = "ReturnSize")]
    pub return_size: i64,
    /// <p>The tags that match your request.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<String>,
    /// <p>The total number of query results.</p>
    #[serde(rename = "TotalSize")]
    pub total_size: i64,
}

/// <p>One level of grouped data within the results.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Group {
    /// <p>The keys that are included in this group.</p>
    #[serde(rename = "Keys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<String>>,
    /// <p>The metrics that are included in this group.</p>
    #[serde(rename = "Metrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<::std::collections::HashMap<String, MetricValue>>,
}

/// <p>Represents a group when you specify a group by criteria, or in the response to a query with a specific grouping.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupDefinition {
    /// <p>The string that represents a key for a specified group.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The string that represents the type of group.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Details about the instances that AWS recommends that you purchase.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct InstanceDetails {
    /// <p>The EC2 instances that AWS recommends that you purchase.</p>
    #[serde(rename = "EC2InstanceDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_instance_details: Option<EC2InstanceDetails>,
    /// <p>The Amazon ES instances that AWS recommends that you purchase.</p>
    #[serde(rename = "ESInstanceDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub es_instance_details: Option<ESInstanceDetails>,
    /// <p>The ElastiCache instances that AWS recommends that you purchase.</p>
    #[serde(rename = "ElastiCacheInstanceDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasti_cache_instance_details: Option<ElastiCacheInstanceDetails>,
    /// <p>The RDS instances that AWS recommends that you purchase.</p>
    #[serde(rename = "RDSInstanceDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rds_instance_details: Option<RDSInstanceDetails>,
    /// <p>The Amazon Redshift instances that AWS recommends that you purchase.</p>
    #[serde(rename = "RedshiftInstanceDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_instance_details: Option<RedshiftInstanceDetails>,
}

/// <p>The aggregated value for a metric.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct MetricValue {
    /// <p>The actual number that represents the metric.</p>
    #[serde(rename = "Amount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    /// <p>The unit that the metric is given in.</p>
    #[serde(rename = "Unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

/// <p>Details about the RDS instances that AWS recommends that you purchase.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RDSInstanceDetails {
    /// <p>Whether the recommendation is for a current generation instance. </p>
    #[serde(rename = "CurrentGeneration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_generation: Option<bool>,
    /// <p>The database edition that the recommended reservation supports.</p>
    #[serde(rename = "DatabaseEdition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_edition: Option<String>,
    /// <p>The database engine that the recommended reservation supports.</p>
    #[serde(rename = "DatabaseEngine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_engine: Option<String>,
    /// <p>Whether the recommendation is for a reservation in a single Availability Zone or a reservation with a backup in a second Availability Zone.</p>
    #[serde(rename = "DeploymentOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_option: Option<String>,
    /// <p>The instance family of the recommended reservation.</p>
    #[serde(rename = "Family")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    /// <p>The type of instance that AWS recommends.</p>
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p>The license model that the recommended reservation supports.</p>
    #[serde(rename = "LicenseModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_model: Option<String>,
    /// <p>The AWS Region of the recommended reservation.</p>
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>Whether the recommended reservation is size flexible.</p>
    #[serde(rename = "SizeFlexEligible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_flex_eligible: Option<bool>,
}

/// <p>Details about the Amazon Redshift instances that AWS recommends that you purchase.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RedshiftInstanceDetails {
    /// <p>Whether the recommendation is for a current generation instance.</p>
    #[serde(rename = "CurrentGeneration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_generation: Option<bool>,
    /// <p>The instance family of the recommended reservation.</p>
    #[serde(rename = "Family")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    /// <p>The type of node that AWS recommends.</p>
    #[serde(rename = "NodeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    /// <p>The AWS Region of the recommended reservation.</p>
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>Whether the recommended reservation is size flexible.</p>
    #[serde(rename = "SizeFlexEligible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_flex_eligible: Option<bool>,
}

/// <p>The aggregated numbers for your Reserved Instance (RI) usage.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ReservationAggregates {
    /// <p>The monthly cost of your RI, amortized over the RI period.</p>
    #[serde(rename = "AmortizedRecurringFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amortized_recurring_fee: Option<String>,
    /// <p>The upfront cost of your RI, amortized over the RI period.</p>
    #[serde(rename = "AmortizedUpfrontFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amortized_upfront_fee: Option<String>,
    /// <p>How much you saved due to purchasing and utilizing RIs. AWS calculates this by subtracting <code>TotalAmortizedFee</code> from <code>OnDemandCostOfRIHoursUsed</code>.</p>
    #[serde(rename = "NetRISavings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_ri_savings: Option<String>,
    /// <p>How much your RIs would cost if charged On-Demand rates.</p>
    #[serde(rename = "OnDemandCostOfRIHoursUsed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_cost_of_ri_hours_used: Option<String>,
    /// <p>How many RI hours that you purchased.</p>
    #[serde(rename = "PurchasedHours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchased_hours: Option<String>,
    /// <p>The total number of RI hours that you used.</p>
    #[serde(rename = "TotalActualHours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_actual_hours: Option<String>,
    /// <p>The total cost of your RI, amortized over the RI period.</p>
    #[serde(rename = "TotalAmortizedFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amortized_fee: Option<String>,
    /// <p>How much you could save if you use your entire reservation.</p>
    #[serde(rename = "TotalPotentialRISavings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_potential_ri_savings: Option<String>,
    /// <p>The number of RI hours that you didn't use.</p>
    #[serde(rename = "UnusedHours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unused_hours: Option<String>,
    /// <p>The percentage of RI time that you used.</p>
    #[serde(rename = "UtilizationPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utilization_percentage: Option<String>,
}

/// <p>A group of reservations that share a set of attributes.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ReservationCoverageGroup {
    /// <p>The attributes for this group of reservations.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>How much instance usage this group of reservations covered.</p>
    #[serde(rename = "Coverage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage: Option<Coverage>,
}

/// <p>A specific reservation that AWS recommends for purchase.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ReservationPurchaseRecommendation {
    /// <p>The account scope that AWS recommends that you purchase this instance for. For example, you can purchase this reservation for an entire organization in AWS Organizations.</p>
    #[serde(rename = "AccountScope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_scope: Option<String>,
    /// <p>How many days of previous usage that AWS considers when making this recommendation.</p>
    #[serde(rename = "LookbackPeriodInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookback_period_in_days: Option<String>,
    /// <p>The payment option for the reservation. For example, <code>AllUpfront</code> or <code>NoUpfront</code>.</p>
    #[serde(rename = "PaymentOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_option: Option<String>,
    /// <p>Details about the recommended purchases.</p>
    #[serde(rename = "RecommendationDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_details: Option<Vec<ReservationPurchaseRecommendationDetail>>,
    /// <p>A summary about the recommended purchase.</p>
    #[serde(rename = "RecommendationSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_summary: Option<ReservationPurchaseRecommendationSummary>,
    /// <p>Hardware specifications for the service that you want recommendations for.</p>
    #[serde(rename = "ServiceSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<ServiceSpecification>,
    /// <p>The term of the reservation that you want recommendations for, in years.</p>
    #[serde(rename = "TermInYears")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term_in_years: Option<String>,
}

/// <p>Details about your recommended reservation purchase.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ReservationPurchaseRecommendationDetail {
    /// <p>The average number of normalized units that you used in an hour during the historical period. AWS uses this to calculate your recommended reservation purchases.</p>
    #[serde(rename = "AverageNormalizedUnitsUsedPerHour")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_normalized_units_used_per_hour: Option<String>,
    /// <p>The average number of instances that you used in an hour during the historical period. AWS uses this to calculate your recommended reservation purchases.</p>
    #[serde(rename = "AverageNumberOfInstancesUsedPerHour")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_number_of_instances_used_per_hour: Option<String>,
    /// <p>The average utilization of your instances. AWS uses this to calculate your recommended reservation purchases.</p>
    #[serde(rename = "AverageUtilization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_utilization: Option<String>,
    /// <p>The currency code that AWS used to calculate the costs for this instance.</p>
    #[serde(rename = "CurrencyCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    /// <p>How long AWS estimates that it takes for this instance to start saving you money, in months.</p>
    #[serde(rename = "EstimatedBreakEvenInMonths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_break_even_in_months: Option<String>,
    /// <p>How much AWS estimates that you spend on On-Demand Instances in a month.</p>
    #[serde(rename = "EstimatedMonthlyOnDemandCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_monthly_on_demand_cost: Option<String>,
    /// <p>How much AWS estimates that this specific recommendation could save you in a month.</p>
    #[serde(rename = "EstimatedMonthlySavingsAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_monthly_savings_amount: Option<String>,
    /// <p>How much AWS estimates that this specific recommendation could save you in a month, as a percentage of your overall costs.</p>
    #[serde(rename = "EstimatedMonthlySavingsPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_monthly_savings_percentage: Option<String>,
    /// <p>How much AWS estimates that you would have spent for all usage during the specified historical period if you had had a reservation.</p>
    #[serde(rename = "EstimatedReservationCostForLookbackPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_reservation_cost_for_lookback_period: Option<String>,
    /// <p>Details about the instances that AWS recommends that you purchase.</p>
    #[serde(rename = "InstanceDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_details: Option<InstanceDetails>,
    /// <p>The maximum number of normalized units that you used in an hour during the historical period. AWS uses this to calculate your recommended reservation purchases.</p>
    #[serde(rename = "MaximumNormalizedUnitsUsedPerHour")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_normalized_units_used_per_hour: Option<String>,
    /// <p>The maximum number of instances that you used in an hour during the historical period. AWS uses this to calculate your recommended reservation purchases.</p>
    #[serde(rename = "MaximumNumberOfInstancesUsedPerHour")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_number_of_instances_used_per_hour: Option<String>,
    /// <p>The minimum number of hours that you used in an hour during the historical period. AWS uses this to calculate your recommended reservation purchases.</p>
    #[serde(rename = "MinimumNormalizedUnitsUsedPerHour")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_normalized_units_used_per_hour: Option<String>,
    /// <p>The minimum number of instances that you used in an hour during the historical period. AWS uses this to calculate your recommended reservation purchases.</p>
    #[serde(rename = "MinimumNumberOfInstancesUsedPerHour")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_number_of_instances_used_per_hour: Option<String>,
    /// <p>The number of normalized units that AWS recommends that you purchase.</p>
    #[serde(rename = "RecommendedNormalizedUnitsToPurchase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_normalized_units_to_purchase: Option<String>,
    /// <p>The number of instances that AWS recommends that you purchase.</p>
    #[serde(rename = "RecommendedNumberOfInstancesToPurchase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_number_of_instances_to_purchase: Option<String>,
    /// <p>How much purchasing this instance costs you on a monthly basis.</p>
    #[serde(rename = "RecurringStandardMonthlyCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_standard_monthly_cost: Option<String>,
    /// <p>How much purchasing this instance costs you upfront.</p>
    #[serde(rename = "UpfrontCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upfront_cost: Option<String>,
}

/// <p>Information about this specific recommendation, such as the time stamp for when AWS made a specific recommendation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ReservationPurchaseRecommendationMetadata {
    /// <p>The time stamp for when AWS made this recommendation.</p>
    #[serde(rename = "GenerationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_timestamp: Option<String>,
    /// <p>The ID for this specific recommendation.</p>
    #[serde(rename = "RecommendationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_id: Option<String>,
}

/// <p>A summary about this recommendation, such as the currency code, the amount that AWS estimates that you could save, and the total amount of reservation to purchase.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ReservationPurchaseRecommendationSummary {
    /// <p>The currency code used for this recommendation.</p>
    #[serde(rename = "CurrencyCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    /// <p>The total amount that AWS estimates that this recommendation could save you in a month.</p>
    #[serde(rename = "TotalEstimatedMonthlySavingsAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_estimated_monthly_savings_amount: Option<String>,
    /// <p>The total amount that AWS estimates that this recommendation could save you in a month, as a percentage of your costs.</p>
    #[serde(rename = "TotalEstimatedMonthlySavingsPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_estimated_monthly_savings_percentage: Option<String>,
}

/// <p>A group of Reserved Instances (RIs) that share a set of attributes.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ReservationUtilizationGroup {
    /// <p>The attributes for this group of RIs.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>The key for a specific RI attribute.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>How much you used this group of RIs.</p>
    #[serde(rename = "Utilization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utilization: Option<ReservationAggregates>,
    /// <p>The value of a specific RI attribute.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>The result that is associated with a time period.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ResultByTime {
    /// <p>Whether this result is estimated.</p>
    #[serde(rename = "Estimated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated: Option<bool>,
    /// <p>The groups that are included in this time period.</p>
    #[serde(rename = "Groups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<Group>>,
    /// <p>The time period covered by a result.</p>
    #[serde(rename = "TimePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_period: Option<DateInterval>,
    /// <p>The total amount of cost or usage accrued during the time period.</p>
    #[serde(rename = "Total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<::std::collections::HashMap<String, MetricValue>>,
}

/// <p>Hardware specifications for the service that you want recommendations for.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceSpecification {
    /// <p>The EC2 hardware specifications that you want AWS to provide recommendations for.</p>
    #[serde(rename = "EC2Specification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_specification: Option<EC2Specification>,
}

/// <p>The values that are available for a tag.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagValues {
    /// <p>The key for a tag.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The specific value of a tag.</p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// <p>The amount of utilization, in hours.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UtilizationByTime {
    /// <p>The groups that are included in this utilization result.</p>
    #[serde(rename = "Groups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<ReservationUtilizationGroup>>,
    /// <p>The period of time over which this utilization was used.</p>
    #[serde(rename = "TimePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_period: Option<DateInterval>,
    /// <p>The total number of RI hours that were used.</p>
    #[serde(rename = "Total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<ReservationAggregates>,
}

/// Errors returned by GetCostAndUsage
#[derive(Debug, PartialEq)]
pub enum GetCostAndUsageError {
    /// <p>The requested report expired. Update the date interval and try again.</p>
    BillExpiration(String),
    /// <p>The requested data is unavailable.</p>
    DataUnavailable(String),
    /// <p>The pagination token is invalid. Try again without a pagination token.</p>
    InvalidNextToken(String),
    /// <p>You made too many calls in a short period of time. Try again later.</p>
    LimitExceeded(String),
    /// <p>Your request parameters changed between pages. Try again with the old parameters or without a pagination token.</p>
    RequestChanged(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetCostAndUsageError {
    pub fn from_body(body: &str) -> GetCostAndUsageError {
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
                    "BillExpirationException" => {
                        GetCostAndUsageError::BillExpiration(String::from(error_message))
                    }
                    "DataUnavailableException" => {
                        GetCostAndUsageError::DataUnavailable(String::from(error_message))
                    }
                    "InvalidNextTokenException" => {
                        GetCostAndUsageError::InvalidNextToken(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        GetCostAndUsageError::LimitExceeded(String::from(error_message))
                    }
                    "RequestChangedException" => {
                        GetCostAndUsageError::RequestChanged(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetCostAndUsageError::Validation(error_message.to_string())
                    }
                    _ => GetCostAndUsageError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetCostAndUsageError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetCostAndUsageError {
    fn from(err: serde_json::error::Error) -> GetCostAndUsageError {
        GetCostAndUsageError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetCostAndUsageError {
    fn from(err: CredentialsError) -> GetCostAndUsageError {
        GetCostAndUsageError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetCostAndUsageError {
    fn from(err: HttpDispatchError) -> GetCostAndUsageError {
        GetCostAndUsageError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetCostAndUsageError {
    fn from(err: io::Error) -> GetCostAndUsageError {
        GetCostAndUsageError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetCostAndUsageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCostAndUsageError {
    fn description(&self) -> &str {
        match *self {
            GetCostAndUsageError::BillExpiration(ref cause) => cause,
            GetCostAndUsageError::DataUnavailable(ref cause) => cause,
            GetCostAndUsageError::InvalidNextToken(ref cause) => cause,
            GetCostAndUsageError::LimitExceeded(ref cause) => cause,
            GetCostAndUsageError::RequestChanged(ref cause) => cause,
            GetCostAndUsageError::Validation(ref cause) => cause,
            GetCostAndUsageError::Credentials(ref err) => err.description(),
            GetCostAndUsageError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetCostAndUsageError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDimensionValues
#[derive(Debug, PartialEq)]
pub enum GetDimensionValuesError {
    /// <p>The requested report expired. Update the date interval and try again.</p>
    BillExpiration(String),
    /// <p>The requested data is unavailable.</p>
    DataUnavailable(String),
    /// <p>The pagination token is invalid. Try again without a pagination token.</p>
    InvalidNextToken(String),
    /// <p>You made too many calls in a short period of time. Try again later.</p>
    LimitExceeded(String),
    /// <p>Your request parameters changed between pages. Try again with the old parameters or without a pagination token.</p>
    RequestChanged(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetDimensionValuesError {
    pub fn from_body(body: &str) -> GetDimensionValuesError {
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
                    "BillExpirationException" => {
                        GetDimensionValuesError::BillExpiration(String::from(error_message))
                    }
                    "DataUnavailableException" => {
                        GetDimensionValuesError::DataUnavailable(String::from(error_message))
                    }
                    "InvalidNextTokenException" => {
                        GetDimensionValuesError::InvalidNextToken(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        GetDimensionValuesError::LimitExceeded(String::from(error_message))
                    }
                    "RequestChangedException" => {
                        GetDimensionValuesError::RequestChanged(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetDimensionValuesError::Validation(error_message.to_string())
                    }
                    _ => GetDimensionValuesError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetDimensionValuesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetDimensionValuesError {
    fn from(err: serde_json::error::Error) -> GetDimensionValuesError {
        GetDimensionValuesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDimensionValuesError {
    fn from(err: CredentialsError) -> GetDimensionValuesError {
        GetDimensionValuesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDimensionValuesError {
    fn from(err: HttpDispatchError) -> GetDimensionValuesError {
        GetDimensionValuesError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDimensionValuesError {
    fn from(err: io::Error) -> GetDimensionValuesError {
        GetDimensionValuesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDimensionValuesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDimensionValuesError {
    fn description(&self) -> &str {
        match *self {
            GetDimensionValuesError::BillExpiration(ref cause) => cause,
            GetDimensionValuesError::DataUnavailable(ref cause) => cause,
            GetDimensionValuesError::InvalidNextToken(ref cause) => cause,
            GetDimensionValuesError::LimitExceeded(ref cause) => cause,
            GetDimensionValuesError::RequestChanged(ref cause) => cause,
            GetDimensionValuesError::Validation(ref cause) => cause,
            GetDimensionValuesError::Credentials(ref err) => err.description(),
            GetDimensionValuesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetDimensionValuesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetReservationCoverage
#[derive(Debug, PartialEq)]
pub enum GetReservationCoverageError {
    /// <p>The requested data is unavailable.</p>
    DataUnavailable(String),
    /// <p>The pagination token is invalid. Try again without a pagination token.</p>
    InvalidNextToken(String),
    /// <p>You made too many calls in a short period of time. Try again later.</p>
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

impl GetReservationCoverageError {
    pub fn from_body(body: &str) -> GetReservationCoverageError {
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
                    "DataUnavailableException" => {
                        GetReservationCoverageError::DataUnavailable(String::from(error_message))
                    }
                    "InvalidNextTokenException" => {
                        GetReservationCoverageError::InvalidNextToken(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        GetReservationCoverageError::LimitExceeded(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetReservationCoverageError::Validation(error_message.to_string())
                    }
                    _ => GetReservationCoverageError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetReservationCoverageError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetReservationCoverageError {
    fn from(err: serde_json::error::Error) -> GetReservationCoverageError {
        GetReservationCoverageError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetReservationCoverageError {
    fn from(err: CredentialsError) -> GetReservationCoverageError {
        GetReservationCoverageError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetReservationCoverageError {
    fn from(err: HttpDispatchError) -> GetReservationCoverageError {
        GetReservationCoverageError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetReservationCoverageError {
    fn from(err: io::Error) -> GetReservationCoverageError {
        GetReservationCoverageError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetReservationCoverageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetReservationCoverageError {
    fn description(&self) -> &str {
        match *self {
            GetReservationCoverageError::DataUnavailable(ref cause) => cause,
            GetReservationCoverageError::InvalidNextToken(ref cause) => cause,
            GetReservationCoverageError::LimitExceeded(ref cause) => cause,
            GetReservationCoverageError::Validation(ref cause) => cause,
            GetReservationCoverageError::Credentials(ref err) => err.description(),
            GetReservationCoverageError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetReservationCoverageError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetReservationPurchaseRecommendation
#[derive(Debug, PartialEq)]
pub enum GetReservationPurchaseRecommendationError {
    /// <p>The requested data is unavailable.</p>
    DataUnavailable(String),
    /// <p>The pagination token is invalid. Try again without a pagination token.</p>
    InvalidNextToken(String),
    /// <p>You made too many calls in a short period of time. Try again later.</p>
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

impl GetReservationPurchaseRecommendationError {
    pub fn from_body(body: &str) -> GetReservationPurchaseRecommendationError {
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
                    "DataUnavailableException" => {
                        GetReservationPurchaseRecommendationError::DataUnavailable(String::from(
                            error_message,
                        ))
                    }
                    "InvalidNextTokenException" => {
                        GetReservationPurchaseRecommendationError::InvalidNextToken(String::from(
                            error_message,
                        ))
                    }
                    "LimitExceededException" => {
                        GetReservationPurchaseRecommendationError::LimitExceeded(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => GetReservationPurchaseRecommendationError::Validation(
                        error_message.to_string(),
                    ),
                    _ => GetReservationPurchaseRecommendationError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetReservationPurchaseRecommendationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetReservationPurchaseRecommendationError {
    fn from(err: serde_json::error::Error) -> GetReservationPurchaseRecommendationError {
        GetReservationPurchaseRecommendationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetReservationPurchaseRecommendationError {
    fn from(err: CredentialsError) -> GetReservationPurchaseRecommendationError {
        GetReservationPurchaseRecommendationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetReservationPurchaseRecommendationError {
    fn from(err: HttpDispatchError) -> GetReservationPurchaseRecommendationError {
        GetReservationPurchaseRecommendationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetReservationPurchaseRecommendationError {
    fn from(err: io::Error) -> GetReservationPurchaseRecommendationError {
        GetReservationPurchaseRecommendationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetReservationPurchaseRecommendationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetReservationPurchaseRecommendationError {
    fn description(&self) -> &str {
        match *self {
            GetReservationPurchaseRecommendationError::DataUnavailable(ref cause) => cause,
            GetReservationPurchaseRecommendationError::InvalidNextToken(ref cause) => cause,
            GetReservationPurchaseRecommendationError::LimitExceeded(ref cause) => cause,
            GetReservationPurchaseRecommendationError::Validation(ref cause) => cause,
            GetReservationPurchaseRecommendationError::Credentials(ref err) => err.description(),
            GetReservationPurchaseRecommendationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetReservationPurchaseRecommendationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetReservationUtilization
#[derive(Debug, PartialEq)]
pub enum GetReservationUtilizationError {
    /// <p>The requested data is unavailable.</p>
    DataUnavailable(String),
    /// <p>The pagination token is invalid. Try again without a pagination token.</p>
    InvalidNextToken(String),
    /// <p>You made too many calls in a short period of time. Try again later.</p>
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

impl GetReservationUtilizationError {
    pub fn from_body(body: &str) -> GetReservationUtilizationError {
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
                    "DataUnavailableException" => {
                        GetReservationUtilizationError::DataUnavailable(String::from(error_message))
                    }
                    "InvalidNextTokenException" => {
                        GetReservationUtilizationError::InvalidNextToken(String::from(
                            error_message,
                        ))
                    }
                    "LimitExceededException" => {
                        GetReservationUtilizationError::LimitExceeded(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetReservationUtilizationError::Validation(error_message.to_string())
                    }
                    _ => GetReservationUtilizationError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetReservationUtilizationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetReservationUtilizationError {
    fn from(err: serde_json::error::Error) -> GetReservationUtilizationError {
        GetReservationUtilizationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetReservationUtilizationError {
    fn from(err: CredentialsError) -> GetReservationUtilizationError {
        GetReservationUtilizationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetReservationUtilizationError {
    fn from(err: HttpDispatchError) -> GetReservationUtilizationError {
        GetReservationUtilizationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetReservationUtilizationError {
    fn from(err: io::Error) -> GetReservationUtilizationError {
        GetReservationUtilizationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetReservationUtilizationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetReservationUtilizationError {
    fn description(&self) -> &str {
        match *self {
            GetReservationUtilizationError::DataUnavailable(ref cause) => cause,
            GetReservationUtilizationError::InvalidNextToken(ref cause) => cause,
            GetReservationUtilizationError::LimitExceeded(ref cause) => cause,
            GetReservationUtilizationError::Validation(ref cause) => cause,
            GetReservationUtilizationError::Credentials(ref err) => err.description(),
            GetReservationUtilizationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetReservationUtilizationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetTags
#[derive(Debug, PartialEq)]
pub enum GetTagsError {
    /// <p>The requested report expired. Update the date interval and try again.</p>
    BillExpiration(String),
    /// <p>The requested data is unavailable.</p>
    DataUnavailable(String),
    /// <p>The pagination token is invalid. Try again without a pagination token.</p>
    InvalidNextToken(String),
    /// <p>You made too many calls in a short period of time. Try again later.</p>
    LimitExceeded(String),
    /// <p>Your request parameters changed between pages. Try again with the old parameters or without a pagination token.</p>
    RequestChanged(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetTagsError {
    pub fn from_body(body: &str) -> GetTagsError {
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
                    "BillExpirationException" => {
                        GetTagsError::BillExpiration(String::from(error_message))
                    }
                    "DataUnavailableException" => {
                        GetTagsError::DataUnavailable(String::from(error_message))
                    }
                    "InvalidNextTokenException" => {
                        GetTagsError::InvalidNextToken(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        GetTagsError::LimitExceeded(String::from(error_message))
                    }
                    "RequestChangedException" => {
                        GetTagsError::RequestChanged(String::from(error_message))
                    }
                    "ValidationException" => GetTagsError::Validation(error_message.to_string()),
                    _ => GetTagsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetTagsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetTagsError {
    fn from(err: serde_json::error::Error) -> GetTagsError {
        GetTagsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetTagsError {
    fn from(err: CredentialsError) -> GetTagsError {
        GetTagsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetTagsError {
    fn from(err: HttpDispatchError) -> GetTagsError {
        GetTagsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetTagsError {
    fn from(err: io::Error) -> GetTagsError {
        GetTagsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetTagsError {
    fn description(&self) -> &str {
        match *self {
            GetTagsError::BillExpiration(ref cause) => cause,
            GetTagsError::DataUnavailable(ref cause) => cause,
            GetTagsError::InvalidNextToken(ref cause) => cause,
            GetTagsError::LimitExceeded(ref cause) => cause,
            GetTagsError::RequestChanged(ref cause) => cause,
            GetTagsError::Validation(ref cause) => cause,
            GetTagsError::Credentials(ref err) => err.description(),
            GetTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetTagsError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS Cost Explorer API. AWS Cost Explorer clients implement this trait.
pub trait CostExplorer {
    /// <p>Retrieves cost and usage metrics for your account. You can specify which cost and usage-related metric, such as <code>BlendedCosts</code> or <code>UsageQuantity</code>, that you want the request to return. You can also filter and group your data by various dimensions, such as <code>SERVICE</code> or <code>AZ</code>, in a specific time range. For a complete list of valid dimensions, see the <code> <a href="http://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_GetDimensionValues.html">GetDimensionValues</a> </code> operation. Master accounts in an organization in AWS Organizations have access to all member accounts.</p>
    fn get_cost_and_usage(
        &self,
        input: GetCostAndUsageRequest,
    ) -> RusotoFuture<GetCostAndUsageResponse, GetCostAndUsageError>;

    /// <p>Retrieves all available filter values for a specified filter over a period of time. You can search the dimension values for an arbitrary string. </p>
    fn get_dimension_values(
        &self,
        input: GetDimensionValuesRequest,
    ) -> RusotoFuture<GetDimensionValuesResponse, GetDimensionValuesError>;

    /// <p>Retrieves the reservation coverage for your account. This allows you to see how much of your Amazon Elastic Compute Cloud, Amazon ElastiCache, Amazon Relational Database Service, or Amazon Redshift usage is covered by a reservation. An organization's master account can see the coverage of the associated member accounts. For any time period, you can filter data about reservation usage by the following dimensions:</p> <ul> <li> <p>AZ</p> </li> <li> <p>CACHE_ENGINE</p> </li> <li> <p>DATABASE_ENGINE</p> </li> <li> <p>DEPLOYMENT_OPTION</p> </li> <li> <p>INSTANCE_TYPE</p> </li> <li> <p>LINKED_ACCOUNT</p> </li> <li> <p>OPERATING_SYSTEM</p> </li> <li> <p>PLATFORM</p> </li> <li> <p>REGION</p> </li> <li> <p>SERVICE</p> </li> <li> <p>TAG</p> </li> <li> <p>TENANCY</p> </li> </ul> <p>To determine valid values for a dimension, use the <code>GetDimensionValues</code> operation. </p>
    fn get_reservation_coverage(
        &self,
        input: GetReservationCoverageRequest,
    ) -> RusotoFuture<GetReservationCoverageResponse, GetReservationCoverageError>;

    /// <p>Gets recommendations for which reservations to purchase. These recommendations could help you reduce your costs. Reservations provide a discounted hourly rate (up to 75%) compared to On-Demand pricing.</p> <p>AWS generates your recommendations by identifying your On-Demand usage during a specific time period and collecting your usage into categories that are eligible for a reservation. After AWS has these categories, it simulates every combination of reservations in each category of usage to identify the best number of each type of RI to purchase to maximize your estimated savings. </p> <p>For example, AWS automatically aggregates your EC2 Linux, shared tenancy, and c4 family usage in the US West (Oregon) Region and recommends that you buy size-flexible regional reservations to apply to the c4 family usage. AWS recommends the smallest size instance in an instance family. This makes it easier to purchase a size-flexible RI. AWS also shows the equal number of normalized units so that you can purchase any instance size that you want. For this example, your RI recommendation would be for <code>c4.large</code>, because that is the smallest size instance in the c4 instance family.</p>
    fn get_reservation_purchase_recommendation(
        &self,
        input: GetReservationPurchaseRecommendationRequest,
    ) -> RusotoFuture<
        GetReservationPurchaseRecommendationResponse,
        GetReservationPurchaseRecommendationError,
    >;

    /// <p>Retrieves the reservation utilization for your account. Master accounts in an organization have access to member accounts. You can filter data by dimensions in a time period. You can use <code>GetDimensionValues</code> to determine the possible dimension values. Currently, you can group only by <code>SUBSCRIPTION_ID</code>. </p>
    fn get_reservation_utilization(
        &self,
        input: GetReservationUtilizationRequest,
    ) -> RusotoFuture<GetReservationUtilizationResponse, GetReservationUtilizationError>;

    /// <p>Queries for available tag keys and tag values for a specified period. You can search the tag values for an arbitrary string. </p>
    fn get_tags(&self, input: GetTagsRequest) -> RusotoFuture<GetTagsResponse, GetTagsError>;
}
/// A client for the AWS Cost Explorer API.
pub struct CostExplorerClient {
    client: Client,
    region: region::Region,
}

impl CostExplorerClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> CostExplorerClient {
        CostExplorerClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> CostExplorerClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        CostExplorerClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl CostExplorer for CostExplorerClient {
    /// <p>Retrieves cost and usage metrics for your account. You can specify which cost and usage-related metric, such as <code>BlendedCosts</code> or <code>UsageQuantity</code>, that you want the request to return. You can also filter and group your data by various dimensions, such as <code>SERVICE</code> or <code>AZ</code>, in a specific time range. For a complete list of valid dimensions, see the <code> <a href="http://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_GetDimensionValues.html">GetDimensionValues</a> </code> operation. Master accounts in an organization in AWS Organizations have access to all member accounts.</p>
    fn get_cost_and_usage(
        &self,
        input: GetCostAndUsageRequest,
    ) -> RusotoFuture<GetCostAndUsageResponse, GetCostAndUsageError> {
        let mut request = SignedRequest::new("POST", "ce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSInsightsIndexService.GetCostAndUsage");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetCostAndUsageResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetCostAndUsageError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Retrieves all available filter values for a specified filter over a period of time. You can search the dimension values for an arbitrary string. </p>
    fn get_dimension_values(
        &self,
        input: GetDimensionValuesRequest,
    ) -> RusotoFuture<GetDimensionValuesResponse, GetDimensionValuesError> {
        let mut request = SignedRequest::new("POST", "ce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSInsightsIndexService.GetDimensionValues");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetDimensionValuesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetDimensionValuesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Retrieves the reservation coverage for your account. This allows you to see how much of your Amazon Elastic Compute Cloud, Amazon ElastiCache, Amazon Relational Database Service, or Amazon Redshift usage is covered by a reservation. An organization's master account can see the coverage of the associated member accounts. For any time period, you can filter data about reservation usage by the following dimensions:</p> <ul> <li> <p>AZ</p> </li> <li> <p>CACHE_ENGINE</p> </li> <li> <p>DATABASE_ENGINE</p> </li> <li> <p>DEPLOYMENT_OPTION</p> </li> <li> <p>INSTANCE_TYPE</p> </li> <li> <p>LINKED_ACCOUNT</p> </li> <li> <p>OPERATING_SYSTEM</p> </li> <li> <p>PLATFORM</p> </li> <li> <p>REGION</p> </li> <li> <p>SERVICE</p> </li> <li> <p>TAG</p> </li> <li> <p>TENANCY</p> </li> </ul> <p>To determine valid values for a dimension, use the <code>GetDimensionValues</code> operation. </p>
    fn get_reservation_coverage(
        &self,
        input: GetReservationCoverageRequest,
    ) -> RusotoFuture<GetReservationCoverageResponse, GetReservationCoverageError> {
        let mut request = SignedRequest::new("POST", "ce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSInsightsIndexService.GetReservationCoverage",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetReservationCoverageResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetReservationCoverageError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Gets recommendations for which reservations to purchase. These recommendations could help you reduce your costs. Reservations provide a discounted hourly rate (up to 75%) compared to On-Demand pricing.</p> <p>AWS generates your recommendations by identifying your On-Demand usage during a specific time period and collecting your usage into categories that are eligible for a reservation. After AWS has these categories, it simulates every combination of reservations in each category of usage to identify the best number of each type of RI to purchase to maximize your estimated savings. </p> <p>For example, AWS automatically aggregates your EC2 Linux, shared tenancy, and c4 family usage in the US West (Oregon) Region and recommends that you buy size-flexible regional reservations to apply to the c4 family usage. AWS recommends the smallest size instance in an instance family. This makes it easier to purchase a size-flexible RI. AWS also shows the equal number of normalized units so that you can purchase any instance size that you want. For this example, your RI recommendation would be for <code>c4.large</code>, because that is the smallest size instance in the c4 instance family.</p>
    fn get_reservation_purchase_recommendation(
        &self,
        input: GetReservationPurchaseRecommendationRequest,
    ) -> RusotoFuture<
        GetReservationPurchaseRecommendationResponse,
        GetReservationPurchaseRecommendationError,
    > {
        let mut request = SignedRequest::new("POST", "ce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSInsightsIndexService.GetReservationPurchaseRecommendation",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetReservationPurchaseRecommendationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetReservationPurchaseRecommendationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Retrieves the reservation utilization for your account. Master accounts in an organization have access to member accounts. You can filter data by dimensions in a time period. You can use <code>GetDimensionValues</code> to determine the possible dimension values. Currently, you can group only by <code>SUBSCRIPTION_ID</code>. </p>
    fn get_reservation_utilization(
        &self,
        input: GetReservationUtilizationRequest,
    ) -> RusotoFuture<GetReservationUtilizationResponse, GetReservationUtilizationError> {
        let mut request = SignedRequest::new("POST", "ce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSInsightsIndexService.GetReservationUtilization",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetReservationUtilizationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetReservationUtilizationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Queries for available tag keys and tag values for a specified period. You can search the tag values for an arbitrary string. </p>
    fn get_tags(&self, input: GetTagsRequest) -> RusotoFuture<GetTagsResponse, GetTagsError> {
        let mut request = SignedRequest::new("POST", "ce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSInsightsIndexService.GetTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetTagsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetTagsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
