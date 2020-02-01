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
/// <p>Container for the parameters to the GenerateDataSet operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GenerateDataSetRequest {
    /// <p>(Optional) Key-value pairs which will be returned, unmodified, in the Amazon SNS notification message and the data set metadata file. These key-value pairs can be used to correlated responses with tracking information from other systems.</p>
    #[serde(rename = "customerDefinedValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_defined_values: Option<::std::collections::HashMap<String, String>>,
    /// <p>The date a data set was published. For daily data sets, provide a date with day-level granularity for the desired day. For weekly data sets, provide a date with day-level granularity within the desired week (the day value will be ignored). For monthly data sets, provide a date with month-level granularity for the desired month (the day value will be ignored).</p>
    #[serde(rename = "dataSetPublicationDate")]
    pub data_set_publication_date: f64,
    /// <p>The desired data set type.</p> <p> <ul> <li> <strong>customer_subscriber_hourly_monthly_subscriptions</strong> <p>From 2017-09-15 to present: Available daily by 24:00 UTC.</p> </li> <li> <strong>customer_subscriber_annual_subscriptions</strong> <p>From 2017-09-15 to present: Available daily by 24:00 UTC.</p> </li> <li> <strong>daily_business_usage_by_instance_type</strong> <p>From 2017-09-15 to present: Available daily by 24:00 UTC.</p> </li> <li> <strong>daily_business_fees</strong> <p>From 2017-09-15 to present: Available daily by 24:00 UTC.</p> </li> <li> <strong>daily_business_free_trial_conversions</strong> <p>From 2017-09-15 to present: Available daily by 24:00 UTC.</p> </li> <li> <strong>daily_business_new_instances</strong> <p>From 2017-09-15 to present: Available daily by 24:00 UTC.</p> </li> <li> <strong>daily_business_new_product_subscribers</strong> <p>From 2017-09-15 to present: Available daily by 24:00 UTC.</p> </li> <li> <strong>daily_business_canceled_product_subscribers</strong> <p>From 2017-09-15 to present: Available daily by 24:00 UTC.</p> </li> <li> <strong>monthly_revenue_billing_and_revenue_data</strong> <p>From 2017-09-15 to present: Available monthly on the 15th day of the month by 24:00 UTC. Data includes metered transactions (e.g. hourly) from one month prior.</p> </li> <li> <strong>monthly_revenue_annual_subscriptions</strong> <p>From 2017-09-15 to present: Available monthly on the 15th day of the month by 24:00 UTC. Data includes up-front software charges (e.g. annual) from one month prior.</p> </li> <li> <strong>monthly_revenue_field_demonstration_usage</strong> <p>From 2018-03-15 to present: Available monthly on the 15th day of the month by 24:00 UTC.</p> </li> <li> <strong>monthly_revenue_flexible_payment_schedule</strong> <p>From 2018-11-15 to present: Available monthly on the 15th day of the month by 24:00 UTC.</p> </li> <li> <strong>disbursed_amount_by_product</strong> <p>From 2017-09-15 to present: Available every 30 days by 24:00 UTC.</p> </li> <li> <strong>disbursed_amount_by_instance_hours</strong> <p>From 2017-09-15 to present: Available every 30 days by 24:00 UTC.</p> </li> <li> <strong>disbursed_amount_by_customer_geo</strong> <p>From 2017-09-15 to present: Available every 30 days by 24:00 UTC.</p> </li> <li> <strong>disbursed_amount_by_age_of_uncollected_funds</strong> <p>From 2017-09-15 to present: Available every 30 days by 24:00 UTC.</p> </li> <li> <strong>disbursed_amount_by_age_of_disbursed_funds</strong> <p>From 2017-09-15 to present: Available every 30 days by 24:00 UTC.</p> </li> <li> <strong>disbursed_amount_by_age_of_past_due_funds</strong> <p>From 2018-04-07 to present: Available every 30 days by 24:00 UTC.</p> </li> <li> <strong>disbursed_amount_by_uncollected_funds_breakdown</strong> <p>From 2019-10-04 to present: Available every 30 days by 24:00 UTC.</p> </li> <li> <strong>sales_compensation_billed_revenue</strong> <p>From 2017-09-15 to present: Available monthly on the 15th day of the month by 24:00 UTC. Data includes metered transactions (e.g. hourly) from one month prior, and up-front software charges (e.g. annual) from one month prior.</p> </li> <li> <strong>us_sales_and_use_tax_records</strong> <p>From 2017-09-15 to present: Available monthly on the 15th day of the month by 24:00 UTC.</p> </li> </ul> </p>
    #[serde(rename = "dataSetType")]
    pub data_set_type: String,
    /// <p>The name (friendly name, not ARN) of the destination S3 bucket.</p>
    #[serde(rename = "destinationS3BucketName")]
    pub destination_s3_bucket_name: String,
    /// <p>(Optional) The desired S3 prefix for the published data set, similar to a directory path in standard file systems. For example, if given the bucket name &quot;mybucket&quot; and the prefix &quot;myprefix/mydatasets&quot;, the output file &quot;outputfile&quot; would be published to &quot;s3://mybucket/myprefix/mydatasets/outputfile&quot;. If the prefix directory structure does not exist, it will be created. If no prefix is provided, the data set will be published to the S3 bucket root.</p>
    #[serde(rename = "destinationS3Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_s3_prefix: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the Role with an attached permissions policy to interact with the provided AWS services.</p>
    #[serde(rename = "roleNameArn")]
    pub role_name_arn: String,
    /// <p>Amazon Resource Name (ARN) for the SNS Topic that will be notified when the data set has been published or if an error has occurred.</p>
    #[serde(rename = "snsTopicArn")]
    pub sns_topic_arn: String,
}

