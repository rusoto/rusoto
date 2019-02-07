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
pub struct GetMediaInput {
    /// <p>Identifies the starting chunk to get from the specified stream. </p>
    #[serde(rename = "StartSelector")]
    pub start_selector: StartSelector,
    /// <p>The ARN of the stream from where you want to get the media content. If you don't specify the <code>streamARN</code>, you must specify the <code>streamName</code>.</p>
    #[serde(rename = "StreamARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_arn: Option<String>,
    /// <p>The Kinesis video stream name from where you want to get the media content. If you don't specify the <code>streamName</code>, you must specify the <code>streamARN</code>.</p>
    #[serde(rename = "StreamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetMediaOutput {
    /// <p>The content type of the requested media.</p>
    pub content_type: Option<String>,
    /// <p><p> The payload Kinesis Video Streams returns is a sequence of chunks from the specified stream. For information about the chunks, see . The chunks that Kinesis Video Streams returns in the <code>GetMedia</code> call also include the following additional Matroska (MKV) tags: </p> <ul> <li> <p>AWS<em>KINESISVIDEO</em>CONTINUATION<em>TOKEN (UTF-8 string) - In the event your <code>GetMedia</code> call terminates, you can use this continuation token in your next request to get the next chunk where the last request terminated.</p> </li> <li> <p>AWS</em>KINESISVIDEO<em>MILLIS</em>BEHIND<em>NOW (UTF-8 string) - Client applications can use this tag value to determine how far behind the chunk returned in the response is from the latest chunk on the stream. </p> </li> <li> <p>AWS</em>KINESISVIDEO<em>FRAGMENT</em>NUMBER - Fragment number returned in the chunk.</p> </li> <li> <p>AWS<em>KINESISVIDEO</em>SERVER<em>TIMESTAMP - Server time stamp of the fragment.</p> </li> <li> <p>AWS</em>KINESISVIDEO<em>PRODUCER</em>TIMESTAMP - Producer time stamp of the fragment.</p> </li> </ul> <p>The following tags will be present if an error occurs:</p> <ul> <li> <p>AWS<em>KINESISVIDEO</em>ERROR<em>CODE - String description of an error that caused GetMedia to stop.</p> </li> <li> <p>AWS</em>KINESISVIDEO<em>ERROR</em>ID: Integer code of the error.</p> </li> </ul> <p>The error codes are as follows:</p> <ul> <li> <p>3002 - Error writing to the stream</p> </li> <li> <p>4000 - Requested fragment is not found</p> </li> <li> <p>4500 - Access denied for the stream&#39;s KMS key</p> </li> <li> <p>4501 - Stream&#39;s KMS key is disabled</p> </li> <li> <p>4502 - Validation error on the Stream&#39;s KMS key</p> </li> <li> <p>4503 - KMS key specified in the stream is unavailable</p> </li> <li> <p>4504 - Invalid usage of the KMS key specified in the stream</p> </li> <li> <p>4505 - Invalid state of the KMS key specified in the stream</p> </li> <li> <p>4506 - Unable to find the KMS key specified in the stream</p> </li> <li> <p>5000 - Internal error</p> </li> </ul></p>
    pub payload: Option<Vec<u8>>,
}

