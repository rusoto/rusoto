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

#[allow(unused_imports)]
use rusoto_core::signature::decode_uri;

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_slice;
use serde_json::Value as SerdeJsonValue;
/// <p>Represents a segment of video or other time-delimited data.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Fragment {
    /// <p>The playback duration or other time value associated with the fragment.</p>
    #[serde(rename = "FragmentLengthInMilliseconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fragment_length_in_milliseconds: Option<i64>,
    /// <p>The index value of the fragment.</p>
    #[serde(rename = "FragmentNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fragment_number: Option<String>,
    /// <p>The total fragment size, including information about the fragment and contained media data.</p>
    #[serde(rename = "FragmentSizeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fragment_size_in_bytes: Option<i64>,
    /// <p>The time stamp from the producer corresponding to the fragment.</p>
    #[serde(rename = "ProducerTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub producer_timestamp: Option<f64>,
    /// <p>The time stamp from the AWS server corresponding to the fragment.</p>
    #[serde(rename = "ServerTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_timestamp: Option<f64>,
}

/// <p>Describes the time stamp range and time stamp origin of a range of fragments.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct FragmentSelector {
    /// <p>The origin of the time stamps to use (Server or Producer).</p>
    #[serde(rename = "FragmentSelectorType")]
    pub fragment_selector_type: String,
    /// <p>The range of time stamps to return.</p>
    #[serde(rename = "TimestampRange")]
    pub timestamp_range: TimestampRange,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetHLSStreamingSessionURLInput {
    /// <p>Specifies when flags marking discontinuities between fragments will be added to the media playlists. The default is <code>ALWAYS</code> when <a>HLSFragmentSelector</a> is <code>SERVER_TIMESTAMP</code>, and <code>NEVER</code> when it is <code>PRODUCER_TIMESTAMP</code>.</p> <p>Media players typically build a timeline of media content to play, based on the time stamps of each fragment. This means that if there is any overlap between fragments (as is typical if <a>HLSFragmentSelector</a> is <code>SERVER_TIMESTAMP</code>), the media player timeline has small gaps between fragments in some places, and overwrites frames in other places. When there are discontinuity flags between fragments, the media player is expected to reset the timeline, resulting in the fragment being played immediately after the previous fragment. We recommend that you always have discontinuity flags between fragments if the fragment time stamps are not accurate or if fragments might be missing. You should not place discontinuity flags between fragments for the player timeline to accurately map to the producer time stamps.</p>
    #[serde(rename = "DiscontinuityMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discontinuity_mode: Option<String>,
    /// <p>The time in seconds until the requested session expires. This value can be between 300 (5 minutes) and 43200 (12 hours).</p> <p>When a session expires, no new calls to <code>GetHLSMasterPlaylist</code>, <code>GetHLSMediaPlaylist</code>, <code>GetMP4InitFragment</code>, or <code>GetMP4MediaFragment</code> can be made for that session.</p> <p>The default is 300 (5 minutes).</p>
    #[serde(rename = "Expires")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<i64>,
    /// <p>The time range of the requested fragment, and the source of the time stamps.</p> <p>This parameter is required if <code>PlaybackMode</code> is <code>ON_DEMAND</code>. This parameter is optional if <code>PlaybackMode</code> is <code>LIVE</code>. If <code>PlaybackMode</code> is <code>LIVE</code>, the <code>FragmentSelectorType</code> can be set, but the <code>TimestampRange</code> should not be set. If <code>PlaybackMode</code> is <code>ON_DEMAND</code>, both <code>FragmentSelectorType</code> and <code>TimestampRange</code> must be set.</p>
    #[serde(rename = "HLSFragmentSelector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_fragment_selector: Option<HLSFragmentSelector>,
    /// <p>The maximum number of fragments that are returned in the HLS media playlists.</p> <p>When the <code>PlaybackMode</code> is <code>LIVE</code>, the most recent fragments are returned up to this value. When the <code>PlaybackMode</code> is <code>ON_DEMAND</code>, the oldest fragments are returned, up to this maximum number.</p> <p>When there are a higher number of fragments available in a live HLS media playlist, video players often buffer content before starting playback. Increasing the buffer size increases the playback latency, but it decreases the likelihood that rebuffering will occur during playback. We recommend that a live HLS media playlist have a minimum of 3 fragments and a maximum of 10 fragments.</p> <p>The default is 5 fragments if <code>PlaybackMode</code> is <code>LIVE</code>, and 1,000 if <code>PlaybackMode</code> is <code>ON_DEMAND</code>. </p> <p>The maximum value of 1,000 fragments corresponds to more than 16 minutes of video on streams with 1-second fragments, and more than 2 1/2 hours of video on streams with 10-second fragments.</p>
    #[serde(rename = "MaxMediaPlaylistFragmentResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_media_playlist_fragment_results: Option<i64>,
    /// <p>Whether to retrieve live or archived, on-demand data.</p> <p>Features of the two types of session include the following:</p> <ul> <li> <p> <b> <code>LIVE</code> </b>: For sessions of this type, the HLS media playlist is continually updated with the latest fragments as they become available. We recommend that the media player retrieve a new playlist on a one-second interval. When this type of session is played in a media player, the user interface typically displays a "live" notification, with no scrubber control for choosing the position in the playback window to display.</p> <note> <p>In <code>LIVE</code> mode, the newest available fragments are included in an HLS media playlist, even if there is a gap between fragments (that is, if a fragment is missing). A gap like this might cause a media player to halt or cause a jump in playback. In this mode, fragments are not added to the HLS media playlist if they are older than the newest fragment in the playlist. If the missing fragment becomes available after a subsequent fragment is added to the playlist, the older fragment is not added, and the gap is not filled.</p> </note> </li> <li> <p> <b> <code>ON_DEMAND</code> </b>: For sessions of this type, the HLS media playlist contains all the fragments for the session, up to the number that is specified in <code>MaxMediaPlaylistFragmentResults</code>. The playlist must be retrieved only once for each session. When this type of session is played in a media player, the user interface typically displays a scrubber control for choosing the position in the playback window to display.</p> </li> </ul> <p>In both playback modes, if <code>FragmentSelectorType</code> is <code>PRODUCER_TIMESTAMP</code>, and if there are multiple fragments with the same start time stamp, the fragment that has the larger fragment number (that is, the newer fragment) is included in the HLS media playlist. The other fragments are not included. Fragments that have different time stamps but have overlapping durations are still included in the HLS media playlist. This can lead to unexpected behavior in the media player.</p> <p>The default is <code>LIVE</code>.</p>
    #[serde(rename = "PlaybackMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_mode: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the stream for which to retrieve the HLS master playlist URL.</p> <p>You must specify either the <code>StreamName</code> or the <code>StreamARN</code>.</p>
    #[serde(rename = "StreamARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_arn: Option<String>,
    /// <p>The name of the stream for which to retrieve the HLS master playlist URL.</p> <p>You must specify either the <code>StreamName</code> or the <code>StreamARN</code>.</p>
    #[serde(rename = "StreamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetHLSStreamingSessionURLOutput {
    /// <p>The URL (containing the session token) that a media player can use to retrieve the HLS master playlist.</p>
    #[serde(rename = "HLSStreamingSessionURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_streaming_session_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetMediaForFragmentListInput {
    /// <p>A list of the numbers of fragments for which to retrieve media. You retrieve these values with <a>ListFragments</a>.</p>
    #[serde(rename = "Fragments")]
    pub fragments: Vec<String>,
    /// <p>The name of the stream from which to retrieve fragment media.</p>
    #[serde(rename = "StreamName")]
    pub stream_name: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetMediaForFragmentListOutput {
    /// <p>The content type of the requested media.</p>
    pub content_type: Option<String>,
    /// <p><p>The payload that Kinesis Video Streams returns is a sequence of chunks from the specified stream. For information about the chunks, see <a href="http://docs.aws.amazon.com/kinesisvideostreams/latest/dg/API_dataplane_PutMedia.html">PutMedia</a>. The chunks that Kinesis Video Streams returns in the <code>GetMediaForFragmentList</code> call also include the following additional Matroska (MKV) tags: </p> <ul> <li> <p>AWS<em>KINESISVIDEO</em>FRAGMENT<em>NUMBER - Fragment number returned in the chunk.</p> </li> <li> <p>AWS</em>KINESISVIDEO<em>SERVER</em>SIDE<em>TIMESTAMP - Server-side time stamp of the fragment.</p> </li> <li> <p>AWS</em>KINESISVIDEO<em>PRODUCER</em>SIDE<em>TIMESTAMP - Producer-side time stamp of the fragment.</p> </li> </ul> <p>The following tags will be included if an exception occurs:</p> <ul> <li> <p>AWS</em>KINESISVIDEO<em>FRAGMENT</em>NUMBER - The number of the fragment that threw the exception</p> </li> <li> <p>AWS<em>KINESISVIDEO</em>EXCEPTION<em>ERROR</em>CODE - The integer code of the exception</p> </li> <li> <p>AWS<em>KINESISVIDEO</em>EXCEPTION_MESSAGE - A text description of the exception</p> </li> </ul></p>
    pub payload: Option<Vec<u8>>,
}

/// <p>Contains the range of time stamps for the requested media, and the source of the time stamps.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct HLSFragmentSelector {
    /// <p>The source of the time stamps for the requested media.</p> <p>When <code>FragmentSelectorType</code> is set to <code>PRODUCER_TIMESTAMP</code> and <a>GetHLSStreamingSessionURLInput$PlaybackMode</a> is <code>ON_DEMAND</code>, the first fragment ingested with a producer time stamp within the specified <a>FragmentSelector$TimestampRange</a> is included in the media playlist. In addition, the fragments with producer time stamps within the <code>TimestampRange</code> ingested immediately following the first fragment (up to the <a>GetHLSStreamingSessionURLInput$MaxMediaPlaylistFragmentResults</a> value) are included. </p> <p>Fragments that have duplicate producer time stamps are deduplicated. This means that if producers are producing a stream of fragments with producer time stamps that are approximately equal to the true clock time, the HLS media playlists will contain all of the fragments within the requested time stamp range. If some fragments are ingested within the same time range and very different points in time, only the oldest ingested collection of fragments are returned.</p> <p>When <code>FragmentSelectorType</code> is set to <code>PRODUCER_TIMESTAMP</code> and <a>GetHLSStreamingSessionURLInput$PlaybackMode</a> is <code>LIVE</code>, the producer time stamps are used in the MP4 fragments and for deduplication. But the most recently ingested fragments based on server time stamps are included in the HLS media playlist. This means that even if fragments ingested in the past have producer time stamps with values now, they are not included in the HLS media playlist.</p> <p>The default is <code>SERVER_TIMESTAMP</code>.</p>
    #[serde(rename = "FragmentSelectorType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fragment_selector_type: Option<String>,
    /// <p>The start and end of the time stamp range for the requested media.</p> <p>This value should not be present if <code>PlaybackType</code> is <code>LIVE</code>.</p>
    #[serde(rename = "TimestampRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_range: Option<HLSTimestampRange>,
}

