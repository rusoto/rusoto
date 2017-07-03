
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

use std::fmt;
use std::error::Error;

use rusoto_core::region;
use rusoto_core::request::{DispatchSignedRequest, HttpDispatchError};
use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};

use serde_json;
use rusoto_core::signature::SignedRequest;
use serde_json::Value as SerdeJsonValue;
use serde_json::from_str;

#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum DataSetType {
    CustomerProfileByGeography,
    CustomerProfileByIndustry,
    CustomerProfileByRevenue,
    CustomerSubscriberAnnualSubscriptions,
    CustomerSubscriberHourlyMonthlySubscriptions,
    DailyBusinessCanceledProductSubscribers,
    DailyBusinessFees,
    DailyBusinessFreeTrialConversions,
    DailyBusinessNewInstances,
    DailyBusinessNewProductSubscribers,
    DailyBusinessUsageByInstanceType,
    DisbursedAmountByAgeOfDisbursedFunds,
    DisbursedAmountByAgeOfUncollectedFunds,
    DisbursedAmountByCustomerGeo,
    DisbursedAmountByInstanceHours,
    DisbursedAmountByProduct,
    DisbursedAmountByProductWithUncollectedFunds,
    MonthlyRevenueAnnualSubscriptions,
    MonthlyRevenueBillingAndRevenueData,
    SalesCompensationBilledRevenue,
    UsSalesAndUseTaxRecords,
}

impl Into<String> for DataSetType {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for DataSetType {
    fn into(self) -> &'static str {
        match self {
            DataSetType::CustomerProfileByGeography => "customer_profile_by_geography",
            DataSetType::CustomerProfileByIndustry => "customer_profile_by_industry",
            DataSetType::CustomerProfileByRevenue => "customer_profile_by_revenue",
            DataSetType::CustomerSubscriberAnnualSubscriptions => {
                "customer_subscriber_annual_subscriptions"
            }
            DataSetType::CustomerSubscriberHourlyMonthlySubscriptions => {
                "customer_subscriber_hourly_monthly_subscriptions"
            }
            DataSetType::DailyBusinessCanceledProductSubscribers => {
                "daily_business_canceled_product_subscribers"
            }
            DataSetType::DailyBusinessFees => "daily_business_fees",
            DataSetType::DailyBusinessFreeTrialConversions => {
                "daily_business_free_trial_conversions"
            }
            DataSetType::DailyBusinessNewInstances => "daily_business_new_instances",
            DataSetType::DailyBusinessNewProductSubscribers => {
                "daily_business_new_product_subscribers"
            }
            DataSetType::DailyBusinessUsageByInstanceType => {
                "daily_business_usage_by_instance_type"
            }
            DataSetType::DisbursedAmountByAgeOfDisbursedFunds => {
                "disbursed_amount_by_age_of_disbursed_funds"
            }
            DataSetType::DisbursedAmountByAgeOfUncollectedFunds => {
                "disbursed_amount_by_age_of_uncollected_funds"
            }
            DataSetType::DisbursedAmountByCustomerGeo => "disbursed_amount_by_customer_geo",
            DataSetType::DisbursedAmountByInstanceHours => "disbursed_amount_by_instance_hours",
            DataSetType::DisbursedAmountByProduct => "disbursed_amount_by_product",
            DataSetType::DisbursedAmountByProductWithUncollectedFunds => {
                "disbursed_amount_by_product_with_uncollected_funds"
            }
            DataSetType::MonthlyRevenueAnnualSubscriptions => {
                "monthly_revenue_annual_subscriptions"
            }
            DataSetType::MonthlyRevenueBillingAndRevenueData => {
                "monthly_revenue_billing_and_revenue_data"
            }
            DataSetType::SalesCompensationBilledRevenue => "sales_compensation_billed_revenue",
            DataSetType::UsSalesAndUseTaxRecords => "us_sales_and_use_tax_records",
        }
    }
}

