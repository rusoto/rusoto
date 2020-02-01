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
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CloseTunnelRequest {
    /// <p>When set to true, AWS IoT Secure Tunneling deletes the tunnel data immediately.</p>
    #[serde(rename = "delete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<bool>,
    /// <p>The ID of the tunnel to close.</p>
    #[serde(rename = "tunnelId")]
    pub tunnel_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CloseTunnelResponse {}

/// <p>The state of a connection.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConnectionState {
    /// <p>The last time the connection status was updated.</p>
    #[serde(rename = "lastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    /// <p>The connection status of the tunnel. Valid values are <code>CONNECTED</code> and <code>DISCONNECTED</code>.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeTunnelRequest {
    /// <p>The tunnel to describe.</p>
    #[serde(rename = "tunnelId")]
    pub tunnel_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeTunnelResponse {
    /// <p>The tunnel being described.</p>
    #[serde(rename = "tunnel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tunnel: Option<Tunnel>,
}

/// <p>The destination configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinationConfig {
    /// <p>A list of service names that identity the target application. Currently, you can only specify a single name. The AWS IoT client running on the destination device reads this value and uses it to look up a port or an IP address and a port. The AWS IoT client instantiates the local proxy which uses this information to connect to the destination application.</p>
    #[serde(rename = "services")]
    pub services: Vec<String>,
    /// <p>The name of the IoT thing to which you want to connect.</p>
    #[serde(rename = "thingName")]
    pub thing_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The resource ARN.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>The tags for the specified resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTunnelsRequest {
    /// <p>The maximum number of results to return at once.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token to retrieve the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the IoT thing associated with the destination device.</p>
    #[serde(rename = "thingName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTunnelsResponse {
    /// <p>A token to used to retrieve the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A short description of the tunnels in an AWS account.</p>
    #[serde(rename = "tunnelSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tunnel_summaries: Option<Vec<TunnelSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct OpenTunnelRequest {
    /// <p>A short text description of the tunnel. </p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The destination configuration for the OpenTunnel request.</p>
    #[serde(rename = "destinationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_config: Option<DestinationConfig>,
    /// <p>A collection of tag metadata.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>Timeout configuration for a tunnel.</p>
    #[serde(rename = "timeoutConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_config: Option<TimeoutConfig>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OpenTunnelResponse {
    /// <p>The access token the destination local proxy uses to connect to AWS IoT Secure Tunneling.</p>
    #[serde(rename = "destinationAccessToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_access_token: Option<String>,
    /// <p>The access token the source local proxy uses to connect to AWS IoT Secure Tunneling.</p>
    #[serde(rename = "sourceAccessToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_access_token: Option<String>,
    /// <p>The Amazon Resource Name for the tunnel. The tunnel ARN format is <code>arn:aws:tunnel:&lt;region&gt;:&lt;account-id&gt;:tunnel/&lt;tunnel-id&gt;</code> </p>
    #[serde(rename = "tunnelArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tunnel_arn: Option<String>,
    /// <p>A unique alpha-numeric tunnel ID.</p>
    #[serde(rename = "tunnelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tunnel_id: Option<String>,
}

/// <p>An arbitary key/value pair used to add searchable metadata to secure tunnel resources.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>The key of the tag.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>The value of the tag.</p>
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The ARN of the resource.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The tags for the resource.</p>
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

/// <p>Tunnel timeout configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeoutConfig {
    /// <p>The maximum amount of time (in minutes) a tunnel can remain open. If not specified, maxLifetimeTimeoutMinutes defaults to 720 minutes. Valid values are from 1 minute to 12 hours (720 minutes) </p>
    #[serde(rename = "maxLifetimeTimeoutMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_lifetime_timeout_minutes: Option<i64>,
}

