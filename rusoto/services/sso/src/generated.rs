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

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
/// <p>Provides information about your AWS account.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AccountInfo {
    /// <p>The identifier of the AWS account that is assigned to the user.</p>
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The display name of the AWS account that is assigned to the user.</p>
    #[serde(rename = "accountName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    /// <p>The email address of the AWS account that is assigned to the user.</p>
    #[serde(rename = "emailAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRoleCredentialsRequest {
    /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>AWS SSO OIDC API Reference Guide</i>.</p>
    #[serde(rename = "accessToken")]
    pub access_token: String,
    /// <p>The identifier for the AWS account that is assigned to the user.</p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The friendly name of the role that is assigned to the user.</p>
    #[serde(rename = "roleName")]
    pub role_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRoleCredentialsResponse {
    /// <p>The credentials for the role that is assigned to the user.</p>
    #[serde(rename = "roleCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_credentials: Option<RoleCredentials>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListAccountRolesRequest {
    /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>AWS SSO OIDC API Reference Guide</i>.</p>
    #[serde(rename = "accessToken")]
    pub access_token: String,
    /// <p>The identifier for the AWS account that is assigned to the user.</p>
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// <p>The number of items that clients can request per page.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The page token from the previous response output when you request subsequent pages.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListAccountRolesResponse {
    /// <p>The page token client that is used to retrieve the list of accounts.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A paginated response with the list of roles and the next token if more results are available.</p>
    #[serde(rename = "roleList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_list: Option<Vec<RoleInfo>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListAccountsRequest {
    /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>AWS SSO OIDC API Reference Guide</i>.</p>
    #[serde(rename = "accessToken")]
    pub access_token: String,
    /// <p>This is the number of items clients can request per page.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>(Optional) When requesting subsequent pages, this is the page token from the previous response output.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListAccountsResponse {
    /// <p>A paginated response with the list of account information and the next token if more results are available.</p>
    #[serde(rename = "accountList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_list: Option<Vec<AccountInfo>>,
    /// <p>The page token client that is used to retrieve the list of accounts.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct LogoutRequest {
    /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>AWS SSO OIDC API Reference Guide</i>.</p>
    #[serde(rename = "accessToken")]
    pub access_token: String,
}

/// <p>Provides information about the role credentials that are assigned to the user.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RoleCredentials {
    /// <p>The identifier used for the temporary security credentials. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_use-resources.html">Using Temporary Security Credentials to Request Access to AWS Resources</a> in the <i>AWS IAM User Guide</i>.</p>
    #[serde(rename = "accessKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    /// <p>The date on which temporary security credentials expire.</p>
    #[serde(rename = "expiration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<i64>,
    /// <p>The key that is used to sign the request. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_use-resources.html">Using Temporary Security Credentials to Request Access to AWS Resources</a> in the <i>AWS IAM User Guide</i>.</p>
    #[serde(rename = "secretAccessKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_access_key: Option<String>,
    /// <p>The token used for temporary credentials. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_use-resources.html">Using Temporary Security Credentials to Request Access to AWS Resources</a> in the <i>AWS IAM User Guide</i>.</p>
    #[serde(rename = "sessionToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_token: Option<String>,
}

/// <p>Provides information about the role that is assigned to the user.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RoleInfo {
    /// <p>The identifier of the AWS account assigned to the user.</p>
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The friendly name of the role that is assigned to the user.</p>
    #[serde(rename = "roleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
}

/// Errors returned by GetRoleCredentials
#[derive(Debug, PartialEq)]
pub enum GetRoleCredentialsError {
    /// <p>Indicates that a problem occurred with the input to the request. For example, a required parameter might be missing or out of range.</p>
    InvalidRequest(String),
    /// <p>The specified resource doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>Indicates that the request is being made too frequently and is more than what the server can handle.</p>
    TooManyRequests(String),
    /// <p>Indicates that the request is not authorized. This can happen due to an invalid access token in the request.</p>
    Unauthorized(String),
}

