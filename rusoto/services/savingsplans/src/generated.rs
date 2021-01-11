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
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateSavingsPlanRequest {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The hourly commitment, in USD. This is a value between 0.001 and 1 million. You cannot specify more than three digits after the decimal point.</p>
    #[serde(rename = "commitment")]
    pub commitment: String,
    /// <p>The time at which to purchase the Savings Plan, in UTC format (YYYY-MM-DDTHH:MM:SSZ).</p>
    #[serde(rename = "purchaseTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_time: Option<f64>,
    /// <p>The ID of the offering.</p>
    #[serde(rename = "savingsPlanOfferingId")]
    pub savings_plan_offering_id: String,
    /// <p>One or more tags.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The up-front payment amount. This is a whole number between 50 and 99 percent of the total value of the Savings Plan. This parameter is supported only if the payment option is <code>Partial Upfront</code>.</p>
    #[serde(rename = "upfrontPaymentAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upfront_payment_amount: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateSavingsPlanResponse {
    /// <p>The ID of the Savings Plan.</p>
    #[serde(rename = "savingsPlanId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plan_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteQueuedSavingsPlanRequest {
    /// <p>The ID of the Savings Plan.</p>
    #[serde(rename = "savingsPlanId")]
    pub savings_plan_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteQueuedSavingsPlanResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeSavingsPlanRatesRequest {
    /// <p>The filters.</p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<SavingsPlanRateFilter>>,
    /// <p>The maximum number of results to return with a single call. To retrieve additional results, make another call with the returned token value.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next page of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the Savings Plan.</p>
    #[serde(rename = "savingsPlanId")]
    pub savings_plan_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeSavingsPlanRatesResponse {
    /// <p>The token to use to retrieve the next page of results. This value is null when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the Savings Plan.</p>
    #[serde(rename = "savingsPlanId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plan_id: Option<String>,
    /// <p>Information about the Savings Plans rates.</p>
    #[serde(rename = "searchResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_results: Option<Vec<SavingsPlanRate>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeSavingsPlansOfferingRatesRequest {
    /// <p>The filters.</p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<SavingsPlanOfferingRateFilterElement>>,
    /// <p>The maximum number of results to return with a single call. To retrieve additional results, make another call with the returned token value.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next page of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The specific AWS operation for the line item in the billing report.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<String>>,
    /// <p>The AWS products.</p>
    #[serde(rename = "products")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<String>>,
    /// <p>The IDs of the offerings.</p>
    #[serde(rename = "savingsPlanOfferingIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plan_offering_ids: Option<Vec<String>>,
    /// <p>The payment options.</p>
    #[serde(rename = "savingsPlanPaymentOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plan_payment_options: Option<Vec<String>>,
    /// <p>The plan types.</p>
    #[serde(rename = "savingsPlanTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plan_types: Option<Vec<String>>,
    /// <p>The services.</p>
    #[serde(rename = "serviceCodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_codes: Option<Vec<String>>,
    /// <p>The usage details of the line item in the billing report.</p>
    #[serde(rename = "usageTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_types: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeSavingsPlansOfferingRatesResponse {
    /// <p>The token to use to retrieve the next page of results. This value is null when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the Savings Plans offering rates.</p>
    #[serde(rename = "searchResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_results: Option<Vec<SavingsPlanOfferingRate>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeSavingsPlansOfferingsRequest {
    /// <p>The currencies.</p>
    #[serde(rename = "currencies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currencies: Option<Vec<String>>,
    /// <p>The descriptions.</p>
    #[serde(rename = "descriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<Vec<String>>,
    /// <p>The durations, in seconds.</p>
    #[serde(rename = "durations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durations: Option<Vec<i64>>,
    /// <p>The filters.</p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<SavingsPlanOfferingFilterElement>>,
    /// <p>The maximum number of results to return with a single call. To retrieve additional results, make another call with the returned token value.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next page of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The IDs of the offerings.</p>
    #[serde(rename = "offeringIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_ids: Option<Vec<String>>,
    /// <p>The specific AWS operation for the line item in the billing report.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<String>>,
    /// <p>The payment options.</p>
    #[serde(rename = "paymentOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_options: Option<Vec<String>>,
    /// <p>The plan type.</p>
    #[serde(rename = "planTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_types: Option<Vec<String>>,
    /// <p>The product type.</p>
    #[serde(rename = "productType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<String>,
    /// <p>The services.</p>
    #[serde(rename = "serviceCodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_codes: Option<Vec<String>>,
    /// <p>The usage details of the line item in the billing report.</p>
    #[serde(rename = "usageTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_types: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeSavingsPlansOfferingsResponse {
    /// <p>The token to use to retrieve the next page of results. This value is null when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the Savings Plans offerings.</p>
    #[serde(rename = "searchResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_results: Option<Vec<SavingsPlanOffering>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeSavingsPlansRequest {
    /// <p>The filters.</p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<SavingsPlanFilter>>,
    /// <p>The maximum number of results to return with a single call. To retrieve additional results, make another call with the returned token value.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next page of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The Amazon Resource Names (ARN) of the Savings Plans.</p>
    #[serde(rename = "savingsPlanArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plan_arns: Option<Vec<String>>,
    /// <p>The IDs of the Savings Plans.</p>
    #[serde(rename = "savingsPlanIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plan_ids: Option<Vec<String>>,
    /// <p>The states.</p>
    #[serde(rename = "states")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeSavingsPlansResponse {
    /// <p>The token to use to retrieve the next page of results. This value is null when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the Savings Plans.</p>
    #[serde(rename = "savingsPlans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plans: Option<Vec<SavingsPlan>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>Information about the tags.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Information about a Savings Plan offering.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ParentSavingsPlanOffering {
    /// <p>The currency.</p>
    #[serde(rename = "currency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// <p>The duration, in seconds.</p>
    #[serde(rename = "durationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i64>,
    /// <p>The ID of the offering.</p>
    #[serde(rename = "offeringId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_id: Option<String>,
    /// <p>The payment option.</p>
    #[serde(rename = "paymentOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_option: Option<String>,
    /// <p>The description.</p>
    #[serde(rename = "planDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_description: Option<String>,
    /// <p>The plan type.</p>
    #[serde(rename = "planType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<String>,
}

/// <p>Information about a Savings Plan.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SavingsPlan {
    /// <p>The hourly commitment, in USD.</p>
    #[serde(rename = "commitment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<String>,
    /// <p>The currency.</p>
    #[serde(rename = "currency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// <p>The description.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The EC2 instance family.</p>
    #[serde(rename = "ec2InstanceFamily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_instance_family: Option<String>,
    /// <p>The end time.</p>
    #[serde(rename = "end")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// <p>The ID of the offering.</p>
    #[serde(rename = "offeringId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_id: Option<String>,
    /// <p>The payment option.</p>
    #[serde(rename = "paymentOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_option: Option<String>,
    /// <p>The product types.</p>
    #[serde(rename = "productTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_types: Option<Vec<String>>,
    /// <p>The recurring payment amount.</p>
    #[serde(rename = "recurringPaymentAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_payment_amount: Option<String>,
    /// <p>The AWS Region.</p>
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the Savings Plan.</p>
    #[serde(rename = "savingsPlanArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plan_arn: Option<String>,
    /// <p>The ID of the Savings Plan.</p>
    #[serde(rename = "savingsPlanId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plan_id: Option<String>,
    /// <p>The plan type.</p>
    #[serde(rename = "savingsPlanType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plan_type: Option<String>,
    /// <p>The start time.</p>
    #[serde(rename = "start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    /// <p>The state.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>One or more tags.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The duration of the term, in seconds.</p>
    #[serde(rename = "termDurationInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term_duration_in_seconds: Option<i64>,
    /// <p>The up-front payment amount.</p>
    #[serde(rename = "upfrontPaymentAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upfront_payment_amount: Option<String>,
}

/// <p>Information about a filter.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SavingsPlanFilter {
    /// <p>The filter name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The filter value.</p>
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// <p>Information about a Savings Plan offering.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SavingsPlanOffering {
    /// <p>The currency.</p>
    #[serde(rename = "currency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// <p>The description.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The duration, in seconds.</p>
    #[serde(rename = "durationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i64>,
    /// <p>The ID of the offering.</p>
    #[serde(rename = "offeringId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_id: Option<String>,
    /// <p>The specific AWS operation for the line item in the billing report.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    /// <p>The payment option.</p>
    #[serde(rename = "paymentOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_option: Option<String>,
    /// <p>The plan type.</p>
    #[serde(rename = "planType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<String>,
    /// <p>The product type.</p>
    #[serde(rename = "productTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_types: Option<Vec<String>>,
    /// <p>The properties.</p>
    #[serde(rename = "properties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<SavingsPlanOfferingProperty>>,
    /// <p>The service.</p>
    #[serde(rename = "serviceCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code: Option<String>,
    /// <p>The usage details of the line item in the billing report.</p>
    #[serde(rename = "usageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_type: Option<String>,
}

