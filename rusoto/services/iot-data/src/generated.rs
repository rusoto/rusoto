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

use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
#[allow(warnings)]
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};

use futures::{FutureExt, TryFutureExt};
use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde::{Deserialize, Serialize};
/// <p>The input for the DeleteThingShadow operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteThingShadowRequest {
    /// <p>The name of the thing.</p>
    #[serde(rename = "thingName")]
    pub thing_name: String,
}

/// <p>The output from the DeleteThingShadow operation.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteThingShadowResponse {
    /// <p>The state information, in JSON format.</p>
    pub payload: bytes::Bytes,
}

/// <p>The input for the GetThingShadow operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetThingShadowRequest {
    /// <p>The name of the thing.</p>
    #[serde(rename = "thingName")]
    pub thing_name: String,
}

/// <p>The output from the GetThingShadow operation.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetThingShadowResponse {
    /// <p>The state information, in JSON format.</p>
    pub payload: Option<bytes::Bytes>,
}

/// <p>The input for the Publish operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PublishRequest {
    /// <p>The state information, in JSON format.</p>
    #[serde(rename = "payload")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<bytes::Bytes>,
    /// <p>The Quality of Service (QoS) level.</p>
    #[serde(rename = "qos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qos: Option<i64>,
    /// <p>The name of the MQTT topic.</p>
    #[serde(rename = "topic")]
    pub topic: String,
}

/// <p>The input for the UpdateThingShadow operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateThingShadowRequest {
    /// <p>The state information, in JSON format.</p>
    #[serde(rename = "payload")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub payload: bytes::Bytes,
    /// <p>The name of the thing.</p>
    #[serde(rename = "thingName")]
    pub thing_name: String,
}

/// <p>The output from the UpdateThingShadow operation.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateThingShadowResponse {
    /// <p>The state information, in JSON format.</p>
    pub payload: Option<bytes::Bytes>,
}

