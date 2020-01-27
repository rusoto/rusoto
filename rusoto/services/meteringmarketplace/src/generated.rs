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
#[allow(warnings)]
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
/// <p>A BatchMeterUsageRequest contains UsageRecords, which indicate quantities of usage within your application.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct MeterUsageRequest {
    /// <p>Checks whether you have the permissions required for the action, but does not make the request. If you have the permissions, the request returns DryRunOperation; otherwise, it returns UnauthorizedException. Defaults to <code>false</code> if not specified.</p>
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// <p>Product code is used to uniquely identify a product in AWS Marketplace. The product code should be the same as the one used during the publishing of a new product.</p>
    #[serde(rename = "ProductCode")]
    pub product_code: String,
    /// <p>Timestamp, in UTC, for which the usage is being reported. Your application can meter usage for up to one hour in the past. Make sure the timestamp value is not before the start of the software usage.</p>
    #[serde(rename = "Timestamp")]
    pub timestamp: f64,
    /// <p>It will be one of the fcp dimension name provided during the publishing of the product.</p>
    #[serde(rename = "UsageDimension")]
    pub usage_dimension: String,
    /// <p>Consumption value for the hour. Defaults to <code>0</code> if not specified.</p>
    #[serde(rename = "UsageQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_quantity: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MeterUsageResult {
    /// <p>Metering record id.</p>
    #[serde(rename = "MeteringRecordId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metering_record_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RegisterUsageRequest {
    /// <p>(Optional) To scope down the registration to a specific running software instance and guard against replay attacks.</p>
    #[serde(rename = "Nonce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    /// <p>Product code is used to uniquely identify a product in AWS Marketplace. The product code should be the same as the one used during the publishing of a new product.</p>
    #[serde(rename = "ProductCode")]
    pub product_code: String,
    /// <p>Public Key Version provided by AWS Marketplace</p>
    #[serde(rename = "PublicKeyVersion")]
    pub public_key_version: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RegisterUsageResult {
    /// <p>(Optional) Only included when public key version has expired</p>
    #[serde(rename = "PublicKeyRotationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key_rotation_timestamp: Option<f64>,
    /// <p>JWT Token</p>
    #[serde(rename = "Signature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

/// <p>Contains input to the ResolveCustomer operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ResolveCustomerRequest {
    /// <p>When a buyer visits your website during the registration process, the buyer submits a registration token through the browser. The registration token is resolved to obtain a CustomerIdentifier and product code.</p>
    #[serde(rename = "RegistrationToken")]
    pub registration_token: String,
}

/// <p>The result of the ResolveCustomer operation. Contains the CustomerIdentifier and product code.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The quantity of usage consumed by the customer for the given dimension and time. Defaults to <code>0</code> if not specified.</p>
    #[serde(rename = "Quantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    /// <p>Timestamp, in UTC, for which the usage is being reported.</p> <p>Your application can meter usage for up to one hour in the past. Make sure the timestamp value is not before the start of the software usage.</p>
    #[serde(rename = "Timestamp")]
    pub timestamp: f64,
}

/// <p>A UsageRecordResult indicates the status of a given UsageRecord processed by BatchMeterUsage.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The API is disabled in the Region.</p>
    DisabledApi(String),
    /// <p>An internal error has occurred. Retry your request. If the problem persists, post a message with details on the AWS forums.</p>
    InternalServiceError(String),
    /// <p>You have metered usage for a CustomerIdentifier that does not exist.</p>
    InvalidCustomerIdentifier(String),
    /// <p>The product code passed does not match the product code used for publishing the product.</p>
    InvalidProductCode(String),
    /// <p>The usage dimension does not match one of the UsageDimensions associated with products.</p>
    InvalidUsageDimension(String),
    /// <p>The calls to the API are throttled.</p>
    Throttling(String),
    /// <p>The timestamp value passed in the meterUsage() is out of allowed range.</p>
    TimestampOutOfBounds(String),
}