impl ::std::str::FromStr for DataSetType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "customer_profile_by_geography" => Ok(DataSetType::CustomerProfileByGeography),
            "customer_profile_by_industry" => Ok(DataSetType::CustomerProfileByIndustry),
            "customer_profile_by_revenue" => Ok(DataSetType::CustomerProfileByRevenue),
            "customer_subscriber_annual_subscriptions" => {
                Ok(DataSetType::CustomerSubscriberAnnualSubscriptions)
            }
            "customer_subscriber_hourly_monthly_subscriptions" => {
                Ok(DataSetType::CustomerSubscriberHourlyMonthlySubscriptions)
            }
            "daily_business_canceled_product_subscribers" => {
                Ok(DataSetType::DailyBusinessCanceledProductSubscribers)
            }
            "daily_business_fees" => Ok(DataSetType::DailyBusinessFees),
            "daily_business_free_trial_conversions" => {
                Ok(DataSetType::DailyBusinessFreeTrialConversions)
            }
            "daily_business_new_instances" => Ok(DataSetType::DailyBusinessNewInstances),
            "daily_business_new_product_subscribers" => {
                Ok(DataSetType::DailyBusinessNewProductSubscribers)
            }
            "daily_business_usage_by_instance_type" => {
                Ok(DataSetType::DailyBusinessUsageByInstanceType)
            }
            "disbursed_amount_by_age_of_disbursed_funds" => {
                Ok(DataSetType::DisbursedAmountByAgeOfDisbursedFunds)
            }
            "disbursed_amount_by_age_of_uncollected_funds" => {
                Ok(DataSetType::DisbursedAmountByAgeOfUncollectedFunds)
            }
            "disbursed_amount_by_customer_geo" => Ok(DataSetType::DisbursedAmountByCustomerGeo),
            "disbursed_amount_by_instance_hours" => Ok(DataSetType::DisbursedAmountByInstanceHours),
            "disbursed_amount_by_product" => Ok(DataSetType::DisbursedAmountByProduct),
            "disbursed_amount_by_product_with_uncollected_funds" => {
                Ok(DataSetType::DisbursedAmountByProductWithUncollectedFunds)
            }
            "monthly_revenue_annual_subscriptions" => {
                Ok(DataSetType::MonthlyRevenueAnnualSubscriptions)
            }
            "monthly_revenue_billing_and_revenue_data" => {
                Ok(DataSetType::MonthlyRevenueBillingAndRevenueData)
            }
            "sales_compensation_billed_revenue" => Ok(DataSetType::SalesCompensationBilledRevenue),
            "us_sales_and_use_tax_records" => Ok(DataSetType::UsSalesAndUseTaxRecords),
            _ => Err(()),
        }
    }
}