/// <p>Container for the result of the GenerateDataSet operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GenerateDataSetResult {
    /// <p>A unique identifier representing a specific request to the GenerateDataSet operation. This identifier can be used to correlate a request with notifications from the SNS topic.</p>
    #[serde(rename = "dataSetRequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_request_id: Option<String>,
}

/// <p>Container for the parameters to the StartSupportDataExport operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartSupportDataExportRequest {
    /// <p>(Optional) Key-value pairs which will be returned, unmodified, in the Amazon SNS notification message and the data set metadata file.</p>
    #[serde(rename = "customerDefinedValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_defined_values: Option<::std::collections::HashMap<String, String>>,
    /// <p> Specifies the data set type to be written to the output csv file. The data set types customer_support_contacts_data and test_customer_support_contacts_data both result in a csv file containing the following fields: Product Id, Product Code, Customer Guid, Subscription Guid, Subscription Start Date, Organization, AWS Account Id, Given Name, Surname, Telephone Number, Email, Title, Country Code, ZIP Code, Operation Type, and Operation Time. </p> <p> <ul> <li><i>customer_support_contacts_data</i> Customer support contact data. The data set will contain all changes (Creates, Updates, and Deletes) to customer support contact data from the date specified in the from_date parameter.</li> <li><i>test_customer_support_contacts_data</i> An example data set containing static test data in the same format as customer_support_contacts_data</li> </ul> </p>
    #[serde(rename = "dataSetType")]
    pub data_set_type: String,
    /// <p>The name (friendly name, not ARN) of the destination S3 bucket.</p>
    #[serde(rename = "destinationS3BucketName")]
    pub destination_s3_bucket_name: String,
    /// <p>(Optional) The desired S3 prefix for the published data set, similar to a directory path in standard file systems. For example, if given the bucket name &quot;mybucket&quot; and the prefix &quot;myprefix/mydatasets&quot;, the output file &quot;outputfile&quot; would be published to &quot;s3://mybucket/myprefix/mydatasets/outputfile&quot;. If the prefix directory structure does not exist, it will be created. If no prefix is provided, the data set will be published to the S3 bucket root.</p>
    #[serde(rename = "destinationS3Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_s3_prefix: Option<String>,
    /// <p>The start date from which to retrieve the data set in UTC. This parameter only affects the customer<em>support</em>contacts_data data set type.</p>
    #[serde(rename = "fromDate")]
    pub from_date: f64,
    /// <p>The Amazon Resource Name (ARN) of the Role with an attached permissions policy to interact with the provided AWS services.</p>
    #[serde(rename = "roleNameArn")]
    pub role_name_arn: String,
    /// <p>Amazon Resource Name (ARN) for the SNS Topic that will be notified when the data set has been published or if an error has occurred.</p>
    #[serde(rename = "snsTopicArn")]
    pub sns_topic_arn: String,
}

/// <p>Container for the result of the StartSupportDataExport operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartSupportDataExportResult {
    /// <p>A unique identifier representing a specific request to the StartSupportDataExport operation. This identifier can be used to correlate a request with notifications from the SNS topic.</p>
    #[serde(rename = "dataSetRequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_request_id: Option<String>,
}

/// Errors returned by GenerateDataSet
#[derive(Debug, PartialEq)]
pub enum GenerateDataSetError {
    /// <p>This exception is thrown when an internal service error occurs.</p>
    MarketplaceCommerceAnalytics(String),
}

