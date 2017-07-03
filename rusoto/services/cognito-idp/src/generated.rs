
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

use std::fmt;
use std::error::Error;

use rusoto_core::region;
use rusoto_core::request::{DispatchSignedRequest, HttpDispatchError};
use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};

use serde_json;
use rusoto_core::signature::SignedRequest;
use serde_json::Value as SerdeJsonValue;
use serde_json::from_str;
#[doc="<p>Represents the request to add custom attributes.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct AddCustomAttributesRequest {
    #[doc="<p>An array of custom attributes, such as Mutable and Name.</p>"]
    #[serde(rename="CustomAttributes")]
    pub custom_attributes: Vec<SchemaAttributeType>,
    #[doc="<p>The user pool ID for the user pool where you want to add custom attributes.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
}

#[doc="<p>Represents the response from the server for the request to add custom attributes.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct AddCustomAttributesResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct AdminAddUserToGroupRequest {
    #[doc="<p>The group name.</p>"]
    #[serde(rename="GroupName")]
    pub group_name: String,
    #[doc="<p>The user pool ID for the user pool.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
    #[doc="<p>The username for the user.</p>"]
    #[serde(rename="Username")]
    pub username: String,
}

#[doc="<p>Represents the request to confirm user registration.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct AdminConfirmSignUpRequest {
    #[doc="<p>The user pool ID for which you want to confirm user registration.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
    #[doc="<p>The user name for which you want to confirm user registration.</p>"]
    #[serde(rename="Username")]
    pub username: String,
}

#[doc="<p>Represents the response from the server for the request to confirm registration.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct AdminConfirmSignUpResponse;

#[doc="<p>The type of configuration for creating a new user profile.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct AdminCreateUserConfigType {
    #[doc="<p>Set to <code>True</code> if only the administrator is allowed to create user profiles. Set to <code>False</code> if users can sign themselves up via an app.</p>"]
    #[serde(rename="AllowAdminCreateUserOnly")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_admin_create_user_only: Option<bool>,
    #[doc="<p>The message template to be used for the welcome message to new users.</p>"]
    #[serde(rename="InviteMessageTemplate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub invite_message_template: Option<MessageTemplateType>,
    #[doc="<p>The user account expiration limit, in days, after which the account is no longer usable. To reset the account after that time limit, you must call <code>AdminCreateUser</code> again, specifying <code>\"RESEND\"</code> for the <code>MessageAction</code> parameter. The default value for this parameter is 7.</p>"]
    #[serde(rename="UnusedAccountValidityDays")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub unused_account_validity_days: Option<i64>,
}

#[doc="<p>Represents the request to create a user in the specified user pool.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct AdminCreateUserRequest {
    #[doc="<p>Specify <code>\"EMAIL\"</code> if email will be used to send the welcome message. Specify <code>\"SMS\"</code> if the phone number will be used. The default value is <code>\"SMS\"</code>. More than one value can be specified.</p>"]
    #[serde(rename="DesiredDeliveryMediums")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub desired_delivery_mediums: Option<Vec<String>>,
    #[doc="<p>This parameter is only used if the <code>phone_number_verified</code> or <code>email_verified</code> attribute is set to <code>True</code>. Otherwise, it is ignored.</p> <p>If this parameter is set to <code>True</code> and the phone number or email address specified in the UserAttributes parameter already exists as an alias with a different user, the API call will migrate the alias from the previous user to the newly created user. The previous user will no longer be able to log in using that alias.</p> <p>If this parameter is set to <code>False</code>, the API throws an <code>AliasExistsException</code> error if the alias already exists. The default value is <code>False</code>.</p>"]
    #[serde(rename="ForceAliasCreation")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub force_alias_creation: Option<bool>,
    #[doc="<p>Set to <code>\"RESEND\"</code> to resend the invitation message to a user that already exists and reset the expiration limit on the user's account. Set to <code>\"SUPPRESS\"</code> to suppress sending the message. Only one value can be specified.</p>"]
    #[serde(rename="MessageAction")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message_action: Option<String>,
    #[doc="<p>The user's temporary password. This password must conform to the password policy that you specified when you created the user pool.</p> <p>The temporary password is valid only once. To complete the Admin Create User flow, the user must enter the temporary password in the sign-in page along with a new password to be used in all future sign-ins.</p> <p>This parameter is not required. If you do not specify a value, Amazon Cognito generates one for you.</p> <p>The temporary password can only be used until the user account expiration limit that you specified when you created the user pool. To reset the account after that time limit, you must call <code>AdminCreateUser</code> again, specifying <code>\"RESEND\"</code> for the <code>MessageAction</code> parameter.</p>"]
    #[serde(rename="TemporaryPassword")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub temporary_password: Option<String>,
    #[doc="<p>An array of name-value pairs that contain user attributes and attribute values to be set for the user to be created. You can create a user without specifying any attributes other than <code>Username</code>. However, any attributes that you specify as required (in <a href=\"API_CreateUserPool.html\">CreateUserPool</a> or in the <b>Attributes</b> tab of the console) must be supplied either by you (in your call to <code>AdminCreateUser</code>) or by the user (when he or she signs up in response to your welcome message).</p> <p>For custom attributes, you must prepend the <code>custom:</code> prefix to the attribute name.</p> <p>To send a message inviting the user to sign up, you must specify the user's email address or phone number. This can be done in your call to AdminCreateUser or in the <b>Users</b> tab of the Amazon Cognito console for managing your user pools.</p> <p>In your call to <code>AdminCreateUser</code>, you can set the <code>email_verified</code> attribute to <code>True</code>, and you can set the <code>phone_number_verified</code> attribute to <code>True</code>. (You can also do this by calling <a href=\"API_AdminUpdateUserAttributes.html\">AdminUpdateUserAttributes</a>.)</p> <ul> <li> <p> <b>email</b>: The email address of the user to whom the message that contains the code and username will be sent. Required if the <code>email_verified</code> attribute is set to <code>True</code>, or if <code>\"EMAIL\"</code> is specified in the <code>DesiredDeliveryMediums</code> parameter.</p> </li> <li> <p> <b>phone_number</b>: The phone number of the user to whom the message that contains the code and username will be sent. Required if the <code>phone_number_verified</code> attribute is set to <code>True</code>, or if <code>\"SMS\"</code> is specified in the <code>DesiredDeliveryMediums</code> parameter.</p> </li> </ul>"]
    #[serde(rename="UserAttributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_attributes: Option<Vec<AttributeType>>,
    #[doc="<p>The user pool ID for the user pool where the user will be created.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
    #[doc="<p>The username for the user. Must be unique within the user pool. Must be a UTF-8 string between 1 and 128 characters. After the user is created, the username cannot be changed.</p>"]
    #[serde(rename="Username")]
    pub username: String,
    #[doc="<p>The user's validation data. This is an array of name-value pairs that contain user attributes and attribute values that you can use for custom validation, such as restricting the types of user accounts that can be registered. For example, you might choose to allow or disallow user sign-up based on the user's domain.</p> <p>To configure custom validation, you must create a Pre Sign-up Lambda trigger for the user pool as described in the Amazon Cognito Developer Guide. The Lambda trigger receives the validation data and uses it in the validation process.</p> <p>The user's validation data is not persisted.</p>"]
    #[serde(rename="ValidationData")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub validation_data: Option<Vec<AttributeType>>,
}

#[doc="<p>Represents the response from the server to the request to create the user.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct AdminCreateUserResponse {
    #[doc="<p>The user returned in the request to create a new user.</p>"]
    #[serde(rename="User")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<UserType>,
}

#[doc="<p>Represents the request to delete user attributes as an administrator.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct AdminDeleteUserAttributesRequest {
    #[doc="<p>An array of strings representing the user attribute names you wish to delete.</p> <p>For custom attributes, you must prepend the <code>custom:</code> prefix to the attribute name.</p>"]
    #[serde(rename="UserAttributeNames")]
    pub user_attribute_names: Vec<String>,
    #[doc="<p>The user pool ID for the user pool where you want to delete user attributes.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
    #[doc="<p>The user name of the user from which you would like to delete attributes.</p>"]
    #[serde(rename="Username")]
    pub username: String,
}

#[doc="<p>Represents the response received from the server for a request to delete user attributes.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct AdminDeleteUserAttributesResponse;

#[doc="<p>Represents the request to delete a user as an administrator.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct AdminDeleteUserRequest {
    #[doc="<p>The user pool ID for the user pool where you want to delete the user.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
    #[doc="<p>The user name of the user you wish to delete.</p>"]
    #[serde(rename="Username")]
    pub username: String,
}

#[doc="<p>Represents the request to disable any user as an administrator.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct AdminDisableUserRequest {
    #[doc="<p>The user pool ID for the user pool where you want to disable the user.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
    #[doc="<p>The user name of the user you wish to disable.</p>"]
    #[serde(rename="Username")]
    pub username: String,
}

#[doc="<p>Represents the response received from the server to disable the user as an administrator.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct AdminDisableUserResponse;

#[doc="<p>Represents the request that enables the user as an administrator.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct AdminEnableUserRequest {
    #[doc="<p>The user pool ID for the user pool where you want to enable the user.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
    #[doc="<p>The user name of the user you wish to enable.</p>"]
    #[serde(rename="Username")]
    pub username: String,
}

#[doc="<p>Represents the response from the server for the request to enable a user as an administrator.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct AdminEnableUserResponse;

#[doc="<p>Sends the forgot device request, as an administrator.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct AdminForgetDeviceRequest {
    #[doc="<p>The device key.</p>"]
    #[serde(rename="DeviceKey")]
    pub device_key: String,
    #[doc="<p>The user pool ID.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
    #[doc="<p>The user name.</p>"]
    #[serde(rename="Username")]
    pub username: String,
}

#[doc="<p>Represents the request to get the device, as an administrator.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct AdminGetDeviceRequest {
    #[doc="<p>The device key.</p>"]
    #[serde(rename="DeviceKey")]
    pub device_key: String,
    #[doc="<p>The user pool ID.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
    #[doc="<p>The user name.</p>"]
    #[serde(rename="Username")]
    pub username: String,
}

#[doc="<p>Gets the device response, as an administrator.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct AdminGetDeviceResponse {
    #[doc="<p>The device.</p>"]
    #[serde(rename="Device")]
    pub device: DeviceType,
}

#[doc="<p>Represents the request to get the specified user as an administrator.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct AdminGetUserRequest {
    #[doc="<p>The user pool ID for the user pool where you want to get information about the user.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
    #[doc="<p>The user name of the user you wish to retrieve.</p>"]
    #[serde(rename="Username")]
    pub username: String,
}

#[doc="<p>Represents the response from the server from the request to get the specified user as an administrator.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct AdminGetUserResponse {
    #[doc="<p>Indicates that the status is enabled.</p>"]
    #[serde(rename="Enabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enabled: Option<bool>,
    #[doc="<p>Specifies the options for MFA (e.g., email or phone number).</p>"]
    #[serde(rename="MFAOptions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mfa_options: Option<Vec<MFAOptionType>>,
    #[doc="<p>An array of name-value pairs representing user attributes.</p>"]
    #[serde(rename="UserAttributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_attributes: Option<Vec<AttributeType>>,
    #[doc="<p>The date the user was created.</p>"]
    #[serde(rename="UserCreateDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_create_date: Option<f64>,
    #[doc="<p>The date the user was last modified.</p>"]
    #[serde(rename="UserLastModifiedDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_last_modified_date: Option<f64>,
    #[doc="<p>The user status. Can be one of the following:</p> <ul> <li> <p>UNCONFIRMED - User has been created but not confirmed.</p> </li> <li> <p>CONFIRMED - User has been confirmed.</p> </li> <li> <p>ARCHIVED - User is no longer active.</p> </li> <li> <p>COMPROMISED - User is disabled due to a potential security threat.</p> </li> <li> <p>UNKNOWN - User status is not known.</p> </li> </ul>"]
    #[serde(rename="UserStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_status: Option<String>,
    #[doc="<p>The user name of the user about whom you are receiving information.</p>"]
    #[serde(rename="Username")]
    pub username: String,
}

#[doc="<p>Initiates the authorization request, as an administrator.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct AdminInitiateAuthRequest {
    #[doc="<p>The authentication flow for this call to execute. The API action will depend on this value. For example:</p> <ul> <li> <p> <code>REFRESH_TOKEN_AUTH</code> will take in a valid refresh token and return new tokens.</p> </li> <li> <p> <code>USER_SRP_AUTH</code> will take in <code>USERNAME</code> and <code>SRPA</code> and return the SRP variables to be used for next challenge execution.</p> </li> </ul> <p>Valid values include:</p> <ul> <li> <p> <code>USER_SRP_AUTH</code>: Authentication flow for the Secure Remote Password (SRP) protocol.</p> </li> <li> <p> <code>REFRESH_TOKEN_AUTH</code>/<code>REFRESH_TOKEN</code>: Authentication flow for refreshing the access token and ID token by supplying a valid refresh token.</p> </li> <li> <p> <code>CUSTOM_AUTH</code>: Custom authentication flow.</p> </li> <li> <p> <code>ADMIN_NO_SRP_AUTH</code>: Non-SRP authentication flow; you can pass in the USERNAME and PASSWORD directly if the flow is enabled for calling the app client.</p> </li> </ul>"]
    #[serde(rename="AuthFlow")]
    pub auth_flow: String,
    #[doc="<p>The authentication parameters. These are inputs corresponding to the <code>AuthFlow</code> that you are invoking. The required values depend on the value of <code>AuthFlow</code>:</p> <ul> <li> <p>For <code>USER_SRP_AUTH</code>: <code>USERNAME</code> (required), <code>SRPA</code> (required), <code>SECRET_HASH</code> (required if the app client is configured with a client secret), <code>DEVICE_KEY</code> </p> </li> <li> <p>For <code>REFRESH_TOKEN_AUTH/REFRESH_TOKEN</code>: <code>USERNAME</code> (required), <code>SECRET_HASH</code> (required if the app client is configured with a client secret), <code>REFRESH_TOKEN</code> (required), <code>DEVICE_KEY</code> </p> </li> <li> <p>For <code>ADMIN_NO_SRP_AUTH</code>: <code>USERNAME</code> (required), <code>SECRET_HASH</code> (if app client is configured with client secret), <code>PASSWORD</code> (required), <code>DEVICE_KEY</code> </p> </li> <li> <p>For <code>CUSTOM_AUTH</code>: <code>USERNAME</code> (required), <code>SECRET_HASH</code> (if app client is configured with client secret), <code>DEVICE_KEY</code> </p> </li> </ul>"]
    #[serde(rename="AuthParameters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auth_parameters: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>The app client ID.</p>"]
    #[serde(rename="ClientId")]
    pub client_id: String,
    #[doc="<p>This is a random key-value pair map which can contain any key and will be passed to your PreAuthentication Lambda trigger as-is. It can be used to implement additional validations around authentication.</p>"]
    #[serde(rename="ClientMetadata")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub client_metadata: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>The ID of the Amazon Cognito user pool.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
}

#[doc="<p>Initiates the authentication response, as an administrator.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct AdminInitiateAuthResponse {
    #[doc="<p>The result of the authentication response. This is only returned if the caller does not need to pass another challenge. If the caller does need to pass another challenge before it gets tokens, <code>ChallengeName</code>, <code>ChallengeParameters</code>, and <code>Session</code> are returned.</p>"]
    #[serde(rename="AuthenticationResult")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub authentication_result: Option<AuthenticationResultType>,
    #[doc="<p>The name of the challenge which you are responding to with this call. This is returned to you in the <code>AdminInitiateAuth</code> response if you need to pass another challenge.</p> <ul> <li> <p> <code>SMS_MFA</code>: Next challenge is to supply an <code>SMS_MFA_CODE</code>, delivered via SMS.</p> </li> <li> <p> <code>PASSWORD_VERIFIER</code>: Next challenge is to supply <code>PASSWORD_CLAIM_SIGNATURE</code>, <code>PASSWORD_CLAIM_SECRET_BLOCK</code>, and <code>TIMESTAMP</code> after the client-side SRP calculations.</p> </li> <li> <p> <code>CUSTOM_CHALLENGE</code>: This is returned if your custom authentication flow determines that the user should pass another challenge before tokens are issued.</p> </li> <li> <p> <code>DEVICE_SRP_AUTH</code>: If device tracking was enabled on your user pool and the previous challenges were passed, this challenge is returned so that Amazon Cognito can start tracking this device.</p> </li> <li> <p> <code>DEVICE_PASSWORD_VERIFIER</code>: Similar to <code>PASSWORD_VERIFIER</code>, but for devices only.</p> </li> <li> <p> <code>ADMIN_NO_SRP_AUTH</code>: This is returned if you need to authenticate with <code>USERNAME</code> and <code>PASSWORD</code> directly. An app client must be enabled to use this flow.</p> </li> <li> <p> <code>NEW_PASSWORD_REQUIRED</code>: For users which are required to change their passwords after successful first login. This challenge should be passed with <code>NEW_PASSWORD</code> and any other required attributes.</p> </li> </ul>"]
    #[serde(rename="ChallengeName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub challenge_name: Option<String>,
    #[doc="<p>The challenge parameters. These are returned to you in the <code>AdminInitiateAuth</code> response if you need to pass another challenge. The responses in this parameter should be used to compute inputs to the next call (<code>AdminRespondToAuthChallenge</code>).</p> <p>All challenges require <code>USERNAME</code> and <code>SECRET_HASH</code> (if applicable).</p> <p>The value of the <code>USER_IF_FOR_SRP</code> attribute will be the user's actual username, not an alias (such as email address or phone number), even if you specified an alias in your call to <code>AdminInitiateAuth</code>. This is because, in the <code>AdminRespondToAuthChallenge</code> API <code>ChallengeResponses</code>, the <code>USERNAME</code> attribute cannot be an alias.</p>"]
    #[serde(rename="ChallengeParameters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub challenge_parameters: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>The session which should be passed both ways in challenge-response calls to the service. If <code>AdminInitiateAuth</code> or <code>AdminRespondToAuthChallenge</code> API call determines that the caller needs to go through another challenge, they return a session with other challenge parameters. This session should be passed as it is to the next <code>AdminRespondToAuthChallenge</code> API call.</p>"]
    #[serde(rename="Session")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub session: Option<String>,
}

#[doc="<p>Represents the request to list devices, as an administrator.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct AdminListDevicesRequest {
    #[doc="<p>The limit of the devices request.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<i64>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="PaginationToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pagination_token: Option<String>,
    #[doc="<p>The user pool ID.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
    #[doc="<p>The user name.</p>"]
    #[serde(rename="Username")]
    pub username: String,
}

#[doc="<p>Lists the device's response, as an administrator.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct AdminListDevicesResponse {
    #[doc="<p>The devices in the list of devices response.</p>"]
    #[serde(rename="Devices")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub devices: Option<Vec<DeviceType>>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="PaginationToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pagination_token: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct AdminListGroupsForUserRequest {
    #[doc="<p>The limit of the request to list groups.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<i64>,
    #[doc="<p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The user pool ID for the user pool.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
    #[doc="<p>The username for the user.</p>"]
    #[serde(rename="Username")]
    pub username: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct AdminListGroupsForUserResponse {
    #[doc="<p>The groups that the user belongs to.</p>"]
    #[serde(rename="Groups")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub groups: Option<Vec<GroupType>>,
    #[doc="<p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct AdminRemoveUserFromGroupRequest {
    #[doc="<p>The group name.</p>"]
    #[serde(rename="GroupName")]
    pub group_name: String,
    #[doc="<p>The user pool ID for the user pool.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
    #[doc="<p>The username for the user.</p>"]
    #[serde(rename="Username")]
    pub username: String,
}

#[doc="<p>Represents the request to reset a user's password as an administrator.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct AdminResetUserPasswordRequest {
    #[doc="<p>The user pool ID for the user pool where you want to reset the user's password.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
    #[doc="<p>The user name of the user whose password you wish to reset.</p>"]
    #[serde(rename="Username")]
    pub username: String,
}

#[doc="<p>Represents the response from the server to reset a user password as an administrator.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct AdminResetUserPasswordResponse;

#[doc="<p>The request to respond to the authentication challenge, as an administrator.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct AdminRespondToAuthChallengeRequest {
    #[doc="<p>The challenge name. For more information, see <a href=\"API_AdminInitiateAuth.html\">AdminInitiateAuth</a>.</p>"]
    #[serde(rename="ChallengeName")]
    pub challenge_name: String,
    #[doc="<p>The challenge responses. These are inputs corresponding to the value of <code>ChallengeName</code>, for example:</p> <ul> <li> <p> <code>SMS_MFA</code>: <code>SMS_MFA_CODE</code>, <code>USERNAME</code>, <code>SECRET_HASH</code> (if app client is configured with client secret).</p> </li> <li> <p> <code>PASSWORD_VERIFIER</code>: <code>PASSWORD_CLAIM_SIGNATURE</code>, <code>PASSWORD_CLAIM_SECRET_BLOCK</code>, <code>TIMESTAMP</code>, <code>USERNAME</code>, <code>SECRET_HASH</code> (if app client is configured with client secret).</p> </li> <li> <p> <code>ADMIN_NO_SRP_AUTH</code>: <code>PASSWORD</code>, <code>USERNAME</code>, <code>SECRET_HASH</code> (if app client is configured with client secret). </p> </li> <li> <p> <code>NEW_PASSWORD_REQUIRED</code>: <code>NEW_PASSWORD</code>, any other required attributes, <code>USERNAME</code>, <code>SECRET_HASH</code> (if app client is configured with client secret). </p> </li> </ul> <p>The value of the <code>USERNAME</code> attribute must be the user's actual username, not an alias (such as email address or phone number). To make this easier, the <code>AdminInitiateAuth</code> response includes the actual username value in the <code>USERNAMEUSER_ID_FOR_SRP</code> attribute, even if you specified an alias in your call to <code>AdminInitiateAuth</code>.</p>"]
    #[serde(rename="ChallengeResponses")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub challenge_responses: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>The app client ID.</p>"]
    #[serde(rename="ClientId")]
    pub client_id: String,
    #[doc="<p>The session which should be passed both ways in challenge-response calls to the service. If <code>InitiateAuth</code> or <code>RespondToAuthChallenge</code> API call determines that the caller needs to go through another challenge, they return a session with other challenge parameters. This session should be passed as it is to the next <code>RespondToAuthChallenge</code> API call.</p>"]
    #[serde(rename="Session")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub session: Option<String>,
    #[doc="<p>The ID of the Amazon Cognito user pool.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
}

#[doc="<p>Responds to the authentication challenge, as an administrator.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct AdminRespondToAuthChallengeResponse {
    #[doc="<p>The result returned by the server in response to the authentication request.</p>"]
    #[serde(rename="AuthenticationResult")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub authentication_result: Option<AuthenticationResultType>,
    #[doc="<p>The name of the challenge. For more information, see <a href=\"API_AdminInitiateAuth.html\">AdminInitiateAuth</a>.</p>"]
    #[serde(rename="ChallengeName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub challenge_name: Option<String>,
    #[doc="<p>The challenge parameters. For more information, see <a href=\"API_AdminInitiateAuth.html\">AdminInitiateAuth</a>.</p>"]
    #[serde(rename="ChallengeParameters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub challenge_parameters: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>The session which should be passed both ways in challenge-response calls to the service. If the <a href=\"API_InitiateAuth.html\">InitiateAuth</a> or <a href=\"API_RespondToAuthChallenge.html\">RespondToAuthChallenge</a> API call determines that the caller needs to go through another challenge, they return a session with other challenge parameters. This session should be passed as it is to the next <code>RespondToAuthChallenge</code> API call.</p>"]
    #[serde(rename="Session")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub session: Option<String>,
}

#[doc="<p>Represents the request to set user settings as an administrator.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct AdminSetUserSettingsRequest {
    #[doc="<p>Specifies the options for MFA (e.g., email or phone number).</p>"]
    #[serde(rename="MFAOptions")]
    pub mfa_options: Vec<MFAOptionType>,
    #[doc="<p>The user pool ID for the user pool where you want to set the user's settings, such as MFA options.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
    #[doc="<p>The user name of the user for whom you wish to set user settings.</p>"]
    #[serde(rename="Username")]
    pub username: String,
}

#[doc="<p>Represents the response from the server to set user settings as an administrator.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct AdminSetUserSettingsResponse;

#[doc="<p>The request to update the device status, as an administrator.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct AdminUpdateDeviceStatusRequest {
    #[doc="<p>The device key.</p>"]
    #[serde(rename="DeviceKey")]
    pub device_key: String,
    #[doc="<p>The status indicating whether a device has been remembered or not.</p>"]
    #[serde(rename="DeviceRememberedStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub device_remembered_status: Option<String>,
    #[doc="<p>The user pool ID.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
    #[doc="<p>The user name.</p>"]
    #[serde(rename="Username")]
    pub username: String,
}

#[doc="<p>The status response from the request to update the device, as an administrator.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct AdminUpdateDeviceStatusResponse;

#[doc="<p>Represents the request to update the user's attributes as an administrator.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct AdminUpdateUserAttributesRequest {
    #[doc="<p>An array of name-value pairs representing user attributes.</p> <p>For custom attributes, you must prepend the <code>custom:</code> prefix to the attribute name.</p>"]
    #[serde(rename="UserAttributes")]
    pub user_attributes: Vec<AttributeType>,
    #[doc="<p>The user pool ID for the user pool where you want to update user attributes.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
    #[doc="<p>The user name of the user for whom you want to update user attributes.</p>"]
    #[serde(rename="Username")]
    pub username: String,
}

#[doc="<p>Represents the response from the server for the request to update user attributes as an administrator.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct AdminUpdateUserAttributesResponse;

#[doc="<p>The request to sign out of all devices, as an administrator.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct AdminUserGlobalSignOutRequest {
    #[doc="<p>The user pool ID.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
    #[doc="<p>The user name.</p>"]
    #[serde(rename="Username")]
    pub username: String,
}

#[doc="<p>The global sign-out response, as an administrator.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct AdminUserGlobalSignOutResponse;


#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum AliasAttributeType {
    Email,
    PhoneNumber,
    PreferredUsername,
}

impl Into<String> for AliasAttributeType {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for AliasAttributeType {
    fn into(self) -> &'static str {
        match self {
            AliasAttributeType::Email => "email",
            AliasAttributeType::PhoneNumber => "phone_number",
            AliasAttributeType::PreferredUsername => "preferred_username",
        }
    }
}

impl ::std::str::FromStr for AliasAttributeType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "email" => Ok(AliasAttributeType::Email),
            "phone_number" => Ok(AliasAttributeType::PhoneNumber),
            "preferred_username" => Ok(AliasAttributeType::PreferredUsername),
            _ => Err(()),
        }
    }
}


#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum AttributeDataType {
    Boolean,
    DateTime,
    Number,
    String,
}

impl Into<String> for AttributeDataType {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for AttributeDataType {
    fn into(self) -> &'static str {
        match self {
            AttributeDataType::Boolean => "Boolean",
            AttributeDataType::DateTime => "DateTime",
            AttributeDataType::Number => "Number",
            AttributeDataType::String => "String",
        }
    }
}

impl ::std::str::FromStr for AttributeDataType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Boolean" => Ok(AttributeDataType::Boolean),
            "DateTime" => Ok(AttributeDataType::DateTime),
            "Number" => Ok(AttributeDataType::Number),
            "String" => Ok(AttributeDataType::String),
            _ => Err(()),
        }
    }
}

#[doc="<p>Specifies whether the attribute is standard or custom.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct AttributeType {
    #[doc="<p>The name of the attribute.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>The value of the attribute.</p>"]
    #[serde(rename="Value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<String>,
}


#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum AuthFlowType {
    AdminNoSrpAuth,
    CustomAuth,
    RefreshToken,
    RefreshTokenAuth,
    UserSrpAuth,
}

impl Into<String> for AuthFlowType {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for AuthFlowType {
    fn into(self) -> &'static str {
        match self {
            AuthFlowType::AdminNoSrpAuth => "ADMIN_NO_SRP_AUTH",
            AuthFlowType::CustomAuth => "CUSTOM_AUTH",
            AuthFlowType::RefreshToken => "REFRESH_TOKEN",
            AuthFlowType::RefreshTokenAuth => "REFRESH_TOKEN_AUTH",
            AuthFlowType::UserSrpAuth => "USER_SRP_AUTH",
        }
    }
}

impl ::std::str::FromStr for AuthFlowType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ADMIN_NO_SRP_AUTH" => Ok(AuthFlowType::AdminNoSrpAuth),
            "CUSTOM_AUTH" => Ok(AuthFlowType::CustomAuth),
            "REFRESH_TOKEN" => Ok(AuthFlowType::RefreshToken),
            "REFRESH_TOKEN_AUTH" => Ok(AuthFlowType::RefreshTokenAuth),
            "USER_SRP_AUTH" => Ok(AuthFlowType::UserSrpAuth),
            _ => Err(()),
        }
    }
}

#[doc="<p>The result type of the authentication result.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct AuthenticationResultType {
    #[doc="<p>The access token of the authentication result.</p>"]
    #[serde(rename="AccessToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub access_token: Option<String>,
    #[doc="<p>The expiration period of the authentication result.</p>"]
    #[serde(rename="ExpiresIn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub expires_in: Option<i64>,
    #[doc="<p>The ID token of the authentication result.</p>"]
    #[serde(rename="IdToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id_token: Option<String>,
    #[doc="<p>The new device metadata from an authentication result.</p>"]
    #[serde(rename="NewDeviceMetadata")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub new_device_metadata: Option<NewDeviceMetadataType>,
    #[doc="<p>The refresh token of the authentication result.</p>"]
    #[serde(rename="RefreshToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub refresh_token: Option<String>,
    #[doc="<p>The token type of the authentication result.</p>"]
    #[serde(rename="TokenType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub token_type: Option<String>,
}


#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum ChallengeNameType {
    AdminNoSrpAuth,
    CustomChallenge,
    DevicePasswordVerifier,
    DeviceSrpAuth,
    NewPasswordRequired,
    PasswordVerifier,
    SmsMfa,
}

impl Into<String> for ChallengeNameType {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for ChallengeNameType {
    fn into(self) -> &'static str {
        match self {
            ChallengeNameType::AdminNoSrpAuth => "ADMIN_NO_SRP_AUTH",
            ChallengeNameType::CustomChallenge => "CUSTOM_CHALLENGE",
            ChallengeNameType::DevicePasswordVerifier => "DEVICE_PASSWORD_VERIFIER",
            ChallengeNameType::DeviceSrpAuth => "DEVICE_SRP_AUTH",
            ChallengeNameType::NewPasswordRequired => "NEW_PASSWORD_REQUIRED",
            ChallengeNameType::PasswordVerifier => "PASSWORD_VERIFIER",
            ChallengeNameType::SmsMfa => "SMS_MFA",
        }
    }
}

impl ::std::str::FromStr for ChallengeNameType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ADMIN_NO_SRP_AUTH" => Ok(ChallengeNameType::AdminNoSrpAuth),
            "CUSTOM_CHALLENGE" => Ok(ChallengeNameType::CustomChallenge),
            "DEVICE_PASSWORD_VERIFIER" => Ok(ChallengeNameType::DevicePasswordVerifier),
            "DEVICE_SRP_AUTH" => Ok(ChallengeNameType::DeviceSrpAuth),
            "NEW_PASSWORD_REQUIRED" => Ok(ChallengeNameType::NewPasswordRequired),
            "PASSWORD_VERIFIER" => Ok(ChallengeNameType::PasswordVerifier),
            "SMS_MFA" => Ok(ChallengeNameType::SmsMfa),
            _ => Err(()),
        }
    }
}

#[doc="<p>Represents the request to change a user password.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ChangePasswordRequest {
    #[doc="<p>The access token in the change password request.</p>"]
    #[serde(rename="AccessToken")]
    pub access_token: String,
    #[doc="<p>The old password in the change password request.</p>"]
    #[serde(rename="PreviousPassword")]
    pub previous_password: String,
    #[doc="<p>The new password in the change password request.</p>"]
    #[serde(rename="ProposedPassword")]
    pub proposed_password: String,
}

#[doc="<p>The response from the server to the change password request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ChangePasswordResponse;

#[doc="<p>The type of code delivery details being returned from the server.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CodeDeliveryDetailsType {
    #[doc="<p>The name of the attribute in the code delivery details type.</p>"]
    #[serde(rename="AttributeName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attribute_name: Option<String>,
    #[doc="<p>The delivery medium (email message or phone number).</p>"]
    #[serde(rename="DeliveryMedium")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub delivery_medium: Option<String>,
    #[doc="<p>The destination for the code delivery details.</p>"]
    #[serde(rename="Destination")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub destination: Option<String>,
}

#[doc="<p>Confirms the device request.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ConfirmDeviceRequest {
    #[doc="<p>The access token.</p>"]
    #[serde(rename="AccessToken")]
    pub access_token: String,
    #[doc="<p>The device key.</p>"]
    #[serde(rename="DeviceKey")]
    pub device_key: String,
    #[doc="<p>The device name.</p>"]
    #[serde(rename="DeviceName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub device_name: Option<String>,
    #[doc="<p>The configuration of the device secret verifier.</p>"]
    #[serde(rename="DeviceSecretVerifierConfig")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub device_secret_verifier_config: Option<DeviceSecretVerifierConfigType>,
}

#[doc="<p>Confirms the device response.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ConfirmDeviceResponse {
    #[doc="<p>Indicates whether the user confirmation is necessary to confirm the device response.</p>"]
    #[serde(rename="UserConfirmationNecessary")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_confirmation_necessary: Option<bool>,
}

#[doc="<p>The request representing the confirmation for a password reset.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ConfirmForgotPasswordRequest {
    #[doc="<p>The ID of the client associated with the user pool.</p>"]
    #[serde(rename="ClientId")]
    pub client_id: String,
    #[doc="<p>The confirmation code sent by a user's request to retrieve a forgotten password. For more information, see <a href=\"API_ForgotPassword.html\">ForgotPassword</a> </p>"]
    #[serde(rename="ConfirmationCode")]
    pub confirmation_code: String,
    #[doc="<p>The password sent by a user's request to retrieve a forgotten password.</p>"]
    #[serde(rename="Password")]
    pub password: String,
    #[doc="<p>A keyed-hash message authentication code (HMAC) calculated using the secret key of a user pool client and username plus the client ID in the message.</p>"]
    #[serde(rename="SecretHash")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub secret_hash: Option<String>,
    #[doc="<p>The user name of the user for whom you want to enter a code to retrieve a forgotten password.</p>"]
    #[serde(rename="Username")]
    pub username: String,
}

#[doc="<p>The response from the server that results from a user's request to retrieve a forgotten password.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ConfirmForgotPasswordResponse;

#[doc="<p>Represents the request to confirm registration of a user.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ConfirmSignUpRequest {
    #[doc="<p>The ID of the client associated with the user pool.</p>"]
    #[serde(rename="ClientId")]
    pub client_id: String,
    #[doc="<p>The confirmation code sent by a user's request to confirm registration.</p>"]
    #[serde(rename="ConfirmationCode")]
    pub confirmation_code: String,
    #[doc="<p>Boolean to be specified to force user confirmation irrespective of existing alias. By default set to <code>False</code>. If this parameter is set to <code>True</code> and the phone number/email used for sign up confirmation already exists as an alias with a different user, the API call will migrate the alias from the previous user to the newly created user being confirmed. If set to <code>False</code>, the API will throw an <b>AliasExistsException</b> error.</p>"]
    #[serde(rename="ForceAliasCreation")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub force_alias_creation: Option<bool>,
    #[doc="<p>A keyed-hash message authentication code (HMAC) calculated using the secret key of a user pool client and username plus the client ID in the message.</p>"]
    #[serde(rename="SecretHash")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub secret_hash: Option<String>,
    #[doc="<p>The user name of the user whose registration you wish to confirm.</p>"]
    #[serde(rename="Username")]
    pub username: String,
}

#[doc="<p>Represents the response from the server for the registration confirmation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ConfirmSignUpResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateGroupRequest {
    #[doc="<p>A string containing the description of the group.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[doc="<p>The name of the group. Must be unique.</p>"]
    #[serde(rename="GroupName")]
    pub group_name: String,
    #[doc="<p>A nonnegative integer value that specifies the precedence of this group relative to the other groups that a user can belong to in the user pool. Zero is the highest precedence value. Groups with lower <code>Precedence</code> values take precedence over groups with higher or null <code>Precedence</code> values. If a user belongs to two or more groups, it is the group with the lowest precedence value whose role ARN will be used in the <code>cognito:roles</code> and <code>cognito:preferred_role</code> claims in the user's tokens.</p> <p>Two groups can have the same <code>Precedence</code> value. If this happens, neither group takes precedence over the other. If two groups with the same <code>Precedence</code> have the same role ARN, that role is used in the <code>cognito:preferred_role</code> claim in tokens for users in each group. If the two groups have different role ARNs, the <code>cognito:preferred_role</code> claim is not set in users' tokens.</p> <p>The default <code>Precedence</code> value is null.</p>"]
    #[serde(rename="Precedence")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub precedence: Option<i64>,
    #[doc="<p>The role ARN for the group.</p>"]
    #[serde(rename="RoleArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role_arn: Option<String>,
    #[doc="<p>The user pool ID for the user pool.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateGroupResponse {
    #[doc="<p>The group object for the group.</p>"]
    #[serde(rename="Group")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub group: Option<GroupType>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateIdentityProviderRequest {
    #[doc="<p>A mapping of identity provider attributes to standard and custom user pool attributes.</p>"]
    #[serde(rename="AttributeMapping")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attribute_mapping: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>A list of identity provider identifiers.</p>"]
    #[serde(rename="IdpIdentifiers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub idp_identifiers: Option<Vec<String>>,
    #[doc="<p>The identity provider details, such as <code>MetadataURL</code> and <code>MetadataFile</code>.</p>"]
    #[serde(rename="ProviderDetails")]
    pub provider_details: ::std::collections::HashMap<String, String>,
    #[doc="<p>The identity provider name.</p>"]
    #[serde(rename="ProviderName")]
    pub provider_name: String,
    #[doc="<p>The identity provider type.</p>"]
    #[serde(rename="ProviderType")]
    pub provider_type: String,
    #[doc="<p>The user pool ID.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateIdentityProviderResponse {
    #[doc="<p>The newly created identity provider object.</p>"]
    #[serde(rename="IdentityProvider")]
    pub identity_provider: IdentityProviderType,
}

#[doc="<p>Represents the request to create the user import job.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateUserImportJobRequest {
    #[doc="<p>The role ARN for the Amazon CloudWatch Logging role for the user import job.</p>"]
    #[serde(rename="CloudWatchLogsRoleArn")]
    pub cloud_watch_logs_role_arn: String,
    #[doc="<p>The job name for the user import job.</p>"]
    #[serde(rename="JobName")]
    pub job_name: String,
    #[doc="<p>The user pool ID for the user pool that the users are being imported into.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
}

#[doc="<p>Represents the response from the server to the request to create the user import job.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateUserImportJobResponse {
    #[doc="<p>The job object that represents the user import job.</p>"]
    #[serde(rename="UserImportJob")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_import_job: Option<UserImportJobType>,
}

#[doc="<p>Represents the request to create a user pool client.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateUserPoolClientRequest {
    #[doc="<p>Set to <code>code</code> to initiate a code grant flow, which provides an authorization code as the response. This code can be exchanged for access tokens with the token endpoint.</p> <p>Set to <code>token</code> to specify that the client should get the access token (and, optionally, ID token, based on scopes) directly.</p>"]
    #[serde(rename="AllowedOAuthFlows")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allowed_o_auth_flows: Option<Vec<String>>,
    #[doc="<p>Set to <code>True</code> if the client is allowed to follow the OAuth protocol when interacting with Cognito user pools.</p>"]
    #[serde(rename="AllowedOAuthFlowsUserPoolClient")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allowed_o_auth_flows_user_pool_client: Option<bool>,
    #[doc="<p>A list of allowed <code>OAuth</code> scopes. Currently supported values are <code>\"phone\"</code>, <code>\"email\"</code>, <code>\"openid\"</code>, and <code>\"Cognito\"</code>.</p>"]
    #[serde(rename="AllowedOAuthScopes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allowed_o_auth_scopes: Option<Vec<String>>,
    #[doc="<p>A list of allowed callback URLs for the identity providers.</p>"]
    #[serde(rename="CallbackURLs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub callback_ur_ls: Option<Vec<String>>,
    #[doc="<p>The client name for the user pool client you would like to create.</p>"]
    #[serde(rename="ClientName")]
    pub client_name: String,
    #[doc="<p>The default redirect URI. Must be in the <code>CallbackURLs</code> list.</p>"]
    #[serde(rename="DefaultRedirectURI")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_redirect_uri: Option<String>,
    #[doc="<p>The explicit authentication flows.</p>"]
    #[serde(rename="ExplicitAuthFlows")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub explicit_auth_flows: Option<Vec<String>>,
    #[doc="<p>Boolean to specify whether you want to generate a secret for the user pool client being created.</p>"]
    #[serde(rename="GenerateSecret")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub generate_secret: Option<bool>,
    #[doc="<p>A list of allowed logout URLs for the identity providers.</p>"]
    #[serde(rename="LogoutURLs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub logout_ur_ls: Option<Vec<String>>,
    #[doc="<p>The read attributes.</p>"]
    #[serde(rename="ReadAttributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub read_attributes: Option<Vec<String>>,
    #[doc="<p>The time limit, in days, after which the refresh token is no longer valid and cannot be used.</p>"]
    #[serde(rename="RefreshTokenValidity")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub refresh_token_validity: Option<i64>,
    #[doc="<p>A list of provider names for the identity providers that are supported on this client.</p>"]
    #[serde(rename="SupportedIdentityProviders")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub supported_identity_providers: Option<Vec<String>>,
    #[doc="<p>The user pool ID for the user pool where you want to create a user pool client.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
    #[doc="<p>The write attributes.</p>"]
    #[serde(rename="WriteAttributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub write_attributes: Option<Vec<String>>,
}

#[doc="<p>Represents the response from the server to create a user pool client.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateUserPoolClientResponse {
    #[doc="<p>The user pool client that was just created.</p>"]
    #[serde(rename="UserPoolClient")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_pool_client: Option<UserPoolClientType>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateUserPoolDomainRequest {
    #[doc="<p>The domain string.</p>"]
    #[serde(rename="Domain")]
    pub domain: String,
    #[doc="<p>The user pool ID.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateUserPoolDomainResponse;

#[doc="<p>Represents the request to create a user pool.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateUserPoolRequest {
    #[doc="<p>The configuration for <code>AdminCreateUser</code> requests.</p>"]
    #[serde(rename="AdminCreateUserConfig")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub admin_create_user_config: Option<AdminCreateUserConfigType>,
    #[doc="<p>Attributes supported as an alias for this user pool. Possible values: <b>phone_number</b>, <b>email</b>, or <b>preferred_username</b>.</p>"]
    #[serde(rename="AliasAttributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub alias_attributes: Option<Vec<String>>,
    #[doc="<p>The attributes to be auto-verified. Possible values: <b>email</b>, <b>phone_number</b>.</p>"]
    #[serde(rename="AutoVerifiedAttributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auto_verified_attributes: Option<Vec<String>>,
    #[doc="<p>The device configuration.</p>"]
    #[serde(rename="DeviceConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub device_configuration: Option<DeviceConfigurationType>,
    #[doc="<p>The email configuration.</p>"]
    #[serde(rename="EmailConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email_configuration: Option<EmailConfigurationType>,
    #[doc="<p>A string representing the email verification message.</p>"]
    #[serde(rename="EmailVerificationMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email_verification_message: Option<String>,
    #[doc="<p>A string representing the email verification subject.</p>"]
    #[serde(rename="EmailVerificationSubject")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email_verification_subject: Option<String>,
    #[doc="<p>The Lambda trigger configuration information for the new user pool.</p>"]
    #[serde(rename="LambdaConfig")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub lambda_config: Option<LambdaConfigType>,
    #[doc="<p>Specifies MFA configuration details.</p>"]
    #[serde(rename="MfaConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mfa_configuration: Option<String>,
    #[doc="<p>The policies associated with the new user pool.</p>"]
    #[serde(rename="Policies")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub policies: Option<UserPoolPolicyType>,
    #[doc="<p>A string used to name the user pool.</p>"]
    #[serde(rename="PoolName")]
    pub pool_name: String,
    #[doc="<p>An array of schema attributes for the new user pool. These attributes can be standard or custom attributes.</p>"]
    #[serde(rename="Schema")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schema: Option<Vec<SchemaAttributeType>>,
    #[doc="<p>A string representing the SMS authentication message.</p>"]
    #[serde(rename="SmsAuthenticationMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sms_authentication_message: Option<String>,
    #[doc="<p>The SMS configuration.</p>"]
    #[serde(rename="SmsConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sms_configuration: Option<SmsConfigurationType>,
    #[doc="<p>A string representing the SMS verification message.</p>"]
    #[serde(rename="SmsVerificationMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sms_verification_message: Option<String>,
    #[doc="<p>The cost allocation tags for the user pool. For more information, see <a href=\"http://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-cost-allocation-tagging.html\">Adding Cost Allocation Tags to Your User Pool</a> </p>"]
    #[serde(rename="UserPoolTags")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_pool_tags: Option<::std::collections::HashMap<String, String>>,
}

#[doc="<p>Represents the response from the server for the request to create a user pool.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateUserPoolResponse {
    #[doc="<p>A container for the user pool details.</p>"]
    #[serde(rename="UserPool")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_pool: Option<UserPoolType>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteGroupRequest {
    #[doc="<p>The name of the group.</p>"]
    #[serde(rename="GroupName")]
    pub group_name: String,
    #[doc="<p>The user pool ID for the user pool.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteIdentityProviderRequest {
    #[doc="<p>The identity provider name.</p>"]
    #[serde(rename="ProviderName")]
    pub provider_name: String,
    #[doc="<p>The user pool ID.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
}

#[doc="<p>Represents the request to delete user attributes.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteUserAttributesRequest {
    #[doc="<p>The access token used in the request to delete user attributes.</p>"]
    #[serde(rename="AccessToken")]
    pub access_token: String,
    #[doc="<p>An array of strings representing the user attribute names you wish to delete.</p> <p>For custom attributes, you must prepend the <code>custom:</code> prefix to the attribute name.</p>"]
    #[serde(rename="UserAttributeNames")]
    pub user_attribute_names: Vec<String>,
}

#[doc="<p>Represents the response from the server to delete user attributes.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteUserAttributesResponse;

#[doc="<p>Represents the request to delete a user pool client.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteUserPoolClientRequest {
    #[doc="<p>The ID of the client associated with the user pool.</p>"]
    #[serde(rename="ClientId")]
    pub client_id: String,
    #[doc="<p>The user pool ID for the user pool where you want to delete the client.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteUserPoolDomainRequest {
    #[doc="<p>The domain string.</p>"]
    #[serde(rename="Domain")]
    pub domain: String,
    #[doc="<p>The user pool ID.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteUserPoolDomainResponse;

#[doc="<p>Represents the request to delete a user pool.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteUserPoolRequest {
    #[doc="<p>The user pool ID for the user pool you want to delete.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
}

#[doc="<p>Represents the request to delete a user.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteUserRequest {
    #[doc="<p>The access token from a request to delete a user.</p>"]
    #[serde(rename="AccessToken")]
    pub access_token: String,
}


#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum DeliveryMediumType {
    Email,
    Sms,
}

impl Into<String> for DeliveryMediumType {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for DeliveryMediumType {
    fn into(self) -> &'static str {
        match self {
            DeliveryMediumType::Email => "EMAIL",
            DeliveryMediumType::Sms => "SMS",
        }
    }
}

impl ::std::str::FromStr for DeliveryMediumType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "EMAIL" => Ok(DeliveryMediumType::Email),
            "SMS" => Ok(DeliveryMediumType::Sms),
            _ => Err(()),
        }
    }
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeIdentityProviderRequest {
    #[doc="<p>The identity provider name.</p>"]
    #[serde(rename="ProviderName")]
    pub provider_name: String,
    #[doc="<p>The user pool ID.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeIdentityProviderResponse {
    #[doc="<p>The identity provider that was deleted.</p>"]
    #[serde(rename="IdentityProvider")]
    pub identity_provider: IdentityProviderType,
}

#[doc="<p>Represents the request to describe the user import job.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeUserImportJobRequest {
    #[doc="<p>The job ID for the user import job.</p>"]
    #[serde(rename="JobId")]
    pub job_id: String,
    #[doc="<p>The user pool ID for the user pool that the users are being imported into.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
}

#[doc="<p>Represents the response from the server to the request to describe the user import job.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeUserImportJobResponse {
    #[doc="<p>The job object that represents the user import job.</p>"]
    #[serde(rename="UserImportJob")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_import_job: Option<UserImportJobType>,
}

#[doc="<p>Represents the request to describe a user pool client.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeUserPoolClientRequest {
    #[doc="<p>The ID of the client associated with the user pool.</p>"]
    #[serde(rename="ClientId")]
    pub client_id: String,
    #[doc="<p>The user pool ID for the user pool you want to describe.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
}

#[doc="<p>Represents the response from the server from a request to describe the user pool client.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeUserPoolClientResponse {
    #[doc="<p>The user pool client from a server response to describe the user pool client.</p>"]
    #[serde(rename="UserPoolClient")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_pool_client: Option<UserPoolClientType>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeUserPoolDomainRequest {
    #[doc="<p>The domain string.</p>"]
    #[serde(rename="Domain")]
    pub domain: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeUserPoolDomainResponse {
    #[doc="<p>A domain description object containing information about the domain.</p>"]
    #[serde(rename="DomainDescription")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub domain_description: Option<DomainDescriptionType>,
}

#[doc="<p>Represents the request to describe the user pool.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeUserPoolRequest {
    #[doc="<p>The user pool ID for the user pool you want to describe.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
}

#[doc="<p>Represents the response to describe the user pool.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeUserPoolResponse {
    #[doc="<p>The container of metadata returned by the server to describe the pool.</p>"]
    #[serde(rename="UserPool")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_pool: Option<UserPoolType>,
}

#[doc="<p>The type of configuration for the user pool's device tracking.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct DeviceConfigurationType {
    #[doc="<p>Indicates whether a challenge is required on a new device. Only applicable to a new device.</p>"]
    #[serde(rename="ChallengeRequiredOnNewDevice")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub challenge_required_on_new_device: Option<bool>,
    #[doc="<p>If true, a device is only remembered on user prompt.</p>"]
    #[serde(rename="DeviceOnlyRememberedOnUserPrompt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub device_only_remembered_on_user_prompt: Option<bool>,
}


#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum DeviceRememberedStatusType {
    NotRemembered,
    Remembered,
}

impl Into<String> for DeviceRememberedStatusType {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for DeviceRememberedStatusType {
    fn into(self) -> &'static str {
        match self {
            DeviceRememberedStatusType::NotRemembered => "not_remembered",
            DeviceRememberedStatusType::Remembered => "remembered",
        }
    }
}

impl ::std::str::FromStr for DeviceRememberedStatusType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "not_remembered" => Ok(DeviceRememberedStatusType::NotRemembered),
            "remembered" => Ok(DeviceRememberedStatusType::Remembered),
            _ => Err(()),
        }
    }
}

#[doc="<p>The device verifier against which it will be authenticated.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeviceSecretVerifierConfigType {
    #[doc="<p>The password verifier.</p>"]
    #[serde(rename="PasswordVerifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub password_verifier: Option<String>,
    #[doc="<p>The salt.</p>"]
    #[serde(rename="Salt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub salt: Option<String>,
}

#[doc="<p>The device type.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeviceType {
    #[doc="<p>The device attributes.</p>"]
    #[serde(rename="DeviceAttributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub device_attributes: Option<Vec<AttributeType>>,
    #[doc="<p>The creation date of the device.</p>"]
    #[serde(rename="DeviceCreateDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub device_create_date: Option<f64>,
    #[doc="<p>The device key.</p>"]
    #[serde(rename="DeviceKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub device_key: Option<String>,
    #[doc="<p>The date in which the device was last authenticated.</p>"]
    #[serde(rename="DeviceLastAuthenticatedDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub device_last_authenticated_date: Option<f64>,
    #[doc="<p>The last modified date of the device.</p>"]
    #[serde(rename="DeviceLastModifiedDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub device_last_modified_date: Option<f64>,
}

#[doc="<p>A container for information about a domain.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DomainDescriptionType {
    #[doc="<p>The AWS account ID for the user pool owner.</p>"]
    #[serde(rename="AWSAccountId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub aws_account_id: Option<String>,
    #[doc="<p>The ARN of the CloudFront distribution.</p>"]
    #[serde(rename="CloudFrontDistribution")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cloud_front_distribution: Option<String>,
    #[doc="<p>The domain string.</p>"]
    #[serde(rename="Domain")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub domain: Option<String>,
    #[doc="<p>The S3 bucket where the static files for this domain are stored.</p>"]
    #[serde(rename="S3Bucket")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub s3_bucket: Option<String>,
    #[doc="<p>The domain status.</p>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
    #[doc="<p>The user pool ID.</p>"]
    #[serde(rename="UserPoolId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_pool_id: Option<String>,
    #[doc="<p>The app version.</p>"]
    #[serde(rename="Version")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub version: Option<String>,
}


#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum DomainStatusType {
    Active,
    Creating,
    Deleting,
    Updating,
}

impl Into<String> for DomainStatusType {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for DomainStatusType {
    fn into(self) -> &'static str {
        match self {
            DomainStatusType::Active => "ACTIVE",
            DomainStatusType::Creating => "CREATING",
            DomainStatusType::Deleting => "DELETING",
            DomainStatusType::Updating => "UPDATING",
        }
    }
}

impl ::std::str::FromStr for DomainStatusType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ACTIVE" => Ok(DomainStatusType::Active),
            "CREATING" => Ok(DomainStatusType::Creating),
            "DELETING" => Ok(DomainStatusType::Deleting),
            "UPDATING" => Ok(DomainStatusType::Updating),
            _ => Err(()),
        }
    }
}

#[doc="<p>The email configuration type.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct EmailConfigurationType {
    #[doc="<p>The REPLY-TO email address.</p>"]
    #[serde(rename="ReplyToEmailAddress")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reply_to_email_address: Option<String>,
    #[doc="<p>The Amazon Resource Name (ARN) of the email source.</p>"]
    #[serde(rename="SourceArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub source_arn: Option<String>,
}


#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum ExplicitAuthFlowsType {
    AdminNoSrpAuth,
    CustomAuthFlowOnly,
}

impl Into<String> for ExplicitAuthFlowsType {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for ExplicitAuthFlowsType {
    fn into(self) -> &'static str {
        match self {
            ExplicitAuthFlowsType::AdminNoSrpAuth => "ADMIN_NO_SRP_AUTH",
            ExplicitAuthFlowsType::CustomAuthFlowOnly => "CUSTOM_AUTH_FLOW_ONLY",
        }
    }
}

impl ::std::str::FromStr for ExplicitAuthFlowsType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ADMIN_NO_SRP_AUTH" => Ok(ExplicitAuthFlowsType::AdminNoSrpAuth),
            "CUSTOM_AUTH_FLOW_ONLY" => Ok(ExplicitAuthFlowsType::CustomAuthFlowOnly),
            _ => Err(()),
        }
    }
}

#[doc="<p>Represents the request to forget the device.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ForgetDeviceRequest {
    #[doc="<p>The access token for the forgotten device request.</p>"]
    #[serde(rename="AccessToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub access_token: Option<String>,
    #[doc="<p>The device key.</p>"]
    #[serde(rename="DeviceKey")]
    pub device_key: String,
}

#[doc="<p>Represents the request to reset a user's password.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ForgotPasswordRequest {
    #[doc="<p>The ID of the client associated with the user pool.</p>"]
    #[serde(rename="ClientId")]
    pub client_id: String,
    #[doc="<p>A keyed-hash message authentication code (HMAC) calculated using the secret key of a user pool client and username plus the client ID in the message.</p>"]
    #[serde(rename="SecretHash")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub secret_hash: Option<String>,
    #[doc="<p>The user name of the user for whom you want to enter a code to reset a forgotten password.</p>"]
    #[serde(rename="Username")]
    pub username: String,
}

#[doc="<p>Respresents the response from the server regarding the request to reset a password.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ForgotPasswordResponse {
    #[doc="<p>The code delivery details returned by the server in response to the request to reset a password.</p>"]
    #[serde(rename="CodeDeliveryDetails")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub code_delivery_details: Option<CodeDeliveryDetailsType>,
}

#[doc="<p>Represents the request to get the header information for the .csv file for the user import job.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct GetCSVHeaderRequest {
    #[doc="<p>The user pool ID for the user pool that the users are to be imported into.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
}

#[doc="<p>Represents the response from the server to the request to get the header information for the .csv file for the user import job.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetCSVHeaderResponse {
    #[doc="<p>The header information for the .csv file for the user import job.</p>"]
    #[serde(rename="CSVHeader")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub csv_header: Option<Vec<String>>,
    #[doc="<p>The user pool ID for the user pool that the users are to be imported into.</p>"]
    #[serde(rename="UserPoolId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_pool_id: Option<String>,
}

#[doc="<p>Represents the request to get the device.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct GetDeviceRequest {
    #[doc="<p>The access token.</p>"]
    #[serde(rename="AccessToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub access_token: Option<String>,
    #[doc="<p>The device key.</p>"]
    #[serde(rename="DeviceKey")]
    pub device_key: String,
}

#[doc="<p>Gets the device response.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetDeviceResponse {
    #[doc="<p>The device.</p>"]
    #[serde(rename="Device")]
    pub device: DeviceType,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetGroupRequest {
    #[doc="<p>The name of the group.</p>"]
    #[serde(rename="GroupName")]
    pub group_name: String,
    #[doc="<p>The user pool ID for the user pool.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetGroupResponse {
    #[doc="<p>The group object for the group.</p>"]
    #[serde(rename="Group")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub group: Option<GroupType>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetIdentityProviderByIdentifierRequest {
    #[doc="<p>The identity provider ID.</p>"]
    #[serde(rename="IdpIdentifier")]
    pub idp_identifier: String,
    #[doc="<p>The user pool ID.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetIdentityProviderByIdentifierResponse {
    #[doc="<p>The identity provider object.</p>"]
    #[serde(rename="IdentityProvider")]
    pub identity_provider: IdentityProviderType,
}

#[doc="<p>Represents the request to get user attribute verification.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct GetUserAttributeVerificationCodeRequest {
    #[doc="<p>The access token returned by the server response to get the user attribute verification code.</p>"]
    #[serde(rename="AccessToken")]
    pub access_token: String,
    #[doc="<p>The attribute name returned by the server response to get the user attribute verification code.</p>"]
    #[serde(rename="AttributeName")]
    pub attribute_name: String,
}

#[doc="<p>The verification code response returned by the server response to get the user attribute verification code.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetUserAttributeVerificationCodeResponse {
    #[doc="<p>The code delivery details returned by the server in response to the request to get the user attribute verification code.</p>"]
    #[serde(rename="CodeDeliveryDetails")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub code_delivery_details: Option<CodeDeliveryDetailsType>,
}

#[doc="<p>Represents the request to get information about the user.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct GetUserRequest {
    #[doc="<p>The access token returned by the server response to get information about the user.</p>"]
    #[serde(rename="AccessToken")]
    pub access_token: String,
}

#[doc="<p>Represents the response from the server from the request to get information about the user.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetUserResponse {
    #[doc="<p>Specifies the options for MFA (e.g., email or phone number).</p>"]
    #[serde(rename="MFAOptions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mfa_options: Option<Vec<MFAOptionType>>,
    #[doc="<p>An array of name-value pairs representing user attributes.</p> <p>For custom attributes, you must prepend the <code>custom:</code> prefix to the attribute name.</p>"]
    #[serde(rename="UserAttributes")]
    pub user_attributes: Vec<AttributeType>,
    #[doc="<p>The user name of the user you wish to retrieve from the get user request.</p>"]
    #[serde(rename="Username")]
    pub username: String,
}

#[doc="<p>Represents the request to sign out all devices.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct GlobalSignOutRequest {
    #[doc="<p>The access token.</p>"]
    #[serde(rename="AccessToken")]
    pub access_token: String,
}

#[doc="<p>The response to the request to sign out all devices.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GlobalSignOutResponse;

#[doc="<p>The group type.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GroupType {
    #[doc="<p>The date the group was created.</p>"]
    #[serde(rename="CreationDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creation_date: Option<f64>,
    #[doc="<p>A string containing the description of the group.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[doc="<p>The name of the group.</p>"]
    #[serde(rename="GroupName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub group_name: Option<String>,
    #[doc="<p>The date the group was last modified.</p>"]
    #[serde(rename="LastModifiedDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[doc="<p>A nonnegative integer value that specifies the precedence of this group relative to the other groups that a user can belong to in the user pool. If a user belongs to two or more groups, it is the group with the highest precedence whose role ARN will be used in the <code>cognito:roles</code> and <code>cognito:preferred_role</code> claims in the user's tokens. Groups with higher <code>Precedence</code> values take precedence over groups with lower <code>Precedence</code> values or with null <code>Precedence</code> values.</p> <p>Two groups can have the same <code>Precedence</code> value. If this happens, neither group takes precedence over the other. If two groups with the same <code>Precedence</code> have the same role ARN, that role is used in the <code>cognito:preferred_role</code> claim in tokens for users in each group. If the two groups have different role ARNs, the <code>cognito:preferred_role</code> claim is not set in users' tokens.</p> <p>The default <code>Precedence</code> value is null.</p>"]
    #[serde(rename="Precedence")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub precedence: Option<i64>,
    #[doc="<p>The role ARN for the group.</p>"]
    #[serde(rename="RoleArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role_arn: Option<String>,
    #[doc="<p>The user pool ID for the user pool.</p>"]
    #[serde(rename="UserPoolId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_pool_id: Option<String>,
}

#[doc="<p>A container for information about an identity provider.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct IdentityProviderType {
    #[doc="<p>A mapping of identity provider attributes to standard and custom user pool attributes.</p>"]
    #[serde(rename="AttributeMapping")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attribute_mapping: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>The date the identity provider was created.</p>"]
    #[serde(rename="CreationDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creation_date: Option<f64>,
    #[doc="<p>A list of identity provider identifiers.</p>"]
    #[serde(rename="IdpIdentifiers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub idp_identifiers: Option<Vec<String>>,
    #[doc="<p>The date the identity provider was last modified.</p>"]
    #[serde(rename="LastModifiedDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[doc="<p>The identity provider details, such as <code>MetadataURL</code> and <code>MetadataFile</code>.</p>"]
    #[serde(rename="ProviderDetails")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub provider_details: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>The identity provider name.</p>"]
    #[serde(rename="ProviderName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub provider_name: Option<String>,
    #[doc="<p>The identity provider type.</p>"]
    #[serde(rename="ProviderType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub provider_type: Option<String>,
    #[doc="<p>The user pool ID.</p>"]
    #[serde(rename="UserPoolId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_pool_id: Option<String>,
}


#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum IdentityProviderTypeType {
    Saml,
}

impl Into<String> for IdentityProviderTypeType {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for IdentityProviderTypeType {
    fn into(self) -> &'static str {
        match self {
            IdentityProviderTypeType::Saml => "SAML",
        }
    }
}

impl ::std::str::FromStr for IdentityProviderTypeType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SAML" => Ok(IdentityProviderTypeType::Saml),
            _ => Err(()),
        }
    }
}

#[doc="<p>Initiates the authentication request.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct InitiateAuthRequest {
    #[doc="<p>The authentication flow for this call to execute. The API action will depend on this value. For example: </p> <ul> <li> <p> <code>REFRESH_TOKEN_AUTH</code> will take in a valid refresh token and return new tokens.</p> </li> <li> <p> <code>USER_SRP_AUTH</code> will take in USERNAME and SRPA and return the SRP variables to be used for next challenge execution.</p> </li> </ul> <p>Valid values include:</p> <ul> <li> <p> <code>USER_SRP_AUTH</code>: Authentication flow for the Secure Remote Password (SRP) protocol.</p> </li> <li> <p> <code>REFRESH_TOKEN_AUTH</code>/<code>REFRESH_TOKEN</code>: Authentication flow for refreshing the access token and ID token by supplying a valid refresh token.</p> </li> <li> <p> <code>CUSTOM_AUTH</code>: Custom authentication flow.</p> </li> </ul> <p> <code>ADMIN_NO_SRP_AUTH</code> is not a valid value.</p>"]
    #[serde(rename="AuthFlow")]
    pub auth_flow: String,
    #[doc="<p>The authentication parameters. These are inputs corresponding to the <code>AuthFlow</code> that you are invoking. The required values depend on the value of <code>AuthFlow</code>:</p> <ul> <li> <p>For <code>USER_SRP_AUTH</code>: <code>USERNAME</code> (required), <code>SRPA</code> (required), <code>SECRET_HASH</code> (required if the app client is configured with a client secret), <code>DEVICE_KEY</code> </p> </li> <li> <p>For <code>REFRESH_TOKEN_AUTH/REFRESH_TOKEN</code>: <code>USERNAME</code> (required), <code>SECRET_HASH</code> (required if the app client is configured with a client secret), <code>REFRESH_TOKEN</code> (required), <code>DEVICE_KEY</code> </p> </li> <li> <p>For <code>CUSTOM_AUTH</code>: <code>USERNAME</code> (required), <code>SECRET_HASH</code> (if app client is configured with client secret), <code>DEVICE_KEY</code> </p> </li> </ul>"]
    #[serde(rename="AuthParameters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auth_parameters: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>The app client ID.</p>"]
    #[serde(rename="ClientId")]
    pub client_id: String,
    #[doc="<p>This is a random key-value pair map which can contain any key and will be passed to your PreAuthentication Lambda trigger as-is. It can be used to implement additional validations around authentication.</p>"]
    #[serde(rename="ClientMetadata")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub client_metadata: Option<::std::collections::HashMap<String, String>>,
}

#[doc="<p>Initiates the authentication response.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct InitiateAuthResponse {
    #[doc="<p>The result of the authentication response. This is only returned if the caller does not need to pass another challenge. If the caller does need to pass another challenge before it gets tokens, <code>ChallengeName</code>, <code>ChallengeParameters</code>, and <code>Session</code> are returned.</p>"]
    #[serde(rename="AuthenticationResult")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub authentication_result: Option<AuthenticationResultType>,
    #[doc="<p>The name of the challenge which you are responding to with this call. This is returned to you in the <code>AdminInitiateAuth</code> response if you need to pass another challenge.</p> <p>Valid values include the following. Note that all of these challenges require <code>USERNAME</code> and <code>SECRET_HASH</code> (if applicable) in the parameters.</p> <ul> <li> <p> <code>SMS_MFA</code>: Next challenge is to supply an <code>SMS_MFA_CODE</code>, delivered via SMS.</p> </li> <li> <p> <code>PASSWORD_VERIFIER</code>: Next challenge is to supply <code>PASSWORD_CLAIM_SIGNATURE</code>, <code>PASSWORD_CLAIM_SECRET_BLOCK</code>, and <code>TIMESTAMP</code> after the client-side SRP calculations.</p> </li> <li> <p> <code>CUSTOM_CHALLENGE</code>: This is returned if your custom authentication flow determines that the user should pass another challenge before tokens are issued.</p> </li> <li> <p> <code>DEVICE_SRP_AUTH</code>: If device tracking was enabled on your user pool and the previous challenges were passed, this challenge is returned so that Amazon Cognito can start tracking this device.</p> </li> <li> <p> <code>DEVICE_PASSWORD_VERIFIER</code>: Similar to <code>PASSWORD_VERIFIER</code>, but for devices only.</p> </li> <li> <p> <code>NEW_PASSWORD_REQUIRED</code>: For users which are required to change their passwords after successful first login. This challenge should be passed with <code>NEW_PASSWORD</code> and any other required attributes.</p> </li> </ul>"]
    #[serde(rename="ChallengeName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub challenge_name: Option<String>,
    #[doc="<p>The challenge parameters. These are returned to you in the <code>InitiateAuth</code> response if you need to pass another challenge. The responses in this parameter should be used to compute inputs to the next call (<code>RespondToAuthChallenge</code>). </p> <p>All challenges require <code>USERNAME</code> and <code>SECRET_HASH</code> (if applicable).</p>"]
    #[serde(rename="ChallengeParameters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub challenge_parameters: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>The session which should be passed both ways in challenge-response calls to the service. If the <a href=\"API_InitiateAuth.html\">InitiateAuth</a> or <a href=\"API_RespondToAuthChallenge.html\">RespondToAuthChallenge</a> API call determines that the caller needs to go through another challenge, they return a session with other challenge parameters. This session should be passed as it is to the next <code>RespondToAuthChallenge</code> API call.</p>"]
    #[serde(rename="Session")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub session: Option<String>,
}

#[doc="<p>Specifies the type of configuration for AWS Lambda triggers.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct LambdaConfigType {
    #[doc="<p>Creates an authentication challenge.</p>"]
    #[serde(rename="CreateAuthChallenge")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub create_auth_challenge: Option<String>,
    #[doc="<p>A custom Message AWS Lambda trigger.</p>"]
    #[serde(rename="CustomMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub custom_message: Option<String>,
    #[doc="<p>Defines the authentication challenge.</p>"]
    #[serde(rename="DefineAuthChallenge")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub define_auth_challenge: Option<String>,
    #[doc="<p>A post-authentication AWS Lambda trigger.</p>"]
    #[serde(rename="PostAuthentication")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub post_authentication: Option<String>,
    #[doc="<p>A post-confirmation AWS Lambda trigger.</p>"]
    #[serde(rename="PostConfirmation")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub post_confirmation: Option<String>,
    #[doc="<p>A pre-authentication AWS Lambda trigger.</p>"]
    #[serde(rename="PreAuthentication")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pre_authentication: Option<String>,
    #[doc="<p>A pre-registration AWS Lambda trigger.</p>"]
    #[serde(rename="PreSignUp")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pre_sign_up: Option<String>,
    #[doc="<p>Verifies the authentication challenge response.</p>"]
    #[serde(rename="VerifyAuthChallengeResponse")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub verify_auth_challenge_response: Option<String>,
}

#[doc="<p>Represents the request to list the devices.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListDevicesRequest {
    #[doc="<p>The access tokens for the request to list devices.</p>"]
    #[serde(rename="AccessToken")]
    pub access_token: String,
    #[doc="<p>The limit of the device request.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<i64>,
    #[doc="<p>The pagination token for the list request.</p>"]
    #[serde(rename="PaginationToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pagination_token: Option<String>,
}

#[doc="<p>Represents the response to list devices.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListDevicesResponse {
    #[doc="<p>The devices returned in the list devices response.</p>"]
    #[serde(rename="Devices")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub devices: Option<Vec<DeviceType>>,
    #[doc="<p>The pagination token for the list device response.</p>"]
    #[serde(rename="PaginationToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pagination_token: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListGroupsRequest {
    #[doc="<p>The limit of the request to list groups.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<i64>,
    #[doc="<p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The user pool ID for the user pool.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListGroupsResponse {
    #[doc="<p>The group objects for the groups.</p>"]
    #[serde(rename="Groups")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub groups: Option<Vec<GroupType>>,
    #[doc="<p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListIdentityProvidersRequest {
    #[doc="<p>The maximum number of identity providers to return.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>A pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The user pool ID.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListIdentityProvidersResponse {
    #[doc="<p>A pagination token.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>A list of identity provider objects.</p>"]
    #[serde(rename="Providers")]
    pub providers: Vec<ProviderDescription>,
}

#[doc="<p>Represents the request to list the user import jobs.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListUserImportJobsRequest {
    #[doc="<p>The maximum number of import jobs you want the request to return.</p>"]
    #[serde(rename="MaxResults")]
    pub max_results: i64,
    #[doc="<p>An identifier that was returned from the previous call to <code>ListUserImportJobs</code>, which can be used to return the next set of import jobs in the list.</p>"]
    #[serde(rename="PaginationToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pagination_token: Option<String>,
    #[doc="<p>The user pool ID for the user pool that the users are being imported into.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
}

#[doc="<p>Represents the response from the server to the request to list the user import jobs.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListUserImportJobsResponse {
    #[doc="<p>An identifier that can be used to return the next set of user import jobs in the list.</p>"]
    #[serde(rename="PaginationToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pagination_token: Option<String>,
    #[doc="<p>The user import jobs.</p>"]
    #[serde(rename="UserImportJobs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_import_jobs: Option<Vec<UserImportJobType>>,
}

#[doc="<p>Represents the request to list the user pool clients.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListUserPoolClientsRequest {
    #[doc="<p>The maximum number of results you want the request to return when listing the user pool clients.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The user pool ID for the user pool where you want to list user pool clients.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
}

#[doc="<p>Represents the response from the server that lists user pool clients.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListUserPoolClientsResponse {
    #[doc="<p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The user pool clients in the response that lists user pool clients.</p>"]
    #[serde(rename="UserPoolClients")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_pool_clients: Option<Vec<UserPoolClientDescription>>,
}

#[doc="<p>Represents the request to list user pools.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListUserPoolsRequest {
    #[doc="<p>The maximum number of results you want the request to return when listing the user pools.</p>"]
    #[serde(rename="MaxResults")]
    pub max_results: i64,
    #[doc="<p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[doc="<p>Represents the response to list user pools.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListUserPoolsResponse {
    #[doc="<p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The user pools from the response to list users.</p>"]
    #[serde(rename="UserPools")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_pools: Option<Vec<UserPoolDescriptionType>>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListUsersInGroupRequest {
    #[doc="<p>The name of the group.</p>"]
    #[serde(rename="GroupName")]
    pub group_name: String,
    #[doc="<p>The limit of the request to list users.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<i64>,
    #[doc="<p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The user pool ID for the user pool.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListUsersInGroupResponse {
    #[doc="<p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The users returned in the request to list users.</p>"]
    #[serde(rename="Users")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub users: Option<Vec<UserType>>,
}

#[doc="<p>Represents the request to list users.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListUsersRequest {
    #[doc="<p>An array of strings, where each string is the name of a user attribute to be returned for each user in the search results. If the array is empty, all attributes are returned.</p>"]
    #[serde(rename="AttributesToGet")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attributes_to_get: Option<Vec<String>>,
    #[doc="<p>A filter string of the form \"<i>AttributeName</i> <i>Filter-Type</i> \"<i>AttributeValue</i>\"\". Quotation marks within the filter string must be escaped using the backslash (\\) character. For example, \"<code>family_name</code> = \\\"Reddy\\\"\".</p> <ul> <li> <p> <i>AttributeName</i>: The name of the attribute to search for. You can only search for one attribute at a time.</p> </li> <li> <p> <i>Filter-Type</i>: For an exact match, use =, for example, \"<code>given_name</code> = \\\"Jon\\\"\". For a prefix (\"starts with\") match, use ^=, for example, \"<code>given_name</code> ^= \\\"Jon\\\"\". </p> </li> <li> <p> <i>AttributeValue</i>: The attribute value that must be matched for each user.</p> </li> </ul> <p>If the filter string is empty, <code>ListUsers</code> returns all users in the user pool.</p> <p>You can only search for the following standard attributes:</p> <ul> <li> <p> <code>username</code> (case-sensitive)</p> </li> <li> <p> <code>email</code> </p> </li> <li> <p> <code>phone_number</code> </p> </li> <li> <p> <code>name</code> </p> </li> <li> <p> <code>given_name</code> </p> </li> <li> <p> <code>family_name</code> </p> </li> <li> <p> <code>preferred_username</code> </p> </li> <li> <p> <code>cognito:user_status</code> (called <b>Enabled</b> in the Console) (case-sensitive)</p> </li> <li> <p> <code>status</code> (case-insensitive)</p> </li> </ul> <p>Custom attributes are not searchable.</p> <p>For more information, see <a href=\"http://docs.aws.amazon.com/cognito/latest/developerguide/how-to-manage-user-accounts.html#cognito-user-pools-searching-for-users-using-listusers-api\">Searching for Users Using the ListUsers API</a> and <a href=\"http://docs.aws.amazon.com/cognito/latest/developerguide/how-to-manage-user-accounts.html#cognito-user-pools-searching-for-users-listusers-api-examples\">Examples of Using the ListUsers API</a> in the <i>Amazon Cognito Developer Guide</i>.</p>"]
    #[serde(rename="Filter")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub filter: Option<String>,
    #[doc="<p>Maximum number of users to be returned.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<i64>,
    #[doc="<p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>"]
    #[serde(rename="PaginationToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pagination_token: Option<String>,
    #[doc="<p>The user pool ID for the user pool on which the search should be performed.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
}

#[doc="<p>The response from the request to list users.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListUsersResponse {
    #[doc="<p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>"]
    #[serde(rename="PaginationToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pagination_token: Option<String>,
    #[doc="<p>The users returned in the request to list users.</p>"]
    #[serde(rename="Users")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub users: Option<Vec<UserType>>,
}

#[doc="<p>Specifies the different settings for multi-factor authentication (MFA).</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct MFAOptionType {
    #[doc="<p>The attribute name of the MFA option type.</p>"]
    #[serde(rename="AttributeName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attribute_name: Option<String>,
    #[doc="<p>The delivery medium (email message or SMS message) to send the MFA code.</p>"]
    #[serde(rename="DeliveryMedium")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub delivery_medium: Option<String>,
}


#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum MessageActionType {
    Resend,
    Suppress,
}

impl Into<String> for MessageActionType {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for MessageActionType {
    fn into(self) -> &'static str {
        match self {
            MessageActionType::Resend => "RESEND",
            MessageActionType::Suppress => "SUPPRESS",
        }
    }
}

impl ::std::str::FromStr for MessageActionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RESEND" => Ok(MessageActionType::Resend),
            "SUPPRESS" => Ok(MessageActionType::Suppress),
            _ => Err(()),
        }
    }
}

#[doc="<p>The message template structure.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct MessageTemplateType {
    #[doc="<p>The message template for email messages.</p>"]
    #[serde(rename="EmailMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email_message: Option<String>,
    #[doc="<p>The subject line for email messages.</p>"]
    #[serde(rename="EmailSubject")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email_subject: Option<String>,
    #[doc="<p>The message template for SMS messages.</p>"]
    #[serde(rename="SMSMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sms_message: Option<String>,
}

#[doc="<p>The new device metadata type.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct NewDeviceMetadataType {
    #[doc="<p>The device group key.</p>"]
    #[serde(rename="DeviceGroupKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub device_group_key: Option<String>,
    #[doc="<p>The device key.</p>"]
    #[serde(rename="DeviceKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub device_key: Option<String>,
}

#[doc="<p>The minimum and maximum value of an attribute that is of the number data type.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct NumberAttributeConstraintsType {
    #[doc="<p>The maximum value of an attribute that is of the number data type.</p>"]
    #[serde(rename="MaxValue")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_value: Option<String>,
    #[doc="<p>The minimum value of an attribute that is of the number data type.</p>"]
    #[serde(rename="MinValue")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub min_value: Option<String>,
}


#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum OAuthFlowType {
    ClientCredentials,
    Code,
    Implicit,
}

impl Into<String> for OAuthFlowType {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for OAuthFlowType {
    fn into(self) -> &'static str {
        match self {
            OAuthFlowType::ClientCredentials => "client_credentials",
            OAuthFlowType::Code => "code",
            OAuthFlowType::Implicit => "implicit",
        }
    }
}

impl ::std::str::FromStr for OAuthFlowType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "client_credentials" => Ok(OAuthFlowType::ClientCredentials),
            "code" => Ok(OAuthFlowType::Code),
            "implicit" => Ok(OAuthFlowType::Implicit),
            _ => Err(()),
        }
    }
}

#[doc="<p>The password policy type.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct PasswordPolicyType {
    #[doc="<p>The minimum length of the password policy that you have set. Cannot be less than 6.</p>"]
    #[serde(rename="MinimumLength")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub minimum_length: Option<i64>,
    #[doc="<p>In the password policy that you have set, refers to whether you have required users to use at least one lowercase letter in their password.</p>"]
    #[serde(rename="RequireLowercase")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub require_lowercase: Option<bool>,
    #[doc="<p>In the password policy that you have set, refers to whether you have required users to use at least one number in their password.</p>"]
    #[serde(rename="RequireNumbers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub require_numbers: Option<bool>,
    #[doc="<p>In the password policy that you have set, refers to whether you have required users to use at least one symbol in their password.</p>"]
    #[serde(rename="RequireSymbols")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub require_symbols: Option<bool>,
    #[doc="<p>In the password policy that you have set, refers to whether you have required users to use at least one uppercase letter in their password.</p>"]
    #[serde(rename="RequireUppercase")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub require_uppercase: Option<bool>,
}

#[doc="<p>A container for identity provider details.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ProviderDescription {
    #[doc="<p>The date the provider was added to the user pool.</p>"]
    #[serde(rename="CreationDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creation_date: Option<f64>,
    #[doc="<p>The date the provider was last modified.</p>"]
    #[serde(rename="LastModifiedDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[doc="<p>The identity provider name.</p>"]
    #[serde(rename="ProviderName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub provider_name: Option<String>,
    #[doc="<p>The identity provider type.</p>"]
    #[serde(rename="ProviderType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub provider_type: Option<String>,
}

#[doc="<p>Represents the request to resend the confirmation code.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ResendConfirmationCodeRequest {
    #[doc="<p>The ID of the client associated with the user pool.</p>"]
    #[serde(rename="ClientId")]
    pub client_id: String,
    #[doc="<p>A keyed-hash message authentication code (HMAC) calculated using the secret key of a user pool client and username plus the client ID in the message.</p>"]
    #[serde(rename="SecretHash")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub secret_hash: Option<String>,
    #[doc="<p>The user name of the user to whom you wish to resend a confirmation code.</p>"]
    #[serde(rename="Username")]
    pub username: String,
}

#[doc="<p>The response from the server when the Amazon Cognito Your User Pools service makes the request to resend a confirmation code.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ResendConfirmationCodeResponse {
    #[doc="<p>The code delivery details returned by the server in response to the request to resend the confirmation code.</p>"]
    #[serde(rename="CodeDeliveryDetails")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub code_delivery_details: Option<CodeDeliveryDetailsType>,
}

#[doc="<p>The request to respond to an authentication challenge.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct RespondToAuthChallengeRequest {
    #[doc="<p>The challenge name. For more information, see <a href=\"API_InitiateAuth.html\">InitiateAuth</a>.</p> <p> <code>ADMIN_NO_SRP_AUTH</code> is not a valid value.</p>"]
    #[serde(rename="ChallengeName")]
    pub challenge_name: String,
    #[doc="<p>The challenge responses. These are inputs corresponding to the value of <code>ChallengeName</code>, for example:</p> <ul> <li> <p> <code>SMS_MFA</code>: <code>SMS_MFA_CODE</code>, <code>USERNAME</code>, <code>SECRET_HASH</code> (if app client is configured with client secret).</p> </li> <li> <p> <code>PASSWORD_VERIFIER</code>: <code>PASSWORD_CLAIM_SIGNATURE</code>, <code>PASSWORD_CLAIM_SECRET_BLOCK</code>, <code>TIMESTAMP</code>, <code>USERNAME</code>, <code>SECRET_HASH</code> (if app client is configured with client secret).</p> </li> <li> <p> <code>NEW_PASSWORD_REQUIRED</code>: <code>NEW_PASSWORD</code>, any other required attributes, <code>USERNAME</code>, <code>SECRET_HASH</code> (if app client is configured with client secret). </p> </li> </ul>"]
    #[serde(rename="ChallengeResponses")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub challenge_responses: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>The app client ID.</p>"]
    #[serde(rename="ClientId")]
    pub client_id: String,
    #[doc="<p>The session which should be passed both ways in challenge-response calls to the service. If <code>InitiateAuth</code> or <code>RespondToAuthChallenge</code> API call determines that the caller needs to go through another challenge, they return a session with other challenge parameters. This session should be passed as it is to the next <code>RespondToAuthChallenge</code> API call.</p>"]
    #[serde(rename="Session")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub session: Option<String>,
}

#[doc="<p>The response to respond to the authentication challenge.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct RespondToAuthChallengeResponse {
    #[doc="<p>The result returned by the server in response to the request to respond to the authentication challenge.</p>"]
    #[serde(rename="AuthenticationResult")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub authentication_result: Option<AuthenticationResultType>,
    #[doc="<p>The challenge name. For more information, see <a href=\"API_InitiateAuth.html\">InitiateAuth</a>.</p>"]
    #[serde(rename="ChallengeName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub challenge_name: Option<String>,
    #[doc="<p>The challenge parameters. For more information, see <a href=\"API_InitiateAuth.html\">InitiateAuth</a>.</p>"]
    #[serde(rename="ChallengeParameters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub challenge_parameters: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>The session which should be passed both ways in challenge-response calls to the service. If the <a href=\"API_InitiateAuth.html\">InitiateAuth</a> or <a href=\"API_RespondToAuthChallenge.html\">RespondToAuthChallenge</a> API call determines that the caller needs to go through another challenge, they return a session with other challenge parameters. This session should be passed as it is to the next <code>RespondToAuthChallenge</code> API call.</p>"]
    #[serde(rename="Session")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub session: Option<String>,
}

#[doc="<p>Contains information about the schema attribute.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct SchemaAttributeType {
    #[doc="<p>The attribute data type.</p>"]
    #[serde(rename="AttributeDataType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attribute_data_type: Option<String>,
    #[doc="<p>Specifies whether the attribute type is developer only.</p>"]
    #[serde(rename="DeveloperOnlyAttribute")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub developer_only_attribute: Option<bool>,
    #[doc="<p>Specifies whether the attribute can be changed once it has been created.</p>"]
    #[serde(rename="Mutable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mutable: Option<bool>,
    #[doc="<p>A schema attribute of the name type.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>Specifies the constraints for an attribute of the number type.</p>"]
    #[serde(rename="NumberAttributeConstraints")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub number_attribute_constraints: Option<NumberAttributeConstraintsType>,
    #[doc="<p>Specifies whether a user pool attribute is required. If the attribute is required and the user does not provide a value, registration or sign-in will fail.</p>"]
    #[serde(rename="Required")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub required: Option<bool>,
    #[doc="<p>Specifies the constraints for an attribute of the string type.</p>"]
    #[serde(rename="StringAttributeConstraints")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub string_attribute_constraints: Option<StringAttributeConstraintsType>,
}

#[doc="<p>Represents the request to set user settings.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct SetUserSettingsRequest {
    #[doc="<p>The access token for the set user settings request.</p>"]
    #[serde(rename="AccessToken")]
    pub access_token: String,
    #[doc="<p>Specifies the options for MFA (e.g., email or phone number).</p>"]
    #[serde(rename="MFAOptions")]
    pub mfa_options: Vec<MFAOptionType>,
}

#[doc="<p>The response from the server for a set user settings request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct SetUserSettingsResponse;

#[doc="<p>Represents the request to register a user.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct SignUpRequest {
    #[doc="<p>The ID of the client associated with the user pool.</p>"]
    #[serde(rename="ClientId")]
    pub client_id: String,
    #[doc="<p>The password of the user you wish to register.</p>"]
    #[serde(rename="Password")]
    pub password: String,
    #[doc="<p>A keyed-hash message authentication code (HMAC) calculated using the secret key of a user pool client and username plus the client ID in the message.</p>"]
    #[serde(rename="SecretHash")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub secret_hash: Option<String>,
    #[doc="<p>An array of name-value pairs representing user attributes.</p> <p>For custom attributes, you must prepend the <code>custom:</code> prefix to the attribute name.</p>"]
    #[serde(rename="UserAttributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_attributes: Option<Vec<AttributeType>>,
    #[doc="<p>The user name of the user you wish to register.</p>"]
    #[serde(rename="Username")]
    pub username: String,
    #[doc="<p>The validation data in the request to register a user.</p>"]
    #[serde(rename="ValidationData")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub validation_data: Option<Vec<AttributeType>>,
}

#[doc="<p>The response from the server for a registration request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct SignUpResponse {
    #[doc="<p>The code delivery details returned by the server response to the user registration request.</p>"]
    #[serde(rename="CodeDeliveryDetails")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub code_delivery_details: Option<CodeDeliveryDetailsType>,
    #[doc="<p>A response from the server indicating that a user registration has been confirmed.</p>"]
    #[serde(rename="UserConfirmed")]
    pub user_confirmed: bool,
    #[doc="<p>The UUID of the authenticated user. This is not the same as <code>username</code>.</p>"]
    #[serde(rename="UserSub")]
    pub user_sub: String,
}

#[doc="<p>The SMS configuration type.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct SmsConfigurationType {
    #[doc="<p>The external ID.</p>"]
    #[serde(rename="ExternalId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub external_id: Option<String>,
    #[doc="<p>The Amazon Resource Name (ARN) of the Amazon Simple Notification Service (SNS) caller.</p>"]
    #[serde(rename="SnsCallerArn")]
    pub sns_caller_arn: String,
}

#[doc="<p>Represents the request to start the user import job.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct StartUserImportJobRequest {
    #[doc="<p>The job ID for the user import job.</p>"]
    #[serde(rename="JobId")]
    pub job_id: String,
    #[doc="<p>The user pool ID for the user pool that the users are being imported into.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
}

#[doc="<p>Represents the response from the server to the request to start the user import job.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct StartUserImportJobResponse {
    #[doc="<p>The job object that represents the user import job.</p>"]
    #[serde(rename="UserImportJob")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_import_job: Option<UserImportJobType>,
}


#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum StatusType {
    Disabled,
    Enabled,
}

impl Into<String> for StatusType {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for StatusType {
    fn into(self) -> &'static str {
        match self {
            StatusType::Disabled => "Disabled",
            StatusType::Enabled => "Enabled",
        }
    }
}

impl ::std::str::FromStr for StatusType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Disabled" => Ok(StatusType::Disabled),
            "Enabled" => Ok(StatusType::Enabled),
            _ => Err(()),
        }
    }
}

#[doc="<p>Represents the request to stop the user import job.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct StopUserImportJobRequest {
    #[doc="<p>The job ID for the user import job.</p>"]
    #[serde(rename="JobId")]
    pub job_id: String,
    #[doc="<p>The user pool ID for the user pool that the users are being imported into.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
}

#[doc="<p>Represents the response from the server to the request to stop the user import job.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct StopUserImportJobResponse {
    #[doc="<p>The job object that represents the user import job.</p>"]
    #[serde(rename="UserImportJob")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_import_job: Option<UserImportJobType>,
}

#[doc="<p>The type of constraints associated with an attribute of the string type.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct StringAttributeConstraintsType {
    #[doc="<p>The maximum length of an attribute value of the string type.</p>"]
    #[serde(rename="MaxLength")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_length: Option<String>,
    #[doc="<p>The minimum length of an attribute value of the string type.</p>"]
    #[serde(rename="MinLength")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub min_length: Option<String>,
}

#[doc="<p>Represents the request to update the device status.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateDeviceStatusRequest {
    #[doc="<p>The access token.</p>"]
    #[serde(rename="AccessToken")]
    pub access_token: String,
    #[doc="<p>The device key.</p>"]
    #[serde(rename="DeviceKey")]
    pub device_key: String,
    #[doc="<p>The status of whether a device is remembered.</p>"]
    #[serde(rename="DeviceRememberedStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub device_remembered_status: Option<String>,
}

#[doc="<p>The response to the request to update the device status.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateDeviceStatusResponse;

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateGroupRequest {
    #[doc="<p>A string containing the new description of the group.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[doc="<p>The name of the group.</p>"]
    #[serde(rename="GroupName")]
    pub group_name: String,
    #[doc="<p>The new precedence value for the group. For more information about this parameter, see <a href=\"API_CreateGroup.html\">CreateGroup</a>.</p>"]
    #[serde(rename="Precedence")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub precedence: Option<i64>,
    #[doc="<p>The new role ARN for the group. This is used for setting the <code>cognito:roles</code> and <code>cognito:preferred_role</code> claims in the token.</p>"]
    #[serde(rename="RoleArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role_arn: Option<String>,
    #[doc="<p>The user pool ID for the user pool.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateGroupResponse {
    #[doc="<p>The group object for the group.</p>"]
    #[serde(rename="Group")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub group: Option<GroupType>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateIdentityProviderRequest {
    #[doc="<p>The identity provider attribute mapping to be changed.</p>"]
    #[serde(rename="AttributeMapping")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attribute_mapping: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>A list of identity provider identifiers.</p>"]
    #[serde(rename="IdpIdentifiers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub idp_identifiers: Option<Vec<String>>,
    #[doc="<p>The identity provider details to be updated, such as <code>MetadataURL</code> and <code>MetadataFile</code>.</p>"]
    #[serde(rename="ProviderDetails")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub provider_details: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>The identity provider name.</p>"]
    #[serde(rename="ProviderName")]
    pub provider_name: String,
    #[doc="<p>The user pool ID.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateIdentityProviderResponse {
    #[doc="<p>The identity provider object.</p>"]
    #[serde(rename="IdentityProvider")]
    pub identity_provider: IdentityProviderType,
}

#[doc="<p>Represents the request to update user attributes.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateUserAttributesRequest {
    #[doc="<p>The access token for the request to update user attributes.</p>"]
    #[serde(rename="AccessToken")]
    pub access_token: String,
    #[doc="<p>An array of name-value pairs representing user attributes.</p> <p>For custom attributes, you must prepend the <code>custom:</code> prefix to the attribute name.</p>"]
    #[serde(rename="UserAttributes")]
    pub user_attributes: Vec<AttributeType>,
}

#[doc="<p>Represents the response from the server for the request to update user attributes.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateUserAttributesResponse {
    #[doc="<p>The code delivery details list from the server for the request to update user attributes.</p>"]
    #[serde(rename="CodeDeliveryDetailsList")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub code_delivery_details_list: Option<Vec<CodeDeliveryDetailsType>>,
}

#[doc="<p>Represents the request to update the user pool client.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateUserPoolClientRequest {
    #[doc="<p>Set to <code>code</code> to initiate a code grant flow, which provides an authorization code as the response. This code can be exchanged for access tokens with the token endpoint.</p> <p>Set to <code>token</code> to specify that the client should get the access token (and, optionally, ID token, based on scopes) directly.</p>"]
    #[serde(rename="AllowedOAuthFlows")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allowed_o_auth_flows: Option<Vec<String>>,
    #[doc="<p>Set to TRUE if the client is allowed to follow the OAuth protocol when interacting with Cognito user pools.</p>"]
    #[serde(rename="AllowedOAuthFlowsUserPoolClient")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allowed_o_auth_flows_user_pool_client: Option<bool>,
    #[doc="<p>A list of allowed <code>OAuth</code> scopes. Currently supported values are <code>\"phone\"</code>, <code>\"email\"</code>, <code>\"openid\"</code>, and <code>\"Cognito\"</code>.</p>"]
    #[serde(rename="AllowedOAuthScopes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allowed_o_auth_scopes: Option<Vec<String>>,
    #[doc="<p>A list of allowed callback URLs for the identity providers.</p>"]
    #[serde(rename="CallbackURLs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub callback_ur_ls: Option<Vec<String>>,
    #[doc="<p>The ID of the client associated with the user pool.</p>"]
    #[serde(rename="ClientId")]
    pub client_id: String,
    #[doc="<p>The client name from the update user pool client request.</p>"]
    #[serde(rename="ClientName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub client_name: Option<String>,
    #[doc="<p>The default redirect URI. Must be in the <code>CallbackURLs</code> list.</p>"]
    #[serde(rename="DefaultRedirectURI")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_redirect_uri: Option<String>,
    #[doc="<p>Explicit authentication flows.</p>"]
    #[serde(rename="ExplicitAuthFlows")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub explicit_auth_flows: Option<Vec<String>>,
    #[doc="<p>A list ofallowed logout URLs for the identity providers.</p>"]
    #[serde(rename="LogoutURLs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub logout_ur_ls: Option<Vec<String>>,
    #[doc="<p>The read-only attributes of the user pool.</p>"]
    #[serde(rename="ReadAttributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub read_attributes: Option<Vec<String>>,
    #[doc="<p>The time limit, in days, after which the refresh token is no longer valid and cannot be used.</p>"]
    #[serde(rename="RefreshTokenValidity")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub refresh_token_validity: Option<i64>,
    #[doc="<p>A list of provider names for the identity providers that are supported on this client.</p>"]
    #[serde(rename="SupportedIdentityProviders")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub supported_identity_providers: Option<Vec<String>>,
    #[doc="<p>The user pool ID for the user pool where you want to update the user pool client.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
    #[doc="<p>The writeable attributes of the user pool.</p>"]
    #[serde(rename="WriteAttributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub write_attributes: Option<Vec<String>>,
}

#[doc="<p>Represents the response from the server to the request to update the user pool client.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateUserPoolClientResponse {
    #[doc="<p>The user pool client value from the response from the server when an update user pool client request is made.</p>"]
    #[serde(rename="UserPoolClient")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_pool_client: Option<UserPoolClientType>,
}

#[doc="<p>Represents the request to update the user pool.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateUserPoolRequest {
    #[doc="<p>The configuration for <code>AdminCreateUser</code> requests.</p>"]
    #[serde(rename="AdminCreateUserConfig")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub admin_create_user_config: Option<AdminCreateUserConfigType>,
    #[doc="<p>The attributes that are automatically verified when the Amazon Cognito service makes a request to update user pools.</p>"]
    #[serde(rename="AutoVerifiedAttributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auto_verified_attributes: Option<Vec<String>>,
    #[doc="<p>Device configuration.</p>"]
    #[serde(rename="DeviceConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub device_configuration: Option<DeviceConfigurationType>,
    #[doc="<p>Email configuration.</p>"]
    #[serde(rename="EmailConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email_configuration: Option<EmailConfigurationType>,
    #[doc="<p>The contents of the email verification message.</p>"]
    #[serde(rename="EmailVerificationMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email_verification_message: Option<String>,
    #[doc="<p>The subject of the email verification message.</p>"]
    #[serde(rename="EmailVerificationSubject")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email_verification_subject: Option<String>,
    #[doc="<p>The AWS Lambda configuration information from the request to update the user pool.</p>"]
    #[serde(rename="LambdaConfig")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub lambda_config: Option<LambdaConfigType>,
    #[doc="<p>Can be one of the following values:</p> <ul> <li> <p> <code>OFF</code> - MFA tokens are not required and cannot be specified during user registration.</p> </li> <li> <p> <code>ON</code> - MFA tokens are required for all user registrations. You can only specify required when you are initially creating a user pool.</p> </li> <li> <p> <code>OPTIONAL</code> - Users have the option when registering to create an MFA token.</p> </li> </ul>"]
    #[serde(rename="MfaConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mfa_configuration: Option<String>,
    #[doc="<p>A container with the policies you wish to update in a user pool.</p>"]
    #[serde(rename="Policies")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub policies: Option<UserPoolPolicyType>,
    #[doc="<p>The contents of the SMS authentication message.</p>"]
    #[serde(rename="SmsAuthenticationMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sms_authentication_message: Option<String>,
    #[doc="<p>SMS configuration.</p>"]
    #[serde(rename="SmsConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sms_configuration: Option<SmsConfigurationType>,
    #[doc="<p>A container with information about the SMS verification message.</p>"]
    #[serde(rename="SmsVerificationMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sms_verification_message: Option<String>,
    #[doc="<p>The user pool ID for the user pool you want to update.</p>"]
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
    #[doc="<p>The cost allocation tags for the user pool. For more information, see <a href=\"http://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-cost-allocation-tagging.html\">Adding Cost Allocation Tags to Your User Pool</a> </p>"]
    #[serde(rename="UserPoolTags")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_pool_tags: Option<::std::collections::HashMap<String, String>>,
}

#[doc="<p>Represents the response from the server when you make a request to update the user pool.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateUserPoolResponse;


#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum UserImportJobStatusType {
    Created,
    Expired,
    Failed,
    InProgress,
    Pending,
    Stopped,
    Stopping,
    Succeeded,
}

impl Into<String> for UserImportJobStatusType {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for UserImportJobStatusType {
    fn into(self) -> &'static str {
        match self {
            UserImportJobStatusType::Created => "Created",
            UserImportJobStatusType::Expired => "Expired",
            UserImportJobStatusType::Failed => "Failed",
            UserImportJobStatusType::InProgress => "InProgress",
            UserImportJobStatusType::Pending => "Pending",
            UserImportJobStatusType::Stopped => "Stopped",
            UserImportJobStatusType::Stopping => "Stopping",
            UserImportJobStatusType::Succeeded => "Succeeded",
        }
    }
}

impl ::std::str::FromStr for UserImportJobStatusType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Created" => Ok(UserImportJobStatusType::Created),
            "Expired" => Ok(UserImportJobStatusType::Expired),
            "Failed" => Ok(UserImportJobStatusType::Failed),
            "InProgress" => Ok(UserImportJobStatusType::InProgress),
            "Pending" => Ok(UserImportJobStatusType::Pending),
            "Stopped" => Ok(UserImportJobStatusType::Stopped),
            "Stopping" => Ok(UserImportJobStatusType::Stopping),
            "Succeeded" => Ok(UserImportJobStatusType::Succeeded),
            _ => Err(()),
        }
    }
}

#[doc="<p>The user import job type.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct UserImportJobType {
    #[doc="<p>The role ARN for the Amazon CloudWatch Logging role for the user import job. For more information, see \"Creating the CloudWatch Logs IAM Role\" in the Amazon Cognito Developer Guide.</p>"]
    #[serde(rename="CloudWatchLogsRoleArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cloud_watch_logs_role_arn: Option<String>,
    #[doc="<p>The date when the user import job was completed.</p>"]
    #[serde(rename="CompletionDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub completion_date: Option<f64>,
    #[doc="<p>The message returned when the user import job is completed.</p>"]
    #[serde(rename="CompletionMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub completion_message: Option<String>,
    #[doc="<p>The date the user import job was created.</p>"]
    #[serde(rename="CreationDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creation_date: Option<f64>,
    #[doc="<p>The number of users that could not be imported.</p>"]
    #[serde(rename="FailedUsers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub failed_users: Option<i64>,
    #[doc="<p>The number of users that were successfully imported.</p>"]
    #[serde(rename="ImportedUsers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub imported_users: Option<i64>,
    #[doc="<p>The job ID for the user import job.</p>"]
    #[serde(rename="JobId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub job_id: Option<String>,
    #[doc="<p>The job name for the user import job.</p>"]
    #[serde(rename="JobName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub job_name: Option<String>,
    #[doc="<p>The pre-signed URL to be used to upload the <code>.csv</code> file.</p>"]
    #[serde(rename="PreSignedUrl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pre_signed_url: Option<String>,
    #[doc="<p>The number of users that were skipped.</p>"]
    #[serde(rename="SkippedUsers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub skipped_users: Option<i64>,
    #[doc="<p>The date when the user import job was started.</p>"]
    #[serde(rename="StartDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_date: Option<f64>,
    #[doc="<p>The status of the user import job. One of the following:</p> <ul> <li> <p> <code>Created</code> - The job was created but not started.</p> </li> <li> <p> <code>Pending</code> - A transition state. You have started the job, but it has not begun importing users yet.</p> </li> <li> <p> <code>InProgress</code> - The job has started, and users are being imported.</p> </li> <li> <p> <code>Stopping</code> - You have stopped the job, but the job has not stopped importing users yet.</p> </li> <li> <p> <code>Stopped</code> - You have stopped the job, and the job has stopped importing users.</p> </li> <li> <p> <code>Succeeded</code> - The job has completed successfully.</p> </li> <li> <p> <code>Failed</code> - The job has stopped due to an error.</p> </li> <li> <p> <code>Expired</code> - You created a job, but did not start the job within 24-48 hours. All data associated with the job was deleted, and the job cannot be started.</p> </li> </ul>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
    #[doc="<p>The user pool ID for the user pool that the users are being imported into.</p>"]
    #[serde(rename="UserPoolId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_pool_id: Option<String>,
}

#[doc="<p>The description of the user pool client.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct UserPoolClientDescription {
    #[doc="<p>The ID of the client associated with the user pool.</p>"]
    #[serde(rename="ClientId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub client_id: Option<String>,
    #[doc="<p>The client name from the user pool client description.</p>"]
    #[serde(rename="ClientName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub client_name: Option<String>,
    #[doc="<p>The user pool ID for the user pool where you want to describe the user pool client.</p>"]
    #[serde(rename="UserPoolId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_pool_id: Option<String>,
}

#[doc="<p>A user pool of the client type.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct UserPoolClientType {
    #[doc="<p>Set to <code>code</code> to initiate a code grant flow, which provides an authorization code as the response. This code can be exchanged for access tokens with the token endpoint.</p> <p>Set to <code>token</code> to specify that the client should get the access token (and, optionally, ID token, based on scopes) directly.</p>"]
    #[serde(rename="AllowedOAuthFlows")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allowed_o_auth_flows: Option<Vec<String>>,
    #[doc="<p>Set to TRUE if the client is allowed to follow the OAuth protocol when interacting with Cognito user pools.</p>"]
    #[serde(rename="AllowedOAuthFlowsUserPoolClient")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allowed_o_auth_flows_user_pool_client: Option<bool>,
    #[doc="<p>A list of allowed <code>OAuth</code> scopes. Currently supported values are <code>\"phone\"</code>, <code>\"email\"</code>, <code>\"openid\"</code>, and <code>\"Cognito\"</code>.</p>"]
    #[serde(rename="AllowedOAuthScopes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allowed_o_auth_scopes: Option<Vec<String>>,
    #[doc="<p>A list of allowed callback URLs for the identity providers.</p>"]
    #[serde(rename="CallbackURLs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub callback_ur_ls: Option<Vec<String>>,
    #[doc="<p>The ID of the client associated with the user pool.</p>"]
    #[serde(rename="ClientId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub client_id: Option<String>,
    #[doc="<p>The client name from the user pool request of the client type.</p>"]
    #[serde(rename="ClientName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub client_name: Option<String>,
    #[doc="<p>The client secret from the user pool request of the client type.</p>"]
    #[serde(rename="ClientSecret")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub client_secret: Option<String>,
    #[doc="<p>The date the user pool client was created.</p>"]
    #[serde(rename="CreationDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creation_date: Option<f64>,
    #[doc="<p>The default redirect URI. Must be in the <code>CallbackURLs</code> list.</p>"]
    #[serde(rename="DefaultRedirectURI")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_redirect_uri: Option<String>,
    #[doc="<p>The explicit authentication flows.</p>"]
    #[serde(rename="ExplicitAuthFlows")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub explicit_auth_flows: Option<Vec<String>>,
    #[doc="<p>The date the user pool client was last modified.</p>"]
    #[serde(rename="LastModifiedDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[doc="<p>A list ofallowed logout URLs for the identity providers.</p>"]
    #[serde(rename="LogoutURLs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub logout_ur_ls: Option<Vec<String>>,
    #[doc="<p>The Read-only attributes.</p>"]
    #[serde(rename="ReadAttributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub read_attributes: Option<Vec<String>>,
    #[doc="<p>The time limit, in days, after which the refresh token is no longer valid and cannot be used.</p>"]
    #[serde(rename="RefreshTokenValidity")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub refresh_token_validity: Option<i64>,
    #[doc="<p>A list of provider names for the identity providers that are supported on this client.</p>"]
    #[serde(rename="SupportedIdentityProviders")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub supported_identity_providers: Option<Vec<String>>,
    #[doc="<p>The user pool ID for the user pool client.</p>"]
    #[serde(rename="UserPoolId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_pool_id: Option<String>,
    #[doc="<p>The writeable attributes.</p>"]
    #[serde(rename="WriteAttributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub write_attributes: Option<Vec<String>>,
}

#[doc="<p>A user pool description.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct UserPoolDescriptionType {
    #[doc="<p>The date the user pool description was created.</p>"]
    #[serde(rename="CreationDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creation_date: Option<f64>,
    #[doc="<p>The ID in a user pool description.</p>"]
    #[serde(rename="Id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,
    #[doc="<p>The AWS Lambda configuration information in a user pool description.</p>"]
    #[serde(rename="LambdaConfig")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub lambda_config: Option<LambdaConfigType>,
    #[doc="<p>The date the user pool description was last modified.</p>"]
    #[serde(rename="LastModifiedDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[doc="<p>The name in a user pool description.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>The user pool status in a user pool description.</p>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
}


#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum UserPoolMfaType {
    Off,
    On,
    Optional,
}

impl Into<String> for UserPoolMfaType {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for UserPoolMfaType {
    fn into(self) -> &'static str {
        match self {
            UserPoolMfaType::Off => "OFF",
            UserPoolMfaType::On => "ON",
            UserPoolMfaType::Optional => "OPTIONAL",
        }
    }
}

impl ::std::str::FromStr for UserPoolMfaType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "OFF" => Ok(UserPoolMfaType::Off),
            "ON" => Ok(UserPoolMfaType::On),
            "OPTIONAL" => Ok(UserPoolMfaType::Optional),
            _ => Err(()),
        }
    }
}

#[doc="<p>The type of policy in a user pool.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct UserPoolPolicyType {
    #[doc="<p>A container for information about the user pool password policy.</p>"]
    #[serde(rename="PasswordPolicy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub password_policy: Option<PasswordPolicyType>,
}

#[doc="<p>A container for information about the user pool type.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct UserPoolType {
    #[doc="<p>The configuration for <code>AdminCreateUser</code> requests.</p>"]
    #[serde(rename="AdminCreateUserConfig")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub admin_create_user_config: Option<AdminCreateUserConfigType>,
    #[doc="<p>Specifies the attributes that are aliased in a user pool.</p>"]
    #[serde(rename="AliasAttributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub alias_attributes: Option<Vec<String>>,
    #[doc="<p>Specifies the attributes that are auto-verified in a user pool.</p>"]
    #[serde(rename="AutoVerifiedAttributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auto_verified_attributes: Option<Vec<String>>,
    #[doc="<p>The date the user pool was created.</p>"]
    #[serde(rename="CreationDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creation_date: Option<f64>,
    #[doc="<p>The device configuration.</p>"]
    #[serde(rename="DeviceConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub device_configuration: Option<DeviceConfigurationType>,
    #[doc="<p>The email configuration.</p>"]
    #[serde(rename="EmailConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email_configuration: Option<EmailConfigurationType>,
    #[doc="<p>The reason why the email configuration cannot send the messages to your users.</p>"]
    #[serde(rename="EmailConfigurationFailure")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email_configuration_failure: Option<String>,
    #[doc="<p>The contents of the email verification message.</p>"]
    #[serde(rename="EmailVerificationMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email_verification_message: Option<String>,
    #[doc="<p>The subject of the email verification message.</p>"]
    #[serde(rename="EmailVerificationSubject")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email_verification_subject: Option<String>,
    #[doc="<p>A number estimating the size of the user pool.</p>"]
    #[serde(rename="EstimatedNumberOfUsers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub estimated_number_of_users: Option<i64>,
    #[doc="<p>The ID of the user pool.</p>"]
    #[serde(rename="Id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,
    #[doc="<p>A container for the AWS Lambda triggers associated with a user pool.</p>"]
    #[serde(rename="LambdaConfig")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub lambda_config: Option<LambdaConfigType>,
    #[doc="<p>The date the user pool was last modified.</p>"]
    #[serde(rename="LastModifiedDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[doc="<p>Can be one of the following values:</p> <ul> <li> <p> <code>OFF</code> - MFA tokens are not required and cannot be specified during user registration.</p> </li> <li> <p> <code>ON</code> - MFA tokens are required for all user registrations. You can only specify required when you are initially creating a user pool.</p> </li> <li> <p> <code>OPTIONAL</code> - Users have the option when registering to create an MFA token.</p> </li> </ul>"]
    #[serde(rename="MfaConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mfa_configuration: Option<String>,
    #[doc="<p>The name of the user pool.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>A container for the policies associated with a user pool.</p>"]
    #[serde(rename="Policies")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub policies: Option<UserPoolPolicyType>,
    #[doc="<p>A container with the schema attributes of a user pool.</p>"]
    #[serde(rename="SchemaAttributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schema_attributes: Option<Vec<SchemaAttributeType>>,
    #[doc="<p>The contents of the SMS authentication message.</p>"]
    #[serde(rename="SmsAuthenticationMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sms_authentication_message: Option<String>,
    #[doc="<p>The SMS configuration.</p>"]
    #[serde(rename="SmsConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sms_configuration: Option<SmsConfigurationType>,
    #[doc="<p>The reason why the SMS configuration cannot send the messages to your users.</p>"]
    #[serde(rename="SmsConfigurationFailure")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sms_configuration_failure: Option<String>,
    #[doc="<p>The contents of the SMS verification message.</p>"]
    #[serde(rename="SmsVerificationMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sms_verification_message: Option<String>,
    #[doc="<p>The status of a user pool.</p>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
    #[doc="<p>The cost allocation tags for the user pool. For more information, see <a href=\"http://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-cost-allocation-tagging.html\">Adding Cost Allocation Tags to Your User Pool</a> </p>"]
    #[serde(rename="UserPoolTags")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_pool_tags: Option<::std::collections::HashMap<String, String>>,
}


#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum UserStatusType {
    Archived,
    Compromised,
    Confirmed,
    ForceChangePassword,
    ResetRequired,
    Unconfirmed,
    Unknown,
}

impl Into<String> for UserStatusType {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for UserStatusType {
    fn into(self) -> &'static str {
        match self {
            UserStatusType::Archived => "ARCHIVED",
            UserStatusType::Compromised => "COMPROMISED",
            UserStatusType::Confirmed => "CONFIRMED",
            UserStatusType::ForceChangePassword => "FORCE_CHANGE_PASSWORD",
            UserStatusType::ResetRequired => "RESET_REQUIRED",
            UserStatusType::Unconfirmed => "UNCONFIRMED",
            UserStatusType::Unknown => "UNKNOWN",
        }
    }
}

impl ::std::str::FromStr for UserStatusType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ARCHIVED" => Ok(UserStatusType::Archived),
            "COMPROMISED" => Ok(UserStatusType::Compromised),
            "CONFIRMED" => Ok(UserStatusType::Confirmed),
            "FORCE_CHANGE_PASSWORD" => Ok(UserStatusType::ForceChangePassword),
            "RESET_REQUIRED" => Ok(UserStatusType::ResetRequired),
            "UNCONFIRMED" => Ok(UserStatusType::Unconfirmed),
            "UNKNOWN" => Ok(UserStatusType::Unknown),
            _ => Err(()),
        }
    }
}

#[doc="<p>The user type.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct UserType {
    #[doc="<p>A container with information about the user type attributes.</p>"]
    #[serde(rename="Attributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attributes: Option<Vec<AttributeType>>,
    #[doc="<p>Specifies whether the user is enabled.</p>"]
    #[serde(rename="Enabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enabled: Option<bool>,
    #[doc="<p>The MFA options for the user.</p>"]
    #[serde(rename="MFAOptions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mfa_options: Option<Vec<MFAOptionType>>,
    #[doc="<p>The creation date of the user.</p>"]
    #[serde(rename="UserCreateDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_create_date: Option<f64>,
    #[doc="<p>The last modified date of the user.</p>"]
    #[serde(rename="UserLastModifiedDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_last_modified_date: Option<f64>,
    #[doc="<p>The user status. Can be one of the following:</p> <ul> <li> <p>UNCONFIRMED - User has been created but not confirmed.</p> </li> <li> <p>CONFIRMED - User has been confirmed.</p> </li> <li> <p>ARCHIVED - User is no longer active.</p> </li> <li> <p>COMPROMISED - User is disabled due to a potential security threat.</p> </li> <li> <p>UNKNOWN - User status is not known.</p> </li> </ul>"]
    #[serde(rename="UserStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_status: Option<String>,
    #[doc="<p>The user name of the user you wish to describe.</p>"]
    #[serde(rename="Username")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub username: Option<String>,
}


#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum VerifiedAttributeType {
    Email,
    PhoneNumber,
}

impl Into<String> for VerifiedAttributeType {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for VerifiedAttributeType {
    fn into(self) -> &'static str {
        match self {
            VerifiedAttributeType::Email => "email",
            VerifiedAttributeType::PhoneNumber => "phone_number",
        }
    }
}

impl ::std::str::FromStr for VerifiedAttributeType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "email" => Ok(VerifiedAttributeType::Email),
            "phone_number" => Ok(VerifiedAttributeType::PhoneNumber),
            _ => Err(()),
        }
    }
}

#[doc="<p>Represents the request to verify user attributes.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct VerifyUserAttributeRequest {
    #[doc="<p>Represents the access token of the request to verify user attributes.</p>"]
    #[serde(rename="AccessToken")]
    pub access_token: String,
    #[doc="<p>The attribute name in the request to verify user attributes.</p>"]
    #[serde(rename="AttributeName")]
    pub attribute_name: String,
    #[doc="<p>The verification code in the request to verify user attributes.</p>"]
    #[serde(rename="Code")]
    pub code: String,
}

#[doc="<p>A container representing the response from the server from the request to verify user attributes.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct VerifyUserAttributeResponse;

/// Errors returned by AddCustomAttributes
#[derive(Debug, PartialEq)]
pub enum AddCustomAttributesError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    ///<p>This exception is thrown when you are trying to modify a user pool while a user import job is in progress for that pool.</p>
    UserImportInProgress(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AddCustomAttributesError {
    pub fn from_body(body: &str) -> AddCustomAttributesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        AddCustomAttributesError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        AddCustomAttributesError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        AddCustomAttributesError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AddCustomAttributesError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        AddCustomAttributesError::TooManyRequests(String::from(error_message))
                    }
                    "UserImportInProgressException" => {
                        AddCustomAttributesError::UserImportInProgress(String::from(error_message))
                    }
                    "ValidationException" => {
                        AddCustomAttributesError::Validation(error_message.to_string())
                    }
                    _ => AddCustomAttributesError::Unknown(String::from(body)),
                }
            }
            Err(_) => AddCustomAttributesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AddCustomAttributesError {
    fn from(err: serde_json::error::Error) -> AddCustomAttributesError {
        AddCustomAttributesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AddCustomAttributesError {
    fn from(err: CredentialsError) -> AddCustomAttributesError {
        AddCustomAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AddCustomAttributesError {
    fn from(err: HttpDispatchError) -> AddCustomAttributesError {
        AddCustomAttributesError::HttpDispatch(err)
    }
}
impl fmt::Display for AddCustomAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddCustomAttributesError {
    fn description(&self) -> &str {
        match *self {
            AddCustomAttributesError::InternalError(ref cause) => cause,
            AddCustomAttributesError::InvalidParameter(ref cause) => cause,
            AddCustomAttributesError::NotAuthorized(ref cause) => cause,
            AddCustomAttributesError::ResourceNotFound(ref cause) => cause,
            AddCustomAttributesError::TooManyRequests(ref cause) => cause,
            AddCustomAttributesError::UserImportInProgress(ref cause) => cause,
            AddCustomAttributesError::Validation(ref cause) => cause,
            AddCustomAttributesError::Credentials(ref err) => err.description(),
            AddCustomAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AddCustomAttributesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AdminAddUserToGroup
#[derive(Debug, PartialEq)]
pub enum AdminAddUserToGroupError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    ///<p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AdminAddUserToGroupError {
    pub fn from_body(body: &str) -> AdminAddUserToGroupError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        AdminAddUserToGroupError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        AdminAddUserToGroupError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        AdminAddUserToGroupError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AdminAddUserToGroupError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        AdminAddUserToGroupError::TooManyRequests(String::from(error_message))
                    }
                    "UserNotFoundException" => {
                        AdminAddUserToGroupError::UserNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        AdminAddUserToGroupError::Validation(error_message.to_string())
                    }
                    _ => AdminAddUserToGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => AdminAddUserToGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AdminAddUserToGroupError {
    fn from(err: serde_json::error::Error) -> AdminAddUserToGroupError {
        AdminAddUserToGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AdminAddUserToGroupError {
    fn from(err: CredentialsError) -> AdminAddUserToGroupError {
        AdminAddUserToGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AdminAddUserToGroupError {
    fn from(err: HttpDispatchError) -> AdminAddUserToGroupError {
        AdminAddUserToGroupError::HttpDispatch(err)
    }
}
impl fmt::Display for AdminAddUserToGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AdminAddUserToGroupError {
    fn description(&self) -> &str {
        match *self {
            AdminAddUserToGroupError::InternalError(ref cause) => cause,
            AdminAddUserToGroupError::InvalidParameter(ref cause) => cause,
            AdminAddUserToGroupError::NotAuthorized(ref cause) => cause,
            AdminAddUserToGroupError::ResourceNotFound(ref cause) => cause,
            AdminAddUserToGroupError::TooManyRequests(ref cause) => cause,
            AdminAddUserToGroupError::UserNotFound(ref cause) => cause,
            AdminAddUserToGroupError::Validation(ref cause) => cause,
            AdminAddUserToGroupError::Credentials(ref err) => err.description(),
            AdminAddUserToGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AdminAddUserToGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AdminConfirmSignUp
#[derive(Debug, PartialEq)]
pub enum AdminConfirmSignUpError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid AWS Lambda response.</p>
    InvalidLambdaResponse(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user exceeds the limit for a requested AWS resource.</p>
    LimitExceeded(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many failed attempts for a given action (e.g., sign in).</p>
    TooManyFailedAttempts(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an unexpected exception with the AWS Lambda service.</p>
    UnexpectedLambda(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters a user validation exception with the AWS Lambda service.</p>
    UserLambdaValidation(String),
    ///<p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AdminConfirmSignUpError {
    pub fn from_body(body: &str) -> AdminConfirmSignUpError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        AdminConfirmSignUpError::InternalError(String::from(error_message))
                    }
                    "InvalidLambdaResponseException" => {
                        AdminConfirmSignUpError::InvalidLambdaResponse(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        AdminConfirmSignUpError::InvalidParameter(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        AdminConfirmSignUpError::LimitExceeded(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        AdminConfirmSignUpError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AdminConfirmSignUpError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyFailedAttemptsException" => {
                        AdminConfirmSignUpError::TooManyFailedAttempts(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        AdminConfirmSignUpError::TooManyRequests(String::from(error_message))
                    }
                    "UnexpectedLambdaException" => {
                        AdminConfirmSignUpError::UnexpectedLambda(String::from(error_message))
                    }
                    "UserLambdaValidationException" => {
                        AdminConfirmSignUpError::UserLambdaValidation(String::from(error_message))
                    }
                    "UserNotFoundException" => {
                        AdminConfirmSignUpError::UserNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        AdminConfirmSignUpError::Validation(error_message.to_string())
                    }
                    _ => AdminConfirmSignUpError::Unknown(String::from(body)),
                }
            }
            Err(_) => AdminConfirmSignUpError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AdminConfirmSignUpError {
    fn from(err: serde_json::error::Error) -> AdminConfirmSignUpError {
        AdminConfirmSignUpError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AdminConfirmSignUpError {
    fn from(err: CredentialsError) -> AdminConfirmSignUpError {
        AdminConfirmSignUpError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AdminConfirmSignUpError {
    fn from(err: HttpDispatchError) -> AdminConfirmSignUpError {
        AdminConfirmSignUpError::HttpDispatch(err)
    }
}
impl fmt::Display for AdminConfirmSignUpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AdminConfirmSignUpError {
    fn description(&self) -> &str {
        match *self {
            AdminConfirmSignUpError::InternalError(ref cause) => cause,
            AdminConfirmSignUpError::InvalidLambdaResponse(ref cause) => cause,
            AdminConfirmSignUpError::InvalidParameter(ref cause) => cause,
            AdminConfirmSignUpError::LimitExceeded(ref cause) => cause,
            AdminConfirmSignUpError::NotAuthorized(ref cause) => cause,
            AdminConfirmSignUpError::ResourceNotFound(ref cause) => cause,
            AdminConfirmSignUpError::TooManyFailedAttempts(ref cause) => cause,
            AdminConfirmSignUpError::TooManyRequests(ref cause) => cause,
            AdminConfirmSignUpError::UnexpectedLambda(ref cause) => cause,
            AdminConfirmSignUpError::UserLambdaValidation(ref cause) => cause,
            AdminConfirmSignUpError::UserNotFound(ref cause) => cause,
            AdminConfirmSignUpError::Validation(ref cause) => cause,
            AdminConfirmSignUpError::Credentials(ref err) => err.description(),
            AdminConfirmSignUpError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AdminConfirmSignUpError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AdminCreateUser
#[derive(Debug, PartialEq)]
pub enum AdminCreateUserError {
    ///<p>This exception is thrown when a verification code fails to deliver successfully.</p>
    CodeDeliveryFailure(String),
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid AWS Lambda response.</p>
    InvalidLambdaResponse(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid password.</p>
    InvalidPassword(String),
    ///<p>This exception is returned when the role provided for SMS configuration does not have permission to publish using Amazon SNS.</p>
    InvalidSmsRoleAccessPolicy(String),
    ///<p>This exception is thrown when the trust relationship is invalid for the role provided for SMS configuration. This can happen if you do not trust <b>cognito-idp.amazonaws.com</b> or the external ID provided in the role does not match what is provided in the SMS configuration for the user pool.</p>
    InvalidSmsRoleTrustRelationship(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when a precondition is not met.</p>
    PreconditionNotMet(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an unexpected exception with the AWS Lambda service.</p>
    UnexpectedLambda(String),
    ///<p>The request failed because the user is in an unsupported state.</p>
    UnsupportedUserState(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters a user validation exception with the AWS Lambda service.</p>
    UserLambdaValidation(String),
    ///<p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
    ///<p>This exception is thrown when Amazon Cognito encounters a user name that already exists in the user pool.</p>
    UsernameExists(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AdminCreateUserError {
    pub fn from_body(body: &str) -> AdminCreateUserError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "CodeDeliveryFailureException" => {
                        AdminCreateUserError::CodeDeliveryFailure(String::from(error_message))
                    }
                    "InternalErrorException" => {
                        AdminCreateUserError::InternalError(String::from(error_message))
                    }
                    "InvalidLambdaResponseException" => {
                        AdminCreateUserError::InvalidLambdaResponse(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        AdminCreateUserError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidPasswordException" => {
                        AdminCreateUserError::InvalidPassword(String::from(error_message))
                    }
                    "InvalidSmsRoleAccessPolicyException" => AdminCreateUserError::InvalidSmsRoleAccessPolicy(String::from(error_message)),
                    "InvalidSmsRoleTrustRelationshipException" => AdminCreateUserError::InvalidSmsRoleTrustRelationship(String::from(error_message)),
                    "NotAuthorizedException" => {
                        AdminCreateUserError::NotAuthorized(String::from(error_message))
                    }
                    "PreconditionNotMetException" => {
                        AdminCreateUserError::PreconditionNotMet(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AdminCreateUserError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        AdminCreateUserError::TooManyRequests(String::from(error_message))
                    }
                    "UnexpectedLambdaException" => {
                        AdminCreateUserError::UnexpectedLambda(String::from(error_message))
                    }
                    "UnsupportedUserStateException" => {
                        AdminCreateUserError::UnsupportedUserState(String::from(error_message))
                    }
                    "UserLambdaValidationException" => {
                        AdminCreateUserError::UserLambdaValidation(String::from(error_message))
                    }
                    "UserNotFoundException" => {
                        AdminCreateUserError::UserNotFound(String::from(error_message))
                    }
                    "UsernameExistsException" => {
                        AdminCreateUserError::UsernameExists(String::from(error_message))
                    }
                    "ValidationException" => {
                        AdminCreateUserError::Validation(error_message.to_string())
                    }
                    _ => AdminCreateUserError::Unknown(String::from(body)),
                }
            }
            Err(_) => AdminCreateUserError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AdminCreateUserError {
    fn from(err: serde_json::error::Error) -> AdminCreateUserError {
        AdminCreateUserError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AdminCreateUserError {
    fn from(err: CredentialsError) -> AdminCreateUserError {
        AdminCreateUserError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AdminCreateUserError {
    fn from(err: HttpDispatchError) -> AdminCreateUserError {
        AdminCreateUserError::HttpDispatch(err)
    }
}
impl fmt::Display for AdminCreateUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AdminCreateUserError {
    fn description(&self) -> &str {
        match *self {
            AdminCreateUserError::CodeDeliveryFailure(ref cause) => cause,
            AdminCreateUserError::InternalError(ref cause) => cause,
            AdminCreateUserError::InvalidLambdaResponse(ref cause) => cause,
            AdminCreateUserError::InvalidParameter(ref cause) => cause,
            AdminCreateUserError::InvalidPassword(ref cause) => cause,
            AdminCreateUserError::InvalidSmsRoleAccessPolicy(ref cause) => cause,
            AdminCreateUserError::InvalidSmsRoleTrustRelationship(ref cause) => cause,
            AdminCreateUserError::NotAuthorized(ref cause) => cause,
            AdminCreateUserError::PreconditionNotMet(ref cause) => cause,
            AdminCreateUserError::ResourceNotFound(ref cause) => cause,
            AdminCreateUserError::TooManyRequests(ref cause) => cause,
            AdminCreateUserError::UnexpectedLambda(ref cause) => cause,
            AdminCreateUserError::UnsupportedUserState(ref cause) => cause,
            AdminCreateUserError::UserLambdaValidation(ref cause) => cause,
            AdminCreateUserError::UserNotFound(ref cause) => cause,
            AdminCreateUserError::UsernameExists(ref cause) => cause,
            AdminCreateUserError::Validation(ref cause) => cause,
            AdminCreateUserError::Credentials(ref err) => err.description(),
            AdminCreateUserError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AdminCreateUserError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AdminDeleteUser
#[derive(Debug, PartialEq)]
pub enum AdminDeleteUserError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    ///<p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AdminDeleteUserError {
    pub fn from_body(body: &str) -> AdminDeleteUserError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        AdminDeleteUserError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        AdminDeleteUserError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        AdminDeleteUserError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AdminDeleteUserError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        AdminDeleteUserError::TooManyRequests(String::from(error_message))
                    }
                    "UserNotFoundException" => {
                        AdminDeleteUserError::UserNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        AdminDeleteUserError::Validation(error_message.to_string())
                    }
                    _ => AdminDeleteUserError::Unknown(String::from(body)),
                }
            }
            Err(_) => AdminDeleteUserError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AdminDeleteUserError {
    fn from(err: serde_json::error::Error) -> AdminDeleteUserError {
        AdminDeleteUserError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AdminDeleteUserError {
    fn from(err: CredentialsError) -> AdminDeleteUserError {
        AdminDeleteUserError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AdminDeleteUserError {
    fn from(err: HttpDispatchError) -> AdminDeleteUserError {
        AdminDeleteUserError::HttpDispatch(err)
    }
}
impl fmt::Display for AdminDeleteUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AdminDeleteUserError {
    fn description(&self) -> &str {
        match *self {
            AdminDeleteUserError::InternalError(ref cause) => cause,
            AdminDeleteUserError::InvalidParameter(ref cause) => cause,
            AdminDeleteUserError::NotAuthorized(ref cause) => cause,
            AdminDeleteUserError::ResourceNotFound(ref cause) => cause,
            AdminDeleteUserError::TooManyRequests(ref cause) => cause,
            AdminDeleteUserError::UserNotFound(ref cause) => cause,
            AdminDeleteUserError::Validation(ref cause) => cause,
            AdminDeleteUserError::Credentials(ref err) => err.description(),
            AdminDeleteUserError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AdminDeleteUserError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AdminDeleteUserAttributes
#[derive(Debug, PartialEq)]
pub enum AdminDeleteUserAttributesError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    ///<p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AdminDeleteUserAttributesError {
    pub fn from_body(body: &str) -> AdminDeleteUserAttributesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        AdminDeleteUserAttributesError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => AdminDeleteUserAttributesError::InvalidParameter(String::from(error_message)),
                    "NotAuthorizedException" => {
                        AdminDeleteUserAttributesError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => AdminDeleteUserAttributesError::ResourceNotFound(String::from(error_message)),
                    "TooManyRequestsException" => {
                        AdminDeleteUserAttributesError::TooManyRequests(String::from(error_message))
                    }
                    "UserNotFoundException" => {
                        AdminDeleteUserAttributesError::UserNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        AdminDeleteUserAttributesError::Validation(error_message.to_string())
                    }
                    _ => AdminDeleteUserAttributesError::Unknown(String::from(body)),
                }
            }
            Err(_) => AdminDeleteUserAttributesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AdminDeleteUserAttributesError {
    fn from(err: serde_json::error::Error) -> AdminDeleteUserAttributesError {
        AdminDeleteUserAttributesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AdminDeleteUserAttributesError {
    fn from(err: CredentialsError) -> AdminDeleteUserAttributesError {
        AdminDeleteUserAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AdminDeleteUserAttributesError {
    fn from(err: HttpDispatchError) -> AdminDeleteUserAttributesError {
        AdminDeleteUserAttributesError::HttpDispatch(err)
    }
}
impl fmt::Display for AdminDeleteUserAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AdminDeleteUserAttributesError {
    fn description(&self) -> &str {
        match *self {
            AdminDeleteUserAttributesError::InternalError(ref cause) => cause,
            AdminDeleteUserAttributesError::InvalidParameter(ref cause) => cause,
            AdminDeleteUserAttributesError::NotAuthorized(ref cause) => cause,
            AdminDeleteUserAttributesError::ResourceNotFound(ref cause) => cause,
            AdminDeleteUserAttributesError::TooManyRequests(ref cause) => cause,
            AdminDeleteUserAttributesError::UserNotFound(ref cause) => cause,
            AdminDeleteUserAttributesError::Validation(ref cause) => cause,
            AdminDeleteUserAttributesError::Credentials(ref err) => err.description(),
            AdminDeleteUserAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AdminDeleteUserAttributesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AdminDisableUser
#[derive(Debug, PartialEq)]
pub enum AdminDisableUserError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    ///<p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AdminDisableUserError {
    pub fn from_body(body: &str) -> AdminDisableUserError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        AdminDisableUserError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        AdminDisableUserError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        AdminDisableUserError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AdminDisableUserError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        AdminDisableUserError::TooManyRequests(String::from(error_message))
                    }
                    "UserNotFoundException" => {
                        AdminDisableUserError::UserNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        AdminDisableUserError::Validation(error_message.to_string())
                    }
                    _ => AdminDisableUserError::Unknown(String::from(body)),
                }
            }
            Err(_) => AdminDisableUserError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AdminDisableUserError {
    fn from(err: serde_json::error::Error) -> AdminDisableUserError {
        AdminDisableUserError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AdminDisableUserError {
    fn from(err: CredentialsError) -> AdminDisableUserError {
        AdminDisableUserError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AdminDisableUserError {
    fn from(err: HttpDispatchError) -> AdminDisableUserError {
        AdminDisableUserError::HttpDispatch(err)
    }
}
impl fmt::Display for AdminDisableUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AdminDisableUserError {
    fn description(&self) -> &str {
        match *self {
            AdminDisableUserError::InternalError(ref cause) => cause,
            AdminDisableUserError::InvalidParameter(ref cause) => cause,
            AdminDisableUserError::NotAuthorized(ref cause) => cause,
            AdminDisableUserError::ResourceNotFound(ref cause) => cause,
            AdminDisableUserError::TooManyRequests(ref cause) => cause,
            AdminDisableUserError::UserNotFound(ref cause) => cause,
            AdminDisableUserError::Validation(ref cause) => cause,
            AdminDisableUserError::Credentials(ref err) => err.description(),
            AdminDisableUserError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AdminDisableUserError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AdminEnableUser
#[derive(Debug, PartialEq)]
pub enum AdminEnableUserError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    ///<p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AdminEnableUserError {
    pub fn from_body(body: &str) -> AdminEnableUserError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        AdminEnableUserError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        AdminEnableUserError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        AdminEnableUserError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AdminEnableUserError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        AdminEnableUserError::TooManyRequests(String::from(error_message))
                    }
                    "UserNotFoundException" => {
                        AdminEnableUserError::UserNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        AdminEnableUserError::Validation(error_message.to_string())
                    }
                    _ => AdminEnableUserError::Unknown(String::from(body)),
                }
            }
            Err(_) => AdminEnableUserError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AdminEnableUserError {
    fn from(err: serde_json::error::Error) -> AdminEnableUserError {
        AdminEnableUserError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AdminEnableUserError {
    fn from(err: CredentialsError) -> AdminEnableUserError {
        AdminEnableUserError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AdminEnableUserError {
    fn from(err: HttpDispatchError) -> AdminEnableUserError {
        AdminEnableUserError::HttpDispatch(err)
    }
}
impl fmt::Display for AdminEnableUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AdminEnableUserError {
    fn description(&self) -> &str {
        match *self {
            AdminEnableUserError::InternalError(ref cause) => cause,
            AdminEnableUserError::InvalidParameter(ref cause) => cause,
            AdminEnableUserError::NotAuthorized(ref cause) => cause,
            AdminEnableUserError::ResourceNotFound(ref cause) => cause,
            AdminEnableUserError::TooManyRequests(ref cause) => cause,
            AdminEnableUserError::UserNotFound(ref cause) => cause,
            AdminEnableUserError::Validation(ref cause) => cause,
            AdminEnableUserError::Credentials(ref err) => err.description(),
            AdminEnableUserError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AdminEnableUserError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AdminForgetDevice
#[derive(Debug, PartialEq)]
pub enum AdminForgetDeviceError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when the user pool configuration is invalid.</p>
    InvalidUserPoolConfiguration(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    ///<p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AdminForgetDeviceError {
    pub fn from_body(body: &str) -> AdminForgetDeviceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        AdminForgetDeviceError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        AdminForgetDeviceError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidUserPoolConfigurationException" => AdminForgetDeviceError::InvalidUserPoolConfiguration(String::from(error_message)),
                    "NotAuthorizedException" => {
                        AdminForgetDeviceError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AdminForgetDeviceError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        AdminForgetDeviceError::TooManyRequests(String::from(error_message))
                    }
                    "UserNotFoundException" => {
                        AdminForgetDeviceError::UserNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        AdminForgetDeviceError::Validation(error_message.to_string())
                    }
                    _ => AdminForgetDeviceError::Unknown(String::from(body)),
                }
            }
            Err(_) => AdminForgetDeviceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AdminForgetDeviceError {
    fn from(err: serde_json::error::Error) -> AdminForgetDeviceError {
        AdminForgetDeviceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AdminForgetDeviceError {
    fn from(err: CredentialsError) -> AdminForgetDeviceError {
        AdminForgetDeviceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AdminForgetDeviceError {
    fn from(err: HttpDispatchError) -> AdminForgetDeviceError {
        AdminForgetDeviceError::HttpDispatch(err)
    }
}
impl fmt::Display for AdminForgetDeviceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AdminForgetDeviceError {
    fn description(&self) -> &str {
        match *self {
            AdminForgetDeviceError::InternalError(ref cause) => cause,
            AdminForgetDeviceError::InvalidParameter(ref cause) => cause,
            AdminForgetDeviceError::InvalidUserPoolConfiguration(ref cause) => cause,
            AdminForgetDeviceError::NotAuthorized(ref cause) => cause,
            AdminForgetDeviceError::ResourceNotFound(ref cause) => cause,
            AdminForgetDeviceError::TooManyRequests(ref cause) => cause,
            AdminForgetDeviceError::UserNotFound(ref cause) => cause,
            AdminForgetDeviceError::Validation(ref cause) => cause,
            AdminForgetDeviceError::Credentials(ref err) => err.description(),
            AdminForgetDeviceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AdminForgetDeviceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AdminGetDevice
#[derive(Debug, PartialEq)]
pub enum AdminGetDeviceError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when the user pool configuration is invalid.</p>
    InvalidUserPoolConfiguration(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AdminGetDeviceError {
    pub fn from_body(body: &str) -> AdminGetDeviceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        AdminGetDeviceError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        AdminGetDeviceError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidUserPoolConfigurationException" => AdminGetDeviceError::InvalidUserPoolConfiguration(String::from(error_message)),
                    "NotAuthorizedException" => {
                        AdminGetDeviceError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AdminGetDeviceError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        AdminGetDeviceError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        AdminGetDeviceError::Validation(error_message.to_string())
                    }
                    _ => AdminGetDeviceError::Unknown(String::from(body)),
                }
            }
            Err(_) => AdminGetDeviceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AdminGetDeviceError {
    fn from(err: serde_json::error::Error) -> AdminGetDeviceError {
        AdminGetDeviceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AdminGetDeviceError {
    fn from(err: CredentialsError) -> AdminGetDeviceError {
        AdminGetDeviceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AdminGetDeviceError {
    fn from(err: HttpDispatchError) -> AdminGetDeviceError {
        AdminGetDeviceError::HttpDispatch(err)
    }
}
impl fmt::Display for AdminGetDeviceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AdminGetDeviceError {
    fn description(&self) -> &str {
        match *self {
            AdminGetDeviceError::InternalError(ref cause) => cause,
            AdminGetDeviceError::InvalidParameter(ref cause) => cause,
            AdminGetDeviceError::InvalidUserPoolConfiguration(ref cause) => cause,
            AdminGetDeviceError::NotAuthorized(ref cause) => cause,
            AdminGetDeviceError::ResourceNotFound(ref cause) => cause,
            AdminGetDeviceError::TooManyRequests(ref cause) => cause,
            AdminGetDeviceError::Validation(ref cause) => cause,
            AdminGetDeviceError::Credentials(ref err) => err.description(),
            AdminGetDeviceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AdminGetDeviceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AdminGetUser
#[derive(Debug, PartialEq)]
pub enum AdminGetUserError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    ///<p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AdminGetUserError {
    pub fn from_body(body: &str) -> AdminGetUserError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        AdminGetUserError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        AdminGetUserError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        AdminGetUserError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AdminGetUserError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        AdminGetUserError::TooManyRequests(String::from(error_message))
                    }
                    "UserNotFoundException" => {
                        AdminGetUserError::UserNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        AdminGetUserError::Validation(error_message.to_string())
                    }
                    _ => AdminGetUserError::Unknown(String::from(body)),
                }
            }
            Err(_) => AdminGetUserError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AdminGetUserError {
    fn from(err: serde_json::error::Error) -> AdminGetUserError {
        AdminGetUserError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AdminGetUserError {
    fn from(err: CredentialsError) -> AdminGetUserError {
        AdminGetUserError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AdminGetUserError {
    fn from(err: HttpDispatchError) -> AdminGetUserError {
        AdminGetUserError::HttpDispatch(err)
    }
}
impl fmt::Display for AdminGetUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AdminGetUserError {
    fn description(&self) -> &str {
        match *self {
            AdminGetUserError::InternalError(ref cause) => cause,
            AdminGetUserError::InvalidParameter(ref cause) => cause,
            AdminGetUserError::NotAuthorized(ref cause) => cause,
            AdminGetUserError::ResourceNotFound(ref cause) => cause,
            AdminGetUserError::TooManyRequests(ref cause) => cause,
            AdminGetUserError::UserNotFound(ref cause) => cause,
            AdminGetUserError::Validation(ref cause) => cause,
            AdminGetUserError::Credentials(ref err) => err.description(),
            AdminGetUserError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AdminGetUserError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AdminInitiateAuth
#[derive(Debug, PartialEq)]
pub enum AdminInitiateAuthError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid AWS Lambda response.</p>
    InvalidLambdaResponse(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is returned when the role provided for SMS configuration does not have permission to publish using Amazon SNS.</p>
    InvalidSmsRoleAccessPolicy(String),
    ///<p>This exception is thrown when the trust relationship is invalid for the role provided for SMS configuration. This can happen if you do not trust <b>cognito-idp.amazonaws.com</b> or the external ID provided in the role does not match what is provided in the SMS configuration for the user pool.</p>
    InvalidSmsRoleTrustRelationship(String),
    ///<p>This exception is thrown when the user pool configuration is invalid.</p>
    InvalidUserPoolConfiguration(String),
    ///<p>This exception is thrown when Amazon Cognito cannot find a multi-factor authentication (MFA) method.</p>
    MFAMethodNotFound(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when a password reset is required.</p>
    PasswordResetRequired(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an unexpected exception with the AWS Lambda service.</p>
    UnexpectedLambda(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters a user validation exception with the AWS Lambda service.</p>
    UserLambdaValidation(String),
    ///<p>This exception is thrown when a user is not confirmed successfully.</p>
    UserNotConfirmed(String),
    ///<p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AdminInitiateAuthError {
    pub fn from_body(body: &str) -> AdminInitiateAuthError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        AdminInitiateAuthError::InternalError(String::from(error_message))
                    }
                    "InvalidLambdaResponseException" => {
                        AdminInitiateAuthError::InvalidLambdaResponse(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        AdminInitiateAuthError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidSmsRoleAccessPolicyException" => AdminInitiateAuthError::InvalidSmsRoleAccessPolicy(String::from(error_message)),
                    "InvalidSmsRoleTrustRelationshipException" => AdminInitiateAuthError::InvalidSmsRoleTrustRelationship(String::from(error_message)),
                    "InvalidUserPoolConfigurationException" => AdminInitiateAuthError::InvalidUserPoolConfiguration(String::from(error_message)),
                    "MFAMethodNotFoundException" => {
                        AdminInitiateAuthError::MFAMethodNotFound(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        AdminInitiateAuthError::NotAuthorized(String::from(error_message))
                    }
                    "PasswordResetRequiredException" => {
                        AdminInitiateAuthError::PasswordResetRequired(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AdminInitiateAuthError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        AdminInitiateAuthError::TooManyRequests(String::from(error_message))
                    }
                    "UnexpectedLambdaException" => {
                        AdminInitiateAuthError::UnexpectedLambda(String::from(error_message))
                    }
                    "UserLambdaValidationException" => {
                        AdminInitiateAuthError::UserLambdaValidation(String::from(error_message))
                    }
                    "UserNotConfirmedException" => {
                        AdminInitiateAuthError::UserNotConfirmed(String::from(error_message))
                    }
                    "UserNotFoundException" => {
                        AdminInitiateAuthError::UserNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        AdminInitiateAuthError::Validation(error_message.to_string())
                    }
                    _ => AdminInitiateAuthError::Unknown(String::from(body)),
                }
            }
            Err(_) => AdminInitiateAuthError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AdminInitiateAuthError {
    fn from(err: serde_json::error::Error) -> AdminInitiateAuthError {
        AdminInitiateAuthError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AdminInitiateAuthError {
    fn from(err: CredentialsError) -> AdminInitiateAuthError {
        AdminInitiateAuthError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AdminInitiateAuthError {
    fn from(err: HttpDispatchError) -> AdminInitiateAuthError {
        AdminInitiateAuthError::HttpDispatch(err)
    }
}
impl fmt::Display for AdminInitiateAuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AdminInitiateAuthError {
    fn description(&self) -> &str {
        match *self {
            AdminInitiateAuthError::InternalError(ref cause) => cause,
            AdminInitiateAuthError::InvalidLambdaResponse(ref cause) => cause,
            AdminInitiateAuthError::InvalidParameter(ref cause) => cause,
            AdminInitiateAuthError::InvalidSmsRoleAccessPolicy(ref cause) => cause,
            AdminInitiateAuthError::InvalidSmsRoleTrustRelationship(ref cause) => cause,
            AdminInitiateAuthError::InvalidUserPoolConfiguration(ref cause) => cause,
            AdminInitiateAuthError::MFAMethodNotFound(ref cause) => cause,
            AdminInitiateAuthError::NotAuthorized(ref cause) => cause,
            AdminInitiateAuthError::PasswordResetRequired(ref cause) => cause,
            AdminInitiateAuthError::ResourceNotFound(ref cause) => cause,
            AdminInitiateAuthError::TooManyRequests(ref cause) => cause,
            AdminInitiateAuthError::UnexpectedLambda(ref cause) => cause,
            AdminInitiateAuthError::UserLambdaValidation(ref cause) => cause,
            AdminInitiateAuthError::UserNotConfirmed(ref cause) => cause,
            AdminInitiateAuthError::UserNotFound(ref cause) => cause,
            AdminInitiateAuthError::Validation(ref cause) => cause,
            AdminInitiateAuthError::Credentials(ref err) => err.description(),
            AdminInitiateAuthError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AdminInitiateAuthError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AdminListDevices
#[derive(Debug, PartialEq)]
pub enum AdminListDevicesError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when the user pool configuration is invalid.</p>
    InvalidUserPoolConfiguration(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AdminListDevicesError {
    pub fn from_body(body: &str) -> AdminListDevicesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        AdminListDevicesError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        AdminListDevicesError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidUserPoolConfigurationException" => AdminListDevicesError::InvalidUserPoolConfiguration(String::from(error_message)),
                    "NotAuthorizedException" => {
                        AdminListDevicesError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AdminListDevicesError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        AdminListDevicesError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        AdminListDevicesError::Validation(error_message.to_string())
                    }
                    _ => AdminListDevicesError::Unknown(String::from(body)),
                }
            }
            Err(_) => AdminListDevicesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AdminListDevicesError {
    fn from(err: serde_json::error::Error) -> AdminListDevicesError {
        AdminListDevicesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AdminListDevicesError {
    fn from(err: CredentialsError) -> AdminListDevicesError {
        AdminListDevicesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AdminListDevicesError {
    fn from(err: HttpDispatchError) -> AdminListDevicesError {
        AdminListDevicesError::HttpDispatch(err)
    }
}
impl fmt::Display for AdminListDevicesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AdminListDevicesError {
    fn description(&self) -> &str {
        match *self {
            AdminListDevicesError::InternalError(ref cause) => cause,
            AdminListDevicesError::InvalidParameter(ref cause) => cause,
            AdminListDevicesError::InvalidUserPoolConfiguration(ref cause) => cause,
            AdminListDevicesError::NotAuthorized(ref cause) => cause,
            AdminListDevicesError::ResourceNotFound(ref cause) => cause,
            AdminListDevicesError::TooManyRequests(ref cause) => cause,
            AdminListDevicesError::Validation(ref cause) => cause,
            AdminListDevicesError::Credentials(ref err) => err.description(),
            AdminListDevicesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AdminListDevicesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AdminListGroupsForUser
#[derive(Debug, PartialEq)]
pub enum AdminListGroupsForUserError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    ///<p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AdminListGroupsForUserError {
    pub fn from_body(body: &str) -> AdminListGroupsForUserError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        AdminListGroupsForUserError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        AdminListGroupsForUserError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        AdminListGroupsForUserError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AdminListGroupsForUserError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        AdminListGroupsForUserError::TooManyRequests(String::from(error_message))
                    }
                    "UserNotFoundException" => {
                        AdminListGroupsForUserError::UserNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        AdminListGroupsForUserError::Validation(error_message.to_string())
                    }
                    _ => AdminListGroupsForUserError::Unknown(String::from(body)),
                }
            }
            Err(_) => AdminListGroupsForUserError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AdminListGroupsForUserError {
    fn from(err: serde_json::error::Error) -> AdminListGroupsForUserError {
        AdminListGroupsForUserError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AdminListGroupsForUserError {
    fn from(err: CredentialsError) -> AdminListGroupsForUserError {
        AdminListGroupsForUserError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AdminListGroupsForUserError {
    fn from(err: HttpDispatchError) -> AdminListGroupsForUserError {
        AdminListGroupsForUserError::HttpDispatch(err)
    }
}
impl fmt::Display for AdminListGroupsForUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AdminListGroupsForUserError {
    fn description(&self) -> &str {
        match *self {
            AdminListGroupsForUserError::InternalError(ref cause) => cause,
            AdminListGroupsForUserError::InvalidParameter(ref cause) => cause,
            AdminListGroupsForUserError::NotAuthorized(ref cause) => cause,
            AdminListGroupsForUserError::ResourceNotFound(ref cause) => cause,
            AdminListGroupsForUserError::TooManyRequests(ref cause) => cause,
            AdminListGroupsForUserError::UserNotFound(ref cause) => cause,
            AdminListGroupsForUserError::Validation(ref cause) => cause,
            AdminListGroupsForUserError::Credentials(ref err) => err.description(),
            AdminListGroupsForUserError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AdminListGroupsForUserError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AdminRemoveUserFromGroup
#[derive(Debug, PartialEq)]
pub enum AdminRemoveUserFromGroupError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    ///<p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AdminRemoveUserFromGroupError {
    pub fn from_body(body: &str) -> AdminRemoveUserFromGroupError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        AdminRemoveUserFromGroupError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        AdminRemoveUserFromGroupError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        AdminRemoveUserFromGroupError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AdminRemoveUserFromGroupError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        AdminRemoveUserFromGroupError::TooManyRequests(String::from(error_message))
                    }
                    "UserNotFoundException" => {
                        AdminRemoveUserFromGroupError::UserNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        AdminRemoveUserFromGroupError::Validation(error_message.to_string())
                    }
                    _ => AdminRemoveUserFromGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => AdminRemoveUserFromGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AdminRemoveUserFromGroupError {
    fn from(err: serde_json::error::Error) -> AdminRemoveUserFromGroupError {
        AdminRemoveUserFromGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AdminRemoveUserFromGroupError {
    fn from(err: CredentialsError) -> AdminRemoveUserFromGroupError {
        AdminRemoveUserFromGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AdminRemoveUserFromGroupError {
    fn from(err: HttpDispatchError) -> AdminRemoveUserFromGroupError {
        AdminRemoveUserFromGroupError::HttpDispatch(err)
    }
}
impl fmt::Display for AdminRemoveUserFromGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AdminRemoveUserFromGroupError {
    fn description(&self) -> &str {
        match *self {
            AdminRemoveUserFromGroupError::InternalError(ref cause) => cause,
            AdminRemoveUserFromGroupError::InvalidParameter(ref cause) => cause,
            AdminRemoveUserFromGroupError::NotAuthorized(ref cause) => cause,
            AdminRemoveUserFromGroupError::ResourceNotFound(ref cause) => cause,
            AdminRemoveUserFromGroupError::TooManyRequests(ref cause) => cause,
            AdminRemoveUserFromGroupError::UserNotFound(ref cause) => cause,
            AdminRemoveUserFromGroupError::Validation(ref cause) => cause,
            AdminRemoveUserFromGroupError::Credentials(ref err) => err.description(),
            AdminRemoveUserFromGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AdminRemoveUserFromGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AdminResetUserPassword
#[derive(Debug, PartialEq)]
pub enum AdminResetUserPasswordError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid AWS Lambda response.</p>
    InvalidLambdaResponse(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user exceeds the limit for a requested AWS resource.</p>
    LimitExceeded(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an unexpected exception with the AWS Lambda service.</p>
    UnexpectedLambda(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters a user validation exception with the AWS Lambda service.</p>
    UserLambdaValidation(String),
    ///<p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AdminResetUserPasswordError {
    pub fn from_body(body: &str) -> AdminResetUserPasswordError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        AdminResetUserPasswordError::InternalError(String::from(error_message))
                    }
                    "InvalidLambdaResponseException" => AdminResetUserPasswordError::InvalidLambdaResponse(String::from(error_message)),
                    "InvalidParameterException" => {
                        AdminResetUserPasswordError::InvalidParameter(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        AdminResetUserPasswordError::LimitExceeded(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        AdminResetUserPasswordError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AdminResetUserPasswordError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        AdminResetUserPasswordError::TooManyRequests(String::from(error_message))
                    }
                    "UnexpectedLambdaException" => {
                        AdminResetUserPasswordError::UnexpectedLambda(String::from(error_message))
                    }
                    "UserLambdaValidationException" => AdminResetUserPasswordError::UserLambdaValidation(String::from(error_message)),
                    "UserNotFoundException" => {
                        AdminResetUserPasswordError::UserNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        AdminResetUserPasswordError::Validation(error_message.to_string())
                    }
                    _ => AdminResetUserPasswordError::Unknown(String::from(body)),
                }
            }
            Err(_) => AdminResetUserPasswordError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AdminResetUserPasswordError {
    fn from(err: serde_json::error::Error) -> AdminResetUserPasswordError {
        AdminResetUserPasswordError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AdminResetUserPasswordError {
    fn from(err: CredentialsError) -> AdminResetUserPasswordError {
        AdminResetUserPasswordError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AdminResetUserPasswordError {
    fn from(err: HttpDispatchError) -> AdminResetUserPasswordError {
        AdminResetUserPasswordError::HttpDispatch(err)
    }
}
impl fmt::Display for AdminResetUserPasswordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AdminResetUserPasswordError {
    fn description(&self) -> &str {
        match *self {
            AdminResetUserPasswordError::InternalError(ref cause) => cause,
            AdminResetUserPasswordError::InvalidLambdaResponse(ref cause) => cause,
            AdminResetUserPasswordError::InvalidParameter(ref cause) => cause,
            AdminResetUserPasswordError::LimitExceeded(ref cause) => cause,
            AdminResetUserPasswordError::NotAuthorized(ref cause) => cause,
            AdminResetUserPasswordError::ResourceNotFound(ref cause) => cause,
            AdminResetUserPasswordError::TooManyRequests(ref cause) => cause,
            AdminResetUserPasswordError::UnexpectedLambda(ref cause) => cause,
            AdminResetUserPasswordError::UserLambdaValidation(ref cause) => cause,
            AdminResetUserPasswordError::UserNotFound(ref cause) => cause,
            AdminResetUserPasswordError::Validation(ref cause) => cause,
            AdminResetUserPasswordError::Credentials(ref err) => err.description(),
            AdminResetUserPasswordError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AdminResetUserPasswordError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AdminRespondToAuthChallenge
#[derive(Debug, PartialEq)]
pub enum AdminRespondToAuthChallengeError {
    ///<p>This exception is thrown when a user tries to confirm the account with an email or phone number that has already been supplied as an alias from a different account. This exception tells user that an account with this email or phone already exists.</p>
    AliasExists(String),
    ///<p>This exception is thrown if the provided code does not match what the server was expecting.</p>
    CodeMismatch(String),
    ///<p>This exception is thrown if a code has expired.</p>
    ExpiredCode(String),
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid AWS Lambda response.</p>
    InvalidLambdaResponse(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid password.</p>
    InvalidPassword(String),
    ///<p>This exception is returned when the role provided for SMS configuration does not have permission to publish using Amazon SNS.</p>
    InvalidSmsRoleAccessPolicy(String),
    ///<p>This exception is thrown when the trust relationship is invalid for the role provided for SMS configuration. This can happen if you do not trust <b>cognito-idp.amazonaws.com</b> or the external ID provided in the role does not match what is provided in the SMS configuration for the user pool.</p>
    InvalidSmsRoleTrustRelationship(String),
    ///<p>This exception is thrown when the user pool configuration is invalid.</p>
    InvalidUserPoolConfiguration(String),
    ///<p>This exception is thrown when Amazon Cognito cannot find a multi-factor authentication (MFA) method.</p>
    MFAMethodNotFound(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when a password reset is required.</p>
    PasswordResetRequired(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an unexpected exception with the AWS Lambda service.</p>
    UnexpectedLambda(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters a user validation exception with the AWS Lambda service.</p>
    UserLambdaValidation(String),
    ///<p>This exception is thrown when a user is not confirmed successfully.</p>
    UserNotConfirmed(String),
    ///<p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AdminRespondToAuthChallengeError {
    pub fn from_body(body: &str) -> AdminRespondToAuthChallengeError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AliasExistsException" => {
                        AdminRespondToAuthChallengeError::AliasExists(String::from(error_message))
                    }
                    "CodeMismatchException" => {
                        AdminRespondToAuthChallengeError::CodeMismatch(String::from(error_message))
                    }
                    "ExpiredCodeException" => {
                        AdminRespondToAuthChallengeError::ExpiredCode(String::from(error_message))
                    }
                    "InternalErrorException" => {
                        AdminRespondToAuthChallengeError::InternalError(String::from(error_message))
                    }
                    "InvalidLambdaResponseException" => AdminRespondToAuthChallengeError::InvalidLambdaResponse(String::from(error_message)),
                    "InvalidParameterException" => AdminRespondToAuthChallengeError::InvalidParameter(String::from(error_message)),
                    "InvalidPasswordException" => AdminRespondToAuthChallengeError::InvalidPassword(String::from(error_message)),
                    "InvalidSmsRoleAccessPolicyException" => AdminRespondToAuthChallengeError::InvalidSmsRoleAccessPolicy(String::from(error_message)),
                    "InvalidSmsRoleTrustRelationshipException" => AdminRespondToAuthChallengeError::InvalidSmsRoleTrustRelationship(String::from(error_message)),
                    "InvalidUserPoolConfigurationException" => AdminRespondToAuthChallengeError::InvalidUserPoolConfiguration(String::from(error_message)),
                    "MFAMethodNotFoundException" => AdminRespondToAuthChallengeError::MFAMethodNotFound(String::from(error_message)),
                    "NotAuthorizedException" => {
                        AdminRespondToAuthChallengeError::NotAuthorized(String::from(error_message))
                    }
                    "PasswordResetRequiredException" => AdminRespondToAuthChallengeError::PasswordResetRequired(String::from(error_message)),
                    "ResourceNotFoundException" => AdminRespondToAuthChallengeError::ResourceNotFound(String::from(error_message)),
                    "TooManyRequestsException" => AdminRespondToAuthChallengeError::TooManyRequests(String::from(error_message)),
                    "UnexpectedLambdaException" => AdminRespondToAuthChallengeError::UnexpectedLambda(String::from(error_message)),
                    "UserLambdaValidationException" => AdminRespondToAuthChallengeError::UserLambdaValidation(String::from(error_message)),
                    "UserNotConfirmedException" => AdminRespondToAuthChallengeError::UserNotConfirmed(String::from(error_message)),
                    "UserNotFoundException" => {
                        AdminRespondToAuthChallengeError::UserNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        AdminRespondToAuthChallengeError::Validation(error_message.to_string())
                    }
                    _ => AdminRespondToAuthChallengeError::Unknown(String::from(body)),
                }
            }
            Err(_) => AdminRespondToAuthChallengeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AdminRespondToAuthChallengeError {
    fn from(err: serde_json::error::Error) -> AdminRespondToAuthChallengeError {
        AdminRespondToAuthChallengeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AdminRespondToAuthChallengeError {
    fn from(err: CredentialsError) -> AdminRespondToAuthChallengeError {
        AdminRespondToAuthChallengeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AdminRespondToAuthChallengeError {
    fn from(err: HttpDispatchError) -> AdminRespondToAuthChallengeError {
        AdminRespondToAuthChallengeError::HttpDispatch(err)
    }
}
impl fmt::Display for AdminRespondToAuthChallengeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AdminRespondToAuthChallengeError {
    fn description(&self) -> &str {
        match *self {
            AdminRespondToAuthChallengeError::AliasExists(ref cause) => cause,
            AdminRespondToAuthChallengeError::CodeMismatch(ref cause) => cause,
            AdminRespondToAuthChallengeError::ExpiredCode(ref cause) => cause,
            AdminRespondToAuthChallengeError::InternalError(ref cause) => cause,
            AdminRespondToAuthChallengeError::InvalidLambdaResponse(ref cause) => cause,
            AdminRespondToAuthChallengeError::InvalidParameter(ref cause) => cause,
            AdminRespondToAuthChallengeError::InvalidPassword(ref cause) => cause,
            AdminRespondToAuthChallengeError::InvalidSmsRoleAccessPolicy(ref cause) => cause,
            AdminRespondToAuthChallengeError::InvalidSmsRoleTrustRelationship(ref cause) => cause,
            AdminRespondToAuthChallengeError::InvalidUserPoolConfiguration(ref cause) => cause,
            AdminRespondToAuthChallengeError::MFAMethodNotFound(ref cause) => cause,
            AdminRespondToAuthChallengeError::NotAuthorized(ref cause) => cause,
            AdminRespondToAuthChallengeError::PasswordResetRequired(ref cause) => cause,
            AdminRespondToAuthChallengeError::ResourceNotFound(ref cause) => cause,
            AdminRespondToAuthChallengeError::TooManyRequests(ref cause) => cause,
            AdminRespondToAuthChallengeError::UnexpectedLambda(ref cause) => cause,
            AdminRespondToAuthChallengeError::UserLambdaValidation(ref cause) => cause,
            AdminRespondToAuthChallengeError::UserNotConfirmed(ref cause) => cause,
            AdminRespondToAuthChallengeError::UserNotFound(ref cause) => cause,
            AdminRespondToAuthChallengeError::Validation(ref cause) => cause,
            AdminRespondToAuthChallengeError::Credentials(ref err) => err.description(),
            AdminRespondToAuthChallengeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AdminRespondToAuthChallengeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AdminSetUserSettings
#[derive(Debug, PartialEq)]
pub enum AdminSetUserSettingsError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AdminSetUserSettingsError {
    pub fn from_body(body: &str) -> AdminSetUserSettingsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        AdminSetUserSettingsError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        AdminSetUserSettingsError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        AdminSetUserSettingsError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AdminSetUserSettingsError::ResourceNotFound(String::from(error_message))
                    }
                    "UserNotFoundException" => {
                        AdminSetUserSettingsError::UserNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        AdminSetUserSettingsError::Validation(error_message.to_string())
                    }
                    _ => AdminSetUserSettingsError::Unknown(String::from(body)),
                }
            }
            Err(_) => AdminSetUserSettingsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AdminSetUserSettingsError {
    fn from(err: serde_json::error::Error) -> AdminSetUserSettingsError {
        AdminSetUserSettingsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AdminSetUserSettingsError {
    fn from(err: CredentialsError) -> AdminSetUserSettingsError {
        AdminSetUserSettingsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AdminSetUserSettingsError {
    fn from(err: HttpDispatchError) -> AdminSetUserSettingsError {
        AdminSetUserSettingsError::HttpDispatch(err)
    }
}
impl fmt::Display for AdminSetUserSettingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AdminSetUserSettingsError {
    fn description(&self) -> &str {
        match *self {
            AdminSetUserSettingsError::InternalError(ref cause) => cause,
            AdminSetUserSettingsError::InvalidParameter(ref cause) => cause,
            AdminSetUserSettingsError::NotAuthorized(ref cause) => cause,
            AdminSetUserSettingsError::ResourceNotFound(ref cause) => cause,
            AdminSetUserSettingsError::UserNotFound(ref cause) => cause,
            AdminSetUserSettingsError::Validation(ref cause) => cause,
            AdminSetUserSettingsError::Credentials(ref err) => err.description(),
            AdminSetUserSettingsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AdminSetUserSettingsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AdminUpdateDeviceStatus
#[derive(Debug, PartialEq)]
pub enum AdminUpdateDeviceStatusError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when the user pool configuration is invalid.</p>
    InvalidUserPoolConfiguration(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    ///<p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AdminUpdateDeviceStatusError {
    pub fn from_body(body: &str) -> AdminUpdateDeviceStatusError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        AdminUpdateDeviceStatusError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        AdminUpdateDeviceStatusError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidUserPoolConfigurationException" => AdminUpdateDeviceStatusError::InvalidUserPoolConfiguration(String::from(error_message)),
                    "NotAuthorizedException" => {
                        AdminUpdateDeviceStatusError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AdminUpdateDeviceStatusError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        AdminUpdateDeviceStatusError::TooManyRequests(String::from(error_message))
                    }
                    "UserNotFoundException" => {
                        AdminUpdateDeviceStatusError::UserNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        AdminUpdateDeviceStatusError::Validation(error_message.to_string())
                    }
                    _ => AdminUpdateDeviceStatusError::Unknown(String::from(body)),
                }
            }
            Err(_) => AdminUpdateDeviceStatusError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AdminUpdateDeviceStatusError {
    fn from(err: serde_json::error::Error) -> AdminUpdateDeviceStatusError {
        AdminUpdateDeviceStatusError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AdminUpdateDeviceStatusError {
    fn from(err: CredentialsError) -> AdminUpdateDeviceStatusError {
        AdminUpdateDeviceStatusError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AdminUpdateDeviceStatusError {
    fn from(err: HttpDispatchError) -> AdminUpdateDeviceStatusError {
        AdminUpdateDeviceStatusError::HttpDispatch(err)
    }
}
impl fmt::Display for AdminUpdateDeviceStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AdminUpdateDeviceStatusError {
    fn description(&self) -> &str {
        match *self {
            AdminUpdateDeviceStatusError::InternalError(ref cause) => cause,
            AdminUpdateDeviceStatusError::InvalidParameter(ref cause) => cause,
            AdminUpdateDeviceStatusError::InvalidUserPoolConfiguration(ref cause) => cause,
            AdminUpdateDeviceStatusError::NotAuthorized(ref cause) => cause,
            AdminUpdateDeviceStatusError::ResourceNotFound(ref cause) => cause,
            AdminUpdateDeviceStatusError::TooManyRequests(ref cause) => cause,
            AdminUpdateDeviceStatusError::UserNotFound(ref cause) => cause,
            AdminUpdateDeviceStatusError::Validation(ref cause) => cause,
            AdminUpdateDeviceStatusError::Credentials(ref err) => err.description(),
            AdminUpdateDeviceStatusError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AdminUpdateDeviceStatusError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AdminUpdateUserAttributes
#[derive(Debug, PartialEq)]
pub enum AdminUpdateUserAttributesError {
    ///<p>This exception is thrown when a user tries to confirm the account with an email or phone number that has already been supplied as an alias from a different account. This exception tells user that an account with this email or phone already exists.</p>
    AliasExists(String),
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid AWS Lambda response.</p>
    InvalidLambdaResponse(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an unexpected exception with the AWS Lambda service.</p>
    UnexpectedLambda(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters a user validation exception with the AWS Lambda service.</p>
    UserLambdaValidation(String),
    ///<p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AdminUpdateUserAttributesError {
    pub fn from_body(body: &str) -> AdminUpdateUserAttributesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AliasExistsException" => {
                        AdminUpdateUserAttributesError::AliasExists(String::from(error_message))
                    }
                    "InternalErrorException" => {
                        AdminUpdateUserAttributesError::InternalError(String::from(error_message))
                    }
                    "InvalidLambdaResponseException" => AdminUpdateUserAttributesError::InvalidLambdaResponse(String::from(error_message)),
                    "InvalidParameterException" => AdminUpdateUserAttributesError::InvalidParameter(String::from(error_message)),
                    "NotAuthorizedException" => {
                        AdminUpdateUserAttributesError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => AdminUpdateUserAttributesError::ResourceNotFound(String::from(error_message)),
                    "TooManyRequestsException" => {
                        AdminUpdateUserAttributesError::TooManyRequests(String::from(error_message))
                    }
                    "UnexpectedLambdaException" => AdminUpdateUserAttributesError::UnexpectedLambda(String::from(error_message)),
                    "UserLambdaValidationException" => AdminUpdateUserAttributesError::UserLambdaValidation(String::from(error_message)),
                    "UserNotFoundException" => {
                        AdminUpdateUserAttributesError::UserNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        AdminUpdateUserAttributesError::Validation(error_message.to_string())
                    }
                    _ => AdminUpdateUserAttributesError::Unknown(String::from(body)),
                }
            }
            Err(_) => AdminUpdateUserAttributesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AdminUpdateUserAttributesError {
    fn from(err: serde_json::error::Error) -> AdminUpdateUserAttributesError {
        AdminUpdateUserAttributesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AdminUpdateUserAttributesError {
    fn from(err: CredentialsError) -> AdminUpdateUserAttributesError {
        AdminUpdateUserAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AdminUpdateUserAttributesError {
    fn from(err: HttpDispatchError) -> AdminUpdateUserAttributesError {
        AdminUpdateUserAttributesError::HttpDispatch(err)
    }
}
impl fmt::Display for AdminUpdateUserAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AdminUpdateUserAttributesError {
    fn description(&self) -> &str {
        match *self {
            AdminUpdateUserAttributesError::AliasExists(ref cause) => cause,
            AdminUpdateUserAttributesError::InternalError(ref cause) => cause,
            AdminUpdateUserAttributesError::InvalidLambdaResponse(ref cause) => cause,
            AdminUpdateUserAttributesError::InvalidParameter(ref cause) => cause,
            AdminUpdateUserAttributesError::NotAuthorized(ref cause) => cause,
            AdminUpdateUserAttributesError::ResourceNotFound(ref cause) => cause,
            AdminUpdateUserAttributesError::TooManyRequests(ref cause) => cause,
            AdminUpdateUserAttributesError::UnexpectedLambda(ref cause) => cause,
            AdminUpdateUserAttributesError::UserLambdaValidation(ref cause) => cause,
            AdminUpdateUserAttributesError::UserNotFound(ref cause) => cause,
            AdminUpdateUserAttributesError::Validation(ref cause) => cause,
            AdminUpdateUserAttributesError::Credentials(ref err) => err.description(),
            AdminUpdateUserAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AdminUpdateUserAttributesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AdminUserGlobalSignOut
#[derive(Debug, PartialEq)]
pub enum AdminUserGlobalSignOutError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    ///<p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AdminUserGlobalSignOutError {
    pub fn from_body(body: &str) -> AdminUserGlobalSignOutError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        AdminUserGlobalSignOutError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        AdminUserGlobalSignOutError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        AdminUserGlobalSignOutError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AdminUserGlobalSignOutError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        AdminUserGlobalSignOutError::TooManyRequests(String::from(error_message))
                    }
                    "UserNotFoundException" => {
                        AdminUserGlobalSignOutError::UserNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        AdminUserGlobalSignOutError::Validation(error_message.to_string())
                    }
                    _ => AdminUserGlobalSignOutError::Unknown(String::from(body)),
                }
            }
            Err(_) => AdminUserGlobalSignOutError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AdminUserGlobalSignOutError {
    fn from(err: serde_json::error::Error) -> AdminUserGlobalSignOutError {
        AdminUserGlobalSignOutError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AdminUserGlobalSignOutError {
    fn from(err: CredentialsError) -> AdminUserGlobalSignOutError {
        AdminUserGlobalSignOutError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AdminUserGlobalSignOutError {
    fn from(err: HttpDispatchError) -> AdminUserGlobalSignOutError {
        AdminUserGlobalSignOutError::HttpDispatch(err)
    }
}
impl fmt::Display for AdminUserGlobalSignOutError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AdminUserGlobalSignOutError {
    fn description(&self) -> &str {
        match *self {
            AdminUserGlobalSignOutError::InternalError(ref cause) => cause,
            AdminUserGlobalSignOutError::InvalidParameter(ref cause) => cause,
            AdminUserGlobalSignOutError::NotAuthorized(ref cause) => cause,
            AdminUserGlobalSignOutError::ResourceNotFound(ref cause) => cause,
            AdminUserGlobalSignOutError::TooManyRequests(ref cause) => cause,
            AdminUserGlobalSignOutError::UserNotFound(ref cause) => cause,
            AdminUserGlobalSignOutError::Validation(ref cause) => cause,
            AdminUserGlobalSignOutError::Credentials(ref err) => err.description(),
            AdminUserGlobalSignOutError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AdminUserGlobalSignOutError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ChangePassword
#[derive(Debug, PartialEq)]
pub enum ChangePasswordError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid password.</p>
    InvalidPassword(String),
    ///<p>This exception is thrown when a user exceeds the limit for a requested AWS resource.</p>
    LimitExceeded(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when a password reset is required.</p>
    PasswordResetRequired(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    ///<p>This exception is thrown when a user is not confirmed successfully.</p>
    UserNotConfirmed(String),
    ///<p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ChangePasswordError {
    pub fn from_body(body: &str) -> ChangePasswordError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        ChangePasswordError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        ChangePasswordError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidPasswordException" => {
                        ChangePasswordError::InvalidPassword(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        ChangePasswordError::LimitExceeded(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        ChangePasswordError::NotAuthorized(String::from(error_message))
                    }
                    "PasswordResetRequiredException" => {
                        ChangePasswordError::PasswordResetRequired(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ChangePasswordError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        ChangePasswordError::TooManyRequests(String::from(error_message))
                    }
                    "UserNotConfirmedException" => {
                        ChangePasswordError::UserNotConfirmed(String::from(error_message))
                    }
                    "UserNotFoundException" => {
                        ChangePasswordError::UserNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        ChangePasswordError::Validation(error_message.to_string())
                    }
                    _ => ChangePasswordError::Unknown(String::from(body)),
                }
            }
            Err(_) => ChangePasswordError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ChangePasswordError {
    fn from(err: serde_json::error::Error) -> ChangePasswordError {
        ChangePasswordError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ChangePasswordError {
    fn from(err: CredentialsError) -> ChangePasswordError {
        ChangePasswordError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ChangePasswordError {
    fn from(err: HttpDispatchError) -> ChangePasswordError {
        ChangePasswordError::HttpDispatch(err)
    }
}
impl fmt::Display for ChangePasswordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ChangePasswordError {
    fn description(&self) -> &str {
        match *self {
            ChangePasswordError::InternalError(ref cause) => cause,
            ChangePasswordError::InvalidParameter(ref cause) => cause,
            ChangePasswordError::InvalidPassword(ref cause) => cause,
            ChangePasswordError::LimitExceeded(ref cause) => cause,
            ChangePasswordError::NotAuthorized(ref cause) => cause,
            ChangePasswordError::PasswordResetRequired(ref cause) => cause,
            ChangePasswordError::ResourceNotFound(ref cause) => cause,
            ChangePasswordError::TooManyRequests(ref cause) => cause,
            ChangePasswordError::UserNotConfirmed(ref cause) => cause,
            ChangePasswordError::UserNotFound(ref cause) => cause,
            ChangePasswordError::Validation(ref cause) => cause,
            ChangePasswordError::Credentials(ref err) => err.description(),
            ChangePasswordError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ChangePasswordError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ConfirmDevice
#[derive(Debug, PartialEq)]
pub enum ConfirmDeviceError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid AWS Lambda response.</p>
    InvalidLambdaResponse(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid password.</p>
    InvalidPassword(String),
    ///<p>This exception is thrown when the user pool configuration is invalid.</p>
    InvalidUserPoolConfiguration(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when a password reset is required.</p>
    PasswordResetRequired(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    ///<p>This exception is thrown when a user is not confirmed successfully.</p>
    UserNotConfirmed(String),
    ///<p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
    ///<p>This exception is thrown when Amazon Cognito encounters a user name that already exists in the user pool.</p>
    UsernameExists(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ConfirmDeviceError {
    pub fn from_body(body: &str) -> ConfirmDeviceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        ConfirmDeviceError::InternalError(String::from(error_message))
                    }
                    "InvalidLambdaResponseException" => {
                        ConfirmDeviceError::InvalidLambdaResponse(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        ConfirmDeviceError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidPasswordException" => {
                        ConfirmDeviceError::InvalidPassword(String::from(error_message))
                    }
                    "InvalidUserPoolConfigurationException" => ConfirmDeviceError::InvalidUserPoolConfiguration(String::from(error_message)),
                    "NotAuthorizedException" => {
                        ConfirmDeviceError::NotAuthorized(String::from(error_message))
                    }
                    "PasswordResetRequiredException" => {
                        ConfirmDeviceError::PasswordResetRequired(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ConfirmDeviceError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        ConfirmDeviceError::TooManyRequests(String::from(error_message))
                    }
                    "UserNotConfirmedException" => {
                        ConfirmDeviceError::UserNotConfirmed(String::from(error_message))
                    }
                    "UserNotFoundException" => {
                        ConfirmDeviceError::UserNotFound(String::from(error_message))
                    }
                    "UsernameExistsException" => {
                        ConfirmDeviceError::UsernameExists(String::from(error_message))
                    }
                    "ValidationException" => {
                        ConfirmDeviceError::Validation(error_message.to_string())
                    }
                    _ => ConfirmDeviceError::Unknown(String::from(body)),
                }
            }
            Err(_) => ConfirmDeviceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ConfirmDeviceError {
    fn from(err: serde_json::error::Error) -> ConfirmDeviceError {
        ConfirmDeviceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ConfirmDeviceError {
    fn from(err: CredentialsError) -> ConfirmDeviceError {
        ConfirmDeviceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ConfirmDeviceError {
    fn from(err: HttpDispatchError) -> ConfirmDeviceError {
        ConfirmDeviceError::HttpDispatch(err)
    }
}
impl fmt::Display for ConfirmDeviceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ConfirmDeviceError {
    fn description(&self) -> &str {
        match *self {
            ConfirmDeviceError::InternalError(ref cause) => cause,
            ConfirmDeviceError::InvalidLambdaResponse(ref cause) => cause,
            ConfirmDeviceError::InvalidParameter(ref cause) => cause,
            ConfirmDeviceError::InvalidPassword(ref cause) => cause,
            ConfirmDeviceError::InvalidUserPoolConfiguration(ref cause) => cause,
            ConfirmDeviceError::NotAuthorized(ref cause) => cause,
            ConfirmDeviceError::PasswordResetRequired(ref cause) => cause,
            ConfirmDeviceError::ResourceNotFound(ref cause) => cause,
            ConfirmDeviceError::TooManyRequests(ref cause) => cause,
            ConfirmDeviceError::UserNotConfirmed(ref cause) => cause,
            ConfirmDeviceError::UserNotFound(ref cause) => cause,
            ConfirmDeviceError::UsernameExists(ref cause) => cause,
            ConfirmDeviceError::Validation(ref cause) => cause,
            ConfirmDeviceError::Credentials(ref err) => err.description(),
            ConfirmDeviceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ConfirmDeviceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ConfirmForgotPassword
#[derive(Debug, PartialEq)]
pub enum ConfirmForgotPasswordError {
    ///<p>This exception is thrown if the provided code does not match what the server was expecting.</p>
    CodeMismatch(String),
    ///<p>This exception is thrown if a code has expired.</p>
    ExpiredCode(String),
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid AWS Lambda response.</p>
    InvalidLambdaResponse(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid password.</p>
    InvalidPassword(String),
    ///<p>This exception is thrown when a user exceeds the limit for a requested AWS resource.</p>
    LimitExceeded(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many failed attempts for a given action (e.g., sign in).</p>
    TooManyFailedAttempts(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an unexpected exception with the AWS Lambda service.</p>
    UnexpectedLambda(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters a user validation exception with the AWS Lambda service.</p>
    UserLambdaValidation(String),
    ///<p>This exception is thrown when a user is not confirmed successfully.</p>
    UserNotConfirmed(String),
    ///<p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ConfirmForgotPasswordError {
    pub fn from_body(body: &str) -> ConfirmForgotPasswordError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "CodeMismatchException" => {
                        ConfirmForgotPasswordError::CodeMismatch(String::from(error_message))
                    }
                    "ExpiredCodeException" => {
                        ConfirmForgotPasswordError::ExpiredCode(String::from(error_message))
                    }
                    "InternalErrorException" => {
                        ConfirmForgotPasswordError::InternalError(String::from(error_message))
                    }
                    "InvalidLambdaResponseException" => ConfirmForgotPasswordError::InvalidLambdaResponse(String::from(error_message)),
                    "InvalidParameterException" => {
                        ConfirmForgotPasswordError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidPasswordException" => {
                        ConfirmForgotPasswordError::InvalidPassword(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        ConfirmForgotPasswordError::LimitExceeded(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        ConfirmForgotPasswordError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ConfirmForgotPasswordError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyFailedAttemptsException" => ConfirmForgotPasswordError::TooManyFailedAttempts(String::from(error_message)),
                    "TooManyRequestsException" => {
                        ConfirmForgotPasswordError::TooManyRequests(String::from(error_message))
                    }
                    "UnexpectedLambdaException" => {
                        ConfirmForgotPasswordError::UnexpectedLambda(String::from(error_message))
                    }
                    "UserLambdaValidationException" => ConfirmForgotPasswordError::UserLambdaValidation(String::from(error_message)),
                    "UserNotConfirmedException" => {
                        ConfirmForgotPasswordError::UserNotConfirmed(String::from(error_message))
                    }
                    "UserNotFoundException" => {
                        ConfirmForgotPasswordError::UserNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        ConfirmForgotPasswordError::Validation(error_message.to_string())
                    }
                    _ => ConfirmForgotPasswordError::Unknown(String::from(body)),
                }
            }
            Err(_) => ConfirmForgotPasswordError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ConfirmForgotPasswordError {
    fn from(err: serde_json::error::Error) -> ConfirmForgotPasswordError {
        ConfirmForgotPasswordError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ConfirmForgotPasswordError {
    fn from(err: CredentialsError) -> ConfirmForgotPasswordError {
        ConfirmForgotPasswordError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ConfirmForgotPasswordError {
    fn from(err: HttpDispatchError) -> ConfirmForgotPasswordError {
        ConfirmForgotPasswordError::HttpDispatch(err)
    }
}
impl fmt::Display for ConfirmForgotPasswordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ConfirmForgotPasswordError {
    fn description(&self) -> &str {
        match *self {
            ConfirmForgotPasswordError::CodeMismatch(ref cause) => cause,
            ConfirmForgotPasswordError::ExpiredCode(ref cause) => cause,
            ConfirmForgotPasswordError::InternalError(ref cause) => cause,
            ConfirmForgotPasswordError::InvalidLambdaResponse(ref cause) => cause,
            ConfirmForgotPasswordError::InvalidParameter(ref cause) => cause,
            ConfirmForgotPasswordError::InvalidPassword(ref cause) => cause,
            ConfirmForgotPasswordError::LimitExceeded(ref cause) => cause,
            ConfirmForgotPasswordError::NotAuthorized(ref cause) => cause,
            ConfirmForgotPasswordError::ResourceNotFound(ref cause) => cause,
            ConfirmForgotPasswordError::TooManyFailedAttempts(ref cause) => cause,
            ConfirmForgotPasswordError::TooManyRequests(ref cause) => cause,
            ConfirmForgotPasswordError::UnexpectedLambda(ref cause) => cause,
            ConfirmForgotPasswordError::UserLambdaValidation(ref cause) => cause,
            ConfirmForgotPasswordError::UserNotConfirmed(ref cause) => cause,
            ConfirmForgotPasswordError::UserNotFound(ref cause) => cause,
            ConfirmForgotPasswordError::Validation(ref cause) => cause,
            ConfirmForgotPasswordError::Credentials(ref err) => err.description(),
            ConfirmForgotPasswordError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ConfirmForgotPasswordError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ConfirmSignUp
#[derive(Debug, PartialEq)]
pub enum ConfirmSignUpError {
    ///<p>This exception is thrown when a user tries to confirm the account with an email or phone number that has already been supplied as an alias from a different account. This exception tells user that an account with this email or phone already exists.</p>
    AliasExists(String),
    ///<p>This exception is thrown if the provided code does not match what the server was expecting.</p>
    CodeMismatch(String),
    ///<p>This exception is thrown if a code has expired.</p>
    ExpiredCode(String),
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid AWS Lambda response.</p>
    InvalidLambdaResponse(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user exceeds the limit for a requested AWS resource.</p>
    LimitExceeded(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many failed attempts for a given action (e.g., sign in).</p>
    TooManyFailedAttempts(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an unexpected exception with the AWS Lambda service.</p>
    UnexpectedLambda(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters a user validation exception with the AWS Lambda service.</p>
    UserLambdaValidation(String),
    ///<p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ConfirmSignUpError {
    pub fn from_body(body: &str) -> ConfirmSignUpError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AliasExistsException" => {
                        ConfirmSignUpError::AliasExists(String::from(error_message))
                    }
                    "CodeMismatchException" => {
                        ConfirmSignUpError::CodeMismatch(String::from(error_message))
                    }
                    "ExpiredCodeException" => {
                        ConfirmSignUpError::ExpiredCode(String::from(error_message))
                    }
                    "InternalErrorException" => {
                        ConfirmSignUpError::InternalError(String::from(error_message))
                    }
                    "InvalidLambdaResponseException" => {
                        ConfirmSignUpError::InvalidLambdaResponse(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        ConfirmSignUpError::InvalidParameter(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        ConfirmSignUpError::LimitExceeded(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        ConfirmSignUpError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ConfirmSignUpError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyFailedAttemptsException" => {
                        ConfirmSignUpError::TooManyFailedAttempts(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        ConfirmSignUpError::TooManyRequests(String::from(error_message))
                    }
                    "UnexpectedLambdaException" => {
                        ConfirmSignUpError::UnexpectedLambda(String::from(error_message))
                    }
                    "UserLambdaValidationException" => {
                        ConfirmSignUpError::UserLambdaValidation(String::from(error_message))
                    }
                    "UserNotFoundException" => {
                        ConfirmSignUpError::UserNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        ConfirmSignUpError::Validation(error_message.to_string())
                    }
                    _ => ConfirmSignUpError::Unknown(String::from(body)),
                }
            }
            Err(_) => ConfirmSignUpError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ConfirmSignUpError {
    fn from(err: serde_json::error::Error) -> ConfirmSignUpError {
        ConfirmSignUpError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ConfirmSignUpError {
    fn from(err: CredentialsError) -> ConfirmSignUpError {
        ConfirmSignUpError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ConfirmSignUpError {
    fn from(err: HttpDispatchError) -> ConfirmSignUpError {
        ConfirmSignUpError::HttpDispatch(err)
    }
}
impl fmt::Display for ConfirmSignUpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ConfirmSignUpError {
    fn description(&self) -> &str {
        match *self {
            ConfirmSignUpError::AliasExists(ref cause) => cause,
            ConfirmSignUpError::CodeMismatch(ref cause) => cause,
            ConfirmSignUpError::ExpiredCode(ref cause) => cause,
            ConfirmSignUpError::InternalError(ref cause) => cause,
            ConfirmSignUpError::InvalidLambdaResponse(ref cause) => cause,
            ConfirmSignUpError::InvalidParameter(ref cause) => cause,
            ConfirmSignUpError::LimitExceeded(ref cause) => cause,
            ConfirmSignUpError::NotAuthorized(ref cause) => cause,
            ConfirmSignUpError::ResourceNotFound(ref cause) => cause,
            ConfirmSignUpError::TooManyFailedAttempts(ref cause) => cause,
            ConfirmSignUpError::TooManyRequests(ref cause) => cause,
            ConfirmSignUpError::UnexpectedLambda(ref cause) => cause,
            ConfirmSignUpError::UserLambdaValidation(ref cause) => cause,
            ConfirmSignUpError::UserNotFound(ref cause) => cause,
            ConfirmSignUpError::Validation(ref cause) => cause,
            ConfirmSignUpError::Credentials(ref err) => err.description(),
            ConfirmSignUpError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ConfirmSignUpError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateGroup
#[derive(Debug, PartialEq)]
pub enum CreateGroupError {
    ///<p>This exception is thrown when Amazon Cognito encounters a group that already exists in the user pool.</p>
    GroupExists(String),
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user exceeds the limit for a requested AWS resource.</p>
    LimitExceeded(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateGroupError {
    pub fn from_body(body: &str) -> CreateGroupError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "GroupExistsException" => {
                        CreateGroupError::GroupExists(String::from(error_message))
                    }
                    "InternalErrorException" => {
                        CreateGroupError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        CreateGroupError::InvalidParameter(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateGroupError::LimitExceeded(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        CreateGroupError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        CreateGroupError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        CreateGroupError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateGroupError::Validation(error_message.to_string())
                    }
                    _ => CreateGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateGroupError {
    fn from(err: serde_json::error::Error) -> CreateGroupError {
        CreateGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateGroupError {
    fn from(err: CredentialsError) -> CreateGroupError {
        CreateGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateGroupError {
    fn from(err: HttpDispatchError) -> CreateGroupError {
        CreateGroupError::HttpDispatch(err)
    }
}
impl fmt::Display for CreateGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateGroupError {
    fn description(&self) -> &str {
        match *self {
            CreateGroupError::GroupExists(ref cause) => cause,
            CreateGroupError::InternalError(ref cause) => cause,
            CreateGroupError::InvalidParameter(ref cause) => cause,
            CreateGroupError::LimitExceeded(ref cause) => cause,
            CreateGroupError::NotAuthorized(ref cause) => cause,
            CreateGroupError::ResourceNotFound(ref cause) => cause,
            CreateGroupError::TooManyRequests(ref cause) => cause,
            CreateGroupError::Validation(ref cause) => cause,
            CreateGroupError::Credentials(ref err) => err.description(),
            CreateGroupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateIdentityProvider
#[derive(Debug, PartialEq)]
pub enum CreateIdentityProviderError {
    ///<p>This exception is thrown when the provider is already supported by the user pool.</p>
    DuplicateProvider(String),
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user exceeds the limit for a requested AWS resource.</p>
    LimitExceeded(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateIdentityProviderError {
    pub fn from_body(body: &str) -> CreateIdentityProviderError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DuplicateProviderException" => {
                        CreateIdentityProviderError::DuplicateProvider(String::from(error_message))
                    }
                    "InternalErrorException" => {
                        CreateIdentityProviderError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        CreateIdentityProviderError::InvalidParameter(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateIdentityProviderError::LimitExceeded(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        CreateIdentityProviderError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        CreateIdentityProviderError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        CreateIdentityProviderError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateIdentityProviderError::Validation(error_message.to_string())
                    }
                    _ => CreateIdentityProviderError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateIdentityProviderError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateIdentityProviderError {
    fn from(err: serde_json::error::Error) -> CreateIdentityProviderError {
        CreateIdentityProviderError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateIdentityProviderError {
    fn from(err: CredentialsError) -> CreateIdentityProviderError {
        CreateIdentityProviderError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateIdentityProviderError {
    fn from(err: HttpDispatchError) -> CreateIdentityProviderError {
        CreateIdentityProviderError::HttpDispatch(err)
    }
}
impl fmt::Display for CreateIdentityProviderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateIdentityProviderError {
    fn description(&self) -> &str {
        match *self {
            CreateIdentityProviderError::DuplicateProvider(ref cause) => cause,
            CreateIdentityProviderError::InternalError(ref cause) => cause,
            CreateIdentityProviderError::InvalidParameter(ref cause) => cause,
            CreateIdentityProviderError::LimitExceeded(ref cause) => cause,
            CreateIdentityProviderError::NotAuthorized(ref cause) => cause,
            CreateIdentityProviderError::ResourceNotFound(ref cause) => cause,
            CreateIdentityProviderError::TooManyRequests(ref cause) => cause,
            CreateIdentityProviderError::Validation(ref cause) => cause,
            CreateIdentityProviderError::Credentials(ref err) => err.description(),
            CreateIdentityProviderError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateIdentityProviderError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateUserImportJob
#[derive(Debug, PartialEq)]
pub enum CreateUserImportJobError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user exceeds the limit for a requested AWS resource.</p>
    LimitExceeded(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when a precondition is not met.</p>
    PreconditionNotMet(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateUserImportJobError {
    pub fn from_body(body: &str) -> CreateUserImportJobError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        CreateUserImportJobError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        CreateUserImportJobError::InvalidParameter(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateUserImportJobError::LimitExceeded(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        CreateUserImportJobError::NotAuthorized(String::from(error_message))
                    }
                    "PreconditionNotMetException" => {
                        CreateUserImportJobError::PreconditionNotMet(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        CreateUserImportJobError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        CreateUserImportJobError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateUserImportJobError::Validation(error_message.to_string())
                    }
                    _ => CreateUserImportJobError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateUserImportJobError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateUserImportJobError {
    fn from(err: serde_json::error::Error) -> CreateUserImportJobError {
        CreateUserImportJobError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateUserImportJobError {
    fn from(err: CredentialsError) -> CreateUserImportJobError {
        CreateUserImportJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateUserImportJobError {
    fn from(err: HttpDispatchError) -> CreateUserImportJobError {
        CreateUserImportJobError::HttpDispatch(err)
    }
}
impl fmt::Display for CreateUserImportJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateUserImportJobError {
    fn description(&self) -> &str {
        match *self {
            CreateUserImportJobError::InternalError(ref cause) => cause,
            CreateUserImportJobError::InvalidParameter(ref cause) => cause,
            CreateUserImportJobError::LimitExceeded(ref cause) => cause,
            CreateUserImportJobError::NotAuthorized(ref cause) => cause,
            CreateUserImportJobError::PreconditionNotMet(ref cause) => cause,
            CreateUserImportJobError::ResourceNotFound(ref cause) => cause,
            CreateUserImportJobError::TooManyRequests(ref cause) => cause,
            CreateUserImportJobError::Validation(ref cause) => cause,
            CreateUserImportJobError::Credentials(ref err) => err.description(),
            CreateUserImportJobError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateUserImportJobError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateUserPool
#[derive(Debug, PartialEq)]
pub enum CreateUserPoolError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when Amazon Cognito is not allowed to use your email identity. HTTP status code: 400.</p>
    InvalidEmailRoleAccessPolicy(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is returned when the role provided for SMS configuration does not have permission to publish using Amazon SNS.</p>
    InvalidSmsRoleAccessPolicy(String),
    ///<p>This exception is thrown when the trust relationship is invalid for the role provided for SMS configuration. This can happen if you do not trust <b>cognito-idp.amazonaws.com</b> or the external ID provided in the role does not match what is provided in the SMS configuration for the user pool.</p>
    InvalidSmsRoleTrustRelationship(String),
    ///<p>This exception is thrown when a user exceeds the limit for a requested AWS resource.</p>
    LimitExceeded(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    ///<p>This exception is thrown when a user pool tag cannot be set or updated.</p>
    UserPoolTagging(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateUserPoolError {
    pub fn from_body(body: &str) -> CreateUserPoolError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        CreateUserPoolError::InternalError(String::from(error_message))
                    }
                    "InvalidEmailRoleAccessPolicyException" => CreateUserPoolError::InvalidEmailRoleAccessPolicy(String::from(error_message)),
                    "InvalidParameterException" => {
                        CreateUserPoolError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidSmsRoleAccessPolicyException" => {
                        CreateUserPoolError::InvalidSmsRoleAccessPolicy(String::from(error_message))
                    }
                    "InvalidSmsRoleTrustRelationshipException" => CreateUserPoolError::InvalidSmsRoleTrustRelationship(String::from(error_message)),
                    "LimitExceededException" => {
                        CreateUserPoolError::LimitExceeded(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        CreateUserPoolError::NotAuthorized(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        CreateUserPoolError::TooManyRequests(String::from(error_message))
                    }
                    "UserPoolTaggingException" => {
                        CreateUserPoolError::UserPoolTagging(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateUserPoolError::Validation(error_message.to_string())
                    }
                    _ => CreateUserPoolError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateUserPoolError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateUserPoolError {
    fn from(err: serde_json::error::Error) -> CreateUserPoolError {
        CreateUserPoolError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateUserPoolError {
    fn from(err: CredentialsError) -> CreateUserPoolError {
        CreateUserPoolError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateUserPoolError {
    fn from(err: HttpDispatchError) -> CreateUserPoolError {
        CreateUserPoolError::HttpDispatch(err)
    }
}
impl fmt::Display for CreateUserPoolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateUserPoolError {
    fn description(&self) -> &str {
        match *self {
            CreateUserPoolError::InternalError(ref cause) => cause,
            CreateUserPoolError::InvalidEmailRoleAccessPolicy(ref cause) => cause,
            CreateUserPoolError::InvalidParameter(ref cause) => cause,
            CreateUserPoolError::InvalidSmsRoleAccessPolicy(ref cause) => cause,
            CreateUserPoolError::InvalidSmsRoleTrustRelationship(ref cause) => cause,
            CreateUserPoolError::LimitExceeded(ref cause) => cause,
            CreateUserPoolError::NotAuthorized(ref cause) => cause,
            CreateUserPoolError::TooManyRequests(ref cause) => cause,
            CreateUserPoolError::UserPoolTagging(ref cause) => cause,
            CreateUserPoolError::Validation(ref cause) => cause,
            CreateUserPoolError::Credentials(ref err) => err.description(),
            CreateUserPoolError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateUserPoolError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateUserPoolClient
#[derive(Debug, PartialEq)]
pub enum CreateUserPoolClientError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the specified OAuth flow is invalid.</p>
    InvalidOAuthFlow(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user exceeds the limit for a requested AWS resource.</p>
    LimitExceeded(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the specified scope does not exist.</p>
    ScopeDoesNotExist(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateUserPoolClientError {
    pub fn from_body(body: &str) -> CreateUserPoolClientError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        CreateUserPoolClientError::InternalError(String::from(error_message))
                    }
                    "InvalidOAuthFlowException" => {
                        CreateUserPoolClientError::InvalidOAuthFlow(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        CreateUserPoolClientError::InvalidParameter(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateUserPoolClientError::LimitExceeded(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        CreateUserPoolClientError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        CreateUserPoolClientError::ResourceNotFound(String::from(error_message))
                    }
                    "ScopeDoesNotExistException" => {
                        CreateUserPoolClientError::ScopeDoesNotExist(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        CreateUserPoolClientError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateUserPoolClientError::Validation(error_message.to_string())
                    }
                    _ => CreateUserPoolClientError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateUserPoolClientError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateUserPoolClientError {
    fn from(err: serde_json::error::Error) -> CreateUserPoolClientError {
        CreateUserPoolClientError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateUserPoolClientError {
    fn from(err: CredentialsError) -> CreateUserPoolClientError {
        CreateUserPoolClientError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateUserPoolClientError {
    fn from(err: HttpDispatchError) -> CreateUserPoolClientError {
        CreateUserPoolClientError::HttpDispatch(err)
    }
}
impl fmt::Display for CreateUserPoolClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateUserPoolClientError {
    fn description(&self) -> &str {
        match *self {
            CreateUserPoolClientError::InternalError(ref cause) => cause,
            CreateUserPoolClientError::InvalidOAuthFlow(ref cause) => cause,
            CreateUserPoolClientError::InvalidParameter(ref cause) => cause,
            CreateUserPoolClientError::LimitExceeded(ref cause) => cause,
            CreateUserPoolClientError::NotAuthorized(ref cause) => cause,
            CreateUserPoolClientError::ResourceNotFound(ref cause) => cause,
            CreateUserPoolClientError::ScopeDoesNotExist(ref cause) => cause,
            CreateUserPoolClientError::TooManyRequests(ref cause) => cause,
            CreateUserPoolClientError::Validation(ref cause) => cause,
            CreateUserPoolClientError::Credentials(ref err) => err.description(),
            CreateUserPoolClientError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateUserPoolClientError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateUserPoolDomain
#[derive(Debug, PartialEq)]
pub enum CreateUserPoolDomainError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateUserPoolDomainError {
    pub fn from_body(body: &str) -> CreateUserPoolDomainError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        CreateUserPoolDomainError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        CreateUserPoolDomainError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        CreateUserPoolDomainError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        CreateUserPoolDomainError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateUserPoolDomainError::Validation(error_message.to_string())
                    }
                    _ => CreateUserPoolDomainError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateUserPoolDomainError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateUserPoolDomainError {
    fn from(err: serde_json::error::Error) -> CreateUserPoolDomainError {
        CreateUserPoolDomainError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateUserPoolDomainError {
    fn from(err: CredentialsError) -> CreateUserPoolDomainError {
        CreateUserPoolDomainError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateUserPoolDomainError {
    fn from(err: HttpDispatchError) -> CreateUserPoolDomainError {
        CreateUserPoolDomainError::HttpDispatch(err)
    }
}
impl fmt::Display for CreateUserPoolDomainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateUserPoolDomainError {
    fn description(&self) -> &str {
        match *self {
            CreateUserPoolDomainError::InternalError(ref cause) => cause,
            CreateUserPoolDomainError::InvalidParameter(ref cause) => cause,
            CreateUserPoolDomainError::NotAuthorized(ref cause) => cause,
            CreateUserPoolDomainError::ResourceNotFound(ref cause) => cause,
            CreateUserPoolDomainError::Validation(ref cause) => cause,
            CreateUserPoolDomainError::Credentials(ref err) => err.description(),
            CreateUserPoolDomainError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateUserPoolDomainError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteGroup
#[derive(Debug, PartialEq)]
pub enum DeleteGroupError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteGroupError {
    pub fn from_body(body: &str) -> DeleteGroupError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        DeleteGroupError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DeleteGroupError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        DeleteGroupError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteGroupError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DeleteGroupError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteGroupError::Validation(error_message.to_string())
                    }
                    _ => DeleteGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteGroupError {
    fn from(err: serde_json::error::Error) -> DeleteGroupError {
        DeleteGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteGroupError {
    fn from(err: CredentialsError) -> DeleteGroupError {
        DeleteGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteGroupError {
    fn from(err: HttpDispatchError) -> DeleteGroupError {
        DeleteGroupError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteGroupError {
    fn description(&self) -> &str {
        match *self {
            DeleteGroupError::InternalError(ref cause) => cause,
            DeleteGroupError::InvalidParameter(ref cause) => cause,
            DeleteGroupError::NotAuthorized(ref cause) => cause,
            DeleteGroupError::ResourceNotFound(ref cause) => cause,
            DeleteGroupError::TooManyRequests(ref cause) => cause,
            DeleteGroupError::Validation(ref cause) => cause,
            DeleteGroupError::Credentials(ref err) => err.description(),
            DeleteGroupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteIdentityProvider
#[derive(Debug, PartialEq)]
pub enum DeleteIdentityProviderError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    ///<p>This exception is thrown when the specified identifier is not supported.</p>
    UnsupportedIdentityProvider(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteIdentityProviderError {
    pub fn from_body(body: &str) -> DeleteIdentityProviderError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        DeleteIdentityProviderError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DeleteIdentityProviderError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        DeleteIdentityProviderError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteIdentityProviderError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DeleteIdentityProviderError::TooManyRequests(String::from(error_message))
                    }
                    "UnsupportedIdentityProviderException" => DeleteIdentityProviderError::UnsupportedIdentityProvider(String::from(error_message)),
                    "ValidationException" => {
                        DeleteIdentityProviderError::Validation(error_message.to_string())
                    }
                    _ => DeleteIdentityProviderError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteIdentityProviderError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteIdentityProviderError {
    fn from(err: serde_json::error::Error) -> DeleteIdentityProviderError {
        DeleteIdentityProviderError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteIdentityProviderError {
    fn from(err: CredentialsError) -> DeleteIdentityProviderError {
        DeleteIdentityProviderError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteIdentityProviderError {
    fn from(err: HttpDispatchError) -> DeleteIdentityProviderError {
        DeleteIdentityProviderError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteIdentityProviderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteIdentityProviderError {
    fn description(&self) -> &str {
        match *self {
            DeleteIdentityProviderError::InternalError(ref cause) => cause,
            DeleteIdentityProviderError::InvalidParameter(ref cause) => cause,
            DeleteIdentityProviderError::NotAuthorized(ref cause) => cause,
            DeleteIdentityProviderError::ResourceNotFound(ref cause) => cause,
            DeleteIdentityProviderError::TooManyRequests(ref cause) => cause,
            DeleteIdentityProviderError::UnsupportedIdentityProvider(ref cause) => cause,
            DeleteIdentityProviderError::Validation(ref cause) => cause,
            DeleteIdentityProviderError::Credentials(ref err) => err.description(),
            DeleteIdentityProviderError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteIdentityProviderError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteUser
#[derive(Debug, PartialEq)]
pub enum DeleteUserError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when a password reset is required.</p>
    PasswordResetRequired(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    ///<p>This exception is thrown when a user is not confirmed successfully.</p>
    UserNotConfirmed(String),
    ///<p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteUserError {
    pub fn from_body(body: &str) -> DeleteUserError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        DeleteUserError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DeleteUserError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        DeleteUserError::NotAuthorized(String::from(error_message))
                    }
                    "PasswordResetRequiredException" => {
                        DeleteUserError::PasswordResetRequired(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteUserError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DeleteUserError::TooManyRequests(String::from(error_message))
                    }
                    "UserNotConfirmedException" => {
                        DeleteUserError::UserNotConfirmed(String::from(error_message))
                    }
                    "UserNotFoundException" => {
                        DeleteUserError::UserNotFound(String::from(error_message))
                    }
                    "ValidationException" => DeleteUserError::Validation(error_message.to_string()),
                    _ => DeleteUserError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteUserError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteUserError {
    fn from(err: serde_json::error::Error) -> DeleteUserError {
        DeleteUserError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteUserError {
    fn from(err: CredentialsError) -> DeleteUserError {
        DeleteUserError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteUserError {
    fn from(err: HttpDispatchError) -> DeleteUserError {
        DeleteUserError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteUserError {
    fn description(&self) -> &str {
        match *self {
            DeleteUserError::InternalError(ref cause) => cause,
            DeleteUserError::InvalidParameter(ref cause) => cause,
            DeleteUserError::NotAuthorized(ref cause) => cause,
            DeleteUserError::PasswordResetRequired(ref cause) => cause,
            DeleteUserError::ResourceNotFound(ref cause) => cause,
            DeleteUserError::TooManyRequests(ref cause) => cause,
            DeleteUserError::UserNotConfirmed(ref cause) => cause,
            DeleteUserError::UserNotFound(ref cause) => cause,
            DeleteUserError::Validation(ref cause) => cause,
            DeleteUserError::Credentials(ref err) => err.description(),
            DeleteUserError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteUserError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteUserAttributes
#[derive(Debug, PartialEq)]
pub enum DeleteUserAttributesError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when a password reset is required.</p>
    PasswordResetRequired(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    ///<p>This exception is thrown when a user is not confirmed successfully.</p>
    UserNotConfirmed(String),
    ///<p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteUserAttributesError {
    pub fn from_body(body: &str) -> DeleteUserAttributesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        DeleteUserAttributesError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DeleteUserAttributesError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        DeleteUserAttributesError::NotAuthorized(String::from(error_message))
                    }
                    "PasswordResetRequiredException" => DeleteUserAttributesError::PasswordResetRequired(String::from(error_message)),
                    "ResourceNotFoundException" => {
                        DeleteUserAttributesError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DeleteUserAttributesError::TooManyRequests(String::from(error_message))
                    }
                    "UserNotConfirmedException" => {
                        DeleteUserAttributesError::UserNotConfirmed(String::from(error_message))
                    }
                    "UserNotFoundException" => {
                        DeleteUserAttributesError::UserNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteUserAttributesError::Validation(error_message.to_string())
                    }
                    _ => DeleteUserAttributesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteUserAttributesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteUserAttributesError {
    fn from(err: serde_json::error::Error) -> DeleteUserAttributesError {
        DeleteUserAttributesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteUserAttributesError {
    fn from(err: CredentialsError) -> DeleteUserAttributesError {
        DeleteUserAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteUserAttributesError {
    fn from(err: HttpDispatchError) -> DeleteUserAttributesError {
        DeleteUserAttributesError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteUserAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteUserAttributesError {
    fn description(&self) -> &str {
        match *self {
            DeleteUserAttributesError::InternalError(ref cause) => cause,
            DeleteUserAttributesError::InvalidParameter(ref cause) => cause,
            DeleteUserAttributesError::NotAuthorized(ref cause) => cause,
            DeleteUserAttributesError::PasswordResetRequired(ref cause) => cause,
            DeleteUserAttributesError::ResourceNotFound(ref cause) => cause,
            DeleteUserAttributesError::TooManyRequests(ref cause) => cause,
            DeleteUserAttributesError::UserNotConfirmed(ref cause) => cause,
            DeleteUserAttributesError::UserNotFound(ref cause) => cause,
            DeleteUserAttributesError::Validation(ref cause) => cause,
            DeleteUserAttributesError::Credentials(ref err) => err.description(),
            DeleteUserAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteUserAttributesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteUserPool
#[derive(Debug, PartialEq)]
pub enum DeleteUserPoolError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    ///<p>This exception is thrown when you are trying to modify a user pool while a user import job is in progress for that pool.</p>
    UserImportInProgress(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteUserPoolError {
    pub fn from_body(body: &str) -> DeleteUserPoolError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        DeleteUserPoolError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DeleteUserPoolError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        DeleteUserPoolError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteUserPoolError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DeleteUserPoolError::TooManyRequests(String::from(error_message))
                    }
                    "UserImportInProgressException" => {
                        DeleteUserPoolError::UserImportInProgress(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteUserPoolError::Validation(error_message.to_string())
                    }
                    _ => DeleteUserPoolError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteUserPoolError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteUserPoolError {
    fn from(err: serde_json::error::Error) -> DeleteUserPoolError {
        DeleteUserPoolError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteUserPoolError {
    fn from(err: CredentialsError) -> DeleteUserPoolError {
        DeleteUserPoolError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteUserPoolError {
    fn from(err: HttpDispatchError) -> DeleteUserPoolError {
        DeleteUserPoolError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteUserPoolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteUserPoolError {
    fn description(&self) -> &str {
        match *self {
            DeleteUserPoolError::InternalError(ref cause) => cause,
            DeleteUserPoolError::InvalidParameter(ref cause) => cause,
            DeleteUserPoolError::NotAuthorized(ref cause) => cause,
            DeleteUserPoolError::ResourceNotFound(ref cause) => cause,
            DeleteUserPoolError::TooManyRequests(ref cause) => cause,
            DeleteUserPoolError::UserImportInProgress(ref cause) => cause,
            DeleteUserPoolError::Validation(ref cause) => cause,
            DeleteUserPoolError::Credentials(ref err) => err.description(),
            DeleteUserPoolError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteUserPoolError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteUserPoolClient
#[derive(Debug, PartialEq)]
pub enum DeleteUserPoolClientError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteUserPoolClientError {
    pub fn from_body(body: &str) -> DeleteUserPoolClientError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        DeleteUserPoolClientError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DeleteUserPoolClientError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        DeleteUserPoolClientError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteUserPoolClientError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DeleteUserPoolClientError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteUserPoolClientError::Validation(error_message.to_string())
                    }
                    _ => DeleteUserPoolClientError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteUserPoolClientError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteUserPoolClientError {
    fn from(err: serde_json::error::Error) -> DeleteUserPoolClientError {
        DeleteUserPoolClientError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteUserPoolClientError {
    fn from(err: CredentialsError) -> DeleteUserPoolClientError {
        DeleteUserPoolClientError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteUserPoolClientError {
    fn from(err: HttpDispatchError) -> DeleteUserPoolClientError {
        DeleteUserPoolClientError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteUserPoolClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteUserPoolClientError {
    fn description(&self) -> &str {
        match *self {
            DeleteUserPoolClientError::InternalError(ref cause) => cause,
            DeleteUserPoolClientError::InvalidParameter(ref cause) => cause,
            DeleteUserPoolClientError::NotAuthorized(ref cause) => cause,
            DeleteUserPoolClientError::ResourceNotFound(ref cause) => cause,
            DeleteUserPoolClientError::TooManyRequests(ref cause) => cause,
            DeleteUserPoolClientError::Validation(ref cause) => cause,
            DeleteUserPoolClientError::Credentials(ref err) => err.description(),
            DeleteUserPoolClientError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteUserPoolClientError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteUserPoolDomain
#[derive(Debug, PartialEq)]
pub enum DeleteUserPoolDomainError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteUserPoolDomainError {
    pub fn from_body(body: &str) -> DeleteUserPoolDomainError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        DeleteUserPoolDomainError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DeleteUserPoolDomainError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        DeleteUserPoolDomainError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteUserPoolDomainError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteUserPoolDomainError::Validation(error_message.to_string())
                    }
                    _ => DeleteUserPoolDomainError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteUserPoolDomainError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteUserPoolDomainError {
    fn from(err: serde_json::error::Error) -> DeleteUserPoolDomainError {
        DeleteUserPoolDomainError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteUserPoolDomainError {
    fn from(err: CredentialsError) -> DeleteUserPoolDomainError {
        DeleteUserPoolDomainError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteUserPoolDomainError {
    fn from(err: HttpDispatchError) -> DeleteUserPoolDomainError {
        DeleteUserPoolDomainError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteUserPoolDomainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteUserPoolDomainError {
    fn description(&self) -> &str {
        match *self {
            DeleteUserPoolDomainError::InternalError(ref cause) => cause,
            DeleteUserPoolDomainError::InvalidParameter(ref cause) => cause,
            DeleteUserPoolDomainError::NotAuthorized(ref cause) => cause,
            DeleteUserPoolDomainError::ResourceNotFound(ref cause) => cause,
            DeleteUserPoolDomainError::Validation(ref cause) => cause,
            DeleteUserPoolDomainError::Credentials(ref err) => err.description(),
            DeleteUserPoolDomainError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteUserPoolDomainError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeIdentityProvider
#[derive(Debug, PartialEq)]
pub enum DescribeIdentityProviderError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeIdentityProviderError {
    pub fn from_body(body: &str) -> DescribeIdentityProviderError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        DescribeIdentityProviderError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DescribeIdentityProviderError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        DescribeIdentityProviderError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DescribeIdentityProviderError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DescribeIdentityProviderError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeIdentityProviderError::Validation(error_message.to_string())
                    }
                    _ => DescribeIdentityProviderError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeIdentityProviderError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeIdentityProviderError {
    fn from(err: serde_json::error::Error) -> DescribeIdentityProviderError {
        DescribeIdentityProviderError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeIdentityProviderError {
    fn from(err: CredentialsError) -> DescribeIdentityProviderError {
        DescribeIdentityProviderError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeIdentityProviderError {
    fn from(err: HttpDispatchError) -> DescribeIdentityProviderError {
        DescribeIdentityProviderError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeIdentityProviderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeIdentityProviderError {
    fn description(&self) -> &str {
        match *self {
            DescribeIdentityProviderError::InternalError(ref cause) => cause,
            DescribeIdentityProviderError::InvalidParameter(ref cause) => cause,
            DescribeIdentityProviderError::NotAuthorized(ref cause) => cause,
            DescribeIdentityProviderError::ResourceNotFound(ref cause) => cause,
            DescribeIdentityProviderError::TooManyRequests(ref cause) => cause,
            DescribeIdentityProviderError::Validation(ref cause) => cause,
            DescribeIdentityProviderError::Credentials(ref err) => err.description(),
            DescribeIdentityProviderError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeIdentityProviderError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeUserImportJob
#[derive(Debug, PartialEq)]
pub enum DescribeUserImportJobError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeUserImportJobError {
    pub fn from_body(body: &str) -> DescribeUserImportJobError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        DescribeUserImportJobError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DescribeUserImportJobError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        DescribeUserImportJobError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DescribeUserImportJobError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DescribeUserImportJobError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeUserImportJobError::Validation(error_message.to_string())
                    }
                    _ => DescribeUserImportJobError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeUserImportJobError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeUserImportJobError {
    fn from(err: serde_json::error::Error) -> DescribeUserImportJobError {
        DescribeUserImportJobError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeUserImportJobError {
    fn from(err: CredentialsError) -> DescribeUserImportJobError {
        DescribeUserImportJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeUserImportJobError {
    fn from(err: HttpDispatchError) -> DescribeUserImportJobError {
        DescribeUserImportJobError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeUserImportJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeUserImportJobError {
    fn description(&self) -> &str {
        match *self {
            DescribeUserImportJobError::InternalError(ref cause) => cause,
            DescribeUserImportJobError::InvalidParameter(ref cause) => cause,
            DescribeUserImportJobError::NotAuthorized(ref cause) => cause,
            DescribeUserImportJobError::ResourceNotFound(ref cause) => cause,
            DescribeUserImportJobError::TooManyRequests(ref cause) => cause,
            DescribeUserImportJobError::Validation(ref cause) => cause,
            DescribeUserImportJobError::Credentials(ref err) => err.description(),
            DescribeUserImportJobError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeUserImportJobError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeUserPool
#[derive(Debug, PartialEq)]
pub enum DescribeUserPoolError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    ///<p>This exception is thrown when a user pool tag cannot be set or updated.</p>
    UserPoolTagging(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeUserPoolError {
    pub fn from_body(body: &str) -> DescribeUserPoolError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        DescribeUserPoolError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DescribeUserPoolError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        DescribeUserPoolError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DescribeUserPoolError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DescribeUserPoolError::TooManyRequests(String::from(error_message))
                    }
                    "UserPoolTaggingException" => {
                        DescribeUserPoolError::UserPoolTagging(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeUserPoolError::Validation(error_message.to_string())
                    }
                    _ => DescribeUserPoolError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeUserPoolError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeUserPoolError {
    fn from(err: serde_json::error::Error) -> DescribeUserPoolError {
        DescribeUserPoolError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeUserPoolError {
    fn from(err: CredentialsError) -> DescribeUserPoolError {
        DescribeUserPoolError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeUserPoolError {
    fn from(err: HttpDispatchError) -> DescribeUserPoolError {
        DescribeUserPoolError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeUserPoolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeUserPoolError {
    fn description(&self) -> &str {
        match *self {
            DescribeUserPoolError::InternalError(ref cause) => cause,
            DescribeUserPoolError::InvalidParameter(ref cause) => cause,
            DescribeUserPoolError::NotAuthorized(ref cause) => cause,
            DescribeUserPoolError::ResourceNotFound(ref cause) => cause,
            DescribeUserPoolError::TooManyRequests(ref cause) => cause,
            DescribeUserPoolError::UserPoolTagging(ref cause) => cause,
            DescribeUserPoolError::Validation(ref cause) => cause,
            DescribeUserPoolError::Credentials(ref err) => err.description(),
            DescribeUserPoolError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeUserPoolError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeUserPoolClient
#[derive(Debug, PartialEq)]
pub enum DescribeUserPoolClientError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeUserPoolClientError {
    pub fn from_body(body: &str) -> DescribeUserPoolClientError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        DescribeUserPoolClientError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DescribeUserPoolClientError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        DescribeUserPoolClientError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DescribeUserPoolClientError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DescribeUserPoolClientError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeUserPoolClientError::Validation(error_message.to_string())
                    }
                    _ => DescribeUserPoolClientError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeUserPoolClientError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeUserPoolClientError {
    fn from(err: serde_json::error::Error) -> DescribeUserPoolClientError {
        DescribeUserPoolClientError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeUserPoolClientError {
    fn from(err: CredentialsError) -> DescribeUserPoolClientError {
        DescribeUserPoolClientError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeUserPoolClientError {
    fn from(err: HttpDispatchError) -> DescribeUserPoolClientError {
        DescribeUserPoolClientError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeUserPoolClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeUserPoolClientError {
    fn description(&self) -> &str {
        match *self {
            DescribeUserPoolClientError::InternalError(ref cause) => cause,
            DescribeUserPoolClientError::InvalidParameter(ref cause) => cause,
            DescribeUserPoolClientError::NotAuthorized(ref cause) => cause,
            DescribeUserPoolClientError::ResourceNotFound(ref cause) => cause,
            DescribeUserPoolClientError::TooManyRequests(ref cause) => cause,
            DescribeUserPoolClientError::Validation(ref cause) => cause,
            DescribeUserPoolClientError::Credentials(ref err) => err.description(),
            DescribeUserPoolClientError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeUserPoolClientError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeUserPoolDomain
#[derive(Debug, PartialEq)]
pub enum DescribeUserPoolDomainError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeUserPoolDomainError {
    pub fn from_body(body: &str) -> DescribeUserPoolDomainError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        DescribeUserPoolDomainError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DescribeUserPoolDomainError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        DescribeUserPoolDomainError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DescribeUserPoolDomainError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeUserPoolDomainError::Validation(error_message.to_string())
                    }
                    _ => DescribeUserPoolDomainError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeUserPoolDomainError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeUserPoolDomainError {
    fn from(err: serde_json::error::Error) -> DescribeUserPoolDomainError {
        DescribeUserPoolDomainError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeUserPoolDomainError {
    fn from(err: CredentialsError) -> DescribeUserPoolDomainError {
        DescribeUserPoolDomainError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeUserPoolDomainError {
    fn from(err: HttpDispatchError) -> DescribeUserPoolDomainError {
        DescribeUserPoolDomainError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeUserPoolDomainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeUserPoolDomainError {
    fn description(&self) -> &str {
        match *self {
            DescribeUserPoolDomainError::InternalError(ref cause) => cause,
            DescribeUserPoolDomainError::InvalidParameter(ref cause) => cause,
            DescribeUserPoolDomainError::NotAuthorized(ref cause) => cause,
            DescribeUserPoolDomainError::ResourceNotFound(ref cause) => cause,
            DescribeUserPoolDomainError::Validation(ref cause) => cause,
            DescribeUserPoolDomainError::Credentials(ref err) => err.description(),
            DescribeUserPoolDomainError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeUserPoolDomainError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ForgetDevice
#[derive(Debug, PartialEq)]
pub enum ForgetDeviceError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when the user pool configuration is invalid.</p>
    InvalidUserPoolConfiguration(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when a password reset is required.</p>
    PasswordResetRequired(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    ///<p>This exception is thrown when a user is not confirmed successfully.</p>
    UserNotConfirmed(String),
    ///<p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ForgetDeviceError {
    pub fn from_body(body: &str) -> ForgetDeviceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        ForgetDeviceError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        ForgetDeviceError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidUserPoolConfigurationException" => {
                        ForgetDeviceError::InvalidUserPoolConfiguration(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        ForgetDeviceError::NotAuthorized(String::from(error_message))
                    }
                    "PasswordResetRequiredException" => {
                        ForgetDeviceError::PasswordResetRequired(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ForgetDeviceError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        ForgetDeviceError::TooManyRequests(String::from(error_message))
                    }
                    "UserNotConfirmedException" => {
                        ForgetDeviceError::UserNotConfirmed(String::from(error_message))
                    }
                    "UserNotFoundException" => {
                        ForgetDeviceError::UserNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        ForgetDeviceError::Validation(error_message.to_string())
                    }
                    _ => ForgetDeviceError::Unknown(String::from(body)),
                }
            }
            Err(_) => ForgetDeviceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ForgetDeviceError {
    fn from(err: serde_json::error::Error) -> ForgetDeviceError {
        ForgetDeviceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ForgetDeviceError {
    fn from(err: CredentialsError) -> ForgetDeviceError {
        ForgetDeviceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ForgetDeviceError {
    fn from(err: HttpDispatchError) -> ForgetDeviceError {
        ForgetDeviceError::HttpDispatch(err)
    }
}
impl fmt::Display for ForgetDeviceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ForgetDeviceError {
    fn description(&self) -> &str {
        match *self {
            ForgetDeviceError::InternalError(ref cause) => cause,
            ForgetDeviceError::InvalidParameter(ref cause) => cause,
            ForgetDeviceError::InvalidUserPoolConfiguration(ref cause) => cause,
            ForgetDeviceError::NotAuthorized(ref cause) => cause,
            ForgetDeviceError::PasswordResetRequired(ref cause) => cause,
            ForgetDeviceError::ResourceNotFound(ref cause) => cause,
            ForgetDeviceError::TooManyRequests(ref cause) => cause,
            ForgetDeviceError::UserNotConfirmed(ref cause) => cause,
            ForgetDeviceError::UserNotFound(ref cause) => cause,
            ForgetDeviceError::Validation(ref cause) => cause,
            ForgetDeviceError::Credentials(ref err) => err.description(),
            ForgetDeviceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ForgetDeviceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ForgotPassword
#[derive(Debug, PartialEq)]
pub enum ForgotPasswordError {
    ///<p>This exception is thrown when a verification code fails to deliver successfully.</p>
    CodeDeliveryFailure(String),
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when Amazon Cognito is not allowed to use your email identity. HTTP status code: 400.</p>
    InvalidEmailRoleAccessPolicy(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid AWS Lambda response.</p>
    InvalidLambdaResponse(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is returned when the role provided for SMS configuration does not have permission to publish using Amazon SNS.</p>
    InvalidSmsRoleAccessPolicy(String),
    ///<p>This exception is thrown when the trust relationship is invalid for the role provided for SMS configuration. This can happen if you do not trust <b>cognito-idp.amazonaws.com</b> or the external ID provided in the role does not match what is provided in the SMS configuration for the user pool.</p>
    InvalidSmsRoleTrustRelationship(String),
    ///<p>This exception is thrown when a user exceeds the limit for a requested AWS resource.</p>
    LimitExceeded(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an unexpected exception with the AWS Lambda service.</p>
    UnexpectedLambda(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters a user validation exception with the AWS Lambda service.</p>
    UserLambdaValidation(String),
    ///<p>This exception is thrown when a user is not confirmed successfully.</p>
    UserNotConfirmed(String),
    ///<p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ForgotPasswordError {
    pub fn from_body(body: &str) -> ForgotPasswordError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "CodeDeliveryFailureException" => {
                        ForgotPasswordError::CodeDeliveryFailure(String::from(error_message))
                    }
                    "InternalErrorException" => {
                        ForgotPasswordError::InternalError(String::from(error_message))
                    }
                    "InvalidEmailRoleAccessPolicyException" => ForgotPasswordError::InvalidEmailRoleAccessPolicy(String::from(error_message)),
                    "InvalidLambdaResponseException" => {
                        ForgotPasswordError::InvalidLambdaResponse(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        ForgotPasswordError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidSmsRoleAccessPolicyException" => {
                        ForgotPasswordError::InvalidSmsRoleAccessPolicy(String::from(error_message))
                    }
                    "InvalidSmsRoleTrustRelationshipException" => ForgotPasswordError::InvalidSmsRoleTrustRelationship(String::from(error_message)),
                    "LimitExceededException" => {
                        ForgotPasswordError::LimitExceeded(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        ForgotPasswordError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ForgotPasswordError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        ForgotPasswordError::TooManyRequests(String::from(error_message))
                    }
                    "UnexpectedLambdaException" => {
                        ForgotPasswordError::UnexpectedLambda(String::from(error_message))
                    }
                    "UserLambdaValidationException" => {
                        ForgotPasswordError::UserLambdaValidation(String::from(error_message))
                    }
                    "UserNotConfirmedException" => {
                        ForgotPasswordError::UserNotConfirmed(String::from(error_message))
                    }
                    "UserNotFoundException" => {
                        ForgotPasswordError::UserNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        ForgotPasswordError::Validation(error_message.to_string())
                    }
                    _ => ForgotPasswordError::Unknown(String::from(body)),
                }
            }
            Err(_) => ForgotPasswordError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ForgotPasswordError {
    fn from(err: serde_json::error::Error) -> ForgotPasswordError {
        ForgotPasswordError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ForgotPasswordError {
    fn from(err: CredentialsError) -> ForgotPasswordError {
        ForgotPasswordError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ForgotPasswordError {
    fn from(err: HttpDispatchError) -> ForgotPasswordError {
        ForgotPasswordError::HttpDispatch(err)
    }
}
impl fmt::Display for ForgotPasswordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ForgotPasswordError {
    fn description(&self) -> &str {
        match *self {
            ForgotPasswordError::CodeDeliveryFailure(ref cause) => cause,
            ForgotPasswordError::InternalError(ref cause) => cause,
            ForgotPasswordError::InvalidEmailRoleAccessPolicy(ref cause) => cause,
            ForgotPasswordError::InvalidLambdaResponse(ref cause) => cause,
            ForgotPasswordError::InvalidParameter(ref cause) => cause,
            ForgotPasswordError::InvalidSmsRoleAccessPolicy(ref cause) => cause,
            ForgotPasswordError::InvalidSmsRoleTrustRelationship(ref cause) => cause,
            ForgotPasswordError::LimitExceeded(ref cause) => cause,
            ForgotPasswordError::NotAuthorized(ref cause) => cause,
            ForgotPasswordError::ResourceNotFound(ref cause) => cause,
            ForgotPasswordError::TooManyRequests(ref cause) => cause,
            ForgotPasswordError::UnexpectedLambda(ref cause) => cause,
            ForgotPasswordError::UserLambdaValidation(ref cause) => cause,
            ForgotPasswordError::UserNotConfirmed(ref cause) => cause,
            ForgotPasswordError::UserNotFound(ref cause) => cause,
            ForgotPasswordError::Validation(ref cause) => cause,
            ForgotPasswordError::Credentials(ref err) => err.description(),
            ForgotPasswordError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ForgotPasswordError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetCSVHeader
#[derive(Debug, PartialEq)]
pub enum GetCSVHeaderError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetCSVHeaderError {
    pub fn from_body(body: &str) -> GetCSVHeaderError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        GetCSVHeaderError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        GetCSVHeaderError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        GetCSVHeaderError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        GetCSVHeaderError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetCSVHeaderError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetCSVHeaderError::Validation(error_message.to_string())
                    }
                    _ => GetCSVHeaderError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetCSVHeaderError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetCSVHeaderError {
    fn from(err: serde_json::error::Error) -> GetCSVHeaderError {
        GetCSVHeaderError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetCSVHeaderError {
    fn from(err: CredentialsError) -> GetCSVHeaderError {
        GetCSVHeaderError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetCSVHeaderError {
    fn from(err: HttpDispatchError) -> GetCSVHeaderError {
        GetCSVHeaderError::HttpDispatch(err)
    }
}
impl fmt::Display for GetCSVHeaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCSVHeaderError {
    fn description(&self) -> &str {
        match *self {
            GetCSVHeaderError::InternalError(ref cause) => cause,
            GetCSVHeaderError::InvalidParameter(ref cause) => cause,
            GetCSVHeaderError::NotAuthorized(ref cause) => cause,
            GetCSVHeaderError::ResourceNotFound(ref cause) => cause,
            GetCSVHeaderError::TooManyRequests(ref cause) => cause,
            GetCSVHeaderError::Validation(ref cause) => cause,
            GetCSVHeaderError::Credentials(ref err) => err.description(),
            GetCSVHeaderError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetCSVHeaderError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDevice
#[derive(Debug, PartialEq)]
pub enum GetDeviceError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when the user pool configuration is invalid.</p>
    InvalidUserPoolConfiguration(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when a password reset is required.</p>
    PasswordResetRequired(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    ///<p>This exception is thrown when a user is not confirmed successfully.</p>
    UserNotConfirmed(String),
    ///<p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetDeviceError {
    pub fn from_body(body: &str) -> GetDeviceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        GetDeviceError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        GetDeviceError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidUserPoolConfigurationException" => {
                        GetDeviceError::InvalidUserPoolConfiguration(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        GetDeviceError::NotAuthorized(String::from(error_message))
                    }
                    "PasswordResetRequiredException" => {
                        GetDeviceError::PasswordResetRequired(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        GetDeviceError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetDeviceError::TooManyRequests(String::from(error_message))
                    }
                    "UserNotConfirmedException" => {
                        GetDeviceError::UserNotConfirmed(String::from(error_message))
                    }
                    "UserNotFoundException" => {
                        GetDeviceError::UserNotFound(String::from(error_message))
                    }
                    "ValidationException" => GetDeviceError::Validation(error_message.to_string()),
                    _ => GetDeviceError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetDeviceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetDeviceError {
    fn from(err: serde_json::error::Error) -> GetDeviceError {
        GetDeviceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDeviceError {
    fn from(err: CredentialsError) -> GetDeviceError {
        GetDeviceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDeviceError {
    fn from(err: HttpDispatchError) -> GetDeviceError {
        GetDeviceError::HttpDispatch(err)
    }
}
impl fmt::Display for GetDeviceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDeviceError {
    fn description(&self) -> &str {
        match *self {
            GetDeviceError::InternalError(ref cause) => cause,
            GetDeviceError::InvalidParameter(ref cause) => cause,
            GetDeviceError::InvalidUserPoolConfiguration(ref cause) => cause,
            GetDeviceError::NotAuthorized(ref cause) => cause,
            GetDeviceError::PasswordResetRequired(ref cause) => cause,
            GetDeviceError::ResourceNotFound(ref cause) => cause,
            GetDeviceError::TooManyRequests(ref cause) => cause,
            GetDeviceError::UserNotConfirmed(ref cause) => cause,
            GetDeviceError::UserNotFound(ref cause) => cause,
            GetDeviceError::Validation(ref cause) => cause,
            GetDeviceError::Credentials(ref err) => err.description(),
            GetDeviceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetDeviceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetGroup
#[derive(Debug, PartialEq)]
pub enum GetGroupError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetGroupError {
    pub fn from_body(body: &str) -> GetGroupError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        GetGroupError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        GetGroupError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        GetGroupError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        GetGroupError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetGroupError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => GetGroupError::Validation(error_message.to_string()),
                    _ => GetGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetGroupError {
    fn from(err: serde_json::error::Error) -> GetGroupError {
        GetGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetGroupError {
    fn from(err: CredentialsError) -> GetGroupError {
        GetGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetGroupError {
    fn from(err: HttpDispatchError) -> GetGroupError {
        GetGroupError::HttpDispatch(err)
    }
}
impl fmt::Display for GetGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetGroupError {
    fn description(&self) -> &str {
        match *self {
            GetGroupError::InternalError(ref cause) => cause,
            GetGroupError::InvalidParameter(ref cause) => cause,
            GetGroupError::NotAuthorized(ref cause) => cause,
            GetGroupError::ResourceNotFound(ref cause) => cause,
            GetGroupError::TooManyRequests(ref cause) => cause,
            GetGroupError::Validation(ref cause) => cause,
            GetGroupError::Credentials(ref err) => err.description(),
            GetGroupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetIdentityProviderByIdentifier
#[derive(Debug, PartialEq)]
pub enum GetIdentityProviderByIdentifierError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetIdentityProviderByIdentifierError {
    pub fn from_body(body: &str) -> GetIdentityProviderByIdentifierError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => GetIdentityProviderByIdentifierError::InternalError(String::from(error_message)),
                    "InvalidParameterException" => GetIdentityProviderByIdentifierError::InvalidParameter(String::from(error_message)),
                    "NotAuthorizedException" => GetIdentityProviderByIdentifierError::NotAuthorized(String::from(error_message)),
                    "ResourceNotFoundException" => GetIdentityProviderByIdentifierError::ResourceNotFound(String::from(error_message)),
                    "TooManyRequestsException" => GetIdentityProviderByIdentifierError::TooManyRequests(String::from(error_message)),
                    "ValidationException" => {
                        GetIdentityProviderByIdentifierError::Validation(error_message.to_string())
                    }
                    _ => GetIdentityProviderByIdentifierError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetIdentityProviderByIdentifierError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetIdentityProviderByIdentifierError {
    fn from(err: serde_json::error::Error) -> GetIdentityProviderByIdentifierError {
        GetIdentityProviderByIdentifierError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetIdentityProviderByIdentifierError {
    fn from(err: CredentialsError) -> GetIdentityProviderByIdentifierError {
        GetIdentityProviderByIdentifierError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetIdentityProviderByIdentifierError {
    fn from(err: HttpDispatchError) -> GetIdentityProviderByIdentifierError {
        GetIdentityProviderByIdentifierError::HttpDispatch(err)
    }
}
impl fmt::Display for GetIdentityProviderByIdentifierError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetIdentityProviderByIdentifierError {
    fn description(&self) -> &str {
        match *self {
            GetIdentityProviderByIdentifierError::InternalError(ref cause) => cause,
            GetIdentityProviderByIdentifierError::InvalidParameter(ref cause) => cause,
            GetIdentityProviderByIdentifierError::NotAuthorized(ref cause) => cause,
            GetIdentityProviderByIdentifierError::ResourceNotFound(ref cause) => cause,
            GetIdentityProviderByIdentifierError::TooManyRequests(ref cause) => cause,
            GetIdentityProviderByIdentifierError::Validation(ref cause) => cause,
            GetIdentityProviderByIdentifierError::Credentials(ref err) => err.description(),
            GetIdentityProviderByIdentifierError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetIdentityProviderByIdentifierError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetUser
#[derive(Debug, PartialEq)]
pub enum GetUserError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when a password reset is required.</p>
    PasswordResetRequired(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    ///<p>This exception is thrown when a user is not confirmed successfully.</p>
    UserNotConfirmed(String),
    ///<p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetUserError {
    pub fn from_body(body: &str) -> GetUserError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        GetUserError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        GetUserError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        GetUserError::NotAuthorized(String::from(error_message))
                    }
                    "PasswordResetRequiredException" => {
                        GetUserError::PasswordResetRequired(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        GetUserError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetUserError::TooManyRequests(String::from(error_message))
                    }
                    "UserNotConfirmedException" => {
                        GetUserError::UserNotConfirmed(String::from(error_message))
                    }
                    "UserNotFoundException" => {
                        GetUserError::UserNotFound(String::from(error_message))
                    }
                    "ValidationException" => GetUserError::Validation(error_message.to_string()),
                    _ => GetUserError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetUserError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetUserError {
    fn from(err: serde_json::error::Error) -> GetUserError {
        GetUserError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetUserError {
    fn from(err: CredentialsError) -> GetUserError {
        GetUserError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetUserError {
    fn from(err: HttpDispatchError) -> GetUserError {
        GetUserError::HttpDispatch(err)
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
            GetUserError::InternalError(ref cause) => cause,
            GetUserError::InvalidParameter(ref cause) => cause,
            GetUserError::NotAuthorized(ref cause) => cause,
            GetUserError::PasswordResetRequired(ref cause) => cause,
            GetUserError::ResourceNotFound(ref cause) => cause,
            GetUserError::TooManyRequests(ref cause) => cause,
            GetUserError::UserNotConfirmed(ref cause) => cause,
            GetUserError::UserNotFound(ref cause) => cause,
            GetUserError::Validation(ref cause) => cause,
            GetUserError::Credentials(ref err) => err.description(),
            GetUserError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetUserError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetUserAttributeVerificationCode
#[derive(Debug, PartialEq)]
pub enum GetUserAttributeVerificationCodeError {
    ///<p>This exception is thrown when a verification code fails to deliver successfully.</p>
    CodeDeliveryFailure(String),
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when Amazon Cognito is not allowed to use your email identity. HTTP status code: 400.</p>
    InvalidEmailRoleAccessPolicy(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid AWS Lambda response.</p>
    InvalidLambdaResponse(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is returned when the role provided for SMS configuration does not have permission to publish using Amazon SNS.</p>
    InvalidSmsRoleAccessPolicy(String),
    ///<p>This exception is thrown when the trust relationship is invalid for the role provided for SMS configuration. This can happen if you do not trust <b>cognito-idp.amazonaws.com</b> or the external ID provided in the role does not match what is provided in the SMS configuration for the user pool.</p>
    InvalidSmsRoleTrustRelationship(String),
    ///<p>This exception is thrown when a user exceeds the limit for a requested AWS resource.</p>
    LimitExceeded(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when a password reset is required.</p>
    PasswordResetRequired(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an unexpected exception with the AWS Lambda service.</p>
    UnexpectedLambda(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters a user validation exception with the AWS Lambda service.</p>
    UserLambdaValidation(String),
    ///<p>This exception is thrown when a user is not confirmed successfully.</p>
    UserNotConfirmed(String),
    ///<p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetUserAttributeVerificationCodeError {
    pub fn from_body(body: &str) -> GetUserAttributeVerificationCodeError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "CodeDeliveryFailureException" => GetUserAttributeVerificationCodeError::CodeDeliveryFailure(String::from(error_message)),
                    "InternalErrorException" => GetUserAttributeVerificationCodeError::InternalError(String::from(error_message)),
                    "InvalidEmailRoleAccessPolicyException" => GetUserAttributeVerificationCodeError::InvalidEmailRoleAccessPolicy(String::from(error_message)),
                    "InvalidLambdaResponseException" => GetUserAttributeVerificationCodeError::InvalidLambdaResponse(String::from(error_message)),
                    "InvalidParameterException" => GetUserAttributeVerificationCodeError::InvalidParameter(String::from(error_message)),
                    "InvalidSmsRoleAccessPolicyException" => GetUserAttributeVerificationCodeError::InvalidSmsRoleAccessPolicy(String::from(error_message)),
                    "InvalidSmsRoleTrustRelationshipException" => GetUserAttributeVerificationCodeError::InvalidSmsRoleTrustRelationship(String::from(error_message)),
                    "LimitExceededException" => GetUserAttributeVerificationCodeError::LimitExceeded(String::from(error_message)),
                    "NotAuthorizedException" => GetUserAttributeVerificationCodeError::NotAuthorized(String::from(error_message)),
                    "PasswordResetRequiredException" => GetUserAttributeVerificationCodeError::PasswordResetRequired(String::from(error_message)),
                    "ResourceNotFoundException" => GetUserAttributeVerificationCodeError::ResourceNotFound(String::from(error_message)),
                    "TooManyRequestsException" => GetUserAttributeVerificationCodeError::TooManyRequests(String::from(error_message)),
                    "UnexpectedLambdaException" => GetUserAttributeVerificationCodeError::UnexpectedLambda(String::from(error_message)),
                    "UserLambdaValidationException" => GetUserAttributeVerificationCodeError::UserLambdaValidation(String::from(error_message)),
                    "UserNotConfirmedException" => GetUserAttributeVerificationCodeError::UserNotConfirmed(String::from(error_message)),
                    "UserNotFoundException" => GetUserAttributeVerificationCodeError::UserNotFound(String::from(error_message)),
                    "ValidationException" => {
                        GetUserAttributeVerificationCodeError::Validation(error_message.to_string())
                    }
                    _ => GetUserAttributeVerificationCodeError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetUserAttributeVerificationCodeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetUserAttributeVerificationCodeError {
    fn from(err: serde_json::error::Error) -> GetUserAttributeVerificationCodeError {
        GetUserAttributeVerificationCodeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetUserAttributeVerificationCodeError {
    fn from(err: CredentialsError) -> GetUserAttributeVerificationCodeError {
        GetUserAttributeVerificationCodeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetUserAttributeVerificationCodeError {
    fn from(err: HttpDispatchError) -> GetUserAttributeVerificationCodeError {
        GetUserAttributeVerificationCodeError::HttpDispatch(err)
    }
}
impl fmt::Display for GetUserAttributeVerificationCodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetUserAttributeVerificationCodeError {
    fn description(&self) -> &str {
        match *self {
            GetUserAttributeVerificationCodeError::CodeDeliveryFailure(ref cause) => cause,
            GetUserAttributeVerificationCodeError::InternalError(ref cause) => cause,
            GetUserAttributeVerificationCodeError::InvalidEmailRoleAccessPolicy(ref cause) => cause,
            GetUserAttributeVerificationCodeError::InvalidLambdaResponse(ref cause) => cause,
            GetUserAttributeVerificationCodeError::InvalidParameter(ref cause) => cause,
            GetUserAttributeVerificationCodeError::InvalidSmsRoleAccessPolicy(ref cause) => cause,
            GetUserAttributeVerificationCodeError::InvalidSmsRoleTrustRelationship(ref cause) => {
                cause
            }
            GetUserAttributeVerificationCodeError::LimitExceeded(ref cause) => cause,
            GetUserAttributeVerificationCodeError::NotAuthorized(ref cause) => cause,
            GetUserAttributeVerificationCodeError::PasswordResetRequired(ref cause) => cause,
            GetUserAttributeVerificationCodeError::ResourceNotFound(ref cause) => cause,
            GetUserAttributeVerificationCodeError::TooManyRequests(ref cause) => cause,
            GetUserAttributeVerificationCodeError::UnexpectedLambda(ref cause) => cause,
            GetUserAttributeVerificationCodeError::UserLambdaValidation(ref cause) => cause,
            GetUserAttributeVerificationCodeError::UserNotConfirmed(ref cause) => cause,
            GetUserAttributeVerificationCodeError::UserNotFound(ref cause) => cause,
            GetUserAttributeVerificationCodeError::Validation(ref cause) => cause,
            GetUserAttributeVerificationCodeError::Credentials(ref err) => err.description(),
            GetUserAttributeVerificationCodeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetUserAttributeVerificationCodeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GlobalSignOut
#[derive(Debug, PartialEq)]
pub enum GlobalSignOutError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when a password reset is required.</p>
    PasswordResetRequired(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    ///<p>This exception is thrown when a user is not confirmed successfully.</p>
    UserNotConfirmed(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GlobalSignOutError {
    pub fn from_body(body: &str) -> GlobalSignOutError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        GlobalSignOutError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        GlobalSignOutError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        GlobalSignOutError::NotAuthorized(String::from(error_message))
                    }
                    "PasswordResetRequiredException" => {
                        GlobalSignOutError::PasswordResetRequired(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        GlobalSignOutError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GlobalSignOutError::TooManyRequests(String::from(error_message))
                    }
                    "UserNotConfirmedException" => {
                        GlobalSignOutError::UserNotConfirmed(String::from(error_message))
                    }
                    "ValidationException" => {
                        GlobalSignOutError::Validation(error_message.to_string())
                    }
                    _ => GlobalSignOutError::Unknown(String::from(body)),
                }
            }
            Err(_) => GlobalSignOutError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GlobalSignOutError {
    fn from(err: serde_json::error::Error) -> GlobalSignOutError {
        GlobalSignOutError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GlobalSignOutError {
    fn from(err: CredentialsError) -> GlobalSignOutError {
        GlobalSignOutError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GlobalSignOutError {
    fn from(err: HttpDispatchError) -> GlobalSignOutError {
        GlobalSignOutError::HttpDispatch(err)
    }
}
impl fmt::Display for GlobalSignOutError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GlobalSignOutError {
    fn description(&self) -> &str {
        match *self {
            GlobalSignOutError::InternalError(ref cause) => cause,
            GlobalSignOutError::InvalidParameter(ref cause) => cause,
            GlobalSignOutError::NotAuthorized(ref cause) => cause,
            GlobalSignOutError::PasswordResetRequired(ref cause) => cause,
            GlobalSignOutError::ResourceNotFound(ref cause) => cause,
            GlobalSignOutError::TooManyRequests(ref cause) => cause,
            GlobalSignOutError::UserNotConfirmed(ref cause) => cause,
            GlobalSignOutError::Validation(ref cause) => cause,
            GlobalSignOutError::Credentials(ref err) => err.description(),
            GlobalSignOutError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GlobalSignOutError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by InitiateAuth
#[derive(Debug, PartialEq)]
pub enum InitiateAuthError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid AWS Lambda response.</p>
    InvalidLambdaResponse(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when the user pool configuration is invalid.</p>
    InvalidUserPoolConfiguration(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when a password reset is required.</p>
    PasswordResetRequired(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an unexpected exception with the AWS Lambda service.</p>
    UnexpectedLambda(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters a user validation exception with the AWS Lambda service.</p>
    UserLambdaValidation(String),
    ///<p>This exception is thrown when a user is not confirmed successfully.</p>
    UserNotConfirmed(String),
    ///<p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl InitiateAuthError {
    pub fn from_body(body: &str) -> InitiateAuthError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        InitiateAuthError::InternalError(String::from(error_message))
                    }
                    "InvalidLambdaResponseException" => {
                        InitiateAuthError::InvalidLambdaResponse(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        InitiateAuthError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidUserPoolConfigurationException" => {
                        InitiateAuthError::InvalidUserPoolConfiguration(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        InitiateAuthError::NotAuthorized(String::from(error_message))
                    }
                    "PasswordResetRequiredException" => {
                        InitiateAuthError::PasswordResetRequired(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        InitiateAuthError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        InitiateAuthError::TooManyRequests(String::from(error_message))
                    }
                    "UnexpectedLambdaException" => {
                        InitiateAuthError::UnexpectedLambda(String::from(error_message))
                    }
                    "UserLambdaValidationException" => {
                        InitiateAuthError::UserLambdaValidation(String::from(error_message))
                    }
                    "UserNotConfirmedException" => {
                        InitiateAuthError::UserNotConfirmed(String::from(error_message))
                    }
                    "UserNotFoundException" => {
                        InitiateAuthError::UserNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        InitiateAuthError::Validation(error_message.to_string())
                    }
                    _ => InitiateAuthError::Unknown(String::from(body)),
                }
            }
            Err(_) => InitiateAuthError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for InitiateAuthError {
    fn from(err: serde_json::error::Error) -> InitiateAuthError {
        InitiateAuthError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for InitiateAuthError {
    fn from(err: CredentialsError) -> InitiateAuthError {
        InitiateAuthError::Credentials(err)
    }
}
impl From<HttpDispatchError> for InitiateAuthError {
    fn from(err: HttpDispatchError) -> InitiateAuthError {
        InitiateAuthError::HttpDispatch(err)
    }
}
impl fmt::Display for InitiateAuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for InitiateAuthError {
    fn description(&self) -> &str {
        match *self {
            InitiateAuthError::InternalError(ref cause) => cause,
            InitiateAuthError::InvalidLambdaResponse(ref cause) => cause,
            InitiateAuthError::InvalidParameter(ref cause) => cause,
            InitiateAuthError::InvalidUserPoolConfiguration(ref cause) => cause,
            InitiateAuthError::NotAuthorized(ref cause) => cause,
            InitiateAuthError::PasswordResetRequired(ref cause) => cause,
            InitiateAuthError::ResourceNotFound(ref cause) => cause,
            InitiateAuthError::TooManyRequests(ref cause) => cause,
            InitiateAuthError::UnexpectedLambda(ref cause) => cause,
            InitiateAuthError::UserLambdaValidation(ref cause) => cause,
            InitiateAuthError::UserNotConfirmed(ref cause) => cause,
            InitiateAuthError::UserNotFound(ref cause) => cause,
            InitiateAuthError::Validation(ref cause) => cause,
            InitiateAuthError::Credentials(ref err) => err.description(),
            InitiateAuthError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            InitiateAuthError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListDevices
#[derive(Debug, PartialEq)]
pub enum ListDevicesError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when the user pool configuration is invalid.</p>
    InvalidUserPoolConfiguration(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when a password reset is required.</p>
    PasswordResetRequired(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    ///<p>This exception is thrown when a user is not confirmed successfully.</p>
    UserNotConfirmed(String),
    ///<p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListDevicesError {
    pub fn from_body(body: &str) -> ListDevicesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        ListDevicesError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        ListDevicesError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidUserPoolConfigurationException" => {
                        ListDevicesError::InvalidUserPoolConfiguration(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        ListDevicesError::NotAuthorized(String::from(error_message))
                    }
                    "PasswordResetRequiredException" => {
                        ListDevicesError::PasswordResetRequired(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListDevicesError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        ListDevicesError::TooManyRequests(String::from(error_message))
                    }
                    "UserNotConfirmedException" => {
                        ListDevicesError::UserNotConfirmed(String::from(error_message))
                    }
                    "UserNotFoundException" => {
                        ListDevicesError::UserNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListDevicesError::Validation(error_message.to_string())
                    }
                    _ => ListDevicesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListDevicesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListDevicesError {
    fn from(err: serde_json::error::Error) -> ListDevicesError {
        ListDevicesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListDevicesError {
    fn from(err: CredentialsError) -> ListDevicesError {
        ListDevicesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListDevicesError {
    fn from(err: HttpDispatchError) -> ListDevicesError {
        ListDevicesError::HttpDispatch(err)
    }
}
impl fmt::Display for ListDevicesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDevicesError {
    fn description(&self) -> &str {
        match *self {
            ListDevicesError::InternalError(ref cause) => cause,
            ListDevicesError::InvalidParameter(ref cause) => cause,
            ListDevicesError::InvalidUserPoolConfiguration(ref cause) => cause,
            ListDevicesError::NotAuthorized(ref cause) => cause,
            ListDevicesError::PasswordResetRequired(ref cause) => cause,
            ListDevicesError::ResourceNotFound(ref cause) => cause,
            ListDevicesError::TooManyRequests(ref cause) => cause,
            ListDevicesError::UserNotConfirmed(ref cause) => cause,
            ListDevicesError::UserNotFound(ref cause) => cause,
            ListDevicesError::Validation(ref cause) => cause,
            ListDevicesError::Credentials(ref err) => err.description(),
            ListDevicesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListDevicesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListGroups
#[derive(Debug, PartialEq)]
pub enum ListGroupsError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListGroupsError {
    pub fn from_body(body: &str) -> ListGroupsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        ListGroupsError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        ListGroupsError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        ListGroupsError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListGroupsError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        ListGroupsError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => ListGroupsError::Validation(error_message.to_string()),
                    _ => ListGroupsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListGroupsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListGroupsError {
    fn from(err: serde_json::error::Error) -> ListGroupsError {
        ListGroupsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListGroupsError {
    fn from(err: CredentialsError) -> ListGroupsError {
        ListGroupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListGroupsError {
    fn from(err: HttpDispatchError) -> ListGroupsError {
        ListGroupsError::HttpDispatch(err)
    }
}
impl fmt::Display for ListGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListGroupsError {
    fn description(&self) -> &str {
        match *self {
            ListGroupsError::InternalError(ref cause) => cause,
            ListGroupsError::InvalidParameter(ref cause) => cause,
            ListGroupsError::NotAuthorized(ref cause) => cause,
            ListGroupsError::ResourceNotFound(ref cause) => cause,
            ListGroupsError::TooManyRequests(ref cause) => cause,
            ListGroupsError::Validation(ref cause) => cause,
            ListGroupsError::Credentials(ref err) => err.description(),
            ListGroupsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListGroupsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListIdentityProviders
#[derive(Debug, PartialEq)]
pub enum ListIdentityProvidersError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListIdentityProvidersError {
    pub fn from_body(body: &str) -> ListIdentityProvidersError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        ListIdentityProvidersError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        ListIdentityProvidersError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        ListIdentityProvidersError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListIdentityProvidersError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        ListIdentityProvidersError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListIdentityProvidersError::Validation(error_message.to_string())
                    }
                    _ => ListIdentityProvidersError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListIdentityProvidersError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListIdentityProvidersError {
    fn from(err: serde_json::error::Error) -> ListIdentityProvidersError {
        ListIdentityProvidersError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListIdentityProvidersError {
    fn from(err: CredentialsError) -> ListIdentityProvidersError {
        ListIdentityProvidersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListIdentityProvidersError {
    fn from(err: HttpDispatchError) -> ListIdentityProvidersError {
        ListIdentityProvidersError::HttpDispatch(err)
    }
}
impl fmt::Display for ListIdentityProvidersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListIdentityProvidersError {
    fn description(&self) -> &str {
        match *self {
            ListIdentityProvidersError::InternalError(ref cause) => cause,
            ListIdentityProvidersError::InvalidParameter(ref cause) => cause,
            ListIdentityProvidersError::NotAuthorized(ref cause) => cause,
            ListIdentityProvidersError::ResourceNotFound(ref cause) => cause,
            ListIdentityProvidersError::TooManyRequests(ref cause) => cause,
            ListIdentityProvidersError::Validation(ref cause) => cause,
            ListIdentityProvidersError::Credentials(ref err) => err.description(),
            ListIdentityProvidersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListIdentityProvidersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListUserImportJobs
#[derive(Debug, PartialEq)]
pub enum ListUserImportJobsError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListUserImportJobsError {
    pub fn from_body(body: &str) -> ListUserImportJobsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        ListUserImportJobsError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        ListUserImportJobsError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        ListUserImportJobsError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListUserImportJobsError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        ListUserImportJobsError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListUserImportJobsError::Validation(error_message.to_string())
                    }
                    _ => ListUserImportJobsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListUserImportJobsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListUserImportJobsError {
    fn from(err: serde_json::error::Error) -> ListUserImportJobsError {
        ListUserImportJobsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListUserImportJobsError {
    fn from(err: CredentialsError) -> ListUserImportJobsError {
        ListUserImportJobsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListUserImportJobsError {
    fn from(err: HttpDispatchError) -> ListUserImportJobsError {
        ListUserImportJobsError::HttpDispatch(err)
    }
}
impl fmt::Display for ListUserImportJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListUserImportJobsError {
    fn description(&self) -> &str {
        match *self {
            ListUserImportJobsError::InternalError(ref cause) => cause,
            ListUserImportJobsError::InvalidParameter(ref cause) => cause,
            ListUserImportJobsError::NotAuthorized(ref cause) => cause,
            ListUserImportJobsError::ResourceNotFound(ref cause) => cause,
            ListUserImportJobsError::TooManyRequests(ref cause) => cause,
            ListUserImportJobsError::Validation(ref cause) => cause,
            ListUserImportJobsError::Credentials(ref err) => err.description(),
            ListUserImportJobsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListUserImportJobsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListUserPoolClients
#[derive(Debug, PartialEq)]
pub enum ListUserPoolClientsError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListUserPoolClientsError {
    pub fn from_body(body: &str) -> ListUserPoolClientsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        ListUserPoolClientsError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        ListUserPoolClientsError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        ListUserPoolClientsError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListUserPoolClientsError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        ListUserPoolClientsError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListUserPoolClientsError::Validation(error_message.to_string())
                    }
                    _ => ListUserPoolClientsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListUserPoolClientsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListUserPoolClientsError {
    fn from(err: serde_json::error::Error) -> ListUserPoolClientsError {
        ListUserPoolClientsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListUserPoolClientsError {
    fn from(err: CredentialsError) -> ListUserPoolClientsError {
        ListUserPoolClientsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListUserPoolClientsError {
    fn from(err: HttpDispatchError) -> ListUserPoolClientsError {
        ListUserPoolClientsError::HttpDispatch(err)
    }
}
impl fmt::Display for ListUserPoolClientsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListUserPoolClientsError {
    fn description(&self) -> &str {
        match *self {
            ListUserPoolClientsError::InternalError(ref cause) => cause,
            ListUserPoolClientsError::InvalidParameter(ref cause) => cause,
            ListUserPoolClientsError::NotAuthorized(ref cause) => cause,
            ListUserPoolClientsError::ResourceNotFound(ref cause) => cause,
            ListUserPoolClientsError::TooManyRequests(ref cause) => cause,
            ListUserPoolClientsError::Validation(ref cause) => cause,
            ListUserPoolClientsError::Credentials(ref err) => err.description(),
            ListUserPoolClientsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListUserPoolClientsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListUserPools
#[derive(Debug, PartialEq)]
pub enum ListUserPoolsError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListUserPoolsError {
    pub fn from_body(body: &str) -> ListUserPoolsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        ListUserPoolsError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        ListUserPoolsError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        ListUserPoolsError::NotAuthorized(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        ListUserPoolsError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListUserPoolsError::Validation(error_message.to_string())
                    }
                    _ => ListUserPoolsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListUserPoolsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListUserPoolsError {
    fn from(err: serde_json::error::Error) -> ListUserPoolsError {
        ListUserPoolsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListUserPoolsError {
    fn from(err: CredentialsError) -> ListUserPoolsError {
        ListUserPoolsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListUserPoolsError {
    fn from(err: HttpDispatchError) -> ListUserPoolsError {
        ListUserPoolsError::HttpDispatch(err)
    }
}
impl fmt::Display for ListUserPoolsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListUserPoolsError {
    fn description(&self) -> &str {
        match *self {
            ListUserPoolsError::InternalError(ref cause) => cause,
            ListUserPoolsError::InvalidParameter(ref cause) => cause,
            ListUserPoolsError::NotAuthorized(ref cause) => cause,
            ListUserPoolsError::TooManyRequests(ref cause) => cause,
            ListUserPoolsError::Validation(ref cause) => cause,
            ListUserPoolsError::Credentials(ref err) => err.description(),
            ListUserPoolsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListUserPoolsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListUsers
#[derive(Debug, PartialEq)]
pub enum ListUsersError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListUsersError {
    pub fn from_body(body: &str) -> ListUsersError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        ListUsersError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        ListUsersError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        ListUsersError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListUsersError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        ListUsersError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => ListUsersError::Validation(error_message.to_string()),
                    _ => ListUsersError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListUsersError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListUsersError {
    fn from(err: serde_json::error::Error) -> ListUsersError {
        ListUsersError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListUsersError {
    fn from(err: CredentialsError) -> ListUsersError {
        ListUsersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListUsersError {
    fn from(err: HttpDispatchError) -> ListUsersError {
        ListUsersError::HttpDispatch(err)
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
            ListUsersError::InternalError(ref cause) => cause,
            ListUsersError::InvalidParameter(ref cause) => cause,
            ListUsersError::NotAuthorized(ref cause) => cause,
            ListUsersError::ResourceNotFound(ref cause) => cause,
            ListUsersError::TooManyRequests(ref cause) => cause,
            ListUsersError::Validation(ref cause) => cause,
            ListUsersError::Credentials(ref err) => err.description(),
            ListUsersError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListUsersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListUsersInGroup
#[derive(Debug, PartialEq)]
pub enum ListUsersInGroupError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListUsersInGroupError {
    pub fn from_body(body: &str) -> ListUsersInGroupError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        ListUsersInGroupError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        ListUsersInGroupError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        ListUsersInGroupError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListUsersInGroupError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        ListUsersInGroupError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListUsersInGroupError::Validation(error_message.to_string())
                    }
                    _ => ListUsersInGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListUsersInGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListUsersInGroupError {
    fn from(err: serde_json::error::Error) -> ListUsersInGroupError {
        ListUsersInGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListUsersInGroupError {
    fn from(err: CredentialsError) -> ListUsersInGroupError {
        ListUsersInGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListUsersInGroupError {
    fn from(err: HttpDispatchError) -> ListUsersInGroupError {
        ListUsersInGroupError::HttpDispatch(err)
    }
}
impl fmt::Display for ListUsersInGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListUsersInGroupError {
    fn description(&self) -> &str {
        match *self {
            ListUsersInGroupError::InternalError(ref cause) => cause,
            ListUsersInGroupError::InvalidParameter(ref cause) => cause,
            ListUsersInGroupError::NotAuthorized(ref cause) => cause,
            ListUsersInGroupError::ResourceNotFound(ref cause) => cause,
            ListUsersInGroupError::TooManyRequests(ref cause) => cause,
            ListUsersInGroupError::Validation(ref cause) => cause,
            ListUsersInGroupError::Credentials(ref err) => err.description(),
            ListUsersInGroupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListUsersInGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ResendConfirmationCode
#[derive(Debug, PartialEq)]
pub enum ResendConfirmationCodeError {
    ///<p>This exception is thrown when a verification code fails to deliver successfully.</p>
    CodeDeliveryFailure(String),
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when Amazon Cognito is not allowed to use your email identity. HTTP status code: 400.</p>
    InvalidEmailRoleAccessPolicy(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid AWS Lambda response.</p>
    InvalidLambdaResponse(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is returned when the role provided for SMS configuration does not have permission to publish using Amazon SNS.</p>
    InvalidSmsRoleAccessPolicy(String),
    ///<p>This exception is thrown when the trust relationship is invalid for the role provided for SMS configuration. This can happen if you do not trust <b>cognito-idp.amazonaws.com</b> or the external ID provided in the role does not match what is provided in the SMS configuration for the user pool.</p>
    InvalidSmsRoleTrustRelationship(String),
    ///<p>This exception is thrown when a user exceeds the limit for a requested AWS resource.</p>
    LimitExceeded(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an unexpected exception with the AWS Lambda service.</p>
    UnexpectedLambda(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters a user validation exception with the AWS Lambda service.</p>
    UserLambdaValidation(String),
    ///<p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ResendConfirmationCodeError {
    pub fn from_body(body: &str) -> ResendConfirmationCodeError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "CodeDeliveryFailureException" => ResendConfirmationCodeError::CodeDeliveryFailure(String::from(error_message)),
                    "InternalErrorException" => {
                        ResendConfirmationCodeError::InternalError(String::from(error_message))
                    }
                    "InvalidEmailRoleAccessPolicyException" => ResendConfirmationCodeError::InvalidEmailRoleAccessPolicy(String::from(error_message)),
                    "InvalidLambdaResponseException" => ResendConfirmationCodeError::InvalidLambdaResponse(String::from(error_message)),
                    "InvalidParameterException" => {
                        ResendConfirmationCodeError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidSmsRoleAccessPolicyException" => ResendConfirmationCodeError::InvalidSmsRoleAccessPolicy(String::from(error_message)),
                    "InvalidSmsRoleTrustRelationshipException" => ResendConfirmationCodeError::InvalidSmsRoleTrustRelationship(String::from(error_message)),
                    "LimitExceededException" => {
                        ResendConfirmationCodeError::LimitExceeded(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        ResendConfirmationCodeError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ResendConfirmationCodeError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        ResendConfirmationCodeError::TooManyRequests(String::from(error_message))
                    }
                    "UnexpectedLambdaException" => {
                        ResendConfirmationCodeError::UnexpectedLambda(String::from(error_message))
                    }
                    "UserLambdaValidationException" => ResendConfirmationCodeError::UserLambdaValidation(String::from(error_message)),
                    "UserNotFoundException" => {
                        ResendConfirmationCodeError::UserNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        ResendConfirmationCodeError::Validation(error_message.to_string())
                    }
                    _ => ResendConfirmationCodeError::Unknown(String::from(body)),
                }
            }
            Err(_) => ResendConfirmationCodeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ResendConfirmationCodeError {
    fn from(err: serde_json::error::Error) -> ResendConfirmationCodeError {
        ResendConfirmationCodeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ResendConfirmationCodeError {
    fn from(err: CredentialsError) -> ResendConfirmationCodeError {
        ResendConfirmationCodeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ResendConfirmationCodeError {
    fn from(err: HttpDispatchError) -> ResendConfirmationCodeError {
        ResendConfirmationCodeError::HttpDispatch(err)
    }
}
impl fmt::Display for ResendConfirmationCodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ResendConfirmationCodeError {
    fn description(&self) -> &str {
        match *self {
            ResendConfirmationCodeError::CodeDeliveryFailure(ref cause) => cause,
            ResendConfirmationCodeError::InternalError(ref cause) => cause,
            ResendConfirmationCodeError::InvalidEmailRoleAccessPolicy(ref cause) => cause,
            ResendConfirmationCodeError::InvalidLambdaResponse(ref cause) => cause,
            ResendConfirmationCodeError::InvalidParameter(ref cause) => cause,
            ResendConfirmationCodeError::InvalidSmsRoleAccessPolicy(ref cause) => cause,
            ResendConfirmationCodeError::InvalidSmsRoleTrustRelationship(ref cause) => cause,
            ResendConfirmationCodeError::LimitExceeded(ref cause) => cause,
            ResendConfirmationCodeError::NotAuthorized(ref cause) => cause,
            ResendConfirmationCodeError::ResourceNotFound(ref cause) => cause,
            ResendConfirmationCodeError::TooManyRequests(ref cause) => cause,
            ResendConfirmationCodeError::UnexpectedLambda(ref cause) => cause,
            ResendConfirmationCodeError::UserLambdaValidation(ref cause) => cause,
            ResendConfirmationCodeError::UserNotFound(ref cause) => cause,
            ResendConfirmationCodeError::Validation(ref cause) => cause,
            ResendConfirmationCodeError::Credentials(ref err) => err.description(),
            ResendConfirmationCodeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ResendConfirmationCodeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RespondToAuthChallenge
#[derive(Debug, PartialEq)]
pub enum RespondToAuthChallengeError {
    ///<p>This exception is thrown when a user tries to confirm the account with an email or phone number that has already been supplied as an alias from a different account. This exception tells user that an account with this email or phone already exists.</p>
    AliasExists(String),
    ///<p>This exception is thrown if the provided code does not match what the server was expecting.</p>
    CodeMismatch(String),
    ///<p>This exception is thrown if a code has expired.</p>
    ExpiredCode(String),
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid AWS Lambda response.</p>
    InvalidLambdaResponse(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid password.</p>
    InvalidPassword(String),
    ///<p>This exception is returned when the role provided for SMS configuration does not have permission to publish using Amazon SNS.</p>
    InvalidSmsRoleAccessPolicy(String),
    ///<p>This exception is thrown when the trust relationship is invalid for the role provided for SMS configuration. This can happen if you do not trust <b>cognito-idp.amazonaws.com</b> or the external ID provided in the role does not match what is provided in the SMS configuration for the user pool.</p>
    InvalidSmsRoleTrustRelationship(String),
    ///<p>This exception is thrown when the user pool configuration is invalid.</p>
    InvalidUserPoolConfiguration(String),
    ///<p>This exception is thrown when Amazon Cognito cannot find a multi-factor authentication (MFA) method.</p>
    MFAMethodNotFound(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when a password reset is required.</p>
    PasswordResetRequired(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an unexpected exception with the AWS Lambda service.</p>
    UnexpectedLambda(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters a user validation exception with the AWS Lambda service.</p>
    UserLambdaValidation(String),
    ///<p>This exception is thrown when a user is not confirmed successfully.</p>
    UserNotConfirmed(String),
    ///<p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl RespondToAuthChallengeError {
    pub fn from_body(body: &str) -> RespondToAuthChallengeError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AliasExistsException" => {
                        RespondToAuthChallengeError::AliasExists(String::from(error_message))
                    }
                    "CodeMismatchException" => {
                        RespondToAuthChallengeError::CodeMismatch(String::from(error_message))
                    }
                    "ExpiredCodeException" => {
                        RespondToAuthChallengeError::ExpiredCode(String::from(error_message))
                    }
                    "InternalErrorException" => {
                        RespondToAuthChallengeError::InternalError(String::from(error_message))
                    }
                    "InvalidLambdaResponseException" => RespondToAuthChallengeError::InvalidLambdaResponse(String::from(error_message)),
                    "InvalidParameterException" => {
                        RespondToAuthChallengeError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidPasswordException" => {
                        RespondToAuthChallengeError::InvalidPassword(String::from(error_message))
                    }
                    "InvalidSmsRoleAccessPolicyException" => RespondToAuthChallengeError::InvalidSmsRoleAccessPolicy(String::from(error_message)),
                    "InvalidSmsRoleTrustRelationshipException" => RespondToAuthChallengeError::InvalidSmsRoleTrustRelationship(String::from(error_message)),
                    "InvalidUserPoolConfigurationException" => RespondToAuthChallengeError::InvalidUserPoolConfiguration(String::from(error_message)),
                    "MFAMethodNotFoundException" => {
                        RespondToAuthChallengeError::MFAMethodNotFound(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        RespondToAuthChallengeError::NotAuthorized(String::from(error_message))
                    }
                    "PasswordResetRequiredException" => RespondToAuthChallengeError::PasswordResetRequired(String::from(error_message)),
                    "ResourceNotFoundException" => {
                        RespondToAuthChallengeError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        RespondToAuthChallengeError::TooManyRequests(String::from(error_message))
                    }
                    "UnexpectedLambdaException" => {
                        RespondToAuthChallengeError::UnexpectedLambda(String::from(error_message))
                    }
                    "UserLambdaValidationException" => RespondToAuthChallengeError::UserLambdaValidation(String::from(error_message)),
                    "UserNotConfirmedException" => {
                        RespondToAuthChallengeError::UserNotConfirmed(String::from(error_message))
                    }
                    "UserNotFoundException" => {
                        RespondToAuthChallengeError::UserNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        RespondToAuthChallengeError::Validation(error_message.to_string())
                    }
                    _ => RespondToAuthChallengeError::Unknown(String::from(body)),
                }
            }
            Err(_) => RespondToAuthChallengeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RespondToAuthChallengeError {
    fn from(err: serde_json::error::Error) -> RespondToAuthChallengeError {
        RespondToAuthChallengeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RespondToAuthChallengeError {
    fn from(err: CredentialsError) -> RespondToAuthChallengeError {
        RespondToAuthChallengeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RespondToAuthChallengeError {
    fn from(err: HttpDispatchError) -> RespondToAuthChallengeError {
        RespondToAuthChallengeError::HttpDispatch(err)
    }
}
impl fmt::Display for RespondToAuthChallengeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RespondToAuthChallengeError {
    fn description(&self) -> &str {
        match *self {
            RespondToAuthChallengeError::AliasExists(ref cause) => cause,
            RespondToAuthChallengeError::CodeMismatch(ref cause) => cause,
            RespondToAuthChallengeError::ExpiredCode(ref cause) => cause,
            RespondToAuthChallengeError::InternalError(ref cause) => cause,
            RespondToAuthChallengeError::InvalidLambdaResponse(ref cause) => cause,
            RespondToAuthChallengeError::InvalidParameter(ref cause) => cause,
            RespondToAuthChallengeError::InvalidPassword(ref cause) => cause,
            RespondToAuthChallengeError::InvalidSmsRoleAccessPolicy(ref cause) => cause,
            RespondToAuthChallengeError::InvalidSmsRoleTrustRelationship(ref cause) => cause,
            RespondToAuthChallengeError::InvalidUserPoolConfiguration(ref cause) => cause,
            RespondToAuthChallengeError::MFAMethodNotFound(ref cause) => cause,
            RespondToAuthChallengeError::NotAuthorized(ref cause) => cause,
            RespondToAuthChallengeError::PasswordResetRequired(ref cause) => cause,
            RespondToAuthChallengeError::ResourceNotFound(ref cause) => cause,
            RespondToAuthChallengeError::TooManyRequests(ref cause) => cause,
            RespondToAuthChallengeError::UnexpectedLambda(ref cause) => cause,
            RespondToAuthChallengeError::UserLambdaValidation(ref cause) => cause,
            RespondToAuthChallengeError::UserNotConfirmed(ref cause) => cause,
            RespondToAuthChallengeError::UserNotFound(ref cause) => cause,
            RespondToAuthChallengeError::Validation(ref cause) => cause,
            RespondToAuthChallengeError::Credentials(ref err) => err.description(),
            RespondToAuthChallengeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RespondToAuthChallengeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetUserSettings
#[derive(Debug, PartialEq)]
pub enum SetUserSettingsError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when a password reset is required.</p>
    PasswordResetRequired(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when a user is not confirmed successfully.</p>
    UserNotConfirmed(String),
    ///<p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl SetUserSettingsError {
    pub fn from_body(body: &str) -> SetUserSettingsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        SetUserSettingsError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        SetUserSettingsError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        SetUserSettingsError::NotAuthorized(String::from(error_message))
                    }
                    "PasswordResetRequiredException" => {
                        SetUserSettingsError::PasswordResetRequired(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        SetUserSettingsError::ResourceNotFound(String::from(error_message))
                    }
                    "UserNotConfirmedException" => {
                        SetUserSettingsError::UserNotConfirmed(String::from(error_message))
                    }
                    "UserNotFoundException" => {
                        SetUserSettingsError::UserNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        SetUserSettingsError::Validation(error_message.to_string())
                    }
                    _ => SetUserSettingsError::Unknown(String::from(body)),
                }
            }
            Err(_) => SetUserSettingsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SetUserSettingsError {
    fn from(err: serde_json::error::Error) -> SetUserSettingsError {
        SetUserSettingsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SetUserSettingsError {
    fn from(err: CredentialsError) -> SetUserSettingsError {
        SetUserSettingsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetUserSettingsError {
    fn from(err: HttpDispatchError) -> SetUserSettingsError {
        SetUserSettingsError::HttpDispatch(err)
    }
}
impl fmt::Display for SetUserSettingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetUserSettingsError {
    fn description(&self) -> &str {
        match *self {
            SetUserSettingsError::InternalError(ref cause) => cause,
            SetUserSettingsError::InvalidParameter(ref cause) => cause,
            SetUserSettingsError::NotAuthorized(ref cause) => cause,
            SetUserSettingsError::PasswordResetRequired(ref cause) => cause,
            SetUserSettingsError::ResourceNotFound(ref cause) => cause,
            SetUserSettingsError::UserNotConfirmed(ref cause) => cause,
            SetUserSettingsError::UserNotFound(ref cause) => cause,
            SetUserSettingsError::Validation(ref cause) => cause,
            SetUserSettingsError::Credentials(ref err) => err.description(),
            SetUserSettingsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            SetUserSettingsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SignUp
#[derive(Debug, PartialEq)]
pub enum SignUpError {
    ///<p>This exception is thrown when a verification code fails to deliver successfully.</p>
    CodeDeliveryFailure(String),
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when Amazon Cognito is not allowed to use your email identity. HTTP status code: 400.</p>
    InvalidEmailRoleAccessPolicy(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid AWS Lambda response.</p>
    InvalidLambdaResponse(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid password.</p>
    InvalidPassword(String),
    ///<p>This exception is returned when the role provided for SMS configuration does not have permission to publish using Amazon SNS.</p>
    InvalidSmsRoleAccessPolicy(String),
    ///<p>This exception is thrown when the trust relationship is invalid for the role provided for SMS configuration. This can happen if you do not trust <b>cognito-idp.amazonaws.com</b> or the external ID provided in the role does not match what is provided in the SMS configuration for the user pool.</p>
    InvalidSmsRoleTrustRelationship(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an unexpected exception with the AWS Lambda service.</p>
    UnexpectedLambda(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters a user validation exception with the AWS Lambda service.</p>
    UserLambdaValidation(String),
    ///<p>This exception is thrown when Amazon Cognito encounters a user name that already exists in the user pool.</p>
    UsernameExists(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl SignUpError {
    pub fn from_body(body: &str) -> SignUpError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "CodeDeliveryFailureException" => {
                        SignUpError::CodeDeliveryFailure(String::from(error_message))
                    }
                    "InternalErrorException" => {
                        SignUpError::InternalError(String::from(error_message))
                    }
                    "InvalidEmailRoleAccessPolicyException" => {
                        SignUpError::InvalidEmailRoleAccessPolicy(String::from(error_message))
                    }
                    "InvalidLambdaResponseException" => {
                        SignUpError::InvalidLambdaResponse(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        SignUpError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidPasswordException" => {
                        SignUpError::InvalidPassword(String::from(error_message))
                    }
                    "InvalidSmsRoleAccessPolicyException" => {
                        SignUpError::InvalidSmsRoleAccessPolicy(String::from(error_message))
                    }
                    "InvalidSmsRoleTrustRelationshipException" => {
                        SignUpError::InvalidSmsRoleTrustRelationship(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        SignUpError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        SignUpError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        SignUpError::TooManyRequests(String::from(error_message))
                    }
                    "UnexpectedLambdaException" => {
                        SignUpError::UnexpectedLambda(String::from(error_message))
                    }
                    "UserLambdaValidationException" => {
                        SignUpError::UserLambdaValidation(String::from(error_message))
                    }
                    "UsernameExistsException" => {
                        SignUpError::UsernameExists(String::from(error_message))
                    }
                    "ValidationException" => SignUpError::Validation(error_message.to_string()),
                    _ => SignUpError::Unknown(String::from(body)),
                }
            }
            Err(_) => SignUpError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SignUpError {
    fn from(err: serde_json::error::Error) -> SignUpError {
        SignUpError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SignUpError {
    fn from(err: CredentialsError) -> SignUpError {
        SignUpError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SignUpError {
    fn from(err: HttpDispatchError) -> SignUpError {
        SignUpError::HttpDispatch(err)
    }
}
impl fmt::Display for SignUpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SignUpError {
    fn description(&self) -> &str {
        match *self {
            SignUpError::CodeDeliveryFailure(ref cause) => cause,
            SignUpError::InternalError(ref cause) => cause,
            SignUpError::InvalidEmailRoleAccessPolicy(ref cause) => cause,
            SignUpError::InvalidLambdaResponse(ref cause) => cause,
            SignUpError::InvalidParameter(ref cause) => cause,
            SignUpError::InvalidPassword(ref cause) => cause,
            SignUpError::InvalidSmsRoleAccessPolicy(ref cause) => cause,
            SignUpError::InvalidSmsRoleTrustRelationship(ref cause) => cause,
            SignUpError::NotAuthorized(ref cause) => cause,
            SignUpError::ResourceNotFound(ref cause) => cause,
            SignUpError::TooManyRequests(ref cause) => cause,
            SignUpError::UnexpectedLambda(ref cause) => cause,
            SignUpError::UserLambdaValidation(ref cause) => cause,
            SignUpError::UsernameExists(ref cause) => cause,
            SignUpError::Validation(ref cause) => cause,
            SignUpError::Credentials(ref err) => err.description(),
            SignUpError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            SignUpError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StartUserImportJob
#[derive(Debug, PartialEq)]
pub enum StartUserImportJobError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when a precondition is not met.</p>
    PreconditionNotMet(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl StartUserImportJobError {
    pub fn from_body(body: &str) -> StartUserImportJobError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        StartUserImportJobError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        StartUserImportJobError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        StartUserImportJobError::NotAuthorized(String::from(error_message))
                    }
                    "PreconditionNotMetException" => {
                        StartUserImportJobError::PreconditionNotMet(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        StartUserImportJobError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        StartUserImportJobError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        StartUserImportJobError::Validation(error_message.to_string())
                    }
                    _ => StartUserImportJobError::Unknown(String::from(body)),
                }
            }
            Err(_) => StartUserImportJobError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StartUserImportJobError {
    fn from(err: serde_json::error::Error) -> StartUserImportJobError {
        StartUserImportJobError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StartUserImportJobError {
    fn from(err: CredentialsError) -> StartUserImportJobError {
        StartUserImportJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartUserImportJobError {
    fn from(err: HttpDispatchError) -> StartUserImportJobError {
        StartUserImportJobError::HttpDispatch(err)
    }
}
impl fmt::Display for StartUserImportJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartUserImportJobError {
    fn description(&self) -> &str {
        match *self {
            StartUserImportJobError::InternalError(ref cause) => cause,
            StartUserImportJobError::InvalidParameter(ref cause) => cause,
            StartUserImportJobError::NotAuthorized(ref cause) => cause,
            StartUserImportJobError::PreconditionNotMet(ref cause) => cause,
            StartUserImportJobError::ResourceNotFound(ref cause) => cause,
            StartUserImportJobError::TooManyRequests(ref cause) => cause,
            StartUserImportJobError::Validation(ref cause) => cause,
            StartUserImportJobError::Credentials(ref err) => err.description(),
            StartUserImportJobError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StartUserImportJobError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StopUserImportJob
#[derive(Debug, PartialEq)]
pub enum StopUserImportJobError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when a precondition is not met.</p>
    PreconditionNotMet(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl StopUserImportJobError {
    pub fn from_body(body: &str) -> StopUserImportJobError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        StopUserImportJobError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        StopUserImportJobError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        StopUserImportJobError::NotAuthorized(String::from(error_message))
                    }
                    "PreconditionNotMetException" => {
                        StopUserImportJobError::PreconditionNotMet(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        StopUserImportJobError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        StopUserImportJobError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        StopUserImportJobError::Validation(error_message.to_string())
                    }
                    _ => StopUserImportJobError::Unknown(String::from(body)),
                }
            }
            Err(_) => StopUserImportJobError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StopUserImportJobError {
    fn from(err: serde_json::error::Error) -> StopUserImportJobError {
        StopUserImportJobError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StopUserImportJobError {
    fn from(err: CredentialsError) -> StopUserImportJobError {
        StopUserImportJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopUserImportJobError {
    fn from(err: HttpDispatchError) -> StopUserImportJobError {
        StopUserImportJobError::HttpDispatch(err)
    }
}
impl fmt::Display for StopUserImportJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopUserImportJobError {
    fn description(&self) -> &str {
        match *self {
            StopUserImportJobError::InternalError(ref cause) => cause,
            StopUserImportJobError::InvalidParameter(ref cause) => cause,
            StopUserImportJobError::NotAuthorized(ref cause) => cause,
            StopUserImportJobError::PreconditionNotMet(ref cause) => cause,
            StopUserImportJobError::ResourceNotFound(ref cause) => cause,
            StopUserImportJobError::TooManyRequests(ref cause) => cause,
            StopUserImportJobError::Validation(ref cause) => cause,
            StopUserImportJobError::Credentials(ref err) => err.description(),
            StopUserImportJobError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StopUserImportJobError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateDeviceStatus
#[derive(Debug, PartialEq)]
pub enum UpdateDeviceStatusError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when the user pool configuration is invalid.</p>
    InvalidUserPoolConfiguration(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when a password reset is required.</p>
    PasswordResetRequired(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    ///<p>This exception is thrown when a user is not confirmed successfully.</p>
    UserNotConfirmed(String),
    ///<p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateDeviceStatusError {
    pub fn from_body(body: &str) -> UpdateDeviceStatusError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        UpdateDeviceStatusError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        UpdateDeviceStatusError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidUserPoolConfigurationException" => UpdateDeviceStatusError::InvalidUserPoolConfiguration(String::from(error_message)),
                    "NotAuthorizedException" => {
                        UpdateDeviceStatusError::NotAuthorized(String::from(error_message))
                    }
                    "PasswordResetRequiredException" => {
                        UpdateDeviceStatusError::PasswordResetRequired(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateDeviceStatusError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        UpdateDeviceStatusError::TooManyRequests(String::from(error_message))
                    }
                    "UserNotConfirmedException" => {
                        UpdateDeviceStatusError::UserNotConfirmed(String::from(error_message))
                    }
                    "UserNotFoundException" => {
                        UpdateDeviceStatusError::UserNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateDeviceStatusError::Validation(error_message.to_string())
                    }
                    _ => UpdateDeviceStatusError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateDeviceStatusError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateDeviceStatusError {
    fn from(err: serde_json::error::Error) -> UpdateDeviceStatusError {
        UpdateDeviceStatusError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateDeviceStatusError {
    fn from(err: CredentialsError) -> UpdateDeviceStatusError {
        UpdateDeviceStatusError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateDeviceStatusError {
    fn from(err: HttpDispatchError) -> UpdateDeviceStatusError {
        UpdateDeviceStatusError::HttpDispatch(err)
    }
}
impl fmt::Display for UpdateDeviceStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDeviceStatusError {
    fn description(&self) -> &str {
        match *self {
            UpdateDeviceStatusError::InternalError(ref cause) => cause,
            UpdateDeviceStatusError::InvalidParameter(ref cause) => cause,
            UpdateDeviceStatusError::InvalidUserPoolConfiguration(ref cause) => cause,
            UpdateDeviceStatusError::NotAuthorized(ref cause) => cause,
            UpdateDeviceStatusError::PasswordResetRequired(ref cause) => cause,
            UpdateDeviceStatusError::ResourceNotFound(ref cause) => cause,
            UpdateDeviceStatusError::TooManyRequests(ref cause) => cause,
            UpdateDeviceStatusError::UserNotConfirmed(ref cause) => cause,
            UpdateDeviceStatusError::UserNotFound(ref cause) => cause,
            UpdateDeviceStatusError::Validation(ref cause) => cause,
            UpdateDeviceStatusError::Credentials(ref err) => err.description(),
            UpdateDeviceStatusError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateDeviceStatusError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateGroup
#[derive(Debug, PartialEq)]
pub enum UpdateGroupError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateGroupError {
    pub fn from_body(body: &str) -> UpdateGroupError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        UpdateGroupError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        UpdateGroupError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        UpdateGroupError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateGroupError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        UpdateGroupError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateGroupError::Validation(error_message.to_string())
                    }
                    _ => UpdateGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateGroupError {
    fn from(err: serde_json::error::Error) -> UpdateGroupError {
        UpdateGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateGroupError {
    fn from(err: CredentialsError) -> UpdateGroupError {
        UpdateGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateGroupError {
    fn from(err: HttpDispatchError) -> UpdateGroupError {
        UpdateGroupError::HttpDispatch(err)
    }
}
impl fmt::Display for UpdateGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateGroupError {
    fn description(&self) -> &str {
        match *self {
            UpdateGroupError::InternalError(ref cause) => cause,
            UpdateGroupError::InvalidParameter(ref cause) => cause,
            UpdateGroupError::NotAuthorized(ref cause) => cause,
            UpdateGroupError::ResourceNotFound(ref cause) => cause,
            UpdateGroupError::TooManyRequests(ref cause) => cause,
            UpdateGroupError::Validation(ref cause) => cause,
            UpdateGroupError::Credentials(ref err) => err.description(),
            UpdateGroupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateIdentityProvider
#[derive(Debug, PartialEq)]
pub enum UpdateIdentityProviderError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    ///<p>This exception is thrown when the specified identifier is not supported.</p>
    UnsupportedIdentityProvider(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateIdentityProviderError {
    pub fn from_body(body: &str) -> UpdateIdentityProviderError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        UpdateIdentityProviderError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        UpdateIdentityProviderError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        UpdateIdentityProviderError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateIdentityProviderError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        UpdateIdentityProviderError::TooManyRequests(String::from(error_message))
                    }
                    "UnsupportedIdentityProviderException" => UpdateIdentityProviderError::UnsupportedIdentityProvider(String::from(error_message)),
                    "ValidationException" => {
                        UpdateIdentityProviderError::Validation(error_message.to_string())
                    }
                    _ => UpdateIdentityProviderError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateIdentityProviderError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateIdentityProviderError {
    fn from(err: serde_json::error::Error) -> UpdateIdentityProviderError {
        UpdateIdentityProviderError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateIdentityProviderError {
    fn from(err: CredentialsError) -> UpdateIdentityProviderError {
        UpdateIdentityProviderError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateIdentityProviderError {
    fn from(err: HttpDispatchError) -> UpdateIdentityProviderError {
        UpdateIdentityProviderError::HttpDispatch(err)
    }
}
impl fmt::Display for UpdateIdentityProviderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateIdentityProviderError {
    fn description(&self) -> &str {
        match *self {
            UpdateIdentityProviderError::InternalError(ref cause) => cause,
            UpdateIdentityProviderError::InvalidParameter(ref cause) => cause,
            UpdateIdentityProviderError::NotAuthorized(ref cause) => cause,
            UpdateIdentityProviderError::ResourceNotFound(ref cause) => cause,
            UpdateIdentityProviderError::TooManyRequests(ref cause) => cause,
            UpdateIdentityProviderError::UnsupportedIdentityProvider(ref cause) => cause,
            UpdateIdentityProviderError::Validation(ref cause) => cause,
            UpdateIdentityProviderError::Credentials(ref err) => err.description(),
            UpdateIdentityProviderError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateIdentityProviderError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateUserAttributes
#[derive(Debug, PartialEq)]
pub enum UpdateUserAttributesError {
    ///<p>This exception is thrown when a user tries to confirm the account with an email or phone number that has already been supplied as an alias from a different account. This exception tells user that an account with this email or phone already exists.</p>
    AliasExists(String),
    ///<p>This exception is thrown when a verification code fails to deliver successfully.</p>
    CodeDeliveryFailure(String),
    ///<p>This exception is thrown if the provided code does not match what the server was expecting.</p>
    CodeMismatch(String),
    ///<p>This exception is thrown if a code has expired.</p>
    ExpiredCode(String),
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when Amazon Cognito is not allowed to use your email identity. HTTP status code: 400.</p>
    InvalidEmailRoleAccessPolicy(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid AWS Lambda response.</p>
    InvalidLambdaResponse(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is returned when the role provided for SMS configuration does not have permission to publish using Amazon SNS.</p>
    InvalidSmsRoleAccessPolicy(String),
    ///<p>This exception is thrown when the trust relationship is invalid for the role provided for SMS configuration. This can happen if you do not trust <b>cognito-idp.amazonaws.com</b> or the external ID provided in the role does not match what is provided in the SMS configuration for the user pool.</p>
    InvalidSmsRoleTrustRelationship(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when a password reset is required.</p>
    PasswordResetRequired(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an unexpected exception with the AWS Lambda service.</p>
    UnexpectedLambda(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters a user validation exception with the AWS Lambda service.</p>
    UserLambdaValidation(String),
    ///<p>This exception is thrown when a user is not confirmed successfully.</p>
    UserNotConfirmed(String),
    ///<p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateUserAttributesError {
    pub fn from_body(body: &str) -> UpdateUserAttributesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AliasExistsException" => {
                        UpdateUserAttributesError::AliasExists(String::from(error_message))
                    }
                    "CodeDeliveryFailureException" => {
                        UpdateUserAttributesError::CodeDeliveryFailure(String::from(error_message))
                    }
                    "CodeMismatchException" => {
                        UpdateUserAttributesError::CodeMismatch(String::from(error_message))
                    }
                    "ExpiredCodeException" => {
                        UpdateUserAttributesError::ExpiredCode(String::from(error_message))
                    }
                    "InternalErrorException" => {
                        UpdateUserAttributesError::InternalError(String::from(error_message))
                    }
                    "InvalidEmailRoleAccessPolicyException" => UpdateUserAttributesError::InvalidEmailRoleAccessPolicy(String::from(error_message)),
                    "InvalidLambdaResponseException" => UpdateUserAttributesError::InvalidLambdaResponse(String::from(error_message)),
                    "InvalidParameterException" => {
                        UpdateUserAttributesError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidSmsRoleAccessPolicyException" => UpdateUserAttributesError::InvalidSmsRoleAccessPolicy(String::from(error_message)),
                    "InvalidSmsRoleTrustRelationshipException" => UpdateUserAttributesError::InvalidSmsRoleTrustRelationship(String::from(error_message)),
                    "NotAuthorizedException" => {
                        UpdateUserAttributesError::NotAuthorized(String::from(error_message))
                    }
                    "PasswordResetRequiredException" => UpdateUserAttributesError::PasswordResetRequired(String::from(error_message)),
                    "ResourceNotFoundException" => {
                        UpdateUserAttributesError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        UpdateUserAttributesError::TooManyRequests(String::from(error_message))
                    }
                    "UnexpectedLambdaException" => {
                        UpdateUserAttributesError::UnexpectedLambda(String::from(error_message))
                    }
                    "UserLambdaValidationException" => {
                        UpdateUserAttributesError::UserLambdaValidation(String::from(error_message))
                    }
                    "UserNotConfirmedException" => {
                        UpdateUserAttributesError::UserNotConfirmed(String::from(error_message))
                    }
                    "UserNotFoundException" => {
                        UpdateUserAttributesError::UserNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateUserAttributesError::Validation(error_message.to_string())
                    }
                    _ => UpdateUserAttributesError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateUserAttributesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateUserAttributesError {
    fn from(err: serde_json::error::Error) -> UpdateUserAttributesError {
        UpdateUserAttributesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateUserAttributesError {
    fn from(err: CredentialsError) -> UpdateUserAttributesError {
        UpdateUserAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateUserAttributesError {
    fn from(err: HttpDispatchError) -> UpdateUserAttributesError {
        UpdateUserAttributesError::HttpDispatch(err)
    }
}
impl fmt::Display for UpdateUserAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateUserAttributesError {
    fn description(&self) -> &str {
        match *self {
            UpdateUserAttributesError::AliasExists(ref cause) => cause,
            UpdateUserAttributesError::CodeDeliveryFailure(ref cause) => cause,
            UpdateUserAttributesError::CodeMismatch(ref cause) => cause,
            UpdateUserAttributesError::ExpiredCode(ref cause) => cause,
            UpdateUserAttributesError::InternalError(ref cause) => cause,
            UpdateUserAttributesError::InvalidEmailRoleAccessPolicy(ref cause) => cause,
            UpdateUserAttributesError::InvalidLambdaResponse(ref cause) => cause,
            UpdateUserAttributesError::InvalidParameter(ref cause) => cause,
            UpdateUserAttributesError::InvalidSmsRoleAccessPolicy(ref cause) => cause,
            UpdateUserAttributesError::InvalidSmsRoleTrustRelationship(ref cause) => cause,
            UpdateUserAttributesError::NotAuthorized(ref cause) => cause,
            UpdateUserAttributesError::PasswordResetRequired(ref cause) => cause,
            UpdateUserAttributesError::ResourceNotFound(ref cause) => cause,
            UpdateUserAttributesError::TooManyRequests(ref cause) => cause,
            UpdateUserAttributesError::UnexpectedLambda(ref cause) => cause,
            UpdateUserAttributesError::UserLambdaValidation(ref cause) => cause,
            UpdateUserAttributesError::UserNotConfirmed(ref cause) => cause,
            UpdateUserAttributesError::UserNotFound(ref cause) => cause,
            UpdateUserAttributesError::Validation(ref cause) => cause,
            UpdateUserAttributesError::Credentials(ref err) => err.description(),
            UpdateUserAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateUserAttributesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateUserPool
#[derive(Debug, PartialEq)]
pub enum UpdateUserPoolError {
    ///<p>This exception is thrown if two or more modifications are happening concurrently.</p>
    ConcurrentModification(String),
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when Amazon Cognito is not allowed to use your email identity. HTTP status code: 400.</p>
    InvalidEmailRoleAccessPolicy(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is returned when the role provided for SMS configuration does not have permission to publish using Amazon SNS.</p>
    InvalidSmsRoleAccessPolicy(String),
    ///<p>This exception is thrown when the trust relationship is invalid for the role provided for SMS configuration. This can happen if you do not trust <b>cognito-idp.amazonaws.com</b> or the external ID provided in the role does not match what is provided in the SMS configuration for the user pool.</p>
    InvalidSmsRoleTrustRelationship(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    ///<p>This exception is thrown when you are trying to modify a user pool while a user import job is in progress for that pool.</p>
    UserImportInProgress(String),
    ///<p>This exception is thrown when a user pool tag cannot be set or updated.</p>
    UserPoolTagging(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateUserPoolError {
    pub fn from_body(body: &str) -> UpdateUserPoolError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ConcurrentModificationException" => {
                        UpdateUserPoolError::ConcurrentModification(String::from(error_message))
                    }
                    "InternalErrorException" => {
                        UpdateUserPoolError::InternalError(String::from(error_message))
                    }
                    "InvalidEmailRoleAccessPolicyException" => UpdateUserPoolError::InvalidEmailRoleAccessPolicy(String::from(error_message)),
                    "InvalidParameterException" => {
                        UpdateUserPoolError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidSmsRoleAccessPolicyException" => {
                        UpdateUserPoolError::InvalidSmsRoleAccessPolicy(String::from(error_message))
                    }
                    "InvalidSmsRoleTrustRelationshipException" => UpdateUserPoolError::InvalidSmsRoleTrustRelationship(String::from(error_message)),
                    "NotAuthorizedException" => {
                        UpdateUserPoolError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateUserPoolError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        UpdateUserPoolError::TooManyRequests(String::from(error_message))
                    }
                    "UserImportInProgressException" => {
                        UpdateUserPoolError::UserImportInProgress(String::from(error_message))
                    }
                    "UserPoolTaggingException" => {
                        UpdateUserPoolError::UserPoolTagging(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateUserPoolError::Validation(error_message.to_string())
                    }
                    _ => UpdateUserPoolError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateUserPoolError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateUserPoolError {
    fn from(err: serde_json::error::Error) -> UpdateUserPoolError {
        UpdateUserPoolError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateUserPoolError {
    fn from(err: CredentialsError) -> UpdateUserPoolError {
        UpdateUserPoolError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateUserPoolError {
    fn from(err: HttpDispatchError) -> UpdateUserPoolError {
        UpdateUserPoolError::HttpDispatch(err)
    }
}
impl fmt::Display for UpdateUserPoolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateUserPoolError {
    fn description(&self) -> &str {
        match *self {
            UpdateUserPoolError::ConcurrentModification(ref cause) => cause,
            UpdateUserPoolError::InternalError(ref cause) => cause,
            UpdateUserPoolError::InvalidEmailRoleAccessPolicy(ref cause) => cause,
            UpdateUserPoolError::InvalidParameter(ref cause) => cause,
            UpdateUserPoolError::InvalidSmsRoleAccessPolicy(ref cause) => cause,
            UpdateUserPoolError::InvalidSmsRoleTrustRelationship(ref cause) => cause,
            UpdateUserPoolError::NotAuthorized(ref cause) => cause,
            UpdateUserPoolError::ResourceNotFound(ref cause) => cause,
            UpdateUserPoolError::TooManyRequests(ref cause) => cause,
            UpdateUserPoolError::UserImportInProgress(ref cause) => cause,
            UpdateUserPoolError::UserPoolTagging(ref cause) => cause,
            UpdateUserPoolError::Validation(ref cause) => cause,
            UpdateUserPoolError::Credentials(ref err) => err.description(),
            UpdateUserPoolError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateUserPoolError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateUserPoolClient
#[derive(Debug, PartialEq)]
pub enum UpdateUserPoolClientError {
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the specified OAuth flow is invalid.</p>
    InvalidOAuthFlow(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the specified scope does not exist.</p>
    ScopeDoesNotExist(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateUserPoolClientError {
    pub fn from_body(body: &str) -> UpdateUserPoolClientError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        UpdateUserPoolClientError::InternalError(String::from(error_message))
                    }
                    "InvalidOAuthFlowException" => {
                        UpdateUserPoolClientError::InvalidOAuthFlow(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        UpdateUserPoolClientError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        UpdateUserPoolClientError::NotAuthorized(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateUserPoolClientError::ResourceNotFound(String::from(error_message))
                    }
                    "ScopeDoesNotExistException" => {
                        UpdateUserPoolClientError::ScopeDoesNotExist(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        UpdateUserPoolClientError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateUserPoolClientError::Validation(error_message.to_string())
                    }
                    _ => UpdateUserPoolClientError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateUserPoolClientError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateUserPoolClientError {
    fn from(err: serde_json::error::Error) -> UpdateUserPoolClientError {
        UpdateUserPoolClientError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateUserPoolClientError {
    fn from(err: CredentialsError) -> UpdateUserPoolClientError {
        UpdateUserPoolClientError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateUserPoolClientError {
    fn from(err: HttpDispatchError) -> UpdateUserPoolClientError {
        UpdateUserPoolClientError::HttpDispatch(err)
    }
}
impl fmt::Display for UpdateUserPoolClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateUserPoolClientError {
    fn description(&self) -> &str {
        match *self {
            UpdateUserPoolClientError::InternalError(ref cause) => cause,
            UpdateUserPoolClientError::InvalidOAuthFlow(ref cause) => cause,
            UpdateUserPoolClientError::InvalidParameter(ref cause) => cause,
            UpdateUserPoolClientError::NotAuthorized(ref cause) => cause,
            UpdateUserPoolClientError::ResourceNotFound(ref cause) => cause,
            UpdateUserPoolClientError::ScopeDoesNotExist(ref cause) => cause,
            UpdateUserPoolClientError::TooManyRequests(ref cause) => cause,
            UpdateUserPoolClientError::Validation(ref cause) => cause,
            UpdateUserPoolClientError::Credentials(ref err) => err.description(),
            UpdateUserPoolClientError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateUserPoolClientError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by VerifyUserAttribute
#[derive(Debug, PartialEq)]
pub enum VerifyUserAttributeError {
    ///<p>This exception is thrown if the provided code does not match what the server was expecting.</p>
    CodeMismatch(String),
    ///<p>This exception is thrown if a code has expired.</p>
    ExpiredCode(String),
    ///<p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    ///<p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    ///<p>This exception is thrown when a user exceeds the limit for a requested AWS resource.</p>
    LimitExceeded(String),
    ///<p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    ///<p>This exception is thrown when a password reset is required.</p>
    PasswordResetRequired(String),
    ///<p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    ///<p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    ///<p>This exception is thrown when a user is not confirmed successfully.</p>
    UserNotConfirmed(String),
    ///<p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl VerifyUserAttributeError {
    pub fn from_body(body: &str) -> VerifyUserAttributeError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "CodeMismatchException" => {
                        VerifyUserAttributeError::CodeMismatch(String::from(error_message))
                    }
                    "ExpiredCodeException" => {
                        VerifyUserAttributeError::ExpiredCode(String::from(error_message))
                    }
                    "InternalErrorException" => {
                        VerifyUserAttributeError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        VerifyUserAttributeError::InvalidParameter(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        VerifyUserAttributeError::LimitExceeded(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        VerifyUserAttributeError::NotAuthorized(String::from(error_message))
                    }
                    "PasswordResetRequiredException" => {
                        VerifyUserAttributeError::PasswordResetRequired(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        VerifyUserAttributeError::ResourceNotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        VerifyUserAttributeError::TooManyRequests(String::from(error_message))
                    }
                    "UserNotConfirmedException" => {
                        VerifyUserAttributeError::UserNotConfirmed(String::from(error_message))
                    }
                    "UserNotFoundException" => {
                        VerifyUserAttributeError::UserNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        VerifyUserAttributeError::Validation(error_message.to_string())
                    }
                    _ => VerifyUserAttributeError::Unknown(String::from(body)),
                }
            }
            Err(_) => VerifyUserAttributeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for VerifyUserAttributeError {
    fn from(err: serde_json::error::Error) -> VerifyUserAttributeError {
        VerifyUserAttributeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for VerifyUserAttributeError {
    fn from(err: CredentialsError) -> VerifyUserAttributeError {
        VerifyUserAttributeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for VerifyUserAttributeError {
    fn from(err: HttpDispatchError) -> VerifyUserAttributeError {
        VerifyUserAttributeError::HttpDispatch(err)
    }
}
impl fmt::Display for VerifyUserAttributeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for VerifyUserAttributeError {
    fn description(&self) -> &str {
        match *self {
            VerifyUserAttributeError::CodeMismatch(ref cause) => cause,
            VerifyUserAttributeError::ExpiredCode(ref cause) => cause,
            VerifyUserAttributeError::InternalError(ref cause) => cause,
            VerifyUserAttributeError::InvalidParameter(ref cause) => cause,
            VerifyUserAttributeError::LimitExceeded(ref cause) => cause,
            VerifyUserAttributeError::NotAuthorized(ref cause) => cause,
            VerifyUserAttributeError::PasswordResetRequired(ref cause) => cause,
            VerifyUserAttributeError::ResourceNotFound(ref cause) => cause,
            VerifyUserAttributeError::TooManyRequests(ref cause) => cause,
            VerifyUserAttributeError::UserNotConfirmed(ref cause) => cause,
            VerifyUserAttributeError::UserNotFound(ref cause) => cause,
            VerifyUserAttributeError::Validation(ref cause) => cause,
            VerifyUserAttributeError::Credentials(ref err) => err.description(),
            VerifyUserAttributeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            VerifyUserAttributeError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon Cognito Identity Provider API. Amazon Cognito Identity Provider clients implement this trait.
pub trait CognitoIdentityProvider {
    #[doc="<p>Adds additional user attributes to the user pool schema.</p>"]
    fn add_custom_attributes(&self,
                             input: &AddCustomAttributesRequest)
                             -> Result<AddCustomAttributesResponse, AddCustomAttributesError>;


    #[doc="<p>Adds the specified user to the specified group.</p> <p>Requires developer credentials.</p>"]
    fn admin_add_user_to_group(&self,
                               input: &AdminAddUserToGroupRequest)
                               -> Result<(), AdminAddUserToGroupError>;


    #[doc="<p>Confirms user registration as an admin without using a confirmation code. Works on any user.</p> <p>Requires developer credentials.</p>"]
    fn admin_confirm_sign_up(&self,
                             input: &AdminConfirmSignUpRequest)
                             -> Result<AdminConfirmSignUpResponse, AdminConfirmSignUpError>;


    #[doc="<p>Creates a new user in the specified user pool and sends a welcome message via email or phone (SMS). This message is based on a template that you configured in your call to <a href=\"API_CreateUserPool.html\">CreateUserPool</a> or <a href=\"API_UpdateUserPool.html\">UpdateUserPool</a>. This template includes your custom sign-up instructions and placeholders for user name and temporary password.</p> <p>Requires developer credentials.</p>"]
    fn admin_create_user(&self,
                         input: &AdminCreateUserRequest)
                         -> Result<AdminCreateUserResponse, AdminCreateUserError>;


    #[doc="<p>Deletes a user as an administrator. Works on any user.</p> <p>Requires developer credentials.</p>"]
    fn admin_delete_user(&self,
                         input: &AdminDeleteUserRequest)
                         -> Result<(), AdminDeleteUserError>;


    #[doc="<p>Deletes the user attributes in a user pool as an administrator. Works on any user.</p> <p>Requires developer credentials.</p>"]
    fn admin_delete_user_attributes
        (&self,
         input: &AdminDeleteUserAttributesRequest)
         -> Result<AdminDeleteUserAttributesResponse, AdminDeleteUserAttributesError>;


    #[doc="<p>Disables the specified user as an administrator. Works on any user.</p> <p>Requires developer credentials.</p>"]
    fn admin_disable_user(&self,
                          input: &AdminDisableUserRequest)
                          -> Result<AdminDisableUserResponse, AdminDisableUserError>;


    #[doc="<p>Enables the specified user as an administrator. Works on any user.</p> <p>Requires developer credentials.</p>"]
    fn admin_enable_user(&self,
                         input: &AdminEnableUserRequest)
                         -> Result<AdminEnableUserResponse, AdminEnableUserError>;


    #[doc="<p>Forgets the device, as an administrator.</p> <p>Requires developer credentials.</p>"]
    fn admin_forget_device(&self,
                           input: &AdminForgetDeviceRequest)
                           -> Result<(), AdminForgetDeviceError>;


    #[doc="<p>Gets the device, as an administrator.</p> <p>Requires developer credentials.</p>"]
    fn admin_get_device(&self,
                        input: &AdminGetDeviceRequest)
                        -> Result<AdminGetDeviceResponse, AdminGetDeviceError>;


    #[doc="<p>Gets the specified user by user name in a user pool as an administrator. Works on any user.</p> <p>Requires developer credentials.</p>"]
    fn admin_get_user(&self,
                      input: &AdminGetUserRequest)
                      -> Result<AdminGetUserResponse, AdminGetUserError>;


    #[doc="<p>Initiates the authentication flow, as an administrator.</p> <p>Requires developer credentials.</p>"]
    fn admin_initiate_auth(&self,
                           input: &AdminInitiateAuthRequest)
                           -> Result<AdminInitiateAuthResponse, AdminInitiateAuthError>;


    #[doc="<p>Lists devices, as an administrator.</p> <p>Requires developer credentials.</p>"]
    fn admin_list_devices(&self,
                          input: &AdminListDevicesRequest)
                          -> Result<AdminListDevicesResponse, AdminListDevicesError>;


    #[doc="<p>Lists the groups that the user belongs to.</p> <p>Requires developer credentials.</p>"]
    fn admin_list_groups_for_user
        (&self,
         input: &AdminListGroupsForUserRequest)
         -> Result<AdminListGroupsForUserResponse, AdminListGroupsForUserError>;


    #[doc="<p>Removes the specified user from the specified group.</p> <p>Requires developer credentials.</p>"]
    fn admin_remove_user_from_group(&self,
                                    input: &AdminRemoveUserFromGroupRequest)
                                    -> Result<(), AdminRemoveUserFromGroupError>;


    #[doc="<p>Resets the specified user's password in a user pool as an administrator. Works on any user.</p> <p>When a developer calls this API, the current password is invalidated, so it must be changed. If a user tries to sign in after the API is called, the app will get a PasswordResetRequiredException exception back and should direct the user down the flow to reset the password, which is the same as the forgot password flow. In addition, if the user pool has phone verification selected and a verified phone number exists for the user, or if email verification is selected and a verified email exists for the user, calling this API will also result in sending a message to the end user with the code to change their password.</p> <p>Requires developer credentials.</p>"]
    fn admin_reset_user_password
        (&self,
         input: &AdminResetUserPasswordRequest)
         -> Result<AdminResetUserPasswordResponse, AdminResetUserPasswordError>;


    #[doc="<p>Responds to an authentication challenge, as an administrator.</p> <p>Requires developer credentials.</p>"]
    fn admin_respond_to_auth_challenge
        (&self,
         input: &AdminRespondToAuthChallengeRequest)
         -> Result<AdminRespondToAuthChallengeResponse, AdminRespondToAuthChallengeError>;


    #[doc="<p>Sets all the user settings for a specified user name. Works on any user.</p> <p>Requires developer credentials.</p>"]
    fn admin_set_user_settings
        (&self,
         input: &AdminSetUserSettingsRequest)
         -> Result<AdminSetUserSettingsResponse, AdminSetUserSettingsError>;


    #[doc="<p>Updates the device status as an administrator.</p> <p>Requires developer credentials.</p>"]
    fn admin_update_device_status
        (&self,
         input: &AdminUpdateDeviceStatusRequest)
         -> Result<AdminUpdateDeviceStatusResponse, AdminUpdateDeviceStatusError>;


    #[doc="<p>Updates the specified user's attributes, including developer attributes, as an administrator. Works on any user.</p> <p>For custom attributes, you must prepend the <code>custom:</code> prefix to the attribute name.</p> <p>In addition to updating user attributes, this API can also be used to mark phone and email as verified.</p> <p>Requires developer credentials.</p>"]
    fn admin_update_user_attributes
        (&self,
         input: &AdminUpdateUserAttributesRequest)
         -> Result<AdminUpdateUserAttributesResponse, AdminUpdateUserAttributesError>;


    #[doc="<p>Signs out users from all devices, as an administrator.</p> <p>Requires developer credentials.</p>"]
    fn admin_user_global_sign_out
        (&self,
         input: &AdminUserGlobalSignOutRequest)
         -> Result<AdminUserGlobalSignOutResponse, AdminUserGlobalSignOutError>;


    #[doc="<p>Changes the password for a specified user in a user pool.</p>"]
    fn change_password(&self,
                       input: &ChangePasswordRequest)
                       -> Result<ChangePasswordResponse, ChangePasswordError>;


    #[doc="<p>Confirms tracking of the device. This API call is the call that begins device tracking.</p>"]
    fn confirm_device(&self,
                      input: &ConfirmDeviceRequest)
                      -> Result<ConfirmDeviceResponse, ConfirmDeviceError>;


    #[doc="<p>Allows a user to enter a confirmation code to reset a forgotten password.</p>"]
    fn confirm_forgot_password
        (&self,
         input: &ConfirmForgotPasswordRequest)
         -> Result<ConfirmForgotPasswordResponse, ConfirmForgotPasswordError>;


    #[doc="<p>Confirms registration of a user and handles the existing alias from a previous user.</p>"]
    fn confirm_sign_up(&self,
                       input: &ConfirmSignUpRequest)
                       -> Result<ConfirmSignUpResponse, ConfirmSignUpError>;


    #[doc="<p>Creates a new group in the specified user pool.</p> <p>Requires developer credentials.</p>"]
    fn create_group(&self,
                    input: &CreateGroupRequest)
                    -> Result<CreateGroupResponse, CreateGroupError>;


    #[doc="<p>Creates an identity provider for a user pool.</p>"]
    fn create_identity_provider
        (&self,
         input: &CreateIdentityProviderRequest)
         -> Result<CreateIdentityProviderResponse, CreateIdentityProviderError>;


    #[doc="<p>Creates the user import job.</p>"]
    fn create_user_import_job(&self,
                              input: &CreateUserImportJobRequest)
                              -> Result<CreateUserImportJobResponse, CreateUserImportJobError>;


    #[doc="<p>Creates a new Amazon Cognito user pool and sets the password policy for the pool.</p>"]
    fn create_user_pool(&self,
                        input: &CreateUserPoolRequest)
                        -> Result<CreateUserPoolResponse, CreateUserPoolError>;


    #[doc="<p>Creates the user pool client.</p>"]
    fn create_user_pool_client
        (&self,
         input: &CreateUserPoolClientRequest)
         -> Result<CreateUserPoolClientResponse, CreateUserPoolClientError>;


    #[doc="<p>Creates a new domain for a user pool.</p>"]
    fn create_user_pool_domain
        (&self,
         input: &CreateUserPoolDomainRequest)
         -> Result<CreateUserPoolDomainResponse, CreateUserPoolDomainError>;


    #[doc="<p>Deletes a group. Currently only groups with no members can be deleted.</p> <p>Requires developer credentials.</p>"]
    fn delete_group(&self, input: &DeleteGroupRequest) -> Result<(), DeleteGroupError>;


    #[doc="<p>Deletes an identity provider for a user pool.</p>"]
    fn delete_identity_provider(&self,
                                input: &DeleteIdentityProviderRequest)
                                -> Result<(), DeleteIdentityProviderError>;


    #[doc="<p>Allows a user to delete one's self.</p>"]
    fn delete_user(&self, input: &DeleteUserRequest) -> Result<(), DeleteUserError>;


    #[doc="<p>Deletes the attributes for a user.</p>"]
    fn delete_user_attributes
        (&self,
         input: &DeleteUserAttributesRequest)
         -> Result<DeleteUserAttributesResponse, DeleteUserAttributesError>;


    #[doc="<p>Deletes the specified Amazon Cognito user pool.</p>"]
    fn delete_user_pool(&self, input: &DeleteUserPoolRequest) -> Result<(), DeleteUserPoolError>;


    #[doc="<p>Allows the developer to delete the user pool client.</p>"]
    fn delete_user_pool_client(&self,
                               input: &DeleteUserPoolClientRequest)
                               -> Result<(), DeleteUserPoolClientError>;


    #[doc="<p>Deletes a domain for a user pool.</p>"]
    fn delete_user_pool_domain
        (&self,
         input: &DeleteUserPoolDomainRequest)
         -> Result<DeleteUserPoolDomainResponse, DeleteUserPoolDomainError>;


    #[doc="<p>Gets information about a specific identity provider.</p>"]
    fn describe_identity_provider
        (&self,
         input: &DescribeIdentityProviderRequest)
         -> Result<DescribeIdentityProviderResponse, DescribeIdentityProviderError>;


    #[doc="<p>Describes the user import job.</p>"]
    fn describe_user_import_job
        (&self,
         input: &DescribeUserImportJobRequest)
         -> Result<DescribeUserImportJobResponse, DescribeUserImportJobError>;


    #[doc="<p>Returns the configuration information and metadata of the specified user pool.</p>"]
    fn describe_user_pool(&self,
                          input: &DescribeUserPoolRequest)
                          -> Result<DescribeUserPoolResponse, DescribeUserPoolError>;


    #[doc="<p>Client method for returning the configuration information and metadata of the specified user pool client.</p>"]
    fn describe_user_pool_client
        (&self,
         input: &DescribeUserPoolClientRequest)
         -> Result<DescribeUserPoolClientResponse, DescribeUserPoolClientError>;


    #[doc="<p>Gets information about a domain.</p>"]
    fn describe_user_pool_domain
        (&self,
         input: &DescribeUserPoolDomainRequest)
         -> Result<DescribeUserPoolDomainResponse, DescribeUserPoolDomainError>;


    #[doc="<p>Forgets the specified device.</p>"]
    fn forget_device(&self, input: &ForgetDeviceRequest) -> Result<(), ForgetDeviceError>;


    #[doc="<p>Calling this API causes a message to be sent to the end user with a confirmation code that is required to change the user's password. For the <code>Username</code> parameter, you can use the username or user alias. If a verified phone number exists for the user, the confirmation code is sent to the phone number. Otherwise, if a verified email exists, the confirmation code is sent to the email. If neither a verified phone number nor a verified email exists, <code>InvalidParameterException</code> is thrown. To use the confirmation code for resetting the password, call <a href=\"API_ConfirmForgotPassword.html\">ConfirmForgotPassword</a>.</p>"]
    fn forgot_password(&self,
                       input: &ForgotPasswordRequest)
                       -> Result<ForgotPasswordResponse, ForgotPasswordError>;


    #[doc="<p>Gets the header information for the .csv file to be used as input for the user import job.</p>"]
    fn get_csv_header(&self,
                      input: &GetCSVHeaderRequest)
                      -> Result<GetCSVHeaderResponse, GetCSVHeaderError>;


    #[doc="<p>Gets the device.</p>"]
    fn get_device(&self, input: &GetDeviceRequest) -> Result<GetDeviceResponse, GetDeviceError>;


    #[doc="<p>Gets a group.</p> <p>Requires developer credentials.</p>"]
    fn get_group(&self, input: &GetGroupRequest) -> Result<GetGroupResponse, GetGroupError>;


    #[doc="<p>Gets the specified identity provider.</p>"]
    fn get_identity_provider_by_identifier
        (&self,
         input: &GetIdentityProviderByIdentifierRequest)
         -> Result<GetIdentityProviderByIdentifierResponse, GetIdentityProviderByIdentifierError>;


    #[doc="<p>Gets the user attributes and metadata for a user.</p>"]
    fn get_user(&self, input: &GetUserRequest) -> Result<GetUserResponse, GetUserError>;


    #[doc="<p>Gets the user attribute verification code for the specified attribute name.</p>"]
    fn get_user_attribute_verification_code
        (&self,
         input: &GetUserAttributeVerificationCodeRequest)
         -> Result<GetUserAttributeVerificationCodeResponse, GetUserAttributeVerificationCodeError>;


    #[doc="<p>Signs out users from all devices.</p>"]
    fn global_sign_out(&self,
                       input: &GlobalSignOutRequest)
                       -> Result<GlobalSignOutResponse, GlobalSignOutError>;


    #[doc="<p>Initiates the authentication flow.</p>"]
    fn initiate_auth(&self,
                     input: &InitiateAuthRequest)
                     -> Result<InitiateAuthResponse, InitiateAuthError>;


    #[doc="<p>Lists the devices.</p>"]
    fn list_devices(&self,
                    input: &ListDevicesRequest)
                    -> Result<ListDevicesResponse, ListDevicesError>;


    #[doc="<p>Lists the groups associated with a user pool.</p> <p>Requires developer credentials.</p>"]
    fn list_groups(&self,
                   input: &ListGroupsRequest)
                   -> Result<ListGroupsResponse, ListGroupsError>;


    #[doc="<p>Lists information about all identity providers for a user pool.</p>"]
    fn list_identity_providers
        (&self,
         input: &ListIdentityProvidersRequest)
         -> Result<ListIdentityProvidersResponse, ListIdentityProvidersError>;


    #[doc="<p>Lists the user import jobs.</p>"]
    fn list_user_import_jobs(&self,
                             input: &ListUserImportJobsRequest)
                             -> Result<ListUserImportJobsResponse, ListUserImportJobsError>;


    #[doc="<p>Lists the clients that have been created for the specified user pool.</p>"]
    fn list_user_pool_clients(&self,
                              input: &ListUserPoolClientsRequest)
                              -> Result<ListUserPoolClientsResponse, ListUserPoolClientsError>;


    #[doc="<p>Lists the user pools associated with an AWS account.</p>"]
    fn list_user_pools(&self,
                       input: &ListUserPoolsRequest)
                       -> Result<ListUserPoolsResponse, ListUserPoolsError>;


    #[doc="<p>Lists the users in the Amazon Cognito user pool.</p>"]
    fn list_users(&self, input: &ListUsersRequest) -> Result<ListUsersResponse, ListUsersError>;


    #[doc="<p>Lists the users in the specified group.</p> <p>Requires developer credentials.</p>"]
    fn list_users_in_group(&self,
                           input: &ListUsersInGroupRequest)
                           -> Result<ListUsersInGroupResponse, ListUsersInGroupError>;


    #[doc="<p>Resends the confirmation (for confirmation of registration) to a specific user in the user pool.</p>"]
    fn resend_confirmation_code
        (&self,
         input: &ResendConfirmationCodeRequest)
         -> Result<ResendConfirmationCodeResponse, ResendConfirmationCodeError>;


    #[doc="<p>Responds to the authentication challenge.</p>"]
    fn respond_to_auth_challenge
        (&self,
         input: &RespondToAuthChallengeRequest)
         -> Result<RespondToAuthChallengeResponse, RespondToAuthChallengeError>;


    #[doc="<p>Sets the user settings like multi-factor authentication (MFA). If MFA is to be removed for a particular attribute pass the attribute with code delivery as null. If null list is passed, all MFA options are removed.</p>"]
    fn set_user_settings(&self,
                         input: &SetUserSettingsRequest)
                         -> Result<SetUserSettingsResponse, SetUserSettingsError>;


    #[doc="<p>Registers the user in the specified user pool and creates a user name, password, and user attributes.</p>"]
    fn sign_up(&self, input: &SignUpRequest) -> Result<SignUpResponse, SignUpError>;


    #[doc="<p>Starts the user import.</p>"]
    fn start_user_import_job(&self,
                             input: &StartUserImportJobRequest)
                             -> Result<StartUserImportJobResponse, StartUserImportJobError>;


    #[doc="<p>Stops the user import job.</p>"]
    fn stop_user_import_job(&self,
                            input: &StopUserImportJobRequest)
                            -> Result<StopUserImportJobResponse, StopUserImportJobError>;


    #[doc="<p>Updates the device status.</p>"]
    fn update_device_status(&self,
                            input: &UpdateDeviceStatusRequest)
                            -> Result<UpdateDeviceStatusResponse, UpdateDeviceStatusError>;


    #[doc="<p>Updates the specified group with the specified attributes.</p> <p>Requires developer credentials.</p>"]
    fn update_group(&self,
                    input: &UpdateGroupRequest)
                    -> Result<UpdateGroupResponse, UpdateGroupError>;


    #[doc="<p>Updates identity provider information for a user pool.</p>"]
    fn update_identity_provider
        (&self,
         input: &UpdateIdentityProviderRequest)
         -> Result<UpdateIdentityProviderResponse, UpdateIdentityProviderError>;


    #[doc="<p>Allows a user to update a specific attribute (one at a time).</p>"]
    fn update_user_attributes
        (&self,
         input: &UpdateUserAttributesRequest)
         -> Result<UpdateUserAttributesResponse, UpdateUserAttributesError>;


    #[doc="<p>Updates the specified user pool with the specified attributes.</p>"]
    fn update_user_pool(&self,
                        input: &UpdateUserPoolRequest)
                        -> Result<UpdateUserPoolResponse, UpdateUserPoolError>;


    #[doc="<p>Allows the developer to update the specified user pool client and password policy.</p>"]
    fn update_user_pool_client
        (&self,
         input: &UpdateUserPoolClientRequest)
         -> Result<UpdateUserPoolClientResponse, UpdateUserPoolClientError>;


    #[doc="<p>Verifies the specified user attributes in the user pool.</p>"]
    fn verify_user_attribute(&self,
                             input: &VerifyUserAttributeRequest)
                             -> Result<VerifyUserAttributeResponse, VerifyUserAttributeError>;
}
/// A client for the Amazon Cognito Identity Provider API.
pub struct CognitoIdentityProviderClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> CognitoIdentityProviderClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        CognitoIdentityProviderClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> CognitoIdentityProvider for CognitoIdentityProviderClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    #[doc="<p>Adds additional user attributes to the user pool schema.</p>"]
    fn add_custom_attributes(&self,
                             input: &AddCustomAttributesRequest)
                             -> Result<AddCustomAttributesResponse, AddCustomAttributesError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.AddCustomAttributes");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<AddCustomAttributesResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(AddCustomAttributesError::from_body(String::from_utf8_lossy(&response.body)
                                                            .as_ref()))
            }
        }
    }


    #[doc="<p>Adds the specified user to the specified group.</p> <p>Requires developer credentials.</p>"]
    fn admin_add_user_to_group(&self,
                               input: &AdminAddUserToGroupRequest)
                               -> Result<(), AdminAddUserToGroupError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.AdminAddUserToGroup");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => Ok(()),
            _ => {
                Err(AdminAddUserToGroupError::from_body(String::from_utf8_lossy(&response.body)
                                                            .as_ref()))
            }
        }
    }


    #[doc="<p>Confirms user registration as an admin without using a confirmation code. Works on any user.</p> <p>Requires developer credentials.</p>"]
    fn admin_confirm_sign_up(&self,
                             input: &AdminConfirmSignUpRequest)
                             -> Result<AdminConfirmSignUpResponse, AdminConfirmSignUpError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.AdminConfirmSignUp");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<AdminConfirmSignUpResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(AdminConfirmSignUpError::from_body(String::from_utf8_lossy(&response.body)
                                                           .as_ref()))
            }
        }
    }


    #[doc="<p>Creates a new user in the specified user pool and sends a welcome message via email or phone (SMS). This message is based on a template that you configured in your call to <a href=\"API_CreateUserPool.html\">CreateUserPool</a> or <a href=\"API_UpdateUserPool.html\">UpdateUserPool</a>. This template includes your custom sign-up instructions and placeholders for user name and temporary password.</p> <p>Requires developer credentials.</p>"]
    fn admin_create_user(&self,
                         input: &AdminCreateUserRequest)
                         -> Result<AdminCreateUserResponse, AdminCreateUserError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.AdminCreateUser");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<AdminCreateUserResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(AdminCreateUserError::from_body(String::from_utf8_lossy(&response.body)
                                                        .as_ref()))
            }
        }
    }


    #[doc="<p>Deletes a user as an administrator. Works on any user.</p> <p>Requires developer credentials.</p>"]
    fn admin_delete_user(&self,
                         input: &AdminDeleteUserRequest)
                         -> Result<(), AdminDeleteUserError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.AdminDeleteUser");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => Ok(()),
            _ => {
                Err(AdminDeleteUserError::from_body(String::from_utf8_lossy(&response.body)
                                                        .as_ref()))
            }
        }
    }


    #[doc="<p>Deletes the user attributes in a user pool as an administrator. Works on any user.</p> <p>Requires developer credentials.</p>"]
    fn admin_delete_user_attributes
        (&self,
         input: &AdminDeleteUserAttributesRequest)
         -> Result<AdminDeleteUserAttributesResponse, AdminDeleteUserAttributesError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.AdminDeleteUserAttributes");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<AdminDeleteUserAttributesResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(AdminDeleteUserAttributesError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Disables the specified user as an administrator. Works on any user.</p> <p>Requires developer credentials.</p>"]
    fn admin_disable_user(&self,
                          input: &AdminDisableUserRequest)
                          -> Result<AdminDisableUserResponse, AdminDisableUserError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.AdminDisableUser");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<AdminDisableUserResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(AdminDisableUserError::from_body(String::from_utf8_lossy(&response.body)
                                                         .as_ref()))
            }
        }
    }


    #[doc="<p>Enables the specified user as an administrator. Works on any user.</p> <p>Requires developer credentials.</p>"]
    fn admin_enable_user(&self,
                         input: &AdminEnableUserRequest)
                         -> Result<AdminEnableUserResponse, AdminEnableUserError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.AdminEnableUser");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<AdminEnableUserResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(AdminEnableUserError::from_body(String::from_utf8_lossy(&response.body)
                                                        .as_ref()))
            }
        }
    }


    #[doc="<p>Forgets the device, as an administrator.</p> <p>Requires developer credentials.</p>"]
    fn admin_forget_device(&self,
                           input: &AdminForgetDeviceRequest)
                           -> Result<(), AdminForgetDeviceError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.AdminForgetDevice");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => Ok(()),
            _ => {
                Err(AdminForgetDeviceError::from_body(String::from_utf8_lossy(&response.body)
                                                          .as_ref()))
            }
        }
    }


    #[doc="<p>Gets the device, as an administrator.</p> <p>Requires developer credentials.</p>"]
    fn admin_get_device(&self,
                        input: &AdminGetDeviceRequest)
                        -> Result<AdminGetDeviceResponse, AdminGetDeviceError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.AdminGetDevice");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<AdminGetDeviceResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(AdminGetDeviceError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Gets the specified user by user name in a user pool as an administrator. Works on any user.</p> <p>Requires developer credentials.</p>"]
    fn admin_get_user(&self,
                      input: &AdminGetUserRequest)
                      -> Result<AdminGetUserResponse, AdminGetUserError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.AdminGetUser");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<AdminGetUserResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(AdminGetUserError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Initiates the authentication flow, as an administrator.</p> <p>Requires developer credentials.</p>"]
    fn admin_initiate_auth(&self,
                           input: &AdminInitiateAuthRequest)
                           -> Result<AdminInitiateAuthResponse, AdminInitiateAuthError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.AdminInitiateAuth");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<AdminInitiateAuthResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(AdminInitiateAuthError::from_body(String::from_utf8_lossy(&response.body)
                                                          .as_ref()))
            }
        }
    }


    #[doc="<p>Lists devices, as an administrator.</p> <p>Requires developer credentials.</p>"]
    fn admin_list_devices(&self,
                          input: &AdminListDevicesRequest)
                          -> Result<AdminListDevicesResponse, AdminListDevicesError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.AdminListDevices");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<AdminListDevicesResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(AdminListDevicesError::from_body(String::from_utf8_lossy(&response.body)
                                                         .as_ref()))
            }
        }
    }


    #[doc="<p>Lists the groups that the user belongs to.</p> <p>Requires developer credentials.</p>"]
    fn admin_list_groups_for_user
        (&self,
         input: &AdminListGroupsForUserRequest)
         -> Result<AdminListGroupsForUserResponse, AdminListGroupsForUserError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.AdminListGroupsForUser");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<AdminListGroupsForUserResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(AdminListGroupsForUserError::from_body(String::from_utf8_lossy(&response.body)
                                                               .as_ref()))
            }
        }
    }


    #[doc="<p>Removes the specified user from the specified group.</p> <p>Requires developer credentials.</p>"]
    fn admin_remove_user_from_group(&self,
                                    input: &AdminRemoveUserFromGroupRequest)
                                    -> Result<(), AdminRemoveUserFromGroupError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.AdminRemoveUserFromGroup");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => Ok(()),
            _ => Err(AdminRemoveUserFromGroupError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Resets the specified user's password in a user pool as an administrator. Works on any user.</p> <p>When a developer calls this API, the current password is invalidated, so it must be changed. If a user tries to sign in after the API is called, the app will get a PasswordResetRequiredException exception back and should direct the user down the flow to reset the password, which is the same as the forgot password flow. In addition, if the user pool has phone verification selected and a verified phone number exists for the user, or if email verification is selected and a verified email exists for the user, calling this API will also result in sending a message to the end user with the code to change their password.</p> <p>Requires developer credentials.</p>"]
    fn admin_reset_user_password
        (&self,
         input: &AdminResetUserPasswordRequest)
         -> Result<AdminResetUserPasswordResponse, AdminResetUserPasswordError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.AdminResetUserPassword");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<AdminResetUserPasswordResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(AdminResetUserPasswordError::from_body(String::from_utf8_lossy(&response.body)
                                                               .as_ref()))
            }
        }
    }


    #[doc="<p>Responds to an authentication challenge, as an administrator.</p> <p>Requires developer credentials.</p>"]
    fn admin_respond_to_auth_challenge
        (&self,
         input: &AdminRespondToAuthChallengeRequest)
         -> Result<AdminRespondToAuthChallengeResponse, AdminRespondToAuthChallengeError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.AdminRespondToAuthChallenge");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<AdminRespondToAuthChallengeResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(AdminRespondToAuthChallengeError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Sets all the user settings for a specified user name. Works on any user.</p> <p>Requires developer credentials.</p>"]
    fn admin_set_user_settings
        (&self,
         input: &AdminSetUserSettingsRequest)
         -> Result<AdminSetUserSettingsResponse, AdminSetUserSettingsError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.AdminSetUserSettings");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<AdminSetUserSettingsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(AdminSetUserSettingsError::from_body(String::from_utf8_lossy(&response.body)
                                                             .as_ref()))
            }
        }
    }


    #[doc="<p>Updates the device status as an administrator.</p> <p>Requires developer credentials.</p>"]
    fn admin_update_device_status
        (&self,
         input: &AdminUpdateDeviceStatusRequest)
         -> Result<AdminUpdateDeviceStatusResponse, AdminUpdateDeviceStatusError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.AdminUpdateDeviceStatus");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<AdminUpdateDeviceStatusResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(AdminUpdateDeviceStatusError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Updates the specified user's attributes, including developer attributes, as an administrator. Works on any user.</p> <p>For custom attributes, you must prepend the <code>custom:</code> prefix to the attribute name.</p> <p>In addition to updating user attributes, this API can also be used to mark phone and email as verified.</p> <p>Requires developer credentials.</p>"]
    fn admin_update_user_attributes
        (&self,
         input: &AdminUpdateUserAttributesRequest)
         -> Result<AdminUpdateUserAttributesResponse, AdminUpdateUserAttributesError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.AdminUpdateUserAttributes");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<AdminUpdateUserAttributesResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(AdminUpdateUserAttributesError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Signs out users from all devices, as an administrator.</p> <p>Requires developer credentials.</p>"]
    fn admin_user_global_sign_out
        (&self,
         input: &AdminUserGlobalSignOutRequest)
         -> Result<AdminUserGlobalSignOutResponse, AdminUserGlobalSignOutError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.AdminUserGlobalSignOut");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<AdminUserGlobalSignOutResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(AdminUserGlobalSignOutError::from_body(String::from_utf8_lossy(&response.body)
                                                               .as_ref()))
            }
        }
    }


    #[doc="<p>Changes the password for a specified user in a user pool.</p>"]
    fn change_password(&self,
                       input: &ChangePasswordRequest)
                       -> Result<ChangePasswordResponse, ChangePasswordError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.ChangePassword");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<ChangePasswordResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(ChangePasswordError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Confirms tracking of the device. This API call is the call that begins device tracking.</p>"]
    fn confirm_device(&self,
                      input: &ConfirmDeviceRequest)
                      -> Result<ConfirmDeviceResponse, ConfirmDeviceError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.ConfirmDevice");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<ConfirmDeviceResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(ConfirmDeviceError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Allows a user to enter a confirmation code to reset a forgotten password.</p>"]
    fn confirm_forgot_password
        (&self,
         input: &ConfirmForgotPasswordRequest)
         -> Result<ConfirmForgotPasswordResponse, ConfirmForgotPasswordError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.ConfirmForgotPassword");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<ConfirmForgotPasswordResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(ConfirmForgotPasswordError::from_body(String::from_utf8_lossy(&response.body)
                                                              .as_ref()))
            }
        }
    }


    #[doc="<p>Confirms registration of a user and handles the existing alias from a previous user.</p>"]
    fn confirm_sign_up(&self,
                       input: &ConfirmSignUpRequest)
                       -> Result<ConfirmSignUpResponse, ConfirmSignUpError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.ConfirmSignUp");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<ConfirmSignUpResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(ConfirmSignUpError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Creates a new group in the specified user pool.</p> <p>Requires developer credentials.</p>"]
    fn create_group(&self,
                    input: &CreateGroupRequest)
                    -> Result<CreateGroupResponse, CreateGroupError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.CreateGroup");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateGroupResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(CreateGroupError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Creates an identity provider for a user pool.</p>"]
    fn create_identity_provider
        (&self,
         input: &CreateIdentityProviderRequest)
         -> Result<CreateIdentityProviderResponse, CreateIdentityProviderError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.CreateIdentityProvider");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateIdentityProviderResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(CreateIdentityProviderError::from_body(String::from_utf8_lossy(&response.body)
                                                               .as_ref()))
            }
        }
    }


    #[doc="<p>Creates the user import job.</p>"]
    fn create_user_import_job(&self,
                              input: &CreateUserImportJobRequest)
                              -> Result<CreateUserImportJobResponse, CreateUserImportJobError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.CreateUserImportJob");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateUserImportJobResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(CreateUserImportJobError::from_body(String::from_utf8_lossy(&response.body)
                                                            .as_ref()))
            }
        }
    }


    #[doc="<p>Creates a new Amazon Cognito user pool and sets the password policy for the pool.</p>"]
    fn create_user_pool(&self,
                        input: &CreateUserPoolRequest)
                        -> Result<CreateUserPoolResponse, CreateUserPoolError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.CreateUserPool");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateUserPoolResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(CreateUserPoolError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Creates the user pool client.</p>"]
    fn create_user_pool_client
        (&self,
         input: &CreateUserPoolClientRequest)
         -> Result<CreateUserPoolClientResponse, CreateUserPoolClientError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.CreateUserPoolClient");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateUserPoolClientResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(CreateUserPoolClientError::from_body(String::from_utf8_lossy(&response.body)
                                                             .as_ref()))
            }
        }
    }


    #[doc="<p>Creates a new domain for a user pool.</p>"]
    fn create_user_pool_domain
        (&self,
         input: &CreateUserPoolDomainRequest)
         -> Result<CreateUserPoolDomainResponse, CreateUserPoolDomainError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.CreateUserPoolDomain");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateUserPoolDomainResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(CreateUserPoolDomainError::from_body(String::from_utf8_lossy(&response.body)
                                                             .as_ref()))
            }
        }
    }


    #[doc="<p>Deletes a group. Currently only groups with no members can be deleted.</p> <p>Requires developer credentials.</p>"]
    fn delete_group(&self, input: &DeleteGroupRequest) -> Result<(), DeleteGroupError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.DeleteGroup");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => Ok(()),
            _ => Err(DeleteGroupError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Deletes an identity provider for a user pool.</p>"]
    fn delete_identity_provider(&self,
                                input: &DeleteIdentityProviderRequest)
                                -> Result<(), DeleteIdentityProviderError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.DeleteIdentityProvider");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => Ok(()),
            _ => {
                Err(DeleteIdentityProviderError::from_body(String::from_utf8_lossy(&response.body)
                                                               .as_ref()))
            }
        }
    }


    #[doc="<p>Allows a user to delete one's self.</p>"]
    fn delete_user(&self, input: &DeleteUserRequest) -> Result<(), DeleteUserError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.DeleteUser");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => Ok(()),
            _ => Err(DeleteUserError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Deletes the attributes for a user.</p>"]
    fn delete_user_attributes
        (&self,
         input: &DeleteUserAttributesRequest)
         -> Result<DeleteUserAttributesResponse, DeleteUserAttributesError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.DeleteUserAttributes");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeleteUserAttributesResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DeleteUserAttributesError::from_body(String::from_utf8_lossy(&response.body)
                                                             .as_ref()))
            }
        }
    }


    #[doc="<p>Deletes the specified Amazon Cognito user pool.</p>"]
    fn delete_user_pool(&self, input: &DeleteUserPoolRequest) -> Result<(), DeleteUserPoolError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.DeleteUserPool");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => Ok(()),
            _ => {
                Err(DeleteUserPoolError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Allows the developer to delete the user pool client.</p>"]
    fn delete_user_pool_client(&self,
                               input: &DeleteUserPoolClientRequest)
                               -> Result<(), DeleteUserPoolClientError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.DeleteUserPoolClient");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => Ok(()),
            _ => {
                Err(DeleteUserPoolClientError::from_body(String::from_utf8_lossy(&response.body)
                                                             .as_ref()))
            }
        }
    }


    #[doc="<p>Deletes a domain for a user pool.</p>"]
    fn delete_user_pool_domain
        (&self,
         input: &DeleteUserPoolDomainRequest)
         -> Result<DeleteUserPoolDomainResponse, DeleteUserPoolDomainError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.DeleteUserPoolDomain");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeleteUserPoolDomainResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DeleteUserPoolDomainError::from_body(String::from_utf8_lossy(&response.body)
                                                             .as_ref()))
            }
        }
    }


    #[doc="<p>Gets information about a specific identity provider.</p>"]
    fn describe_identity_provider
        (&self,
         input: &DescribeIdentityProviderRequest)
         -> Result<DescribeIdentityProviderResponse, DescribeIdentityProviderError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.DescribeIdentityProvider");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeIdentityProviderResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DescribeIdentityProviderError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Describes the user import job.</p>"]
    fn describe_user_import_job
        (&self,
         input: &DescribeUserImportJobRequest)
         -> Result<DescribeUserImportJobResponse, DescribeUserImportJobError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.DescribeUserImportJob");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeUserImportJobResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeUserImportJobError::from_body(String::from_utf8_lossy(&response.body)
                                                              .as_ref()))
            }
        }
    }


    #[doc="<p>Returns the configuration information and metadata of the specified user pool.</p>"]
    fn describe_user_pool(&self,
                          input: &DescribeUserPoolRequest)
                          -> Result<DescribeUserPoolResponse, DescribeUserPoolError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.DescribeUserPool");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeUserPoolResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeUserPoolError::from_body(String::from_utf8_lossy(&response.body)
                                                         .as_ref()))
            }
        }
    }


    #[doc="<p>Client method for returning the configuration information and metadata of the specified user pool client.</p>"]
    fn describe_user_pool_client
        (&self,
         input: &DescribeUserPoolClientRequest)
         -> Result<DescribeUserPoolClientResponse, DescribeUserPoolClientError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.DescribeUserPoolClient");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeUserPoolClientResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeUserPoolClientError::from_body(String::from_utf8_lossy(&response.body)
                                                               .as_ref()))
            }
        }
    }


    #[doc="<p>Gets information about a domain.</p>"]
    fn describe_user_pool_domain
        (&self,
         input: &DescribeUserPoolDomainRequest)
         -> Result<DescribeUserPoolDomainResponse, DescribeUserPoolDomainError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.DescribeUserPoolDomain");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeUserPoolDomainResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeUserPoolDomainError::from_body(String::from_utf8_lossy(&response.body)
                                                               .as_ref()))
            }
        }
    }


    #[doc="<p>Forgets the specified device.</p>"]
    fn forget_device(&self, input: &ForgetDeviceRequest) -> Result<(), ForgetDeviceError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.ForgetDevice");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => Ok(()),
            _ => {
                Err(ForgetDeviceError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Calling this API causes a message to be sent to the end user with a confirmation code that is required to change the user's password. For the <code>Username</code> parameter, you can use the username or user alias. If a verified phone number exists for the user, the confirmation code is sent to the phone number. Otherwise, if a verified email exists, the confirmation code is sent to the email. If neither a verified phone number nor a verified email exists, <code>InvalidParameterException</code> is thrown. To use the confirmation code for resetting the password, call <a href=\"API_ConfirmForgotPassword.html\">ConfirmForgotPassword</a>.</p>"]
    fn forgot_password(&self,
                       input: &ForgotPasswordRequest)
                       -> Result<ForgotPasswordResponse, ForgotPasswordError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.ForgotPassword");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<ForgotPasswordResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(ForgotPasswordError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Gets the header information for the .csv file to be used as input for the user import job.</p>"]
    fn get_csv_header(&self,
                      input: &GetCSVHeaderRequest)
                      -> Result<GetCSVHeaderResponse, GetCSVHeaderError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.GetCSVHeader");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetCSVHeaderResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(GetCSVHeaderError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Gets the device.</p>"]
    fn get_device(&self, input: &GetDeviceRequest) -> Result<GetDeviceResponse, GetDeviceError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.GetDevice");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetDeviceResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(GetDeviceError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Gets a group.</p> <p>Requires developer credentials.</p>"]
    fn get_group(&self, input: &GetGroupRequest) -> Result<GetGroupResponse, GetGroupError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSCognitoIdentityProviderService.GetGroup");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetGroupResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(GetGroupError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Gets the specified identity provider.</p>"]
    fn get_identity_provider_by_identifier
        (&self,
         input: &GetIdentityProviderByIdentifierRequest)
         -> Result<GetIdentityProviderByIdentifierResponse, GetIdentityProviderByIdentifierError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.GetIdentityProviderByIdentifier");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetIdentityProviderByIdentifierResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(GetIdentityProviderByIdentifierError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Gets the user attributes and metadata for a user.</p>"]
    fn get_user(&self, input: &GetUserRequest) -> Result<GetUserResponse, GetUserError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSCognitoIdentityProviderService.GetUser");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                Ok(serde_json::from_str::<GetUserResponse>(String::from_utf8_lossy(&response.body)
                                                               .as_ref())
                           .unwrap())
            }
            _ => Err(GetUserError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Gets the user attribute verification code for the specified attribute name.</p>"]
    fn get_user_attribute_verification_code
        (&self,
         input: &GetUserAttributeVerificationCodeRequest)
         -> Result<GetUserAttributeVerificationCodeResponse, GetUserAttributeVerificationCodeError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.GetUserAttributeVerificationCode");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetUserAttributeVerificationCodeResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(GetUserAttributeVerificationCodeError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Signs out users from all devices.</p>"]
    fn global_sign_out(&self,
                       input: &GlobalSignOutRequest)
                       -> Result<GlobalSignOutResponse, GlobalSignOutError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.GlobalSignOut");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<GlobalSignOutResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(GlobalSignOutError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Initiates the authentication flow.</p>"]
    fn initiate_auth(&self,
                     input: &InitiateAuthRequest)
                     -> Result<InitiateAuthResponse, InitiateAuthError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.InitiateAuth");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<InitiateAuthResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(InitiateAuthError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Lists the devices.</p>"]
    fn list_devices(&self,
                    input: &ListDevicesRequest)
                    -> Result<ListDevicesResponse, ListDevicesError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.ListDevices");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListDevicesResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(ListDevicesError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Lists the groups associated with a user pool.</p> <p>Requires developer credentials.</p>"]
    fn list_groups(&self,
                   input: &ListGroupsRequest)
                   -> Result<ListGroupsResponse, ListGroupsError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.ListGroups");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListGroupsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(ListGroupsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Lists information about all identity providers for a user pool.</p>"]
    fn list_identity_providers
        (&self,
         input: &ListIdentityProvidersRequest)
         -> Result<ListIdentityProvidersResponse, ListIdentityProvidersError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.ListIdentityProviders");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListIdentityProvidersResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(ListIdentityProvidersError::from_body(String::from_utf8_lossy(&response.body)
                                                              .as_ref()))
            }
        }
    }


    #[doc="<p>Lists the user import jobs.</p>"]
    fn list_user_import_jobs(&self,
                             input: &ListUserImportJobsRequest)
                             -> Result<ListUserImportJobsResponse, ListUserImportJobsError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.ListUserImportJobs");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListUserImportJobsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(ListUserImportJobsError::from_body(String::from_utf8_lossy(&response.body)
                                                           .as_ref()))
            }
        }
    }


    #[doc="<p>Lists the clients that have been created for the specified user pool.</p>"]
    fn list_user_pool_clients(&self,
                              input: &ListUserPoolClientsRequest)
                              -> Result<ListUserPoolClientsResponse, ListUserPoolClientsError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.ListUserPoolClients");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListUserPoolClientsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(ListUserPoolClientsError::from_body(String::from_utf8_lossy(&response.body)
                                                            .as_ref()))
            }
        }
    }


    #[doc="<p>Lists the user pools associated with an AWS account.</p>"]
    fn list_user_pools(&self,
                       input: &ListUserPoolsRequest)
                       -> Result<ListUserPoolsResponse, ListUserPoolsError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.ListUserPools");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListUserPoolsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(ListUserPoolsError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Lists the users in the Amazon Cognito user pool.</p>"]
    fn list_users(&self, input: &ListUsersRequest) -> Result<ListUsersResponse, ListUsersError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.ListUsers");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListUsersResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(ListUsersError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Lists the users in the specified group.</p> <p>Requires developer credentials.</p>"]
    fn list_users_in_group(&self,
                           input: &ListUsersInGroupRequest)
                           -> Result<ListUsersInGroupResponse, ListUsersInGroupError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.ListUsersInGroup");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListUsersInGroupResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(ListUsersInGroupError::from_body(String::from_utf8_lossy(&response.body)
                                                         .as_ref()))
            }
        }
    }


    #[doc="<p>Resends the confirmation (for confirmation of registration) to a specific user in the user pool.</p>"]
    fn resend_confirmation_code
        (&self,
         input: &ResendConfirmationCodeRequest)
         -> Result<ResendConfirmationCodeResponse, ResendConfirmationCodeError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.ResendConfirmationCode");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<ResendConfirmationCodeResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(ResendConfirmationCodeError::from_body(String::from_utf8_lossy(&response.body)
                                                               .as_ref()))
            }
        }
    }


    #[doc="<p>Responds to the authentication challenge.</p>"]
    fn respond_to_auth_challenge
        (&self,
         input: &RespondToAuthChallengeRequest)
         -> Result<RespondToAuthChallengeResponse, RespondToAuthChallengeError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.RespondToAuthChallenge");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<RespondToAuthChallengeResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(RespondToAuthChallengeError::from_body(String::from_utf8_lossy(&response.body)
                                                               .as_ref()))
            }
        }
    }


    #[doc="<p>Sets the user settings like multi-factor authentication (MFA). If MFA is to be removed for a particular attribute pass the attribute with code delivery as null. If null list is passed, all MFA options are removed.</p>"]
    fn set_user_settings(&self,
                         input: &SetUserSettingsRequest)
                         -> Result<SetUserSettingsResponse, SetUserSettingsError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.SetUserSettings");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<SetUserSettingsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(SetUserSettingsError::from_body(String::from_utf8_lossy(&response.body)
                                                        .as_ref()))
            }
        }
    }


    #[doc="<p>Registers the user in the specified user pool and creates a user name, password, and user attributes.</p>"]
    fn sign_up(&self, input: &SignUpRequest) -> Result<SignUpResponse, SignUpError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSCognitoIdentityProviderService.SignUp");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                Ok(serde_json::from_str::<SignUpResponse>(String::from_utf8_lossy(&response.body)
                                                              .as_ref())
                           .unwrap())
            }
            _ => Err(SignUpError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Starts the user import.</p>"]
    fn start_user_import_job(&self,
                             input: &StartUserImportJobRequest)
                             -> Result<StartUserImportJobResponse, StartUserImportJobError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.StartUserImportJob");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<StartUserImportJobResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(StartUserImportJobError::from_body(String::from_utf8_lossy(&response.body)
                                                           .as_ref()))
            }
        }
    }


    #[doc="<p>Stops the user import job.</p>"]
    fn stop_user_import_job(&self,
                            input: &StopUserImportJobRequest)
                            -> Result<StopUserImportJobResponse, StopUserImportJobError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.StopUserImportJob");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<StopUserImportJobResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(StopUserImportJobError::from_body(String::from_utf8_lossy(&response.body)
                                                          .as_ref()))
            }
        }
    }


    #[doc="<p>Updates the device status.</p>"]
    fn update_device_status(&self,
                            input: &UpdateDeviceStatusRequest)
                            -> Result<UpdateDeviceStatusResponse, UpdateDeviceStatusError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.UpdateDeviceStatus");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<UpdateDeviceStatusResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(UpdateDeviceStatusError::from_body(String::from_utf8_lossy(&response.body)
                                                           .as_ref()))
            }
        }
    }


    #[doc="<p>Updates the specified group with the specified attributes.</p> <p>Requires developer credentials.</p>"]
    fn update_group(&self,
                    input: &UpdateGroupRequest)
                    -> Result<UpdateGroupResponse, UpdateGroupError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.UpdateGroup");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<UpdateGroupResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(UpdateGroupError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Updates identity provider information for a user pool.</p>"]
    fn update_identity_provider
        (&self,
         input: &UpdateIdentityProviderRequest)
         -> Result<UpdateIdentityProviderResponse, UpdateIdentityProviderError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.UpdateIdentityProvider");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<UpdateIdentityProviderResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(UpdateIdentityProviderError::from_body(String::from_utf8_lossy(&response.body)
                                                               .as_ref()))
            }
        }
    }


    #[doc="<p>Allows a user to update a specific attribute (one at a time).</p>"]
    fn update_user_attributes
        (&self,
         input: &UpdateUserAttributesRequest)
         -> Result<UpdateUserAttributesResponse, UpdateUserAttributesError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.UpdateUserAttributes");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<UpdateUserAttributesResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(UpdateUserAttributesError::from_body(String::from_utf8_lossy(&response.body)
                                                             .as_ref()))
            }
        }
    }


    #[doc="<p>Updates the specified user pool with the specified attributes.</p>"]
    fn update_user_pool(&self,
                        input: &UpdateUserPoolRequest)
                        -> Result<UpdateUserPoolResponse, UpdateUserPoolError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.UpdateUserPool");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<UpdateUserPoolResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(UpdateUserPoolError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Allows the developer to update the specified user pool client and password policy.</p>"]
    fn update_user_pool_client
        (&self,
         input: &UpdateUserPoolClientRequest)
         -> Result<UpdateUserPoolClientResponse, UpdateUserPoolClientError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.UpdateUserPoolClient");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<UpdateUserPoolClientResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(UpdateUserPoolClientError::from_body(String::from_utf8_lossy(&response.body)
                                                             .as_ref()))
            }
        }
    }


    #[doc="<p>Verifies the specified user attributes in the user pool.</p>"]
    fn verify_user_attribute(&self,
                             input: &VerifyUserAttributeRequest)
                             -> Result<VerifyUserAttributeResponse, VerifyUserAttributeError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSCognitoIdentityProviderService.VerifyUserAttribute");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<VerifyUserAttributeResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(VerifyUserAttributeError::from_body(String::from_utf8_lossy(&response.body)
                                                            .as_ref()))
            }
        }
    }
}

#[cfg(test)]
mod protocol_tests {}
