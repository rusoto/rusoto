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
    /// <p>The timestamp from the producer corresponding to the fragment.</p>
    #[serde(rename = "ProducerTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub producer_timestamp: Option<f64>,
    /// <p>The timestamp from the AWS server corresponding to the fragment.</p>
    #[serde(rename = "ServerTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_timestamp: Option<f64>,
}

/// <p>Describes the timestamp range and timestamp origin of a range of fragments.</p> <p>Only fragments with a start timestamp greater than or equal to the given start time and less than or equal to the end time are returned. For example, if a stream contains fragments with the following start timestamps: </p> <ul> <li> <p>00:00:00</p> </li> <li> <p>00:00:02</p> </li> <li> <p>00:00:04</p> </li> <li> <p>00:00:06</p> </li> </ul> <p> A fragment selector range with a start time of 00:00:01 and end time of 00:00:04 would return the fragments with start times of 00:00:02 and 00:00:04. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct FragmentSelector {
    /// <p>The origin of the timestamps to use (Server or Producer).</p>
    #[serde(rename = "FragmentSelectorType")]
    pub fragment_selector_type: String,
    /// <p>The range of timestamps to return.</p>
    #[serde(rename = "TimestampRange")]
    pub timestamp_range: TimestampRange,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetHLSStreamingSessionURLInput {
    /// <p>Specifies which format should be used for packaging the media. Specifying the <code>FRAGMENTED_MP4</code> container format packages the media into MP4 fragments (fMP4 or CMAF). This is the recommended packaging because there is minimal packaging overhead. The other container format option is <code>MPEG_TS</code>. HLS has supported MPEG TS chunks since it was released and is sometimes the only supported packaging on older HLS players. MPEG TS typically has a 5-25 percent packaging overhead. This means MPEG TS typically requires 5-25 percent more bandwidth and cost than fMP4.</p> <p>The default is <code>FRAGMENTED_MP4</code>.</p>
    #[serde(rename = "ContainerFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_format: Option<String>,
    /// <p>Specifies when flags marking discontinuities between fragments will be added to the media playlists. The default is <code>ALWAYS</code> when <a>HLSFragmentSelector</a> is <code>SERVER_TIMESTAMP</code>, and <code>NEVER</code> when it is <code>PRODUCER_TIMESTAMP</code>.</p> <p>Media players typically build a timeline of media content to play, based on the timestamps of each fragment. This means that if there is any overlap between fragments (as is typical if <a>HLSFragmentSelector</a> is <code>SERVER_TIMESTAMP</code>), the media player timeline has small gaps between fragments in some places, and overwrites frames in other places. When there are discontinuity flags between fragments, the media player is expected to reset the timeline, resulting in the fragment being played immediately after the previous fragment. We recommend that you always have discontinuity flags between fragments if the fragment timestamps are not accurate or if fragments might be missing. You should not place discontinuity flags between fragments for the player timeline to accurately map to the producer timestamps.</p>
    #[serde(rename = "DiscontinuityMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discontinuity_mode: Option<String>,
    /// <p>Specifies when the fragment start timestamps should be included in the HLS media playlist. Typically, media players report the playhead position as a time relative to the start of the first fragment in the playback session. However, when the start timestamps are included in the HLS media playlist, some media players might report the current playhead as an absolute time based on the fragment timestamps. This can be useful for creating a playback experience that shows viewers the wall-clock time of the media.</p> <p>The default is <code>NEVER</code>. When <a>HLSFragmentSelector</a> is <code>SERVER_TIMESTAMP</code>, the timestamps will be the server start timestamps. Similarly, when <a>HLSFragmentSelector</a> is <code>PRODUCER_TIMESTAMP</code>, the timestamps will be the producer start timestamps. </p>
    #[serde(rename = "DisplayFragmentTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_fragment_timestamp: Option<String>,
    /// <p>The time in seconds until the requested session expires. This value can be between 300 (5 minutes) and 43200 (12 hours).</p> <p>When a session expires, no new calls to <code>GetHLSMasterPlaylist</code>, <code>GetHLSMediaPlaylist</code>, <code>GetMP4InitFragment</code>, or <code>GetMP4MediaFragment</code> can be made for that session.</p> <p>The default is 300 (5 minutes).</p>
    #[serde(rename = "Expires")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<i64>,
    /// <p>The time range of the requested fragment, and the source of the timestamps.</p> <p>This parameter is required if <code>PlaybackMode</code> is <code>ON_DEMAND</code>. This parameter is optional if <code>PlaybackMode</code> is <code>LIVE</code>. If <code>PlaybackMode</code> is <code>LIVE</code>, the <code>FragmentSelectorType</code> can be set, but the <code>TimestampRange</code> should not be set. If <code>PlaybackMode</code> is <code>ON_DEMAND</code>, both <code>FragmentSelectorType</code> and <code>TimestampRange</code> must be set.</p>
    #[serde(rename = "HLSFragmentSelector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_fragment_selector: Option<HLSFragmentSelector>,
    /// <p>The maximum number of fragments that are returned in the HLS media playlists.</p> <p>When the <code>PlaybackMode</code> is <code>LIVE</code>, the most recent fragments are returned up to this value. When the <code>PlaybackMode</code> is <code>ON_DEMAND</code>, the oldest fragments are returned, up to this maximum number.</p> <p>When there are a higher number of fragments available in a live HLS media playlist, video players often buffer content before starting playback. Increasing the buffer size increases the playback latency, but it decreases the likelihood that rebuffering will occur during playback. We recommend that a live HLS media playlist have a minimum of 3 fragments and a maximum of 10 fragments.</p> <p>The default is 5 fragments if <code>PlaybackMode</code> is <code>LIVE</code>, and 1,000 if <code>PlaybackMode</code> is <code>ON_DEMAND</code>. </p> <p>The maximum value of 1,000 fragments corresponds to more than 16 minutes of video on streams with 1-second fragments, and more than 2 1/2 hours of video on streams with 10-second fragments.</p>
    #[serde(rename = "MaxMediaPlaylistFragmentResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_media_playlist_fragment_results: Option<i64>,
    /// <p>Whether to retrieve live or archived, on-demand data.</p> <p>Features of the two types of session include the following:</p> <ul> <li> <p> <b> <code>LIVE</code> </b>: For sessions of this type, the HLS media playlist is continually updated with the latest fragments as they become available. We recommend that the media player retrieve a new playlist on a one-second interval. When this type of session is played in a media player, the user interface typically displays a "live" notification, with no scrubber control for choosing the position in the playback window to display.</p> <note> <p>In <code>LIVE</code> mode, the newest available fragments are included in an HLS media playlist, even if there is a gap between fragments (that is, if a fragment is missing). A gap like this might cause a media player to halt or cause a jump in playback. In this mode, fragments are not added to the HLS media playlist if they are older than the newest fragment in the playlist. If the missing fragment becomes available after a subsequent fragment is added to the playlist, the older fragment is not added, and the gap is not filled.</p> </note> </li> <li> <p> <b> <code>ON_DEMAND</code> </b>: For sessions of this type, the HLS media playlist contains all the fragments for the session, up to the number that is specified in <code>MaxMediaPlaylistFragmentResults</code>. The playlist must be retrieved only once for each session. When this type of session is played in a media player, the user interface typically displays a scrubber control for choosing the position in the playback window to display.</p> </li> </ul> <p>In both playback modes, if <code>FragmentSelectorType</code> is <code>PRODUCER_TIMESTAMP</code>, and if there are multiple fragments with the same start timestamp, the fragment that has the larger fragment number (that is, the newer fragment) is included in the HLS media playlist. The other fragments are not included. Fragments that have different timestamps but have overlapping durations are still included in the HLS media playlist. This can lead to unexpected behavior in the media player.</p> <p>The default is <code>LIVE</code>.</p>
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
    /// <p><p>The payload that Kinesis Video Streams returns is a sequence of chunks from the specified stream. For information about the chunks, see <a href="http://docs.aws.amazon.com/kinesisvideostreams/latest/dg/API_dataplane_PutMedia.html">PutMedia</a>. The chunks that Kinesis Video Streams returns in the <code>GetMediaForFragmentList</code> call also include the following additional Matroska (MKV) tags: </p> <ul> <li> <p>AWS<em>KINESISVIDEO</em>FRAGMENT<em>NUMBER - Fragment number returned in the chunk.</p> </li> <li> <p>AWS</em>KINESISVIDEO<em>SERVER</em>SIDE<em>TIMESTAMP - Server-side timestamp of the fragment.</p> </li> <li> <p>AWS</em>KINESISVIDEO<em>PRODUCER</em>SIDE<em>TIMESTAMP - Producer-side timestamp of the fragment.</p> </li> </ul> <p>The following tags will be included if an exception occurs:</p> <ul> <li> <p>AWS</em>KINESISVIDEO<em>FRAGMENT</em>NUMBER - The number of the fragment that threw the exception</p> </li> <li> <p>AWS<em>KINESISVIDEO</em>EXCEPTION<em>ERROR</em>CODE - The integer code of the exception</p> </li> <li> <p>AWS<em>KINESISVIDEO</em>EXCEPTION_MESSAGE - A text description of the exception</p> </li> </ul></p>
    pub payload: Option<bytes::Bytes>,
}