/// <p><p>Identifies the chunk on the Kinesis video stream where you want the <code>GetMedia</code> API to start returning media data. You have the following options to identify the starting chunk: </p> <ul> <li> <p>Choose the latest (or oldest) chunk.</p> </li> <li> <p>Identify a specific chunk. You can identify a specific chunk either by providing a fragment number or time stamp (server or producer). </p> </li> <li> <p>Each chunk&#39;s metadata includes a continuation token as a Matroska (MKV) tag (<code>AWS<em>KINESISVIDEO</em>CONTINUATION_TOKEN</code>). If your previous <code>GetMedia</code> request terminated, you can use this tag value in your next <code>GetMedia</code> request. The API then starts returning chunks starting where the last API ended.</p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartSelector {
    /// <p>Specifies the fragment number from where you want the <code>GetMedia</code> API to start returning the fragments. </p>
    #[serde(rename = "AfterFragmentNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_fragment_number: Option<String>,
    /// <p>Continuation token that Kinesis Video Streams returned in the previous <code>GetMedia</code> response. The <code>GetMedia</code> API then starts with the chunk identified by the continuation token.</p>
    #[serde(rename = "ContinuationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    /// <p><p>Identifies the fragment on the Kinesis video stream where you want to start getting the data from.</p> <ul> <li> <p>NOW - Start with the latest chunk on the stream.</p> </li> <li> <p>EARLIEST - Start with earliest available chunk on the stream.</p> </li> <li> <p>FRAGMENT<em>NUMBER - Start with the chunk containing the specific fragment. You must also specify the <code>StartFragmentNumber</code>.</p> </li> <li> <p>PRODUCER</em>TIMESTAMP or SERVER<em>TIMESTAMP - Start with the chunk containing a fragment with the specified producer or server time stamp. You specify the time stamp by adding <code>StartTimestamp</code>.</p> </li> <li> <p> CONTINUATION</em>TOKEN - Read using the specified continuation token. </p> </li> </ul> <note> <p>If you choose the NOW, EARLIEST, or CONTINUATION_TOKEN as the <code>startSelectorType</code>, you don&#39;t provide any additional information in the <code>startSelector</code>.</p> </note></p>
    #[serde(rename = "StartSelectorType")]
    pub start_selector_type: String,
    /// <p>A time stamp value. This value is required if you choose the PRODUCER_TIMESTAMP or the SERVER_TIMESTAMP as the <code>startSelectorType</code>. The <code>GetMedia</code> API then starts with the chunk containing the fragment that has the specified time stamp.</p>
    #[serde(rename = "StartTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<f64>,
}

