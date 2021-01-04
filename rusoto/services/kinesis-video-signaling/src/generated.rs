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
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetIceServerConfigRequest {
    /// <p>The ARN of the signaling channel to be used for the peer-to-peer connection between configured peers. </p>
    #[serde(rename = "ChannelARN")]
    pub channel_arn: String,
    /// <p>Unique identifier for the viewer. Must be unique within the signaling channel.</p>
    #[serde(rename = "ClientId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// <p>Specifies the desired service. Currently, <code>TURN</code> is the only valid value.</p>
    #[serde(rename = "Service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    /// <p>An optional user ID to be associated with the credentials.</p>
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetIceServerConfigResponse {
    /// <p>The list of ICE server information objects.</p>
    #[serde(rename = "IceServerList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ice_server_list: Option<Vec<IceServer>>,
}

/// <p>A structure for the ICE server connection data.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct IceServer {
    /// <p>A password to login to the ICE server.</p>
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// <p>The period of time, in seconds, during which the username and password are valid.</p>
    #[serde(rename = "Ttl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i64>,
    /// <p>An array of URIs, in the form specified in the <a href="https://tools.ietf.org/html/draft-petithuguenin-behave-turn-uris-03">I-D.petithuguenin-behave-turn-uris</a> spec. These URIs provide the different addresses and/or protocols that can be used to reach the TURN server.</p>
    #[serde(rename = "Uris")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uris: Option<Vec<String>>,
    /// <p>A username to login to the ICE server.</p>
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SendAlexaOfferToMasterRequest {
    /// <p>The ARN of the signaling channel by which Alexa and the master peer communicate.</p>
    #[serde(rename = "ChannelARN")]
    pub channel_arn: String,
    /// <p>The base64-encoded SDP offer content.</p>
    #[serde(rename = "MessagePayload")]
    pub message_payload: String,
    /// <p>The unique identifier for the sender client.</p>
    #[serde(rename = "SenderClientId")]
    pub sender_client_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SendAlexaOfferToMasterResponse {
    /// <p>The base64-encoded SDP answer content.</p>
    #[serde(rename = "Answer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer: Option<String>,
}

/// Errors returned by GetIceServerConfig
#[derive(Debug, PartialEq)]
pub enum GetIceServerConfigError {
    /// <p>Your request was throttled because you have exceeded the limit of allowed client calls. Try making the call later.</p>
    ClientLimitExceeded(String),
    /// <p>The value for this input parameter is invalid.</p>
    InvalidArgument(String),
    /// <p>The specified client is invalid.</p>
    InvalidClient(String),
    /// <p>The caller is not authorized to perform this operation.</p>
    NotAuthorized(String),
    /// <p>The specified resource is not found.</p>
    ResourceNotFound(String),
    /// <p>If the client session is expired. Once the client is connected, the session is valid for 45 minutes. Client should reconnect to the channel to continue sending/receiving messages.</p>
    SessionExpired(String),
}

impl GetIceServerConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetIceServerConfigError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "ClientLimitExceededException" => {
                    return RusotoError::Service(GetIceServerConfigError::ClientLimitExceeded(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(GetIceServerConfigError::InvalidArgument(err.msg))
                }
                "InvalidClientException" => {
                    return RusotoError::Service(GetIceServerConfigError::InvalidClient(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(GetIceServerConfigError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetIceServerConfigError::ResourceNotFound(err.msg))
                }
                "SessionExpiredException" => {
                    return RusotoError::Service(GetIceServerConfigError::SessionExpired(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetIceServerConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetIceServerConfigError::ClientLimitExceeded(ref cause) => write!(f, "{}", cause),
            GetIceServerConfigError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            GetIceServerConfigError::InvalidClient(ref cause) => write!(f, "{}", cause),
            GetIceServerConfigError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            GetIceServerConfigError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetIceServerConfigError::SessionExpired(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetIceServerConfigError {}
/// Errors returned by SendAlexaOfferToMaster
#[derive(Debug, PartialEq)]
pub enum SendAlexaOfferToMasterError {
    /// <p>Your request was throttled because you have exceeded the limit of allowed client calls. Try making the call later.</p>
    ClientLimitExceeded(String),
    /// <p>The value for this input parameter is invalid.</p>
    InvalidArgument(String),
    /// <p>The caller is not authorized to perform this operation.</p>
    NotAuthorized(String),
    /// <p>The specified resource is not found.</p>
    ResourceNotFound(String),
}

impl SendAlexaOfferToMasterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SendAlexaOfferToMasterError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "ClientLimitExceededException" => {
                    return RusotoError::Service(SendAlexaOfferToMasterError::ClientLimitExceeded(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(SendAlexaOfferToMasterError::InvalidArgument(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(SendAlexaOfferToMasterError::NotAuthorized(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(SendAlexaOfferToMasterError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SendAlexaOfferToMasterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SendAlexaOfferToMasterError::ClientLimitExceeded(ref cause) => write!(f, "{}", cause),
            SendAlexaOfferToMasterError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            SendAlexaOfferToMasterError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            SendAlexaOfferToMasterError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SendAlexaOfferToMasterError {}
/// Trait representing the capabilities of the Amazon Kinesis Video Signaling Channels API. Amazon Kinesis Video Signaling Channels clients implement this trait.
#[async_trait]
pub trait KinesisVideoSignaling {
    /// <p>Gets the Interactive Connectivity Establishment (ICE) server configuration information, including URIs, username, and password which can be used to configure the WebRTC connection. The ICE component uses this configuration information to setup the WebRTC connection, including authenticating with the Traversal Using Relays around NAT (TURN) relay server. </p> <p>TURN is a protocol that is used to improve the connectivity of peer-to-peer applications. By providing a cloud-based relay service, TURN ensures that a connection can be established even when one or more peers are incapable of a direct peer-to-peer connection. For more information, see <a href="https://tools.ietf.org/html/draft-uberti-rtcweb-turn-rest-00">A REST API For Access To TURN Services</a>.</p> <p> You can invoke this API to establish a fallback mechanism in case either of the peers is unable to establish a direct peer-to-peer connection over a signaling channel. You must specify either a signaling channel ARN or the client ID in order to invoke this API.</p>
    async fn get_ice_server_config(
        &self,
        input: GetIceServerConfigRequest,
    ) -> Result<GetIceServerConfigResponse, RusotoError<GetIceServerConfigError>>;

    /// <p>This API allows you to connect WebRTC-enabled devices with Alexa display devices. When invoked, it sends the Alexa Session Description Protocol (SDP) offer to the master peer. The offer is delivered as soon as the master is connected to the specified signaling channel. This API returns the SDP answer from the connected master. If the master is not connected to the signaling channel, redelivery requests are made until the message expires.</p>
    async fn send_alexa_offer_to_master(
        &self,
        input: SendAlexaOfferToMasterRequest,
    ) -> Result<SendAlexaOfferToMasterResponse, RusotoError<SendAlexaOfferToMasterError>>;
}
/// A client for the Amazon Kinesis Video Signaling Channels API.
#[derive(Clone)]
pub struct KinesisVideoSignalingClient {
    client: Client,
    region: region::Region,
}

impl KinesisVideoSignalingClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> KinesisVideoSignalingClient {
        KinesisVideoSignalingClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> KinesisVideoSignalingClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        KinesisVideoSignalingClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> KinesisVideoSignalingClient {
        KinesisVideoSignalingClient { client, region }
    }
}

#[async_trait]
impl KinesisVideoSignaling for KinesisVideoSignalingClient {
    /// <p>Gets the Interactive Connectivity Establishment (ICE) server configuration information, including URIs, username, and password which can be used to configure the WebRTC connection. The ICE component uses this configuration information to setup the WebRTC connection, including authenticating with the Traversal Using Relays around NAT (TURN) relay server. </p> <p>TURN is a protocol that is used to improve the connectivity of peer-to-peer applications. By providing a cloud-based relay service, TURN ensures that a connection can be established even when one or more peers are incapable of a direct peer-to-peer connection. For more information, see <a href="https://tools.ietf.org/html/draft-uberti-rtcweb-turn-rest-00">A REST API For Access To TURN Services</a>.</p> <p> You can invoke this API to establish a fallback mechanism in case either of the peers is unable to establish a direct peer-to-peer connection over a signaling channel. You must specify either a signaling channel ARN or the client ID in order to invoke this API.</p>
    #[allow(unused_mut)]
    async fn get_ice_server_config(
        &self,
        input: GetIceServerConfigRequest,
    ) -> Result<GetIceServerConfigResponse, RusotoError<GetIceServerConfigError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/v1/get-ice-server-config";

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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetIceServerConfigResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetIceServerConfigError::from_response(response))
        }
    }

    /// <p>This API allows you to connect WebRTC-enabled devices with Alexa display devices. When invoked, it sends the Alexa Session Description Protocol (SDP) offer to the master peer. The offer is delivered as soon as the master is connected to the specified signaling channel. This API returns the SDP answer from the connected master. If the master is not connected to the signaling channel, redelivery requests are made until the message expires.</p>
    #[allow(unused_mut)]
    async fn send_alexa_offer_to_master(
        &self,
        input: SendAlexaOfferToMasterRequest,
    ) -> Result<SendAlexaOfferToMasterResponse, RusotoError<SendAlexaOfferToMasterError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/v1/send-alexa-offer-to-master";

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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<SendAlexaOfferToMasterResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(SendAlexaOfferToMasterError::from_response(response))
        }
    }
}