/// <p>Contains the range of timestamps for the requested media, and the source of the timestamps.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct HLSFragmentSelector {
    /// <p>The source of the timestamps for the requested media.</p> <p>When <code>FragmentSelectorType</code> is set to <code>PRODUCER_TIMESTAMP</code> and <a>GetHLSStreamingSessionURLInput$PlaybackMode</a> is <code>ON_DEMAND</code>, the first fragment ingested with a producer timestamp within the specified <a>FragmentSelector$TimestampRange</a> is included in the media playlist. In addition, the fragments with producer timestamps within the <code>TimestampRange</code> ingested immediately following the first fragment (up to the <a>GetHLSStreamingSessionURLInput$MaxMediaPlaylistFragmentResults</a> value) are included. </p> <p>Fragments that have duplicate producer timestamps are deduplicated. This means that if producers are producing a stream of fragments with producer timestamps that are approximately equal to the true clock time, the HLS media playlists will contain all of the fragments within the requested timestamp range. If some fragments are ingested within the same time range and very different points in time, only the oldest ingested collection of fragments are returned.</p> <p>When <code>FragmentSelectorType</code> is set to <code>PRODUCER_TIMESTAMP</code> and <a>GetHLSStreamingSessionURLInput$PlaybackMode</a> is <code>LIVE</code>, the producer timestamps are used in the MP4 fragments and for deduplication. But the most recently ingested fragments based on server timestamps are included in the HLS media playlist. This means that even if fragments ingested in the past have producer timestamps with values now, they are not included in the HLS media playlist.</p> <p>The default is <code>SERVER_TIMESTAMP</code>.</p>
    #[serde(rename = "FragmentSelectorType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fragment_selector_type: Option<String>,
    /// <p>The start and end of the timestamp range for the requested media.</p> <p>This value should not be present if <code>PlaybackType</code> is <code>LIVE</code>.</p>
    #[serde(rename = "TimestampRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_range: Option<HLSTimestampRange>,
}

