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

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p><p> The payload Kinesis Video Streams returns is a sequence of chunks from the specified stream. For information about the chunks, see . The chunks that Kinesis Video Streams returns in the <code>GetMedia</code> call also include the following additional Matroska (MKV) tags: </p> <ul> <li> <p>AWS<em>KINESISVIDEO</em>CONTINUATION<em>TOKEN (UTF-8 string) - In the event your <code>GetMedia</code> call terminates, you can use this continuation token in your next request to get the next chunk where the last request terminated.</p> </li> <li> <p>AWS</em>KINESISVIDEO<em>MILLIS</em>BEHIND<em>NOW (UTF-8 string) - Client applications can use this tag value to determine how far behind the chunk returned in the response is from the latest chunk on the stream. </p> </li> <li> <p>AWS</em>KINESISVIDEO<em>FRAGMENT</em>NUMBER - Fragment number returned in the chunk.</p> </li> <li> <p>AWS<em>KINESISVIDEO</em>SERVER<em>TIMESTAMP - Server timestamp of the fragment.</p> </li> <li> <p>AWS</em>KINESISVIDEO<em>PRODUCER</em>TIMESTAMP - Producer timestamp of the fragment.</p> </li> </ul> <p>The following tags will be present if an error occurs:</p> <ul> <li> <p>AWS<em>KINESISVIDEO</em>ERROR<em>CODE - String description of an error that caused GetMedia to stop.</p> </li> <li> <p>AWS</em>KINESISVIDEO<em>ERROR</em>ID: Integer code of the error.</p> </li> </ul> <p>The error codes are as follows:</p> <ul> <li> <p>3002 - Error writing to the stream</p> </li> <li> <p>4000 - Requested fragment is not found</p> </li> <li> <p>4500 - Access denied for the stream&#39;s KMS key</p> </li> <li> <p>4501 - Stream&#39;s KMS key is disabled</p> </li> <li> <p>4502 - Validation error on the stream&#39;s KMS key</p> </li> <li> <p>4503 - KMS key specified in the stream is unavailable</p> </li> <li> <p>4504 - Invalid usage of the KMS key specified in the stream</p> </li> <li> <p>4505 - Invalid state of the KMS key specified in the stream</p> </li> <li> <p>4506 - Unable to find the KMS key specified in the stream</p> </li> <li> <p>5000 - Internal error</p> </li> </ul></p>
    pub payload: Option<bytes::Bytes>,
}

/// <p><p>Identifies the chunk on the Kinesis video stream where you want the <code>GetMedia</code> API to start returning media data. You have the following options to identify the starting chunk: </p> <ul> <li> <p>Choose the latest (or oldest) chunk.</p> </li> <li> <p>Identify a specific chunk. You can identify a specific chunk either by providing a fragment number or timestamp (server or producer). </p> </li> <li> <p>Each chunk&#39;s metadata includes a continuation token as a Matroska (MKV) tag (<code>AWS<em>KINESISVIDEO</em>CONTINUATION_TOKEN</code>). If your previous <code>GetMedia</code> request terminated, you can use this tag value in your next <code>GetMedia</code> request. The API then starts returning chunks starting where the last API ended.</p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartSelector {
    /// <p>Specifies the fragment number from where you want the <code>GetMedia</code> API to start returning the fragments. </p>
    #[serde(rename = "AfterFragmentNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_fragment_number: Option<String>,
    /// <p>Continuation token that Kinesis Video Streams returned in the previous <code>GetMedia</code> response. The <code>GetMedia</code> API then starts with the chunk identified by the continuation token.</p>
    #[serde(rename = "ContinuationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    /// <p><p>Identifies the fragment on the Kinesis video stream where you want to start getting the data from.</p> <ul> <li> <p>NOW - Start with the latest chunk on the stream.</p> </li> <li> <p>EARLIEST - Start with earliest available chunk on the stream.</p> </li> <li> <p>FRAGMENT<em>NUMBER - Start with the chunk after a specific fragment. You must also specify the <code>AfterFragmentNumber</code> parameter.</p> </li> <li> <p>PRODUCER</em>TIMESTAMP or SERVER<em>TIMESTAMP - Start with the chunk containing a fragment with the specified producer or server timestamp. You specify the timestamp by adding <code>StartTimestamp</code>.</p> </li> <li> <p> CONTINUATION</em>TOKEN - Read using the specified continuation token. </p> </li> </ul> <note> <p>If you choose the NOW, EARLIEST, or CONTINUATION_TOKEN as the <code>startSelectorType</code>, you don&#39;t provide any additional information in the <code>startSelector</code>.</p> </note></p>
    #[serde(rename = "StartSelectorType")]
    pub start_selector_type: String,
    /// <p>A timestamp value. This value is required if you choose the PRODUCER_TIMESTAMP or the SERVER_TIMESTAMP as the <code>startSelectorType</code>. The <code>GetMedia</code> API then starts with the chunk containing the fragment that has the specified timestamp.</p>
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
}