/// Errors returned by DeleteThingShadow
#[derive(Debug, PartialEq)]
pub enum DeleteThingShadowError {
    /// <p>An unexpected error has occurred.</p>
    InternalFailure(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified combination of HTTP verb and URI is not supported.</p>
    MethodNotAllowed(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The rate exceeds the limit.</p>
    Throttling(String),
    /// <p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// <p>The document encoding is not supported.</p>
    UnsupportedDocumentEncoding(String),
}

impl DeleteThingShadowError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteThingShadowError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteThingShadowError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteThingShadowError::InvalidRequest(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(DeleteThingShadowError::MethodNotAllowed(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteThingShadowError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteThingShadowError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteThingShadowError::Throttling(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteThingShadowError::Unauthorized(err.msg))
                }
                "UnsupportedDocumentEncodingException" => {
                    return RusotoError::Service(
                        DeleteThingShadowError::UnsupportedDocumentEncoding(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteThingShadowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteThingShadowError {
    fn description(&self) -> &str {
        match *self {
            DeleteThingShadowError::InternalFailure(ref cause) => cause,
            DeleteThingShadowError::InvalidRequest(ref cause) => cause,
            DeleteThingShadowError::MethodNotAllowed(ref cause) => cause,
            DeleteThingShadowError::ResourceNotFound(ref cause) => cause,
            DeleteThingShadowError::ServiceUnavailable(ref cause) => cause,
            DeleteThingShadowError::Throttling(ref cause) => cause,
            DeleteThingShadowError::Unauthorized(ref cause) => cause,
            DeleteThingShadowError::UnsupportedDocumentEncoding(ref cause) => cause,
        }
    }
}
/// Errors returned by GetThingShadow
#[derive(Debug, PartialEq)]
pub enum GetThingShadowError {
    /// <p>An unexpected error has occurred.</p>
    InternalFailure(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified combination of HTTP verb and URI is not supported.</p>
    MethodNotAllowed(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The rate exceeds the limit.</p>
    Throttling(String),
    /// <p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// <p>The document encoding is not supported.</p>
    UnsupportedDocumentEncoding(String),
}

impl GetThingShadowError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetThingShadowError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(GetThingShadowError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetThingShadowError::InvalidRequest(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetThingShadowError::MethodNotAllowed(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetThingShadowError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetThingShadowError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetThingShadowError::Throttling(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetThingShadowError::Unauthorized(err.msg))
                }
                "UnsupportedDocumentEncodingException" => {
                    return RusotoError::Service(GetThingShadowError::UnsupportedDocumentEncoding(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetThingShadowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetThingShadowError {
    fn description(&self) -> &str {
        match *self {
            GetThingShadowError::InternalFailure(ref cause) => cause,
            GetThingShadowError::InvalidRequest(ref cause) => cause,
            GetThingShadowError::MethodNotAllowed(ref cause) => cause,
            GetThingShadowError::ResourceNotFound(ref cause) => cause,
            GetThingShadowError::ServiceUnavailable(ref cause) => cause,
            GetThingShadowError::Throttling(ref cause) => cause,
            GetThingShadowError::Unauthorized(ref cause) => cause,
            GetThingShadowError::UnsupportedDocumentEncoding(ref cause) => cause,
        }
    }
}
/// Errors returned by Publish
#[derive(Debug, PartialEq)]
pub enum PublishError {
    /// <p>An unexpected error has occurred.</p>
    InternalFailure(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified combination of HTTP verb and URI is not supported.</p>
    MethodNotAllowed(String),
    /// <p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
}

impl PublishError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PublishError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(PublishError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(PublishError::InvalidRequest(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(PublishError::MethodNotAllowed(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(PublishError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PublishError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PublishError {
    fn description(&self) -> &str {
        match *self {
            PublishError::InternalFailure(ref cause) => cause,
            PublishError::InvalidRequest(ref cause) => cause,
            PublishError::MethodNotAllowed(ref cause) => cause,
            PublishError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateThingShadow
#[derive(Debug, PartialEq)]
pub enum UpdateThingShadowError {
    /// <p>The specified version does not match the version of the document.</p>
    Conflict(String),
    /// <p>An unexpected error has occurred.</p>
    InternalFailure(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified combination of HTTP verb and URI is not supported.</p>
    MethodNotAllowed(String),
    /// <p>The payload exceeds the maximum size allowed.</p>
    RequestEntityTooLarge(String),
    /// <p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The rate exceeds the limit.</p>
    Throttling(String),
    /// <p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// <p>The document encoding is not supported.</p>
    UnsupportedDocumentEncoding(String),
}

impl UpdateThingShadowError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateThingShadowError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(UpdateThingShadowError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateThingShadowError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateThingShadowError::InvalidRequest(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(UpdateThingShadowError::MethodNotAllowed(err.msg))
                }
                "RequestEntityTooLargeException" => {
                    return RusotoError::Service(UpdateThingShadowError::RequestEntityTooLarge(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateThingShadowError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateThingShadowError::Throttling(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateThingShadowError::Unauthorized(err.msg))
                }
                "UnsupportedDocumentEncodingException" => {
                    return RusotoError::Service(
                        UpdateThingShadowError::UnsupportedDocumentEncoding(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateThingShadowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateThingShadowError {
    fn description(&self) -> &str {
        match *self {
            UpdateThingShadowError::Conflict(ref cause) => cause,
            UpdateThingShadowError::InternalFailure(ref cause) => cause,
            UpdateThingShadowError::InvalidRequest(ref cause) => cause,
            UpdateThingShadowError::MethodNotAllowed(ref cause) => cause,
            UpdateThingShadowError::RequestEntityTooLarge(ref cause) => cause,
            UpdateThingShadowError::ServiceUnavailable(ref cause) => cause,
            UpdateThingShadowError::Throttling(ref cause) => cause,
            UpdateThingShadowError::Unauthorized(ref cause) => cause,
            UpdateThingShadowError::UnsupportedDocumentEncoding(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS IoT Data Plane API. AWS IoT Data Plane clients implement this trait.
pub trait IotData {
    /// <p>Deletes the thing shadow for the specified thing.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/iot/latest/developerguide/API_DeleteThingShadow.html">DeleteThingShadow</a> in the <i>AWS IoT Developer Guide</i>.</p>
    fn delete_thing_shadow(
        &self,
        input: DeleteThingShadowRequest,
    ) -> RusotoFuture<DeleteThingShadowResponse, DeleteThingShadowError>;

    /// <p>Gets the thing shadow for the specified thing.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/iot/latest/developerguide/API_GetThingShadow.html">GetThingShadow</a> in the <i>AWS IoT Developer Guide</i>.</p>
    fn get_thing_shadow(
        &self,
        input: GetThingShadowRequest,
    ) -> RusotoFuture<GetThingShadowResponse, GetThingShadowError>;

    /// <p>Publishes state information.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/iot/latest/developerguide/protocols.html#http">HTTP Protocol</a> in the <i>AWS IoT Developer Guide</i>.</p>
    fn publish(&self, input: PublishRequest) -> RusotoFuture<(), PublishError>;

    /// <p>Updates the thing shadow for the specified thing.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/iot/latest/developerguide/API_UpdateThingShadow.html">UpdateThingShadow</a> in the <i>AWS IoT Developer Guide</i>.</p>
    fn update_thing_shadow(
        &self,
        input: UpdateThingShadowRequest,
    ) -> RusotoFuture<UpdateThingShadowResponse, UpdateThingShadowError>;
}
/// A client for the AWS IoT Data Plane API.
#[derive(Clone)]
pub struct IotDataClient {
    client: Client,
    region: region::Region,
}

impl IotDataClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> IotDataClient {
        IotDataClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> IotDataClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        IotDataClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }
}

impl IotData for IotDataClient {
    /// <p>Deletes the thing shadow for the specified thing.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/iot/latest/developerguide/API_DeleteThingShadow.html">DeleteThingShadow</a> in the <i>AWS IoT Developer Guide</i>.</p>
    fn delete_thing_shadow(
        &self,
        input: DeleteThingShadowRequest,
    ) -> RusotoFuture<DeleteThingShadowResponse, DeleteThingShadowError> {
        let request_uri = format!("/things/{thing_name}/shadow", thing_name = input.thing_name);

        let mut request = SignedRequest::new("DELETE", "iotdata", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("data.iot".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| DeleteThingShadowError::from(e))
                    .map(|try_response| {
                        try_response.map_err(|e| e.into()).and_then(|response| {
                            let mut result = DeleteThingShadowResponse::default();
                            result.payload = response.body;

                            result
                        })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| e.into::<DeleteThingShadowError>())
                            .and_then(|response| {
                                Err(DeleteThingShadowError::from_response(response))
                            })
                    })
                    .boxed()
            }
        })
    }

    /// <p>Gets the thing shadow for the specified thing.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/iot/latest/developerguide/API_GetThingShadow.html">GetThingShadow</a> in the <i>AWS IoT Developer Guide</i>.</p>
    fn get_thing_shadow(
        &self,
        input: GetThingShadowRequest,
    ) -> RusotoFuture<GetThingShadowResponse, GetThingShadowError> {
        let request_uri = format!("/things/{thing_name}/shadow", thing_name = input.thing_name);

        let mut request = SignedRequest::new("GET", "iotdata", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("data.iot".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| GetThingShadowError::from(e))
                    .map(|try_response| {
                        try_response.map_err(|e| e.into()).and_then(|response| {
                            let mut result = GetThingShadowResponse::default();
                            result.payload = Some(response.body);

                            result
                        })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| e.into::<GetThingShadowError>())
                            .and_then(|response| Err(GetThingShadowError::from_response(response)))
                    })
                    .boxed()
            }
        })
    }

    /// <p>Publishes state information.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/iot/latest/developerguide/protocols.html#http">HTTP Protocol</a> in the <i>AWS IoT Developer Guide</i>.</p>
    fn publish(&self, input: PublishRequest) -> RusotoFuture<(), PublishError> {
        let request_uri = format!("/topics/{topic}", topic = input.topic);

        let mut request = SignedRequest::new("POST", "iotdata", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("data.iot".to_string());
        let encoded = if let Some(ref payload) = input.payload {
            Some(payload.to_owned())
        } else {
            None
        };
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.qos {
            params.put("qos", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| PublishError::from(e))
                    .map(|try_response| {
                        try_response.map_err(|e| e.into()).and_then(|response| {
                            let result = ::std::mem::drop(response);

                            result
                        })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| e.into::<PublishError>())
                            .and_then(|response| Err(PublishError::from_response(response)))
                    })
                    .boxed()
            }
        })
    }

    /// <p>Updates the thing shadow for the specified thing.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/iot/latest/developerguide/API_UpdateThingShadow.html">UpdateThingShadow</a> in the <i>AWS IoT Developer Guide</i>.</p>
    fn update_thing_shadow(
        &self,
        input: UpdateThingShadowRequest,
    ) -> RusotoFuture<UpdateThingShadowResponse, UpdateThingShadowError> {
        let request_uri = format!("/things/{thing_name}/shadow", thing_name = input.thing_name);

        let mut request = SignedRequest::new("POST", "iotdata", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("data.iot".to_string());
        let encoded = Some(input.payload.to_owned());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| UpdateThingShadowError::from(e))
                    .map(|try_response| {
                        try_response.map_err(|e| e.into()).and_then(|response| {
                            let mut result = UpdateThingShadowResponse::default();
                            result.payload = Some(response.body);

                            result
                        })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| e.into::<UpdateThingShadowError>())
                            .and_then(|response| {
                                Err(UpdateThingShadowError::from_response(response))
                            })
                    })
                    .boxed()
            }
        })
    }
}