/// <p><p>The start and end of the timestamp range for the requested media.</p> <p>This value should not be present if <code>PlaybackType</code> is <code>LIVE</code>.</p> <note> <p>The values in the <code>HLSTimestampRange</code> are inclusive. Fragments that begin before the start time but continue past it, or fragments that begin before the end time but continue past it, are included in the session.</p> </note></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct HLSTimestampRange {
    /// <p><p>The end of the timestamp range for the requested media. This value must be within 3 hours of the specified <code>StartTimestamp</code>, and it must be later than the <code>StartTimestamp</code> value.</p> <p>If <code>FragmentSelectorType</code> for the request is <code>SERVER_TIMESTAMP</code>, this value must be in the past.</p> <p>If the <code>HLSTimestampRange</code> value is specified, the <code>EndTimestamp</code> value is required.</p> <note> <p>This value is inclusive. The <code>EndTimestamp</code> is compared to the (starting) timestamp of the fragment. Fragments that start before the <code>EndTimestamp</code> value and continue past it are included in the session.</p> </note></p>
    #[serde(rename = "EndTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<f64>,
    /// <p><p>The start of the timestamp range for the requested media.</p> <p>If the <code>HLSTimestampRange</code> value is specified, the <code>StartTimestamp</code> value is required.</p> <note> <p>This value is inclusive. Fragments that start before the <code>StartTimestamp</code> and continue past it are included in the session. If <code>FragmentSelectorType</code> is <code>SERVER_TIMESTAMP</code>, the <code>StartTimestamp</code> must be later than the stream head.</p> </note></p>
    #[serde(rename = "StartTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListFragmentsInput {
    /// <p>Describes the timestamp range and timestamp origin for the range of fragments to return.</p>
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
    /// <p>A list of archived <a>Fragment</a> objects from the stream that meet the selector criteria. Results are in no specific order, even across pages.</p>
    #[serde(rename = "Fragments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fragments: Option<Vec<Fragment>>,
    /// <p>If the returned list is truncated, the operation returns this token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>The range of timestamps for which to return fragments.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TimestampRange {
    /// <p>The ending timestamp in the range of timestamps for which to return fragments.</p>
    #[serde(rename = "EndTimestamp")]
    pub end_timestamp: f64,
    /// <p>The starting timestamp in the range of timestamps for which to return fragments.</p>
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
    /// <p>The codec private data in at least one of the tracks of the video stream is not valid for this operation.</p>
    InvalidCodecPrivateData(String),
    /// <p>No codec private data was found in at least one of tracks of the video stream.</p>
    MissingCodecPrivateData(String),
    /// <p>A <code>PlaybackMode</code> of <code>ON_DEMAND</code> was requested for a stream that does not retain data (that is, has a <code>DataRetentionInHours</code> of 0). </p>
    NoDataRetention(String),
    /// <p>Status Code: 403, The caller is not authorized to perform an operation on the given stream, or the token has expired.</p>
    NotAuthorized(String),
    /// <p> <code>GetMedia</code> throws this error when Kinesis Video Streams can't find the stream that you specified.</p> <p> <code>GetHLSStreamingSessionURL</code> throws this error if a session with a <code>PlaybackMode</code> of <code>ON_DEMAND</code> is requested for a stream that has no fragments within the requested time range, or if a session with a <code>PlaybackMode</code> of <code>LIVE</code> is requested for a stream that has no fragments within the last 30 seconds.</p>
    ResourceNotFound(String),
    /// <p>The type of the media (for example, h.264 video or ACC audio) could not be determined from the codec IDs of the tracks in the first fragment for a playback session. The codec ID for track 1 should be <code>V_MPEG/ISO/AVC</code> and, optionally, the codec ID for track 2 should be <code>A_AAC</code>.</p>
    UnsupportedStreamMediaType(String),
}

