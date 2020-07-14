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
use serde_json;
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateTokenRequest {
    /// <p>The unique identifier string for each client. This value should come from the persisted result of the <a>RegisterClient</a> API.</p>
    #[serde(rename = "clientId")]
    pub client_id: String,
    /// <p>A secret string generated for the client. This value should come from the persisted result of the <a>RegisterClient</a> API.</p>
    #[serde(rename = "clientSecret")]
    pub client_secret: String,
    /// <p>The authorization code received from the authorization service. This parameter is required to perform an authorization grant request to get access to a token.</p>
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// <p>Used only when calling this API for the device code grant type. This short-term code is used to identify this authentication attempt. This should come from an in-memory reference to the result of the <a>StartDeviceAuthorization</a> API.</p>
    #[serde(rename = "deviceCode")]
    pub device_code: String,
    /// <p>Supports grant types for authorization code, refresh token, and device code request.</p>
    #[serde(rename = "grantType")]
    pub grant_type: String,
    /// <p>The location of the application that will receive the authorization code. Users authorize the service to send the request to this location.</p>
    #[serde(rename = "redirectUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<String>,
    /// <p>The token used to obtain an access token in the event that the access token is invalid or expired. This token is not issued by the service.</p>
    #[serde(rename = "refreshToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    /// <p>The list of scopes that is defined by the client. Upon authorization, this list is used to restrict permissions when granting an access token.</p>
    #[serde(rename = "scope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateTokenResponse {
    /// <p>An opaque token to access AWS SSO resources assigned to a user.</p>
    #[serde(rename = "accessToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    /// <p>Indicates the time in seconds when an access token will expire.</p>
    #[serde(rename = "expiresIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<i64>,
    /// <p>The identifier of the user that associated with the access token, if present.</p>
    #[serde(rename = "idToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token: Option<String>,
    /// <p>A token that, if present, can be used to refresh a previously issued access token that might have expired.</p>
    #[serde(rename = "refreshToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    /// <p>Used to notify the client that the returned token is an access token. The supported type is <code>BearerToken</code>.</p>
    #[serde(rename = "tokenType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_type: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RegisterClientRequest {
    /// <p>The friendly name of the client.</p>
    #[serde(rename = "clientName")]
    pub client_name: String,
    /// <p>The type of client. The service supports only <code>public</code> as a client type. Anything other than public will be rejected by the service.</p>
    #[serde(rename = "clientType")]
    pub client_type: String,
    /// <p>The list of scopes that are defined by the client. Upon authorization, this list is used to restrict permissions when granting an access token.</p>
    #[serde(rename = "scopes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RegisterClientResponse {
    /// <p>The endpoint where the client can request authorization.</p>
    #[serde(rename = "authorizationEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_endpoint: Option<String>,
    /// <p>The unique identifier string for each client. This client uses this identifier to get authenticated by the service in subsequent calls.</p>
    #[serde(rename = "clientId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// <p>Indicates the time at which the <code>clientId</code> and <code>clientSecret</code> were issued.</p>
    #[serde(rename = "clientIdIssuedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id_issued_at: Option<i64>,
    /// <p>A secret string generated for the client. The client will use this string to get authenticated by the service in subsequent calls.</p>
    #[serde(rename = "clientSecret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    /// <p>Indicates the time at which the <code>clientId</code> and <code>clientSecret</code> will become invalid.</p>
    #[serde(rename = "clientSecretExpiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret_expires_at: Option<i64>,
    /// <p>The endpoint where the client can get an access token.</p>
    #[serde(rename = "tokenEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_endpoint: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartDeviceAuthorizationRequest {
    /// <p>The unique identifier string for the client that is registered with AWS SSO. This value should come from the persisted result of the <a>RegisterClient</a> API operation.</p>
    #[serde(rename = "clientId")]
    pub client_id: String,
    /// <p>A secret string that is generated for the client. This value should come from the persisted result of the <a>RegisterClient</a> API operation.</p>
    #[serde(rename = "clientSecret")]
    pub client_secret: String,
    /// <p>The URL for the AWS SSO user portal. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/userguide/using-the-portal.html">Using the User Portal</a> in the <i>AWS Single Sign-On User Guide</i>.</p>
    #[serde(rename = "startUrl")]
    pub start_url: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartDeviceAuthorizationResponse {
    /// <p>The short-lived code that is used by the device when polling for a session token.</p>
    #[serde(rename = "deviceCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_code: Option<String>,
    /// <p>Indicates the number of seconds in which the verification code will become invalid.</p>
    #[serde(rename = "expiresIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<i64>,
    /// <p>Indicates the number of seconds the client must wait between attempts when polling for a session.</p>
    #[serde(rename = "interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
    /// <p>A one-time user verification code. This is needed to authorize an in-use device.</p>
    #[serde(rename = "userCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_code: Option<String>,
    /// <p>The URI of the verification page that takes the <code>userCode</code> to authorize the device.</p>
    #[serde(rename = "verificationUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_uri: Option<String>,
    /// <p>An alternate URL that the client can use to automatically launch a browser. This process skips the manual step in which the user visits the verification page and enters their code.</p>
    #[serde(rename = "verificationUriComplete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_uri_complete: Option<String>,
}

/// Errors returned by CreateToken
#[derive(Debug, PartialEq)]
pub enum CreateTokenError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>Indicates that a request to authorize a client with an access user session token is pending.</p>
    AuthorizationPending(String),
    /// <p>Indicates that the token issued by the service is expired and is no longer valid.</p>
    ExpiredToken(String),
    /// <p>Indicates that an error from the service occurred while trying to process a request.</p>
    InternalServer(String),
    /// <p>Indicates that the <code>clientId</code> or <code>clientSecret</code> in the request is invalid. For example, this can occur when a client sends an incorrect <code>clientId</code> or an expired <code>clientSecret</code>.</p>
    InvalidClient(String),
    /// <p>Indicates that a request contains an invalid grant. This can occur if a client makes a <a>CreateToken</a> request with an invalid grant type.</p>
    InvalidGrant(String),
    /// <p>Indicates that something is wrong with the input to the request. For example, a required parameter might be missing or out of range.</p>
    InvalidRequest(String),
    /// <p>Indicates that the scope provided in the request is invalid.</p>
    InvalidScope(String),
    /// <p>Indicates that the client is making the request too frequently and is more than the service can handle. </p>
    SlowDown(String),
    /// <p>Indicates that the client is not currently authorized to make the request. This can happen when a <code>clientId</code> is not issued for a public client.</p>
    UnauthorizedClient(String),
    /// <p>Indicates that the grant type in the request is not supported by the service.</p>
    UnsupportedGrantType(String),
}