impl BatchMeterUsageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchMeterUsageError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DisabledApiException" => {
                    return RusotoError::Service(BatchMeterUsageError::DisabledApi(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(BatchMeterUsageError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidCustomerIdentifierException" => {
                    return RusotoError::Service(BatchMeterUsageError::InvalidCustomerIdentifier(
                        err.msg,
                    ))
                }
                "InvalidProductCodeException" => {
                    return RusotoError::Service(BatchMeterUsageError::InvalidProductCode(err.msg))
                }
                "InvalidUsageDimensionException" => {
                    return RusotoError::Service(BatchMeterUsageError::InvalidUsageDimension(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(BatchMeterUsageError::Throttling(err.msg))
                }
                "TimestampOutOfBoundsException" => {
                    return RusotoError::Service(BatchMeterUsageError::TimestampOutOfBounds(
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
impl fmt::Display for BatchMeterUsageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchMeterUsageError::DisabledApi(ref cause) => write!(f, "{}", cause),
            BatchMeterUsageError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            BatchMeterUsageError::InvalidCustomerIdentifier(ref cause) => write!(f, "{}", cause),
            BatchMeterUsageError::InvalidProductCode(ref cause) => write!(f, "{}", cause),
            BatchMeterUsageError::InvalidUsageDimension(ref cause) => write!(f, "{}", cause),
            BatchMeterUsageError::Throttling(ref cause) => write!(f, "{}", cause),
            BatchMeterUsageError::TimestampOutOfBounds(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchMeterUsageError {}
/// Errors returned by MeterUsage
#[derive(Debug, PartialEq)]
pub enum MeterUsageError {
    /// <p>Exception thrown when the customer does not have a valid subscription for the product.</p>
    CustomerNotEntitled(String),
    /// <p>A metering record has already been emitted by the same EC2 instance, ECS task, or EKS pod for the given {usageDimension, timestamp} with a different usageQuantity.</p>
    DuplicateRequest(String),
    /// <p>An internal error has occurred. Retry your request. If the problem persists, post a message with details on the AWS forums.</p>
    InternalServiceError(String),
    /// <p>The endpoint being called is in a AWS Region different from your EC2 instance, ECS task, or EKS pod. The Region of the Metering Service endpoint and the AWS Region of the resource must match.</p>
    InvalidEndpointRegion(String),
    /// <p>The product code passed does not match the product code used for publishing the product.</p>
    InvalidProductCode(String),
    /// <p>The usage dimension does not match one of the UsageDimensions associated with products.</p>
    InvalidUsageDimension(String),
    /// <p>The calls to the API are throttled.</p>
    Throttling(String),
    /// <p>The timestamp value passed in the meterUsage() is out of allowed range.</p>
    TimestampOutOfBounds(String),
}

impl MeterUsageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<MeterUsageError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CustomerNotEntitledException" => {
                    return RusotoError::Service(MeterUsageError::CustomerNotEntitled(err.msg))
                }
                "DuplicateRequestException" => {
                    return RusotoError::Service(MeterUsageError::DuplicateRequest(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(MeterUsageError::InternalServiceError(err.msg))
                }
                "InvalidEndpointRegionException" => {
                    return RusotoError::Service(MeterUsageError::InvalidEndpointRegion(err.msg))
                }
                "InvalidProductCodeException" => {
                    return RusotoError::Service(MeterUsageError::InvalidProductCode(err.msg))
                }
                "InvalidUsageDimensionException" => {
                    return RusotoError::Service(MeterUsageError::InvalidUsageDimension(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(MeterUsageError::Throttling(err.msg))
                }
                "TimestampOutOfBoundsException" => {
                    return RusotoError::Service(MeterUsageError::TimestampOutOfBounds(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for MeterUsageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MeterUsageError::CustomerNotEntitled(ref cause) => write!(f, "{}", cause),
            MeterUsageError::DuplicateRequest(ref cause) => write!(f, "{}", cause),
            MeterUsageError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            MeterUsageError::InvalidEndpointRegion(ref cause) => write!(f, "{}", cause),
            MeterUsageError::InvalidProductCode(ref cause) => write!(f, "{}", cause),
            MeterUsageError::InvalidUsageDimension(ref cause) => write!(f, "{}", cause),
            MeterUsageError::Throttling(ref cause) => write!(f, "{}", cause),
            MeterUsageError::TimestampOutOfBounds(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for MeterUsageError {}
/// Errors returned by RegisterUsage
#[derive(Debug, PartialEq)]
pub enum RegisterUsageError {
    /// <p>Exception thrown when the customer does not have a valid subscription for the product.</p>
    CustomerNotEntitled(String),
    /// <p>The API is disabled in the Region.</p>
    DisabledApi(String),
    /// <p>An internal error has occurred. Retry your request. If the problem persists, post a message with details on the AWS forums.</p>
    InternalServiceError(String),
    /// <p>The product code passed does not match the product code used for publishing the product.</p>
    InvalidProductCode(String),
    /// <p>Public Key version is invalid.</p>
    InvalidPublicKeyVersion(String),
    /// <p>RegisterUsage must be called in the same AWS Region the ECS task was launched in. This prevents a container from hardcoding a Region (e.g. withRegion(“us-east-1”) when calling RegisterUsage.</p>
    InvalidRegion(String),
    /// <p>AWS Marketplace does not support metering usage from the underlying platform. Currently, only Amazon ECS is supported.</p>
    PlatformNotSupported(String),
    /// <p>The calls to the API are throttled.</p>
    Throttling(String),
}

impl RegisterUsageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RegisterUsageError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CustomerNotEntitledException" => {
                    return RusotoError::Service(RegisterUsageError::CustomerNotEntitled(err.msg))
                }
                "DisabledApiException" => {
                    return RusotoError::Service(RegisterUsageError::DisabledApi(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(RegisterUsageError::InternalServiceError(err.msg))
                }
                "InvalidProductCodeException" => {
                    return RusotoError::Service(RegisterUsageError::InvalidProductCode(err.msg))
                }
                "InvalidPublicKeyVersionException" => {
                    return RusotoError::Service(RegisterUsageError::InvalidPublicKeyVersion(
                        err.msg,
                    ))
                }
                "InvalidRegionException" => {
                    return RusotoError::Service(RegisterUsageError::InvalidRegion(err.msg))
                }
                "PlatformNotSupportedException" => {
                    return RusotoError::Service(RegisterUsageError::PlatformNotSupported(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(RegisterUsageError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RegisterUsageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RegisterUsageError::CustomerNotEntitled(ref cause) => write!(f, "{}", cause),
            RegisterUsageError::DisabledApi(ref cause) => write!(f, "{}", cause),
            RegisterUsageError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            RegisterUsageError::InvalidProductCode(ref cause) => write!(f, "{}", cause),
            RegisterUsageError::InvalidPublicKeyVersion(ref cause) => write!(f, "{}", cause),
            RegisterUsageError::InvalidRegion(ref cause) => write!(f, "{}", cause),
            RegisterUsageError::PlatformNotSupported(ref cause) => write!(f, "{}", cause),
            RegisterUsageError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RegisterUsageError {}
/// Errors returned by ResolveCustomer
#[derive(Debug, PartialEq)]
pub enum ResolveCustomerError {
    /// <p>The API is disabled in the Region.</p>
    DisabledApi(String),
    /// <p>The submitted registration token has expired. This can happen if the buyer's browser takes too long to redirect to your page, the buyer has resubmitted the registration token, or your application has held on to the registration token for too long. Your SaaS registration website should redeem this token as soon as it is submitted by the buyer's browser.</p>
    ExpiredToken(String),
    /// <p>An internal error has occurred. Retry your request. If the problem persists, post a message with details on the AWS forums.</p>
    InternalServiceError(String),
    /// <p>Registration token is invalid.</p>
    InvalidToken(String),
    /// <p>The calls to the API are throttled.</p>
    Throttling(String),
}

impl ResolveCustomerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ResolveCustomerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DisabledApiException" => {
                    return RusotoError::Service(ResolveCustomerError::DisabledApi(err.msg))
                }
                "ExpiredTokenException" => {
                    return RusotoError::Service(ResolveCustomerError::ExpiredToken(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(ResolveCustomerError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidTokenException" => {
                    return RusotoError::Service(ResolveCustomerError::InvalidToken(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ResolveCustomerError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ResolveCustomerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ResolveCustomerError::DisabledApi(ref cause) => write!(f, "{}", cause),
            ResolveCustomerError::ExpiredToken(ref cause) => write!(f, "{}", cause),
            ResolveCustomerError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            ResolveCustomerError::InvalidToken(ref cause) => write!(f, "{}", cause),
            ResolveCustomerError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ResolveCustomerError {}
/// Trait representing the capabilities of the AWSMarketplace Metering API. AWSMarketplace Metering clients implement this trait.
#[async_trait]
pub trait MarketplaceMetering {
    /// <p>BatchMeterUsage is called from a SaaS application listed on the AWS Marketplace to post metering records for a set of customers.</p> <p>For identical requests, the API is idempotent; requests can be retried with the same records or a subset of the input records.</p> <p>Every request to BatchMeterUsage is for one product. If you need to meter usage for multiple products, you must make multiple calls to BatchMeterUsage.</p> <p>BatchMeterUsage can process up to 25 UsageRecords at a time.</p>
    async fn batch_meter_usage(
        &self,
        input: BatchMeterUsageRequest,
    ) -> Result<BatchMeterUsageResult, RusotoError<BatchMeterUsageError>>;

    /// <p>API to emit metering records. For identical requests, the API is idempotent. It simply returns the metering record ID.</p> <p>MeterUsage is authenticated on the buyer's AWS account using credentials from the EC2 instance, ECS task, or EKS pod.</p>
    async fn meter_usage(
        &self,
        input: MeterUsageRequest,
    ) -> Result<MeterUsageResult, RusotoError<MeterUsageError>>;

    /// <p><p>Paid container software products sold through AWS Marketplace must integrate with the AWS Marketplace Metering Service and call the RegisterUsage operation for software entitlement and metering. Free and BYOL products for Amazon ECS or Amazon EKS aren&#39;t required to call RegisterUsage, but you may choose to do so if you would like to receive usage data in your seller reports. The sections below explain the behavior of RegisterUsage. RegisterUsage performs two primary functions: metering and entitlement.</p> <ul> <li> <p> <i>Entitlement</i>: RegisterUsage allows you to verify that the customer running your paid software is subscribed to your product on AWS Marketplace, enabling you to guard against unauthorized use. Your container image that integrates with RegisterUsage is only required to guard against unauthorized use at container startup, as such a CustomerNotSubscribedException/PlatformNotSupportedException will only be thrown on the initial call to RegisterUsage. Subsequent calls from the same Amazon ECS task instance (e.g. task-id) or Amazon EKS pod will not throw a CustomerNotSubscribedException, even if the customer unsubscribes while the Amazon ECS task or Amazon EKS pod is still running.</p> </li> <li> <p> <i>Metering</i>: RegisterUsage meters software use per ECS task, per hour, or per pod for Amazon EKS with usage prorated to the second. A minimum of 1 minute of usage applies to tasks that are short lived. For example, if a customer has a 10 node Amazon ECS or Amazon EKS cluster and a service configured as a Daemon Set, then Amazon ECS or Amazon EKS will launch a task on all 10 cluster nodes and the customer will be charged: (10 * hourly_rate). Metering for software use is automatically handled by the AWS Marketplace Metering Control Plane -- your software is not required to perform any metering specific actions, other than call RegisterUsage once for metering of software use to commence. The AWS Marketplace Metering Control Plane will also continue to bill customers for running ECS tasks and Amazon EKS pods, regardless of the customers subscription state, removing the need for your software to perform entitlement checks at runtime.</p> </li> </ul></p>
    async fn register_usage(
        &self,
        input: RegisterUsageRequest,
    ) -> Result<RegisterUsageResult, RusotoError<RegisterUsageError>>;

    /// <p>ResolveCustomer is called by a SaaS application during the registration process. When a buyer visits your website during the registration process, the buyer submits a registration token through their browser. The registration token is resolved through this API to obtain a CustomerIdentifier and product code.</p>
    async fn resolve_customer(
        &self,
        input: ResolveCustomerRequest,
    ) -> Result<ResolveCustomerResult, RusotoError<ResolveCustomerError>>;
}
/// A client for the AWSMarketplace Metering API.
#[derive(Clone)]
pub struct MarketplaceMeteringClient {
    client: Client,
    region: region::Region,
}

impl MarketplaceMeteringClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> MarketplaceMeteringClient {
        MarketplaceMeteringClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> MarketplaceMeteringClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        MarketplaceMeteringClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> MarketplaceMeteringClient {
        MarketplaceMeteringClient { client, region }
    }
}

#[async_trait]
impl MarketplaceMetering for MarketplaceMeteringClient {
    /// <p>BatchMeterUsage is called from a SaaS application listed on the AWS Marketplace to post metering records for a set of customers.</p> <p>For identical requests, the API is idempotent; requests can be retried with the same records or a subset of the input records.</p> <p>Every request to BatchMeterUsage is for one product. If you need to meter usage for multiple products, you must make multiple calls to BatchMeterUsage.</p> <p>BatchMeterUsage can process up to 25 UsageRecords at a time.</p>
    async fn batch_meter_usage(
        &self,
        input: BatchMeterUsageRequest,
    ) -> Result<BatchMeterUsageResult, RusotoError<BatchMeterUsageError>> {
        let mut request = SignedRequest::new("POST", "aws-marketplace", &self.region, "/");
        request.set_endpoint_prefix("metering.marketplace".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSMPMeteringService.BatchMeterUsage");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<BatchMeterUsageResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(BatchMeterUsageError::from_response(response))
        }
    }

    /// <p>API to emit metering records. For identical requests, the API is idempotent. It simply returns the metering record ID.</p> <p>MeterUsage is authenticated on the buyer's AWS account using credentials from the EC2 instance, ECS task, or EKS pod.</p>
    async fn meter_usage(
        &self,
        input: MeterUsageRequest,
    ) -> Result<MeterUsageResult, RusotoError<MeterUsageError>> {
        let mut request = SignedRequest::new("POST", "aws-marketplace", &self.region, "/");
        request.set_endpoint_prefix("metering.marketplace".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSMPMeteringService.MeterUsage");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<MeterUsageResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(MeterUsageError::from_response(response))
        }
    }

    /// <p><p>Paid container software products sold through AWS Marketplace must integrate with the AWS Marketplace Metering Service and call the RegisterUsage operation for software entitlement and metering. Free and BYOL products for Amazon ECS or Amazon EKS aren&#39;t required to call RegisterUsage, but you may choose to do so if you would like to receive usage data in your seller reports. The sections below explain the behavior of RegisterUsage. RegisterUsage performs two primary functions: metering and entitlement.</p> <ul> <li> <p> <i>Entitlement</i>: RegisterUsage allows you to verify that the customer running your paid software is subscribed to your product on AWS Marketplace, enabling you to guard against unauthorized use. Your container image that integrates with RegisterUsage is only required to guard against unauthorized use at container startup, as such a CustomerNotSubscribedException/PlatformNotSupportedException will only be thrown on the initial call to RegisterUsage. Subsequent calls from the same Amazon ECS task instance (e.g. task-id) or Amazon EKS pod will not throw a CustomerNotSubscribedException, even if the customer unsubscribes while the Amazon ECS task or Amazon EKS pod is still running.</p> </li> <li> <p> <i>Metering</i>: RegisterUsage meters software use per ECS task, per hour, or per pod for Amazon EKS with usage prorated to the second. A minimum of 1 minute of usage applies to tasks that are short lived. For example, if a customer has a 10 node Amazon ECS or Amazon EKS cluster and a service configured as a Daemon Set, then Amazon ECS or Amazon EKS will launch a task on all 10 cluster nodes and the customer will be charged: (10 * hourly_rate). Metering for software use is automatically handled by the AWS Marketplace Metering Control Plane -- your software is not required to perform any metering specific actions, other than call RegisterUsage once for metering of software use to commence. The AWS Marketplace Metering Control Plane will also continue to bill customers for running ECS tasks and Amazon EKS pods, regardless of the customers subscription state, removing the need for your software to perform entitlement checks at runtime.</p> </li> </ul></p>
    async fn register_usage(
        &self,
        input: RegisterUsageRequest,
    ) -> Result<RegisterUsageResult, RusotoError<RegisterUsageError>> {
        let mut request = SignedRequest::new("POST", "aws-marketplace", &self.region, "/");
        request.set_endpoint_prefix("metering.marketplace".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSMPMeteringService.RegisterUsage");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<RegisterUsageResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(RegisterUsageError::from_response(response))
        }
    }

    /// <p>ResolveCustomer is called by a SaaS application during the registration process. When a buyer visits your website during the registration process, the buyer submits a registration token through their browser. The registration token is resolved through this API to obtain a CustomerIdentifier and product code.</p>
    async fn resolve_customer(
        &self,
        input: ResolveCustomerRequest,
    ) -> Result<ResolveCustomerResult, RusotoError<ResolveCustomerError>> {
        let mut request = SignedRequest::new("POST", "aws-marketplace", &self.region, "/");
        request.set_endpoint_prefix("metering.marketplace".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSMPMeteringService.ResolveCustomer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ResolveCustomerResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ResolveCustomerError::from_response(response))
        }
    }
}