#[doc="Container for the parameters to the GenerateDataSet operation."]
#[derive(Default,Debug,Clone,Serialize)]
pub struct GenerateDataSetRequest {
    #[doc="(Optional) Key-value pairs which will be returned, unmodified, in the Amazon SNS notification message and the data set metadata file. These key-value pairs can be used to correlated responses with tracking information from other systems."]
    #[serde(rename="customerDefinedValues")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub customer_defined_values: Option<::std::collections::HashMap<String, String>>,
    #[doc="The date a data set was published. For daily data sets, provide a date with day-level granularity for the desired day. For weekly data sets, provide a date with day-level granularity within the desired week (the day value will be ignored). For monthly data sets, provide a date with month-level granularity for the desired month (the day value will be ignored)."]
    #[serde(rename="dataSetPublicationDate")]
    pub data_set_publication_date: f64,
    #[doc="<p>The desired data set type.</p> <p> <ul> <li><i>customer_subscriber_hourly_monthly_subscriptions</i> - Available daily by 5:00 PM Pacific Time since 2014-07-21.</li> <li><i>customer_subscriber_annual_subscriptions</i> - Available daily by 5:00 PM Pacific Time since 2014-07-21.</li> <li><i>daily_business_usage_by_instance_type</i> - Available daily by 5:00 PM Pacific Time since 2015-01-26.</li> <li><i>daily_business_fees</i> - Available daily by 5:00 PM Pacific Time since 2015-01-26.</li> <li><i>daily_business_free_trial_conversions</i> - Available daily by 5:00 PM Pacific Time since 2015-01-26.</li> <li><i>daily_business_new_instances</i> - Available daily by 5:00 PM Pacific Time since 2015-01-26.</li> <li><i>daily_business_new_product_subscribers</i> - Available daily by 5:00 PM Pacific Time since 2015-01-26.</li> <li><i>daily_business_canceled_product_subscribers</i> - Available daily by 5:00 PM Pacific Time since 2015-01-26.</li> <li><i>monthly_revenue_billing_and_revenue_data</i> - Available monthly on the 4th day of the month by 5:00 PM Pacific Time since 2015-02.</li> <li><i>monthly_revenue_annual_subscriptions</i> - Available monthly on the 4th day of the month by 5:00 PM Pacific Time since 2015-02.</li> <li><i>disbursed_amount_by_product</i> - Available every 30 days by 5:00 PM Pacific Time since 2015-01-26.</li> <li><i>disbursed_amount_by_product_with_uncollected_funds</i> -This data set is only available from 2012-04-19 until 2015-01-25. After 2015-01-25, this data set was split into three data sets: disbursed_amount_by_product, disbursed_amount_by_age_of_uncollected_funds, and disbursed_amount_by_age_of_disbursed_funds. </li> <li><i>disbursed_amount_by_instance_hours</i> - Available every 30 days by 5:00 PM Pacific Time since 2012-09-04.</li> <li><i>disbursed_amount_by_customer_geo</i> - Available every 30 days by 5:00 PM Pacific Time since 2012-04-19.</li> <li><i>disbursed_amount_by_age_of_uncollected_funds</i> - Available every 30 days by 5:00 PM Pacific Time since 2015-01-26.</li> <li><i>disbursed_amount_by_age_of_disbursed_funds</i> - Available every 30 days by 5:00 PM Pacific Time since 2015-01-26.</li> <li><i>customer_profile_by_industry</i> - Available daily by 5:00 PM Pacific Time since 2015-10-01.</li> <li><i>customer_profile_by_revenue</i> - Available daily by 5:00 PM Pacific Time since 2015-10-01.</li> <li><i>customer_profile_by_geography</i> - Available daily by 5:00 PM Pacific Time since 2015-10-01.</li> <li><i>sales_compensation_billed_revenue</i> - Available monthly on the 4th day of the month by 5:00 PM Pacific Time since 2016-12.</li> <li><i>us_sales_and_use_tax_records</i> - Available monthly on the 15th day of the month by 5:00 PM Pacific Time since 2017-02-15.</li> </ul> </p>"]
    #[serde(rename="dataSetType")]
    pub data_set_type: String,
    #[doc="The name (friendly name, not ARN) of the destination S3 bucket."]
    #[serde(rename="destinationS3BucketName")]
    pub destination_s3_bucket_name: String,
    #[doc="(Optional) The desired S3 prefix for the published data set, similar to a directory path in standard file systems. For example, if given the bucket name \"mybucket\" and the prefix \"myprefix/mydatasets\", the output file \"outputfile\" would be published to \"s3://mybucket/myprefix/mydatasets/outputfile\". If the prefix directory structure does not exist, it will be created. If no prefix is provided, the data set will be published to the S3 bucket root."]
    #[serde(rename="destinationS3Prefix")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub destination_s3_prefix: Option<String>,
    #[doc="The Amazon Resource Name (ARN) of the Role with an attached permissions policy to interact with the provided AWS services."]
    #[serde(rename="roleNameArn")]
    pub role_name_arn: String,
    #[doc="Amazon Resource Name (ARN) for the SNS Topic that will be notified when the data set has been published or if an error has occurred."]
    #[serde(rename="snsTopicArn")]
    pub sns_topic_arn: String,
}

