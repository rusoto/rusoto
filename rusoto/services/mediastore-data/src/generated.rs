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
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteObjectRequest {
    /// <p>The path (including the file name) where the object is stored in the container. Format: &lt;folder name&gt;/&lt;folder name&gt;/&lt;file name&gt;</p>
    #[serde(rename = "Path")]
    pub path: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteObjectResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeObjectRequest {
    /// <p>The path (including the file name) where the object is stored in the container. Format: &lt;folder name&gt;/&lt;folder name&gt;/&lt;file name&gt;</p>
    #[serde(rename = "Path")]
    pub path: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeObjectResponse {
    /// <p>An optional <code>CacheControl</code> header that allows the caller to control the object's cache behavior. Headers can be passed in as specified in the HTTP at <a href="https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9">https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9</a>.</p> <p>Headers with a custom user-defined value are also accepted.</p>
    #[serde(rename = "CacheControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<String>,
    /// <p>The length of the object in bytes.</p>
    #[serde(rename = "ContentLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_length: Option<i64>,
    /// <p>The content type of the object.</p>
    #[serde(rename = "ContentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// <p>The ETag that represents a unique instance of the object.</p>
    #[serde(rename = "ETag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    /// <p>The date and time that the object was last modified.</p>
    #[serde(rename = "LastModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetObjectRequest {
    /// <p>The path (including the file name) where the object is stored in the container. Format: &lt;folder name&gt;/&lt;folder name&gt;/&lt;file name&gt;</p> <p>For example, to upload the file <code>mlaw.avi</code> to the folder path <code>premium\canada</code> in the container <code>movies</code>, enter the path <code>premium/canada/mlaw.avi</code>.</p> <p>Do not include the container name in this path.</p> <p>If the path includes any folders that don't exist yet, the service creates them. For example, suppose you have an existing <code>premium/usa</code> subfolder. If you specify <code>premium/canada</code>, the service creates a <code>canada</code> subfolder in the <code>premium</code> folder. You then have two subfolders, <code>usa</code> and <code>canada</code>, in the <code>premium</code> folder. </p> <p>There is no correlation between the path to the source and the path (folders) in the container in AWS Elemental MediaStore.</p> <p>For more information about folders and how they exist in a container, see the <a href="http://docs.aws.amazon.com/mediastore/latest/ug/">AWS Elemental MediaStore User Guide</a>.</p> <p>The file name is the name that is assigned to the file that you upload. The file can have the same name inside and outside of AWS Elemental MediaStore, or it can have the same name. The file name can include or omit an extension. </p>
    #[serde(rename = "Path")]
    pub path: String,
    /// <p>The range bytes of an object to retrieve. For more information about the <code>Range</code> header, see <a href="http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.35">http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.35</a>. AWS Elemental MediaStore ignores this header for partially uploaded objects that have streaming upload availability.</p>
    #[serde(rename = "Range")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetObjectResponse {
    /// <p>The bytes of the object. </p>
    pub body: Option<bytes::Bytes>,
    /// <p>An optional <code>CacheControl</code> header that allows the caller to control the object's cache behavior. Headers can be passed in as specified in the HTTP spec at <a href="https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9">https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9</a>.</p> <p>Headers with a custom user-defined value are also accepted.</p>
    pub cache_control: Option<String>,
    /// <p>The length of the object in bytes.</p>
    pub content_length: Option<i64>,
    /// <p>The range of bytes to retrieve.</p>
    pub content_range: Option<String>,
    /// <p>The content type of the object.</p>
    pub content_type: Option<String>,
    /// <p>The ETag that represents a unique instance of the object.</p>
    pub e_tag: Option<String>,
    /// <p>The date and time that the object was last modified.</p>
    pub last_modified: Option<f64>,
    /// <p>The HTML status code of the request. Status codes ranging from 200 to 299 indicate success. All other status codes indicate the type of error that occurred.</p>
    pub status_code: i64,
}

/// <p>A metadata entry for a folder or object.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Item {
    /// <p>The length of the item in bytes.</p>
    #[serde(rename = "ContentLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_length: Option<i64>,
    /// <p>The content type of the item.</p>
    #[serde(rename = "ContentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// <p>The ETag that represents a unique instance of the item.</p>
    #[serde(rename = "ETag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    /// <p>The date and time that the item was last modified.</p>
    #[serde(rename = "LastModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<f64>,
    /// <p>The name of the item.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The item type (folder or object).</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListItemsRequest {
    /// <p>The maximum number of results to return per API request. For example, you submit a <code>ListItems</code> request with <code>MaxResults</code> set at 500. Although 2,000 items match your request, the service returns no more than the first 500 items. (The service also returns a <code>NextToken</code> value that you can use to fetch the next batch of results.) The service might return fewer results than the <code>MaxResults</code> value.</p> <p>If <code>MaxResults</code> is not included in the request, the service defaults to pagination with a maximum of 1,000 results per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that identifies which batch of results that you want to see. For example, you submit a <code>ListItems</code> request with <code>MaxResults</code> set at 500. The service returns the first batch of results (up to 500) and a <code>NextToken</code> value. To see the next batch of results, you can submit the <code>ListItems</code> request a second time and specify the <code>NextToken</code> value.</p> <p>Tokens expire after 15 minutes.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The path in the container from which to retrieve items. Format: &lt;folder name&gt;/&lt;folder name&gt;/&lt;file name&gt;</p>
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListItemsResponse {
    /// <p>The metadata entries for the folders and objects at the requested path.</p>
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Item>>,
    /// <p>The token that can be used in a request to view the next set of results. For example, you submit a <code>ListItems</code> request that matches 2,000 items with <code>MaxResults</code> set at 500. The service returns the first batch of results (up to 500) and a <code>NextToken</code> value that can be used to fetch the next batch of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutObjectRequest {
    /// <p>The bytes to be stored. </p>
    #[serde(rename = "Body")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub body: bytes::Bytes,
    /// <p>An optional <code>CacheControl</code> header that allows the caller to control the object's cache behavior. Headers can be passed in as specified in the HTTP at <a href="https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9">https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9</a>.</p> <p>Headers with a custom user-defined value are also accepted.</p>
    #[serde(rename = "CacheControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<String>,
    /// <p>The content type of the object.</p>
    #[serde(rename = "ContentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// <p>The path (including the file name) where the object is stored in the container. Format: &lt;folder name&gt;/&lt;folder name&gt;/&lt;file name&gt;</p> <p>For example, to upload the file <code>mlaw.avi</code> to the folder path <code>premium\canada</code> in the container <code>movies</code>, enter the path <code>premium/canada/mlaw.avi</code>.</p> <p>Do not include the container name in this path.</p> <p>If the path includes any folders that don't exist yet, the service creates them. For example, suppose you have an existing <code>premium/usa</code> subfolder. If you specify <code>premium/canada</code>, the service creates a <code>canada</code> subfolder in the <code>premium</code> folder. You then have two subfolders, <code>usa</code> and <code>canada</code>, in the <code>premium</code> folder. </p> <p>There is no correlation between the path to the source and the path (folders) in the container in AWS Elemental MediaStore.</p> <p>For more information about folders and how they exist in a container, see the <a href="http://docs.aws.amazon.com/mediastore/latest/ug/">AWS Elemental MediaStore User Guide</a>.</p> <p>The file name is the name that is assigned to the file that you upload. The file can have the same name inside and outside of AWS Elemental MediaStore, or it can have the same name. The file name can include or omit an extension. </p>
    #[serde(rename = "Path")]
    pub path: String,
    /// <p>Indicates the storage class of a <code>Put</code> request. Defaults to high-performance temporal storage class, and objects are persisted into durable storage shortly after being received.</p>
    #[serde(rename = "StorageClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<String>,
    /// <p>Indicates the availability of an object while it is still uploading. If the value is set to <code>streaming</code>, the object is available for downloading after some initial buffering but before the object is uploaded completely. If the value is set to <code>standard</code>, the object is available for downloading only when it is uploaded completely. The default value for this header is <code>standard</code>.</p> <p>To use this header, you must also set the HTTP <code>Transfer-Encoding</code> header to <code>chunked</code>.</p>
    #[serde(rename = "UploadAvailability")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_availability: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutObjectResponse {
    /// <p>The SHA256 digest of the object that is persisted.</p>
    #[serde(rename = "ContentSHA256")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_sha256: Option<String>,
    /// <p>Unique identifier of the object in the container.</p>
    #[serde(rename = "ETag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    /// <p>The storage class where the object was persisted. The class should be “Temporal”.</p>
    #[serde(rename = "StorageClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<String>,
}

/// Errors returned by DeleteObject
#[derive(Debug, PartialEq)]
pub enum DeleteObjectError {
    /// <p>The specified container was not found for the specified account.</p>
    ContainerNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// <p>Could not perform an operation on an object that does not exist.</p>
    ObjectNotFound(String),
}

impl DeleteObjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteObjectError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ContainerNotFoundException" => {
                    return RusotoError::Service(DeleteObjectError::ContainerNotFound(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DeleteObjectError::InternalServerError(err.msg))
                }
                "ObjectNotFoundException" => {
                    return RusotoError::Service(DeleteObjectError::ObjectNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteObjectError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteObjectError::ContainerNotFound(ref cause) => write!(f, "{}", cause),
            DeleteObjectError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteObjectError::ObjectNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteObjectError {}
/// Errors returned by DescribeObject
#[derive(Debug, PartialEq)]
pub enum DescribeObjectError {
    /// <p>The specified container was not found for the specified account.</p>
    ContainerNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// <p>Could not perform an operation on an object that does not exist.</p>
    ObjectNotFound(String),
}

impl DescribeObjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeObjectError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ContainerNotFoundException" => {
                    return RusotoError::Service(DescribeObjectError::ContainerNotFound(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DescribeObjectError::InternalServerError(err.msg))
                }
                "ObjectNotFoundException" => {
                    return RusotoError::Service(DescribeObjectError::ObjectNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeObjectError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeObjectError::ContainerNotFound(ref cause) => write!(f, "{}", cause),
            DescribeObjectError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeObjectError::ObjectNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeObjectError {}
/// Errors returned by GetObject
#[derive(Debug, PartialEq)]
pub enum GetObjectError {
    /// <p>The specified container was not found for the specified account.</p>
    ContainerNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
    /// <p>Could not perform an operation on an object that does not exist.</p>
    ObjectNotFound(String),
    /// <p>The requested content range is not valid.</p>
    RequestedRangeNotSatisfiable(String),
}

impl GetObjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetObjectError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ContainerNotFoundException" => {
                    return RusotoError::Service(GetObjectError::ContainerNotFound(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(GetObjectError::InternalServerError(err.msg))
                }
                "ObjectNotFoundException" => {
                    return RusotoError::Service(GetObjectError::ObjectNotFound(err.msg))
                }
                "RequestedRangeNotSatisfiableException" => {
                    return RusotoError::Service(GetObjectError::RequestedRangeNotSatisfiable(
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
impl fmt::Display for GetObjectError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetObjectError::ContainerNotFound(ref cause) => write!(f, "{}", cause),
            GetObjectError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetObjectError::ObjectNotFound(ref cause) => write!(f, "{}", cause),
            GetObjectError::RequestedRangeNotSatisfiable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetObjectError {}
/// Errors returned by ListItems
#[derive(Debug, PartialEq)]
pub enum ListItemsError {
    /// <p>The specified container was not found for the specified account.</p>
    ContainerNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
}

impl ListItemsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListItemsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ContainerNotFoundException" => {
                    return RusotoError::Service(ListItemsError::ContainerNotFound(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(ListItemsError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListItemsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListItemsError::ContainerNotFound(ref cause) => write!(f, "{}", cause),
            ListItemsError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListItemsError {}
/// Errors returned by PutObject
#[derive(Debug, PartialEq)]
pub enum PutObjectError {
    /// <p>The specified container was not found for the specified account.</p>
    ContainerNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(String),
}

impl PutObjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutObjectError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ContainerNotFoundException" => {
                    return RusotoError::Service(PutObjectError::ContainerNotFound(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(PutObjectError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutObjectError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutObjectError::ContainerNotFound(ref cause) => write!(f, "{}", cause),
            PutObjectError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutObjectError {}
/// Trait representing the capabilities of the MediaStore Data API. MediaStore Data clients implement this trait.
#[async_trait]
pub trait MediaStoreData {
    /// <p>Deletes an object at the specified path.</p>
    async fn delete_object(
        &self,
        input: DeleteObjectRequest,
    ) -> Result<DeleteObjectResponse, RusotoError<DeleteObjectError>>;

    /// <p>Gets the headers for an object at the specified path.</p>
    async fn describe_object(
        &self,
        input: DescribeObjectRequest,
    ) -> Result<DescribeObjectResponse, RusotoError<DescribeObjectError>>;

    /// <p>Downloads the object at the specified path. If the object’s upload availability is set to <code>streaming</code>, AWS Elemental MediaStore downloads the object even if it’s still uploading the object.</p>
    async fn get_object(
        &self,
        input: GetObjectRequest,
    ) -> Result<GetObjectResponse, RusotoError<GetObjectError>>;

    /// <p>Provides a list of metadata entries about folders and objects in the specified folder.</p>
    async fn list_items(
        &self,
        input: ListItemsRequest,
    ) -> Result<ListItemsResponse, RusotoError<ListItemsError>>;

    /// <p>Uploads an object to the specified path. Object sizes are limited to 25 MB for standard upload availability and 10 MB for streaming upload availability.</p>
    async fn put_object(
        &self,
        input: PutObjectRequest,
    ) -> Result<PutObjectResponse, RusotoError<PutObjectError>>;
}
/// A client for the MediaStore Data API.
#[derive(Clone)]
pub struct MediaStoreDataClient {
    client: Client,
    region: region::Region,
}

impl MediaStoreDataClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> MediaStoreDataClient {
        MediaStoreDataClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> MediaStoreDataClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        MediaStoreDataClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> MediaStoreDataClient {
        MediaStoreDataClient { client, region }
    }
}

#[async_trait]
impl MediaStoreData for MediaStoreDataClient {
    /// <p>Deletes an object at the specified path.</p>
    async fn delete_object(
        &self,
        input: DeleteObjectRequest,
    ) -> Result<DeleteObjectResponse, RusotoError<DeleteObjectError>> {
        let request_uri = format!("/{path}", path = input.path);

        let mut request = SignedRequest::new("DELETE", "mediastore", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("data.mediastore".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteObjectResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteObjectError::from_response(response))
        }
    }

    /// <p>Gets the headers for an object at the specified path.</p>
    async fn describe_object(
        &self,
        input: DescribeObjectRequest,
    ) -> Result<DescribeObjectResponse, RusotoError<DescribeObjectError>> {
        let request_uri = format!("/{path}", path = input.path);

        let mut request = SignedRequest::new("HEAD", "mediastore", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("data.mediastore".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeObjectResponse, _>()?;
            if let Some(cache_control) = response.headers.get("Cache-Control") {
                let value = cache_control.to_owned();
                result.cache_control = Some(value)
            };
            if let Some(content_length) = response.headers.get("Content-Length") {
                let value = content_length.to_owned();
                result.content_length = Some(value.parse::<i64>().unwrap())
            };
            if let Some(content_type) = response.headers.get("Content-Type") {
                let value = content_type.to_owned();
                result.content_type = Some(value)
            };
            if let Some(e_tag) = response.headers.get("ETag") {
                let value = e_tag.to_owned();
                result.e_tag = Some(value)
            };
            if let Some(last_modified) = response.headers.get("Last-Modified") {
                let value = last_modified.to_owned();
                result.last_modified = Some(value)
            };

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeObjectError::from_response(response))
        }
    }

    /// <p>Downloads the object at the specified path. If the object’s upload availability is set to <code>streaming</code>, AWS Elemental MediaStore downloads the object even if it’s still uploading the object.</p>
    async fn get_object(
        &self,
        input: GetObjectRequest,
    ) -> Result<GetObjectResponse, RusotoError<GetObjectError>> {
        let request_uri = format!("/{path}", path = input.path);

        let mut request = SignedRequest::new("GET", "mediastore", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("data.mediastore".to_string());

        if let Some(ref range) = input.range {
            request.add_header("Range", &range.to_string());
        }

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;

            let mut result = GetObjectResponse::default();
            result.body = response.body;

            if let Some(cache_control) = response.headers.get("Cache-Control") {
                let value = cache_control.to_owned();
                result.cache_control = Some(value)
            };
            if let Some(content_length) = response.headers.get("Content-Length") {
                let value = content_length.to_owned();
                result.content_length = Some(value.parse::<i64>().unwrap())
            };
            if let Some(content_range) = response.headers.get("Content-Range") {
                let value = content_range.to_owned();
                result.content_range = Some(value)
            };
            if let Some(content_type) = response.headers.get("Content-Type") {
                let value = content_type.to_owned();
                result.content_type = Some(value)
            };
            if let Some(e_tag) = response.headers.get("ETag") {
                let value = e_tag.to_owned();
                result.e_tag = Some(value)
            };
            if let Some(last_modified) = response.headers.get("Last-Modified") {
                let value = last_modified.to_owned();
                result.last_modified = Some(value)
            };
            result.status_code = Some(response.status.as_u16());
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetObjectError::from_response(response))
        }
    }

    /// <p>Provides a list of metadata entries about folders and objects in the specified folder.</p>
    async fn list_items(
        &self,
        input: ListItemsRequest,
    ) -> Result<ListItemsResponse, RusotoError<ListItemsError>> {
        let request_uri = "/";

        let mut request = SignedRequest::new("GET", "mediastore", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("data.mediastore".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        if let Some(ref x) = input.path {
            params.put("Path", x);
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
                .deserialize::<ListItemsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListItemsError::from_response(response))
        }
    }

    /// <p>Uploads an object to the specified path. Object sizes are limited to 25 MB for standard upload availability and 10 MB for streaming upload availability.</p>
    async fn put_object(
        &self,
        input: PutObjectRequest,
    ) -> Result<PutObjectResponse, RusotoError<PutObjectError>> {
        let request_uri = format!("/{path}", path = input.path);

        let mut request = SignedRequest::new("PUT", "mediastore", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("data.mediastore".to_string());
        let encoded = Some(input.body.to_owned());
        request.set_payload(encoded);

        if let Some(ref cache_control) = input.cache_control {
            request.add_header("Cache-Control", &cache_control.to_string());
        }

        if let Some(ref content_type) = input.content_type {
            request.add_header("Content-Type", &content_type.to_string());
        }

        if let Some(ref storage_class) = input.storage_class {
            request.add_header("x-amz-storage-class", &storage_class.to_string());
        }

        if let Some(ref upload_availability) = input.upload_availability {
            request.add_header(
                "x-amz-upload-availability",
                &upload_availability.to_string(),
            );
        }

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutObjectResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutObjectError::from_response(response))
        }
    }
}
