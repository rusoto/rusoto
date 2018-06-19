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
/// <p>A BatchMeterUsageRequest contains UsageRecords, which indicate quantities of usage within your application.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchMeterUsageRequest {
    /// <p>Product code is used to uniquely identify a product in AWS Marketplace. The product code should be the same as the one used during the publishing of a new product.</p>
    #[serde(rename = "ProductCode")]
    pub product_code: String,
    /// <p>The set of UsageRecords to submit. BatchMeterUsage accepts up to 25 UsageRecords at a time.</p>
    #[serde(rename = "UsageRecords")]
    pub usage_records: Vec<UsageRecord>,
}

/// <p>Contains the UsageRecords processed by BatchMeterUsage and any records that have failed due to transient error.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchMeterUsageResult {
    /// <p>Contains all UsageRecords processed by BatchMeterUsage. These records were either honored by AWS Marketplace Metering Service or were invalid.</p>
    #[serde(rename = "Results")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<UsageRecordResult>>,
    /// <p>Contains all UsageRecords that were not processed by BatchMeterUsage. This is a list of UsageRecords. You can retry the failed request by making another BatchMeterUsage call with this list as input in the BatchMeterUsageRequest.</p>
    #[serde(rename = "UnprocessedRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_records: Option<Vec<UsageRecord>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct MeterUsageRequest {
    /// <p>Checks whether you have the permissions required for the action, but does not make the request. If you have the permissions, the request returns DryRunOperation; otherwise, it returns UnauthorizedException.</p>
    #[serde(rename = "DryRun")]
    pub dry_run: bool,
    /// <p>Product code is used to uniquely identify a product in AWS Marketplace. The product code should be the same as the one used during the publishing of a new product.</p>
    #[serde(rename = "ProductCode")]
    pub product_code: String,
    /// <p>Timestamp of the hour, recorded in UTC. The seconds and milliseconds portions of the timestamp will be ignored.</p>
    #[serde(rename = "Timestamp")]
    pub timestamp: f64,
    /// <p>It will be one of the fcp dimension name provided during the publishing of the product.</p>
    #[serde(rename = "UsageDimension")]
    pub usage_dimension: String,
    /// <p>Consumption value for the hour.</p>
    #[serde(rename = "UsageQuantity")]
    pub usage_quantity: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct MeterUsageResult {
    #[serde(rename = "MeteringRecordId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metering_record_id: Option<String>,
}

/// <p>Contains input to the ResolveCustomer operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ResolveCustomerRequest {
    /// <p>When a buyer visits your website during the registration process, the buyer submits a registration token through the browser. The registration token is resolved to obtain a CustomerIdentifier and product code.</p>
    #[serde(rename = "RegistrationToken")]
    pub registration_token: String,
}

/// <p>The result of the ResolveCustomer operation. Contains the CustomerIdentifier and product code.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ResolveCustomerResult {
    /// <p>The CustomerIdentifier is used to identify an individual customer in your application. Calls to BatchMeterUsage require CustomerIdentifiers for each UsageRecord.</p>
    #[serde(rename = "CustomerIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_identifier: Option<String>,
    /// <p>The product code is returned to confirm that the buyer is registering for your product. Subsequent BatchMeterUsage calls should be made using this product code.</p>
    #[serde(rename = "ProductCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_code: Option<String>,
}

/// <p>A UsageRecord indicates a quantity of usage for a given product, customer, dimension and time.</p> <p>Multiple requests with the same UsageRecords as input will be deduplicated to prevent double charges.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UsageRecord {
    /// <p>The CustomerIdentifier is obtained through the ResolveCustomer operation and represents an individual buyer in your application.</p>
    #[serde(rename = "CustomerIdentifier")]
    pub customer_identifier: String,
    /// <p>During the process of registering a product on AWS Marketplace, up to eight dimensions are specified. These represent different units of value in your application.</p>
    #[serde(rename = "Dimension")]
    pub dimension: String,
    /// <p>The quantity of usage consumed by the customer for the given dimension and time.</p>
    #[serde(rename = "Quantity")]
    pub quantity: i64,
    /// <p>Timestamp of the hour, recorded in UTC. The seconds and milliseconds portions of the timestamp will be ignored.</p> <p>Your application can meter usage for up to one hour in the past.</p>
    #[serde(rename = "Timestamp")]
    pub timestamp: f64,
}