#[doc="Container for the result of the GenerateDataSet operation."]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GenerateDataSetResult {
    #[doc="A unique identifier representing a specific request to the GenerateDataSet operation. This identifier can be used to correlate a request with notifications from the SNS topic."]
    #[serde(rename="dataSetRequestId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub data_set_request_id: Option<String>,
}

#[doc="Container for the parameters to the StartSupportDataExport operation."]
#[derive(Default,Debug,Clone,Serialize)]
pub struct StartSupportDataExportRequest {
    #[doc="(Optional) Key-value pairs which will be returned, unmodified, in the Amazon SNS notification message and the data set metadata file."]
    #[serde(rename="customerDefinedValues")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub customer_defined_values: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p> Specifies the data set type to be written to the output csv file. The data set types customer_support_contacts_data and test_customer_support_contacts_data both result in a csv file containing the following fields: Product Id, Product Code, Customer Guid, Subscription Guid, Subscription Start Date, Organization, AWS Account Id, Given Name, Surname, Telephone Number, Email, Title, Country Code, ZIP Code, Operation Type, and Operation Time. </p> <p> <ul> <li><i>customer_support_contacts_data</i> Customer support contact data. The data set will contain all changes (Creates, Updates, and Deletes) to customer support contact data from the date specified in the from_date parameter.</li> <li><i>test_customer_support_contacts_data</i> An example data set containing static test data in the same format as customer_support_contacts_data</li> </ul> </p>"]
    #[serde(rename="dataSetType")]
    pub data_set_type: String,
    #[doc="The name (friendly name, not ARN) of the destination S3 bucket."]
    #[serde(rename="destinationS3BucketName")]
    pub destination_s3_bucket_name: String,
    #[doc="(Optional) The desired S3 prefix for the published data set, similar to a directory path in standard file systems. For example, if given the bucket name \"mybucket\" and the prefix \"myprefix/mydatasets\", the output file \"outputfile\" would be published to \"s3://mybucket/myprefix/mydatasets/outputfile\". If the prefix directory structure does not exist, it will be created. If no prefix is provided, the data set will be published to the S3 bucket root."]
    #[serde(rename="destinationS3Prefix")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub destination_s3_prefix: Option<String>,
    #[doc="The start date from which to retrieve the data set in UTC. This parameter only affects the customer_support_contacts_data data set type."]
    #[serde(rename="fromDate")]
    pub from_date: f64,
    #[doc="The Amazon Resource Name (ARN) of the Role with an attached permissions policy to interact with the provided AWS services."]
    #[serde(rename="roleNameArn")]
    pub role_name_arn: String,
    #[doc="Amazon Resource Name (ARN) for the SNS Topic that will be notified when the data set has been published or if an error has occurred."]
    #[serde(rename="snsTopicArn")]
    pub sns_topic_arn: String,
}

#[doc="Container for the result of the StartSupportDataExport operation."]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct StartSupportDataExportResult {
    #[doc="A unique identifier representing a specific request to the StartSupportDataExport operation. This identifier can be used to correlate a request with notifications from the SNS topic."]
    #[serde(rename="dataSetRequestId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub data_set_request_id: Option<String>,
}


#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum SupportDataSetType {
    CustomerSupportContactsData,
    TestCustomerSupportContactsData,
}

impl Into<String> for SupportDataSetType {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for SupportDataSetType {
    fn into(self) -> &'static str {
        match self {
            SupportDataSetType::CustomerSupportContactsData => "customer_support_contacts_data",
            SupportDataSetType::TestCustomerSupportContactsData => {
                "test_customer_support_contacts_data"
            }
        }
    }
}