/// <p>Information about a filter.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SavingsPlanOfferingFilterElement {
    /// <p>The filter name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The filter values.</p>
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// <p>Information about a property.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SavingsPlanOfferingProperty {
    /// <p>The property name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The property value.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Information about a Savings Plan offering rate.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SavingsPlanOfferingRate {
    /// <p>The specific AWS operation for the line item in the billing report.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    /// <p>The product type.</p>
    #[serde(rename = "productType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<String>,
    /// <p>The properties.</p>
    #[serde(rename = "properties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<SavingsPlanOfferingRateProperty>>,
    /// <p>The Savings Plan rate.</p>
    #[serde(rename = "rate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<String>,
    /// <p>The Savings Plan offering.</p>
    #[serde(rename = "savingsPlanOffering")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plan_offering: Option<ParentSavingsPlanOffering>,
    /// <p>The service.</p>
    #[serde(rename = "serviceCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code: Option<String>,
    /// <p>The unit.</p>
    #[serde(rename = "unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// <p>The usage details of the line item in the billing report.</p>
    #[serde(rename = "usageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_type: Option<String>,
}

/// <p>Information about a filter.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SavingsPlanOfferingRateFilterElement {
    /// <p>The filter name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The filter values.</p>
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// <p>Information about a property.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SavingsPlanOfferingRateProperty {
    /// <p>The property name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The property value.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Information about a Savings Plan rate.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SavingsPlanRate {
    /// <p>The currency.</p>
    #[serde(rename = "currency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// <p>The specific AWS operation for the line item in the billing report.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    /// <p>The product type.</p>
    #[serde(rename = "productType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<String>,
    /// <p>The properties.</p>
    #[serde(rename = "properties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<SavingsPlanRateProperty>>,
    /// <p>The rate.</p>
    #[serde(rename = "rate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<String>,
    /// <p>The service.</p>
    #[serde(rename = "serviceCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code: Option<String>,
    /// <p>The unit.</p>
    #[serde(rename = "unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// <p>The usage details of the line item in the billing report.</p>
    #[serde(rename = "usageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_type: Option<String>,
}

