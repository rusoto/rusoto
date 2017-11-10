
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

#[allow(warnings)]
use futures::future;
#[allow(unused_imports)]
use futures::{Future, Poll, Stream as FuturesStream};
use hyper::StatusCode;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::region;
use rusoto_core::RusotoFuture;

use std::fmt;
use std::error::Error;
use std::io;
use rusoto_core::request::HttpDispatchError;
use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};

use serde_json;
use rusoto_core::signature::SignedRequest;
use serde_json::Value as SerdeJsonValue;
use serde_json::from_str;
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
    #[doc="<p>The desired data set type.</p> <p> <ul> <li> <strong>customer_subscriber_hourly_monthly_subscriptions</strong> <p>From 2014-07-21 to present: Available daily by 5:00 PM Pacific Time.</p> </li> <li> <strong>customer_subscriber_annual_subscriptions</strong> <p>From 2014-07-21 to present: Available daily by 5:00 PM Pacific Time.</p> </li> <li> <strong>daily_business_usage_by_instance_type</strong> <p>From 2015-01-26 to present: Available daily by 5:00 PM Pacific Time.</p> </li> <li> <strong>daily_business_fees</strong> <p>From 2015-01-26 to present: Available daily by 5:00 PM Pacific Time.</p> </li> <li> <strong>daily_business_free_trial_conversions</strong> <p>From 2015-01-26 to present: Available daily by 5:00 PM Pacific Time.</p> </li> <li> <strong>daily_business_new_instances</strong> <p>From 2015-01-26 to present: Available daily by 5:00 PM Pacific Time.</p> </li> <li> <strong>daily_business_new_product_subscribers</strong> <p>From 2015-01-26 to present: Available daily by 5:00 PM Pacific Time.</p> </li> <li> <strong>daily_business_canceled_product_subscribers</strong> <p>From 2015-01-26 to present: Available daily by 5:00 PM Pacific Time.</p> </li> <li> <strong>monthly_revenue_billing_and_revenue_data</strong> <p>From 2015-02 to 2017-06: Available monthly on the 4th day of the month by 5:00pm Pacific Time. Data includes metered transactions (e.g. hourly) from two months prior.</p> <p>From 2017-07 to present: Available monthly on the 15th day of the month by 5:00pm Pacific Time. Data includes metered transactions (e.g. hourly) from one month prior.</p> </li> <li> <strong>monthly_revenue_annual_subscriptions</strong> <p>From 2015-02 to 2017-06: Available monthly on the 4th day of the month by 5:00pm Pacific Time. Data includes up-front software charges (e.g. annual) from one month prior.</p> <p>From 2017-07 to present: Available monthly on the 15th day of the month by 5:00pm Pacific Time. Data includes up-front software charges (e.g. annual) from one month prior.</p> </li> <li> <strong>disbursed_amount_by_product</strong> <p>From 2015-01-26 to present: Available every 30 days by 5:00 PM Pacific Time.</p> </li> <li> <strong>disbursed_amount_by_product_with_uncollected_funds</strong> <p>From 2012-04-19 to 2015-01-25: Available every 30 days by 5:00 PM Pacific Time.</p> <p>From 2015-01-26 to present: This data set was split into three data sets: disbursed_amount_by_product, disbursed_amount_by_age_of_uncollected_funds, and disbursed_amount_by_age_of_disbursed_funds.</p> </li> <li> <strong>disbursed_amount_by_instance_hours</strong> <p>From 2012-09-04 to present: Available every 30 days by 5:00 PM Pacific Time.</p> </li> <li> <strong>disbursed_amount_by_customer_geo</strong> <p>From 2012-04-19 to present: Available every 30 days by 5:00 PM Pacific Time.</p> </li> <li> <strong>disbursed_amount_by_age_of_uncollected_funds</strong> <p>From 2015-01-26 to present: Available every 30 days by 5:00 PM Pacific Time.</p> </li> <li> <strong>disbursed_amount_by_age_of_disbursed_funds</strong> <p>From 2015-01-26 to present: Available every 30 days by 5:00 PM Pacific Time.</p> </li> <li> <strong>customer_profile_by_industry</strong> <p>From 2015-10-01 to 2017-06-29: Available daily by 5:00 PM Pacific Time.</p> <p>From 2017-06-30 to present: This data set is no longer available.</p> </li> <li> <strong>customer_profile_by_revenue</strong> <p>From 2015-10-01 to 2017-06-29: Available daily by 5:00 PM Pacific Time.</p> <p>From 2017-06-30 to present: This data set is no longer available.</p> </li> <li> <strong>customer_profile_by_geography</strong> <p>From 2015-10-01 to 2017-06-29: Available daily by 5:00 PM Pacific Time.</p> <p>From 2017-06-30 to present: This data set is no longer available.</p> </li> <li> <strong>sales_compensation_billed_revenue</strong> <p>From 2016-12 to 2017-06: Available monthly on the 4th day of the month by 5:00pm Pacific Time. Data includes metered transactions (e.g. hourly) from two months prior, and up-front software charges (e.g. annual) from one month prior.</p> <p>From 2017-06 to present: Available monthly on the 15th day of the month by 5:00pm Pacific Time. Data includes metered transactions (e.g. hourly) from one month prior, and up-front software charges (e.g. annual) from one month prior.</p> </li> <li> <strong>us_sales_and_use_tax_records</strong> <p>From 2017-02-15 to present: Available monthly on the 15th day of the month by 5:00 PM Pacific Time.</p> </li> </ul> </p>"]
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
impl From<io::Error> for GenerateDataSetError {
    fn from(err: io::Error) -> GenerateDataSetError {
        GenerateDataSetError::HttpDispatch(HttpDispatchError::from(err))
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
impl From<io::Error> for StartSupportDataExportError {
    fn from(err: io::Error) -> StartSupportDataExportError {
        StartSupportDataExportError::HttpDispatch(HttpDispatchError::from(err))
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
                         -> RusotoFuture<GenerateDataSetResult, GenerateDataSetError>;


    #[doc="Given a data set type and a from date, asynchronously publishes the requested customer support data to the specified S3 bucket and notifies the specified SNS topic once the data is available. Returns a unique request identifier that can be used to correlate requests with notifications from the SNS topic. Data sets will be published in comma-separated values (CSV) format with the file name {data_set_type}_YYYY-MM-DD'T'HH-mm-ss'Z'.csv. If a file with the same name already exists (e.g. if the same data set is requested twice), the original file will be overwritten by the new file. Requires a Role with an attached permissions policy providing Allow permissions for the following actions: s3:PutObject, s3:GetBucketLocation, sns:GetTopicAttributes, sns:Publish, iam:GetRolePolicy."]
    fn start_support_data_export
        (&self,
         input: &StartSupportDataExportRequest)
         -> RusotoFuture<StartSupportDataExportResult, StartSupportDataExportError>;
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
                         -> RusotoFuture<GenerateDataSetResult, GenerateDataSetError> {
        let mut request =
            SignedRequest::new("POST", "marketplacecommerceanalytics", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "MarketplaceCommerceAnalytics20150701.GenerateDataSet");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        match self.credentials_provider.credentials() {
            Err(err) => {
                return RusotoFuture::new(future::err(err.into()));
            }
            Ok(credentials) => {
                request.sign_with_plus(&credentials, true);
            }
        };

        RusotoFuture::new({
            self.dispatcher.dispatch(request).from_err().and_then(|response| {
                            match response.status {
                                StatusCode::Ok => 
            {
                future::Either::A(response.body.concat2().map_err(|err| err.into()).map(|body| {
                    serde_json::from_str::<GenerateDataSetResult>(String::from_utf8_lossy(body.as_ref()).as_ref()).unwrap()
                }))
            },
                                _ => {
                                    future::Either::B(response.body.concat2().from_err().and_then(|body| {
                                        Err(GenerateDataSetError::from_body(String::from_utf8_lossy(body.as_ref()).as_ref()))
                                    }))
                                }
                            }
                        })
        })
    }


    #[doc="Given a data set type and a from date, asynchronously publishes the requested customer support data to the specified S3 bucket and notifies the specified SNS topic once the data is available. Returns a unique request identifier that can be used to correlate requests with notifications from the SNS topic. Data sets will be published in comma-separated values (CSV) format with the file name {data_set_type}_YYYY-MM-DD'T'HH-mm-ss'Z'.csv. If a file with the same name already exists (e.g. if the same data set is requested twice), the original file will be overwritten by the new file. Requires a Role with an attached permissions policy providing Allow permissions for the following actions: s3:PutObject, s3:GetBucketLocation, sns:GetTopicAttributes, sns:Publish, iam:GetRolePolicy."]
    fn start_support_data_export
        (&self,
         input: &StartSupportDataExportRequest)
         -> RusotoFuture<StartSupportDataExportResult, StartSupportDataExportError> {
        let mut request =
            SignedRequest::new("POST", "marketplacecommerceanalytics", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "MarketplaceCommerceAnalytics20150701.StartSupportDataExport");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        match self.credentials_provider.credentials() {
            Err(err) => {
                return RusotoFuture::new(future::err(err.into()));
            }
            Ok(credentials) => {
                request.sign_with_plus(&credentials, true);
            }
        };

        RusotoFuture::new({
            self.dispatcher.dispatch(request).from_err().and_then(|response| {
                            match response.status {
                                StatusCode::Ok => 
            {
                future::Either::A(response.body.concat2().map_err(|err| err.into()).map(|body| {
                    serde_json::from_str::<StartSupportDataExportResult>(String::from_utf8_lossy(body.as_ref()).as_ref()).unwrap()
                }))
            },
                                _ => {
                                    future::Either::B(response.body.concat2().from_err().and_then(|body| {
                                        Err(StartSupportDataExportError::from_body(String::from_utf8_lossy(body.as_ref()).as_ref()))
                                    }))
                                }
                            }
                        })
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
