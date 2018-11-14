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

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_slice;
use serde_json::Value as SerdeJsonValue;
/// <p>The configuration for using a content delivery network (CDN), like Amazon CloudFront, for content and ad segment management. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CdnConfiguration {
    /// <p>A non-default content delivery network (CDN) to serve ad segments. By default, AWS Elemental MediaTailor uses Amazon CloudFront with default cache settings as its CDN for ad segments. To set up an alternate CDN, create a rule in your CDN for the following origin: ads.mediatailor.&lt;region>.amazonaws.com. Then specify the rule's name in this AdSegmentUrlPrefix. When AWS Elemental MediaTailor serves a manifest, it reports your CDN as the source for ad segments.</p>
    #[serde(rename = "AdSegmentUrlPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_segment_url_prefix: Option<String>,
    /// <p>A content delivery network (CDN) to cache content segments, so that content requests don’t always have to go to the origin server. First, create a rule in your CDN for the content segment origin server. Then specify the rule's name in this ContentSegmentUrlPrefix. When AWS Elemental MediaTailor serves a manifest, it reports your CDN as the source for content segments.</p>
    #[serde(rename = "ContentSegmentUrlPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_segment_url_prefix: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeletePlaybackConfigurationRequest {
    /// <p>The identifier for the configuration.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeletePlaybackConfigurationResponse {}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Empty {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetPlaybackConfigurationRequest {
    /// <p>The identifier for the configuration.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetPlaybackConfigurationResponse {
    /// <p>The URL for the ad decision server (ADS). This includes the specification of static parameters and placeholders for dynamic parameters. AWS Elemental MediaTailor substitutes player-specific and session-specific parameters as needed when calling the ADS. Alternately, for testing, you can provide a static VAST URL. The maximum length is 25000 characters.</p>
    #[serde(rename = "AdDecisionServerUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_decision_server_url: Option<String>,
    /// <p>The configuration for using a content delivery network (CDN), like Amazon CloudFront, for content and ad segment management. </p>
    #[serde(rename = "CdnConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdn_configuration: Option<CdnConfiguration>,
    /// <p>The configuration for HLS content. </p>
    #[serde(rename = "HlsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_configuration: Option<HlsConfiguration>,
    /// <p>The identifier for the configuration.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The URL that the player accesses to get a manifest from AWS Elemental MediaTailor. This session will use server-side reporting. </p>
    #[serde(rename = "PlaybackEndpointPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_endpoint_prefix: Option<String>,
    /// <p>The URL that the player uses to initialize a session that uses client-side reporting. </p>
    #[serde(rename = "SessionInitializationEndpointPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_initialization_endpoint_prefix: Option<String>,
    /// <p>URL for a high-quality video asset to transcode and use to fill in time that's not used by ads. AWS Elemental MediaTailor shows the slate to fill in gaps in media content. Configuring the slate is optional for non-VPAID configurations. For VPAID, the slate is required because AWS Elemental MediaTailor provides it in the slots designated for dynamic ad content. The slate must be a high-quality asset that contains both audio and video. </p>
    #[serde(rename = "SlateAdUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slate_ad_url: Option<String>,
    /// <p>The URL prefix for the master playlist for the stream, minus the asset ID. The maximum length is 512 characters.</p>
    #[serde(rename = "VideoContentSourceUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_content_source_url: Option<String>,
}