impl GenerateDataSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GenerateDataSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "MarketplaceCommerceAnalyticsException" => {
                    return RusotoError::Service(
                        GenerateDataSetError::MarketplaceCommerceAnalytics(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GenerateDataSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GenerateDataSetError::MarketplaceCommerceAnalytics(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GenerateDataSetError {}
/// Errors returned by StartSupportDataExport
#[derive(Debug, PartialEq)]
pub enum StartSupportDataExportError {
    /// <p>This exception is thrown when an internal service error occurs.</p>
    MarketplaceCommerceAnalytics(String),
}

impl StartSupportDataExportError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartSupportDataExportError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "MarketplaceCommerceAnalyticsException" => {
                    return RusotoError::Service(
                        StartSupportDataExportError::MarketplaceCommerceAnalytics(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartSupportDataExportError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartSupportDataExportError::MarketplaceCommerceAnalytics(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for StartSupportDataExportError {}
/// Trait representing the capabilities of the AWS Marketplace Commerce Analytics API. AWS Marketplace Commerce Analytics clients implement this trait.
#[async_trait]
pub trait MarketplaceCommerceAnalytics {
    /// <p>Given a data set type and data set publication date, asynchronously publishes the requested data set to the specified S3 bucket and notifies the specified SNS topic once the data is available. Returns a unique request identifier that can be used to correlate requests with notifications from the SNS topic. Data sets will be published in comma-separated values (CSV) format with the file name {data<em>set</em>type}_YYYY-MM-DD.csv. If a file with the same name already exists (e.g. if the same data set is requested twice), the original file will be overwritten by the new file. Requires a Role with an attached permissions policy providing Allow permissions for the following actions: s3:PutObject, s3:GetBucketLocation, sns:GetTopicAttributes, sns:Publish, iam:GetRolePolicy.</p>
    async fn generate_data_set(
        &self,
        input: GenerateDataSetRequest,
    ) -> Result<GenerateDataSetResult, RusotoError<GenerateDataSetError>>;

    /// <p>Given a data set type and a from date, asynchronously publishes the requested customer support data to the specified S3 bucket and notifies the specified SNS topic once the data is available. Returns a unique request identifier that can be used to correlate requests with notifications from the SNS topic. Data sets will be published in comma-separated values (CSV) format with the file name {data<em>set</em>type}_YYYY-MM-DD&#39;T&#39;HH-mm-ss&#39;Z&#39;.csv. If a file with the same name already exists (e.g. if the same data set is requested twice), the original file will be overwritten by the new file. Requires a Role with an attached permissions policy providing Allow permissions for the following actions: s3:PutObject, s3:GetBucketLocation, sns:GetTopicAttributes, sns:Publish, iam:GetRolePolicy.</p>
    async fn start_support_data_export(
        &self,
        input: StartSupportDataExportRequest,
    ) -> Result<StartSupportDataExportResult, RusotoError<StartSupportDataExportError>>;
}
/// A client for the AWS Marketplace Commerce Analytics API.
#[derive(Clone)]
pub struct MarketplaceCommerceAnalyticsClient {
    client: Client,
    region: region::Region,
}

impl MarketplaceCommerceAnalyticsClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> MarketplaceCommerceAnalyticsClient {
        MarketplaceCommerceAnalyticsClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> MarketplaceCommerceAnalyticsClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        MarketplaceCommerceAnalyticsClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(
        client: Client,
        region: region::Region,
    ) -> MarketplaceCommerceAnalyticsClient {
        MarketplaceCommerceAnalyticsClient { client, region }
    }
}

#[async_trait]
impl MarketplaceCommerceAnalytics for MarketplaceCommerceAnalyticsClient {
    /// <p>Given a data set type and data set publication date, asynchronously publishes the requested data set to the specified S3 bucket and notifies the specified SNS topic once the data is available. Returns a unique request identifier that can be used to correlate requests with notifications from the SNS topic. Data sets will be published in comma-separated values (CSV) format with the file name {data<em>set</em>type}_YYYY-MM-DD.csv. If a file with the same name already exists (e.g. if the same data set is requested twice), the original file will be overwritten by the new file. Requires a Role with an attached permissions policy providing Allow permissions for the following actions: s3:PutObject, s3:GetBucketLocation, sns:GetTopicAttributes, sns:Publish, iam:GetRolePolicy.</p>
    async fn generate_data_set(
        &self,
        input: GenerateDataSetRequest,
    ) -> Result<GenerateDataSetResult, RusotoError<GenerateDataSetError>> {
        let mut request =
            SignedRequest::new("POST", "marketplacecommerceanalytics", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MarketplaceCommerceAnalytics20150701.GenerateDataSet",
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
            proto::json::ResponsePayload::new(&response).deserialize::<GenerateDataSetResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GenerateDataSetError::from_response(response))
        }
    }

    /// <p>Given a data set type and a from date, asynchronously publishes the requested customer support data to the specified S3 bucket and notifies the specified SNS topic once the data is available. Returns a unique request identifier that can be used to correlate requests with notifications from the SNS topic. Data sets will be published in comma-separated values (CSV) format with the file name {data<em>set</em>type}_YYYY-MM-DD&#39;T&#39;HH-mm-ss&#39;Z&#39;.csv. If a file with the same name already exists (e.g. if the same data set is requested twice), the original file will be overwritten by the new file. Requires a Role with an attached permissions policy providing Allow permissions for the following actions: s3:PutObject, s3:GetBucketLocation, sns:GetTopicAttributes, sns:Publish, iam:GetRolePolicy.</p>
    async fn start_support_data_export(
        &self,
        input: StartSupportDataExportRequest,
    ) -> Result<StartSupportDataExportResult, RusotoError<StartSupportDataExportError>> {
        let mut request =
            SignedRequest::new("POST", "marketplacecommerceanalytics", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "MarketplaceCommerceAnalytics20150701.StartSupportDataExport",
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
                .deserialize::<StartSupportDataExportResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StartSupportDataExportError::from_response(response))
        }
    }
}
