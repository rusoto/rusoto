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
use rusoto_core::request::HttpResponse;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

impl ServiceQuotasClient {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request =
            SignedRequest::new(http_method, "servicequotas", &self.region, request_uri);

        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request
    }

    async fn sign_and_dispatch<E>(
        &self,
        request: SignedRequest,
        from_response: fn(BufferedHttpResponse) -> RusotoError<E>,
    ) -> Result<HttpResponse, RusotoError<E>> {
        let mut response = self.client.sign_and_dispatch(request).await?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(from_response(response));
        }

        Ok(response)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateServiceQuotaTemplateRequest {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateServiceQuotaTemplateResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteServiceQuotaIncreaseRequestFromTemplateRequest {
    /// <p>The AWS Region.</p>
    #[serde(rename = "AwsRegion")]
    pub aws_region: String,
    /// <p>The quota identifier.</p>
    #[serde(rename = "QuotaCode")]
    pub quota_code: String,
    /// <p>The service identifier.</p>
    #[serde(rename = "ServiceCode")]
    pub service_code: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteServiceQuotaIncreaseRequestFromTemplateResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateServiceQuotaTemplateRequest {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateServiceQuotaTemplateResponse {}

/// <p>An error that explains why an action did not succeed.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ErrorReason {
    /// <p><p>Service Quotas returns the following error values:</p> <ul> <li> <p> <code>DEPENDENCY<em>ACCESS</em>DENIED<em>ERROR</code> - The caller does not have the required permissions to complete the action. To resolve the error, you must have permission to access the service or quota.</p> </li> <li> <p> <code>DEPENDENCY</em>THROTTLING<em>ERROR</code> - The service is throttling Service Quotas.</p> </li> <li> <p> <code>DEPENDENCY</em>SERVICE<em>ERROR</code> - The service is not available.</p> </li> <li> <p> <code>SERVICE</em>QUOTA<em>NOT</em>AVAILABLE_ERROR</code> - There was an error in Service Quotas.</p> </li> </ul></p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The error message.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAWSDefaultServiceQuotaRequest {
    /// <p>The quota identifier.</p>
    #[serde(rename = "QuotaCode")]
    pub quota_code: String,
    /// <p>The service identifier.</p>
    #[serde(rename = "ServiceCode")]
    pub service_code: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAWSDefaultServiceQuotaResponse {
    /// <p>Information about the quota.</p>
    #[serde(rename = "Quota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota: Option<ServiceQuota>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAssociationForServiceQuotaTemplateRequest {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAssociationForServiceQuotaTemplateResponse {
    /// <p>The association status. If the status is <code>ASSOCIATED</code>, the quota increase requests in the template are automatically applied to new accounts in your organization.</p>
    #[serde(rename = "ServiceQuotaTemplateAssociationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_quota_template_association_status: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRequestedServiceQuotaChangeRequest {
    /// <p>The ID of the quota increase request.</p>
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRequestedServiceQuotaChangeResponse {
    /// <p>Information about the quota increase request.</p>
    #[serde(rename = "RequestedQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_quota: Option<RequestedServiceQuotaChange>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetServiceQuotaIncreaseRequestFromTemplateRequest {
    /// <p>The AWS Region.</p>
    #[serde(rename = "AwsRegion")]
    pub aws_region: String,
    /// <p>The quota identifier.</p>
    #[serde(rename = "QuotaCode")]
    pub quota_code: String,
    /// <p>The service identifier.</p>
    #[serde(rename = "ServiceCode")]
    pub service_code: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetServiceQuotaIncreaseRequestFromTemplateResponse {
    /// <p>Information about the quota increase request.</p>
    #[serde(rename = "ServiceQuotaIncreaseRequestInTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_quota_increase_request_in_template: Option<ServiceQuotaIncreaseRequestInTemplate>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetServiceQuotaRequest {
    /// <p>The quota identifier.</p>
    #[serde(rename = "QuotaCode")]
    pub quota_code: String,
    /// <p>The service identifier.</p>
    #[serde(rename = "ServiceCode")]
    pub service_code: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetServiceQuotaResponse {
    /// <p>Information about the quota.</p>
    #[serde(rename = "Quota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota: Option<ServiceQuota>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListAWSDefaultServiceQuotasRequest {
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, if any, make another call with the token returned from this call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The service identifier.</p>
    #[serde(rename = "ServiceCode")]
    pub service_code: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListAWSDefaultServiceQuotasResponse {
    /// <p>The token to use to retrieve the next page of results. This value is null when there are no more results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the quotas.</p>
    #[serde(rename = "Quotas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quotas: Option<Vec<ServiceQuota>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListRequestedServiceQuotaChangeHistoryByQuotaRequest {
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, if any, make another call with the token returned from this call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The quota identifier.</p>
    #[serde(rename = "QuotaCode")]
    pub quota_code: String,
    /// <p>The service identifier.</p>
    #[serde(rename = "ServiceCode")]
    pub service_code: String,
    /// <p>The status value of the quota increase request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListRequestedServiceQuotaChangeHistoryByQuotaResponse {
    /// <p>The token to use to retrieve the next page of results. This value is null when there are no more results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the quota increase requests.</p>
    #[serde(rename = "RequestedQuotas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_quotas: Option<Vec<RequestedServiceQuotaChange>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListRequestedServiceQuotaChangeHistoryRequest {
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, if any, make another call with the token returned from this call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The service identifier.</p>
    #[serde(rename = "ServiceCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code: Option<String>,
    /// <p>The status of the quota increase request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListRequestedServiceQuotaChangeHistoryResponse {
    /// <p>The token to use to retrieve the next page of results. This value is null when there are no more results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the quota increase requests.</p>
    #[serde(rename = "RequestedQuotas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_quotas: Option<Vec<RequestedServiceQuotaChange>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListServiceQuotaIncreaseRequestsInTemplateRequest {
    /// <p>The AWS Region.</p>
    #[serde(rename = "AwsRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, if any, make another call with the token returned from this call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The service identifier.</p>
    #[serde(rename = "ServiceCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListServiceQuotaIncreaseRequestsInTemplateResponse {
    /// <p>The token to use to retrieve the next page of results. This value is null when there are no more results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the quota increase requests.</p>
    #[serde(rename = "ServiceQuotaIncreaseRequestInTemplateList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_quota_increase_request_in_template_list:
        Option<Vec<ServiceQuotaIncreaseRequestInTemplate>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListServiceQuotasRequest {
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, if any, make another call with the token returned from this call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The service identifier.</p>
    #[serde(rename = "ServiceCode")]
    pub service_code: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListServiceQuotasResponse {
    /// <p>The token to use to retrieve the next page of results. This value is null when there are no more results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the quotas.</p>
    #[serde(rename = "Quotas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quotas: Option<Vec<ServiceQuota>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListServicesRequest {
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, if any, make another call with the token returned from this call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListServicesResponse {
    /// <p>The token to use to retrieve the next page of results. This value is null when there are no more results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the services.</p>
    #[serde(rename = "Services")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<ServiceInfo>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The Amazon Resource Name (ARN) for the applied quota for which you want to list tags. You can get this information by using the Service Quotas console, or by listing the quotas using the <a href="https://docs.aws.amazon.com/cli/latest/reference/service-quotas/list-service-quotas.html">list-service-quotas</a> AWS CLI command or the <a href="https://docs.aws.amazon.com/servicequotas/2019-06-24/apireference/API_ListServiceQuotas.html">ListServiceQuotas</a> AWS API operation.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>A complex data type that contains zero or more tag elements.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Information about the CloudWatch metric that reflects quota usage.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MetricInfo {
    /// <p>The metric dimension. This is a name/value pair that is part of the identity of a metric.</p>
    #[serde(rename = "MetricDimensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_dimensions: Option<::std::collections::HashMap<String, String>>,
    /// <p>The name of the metric.</p>
    #[serde(rename = "MetricName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    /// <p>The namespace of the metric.</p>
    #[serde(rename = "MetricNamespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_namespace: Option<String>,
    /// <p>The metric statistic that we recommend you use when determining quota usage.</p>
    #[serde(rename = "MetricStatisticRecommendation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_statistic_recommendation: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutServiceQuotaIncreaseRequestIntoTemplateRequest {
    /// <p>The AWS Region.</p>
    #[serde(rename = "AwsRegion")]
    pub aws_region: String,
    /// <p>The new, increased value for the quota.</p>
    #[serde(rename = "DesiredValue")]
    pub desired_value: f64,
    /// <p>The quota identifier.</p>
    #[serde(rename = "QuotaCode")]
    pub quota_code: String,
    /// <p>The service identifier.</p>
    #[serde(rename = "ServiceCode")]
    pub service_code: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutServiceQuotaIncreaseRequestIntoTemplateResponse {
    /// <p>Information about the quota increase request.</p>
    #[serde(rename = "ServiceQuotaIncreaseRequestInTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_quota_increase_request_in_template: Option<ServiceQuotaIncreaseRequestInTemplate>,
}

/// <p>Information about the quota period.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct QuotaPeriod {
    /// <p>The time unit.</p>
    #[serde(rename = "PeriodUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_unit: Option<String>,
    /// <p>The value.</p>
    #[serde(rename = "PeriodValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_value: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RequestServiceQuotaIncreaseRequest {
    /// <p>The new, increased value for the quota.</p>
    #[serde(rename = "DesiredValue")]
    pub desired_value: f64,
    /// <p>The quota identifier.</p>
    #[serde(rename = "QuotaCode")]
    pub quota_code: String,
    /// <p>The service identifier.</p>
    #[serde(rename = "ServiceCode")]
    pub service_code: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RequestServiceQuotaIncreaseResponse {
    /// <p>Information about the quota increase request.</p>
    #[serde(rename = "RequestedQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_quota: Option<RequestedServiceQuotaChange>,
}

/// <p>Information about a quota increase request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RequestedServiceQuotaChange {
    /// <p>The case ID.</p>
    #[serde(rename = "CaseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_id: Option<String>,
    /// <p>The date and time when the quota increase request was received and the case ID was created.</p>
    #[serde(rename = "Created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    /// <p>The new, increased value for the quota.</p>
    #[serde(rename = "DesiredValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_value: Option<f64>,
    /// <p>Indicates whether the quota is global.</p>
    #[serde(rename = "GlobalQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_quota: Option<bool>,
    /// <p>The unique identifier.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The date and time of the most recent change.</p>
    #[serde(rename = "LastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the quota.</p>
    #[serde(rename = "QuotaArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_arn: Option<String>,
    /// <p>The quota identifier.</p>
    #[serde(rename = "QuotaCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_code: Option<String>,
    /// <p>The quota name.</p>
    #[serde(rename = "QuotaName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_name: Option<String>,
    /// <p>The IAM identity of the requester.</p>
    #[serde(rename = "Requester")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester: Option<String>,
    /// <p>The service identifier.</p>
    #[serde(rename = "ServiceCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code: Option<String>,
    /// <p>The service name.</p>
    #[serde(rename = "ServiceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    /// <p>The state of the quota increase request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The unit of measurement.</p>
    #[serde(rename = "Unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

/// <p>Information about a service.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ServiceInfo {
    /// <p>The service identifier.</p>
    #[serde(rename = "ServiceCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code: Option<String>,
    /// <p>The service name.</p>
    #[serde(rename = "ServiceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
}

/// <p>Information about a quota.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ServiceQuota {
    /// <p>Indicates whether the quota value can be increased.</p>
    #[serde(rename = "Adjustable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustable: Option<bool>,
    /// <p>The error code and error reason.</p>
    #[serde(rename = "ErrorReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_reason: Option<ErrorReason>,
    /// <p>Indicates whether the quota is global.</p>
    #[serde(rename = "GlobalQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_quota: Option<bool>,
    /// <p>The period of time.</p>
    #[serde(rename = "Period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<QuotaPeriod>,
    /// <p>The Amazon Resource Name (ARN) of the quota.</p>
    #[serde(rename = "QuotaArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_arn: Option<String>,
    /// <p>The quota identifier.</p>
    #[serde(rename = "QuotaCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_code: Option<String>,
    /// <p>The quota name.</p>
    #[serde(rename = "QuotaName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_name: Option<String>,
    /// <p>The service identifier.</p>
    #[serde(rename = "ServiceCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code: Option<String>,
    /// <p>The service name.</p>
    #[serde(rename = "ServiceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    /// <p>The unit of measurement.</p>
    #[serde(rename = "Unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// <p>Information about the measurement.</p>
    #[serde(rename = "UsageMetric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_metric: Option<MetricInfo>,
    /// <p>The quota value.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

/// <p>Information about a quota increase request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ServiceQuotaIncreaseRequestInTemplate {
    /// <p>The AWS Region.</p>
    #[serde(rename = "AwsRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
    /// <p>The new, increased value of the quota.</p>
    #[serde(rename = "DesiredValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_value: Option<f64>,
    /// <p>Indicates whether the quota is global.</p>
    #[serde(rename = "GlobalQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_quota: Option<bool>,
    /// <p>The quota identifier.</p>
    #[serde(rename = "QuotaCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_code: Option<String>,
    /// <p>The quota name.</p>
    #[serde(rename = "QuotaName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_name: Option<String>,
    /// <p>The service identifier.</p>
    #[serde(rename = "ServiceCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code: Option<String>,
    /// <p>The service name.</p>
    #[serde(rename = "ServiceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    /// <p>The unit of measurement.</p>
    #[serde(rename = "Unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

/// <p>A complex data type that contains a tag key and tag value.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tag {
    /// <p>A string that contains a tag key. The string length should be between 1 and 128 characters. Valid characters include a-z, A-Z, 0-9, space, and the special characters _ - . : / = + @.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>A string that contains an optional tag value. The string length should be between 0 and 256 characters. Valid characters include a-z, A-Z, 0-9, space, and the special characters _ - . : / = + @.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) for the applied quota. You can get this information by using the Service Quotas console, or by listing the quotas using the <a href="https://docs.aws.amazon.com/cli/latest/reference/service-quotas/list-service-quotas.html">list-service-quotas</a> AWS CLI command or the <a href="https://docs.aws.amazon.com/servicequotas/2019-06-24/apireference/API_ListServiceQuotas.html">ListServiceQuotas</a> AWS API operation.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    /// <p>The tags that you want to add to the resource.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) for the applied quota that you want to untag. You can get this information by using the Service Quotas console, or by listing the quotas using the <a href="https://docs.aws.amazon.com/cli/latest/reference/service-quotas/list-service-quotas.html">list-service-quotas</a> AWS CLI command or the <a href="https://docs.aws.amazon.com/servicequotas/2019-06-24/apireference/API_ListServiceQuotas.html">ListServiceQuotas</a> AWS API operation.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    /// <p>The keys of the tags that you want to remove from the resource.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

/// Errors returned by AssociateServiceQuotaTemplate
#[derive(Debug, PartialEq)]
pub enum AssociateServiceQuotaTemplateError {
    /// <p>The action you attempted is not allowed unless Service Access with Service Quotas is enabled in your organization.</p>
    AWSServiceAccessNotEnabled(String),
    /// <p>You do not have sufficient permission to perform this action.</p>
    AccessDenied(String),
    /// <p>You can't perform this action because a dependency does not have access.</p>
    DependencyAccessDenied(String),
    /// <p>The account making this call is not a member of an organization.</p>
    NoAvailableOrganization(String),
    /// <p>The organization that your account belongs to is not in All Features mode.</p>
    OrganizationNotInAllFeaturesMode(String),
    /// <p>Something went wrong.</p>
    Service(String),
    /// <p>The Service Quotas template is not available in this AWS Region.</p>
    TemplatesNotAvailableInRegion(String),
    /// <p>Due to throttling, the request was denied. Slow down the rate of request calls, or request an increase for this quota.</p>
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
    /// <p>The action you attempted is not allowed unless Service Access with Service Quotas is enabled in your organization.</p>
    AWSServiceAccessNotEnabled(String),
    /// <p>You do not have sufficient permission to perform this action.</p>
    AccessDenied(String),
    /// <p>You can't perform this action because a dependency does not have access.</p>
    DependencyAccessDenied(String),
    /// <p>Invalid input was provided.</p>
    IllegalArgument(String),
    /// <p>The account making this call is not a member of an organization.</p>
    NoAvailableOrganization(String),
    /// <p>The specified resource does not exist.</p>
    NoSuchResource(String),
    /// <p>Something went wrong.</p>
    Service(String),
    /// <p>The Service Quotas template is not available in this AWS Region.</p>
    TemplatesNotAvailableInRegion(String),
    /// <p>Due to throttling, the request was denied. Slow down the rate of request calls, or request an increase for this quota.</p>
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
    /// <p>The action you attempted is not allowed unless Service Access with Service Quotas is enabled in your organization.</p>
    AWSServiceAccessNotEnabled(String),
    /// <p>You do not have sufficient permission to perform this action.</p>
    AccessDenied(String),
    /// <p>You can't perform this action because a dependency does not have access.</p>
    DependencyAccessDenied(String),
    /// <p>The account making this call is not a member of an organization.</p>
    NoAvailableOrganization(String),
    /// <p>Something went wrong.</p>
    Service(String),
    /// <p>The quota request template is not associated with your organization.</p>
    ServiceQuotaTemplateNotInUse(String),
    /// <p>The Service Quotas template is not available in this AWS Region.</p>
    TemplatesNotAvailableInRegion(String),
    /// <p>Due to throttling, the request was denied. Slow down the rate of request calls, or request an increase for this quota.</p>
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
    /// <p>You do not have sufficient permission to perform this action.</p>
    AccessDenied(String),
    /// <p>Invalid input was provided.</p>
    IllegalArgument(String),
    /// <p>The specified resource does not exist.</p>
    NoSuchResource(String),
    /// <p>Something went wrong.</p>
    Service(String),
    /// <p>Due to throttling, the request was denied. Slow down the rate of request calls, or request an increase for this quota.</p>
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
    /// <p>The action you attempted is not allowed unless Service Access with Service Quotas is enabled in your organization.</p>
    AWSServiceAccessNotEnabled(String),
    /// <p>You do not have sufficient permission to perform this action.</p>
    AccessDenied(String),
    /// <p>You can't perform this action because a dependency does not have access.</p>
    DependencyAccessDenied(String),
    /// <p>The account making this call is not a member of an organization.</p>
    NoAvailableOrganization(String),
    /// <p>Something went wrong.</p>
    Service(String),
    /// <p>The quota request template is not associated with your organization.</p>
    ServiceQuotaTemplateNotInUse(String),
    /// <p>The Service Quotas template is not available in this AWS Region.</p>
    TemplatesNotAvailableInRegion(String),
    /// <p>Due to throttling, the request was denied. Slow down the rate of request calls, or request an increase for this quota.</p>
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
    /// <p>You do not have sufficient permission to perform this action.</p>
    AccessDenied(String),
    /// <p>Invalid input was provided.</p>
    IllegalArgument(String),
    /// <p>The specified resource does not exist.</p>
    NoSuchResource(String),
    /// <p>Something went wrong.</p>
    Service(String),
    /// <p>Due to throttling, the request was denied. Slow down the rate of request calls, or request an increase for this quota.</p>
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
    /// <p>You do not have sufficient permission to perform this action.</p>
    AccessDenied(String),
    /// <p>Invalid input was provided.</p>
    IllegalArgument(String),
    /// <p>The specified resource does not exist.</p>
    NoSuchResource(String),
    /// <p>Something went wrong.</p>
    Service(String),
    /// <p>Due to throttling, the request was denied. Slow down the rate of request calls, or request an increase for this quota.</p>
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
    /// <p>The action you attempted is not allowed unless Service Access with Service Quotas is enabled in your organization.</p>
    AWSServiceAccessNotEnabled(String),
    /// <p>You do not have sufficient permission to perform this action.</p>
    AccessDenied(String),
    /// <p>You can't perform this action because a dependency does not have access.</p>
    DependencyAccessDenied(String),
    /// <p>Invalid input was provided.</p>
    IllegalArgument(String),
    /// <p>The account making this call is not a member of an organization.</p>
    NoAvailableOrganization(String),
    /// <p>The specified resource does not exist.</p>
    NoSuchResource(String),
    /// <p>Something went wrong.</p>
    Service(String),
    /// <p>The Service Quotas template is not available in this AWS Region.</p>
    TemplatesNotAvailableInRegion(String),
    /// <p>Due to throttling, the request was denied. Slow down the rate of request calls, or request an increase for this quota.</p>
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
    /// <p>You do not have sufficient permission to perform this action.</p>
    AccessDenied(String),
    /// <p>Invalid input was provided.</p>
    IllegalArgument(String),
    /// <p>Invalid input was provided.</p>
    InvalidPaginationToken(String),
    /// <p>The specified resource does not exist.</p>
    NoSuchResource(String),
    /// <p>Something went wrong.</p>
    Service(String),
    /// <p>Due to throttling, the request was denied. Slow down the rate of request calls, or request an increase for this quota.</p>
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
    /// <p>You do not have sufficient permission to perform this action.</p>
    AccessDenied(String),
    /// <p>Invalid input was provided.</p>
    IllegalArgument(String),
    /// <p>Invalid input was provided.</p>
    InvalidPaginationToken(String),
    /// <p>The specified resource does not exist.</p>
    NoSuchResource(String),
    /// <p>Something went wrong.</p>
    Service(String),
    /// <p>Due to throttling, the request was denied. Slow down the rate of request calls, or request an increase for this quota.</p>
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
    /// <p>You do not have sufficient permission to perform this action.</p>
    AccessDenied(String),
    /// <p>Invalid input was provided.</p>
    IllegalArgument(String),
    /// <p>Invalid input was provided.</p>
    InvalidPaginationToken(String),
    /// <p>The specified resource does not exist.</p>
    NoSuchResource(String),
    /// <p>Something went wrong.</p>
    Service(String),
    /// <p>Due to throttling, the request was denied. Slow down the rate of request calls, or request an increase for this quota.</p>
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
    /// <p>The action you attempted is not allowed unless Service Access with Service Quotas is enabled in your organization.</p>
    AWSServiceAccessNotEnabled(String),
    /// <p>You do not have sufficient permission to perform this action.</p>
    AccessDenied(String),
    /// <p>You can't perform this action because a dependency does not have access.</p>
    DependencyAccessDenied(String),
    /// <p>Invalid input was provided.</p>
    IllegalArgument(String),
    /// <p>The account making this call is not a member of an organization.</p>
    NoAvailableOrganization(String),
    /// <p>Something went wrong.</p>
    Service(String),
    /// <p>The Service Quotas template is not available in this AWS Region.</p>
    TemplatesNotAvailableInRegion(String),
    /// <p>Due to throttling, the request was denied. Slow down the rate of request calls, or request an increase for this quota.</p>
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
    /// <p>You do not have sufficient permission to perform this action.</p>
    AccessDenied(String),
    /// <p>Invalid input was provided.</p>
    IllegalArgument(String),
    /// <p>Invalid input was provided.</p>
    InvalidPaginationToken(String),
    /// <p>The specified resource does not exist.</p>
    NoSuchResource(String),
    /// <p>Something went wrong.</p>
    Service(String),
    /// <p>Due to throttling, the request was denied. Slow down the rate of request calls, or request an increase for this quota.</p>
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
    /// <p>You do not have sufficient permission to perform this action.</p>
    AccessDenied(String),
    /// <p>Invalid input was provided.</p>
    IllegalArgument(String),
    /// <p>Invalid input was provided.</p>
    InvalidPaginationToken(String),
    /// <p>Something went wrong.</p>
    Service(String),
    /// <p>Due to throttling, the request was denied. Slow down the rate of request calls, or request an increase for this quota.</p>
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
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>You do not have sufficient permission to perform this action.</p>
    AccessDenied(String),
    /// <p>Invalid input was provided.</p>
    IllegalArgument(String),
    /// <p>The specified resource does not exist.</p>
    NoSuchResource(String),
    /// <p>Something went wrong.</p>
    Service(String),
    /// <p>Due to throttling, the request was denied. Slow down the rate of request calls, or request an increase for this quota.</p>
    TooManyRequests(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListTagsForResourceError::AccessDenied(err.msg))
                }
                "IllegalArgumentException" => {
                    return RusotoError::Service(ListTagsForResourceError::IllegalArgument(err.msg))
                }
                "NoSuchResourceException" => {
                    return RusotoError::Service(ListTagsForResourceError::NoSuchResource(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListTagsForResourceError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListTagsForResourceError::TooManyRequests(err.msg))
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
            ListTagsForResourceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::IllegalArgument(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::NoSuchResource(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::Service(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by PutServiceQuotaIncreaseRequestIntoTemplate
#[derive(Debug, PartialEq)]
pub enum PutServiceQuotaIncreaseRequestIntoTemplateError {
    /// <p>The action you attempted is not allowed unless Service Access with Service Quotas is enabled in your organization.</p>
    AWSServiceAccessNotEnabled(String),
    /// <p>You do not have sufficient permission to perform this action.</p>
    AccessDenied(String),
    /// <p>You can't perform this action because a dependency does not have access.</p>
    DependencyAccessDenied(String),
    /// <p>Invalid input was provided.</p>
    IllegalArgument(String),
    /// <p>The account making this call is not a member of an organization.</p>
    NoAvailableOrganization(String),
    /// <p>The specified resource does not exist.</p>
    NoSuchResource(String),
    /// <p>You have exceeded your service quota. To perform the requested action, remove some of the relevant resources, or use Service Quotas to request a service quota increase.</p>
    QuotaExceeded(String),
    /// <p>Something went wrong.</p>
    Service(String),
    /// <p>The Service Quotas template is not available in this AWS Region.</p>
    TemplatesNotAvailableInRegion(String),
    /// <p>Due to throttling, the request was denied. Slow down the rate of request calls, or request an increase for this quota.</p>
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
    /// <p>You do not have sufficient permission to perform this action.</p>
    AccessDenied(String),
    /// <p>You can't perform this action because a dependency does not have access.</p>
    DependencyAccessDenied(String),
    /// <p>Invalid input was provided.</p>
    IllegalArgument(String),
    /// <p>The resource is in an invalid state.</p>
    InvalidResourceState(String),
    /// <p>The specified resource does not exist.</p>
    NoSuchResource(String),
    /// <p>You have exceeded your service quota. To perform the requested action, remove some of the relevant resources, or use Service Quotas to request a service quota increase.</p>
    QuotaExceeded(String),
    /// <p>The specified resource already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>Something went wrong.</p>
    Service(String),
    /// <p>Due to throttling, the request was denied. Slow down the rate of request calls, or request an increase for this quota.</p>
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
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>You do not have sufficient permission to perform this action.</p>
    AccessDenied(String),
    /// <p>Invalid input was provided.</p>
    IllegalArgument(String),
    /// <p>The specified resource does not exist.</p>
    NoSuchResource(String),
    /// <p>Something went wrong.</p>
    Service(String),
    /// <p>The specified tag is a reserved word and cannot be used.</p>
    TagPolicyViolation(String),
    /// <p>Due to throttling, the request was denied. Slow down the rate of request calls, or request an increase for this quota.</p>
    TooManyRequests(String),
    /// <p>You've exceeded the number of tags allowed for a resource. For more information, see <a href="https://docs.aws.amazon.com/servicequotas/latest/userguide/sq-tagging.html#sq-tagging-restrictions">Tag restrictions</a> in the <i>Service Quotas User Guide</i>.</p>
    TooManyTags(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(TagResourceError::AccessDenied(err.msg))
                }
                "IllegalArgumentException" => {
                    return RusotoError::Service(TagResourceError::IllegalArgument(err.msg))
                }
                "NoSuchResourceException" => {
                    return RusotoError::Service(TagResourceError::NoSuchResource(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(TagResourceError::Service(err.msg))
                }
                "TagPolicyViolationException" => {
                    return RusotoError::Service(TagResourceError::TagPolicyViolation(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(TagResourceError::TooManyRequests(err.msg))
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
            TagResourceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            TagResourceError::IllegalArgument(ref cause) => write!(f, "{}", cause),
            TagResourceError::NoSuchResource(ref cause) => write!(f, "{}", cause),
            TagResourceError::Service(ref cause) => write!(f, "{}", cause),
            TagResourceError::TagPolicyViolation(ref cause) => write!(f, "{}", cause),
            TagResourceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            TagResourceError::TooManyTags(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>You do not have sufficient permission to perform this action.</p>
    AccessDenied(String),
    /// <p>Invalid input was provided.</p>
    IllegalArgument(String),
    /// <p>The specified resource does not exist.</p>
    NoSuchResource(String),
    /// <p>Something went wrong.</p>
    Service(String),
    /// <p>Due to throttling, the request was denied. Slow down the rate of request calls, or request an increase for this quota.</p>
    TooManyRequests(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UntagResourceError::AccessDenied(err.msg))
                }
                "IllegalArgumentException" => {
                    return RusotoError::Service(UntagResourceError::IllegalArgument(err.msg))
                }
                "NoSuchResourceException" => {
                    return RusotoError::Service(UntagResourceError::NoSuchResource(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(UntagResourceError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UntagResourceError::TooManyRequests(err.msg))
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
            UntagResourceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UntagResourceError::IllegalArgument(ref cause) => write!(f, "{}", cause),
            UntagResourceError::NoSuchResource(ref cause) => write!(f, "{}", cause),
            UntagResourceError::Service(ref cause) => write!(f, "{}", cause),
            UntagResourceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Trait representing the capabilities of the Service Quotas API. Service Quotas clients implement this trait.
#[async_trait]
pub trait ServiceQuotas {
    /// <p>Associates your quota request template with your organization. When a new account is created in your organization, the quota increase requests in the template are automatically applied to the account. You can add a quota increase request for any adjustable quota to your template.</p>
    async fn associate_service_quota_template(
        &self,
    ) -> Result<
        AssociateServiceQuotaTemplateResponse,
        RusotoError<AssociateServiceQuotaTemplateError>,
    >;

    /// <p>Deletes the quota increase request for the specified quota from your quota request template.</p>
    async fn delete_service_quota_increase_request_from_template(
        &self,
        input: DeleteServiceQuotaIncreaseRequestFromTemplateRequest,
    ) -> Result<
        DeleteServiceQuotaIncreaseRequestFromTemplateResponse,
        RusotoError<DeleteServiceQuotaIncreaseRequestFromTemplateError>,
    >;

    /// <p>Disables your quota request template. After a template is disabled, the quota increase requests in the template are not applied to new accounts in your organization. Disabling a quota request template does not apply its quota increase requests.</p>
    async fn disassociate_service_quota_template(
        &self,
    ) -> Result<
        DisassociateServiceQuotaTemplateResponse,
        RusotoError<DisassociateServiceQuotaTemplateError>,
    >;

    /// <p>Retrieves the default value for the specified quota. The default value does not reflect any quota increases.</p>
    async fn get_aws_default_service_quota(
        &self,
        input: GetAWSDefaultServiceQuotaRequest,
    ) -> Result<GetAWSDefaultServiceQuotaResponse, RusotoError<GetAWSDefaultServiceQuotaError>>;

    /// <p>Retrieves the status of the association for the quota request template.</p>
    async fn get_association_for_service_quota_template(
        &self,
    ) -> Result<
        GetAssociationForServiceQuotaTemplateResponse,
        RusotoError<GetAssociationForServiceQuotaTemplateError>,
    >;

    /// <p>Retrieves information about the specified quota increase request.</p>
    async fn get_requested_service_quota_change(
        &self,
        input: GetRequestedServiceQuotaChangeRequest,
    ) -> Result<
        GetRequestedServiceQuotaChangeResponse,
        RusotoError<GetRequestedServiceQuotaChangeError>,
    >;

    /// <p>Retrieves the applied quota value for the specified quota. For some quotas, only the default values are available. If the applied quota value is not available for a quota, the quota is not retrieved.</p>
    async fn get_service_quota(
        &self,
        input: GetServiceQuotaRequest,
    ) -> Result<GetServiceQuotaResponse, RusotoError<GetServiceQuotaError>>;

    /// <p>Retrieves information about the specified quota increase request in your quota request template.</p>
    async fn get_service_quota_increase_request_from_template(
        &self,
        input: GetServiceQuotaIncreaseRequestFromTemplateRequest,
    ) -> Result<
        GetServiceQuotaIncreaseRequestFromTemplateResponse,
        RusotoError<GetServiceQuotaIncreaseRequestFromTemplateError>,
    >;

    /// <p>Lists the default values for the quotas for the specified AWS service. A default value does not reflect any quota increases.</p>
    async fn list_aws_default_service_quotas(
        &self,
        input: ListAWSDefaultServiceQuotasRequest,
    ) -> Result<ListAWSDefaultServiceQuotasResponse, RusotoError<ListAWSDefaultServiceQuotasError>>;

    /// <p>Retrieves the quota increase requests for the specified service.</p>
    async fn list_requested_service_quota_change_history(
        &self,
        input: ListRequestedServiceQuotaChangeHistoryRequest,
    ) -> Result<
        ListRequestedServiceQuotaChangeHistoryResponse,
        RusotoError<ListRequestedServiceQuotaChangeHistoryError>,
    >;

    /// <p>Retrieves the quota increase requests for the specified quota.</p>
    async fn list_requested_service_quota_change_history_by_quota(
        &self,
        input: ListRequestedServiceQuotaChangeHistoryByQuotaRequest,
    ) -> Result<
        ListRequestedServiceQuotaChangeHistoryByQuotaResponse,
        RusotoError<ListRequestedServiceQuotaChangeHistoryByQuotaError>,
    >;

    /// <p>Lists the quota increase requests in the specified quota request template.</p>
    async fn list_service_quota_increase_requests_in_template(
        &self,
        input: ListServiceQuotaIncreaseRequestsInTemplateRequest,
    ) -> Result<
        ListServiceQuotaIncreaseRequestsInTemplateResponse,
        RusotoError<ListServiceQuotaIncreaseRequestsInTemplateError>,
    >;

    /// <p>Lists the applied quota values for the specified AWS service. For some quotas, only the default values are available. If the applied quota value is not available for a quota, the quota is not retrieved.</p>
    async fn list_service_quotas(
        &self,
        input: ListServiceQuotasRequest,
    ) -> Result<ListServiceQuotasResponse, RusotoError<ListServiceQuotasError>>;

    /// <p>Lists the names and codes for the services integrated with Service Quotas.</p>
    async fn list_services(
        &self,
        input: ListServicesRequest,
    ) -> Result<ListServicesResponse, RusotoError<ListServicesError>>;

    /// <p>Returns a list of the tags assigned to the specified applied quota.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Adds a quota increase request to your quota request template.</p>
    async fn put_service_quota_increase_request_into_template(
        &self,
        input: PutServiceQuotaIncreaseRequestIntoTemplateRequest,
    ) -> Result<
        PutServiceQuotaIncreaseRequestIntoTemplateResponse,
        RusotoError<PutServiceQuotaIncreaseRequestIntoTemplateError>,
    >;

    /// <p>Submits a quota increase request for the specified quota.</p>
    async fn request_service_quota_increase(
        &self,
        input: RequestServiceQuotaIncreaseRequest,
    ) -> Result<RequestServiceQuotaIncreaseResponse, RusotoError<RequestServiceQuotaIncreaseError>>;

    /// <p>Adds tags to the specified applied quota. You can include one or more tags to add to the quota.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Removes tags from the specified applied quota. You can specify one or more tags to remove.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;
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

#[async_trait]
impl ServiceQuotas for ServiceQuotasClient {
    /// <p>Associates your quota request template with your organization. When a new account is created in your organization, the quota increase requests in the template are automatically applied to the account. You can add a quota increase request for any adjustable quota to your template.</p>
    async fn associate_service_quota_template(
        &self,
    ) -> Result<
        AssociateServiceQuotaTemplateResponse,
        RusotoError<AssociateServiceQuotaTemplateError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ServiceQuotasV20190624.AssociateServiceQuotaTemplate",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let response = self
            .sign_and_dispatch(request, AssociateServiceQuotaTemplateError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<AssociateServiceQuotaTemplateResponse, _>()
    }

    /// <p>Deletes the quota increase request for the specified quota from your quota request template.</p>
    async fn delete_service_quota_increase_request_from_template(
        &self,
        input: DeleteServiceQuotaIncreaseRequestFromTemplateRequest,
    ) -> Result<
        DeleteServiceQuotaIncreaseRequestFromTemplateResponse,
        RusotoError<DeleteServiceQuotaIncreaseRequestFromTemplateError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ServiceQuotasV20190624.DeleteServiceQuotaIncreaseRequestFromTemplate",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                DeleteServiceQuotaIncreaseRequestFromTemplateError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteServiceQuotaIncreaseRequestFromTemplateResponse, _>()
    }

    /// <p>Disables your quota request template. After a template is disabled, the quota increase requests in the template are not applied to new accounts in your organization. Disabling a quota request template does not apply its quota increase requests.</p>
    async fn disassociate_service_quota_template(
        &self,
    ) -> Result<
        DisassociateServiceQuotaTemplateResponse,
        RusotoError<DisassociateServiceQuotaTemplateError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ServiceQuotasV20190624.DisassociateServiceQuotaTemplate",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let response = self
            .sign_and_dispatch(
                request,
                DisassociateServiceQuotaTemplateError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DisassociateServiceQuotaTemplateResponse, _>()
    }

    /// <p>Retrieves the default value for the specified quota. The default value does not reflect any quota increases.</p>
    async fn get_aws_default_service_quota(
        &self,
        input: GetAWSDefaultServiceQuotaRequest,
    ) -> Result<GetAWSDefaultServiceQuotaResponse, RusotoError<GetAWSDefaultServiceQuotaError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ServiceQuotasV20190624.GetAWSDefaultServiceQuota",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetAWSDefaultServiceQuotaError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetAWSDefaultServiceQuotaResponse, _>()
    }

    /// <p>Retrieves the status of the association for the quota request template.</p>
    async fn get_association_for_service_quota_template(
        &self,
    ) -> Result<
        GetAssociationForServiceQuotaTemplateResponse,
        RusotoError<GetAssociationForServiceQuotaTemplateError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ServiceQuotasV20190624.GetAssociationForServiceQuotaTemplate",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let response = self
            .sign_and_dispatch(
                request,
                GetAssociationForServiceQuotaTemplateError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetAssociationForServiceQuotaTemplateResponse, _>()
    }

    /// <p>Retrieves information about the specified quota increase request.</p>
    async fn get_requested_service_quota_change(
        &self,
        input: GetRequestedServiceQuotaChangeRequest,
    ) -> Result<
        GetRequestedServiceQuotaChangeResponse,
        RusotoError<GetRequestedServiceQuotaChangeError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ServiceQuotasV20190624.GetRequestedServiceQuotaChange",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetRequestedServiceQuotaChangeError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetRequestedServiceQuotaChangeResponse, _>()
    }

    /// <p>Retrieves the applied quota value for the specified quota. For some quotas, only the default values are available. If the applied quota value is not available for a quota, the quota is not retrieved.</p>
    async fn get_service_quota(
        &self,
        input: GetServiceQuotaRequest,
    ) -> Result<GetServiceQuotaResponse, RusotoError<GetServiceQuotaError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "ServiceQuotasV20190624.GetServiceQuota");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetServiceQuotaError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetServiceQuotaResponse, _>()
    }

    /// <p>Retrieves information about the specified quota increase request in your quota request template.</p>
    async fn get_service_quota_increase_request_from_template(
        &self,
        input: GetServiceQuotaIncreaseRequestFromTemplateRequest,
    ) -> Result<
        GetServiceQuotaIncreaseRequestFromTemplateResponse,
        RusotoError<GetServiceQuotaIncreaseRequestFromTemplateError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ServiceQuotasV20190624.GetServiceQuotaIncreaseRequestFromTemplate",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                GetServiceQuotaIncreaseRequestFromTemplateError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetServiceQuotaIncreaseRequestFromTemplateResponse, _>()
    }

    /// <p>Lists the default values for the quotas for the specified AWS service. A default value does not reflect any quota increases.</p>
    async fn list_aws_default_service_quotas(
        &self,
        input: ListAWSDefaultServiceQuotasRequest,
    ) -> Result<ListAWSDefaultServiceQuotasResponse, RusotoError<ListAWSDefaultServiceQuotasError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ServiceQuotasV20190624.ListAWSDefaultServiceQuotas",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListAWSDefaultServiceQuotasError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListAWSDefaultServiceQuotasResponse, _>()
    }

    /// <p>Retrieves the quota increase requests for the specified service.</p>
    async fn list_requested_service_quota_change_history(
        &self,
        input: ListRequestedServiceQuotaChangeHistoryRequest,
    ) -> Result<
        ListRequestedServiceQuotaChangeHistoryResponse,
        RusotoError<ListRequestedServiceQuotaChangeHistoryError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ServiceQuotasV20190624.ListRequestedServiceQuotaChangeHistory",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                ListRequestedServiceQuotaChangeHistoryError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListRequestedServiceQuotaChangeHistoryResponse, _>()
    }

    /// <p>Retrieves the quota increase requests for the specified quota.</p>
    async fn list_requested_service_quota_change_history_by_quota(
        &self,
        input: ListRequestedServiceQuotaChangeHistoryByQuotaRequest,
    ) -> Result<
        ListRequestedServiceQuotaChangeHistoryByQuotaResponse,
        RusotoError<ListRequestedServiceQuotaChangeHistoryByQuotaError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ServiceQuotasV20190624.ListRequestedServiceQuotaChangeHistoryByQuota",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                ListRequestedServiceQuotaChangeHistoryByQuotaError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListRequestedServiceQuotaChangeHistoryByQuotaResponse, _>()
    }

    /// <p>Lists the quota increase requests in the specified quota request template.</p>
    async fn list_service_quota_increase_requests_in_template(
        &self,
        input: ListServiceQuotaIncreaseRequestsInTemplateRequest,
    ) -> Result<
        ListServiceQuotaIncreaseRequestsInTemplateResponse,
        RusotoError<ListServiceQuotaIncreaseRequestsInTemplateError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ServiceQuotasV20190624.ListServiceQuotaIncreaseRequestsInTemplate",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                ListServiceQuotaIncreaseRequestsInTemplateError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListServiceQuotaIncreaseRequestsInTemplateResponse, _>()
    }

    /// <p>Lists the applied quota values for the specified AWS service. For some quotas, only the default values are available. If the applied quota value is not available for a quota, the quota is not retrieved.</p>
    async fn list_service_quotas(
        &self,
        input: ListServiceQuotasRequest,
    ) -> Result<ListServiceQuotasResponse, RusotoError<ListServiceQuotasError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "ServiceQuotasV20190624.ListServiceQuotas");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListServiceQuotasError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListServiceQuotasResponse, _>()
    }

    /// <p>Lists the names and codes for the services integrated with Service Quotas.</p>
    async fn list_services(
        &self,
        input: ListServicesRequest,
    ) -> Result<ListServicesResponse, RusotoError<ListServicesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "ServiceQuotasV20190624.ListServices");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListServicesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListServicesResponse, _>()
    }

    /// <p>Returns a list of the tags assigned to the specified applied quota.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "ServiceQuotasV20190624.ListTagsForResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListTagsForResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListTagsForResourceResponse, _>()
    }

    /// <p>Adds a quota increase request to your quota request template.</p>
    async fn put_service_quota_increase_request_into_template(
        &self,
        input: PutServiceQuotaIncreaseRequestIntoTemplateRequest,
    ) -> Result<
        PutServiceQuotaIncreaseRequestIntoTemplateResponse,
        RusotoError<PutServiceQuotaIncreaseRequestIntoTemplateError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ServiceQuotasV20190624.PutServiceQuotaIncreaseRequestIntoTemplate",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                PutServiceQuotaIncreaseRequestIntoTemplateError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<PutServiceQuotaIncreaseRequestIntoTemplateResponse, _>()
    }

    /// <p>Submits a quota increase request for the specified quota.</p>
    async fn request_service_quota_increase(
        &self,
        input: RequestServiceQuotaIncreaseRequest,
    ) -> Result<RequestServiceQuotaIncreaseResponse, RusotoError<RequestServiceQuotaIncreaseError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ServiceQuotasV20190624.RequestServiceQuotaIncrease",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, RequestServiceQuotaIncreaseError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<RequestServiceQuotaIncreaseResponse, _>()
    }

    /// <p>Adds tags to the specified applied quota. You can include one or more tags to add to the quota.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "ServiceQuotasV20190624.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, TagResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<TagResourceResponse, _>()
    }

    /// <p>Removes tags from the specified applied quota. You can specify one or more tags to remove.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "ServiceQuotasV20190624.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UntagResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UntagResourceResponse, _>()
    }
}