/// <p>The configuration for HLS content. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct HlsConfiguration {
    /// <p>The URL that is used to initiate a playback session for devices that support Apple HLS. The session uses server-side reporting.</p>
    #[serde(rename = "ManifestEndpointPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_endpoint_prefix: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListPlaybackConfigurationsRequest {
    /// <p>Maximum number of records to return. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Pagination token returned by the GET list request when results overrun the meximum allowed. Use the token to fetch the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListPlaybackConfigurationsResponse {
    /// <p>Array of playback configurations. This may be all of the available configurations or a subset, depending on the settings you provide and on the total number of configurations stored. </p>
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<PlaybackConfiguration>>,
    /// <p>Pagination token returned by the GET list request when results overrun the meximum allowed. Use the token to fetch the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>The AWSMediaTailor configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PlaybackConfiguration {
    /// <p>The URL for the ad decision server (ADS). This includes the specification of static parameters and placeholders for dynamic parameters. AWS Elemental MediaTailor substitutes player-specific and session-specific parameters as needed when calling the ADS. Alternately, for testing, you can provide a static VAST URL. The maximum length is 25000 characters.</p>
    #[serde(rename = "AdDecisionServerUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_decision_server_url: Option<String>,
    /// <p>The configuration for using a content delivery network (CDN), like Amazon CloudFront, for content and ad segment management. </p>
    #[serde(rename = "CdnConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdn_configuration: Option<CdnConfiguration>,
    /// <p>The identifier for the configuration.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>URL for a high-quality video asset to transcode and use to fill in time that's not used by ads. AWS Elemental MediaTailor shows the slate to fill in gaps in media content. Configuring the slate is optional for non-VPAID configurations. For VPAID, the slate is required because AWS Elemental MediaTailor provides it in the slots designated for dynamic ad content. The slate must be a high-quality asset that contains both audio and video. </p>
    #[serde(rename = "SlateAdUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slate_ad_url: Option<String>,
    /// <p>The URL prefix for the master playlist for the stream, minus the asset ID. The maximum length is 512 characters.</p>
    #[serde(rename = "VideoContentSourceUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_content_source_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutPlaybackConfigurationRequest {
    /// <p>The URL for the ad decision server (ADS). This includes the specification of static parameters and placeholders for dynamic parameters. AWS Elemental MediaTailor substitutes player-specific and session-specific parameters as needed when calling the ADS. Alternately, for testing you can provide a static VAST URL. The maximum length is 25000 characters.</p>
    #[serde(rename = "AdDecisionServerUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_decision_server_url: Option<String>,
    /// <p>The configuration for using a content delivery network (CDN), like Amazon CloudFront, for content and ad segment management. </p>
    #[serde(rename = "CdnConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdn_configuration: Option<CdnConfiguration>,
    /// <p>The identifier for the configuration.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The URL for a high-quality video asset to transcode and use to fill in time that's not used by ads. AWS Elemental MediaTailor shows the slate to fill in gaps in media content. Configuring the slate is optional for non-VPAID configurations. For VPAID, the slate is required because AWS Elemental MediaTailor provides it in the slots that are designated for dynamic ad content. The slate must be a high-quality asset that contains both audio and video. </p>
    #[serde(rename = "SlateAdUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slate_ad_url: Option<String>,
    /// <p>The URL prefix for the master playlist for the stream, minus the asset ID. The maximum length is 512 characters.</p>
    #[serde(rename = "VideoContentSourceUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_content_source_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PutPlaybackConfigurationResponse {
    /// <p>The URL for the ad decision server (ADS). This includes the specification of static parameters and placeholders for dynamic parameters. AWS Elemental MediaTailor substitutes player-specific and session-specific parameters as needed when calling the ADS. Alternately, for testing, you can provide a static VAST URL. The maximum length is 25000 characters.</p>
    #[serde(rename = "AdDecisionServerUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_decision_server_url: Option<String>,
    /// <p>The configuration for using a content delivery network (CDN), like Amazon CloudFront, for content and ad segment management. </p>
    #[serde(rename = "CdnConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdn_configuration: Option<CdnConfiguration>,
    /// <p>The configuration for HLS content. </p>
    #[serde(rename = "HlsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_configuration: Option<HlsConfiguration>,
    /// <p>The identifier for the configuration.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The URL that the player accesses to get a manifest from AWS Elemental MediaTailor. This session will use server-side reporting. </p>
    #[serde(rename = "PlaybackEndpointPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_endpoint_prefix: Option<String>,
    /// <p>The URL that the player uses to initialize a session that uses client-side reporting. </p>
    #[serde(rename = "SessionInitializationEndpointPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_initialization_endpoint_prefix: Option<String>,
    /// <p>URL for a high-quality video asset to transcode and use to fill in time that's not used by ads. AWS Elemental MediaTailor shows the slate to fill in gaps in media content. Configuring the slate is optional for non-VPAID configurations. For VPAID, the slate is required because AWS Elemental MediaTailor provides it in the slots designated for dynamic ad content. The slate must be a high-quality asset that contains both audio and video. </p>
    #[serde(rename = "SlateAdUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slate_ad_url: Option<String>,
    /// <p>The URL prefix for the master playlist for the stream, minus the asset ID. The maximum length is 512 characters.</p>
    #[serde(rename = "VideoContentSourceUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_content_source_url: Option<String>,
}