/// <p>A connection between a source computer and a destination device.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Tunnel {
    /// <p>The time when the tunnel was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>A description of the tunnel.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The destination configuration that specifies the thing name of the destination device and a service name that the local proxy uses to connect to the destination application.</p>
    #[serde(rename = "destinationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_config: Option<DestinationConfig>,
    /// <p>The connection state of the destination application.</p>
    #[serde(rename = "destinationConnectionState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_connection_state: Option<ConnectionState>,
    /// <p>The last time the tunnel was updated.</p>
    #[serde(rename = "lastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    /// <p>The connection state of the source application.</p>
    #[serde(rename = "sourceConnectionState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_connection_state: Option<ConnectionState>,
    /// <p>The status of a tunnel. Valid values are: Open and Closed.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A list of tag metadata associated with the secure tunnel.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>Timeout configuration for the tunnel.</p>
    #[serde(rename = "timeoutConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_config: Option<TimeoutConfig>,
    /// <p>The Amazon Resource Name (ARN) of a tunnel. The tunnel ARN format is <code>arn:aws:tunnel:&lt;region&gt;:&lt;account-id&gt;:tunnel/&lt;tunnel-id&gt;</code> </p>
    #[serde(rename = "tunnelArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tunnel_arn: Option<String>,
    /// <p>A unique alpha-numeric ID that identifies a tunnel.</p>
    #[serde(rename = "tunnelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tunnel_id: Option<String>,
}

/// <p>Information about the tunnel.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TunnelSummary {
    /// <p>The time the tunnel was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>A description of the tunnel.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The time the tunnel was last updated.</p>
    #[serde(rename = "lastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    /// <p>The status of a tunnel. Valid values are: Open and Closed.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The Amazon Resource Name of the tunnel. The tunnel ARN format is <code>arn:aws:tunnel:&lt;region&gt;:&lt;account-id&gt;:tunnel/&lt;tunnel-id&gt;</code> </p>
    #[serde(rename = "tunnelArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tunnel_arn: Option<String>,
    /// <p>The unique alpha-numeric identifier for the tunnel.</p>
    #[serde(rename = "tunnelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tunnel_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The resource ARN.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The keys of the tags to remove.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

/// Errors returned by CloseTunnel
#[derive(Debug, PartialEq)]
pub enum CloseTunnelError {
    /// <p>Thrown when an operation is attempted on a resource that does not exist.</p>
    ResourceNotFound(String),
}

impl CloseTunnelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CloseTunnelError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CloseTunnelError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CloseTunnelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CloseTunnelError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CloseTunnelError {}
/// Errors returned by DescribeTunnel
#[derive(Debug, PartialEq)]
pub enum DescribeTunnelError {
    /// <p>Thrown when an operation is attempted on a resource that does not exist.</p>
    ResourceNotFound(String),
}

impl DescribeTunnelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeTunnelError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeTunnelError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeTunnelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeTunnelError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeTunnelError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>Thrown when an operation is attempted on a resource that does not exist.</p>
    ResourceNotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
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
            ListTagsForResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by ListTunnels
#[derive(Debug, PartialEq)]
pub enum ListTunnelsError {}

impl ListTunnelsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTunnelsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTunnelsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListTunnelsError {}
/// Errors returned by OpenTunnel
#[derive(Debug, PartialEq)]
pub enum OpenTunnelError {
    /// <p>Thrown when a tunnel limit is exceeded.</p>
    LimitExceeded(String),
}

