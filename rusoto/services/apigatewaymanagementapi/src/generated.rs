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
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_slice;
use serde_json::Value as SerdeJsonValue;
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
    pub data: Vec<u8>,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl PostToConnectionError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> PostToConnectionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ForbiddenException" => {
                    return PostToConnectionError::Forbidden(String::from(error_message));
                }
                "GoneException" => return PostToConnectionError::Gone(String::from(error_message)),
                "LimitExceededException" => {
                    return PostToConnectionError::LimitExceeded(String::from(error_message));
                }
                "PayloadTooLargeException" => {
                    return PostToConnectionError::PayloadTooLarge(String::from(error_message));
                }
                "ValidationException" => {
                    return PostToConnectionError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return PostToConnectionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for PostToConnectionError {
    fn from(err: serde_json::error::Error) -> PostToConnectionError {
        PostToConnectionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for PostToConnectionError {
    fn from(err: CredentialsError) -> PostToConnectionError {
        PostToConnectionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PostToConnectionError {
    fn from(err: HttpDispatchError) -> PostToConnectionError {
        PostToConnectionError::HttpDispatch(err)
    }
}
impl From<io::Error> for PostToConnectionError {
    fn from(err: io::Error) -> PostToConnectionError {
        PostToConnectionError::HttpDispatch(HttpDispatchError::from(err))
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
            PostToConnectionError::Validation(ref cause) => cause,
            PostToConnectionError::Credentials(ref err) => err.description(),
            PostToConnectionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PostToConnectionError::ParseError(ref cause) => cause,
            PostToConnectionError::Unknown(_) => "unknown error",
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
