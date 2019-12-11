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
#![allow(warnings)]

use futures::future;
use futures::Future;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};
use std::error::Error;
use std::fmt;

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteConnectionRequest {
    #[serde(rename = "ConnectionId")]
    pub connection_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetConnectionRequest {
    #[serde(rename = "ConnectionId")]
    pub connection_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetConnectionResponse {
    /// <p>The time in ISO 8601 format for when the connection was established.</p>
    #[serde(rename = "ConnectedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_at: Option<f64>,
    #[serde(rename = "Identity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    /// <p>The time in ISO 8601 format for when the connection was last active.</p>
    #[serde(rename = "LastActiveAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_active_at: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Identity {
    /// <p>The source IP address of the TCP connection making the request to API Gateway.</p>
    #[serde(rename = "SourceIp")]
    pub source_ip: String,
    /// <p>The User Agent of the API caller.</p>
    #[serde(rename = "UserAgent")]
    pub user_agent: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PostToConnectionRequest {
    /// <p>The identifier of the connection that a specific client is using.</p>
    #[serde(rename = "ConnectionId")]
    pub connection_id: String,
    /// <p>The data to be sent to the client specified by its connection id.</p>
    #[serde(rename = "Data")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub data: bytes::Bytes,
}

/// Errors returned by DeleteConnection
#[derive(Debug, PartialEq)]
pub enum DeleteConnectionError {
    /// <p>The caller is not authorized to invoke this operation.</p>
    Forbidden(String),
    /// <p>The connection with the provided id no longer exists.</p>
    Gone(String),
    /// <p>The client is sending more than the allowed number of requests per unit of time or the WebSocket client side buffer is full.</p>
    LimitExceeded(String),
}

impl DeleteConnectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteConnectionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteConnectionError::Forbidden(err.msg))
                }
                "GoneException" => {
                    return RusotoError::Service(DeleteConnectionError::Gone(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteConnectionError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteConnectionError {
    fn description(&self) -> &str {
        match *self {
            DeleteConnectionError::Forbidden(ref cause) => cause,
            DeleteConnectionError::Gone(ref cause) => cause,
            DeleteConnectionError::LimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by GetConnection
#[derive(Debug, PartialEq)]
pub enum GetConnectionError {
    /// <p>The caller is not authorized to invoke this operation.</p>
    Forbidden(String),
    /// <p>The connection with the provided id no longer exists.</p>
    Gone(String),
    /// <p>The client is sending more than the allowed number of requests per unit of time or the WebSocket client side buffer is full.</p>
    LimitExceeded(String),
}

impl GetConnectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetConnectionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ForbiddenException" => {
                    return RusotoError::Service(GetConnectionError::Forbidden(err.msg))
                }
                "GoneException" => return RusotoError::Service(GetConnectionError::Gone(err.msg)),
                "LimitExceededException" => {
                    return RusotoError::Service(GetConnectionError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetConnectionError {
    fn description(&self) -> &str {
        match *self {
            GetConnectionError::Forbidden(ref cause) => cause,
            GetConnectionError::Gone(ref cause) => cause,
            GetConnectionError::LimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by PostToConnection
#[derive(Debug, PartialEq)]
pub enum PostToConnectionError {
    /// <p>The caller is not authorized to invoke this operation.</p>
    Forbidden(String),
    /// <p>The connection with the provided id no longer exists.</p>
    Gone(String),
    /// <p>The client is sending more than the allowed number of requests per unit of time or the WebSocket client side buffer is full.</p>
    LimitExceeded(String),
    /// <p>The data has exceeded the maximum size allowed.</p>
    PayloadTooLarge(String),
}

impl PostToConnectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PostToConnectionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ForbiddenException" => {
                    return RusotoError::Service(PostToConnectionError::Forbidden(err.msg))
                }
                "GoneException" => {
                    return RusotoError::Service(PostToConnectionError::Gone(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(PostToConnectionError::LimitExceeded(err.msg))
                }
                "PayloadTooLargeException" => {
                    return RusotoError::Service(PostToConnectionError::PayloadTooLarge(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PostToConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PostToConnectionError {
    fn description(&self) -> &str {
        match *self {
            PostToConnectionError::Forbidden(ref cause) => cause,
            PostToConnectionError::Gone(ref cause) => cause,
            PostToConnectionError::LimitExceeded(ref cause) => cause,
            PostToConnectionError::PayloadTooLarge(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AmazonApiGatewayManagementApi API. AmazonApiGatewayManagementApi clients implement this trait.
pub trait ApiGatewayManagementApi {
    /// <p>Delete the connection with the provided id.</p>
    fn delete_connection(
        &self,
        input: DeleteConnectionRequest,
    ) -> RusotoFuture<(), DeleteConnectionError>;

    /// <p>Get information about the connection with the provided id.</p>
    fn get_connection(
        &self,
        input: GetConnectionRequest,
    ) -> RusotoFuture<GetConnectionResponse, GetConnectionError>;

    /// <p>Sends the provided data to the specified connection.</p>
    fn post_to_connection(
        &self,
        input: PostToConnectionRequest,
    ) -> RusotoFuture<(), PostToConnectionError>;
}
/// A client for the AmazonApiGatewayManagementApi API.
#[derive(Clone)]
pub struct ApiGatewayManagementApiClient {
    client: Client,
    region: region::Region,
}

impl ApiGatewayManagementApiClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ApiGatewayManagementApiClient {
        Self::new_with_client(Client::shared(), region)
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ApiGatewayManagementApiClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        Self::new_with_client(
            Client::new_with(credentials_provider, request_dispatcher),
            region,
        )
    }

    pub fn new_with_client(
        client: Client,
        region: region::Region,
    ) -> ApiGatewayManagementApiClient {
        ApiGatewayManagementApiClient { client, region }
    }
}

impl ApiGatewayManagementApi for ApiGatewayManagementApiClient {
    /// <p>Delete the connection with the provided id.</p>
    fn delete_connection(
        &self,
        input: DeleteConnectionRequest,
    ) -> RusotoFuture<(), DeleteConnectionError> {
        let request_uri = format!(
            "/@connections/{connection_id}",
            connection_id = input.connection_id
        );

        let mut request = SignedRequest::new("DELETE", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteConnectionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Get information about the connection with the provided id.</p>
    fn get_connection(
        &self,
        input: GetConnectionRequest,
    ) -> RusotoFuture<GetConnectionResponse, GetConnectionError> {
        let request_uri = format!(
            "/@connections/{connection_id}",
            connection_id = input.connection_id
        );

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetConnectionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetConnectionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Sends the provided data to the specified connection.</p>
    fn post_to_connection(
        &self,
        input: PostToConnectionRequest,
    ) -> RusotoFuture<(), PostToConnectionError> {
        let request_uri = format!(
            "/@connections/{connection_id}",
            connection_id = input.connection_id
        );

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(input.data.to_owned());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PostToConnectionError::from_response(response))),
                )
            }
        })
    }
}