/// Errors returned by DeletePlaybackConfiguration
#[derive(Debug, PartialEq)]
pub enum DeletePlaybackConfigurationError {
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

impl DeletePlaybackConfigurationError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeletePlaybackConfigurationError {
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
                "ValidationException" => {
                    return DeletePlaybackConfigurationError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeletePlaybackConfigurationError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeletePlaybackConfigurationError {
    fn from(err: serde_json::error::Error) -> DeletePlaybackConfigurationError {
        DeletePlaybackConfigurationError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeletePlaybackConfigurationError {
    fn from(err: CredentialsError) -> DeletePlaybackConfigurationError {
        DeletePlaybackConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeletePlaybackConfigurationError {
    fn from(err: HttpDispatchError) -> DeletePlaybackConfigurationError {
        DeletePlaybackConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeletePlaybackConfigurationError {
    fn from(err: io::Error) -> DeletePlaybackConfigurationError {
        DeletePlaybackConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeletePlaybackConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeletePlaybackConfigurationError {
    fn description(&self) -> &str {
        match *self {
            DeletePlaybackConfigurationError::Validation(ref cause) => cause,
            DeletePlaybackConfigurationError::Credentials(ref err) => err.description(),
            DeletePlaybackConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeletePlaybackConfigurationError::ParseError(ref cause) => cause,
            DeletePlaybackConfigurationError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetPlaybackConfiguration
#[derive(Debug, PartialEq)]
pub enum GetPlaybackConfigurationError {
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

impl GetPlaybackConfigurationError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetPlaybackConfigurationError {
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
                "ValidationException" => {
                    return GetPlaybackConfigurationError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetPlaybackConfigurationError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetPlaybackConfigurationError {
    fn from(err: serde_json::error::Error) -> GetPlaybackConfigurationError {
        GetPlaybackConfigurationError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetPlaybackConfigurationError {
    fn from(err: CredentialsError) -> GetPlaybackConfigurationError {
        GetPlaybackConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetPlaybackConfigurationError {
    fn from(err: HttpDispatchError) -> GetPlaybackConfigurationError {
        GetPlaybackConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetPlaybackConfigurationError {
    fn from(err: io::Error) -> GetPlaybackConfigurationError {
        GetPlaybackConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetPlaybackConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetPlaybackConfigurationError {
    fn description(&self) -> &str {
        match *self {
            GetPlaybackConfigurationError::Validation(ref cause) => cause,
            GetPlaybackConfigurationError::Credentials(ref err) => err.description(),
            GetPlaybackConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetPlaybackConfigurationError::ParseError(ref cause) => cause,
            GetPlaybackConfigurationError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListPlaybackConfigurations
#[derive(Debug, PartialEq)]
pub enum ListPlaybackConfigurationsError {
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

impl ListPlaybackConfigurationsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListPlaybackConfigurationsError {
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
                "ValidationException" => {
                    return ListPlaybackConfigurationsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListPlaybackConfigurationsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListPlaybackConfigurationsError {
    fn from(err: serde_json::error::Error) -> ListPlaybackConfigurationsError {
        ListPlaybackConfigurationsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListPlaybackConfigurationsError {
    fn from(err: CredentialsError) -> ListPlaybackConfigurationsError {
        ListPlaybackConfigurationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListPlaybackConfigurationsError {
    fn from(err: HttpDispatchError) -> ListPlaybackConfigurationsError {
        ListPlaybackConfigurationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListPlaybackConfigurationsError {
    fn from(err: io::Error) -> ListPlaybackConfigurationsError {
        ListPlaybackConfigurationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListPlaybackConfigurationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListPlaybackConfigurationsError {
    fn description(&self) -> &str {
        match *self {
            ListPlaybackConfigurationsError::Validation(ref cause) => cause,
            ListPlaybackConfigurationsError::Credentials(ref err) => err.description(),
            ListPlaybackConfigurationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListPlaybackConfigurationsError::ParseError(ref cause) => cause,
            ListPlaybackConfigurationsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by PutPlaybackConfiguration
#[derive(Debug, PartialEq)]
pub enum PutPlaybackConfigurationError {
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

impl PutPlaybackConfigurationError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> PutPlaybackConfigurationError {
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
                "ValidationException" => {
                    return PutPlaybackConfigurationError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return PutPlaybackConfigurationError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for PutPlaybackConfigurationError {
    fn from(err: serde_json::error::Error) -> PutPlaybackConfigurationError {
        PutPlaybackConfigurationError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for PutPlaybackConfigurationError {
    fn from(err: CredentialsError) -> PutPlaybackConfigurationError {
        PutPlaybackConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutPlaybackConfigurationError {
    fn from(err: HttpDispatchError) -> PutPlaybackConfigurationError {
        PutPlaybackConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutPlaybackConfigurationError {
    fn from(err: io::Error) -> PutPlaybackConfigurationError {
        PutPlaybackConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutPlaybackConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutPlaybackConfigurationError {
    fn description(&self) -> &str {
        match *self {
            PutPlaybackConfigurationError::Validation(ref cause) => cause,
            PutPlaybackConfigurationError::Credentials(ref err) => err.description(),
            PutPlaybackConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutPlaybackConfigurationError::ParseError(ref cause) => cause,
            PutPlaybackConfigurationError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the MediaTailor API. MediaTailor clients implement this trait.
pub trait MediaTailor {
    /// <p>Deletes the configuration for the specified name. </p>
    fn delete_playback_configuration(
        &self,
        input: DeletePlaybackConfigurationRequest,
    ) -> RusotoFuture<DeletePlaybackConfigurationResponse, DeletePlaybackConfigurationError>;

    /// <p>Returns the configuration for the specified name. </p>
    fn get_playback_configuration(
        &self,
        input: GetPlaybackConfigurationRequest,
    ) -> RusotoFuture<GetPlaybackConfigurationResponse, GetPlaybackConfigurationError>;

    /// <p>Returns a list of the configurations defined in AWS Elemental MediaTailor. You can specify a max number of configurations to return at a time. The default max is 50. Results are returned in pagefuls. If AWS Elemental MediaTailor has more configurations than the specified max, it provides parameters in the response that you can use to retrieve the next pageful. </p>
    fn list_playback_configurations(
        &self,
        input: ListPlaybackConfigurationsRequest,
    ) -> RusotoFuture<ListPlaybackConfigurationsResponse, ListPlaybackConfigurationsError>;

    /// <p>Adds a new configuration to AWS Elemental MediaTailor.</p>
    fn put_playback_configuration(
        &self,
        input: PutPlaybackConfigurationRequest,
    ) -> RusotoFuture<PutPlaybackConfigurationResponse, PutPlaybackConfigurationError>;
}
/// A client for the MediaTailor API.
pub struct MediaTailorClient {
    client: Client,
    region: region::Region,
}

impl MediaTailorClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> MediaTailorClient {
        MediaTailorClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> MediaTailorClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        MediaTailorClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl MediaTailor for MediaTailorClient {
    /// <p>Deletes the configuration for the specified name. </p>
    fn delete_playback_configuration(
        &self,
        input: DeletePlaybackConfigurationRequest,
    ) -> RusotoFuture<DeletePlaybackConfigurationResponse, DeletePlaybackConfigurationError> {
        let request_uri = format!("/playbackConfiguration/{name}", name = input.name);

        let mut request = SignedRequest::new("DELETE", "mediatailor", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.mediatailor".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<DeletePlaybackConfigurationResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeletePlaybackConfigurationError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns the configuration for the specified name. </p>
    fn get_playback_configuration(
        &self,
        input: GetPlaybackConfigurationRequest,
    ) -> RusotoFuture<GetPlaybackConfigurationResponse, GetPlaybackConfigurationError> {
        let request_uri = format!("/playbackConfiguration/{name}", name = input.name);

        let mut request = SignedRequest::new("GET", "mediatailor", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.mediatailor".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetPlaybackConfigurationResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetPlaybackConfigurationError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns a list of the configurations defined in AWS Elemental MediaTailor. You can specify a max number of configurations to return at a time. The default max is 50. Results are returned in pagefuls. If AWS Elemental MediaTailor has more configurations than the specified max, it provides parameters in the response that you can use to retrieve the next pageful. </p>
    fn list_playback_configurations(
        &self,
        input: ListPlaybackConfigurationsRequest,
    ) -> RusotoFuture<ListPlaybackConfigurationsResponse, ListPlaybackConfigurationsError> {
        let request_uri = "/playbackConfigurations";

        let mut request = SignedRequest::new("GET", "mediatailor", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.mediatailor".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListPlaybackConfigurationsResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListPlaybackConfigurationsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Adds a new configuration to AWS Elemental MediaTailor.</p>
    fn put_playback_configuration(
        &self,
        input: PutPlaybackConfigurationRequest,
    ) -> RusotoFuture<PutPlaybackConfigurationResponse, PutPlaybackConfigurationError> {
        let request_uri = "/playbackConfiguration";

        let mut request = SignedRequest::new("PUT", "mediatailor", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.mediatailor".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<PutPlaybackConfigurationResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutPlaybackConfigurationError::from_response(response))
                }))
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