/// <p>A UsageRecordResult indicates the status of a given UsageRecord processed by BatchMeterUsage.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UsageRecordResult {
    /// <p>The MeteringRecordId is a unique identifier for this metering event.</p>
    #[serde(rename = "MeteringRecordId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metering_record_id: Option<String>,
    /// <p><p>The UsageRecordResult Status indicates the status of an individual UsageRecord processed by BatchMeterUsage.</p> <ul> <li> <p> <i>Success</i>- The UsageRecord was accepted and honored by BatchMeterUsage.</p> </li> <li> <p> <i>CustomerNotSubscribed</i>- The CustomerIdentifier specified is not subscribed to your product. The UsageRecord was not honored. Future UsageRecords for this customer will fail until the customer subscribes to your product.</p> </li> <li> <p> <i>DuplicateRecord</i>- Indicates that the UsageRecord was invalid and not honored. A previously metered UsageRecord had the same customer, dimension, and time, but a different quantity.</p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The UsageRecord that was part of the BatchMeterUsage request.</p>
    #[serde(rename = "UsageRecord")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_record: Option<UsageRecord>,
}

/// Errors returned by BatchMeterUsage
#[derive(Debug, PartialEq)]
pub enum BatchMeterUsageError {
    /// <p>An internal error has occurred. Retry your request. If the problem persists, post a message with details on the AWS forums.</p>
    InternalServiceError(String),
    /// <p>You have metered usage for a CustomerIdentifier that does not exist.</p>
    InvalidCustomerIdentifier(String),
    /// <p>The product code passed does not match the product code used for publishing the product.</p>
    InvalidProductCode(String),
    /// <p>The usage dimension does not match one of the UsageDimensions associated with products.</p>
    InvalidUsageDimension(String),
    /// <p>The calls to the MeterUsage API are throttled.</p>
    Throttling(String),
    /// <p>The timestamp value passed in the meterUsage() is out of allowed range.</p>
    TimestampOutOfBounds(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl BatchMeterUsageError {
    pub fn from_body(body: &str) -> BatchMeterUsageError {
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
                    "InternalServiceErrorException" => {
                        BatchMeterUsageError::InternalServiceError(String::from(error_message))
                    }
                    "InvalidCustomerIdentifierException" => {
                        BatchMeterUsageError::InvalidCustomerIdentifier(String::from(error_message))
                    }
                    "InvalidProductCodeException" => {
                        BatchMeterUsageError::InvalidProductCode(String::from(error_message))
                    }
                    "InvalidUsageDimensionException" => {
                        BatchMeterUsageError::InvalidUsageDimension(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        BatchMeterUsageError::Throttling(String::from(error_message))
                    }
                    "TimestampOutOfBoundsException" => {
                        BatchMeterUsageError::TimestampOutOfBounds(String::from(error_message))
                    }
                    "ValidationException" => {
                        BatchMeterUsageError::Validation(error_message.to_string())
                    }
                    _ => BatchMeterUsageError::Unknown(String::from(body)),
                }
            }
            Err(_) => BatchMeterUsageError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for BatchMeterUsageError {
    fn from(err: serde_json::error::Error) -> BatchMeterUsageError {
        BatchMeterUsageError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for BatchMeterUsageError {
    fn from(err: CredentialsError) -> BatchMeterUsageError {
        BatchMeterUsageError::Credentials(err)
    }
}
impl From<HttpDispatchError> for BatchMeterUsageError {
    fn from(err: HttpDispatchError) -> BatchMeterUsageError {
        BatchMeterUsageError::HttpDispatch(err)
    }
}
impl From<io::Error> for BatchMeterUsageError {
    fn from(err: io::Error) -> BatchMeterUsageError {
        BatchMeterUsageError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for BatchMeterUsageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchMeterUsageError {
    fn description(&self) -> &str {
        match *self {
            BatchMeterUsageError::InternalServiceError(ref cause) => cause,
            BatchMeterUsageError::InvalidCustomerIdentifier(ref cause) => cause,
            BatchMeterUsageError::InvalidProductCode(ref cause) => cause,
            BatchMeterUsageError::InvalidUsageDimension(ref cause) => cause,
            BatchMeterUsageError::Throttling(ref cause) => cause,
            BatchMeterUsageError::TimestampOutOfBounds(ref cause) => cause,
            BatchMeterUsageError::Validation(ref cause) => cause,
            BatchMeterUsageError::Credentials(ref err) => err.description(),
            BatchMeterUsageError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            BatchMeterUsageError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by MeterUsage
#[derive(Debug, PartialEq)]
pub enum MeterUsageError {
    /// <p>A metering record has already been emitted by the same EC2 instance for the given {usageDimension, timestamp} with a different usageQuantity.</p>
    DuplicateRequest(String),
    /// <p>An internal error has occurred. Retry your request. If the problem persists, post a message with details on the AWS forums.</p>
    InternalServiceError(String),
    /// <p>The endpoint being called is in a region different from your EC2 instance. The region of the Metering service endpoint and the region of the EC2 instance must match.</p>
    InvalidEndpointRegion(String),
    /// <p>The product code passed does not match the product code used for publishing the product.</p>
    InvalidProductCode(String),
    /// <p>The usage dimension does not match one of the UsageDimensions associated with products.</p>
    InvalidUsageDimension(String),
    /// <p>The calls to the MeterUsage API are throttled.</p>
    Throttling(String),
    /// <p>The timestamp value passed in the meterUsage() is out of allowed range.</p>
    TimestampOutOfBounds(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl MeterUsageError {
    pub fn from_body(body: &str) -> MeterUsageError {
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
                    "DuplicateRequestException" => {
                        MeterUsageError::DuplicateRequest(String::from(error_message))
                    }
                    "InternalServiceErrorException" => {
                        MeterUsageError::InternalServiceError(String::from(error_message))
                    }
                    "InvalidEndpointRegionException" => {
                        MeterUsageError::InvalidEndpointRegion(String::from(error_message))
                    }
                    "InvalidProductCodeException" => {
                        MeterUsageError::InvalidProductCode(String::from(error_message))
                    }
                    "InvalidUsageDimensionException" => {
                        MeterUsageError::InvalidUsageDimension(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        MeterUsageError::Throttling(String::from(error_message))
                    }
                    "TimestampOutOfBoundsException" => {
                        MeterUsageError::TimestampOutOfBounds(String::from(error_message))
                    }
                    "ValidationException" => MeterUsageError::Validation(error_message.to_string()),
                    _ => MeterUsageError::Unknown(String::from(body)),
                }
            }
            Err(_) => MeterUsageError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for MeterUsageError {
    fn from(err: serde_json::error::Error) -> MeterUsageError {
        MeterUsageError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for MeterUsageError {
    fn from(err: CredentialsError) -> MeterUsageError {
        MeterUsageError::Credentials(err)
    }
}
impl From<HttpDispatchError> for MeterUsageError {
    fn from(err: HttpDispatchError) -> MeterUsageError {
        MeterUsageError::HttpDispatch(err)
    }
}
impl From<io::Error> for MeterUsageError {
    fn from(err: io::Error) -> MeterUsageError {
        MeterUsageError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for MeterUsageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for MeterUsageError {
    fn description(&self) -> &str {
        match *self {
            MeterUsageError::DuplicateRequest(ref cause) => cause,
            MeterUsageError::InternalServiceError(ref cause) => cause,
            MeterUsageError::InvalidEndpointRegion(ref cause) => cause,
            MeterUsageError::InvalidProductCode(ref cause) => cause,
            MeterUsageError::InvalidUsageDimension(ref cause) => cause,
            MeterUsageError::Throttling(ref cause) => cause,
            MeterUsageError::TimestampOutOfBounds(ref cause) => cause,
            MeterUsageError::Validation(ref cause) => cause,
            MeterUsageError::Credentials(ref err) => err.description(),
            MeterUsageError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            MeterUsageError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ResolveCustomer
#[derive(Debug, PartialEq)]
pub enum ResolveCustomerError {
    /// <p>The submitted registration token has expired. This can happen if the buyer's browser takes too long to redirect to your page, the buyer has resubmitted the registration token, or your application has held on to the registration token for too long. Your SaaS registration website should redeem this token as soon as it is submitted by the buyer's browser.</p>
    ExpiredToken(String),
    /// <p>An internal error has occurred. Retry your request. If the problem persists, post a message with details on the AWS forums.</p>
    InternalServiceError(String),

    InvalidToken(String),
    /// <p>The calls to the MeterUsage API are throttled.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ResolveCustomerError {
    pub fn from_body(body: &str) -> ResolveCustomerError {
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
                    "ExpiredTokenException" => {
                        ResolveCustomerError::ExpiredToken(String::from(error_message))
                    }
                    "InternalServiceErrorException" => {
                        ResolveCustomerError::InternalServiceError(String::from(error_message))
                    }
                    "InvalidTokenException" => {
                        ResolveCustomerError::InvalidToken(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        ResolveCustomerError::Throttling(String::from(error_message))
                    }
                    "ValidationException" => {
                        ResolveCustomerError::Validation(error_message.to_string())
                    }
                    _ => ResolveCustomerError::Unknown(String::from(body)),
                }
            }
            Err(_) => ResolveCustomerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ResolveCustomerError {
    fn from(err: serde_json::error::Error) -> ResolveCustomerError {
        ResolveCustomerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ResolveCustomerError {
    fn from(err: CredentialsError) -> ResolveCustomerError {
        ResolveCustomerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ResolveCustomerError {
    fn from(err: HttpDispatchError) -> ResolveCustomerError {
        ResolveCustomerError::HttpDispatch(err)
    }
}
impl From<io::Error> for ResolveCustomerError {
    fn from(err: io::Error) -> ResolveCustomerError {
        ResolveCustomerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ResolveCustomerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ResolveCustomerError {
    fn description(&self) -> &str {
        match *self {
            ResolveCustomerError::ExpiredToken(ref cause) => cause,
            ResolveCustomerError::InternalServiceError(ref cause) => cause,
            ResolveCustomerError::InvalidToken(ref cause) => cause,
            ResolveCustomerError::Throttling(ref cause) => cause,
            ResolveCustomerError::Validation(ref cause) => cause,
            ResolveCustomerError::Credentials(ref err) => err.description(),
            ResolveCustomerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ResolveCustomerError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWSMarketplace Metering API. AWSMarketplace Metering clients implement this trait.
pub trait MarketplaceMetering {
    /// <p>BatchMeterUsage is called from a SaaS application listed on the AWS Marketplace to post metering records for a set of customers.</p> <p>For identical requests, the API is idempotent; requests can be retried with the same records or a subset of the input records.</p> <p>Every request to BatchMeterUsage is for one product. If you need to meter usage for multiple products, you must make multiple calls to BatchMeterUsage.</p> <p>BatchMeterUsage can process up to 25 UsageRecords at a time.</p>
    fn batch_meter_usage(
        &self,
        input: BatchMeterUsageRequest,
    ) -> RusotoFuture<BatchMeterUsageResult, BatchMeterUsageError>;

    /// <p>API to emit metering records. For identical requests, the API is idempotent. It simply returns the metering record ID.</p> <p>MeterUsage is authenticated on the buyer's AWS account, generally when running from an EC2 instance on the AWS Marketplace.</p>
    fn meter_usage(
        &self,
        input: MeterUsageRequest,
    ) -> RusotoFuture<MeterUsageResult, MeterUsageError>;

    /// <p>ResolveCustomer is called by a SaaS application during the registration process. When a buyer visits your website during the registration process, the buyer submits a registration token through their browser. The registration token is resolved through this API to obtain a CustomerIdentifier and product code.</p>
    fn resolve_customer(
        &self,
        input: ResolveCustomerRequest,
    ) -> RusotoFuture<ResolveCustomerResult, ResolveCustomerError>;
}
/// A client for the AWSMarketplace Metering API.
pub struct MarketplaceMeteringClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl MarketplaceMeteringClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> MarketplaceMeteringClient {
        MarketplaceMeteringClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> MarketplaceMeteringClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        MarketplaceMeteringClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> MarketplaceMetering for MarketplaceMeteringClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p>BatchMeterUsage is called from a SaaS application listed on the AWS Marketplace to post metering records for a set of customers.</p> <p>For identical requests, the API is idempotent; requests can be retried with the same records or a subset of the input records.</p> <p>Every request to BatchMeterUsage is for one product. If you need to meter usage for multiple products, you must make multiple calls to BatchMeterUsage.</p> <p>BatchMeterUsage can process up to 25 UsageRecords at a time.</p>
    fn batch_meter_usage(
        &self,
        input: BatchMeterUsageRequest,
    ) -> RusotoFuture<BatchMeterUsageResult, BatchMeterUsageError> {
        let mut request = SignedRequest::new("POST", "aws-marketplace", &self.region, "/");
        request.set_endpoint_prefix("metering.marketplace".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSMPMeteringService.BatchMeterUsage");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<BatchMeterUsageResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(BatchMeterUsageError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>API to emit metering records. For identical requests, the API is idempotent. It simply returns the metering record ID.</p> <p>MeterUsage is authenticated on the buyer's AWS account, generally when running from an EC2 instance on the AWS Marketplace.</p>
    fn meter_usage(
        &self,
        input: MeterUsageRequest,
    ) -> RusotoFuture<MeterUsageResult, MeterUsageError> {
        let mut request = SignedRequest::new("POST", "aws-marketplace", &self.region, "/");
        request.set_endpoint_prefix("metering.marketplace".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSMPMeteringService.MeterUsage");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<MeterUsageResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(MeterUsageError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>ResolveCustomer is called by a SaaS application during the registration process. When a buyer visits your website during the registration process, the buyer submits a registration token through their browser. The registration token is resolved through this API to obtain a CustomerIdentifier and product code.</p>
    fn resolve_customer(
        &self,
        input: ResolveCustomerRequest,
    ) -> RusotoFuture<ResolveCustomerResult, ResolveCustomerError> {
        let mut request = SignedRequest::new("POST", "aws-marketplace", &self.region, "/");
        request.set_endpoint_prefix("metering.marketplace".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSMPMeteringService.ResolveCustomer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ResolveCustomerResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ResolveCustomerError::from_body(
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