/// <p>Information about a filter.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SavingsPlanRateFilter {
    /// <p>The filter name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The filter values.</p>
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// <p>Information about a property.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SavingsPlanRateProperty {
    /// <p>The property name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The property value.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>One or more tags. For example, { "tags": {"key1":"value1", "key2":"value2"} }.</p>
    #[serde(rename = "tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The tag keys.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

/// Errors returned by CreateSavingsPlan
#[derive(Debug, PartialEq)]
pub enum CreateSavingsPlanError {
    /// <p>An unexpected error occurred.</p>
    InternalServer(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>A service quota has been exceeded.</p>
    ServiceQuotaExceeded(String),
}

impl CreateSavingsPlanError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateSavingsPlanError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(CreateSavingsPlanError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateSavingsPlanError::ResourceNotFound(err.msg))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(CreateSavingsPlanError::ServiceQuotaExceeded(
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
impl fmt::Display for CreateSavingsPlanError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateSavingsPlanError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateSavingsPlanError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateSavingsPlanError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateSavingsPlanError {}
/// Errors returned by DeleteQueuedSavingsPlan
#[derive(Debug, PartialEq)]
pub enum DeleteQueuedSavingsPlanError {
    /// <p>An unexpected error occurred.</p>
    InternalServer(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>A service quota has been exceeded.</p>
    ServiceQuotaExceeded(String),
}

impl DeleteQueuedSavingsPlanError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteQueuedSavingsPlanError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DeleteQueuedSavingsPlanError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteQueuedSavingsPlanError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(
                        DeleteQueuedSavingsPlanError::ServiceQuotaExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteQueuedSavingsPlanError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteQueuedSavingsPlanError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteQueuedSavingsPlanError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteQueuedSavingsPlanError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteQueuedSavingsPlanError {}
/// Errors returned by DescribeSavingsPlanRates
#[derive(Debug, PartialEq)]
pub enum DescribeSavingsPlanRatesError {
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeSavingsPlanRatesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeSavingsPlanRatesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeSavingsPlanRatesError::ResourceNotFound(
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
impl fmt::Display for DescribeSavingsPlanRatesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeSavingsPlanRatesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeSavingsPlanRatesError {}
/// Errors returned by DescribeSavingsPlans
#[derive(Debug, PartialEq)]
pub enum DescribeSavingsPlansError {
    /// <p>An unexpected error occurred.</p>
    InternalServer(String),
}

impl DescribeSavingsPlansError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeSavingsPlansError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeSavingsPlansError::InternalServer(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeSavingsPlansError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeSavingsPlansError::InternalServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeSavingsPlansError {}
/// Errors returned by DescribeSavingsPlansOfferingRates
#[derive(Debug, PartialEq)]
pub enum DescribeSavingsPlansOfferingRatesError {
    /// <p>An unexpected error occurred.</p>
    InternalServer(String),
}

impl DescribeSavingsPlansOfferingRatesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeSavingsPlansOfferingRatesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(
                        DescribeSavingsPlansOfferingRatesError::InternalServer(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeSavingsPlansOfferingRatesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeSavingsPlansOfferingRatesError::InternalServer(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeSavingsPlansOfferingRatesError {}
/// Errors returned by DescribeSavingsPlansOfferings
#[derive(Debug, PartialEq)]
pub enum DescribeSavingsPlansOfferingsError {
    /// <p>An unexpected error occurred.</p>
    InternalServer(String),
}

impl DescribeSavingsPlansOfferingsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeSavingsPlansOfferingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(
                        DescribeSavingsPlansOfferingsError::InternalServer(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeSavingsPlansOfferingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeSavingsPlansOfferingsError::InternalServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeSavingsPlansOfferingsError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>An unexpected error occurred.</p>
    InternalServer(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListTagsForResourceError::InternalServer(err.msg))
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
            ListTagsForResourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>An unexpected error occurred.</p>
    InternalServer(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>A service quota has been exceeded.</p>
    ServiceQuotaExceeded(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(TagResourceError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(TagResourceError::ServiceQuotaExceeded(err.msg))
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
            TagResourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>An unexpected error occurred.</p>
    InternalServer(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(UntagResourceError::InternalServer(err.msg))
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
            UntagResourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Trait representing the capabilities of the AWSSavingsPlans API. AWSSavingsPlans clients implement this trait.
#[async_trait]
pub trait SavingsPlans {
    /// <p>Creates a Savings Plan.</p>
    async fn create_savings_plan(
        &self,
        input: CreateSavingsPlanRequest,
    ) -> Result<CreateSavingsPlanResponse, RusotoError<CreateSavingsPlanError>>;

    /// <p>Deletes the queued purchase for the specified Savings Plan.</p>
    async fn delete_queued_savings_plan(
        &self,
        input: DeleteQueuedSavingsPlanRequest,
    ) -> Result<DeleteQueuedSavingsPlanResponse, RusotoError<DeleteQueuedSavingsPlanError>>;

    /// <p>Describes the specified Savings Plans rates.</p>
    async fn describe_savings_plan_rates(
        &self,
        input: DescribeSavingsPlanRatesRequest,
    ) -> Result<DescribeSavingsPlanRatesResponse, RusotoError<DescribeSavingsPlanRatesError>>;

    /// <p>Describes the specified Savings Plans.</p>
    async fn describe_savings_plans(
        &self,
        input: DescribeSavingsPlansRequest,
    ) -> Result<DescribeSavingsPlansResponse, RusotoError<DescribeSavingsPlansError>>;

    /// <p>Describes the specified Savings Plans offering rates.</p>
    async fn describe_savings_plans_offering_rates(
        &self,
        input: DescribeSavingsPlansOfferingRatesRequest,
    ) -> Result<
        DescribeSavingsPlansOfferingRatesResponse,
        RusotoError<DescribeSavingsPlansOfferingRatesError>,
    >;

    /// <p>Describes the specified Savings Plans offerings.</p>
    async fn describe_savings_plans_offerings(
        &self,
        input: DescribeSavingsPlansOfferingsRequest,
    ) -> Result<
        DescribeSavingsPlansOfferingsResponse,
        RusotoError<DescribeSavingsPlansOfferingsError>,
    >;

    /// <p>Lists the tags for the specified resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Adds the specified tags to the specified resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Removes the specified tags from the specified resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;
}
/// A client for the AWSSavingsPlans API.
#[derive(Clone)]
pub struct SavingsPlansClient {
    client: Client,
    region: region::Region,
}

impl SavingsPlansClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> SavingsPlansClient {
        SavingsPlansClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> SavingsPlansClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        SavingsPlansClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> SavingsPlansClient {
        SavingsPlansClient { client, region }
    }
}

#[async_trait]
impl SavingsPlans for SavingsPlansClient {
    /// <p>Creates a Savings Plan.</p>
    #[allow(unused_mut)]
    async fn create_savings_plan(
        &self,
        input: CreateSavingsPlanRequest,
    ) -> Result<CreateSavingsPlanResponse, RusotoError<CreateSavingsPlanError>> {
        let request_uri = "/CreateSavingsPlan";

        let mut request = SignedRequest::new("POST", "savingsplans", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateSavingsPlanResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateSavingsPlanError::from_response(response))
        }
    }

    /// <p>Deletes the queued purchase for the specified Savings Plan.</p>
    #[allow(unused_mut)]
    async fn delete_queued_savings_plan(
        &self,
        input: DeleteQueuedSavingsPlanRequest,
    ) -> Result<DeleteQueuedSavingsPlanResponse, RusotoError<DeleteQueuedSavingsPlanError>> {
        let request_uri = "/DeleteQueuedSavingsPlan";

        let mut request = SignedRequest::new("POST", "savingsplans", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteQueuedSavingsPlanResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteQueuedSavingsPlanError::from_response(response))
        }
    }

    /// <p>Describes the specified Savings Plans rates.</p>
    #[allow(unused_mut)]
    async fn describe_savings_plan_rates(
        &self,
        input: DescribeSavingsPlanRatesRequest,
    ) -> Result<DescribeSavingsPlanRatesResponse, RusotoError<DescribeSavingsPlanRatesError>> {
        let request_uri = "/DescribeSavingsPlanRates";

        let mut request = SignedRequest::new("POST", "savingsplans", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeSavingsPlanRatesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeSavingsPlanRatesError::from_response(response))
        }
    }

    /// <p>Describes the specified Savings Plans.</p>
    #[allow(unused_mut)]
    async fn describe_savings_plans(
        &self,
        input: DescribeSavingsPlansRequest,
    ) -> Result<DescribeSavingsPlansResponse, RusotoError<DescribeSavingsPlansError>> {
        let request_uri = "/DescribeSavingsPlans";

        let mut request = SignedRequest::new("POST", "savingsplans", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeSavingsPlansResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeSavingsPlansError::from_response(response))
        }
    }

    /// <p>Describes the specified Savings Plans offering rates.</p>
    #[allow(unused_mut)]
    async fn describe_savings_plans_offering_rates(
        &self,
        input: DescribeSavingsPlansOfferingRatesRequest,
    ) -> Result<
        DescribeSavingsPlansOfferingRatesResponse,
        RusotoError<DescribeSavingsPlansOfferingRatesError>,
    > {
        let request_uri = "/DescribeSavingsPlansOfferingRates";

        let mut request = SignedRequest::new("POST", "savingsplans", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeSavingsPlansOfferingRatesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeSavingsPlansOfferingRatesError::from_response(
                response,
            ))
        }
    }

    /// <p>Describes the specified Savings Plans offerings.</p>
    #[allow(unused_mut)]
    async fn describe_savings_plans_offerings(
        &self,
        input: DescribeSavingsPlansOfferingsRequest,
    ) -> Result<
        DescribeSavingsPlansOfferingsResponse,
        RusotoError<DescribeSavingsPlansOfferingsError>,
    > {
        let request_uri = "/DescribeSavingsPlansOfferings";

        let mut request = SignedRequest::new("POST", "savingsplans", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeSavingsPlansOfferingsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeSavingsPlansOfferingsError::from_response(response))
        }
    }

    /// <p>Lists the tags for the specified resource.</p>
    #[allow(unused_mut)]
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let request_uri = "/ListTagsForResource";

        let mut request = SignedRequest::new("POST", "savingsplans", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTagsForResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p>Adds the specified tags to the specified resource.</p>
    #[allow(unused_mut)]
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let request_uri = "/TagResource";

        let mut request = SignedRequest::new("POST", "savingsplans", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<TagResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Removes the specified tags from the specified resource.</p>
    #[allow(unused_mut)]
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let request_uri = "/UntagResource";

        let mut request = SignedRequest::new("POST", "savingsplans", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UntagResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }
}
