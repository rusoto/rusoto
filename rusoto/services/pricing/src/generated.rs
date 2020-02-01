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
/// <p>The values of a given attribute, such as <code>Throughput Optimized HDD</code> or <code>Provisioned IOPS</code> for the <code>Amazon EC2</code> <code>volumeType</code> attribute.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AttributeValue {
    /// <p>The specific value of an <code>attributeName</code>.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeServicesRequest {
    /// <p>The format version that you want the response to be in.</p> <p>Valid values are: <code>aws_v1</code> </p>
    #[serde(rename = "FormatVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_version: Option<String>,
    /// <p>The maximum number of results that you want returned in the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token that indicates the next set of results that you want to retrieve.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The code for the service whose information you want to retrieve, such as <code>AmazonEC2</code>. You can use the <code>ServiceCode</code> to filter the results in a <code>GetProducts</code> call. To retrieve a list of all services, leave this blank.</p>
    #[serde(rename = "ServiceCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeServicesResponse {
    /// <p>The format version of the response. For example, <code>aws_v1</code>.</p>
    #[serde(rename = "FormatVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_version: Option<String>,
    /// <p>The pagination token for the next set of retreivable results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The service metadata for the service or services in the response.</p>
    #[serde(rename = "Services")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<Service>>,
}

/// <p>The constraints that you want all returned products to match.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Filter {
    /// <p>The product metadata field that you want to filter on. You can filter by just the service code to see all products for a specific service, filter by just the attribute name to see a specific attribute for multiple services, or use both a service code and an attribute name to retrieve only products that match both fields.</p> <p>Valid values include: <code>ServiceCode</code>, and all attribute names</p> <p>For example, you can filter by the <code>AmazonEC2</code> service code and the <code>volumeType</code> attribute name to get the prices for only Amazon EC2 volumes.</p>
    #[serde(rename = "Field")]
    pub field: String,
    /// <p>The type of filter that you want to use.</p> <p>Valid values are: <code>TERM_MATCH</code>. <code>TERM_MATCH</code> returns only products that match both the given filter field and the given value.</p>
    #[serde(rename = "Type")]
    pub type_: String,
    /// <p>The service code or attribute value that you want to filter by. If you are filtering by service code this is the actual service code, such as <code>AmazonEC2</code>. If you are filtering by attribute name, this is the attribute value that you want the returned products to match, such as a <code>Provisioned IOPS</code> volume.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAttributeValuesRequest {
    /// <p>The name of the attribute that you want to retrieve the values for, such as <code>volumeType</code>.</p>
    #[serde(rename = "AttributeName")]
    pub attribute_name: String,
    /// <p>The maximum number of results to return in response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token that indicates the next set of results that you want to retrieve.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The service code for the service whose attributes you want to retrieve. For example, if you want the retrieve an EC2 attribute, use <code>AmazonEC2</code>.</p>
    #[serde(rename = "ServiceCode")]
    pub service_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAttributeValuesResponse {
    /// <p>The list of values for an attribute. For example, <code>Throughput Optimized HDD</code> and <code>Provisioned IOPS</code> are two available values for the <code>AmazonEC2</code> <code>volumeType</code>.</p>
    #[serde(rename = "AttributeValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_values: Option<Vec<AttributeValue>>,
    /// <p>The pagination token that indicates the next set of results to retrieve.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetProductsRequest {
    /// <p>The list of filters that limit the returned products. only products that match all filters are returned.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>The format version that you want the response to be in.</p> <p>Valid values are: <code>aws_v1</code> </p>
    #[serde(rename = "FormatVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_version: Option<String>,
    /// <p>The maximum number of results to return in the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token that indicates the next set of results that you want to retrieve.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The code for the service whose products you want to retrieve. </p>
    #[serde(rename = "ServiceCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetProductsResponse {
    /// <p>The format version of the response. For example, aws_v1.</p>
    #[serde(rename = "FormatVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_version: Option<String>,
    /// <p>The pagination token that indicates the next set of results to retrieve.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of products that match your filters. The list contains both the product metadata and the price information.</p>
    #[serde(rename = "PriceList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_list: Option<Vec<String>>,
}