impl OpenTunnelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<OpenTunnelError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "LimitExceededException" => {
                    return RusotoError::Service(OpenTunnelError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for OpenTunnelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OpenTunnelError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for OpenTunnelError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>Thrown when an operation is attempted on a resource that does not exist.</p>
    ResourceNotFound(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
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
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>Thrown when an operation is attempted on a resource that does not exist.</p>
    ResourceNotFound(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
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
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Trait representing the capabilities of the AWS IoT Secure Tunneling API. AWS IoT Secure Tunneling clients implement this trait.
#[async_trait]
pub trait IoTSecureTunneling {
    /// <p>Closes a tunnel identified by the unique tunnel id. When a <code>CloseTunnel</code> request is received, we close the WebSocket connections between the client and proxy server so no data can be transmitted.</p>
    async fn close_tunnel(
        &self,
        input: CloseTunnelRequest,
    ) -> Result<CloseTunnelResponse, RusotoError<CloseTunnelError>>;

    /// <p>Gets information about a tunnel identified by the unique tunnel id.</p>
    async fn describe_tunnel(
        &self,
        input: DescribeTunnelRequest,
    ) -> Result<DescribeTunnelResponse, RusotoError<DescribeTunnelError>>;

    /// <p>Lists the tags for the specified resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>List all tunnels for an AWS account. Tunnels are listed by creation time in descending order, newer tunnels will be listed before older tunnels.</p>
    async fn list_tunnels(
        &self,
        input: ListTunnelsRequest,
    ) -> Result<ListTunnelsResponse, RusotoError<ListTunnelsError>>;

    /// <p>Creates a new tunnel, and returns two client access tokens for clients to use to connect to the AWS IoT Secure Tunneling proxy server. .</p>
    async fn open_tunnel(
        &self,
        input: OpenTunnelRequest,
    ) -> Result<OpenTunnelResponse, RusotoError<OpenTunnelError>>;

    /// <p>A resource tag.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Removes a tag from a resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;
}
/// A client for the AWS IoT Secure Tunneling API.
#[derive(Clone)]
pub struct IoTSecureTunnelingClient {
    client: Client,
    region: region::Region,
}

impl IoTSecureTunnelingClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> IoTSecureTunnelingClient {
        IoTSecureTunnelingClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> IoTSecureTunnelingClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        IoTSecureTunnelingClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> IoTSecureTunnelingClient {
        IoTSecureTunnelingClient { client, region }
    }
}

#[async_trait]
impl IoTSecureTunneling for IoTSecureTunnelingClient {
    /// <p>Closes a tunnel identified by the unique tunnel id. When a <code>CloseTunnel</code> request is received, we close the WebSocket connections between the client and proxy server so no data can be transmitted.</p>
    async fn close_tunnel(
        &self,
        input: CloseTunnelRequest,
    ) -> Result<CloseTunnelResponse, RusotoError<CloseTunnelError>> {
        let mut request = SignedRequest::new("POST", "IoTSecuredTunneling", &self.region, "/");
        request.set_endpoint_prefix("api.tunneling.iot".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "IoTSecuredTunneling.CloseTunnel");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CloseTunnelResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CloseTunnelError::from_response(response))
        }
    }

    /// <p>Gets information about a tunnel identified by the unique tunnel id.</p>
    async fn describe_tunnel(
        &self,
        input: DescribeTunnelRequest,
    ) -> Result<DescribeTunnelResponse, RusotoError<DescribeTunnelError>> {
        let mut request = SignedRequest::new("POST", "IoTSecuredTunneling", &self.region, "/");
        request.set_endpoint_prefix("api.tunneling.iot".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "IoTSecuredTunneling.DescribeTunnel");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeTunnelResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeTunnelError::from_response(response))
        }
    }

    /// <p>Lists the tags for the specified resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let mut request = SignedRequest::new("POST", "IoTSecuredTunneling", &self.region, "/");
        request.set_endpoint_prefix("api.tunneling.iot".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "IoTSecuredTunneling.ListTagsForResource");
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
                .deserialize::<ListTagsForResourceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p>List all tunnels for an AWS account. Tunnels are listed by creation time in descending order, newer tunnels will be listed before older tunnels.</p>
    async fn list_tunnels(
        &self,
        input: ListTunnelsRequest,
    ) -> Result<ListTunnelsResponse, RusotoError<ListTunnelsError>> {
        let mut request = SignedRequest::new("POST", "IoTSecuredTunneling", &self.region, "/");
        request.set_endpoint_prefix("api.tunneling.iot".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "IoTSecuredTunneling.ListTunnels");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListTunnelsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListTunnelsError::from_response(response))
        }
    }

    /// <p>Creates a new tunnel, and returns two client access tokens for clients to use to connect to the AWS IoT Secure Tunneling proxy server. .</p>
    async fn open_tunnel(
        &self,
        input: OpenTunnelRequest,
    ) -> Result<OpenTunnelResponse, RusotoError<OpenTunnelError>> {
        let mut request = SignedRequest::new("POST", "IoTSecuredTunneling", &self.region, "/");
        request.set_endpoint_prefix("api.tunneling.iot".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "IoTSecuredTunneling.OpenTunnel");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<OpenTunnelResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(OpenTunnelError::from_response(response))
        }
    }

    /// <p>A resource tag.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let mut request = SignedRequest::new("POST", "IoTSecuredTunneling", &self.region, "/");
        request.set_endpoint_prefix("api.tunneling.iot".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "IoTSecuredTunneling.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<TagResourceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Removes a tag from a resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let mut request = SignedRequest::new("POST", "IoTSecuredTunneling", &self.region, "/");
        request.set_endpoint_prefix("api.tunneling.iot".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "IoTSecuredTunneling.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UntagResourceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }
}
