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

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
/// <p> The details of an Elastic Inference Accelerator type. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AcceleratorType {
    /// <p> The name of the Elastic Inference Accelerator type. </p>
    #[serde(rename = "acceleratorTypeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerator_type_name: Option<String>,
    /// <p> The memory information of the Elastic Inference Accelerator type. </p>
    #[serde(rename = "memoryInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_info: Option<MemoryInfo>,
    /// <p> The throughput information of the Elastic Inference Accelerator type. </p>
    #[serde(rename = "throughputInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput_info: Option<Vec<KeyValuePair>>,
}

/// <p> The offering for an Elastic Inference Accelerator type. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AcceleratorTypeOffering {
    /// <p> The name of the Elastic Inference Accelerator type. </p>
    #[serde(rename = "acceleratorType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerator_type: Option<String>,
    /// <p> The location for the offering. It will return either the region, availability zone or availability zone id for the offering depending on the locationType value. </p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// <p> The location type for the offering. It can assume the following values: region: defines that the offering is at the regional level. availability-zone: defines that the offering is at the availability zone level. availability-zone-id: defines that the offering is at the availability zone level, defined by the availability zone id. </p>
    #[serde(rename = "locationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeAcceleratorOfferingsRequest {
    /// <p> The list of accelerator types to describe. </p>
    #[serde(rename = "acceleratorTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerator_types: Option<Vec<String>>,
    /// <p> The location type that you want to describe accelerator type offerings for. It can assume the following values: region: will return the accelerator type offering at the regional level. availability-zone: will return the accelerator type offering at the availability zone level. availability-zone-id: will return the accelerator type offering at the availability zone level returning the availability zone id. </p>
    #[serde(rename = "locationType")]
    pub location_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAcceleratorOfferingsResponse {
    /// <p> The list of accelerator type offerings for a specific location. </p>
    #[serde(rename = "acceleratorTypeOfferings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerator_type_offerings: Option<Vec<AcceleratorTypeOffering>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeAcceleratorTypesRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAcceleratorTypesResponse {
    /// <p> The available accelerator types. </p>
    #[serde(rename = "acceleratorTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerator_types: Option<Vec<AcceleratorType>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeAcceleratorsRequest {
    /// <p> The IDs of the accelerators to describe. </p>
    #[serde(rename = "acceleratorIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerator_ids: Option<Vec<String>>,
    /// <p> One or more filters. Filter names and values are case-sensitive. Valid filter names are: accelerator-types: can provide a list of accelerator type names to filter for. instance-id: can provide a list of EC2 instance ids to filter for. </p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p> The total number of items to return in the command's output. If the total number of items available is more than the value specified, a NextToken is provided in the command's output. To resume pagination, provide the NextToken value in the starting-token argument of a subsequent command. Do not use the NextToken response element directly outside of the AWS CLI. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p> A token to specify where to start paginating. This is the NextToken from a previously truncated response. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAcceleratorsResponse {
    /// <p> The details of the Elastic Inference Accelerators. </p>
    #[serde(rename = "acceleratorSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerator_set: Option<Vec<ElasticInferenceAccelerator>>,
    /// <p> A token to specify where to start paginating. This is the NextToken from a previously truncated response. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p> The details of an Elastic Inference Accelerator. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ElasticInferenceAccelerator {
    /// <p> The health of the Elastic Inference Accelerator. </p>
    #[serde(rename = "acceleratorHealth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerator_health: Option<ElasticInferenceAcceleratorHealth>,
    /// <p> The ID of the Elastic Inference Accelerator. </p>
    #[serde(rename = "acceleratorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerator_id: Option<String>,
    /// <p> The type of the Elastic Inference Accelerator. </p>
    #[serde(rename = "acceleratorType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerator_type: Option<String>,
    /// <p> The ARN of the resource that the Elastic Inference Accelerator is attached to. </p>
    #[serde(rename = "attachedResource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_resource: Option<String>,
    /// <p> The availability zone where the Elastic Inference Accelerator is present. </p>
    #[serde(rename = "availabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
}

