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
pub struct InvokeEndpointInput {
    /// <p>The desired MIME type of the inference in the response.</p>
    #[serde(rename = "Accept")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept: Option<String>,
    /// <p>Provides input data, in the format specified in the <code>ContentType</code> request header. Amazon SageMaker passes all of the data in the body to the model. </p>
    #[serde(rename = "Body")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub body: Vec<u8>,
    /// <p>The MIME type of the input data in the request body.</p>
    #[serde(rename = "ContentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// <p>The name of the endpoint that you specified when you created the endpoint using the <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/API_CreateEndpoint.html">CreateEndpoint</a> API. </p>
    #[serde(rename = "EndpointName")]
    pub endpoint_name: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct InvokeEndpointOutput {
    /// <p>Includes the inference provided by the model.</p>
    pub body: Vec<u8>,
    /// <p>The MIME type of the inference returned in the response body.</p>
    pub content_type: Option<String>,
    /// <p>Identifies the production variant that was invoked.</p>
    pub invoked_production_variant: Option<String>,
}

/// Errors returned by InvokeEndpoint
#[derive(Debug, PartialEq)]
pub enum InvokeEndpointError {
    /// <p> Internal failure occurred. </p>
    InternalFailure(String),
    /// <p> Model (owned by the customer in the container) returned an error 500. </p>
    ModelError(String),
    /// <p> Service is unavailable. Try your call again. </p>
    ServiceUnavailable(String),
    /// <p> Inspect your request and try again. </p>
    ValidationError(String),
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

impl InvokeEndpointError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> InvokeEndpointError {
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
                "InternalFailure" => {
                    return InvokeEndpointError::InternalFailure(String::from(error_message))
                }
                "ModelError" => return InvokeEndpointError::ModelError(String::from(error_message)),
                "ServiceUnavailable" => {
                    return InvokeEndpointError::ServiceUnavailable(String::from(error_message))
                }
                "ValidationError" => {
                    return InvokeEndpointError::ValidationError(String::from(error_message))
                }
                "ValidationException" => {
                    return InvokeEndpointError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return InvokeEndpointError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for InvokeEndpointError {
    fn from(err: serde_json::error::Error) -> InvokeEndpointError {
        InvokeEndpointError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for InvokeEndpointError {
    fn from(err: CredentialsError) -> InvokeEndpointError {
        InvokeEndpointError::Credentials(err)
    }
}
impl From<HttpDispatchError> for InvokeEndpointError {
    fn from(err: HttpDispatchError) -> InvokeEndpointError {
        InvokeEndpointError::HttpDispatch(err)
    }
}
impl From<io::Error> for InvokeEndpointError {
    fn from(err: io::Error) -> InvokeEndpointError {
        InvokeEndpointError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for InvokeEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for InvokeEndpointError {
    fn description(&self) -> &str {
        match *self {
            InvokeEndpointError::InternalFailure(ref cause) => cause,
            InvokeEndpointError::ModelError(ref cause) => cause,
            InvokeEndpointError::ServiceUnavailable(ref cause) => cause,
            InvokeEndpointError::ValidationError(ref cause) => cause,
            InvokeEndpointError::Validation(ref cause) => cause,
            InvokeEndpointError::Credentials(ref err) => err.description(),
            InvokeEndpointError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            InvokeEndpointError::ParseError(ref cause) => cause,
            InvokeEndpointError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the Amazon SageMaker Runtime API. Amazon SageMaker Runtime clients implement this trait.
pub trait SageMakerRuntime {
    /// <p>After you deploy a model into production using Amazon SageMaker hosting services, your client applications use this API to get inferences from the model hosted at the specified endpoint. </p> <p>For an overview of Amazon SageMaker, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/how-it-works.html">How It Works</a> </p> <p> Amazon SageMaker strips all POST headers except those supported by the API. Amazon SageMaker might add additional headers. You should not rely on the behavior of headers outside those enumerated in the request syntax. </p>
    fn invoke_endpoint(
        &self,
        input: InvokeEndpointInput,
    ) -> RusotoFuture<InvokeEndpointOutput, InvokeEndpointError>;
}
/// A client for the Amazon SageMaker Runtime API.
pub struct SageMakerRuntimeClient {
    client: Client,
    region: region::Region,
}

impl SageMakerRuntimeClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> SageMakerRuntimeClient {
        SageMakerRuntimeClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> SageMakerRuntimeClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        SageMakerRuntimeClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl SageMakerRuntime for SageMakerRuntimeClient {
    /// <p>After you deploy a model into production using Amazon SageMaker hosting services, your client applications use this API to get inferences from the model hosted at the specified endpoint. </p> <p>For an overview of Amazon SageMaker, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/how-it-works.html">How It Works</a> </p> <p> Amazon SageMaker strips all POST headers except those supported by the API. Amazon SageMaker might add additional headers. You should not rely on the behavior of headers outside those enumerated in the request syntax. </p>
    fn invoke_endpoint(
        &self,
        input: InvokeEndpointInput,
    ) -> RusotoFuture<InvokeEndpointOutput, InvokeEndpointError> {
        let request_uri = format!(
            "/endpoints/{endpoint_name}/invocations",
            endpoint_name = input.endpoint_name
        );

        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, &request_uri);
        if input.content_type.is_none() {
            request.set_content_type("application/x-amz-json-1.1".to_owned());
        }

        request.set_endpoint_prefix("runtime.sagemaker".to_string());
        let encoded = Some(input.body.to_owned());
        request.set_payload(encoded);

        if let Some(ref accept) = input.accept {
            request.add_header("Accept", &accept.to_string());
        }

        if let Some(ref content_type) = input.content_type {
            request.add_header("Content-Type", &content_type.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut result = InvokeEndpointOutput::default();
                    result.body = response.body;

                    if let Some(content_type) = response.headers.get("Content-Type") {
                        let value = content_type.to_owned();
                        result.content_type = Some(value)
                    };
                    if let Some(invoked_production_variant) =
                        response.headers.get("x-Amzn-Invoked-Production-Variant")
                    {
                        let value = invoked_production_variant.to_owned();
                        result.invoked_production_variant = Some(value)
                    };

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(InvokeEndpointError::from_response(response))),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