/// Errors returned by GetMedia
#[derive(Debug, PartialEq)]
pub enum GetMediaError {
    /// <p>Kinesis Video Streams has throttled the request because you have exceeded the limit of allowed client calls. Try making the call later.</p>
    ClientLimitExceeded(String),
    /// <p>Kinesis Video Streams has throttled the request because you have exceeded the limit of allowed client connections.</p>
    ConnectionLimitExceeded(String),
    /// <p>The value for this input parameter is invalid.</p>
    InvalidArgument(String),
    /// <p> Status Code: 400, Caller used wrong endpoint to write data to a stream. On receiving such an exception, the user must call <code>GetDataEndpoint</code> with <code>AccessMode</code> set to "READ" and use the endpoint Kinesis Video returns in the next <code>GetMedia</code> call. </p>
    InvalidEndpoint(String),
    /// <p>Status Code: 403, The caller is not authorized to perform an operation on the given stream, or the token has expired.</p>
    NotAuthorized(String),
    /// <p>Status Code: 404, The stream with the given name does not exist.</p>
    ResourceNotFound(String),
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

impl GetMediaError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetMediaError {
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
                "ClientLimitExceededException" => {
                    return GetMediaError::ClientLimitExceeded(String::from(error_message));
                }
                "ConnectionLimitExceededException" => {
                    return GetMediaError::ConnectionLimitExceeded(String::from(error_message));
                }
                "InvalidArgumentException" => {
                    return GetMediaError::InvalidArgument(String::from(error_message));
                }
                "InvalidEndpointException" => {
                    return GetMediaError::InvalidEndpoint(String::from(error_message));
                }
                "NotAuthorizedException" => {
                    return GetMediaError::NotAuthorized(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return GetMediaError::ResourceNotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return GetMediaError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetMediaError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetMediaError {
    fn from(err: serde_json::error::Error) -> GetMediaError {
        GetMediaError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetMediaError {
    fn from(err: CredentialsError) -> GetMediaError {
        GetMediaError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetMediaError {
    fn from(err: HttpDispatchError) -> GetMediaError {
        GetMediaError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetMediaError {
    fn from(err: io::Error) -> GetMediaError {
        GetMediaError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetMediaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetMediaError {
    fn description(&self) -> &str {
        match *self {
            GetMediaError::ClientLimitExceeded(ref cause) => cause,
            GetMediaError::ConnectionLimitExceeded(ref cause) => cause,
            GetMediaError::InvalidArgument(ref cause) => cause,
            GetMediaError::InvalidEndpoint(ref cause) => cause,
            GetMediaError::NotAuthorized(ref cause) => cause,
            GetMediaError::ResourceNotFound(ref cause) => cause,
            GetMediaError::Validation(ref cause) => cause,
            GetMediaError::Credentials(ref err) => err.description(),
            GetMediaError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetMediaError::ParseError(ref cause) => cause,
            GetMediaError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the Kinesis Video Media API. Kinesis Video Media clients implement this trait.
pub trait KinesisVideoMedia {
    /// <p><p> Use this API to retrieve media content from a Kinesis video stream. In the request, you identify stream name or stream Amazon Resource Name (ARN), and the starting chunk. Kinesis Video Streams then returns a stream of chunks in order by fragment number.</p> <note> <p> You must first call the <code>GetDataEndpoint</code> API to get an endpoint to which you can then send the <code>GetMedia</code> requests. </p> </note> <p>When you put media data (fragments) on a stream, Kinesis Video Streams stores each incoming fragment and related metadata in what is called a &quot;chunk.&quot; For more information, see . The <code>GetMedia</code> API returns a stream of these chunks starting from the chunk that you specify in the request. </p> <p>The following limits apply when using the <code>GetMedia</code> API:</p> <ul> <li> <p>A client can call <code>GetMedia</code> up to five times per second per stream. </p> </li> <li> <p>Kinesis Video Streams sends media data at a rate of up to 25 megabytes per second (or 200 megabits per second) during a <code>GetMedia</code> session. </p> </li> </ul></p>
    fn get_media(&self, input: GetMediaInput) -> RusotoFuture<GetMediaOutput, GetMediaError>;
}
/// A client for the Kinesis Video Media API.
pub struct KinesisVideoMediaClient {
    client: Client,
    region: region::Region,
}

impl KinesisVideoMediaClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> KinesisVideoMediaClient {
        KinesisVideoMediaClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> KinesisVideoMediaClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        KinesisVideoMediaClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl KinesisVideoMedia for KinesisVideoMediaClient {
    /// <p><p> Use this API to retrieve media content from a Kinesis video stream. In the request, you identify stream name or stream Amazon Resource Name (ARN), and the starting chunk. Kinesis Video Streams then returns a stream of chunks in order by fragment number.</p> <note> <p> You must first call the <code>GetDataEndpoint</code> API to get an endpoint to which you can then send the <code>GetMedia</code> requests. </p> </note> <p>When you put media data (fragments) on a stream, Kinesis Video Streams stores each incoming fragment and related metadata in what is called a &quot;chunk.&quot; For more information, see . The <code>GetMedia</code> API returns a stream of these chunks starting from the chunk that you specify in the request. </p> <p>The following limits apply when using the <code>GetMedia</code> API:</p> <ul> <li> <p>A client can call <code>GetMedia</code> up to five times per second per stream. </p> </li> <li> <p>Kinesis Video Streams sends media data at a rate of up to 25 megabytes per second (or 200 megabits per second) during a <code>GetMedia</code> session. </p> </li> </ul></p>
    fn get_media(&self, input: GetMediaInput) -> RusotoFuture<GetMediaOutput, GetMediaError> {
        let request_uri = "/getMedia";

        let mut request = SignedRequest::new("POST", "kinesisvideo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut result = GetMediaOutput::default();
                    result.payload = Some(response.body);

                    if let Some(content_type) = response.headers.get("Content-Type") {
                        let value = content_type.to_owned();
                        result.content_type = Some(value)
                    };

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetMediaError::from_response(response))),
                )
            }
        })
    }
}

impl<T: ?Sized + KinesisVideoMedia> KinesisVideoMedia for ::std::rc::Rc<T> {
    /// <p><p> Use this API to retrieve media content from a Kinesis video stream. In the request, you identify stream name or stream Amazon Resource Name (ARN), and the starting chunk. Kinesis Video Streams then returns a stream of chunks in order by fragment number.</p> <note> <p> You must first call the <code>GetDataEndpoint</code> API to get an endpoint to which you can then send the <code>GetMedia</code> requests. </p> </note> <p>When you put media data (fragments) on a stream, Kinesis Video Streams stores each incoming fragment and related metadata in what is called a &quot;chunk.&quot; For more information, see . The <code>GetMedia</code> API returns a stream of these chunks starting from the chunk that you specify in the request. </p> <p>The following limits apply when using the <code>GetMedia</code> API:</p> <ul> <li> <p>A client can call <code>GetMedia</code> up to five times per second per stream. </p> </li> <li> <p>Kinesis Video Streams sends media data at a rate of up to 25 megabytes per second (or 200 megabits per second) during a <code>GetMedia</code> session. </p> </li> </ul></p>
    fn get_media(&self, input: GetMediaInput) -> RusotoFuture<GetMediaOutput, GetMediaError> {
        KinesisVideoMedia::get_media(&(**self), input)
    }
}

#[cfg(test)]
mod protocol_tests {}