impl GetHLSStreamingSessionURLError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetHLSStreamingSessionURLError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientLimitExceededException" => {
                    return RusotoError::Service(
                        GetHLSStreamingSessionURLError::ClientLimitExceeded(err.msg),
                    )
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(GetHLSStreamingSessionURLError::InvalidArgument(
                        err.msg,
                    ))
                }
                "InvalidCodecPrivateDataException" => {
                    return RusotoError::Service(
                        GetHLSStreamingSessionURLError::InvalidCodecPrivateData(err.msg),
                    )
                }
                "MissingCodecPrivateDataException" => {
                    return RusotoError::Service(
                        GetHLSStreamingSessionURLError::MissingCodecPrivateData(err.msg),
                    )
                }
                "NoDataRetentionException" => {
                    return RusotoError::Service(GetHLSStreamingSessionURLError::NoDataRetention(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(GetHLSStreamingSessionURLError::NotAuthorized(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetHLSStreamingSessionURLError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "UnsupportedStreamMediaTypeException" => {
                    return RusotoError::Service(
                        GetHLSStreamingSessionURLError::UnsupportedStreamMediaType(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl GetMediaForFragmentListError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetMediaForFragmentListError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientLimitExceededException" => {
                    return RusotoError::Service(GetMediaForFragmentListError::ClientLimitExceeded(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(GetMediaForFragmentListError::InvalidArgument(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(GetMediaForFragmentListError::NotAuthorized(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetMediaForFragmentListError::ResourceNotFound(
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
}

impl ListFragmentsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListFragmentsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientLimitExceededException" => {
                    return RusotoError::Service(ListFragmentsError::ClientLimitExceeded(err.msg))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(ListFragmentsError::InvalidArgument(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(ListFragmentsError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListFragmentsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Trait representing the capabilities of the Kinesis Video Archived Media API. Kinesis Video Archived Media clients implement this trait.
pub trait KinesisVideoArchivedMedia {
    /// <p>Retrieves an HTTP Live Streaming (HLS) URL for the stream. You can then open the URL in a browser or media player to view the stream contents.</p> <p>You must specify either the <code>StreamName</code> or the <code>StreamARN</code>.</p> <p>An Amazon Kinesis video stream has the following requirements for providing data through HLS:</p> <ul> <li> <p>The media must contain h.264 encoded video and, optionally, AAC encoded audio. Specifically, the codec id of track 1 should be <code>V_MPEG/ISO/AVC</code>. Optionally, the codec id of track 2 should be <code>A_AAC</code>.</p> </li> <li> <p>Data retention must be greater than 0.</p> </li> <li> <p>The video track of each fragment must contain codec private data in the Advanced Video Coding (AVC) for H.264 format (<a href="https://www.iso.org/standard/55980.html">MPEG-4 specification ISO/IEC 14496-15</a>). For information about adapting stream data to a given format, see <a href="http://docs.aws.amazon.com/kinesisvideostreams/latest/dg/producer-reference-nal.html">NAL Adaptation Flags</a>.</p> </li> <li> <p>The audio track (if present) of each fragment must contain codec private data in the AAC format (<a href="https://www.iso.org/standard/43345.html">AAC specification ISO/IEC 13818-7</a>).</p> </li> </ul> <p>Kinesis Video Streams HLS sessions contain fragments in the fragmented MPEG-4 form (also called fMP4 or CMAF), rather than the MPEG-2 form (also called TS chunks, which the HLS specification also supports). For more information about HLS fragment types, see the <a href="https://tools.ietf.org/html/draft-pantos-http-live-streaming-23">HLS specification</a>.</p> <p>The following procedure shows how to use HLS with Kinesis Video Streams:</p> <ol> <li> <p>Get an endpoint using <a href="http://docs.aws.amazon.com/kinesisvideostreams/latest/dg/API_GetDataEndpoint.html">GetDataEndpoint</a>, specifying <code>GET_HLS_STREAMING_SESSION_URL</code> for the <code>APIName</code> parameter.</p> </li> <li> <p>Retrieve the HLS URL using <code>GetHLSStreamingSessionURL</code>. Kinesis Video Streams creates an HLS streaming session to be used for accessing content in a stream using the HLS protocol. <code>GetHLSStreamingSessionURL</code> returns an authenticated URL (that includes an encrypted session token) for the session's HLS <i>master playlist</i> (the root resource needed for streaming with HLS).</p> <note> <p>Don't share or store this token where an unauthorized entity could access it. The token provides access to the content of the stream. Safeguard the token with the same measures that you would use with your AWS credentials.</p> </note> <p>The media that is made available through the playlist consists only of the requested stream, time range, and format. No other media data (such as frames outside the requested window or alternate bitrates) is made available.</p> </li> <li> <p>Provide the URL (containing the encrypted session token) for the HLS master playlist to a media player that supports the HLS protocol. Kinesis Video Streams makes the HLS media playlist, initialization fragment, and media fragments available through the master playlist URL. The initialization fragment contains the codec private data for the stream, and other data needed to set up the video or audio decoder and renderer. The media fragments contain H.264-encoded video frames or AAC-encoded audio samples.</p> </li> <li> <p>The media player receives the authenticated URL and requests stream metadata and media data normally. When the media player requests data, it calls the following actions:</p> <ul> <li> <p> <b>GetHLSMasterPlaylist:</b> Retrieves an HLS master playlist, which contains a URL for the <code>GetHLSMediaPlaylist</code> action for each track, and additional metadata for the media player, including estimated bitrate and resolution.</p> </li> <li> <p> <b>GetHLSMediaPlaylist:</b> Retrieves an HLS media playlist, which contains a URL to access the MP4 initialization fragment with the <code>GetMP4InitFragment</code> action, and URLs to access the MP4 media fragments with the <code>GetMP4MediaFragment</code> actions. The HLS media playlist also contains metadata about the stream that the player needs to play it, such as whether the <code>PlaybackMode</code> is <code>LIVE</code> or <code>ON_DEMAND</code>. The HLS media playlist is typically static for sessions with a <code>PlaybackType</code> of <code>ON_DEMAND</code>. The HLS media playlist is continually updated with new fragments for sessions with a <code>PlaybackType</code> of <code>LIVE</code>. There is a distinct HLS media playlist for the video track and the audio track (if applicable) that contains MP4 media URLs for the specific track. </p> </li> <li> <p> <b>GetMP4InitFragment:</b> Retrieves the MP4 initialization fragment. The media player typically loads the initialization fragment before loading any media fragments. This fragment contains the "<code>fytp</code>" and "<code>moov</code>" MP4 atoms, and the child atoms that are needed to initialize the media player decoder.</p> <p>The initialization fragment does not correspond to a fragment in a Kinesis video stream. It contains only the codec private data for the stream and respective track, which the media player needs to decode the media frames.</p> </li> <li> <p> <b>GetMP4MediaFragment:</b> Retrieves MP4 media fragments. These fragments contain the "<code>moof</code>" and "<code>mdat</code>" MP4 atoms and their child atoms, containing the encoded fragment's media frames and their timestamps. </p> <note> <p>After the first media fragment is made available in a streaming session, any fragments that don't contain the same codec private data cause an error to be returned when those different media fragments are loaded. Therefore, the codec private data should not change between fragments in a session. This also means that the session fails if the fragments in a stream change from having only video to having both audio and video.</p> </note> <p>Data retrieved with this action is billable. See <a href="https://aws.amazon.com/kinesis/video-streams/pricing/">Pricing</a> for details.</p> </li> <li> <p> <b>GetTSFragment:</b> Retrieves MPEG TS fragments containing both initialization and media data for all tracks in the stream.</p> <note> <p>If the <code>ContainerFormat</code> is <code>MPEG_TS</code>, this API is used instead of <code>GetMP4InitFragment</code> and <code>GetMP4MediaFragment</code> to retrieve stream media.</p> </note> <p>Data retrieved with this action is billable. For more information, see <a href="https://aws.amazon.com/kinesis/video-streams/pricing/">Kinesis Video Streams pricing</a>.</p> </li> </ul> </li> </ol> <note> <p>The following restrictions apply to HLS sessions:</p> <ul> <li> <p>A streaming session URL should not be shared between players. The service might throttle a session if multiple media players are sharing it. For connection limits, see <a href="http://docs.aws.amazon.com/kinesisvideostreams/latest/dg/limits.html">Kinesis Video Streams Limits</a>.</p> </li> <li> <p>A Kinesis video stream can have a maximum of five active HLS streaming sessions. If a new session is created when the maximum number of sessions is already active, the oldest (earliest created) session is closed. The number of active <code>GetMedia</code> connections on a Kinesis video stream does not count against this limit, and the number of active HLS sessions does not count against the active <code>GetMedia</code> connection limit.</p> </li> </ul> </note> <p>You can monitor the amount of data that the media player consumes by monitoring the <code>GetMP4MediaFragment.OutgoingBytes</code> Amazon CloudWatch metric. For information about using CloudWatch to monitor Kinesis Video Streams, see <a href="http://docs.aws.amazon.com/kinesisvideostreams/latest/dg/monitoring.html">Monitoring Kinesis Video Streams</a>. For pricing information, see <a href="https://aws.amazon.com/kinesis/video-streams/pricing/">Amazon Kinesis Video Streams Pricing</a> and <a href="https://aws.amazon.com/pricing/">AWS Pricing</a>. Charges for both HLS sessions and outgoing AWS data apply.</p> <p>For more information about HLS, see <a href="https://developer.apple.com/streaming/">HTTP Live Streaming</a> on the <a href="https://developer.apple.com">Apple Developer site</a>.</p>
    fn get_hls_streaming_session_url(
        &self,
        input: GetHLSStreamingSessionURLInput,
    ) -> RusotoFuture<GetHLSStreamingSessionURLOutput, GetHLSStreamingSessionURLError>;

    /// <p><p>Gets media for a list of fragments (specified by fragment number) from the archived data in an Amazon Kinesis video stream.</p> <note> <p>You must first call the <code>GetDataEndpoint</code> API to get an endpoint. Then send the <code>GetMediaForFragmentList</code> requests to this endpoint using the <a href="https://docs.aws.amazon.com/cli/latest/reference/">--endpoint-url parameter</a>. </p> </note> <p>The following limits apply when using the <code>GetMediaForFragmentList</code> API:</p> <ul> <li> <p>A client can call <code>GetMediaForFragmentList</code> up to five times per second per stream. </p> </li> <li> <p>Kinesis Video Streams sends media data at a rate of up to 25 megabytes per second (or 200 megabits per second) during a <code>GetMediaForFragmentList</code> session. </p> </li> </ul></p>
    fn get_media_for_fragment_list(
        &self,
        input: GetMediaForFragmentListInput,
    ) -> RusotoFuture<GetMediaForFragmentListOutput, GetMediaForFragmentListError>;

    /// <p><p>Returns a list of <a>Fragment</a> objects from the specified stream and timestamp range within the archived data.</p> <p>Listing fragments is eventually consistent. This means that even if the producer receives an acknowledgment that a fragment is persisted, the result might not be returned immediately from a request to <code>ListFragments</code>. However, results are typically available in less than one second.</p> <note> <p>You must first call the <code>GetDataEndpoint</code> API to get an endpoint. Then send the <code>ListFragments</code> requests to this endpoint using the <a href="https://docs.aws.amazon.com/cli/latest/reference/">--endpoint-url parameter</a>. </p> </note></p>
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
    /// <p>Retrieves an HTTP Live Streaming (HLS) URL for the stream. You can then open the URL in a browser or media player to view the stream contents.</p> <p>You must specify either the <code>StreamName</code> or the <code>StreamARN</code>.</p> <p>An Amazon Kinesis video stream has the following requirements for providing data through HLS:</p> <ul> <li> <p>The media must contain h.264 encoded video and, optionally, AAC encoded audio. Specifically, the codec id of track 1 should be <code>V_MPEG/ISO/AVC</code>. Optionally, the codec id of track 2 should be <code>A_AAC</code>.</p> </li> <li> <p>Data retention must be greater than 0.</p> </li> <li> <p>The video track of each fragment must contain codec private data in the Advanced Video Coding (AVC) for H.264 format (<a href="https://www.iso.org/standard/55980.html">MPEG-4 specification ISO/IEC 14496-15</a>). For information about adapting stream data to a given format, see <a href="http://docs.aws.amazon.com/kinesisvideostreams/latest/dg/producer-reference-nal.html">NAL Adaptation Flags</a>.</p> </li> <li> <p>The audio track (if present) of each fragment must contain codec private data in the AAC format (<a href="https://www.iso.org/standard/43345.html">AAC specification ISO/IEC 13818-7</a>).</p> </li> </ul> <p>Kinesis Video Streams HLS sessions contain fragments in the fragmented MPEG-4 form (also called fMP4 or CMAF), rather than the MPEG-2 form (also called TS chunks, which the HLS specification also supports). For more information about HLS fragment types, see the <a href="https://tools.ietf.org/html/draft-pantos-http-live-streaming-23">HLS specification</a>.</p> <p>The following procedure shows how to use HLS with Kinesis Video Streams:</p> <ol> <li> <p>Get an endpoint using <a href="http://docs.aws.amazon.com/kinesisvideostreams/latest/dg/API_GetDataEndpoint.html">GetDataEndpoint</a>, specifying <code>GET_HLS_STREAMING_SESSION_URL</code> for the <code>APIName</code> parameter.</p> </li> <li> <p>Retrieve the HLS URL using <code>GetHLSStreamingSessionURL</code>. Kinesis Video Streams creates an HLS streaming session to be used for accessing content in a stream using the HLS protocol. <code>GetHLSStreamingSessionURL</code> returns an authenticated URL (that includes an encrypted session token) for the session's HLS <i>master playlist</i> (the root resource needed for streaming with HLS).</p> <note> <p>Don't share or store this token where an unauthorized entity could access it. The token provides access to the content of the stream. Safeguard the token with the same measures that you would use with your AWS credentials.</p> </note> <p>The media that is made available through the playlist consists only of the requested stream, time range, and format. No other media data (such as frames outside the requested window or alternate bitrates) is made available.</p> </li> <li> <p>Provide the URL (containing the encrypted session token) for the HLS master playlist to a media player that supports the HLS protocol. Kinesis Video Streams makes the HLS media playlist, initialization fragment, and media fragments available through the master playlist URL. The initialization fragment contains the codec private data for the stream, and other data needed to set up the video or audio decoder and renderer. The media fragments contain H.264-encoded video frames or AAC-encoded audio samples.</p> </li> <li> <p>The media player receives the authenticated URL and requests stream metadata and media data normally. When the media player requests data, it calls the following actions:</p> <ul> <li> <p> <b>GetHLSMasterPlaylist:</b> Retrieves an HLS master playlist, which contains a URL for the <code>GetHLSMediaPlaylist</code> action for each track, and additional metadata for the media player, including estimated bitrate and resolution.</p> </li> <li> <p> <b>GetHLSMediaPlaylist:</b> Retrieves an HLS media playlist, which contains a URL to access the MP4 initialization fragment with the <code>GetMP4InitFragment</code> action, and URLs to access the MP4 media fragments with the <code>GetMP4MediaFragment</code> actions. The HLS media playlist also contains metadata about the stream that the player needs to play it, such as whether the <code>PlaybackMode</code> is <code>LIVE</code> or <code>ON_DEMAND</code>. The HLS media playlist is typically static for sessions with a <code>PlaybackType</code> of <code>ON_DEMAND</code>. The HLS media playlist is continually updated with new fragments for sessions with a <code>PlaybackType</code> of <code>LIVE</code>. There is a distinct HLS media playlist for the video track and the audio track (if applicable) that contains MP4 media URLs for the specific track. </p> </li> <li> <p> <b>GetMP4InitFragment:</b> Retrieves the MP4 initialization fragment. The media player typically loads the initialization fragment before loading any media fragments. This fragment contains the "<code>fytp</code>" and "<code>moov</code>" MP4 atoms, and the child atoms that are needed to initialize the media player decoder.</p> <p>The initialization fragment does not correspond to a fragment in a Kinesis video stream. It contains only the codec private data for the stream and respective track, which the media player needs to decode the media frames.</p> </li> <li> <p> <b>GetMP4MediaFragment:</b> Retrieves MP4 media fragments. These fragments contain the "<code>moof</code>" and "<code>mdat</code>" MP4 atoms and their child atoms, containing the encoded fragment's media frames and their timestamps. </p> <note> <p>After the first media fragment is made available in a streaming session, any fragments that don't contain the same codec private data cause an error to be returned when those different media fragments are loaded. Therefore, the codec private data should not change between fragments in a session. This also means that the session fails if the fragments in a stream change from having only video to having both audio and video.</p> </note> <p>Data retrieved with this action is billable. See <a href="https://aws.amazon.com/kinesis/video-streams/pricing/">Pricing</a> for details.</p> </li> <li> <p> <b>GetTSFragment:</b> Retrieves MPEG TS fragments containing both initialization and media data for all tracks in the stream.</p> <note> <p>If the <code>ContainerFormat</code> is <code>MPEG_TS</code>, this API is used instead of <code>GetMP4InitFragment</code> and <code>GetMP4MediaFragment</code> to retrieve stream media.</p> </note> <p>Data retrieved with this action is billable. For more information, see <a href="https://aws.amazon.com/kinesis/video-streams/pricing/">Kinesis Video Streams pricing</a>.</p> </li> </ul> </li> </ol> <note> <p>The following restrictions apply to HLS sessions:</p> <ul> <li> <p>A streaming session URL should not be shared between players. The service might throttle a session if multiple media players are sharing it. For connection limits, see <a href="http://docs.aws.amazon.com/kinesisvideostreams/latest/dg/limits.html">Kinesis Video Streams Limits</a>.</p> </li> <li> <p>A Kinesis video stream can have a maximum of five active HLS streaming sessions. If a new session is created when the maximum number of sessions is already active, the oldest (earliest created) session is closed. The number of active <code>GetMedia</code> connections on a Kinesis video stream does not count against this limit, and the number of active HLS sessions does not count against the active <code>GetMedia</code> connection limit.</p> </li> </ul> </note> <p>You can monitor the amount of data that the media player consumes by monitoring the <code>GetMP4MediaFragment.OutgoingBytes</code> Amazon CloudWatch metric. For information about using CloudWatch to monitor Kinesis Video Streams, see <a href="http://docs.aws.amazon.com/kinesisvideostreams/latest/dg/monitoring.html">Monitoring Kinesis Video Streams</a>. For pricing information, see <a href="https://aws.amazon.com/kinesis/video-streams/pricing/">Amazon Kinesis Video Streams Pricing</a> and <a href="https://aws.amazon.com/pricing/">AWS Pricing</a>. Charges for both HLS sessions and outgoing AWS data apply.</p> <p>For more information about HLS, see <a href="https://developer.apple.com/streaming/">HTTP Live Streaming</a> on the <a href="https://developer.apple.com">Apple Developer site</a>.</p>
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
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetHLSStreamingSessionURLOutput, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetHLSStreamingSessionURLError::from_response(response))
                }))
            }
        })
    }

    /// <p><p>Gets media for a list of fragments (specified by fragment number) from the archived data in an Amazon Kinesis video stream.</p> <note> <p>You must first call the <code>GetDataEndpoint</code> API to get an endpoint. Then send the <code>GetMediaForFragmentList</code> requests to this endpoint using the <a href="https://docs.aws.amazon.com/cli/latest/reference/">--endpoint-url parameter</a>. </p> </note> <p>The following limits apply when using the <code>GetMediaForFragmentList</code> API:</p> <ul> <li> <p>A client can call <code>GetMediaForFragmentList</code> up to five times per second per stream. </p> </li> <li> <p>Kinesis Video Streams sends media data at a rate of up to 25 megabytes per second (or 200 megabits per second) during a <code>GetMediaForFragmentList</code> session. </p> </li> </ul></p>
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
                Box::new(response.buffer().from_err().and_then(|response| {
                    let mut result = GetMediaForFragmentListOutput::default();
                    result.payload = Some(response.body);

                    if let Some(content_type) = response.headers.get("Content-Type") {
                        let value = content_type.to_owned();
                        result.content_type = Some(value)
                    };

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetMediaForFragmentListError::from_response(response))
                }))
            }
        })
    }

    /// <p><p>Returns a list of <a>Fragment</a> objects from the specified stream and timestamp range within the archived data.</p> <p>Listing fragments is eventually consistent. This means that even if the producer receives an acknowledgment that a fragment is persisted, the result might not be returned immediately from a request to <code>ListFragments</code>. However, results are typically available in less than one second.</p> <note> <p>You must first call the <code>GetDataEndpoint</code> API to get an endpoint. Then send the <code>ListFragments</code> requests to this endpoint using the <a href="https://docs.aws.amazon.com/cli/latest/reference/">--endpoint-url parameter</a>. </p> </note></p>
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
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListFragmentsOutput, _>()?;

                    Ok(result)
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