impl ::std::str::FromStr for SupportDataSetType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "customer_support_contacts_data" => Ok(SupportDataSetType::CustomerSupportContactsData),
            "test_customer_support_contacts_data" => {
                Ok(SupportDataSetType::TestCustomerSupportContactsData)
            }
            _ => Err(()),
        }
    }
}

/// Errors returned by GenerateDataSet
#[derive(Debug, PartialEq)]
pub enum GenerateDataSetError {
    ///This exception is thrown when an internal service error occurs.
    MarketplaceCommerceAnalytics(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GenerateDataSetError {
    pub fn from_body(body: &str) -> GenerateDataSetError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "MarketplaceCommerceAnalyticsException" => GenerateDataSetError::MarketplaceCommerceAnalytics(String::from(error_message)),
                    "ValidationException" => {
                        GenerateDataSetError::Validation(error_message.to_string())
                    }
                    _ => GenerateDataSetError::Unknown(String::from(body)),
                }
            }
            Err(_) => GenerateDataSetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GenerateDataSetError {
    fn from(err: serde_json::error::Error) -> GenerateDataSetError {
        GenerateDataSetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GenerateDataSetError {
    fn from(err: CredentialsError) -> GenerateDataSetError {
        GenerateDataSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GenerateDataSetError {
    fn from(err: HttpDispatchError) -> GenerateDataSetError {
        GenerateDataSetError::HttpDispatch(err)
    }
}
impl fmt::Display for GenerateDataSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GenerateDataSetError {
    fn description(&self) -> &str {
        match *self {
            GenerateDataSetError::MarketplaceCommerceAnalytics(ref cause) => cause,
            GenerateDataSetError::Validation(ref cause) => cause,
            GenerateDataSetError::Credentials(ref err) => err.description(),
            GenerateDataSetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GenerateDataSetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StartSupportDataExport
#[derive(Debug, PartialEq)]
pub enum StartSupportDataExportError {
    ///This exception is thrown when an internal service error occurs.
    MarketplaceCommerceAnalytics(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl StartSupportDataExportError {
    pub fn from_body(body: &str) -> StartSupportDataExportError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "MarketplaceCommerceAnalyticsException" => StartSupportDataExportError::MarketplaceCommerceAnalytics(String::from(error_message)),
                    "ValidationException" => {
                        StartSupportDataExportError::Validation(error_message.to_string())
                    }
                    _ => StartSupportDataExportError::Unknown(String::from(body)),
                }
            }
            Err(_) => StartSupportDataExportError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StartSupportDataExportError {
    fn from(err: serde_json::error::Error) -> StartSupportDataExportError {
        StartSupportDataExportError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StartSupportDataExportError {
    fn from(err: CredentialsError) -> StartSupportDataExportError {
        StartSupportDataExportError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartSupportDataExportError {
    fn from(err: HttpDispatchError) -> StartSupportDataExportError {
        StartSupportDataExportError::HttpDispatch(err)
    }
}
impl fmt::Display for StartSupportDataExportError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartSupportDataExportError {
    fn description(&self) -> &str {
        match *self {
            StartSupportDataExportError::MarketplaceCommerceAnalytics(ref cause) => cause,
            StartSupportDataExportError::Validation(ref cause) => cause,
            StartSupportDataExportError::Credentials(ref err) => err.description(),
            StartSupportDataExportError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StartSupportDataExportError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS Marketplace Commerce Analytics API. AWS Marketplace Commerce Analytics clients implement this trait.
pub trait MarketplaceCommerceAnalytics {
    #[doc="Given a data set type and data set publication date, asynchronously publishes the requested data set to the specified S3 bucket and notifies the specified SNS topic once the data is available. Returns a unique request identifier that can be used to correlate requests with notifications from the SNS topic. Data sets will be published in comma-separated values (CSV) format with the file name {data_set_type}_YYYY-MM-DD.csv. If a file with the same name already exists (e.g. if the same data set is requested twice), the original file will be overwritten by the new file. Requires a Role with an attached permissions policy providing Allow permissions for the following actions: s3:PutObject, s3:GetBucketLocation, sns:GetTopicAttributes, sns:Publish, iam:GetRolePolicy."]
    fn generate_data_set(&self,
                         input: &GenerateDataSetRequest)
                         -> Result<GenerateDataSetResult, GenerateDataSetError>;


    #[doc="Given a data set type and a from date, asynchronously publishes the requested customer support data to the specified S3 bucket and notifies the specified SNS topic once the data is available. Returns a unique request identifier that can be used to correlate requests with notifications from the SNS topic. Data sets will be published in comma-separated values (CSV) format with the file name {data_set_type}_YYYY-MM-DD'T'HH-mm-ss'Z'.csv. If a file with the same name already exists (e.g. if the same data set is requested twice), the original file will be overwritten by the new file. Requires a Role with an attached permissions policy providing Allow permissions for the following actions: s3:PutObject, s3:GetBucketLocation, sns:GetTopicAttributes, sns:Publish, iam:GetRolePolicy."]
    fn start_support_data_export
        (&self,
         input: &StartSupportDataExportRequest)
         -> Result<StartSupportDataExportResult, StartSupportDataExportError>;
}
/// A client for the AWS Marketplace Commerce Analytics API.
pub struct MarketplaceCommerceAnalyticsClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> MarketplaceCommerceAnalyticsClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        MarketplaceCommerceAnalyticsClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> MarketplaceCommerceAnalytics for MarketplaceCommerceAnalyticsClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    #[doc="Given a data set type and data set publication date, asynchronously publishes the requested data set to the specified S3 bucket and notifies the specified SNS topic once the data is available. Returns a unique request identifier that can be used to correlate requests with notifications from the SNS topic. Data sets will be published in comma-separated values (CSV) format with the file name {data_set_type}_YYYY-MM-DD.csv. If a file with the same name already exists (e.g. if the same data set is requested twice), the original file will be overwritten by the new file. Requires a Role with an attached permissions policy providing Allow permissions for the following actions: s3:PutObject, s3:GetBucketLocation, sns:GetTopicAttributes, sns:Publish, iam:GetRolePolicy."]
    fn generate_data_set(&self,
                         input: &GenerateDataSetRequest)
                         -> Result<GenerateDataSetResult, GenerateDataSetError> {
        let mut request =
            SignedRequest::new("POST", "marketplacecommerceanalytics", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "MarketplaceCommerceAnalytics20150701.GenerateDataSet");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<GenerateDataSetResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(GenerateDataSetError::from_body(String::from_utf8_lossy(&response.body)
                                                        .as_ref()))
            }
        }
    }


    #[doc="Given a data set type and a from date, asynchronously publishes the requested customer support data to the specified S3 bucket and notifies the specified SNS topic once the data is available. Returns a unique request identifier that can be used to correlate requests with notifications from the SNS topic. Data sets will be published in comma-separated values (CSV) format with the file name {data_set_type}_YYYY-MM-DD'T'HH-mm-ss'Z'.csv. If a file with the same name already exists (e.g. if the same data set is requested twice), the original file will be overwritten by the new file. Requires a Role with an attached permissions policy providing Allow permissions for the following actions: s3:PutObject, s3:GetBucketLocation, sns:GetTopicAttributes, sns:Publish, iam:GetRolePolicy."]
    fn start_support_data_export
        (&self,
         input: &StartSupportDataExportRequest)
         -> Result<StartSupportDataExportResult, StartSupportDataExportError> {
        let mut request =
            SignedRequest::new("POST", "marketplacecommerceanalytics", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "MarketplaceCommerceAnalytics20150701.StartSupportDataExport");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<StartSupportDataExportResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(StartSupportDataExportError::from_body(String::from_utf8_lossy(&response.body)
                                                               .as_ref()))
            }
        }
    }
}

#[cfg(test)]
mod protocol_tests {}