/// <p><p>The start and end of the time stamp range for the requested media.</p> <p>This value should not be present if <code>PlaybackType</code> is <code>LIVE</code>.</p> <note> <p>The values in the <code>HLSTimestampRange</code> are inclusive. Fragments that begin before the start time but continue past it, or fragments that begin before the end time but continue past it, are included in the session.</p> </note></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct HLSTimestampRange {
    /// <p><p>The end of the time stamp range for the requested media. This value must be within 3 hours of the specified <code>StartTimestamp</code>, and it must be later than the <code>StartTimestamp</code> value.</p> <p>If <code>FragmentSelectorType</code> for the request is <code>SERVER_TIMESTAMP</code>, this value must be in the past.</p> <p>If the <code>HLSTimestampRange</code> value is specified, the <code>EndTimestamp</code> value is required.</p> <note> <p>This value is inclusive. The <code>EndTimestamp</code> is compared to the (starting) time stamp of the fragment. Fragments that start before the <code>EndTimestamp</code> value and continue past it are included in the session.</p> </note></p>
    #[serde(rename = "EndTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<f64>,
    /// <p><p>The start of the time stamp range for the requested media.</p> <p>If the <code>HLSTimestampRange</code> value is specified, the <code>StartTimestamp</code> value is required.</p> <note> <p>This value is inclusive. Fragments that start before the <code>StartTimestamp</code> and continue past it are included in the session. If <code>FragmentSelectorType</code> is <code>SERVER_TIMESTAMP</code>, the <code>StartTimestamp</code> must be later than the stream head.</p> </note></p>
    #[serde(rename = "StartTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListFragmentsInput {
    /// <p>Describes the time stamp range and time stamp origin for the range of fragments to return.</p>
    #[serde(rename = "FragmentSelector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fragment_selector: Option<FragmentSelector>,
    /// <p>The total number of fragments to return. If the total number of fragments available is more than the value specified in <code>max-results</code>, then a <a>ListFragmentsOutput$NextToken</a> is provided in the output that you can use to resume pagination.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token to specify where to start paginating. This is the <a>ListFragmentsOutput$NextToken</a> from a previously truncated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the stream from which to retrieve a fragment list.</p>
    #[serde(rename = "StreamName")]
    pub stream_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListFragmentsOutput {
    /// <p>A list of fragment numbers that correspond to the time stamp range provided.</p>
    #[serde(rename = "Fragments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fragments: Option<Vec<Fragment>>,
    /// <p>If the returned list is truncated, the operation returns this token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>The range of time stamps for which to return fragments.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TimestampRange {
    /// <p>The ending time stamp in the range of time stamps for which to return fragments.</p>
    #[serde(rename = "EndTimestamp")]
    pub end_timestamp: f64,
    /// <p>The starting time stamp in the range of time stamps for which to return fragments.</p>
    #[serde(rename = "StartTimestamp")]
    pub start_timestamp: f64,
}