impl CreateTokenError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateTokenError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateTokenError::AccessDenied(err.msg))
                }
                "AuthorizationPendingException" => {
                    return RusotoError::Service(CreateTokenError::AuthorizationPending(err.msg))
                }
                "ExpiredTokenException" => {
                    return RusotoError::Service(CreateTokenError::ExpiredToken(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(CreateTokenError::InternalServer(err.msg))
                }
                "InvalidClientException" => {
                    return RusotoError::Service(CreateTokenError::InvalidClient(err.msg))
                }
                "InvalidGrantException" => {
                    return RusotoError::Service(CreateTokenError::InvalidGrant(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateTokenError::InvalidRequest(err.msg))
                }
                "InvalidScopeException" => {
                    return RusotoError::Service(CreateTokenError::InvalidScope(err.msg))
                }
                "SlowDownException" => {
                    return RusotoError::Service(CreateTokenError::SlowDown(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(CreateTokenError::UnauthorizedClient(err.msg))
                }
                "UnsupportedGrantTypeException" => {
                    return RusotoError::Service(CreateTokenError::UnsupportedGrantType(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateTokenError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateTokenError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateTokenError::AuthorizationPending(ref cause) => write!(f, "{}", cause),
            CreateTokenError::ExpiredToken(ref cause) => write!(f, "{}", cause),
            CreateTokenError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateTokenError::InvalidClient(ref cause) => write!(f, "{}", cause),
            CreateTokenError::InvalidGrant(ref cause) => write!(f, "{}", cause),
            CreateTokenError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateTokenError::InvalidScope(ref cause) => write!(f, "{}", cause),
            CreateTokenError::SlowDown(ref cause) => write!(f, "{}", cause),
            CreateTokenError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
            CreateTokenError::UnsupportedGrantType(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateTokenError {}
/// Errors returned by RegisterClient
#[derive(Debug, PartialEq)]
pub enum RegisterClientError {
    /// <p>Indicates that an error from the service occurred while trying to process a request.</p>
    InternalServer(String),
    /// <p>Indicates that the client information sent in the request during registration is invalid.</p>
    InvalidClientMetadata(String),
    /// <p>Indicates that something is wrong with the input to the request. For example, a required parameter might be missing or out of range.</p>
    InvalidRequest(String),
    /// <p>Indicates that the scope provided in the request is invalid.</p>
    InvalidScope(String),
}

impl RegisterClientError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RegisterClientError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(RegisterClientError::InternalServer(err.msg))
                }
                "InvalidClientMetadataException" => {
                    return RusotoError::Service(RegisterClientError::InvalidClientMetadata(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(RegisterClientError::InvalidRequest(err.msg))
                }
                "InvalidScopeException" => {
                    return RusotoError::Service(RegisterClientError::InvalidScope(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RegisterClientError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RegisterClientError::InternalServer(ref cause) => write!(f, "{}", cause),
            RegisterClientError::InvalidClientMetadata(ref cause) => write!(f, "{}", cause),
            RegisterClientError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            RegisterClientError::InvalidScope(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RegisterClientError {}
/// Errors returned by StartDeviceAuthorization
#[derive(Debug, PartialEq)]
pub enum StartDeviceAuthorizationError {
    /// <p>Indicates that an error from the service occurred while trying to process a request.</p>
    InternalServer(String),
    /// <p>Indicates that the <code>clientId</code> or <code>clientSecret</code> in the request is invalid. For example, this can occur when a client sends an incorrect <code>clientId</code> or an expired <code>clientSecret</code>.</p>
    InvalidClient(String),
    /// <p>Indicates that something is wrong with the input to the request. For example, a required parameter might be missing or out of range.</p>
    InvalidRequest(String),
    /// <p>Indicates that the client is making the request too frequently and is more than the service can handle. </p>
    SlowDown(String),
    /// <p>Indicates that the client is not currently authorized to make the request. This can happen when a <code>clientId</code> is not issued for a public client.</p>
    UnauthorizedClient(String),
}

impl StartDeviceAuthorizationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartDeviceAuthorizationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(StartDeviceAuthorizationError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidClientException" => {
                    return RusotoError::Service(StartDeviceAuthorizationError::InvalidClient(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StartDeviceAuthorizationError::InvalidRequest(
                        err.msg,
                    ))
                }
                "SlowDownException" => {
                    return RusotoError::Service(StartDeviceAuthorizationError::SlowDown(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(StartDeviceAuthorizationError::UnauthorizedClient(
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
impl fmt::Display for StartDeviceAuthorizationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartDeviceAuthorizationError::InternalServer(ref cause) => write!(f, "{}", cause),
            StartDeviceAuthorizationError::InvalidClient(ref cause) => write!(f, "{}", cause),
            StartDeviceAuthorizationError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StartDeviceAuthorizationError::SlowDown(ref cause) => write!(f, "{}", cause),
            StartDeviceAuthorizationError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartDeviceAuthorizationError {}
/// Trait representing the capabilities of the SSO OIDC API. SSO OIDC clients implement this trait.
#[async_trait]
pub trait SsoOidc {
    /// <p>Creates and returns an access token for the authorized client. The access token issued will be used to fetch short-term credentials for the assigned roles in the AWS account.</p>
    async fn create_token(
        &self,
        input: CreateTokenRequest,
    ) -> Result<CreateTokenResponse, RusotoError<CreateTokenError>>;

    /// <p>Registers a client with AWS SSO. This allows clients to initiate device authorization. The output should be persisted for reuse through many authentication requests.</p>
    async fn register_client(
        &self,
        input: RegisterClientRequest,
    ) -> Result<RegisterClientResponse, RusotoError<RegisterClientError>>;

    /// <p>Initiates device authorization by requesting a pair of verification codes from the authorization service.</p>
    async fn start_device_authorization(
        &self,
        input: StartDeviceAuthorizationRequest,
    ) -> Result<StartDeviceAuthorizationResponse, RusotoError<StartDeviceAuthorizationError>>;
}
/// A client for the SSO OIDC API.
#[derive(Clone)]
pub struct SsoOidcClient {
    client: Client,
    region: region::Region,
}

impl SsoOidcClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> SsoOidcClient {
        SsoOidcClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> SsoOidcClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        SsoOidcClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> SsoOidcClient {
        SsoOidcClient { client, region }
    }
}

#[async_trait]
impl SsoOidc for SsoOidcClient {
    /// <p>Creates and returns an access token for the authorized client. The access token issued will be used to fetch short-term credentials for the assigned roles in the AWS account.</p>
    async fn create_token(
        &self,
        input: CreateTokenRequest,
    ) -> Result<CreateTokenResponse, RusotoError<CreateTokenError>> {
        let request_uri = "/token";

        let mut request = SignedRequest::new("POST", "awsssooidc", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("oidc".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateTokenResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateTokenError::from_response(response))
        }
    }

    /// <p>Registers a client with AWS SSO. This allows clients to initiate device authorization. The output should be persisted for reuse through many authentication requests.</p>
    async fn register_client(
        &self,
        input: RegisterClientRequest,
    ) -> Result<RegisterClientResponse, RusotoError<RegisterClientError>> {
        let request_uri = "/client/register";

        let mut request = SignedRequest::new("POST", "awsssooidc", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("oidc".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<RegisterClientResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(RegisterClientError::from_response(response))
        }
    }

    /// <p>Initiates device authorization by requesting a pair of verification codes from the authorization service.</p>
    async fn start_device_authorization(
        &self,
        input: StartDeviceAuthorizationRequest,
    ) -> Result<StartDeviceAuthorizationResponse, RusotoError<StartDeviceAuthorizationError>> {
        let request_uri = "/device_authorization";

        let mut request = SignedRequest::new("POST", "awsssooidc", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("oidc".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<StartDeviceAuthorizationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StartDeviceAuthorizationError::from_response(response))
        }
    }
}
