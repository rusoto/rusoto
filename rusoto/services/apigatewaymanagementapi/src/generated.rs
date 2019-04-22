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

#[allow(warnings)]
use futures::future;
use futures::Future;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde_json;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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

/// Errors returned by PostToConnection
#[derive(Debug, PartialEq)]
pub enum PostToConnectionError {
    /// <p>The caller is not authorized to invoke this operation.</p>
    Forbidden(String),
    /// <p>The connection with the provided id no longer exists.</p>
    Gone(String),
    /// <p>The client is sending more than the allowed number of requests per unit of time.</p>
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
        ApiGatewayManagementApiClient {
            client: Client::shared(),
            region: region,
        }
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
        ApiGatewayManagementApiClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl ApiGatewayManagementApi for ApiGatewayManagementApiClient {
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
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
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

#[cfg(test)]
mod protocol_tests {}