/// <p> The health details of an Elastic Inference Accelerator. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ElasticInferenceAcceleratorHealth {
    /// <p> The health status of the Elastic Inference Accelerator. </p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p> A filter expression for the Elastic Inference Accelerator list. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Filter {
    /// <p> The filter name for the Elastic Inference Accelerator list. It can assume the following values: accelerator-type: the type of Elastic Inference Accelerator to filter for. instance-id: an EC2 instance id to filter for. </p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p> The values for the filter of the Elastic Inference Accelerator list. </p>
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// <p> A throughput entry for an Elastic Inference Accelerator type. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct KeyValuePair {
    /// <p> The throughput value of the Elastic Inference Accelerator type. It can assume the following values: TFLOPS16bit: the throughput expressed in 16bit TeraFLOPS. TFLOPS32bit: the throughput expressed in 32bit TeraFLOPS. </p>
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p> The throughput value of the Elastic Inference Accelerator type. </p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p> The ARN of the Elastic Inference Accelerator to list the tags for. </p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResult {
    /// <p> The tags of the Elastic Inference Accelerator. </p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p> The memory information of an Elastic Inference Accelerator type. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MemoryInfo {
    /// <p> The size in mebibytes of the Elastic Inference Accelerator type. </p>
    #[serde(rename = "sizeInMiB")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_mi_b: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p> The ARN of the Elastic Inference Accelerator to tag. </p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p> The tags to add to the Elastic Inference Accelerator. </p>
    #[serde(rename = "tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p> The ARN of the Elastic Inference Accelerator to untag. </p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p> The list of tags to remove from the Elastic Inference Accelerator. </p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResult {}

/// Errors returned by DescribeAcceleratorOfferings
#[derive(Debug, PartialEq)]
pub enum DescribeAcceleratorOfferingsError {
    /// <p> Raised when a malformed input has been provided to the API. </p>
    BadRequest(String),
    /// <p> Raised when an unexpected error occurred during request processing. </p>
    InternalServer(String),
    /// <p> Raised when the requested resource cannot be found. </p>
    ResourceNotFound(String),
}

impl DescribeAcceleratorOfferingsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeAcceleratorOfferingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DescribeAcceleratorOfferingsError::BadRequest(
                        err.msg,
                    ))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DescribeAcceleratorOfferingsError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeAcceleratorOfferingsError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeAcceleratorOfferingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeAcceleratorOfferingsError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeAcceleratorOfferingsError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeAcceleratorOfferingsError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeAcceleratorOfferingsError {}
/// Errors returned by DescribeAcceleratorTypes
#[derive(Debug, PartialEq)]
pub enum DescribeAcceleratorTypesError {
    /// <p> Raised when an unexpected error occurred during request processing. </p>
    InternalServer(String),
}

impl DescribeAcceleratorTypesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAcceleratorTypesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeAcceleratorTypesError::InternalServer(
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
impl fmt::Display for DescribeAcceleratorTypesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeAcceleratorTypesError::InternalServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeAcceleratorTypesError {}
/// Errors returned by DescribeAccelerators
#[derive(Debug, PartialEq)]
pub enum DescribeAcceleratorsError {
    /// <p> Raised when a malformed input has been provided to the API. </p>
    BadRequest(String),
    /// <p> Raised when an unexpected error occurred during request processing. </p>
    InternalServer(String),
    /// <p> Raised when the requested resource cannot be found. </p>
    ResourceNotFound(String),
}

impl DescribeAcceleratorsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAcceleratorsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DescribeAcceleratorsError::BadRequest(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DescribeAcceleratorsError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeAcceleratorsError::ResourceNotFound(
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
impl fmt::Display for DescribeAcceleratorsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeAcceleratorsError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeAcceleratorsError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeAcceleratorsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeAcceleratorsError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p> Raised when a malformed input has been provided to the API. </p>
    BadRequest(String),
    /// <p> Raised when an unexpected error occurred during request processing. </p>
    InternalServer(String),
    /// <p> Raised when the requested resource cannot be found. </p>
    ResourceNotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListTagsForResourceError::BadRequest(err.msg))
                }
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
            ListTagsForResourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p> Raised when a malformed input has been provided to the API. </p>
    BadRequest(String),
    /// <p> Raised when an unexpected error occurred during request processing. </p>
    InternalServer(String),
    /// <p> Raised when the requested resource cannot be found. </p>
    ResourceNotFound(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(TagResourceError::BadRequest(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(TagResourceError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
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
            TagResourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            TagResourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p> Raised when a malformed input has been provided to the API. </p>
    BadRequest(String),
    /// <p> Raised when an unexpected error occurred during request processing. </p>
    InternalServer(String),
    /// <p> Raised when the requested resource cannot be found. </p>
    ResourceNotFound(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UntagResourceError::BadRequest(err.msg))
                }
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
            UntagResourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Trait representing the capabilities of the Amazon Elastic Inference API. Amazon Elastic Inference clients implement this trait.
#[async_trait]
pub trait ElasticInference {
    /// <p> Describes the locations in which a given accelerator type or set of types is present in a given region. </p>
    async fn describe_accelerator_offerings(
        &self,
        input: DescribeAcceleratorOfferingsRequest,
    ) -> Result<DescribeAcceleratorOfferingsResponse, RusotoError<DescribeAcceleratorOfferingsError>>;

    /// <p> Describes the accelerator types available in a given region, as well as their characteristics, such as memory and throughput. </p>
    async fn describe_accelerator_types(
        &self,
    ) -> Result<DescribeAcceleratorTypesResponse, RusotoError<DescribeAcceleratorTypesError>>;

    /// <p> Describes information over a provided set of accelerators belonging to an account. </p>
    async fn describe_accelerators(
        &self,
        input: DescribeAcceleratorsRequest,
    ) -> Result<DescribeAcceleratorsResponse, RusotoError<DescribeAcceleratorsError>>;

    /// <p> Returns all tags of an Elastic Inference Accelerator. </p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResult, RusotoError<ListTagsForResourceError>>;

    /// <p> Adds the specified tags to an Elastic Inference Accelerator. </p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResult, RusotoError<TagResourceError>>;

    /// <p> Removes the specified tags from an Elastic Inference Accelerator. </p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResult, RusotoError<UntagResourceError>>;
}
/// A client for the Amazon Elastic Inference API.
#[derive(Clone)]
pub struct ElasticInferenceClient {
    client: Client,
    region: region::Region,
}

impl ElasticInferenceClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ElasticInferenceClient {
        ElasticInferenceClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ElasticInferenceClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        ElasticInferenceClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> ElasticInferenceClient {
        ElasticInferenceClient { client, region }
    }
}

#[async_trait]
impl ElasticInference for ElasticInferenceClient {
    /// <p> Describes the locations in which a given accelerator type or set of types is present in a given region. </p>
    async fn describe_accelerator_offerings(
        &self,
        input: DescribeAcceleratorOfferingsRequest,
    ) -> Result<DescribeAcceleratorOfferingsResponse, RusotoError<DescribeAcceleratorOfferingsError>>
    {
        let request_uri = "/describe-accelerator-offerings";

        let mut request =
            SignedRequest::new("POST", "elastic-inference", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.elastic-inference".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeAcceleratorOfferingsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeAcceleratorOfferingsError::from_response(response))
        }
    }

    /// <p> Describes the accelerator types available in a given region, as well as their characteristics, such as memory and throughput. </p>
    async fn describe_accelerator_types(
        &self,
    ) -> Result<DescribeAcceleratorTypesResponse, RusotoError<DescribeAcceleratorTypesError>> {
        let request_uri = "/describe-accelerator-types";

        let mut request =
            SignedRequest::new("GET", "elastic-inference", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.elastic-inference".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeAcceleratorTypesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeAcceleratorTypesError::from_response(response))
        }
    }

    /// <p> Describes information over a provided set of accelerators belonging to an account. </p>
    async fn describe_accelerators(
        &self,
        input: DescribeAcceleratorsRequest,
    ) -> Result<DescribeAcceleratorsResponse, RusotoError<DescribeAcceleratorsError>> {
        let request_uri = "/describe-accelerators";

        let mut request =
            SignedRequest::new("POST", "elastic-inference", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.elastic-inference".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeAcceleratorsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeAcceleratorsError::from_response(response))
        }
    }

    /// <p> Returns all tags of an Elastic Inference Accelerator. </p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResult, RusotoError<ListTagsForResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request =
            SignedRequest::new("GET", "elastic-inference", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.elastic-inference".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTagsForResourceResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p> Adds the specified tags to an Elastic Inference Accelerator. </p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResult, RusotoError<TagResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request =
            SignedRequest::new("POST", "elastic-inference", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.elastic-inference".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<TagResourceResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p> Removes the specified tags from an Elastic Inference Accelerator. </p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResult, RusotoError<UntagResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request =
            SignedRequest::new("DELETE", "elastic-inference", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.elastic-inference".to_string());

        let mut params = Params::new();
        for item in input.tag_keys.iter() {
            params.put("tagKeys", item);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UntagResourceResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }
}
