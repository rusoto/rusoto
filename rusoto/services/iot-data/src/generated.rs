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

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
/// <p>The input for the DeleteThingShadow operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteThingShadowError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteThingShadowError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteThingShadowError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            DeleteThingShadowError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteThingShadowError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteThingShadowError::Throttling(ref cause) => write!(f, "{}", cause),
            DeleteThingShadowError::Unauthorized(ref cause) => write!(f, "{}", cause),
            DeleteThingShadowError::UnsupportedDocumentEncoding(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteThingShadowError {}
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetThingShadowError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetThingShadowError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetThingShadowError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetThingShadowError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetThingShadowError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            GetThingShadowError::Throttling(ref cause) => write!(f, "{}", cause),
            GetThingShadowError::Unauthorized(ref cause) => write!(f, "{}", cause),
            GetThingShadowError::UnsupportedDocumentEncoding(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetThingShadowError {}
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PublishError::InternalFailure(ref cause) => write!(f, "{}", cause),
            PublishError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            PublishError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            PublishError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PublishError {}
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateThingShadowError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateThingShadowError::InternalFailure(ref cause) => write!(f, "{}", cause),
            UpdateThingShadowError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateThingShadowError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            UpdateThingShadowError::RequestEntityTooLarge(ref cause) => write!(f, "{}", cause),
            UpdateThingShadowError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateThingShadowError::Throttling(ref cause) => write!(f, "{}", cause),
            UpdateThingShadowError::Unauthorized(ref cause) => write!(f, "{}", cause),
            UpdateThingShadowError::UnsupportedDocumentEncoding(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateThingShadowError {}
/// Trait representing the capabilities of the AWS IoT Data Plane API. AWS IoT Data Plane clients implement this trait.
#[async_trait]
pub trait IotData {
    /// <p>Deletes the thing shadow for the specified thing.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/iot/latest/developerguide/API_DeleteThingShadow.html">DeleteThingShadow</a> in the <i>AWS IoT Developer Guide</i>.</p>
    async fn delete_thing_shadow(
        &self,
        input: DeleteThingShadowRequest,
    ) -> Result<DeleteThingShadowResponse, RusotoError<DeleteThingShadowError>>;

    /// <p>Gets the thing shadow for the specified thing.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/iot/latest/developerguide/API_GetThingShadow.html">GetThingShadow</a> in the <i>AWS IoT Developer Guide</i>.</p>
    async fn get_thing_shadow(
        &self,
        input: GetThingShadowRequest,
    ) -> Result<GetThingShadowResponse, RusotoError<GetThingShadowError>>;

    /// <p>Publishes state information.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/iot/latest/developerguide/protocols.html#http">HTTP Protocol</a> in the <i>AWS IoT Developer Guide</i>.</p>
    async fn publish(&self, input: PublishRequest) -> Result<(), RusotoError<PublishError>>;

    /// <p>Updates the thing shadow for the specified thing.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/iot/latest/developerguide/API_UpdateThingShadow.html">UpdateThingShadow</a> in the <i>AWS IoT Developer Guide</i>.</p>
    async fn update_thing_shadow(
        &self,
        input: UpdateThingShadowRequest,
    ) -> Result<UpdateThingShadowResponse, RusotoError<UpdateThingShadowError>>;
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

    pub fn new_with_client(client: Client, region: region::Region) -> IotDataClient {
        IotDataClient { client, region }
    }
}

#[async_trait]
impl IotData for IotDataClient {
    /// <p>Deletes the thing shadow for the specified thing.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/iot/latest/developerguide/API_DeleteThingShadow.html">DeleteThingShadow</a> in the <i>AWS IoT Developer Guide</i>.</p>
    async fn delete_thing_shadow(
        &self,
        input: DeleteThingShadowRequest,
    ) -> Result<DeleteThingShadowResponse, RusotoError<DeleteThingShadowError>> {
        let request_uri = format!("/things/{thing_name}/shadow", thing_name = input.thing_name);

        let mut request = SignedRequest::new("DELETE", "iotdata", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("data.iot".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;

            let mut result = DeleteThingShadowResponse::default();
            result.payload = response.body;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteThingShadowError::from_response(response))
        }
    }

    /// <p>Gets the thing shadow for the specified thing.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/iot/latest/developerguide/API_GetThingShadow.html">GetThingShadow</a> in the <i>AWS IoT Developer Guide</i>.</p>
    async fn get_thing_shadow(
        &self,
        input: GetThingShadowRequest,
    ) -> Result<GetThingShadowResponse, RusotoError<GetThingShadowError>> {
        let request_uri = format!("/things/{thing_name}/shadow", thing_name = input.thing_name);

        let mut request = SignedRequest::new("GET", "iotdata", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("data.iot".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;

            let mut result = GetThingShadowResponse::default();
            result.payload = Some(response.body);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetThingShadowError::from_response(response))
        }
    }

    /// <p>Publishes state information.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/iot/latest/developerguide/protocols.html#http">HTTP Protocol</a> in the <i>AWS IoT Developer Guide</i>.</p>
    async fn publish(&self, input: PublishRequest) -> Result<(), RusotoError<PublishError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PublishError::from_response(response))
        }
    }

    /// <p>Updates the thing shadow for the specified thing.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/iot/latest/developerguide/API_UpdateThingShadow.html">UpdateThingShadow</a> in the <i>AWS IoT Developer Guide</i>.</p>
    async fn update_thing_shadow(
        &self,
        input: UpdateThingShadowRequest,
    ) -> Result<UpdateThingShadowResponse, RusotoError<UpdateThingShadowError>> {
        let request_uri = format!("/things/{thing_name}/shadow", thing_name = input.thing_name);

        let mut request = SignedRequest::new("POST", "iotdata", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("data.iot".to_string());
        let encoded = Some(input.payload.to_owned());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;

            let mut result = UpdateThingShadowResponse::default();
            result.payload = Some(response.body);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateThingShadowError::from_response(response))
        }
    }
}