impl GetRoleCredentialsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetRoleCredentialsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidRequestException" => {
                    return RusotoError::Service(GetRoleCredentialsError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetRoleCredentialsError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetRoleCredentialsError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetRoleCredentialsError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetRoleCredentialsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRoleCredentialsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetRoleCredentialsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetRoleCredentialsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            GetRoleCredentialsError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetRoleCredentialsError {}
/// Errors returned by ListAccountRoles
#[derive(Debug, PartialEq)]
pub enum ListAccountRolesError {
    /// <p>Indicates that a problem occurred with the input to the request. For example, a required parameter might be missing or out of range.</p>
    InvalidRequest(String),
    /// <p>The specified resource doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>Indicates that the request is being made too frequently and is more than what the server can handle.</p>
    TooManyRequests(String),
    /// <p>Indicates that the request is not authorized. This can happen due to an invalid access token in the request.</p>
    Unauthorized(String),
}

impl ListAccountRolesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAccountRolesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidRequestException" => {
                    return RusotoError::Service(ListAccountRolesError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListAccountRolesError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListAccountRolesError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ListAccountRolesError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListAccountRolesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListAccountRolesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListAccountRolesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListAccountRolesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            ListAccountRolesError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListAccountRolesError {}
/// Errors returned by ListAccounts
#[derive(Debug, PartialEq)]
pub enum ListAccountsError {
    /// <p>Indicates that a problem occurred with the input to the request. For example, a required parameter might be missing or out of range.</p>
    InvalidRequest(String),
    /// <p>The specified resource doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>Indicates that the request is being made too frequently and is more than what the server can handle.</p>
    TooManyRequests(String),
    /// <p>Indicates that the request is not authorized. This can happen due to an invalid access token in the request.</p>
    Unauthorized(String),
}

impl ListAccountsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAccountsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidRequestException" => {
                    return RusotoError::Service(ListAccountsError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListAccountsError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListAccountsError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ListAccountsError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListAccountsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListAccountsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListAccountsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListAccountsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            ListAccountsError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListAccountsError {}
/// Errors returned by Logout
#[derive(Debug, PartialEq)]
pub enum LogoutError {
    /// <p>Indicates that a problem occurred with the input to the request. For example, a required parameter might be missing or out of range.</p>
    InvalidRequest(String),
    /// <p>Indicates that the request is being made too frequently and is more than what the server can handle.</p>
    TooManyRequests(String),
    /// <p>Indicates that the request is not authorized. This can happen due to an invalid access token in the request.</p>
    Unauthorized(String),
}

impl LogoutError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<LogoutError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidRequestException" => {
                    return RusotoError::Service(LogoutError::InvalidRequest(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(LogoutError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(LogoutError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for LogoutError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LogoutError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            LogoutError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            LogoutError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for LogoutError {}
/// Trait representing the capabilities of the SSO API. SSO clients implement this trait.
#[async_trait]
pub trait Sso {
    /// <p>Returns the STS short-term credentials for a given role name that is assigned to the user.</p>
    async fn get_role_credentials(
        &self,
        input: GetRoleCredentialsRequest,
    ) -> Result<GetRoleCredentialsResponse, RusotoError<GetRoleCredentialsError>>;

    /// <p>Lists all roles that are assigned to the user for a given AWS account.</p>
    async fn list_account_roles(
        &self,
        input: ListAccountRolesRequest,
    ) -> Result<ListAccountRolesResponse, RusotoError<ListAccountRolesError>>;

    /// <p>Lists all AWS accounts assigned to the user. These AWS accounts are assigned by the administrator of the account. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/userguide/useraccess.html#assignusers">Assign User Access</a> in the <i>AWS SSO User Guide</i>. This operation returns a paginated response.</p>
    async fn list_accounts(
        &self,
        input: ListAccountsRequest,
    ) -> Result<ListAccountsResponse, RusotoError<ListAccountsError>>;

    /// <p>Removes the client- and server-side session that is associated with the user.</p>
    async fn logout(&self, input: LogoutRequest) -> Result<(), RusotoError<LogoutError>>;
}
/// A client for the SSO API.
#[derive(Clone)]
pub struct SsoClient {
    client: Client,
    region: region::Region,
}

impl SsoClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> SsoClient {
        SsoClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> SsoClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        SsoClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> SsoClient {
        SsoClient { client, region }
    }
}

#[async_trait]
impl Sso for SsoClient {
    /// <p>Returns the STS short-term credentials for a given role name that is assigned to the user.</p>
    async fn get_role_credentials(
        &self,
        input: GetRoleCredentialsRequest,
    ) -> Result<GetRoleCredentialsResponse, RusotoError<GetRoleCredentialsError>> {
        let request_uri = "/federation/credentials";

        let mut request = SignedRequest::new("GET", "awsssoportal", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("portal.sso".to_string());

        request.add_header("x-amz-sso_bearer_token", &input.access_token);
        let mut params = Params::new();
        params.put("account_id", &input.account_id);
        params.put("role_name", &input.role_name);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetRoleCredentialsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetRoleCredentialsError::from_response(response))
        }
    }

    /// <p>Lists all roles that are assigned to the user for a given AWS account.</p>
    async fn list_account_roles(
        &self,
        input: ListAccountRolesRequest,
    ) -> Result<ListAccountRolesResponse, RusotoError<ListAccountRolesError>> {
        let request_uri = "/assignment/roles";

        let mut request = SignedRequest::new("GET", "awsssoportal", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("portal.sso".to_string());

        request.add_header("x-amz-sso_bearer_token", &input.access_token);
        let mut params = Params::new();
        params.put("account_id", &input.account_id);
        if let Some(ref x) = input.max_results {
            params.put("max_result", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("next_token", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListAccountRolesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListAccountRolesError::from_response(response))
        }
    }

    /// <p>Lists all AWS accounts assigned to the user. These AWS accounts are assigned by the administrator of the account. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/userguide/useraccess.html#assignusers">Assign User Access</a> in the <i>AWS SSO User Guide</i>. This operation returns a paginated response.</p>
    async fn list_accounts(
        &self,
        input: ListAccountsRequest,
    ) -> Result<ListAccountsResponse, RusotoError<ListAccountsError>> {
        let request_uri = "/assignment/accounts";

        let mut request = SignedRequest::new("GET", "awsssoportal", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("portal.sso".to_string());

        request.add_header("x-amz-sso_bearer_token", &input.access_token);
        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("max_result", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("next_token", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListAccountsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListAccountsError::from_response(response))
        }
    }

    /// <p>Removes the client- and server-side session that is associated with the user.</p>
    async fn logout(&self, input: LogoutRequest) -> Result<(), RusotoError<LogoutError>> {
        let request_uri = "/logout";

        let mut request = SignedRequest::new("POST", "awsssoportal", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("portal.sso".to_string());

        request.add_header("x-amz-sso_bearer_token", &input.access_token);

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
            Err(LogoutError::from_response(response))
        }
    }
}
