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
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde_json;
/// <p>The Amazon Chime account details. An AWS account can have multiple Amazon Chime accounts.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Account {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The Amazon Chime account type. For more information about different account types, see <a href="https://docs.aws.amazon.com/chime/latest/ag/manage-chime-account.html">Managing Your Amazon Chime Accounts</a> in the <i>Amazon Chime Administration Guide</i>.</p>
    #[serde(rename = "AccountType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    /// <p>The AWS account ID.</p>
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// <p>The Amazon Chime account creation timestamp, in ISO 8601 format.</p>
    #[serde(rename = "CreatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    /// <p>The default license for the Amazon Chime account.</p>
    #[serde(rename = "DefaultLicense")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_license: Option<String>,
    /// <p>The Amazon Chime account name.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Supported licenses for the Amazon Chime account.</p>
    #[serde(rename = "SupportedLicenses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_licenses: Option<Vec<String>>,
}

/// <p>Settings related to the Amazon Chime account. This includes settings that start or stop remote control of shared screens, or start or stop the dial-out option in the Amazon Chime web application. For more information about these settings, see <a href="https://docs.aws.amazon.com/chime/latest/ag/policies.html">Use the Policies Page</a> in the <i>Amazon Chime Administration Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccountSettings {
    /// <p>Setting that stops or starts remote control of shared screens during meetings.</p>
    #[serde(rename = "DisableRemoteControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_remote_control: Option<bool>,
    /// <p>Setting that allows meeting participants to choose the <b>Call me at a phone number</b> option. For more information, see <a href="https://docs.aws.amazon.com/chime/latest/ug/chime-join-meeting.html">Join a Meeting without the Amazon Chime App</a>.</p>
    #[serde(rename = "EnableDialOut")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_dial_out: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchSuspendUserRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The request containing the user IDs to suspend.</p>
    #[serde(rename = "UserIdList")]
    pub user_id_list: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct BatchSuspendUserResponse {
    /// <p>If the <a>BatchSuspendUser</a> action fails for one or more of the user IDs in the request, a list of the user IDs is returned, along with error codes and error messages.</p>
    #[serde(rename = "UserErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_errors: Option<Vec<UserError>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchUnsuspendUserRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The request containing the user IDs to unsuspend.</p>
    #[serde(rename = "UserIdList")]
    pub user_id_list: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct BatchUnsuspendUserResponse {
    /// <p>If the <a>BatchUnsuspendUser</a> action fails for one or more of the user IDs in the request, a list of the user IDs is returned, along with error codes and error messages.</p>
    #[serde(rename = "UserErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_errors: Option<Vec<UserError>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchUpdateUserRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The request containing the user IDs and details to update.</p>
    #[serde(rename = "UpdateUserRequestItems")]
    pub update_user_request_items: Vec<UpdateUserRequestItem>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct BatchUpdateUserResponse {
    /// <p>If the <a>BatchUpdateUser</a> action fails for one or more of the user IDs in the request, a list of the user IDs is returned, along with error codes and error messages.</p>
    #[serde(rename = "UserErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_errors: Option<Vec<UserError>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateAccountRequest {
    /// <p>The name of the Amazon Chime account.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateAccountResponse {
    /// <p>The Amazon Chime account details.</p>
    #[serde(rename = "Account")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<Account>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteAccountRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteAccountResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAccountRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetAccountResponse {
    /// <p>The Amazon Chime account details.</p>
    #[serde(rename = "Account")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<Account>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAccountSettingsRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetAccountSettingsResponse {
    /// <p>The Amazon Chime account settings.</p>
    #[serde(rename = "AccountSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_settings: Option<AccountSettings>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetUserRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The user ID.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetUserResponse {
    /// <p>The user details.</p>
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

/// <p>Invitation object returned after emailing users to invite them to join the Amazon Chime <code>Team</code> account.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Invite {
    /// <p>The email address to which the invite is sent.</p>
    #[serde(rename = "EmailAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// <p>The status of the invite email.</p>
    #[serde(rename = "EmailStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_status: Option<String>,
    /// <p>The invite ID.</p>
    #[serde(rename = "InviteId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_id: Option<String>,
    /// <p>The status of the invite.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InviteUsersRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The user email addresses to which to send the invite.</p>
    #[serde(rename = "UserEmailList")]
    pub user_email_list: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct InviteUsersResponse {
    /// <p>The invite details.</p>
    #[serde(rename = "Invites")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invites: Option<Vec<Invite>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListAccountsRequest {
    /// <p>The maximum number of results to return in a single call. Defaults to 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Amazon Chime account name prefix with which to filter results.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The token to use to retrieve the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>User email address with which to filter results.</p>
    #[serde(rename = "UserEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_email: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListAccountsResponse {
    /// <p>List of Amazon Chime accounts and account details.</p>
    #[serde(rename = "Accounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<Account>>,
    /// <p>The token to use to retrieve the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListUsersRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The maximum number of results to return in a single call. Defaults to 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to use to retrieve the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Optional. The user email address used to filter results. Maximum 1.</p>
    #[serde(rename = "UserEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_email: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListUsersResponse {
    /// <p>The token to use to retrieve the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>List of users and user details.</p>
    #[serde(rename = "Users")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<User>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct LogoutUserRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The user ID.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct LogoutUserResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ResetPersonalPINRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The user ID.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ResetPersonalPINResponse {
    /// <p>The user details and new personal meeting PIN.</p>
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateAccountRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The new name for the specified Amazon Chime account.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateAccountResponse {
    /// <p>The updated Amazon Chime account details.</p>
    #[serde(rename = "Account")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<Account>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateAccountSettingsRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The Amazon Chime account settings to update.</p>
    #[serde(rename = "AccountSettings")]
    pub account_settings: AccountSettings,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateAccountSettingsResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateUserRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The user license type to update. This must be a supported license type for the Amazon Chime account that the user belongs to.</p>
    #[serde(rename = "LicenseType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_type: Option<String>,
    /// <p>The user ID.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

/// <p>The user ID and user fields to update, used with the <a>BatchUpdateUser</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateUserRequestItem {
    /// <p>The user license type.</p>
    #[serde(rename = "LicenseType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_type: Option<String>,
    /// <p>The user ID.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateUserResponse {
    /// <p>The updated user details.</p>
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

/// <p>The user on the Amazon Chime account.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct User {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The display name of the user.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>Date and time when the user is invited to the Amazon Chime account, in ISO 8601 format.</p>
    #[serde(rename = "InvitedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_on: Option<f64>,
    /// <p>The license type for the user.</p>
    #[serde(rename = "LicenseType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_type: Option<String>,
    /// <p>The user's personal meeting PIN.</p>
    #[serde(rename = "PersonalPIN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_pin: Option<String>,
    /// <p>The primary email address of the user.</p>
    #[serde(rename = "PrimaryEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_email: Option<String>,
    /// <p>Date and time when the user is registered, in ISO 8601 format.</p>
    #[serde(rename = "RegisteredOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_on: Option<f64>,
    /// <p>The user ID.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
    /// <p>The user invite status.</p>
    #[serde(rename = "UserInvitationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_invitation_status: Option<String>,
    /// <p>The user registration status.</p>
    #[serde(rename = "UserRegistrationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_registration_status: Option<String>,
}

/// <p>The list of errors returned when errors are encountered during the <a>BatchSuspendUser</a>, <a>BatchUnsuspendUser</a>, or <a>BatchUpdateUser</a> actions. This includes user IDs, error codes, and error messages.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UserError {
    /// <p>The error code.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The error message.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The user ID for which the action failed.</p>
    #[serde(rename = "UserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// Errors returned by BatchSuspendUser
#[derive(Debug, PartialEq)]
pub enum BatchSuspendUserError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported region.</p>
    Forbidden(String),
    /// <p>One or more of the resources in the request does not exist in the system.</p>
    NotFound(String),
    /// <p>The service encountered an unexpected error.</p>
    ServiceFailure(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The client exceeded its request rate limit.</p>
    ThrottledClient(String),
    /// <p>The client is not currently authorized to make the request.</p>
    UnauthorizedClient(String),
}

impl BatchSuspendUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchSuspendUserError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(BatchSuspendUserError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(BatchSuspendUserError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(BatchSuspendUserError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(BatchSuspendUserError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(BatchSuspendUserError::ServiceUnavailable(err.msg))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(BatchSuspendUserError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(BatchSuspendUserError::UnauthorizedClient(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for BatchSuspendUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchSuspendUserError {
    fn description(&self) -> &str {
        match *self {
            BatchSuspendUserError::BadRequest(ref cause) => cause,
            BatchSuspendUserError::Forbidden(ref cause) => cause,
            BatchSuspendUserError::NotFound(ref cause) => cause,
            BatchSuspendUserError::ServiceFailure(ref cause) => cause,
            BatchSuspendUserError::ServiceUnavailable(ref cause) => cause,
            BatchSuspendUserError::ThrottledClient(ref cause) => cause,
            BatchSuspendUserError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by BatchUnsuspendUser
#[derive(Debug, PartialEq)]
pub enum BatchUnsuspendUserError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported region.</p>
    Forbidden(String),
    /// <p>One or more of the resources in the request does not exist in the system.</p>
    NotFound(String),
    /// <p>The service encountered an unexpected error.</p>
    ServiceFailure(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The client exceeded its request rate limit.</p>
    ThrottledClient(String),
    /// <p>The client is not currently authorized to make the request.</p>
    UnauthorizedClient(String),
}

impl BatchUnsuspendUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchUnsuspendUserError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(BatchUnsuspendUserError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(BatchUnsuspendUserError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(BatchUnsuspendUserError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(BatchUnsuspendUserError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(BatchUnsuspendUserError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(BatchUnsuspendUserError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(BatchUnsuspendUserError::UnauthorizedClient(
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
impl fmt::Display for BatchUnsuspendUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchUnsuspendUserError {
    fn description(&self) -> &str {
        match *self {
            BatchUnsuspendUserError::BadRequest(ref cause) => cause,
            BatchUnsuspendUserError::Forbidden(ref cause) => cause,
            BatchUnsuspendUserError::NotFound(ref cause) => cause,
            BatchUnsuspendUserError::ServiceFailure(ref cause) => cause,
            BatchUnsuspendUserError::ServiceUnavailable(ref cause) => cause,
            BatchUnsuspendUserError::ThrottledClient(ref cause) => cause,
            BatchUnsuspendUserError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by BatchUpdateUser
#[derive(Debug, PartialEq)]
pub enum BatchUpdateUserError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported region.</p>
    Forbidden(String),
    /// <p>One or more of the resources in the request does not exist in the system.</p>
    NotFound(String),
    /// <p>The service encountered an unexpected error.</p>
    ServiceFailure(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The client exceeded its request rate limit.</p>
    ThrottledClient(String),
    /// <p>The client is not currently authorized to make the request.</p>
    UnauthorizedClient(String),
}

impl BatchUpdateUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchUpdateUserError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(BatchUpdateUserError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(BatchUpdateUserError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(BatchUpdateUserError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(BatchUpdateUserError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(BatchUpdateUserError::ServiceUnavailable(err.msg))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(BatchUpdateUserError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(BatchUpdateUserError::UnauthorizedClient(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for BatchUpdateUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchUpdateUserError {
    fn description(&self) -> &str {
        match *self {
            BatchUpdateUserError::BadRequest(ref cause) => cause,
            BatchUpdateUserError::Forbidden(ref cause) => cause,
            BatchUpdateUserError::NotFound(ref cause) => cause,
            BatchUpdateUserError::ServiceFailure(ref cause) => cause,
            BatchUpdateUserError::ServiceUnavailable(ref cause) => cause,
            BatchUpdateUserError::ThrottledClient(ref cause) => cause,
            BatchUpdateUserError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateAccount
#[derive(Debug, PartialEq)]
pub enum CreateAccountError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported region.</p>
    Forbidden(String),
    /// <p>One or more of the resources in the request does not exist in the system.</p>
    NotFound(String),
    /// <p>The service encountered an unexpected error.</p>
    ServiceFailure(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The client exceeded its request rate limit.</p>
    ThrottledClient(String),
    /// <p>The client is not currently authorized to make the request.</p>
    UnauthorizedClient(String),
}

impl CreateAccountError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateAccountError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateAccountError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateAccountError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateAccountError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(CreateAccountError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateAccountError::ServiceUnavailable(err.msg))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(CreateAccountError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(CreateAccountError::UnauthorizedClient(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateAccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateAccountError {
    fn description(&self) -> &str {
        match *self {
            CreateAccountError::BadRequest(ref cause) => cause,
            CreateAccountError::Forbidden(ref cause) => cause,
            CreateAccountError::NotFound(ref cause) => cause,
            CreateAccountError::ServiceFailure(ref cause) => cause,
            CreateAccountError::ServiceUnavailable(ref cause) => cause,
            CreateAccountError::ThrottledClient(ref cause) => cause,
            CreateAccountError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteAccount
#[derive(Debug, PartialEq)]
pub enum DeleteAccountError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported region.</p>
    Forbidden(String),
    /// <p>One or more of the resources in the request does not exist in the system.</p>
    NotFound(String),
    /// <p>The service encountered an unexpected error.</p>
    ServiceFailure(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The client exceeded its request rate limit.</p>
    ThrottledClient(String),
    /// <p>The client is not currently authorized to make the request.</p>
    UnauthorizedClient(String),
    /// <p>The request was well-formed but was unable to be followed due to semantic errors.</p>
    UnprocessableEntity(String),
}

impl DeleteAccountError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAccountError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteAccountError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteAccountError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteAccountError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(DeleteAccountError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteAccountError::ServiceUnavailable(err.msg))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(DeleteAccountError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(DeleteAccountError::UnauthorizedClient(err.msg))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(DeleteAccountError::UnprocessableEntity(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteAccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteAccountError {
    fn description(&self) -> &str {
        match *self {
            DeleteAccountError::BadRequest(ref cause) => cause,
            DeleteAccountError::Forbidden(ref cause) => cause,
            DeleteAccountError::NotFound(ref cause) => cause,
            DeleteAccountError::ServiceFailure(ref cause) => cause,
            DeleteAccountError::ServiceUnavailable(ref cause) => cause,
            DeleteAccountError::ThrottledClient(ref cause) => cause,
            DeleteAccountError::UnauthorizedClient(ref cause) => cause,
            DeleteAccountError::UnprocessableEntity(ref cause) => cause,
        }
    }
}
/// Errors returned by GetAccount
#[derive(Debug, PartialEq)]
pub enum GetAccountError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported region.</p>
    Forbidden(String),
    /// <p>One or more of the resources in the request does not exist in the system.</p>
    NotFound(String),
    /// <p>The service encountered an unexpected error.</p>
    ServiceFailure(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The client exceeded its request rate limit.</p>
    ThrottledClient(String),
    /// <p>The client is not currently authorized to make the request.</p>
    UnauthorizedClient(String),
}

impl GetAccountError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAccountError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetAccountError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetAccountError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetAccountError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(GetAccountError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetAccountError::ServiceUnavailable(err.msg))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(GetAccountError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(GetAccountError::UnauthorizedClient(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetAccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAccountError {
    fn description(&self) -> &str {
        match *self {
            GetAccountError::BadRequest(ref cause) => cause,
            GetAccountError::Forbidden(ref cause) => cause,
            GetAccountError::NotFound(ref cause) => cause,
            GetAccountError::ServiceFailure(ref cause) => cause,
            GetAccountError::ServiceUnavailable(ref cause) => cause,
            GetAccountError::ThrottledClient(ref cause) => cause,
            GetAccountError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by GetAccountSettings
#[derive(Debug, PartialEq)]
pub enum GetAccountSettingsError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported region.</p>
    Forbidden(String),
    /// <p>One or more of the resources in the request does not exist in the system.</p>
    NotFound(String),
    /// <p>The service encountered an unexpected error.</p>
    ServiceFailure(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The client exceeded its request rate limit.</p>
    ThrottledClient(String),
    /// <p>The client is not currently authorized to make the request.</p>
    UnauthorizedClient(String),
}

impl GetAccountSettingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAccountSettingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetAccountSettingsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetAccountSettingsError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetAccountSettingsError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(GetAccountSettingsError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetAccountSettingsError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(GetAccountSettingsError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(GetAccountSettingsError::UnauthorizedClient(
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
impl fmt::Display for GetAccountSettingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAccountSettingsError {
    fn description(&self) -> &str {
        match *self {
            GetAccountSettingsError::BadRequest(ref cause) => cause,
            GetAccountSettingsError::Forbidden(ref cause) => cause,
            GetAccountSettingsError::NotFound(ref cause) => cause,
            GetAccountSettingsError::ServiceFailure(ref cause) => cause,
            GetAccountSettingsError::ServiceUnavailable(ref cause) => cause,
            GetAccountSettingsError::ThrottledClient(ref cause) => cause,
            GetAccountSettingsError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by GetUser
#[derive(Debug, PartialEq)]
pub enum GetUserError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported region.</p>
    Forbidden(String),
    /// <p>One or more of the resources in the request does not exist in the system.</p>
    NotFound(String),
    /// <p>The service encountered an unexpected error.</p>
    ServiceFailure(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The client exceeded its request rate limit.</p>
    ThrottledClient(String),
    /// <p>The client is not currently authorized to make the request.</p>
    UnauthorizedClient(String),
}

impl GetUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetUserError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetUserError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetUserError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetUserError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(GetUserError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetUserError::ServiceUnavailable(err.msg))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(GetUserError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(GetUserError::UnauthorizedClient(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetUserError {
    fn description(&self) -> &str {
        match *self {
            GetUserError::BadRequest(ref cause) => cause,
            GetUserError::Forbidden(ref cause) => cause,
            GetUserError::NotFound(ref cause) => cause,
            GetUserError::ServiceFailure(ref cause) => cause,
            GetUserError::ServiceUnavailable(ref cause) => cause,
            GetUserError::ThrottledClient(ref cause) => cause,
            GetUserError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by InviteUsers
#[derive(Debug, PartialEq)]
pub enum InviteUsersError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported region.</p>
    Forbidden(String),
    /// <p>One or more of the resources in the request does not exist in the system.</p>
    NotFound(String),
    /// <p>The service encountered an unexpected error.</p>
    ServiceFailure(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The client exceeded its request rate limit.</p>
    ThrottledClient(String),
    /// <p>The client is not currently authorized to make the request.</p>
    UnauthorizedClient(String),
}

impl InviteUsersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<InviteUsersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(InviteUsersError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(InviteUsersError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(InviteUsersError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(InviteUsersError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(InviteUsersError::ServiceUnavailable(err.msg))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(InviteUsersError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(InviteUsersError::UnauthorizedClient(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for InviteUsersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for InviteUsersError {
    fn description(&self) -> &str {
        match *self {
            InviteUsersError::BadRequest(ref cause) => cause,
            InviteUsersError::Forbidden(ref cause) => cause,
            InviteUsersError::NotFound(ref cause) => cause,
            InviteUsersError::ServiceFailure(ref cause) => cause,
            InviteUsersError::ServiceUnavailable(ref cause) => cause,
            InviteUsersError::ThrottledClient(ref cause) => cause,
            InviteUsersError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by ListAccounts
#[derive(Debug, PartialEq)]
pub enum ListAccountsError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported region.</p>
    Forbidden(String),
    /// <p>One or more of the resources in the request does not exist in the system.</p>
    NotFound(String),
    /// <p>The service encountered an unexpected error.</p>
    ServiceFailure(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The client exceeded its request rate limit.</p>
    ThrottledClient(String),
    /// <p>The client is not currently authorized to make the request.</p>
    UnauthorizedClient(String),
}

impl ListAccountsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAccountsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListAccountsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListAccountsError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListAccountsError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(ListAccountsError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListAccountsError::ServiceUnavailable(err.msg))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(ListAccountsError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(ListAccountsError::UnauthorizedClient(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListAccountsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAccountsError {
    fn description(&self) -> &str {
        match *self {
            ListAccountsError::BadRequest(ref cause) => cause,
            ListAccountsError::Forbidden(ref cause) => cause,
            ListAccountsError::NotFound(ref cause) => cause,
            ListAccountsError::ServiceFailure(ref cause) => cause,
            ListAccountsError::ServiceUnavailable(ref cause) => cause,
            ListAccountsError::ThrottledClient(ref cause) => cause,
            ListAccountsError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by ListUsers
#[derive(Debug, PartialEq)]
pub enum ListUsersError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported region.</p>
    Forbidden(String),
    /// <p>One or more of the resources in the request does not exist in the system.</p>
    NotFound(String),
    /// <p>The service encountered an unexpected error.</p>
    ServiceFailure(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The client exceeded its request rate limit.</p>
    ThrottledClient(String),
    /// <p>The client is not currently authorized to make the request.</p>
    UnauthorizedClient(String),
}

impl ListUsersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListUsersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListUsersError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListUsersError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListUsersError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(ListUsersError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListUsersError::ServiceUnavailable(err.msg))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(ListUsersError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(ListUsersError::UnauthorizedClient(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListUsersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListUsersError {
    fn description(&self) -> &str {
        match *self {
            ListUsersError::BadRequest(ref cause) => cause,
            ListUsersError::Forbidden(ref cause) => cause,
            ListUsersError::NotFound(ref cause) => cause,
            ListUsersError::ServiceFailure(ref cause) => cause,
            ListUsersError::ServiceUnavailable(ref cause) => cause,
            ListUsersError::ThrottledClient(ref cause) => cause,
            ListUsersError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by LogoutUser
#[derive(Debug, PartialEq)]
pub enum LogoutUserError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported region.</p>
    Forbidden(String),
    /// <p>One or more of the resources in the request does not exist in the system.</p>
    NotFound(String),
    /// <p>The service encountered an unexpected error.</p>
    ServiceFailure(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The client exceeded its request rate limit.</p>
    ThrottledClient(String),
    /// <p>The client is not currently authorized to make the request.</p>
    UnauthorizedClient(String),
}

impl LogoutUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<LogoutUserError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(LogoutUserError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(LogoutUserError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(LogoutUserError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(LogoutUserError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(LogoutUserError::ServiceUnavailable(err.msg))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(LogoutUserError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(LogoutUserError::UnauthorizedClient(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for LogoutUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for LogoutUserError {
    fn description(&self) -> &str {
        match *self {
            LogoutUserError::BadRequest(ref cause) => cause,
            LogoutUserError::Forbidden(ref cause) => cause,
            LogoutUserError::NotFound(ref cause) => cause,
            LogoutUserError::ServiceFailure(ref cause) => cause,
            LogoutUserError::ServiceUnavailable(ref cause) => cause,
            LogoutUserError::ThrottledClient(ref cause) => cause,
            LogoutUserError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by ResetPersonalPIN
#[derive(Debug, PartialEq)]
pub enum ResetPersonalPINError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported region.</p>
    Forbidden(String),
    /// <p>One or more of the resources in the request does not exist in the system.</p>
    NotFound(String),
    /// <p>The service encountered an unexpected error.</p>
    ServiceFailure(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The client exceeded its request rate limit.</p>
    ThrottledClient(String),
    /// <p>The client is not currently authorized to make the request.</p>
    UnauthorizedClient(String),
}

impl ResetPersonalPINError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ResetPersonalPINError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ResetPersonalPINError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ResetPersonalPINError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ResetPersonalPINError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(ResetPersonalPINError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ResetPersonalPINError::ServiceUnavailable(err.msg))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(ResetPersonalPINError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(ResetPersonalPINError::UnauthorizedClient(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ResetPersonalPINError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ResetPersonalPINError {
    fn description(&self) -> &str {
        match *self {
            ResetPersonalPINError::BadRequest(ref cause) => cause,
            ResetPersonalPINError::Forbidden(ref cause) => cause,
            ResetPersonalPINError::NotFound(ref cause) => cause,
            ResetPersonalPINError::ServiceFailure(ref cause) => cause,
            ResetPersonalPINError::ServiceUnavailable(ref cause) => cause,
            ResetPersonalPINError::ThrottledClient(ref cause) => cause,
            ResetPersonalPINError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateAccount
#[derive(Debug, PartialEq)]
pub enum UpdateAccountError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported region.</p>
    Forbidden(String),
    /// <p>One or more of the resources in the request does not exist in the system.</p>
    NotFound(String),
    /// <p>The service encountered an unexpected error.</p>
    ServiceFailure(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The client exceeded its request rate limit.</p>
    ThrottledClient(String),
    /// <p>The client is not currently authorized to make the request.</p>
    UnauthorizedClient(String),
}

impl UpdateAccountError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateAccountError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateAccountError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateAccountError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateAccountError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(UpdateAccountError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateAccountError::ServiceUnavailable(err.msg))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(UpdateAccountError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(UpdateAccountError::UnauthorizedClient(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateAccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateAccountError {
    fn description(&self) -> &str {
        match *self {
            UpdateAccountError::BadRequest(ref cause) => cause,
            UpdateAccountError::Forbidden(ref cause) => cause,
            UpdateAccountError::NotFound(ref cause) => cause,
            UpdateAccountError::ServiceFailure(ref cause) => cause,
            UpdateAccountError::ServiceUnavailable(ref cause) => cause,
            UpdateAccountError::ThrottledClient(ref cause) => cause,
            UpdateAccountError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateAccountSettings
#[derive(Debug, PartialEq)]
pub enum UpdateAccountSettingsError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The request could not be processed because of conflict in the current state of the resource.</p>
    Conflict(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported region.</p>
    Forbidden(String),
    /// <p>One or more of the resources in the request does not exist in the system.</p>
    NotFound(String),
    /// <p>The service encountered an unexpected error.</p>
    ServiceFailure(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The client exceeded its request rate limit.</p>
    ThrottledClient(String),
    /// <p>The client is not currently authorized to make the request.</p>
    UnauthorizedClient(String),
}

impl UpdateAccountSettingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateAccountSettingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateAccountSettingsError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateAccountSettingsError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateAccountSettingsError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateAccountSettingsError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(UpdateAccountSettingsError::ServiceFailure(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateAccountSettingsError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(UpdateAccountSettingsError::ThrottledClient(
                        err.msg,
                    ))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(UpdateAccountSettingsError::UnauthorizedClient(
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
impl fmt::Display for UpdateAccountSettingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateAccountSettingsError {
    fn description(&self) -> &str {
        match *self {
            UpdateAccountSettingsError::BadRequest(ref cause) => cause,
            UpdateAccountSettingsError::Conflict(ref cause) => cause,
            UpdateAccountSettingsError::Forbidden(ref cause) => cause,
            UpdateAccountSettingsError::NotFound(ref cause) => cause,
            UpdateAccountSettingsError::ServiceFailure(ref cause) => cause,
            UpdateAccountSettingsError::ServiceUnavailable(ref cause) => cause,
            UpdateAccountSettingsError::ThrottledClient(ref cause) => cause,
            UpdateAccountSettingsError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateUser
#[derive(Debug, PartialEq)]
pub enum UpdateUserError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported region.</p>
    Forbidden(String),
    /// <p>One or more of the resources in the request does not exist in the system.</p>
    NotFound(String),
    /// <p>The service encountered an unexpected error.</p>
    ServiceFailure(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The client exceeded its request rate limit.</p>
    ThrottledClient(String),
    /// <p>The client is not currently authorized to make the request.</p>
    UnauthorizedClient(String),
}

impl UpdateUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateUserError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateUserError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateUserError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateUserError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(UpdateUserError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateUserError::ServiceUnavailable(err.msg))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(UpdateUserError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(UpdateUserError::UnauthorizedClient(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateUserError {
    fn description(&self) -> &str {
        match *self {
            UpdateUserError::BadRequest(ref cause) => cause,
            UpdateUserError::Forbidden(ref cause) => cause,
            UpdateUserError::NotFound(ref cause) => cause,
            UpdateUserError::ServiceFailure(ref cause) => cause,
            UpdateUserError::ServiceUnavailable(ref cause) => cause,
            UpdateUserError::ThrottledClient(ref cause) => cause,
            UpdateUserError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon Chime API. Amazon Chime clients implement this trait.
pub trait Chime {
    /// <p>Suspends up to 50 users from a <code>Team</code> or <code>EnterpriseLWA</code> Amazon Chime account. For more information about different account types, see <a href="https://docs.aws.amazon.com/chime/latest/ag/manage-chime-account.html">Managing Your Amazon Chime Accounts</a> in the <i>Amazon Chime Administration Guide</i>.</p> <p>Users suspended from a <code>Team</code> account are dissociated from the account, but they can continue to use Amazon Chime as free users. To remove the suspension from suspended <code>Team</code> account users, invite them to the <code>Team</code> account again. You can use the <a>InviteUsers</a> action to do so.</p> <p>Users suspended from an <code>EnterpriseLWA</code> account are immediately signed out of Amazon Chime and are no longer able to sign in. To remove the suspension from suspended <code>EnterpriseLWA</code> account users, use the <a>BatchUnsuspendUser</a> action. </p> <p>To sign out users without suspending them, use the <a>LogoutUser</a> action.</p>
    fn batch_suspend_user(
        &self,
        input: BatchSuspendUserRequest,
    ) -> RusotoFuture<BatchSuspendUserResponse, BatchSuspendUserError>;

    /// <p>Removes the suspension from up to 50 previously suspended users for the specified Amazon Chime <code>EnterpriseLWA</code> account. Only users on <code>EnterpriseLWA</code> accounts can be unsuspended using this action. For more information about different account types, see <a href="https://docs.aws.amazon.com/chime/latest/ag/manage-chime-account.html">Managing Your Amazon Chime Accounts</a> in the <i>Amazon Chime Administration Guide</i>.</p> <p>Previously suspended users who are unsuspended using this action are returned to <code>Registered</code> status. Users who are not previously suspended are ignored.</p>
    fn batch_unsuspend_user(
        &self,
        input: BatchUnsuspendUserRequest,
    ) -> RusotoFuture<BatchUnsuspendUserResponse, BatchUnsuspendUserError>;

    /// <p>Updates user details within the <a>UpdateUserRequestItem</a> object for up to 20 users for the specified Amazon Chime account. Currently, only <code>LicenseType</code> updates are supported for this action.</p>
    fn batch_update_user(
        &self,
        input: BatchUpdateUserRequest,
    ) -> RusotoFuture<BatchUpdateUserResponse, BatchUpdateUserError>;

    /// <p>Creates an Amazon Chime account under the administrator's AWS account. Only <code>Team</code> account types are currently supported for this action. For more information about different account types, see <a href="https://docs.aws.amazon.com/chime/latest/ag/manage-chime-account.html">Managing Your Amazon Chime Accounts</a> in the <i>Amazon Chime Administration Guide</i>.</p>
    fn create_account(
        &self,
        input: CreateAccountRequest,
    ) -> RusotoFuture<CreateAccountResponse, CreateAccountError>;

    /// <p>Deletes the specified Amazon Chime account. You must suspend all users before deleting a <code>Team</code> account. You can use the <a>BatchSuspendUser</a> action to do so.</p> <p>For <code>EnterpriseLWA</code> and <code>EnterpriseAD</code> accounts, you must release the claimed domains for your Amazon Chime account before deletion. As soon as you release the domain, all users under that account are suspended.</p> <p>Deleted accounts appear in your <code>Disabled</code> accounts list for 90 days. To restore a deleted account from your <code>Disabled</code> accounts list, you must contact AWS Support.</p> <p>After 90 days, deleted accounts are permanently removed from your <code>Disabled</code> accounts list.</p>
    fn delete_account(
        &self,
        input: DeleteAccountRequest,
    ) -> RusotoFuture<DeleteAccountResponse, DeleteAccountError>;

    /// <p>Retrieves details for the specified Amazon Chime account, such as account type and supported licenses.</p>
    fn get_account(
        &self,
        input: GetAccountRequest,
    ) -> RusotoFuture<GetAccountResponse, GetAccountError>;

    /// <p>Retrieves account settings for the specified Amazon Chime account ID, such as remote control and dial out settings. For more information about these settings, see <a href="https://docs.aws.amazon.com/chime/latest/ag/policies.html">Use the Policies Page</a> in the <i>Amazon Chime Administration Guide</i>.</p>
    fn get_account_settings(
        &self,
        input: GetAccountSettingsRequest,
    ) -> RusotoFuture<GetAccountSettingsResponse, GetAccountSettingsError>;

    /// <p>Retrieves details for the specified user ID, such as primary email address, license type, and personal meeting PIN.</p> <p>To retrieve user details with an email address instead of a user ID, use the <a>ListUsers</a> action, and then filter by email address.</p>
    fn get_user(&self, input: GetUserRequest) -> RusotoFuture<GetUserResponse, GetUserError>;

    /// <p>Sends email invites to as many as 50 users, inviting them to the specified Amazon Chime <code>Team</code> account. Only <code>Team</code> account types are currently supported for this action. </p>
    fn invite_users(
        &self,
        input: InviteUsersRequest,
    ) -> RusotoFuture<InviteUsersResponse, InviteUsersError>;

    /// <p>Lists the Amazon Chime accounts under the administrator's AWS account. You can filter accounts by account name prefix. To find out which Amazon Chime account a user belongs to, you can filter by the user's email address, which returns one account result.</p>
    fn list_accounts(
        &self,
        input: ListAccountsRequest,
    ) -> RusotoFuture<ListAccountsResponse, ListAccountsError>;

    /// <p>Lists the users that belong to the specified Amazon Chime account. You can specify an email address to list only the user that the email address belongs to.</p>
    fn list_users(
        &self,
        input: ListUsersRequest,
    ) -> RusotoFuture<ListUsersResponse, ListUsersError>;

    /// <p>Logs out the specified user from all of the devices they are currently logged into.</p>
    fn logout_user(
        &self,
        input: LogoutUserRequest,
    ) -> RusotoFuture<LogoutUserResponse, LogoutUserError>;

    /// <p>Resets the personal meeting PIN for the specified user on an Amazon Chime account. Returns the <a>User</a> object with the updated personal meeting PIN.</p>
    fn reset_personal_pin(
        &self,
        input: ResetPersonalPINRequest,
    ) -> RusotoFuture<ResetPersonalPINResponse, ResetPersonalPINError>;

    /// <p>Updates account details for the specified Amazon Chime account. Currently, only account name updates are supported for this action.</p>
    fn update_account(
        &self,
        input: UpdateAccountRequest,
    ) -> RusotoFuture<UpdateAccountResponse, UpdateAccountError>;

    /// <p>Updates the settings for the specified Amazon Chime account. You can update settings for remote control of shared screens, or for the dial-out option. For more information about these settings, see <a href="https://docs.aws.amazon.com/chime/latest/ag/policies.html">Use the Policies Page</a> in the <i>Amazon Chime Administration Guide</i>.</p>
    fn update_account_settings(
        &self,
        input: UpdateAccountSettingsRequest,
    ) -> RusotoFuture<UpdateAccountSettingsResponse, UpdateAccountSettingsError>;

    /// <p>Updates user details for a specified user ID. Currently, only <code>LicenseType</code> updates are supported for this action.</p>
    fn update_user(
        &self,
        input: UpdateUserRequest,
    ) -> RusotoFuture<UpdateUserResponse, UpdateUserError>;
}
/// A client for the Amazon Chime API.
#[derive(Clone)]
pub struct ChimeClient {
    client: Client,
    region: region::Region,
}

impl ChimeClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ChimeClient {
        ChimeClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ChimeClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        ChimeClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl Chime for ChimeClient {
    /// <p>Suspends up to 50 users from a <code>Team</code> or <code>EnterpriseLWA</code> Amazon Chime account. For more information about different account types, see <a href="https://docs.aws.amazon.com/chime/latest/ag/manage-chime-account.html">Managing Your Amazon Chime Accounts</a> in the <i>Amazon Chime Administration Guide</i>.</p> <p>Users suspended from a <code>Team</code> account are dissociated from the account, but they can continue to use Amazon Chime as free users. To remove the suspension from suspended <code>Team</code> account users, invite them to the <code>Team</code> account again. You can use the <a>InviteUsers</a> action to do so.</p> <p>Users suspended from an <code>EnterpriseLWA</code> account are immediately signed out of Amazon Chime and are no longer able to sign in. To remove the suspension from suspended <code>EnterpriseLWA</code> account users, use the <a>BatchUnsuspendUser</a> action. </p> <p>To sign out users without suspending them, use the <a>LogoutUser</a> action.</p>
    fn batch_suspend_user(
        &self,
        input: BatchSuspendUserRequest,
    ) -> RusotoFuture<BatchSuspendUserResponse, BatchSuspendUserError> {
        let request_uri = format!(
            "/console/accounts/{account_id}/users",
            account_id = input.account_id
        );

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        params.put("operation", "suspend");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<BatchSuspendUserResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(BatchSuspendUserError::from_response(response))),
                )
            }
        })
    }

    /// <p>Removes the suspension from up to 50 previously suspended users for the specified Amazon Chime <code>EnterpriseLWA</code> account. Only users on <code>EnterpriseLWA</code> accounts can be unsuspended using this action. For more information about different account types, see <a href="https://docs.aws.amazon.com/chime/latest/ag/manage-chime-account.html">Managing Your Amazon Chime Accounts</a> in the <i>Amazon Chime Administration Guide</i>.</p> <p>Previously suspended users who are unsuspended using this action are returned to <code>Registered</code> status. Users who are not previously suspended are ignored.</p>
    fn batch_unsuspend_user(
        &self,
        input: BatchUnsuspendUserRequest,
    ) -> RusotoFuture<BatchUnsuspendUserResponse, BatchUnsuspendUserError> {
        let request_uri = format!(
            "/console/accounts/{account_id}/users",
            account_id = input.account_id
        );

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        params.put("operation", "unsuspend");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<BatchUnsuspendUserResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(BatchUnsuspendUserError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates user details within the <a>UpdateUserRequestItem</a> object for up to 20 users for the specified Amazon Chime account. Currently, only <code>LicenseType</code> updates are supported for this action.</p>
    fn batch_update_user(
        &self,
        input: BatchUpdateUserRequest,
    ) -> RusotoFuture<BatchUpdateUserResponse, BatchUpdateUserError> {
        let request_uri = format!(
            "/console/accounts/{account_id}/users",
            account_id = input.account_id
        );

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<BatchUpdateUserResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(BatchUpdateUserError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates an Amazon Chime account under the administrator's AWS account. Only <code>Team</code> account types are currently supported for this action. For more information about different account types, see <a href="https://docs.aws.amazon.com/chime/latest/ag/manage-chime-account.html">Managing Your Amazon Chime Accounts</a> in the <i>Amazon Chime Administration Guide</i>.</p>
    fn create_account(
        &self,
        input: CreateAccountRequest,
    ) -> RusotoFuture<CreateAccountResponse, CreateAccountError> {
        let request_uri = "/console/accounts";

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateAccountResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateAccountError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the specified Amazon Chime account. You must suspend all users before deleting a <code>Team</code> account. You can use the <a>BatchSuspendUser</a> action to do so.</p> <p>For <code>EnterpriseLWA</code> and <code>EnterpriseAD</code> accounts, you must release the claimed domains for your Amazon Chime account before deletion. As soon as you release the domain, all users under that account are suspended.</p> <p>Deleted accounts appear in your <code>Disabled</code> accounts list for 90 days. To restore a deleted account from your <code>Disabled</code> accounts list, you must contact AWS Support.</p> <p>After 90 days, deleted accounts are permanently removed from your <code>Disabled</code> accounts list.</p>
    fn delete_account(
        &self,
        input: DeleteAccountRequest,
    ) -> RusotoFuture<DeleteAccountResponse, DeleteAccountError> {
        let request_uri = format!(
            "/console/accounts/{account_id}",
            account_id = input.account_id
        );

        let mut request = SignedRequest::new("DELETE", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteAccountResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteAccountError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves details for the specified Amazon Chime account, such as account type and supported licenses.</p>
    fn get_account(
        &self,
        input: GetAccountRequest,
    ) -> RusotoFuture<GetAccountResponse, GetAccountError> {
        let request_uri = format!(
            "/console/accounts/{account_id}",
            account_id = input.account_id
        );

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetAccountResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetAccountError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves account settings for the specified Amazon Chime account ID, such as remote control and dial out settings. For more information about these settings, see <a href="https://docs.aws.amazon.com/chime/latest/ag/policies.html">Use the Policies Page</a> in the <i>Amazon Chime Administration Guide</i>.</p>
    fn get_account_settings(
        &self,
        input: GetAccountSettingsRequest,
    ) -> RusotoFuture<GetAccountSettingsResponse, GetAccountSettingsError> {
        let request_uri = format!(
            "/console/accounts/{account_id}/settings",
            account_id = input.account_id
        );

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetAccountSettingsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetAccountSettingsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves details for the specified user ID, such as primary email address, license type, and personal meeting PIN.</p> <p>To retrieve user details with an email address instead of a user ID, use the <a>ListUsers</a> action, and then filter by email address.</p>
    fn get_user(&self, input: GetUserRequest) -> RusotoFuture<GetUserResponse, GetUserError> {
        let request_uri = format!(
            "/console/accounts/{account_id}/users/{user_id}",
            account_id = input.account_id,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetUserResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetUserError::from_response(response))),
                )
            }
        })
    }

    /// <p>Sends email invites to as many as 50 users, inviting them to the specified Amazon Chime <code>Team</code> account. Only <code>Team</code> account types are currently supported for this action. </p>
    fn invite_users(
        &self,
        input: InviteUsersRequest,
    ) -> RusotoFuture<InviteUsersResponse, InviteUsersError> {
        let request_uri = format!(
            "/console/accounts/{account_id}/users",
            account_id = input.account_id
        );

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        params.put("operation", "add");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<InviteUsersResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(InviteUsersError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists the Amazon Chime accounts under the administrator's AWS account. You can filter accounts by account name prefix. To find out which Amazon Chime account a user belongs to, you can filter by the user's email address, which returns one account result.</p>
    fn list_accounts(
        &self,
        input: ListAccountsRequest,
    ) -> RusotoFuture<ListAccountsResponse, ListAccountsError> {
        let request_uri = "/console/accounts";

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("max-results", x);
        }
        if let Some(ref x) = input.name {
            params.put("name", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("next-token", x);
        }
        if let Some(ref x) = input.user_email {
            params.put("user-email", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListAccountsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListAccountsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists the users that belong to the specified Amazon Chime account. You can specify an email address to list only the user that the email address belongs to.</p>
    fn list_users(
        &self,
        input: ListUsersRequest,
    ) -> RusotoFuture<ListUsersResponse, ListUsersError> {
        let request_uri = format!(
            "/console/accounts/{account_id}/users",
            account_id = input.account_id
        );

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("max-results", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("next-token", x);
        }
        if let Some(ref x) = input.user_email {
            params.put("user-email", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListUsersResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListUsersError::from_response(response))),
                )
            }
        })
    }

    /// <p>Logs out the specified user from all of the devices they are currently logged into.</p>
    fn logout_user(
        &self,
        input: LogoutUserRequest,
    ) -> RusotoFuture<LogoutUserResponse, LogoutUserError> {
        let request_uri = format!(
            "/console/accounts/{account_id}/users/{user_id}",
            account_id = input.account_id,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("operation", "logout");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<LogoutUserResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(LogoutUserError::from_response(response))),
                )
            }
        })
    }

    /// <p>Resets the personal meeting PIN for the specified user on an Amazon Chime account. Returns the <a>User</a> object with the updated personal meeting PIN.</p>
    fn reset_personal_pin(
        &self,
        input: ResetPersonalPINRequest,
    ) -> RusotoFuture<ResetPersonalPINResponse, ResetPersonalPINError> {
        let request_uri = format!(
            "/console/accounts/{account_id}/users/{user_id}",
            account_id = input.account_id,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("operation", "reset-personal-pin");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ResetPersonalPINResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ResetPersonalPINError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates account details for the specified Amazon Chime account. Currently, only account name updates are supported for this action.</p>
    fn update_account(
        &self,
        input: UpdateAccountRequest,
    ) -> RusotoFuture<UpdateAccountResponse, UpdateAccountError> {
        let request_uri = format!(
            "/console/accounts/{account_id}",
            account_id = input.account_id
        );

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateAccountResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateAccountError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates the settings for the specified Amazon Chime account. You can update settings for remote control of shared screens, or for the dial-out option. For more information about these settings, see <a href="https://docs.aws.amazon.com/chime/latest/ag/policies.html">Use the Policies Page</a> in the <i>Amazon Chime Administration Guide</i>.</p>
    fn update_account_settings(
        &self,
        input: UpdateAccountSettingsRequest,
    ) -> RusotoFuture<UpdateAccountSettingsResponse, UpdateAccountSettingsError> {
        let request_uri = format!(
            "/console/accounts/{account_id}/settings",
            account_id = input.account_id
        );

        let mut request = SignedRequest::new("PUT", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateAccountSettingsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateAccountSettingsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Updates user details for a specified user ID. Currently, only <code>LicenseType</code> updates are supported for this action.</p>
    fn update_user(
        &self,
        input: UpdateUserRequest,
    ) -> RusotoFuture<UpdateUserResponse, UpdateUserError> {
        let request_uri = format!(
            "/console/accounts/{account_id}/users/{user_id}",
            account_id = input.account_id,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateUserResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateUserError::from_response(response))),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
