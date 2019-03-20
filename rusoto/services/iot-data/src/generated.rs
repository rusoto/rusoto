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

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_slice;
use serde_json::Value as SerdeJsonValue;
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
    pub payload: Vec<u8>,
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
    pub payload: Option<Vec<u8>>,
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
    pub payload: Option<Vec<u8>>,
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
    pub payload: Vec<u8>,
    /// <p>The name of the thing.</p>
    #[serde(rename = "thingName")]
    pub thing_name: String,
}

/// <p>The output from the UpdateThingShadow operation.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateThingShadowResponse {
    /// <p>The state information, in JSON format.</p>
    pub payload: Option<Vec<u8>>,
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
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteThingShadowError> {
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
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteThingShadowError::InternalFailure(
                        String::from(error_message),
                    ));
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteThingShadowError::InvalidRequest(
                        String::from(error_message),
                    ));
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(DeleteThingShadowError::MethodNotAllowed(
                        String::from(error_message),
                    ));
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteThingShadowError::ResourceNotFound(
                        String::from(error_message),
                    ));
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteThingShadowError::ServiceUnavailable(
                        String::from(error_message),
                    ));
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteThingShadowError::Throttling(String::from(
                        error_message,
                    )));
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteThingShadowError::Unauthorized(String::from(
                        error_message,
                    )));
                }
                "UnsupportedDocumentEncodingException" => {
                    return RusotoError::Service(
                        DeleteThingShadowError::UnsupportedDocumentEncoding(String::from(
                            error_message,
                        )),
                    );
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
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
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetThingShadowError> {
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
                "InternalFailureException" => {
                    return RusotoError::Service(GetThingShadowError::InternalFailure(String::from(
                        error_message,
                    )));
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetThingShadowError::InvalidRequest(String::from(
                        error_message,
                    )));
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetThingShadowError::MethodNotAllowed(
                        String::from(error_message),
                    ));
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetThingShadowError::ResourceNotFound(
                        String::from(error_message),
                    ));
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetThingShadowError::ServiceUnavailable(
                        String::from(error_message),
                    ));
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetThingShadowError::Throttling(String::from(
                        error_message,
                    )));
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetThingShadowError::Unauthorized(String::from(
                        error_message,
                    )));
                }
                "UnsupportedDocumentEncodingException" => {
                    return RusotoError::Service(GetThingShadowError::UnsupportedDocumentEncoding(
                        String::from(error_message),
                    ));
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
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
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PublishError> {
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
                "InternalFailureException" => {
                    return RusotoError::Service(PublishError::InternalFailure(String::from(
                        error_message,
                    )));
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(PublishError::InvalidRequest(String::from(
                        error_message,
                    )));
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(PublishError::MethodNotAllowed(String::from(
                        error_message,
                    )));
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(PublishError::Unauthorized(String::from(
                        error_message,
                    )));
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
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
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateThingShadowError> {
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
                "ConflictException" => {
                    return RusotoError::Service(UpdateThingShadowError::Conflict(String::from(
                        error_message,
                    )));
                }
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateThingShadowError::InternalFailure(
                        String::from(error_message),
                    ));
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateThingShadowError::InvalidRequest(
                        String::from(error_message),
                    ));
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(UpdateThingShadowError::MethodNotAllowed(
                        String::from(error_message),
                    ));
                }
                "RequestEntityTooLargeException" => {
                    return RusotoError::Service(UpdateThingShadowError::RequestEntityTooLarge(
                        String::from(error_message),
                    ));
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateThingShadowError::ServiceUnavailable(
                        String::from(error_message),
                    ));
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateThingShadowError::Throttling(String::from(
                        error_message,
                    )));
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateThingShadowError::Unauthorized(String::from(
                        error_message,
                    )));
                }
                "UnsupportedDocumentEncodingException" => {
                    return RusotoError::Service(
                        UpdateThingShadowError::UnsupportedDocumentEncoding(String::from(
                            error_message,
                        )),
                    );
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
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
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> IotDataClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        IotDataClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut result = DeleteThingShadowResponse::default();
                    result.payload = response.body;

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteThingShadowError::from_response(response))),
                )
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut result = GetThingShadowResponse::default();
                    result.payload = Some(response.body);

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetThingShadowError::from_response(response))),
                )
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
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PublishError::from_response(response))),
                )
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut result = UpdateThingShadowResponse::default();
                    result.payload = Some(response.body);

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateThingShadowError::from_response(response))),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