/// <p>The metadata for a service, such as the service code and available attribute names.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Service {
    /// <p>The attributes that are available for this service.</p>
    #[serde(rename = "AttributeNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_names: Option<Vec<String>>,
    /// <p>The code for the AWS service.</p>
    #[serde(rename = "ServiceCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code: Option<String>,
}

/// Errors returned by DescribeServices
#[derive(Debug, PartialEq)]
pub enum DescribeServicesError {
    /// <p>The pagination token expired. Try again without a pagination token.</p>
    ExpiredNextToken(String),
    /// <p>An error on the server occurred during the processing of your request. Try again later.</p>
    InternalError(String),
    /// <p>The pagination token is invalid. Try again without a pagination token.</p>
    InvalidNextToken(String),
    /// <p>One or more parameters had an invalid value.</p>
    InvalidParameter(String),
    /// <p>The requested resource can't be found.</p>
    NotFound(String),
}

impl DescribeServicesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeServicesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ExpiredNextTokenException" => {
                    return RusotoError::Service(DescribeServicesError::ExpiredNextToken(err.msg))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(DescribeServicesError::InternalError(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(DescribeServicesError::InvalidNextToken(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeServicesError::InvalidParameter(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeServicesError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeServicesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeServicesError::ExpiredNextToken(ref cause) => write!(f, "{}", cause),
            DescribeServicesError::InternalError(ref cause) => write!(f, "{}", cause),
            DescribeServicesError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            DescribeServicesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeServicesError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeServicesError {}
/// Errors returned by GetAttributeValues
#[derive(Debug, PartialEq)]
pub enum GetAttributeValuesError {
    /// <p>The pagination token expired. Try again without a pagination token.</p>
    ExpiredNextToken(String),
    /// <p>An error on the server occurred during the processing of your request. Try again later.</p>
    InternalError(String),
    /// <p>The pagination token is invalid. Try again without a pagination token.</p>
    InvalidNextToken(String),
    /// <p>One or more parameters had an invalid value.</p>
    InvalidParameter(String),
    /// <p>The requested resource can't be found.</p>
    NotFound(String),
}

impl GetAttributeValuesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAttributeValuesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ExpiredNextTokenException" => {
                    return RusotoError::Service(GetAttributeValuesError::ExpiredNextToken(err.msg))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(GetAttributeValuesError::InternalError(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(GetAttributeValuesError::InvalidNextToken(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetAttributeValuesError::InvalidParameter(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetAttributeValuesError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetAttributeValuesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAttributeValuesError::ExpiredNextToken(ref cause) => write!(f, "{}", cause),
            GetAttributeValuesError::InternalError(ref cause) => write!(f, "{}", cause),
            GetAttributeValuesError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            GetAttributeValuesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetAttributeValuesError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetAttributeValuesError {}
/// Errors returned by GetProducts
#[derive(Debug, PartialEq)]
pub enum GetProductsError {
    /// <p>The pagination token expired. Try again without a pagination token.</p>
    ExpiredNextToken(String),
    /// <p>An error on the server occurred during the processing of your request. Try again later.</p>
    InternalError(String),
    /// <p>The pagination token is invalid. Try again without a pagination token.</p>
    InvalidNextToken(String),
    /// <p>One or more parameters had an invalid value.</p>
    InvalidParameter(String),
    /// <p>The requested resource can't be found.</p>
    NotFound(String),
}

impl GetProductsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetProductsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ExpiredNextTokenException" => {
                    return RusotoError::Service(GetProductsError::ExpiredNextToken(err.msg))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(GetProductsError::InternalError(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(GetProductsError::InvalidNextToken(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetProductsError::InvalidParameter(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetProductsError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetProductsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetProductsError::ExpiredNextToken(ref cause) => write!(f, "{}", cause),
            GetProductsError::InternalError(ref cause) => write!(f, "{}", cause),
            GetProductsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            GetProductsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetProductsError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetProductsError {}
/// Trait representing the capabilities of the AWS Pricing API. AWS Pricing clients implement this trait.
#[async_trait]
pub trait Pricing {
    /// <p>Returns the metadata for one service or a list of the metadata for all services. Use this without a service code to get the service codes for all services. Use it with a service code, such as <code>AmazonEC2</code>, to get information specific to that service, such as the attribute names available for that service. For example, some of the attribute names available for EC2 are <code>volumeType</code>, <code>maxIopsVolume</code>, <code>operation</code>, <code>locationType</code>, and <code>instanceCapacity10xlarge</code>.</p>
    async fn describe_services(
        &self,
        input: DescribeServicesRequest,
    ) -> Result<DescribeServicesResponse, RusotoError<DescribeServicesError>>;

    /// <p>Returns a list of attribute values. Attibutes are similar to the details in a Price List API offer file. For a list of available attributes, see <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/reading-an-offer.html#pps-defs">Offer File Definitions</a> in the <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/billing-what-is.html">AWS Billing and Cost Management User Guide</a>.</p>
    async fn get_attribute_values(
        &self,
        input: GetAttributeValuesRequest,
    ) -> Result<GetAttributeValuesResponse, RusotoError<GetAttributeValuesError>>;

    /// <p>Returns a list of all products that match the filter criteria.</p>
    async fn get_products(
        &self,
        input: GetProductsRequest,
    ) -> Result<GetProductsResponse, RusotoError<GetProductsError>>;
}
/// A client for the AWS Pricing API.
#[derive(Clone)]
pub struct PricingClient {
    client: Client,
    region: region::Region,
}

impl PricingClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> PricingClient {
        PricingClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> PricingClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        PricingClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> PricingClient {
        PricingClient { client, region }
    }
}

#[async_trait]
impl Pricing for PricingClient {
    /// <p>Returns the metadata for one service or a list of the metadata for all services. Use this without a service code to get the service codes for all services. Use it with a service code, such as <code>AmazonEC2</code>, to get information specific to that service, such as the attribute names available for that service. For example, some of the attribute names available for EC2 are <code>volumeType</code>, <code>maxIopsVolume</code>, <code>operation</code>, <code>locationType</code>, and <code>instanceCapacity10xlarge</code>.</p>
    async fn describe_services(
        &self,
        input: DescribeServicesRequest,
    ) -> Result<DescribeServicesResponse, RusotoError<DescribeServicesError>> {
        let mut request = SignedRequest::new("POST", "pricing", &self.region, "/");
        request.set_endpoint_prefix("api.pricing".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSPriceListService.DescribeServices");
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
                .deserialize::<DescribeServicesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeServicesError::from_response(response))
        }
    }

    /// <p>Returns a list of attribute values. Attibutes are similar to the details in a Price List API offer file. For a list of available attributes, see <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/reading-an-offer.html#pps-defs">Offer File Definitions</a> in the <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/billing-what-is.html">AWS Billing and Cost Management User Guide</a>.</p>
    async fn get_attribute_values(
        &self,
        input: GetAttributeValuesRequest,
    ) -> Result<GetAttributeValuesResponse, RusotoError<GetAttributeValuesError>> {
        let mut request = SignedRequest::new("POST", "pricing", &self.region, "/");
        request.set_endpoint_prefix("api.pricing".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSPriceListService.GetAttributeValues");
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
                .deserialize::<GetAttributeValuesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetAttributeValuesError::from_response(response))
        }
    }

    /// <p>Returns a list of all products that match the filter criteria.</p>
    async fn get_products(
        &self,
        input: GetProductsRequest,
    ) -> Result<GetProductsResponse, RusotoError<GetProductsError>> {
        let mut request = SignedRequest::new("POST", "pricing", &self.region, "/");
        request.set_endpoint_prefix("api.pricing".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSPriceListService.GetProducts");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetProductsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetProductsError::from_response(response))
        }
    }
}