impl GetMediaError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetMediaError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientLimitExceededException" => {
                    return RusotoError::Service(GetMediaError::ClientLimitExceeded(err.msg))
                }
                "ConnectionLimitExceededException" => {
                    return RusotoError::Service(GetMediaError::ConnectionLimitExceeded(err.msg))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(GetMediaError::InvalidArgument(err.msg))
                }
                "InvalidEndpointException" => {
                    return RusotoError::Service(GetMediaError::InvalidEndpoint(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(GetMediaError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetMediaError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetMediaError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetMediaError::ClientLimitExceeded(ref cause) => write!(f, "{}", cause),
            GetMediaError::ConnectionLimitExceeded(ref cause) => write!(f, "{}", cause),
            GetMediaError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            GetMediaError::InvalidEndpoint(ref cause) => write!(f, "{}", cause),
            GetMediaError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            GetMediaError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetMediaError {}
/// Trait representing the capabilities of the Kinesis Video Media API. Kinesis Video Media clients implement this trait.
#[async_trait]
pub trait KinesisVideoMedia {
    /// <p><p> Use this API to retrieve media content from a Kinesis video stream. In the request, you identify the stream name or stream Amazon Resource Name (ARN), and the starting chunk. Kinesis Video Streams then returns a stream of chunks in order by fragment number.</p> <note> <p>You must first call the <code>GetDataEndpoint</code> API to get an endpoint. Then send the <code>GetMedia</code> requests to this endpoint using the <a href="https://docs.aws.amazon.com/cli/latest/reference/">--endpoint-url parameter</a>. </p> </note> <p>When you put media data (fragments) on a stream, Kinesis Video Streams stores each incoming fragment and related metadata in what is called a &quot;chunk.&quot; For more information, see <a href="https://docs.aws.amazon.com/kinesisvideostreams/latest/dg/API_dataplane_PutMedia.html">PutMedia</a>. The <code>GetMedia</code> API returns a stream of these chunks starting from the chunk that you specify in the request. </p> <p>The following limits apply when using the <code>GetMedia</code> API:</p> <ul> <li> <p>A client can call <code>GetMedia</code> up to five times per second per stream. </p> </li> <li> <p>Kinesis Video Streams sends media data at a rate of up to 25 megabytes per second (or 200 megabits per second) during a <code>GetMedia</code> session. </p> </li> </ul> <note> <p>If an error is thrown after invoking a Kinesis Video Streams media API, in addition to the HTTP status code and the response body, it includes the following pieces of information: </p> <ul> <li> <p> <code>x-amz-ErrorType</code> HTTP header – contains a more specific error type in addition to what the HTTP status code provides. </p> </li> <li> <p> <code>x-amz-RequestId</code> HTTP header – if you want to report an issue to AWS, the support team can better diagnose the problem if given the Request Id.</p> </li> </ul> <p>Both the HTTP status code and the ErrorType header can be utilized to make programmatic decisions about whether errors are retry-able and under what conditions, as well as provide information on what actions the client programmer might need to take in order to successfully try again.</p> <p>For more information, see the <b>Errors</b> section at the bottom of this topic, as well as <a href="https://docs.aws.amazon.com/kinesisvideostreams/latest/dg/CommonErrors.html">Common Errors</a>. </p> </note></p>
    async fn get_media(
        &self,
        input: GetMediaInput,
    ) -> Result<GetMediaOutput, RusotoError<GetMediaError>>;
}
/// A client for the Kinesis Video Media API.
#[derive(Clone)]
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
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> KinesisVideoMediaClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        KinesisVideoMediaClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> KinesisVideoMediaClient {
        KinesisVideoMediaClient { client, region }
    }
}

#[async_trait]
impl KinesisVideoMedia for KinesisVideoMediaClient {
    /// <p><p> Use this API to retrieve media content from a Kinesis video stream. In the request, you identify the stream name or stream Amazon Resource Name (ARN), and the starting chunk. Kinesis Video Streams then returns a stream of chunks in order by fragment number.</p> <note> <p>You must first call the <code>GetDataEndpoint</code> API to get an endpoint. Then send the <code>GetMedia</code> requests to this endpoint using the <a href="https://docs.aws.amazon.com/cli/latest/reference/">--endpoint-url parameter</a>. </p> </note> <p>When you put media data (fragments) on a stream, Kinesis Video Streams stores each incoming fragment and related metadata in what is called a &quot;chunk.&quot; For more information, see <a href="https://docs.aws.amazon.com/kinesisvideostreams/latest/dg/API_dataplane_PutMedia.html">PutMedia</a>. The <code>GetMedia</code> API returns a stream of these chunks starting from the chunk that you specify in the request. </p> <p>The following limits apply when using the <code>GetMedia</code> API:</p> <ul> <li> <p>A client can call <code>GetMedia</code> up to five times per second per stream. </p> </li> <li> <p>Kinesis Video Streams sends media data at a rate of up to 25 megabytes per second (or 200 megabits per second) during a <code>GetMedia</code> session. </p> </li> </ul> <note> <p>If an error is thrown after invoking a Kinesis Video Streams media API, in addition to the HTTP status code and the response body, it includes the following pieces of information: </p> <ul> <li> <p> <code>x-amz-ErrorType</code> HTTP header – contains a more specific error type in addition to what the HTTP status code provides. </p> </li> <li> <p> <code>x-amz-RequestId</code> HTTP header – if you want to report an issue to AWS, the support team can better diagnose the problem if given the Request Id.</p> </li> </ul> <p>Both the HTTP status code and the ErrorType header can be utilized to make programmatic decisions about whether errors are retry-able and under what conditions, as well as provide information on what actions the client programmer might need to take in order to successfully try again.</p> <p>For more information, see the <b>Errors</b> section at the bottom of this topic, as well as <a href="https://docs.aws.amazon.com/kinesisvideostreams/latest/dg/CommonErrors.html">Common Errors</a>. </p> </note></p>
    async fn get_media(
        &self,
        input: GetMediaInput,
    ) -> Result<GetMediaOutput, RusotoError<GetMediaError>> {
        let request_uri = "/getMedia";

        let mut request = SignedRequest::new("POST", "kinesisvideo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;

            let mut result = GetMediaOutput::default();
            result.payload = Some(response.body);

            if let Some(content_type) = response.headers.get("Content-Type") {
                let value = content_type.to_owned();
                result.content_type = Some(value)
            };

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetMediaError::from_response(response))
        }
    }
}
