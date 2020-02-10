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
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateServiceQuotaTemplateRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateServiceQuotaTemplateResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteServiceQuotaIncreaseRequestFromTemplateRequest {
    /// <p>Specifies the AWS Region for the quota that you want to delete.</p>
    #[serde(rename = "AwsRegion")]
    pub aws_region: String,
    /// <p>Specifies the code for the quota that you want to delete.</p>
    #[serde(rename = "QuotaCode")]
    pub quota_code: String,
    /// <p>Specifies the code for the service that you want to delete.</p>
    #[serde(rename = "ServiceCode")]
    pub service_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteServiceQuotaIncreaseRequestFromTemplateResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateServiceQuotaTemplateRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateServiceQuotaTemplateResponse {}

/// <p>Returns an error that explains why the action did not succeed.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ErrorReason {
    /// <p>Service Quotas returns the following error values. </p> <p> <code>DEPENDENCY_ACCESS_DENIED_ERROR</code> is returned when the caller does not have permission to call the service or service quota. To resolve the error, you need permission to access the service or service quota.</p> <p> <code>DEPENDENCY_THROTTLING_ERROR</code> is returned when the service being called is throttling Service Quotas.</p> <p> <code>DEPENDENCY_SERVICE_ERROR</code> is returned when the service being called has availability issues.</p> <p> <code>SERVICE_QUOTA_NOT_AVAILABLE_ERROR</code> is returned when there was an error in Service Quotas.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The error message that provides more detail.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAWSDefaultServiceQuotaRequest {
    /// <p>Identifies the service quota you want to select.</p>
    #[serde(rename = "QuotaCode")]
    pub quota_code: String,
    /// <p>Specifies the service that you want to use.</p>
    #[serde(rename = "ServiceCode")]
    pub service_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAWSDefaultServiceQuotaResponse {
    /// <p>Returns the <a>ServiceQuota</a> object which contains all values for a quota.</p>
    #[serde(rename = "Quota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota: Option<ServiceQuota>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAssociationForServiceQuotaTemplateRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAssociationForServiceQuotaTemplateResponse {
    /// <p>Specifies whether the template is <code>ASSOCIATED</code> or <code>DISASSOCIATED</code>. If the template is <code>ASSOCIATED</code>, then it requests service quota increases for all new accounts created in your organization. </p>
    #[serde(rename = "ServiceQuotaTemplateAssociationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_quota_template_association_status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRequestedServiceQuotaChangeRequest {
    /// <p>Identifies the quota increase request.</p>
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRequestedServiceQuotaChangeResponse {
    /// <p>Returns the <code>RequestedServiceQuotaChange</code> object for the specific increase request.</p>
    #[serde(rename = "RequestedQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_quota: Option<RequestedServiceQuotaChange>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetServiceQuotaIncreaseRequestFromTemplateRequest {
    /// <p>Specifies the AWS Region for the quota that you want to use.</p>
    #[serde(rename = "AwsRegion")]
    pub aws_region: String,
    /// <p>Specifies the quota you want.</p>
    #[serde(rename = "QuotaCode")]
    pub quota_code: String,
    /// <p>Specifies the service that you want to use.</p>
    #[serde(rename = "ServiceCode")]
    pub service_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetServiceQuotaIncreaseRequestFromTemplateResponse {
    /// <p>This object contains the details about the quota increase request.</p>
    #[serde(rename = "ServiceQuotaIncreaseRequestInTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_quota_increase_request_in_template: Option<ServiceQuotaIncreaseRequestInTemplate>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetServiceQuotaRequest {
    /// <p>Identifies the service quota you want to select.</p>
    #[serde(rename = "QuotaCode")]
    pub quota_code: String,
    /// <p>Specifies the service that you want to use.</p>
    #[serde(rename = "ServiceCode")]
    pub service_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetServiceQuotaResponse {
    /// <p>Returns the <a>ServiceQuota</a> object which contains all values for a quota.</p>
    #[serde(rename = "Quota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota: Option<ServiceQuota>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListAWSDefaultServiceQuotasRequest {
    /// <p>(Optional) Limits the number of results that you want to include in the response. If you don't include this parameter, the response defaults to a value that's specific to the operation. If additional items exist beyond the specified maximum, the <code>NextToken</code> element is present and has a value (isn't null). Include that value as the <code>NextToken</code> request parameter in the call to the operation to get the next part of the results. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>(Optional) Use this parameter in a request if you receive a <code>NextToken</code> response in a previous request that indicates that there's more output available. In a subsequent call, set it to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from. If additional items exist beyond the specified maximum, the <code>NextToken</code> element is present and has a value (isn't null). Include that value as the <code>NextToken</code> request parameter in the call to the operation to get the next part of the results. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Specifies the service that you want to use.</p>
    #[serde(rename = "ServiceCode")]
    pub service_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListAWSDefaultServiceQuotasResponse {
    /// <p>(Optional) Use this parameter in a request if you receive a <code>NextToken</code> response in a previous request that indicates that there's more output available. In a subsequent call, set it to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of the quotas in the account with the AWS default values. </p>
    #[serde(rename = "Quotas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quotas: Option<Vec<ServiceQuota>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListRequestedServiceQuotaChangeHistoryByQuotaRequest {
    /// <p>(Optional) Limits the number of results that you want to include in the response. If you don't include this parameter, the response defaults to a value that's specific to the operation. If additional items exist beyond the specified maximum, the <code>NextToken</code> element is present and has a value (isn't null). Include that value as the <code>NextToken</code> request parameter in the call to the operation to get the next part of the results. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>(Optional) Use this parameter in a request if you receive a <code>NextToken</code> response in a previous request that indicates that there's more output available. In a subsequent call, set it to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Specifies the service quota that you want to use</p>
    #[serde(rename = "QuotaCode")]
    pub quota_code: String,
    /// <p>Specifies the service that you want to use.</p>
    #[serde(rename = "ServiceCode")]
    pub service_code: String,
    /// <p>Specifies the status value of the quota increase request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListRequestedServiceQuotaChangeHistoryByQuotaResponse {
    /// <p>If present in the response, this value indicates there's more output available that what's included in the current response. This can occur even when the response includes no values at all, such as when you ask for a filtered view of a very long list. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to continue processing and get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back empty (as <code>null</code>).</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Returns a list of service quota requests.</p>
    #[serde(rename = "RequestedQuotas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_quotas: Option<Vec<RequestedServiceQuotaChange>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListRequestedServiceQuotaChangeHistoryRequest {
    /// <p>(Optional) Limits the number of results that you want to include in the response. If you don't include this parameter, the response defaults to a value that's specific to the operation. If additional items exist beyond the specified maximum, the <code>NextToken</code> element is present and has a value (isn't null). Include that value as the <code>NextToken</code> request parameter in the call to the operation to get the next part of the results. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>(Optional) Use this parameter in a request if you receive a <code>NextToken</code> response in a previous request that indicates that there's more output available. In a subsequent call, set it to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Specifies the service that you want to use.</p>
    #[serde(rename = "ServiceCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code: Option<String>,
    /// <p>Specifies the status value of the quota increase request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListRequestedServiceQuotaChangeHistoryResponse {
    /// <p>If present in the response, this value indicates there's more output available that what's included in the current response. This can occur even when the response includes no values at all, such as when you ask for a filtered view of a very long list. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to continue processing and get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back empty (as <code>null</code>).</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Returns a list of service quota requests.</p>
    #[serde(rename = "RequestedQuotas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_quotas: Option<Vec<RequestedServiceQuotaChange>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListServiceQuotaIncreaseRequestsInTemplateRequest {
    /// <p>Specifies the AWS Region for the quota that you want to use.</p>
    #[serde(rename = "AwsRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
    /// <p>(Optional) Limits the number of results that you want to include in the response. If you don't include this parameter, the response defaults to a value that's specific to the operation. If additional items exist beyond the specified maximum, the <code>NextToken</code> element is present and has a value (isn't null). Include that value as the <code>NextToken</code> request parameter in the call to the operation to get the next part of the results. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>(Optional) Use this parameter in a request if you receive a <code>NextToken</code> response in a previous request that indicates that there's more output available. In a subsequent call, set it to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The identifier for a service. When performing an operation, use the <code>ServiceCode</code> to specify a particular service. </p>
    #[serde(rename = "ServiceCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListServiceQuotaIncreaseRequestsInTemplateResponse {
    /// <p>If present in the response, this value indicates there's more output available that what's included in the current response. This can occur even when the response includes no values at all, such as when you ask for a filtered view of a very long list. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to continue processing and get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back empty (as <code>null</code>).</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Returns the list of values of the quota increase request in the template.</p>
    #[serde(rename = "ServiceQuotaIncreaseRequestInTemplateList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_quota_increase_request_in_template_list:
        Option<Vec<ServiceQuotaIncreaseRequestInTemplate>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListServiceQuotasRequest {
    /// <p>(Optional) Limits the number of results that you want to include in the response. If you don't include this parameter, the response defaults to a value that's specific to the operation. If additional items exist beyond the specified maximum, the <code>NextToken</code> element is present and has a value (isn't null). Include that value as the <code>NextToken</code> request parameter in the call to the operation to get the next part of the results. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>(Optional) Use this parameter in a request if you receive a <code>NextToken</code> response in a previous request that indicates that there's more output available. In a subsequent call, set it to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The identifier for a service. When performing an operation, use the <code>ServiceCode</code> to specify a particular service. </p>
    #[serde(rename = "ServiceCode")]
    pub service_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListServiceQuotasResponse {
    /// <p>If present in the response, this value indicates there's more output available that what's included in the current response. This can occur even when the response includes no values at all, such as when you ask for a filtered view of a very long list. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to continue processing and get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back empty (as <code>null</code>).</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The response information for a quota lists all attribute information for the quota. </p>
    #[serde(rename = "Quotas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quotas: Option<Vec<ServiceQuota>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListServicesRequest {
    /// <p>(Optional) Limits the number of results that you want to include in the response. If you don't include this parameter, the response defaults to a value that's specific to the operation. If additional items exist beyond the specified maximum, the <code>NextToken</code> element is present and has a value (isn't null). Include that value as the <code>NextToken</code> request parameter in the call to the operation to get the next part of the results. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>(Optional) Use this parameter in a request if you receive a <code>NextToken</code> response in a previous request that indicates that there's more output available. In a subsequent call, set it to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListServicesResponse {
    /// <p>If present in the response, this value indicates there's more output available that what's included in the current response. This can occur even when the response includes no values at all, such as when you ask for a filtered view of a very long list. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to continue processing and get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back empty (as <code>null</code>).</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Returns a list of services. </p>
    #[serde(rename = "Services")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<ServiceInfo>>,
}

/// <p>A structure that uses CloudWatch metrics to gather data about the service quota.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MetricInfo {
    /// <p>A dimension is a name/value pair that is part of the identity of a metric. Every metric has specific characteristics that describe it, and you can think of dimensions as categories for those characteristics. These dimensions are part of the CloudWatch Metric Identity that measures usage against a particular service quota.</p>
    #[serde(rename = "MetricDimensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_dimensions: Option<::std::collections::HashMap<String, String>>,
    /// <p>The name of the CloudWatch metric that measures usage of a service quota. This is a required field.</p>
    #[serde(rename = "MetricName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    /// <p>The namespace of the metric. The namespace is a container for CloudWatch metrics. You can specify a name for the namespace when you create a metric.</p>
    #[serde(rename = "MetricNamespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_namespace: Option<String>,
    /// <p>Statistics are metric data aggregations over specified periods of time. This is the recommended statistic to use when comparing usage in the CloudWatch Metric against your Service Quota.</p>
    #[serde(rename = "MetricStatisticRecommendation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_statistic_recommendation: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutServiceQuotaIncreaseRequestIntoTemplateRequest {
    /// <p>Specifies the AWS Region for the quota. </p>
    #[serde(rename = "AwsRegion")]
    pub aws_region: String,
    /// <p>Specifies the new, increased value for the quota. </p>
    #[serde(rename = "DesiredValue")]
    pub desired_value: f64,
    /// <p>Specifies the service quota that you want to use.</p>
    #[serde(rename = "QuotaCode")]
    pub quota_code: String,
    /// <p>Specifies the service that you want to use.</p>
    #[serde(rename = "ServiceCode")]
    pub service_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutServiceQuotaIncreaseRequestIntoTemplateResponse {
    /// <p>A structure that contains information about one service quota increase request.</p>
    #[serde(rename = "ServiceQuotaIncreaseRequestInTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_quota_increase_request_in_template: Option<ServiceQuotaIncreaseRequestInTemplate>,
}

/// <p>A structure that contains information about the quota period.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct QuotaPeriod {
    /// <p>The time unit of a period.</p>
    #[serde(rename = "PeriodUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_unit: Option<String>,
    /// <p>The value of a period.</p>
    #[serde(rename = "PeriodValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_value: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RequestServiceQuotaIncreaseRequest {
    /// <p>Specifies the value submitted in the service quota increase request. </p>
    #[serde(rename = "DesiredValue")]
    pub desired_value: f64,
    /// <p>Specifies the service quota that you want to use.</p>
    #[serde(rename = "QuotaCode")]
    pub quota_code: String,
    /// <p>Specifies the service that you want to use.</p>
    #[serde(rename = "ServiceCode")]
    pub service_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RequestServiceQuotaIncreaseResponse {
    /// <p>Returns a list of service quota requests.</p>
    #[serde(rename = "RequestedQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_quota: Option<RequestedServiceQuotaChange>,
}

/// <p>A structure that contains information about a requested change for a quota.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RequestedServiceQuotaChange {
    /// <p>The case Id for the service quota increase request.</p>
    #[serde(rename = "CaseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_id: Option<String>,
    /// <p>The date and time when the service quota increase request was received and the case Id was created. </p>
    #[serde(rename = "Created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    /// <p>New increased value for the service quota.</p>
    #[serde(rename = "DesiredValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_value: Option<f64>,
    /// <p>Identifies if the quota is global.</p>
    #[serde(rename = "GlobalQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_quota: Option<bool>,
    /// <p>The unique identifier of a requested service quota change.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The date and time of the most recent change in the service quota increase request.</p>
    #[serde(rename = "LastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the service quota.</p>
    #[serde(rename = "QuotaArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_arn: Option<String>,
    /// <p>Specifies the service quota that you want to use.</p>
    #[serde(rename = "QuotaCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_code: Option<String>,
    /// <p>Name of the service quota.</p>
    #[serde(rename = "QuotaName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_name: Option<String>,
    /// <p>The IAM identity who submitted the service quota increase request.</p>
    #[serde(rename = "Requester")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester: Option<String>,
    /// <p>Specifies the service that you want to use.</p>
    #[serde(rename = "ServiceCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code: Option<String>,
    /// <p>The name of the AWS service specified in the increase request. </p>
    #[serde(rename = "ServiceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    /// <p>State of the service quota increase request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>Specifies the unit used for the quota.</p>
    #[serde(rename = "Unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

/// <p>A structure that contains the <code>ServiceName</code> and <code>ServiceCode</code>. It does not include all details of the service quota. To get those values, use the <a>ListServiceQuotas</a> operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ServiceInfo {
    /// <p>Specifies the service that you want to use.</p>
    #[serde(rename = "ServiceCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code: Option<String>,
    /// <p>The name of the AWS service specified in the increase request. </p>
    #[serde(rename = "ServiceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
}

/// <p>A structure that contains the full set of details that define the service quota.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ServiceQuota {
    /// <p>Specifies if the quota value can be increased.</p>
    #[serde(rename = "Adjustable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustable: Option<bool>,
    /// <p>Specifies the <code>ErrorCode</code> and <code>ErrorMessage</code> when success isn't achieved.</p>
    #[serde(rename = "ErrorReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_reason: Option<ErrorReason>,
    /// <p>Specifies if the quota is global.</p>
    #[serde(rename = "GlobalQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_quota: Option<bool>,
    /// <p>Identifies the unit and value of how time is measured.</p>
    #[serde(rename = "Period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<QuotaPeriod>,
    /// <p>The Amazon Resource Name (ARN) of the service quota.</p>
    #[serde(rename = "QuotaArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_arn: Option<String>,
    /// <p>The code identifier for the service quota specified.</p>
    #[serde(rename = "QuotaCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_code: Option<String>,
    /// <p>The name identifier of the service quota.</p>
    #[serde(rename = "QuotaName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_name: Option<String>,
    /// <p>Specifies the service that you want to use.</p>
    #[serde(rename = "ServiceCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code: Option<String>,
    /// <p>The name of the AWS service specified in the increase request. </p>
    #[serde(rename = "ServiceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    /// <p>The unit of measurement for the value of the service quota.</p>
    #[serde(rename = "Unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// <p>Specifies the details about the measurement. </p>
    #[serde(rename = "UsageMetric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_metric: Option<MetricInfo>,
    /// <p>The value of service quota.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

/// <p>A structure that contains information about one service quota increase request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ServiceQuotaIncreaseRequestInTemplate {
    /// <p>The AWS Region where the increase request occurs.</p>
    #[serde(rename = "AwsRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
    /// <p>Identifies the new, increased value of the service quota in the increase request. </p>
    #[serde(rename = "DesiredValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_value: Option<f64>,
    /// <p>Specifies if the quota is a global quota.</p>
    #[serde(rename = "GlobalQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_quota: Option<bool>,
    /// <p>The code identifier for the service quota specified in the increase request.</p>
    #[serde(rename = "QuotaCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_code: Option<String>,
    /// <p>The name of the service quota in the increase request.</p>
    #[serde(rename = "QuotaName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_name: Option<String>,
    /// <p>The code identifier for the AWS service specified in the increase request.</p>
    #[serde(rename = "ServiceCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code: Option<String>,
    /// <p>The name of the AWS service specified in the increase request. </p>
    #[serde(rename = "ServiceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    /// <p>The unit of measure for the increase request.</p>
    #[serde(rename = "Unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

/// Errors returned by AssociateServiceQuotaTemplate
#[derive(Debug, PartialEq)]
pub enum AssociateServiceQuotaTemplateError {
    /// <p>The action you attempted is not allowed unless Service Access with Service Quotas is enabled in your organization. To enable, call <a>AssociateServiceQuotaTemplate</a>.</p>
    AWSServiceAccessNotEnabled(String),
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>You can't perform this action because a dependency does not have access.</p>
    DependencyAccessDenied(String),
    /// <p>The account making this call is not a member of an organization.</p>
    NoAvailableOrganization(String),
    /// <p>The organization that your account belongs to, is not in All Features mode. To enable all features mode, see <a href="https://docs.aws.amazon.com/organizations/latest/APIReference/API_EnableAllFeatures.html">EnableAllFeatures</a>.</p>
    OrganizationNotInAllFeaturesMode(String),
    /// <p>Something went wrong. </p>
    Service(String),
    /// <p>The Service Quotas template is not available in the Region where you are making the request. Please make the request in us-east-1. </p>
    TemplatesNotAvailableInRegion(String),
    /// <p>Due to throttling, the request was denied. Slow down the rate of request calls, or request an increase for this quota. </p>
    TooManyRequests(String),
}

impl AssociateServiceQuotaTemplateError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AssociateServiceQuotaTemplateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSServiceAccessNotEnabledException" => {
                    return RusotoError::Service(
                        AssociateServiceQuotaTemplateError::AWSServiceAccessNotEnabled(err.msg),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(AssociateServiceQuotaTemplateError::AccessDenied(
                        err.msg,
                    ))
                }
                "DependencyAccessDeniedException" => {
                    return RusotoError::Service(
                        AssociateServiceQuotaTemplateError::DependencyAccessDenied(err.msg),
                    )
                }
                "NoAvailableOrganizationException" => {
                    return RusotoError::Service(
                        AssociateServiceQuotaTemplateError::NoAvailableOrganization(err.msg),
                    )
                }
                "OrganizationNotInAllFeaturesModeException" => {
                    return RusotoError::Service(
                        AssociateServiceQuotaTemplateError::OrganizationNotInAllFeaturesMode(
                            err.msg,
                        ),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(AssociateServiceQuotaTemplateError::Service(
                        err.msg,
                    ))
                }
                "TemplatesNotAvailableInRegionException" => {
                    return RusotoError::Service(
                        AssociateServiceQuotaTemplateError::TemplatesNotAvailableInRegion(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        AssociateServiceQuotaTemplateError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AssociateServiceQuotaTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateServiceQuotaTemplateError::AWSServiceAccessNotEnabled(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateServiceQuotaTemplateError::AccessDenied(ref cause) => write!(f, "{}", cause),
            AssociateServiceQuotaTemplateError::DependencyAccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateServiceQuotaTemplateError::NoAvailableOrganization(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateServiceQuotaTemplateError::OrganizationNotInAllFeaturesMode(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateServiceQuotaTemplateError::Service(ref cause) => write!(f, "{}", cause),
            AssociateServiceQuotaTemplateError::TemplatesNotAvailableInRegion(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateServiceQuotaTemplateError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AssociateServiceQuotaTemplateError {}
/// Errors returned by DeleteServiceQuotaIncreaseRequestFromTemplate
#[derive(Debug, PartialEq)]
pub enum DeleteServiceQuotaIncreaseRequestFromTemplateError {
    /// <p>The action you attempted is not allowed unless Service Access with Service Quotas is enabled in your organization. To enable, call <a>AssociateServiceQuotaTemplate</a>.</p>
    AWSServiceAccessNotEnabled(String),
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>You can't perform this action because a dependency does not have access.</p>
    DependencyAccessDenied(String),
    /// <p>Invalid input was provided. </p>
    IllegalArgument(String),
    /// <p>The account making this call is not a member of an organization.</p>
    NoAvailableOrganization(String),
    /// <p>The specified resource does not exist.</p>
    NoSuchResource(String),
    /// <p>Something went wrong. </p>
    Service(String),
    /// <p>The Service Quotas template is not available in the Region where you are making the request. Please make the request in us-east-1. </p>
    TemplatesNotAvailableInRegion(String),
    /// <p>Due to throttling, the request was denied. Slow down the rate of request calls, or request an increase for this quota. </p>
    TooManyRequests(String),
}

impl DeleteServiceQuotaIncreaseRequestFromTemplateError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteServiceQuotaIncreaseRequestFromTemplateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                                "AWSServiceAccessNotEnabledException" => return RusotoError::Service(DeleteServiceQuotaIncreaseRequestFromTemplateError::AWSServiceAccessNotEnabled(err.msg)),
"AccessDeniedException" => return RusotoError::Service(DeleteServiceQuotaIncreaseRequestFromTemplateError::AccessDenied(err.msg)),
"DependencyAccessDeniedException" => return RusotoError::Service(DeleteServiceQuotaIncreaseRequestFromTemplateError::DependencyAccessDenied(err.msg)),
"IllegalArgumentException" => return RusotoError::Service(DeleteServiceQuotaIncreaseRequestFromTemplateError::IllegalArgument(err.msg)),
"NoAvailableOrganizationException" => return RusotoError::Service(DeleteServiceQuotaIncreaseRequestFromTemplateError::NoAvailableOrganization(err.msg)),
"NoSuchResourceException" => return RusotoError::Service(DeleteServiceQuotaIncreaseRequestFromTemplateError::NoSuchResource(err.msg)),
"ServiceException" => return RusotoError::Service(DeleteServiceQuotaIncreaseRequestFromTemplateError::Service(err.msg)),
"TemplatesNotAvailableInRegionException" => return RusotoError::Service(DeleteServiceQuotaIncreaseRequestFromTemplateError::TemplatesNotAvailableInRegion(err.msg)),
"TooManyRequestsException" => return RusotoError::Service(DeleteServiceQuotaIncreaseRequestFromTemplateError::TooManyRequests(err.msg)),
"ValidationException" => return RusotoError::Validation(err.msg),
_ => {}
                            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteServiceQuotaIncreaseRequestFromTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteServiceQuotaIncreaseRequestFromTemplateError::AWSServiceAccessNotEnabled(
                ref cause,
            ) => write!(f, "{}", cause),
            DeleteServiceQuotaIncreaseRequestFromTemplateError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteServiceQuotaIncreaseRequestFromTemplateError::DependencyAccessDenied(
                ref cause,
            ) => write!(f, "{}", cause),
            DeleteServiceQuotaIncreaseRequestFromTemplateError::IllegalArgument(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteServiceQuotaIncreaseRequestFromTemplateError::NoAvailableOrganization(
                ref cause,
            ) => write!(f, "{}", cause),
            DeleteServiceQuotaIncreaseRequestFromTemplateError::NoSuchResource(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteServiceQuotaIncreaseRequestFromTemplateError::Service(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteServiceQuotaIncreaseRequestFromTemplateError::TemplatesNotAvailableInRegion(
                ref cause,
            ) => write!(f, "{}", cause),
            DeleteServiceQuotaIncreaseRequestFromTemplateError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteServiceQuotaIncreaseRequestFromTemplateError {}
/// Errors returned by DisassociateServiceQuotaTemplate
#[derive(Debug, PartialEq)]
pub enum DisassociateServiceQuotaTemplateError {
    /// <p>The action you attempted is not allowed unless Service Access with Service Quotas is enabled in your organization. To enable, call <a>AssociateServiceQuotaTemplate</a>.</p>
    AWSServiceAccessNotEnabled(String),
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>You can't perform this action because a dependency does not have access.</p>
    DependencyAccessDenied(String),
    /// <p>The account making this call is not a member of an organization.</p>
    NoAvailableOrganization(String),
    /// <p>Something went wrong. </p>
    Service(String),
    /// <p>The quota request template is not associated with your organization. </p> <p>To use the template, call <a>AssociateServiceQuotaTemplate</a>. </p>
    ServiceQuotaTemplateNotInUse(String),
    /// <p>The Service Quotas template is not available in the Region where you are making the request. Please make the request in us-east-1. </p>
    TemplatesNotAvailableInRegion(String),
    /// <p>Due to throttling, the request was denied. Slow down the rate of request calls, or request an increase for this quota. </p>
    TooManyRequests(String),
}

impl DisassociateServiceQuotaTemplateError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateServiceQuotaTemplateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSServiceAccessNotEnabledException" => {
                    return RusotoError::Service(
                        DisassociateServiceQuotaTemplateError::AWSServiceAccessNotEnabled(err.msg),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        DisassociateServiceQuotaTemplateError::AccessDenied(err.msg),
                    )
                }
                "DependencyAccessDeniedException" => {
                    return RusotoError::Service(
                        DisassociateServiceQuotaTemplateError::DependencyAccessDenied(err.msg),
                    )
                }
                "NoAvailableOrganizationException" => {
                    return RusotoError::Service(
                        DisassociateServiceQuotaTemplateError::NoAvailableOrganization(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(DisassociateServiceQuotaTemplateError::Service(
                        err.msg,
                    ))
                }
                "ServiceQuotaTemplateNotInUseException" => {
                    return RusotoError::Service(
                        DisassociateServiceQuotaTemplateError::ServiceQuotaTemplateNotInUse(
                            err.msg,
                        ),
                    )
                }
                "TemplatesNotAvailableInRegionException" => {
                    return RusotoError::Service(
                        DisassociateServiceQuotaTemplateError::TemplatesNotAvailableInRegion(
                            err.msg,
                        ),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        DisassociateServiceQuotaTemplateError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisassociateServiceQuotaTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateServiceQuotaTemplateError::AWSServiceAccessNotEnabled(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateServiceQuotaTemplateError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateServiceQuotaTemplateError::DependencyAccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateServiceQuotaTemplateError::NoAvailableOrganization(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateServiceQuotaTemplateError::Service(ref cause) => write!(f, "{}", cause),
            DisassociateServiceQuotaTemplateError::ServiceQuotaTemplateNotInUse(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateServiceQuotaTemplateError::TemplatesNotAvailableInRegion(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateServiceQuotaTemplateError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DisassociateServiceQuotaTemplateError {}
/// Errors returned by GetAWSDefaultServiceQuota
#[derive(Debug, PartialEq)]
pub enum GetAWSDefaultServiceQuotaError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>Invalid input was provided. </p>
    IllegalArgument(String),
    /// <p>The specified resource does not exist.</p>
    NoSuchResource(String),
    /// <p>Something went wrong. </p>
    Service(String),
    /// <p>Due to throttling, the request was denied. Slow down the rate of request calls, or request an increase for this quota. </p>
    TooManyRequests(String),
}

impl GetAWSDefaultServiceQuotaError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAWSDefaultServiceQuotaError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetAWSDefaultServiceQuotaError::AccessDenied(
                        err.msg,
                    ))
                }
                "IllegalArgumentException" => {
                    return RusotoError::Service(GetAWSDefaultServiceQuotaError::IllegalArgument(
                        err.msg,
                    ))
                }
                "NoSuchResourceException" => {
                    return RusotoError::Service(GetAWSDefaultServiceQuotaError::NoSuchResource(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetAWSDefaultServiceQuotaError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetAWSDefaultServiceQuotaError::TooManyRequests(
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
impl fmt::Display for GetAWSDefaultServiceQuotaError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAWSDefaultServiceQuotaError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetAWSDefaultServiceQuotaError::IllegalArgument(ref cause) => write!(f, "{}", cause),
            GetAWSDefaultServiceQuotaError::NoSuchResource(ref cause) => write!(f, "{}", cause),
            GetAWSDefaultServiceQuotaError::Service(ref cause) => write!(f, "{}", cause),
            GetAWSDefaultServiceQuotaError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetAWSDefaultServiceQuotaError {}
/// Errors returned by GetAssociationForServiceQuotaTemplate
#[derive(Debug, PartialEq)]
pub enum GetAssociationForServiceQuotaTemplateError {
    /// <p>The action you attempted is not allowed unless Service Access with Service Quotas is enabled in your organization. To enable, call <a>AssociateServiceQuotaTemplate</a>.</p>
    AWSServiceAccessNotEnabled(String),
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>You can't perform this action because a dependency does not have access.</p>
    DependencyAccessDenied(String),
    /// <p>The account making this call is not a member of an organization.</p>
    NoAvailableOrganization(String),
    /// <p>Something went wrong. </p>
    Service(String),
    /// <p>The quota request template is not associated with your organization. </p> <p>To use the template, call <a>AssociateServiceQuotaTemplate</a>. </p>
    ServiceQuotaTemplateNotInUse(String),
    /// <p>The Service Quotas template is not available in the Region where you are making the request. Please make the request in us-east-1. </p>
    TemplatesNotAvailableInRegion(String),
    /// <p>Due to throttling, the request was denied. Slow down the rate of request calls, or request an increase for this quota. </p>
    TooManyRequests(String),
}

impl GetAssociationForServiceQuotaTemplateError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetAssociationForServiceQuotaTemplateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSServiceAccessNotEnabledException" => {
                    return RusotoError::Service(
                        GetAssociationForServiceQuotaTemplateError::AWSServiceAccessNotEnabled(
                            err.msg,
                        ),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        GetAssociationForServiceQuotaTemplateError::AccessDenied(err.msg),
                    )
                }
                "DependencyAccessDeniedException" => {
                    return RusotoError::Service(
                        GetAssociationForServiceQuotaTemplateError::DependencyAccessDenied(err.msg),
                    )
                }
                "NoAvailableOrganizationException" => {
                    return RusotoError::Service(
                        GetAssociationForServiceQuotaTemplateError::NoAvailableOrganization(
                            err.msg,
                        ),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(
                        GetAssociationForServiceQuotaTemplateError::Service(err.msg),
                    )
                }
                "ServiceQuotaTemplateNotInUseException" => {
                    return RusotoError::Service(
                        GetAssociationForServiceQuotaTemplateError::ServiceQuotaTemplateNotInUse(
                            err.msg,
                        ),
                    )
                }
                "TemplatesNotAvailableInRegionException" => {
                    return RusotoError::Service(
                        GetAssociationForServiceQuotaTemplateError::TemplatesNotAvailableInRegion(
                            err.msg,
                        ),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        GetAssociationForServiceQuotaTemplateError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetAssociationForServiceQuotaTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAssociationForServiceQuotaTemplateError::AWSServiceAccessNotEnabled(ref cause) => {
                write!(f, "{}", cause)
            }
            GetAssociationForServiceQuotaTemplateError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            GetAssociationForServiceQuotaTemplateError::DependencyAccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            GetAssociationForServiceQuotaTemplateError::NoAvailableOrganization(ref cause) => {
                write!(f, "{}", cause)
            }
            GetAssociationForServiceQuotaTemplateError::Service(ref cause) => {
                write!(f, "{}", cause)
            }
            GetAssociationForServiceQuotaTemplateError::ServiceQuotaTemplateNotInUse(ref cause) => {
                write!(f, "{}", cause)
            }
            GetAssociationForServiceQuotaTemplateError::TemplatesNotAvailableInRegion(
                ref cause,
            ) => write!(f, "{}", cause),
            GetAssociationForServiceQuotaTemplateError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetAssociationForServiceQuotaTemplateError {}
/// Errors returned by GetRequestedServiceQuotaChange
#[derive(Debug, PartialEq)]
pub enum GetRequestedServiceQuotaChangeError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>Invalid input was provided. </p>
    IllegalArgument(String),
    /// <p>The specified resource does not exist.</p>
    NoSuchResource(String),
    /// <p>Something went wrong. </p>
    Service(String),
    /// <p>Due to throttling, the request was denied. Slow down the rate of request calls, or request an increase for this quota. </p>
    TooManyRequests(String),
}

impl GetRequestedServiceQuotaChangeError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetRequestedServiceQuotaChangeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetRequestedServiceQuotaChangeError::AccessDenied(
                        err.msg,
                    ))
                }
                "IllegalArgumentException" => {
                    return RusotoError::Service(
                        GetRequestedServiceQuotaChangeError::IllegalArgument(err.msg),
                    )
                }
                "NoSuchResourceException" => {
                    return RusotoError::Service(
                        GetRequestedServiceQuotaChangeError::NoSuchResource(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(GetRequestedServiceQuotaChangeError::Service(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        GetRequestedServiceQuotaChangeError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetRequestedServiceQuotaChangeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRequestedServiceQuotaChangeError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetRequestedServiceQuotaChangeError::IllegalArgument(ref cause) => {
                write!(f, "{}", cause)
            }
            GetRequestedServiceQuotaChangeError::NoSuchResource(ref cause) => {
                write!(f, "{}", cause)
            }
            GetRequestedServiceQuotaChangeError::Service(ref cause) => write!(f, "{}", cause),
            GetRequestedServiceQuotaChangeError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetRequestedServiceQuotaChangeError {}
/// Errors returned by GetServiceQuota
#[derive(Debug, PartialEq)]
pub enum GetServiceQuotaError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>Invalid input was provided. </p>
    IllegalArgument(String),
    /// <p>The specified resource does not exist.</p>
    NoSuchResource(String),
    /// <p>Something went wrong. </p>
    Service(String),
    /// <p>Due to throttling, the request was denied. Slow down the rate of request calls, or request an increase for this quota. </p>
    TooManyRequests(String),
}

impl GetServiceQuotaError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetServiceQuotaError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetServiceQuotaError::AccessDenied(err.msg))
                }
                "IllegalArgumentException" => {
                    return RusotoError::Service(GetServiceQuotaError::IllegalArgument(err.msg))
                }
                "NoSuchResourceException" => {
                    return RusotoError::Service(GetServiceQuotaError::NoSuchResource(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetServiceQuotaError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetServiceQuotaError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetServiceQuotaError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetServiceQuotaError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetServiceQuotaError::IllegalArgument(ref cause) => write!(f, "{}", cause),
            GetServiceQuotaError::NoSuchResource(ref cause) => write!(f, "{}", cause),
            GetServiceQuotaError::Service(ref cause) => write!(f, "{}", cause),
            GetServiceQuotaError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetServiceQuotaError {}
/// Errors returned by GetServiceQuotaIncreaseRequestFromTemplate
#[derive(Debug, PartialEq)]
pub enum GetServiceQuotaIncreaseRequestFromTemplateError {
    /// <p>The action you attempted is not allowed unless Service Access with Service Quotas is enabled in your organization. To enable, call <a>AssociateServiceQuotaTemplate</a>.</p>
    AWSServiceAccessNotEnabled(String),
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>You can't perform this action because a dependency does not have access.</p>
    DependencyAccessDenied(String),
    /// <p>Invalid input was provided. </p>
    IllegalArgument(String),
    /// <p>The account making this call is not a member of an organization.</p>
    NoAvailableOrganization(String),
    /// <p>The specified resource does not exist.</p>
    NoSuchResource(String),
    /// <p>Something went wrong. </p>
    Service(String),
    /// <p>The Service Quotas template is not available in the Region where you are making the request. Please make the request in us-east-1. </p>
    TemplatesNotAvailableInRegion(String),
    /// <p>Due to throttling, the request was denied. Slow down the rate of request calls, or request an increase for this quota. </p>
    TooManyRequests(String),
}

impl GetServiceQuotaIncreaseRequestFromTemplateError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetServiceQuotaIncreaseRequestFromTemplateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSServiceAccessNotEnabledException" => {
                    return RusotoError::Service(
                        GetServiceQuotaIncreaseRequestFromTemplateError::AWSServiceAccessNotEnabled(
                            err.msg,
                        ),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        GetServiceQuotaIncreaseRequestFromTemplateError::AccessDenied(err.msg),
                    )
                }
                "DependencyAccessDeniedException" => {
                    return RusotoError::Service(
                        GetServiceQuotaIncreaseRequestFromTemplateError::DependencyAccessDenied(
                            err.msg,
                        ),
                    )
                }
                "IllegalArgumentException" => {
                    return RusotoError::Service(
                        GetServiceQuotaIncreaseRequestFromTemplateError::IllegalArgument(err.msg),
                    )
                }
                "NoAvailableOrganizationException" => {
                    return RusotoError::Service(
                        GetServiceQuotaIncreaseRequestFromTemplateError::NoAvailableOrganization(
                            err.msg,
                        ),
                    )
                }
                "NoSuchResourceException" => {
                    return RusotoError::Service(
                        GetServiceQuotaIncreaseRequestFromTemplateError::NoSuchResource(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(
                        GetServiceQuotaIncreaseRequestFromTemplateError::Service(err.msg),
                    )
                }
                "TemplatesNotAvailableInRegionException" => return RusotoError::Service(
                    GetServiceQuotaIncreaseRequestFromTemplateError::TemplatesNotAvailableInRegion(
                        err.msg,
                    ),
                ),
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        GetServiceQuotaIncreaseRequestFromTemplateError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetServiceQuotaIncreaseRequestFromTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetServiceQuotaIncreaseRequestFromTemplateError::AWSServiceAccessNotEnabled(
                ref cause,
            ) => write!(f, "{}", cause),
            GetServiceQuotaIncreaseRequestFromTemplateError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            GetServiceQuotaIncreaseRequestFromTemplateError::DependencyAccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            GetServiceQuotaIncreaseRequestFromTemplateError::IllegalArgument(ref cause) => {
                write!(f, "{}", cause)
            }
            GetServiceQuotaIncreaseRequestFromTemplateError::NoAvailableOrganization(ref cause) => {
                write!(f, "{}", cause)
            }
            GetServiceQuotaIncreaseRequestFromTemplateError::NoSuchResource(ref cause) => {
                write!(f, "{}", cause)
            }
            GetServiceQuotaIncreaseRequestFromTemplateError::Service(ref cause) => {
                write!(f, "{}", cause)
            }
            GetServiceQuotaIncreaseRequestFromTemplateError::TemplatesNotAvailableInRegion(
                ref cause,
            ) => write!(f, "{}", cause),
            GetServiceQuotaIncreaseRequestFromTemplateError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetServiceQuotaIncreaseRequestFromTemplateError {}
/// Errors returned by ListAWSDefaultServiceQuotas
#[derive(Debug, PartialEq)]
pub enum ListAWSDefaultServiceQuotasError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>Invalid input was provided. </p>
    IllegalArgument(String),
    /// <p>Invalid input was provided.</p>
    InvalidPaginationToken(String),
    /// <p>The specified resource does not exist.</p>
    NoSuchResource(String),
    /// <p>Something went wrong. </p>
    Service(String),
    /// <p>Due to throttling, the request was denied. Slow down the rate of request calls, or request an increase for this quota. </p>
    TooManyRequests(String),
}

impl ListAWSDefaultServiceQuotasError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListAWSDefaultServiceQuotasError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListAWSDefaultServiceQuotasError::AccessDenied(
                        err.msg,
                    ))
                }
                "IllegalArgumentException" => {
                    return RusotoError::Service(ListAWSDefaultServiceQuotasError::IllegalArgument(
                        err.msg,
                    ))
                }
                "InvalidPaginationTokenException" => {
                    return RusotoError::Service(
                        ListAWSDefaultServiceQuotasError::InvalidPaginationToken(err.msg),
                    )
                }
                "NoSuchResourceException" => {
                    return RusotoError::Service(ListAWSDefaultServiceQuotasError::NoSuchResource(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListAWSDefaultServiceQuotasError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListAWSDefaultServiceQuotasError::TooManyRequests(
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
impl fmt::Display for ListAWSDefaultServiceQuotasError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListAWSDefaultServiceQuotasError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListAWSDefaultServiceQuotasError::IllegalArgument(ref cause) => write!(f, "{}", cause),
            ListAWSDefaultServiceQuotasError::InvalidPaginationToken(ref cause) => {
                write!(f, "{}", cause)
            }
            ListAWSDefaultServiceQuotasError::NoSuchResource(ref cause) => write!(f, "{}", cause),
            ListAWSDefaultServiceQuotasError::Service(ref cause) => write!(f, "{}", cause),
            ListAWSDefaultServiceQuotasError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListAWSDefaultServiceQuotasError {}
/// Errors returned by ListRequestedServiceQuotaChangeHistory
#[derive(Debug, PartialEq)]
pub enum ListRequestedServiceQuotaChangeHistoryError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>Invalid input was provided. </p>
    IllegalArgument(String),
    /// <p>Invalid input was provided.</p>
    InvalidPaginationToken(String),
    /// <p>The specified resource does not exist.</p>
    NoSuchResource(String),
    /// <p>Something went wrong. </p>
    Service(String),
    /// <p>Due to throttling, the request was denied. Slow down the rate of request calls, or request an increase for this quota. </p>
    TooManyRequests(String),
}

impl ListRequestedServiceQuotaChangeHistoryError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListRequestedServiceQuotaChangeHistoryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        ListRequestedServiceQuotaChangeHistoryError::AccessDenied(err.msg),
                    )
                }
                "IllegalArgumentException" => {
                    return RusotoError::Service(
                        ListRequestedServiceQuotaChangeHistoryError::IllegalArgument(err.msg),
                    )
                }
                "InvalidPaginationTokenException" => {
                    return RusotoError::Service(
                        ListRequestedServiceQuotaChangeHistoryError::InvalidPaginationToken(
                            err.msg,
                        ),
                    )
                }
                "NoSuchResourceException" => {
                    return RusotoError::Service(
                        ListRequestedServiceQuotaChangeHistoryError::NoSuchResource(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(
                        ListRequestedServiceQuotaChangeHistoryError::Service(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        ListRequestedServiceQuotaChangeHistoryError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListRequestedServiceQuotaChangeHistoryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListRequestedServiceQuotaChangeHistoryError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            ListRequestedServiceQuotaChangeHistoryError::IllegalArgument(ref cause) => {
                write!(f, "{}", cause)
            }
            ListRequestedServiceQuotaChangeHistoryError::InvalidPaginationToken(ref cause) => {
                write!(f, "{}", cause)
            }
            ListRequestedServiceQuotaChangeHistoryError::NoSuchResource(ref cause) => {
                write!(f, "{}", cause)
            }
            ListRequestedServiceQuotaChangeHistoryError::Service(ref cause) => {
                write!(f, "{}", cause)
            }
            ListRequestedServiceQuotaChangeHistoryError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListRequestedServiceQuotaChangeHistoryError {}
/// Errors returned by ListRequestedServiceQuotaChangeHistoryByQuota
#[derive(Debug, PartialEq)]
pub enum ListRequestedServiceQuotaChangeHistoryByQuotaError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>Invalid input was provided. </p>
    IllegalArgument(String),
    /// <p>Invalid input was provided.</p>
    InvalidPaginationToken(String),
    /// <p>The specified resource does not exist.</p>
    NoSuchResource(String),
    /// <p>Something went wrong. </p>
    Service(String),
    /// <p>Due to throttling, the request was denied. Slow down the rate of request calls, or request an increase for this quota. </p>
    TooManyRequests(String),
}

impl ListRequestedServiceQuotaChangeHistoryByQuotaError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListRequestedServiceQuotaChangeHistoryByQuotaError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        ListRequestedServiceQuotaChangeHistoryByQuotaError::AccessDenied(err.msg),
                    )
                }
                "IllegalArgumentException" => {
                    return RusotoError::Service(
                        ListRequestedServiceQuotaChangeHistoryByQuotaError::IllegalArgument(
                            err.msg,
                        ),
                    )
                }
                "InvalidPaginationTokenException" => {
                    return RusotoError::Service(
                        ListRequestedServiceQuotaChangeHistoryByQuotaError::InvalidPaginationToken(
                            err.msg,
                        ),
                    )
                }
                "NoSuchResourceException" => {
                    return RusotoError::Service(
                        ListRequestedServiceQuotaChangeHistoryByQuotaError::NoSuchResource(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(
                        ListRequestedServiceQuotaChangeHistoryByQuotaError::Service(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        ListRequestedServiceQuotaChangeHistoryByQuotaError::TooManyRequests(
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
impl fmt::Display for ListRequestedServiceQuotaChangeHistoryByQuotaError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListRequestedServiceQuotaChangeHistoryByQuotaError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            ListRequestedServiceQuotaChangeHistoryByQuotaError::IllegalArgument(ref cause) => {
                write!(f, "{}", cause)
            }
            ListRequestedServiceQuotaChangeHistoryByQuotaError::InvalidPaginationToken(
                ref cause,
            ) => write!(f, "{}", cause),
            ListRequestedServiceQuotaChangeHistoryByQuotaError::NoSuchResource(ref cause) => {
                write!(f, "{}", cause)
            }
            ListRequestedServiceQuotaChangeHistoryByQuotaError::Service(ref cause) => {
                write!(f, "{}", cause)
            }
            ListRequestedServiceQuotaChangeHistoryByQuotaError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListRequestedServiceQuotaChangeHistoryByQuotaError {}
/// Errors returned by ListServiceQuotaIncreaseRequestsInTemplate
#[derive(Debug, PartialEq)]
pub enum ListServiceQuotaIncreaseRequestsInTemplateError {
    /// <p>The action you attempted is not allowed unless Service Access with Service Quotas is enabled in your organization. To enable, call <a>AssociateServiceQuotaTemplate</a>.</p>
    AWSServiceAccessNotEnabled(String),
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>You can't perform this action because a dependency does not have access.</p>
    DependencyAccessDenied(String),
    /// <p>Invalid input was provided. </p>
    IllegalArgument(String),
    /// <p>The account making this call is not a member of an organization.</p>
    NoAvailableOrganization(String),
    /// <p>Something went wrong. </p>
    Service(String),
    /// <p>The Service Quotas template is not available in the Region where you are making the request. Please make the request in us-east-1. </p>
    TemplatesNotAvailableInRegion(String),
    /// <p>Due to throttling, the request was denied. Slow down the rate of request calls, or request an increase for this quota. </p>
    TooManyRequests(String),
}

impl ListServiceQuotaIncreaseRequestsInTemplateError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListServiceQuotaIncreaseRequestsInTemplateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSServiceAccessNotEnabledException" => {
                    return RusotoError::Service(
                        ListServiceQuotaIncreaseRequestsInTemplateError::AWSServiceAccessNotEnabled(
                            err.msg,
                        ),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        ListServiceQuotaIncreaseRequestsInTemplateError::AccessDenied(err.msg),
                    )
                }
                "DependencyAccessDeniedException" => {
                    return RusotoError::Service(
                        ListServiceQuotaIncreaseRequestsInTemplateError::DependencyAccessDenied(
                            err.msg,
                        ),
                    )
                }
                "IllegalArgumentException" => {
                    return RusotoError::Service(
                        ListServiceQuotaIncreaseRequestsInTemplateError::IllegalArgument(err.msg),
                    )
                }
                "NoAvailableOrganizationException" => {
                    return RusotoError::Service(
                        ListServiceQuotaIncreaseRequestsInTemplateError::NoAvailableOrganization(
                            err.msg,
                        ),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(
                        ListServiceQuotaIncreaseRequestsInTemplateError::Service(err.msg),
                    )
                }
                "TemplatesNotAvailableInRegionException" => return RusotoError::Service(
                    ListServiceQuotaIncreaseRequestsInTemplateError::TemplatesNotAvailableInRegion(
                        err.msg,
                    ),
                ),
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        ListServiceQuotaIncreaseRequestsInTemplateError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListServiceQuotaIncreaseRequestsInTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListServiceQuotaIncreaseRequestsInTemplateError::AWSServiceAccessNotEnabled(
                ref cause,
            ) => write!(f, "{}", cause),
            ListServiceQuotaIncreaseRequestsInTemplateError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            ListServiceQuotaIncreaseRequestsInTemplateError::DependencyAccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            ListServiceQuotaIncreaseRequestsInTemplateError::IllegalArgument(ref cause) => {
                write!(f, "{}", cause)
            }
            ListServiceQuotaIncreaseRequestsInTemplateError::NoAvailableOrganization(ref cause) => {
                write!(f, "{}", cause)
            }
            ListServiceQuotaIncreaseRequestsInTemplateError::Service(ref cause) => {
                write!(f, "{}", cause)
            }
            ListServiceQuotaIncreaseRequestsInTemplateError::TemplatesNotAvailableInRegion(
                ref cause,
            ) => write!(f, "{}", cause),
            ListServiceQuotaIncreaseRequestsInTemplateError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListServiceQuotaIncreaseRequestsInTemplateError {}
/// Errors returned by ListServiceQuotas
#[derive(Debug, PartialEq)]
pub enum ListServiceQuotasError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>Invalid input was provided. </p>
    IllegalArgument(String),
    /// <p>Invalid input was provided.</p>
    InvalidPaginationToken(String),
    /// <p>The specified resource does not exist.</p>
    NoSuchResource(String),
    /// <p>Something went wrong. </p>
    Service(String),
    /// <p>Due to throttling, the request was denied. Slow down the rate of request calls, or request an increase for this quota. </p>
    TooManyRequests(String),
}

impl ListServiceQuotasError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListServiceQuotasError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListServiceQuotasError::AccessDenied(err.msg))
                }
                "IllegalArgumentException" => {
                    return RusotoError::Service(ListServiceQuotasError::IllegalArgument(err.msg))
                }
                "InvalidPaginationTokenException" => {
                    return RusotoError::Service(ListServiceQuotasError::InvalidPaginationToken(
                        err.msg,
                    ))
                }
                "NoSuchResourceException" => {
                    return RusotoError::Service(ListServiceQuotasError::NoSuchResource(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListServiceQuotasError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListServiceQuotasError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListServiceQuotasError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListServiceQuotasError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListServiceQuotasError::IllegalArgument(ref cause) => write!(f, "{}", cause),
            ListServiceQuotasError::InvalidPaginationToken(ref cause) => write!(f, "{}", cause),
            ListServiceQuotasError::NoSuchResource(ref cause) => write!(f, "{}", cause),
            ListServiceQuotasError::Service(ref cause) => write!(f, "{}", cause),
            ListServiceQuotasError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListServiceQuotasError {}
/// Errors returned by ListServices
#[derive(Debug, PartialEq)]
pub enum ListServicesError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>Invalid input was provided. </p>
    IllegalArgument(String),
    /// <p>Invalid input was provided.</p>
    InvalidPaginationToken(String),
    /// <p>Something went wrong. </p>
    Service(String),
    /// <p>Due to throttling, the request was denied. Slow down the rate of request calls, or request an increase for this quota. </p>
    TooManyRequests(String),
}

impl ListServicesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListServicesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListServicesError::AccessDenied(err.msg))
                }
                "IllegalArgumentException" => {
                    return RusotoError::Service(ListServicesError::IllegalArgument(err.msg))
                }
                "InvalidPaginationTokenException" => {
                    return RusotoError::Service(ListServicesError::InvalidPaginationToken(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListServicesError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListServicesError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListServicesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListServicesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListServicesError::IllegalArgument(ref cause) => write!(f, "{}", cause),
            ListServicesError::InvalidPaginationToken(ref cause) => write!(f, "{}", cause),
            ListServicesError::Service(ref cause) => write!(f, "{}", cause),
            ListServicesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListServicesError {}
/// Errors returned by PutServiceQuotaIncreaseRequestIntoTemplate
#[derive(Debug, PartialEq)]
pub enum PutServiceQuotaIncreaseRequestIntoTemplateError {
    /// <p>The action you attempted is not allowed unless Service Access with Service Quotas is enabled in your organization. To enable, call <a>AssociateServiceQuotaTemplate</a>.</p>
    AWSServiceAccessNotEnabled(String),
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>You can't perform this action because a dependency does not have access.</p>
    DependencyAccessDenied(String),
    /// <p>Invalid input was provided. </p>
    IllegalArgument(String),
    /// <p>The account making this call is not a member of an organization.</p>
    NoAvailableOrganization(String),
    /// <p>The specified resource does not exist.</p>
    NoSuchResource(String),
    /// <p>You have exceeded your service quota. To perform the requested action, remove some of the relevant resources, or use Service Quotas to request a service quota increase.</p>
    QuotaExceeded(String),
    /// <p>Something went wrong. </p>
    Service(String),
    /// <p>The Service Quotas template is not available in the Region where you are making the request. Please make the request in us-east-1. </p>
    TemplatesNotAvailableInRegion(String),
    /// <p>Due to throttling, the request was denied. Slow down the rate of request calls, or request an increase for this quota. </p>
    TooManyRequests(String),
}

impl PutServiceQuotaIncreaseRequestIntoTemplateError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutServiceQuotaIncreaseRequestIntoTemplateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSServiceAccessNotEnabledException" => {
                    return RusotoError::Service(
                        PutServiceQuotaIncreaseRequestIntoTemplateError::AWSServiceAccessNotEnabled(
                            err.msg,
                        ),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        PutServiceQuotaIncreaseRequestIntoTemplateError::AccessDenied(err.msg),
                    )
                }
                "DependencyAccessDeniedException" => {
                    return RusotoError::Service(
                        PutServiceQuotaIncreaseRequestIntoTemplateError::DependencyAccessDenied(
                            err.msg,
                        ),
                    )
                }
                "IllegalArgumentException" => {
                    return RusotoError::Service(
                        PutServiceQuotaIncreaseRequestIntoTemplateError::IllegalArgument(err.msg),
                    )
                }
                "NoAvailableOrganizationException" => {
                    return RusotoError::Service(
                        PutServiceQuotaIncreaseRequestIntoTemplateError::NoAvailableOrganization(
                            err.msg,
                        ),
                    )
                }
                "NoSuchResourceException" => {
                    return RusotoError::Service(
                        PutServiceQuotaIncreaseRequestIntoTemplateError::NoSuchResource(err.msg),
                    )
                }
                "QuotaExceededException" => {
                    return RusotoError::Service(
                        PutServiceQuotaIncreaseRequestIntoTemplateError::QuotaExceeded(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(
                        PutServiceQuotaIncreaseRequestIntoTemplateError::Service(err.msg),
                    )
                }
                "TemplatesNotAvailableInRegionException" => return RusotoError::Service(
                    PutServiceQuotaIncreaseRequestIntoTemplateError::TemplatesNotAvailableInRegion(
                        err.msg,
                    ),
                ),
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        PutServiceQuotaIncreaseRequestIntoTemplateError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutServiceQuotaIncreaseRequestIntoTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutServiceQuotaIncreaseRequestIntoTemplateError::AWSServiceAccessNotEnabled(
                ref cause,
            ) => write!(f, "{}", cause),
            PutServiceQuotaIncreaseRequestIntoTemplateError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            PutServiceQuotaIncreaseRequestIntoTemplateError::DependencyAccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            PutServiceQuotaIncreaseRequestIntoTemplateError::IllegalArgument(ref cause) => {
                write!(f, "{}", cause)
            }
            PutServiceQuotaIncreaseRequestIntoTemplateError::NoAvailableOrganization(ref cause) => {
                write!(f, "{}", cause)
            }
            PutServiceQuotaIncreaseRequestIntoTemplateError::NoSuchResource(ref cause) => {
                write!(f, "{}", cause)
            }
            PutServiceQuotaIncreaseRequestIntoTemplateError::QuotaExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            PutServiceQuotaIncreaseRequestIntoTemplateError::Service(ref cause) => {
                write!(f, "{}", cause)
            }
            PutServiceQuotaIncreaseRequestIntoTemplateError::TemplatesNotAvailableInRegion(
                ref cause,
            ) => write!(f, "{}", cause),
            PutServiceQuotaIncreaseRequestIntoTemplateError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutServiceQuotaIncreaseRequestIntoTemplateError {}
/// Errors returned by RequestServiceQuotaIncrease
#[derive(Debug, PartialEq)]
pub enum RequestServiceQuotaIncreaseError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>You can't perform this action because a dependency does not have access.</p>
    DependencyAccessDenied(String),
    /// <p>Invalid input was provided. </p>
    IllegalArgument(String),
    /// <p>Invalid input was provided for the . </p>
    InvalidResourceState(String),
    /// <p>The specified resource does not exist.</p>
    NoSuchResource(String),
    /// <p>You have exceeded your service quota. To perform the requested action, remove some of the relevant resources, or use Service Quotas to request a service quota increase.</p>
    QuotaExceeded(String),
    /// <p>The specified resource already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>Something went wrong. </p>
    Service(String),
    /// <p>Due to throttling, the request was denied. Slow down the rate of request calls, or request an increase for this quota. </p>
    TooManyRequests(String),
}

impl RequestServiceQuotaIncreaseError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<RequestServiceQuotaIncreaseError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(RequestServiceQuotaIncreaseError::AccessDenied(
                        err.msg,
                    ))
                }
                "DependencyAccessDeniedException" => {
                    return RusotoError::Service(
                        RequestServiceQuotaIncreaseError::DependencyAccessDenied(err.msg),
                    )
                }
                "IllegalArgumentException" => {
                    return RusotoError::Service(RequestServiceQuotaIncreaseError::IllegalArgument(
                        err.msg,
                    ))
                }
                "InvalidResourceStateException" => {
                    return RusotoError::Service(
                        RequestServiceQuotaIncreaseError::InvalidResourceState(err.msg),
                    )
                }
                "NoSuchResourceException" => {
                    return RusotoError::Service(RequestServiceQuotaIncreaseError::NoSuchResource(
                        err.msg,
                    ))
                }
                "QuotaExceededException" => {
                    return RusotoError::Service(RequestServiceQuotaIncreaseError::QuotaExceeded(
                        err.msg,
                    ))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(
                        RequestServiceQuotaIncreaseError::ResourceAlreadyExists(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(RequestServiceQuotaIncreaseError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(RequestServiceQuotaIncreaseError::TooManyRequests(
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
impl fmt::Display for RequestServiceQuotaIncreaseError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RequestServiceQuotaIncreaseError::AccessDenied(ref cause) => write!(f, "{}", cause),
            RequestServiceQuotaIncreaseError::DependencyAccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            RequestServiceQuotaIncreaseError::IllegalArgument(ref cause) => write!(f, "{}", cause),
            RequestServiceQuotaIncreaseError::InvalidResourceState(ref cause) => {
                write!(f, "{}", cause)
            }
            RequestServiceQuotaIncreaseError::NoSuchResource(ref cause) => write!(f, "{}", cause),
            RequestServiceQuotaIncreaseError::QuotaExceeded(ref cause) => write!(f, "{}", cause),
            RequestServiceQuotaIncreaseError::ResourceAlreadyExists(ref cause) => {
                write!(f, "{}", cause)
            }
            RequestServiceQuotaIncreaseError::Service(ref cause) => write!(f, "{}", cause),
            RequestServiceQuotaIncreaseError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RequestServiceQuotaIncreaseError {}
/// Trait representing the capabilities of the Service Quotas API. Service Quotas clients implement this trait.
pub trait ServiceQuotas {
    /// <p>Associates the Service Quotas template with your organization so that when new accounts are created in your organization, the template submits increase requests for the specified service quotas. Use the Service Quotas template to request an increase for any adjustable quota value. After you define the Service Quotas template, use this operation to associate, or enable, the template. </p>
    fn associate_service_quota_template(
        &self,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        AssociateServiceQuotaTemplateResponse,
                        RusotoError<AssociateServiceQuotaTemplateError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Removes a service quota increase request from the Service Quotas template. </p>
    fn delete_service_quota_increase_request_from_template(
        &self,
        input: DeleteServiceQuotaIncreaseRequestFromTemplateRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DeleteServiceQuotaIncreaseRequestFromTemplateResponse,
                        RusotoError<DeleteServiceQuotaIncreaseRequestFromTemplateError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p><p>Disables the Service Quotas template. Once the template is disabled, it does not request quota increases for new accounts in your organization. Disabling the quota template does not apply the quota increase requests from the template. </p> <p> <b>Related operations</b> </p> <ul> <li> <p>To enable the quota template, call <a>AssociateServiceQuotaTemplate</a>. </p> </li> <li> <p>To delete a specific service quota from the template, use <a>DeleteServiceQuotaIncreaseRequestFromTemplate</a>.</p> </li> </ul></p>
    fn disassociate_service_quota_template(
        &self,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DisassociateServiceQuotaTemplateResponse,
                        RusotoError<DisassociateServiceQuotaTemplateError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Retrieves the default service quotas values. The Value returned for each quota is the AWS default value, even if the quotas have been increased.. </p>
    fn get_aws_default_service_quota(
        &self,
        input: GetAWSDefaultServiceQuotaRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        GetAWSDefaultServiceQuotaResponse,
                        RusotoError<GetAWSDefaultServiceQuotaError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Retrieves the <code>ServiceQuotaTemplateAssociationStatus</code> value from the service. Use this action to determine if the Service Quota template is associated, or enabled. </p>
    fn get_association_for_service_quota_template(
        &self,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        GetAssociationForServiceQuotaTemplateResponse,
                        RusotoError<GetAssociationForServiceQuotaTemplateError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Retrieves the details for a particular increase request. </p>
    fn get_requested_service_quota_change(
        &self,
        input: GetRequestedServiceQuotaChangeRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        GetRequestedServiceQuotaChangeResponse,
                        RusotoError<GetRequestedServiceQuotaChangeError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Returns the details for the specified service quota. This operation provides a different Value than the <code>GetAWSDefaultServiceQuota</code> operation. This operation returns the applied value for each quota. <code>GetAWSDefaultServiceQuota</code> returns the default AWS value for each quota. </p>
    fn get_service_quota(
        &self,
        input: GetServiceQuotaRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<GetServiceQuotaResponse, RusotoError<GetServiceQuotaError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Returns the details of the service quota increase request in your template.</p>
    fn get_service_quota_increase_request_from_template(
        &self,
        input: GetServiceQuotaIncreaseRequestFromTemplateRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        GetServiceQuotaIncreaseRequestFromTemplateResponse,
                        RusotoError<GetServiceQuotaIncreaseRequestFromTemplateError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p><p>Lists all default service quotas for the specified AWS service or all AWS services. ListAWSDefaultServiceQuotas is similar to <a>ListServiceQuotas</a> except for the Value object. The Value object returned by <code>ListAWSDefaultServiceQuotas</code> is the default value assigned by AWS. This request returns a list of all service quotas for the specified service. The listing of each you&#39;ll see the default values are the values that AWS provides for the quotas. </p> <note> <p>Always check the <code>NextToken</code> response parameter when calling any of the <code>List*</code> operations. These operations can return an unexpected list of results, even when there are more results available. When this happens, the <code>NextToken</code> response parameter contains a value to pass the next call to the same API to request the next part of the list.</p> </note></p>
    fn list_aws_default_service_quotas(
        &self,
        input: ListAWSDefaultServiceQuotasRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListAWSDefaultServiceQuotasResponse,
                        RusotoError<ListAWSDefaultServiceQuotasError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Requests a list of the changes to quotas for a service.</p>
    fn list_requested_service_quota_change_history(
        &self,
        input: ListRequestedServiceQuotaChangeHistoryRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListRequestedServiceQuotaChangeHistoryResponse,
                        RusotoError<ListRequestedServiceQuotaChangeHistoryError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Requests a list of the changes to specific service quotas. This command provides additional granularity over the <code>ListRequestedServiceQuotaChangeHistory</code> command. Once a quota change request has reached <code>CASE_CLOSED, APPROVED,</code> or <code>DENIED</code>, the history has been kept for 90 days.</p>
    fn list_requested_service_quota_change_history_by_quota(
        &self,
        input: ListRequestedServiceQuotaChangeHistoryByQuotaRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListRequestedServiceQuotaChangeHistoryByQuotaResponse,
                        RusotoError<ListRequestedServiceQuotaChangeHistoryByQuotaError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Returns a list of the quota increase requests in the template. </p>
    fn list_service_quota_increase_requests_in_template(
        &self,
        input: ListServiceQuotaIncreaseRequestsInTemplateRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListServiceQuotaIncreaseRequestsInTemplateResponse,
                        RusotoError<ListServiceQuotaIncreaseRequestsInTemplateError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p><p>Lists all service quotas for the specified AWS service. This request returns a list of the service quotas for the specified service. you&#39;ll see the default values are the values that AWS provides for the quotas. </p> <note> <p>Always check the <code>NextToken</code> response parameter when calling any of the <code>List*</code> operations. These operations can return an unexpected list of results, even when there are more results available. When this happens, the <code>NextToken</code> response parameter contains a value to pass the next call to the same API to request the next part of the list.</p> </note></p>
    fn list_service_quotas(
        &self,
        input: ListServiceQuotasRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<ListServiceQuotasResponse, RusotoError<ListServiceQuotasError>>,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Lists the AWS services available in Service Quotas. Not all AWS services are available in Service Quotas. To list the see the list of the service quotas for a specific service, use <a>ListServiceQuotas</a>.</p>
    fn list_services(
        &self,
        input: ListServicesRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<ListServicesResponse, RusotoError<ListServicesError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Defines and adds a quota to the service quota template. To add a quota to the template, you must provide the <code>ServiceCode</code>, <code>QuotaCode</code>, <code>AwsRegion</code>, and <code>DesiredValue</code>. Once you add a quota to the template, use <a>ListServiceQuotaIncreaseRequestsInTemplate</a> to see the list of quotas in the template.</p>
    fn put_service_quota_increase_request_into_template(
        &self,
        input: PutServiceQuotaIncreaseRequestIntoTemplateRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        PutServiceQuotaIncreaseRequestIntoTemplateResponse,
                        RusotoError<PutServiceQuotaIncreaseRequestIntoTemplateError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Retrieves the details of a service quota increase request. The response to this command provides the details in the <a>RequestedServiceQuotaChange</a> object. </p>
    fn request_service_quota_increase(
        &self,
        input: RequestServiceQuotaIncreaseRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        RequestServiceQuotaIncreaseResponse,
                        RusotoError<RequestServiceQuotaIncreaseError>,
                    >,
                > + Send
                + 'static,
        >,
    >;
}
/// A client for the Service Quotas API.
#[derive(Clone)]
pub struct ServiceQuotasClient {
    client: Client,
    region: region::Region,
}

impl ServiceQuotasClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ServiceQuotasClient {
        ServiceQuotasClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ServiceQuotasClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        ServiceQuotasClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> ServiceQuotasClient {
        ServiceQuotasClient { client, region }
    }
}

impl ServiceQuotas for ServiceQuotasClient {
    /// <p>Associates the Service Quotas template with your organization so that when new accounts are created in your organization, the template submits increase requests for the specified service quotas. Use the Service Quotas template to request an increase for any adjustable quota value. After you define the Service Quotas template, use this operation to associate, or enable, the template. </p>
    fn associate_service_quota_template(
        &self,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        AssociateServiceQuotaTemplateResponse,
                        RusotoError<AssociateServiceQuotaTemplateError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "servicequotas", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ServiceQuotasV20190624.AssociateServiceQuotaTemplate",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<AssociateServiceQuotaTemplateResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(AssociateServiceQuotaTemplateError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Removes a service quota increase request from the Service Quotas template. </p>
    fn delete_service_quota_increase_request_from_template(
        &self,
        input: DeleteServiceQuotaIncreaseRequestFromTemplateRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DeleteServiceQuotaIncreaseRequestFromTemplateResponse,
                        RusotoError<DeleteServiceQuotaIncreaseRequestFromTemplateError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "servicequotas", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ServiceQuotasV20190624.DeleteServiceQuotaIncreaseRequestFromTemplate",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<DeleteServiceQuotaIncreaseRequestFromTemplateResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(DeleteServiceQuotaIncreaseRequestFromTemplateError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p><p>Disables the Service Quotas template. Once the template is disabled, it does not request quota increases for new accounts in your organization. Disabling the quota template does not apply the quota increase requests from the template. </p> <p> <b>Related operations</b> </p> <ul> <li> <p>To enable the quota template, call <a>AssociateServiceQuotaTemplate</a>. </p> </li> <li> <p>To delete a specific service quota from the template, use <a>DeleteServiceQuotaIncreaseRequestFromTemplate</a>.</p> </li> </ul></p>
    fn disassociate_service_quota_template(
        &self,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DisassociateServiceQuotaTemplateResponse,
                        RusotoError<DisassociateServiceQuotaTemplateError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "servicequotas", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ServiceQuotasV20190624.DisassociateServiceQuotaTemplate",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<DisassociateServiceQuotaTemplateResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(DisassociateServiceQuotaTemplateError::from_response(
                    response,
                ))
            }
        }
        .boxed()
    }

    /// <p>Retrieves the default service quotas values. The Value returned for each quota is the AWS default value, even if the quotas have been increased.. </p>
    fn get_aws_default_service_quota(
        &self,
        input: GetAWSDefaultServiceQuotaRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        GetAWSDefaultServiceQuotaResponse,
                        RusotoError<GetAWSDefaultServiceQuotaError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "servicequotas", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ServiceQuotasV20190624.GetAWSDefaultServiceQuota",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<GetAWSDefaultServiceQuotaResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(GetAWSDefaultServiceQuotaError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Retrieves the <code>ServiceQuotaTemplateAssociationStatus</code> value from the service. Use this action to determine if the Service Quota template is associated, or enabled. </p>
    fn get_association_for_service_quota_template(
        &self,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        GetAssociationForServiceQuotaTemplateResponse,
                        RusotoError<GetAssociationForServiceQuotaTemplateError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "servicequotas", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ServiceQuotasV20190624.GetAssociationForServiceQuotaTemplate",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<GetAssociationForServiceQuotaTemplateResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(GetAssociationForServiceQuotaTemplateError::from_response(
                    response,
                ))
            }
        }
        .boxed()
    }

    /// <p>Retrieves the details for a particular increase request. </p>
    fn get_requested_service_quota_change(
        &self,
        input: GetRequestedServiceQuotaChangeRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        GetRequestedServiceQuotaChangeResponse,
                        RusotoError<GetRequestedServiceQuotaChangeError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "servicequotas", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ServiceQuotasV20190624.GetRequestedServiceQuotaChange",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<GetRequestedServiceQuotaChangeResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(GetRequestedServiceQuotaChangeError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Returns the details for the specified service quota. This operation provides a different Value than the <code>GetAWSDefaultServiceQuota</code> operation. This operation returns the applied value for each quota. <code>GetAWSDefaultServiceQuota</code> returns the default AWS value for each quota. </p>
    fn get_service_quota(
        &self,
        input: GetServiceQuotaRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<GetServiceQuotaResponse, RusotoError<GetServiceQuotaError>>>
                + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "servicequotas", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ServiceQuotasV20190624.GetServiceQuota");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<GetServiceQuotaResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(GetServiceQuotaError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Returns the details of the service quota increase request in your template.</p>
    fn get_service_quota_increase_request_from_template(
        &self,
        input: GetServiceQuotaIncreaseRequestFromTemplateRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        GetServiceQuotaIncreaseRequestFromTemplateResponse,
                        RusotoError<GetServiceQuotaIncreaseRequestFromTemplateError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "servicequotas", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ServiceQuotasV20190624.GetServiceQuotaIncreaseRequestFromTemplate",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<GetServiceQuotaIncreaseRequestFromTemplateResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(GetServiceQuotaIncreaseRequestFromTemplateError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p><p>Lists all default service quotas for the specified AWS service or all AWS services. ListAWSDefaultServiceQuotas is similar to <a>ListServiceQuotas</a> except for the Value object. The Value object returned by <code>ListAWSDefaultServiceQuotas</code> is the default value assigned by AWS. This request returns a list of all service quotas for the specified service. The listing of each you&#39;ll see the default values are the values that AWS provides for the quotas. </p> <note> <p>Always check the <code>NextToken</code> response parameter when calling any of the <code>List*</code> operations. These operations can return an unexpected list of results, even when there are more results available. When this happens, the <code>NextToken</code> response parameter contains a value to pass the next call to the same API to request the next part of the list.</p> </note></p>
    fn list_aws_default_service_quotas(
        &self,
        input: ListAWSDefaultServiceQuotasRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListAWSDefaultServiceQuotasResponse,
                        RusotoError<ListAWSDefaultServiceQuotasError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "servicequotas", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ServiceQuotasV20190624.ListAWSDefaultServiceQuotas",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<ListAWSDefaultServiceQuotasResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(ListAWSDefaultServiceQuotasError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Requests a list of the changes to quotas for a service.</p>
    fn list_requested_service_quota_change_history(
        &self,
        input: ListRequestedServiceQuotaChangeHistoryRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListRequestedServiceQuotaChangeHistoryResponse,
                        RusotoError<ListRequestedServiceQuotaChangeHistoryError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "servicequotas", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ServiceQuotasV20190624.ListRequestedServiceQuotaChangeHistory",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<ListRequestedServiceQuotaChangeHistoryResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(ListRequestedServiceQuotaChangeHistoryError::from_response(
                    response,
                ))
            }
        }
        .boxed()
    }

    /// <p>Requests a list of the changes to specific service quotas. This command provides additional granularity over the <code>ListRequestedServiceQuotaChangeHistory</code> command. Once a quota change request has reached <code>CASE_CLOSED, APPROVED,</code> or <code>DENIED</code>, the history has been kept for 90 days.</p>
    fn list_requested_service_quota_change_history_by_quota(
        &self,
        input: ListRequestedServiceQuotaChangeHistoryByQuotaRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListRequestedServiceQuotaChangeHistoryByQuotaResponse,
                        RusotoError<ListRequestedServiceQuotaChangeHistoryByQuotaError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "servicequotas", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ServiceQuotasV20190624.ListRequestedServiceQuotaChangeHistoryByQuota",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<ListRequestedServiceQuotaChangeHistoryByQuotaResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(ListRequestedServiceQuotaChangeHistoryByQuotaError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Returns a list of the quota increase requests in the template. </p>
    fn list_service_quota_increase_requests_in_template(
        &self,
        input: ListServiceQuotaIncreaseRequestsInTemplateRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListServiceQuotaIncreaseRequestsInTemplateResponse,
                        RusotoError<ListServiceQuotaIncreaseRequestsInTemplateError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "servicequotas", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ServiceQuotasV20190624.ListServiceQuotaIncreaseRequestsInTemplate",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<ListServiceQuotaIncreaseRequestsInTemplateResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(ListServiceQuotaIncreaseRequestsInTemplateError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p><p>Lists all service quotas for the specified AWS service. This request returns a list of the service quotas for the specified service. you&#39;ll see the default values are the values that AWS provides for the quotas. </p> <note> <p>Always check the <code>NextToken</code> response parameter when calling any of the <code>List*</code> operations. These operations can return an unexpected list of results, even when there are more results available. When this happens, the <code>NextToken</code> response parameter contains a value to pass the next call to the same API to request the next part of the list.</p> </note></p>
    fn list_service_quotas(
        &self,
        input: ListServiceQuotasRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<ListServiceQuotasResponse, RusotoError<ListServiceQuotasError>>,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "servicequotas", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ServiceQuotasV20190624.ListServiceQuotas");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<ListServiceQuotasResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(ListServiceQuotasError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Lists the AWS services available in Service Quotas. Not all AWS services are available in Service Quotas. To list the see the list of the service quotas for a specific service, use <a>ListServiceQuotas</a>.</p>
    fn list_services(
        &self,
        input: ListServicesRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<ListServicesResponse, RusotoError<ListServicesError>>>
                + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "servicequotas", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ServiceQuotasV20190624.ListServices");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<ListServicesResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(ListServicesError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Defines and adds a quota to the service quota template. To add a quota to the template, you must provide the <code>ServiceCode</code>, <code>QuotaCode</code>, <code>AwsRegion</code>, and <code>DesiredValue</code>. Once you add a quota to the template, use <a>ListServiceQuotaIncreaseRequestsInTemplate</a> to see the list of quotas in the template.</p>
    fn put_service_quota_increase_request_into_template(
        &self,
        input: PutServiceQuotaIncreaseRequestIntoTemplateRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        PutServiceQuotaIncreaseRequestIntoTemplateResponse,
                        RusotoError<PutServiceQuotaIncreaseRequestIntoTemplateError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "servicequotas", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ServiceQuotasV20190624.PutServiceQuotaIncreaseRequestIntoTemplate",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<PutServiceQuotaIncreaseRequestIntoTemplateResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(PutServiceQuotaIncreaseRequestIntoTemplateError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Retrieves the details of a service quota increase request. The response to this command provides the details in the <a>RequestedServiceQuotaChange</a> object. </p>
    fn request_service_quota_increase(
        &self,
        input: RequestServiceQuotaIncreaseRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        RequestServiceQuotaIncreaseResponse,
                        RusotoError<RequestServiceQuotaIncreaseError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "servicequotas", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ServiceQuotasV20190624.RequestServiceQuotaIncrease",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<RequestServiceQuotaIncreaseResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(RequestServiceQuotaIncreaseError::from_response(response))
            }
        }
        .boxed()
    }
}