/// Errors returned by GetHLSStreamingSessionURL
#[derive(Debug, PartialEq)]
pub enum GetHLSStreamingSessionURLError {
    /// <p>Kinesis Video Streams has throttled the request because you have exceeded the limit of allowed client calls. Try making the call later.</p>
    ClientLimitExceeded(String),
    /// <p>A specified parameter exceeds its restrictions, is not supported, or can't be used.</p>
    InvalidArgument(String),
    /// <p>The Codec Private Data in the video stream is not valid for this operation.</p>
    InvalidCodecPrivateData(String),
    /// <p>No Codec Private Data was found in the video stream.</p>
    MissingCodecPrivateData(String),
    /// <p>A <code>PlaybackMode</code> of <code>ON_DEMAND</code> was requested for a stream that does not retain data (that is, has a <code>DataRetentionInHours</code> of 0). </p>
    NoDataRetention(String),
    /// <p>Status Code: 403, The caller is not authorized to perform an operation on the given stream, or the token has expired.</p>
    NotAuthorized(String),
    /// <p> <code>GetMedia</code> throws this error when Kinesis Video Streams can't find the stream that you specified.</p> <p> <code>GetHLSStreamingSessionURL</code> throws this error if a session with a <code>PlaybackMode</code> of <code>ON_DEMAND</code> is requested for a stream that has no fragments within the requested time range, or if a session with a <code>PlaybackMode</code> of <code>LIVE</code> is requested for a stream that has no fragments within the last 30 seconds.</p>
    ResourceNotFound(String),
    /// <p>An HLS streaming session was requested for a stream with a media type that is not <code>video/h264</code>.</p>
    UnsupportedStreamMediaType(String),
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

impl GetHLSStreamingSessionURLError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetHLSStreamingSessionURLError {
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
                    return GetHLSStreamingSessionURLError::ClientLimitExceeded(String::from(
                        error_message,
                    ));
                }
                "InvalidArgumentException" => {
                    return GetHLSStreamingSessionURLError::InvalidArgument(String::from(
                        error_message,
                    ));
                }
                "InvalidCodecPrivateDataException" => {
                    return GetHLSStreamingSessionURLError::InvalidCodecPrivateData(String::from(
                        error_message,
                    ));
                }
                "MissingCodecPrivateDataException" => {
                    return GetHLSStreamingSessionURLError::MissingCodecPrivateData(String::from(
                        error_message,
                    ));
                }
                "NoDataRetentionException" => {
                    return GetHLSStreamingSessionURLError::NoDataRetention(String::from(
                        error_message,
                    ));
                }
                "NotAuthorizedException" => {
                    return GetHLSStreamingSessionURLError::NotAuthorized(String::from(
                        error_message,
                    ));
                }
                "ResourceNotFoundException" => {
                    return GetHLSStreamingSessionURLError::ResourceNotFound(String::from(
                        error_message,
                    ));
                }
                "UnsupportedStreamMediaTypeException" => {
                    return GetHLSStreamingSessionURLError::UnsupportedStreamMediaType(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return GetHLSStreamingSessionURLError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetHLSStreamingSessionURLError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetHLSStreamingSessionURLError {
    fn from(err: serde_json::error::Error) -> GetHLSStreamingSessionURLError {
        GetHLSStreamingSessionURLError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetHLSStreamingSessionURLError {
    fn from(err: CredentialsError) -> GetHLSStreamingSessionURLError {
        GetHLSStreamingSessionURLError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetHLSStreamingSessionURLError {
    fn from(err: HttpDispatchError) -> GetHLSStreamingSessionURLError {
        GetHLSStreamingSessionURLError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetHLSStreamingSessionURLError {
    fn from(err: io::Error) -> GetHLSStreamingSessionURLError {
        GetHLSStreamingSessionURLError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetHLSStreamingSessionURLError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetHLSStreamingSessionURLError {
    fn description(&self) -> &str {
        match *self {
            GetHLSStreamingSessionURLError::ClientLimitExceeded(ref cause) => cause,
            GetHLSStreamingSessionURLError::InvalidArgument(ref cause) => cause,
            GetHLSStreamingSessionURLError::InvalidCodecPrivateData(ref cause) => cause,
            GetHLSStreamingSessionURLError::MissingCodecPrivateData(ref cause) => cause,
            GetHLSStreamingSessionURLError::NoDataRetention(ref cause) => cause,
            GetHLSStreamingSessionURLError::NotAuthorized(ref cause) => cause,
            GetHLSStreamingSessionURLError::ResourceNotFound(ref cause) => cause,
            GetHLSStreamingSessionURLError::UnsupportedStreamMediaType(ref cause) => cause,
            GetHLSStreamingSessionURLError::Validation(ref cause) => cause,
            GetHLSStreamingSessionURLError::Credentials(ref err) => err.description(),
            GetHLSStreamingSessionURLError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetHLSStreamingSessionURLError::ParseError(ref cause) => cause,
            GetHLSStreamingSessionURLError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetMediaForFragmentList
#[derive(Debug, PartialEq)]
pub enum GetMediaForFragmentListError {
    /// <p>Kinesis Video Streams has throttled the request because you have exceeded the limit of allowed client calls. Try making the call later.</p>
    ClientLimitExceeded(String),
    /// <p>A specified parameter exceeds its restrictions, is not supported, or can't be used.</p>
    InvalidArgument(String),
    /// <p>Status Code: 403, The caller is not authorized to perform an operation on the given stream, or the token has expired.</p>
    NotAuthorized(String),
    /// <p> <code>GetMedia</code> throws this error when Kinesis Video Streams can't find the stream that you specified.</p> <p> <code>GetHLSStreamingSessionURL</code> throws this error if a session with a <code>PlaybackMode</code> of <code>ON_DEMAND</code> is requested for a stream that has no fragments within the requested time range, or if a session with a <code>PlaybackMode</code> of <code>LIVE</code> is requested for a stream that has no fragments within the last 30 seconds.</p>
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

impl GetMediaForFragmentListError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetMediaForFragmentListError {
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
                    return GetMediaForFragmentListError::ClientLimitExceeded(String::from(
                        error_message,
                    ));
                }
                "InvalidArgumentException" => {
                    return GetMediaForFragmentListError::InvalidArgument(String::from(
                        error_message,
                    ));
                }
                "NotAuthorizedException" => {
                    return GetMediaForFragmentListError::NotAuthorized(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return GetMediaForFragmentListError::ResourceNotFound(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return GetMediaForFragmentListError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetMediaForFragmentListError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetMediaForFragmentListError {
    fn from(err: serde_json::error::Error) -> GetMediaForFragmentListError {
        GetMediaForFragmentListError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetMediaForFragmentListError {
    fn from(err: CredentialsError) -> GetMediaForFragmentListError {
        GetMediaForFragmentListError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetMediaForFragmentListError {
    fn from(err: HttpDispatchError) -> GetMediaForFragmentListError {
        GetMediaForFragmentListError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetMediaForFragmentListError {
    fn from(err: io::Error) -> GetMediaForFragmentListError {
        GetMediaForFragmentListError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetMediaForFragmentListError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetMediaForFragmentListError {
    fn description(&self) -> &str {
        match *self {
            GetMediaForFragmentListError::ClientLimitExceeded(ref cause) => cause,
            GetMediaForFragmentListError::InvalidArgument(ref cause) => cause,
            GetMediaForFragmentListError::NotAuthorized(ref cause) => cause,
            GetMediaForFragmentListError::ResourceNotFound(ref cause) => cause,
            GetMediaForFragmentListError::Validation(ref cause) => cause,
            GetMediaForFragmentListError::Credentials(ref err) => err.description(),
            GetMediaForFragmentListError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetMediaForFragmentListError::ParseError(ref cause) => cause,
            GetMediaForFragmentListError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListFragments
#[derive(Debug, PartialEq)]
pub enum ListFragmentsError {
    /// <p>Kinesis Video Streams has throttled the request because you have exceeded the limit of allowed client calls. Try making the call later.</p>
    ClientLimitExceeded(String),
    /// <p>A specified parameter exceeds its restrictions, is not supported, or can't be used.</p>
    InvalidArgument(String),
    /// <p>Status Code: 403, The caller is not authorized to perform an operation on the given stream, or the token has expired.</p>
    NotAuthorized(String),
    /// <p> <code>GetMedia</code> throws this error when Kinesis Video Streams can't find the stream that you specified.</p> <p> <code>GetHLSStreamingSessionURL</code> throws this error if a session with a <code>PlaybackMode</code> of <code>ON_DEMAND</code> is requested for a stream that has no fragments within the requested time range, or if a session with a <code>PlaybackMode</code> of <code>LIVE</code> is requested for a stream that has no fragments within the last 30 seconds.</p>
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

impl ListFragmentsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListFragmentsError {
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
                    return ListFragmentsError::ClientLimitExceeded(String::from(error_message));
                }
                "InvalidArgumentException" => {
                    return ListFragmentsError::InvalidArgument(String::from(error_message));
                }
                "NotAuthorizedException" => {
                    return ListFragmentsError::NotAuthorized(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return ListFragmentsError::ResourceNotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return ListFragmentsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListFragmentsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListFragmentsError {
    fn from(err: serde_json::error::Error) -> ListFragmentsError {
        ListFragmentsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListFragmentsError {
    fn from(err: CredentialsError) -> ListFragmentsError {
        ListFragmentsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListFragmentsError {
    fn from(err: HttpDispatchError) -> ListFragmentsError {
        ListFragmentsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListFragmentsError {
    fn from(err: io::Error) -> ListFragmentsError {
        ListFragmentsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListFragmentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListFragmentsError {
    fn description(&self) -> &str {
        match *self {
            ListFragmentsError::ClientLimitExceeded(ref cause) => cause,
            ListFragmentsError::InvalidArgument(ref cause) => cause,
            ListFragmentsError::NotAuthorized(ref cause) => cause,
            ListFragmentsError::ResourceNotFound(ref cause) => cause,
            ListFragmentsError::Validation(ref cause) => cause,
            ListFragmentsError::Credentials(ref err) => err.description(),
            ListFragmentsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListFragmentsError::ParseError(ref cause) => cause,
            ListFragmentsError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the Kinesis Video Archived Media API. Kinesis Video Archived Media clients implement this trait.
pub trait KinesisVideoArchivedMedia {
    /// <p>Retrieves an HTTP Live Streaming (HLS) URL for the stream. You can then open the URL in a browser or media player to view the stream contents.</p> <p>You must specify either the <code>StreamName</code> or the <code>StreamARN</code>.</p> <p>An Amazon Kinesis video stream has the following requirements for providing data through HLS:</p> <ul> <li> <p>The media type must be <code>video/h264</code>.</p> </li> <li> <p>Data retention must be greater than 0.</p> </li> <li> <p>The fragments must contain codec private data in the AVC (Advanced Video Coding) for H.264 format (<a href="https://www.iso.org/standard/55980.html">MPEG-4 specification ISO/IEC 14496-15</a>). For information about adapting stream data to a given format, see <a href="http://docs.aws.amazon.com/kinesisvideostreams/latest/dg/latest/dg/producer-reference-nal.html">NAL Adaptation Flags</a>.</p> </li> </ul> <p>Kinesis Video Streams HLS sessions contain fragments in the fragmented MPEG-4 form (also called fMP4 or CMAF), rather than the MPEG-2 form (also called TS chunks, which the HLS specification also supports). For more information about HLS fragment types, see the <a href="https://tools.ietf.org/html/draft-pantos-http-live-streaming-23">HLS specification</a>.</p> <p>The following procedure shows how to use HLS with Kinesis Video Streams:</p> <ol> <li> <p>Get an endpoint using <a href="http://docs.aws.amazon.com/kinesisvideostreams/latest/dg/API_GetDataEndpoint.html">GetDataEndpoint</a>, specifying <code>GET_HLS_STREAMING_SESSION_URL</code> for the <code>APIName</code> parameter.</p> </li> <li> <p>Retrieve the HLS URL using <code>GetHLSStreamingSessionURL</code>. Kinesis Video Streams creates an HLS streaming session to be used for accessing content in a stream using the HLS protocol. <code>GetHLSStreamingSessionURL</code> returns an authenticated URL (that includes an encrypted session token) for the session's HLS <i>master playlist</i> (the root resource needed for streaming with HLS).</p> <note> <p>Don't share or store this token where an unauthorized entity could access it. The token provides access to the content of the stream. Safeguard the token with the same measures that you would use with your AWS credentials.</p> </note> <p>The media that is made available through the playlist consists only of the requested stream, time range, and format. No other media data (such as frames outside the requested window or alternate bit rates) is made available.</p> </li> <li> <p>Provide the URL (containing the encrypted session token) for the HLS master playlist to a media player that supports the HLS protocol. Kinesis Video Streams makes the HLS media playlist, initialization fragment, and media fragments available through the master playlist URL. The initialization fragment contains the codec private data for the stream, and other data needed to set up the video decoder and renderer. The media fragments contain H.264-encoded video frames and time stamps.</p> </li> <li> <p>The media player receives the authenticated URL and requests stream metadata and media data normally. When the media player requests data, it calls the following actions:</p> <ul> <li> <p> <b>GetHLSMasterPlaylist:</b> Retrieves an HLS master playlist, which contains a URL for the <code>GetHLSMediaPlaylist</code> action, and additional metadata for the media player, including estimated bit rate and resolution.</p> </li> <li> <p> <b>GetHLSMediaPlaylist:</b> Retrieves an HLS media playlist, which contains a URL to access the MP4 initialization fragment with the <code>GetMP4InitFragment</code> action, and URLs to access the MP4 media fragments with the <code>GetMP4MediaFragment</code> actions. The HLS media playlist also contains metadata about the stream that the player needs to play it, such as whether the <code>PlaybackMode</code> is <code>LIVE</code> or <code>ON_DEMAND</code>. The HLS media playlist is typically static for sessions with a <code>PlaybackType</code> of <code>ON_DEMAND</code>. The HLS media playlist is continually updated with new fragments for sessions with a <code>PlaybackType</code> of <code>LIVE</code>.</p> </li> <li> <p> <b>GetMP4InitFragment:</b> Retrieves the MP4 initialization fragment. The media player typically loads the initialization fragment before loading any media fragments. This fragment contains the "<code>fytp</code>" and "<code>moov</code>" MP4 atoms, and the child atoms that are needed to initialize the media player decoder.</p> <p>The initialization fragment does not correspond to a fragment in a Kinesis video stream. It contains only the codec private data for the stream, which the media player needs to decode video frames.</p> </li> <li> <p> <b>GetMP4MediaFragment:</b> Retrieves MP4 media fragments. These fragments contain the "<code>moof</code>" and "<code>mdat</code>" MP4 atoms and their child atoms, containing the encoded fragment's video frames and their time stamps. </p> <note> <p>After the first media fragment is made available in a streaming session, any fragments that don't contain the same codec private data are excluded in the HLS media playlist. Therefore, the codec private data does not change between fragments in a session.</p> </note> <p>Data retrieved with this action is billable. See <a href="aws.amazon.comkinesis/video-streams/pricing/">Pricing</a> for details.</p> </li> </ul> </li> </ol> <note> <p>The following restrictions apply to HLS sessions:</p> <ul> <li> <p>A streaming session URL should not be shared between players. The service might throttle a session if multiple media players are sharing it. For connection limits, see <a href="http://docs.aws.amazon.com/kinesisvideostreams/latest/dg/limits.html">Kinesis Video Streams Limits</a>.</p> </li> <li> <p>A Kinesis video stream can have a maximum of five active HLS streaming sessions. If a new session is created when the maximum number of sessions is already active, the oldest (earliest created) session is closed. The number of active <code>GetMedia</code> connections on a Kinesis video stream does not count against this limit, and the number of active HLS sessions does not count against the active <code>GetMedia</code> connection limit.</p> </li> </ul> </note> <p>You can monitor the amount of data that the media player consumes by monitoring the <code>GetMP4MediaFragment.OutgoingBytes</code> Amazon CloudWatch metric. For information about using CloudWatch to monitor Kinesis Video Streams, see <a href="http://docs.aws.amazon.com/kinesisvideostreams/latest/dg/monitoring.html">Monitoring Kinesis Video Streams</a>. For pricing information, see <a href="https://aws.amazon.com/kinesis/video-streams/pricing/">Amazon Kinesis Video Streams Pricing</a> and <a href="https://aws.amazon.com/pricing/">AWS Pricing</a>. Charges for both HLS sessions and outgoing AWS data apply.</p> <p>For more information about HLS, see <a href="https://developer.apple.com/streaming/">HTTP Live Streaming</a> on the <a href="https://developer.apple.com">Apple Developer site</a>.</p>
    fn get_hls_streaming_session_url(
        &self,
        input: GetHLSStreamingSessionURLInput,
    ) -> RusotoFuture<GetHLSStreamingSessionURLOutput, GetHLSStreamingSessionURLError>;

    /// <p><p>Gets media for a list of fragments (specified by fragment number) from the archived data in an Amazon Kinesis video stream.</p> <p>The following limits apply when using the <code>GetMediaForFragmentList</code> API:</p> <ul> <li> <p>A client can call <code>GetMediaForFragmentList</code> up to five times per second per stream. </p> </li> <li> <p>Kinesis Video Streams sends media data at a rate of up to 25 megabytes per second (or 200 megabits per second) during a <code>GetMediaForFragmentList</code> session. </p> </li> </ul></p>
    fn get_media_for_fragment_list(
        &self,
        input: GetMediaForFragmentListInput,
    ) -> RusotoFuture<GetMediaForFragmentListOutput, GetMediaForFragmentListError>;

    /// <p>Returns a list of <a>Fragment</a> objects from the specified stream and start location within the archived data.</p>
    fn list_fragments(
        &self,
        input: ListFragmentsInput,
    ) -> RusotoFuture<ListFragmentsOutput, ListFragmentsError>;
}
/// A client for the Kinesis Video Archived Media API.
#[derive(Clone)]
pub struct KinesisVideoArchivedMediaClient {
    client: Client,
    region: region::Region,
}

impl KinesisVideoArchivedMediaClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> KinesisVideoArchivedMediaClient {
        KinesisVideoArchivedMediaClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> KinesisVideoArchivedMediaClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        KinesisVideoArchivedMediaClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl KinesisVideoArchivedMedia for KinesisVideoArchivedMediaClient {
    /// <p>Retrieves an HTTP Live Streaming (HLS) URL for the stream. You can then open the URL in a browser or media player to view the stream contents.</p> <p>You must specify either the <code>StreamName</code> or the <code>StreamARN</code>.</p> <p>An Amazon Kinesis video stream has the following requirements for providing data through HLS:</p> <ul> <li> <p>The media type must be <code>video/h264</code>.</p> </li> <li> <p>Data retention must be greater than 0.</p> </li> <li> <p>The fragments must contain codec private data in the AVC (Advanced Video Coding) for H.264 format (<a href="https://www.iso.org/standard/55980.html">MPEG-4 specification ISO/IEC 14496-15</a>). For information about adapting stream data to a given format, see <a href="http://docs.aws.amazon.com/kinesisvideostreams/latest/dg/latest/dg/producer-reference-nal.html">NAL Adaptation Flags</a>.</p> </li> </ul> <p>Kinesis Video Streams HLS sessions contain fragments in the fragmented MPEG-4 form (also called fMP4 or CMAF), rather than the MPEG-2 form (also called TS chunks, which the HLS specification also supports). For more information about HLS fragment types, see the <a href="https://tools.ietf.org/html/draft-pantos-http-live-streaming-23">HLS specification</a>.</p> <p>The following procedure shows how to use HLS with Kinesis Video Streams:</p> <ol> <li> <p>Get an endpoint using <a href="http://docs.aws.amazon.com/kinesisvideostreams/latest/dg/API_GetDataEndpoint.html">GetDataEndpoint</a>, specifying <code>GET_HLS_STREAMING_SESSION_URL</code> for the <code>APIName</code> parameter.</p> </li> <li> <p>Retrieve the HLS URL using <code>GetHLSStreamingSessionURL</code>. Kinesis Video Streams creates an HLS streaming session to be used for accessing content in a stream using the HLS protocol. <code>GetHLSStreamingSessionURL</code> returns an authenticated URL (that includes an encrypted session token) for the session's HLS <i>master playlist</i> (the root resource needed for streaming with HLS).</p> <note> <p>Don't share or store this token where an unauthorized entity could access it. The token provides access to the content of the stream. Safeguard the token with the same measures that you would use with your AWS credentials.</p> </note> <p>The media that is made available through the playlist consists only of the requested stream, time range, and format. No other media data (such as frames outside the requested window or alternate bit rates) is made available.</p> </li> <li> <p>Provide the URL (containing the encrypted session token) for the HLS master playlist to a media player that supports the HLS protocol. Kinesis Video Streams makes the HLS media playlist, initialization fragment, and media fragments available through the master playlist URL. The initialization fragment contains the codec private data for the stream, and other data needed to set up the video decoder and renderer. The media fragments contain H.264-encoded video frames and time stamps.</p> </li> <li> <p>The media player receives the authenticated URL and requests stream metadata and media data normally. When the media player requests data, it calls the following actions:</p> <ul> <li> <p> <b>GetHLSMasterPlaylist:</b> Retrieves an HLS master playlist, which contains a URL for the <code>GetHLSMediaPlaylist</code> action, and additional metadata for the media player, including estimated bit rate and resolution.</p> </li> <li> <p> <b>GetHLSMediaPlaylist:</b> Retrieves an HLS media playlist, which contains a URL to access the MP4 initialization fragment with the <code>GetMP4InitFragment</code> action, and URLs to access the MP4 media fragments with the <code>GetMP4MediaFragment</code> actions. The HLS media playlist also contains metadata about the stream that the player needs to play it, such as whether the <code>PlaybackMode</code> is <code>LIVE</code> or <code>ON_DEMAND</code>. The HLS media playlist is typically static for sessions with a <code>PlaybackType</code> of <code>ON_DEMAND</code>. The HLS media playlist is continually updated with new fragments for sessions with a <code>PlaybackType</code> of <code>LIVE</code>.</p> </li> <li> <p> <b>GetMP4InitFragment:</b> Retrieves the MP4 initialization fragment. The media player typically loads the initialization fragment before loading any media fragments. This fragment contains the "<code>fytp</code>" and "<code>moov</code>" MP4 atoms, and the child atoms that are needed to initialize the media player decoder.</p> <p>The initialization fragment does not correspond to a fragment in a Kinesis video stream. It contains only the codec private data for the stream, which the media player needs to decode video frames.</p> </li> <li> <p> <b>GetMP4MediaFragment:</b> Retrieves MP4 media fragments. These fragments contain the "<code>moof</code>" and "<code>mdat</code>" MP4 atoms and their child atoms, containing the encoded fragment's video frames and their time stamps. </p> <note> <p>After the first media fragment is made available in a streaming session, any fragments that don't contain the same codec private data are excluded in the HLS media playlist. Therefore, the codec private data does not change between fragments in a session.</p> </note> <p>Data retrieved with this action is billable. See <a href="aws.amazon.comkinesis/video-streams/pricing/">Pricing</a> for details.</p> </li> </ul> </li> </ol> <note> <p>The following restrictions apply to HLS sessions:</p> <ul> <li> <p>A streaming session URL should not be shared between players. The service might throttle a session if multiple media players are sharing it. For connection limits, see <a href="http://docs.aws.amazon.com/kinesisvideostreams/latest/dg/limits.html">Kinesis Video Streams Limits</a>.</p> </li> <li> <p>A Kinesis video stream can have a maximum of five active HLS streaming sessions. If a new session is created when the maximum number of sessions is already active, the oldest (earliest created) session is closed. The number of active <code>GetMedia</code> connections on a Kinesis video stream does not count against this limit, and the number of active HLS sessions does not count against the active <code>GetMedia</code> connection limit.</p> </li> </ul> </note> <p>You can monitor the amount of data that the media player consumes by monitoring the <code>GetMP4MediaFragment.OutgoingBytes</code> Amazon CloudWatch metric. For information about using CloudWatch to monitor Kinesis Video Streams, see <a href="http://docs.aws.amazon.com/kinesisvideostreams/latest/dg/monitoring.html">Monitoring Kinesis Video Streams</a>. For pricing information, see <a href="https://aws.amazon.com/kinesis/video-streams/pricing/">Amazon Kinesis Video Streams Pricing</a> and <a href="https://aws.amazon.com/pricing/">AWS Pricing</a>. Charges for both HLS sessions and outgoing AWS data apply.</p> <p>For more information about HLS, see <a href="https://developer.apple.com/streaming/">HTTP Live Streaming</a> on the <a href="https://developer.apple.com">Apple Developer site</a>.</p>
    fn get_hls_streaming_session_url(
        &self,
        input: GetHLSStreamingSessionURLInput,
    ) -> RusotoFuture<GetHLSStreamingSessionURLOutput, GetHLSStreamingSessionURLError> {
        let request_uri = "/getHLSStreamingSessionURL";

        let mut request = SignedRequest::new("POST", "kinesisvideo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetHLSStreamingSessionURLOutput>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetHLSStreamingSessionURLError::from_response(response))
                }))
            }
        })
    }

    /// <p><p>Gets media for a list of fragments (specified by fragment number) from the archived data in an Amazon Kinesis video stream.</p> <p>The following limits apply when using the <code>GetMediaForFragmentList</code> API:</p> <ul> <li> <p>A client can call <code>GetMediaForFragmentList</code> up to five times per second per stream. </p> </li> <li> <p>Kinesis Video Streams sends media data at a rate of up to 25 megabytes per second (or 200 megabits per second) during a <code>GetMediaForFragmentList</code> session. </p> </li> </ul></p>
    fn get_media_for_fragment_list(
        &self,
        input: GetMediaForFragmentListInput,
    ) -> RusotoFuture<GetMediaForFragmentListOutput, GetMediaForFragmentListError> {
        let request_uri = "/getMediaForFragmentList";

        let mut request = SignedRequest::new("POST", "kinesisvideo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut result = GetMediaForFragmentListOutput::default();
                    result.payload = Some(response.body);

                    if let Some(content_type) = response.headers.get("Content-Type") {
                        let value = content_type.to_owned();
                        result.content_type = Some(value)
                    };

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetMediaForFragmentListError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns a list of <a>Fragment</a> objects from the specified stream and start location within the archived data.</p>
    fn list_fragments(
        &self,
        input: ListFragmentsInput,
    ) -> RusotoFuture<ListFragmentsOutput, ListFragmentsError> {
        let request_uri = "/listFragments";

        let mut request = SignedRequest::new("POST", "kinesisvideo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListFragmentsOutput>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListFragmentsError::from_response(response))),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
