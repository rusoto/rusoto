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

use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_slice;
use serde_json::Value as SerdeJsonValue;
/// <p>Account takeover action type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccountTakeoverActionType {
    /// <p><p>The event action.</p> <ul> <li> <p> <code>BLOCK</code> Choosing this action will block the request.</p> </li> <li> <p> <code>MFA<em>IF</em>CONFIGURED</code> Throw MFA challenge if user has configured it, else allow the request.</p> </li> <li> <p> <code>MFA<em>REQUIRED</code> Throw MFA challenge if user has configured it, else block the request.</p> </li> <li> <p> <code>NO</em>ACTION</code> Allow the user sign-in.</p> </li> </ul></p>
    #[serde(rename = "EventAction")]
    pub event_action: String,
    /// <p>Flag specifying whether to send a notification.</p>
    #[serde(rename = "Notify")]
    pub notify: bool,
}

/// <p>Account takeover actions type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccountTakeoverActionsType {
    /// <p>Action to take for a high risk.</p>
    #[serde(rename = "HighAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_action: Option<AccountTakeoverActionType>,
    /// <p>Action to take for a low risk.</p>
    #[serde(rename = "LowAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_action: Option<AccountTakeoverActionType>,
    /// <p>Action to take for a medium risk.</p>
    #[serde(rename = "MediumAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium_action: Option<AccountTakeoverActionType>,
}

/// <p>Configuration for mitigation actions and notification for different levels of risk detected for a potential account takeover.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccountTakeoverRiskConfigurationType {
    /// <p>Account takeover risk configuration actions</p>
    #[serde(rename = "Actions")]
    pub actions: AccountTakeoverActionsType,
    /// <p>The notify configuration used to construct email notifications.</p>
    #[serde(rename = "NotifyConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_configuration: Option<NotifyConfigurationType>,
}

/// <p>Represents the request to add custom attributes.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AddCustomAttributesRequest {
    /// <p>An array of custom attributes, such as Mutable and Name.</p>
    #[serde(rename = "CustomAttributes")]
    pub custom_attributes: Vec<SchemaAttributeType>,
    /// <p>The user pool ID for the user pool where you want to add custom attributes.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

/// <p>Represents the response from the server for the request to add custom attributes.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AddCustomAttributesResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AdminAddUserToGroupRequest {
    /// <p>The group name.</p>
    #[serde(rename = "GroupName")]
    pub group_name: String,
    /// <p>The user pool ID for the user pool.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
    /// <p>The username for the user.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

/// <p>Represents the request to confirm user registration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AdminConfirmSignUpRequest {
    /// <p>The user pool ID for which you want to confirm user registration.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
    /// <p>The user name for which you want to confirm user registration.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

/// <p>Represents the response from the server for the request to confirm registration.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AdminConfirmSignUpResponse {}

/// <p>The configuration for creating a new user profile.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdminCreateUserConfigType {
    /// <p>Set to <code>True</code> if only the administrator is allowed to create user profiles. Set to <code>False</code> if users can sign themselves up via an app.</p>
    #[serde(rename = "AllowAdminCreateUserOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_admin_create_user_only: Option<bool>,
    /// <p>The message template to be used for the welcome message to new users.</p> <p>See also <a href="http://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pool-settings-message-customizations.html#cognito-user-pool-settings-user-invitation-message-customization">Customizing User Invitation Messages</a>.</p>
    #[serde(rename = "InviteMessageTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_message_template: Option<MessageTemplateType>,
    /// <p>The user account expiration limit, in days, after which the account is no longer usable. To reset the account after that time limit, you must call <code>AdminCreateUser</code> again, specifying <code>"RESEND"</code> for the <code>MessageAction</code> parameter. The default value for this parameter is 7.</p>
    #[serde(rename = "UnusedAccountValidityDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unused_account_validity_days: Option<i64>,
}

/// <p>Represents the request to create a user in the specified user pool.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AdminCreateUserRequest {
    /// <p>Specify <code>"EMAIL"</code> if email will be used to send the welcome message. Specify <code>"SMS"</code> if the phone number will be used. The default value is <code>"SMS"</code>. More than one value can be specified.</p>
    #[serde(rename = "DesiredDeliveryMediums")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_delivery_mediums: Option<Vec<String>>,
    /// <p>This parameter is only used if the <code>phone_number_verified</code> or <code>email_verified</code> attribute is set to <code>True</code>. Otherwise, it is ignored.</p> <p>If this parameter is set to <code>True</code> and the phone number or email address specified in the UserAttributes parameter already exists as an alias with a different user, the API call will migrate the alias from the previous user to the newly created user. The previous user will no longer be able to log in using that alias.</p> <p>If this parameter is set to <code>False</code>, the API throws an <code>AliasExistsException</code> error if the alias already exists. The default value is <code>False</code>.</p>
    #[serde(rename = "ForceAliasCreation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_alias_creation: Option<bool>,
    /// <p>Set to <code>"RESEND"</code> to resend the invitation message to a user that already exists and reset the expiration limit on the user's account. Set to <code>"SUPPRESS"</code> to suppress sending the message. Only one value can be specified.</p>
    #[serde(rename = "MessageAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_action: Option<String>,
    /// <p>The user's temporary password. This password must conform to the password policy that you specified when you created the user pool.</p> <p>The temporary password is valid only once. To complete the Admin Create User flow, the user must enter the temporary password in the sign-in page along with a new password to be used in all future sign-ins.</p> <p>This parameter is not required. If you do not specify a value, Amazon Cognito generates one for you.</p> <p>The temporary password can only be used until the user account expiration limit that you specified when you created the user pool. To reset the account after that time limit, you must call <code>AdminCreateUser</code> again, specifying <code>"RESEND"</code> for the <code>MessageAction</code> parameter.</p>
    #[serde(rename = "TemporaryPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporary_password: Option<String>,
    /// <p><p>An array of name-value pairs that contain user attributes and attribute values to be set for the user to be created. You can create a user without specifying any attributes other than <code>Username</code>. However, any attributes that you specify as required (in or in the <b>Attributes</b> tab of the console) must be supplied either by you (in your call to <code>AdminCreateUser</code>) or by the user (when he or she signs up in response to your welcome message).</p> <p>For custom attributes, you must prepend the <code>custom:</code> prefix to the attribute name.</p> <p>To send a message inviting the user to sign up, you must specify the user&#39;s email address or phone number. This can be done in your call to AdminCreateUser or in the <b>Users</b> tab of the Amazon Cognito console for managing your user pools.</p> <p>In your call to <code>AdminCreateUser</code>, you can set the <code>email<em>verified</code> attribute to <code>True</code>, and you can set the <code>phone</em>number<em>verified</code> attribute to <code>True</code>. (You can also do this by calling .)</p> <ul> <li> <p> <b>email</b>: The email address of the user to whom the message that contains the code and username will be sent. Required if the <code>email</em>verified</code> attribute is set to <code>True</code>, or if <code>&quot;EMAIL&quot;</code> is specified in the <code>DesiredDeliveryMediums</code> parameter.</p> </li> <li> <p> <b>phone<em>number</b>: The phone number of the user to whom the message that contains the code and username will be sent. Required if the <code>phone</em>number_verified</code> attribute is set to <code>True</code>, or if <code>&quot;SMS&quot;</code> is specified in the <code>DesiredDeliveryMediums</code> parameter.</p> </li> </ul></p>
    #[serde(rename = "UserAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_attributes: Option<Vec<AttributeType>>,
    /// <p>The user pool ID for the user pool where the user will be created.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
    /// <p>The username for the user. Must be unique within the user pool. Must be a UTF-8 string between 1 and 128 characters. After the user is created, the username cannot be changed.</p>
    #[serde(rename = "Username")]
    pub username: String,
    /// <p>The user's validation data. This is an array of name-value pairs that contain user attributes and attribute values that you can use for custom validation, such as restricting the types of user accounts that can be registered. For example, you might choose to allow or disallow user sign-up based on the user's domain.</p> <p>To configure custom validation, you must create a Pre Sign-up Lambda trigger for the user pool as described in the Amazon Cognito Developer Guide. The Lambda trigger receives the validation data and uses it in the validation process.</p> <p>The user's validation data is not persisted.</p>
    #[serde(rename = "ValidationData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_data: Option<Vec<AttributeType>>,
}

/// <p>Represents the response from the server to the request to create the user.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AdminCreateUserResponse {
    /// <p>The newly created user.</p>
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserType>,
}

/// <p>Represents the request to delete user attributes as an administrator.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AdminDeleteUserAttributesRequest {
    /// <p>An array of strings representing the user attribute names you wish to delete.</p> <p>For custom attributes, you must prepend the <code>custom:</code> prefix to the attribute name.</p>
    #[serde(rename = "UserAttributeNames")]
    pub user_attribute_names: Vec<String>,
    /// <p>The user pool ID for the user pool where you want to delete user attributes.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
    /// <p>The user name of the user from which you would like to delete attributes.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

/// <p>Represents the response received from the server for a request to delete user attributes.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AdminDeleteUserAttributesResponse {}

/// <p>Represents the request to delete a user as an administrator.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AdminDeleteUserRequest {
    /// <p>The user pool ID for the user pool where you want to delete the user.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
    /// <p>The user name of the user you wish to delete.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AdminDisableProviderForUserRequest {
    /// <p>The user to be disabled.</p>
    #[serde(rename = "User")]
    pub user: ProviderUserIdentifierType,
    /// <p>The user pool ID for the user pool.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AdminDisableProviderForUserResponse {}

/// <p>Represents the request to disable any user as an administrator.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AdminDisableUserRequest {
    /// <p>The user pool ID for the user pool where you want to disable the user.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
    /// <p>The user name of the user you wish to disable.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

/// <p>Represents the response received from the server to disable the user as an administrator.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AdminDisableUserResponse {}

/// <p>Represents the request that enables the user as an administrator.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AdminEnableUserRequest {
    /// <p>The user pool ID for the user pool where you want to enable the user.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
    /// <p>The user name of the user you wish to enable.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

/// <p>Represents the response from the server for the request to enable a user as an administrator.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AdminEnableUserResponse {}

/// <p>Sends the forgot device request, as an administrator.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AdminForgetDeviceRequest {
    /// <p>The device key.</p>
    #[serde(rename = "DeviceKey")]
    pub device_key: String,
    /// <p>The user pool ID.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
    /// <p>The user name.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

/// <p>Represents the request to get the device, as an administrator.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AdminGetDeviceRequest {
    /// <p>The device key.</p>
    #[serde(rename = "DeviceKey")]
    pub device_key: String,
    /// <p>The user pool ID.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
    /// <p>The user name.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

/// <p>Gets the device response, as an administrator.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AdminGetDeviceResponse {
    /// <p>The device.</p>
    #[serde(rename = "Device")]
    pub device: DeviceType,
}

/// <p>Represents the request to get the specified user as an administrator.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AdminGetUserRequest {
    /// <p>The user pool ID for the user pool where you want to get information about the user.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
    /// <p>The user name of the user you wish to retrieve.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

/// <p>Represents the response from the server from the request to get the specified user as an administrator.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AdminGetUserResponse {
    /// <p>Indicates that the status is enabled.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>Specifies the options for MFA (e.g., email or phone number).</p>
    #[serde(rename = "MFAOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_options: Option<Vec<MFAOptionType>>,
    /// <p>The user's preferred MFA setting.</p>
    #[serde(rename = "PreferredMfaSetting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_mfa_setting: Option<String>,
    /// <p>An array of name-value pairs representing user attributes.</p>
    #[serde(rename = "UserAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_attributes: Option<Vec<AttributeType>>,
    /// <p>The date the user was created.</p>
    #[serde(rename = "UserCreateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_create_date: Option<f64>,
    /// <p>The date the user was last modified.</p>
    #[serde(rename = "UserLastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_last_modified_date: Option<f64>,
    /// <p>The list of the user's MFA settings.</p>
    #[serde(rename = "UserMFASettingList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_mfa_setting_list: Option<Vec<String>>,
    /// <p><p>The user status. Can be one of the following:</p> <ul> <li> <p>UNCONFIRMED - User has been created but not confirmed.</p> </li> <li> <p>CONFIRMED - User has been confirmed.</p> </li> <li> <p>ARCHIVED - User is no longer active.</p> </li> <li> <p>COMPROMISED - User is disabled due to a potential security threat.</p> </li> <li> <p>UNKNOWN - User status is not known.</p> </li> </ul></p>
    #[serde(rename = "UserStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_status: Option<String>,
    /// <p>The user name of the user about whom you are receiving information.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

/// <p>Initiates the authorization request, as an administrator.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AdminInitiateAuthRequest {
    /// <p>The analytics metadata for collecting Amazon Pinpoint metrics for <code>AdminInitiateAuth</code> calls.</p>
    #[serde(rename = "AnalyticsMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_metadata: Option<AnalyticsMetadataType>,
    /// <p><p>The authentication flow for this call to execute. The API action will depend on this value. For example:</p> <ul> <li> <p> <code>REFRESH<em>TOKEN</em>AUTH</code> will take in a valid refresh token and return new tokens.</p> </li> <li> <p> <code>USER<em>SRP</em>AUTH</code> will take in <code>USERNAME</code> and <code>SRP<em>A</code> and return the SRP variables to be used for next challenge execution.</p> </li> <li> <p> <code>USER</em>PASSWORD<em>AUTH</code> will take in <code>USERNAME</code> and <code>PASSWORD</code> and return the next challenge or tokens.</p> </li> </ul> <p>Valid values include:</p> <ul> <li> <p> <code>USER</em>SRP<em>AUTH</code>: Authentication flow for the Secure Remote Password (SRP) protocol.</p> </li> <li> <p> <code>REFRESH</em>TOKEN<em>AUTH</code>/<code>REFRESH</em>TOKEN</code>: Authentication flow for refreshing the access token and ID token by supplying a valid refresh token.</p> </li> <li> <p> <code>CUSTOM<em>AUTH</code>: Custom authentication flow.</p> </li> <li> <p> <code>ADMIN</em>NO<em>SRP</em>AUTH</code>: Non-SRP authentication flow; you can pass in the USERNAME and PASSWORD directly if the flow is enabled for calling the app client.</p> </li> <li> <p> <code>USER<em>PASSWORD</em>AUTH</code>: Non-SRP authentication flow; USERNAME and PASSWORD are passed directly. If a user migration Lambda trigger is set, this flow will invoke the user migration Lambda if the USERNAME is not found in the user pool. </p> </li> </ul></p>
    #[serde(rename = "AuthFlow")]
    pub auth_flow: String,
    /// <p><p>The authentication parameters. These are inputs corresponding to the <code>AuthFlow</code> that you are invoking. The required values depend on the value of <code>AuthFlow</code>:</p> <ul> <li> <p>For <code>USER<em>SRP</em>AUTH</code>: <code>USERNAME</code> (required), <code>SRP<em>A</code> (required), <code>SECRET</em>HASH</code> (required if the app client is configured with a client secret), <code>DEVICE<em>KEY</code> </p> </li> <li> <p>For <code>REFRESH</em>TOKEN<em>AUTH/REFRESH</em>TOKEN</code>: <code>REFRESH<em>TOKEN</code> (required), <code>SECRET</em>HASH</code> (required if the app client is configured with a client secret), <code>DEVICE<em>KEY</code> </p> </li> <li> <p>For <code>ADMIN</em>NO<em>SRP</em>AUTH</code>: <code>USERNAME</code> (required), <code>SECRET<em>HASH</code> (if app client is configured with client secret), <code>PASSWORD</code> (required), <code>DEVICE</em>KEY</code> </p> </li> <li> <p>For <code>CUSTOM<em>AUTH</code>: <code>USERNAME</code> (required), <code>SECRET</em>HASH</code> (if app client is configured with client secret), <code>DEVICE_KEY</code> </p> </li> </ul></p>
    #[serde(rename = "AuthParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The app client ID.</p>
    #[serde(rename = "ClientId")]
    pub client_id: String,
    /// <p>This is a random key-value pair map which can contain any key and will be passed to your PreAuthentication Lambda trigger as-is. It can be used to implement additional validations around authentication.</p>
    #[serde(rename = "ClientMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_metadata: Option<::std::collections::HashMap<String, String>>,
    /// <p>Contextual data such as the user's device fingerprint, IP address, or location used for evaluating the risk of an unexpected event by Amazon Cognito advanced security.</p>
    #[serde(rename = "ContextData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_data: Option<ContextDataType>,
    /// <p>The ID of the Amazon Cognito user pool.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

/// <p>Initiates the authentication response, as an administrator.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AdminInitiateAuthResponse {
    /// <p>The result of the authentication response. This is only returned if the caller does not need to pass another challenge. If the caller does need to pass another challenge before it gets tokens, <code>ChallengeName</code>, <code>ChallengeParameters</code>, and <code>Session</code> are returned.</p>
    #[serde(rename = "AuthenticationResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_result: Option<AuthenticationResultType>,
    /// <p><p>The name of the challenge which you are responding to with this call. This is returned to you in the <code>AdminInitiateAuth</code> response if you need to pass another challenge.</p> <ul> <li> <p> <code>MFA<em>SETUP</code>: If MFA is required, users who do not have at least one of the MFA methods set up are presented with an <code>MFA</em>SETUP</code> challenge. The user must set up at least one MFA type to continue to authenticate.</p> </li> <li> <p> <code>SELECT<em>MFA</em>TYPE</code>: Selects the MFA type. Valid MFA options are <code>SMS<em>MFA</code> for text SMS MFA, and <code>SOFTWARE</em>TOKEN<em>MFA</code> for TOTP software token MFA.</p> </li> <li> <p> <code>SMS</em>MFA</code>: Next challenge is to supply an <code>SMS<em>MFA</em>CODE</code>, delivered via SMS.</p> </li> <li> <p> <code>PASSWORD<em>VERIFIER</code>: Next challenge is to supply <code>PASSWORD</em>CLAIM<em>SIGNATURE</code>, <code>PASSWORD</em>CLAIM<em>SECRET</em>BLOCK</code>, and <code>TIMESTAMP</code> after the client-side SRP calculations.</p> </li> <li> <p> <code>CUSTOM<em>CHALLENGE</code>: This is returned if your custom authentication flow determines that the user should pass another challenge before tokens are issued.</p> </li> <li> <p> <code>DEVICE</em>SRP<em>AUTH</code>: If device tracking was enabled on your user pool and the previous challenges were passed, this challenge is returned so that Amazon Cognito can start tracking this device.</p> </li> <li> <p> <code>DEVICE</em>PASSWORD<em>VERIFIER</code>: Similar to <code>PASSWORD</em>VERIFIER</code>, but for devices only.</p> </li> <li> <p> <code>ADMIN<em>NO</em>SRP<em>AUTH</code>: This is returned if you need to authenticate with <code>USERNAME</code> and <code>PASSWORD</code> directly. An app client must be enabled to use this flow.</p> </li> <li> <p> <code>NEW</em>PASSWORD<em>REQUIRED</code>: For users which are required to change their passwords after successful first login. This challenge should be passed with <code>NEW</em>PASSWORD</code> and any other required attributes.</p> </li> </ul></p>
    #[serde(rename = "ChallengeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_name: Option<String>,
    /// <p>The challenge parameters. These are returned to you in the <code>AdminInitiateAuth</code> response if you need to pass another challenge. The responses in this parameter should be used to compute inputs to the next call (<code>AdminRespondToAuthChallenge</code>).</p> <p>All challenges require <code>USERNAME</code> and <code>SECRET_HASH</code> (if applicable).</p> <p>The value of the <code>USER_ID_FOR_SRP</code> attribute will be the user's actual username, not an alias (such as email address or phone number), even if you specified an alias in your call to <code>AdminInitiateAuth</code>. This is because, in the <code>AdminRespondToAuthChallenge</code> API <code>ChallengeResponses</code>, the <code>USERNAME</code> attribute cannot be an alias.</p>
    #[serde(rename = "ChallengeParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The session which should be passed both ways in challenge-response calls to the service. If <code>AdminInitiateAuth</code> or <code>AdminRespondToAuthChallenge</code> API call determines that the caller needs to go through another challenge, they return a session with other challenge parameters. This session should be passed as it is to the next <code>AdminRespondToAuthChallenge</code> API call.</p>
    #[serde(rename = "Session")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AdminLinkProviderForUserRequest {
    /// <p>The existing user in the user pool to be linked to the external identity provider user account. Can be a native (Username + Password) Cognito User Pools user or a federated user (for example, a SAML or Facebook user). If the user doesn't exist, an exception is thrown. This is the user that is returned when the new user (with the linked identity provider attribute) signs in.</p> <p>For a native username + password user, the <code>ProviderAttributeValue</code> for the <code>DestinationUser</code> should be the username in the user pool. For a federated user, it should be the provider-specific <code>user_id</code>.</p> <p>The <code>ProviderAttributeName</code> of the <code>DestinationUser</code> is ignored.</p> <p>The <code>ProviderName</code> should be set to <code>Cognito</code> for users in Cognito user pools.</p>
    #[serde(rename = "DestinationUser")]
    pub destination_user: ProviderUserIdentifierType,
    /// <p>An external identity provider account for a user who does not currently exist yet in the user pool. This user must be a federated user (for example, a SAML or Facebook user), not another native user.</p> <p>If the <code>SourceUser</code> is a federated social identity provider user (Facebook, Google, or Login with Amazon), you must set the <code>ProviderAttributeName</code> to <code>Cognito_Subject</code>. For social identity providers, the <code>ProviderName</code> will be <code>Facebook</code>, <code>Google</code>, or <code>LoginWithAmazon</code>, and Cognito will automatically parse the Facebook, Google, and Login with Amazon tokens for <code>id</code>, <code>sub</code>, and <code>user_id</code>, respectively. The <code>ProviderAttributeValue</code> for the user must be the same value as the <code>id</code>, <code>sub</code>, or <code>user_id</code> value found in the social identity provider token.</p> <p/> <p>For SAML, the <code>ProviderAttributeName</code> can be any value that matches a claim in the SAML assertion. If you wish to link SAML users based on the subject of the SAML assertion, you should map the subject to a claim through the SAML identity provider and submit that claim name as the <code>ProviderAttributeName</code>. If you set <code>ProviderAttributeName</code> to <code>Cognito_Subject</code>, Cognito will automatically parse the default unique identifier found in the subject from the SAML token.</p>
    #[serde(rename = "SourceUser")]
    pub source_user: ProviderUserIdentifierType,
    /// <p>The user pool ID for the user pool.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AdminLinkProviderForUserResponse {}

/// <p>Represents the request to list devices, as an administrator.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AdminListDevicesRequest {
    /// <p>The limit of the devices request.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The pagination token.</p>
    #[serde(rename = "PaginationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
    /// <p>The user pool ID.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
    /// <p>The user name.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

/// <p>Lists the device's response, as an administrator.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AdminListDevicesResponse {
    /// <p>The devices in the list of devices response.</p>
    #[serde(rename = "Devices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<DeviceType>>,
    /// <p>The pagination token.</p>
    #[serde(rename = "PaginationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AdminListGroupsForUserRequest {
    /// <p>The limit of the request to list groups.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The user pool ID for the user pool.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
    /// <p>The username for the user.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AdminListGroupsForUserResponse {
    /// <p>The groups that the user belongs to.</p>
    #[serde(rename = "Groups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<GroupType>>,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AdminListUserAuthEventsRequest {
    /// <p>The maximum number of authentication events to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The user pool ID.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
    /// <p>The user pool username or an alias.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AdminListUserAuthEventsResponse {
    /// <p>The response object. It includes the <code>EventID</code>, <code>EventType</code>, <code>CreationDate</code>, <code>EventRisk</code>, and <code>EventResponse</code>.</p>
    #[serde(rename = "AuthEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_events: Option<Vec<AuthEventType>>,
    /// <p>A pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AdminRemoveUserFromGroupRequest {
    /// <p>The group name.</p>
    #[serde(rename = "GroupName")]
    pub group_name: String,
    /// <p>The user pool ID for the user pool.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
    /// <p>The username for the user.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

/// <p>Represents the request to reset a user's password as an administrator.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AdminResetUserPasswordRequest {
    /// <p>The user pool ID for the user pool where you want to reset the user's password.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
    /// <p>The user name of the user whose password you wish to reset.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

/// <p>Represents the response from the server to reset a user password as an administrator.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AdminResetUserPasswordResponse {}

/// <p>The request to respond to the authentication challenge, as an administrator.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AdminRespondToAuthChallengeRequest {
    /// <p>The analytics metadata for collecting Amazon Pinpoint metrics for <code>AdminRespondToAuthChallenge</code> calls.</p>
    #[serde(rename = "AnalyticsMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_metadata: Option<AnalyticsMetadataType>,
    /// <p>The challenge name. For more information, see .</p>
    #[serde(rename = "ChallengeName")]
    pub challenge_name: String,
    /// <p>The challenge responses. These are inputs corresponding to the value of <code>ChallengeName</code>, for example:</p> <ul> <li> <p> <code>SMS_MFA</code>: <code>SMS_MFA_CODE</code>, <code>USERNAME</code>, <code>SECRET_HASH</code> (if app client is configured with client secret).</p> </li> <li> <p> <code>PASSWORD_VERIFIER</code>: <code>PASSWORD_CLAIM_SIGNATURE</code>, <code>PASSWORD_CLAIM_SECRET_BLOCK</code>, <code>TIMESTAMP</code>, <code>USERNAME</code>, <code>SECRET_HASH</code> (if app client is configured with client secret).</p> </li> <li> <p> <code>ADMIN_NO_SRP_AUTH</code>: <code>PASSWORD</code>, <code>USERNAME</code>, <code>SECRET_HASH</code> (if app client is configured with client secret). </p> </li> <li> <p> <code>NEW_PASSWORD_REQUIRED</code>: <code>NEW_PASSWORD</code>, any other required attributes, <code>USERNAME</code>, <code>SECRET_HASH</code> (if app client is configured with client secret). </p> </li> </ul> <p>The value of the <code>USERNAME</code> attribute must be the user's actual username, not an alias (such as email address or phone number). To make this easier, the <code>AdminInitiateAuth</code> response includes the actual username value in the <code>USERNAMEUSER_ID_FOR_SRP</code> attribute, even if you specified an alias in your call to <code>AdminInitiateAuth</code>.</p>
    #[serde(rename = "ChallengeResponses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_responses: Option<::std::collections::HashMap<String, String>>,
    /// <p>The app client ID.</p>
    #[serde(rename = "ClientId")]
    pub client_id: String,
    /// <p>Contextual data such as the user's device fingerprint, IP address, or location used for evaluating the risk of an unexpected event by Amazon Cognito advanced security.</p>
    #[serde(rename = "ContextData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_data: Option<ContextDataType>,
    /// <p>The session which should be passed both ways in challenge-response calls to the service. If <code>InitiateAuth</code> or <code>RespondToAuthChallenge</code> API call determines that the caller needs to go through another challenge, they return a session with other challenge parameters. This session should be passed as it is to the next <code>RespondToAuthChallenge</code> API call.</p>
    #[serde(rename = "Session")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
    /// <p>The ID of the Amazon Cognito user pool.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

/// <p>Responds to the authentication challenge, as an administrator.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AdminRespondToAuthChallengeResponse {
    /// <p>The result returned by the server in response to the authentication request.</p>
    #[serde(rename = "AuthenticationResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_result: Option<AuthenticationResultType>,
    /// <p>The name of the challenge. For more information, see .</p>
    #[serde(rename = "ChallengeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_name: Option<String>,
    /// <p>The challenge parameters. For more information, see .</p>
    #[serde(rename = "ChallengeParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The session which should be passed both ways in challenge-response calls to the service. If the or API call determines that the caller needs to go through another challenge, they return a session with other challenge parameters. This session should be passed as it is to the next <code>RespondToAuthChallenge</code> API call.</p>
    #[serde(rename = "Session")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AdminSetUserMFAPreferenceRequest {
    /// <p>The SMS text message MFA settings.</p>
    #[serde(rename = "SMSMfaSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_mfa_settings: Option<SMSMfaSettingsType>,
    /// <p>The time-based one-time password software token MFA settings.</p>
    #[serde(rename = "SoftwareTokenMfaSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub software_token_mfa_settings: Option<SoftwareTokenMfaSettingsType>,
    /// <p>The user pool ID.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
    /// <p>The user pool username or alias.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AdminSetUserMFAPreferenceResponse {}

/// <p>Represents the request to set user settings as an administrator.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AdminSetUserSettingsRequest {
    /// <p>Specifies the options for MFA (e.g., email or phone number).</p>
    #[serde(rename = "MFAOptions")]
    pub mfa_options: Vec<MFAOptionType>,
    /// <p>The user pool ID for the user pool where you want to set the user's settings, such as MFA options.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
    /// <p>The user name of the user for whom you wish to set user settings.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

/// <p>Represents the response from the server to set user settings as an administrator.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AdminSetUserSettingsResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AdminUpdateAuthEventFeedbackRequest {
    /// <p>The authentication event ID.</p>
    #[serde(rename = "EventId")]
    pub event_id: String,
    /// <p>The authentication event feedback value.</p>
    #[serde(rename = "FeedbackValue")]
    pub feedback_value: String,
    /// <p>The user pool ID.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
    /// <p>The user pool username.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AdminUpdateAuthEventFeedbackResponse {}

/// <p>The request to update the device status, as an administrator.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AdminUpdateDeviceStatusRequest {
    /// <p>The device key.</p>
    #[serde(rename = "DeviceKey")]
    pub device_key: String,
    /// <p>The status indicating whether a device has been remembered or not.</p>
    #[serde(rename = "DeviceRememberedStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_remembered_status: Option<String>,
    /// <p>The user pool ID.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
    /// <p>The user name.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

/// <p>The status response from the request to update the device, as an administrator.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AdminUpdateDeviceStatusResponse {}

/// <p>Represents the request to update the user's attributes as an administrator.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AdminUpdateUserAttributesRequest {
    /// <p>An array of name-value pairs representing user attributes.</p> <p>For custom attributes, you must prepend the <code>custom:</code> prefix to the attribute name.</p>
    #[serde(rename = "UserAttributes")]
    pub user_attributes: Vec<AttributeType>,
    /// <p>The user pool ID for the user pool where you want to update user attributes.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
    /// <p>The user name of the user for whom you want to update user attributes.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

/// <p>Represents the response from the server for the request to update user attributes as an administrator.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AdminUpdateUserAttributesResponse {}

/// <p>The request to sign out of all devices, as an administrator.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AdminUserGlobalSignOutRequest {
    /// <p>The user pool ID.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
    /// <p>The user name.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

/// <p>The global sign-out response, as an administrator.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AdminUserGlobalSignOutResponse {}

/// <p>The Amazon Pinpoint analytics configuration for collecting metrics for a user pool.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnalyticsConfigurationType {
    /// <p>The application ID for an Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The external ID.</p>
    #[serde(rename = "ExternalId")]
    pub external_id: String,
    /// <p>The ARN of an IAM role that authorizes Amazon Cognito to publish events to Amazon Pinpoint analytics.</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// <p>If <code>UserDataShared</code> is <code>true</code>, Amazon Cognito will include user data in the events it publishes to Amazon Pinpoint analytics.</p>
    #[serde(rename = "UserDataShared")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data_shared: Option<bool>,
}

/// <p>An Amazon Pinpoint analytics endpoint.</p> <p>An endpoint uniquely identifies a mobile device, email address, or phone number that can receive messages from Amazon Pinpoint analytics.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AnalyticsMetadataType {
    /// <p>The endpoint ID.</p>
    #[serde(rename = "AnalyticsEndpointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_endpoint_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssociateSoftwareTokenRequest {
    /// <p>The access token.</p>
    #[serde(rename = "AccessToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    /// <p>The session which should be passed both ways in challenge-response calls to the service. This allows authentication of the user as part of the MFA setup process.</p>
    #[serde(rename = "Session")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AssociateSoftwareTokenResponse {
    /// <p>A unique generated shared secret code that is used in the TOTP algorithm to generate a one time code.</p>
    #[serde(rename = "SecretCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_code: Option<String>,
    /// <p>The session which should be passed both ways in challenge-response calls to the service. This allows authentication of the user as part of the MFA setup process.</p>
    #[serde(rename = "Session")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
}

/// <p>Specifies whether the attribute is standard or custom.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributeType {
    /// <p>The name of the attribute.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The value of the attribute.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>The authentication event type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AuthEventType {
    /// <p>The challenge responses.</p>
    #[serde(rename = "ChallengeResponses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_responses: Option<Vec<ChallengeResponseType>>,
    /// <p>The creation date</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>The user context data captured at the time of an event request. It provides additional information about the client from which event the request is received.</p>
    #[serde(rename = "EventContextData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_context_data: Option<EventContextDataType>,
    /// <p>A flag specifying the user feedback captured at the time of an event request is good or bad. </p>
    #[serde(rename = "EventFeedback")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_feedback: Option<EventFeedbackType>,
    /// <p>The event ID.</p>
    #[serde(rename = "EventId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// <p>The event response.</p>
    #[serde(rename = "EventResponse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_response: Option<String>,
    /// <p>The event risk.</p>
    #[serde(rename = "EventRisk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_risk: Option<EventRiskType>,
    /// <p>The event type.</p>
    #[serde(rename = "EventType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
}

/// <p>The authentication result.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AuthenticationResultType {
    /// <p>The access token.</p>
    #[serde(rename = "AccessToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    /// <p>The expiration period of the authentication result in seconds.</p>
    #[serde(rename = "ExpiresIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<i64>,
    /// <p>The ID token.</p>
    #[serde(rename = "IdToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token: Option<String>,
    /// <p>The new device metadata from an authentication result.</p>
    #[serde(rename = "NewDeviceMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_device_metadata: Option<NewDeviceMetadataType>,
    /// <p>The refresh token.</p>
    #[serde(rename = "RefreshToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    /// <p>The token type.</p>
    #[serde(rename = "TokenType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_type: Option<String>,
}

/// <p>The challenge response type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ChallengeResponseType {
    /// <p>The challenge name</p>
    #[serde(rename = "ChallengeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_name: Option<String>,
    /// <p>The challenge response.</p>
    #[serde(rename = "ChallengeResponse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_response: Option<String>,
}

/// <p>Represents the request to change a user password.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ChangePasswordRequest {
    /// <p>The access token.</p>
    #[serde(rename = "AccessToken")]
    pub access_token: String,
    /// <p>The old password.</p>
    #[serde(rename = "PreviousPassword")]
    pub previous_password: String,
    /// <p>The new password.</p>
    #[serde(rename = "ProposedPassword")]
    pub proposed_password: String,
}

/// <p>The response from the server to the change password request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ChangePasswordResponse {}

/// <p>The code delivery details being returned from the server.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CodeDeliveryDetailsType {
    /// <p>The attribute name.</p>
    #[serde(rename = "AttributeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    /// <p>The delivery medium (email message or phone number).</p>
    #[serde(rename = "DeliveryMedium")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_medium: Option<String>,
    /// <p>The destination for the code delivery details.</p>
    #[serde(rename = "Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
}

/// <p>The compromised credentials actions type</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompromisedCredentialsActionsType {
    /// <p>The event action.</p>
    #[serde(rename = "EventAction")]
    pub event_action: String,
}

/// <p>The compromised credentials risk configuration type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompromisedCredentialsRiskConfigurationType {
    /// <p>The compromised credentials risk configuration actions.</p>
    #[serde(rename = "Actions")]
    pub actions: CompromisedCredentialsActionsType,
    /// <p>Perform the action for these events. The default is to perform all events if no event filter is specified.</p>
    #[serde(rename = "EventFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_filter: Option<Vec<String>>,
}

/// <p>Confirms the device request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ConfirmDeviceRequest {
    /// <p>The access token.</p>
    #[serde(rename = "AccessToken")]
    pub access_token: String,
    /// <p>The device key.</p>
    #[serde(rename = "DeviceKey")]
    pub device_key: String,
    /// <p>The device name.</p>
    #[serde(rename = "DeviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    /// <p>The configuration of the device secret verifier.</p>
    #[serde(rename = "DeviceSecretVerifierConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_secret_verifier_config: Option<DeviceSecretVerifierConfigType>,
}

/// <p>Confirms the device response.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ConfirmDeviceResponse {
    /// <p>Indicates whether the user confirmation is necessary to confirm the device response.</p>
    #[serde(rename = "UserConfirmationNecessary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_confirmation_necessary: Option<bool>,
}

/// <p>The request representing the confirmation for a password reset.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ConfirmForgotPasswordRequest {
    /// <p>The Amazon Pinpoint analytics metadata for collecting metrics for <code>ConfirmForgotPassword</code> calls.</p>
    #[serde(rename = "AnalyticsMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_metadata: Option<AnalyticsMetadataType>,
    /// <p>The app client ID of the app associated with the user pool.</p>
    #[serde(rename = "ClientId")]
    pub client_id: String,
    /// <p>The confirmation code sent by a user's request to retrieve a forgotten password. For more information, see </p>
    #[serde(rename = "ConfirmationCode")]
    pub confirmation_code: String,
    /// <p>The password sent by a user's request to retrieve a forgotten password.</p>
    #[serde(rename = "Password")]
    pub password: String,
    /// <p>A keyed-hash message authentication code (HMAC) calculated using the secret key of a user pool client and username plus the client ID in the message.</p>
    #[serde(rename = "SecretHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_hash: Option<String>,
    /// <p>Contextual data such as the user's device fingerprint, IP address, or location used for evaluating the risk of an unexpected event by Amazon Cognito advanced security.</p>
    #[serde(rename = "UserContextData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_context_data: Option<UserContextDataType>,
    /// <p>The user name of the user for whom you want to enter a code to retrieve a forgotten password.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

/// <p>The response from the server that results from a user's request to retrieve a forgotten password.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ConfirmForgotPasswordResponse {}

/// <p>Represents the request to confirm registration of a user.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ConfirmSignUpRequest {
    /// <p>The Amazon Pinpoint analytics metadata for collecting metrics for <code>ConfirmSignUp</code> calls.</p>
    #[serde(rename = "AnalyticsMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_metadata: Option<AnalyticsMetadataType>,
    /// <p>The ID of the app client associated with the user pool.</p>
    #[serde(rename = "ClientId")]
    pub client_id: String,
    /// <p>The confirmation code sent by a user's request to confirm registration.</p>
    #[serde(rename = "ConfirmationCode")]
    pub confirmation_code: String,
    /// <p>Boolean to be specified to force user confirmation irrespective of existing alias. By default set to <code>False</code>. If this parameter is set to <code>True</code> and the phone number/email used for sign up confirmation already exists as an alias with a different user, the API call will migrate the alias from the previous user to the newly created user being confirmed. If set to <code>False</code>, the API will throw an <b>AliasExistsException</b> error.</p>
    #[serde(rename = "ForceAliasCreation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_alias_creation: Option<bool>,
    /// <p>A keyed-hash message authentication code (HMAC) calculated using the secret key of a user pool client and username plus the client ID in the message.</p>
    #[serde(rename = "SecretHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_hash: Option<String>,
    /// <p>Contextual data such as the user's device fingerprint, IP address, or location used for evaluating the risk of an unexpected event by Amazon Cognito advanced security.</p>
    #[serde(rename = "UserContextData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_context_data: Option<UserContextDataType>,
    /// <p>The user name of the user whose registration you wish to confirm.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

/// <p>Represents the response from the server for the registration confirmation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ConfirmSignUpResponse {}

/// <p>Contextual user data type used for evaluating the risk of an unexpected event by Amazon Cognito advanced security.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ContextDataType {
    /// <p>Encoded data containing device fingerprinting details, collected using the Amazon Cognito context data collection library.</p>
    #[serde(rename = "EncodedData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoded_data: Option<String>,
    /// <p>HttpHeaders received on your server in same order.</p>
    #[serde(rename = "HttpHeaders")]
    pub http_headers: Vec<HttpHeader>,
    /// <p>Source IP address of your user.</p>
    #[serde(rename = "IpAddress")]
    pub ip_address: String,
    /// <p>Your server endpoint where this API is invoked.</p>
    #[serde(rename = "ServerName")]
    pub server_name: String,
    /// <p>Your server path where this API is invoked. </p>
    #[serde(rename = "ServerPath")]
    pub server_path: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateGroupRequest {
    /// <p>A string containing the description of the group.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the group. Must be unique.</p>
    #[serde(rename = "GroupName")]
    pub group_name: String,
    /// <p>A nonnegative integer value that specifies the precedence of this group relative to the other groups that a user can belong to in the user pool. Zero is the highest precedence value. Groups with lower <code>Precedence</code> values take precedence over groups with higher or null <code>Precedence</code> values. If a user belongs to two or more groups, it is the group with the lowest precedence value whose role ARN will be used in the <code>cognito:roles</code> and <code>cognito:preferred_role</code> claims in the user's tokens.</p> <p>Two groups can have the same <code>Precedence</code> value. If this happens, neither group takes precedence over the other. If two groups with the same <code>Precedence</code> have the same role ARN, that role is used in the <code>cognito:preferred_role</code> claim in tokens for users in each group. If the two groups have different role ARNs, the <code>cognito:preferred_role</code> claim is not set in users' tokens.</p> <p>The default <code>Precedence</code> value is null.</p>
    #[serde(rename = "Precedence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precedence: Option<i64>,
    /// <p>The role ARN for the group.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The user pool ID for the user pool.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateGroupResponse {
    /// <p>The group object for the group.</p>
    #[serde(rename = "Group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<GroupType>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateIdentityProviderRequest {
    /// <p>A mapping of identity provider attributes to standard and custom user pool attributes.</p>
    #[serde(rename = "AttributeMapping")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_mapping: Option<::std::collections::HashMap<String, String>>,
    /// <p>A list of identity provider identifiers.</p>
    #[serde(rename = "IdpIdentifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idp_identifiers: Option<Vec<String>>,
    /// <p>The identity provider details, such as <code>MetadataURL</code> and <code>MetadataFile</code>.</p>
    #[serde(rename = "ProviderDetails")]
    pub provider_details: ::std::collections::HashMap<String, String>,
    /// <p>The identity provider name.</p>
    #[serde(rename = "ProviderName")]
    pub provider_name: String,
    /// <p>The identity provider type.</p>
    #[serde(rename = "ProviderType")]
    pub provider_type: String,
    /// <p>The user pool ID.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateIdentityProviderResponse {
    /// <p>The newly created identity provider object.</p>
    #[serde(rename = "IdentityProvider")]
    pub identity_provider: IdentityProviderType,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateResourceServerRequest {
    /// <p>A unique resource server identifier for the resource server. This could be an HTTPS endpoint where the resource server is located. For example, <code>https://my-weather-api.example.com</code>.</p>
    #[serde(rename = "Identifier")]
    pub identifier: String,
    /// <p>A friendly name for the resource server.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>A list of scopes. Each scope is map, where the keys are <code>name</code> and <code>description</code>.</p>
    #[serde(rename = "Scopes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<ResourceServerScopeType>>,
    /// <p>The user pool ID for the user pool.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateResourceServerResponse {
    /// <p>The newly created resource server.</p>
    #[serde(rename = "ResourceServer")]
    pub resource_server: ResourceServerType,
}

/// <p>Represents the request to create the user import job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateUserImportJobRequest {
    /// <p>The role ARN for the Amazon CloudWatch Logging role for the user import job.</p>
    #[serde(rename = "CloudWatchLogsRoleArn")]
    pub cloud_watch_logs_role_arn: String,
    /// <p>The job name for the user import job.</p>
    #[serde(rename = "JobName")]
    pub job_name: String,
    /// <p>The user pool ID for the user pool that the users are being imported into.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

/// <p>Represents the response from the server to the request to create the user import job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateUserImportJobResponse {
    /// <p>The job object that represents the user import job.</p>
    #[serde(rename = "UserImportJob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_import_job: Option<UserImportJobType>,
}

/// <p>Represents the request to create a user pool client.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateUserPoolClientRequest {
    /// <p>Set to <code>code</code> to initiate a code grant flow, which provides an authorization code as the response. This code can be exchanged for access tokens with the token endpoint.</p> <p>Set to <code>token</code> to specify that the client should get the access token (and, optionally, ID token, based on scopes) directly.</p>
    #[serde(rename = "AllowedOAuthFlows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_o_auth_flows: Option<Vec<String>>,
    /// <p>Set to <code>True</code> if the client is allowed to follow the OAuth protocol when interacting with Cognito user pools.</p>
    #[serde(rename = "AllowedOAuthFlowsUserPoolClient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_o_auth_flows_user_pool_client: Option<bool>,
    /// <p>A list of allowed <code>OAuth</code> scopes. Currently supported values are <code>"phone"</code>, <code>"email"</code>, <code>"openid"</code>, and <code>"Cognito"</code>.</p>
    #[serde(rename = "AllowedOAuthScopes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_o_auth_scopes: Option<Vec<String>>,
    /// <p>The Amazon Pinpoint analytics configuration for collecting metrics for this user pool.</p>
    #[serde(rename = "AnalyticsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_configuration: Option<AnalyticsConfigurationType>,
    /// <p>A list of allowed redirect (callback) URLs for the identity providers.</p> <p>A redirect URI must:</p> <ul> <li> <p>Be an absolute URI.</p> </li> <li> <p>Be registered with the authorization server.</p> </li> <li> <p>Not include a fragment component.</p> </li> </ul> <p>See <a href="https://tools.ietf.org/html/rfc6749#section-3.1.2">OAuth 2.0 - Redirection Endpoint</a>.</p> <p>Amazon Cognito requires HTTPS over HTTP except for http://localhost for testing purposes only.</p> <p>App callback URLs such as myapp://example are also supported.</p>
    #[serde(rename = "CallbackURLs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_ur_ls: Option<Vec<String>>,
    /// <p>The client name for the user pool client you would like to create.</p>
    #[serde(rename = "ClientName")]
    pub client_name: String,
    /// <p>The default redirect URI. Must be in the <code>CallbackURLs</code> list.</p> <p>A redirect URI must:</p> <ul> <li> <p>Be an absolute URI.</p> </li> <li> <p>Be registered with the authorization server.</p> </li> <li> <p>Not include a fragment component.</p> </li> </ul> <p>See <a href="https://tools.ietf.org/html/rfc6749#section-3.1.2">OAuth 2.0 - Redirection Endpoint</a>.</p> <p>Amazon Cognito requires HTTPS over HTTP except for http://localhost for testing purposes only.</p> <p>App callback URLs such as myapp://example are also supported.</p>
    #[serde(rename = "DefaultRedirectURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_redirect_uri: Option<String>,
    /// <p>The explicit authentication flows.</p>
    #[serde(rename = "ExplicitAuthFlows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_auth_flows: Option<Vec<String>>,
    /// <p>Boolean to specify whether you want to generate a secret for the user pool client being created.</p>
    #[serde(rename = "GenerateSecret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_secret: Option<bool>,
    /// <p>A list of allowed logout URLs for the identity providers.</p>
    #[serde(rename = "LogoutURLs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logout_ur_ls: Option<Vec<String>>,
    /// <p>The read attributes.</p>
    #[serde(rename = "ReadAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_attributes: Option<Vec<String>>,
    /// <p>The time limit, in days, after which the refresh token is no longer valid and cannot be used.</p>
    #[serde(rename = "RefreshTokenValidity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token_validity: Option<i64>,
    /// <p>A list of provider names for the identity providers that are supported on this client.</p>
    #[serde(rename = "SupportedIdentityProviders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_identity_providers: Option<Vec<String>>,
    /// <p>The user pool ID for the user pool where you want to create a user pool client.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
    /// <p>The user pool attributes that the app client can write to.</p> <p>If your app client allows users to sign in through an identity provider, this array must include all attributes that are mapped to identity provider attributes. Amazon Cognito updates mapped attributes when users sign in to your application through an identity provider. If your app client lacks write access to a mapped attribute, Amazon Cognito throws an error when it attempts to update the attribute. For more information, see <a href="http://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-specifying-attribute-mapping.html">Specifying Identity Provider Attribute Mappings for Your User Pool</a>.</p>
    #[serde(rename = "WriteAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_attributes: Option<Vec<String>>,
}

/// <p>Represents the response from the server to create a user pool client.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateUserPoolClientResponse {
    /// <p>The user pool client that was just created.</p>
    #[serde(rename = "UserPoolClient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_client: Option<UserPoolClientType>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateUserPoolDomainRequest {
    /// <p>The configuration for a custom domain that hosts the sign-up and sign-in webpages for your application.</p> <p>Provide this parameter only if you want to use a custom domain for your user pool. Otherwise, you can exclude this parameter and use the Amazon Cognito hosted domain instead.</p> <p>For more information about the hosted domain and custom domains, see <a href="http://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-assign-domain.html">Configuring a User Pool Domain</a>.</p>
    #[serde(rename = "CustomDomainConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_domain_config: Option<CustomDomainConfigType>,
    /// <p>The domain string.</p>
    #[serde(rename = "Domain")]
    pub domain: String,
    /// <p>The user pool ID.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateUserPoolDomainResponse {
    /// <p>The Amazon CloudFront endpoint that you use as the target of the alias that you set up with your Domain Name Service (DNS) provider.</p>
    #[serde(rename = "CloudFrontDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_front_domain: Option<String>,
}

/// <p>Represents the request to create a user pool.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateUserPoolRequest {
    /// <p>The configuration for <code>AdminCreateUser</code> requests.</p>
    #[serde(rename = "AdminCreateUserConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_create_user_config: Option<AdminCreateUserConfigType>,
    /// <p>Attributes supported as an alias for this user pool. Possible values: <b>phone_number</b>, <b>email</b>, or <b>preferred_username</b>.</p>
    #[serde(rename = "AliasAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_attributes: Option<Vec<String>>,
    /// <p>The attributes to be auto-verified. Possible values: <b>email</b>, <b>phone_number</b>.</p>
    #[serde(rename = "AutoVerifiedAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_verified_attributes: Option<Vec<String>>,
    /// <p>The device configuration.</p>
    #[serde(rename = "DeviceConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_configuration: Option<DeviceConfigurationType>,
    /// <p>The email configuration.</p>
    #[serde(rename = "EmailConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_configuration: Option<EmailConfigurationType>,
    /// <p>A string representing the email verification message.</p>
    #[serde(rename = "EmailVerificationMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verification_message: Option<String>,
    /// <p>A string representing the email verification subject.</p>
    #[serde(rename = "EmailVerificationSubject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verification_subject: Option<String>,
    /// <p><p>The Lambda trigger configuration information for the new user pool.</p> <note> <p>In a push model, event sources (such as Amazon S3 and custom applications) need permission to invoke a function. So you will need to make an extra call to add permission for these event sources to invoke your Lambda function.</p> <p/> <p>For more information on using the Lambda API to add permission, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/API_AddPermission.html"> AddPermission </a>. </p> <p>For adding permission using the AWS CLI, see <a href="https://docs.aws.amazon.com/cli/latest/reference/lambda/add-permission.html"> add-permission </a>.</p> </note></p>
    #[serde(rename = "LambdaConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_config: Option<LambdaConfigType>,
    /// <p>Specifies MFA configuration details.</p>
    #[serde(rename = "MfaConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_configuration: Option<String>,
    /// <p>The policies associated with the new user pool.</p>
    #[serde(rename = "Policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<UserPoolPolicyType>,
    /// <p>A string used to name the user pool.</p>
    #[serde(rename = "PoolName")]
    pub pool_name: String,
    /// <p>An array of schema attributes for the new user pool. These attributes can be standard or custom attributes.</p>
    #[serde(rename = "Schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Vec<SchemaAttributeType>>,
    /// <p>A string representing the SMS authentication message.</p>
    #[serde(rename = "SmsAuthenticationMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_authentication_message: Option<String>,
    /// <p>The SMS configuration.</p>
    #[serde(rename = "SmsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_configuration: Option<SmsConfigurationType>,
    /// <p>A string representing the SMS verification message.</p>
    #[serde(rename = "SmsVerificationMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_verification_message: Option<String>,
    /// <p>Used to enable advanced security risk detection. Set the key <code>AdvancedSecurityMode</code> to the value "AUDIT".</p>
    #[serde(rename = "UserPoolAddOns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_add_ons: Option<UserPoolAddOnsType>,
    /// <p>The cost allocation tags for the user pool. For more information, see <a href="http://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-cost-allocation-tagging.html">Adding Cost Allocation Tags to Your User Pool</a> </p>
    #[serde(rename = "UserPoolTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>Specifies whether email addresses or phone numbers can be specified as usernames when a user signs up.</p>
    #[serde(rename = "UsernameAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username_attributes: Option<Vec<String>>,
    /// <p>The template for the verification message that the user sees when the app requests permission to access the user's information.</p>
    #[serde(rename = "VerificationMessageTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_message_template: Option<VerificationMessageTemplateType>,
}

/// <p>Represents the response from the server for the request to create a user pool.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateUserPoolResponse {
    /// <p>A container for the user pool details.</p>
    #[serde(rename = "UserPool")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool: Option<UserPoolType>,
}

/// <p>The configuration for a custom domain that hosts the sign-up and sign-in webpages for your application.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomDomainConfigType {
    /// <p>The Amazon Resource Name (ARN) of an AWS Certificate Manager SSL certificate. You use this certificate for the subdomain of your custom domain.</p>
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteGroupRequest {
    /// <p>The name of the group.</p>
    #[serde(rename = "GroupName")]
    pub group_name: String,
    /// <p>The user pool ID for the user pool.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteIdentityProviderRequest {
    /// <p>The identity provider name.</p>
    #[serde(rename = "ProviderName")]
    pub provider_name: String,
    /// <p>The user pool ID.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteResourceServerRequest {
    /// <p>The identifier for the resource server.</p>
    #[serde(rename = "Identifier")]
    pub identifier: String,
    /// <p>The user pool ID for the user pool that hosts the resource server.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

/// <p>Represents the request to delete user attributes.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteUserAttributesRequest {
    /// <p>The access token used in the request to delete user attributes.</p>
    #[serde(rename = "AccessToken")]
    pub access_token: String,
    /// <p>An array of strings representing the user attribute names you wish to delete.</p> <p>For custom attributes, you must prepend the <code>custom:</code> prefix to the attribute name.</p>
    #[serde(rename = "UserAttributeNames")]
    pub user_attribute_names: Vec<String>,
}

/// <p>Represents the response from the server to delete user attributes.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteUserAttributesResponse {}

/// <p>Represents the request to delete a user pool client.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteUserPoolClientRequest {
    /// <p>The app client ID of the app associated with the user pool.</p>
    #[serde(rename = "ClientId")]
    pub client_id: String,
    /// <p>The user pool ID for the user pool where you want to delete the client.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteUserPoolDomainRequest {
    /// <p>The domain string.</p>
    #[serde(rename = "Domain")]
    pub domain: String,
    /// <p>The user pool ID.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteUserPoolDomainResponse {}

/// <p>Represents the request to delete a user pool.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteUserPoolRequest {
    /// <p>The user pool ID for the user pool you want to delete.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

/// <p>Represents the request to delete a user.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteUserRequest {
    /// <p>The access token from a request to delete a user.</p>
    #[serde(rename = "AccessToken")]
    pub access_token: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeIdentityProviderRequest {
    /// <p>The identity provider name.</p>
    #[serde(rename = "ProviderName")]
    pub provider_name: String,
    /// <p>The user pool ID.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeIdentityProviderResponse {
    /// <p>The identity provider that was deleted.</p>
    #[serde(rename = "IdentityProvider")]
    pub identity_provider: IdentityProviderType,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeResourceServerRequest {
    /// <p>The identifier for the resource server</p>
    #[serde(rename = "Identifier")]
    pub identifier: String,
    /// <p>The user pool ID for the user pool that hosts the resource server.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeResourceServerResponse {
    /// <p>The resource server.</p>
    #[serde(rename = "ResourceServer")]
    pub resource_server: ResourceServerType,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeRiskConfigurationRequest {
    /// <p>The app client ID.</p>
    #[serde(rename = "ClientId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// <p>The user pool ID.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeRiskConfigurationResponse {
    /// <p>The risk configuration.</p>
    #[serde(rename = "RiskConfiguration")]
    pub risk_configuration: RiskConfigurationType,
}

/// <p>Represents the request to describe the user import job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeUserImportJobRequest {
    /// <p>The job ID for the user import job.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
    /// <p>The user pool ID for the user pool that the users are being imported into.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

/// <p>Represents the response from the server to the request to describe the user import job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeUserImportJobResponse {
    /// <p>The job object that represents the user import job.</p>
    #[serde(rename = "UserImportJob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_import_job: Option<UserImportJobType>,
}

/// <p>Represents the request to describe a user pool client.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeUserPoolClientRequest {
    /// <p>The app client ID of the app associated with the user pool.</p>
    #[serde(rename = "ClientId")]
    pub client_id: String,
    /// <p>The user pool ID for the user pool you want to describe.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

/// <p>Represents the response from the server from a request to describe the user pool client.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeUserPoolClientResponse {
    /// <p>The user pool client from a server response to describe the user pool client.</p>
    #[serde(rename = "UserPoolClient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_client: Option<UserPoolClientType>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeUserPoolDomainRequest {
    /// <p>The domain string.</p>
    #[serde(rename = "Domain")]
    pub domain: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeUserPoolDomainResponse {
    /// <p>A domain description object containing information about the domain.</p>
    #[serde(rename = "DomainDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_description: Option<DomainDescriptionType>,
}

/// <p>Represents the request to describe the user pool.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeUserPoolRequest {
    /// <p>The user pool ID for the user pool you want to describe.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

/// <p>Represents the response to describe the user pool.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeUserPoolResponse {
    /// <p>The container of metadata returned by the server to describe the pool.</p>
    #[serde(rename = "UserPool")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool: Option<UserPoolType>,
}

/// <p>The configuration for the user pool's device tracking.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceConfigurationType {
    /// <p>Indicates whether a challenge is required on a new device. Only applicable to a new device.</p>
    #[serde(rename = "ChallengeRequiredOnNewDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_required_on_new_device: Option<bool>,
    /// <p>If true, a device is only remembered on user prompt.</p>
    #[serde(rename = "DeviceOnlyRememberedOnUserPrompt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_only_remembered_on_user_prompt: Option<bool>,
}

/// <p>The device verifier against which it will be authenticated.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeviceSecretVerifierConfigType {
    /// <p>The password verifier.</p>
    #[serde(rename = "PasswordVerifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_verifier: Option<String>,
    /// <p>The salt.</p>
    #[serde(rename = "Salt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salt: Option<String>,
}

/// <p>The device type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeviceType {
    /// <p>The device attributes.</p>
    #[serde(rename = "DeviceAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_attributes: Option<Vec<AttributeType>>,
    /// <p>The creation date of the device.</p>
    #[serde(rename = "DeviceCreateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_create_date: Option<f64>,
    /// <p>The device key.</p>
    #[serde(rename = "DeviceKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_key: Option<String>,
    /// <p>The date in which the device was last authenticated.</p>
    #[serde(rename = "DeviceLastAuthenticatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_last_authenticated_date: Option<f64>,
    /// <p>The last modified date of the device.</p>
    #[serde(rename = "DeviceLastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_last_modified_date: Option<f64>,
}

/// <p>A container for information about a domain.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DomainDescriptionType {
    /// <p>The AWS account ID for the user pool owner.</p>
    #[serde(rename = "AWSAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,
    /// <p>The ARN of the CloudFront distribution.</p>
    #[serde(rename = "CloudFrontDistribution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_front_distribution: Option<String>,
    #[serde(rename = "CustomDomainConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_domain_config: Option<CustomDomainConfigType>,
    /// <p>The domain string.</p>
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// <p>The S3 bucket where the static files for this domain are stored.</p>
    #[serde(rename = "S3Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket: Option<String>,
    /// <p>The domain status.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The user pool ID.</p>
    #[serde(rename = "UserPoolId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_id: Option<String>,
    /// <p>The app version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>The email configuration type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmailConfigurationType {
    /// <p>The destination to which the receiver of the email should reply to.</p>
    #[serde(rename = "ReplyToEmailAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_email_address: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the email source.</p>
    #[serde(rename = "SourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
}

/// <p>Specifies the user context data captured at the time of an event request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct EventContextDataType {
    /// <p>The user's city.</p>
    #[serde(rename = "City")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// <p>The user's country.</p>
    #[serde(rename = "Country")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// <p>The user's device name.</p>
    #[serde(rename = "DeviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    /// <p>The user's IP address.</p>
    #[serde(rename = "IpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// <p>The user's time zone.</p>
    #[serde(rename = "Timezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

/// <p>Specifies the event feedback type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct EventFeedbackType {
    /// <p>The event feedback date.</p>
    #[serde(rename = "FeedbackDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_date: Option<f64>,
    /// <p>The event feedback value.</p>
    #[serde(rename = "FeedbackValue")]
    pub feedback_value: String,
    /// <p>The provider.</p>
    #[serde(rename = "Provider")]
    pub provider: String,
}

/// <p>The event risk type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct EventRiskType {
    /// <p>The risk decision.</p>
    #[serde(rename = "RiskDecision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_decision: Option<String>,
    /// <p>The risk level.</p>
    #[serde(rename = "RiskLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_level: Option<String>,
}

/// <p>Represents the request to forget the device.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ForgetDeviceRequest {
    /// <p>The access token for the forgotten device request.</p>
    #[serde(rename = "AccessToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    /// <p>The device key.</p>
    #[serde(rename = "DeviceKey")]
    pub device_key: String,
}

/// <p>Represents the request to reset a user's password.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ForgotPasswordRequest {
    /// <p>The Amazon Pinpoint analytics metadata for collecting metrics for <code>ForgotPassword</code> calls.</p>
    #[serde(rename = "AnalyticsMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_metadata: Option<AnalyticsMetadataType>,
    /// <p>The ID of the client associated with the user pool.</p>
    #[serde(rename = "ClientId")]
    pub client_id: String,
    /// <p>A keyed-hash message authentication code (HMAC) calculated using the secret key of a user pool client and username plus the client ID in the message.</p>
    #[serde(rename = "SecretHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_hash: Option<String>,
    /// <p>Contextual data such as the user's device fingerprint, IP address, or location used for evaluating the risk of an unexpected event by Amazon Cognito advanced security.</p>
    #[serde(rename = "UserContextData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_context_data: Option<UserContextDataType>,
    /// <p>The user name of the user for whom you want to enter a code to reset a forgotten password.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

/// <p>Respresents the response from the server regarding the request to reset a password.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ForgotPasswordResponse {
    /// <p>The code delivery details returned by the server in response to the request to reset a password.</p>
    #[serde(rename = "CodeDeliveryDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_delivery_details: Option<CodeDeliveryDetailsType>,
}

/// <p>Represents the request to get the header information for the .csv file for the user import job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetCSVHeaderRequest {
    /// <p>The user pool ID for the user pool that the users are to be imported into.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

/// <p>Represents the response from the server to the request to get the header information for the .csv file for the user import job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetCSVHeaderResponse {
    /// <p>The header information for the .csv file for the user import job.</p>
    #[serde(rename = "CSVHeader")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_header: Option<Vec<String>>,
    /// <p>The user pool ID for the user pool that the users are to be imported into.</p>
    #[serde(rename = "UserPoolId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_id: Option<String>,
}

/// <p>Represents the request to get the device.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDeviceRequest {
    /// <p>The access token.</p>
    #[serde(rename = "AccessToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    /// <p>The device key.</p>
    #[serde(rename = "DeviceKey")]
    pub device_key: String,
}

/// <p>Gets the device response.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetDeviceResponse {
    /// <p>The device.</p>
    #[serde(rename = "Device")]
    pub device: DeviceType,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetGroupRequest {
    /// <p>The name of the group.</p>
    #[serde(rename = "GroupName")]
    pub group_name: String,
    /// <p>The user pool ID for the user pool.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetGroupResponse {
    /// <p>The group object for the group.</p>
    #[serde(rename = "Group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<GroupType>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetIdentityProviderByIdentifierRequest {
    /// <p>The identity provider ID.</p>
    #[serde(rename = "IdpIdentifier")]
    pub idp_identifier: String,
    /// <p>The user pool ID.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetIdentityProviderByIdentifierResponse {
    /// <p>The identity provider object.</p>
    #[serde(rename = "IdentityProvider")]
    pub identity_provider: IdentityProviderType,
}

/// <p>Request to get a signing certificate from Cognito.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSigningCertificateRequest {
    /// <p>The user pool ID.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

/// <p>Response from Cognito for a signing certificate request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetSigningCertificateResponse {
    /// <p>The signing certificate.</p>
    #[serde(rename = "Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetUICustomizationRequest {
    /// <p>The client ID for the client app.</p>
    #[serde(rename = "ClientId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// <p>The user pool ID for the user pool.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetUICustomizationResponse {
    /// <p>The UI customization information.</p>
    #[serde(rename = "UICustomization")]
    pub ui_customization: UICustomizationType,
}

/// <p>Represents the request to get user attribute verification.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetUserAttributeVerificationCodeRequest {
    /// <p>The access token returned by the server response to get the user attribute verification code.</p>
    #[serde(rename = "AccessToken")]
    pub access_token: String,
    /// <p>The attribute name returned by the server response to get the user attribute verification code.</p>
    #[serde(rename = "AttributeName")]
    pub attribute_name: String,
}

/// <p>The verification code response returned by the server response to get the user attribute verification code.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetUserAttributeVerificationCodeResponse {
    /// <p>The code delivery details returned by the server in response to the request to get the user attribute verification code.</p>
    #[serde(rename = "CodeDeliveryDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_delivery_details: Option<CodeDeliveryDetailsType>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetUserPoolMfaConfigRequest {
    /// <p>The user pool ID.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetUserPoolMfaConfigResponse {
    /// <p>The multi-factor (MFA) configuration.</p>
    #[serde(rename = "MfaConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_configuration: Option<String>,
    /// <p>The SMS text message multi-factor (MFA) configuration.</p>
    #[serde(rename = "SmsMfaConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_mfa_configuration: Option<SmsMfaConfigType>,
    /// <p>The software token multi-factor (MFA) configuration.</p>
    #[serde(rename = "SoftwareTokenMfaConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub software_token_mfa_configuration: Option<SoftwareTokenMfaConfigType>,
}

/// <p>Represents the request to get information about the user.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetUserRequest {
    /// <p>The access token returned by the server response to get information about the user.</p>
    #[serde(rename = "AccessToken")]
    pub access_token: String,
}

/// <p>Represents the response from the server from the request to get information about the user.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetUserResponse {
    /// <p>Specifies the options for MFA (e.g., email or phone number).</p>
    #[serde(rename = "MFAOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_options: Option<Vec<MFAOptionType>>,
    /// <p>The user's preferred MFA setting.</p>
    #[serde(rename = "PreferredMfaSetting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_mfa_setting: Option<String>,
    /// <p>An array of name-value pairs representing user attributes.</p> <p>For custom attributes, you must prepend the <code>custom:</code> prefix to the attribute name.</p>
    #[serde(rename = "UserAttributes")]
    pub user_attributes: Vec<AttributeType>,
    /// <p>The list of the user's MFA settings.</p>
    #[serde(rename = "UserMFASettingList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_mfa_setting_list: Option<Vec<String>>,
    /// <p>The user name of the user you wish to retrieve from the get user request.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

/// <p>Represents the request to sign out all devices.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GlobalSignOutRequest {
    /// <p>The access token.</p>
    #[serde(rename = "AccessToken")]
    pub access_token: String,
}

/// <p>The response to the request to sign out all devices.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GlobalSignOutResponse {}

/// <p>The group type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GroupType {
    /// <p>The date the group was created.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>A string containing the description of the group.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the group.</p>
    #[serde(rename = "GroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// <p>The date the group was last modified.</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    /// <p>A nonnegative integer value that specifies the precedence of this group relative to the other groups that a user can belong to in the user pool. If a user belongs to two or more groups, it is the group with the highest precedence whose role ARN will be used in the <code>cognito:roles</code> and <code>cognito:preferred_role</code> claims in the user's tokens. Groups with higher <code>Precedence</code> values take precedence over groups with lower <code>Precedence</code> values or with null <code>Precedence</code> values.</p> <p>Two groups can have the same <code>Precedence</code> value. If this happens, neither group takes precedence over the other. If two groups with the same <code>Precedence</code> have the same role ARN, that role is used in the <code>cognito:preferred_role</code> claim in tokens for users in each group. If the two groups have different role ARNs, the <code>cognito:preferred_role</code> claim is not set in users' tokens.</p> <p>The default <code>Precedence</code> value is null.</p>
    #[serde(rename = "Precedence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precedence: Option<i64>,
    /// <p>The role ARN for the group.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The user pool ID for the user pool.</p>
    #[serde(rename = "UserPoolId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_id: Option<String>,
}

/// <p>The HTTP header.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct HttpHeader {
    /// <p>The header name</p>
    #[serde(rename = "headerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_name: Option<String>,
    /// <p>The header value.</p>
    #[serde(rename = "headerValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_value: Option<String>,
}

/// <p>A container for information about an identity provider.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct IdentityProviderType {
    /// <p>A mapping of identity provider attributes to standard and custom user pool attributes.</p>
    #[serde(rename = "AttributeMapping")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_mapping: Option<::std::collections::HashMap<String, String>>,
    /// <p>The date the identity provider was created.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>A list of identity provider identifiers.</p>
    #[serde(rename = "IdpIdentifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idp_identifiers: Option<Vec<String>>,
    /// <p>The date the identity provider was last modified.</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    /// <p>The identity provider details, such as <code>MetadataURL</code> and <code>MetadataFile</code>.</p>
    #[serde(rename = "ProviderDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_details: Option<::std::collections::HashMap<String, String>>,
    /// <p>The identity provider name.</p>
    #[serde(rename = "ProviderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    /// <p>The identity provider type.</p>
    #[serde(rename = "ProviderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_type: Option<String>,
    /// <p>The user pool ID.</p>
    #[serde(rename = "UserPoolId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_id: Option<String>,
}

/// <p>Initiates the authentication request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InitiateAuthRequest {
    /// <p>The Amazon Pinpoint analytics metadata for collecting metrics for <code>InitiateAuth</code> calls.</p>
    #[serde(rename = "AnalyticsMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_metadata: Option<AnalyticsMetadataType>,
    /// <p>The authentication flow for this call to execute. The API action will depend on this value. For example: </p> <ul> <li> <p> <code>REFRESH_TOKEN_AUTH</code> will take in a valid refresh token and return new tokens.</p> </li> <li> <p> <code>USER_SRP_AUTH</code> will take in <code>USERNAME</code> and <code>SRP_A</code> and return the SRP variables to be used for next challenge execution.</p> </li> <li> <p> <code>USER_PASSWORD_AUTH</code> will take in <code>USERNAME</code> and <code>PASSWORD</code> and return the next challenge or tokens.</p> </li> </ul> <p>Valid values include:</p> <ul> <li> <p> <code>USER_SRP_AUTH</code>: Authentication flow for the Secure Remote Password (SRP) protocol.</p> </li> <li> <p> <code>REFRESH_TOKEN_AUTH</code>/<code>REFRESH_TOKEN</code>: Authentication flow for refreshing the access token and ID token by supplying a valid refresh token.</p> </li> <li> <p> <code>CUSTOM_AUTH</code>: Custom authentication flow.</p> </li> <li> <p> <code>USER_PASSWORD_AUTH</code>: Non-SRP authentication flow; USERNAME and PASSWORD are passed directly. If a user migration Lambda trigger is set, this flow will invoke the user migration Lambda if the USERNAME is not found in the user pool. </p> </li> </ul> <p> <code>ADMIN_NO_SRP_AUTH</code> is not a valid value.</p>
    #[serde(rename = "AuthFlow")]
    pub auth_flow: String,
    /// <p><p>The authentication parameters. These are inputs corresponding to the <code>AuthFlow</code> that you are invoking. The required values depend on the value of <code>AuthFlow</code>:</p> <ul> <li> <p>For <code>USER<em>SRP</em>AUTH</code>: <code>USERNAME</code> (required), <code>SRP<em>A</code> (required), <code>SECRET</em>HASH</code> (required if the app client is configured with a client secret), <code>DEVICE<em>KEY</code> </p> </li> <li> <p>For <code>REFRESH</em>TOKEN<em>AUTH/REFRESH</em>TOKEN</code>: <code>REFRESH<em>TOKEN</code> (required), <code>SECRET</em>HASH</code> (required if the app client is configured with a client secret), <code>DEVICE<em>KEY</code> </p> </li> <li> <p>For <code>CUSTOM</em>AUTH</code>: <code>USERNAME</code> (required), <code>SECRET<em>HASH</code> (if app client is configured with client secret), <code>DEVICE</em>KEY</code> </p> </li> </ul></p>
    #[serde(rename = "AuthParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The app client ID.</p>
    #[serde(rename = "ClientId")]
    pub client_id: String,
    /// <p>This is a random key-value pair map which can contain any key and will be passed to your PreAuthentication Lambda trigger as-is. It can be used to implement additional validations around authentication.</p>
    #[serde(rename = "ClientMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_metadata: Option<::std::collections::HashMap<String, String>>,
    /// <p>Contextual data such as the user's device fingerprint, IP address, or location used for evaluating the risk of an unexpected event by Amazon Cognito advanced security.</p>
    #[serde(rename = "UserContextData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_context_data: Option<UserContextDataType>,
}

/// <p>Initiates the authentication response.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct InitiateAuthResponse {
    /// <p>The result of the authentication response. This is only returned if the caller does not need to pass another challenge. If the caller does need to pass another challenge before it gets tokens, <code>ChallengeName</code>, <code>ChallengeParameters</code>, and <code>Session</code> are returned.</p>
    #[serde(rename = "AuthenticationResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_result: Option<AuthenticationResultType>,
    /// <p><p>The name of the challenge which you are responding to with this call. This is returned to you in the <code>AdminInitiateAuth</code> response if you need to pass another challenge.</p> <p>Valid values include the following. Note that all of these challenges require <code>USERNAME</code> and <code>SECRET<em>HASH</code> (if applicable) in the parameters.</p> <ul> <li> <p> <code>SMS</em>MFA</code>: Next challenge is to supply an <code>SMS<em>MFA</em>CODE</code>, delivered via SMS.</p> </li> <li> <p> <code>PASSWORD<em>VERIFIER</code>: Next challenge is to supply <code>PASSWORD</em>CLAIM<em>SIGNATURE</code>, <code>PASSWORD</em>CLAIM<em>SECRET</em>BLOCK</code>, and <code>TIMESTAMP</code> after the client-side SRP calculations.</p> </li> <li> <p> <code>CUSTOM<em>CHALLENGE</code>: This is returned if your custom authentication flow determines that the user should pass another challenge before tokens are issued.</p> </li> <li> <p> <code>DEVICE</em>SRP<em>AUTH</code>: If device tracking was enabled on your user pool and the previous challenges were passed, this challenge is returned so that Amazon Cognito can start tracking this device.</p> </li> <li> <p> <code>DEVICE</em>PASSWORD<em>VERIFIER</code>: Similar to <code>PASSWORD</em>VERIFIER</code>, but for devices only.</p> </li> <li> <p> <code>NEW<em>PASSWORD</em>REQUIRED</code>: For users which are required to change their passwords after successful first login. This challenge should be passed with <code>NEW_PASSWORD</code> and any other required attributes.</p> </li> </ul></p>
    #[serde(rename = "ChallengeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_name: Option<String>,
    /// <p>The challenge parameters. These are returned to you in the <code>InitiateAuth</code> response if you need to pass another challenge. The responses in this parameter should be used to compute inputs to the next call (<code>RespondToAuthChallenge</code>). </p> <p>All challenges require <code>USERNAME</code> and <code>SECRET_HASH</code> (if applicable).</p>
    #[serde(rename = "ChallengeParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The session which should be passed both ways in challenge-response calls to the service. If the or API call determines that the caller needs to go through another challenge, they return a session with other challenge parameters. This session should be passed as it is to the next <code>RespondToAuthChallenge</code> API call.</p>
    #[serde(rename = "Session")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
}

/// <p>Specifies the configuration for AWS Lambda triggers.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LambdaConfigType {
    /// <p>Creates an authentication challenge.</p>
    #[serde(rename = "CreateAuthChallenge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_auth_challenge: Option<String>,
    /// <p>A custom Message AWS Lambda trigger.</p>
    #[serde(rename = "CustomMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_message: Option<String>,
    /// <p>Defines the authentication challenge.</p>
    #[serde(rename = "DefineAuthChallenge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub define_auth_challenge: Option<String>,
    /// <p>A post-authentication AWS Lambda trigger.</p>
    #[serde(rename = "PostAuthentication")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_authentication: Option<String>,
    /// <p>A post-confirmation AWS Lambda trigger.</p>
    #[serde(rename = "PostConfirmation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_confirmation: Option<String>,
    /// <p>A pre-authentication AWS Lambda trigger.</p>
    #[serde(rename = "PreAuthentication")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_authentication: Option<String>,
    /// <p>A pre-registration AWS Lambda trigger.</p>
    #[serde(rename = "PreSignUp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_sign_up: Option<String>,
    /// <p>A Lambda trigger that is invoked before token generation.</p>
    #[serde(rename = "PreTokenGeneration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_token_generation: Option<String>,
    /// <p>The user migration Lambda config type.</p>
    #[serde(rename = "UserMigration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_migration: Option<String>,
    /// <p>Verifies the authentication challenge response.</p>
    #[serde(rename = "VerifyAuthChallengeResponse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify_auth_challenge_response: Option<String>,
}

/// <p>Represents the request to list the devices.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListDevicesRequest {
    /// <p>The access tokens for the request to list devices.</p>
    #[serde(rename = "AccessToken")]
    pub access_token: String,
    /// <p>The limit of the device request.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The pagination token for the list request.</p>
    #[serde(rename = "PaginationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
}

/// <p>Represents the response to list devices.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListDevicesResponse {
    /// <p>The devices returned in the list devices response.</p>
    #[serde(rename = "Devices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<DeviceType>>,
    /// <p>The pagination token for the list device response.</p>
    #[serde(rename = "PaginationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListGroupsRequest {
    /// <p>The limit of the request to list groups.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The user pool ID for the user pool.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListGroupsResponse {
    /// <p>The group objects for the groups.</p>
    #[serde(rename = "Groups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<GroupType>>,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListIdentityProvidersRequest {
    /// <p>The maximum number of identity providers to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The user pool ID.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListIdentityProvidersResponse {
    /// <p>A pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of identity provider objects.</p>
    #[serde(rename = "Providers")]
    pub providers: Vec<ProviderDescription>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListResourceServersRequest {
    /// <p>The maximum number of resource servers to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The user pool ID for the user pool.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListResourceServersResponse {
    /// <p>A pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The resource servers.</p>
    #[serde(rename = "ResourceServers")]
    pub resource_servers: Vec<ResourceServerType>,
}

/// <p>Represents the request to list the user import jobs.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListUserImportJobsRequest {
    /// <p>The maximum number of import jobs you want the request to return.</p>
    #[serde(rename = "MaxResults")]
    pub max_results: i64,
    /// <p>An identifier that was returned from the previous call to <code>ListUserImportJobs</code>, which can be used to return the next set of import jobs in the list.</p>
    #[serde(rename = "PaginationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
    /// <p>The user pool ID for the user pool that the users are being imported into.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

/// <p>Represents the response from the server to the request to list the user import jobs.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListUserImportJobsResponse {
    /// <p>An identifier that can be used to return the next set of user import jobs in the list.</p>
    #[serde(rename = "PaginationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
    /// <p>The user import jobs.</p>
    #[serde(rename = "UserImportJobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_import_jobs: Option<Vec<UserImportJobType>>,
}

/// <p>Represents the request to list the user pool clients.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListUserPoolClientsRequest {
    /// <p>The maximum number of results you want the request to return when listing the user pool clients.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The user pool ID for the user pool where you want to list user pool clients.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

/// <p>Represents the response from the server that lists user pool clients.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListUserPoolClientsResponse {
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The user pool clients in the response that lists user pool clients.</p>
    #[serde(rename = "UserPoolClients")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_clients: Option<Vec<UserPoolClientDescription>>,
}

/// <p>Represents the request to list user pools.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListUserPoolsRequest {
    /// <p>The maximum number of results you want the request to return when listing the user pools.</p>
    #[serde(rename = "MaxResults")]
    pub max_results: i64,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the response to list user pools.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListUserPoolsResponse {
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The user pools from the response to list users.</p>
    #[serde(rename = "UserPools")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pools: Option<Vec<UserPoolDescriptionType>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListUsersInGroupRequest {
    /// <p>The name of the group.</p>
    #[serde(rename = "GroupName")]
    pub group_name: String,
    /// <p>The limit of the request to list users.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The user pool ID for the user pool.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListUsersInGroupResponse {
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The users returned in the request to list users.</p>
    #[serde(rename = "Users")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<UserType>>,
}

/// <p>Represents the request to list users.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListUsersRequest {
    /// <p>An array of strings, where each string is the name of a user attribute to be returned for each user in the search results. If the array is null, all attributes are returned.</p>
    #[serde(rename = "AttributesToGet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_to_get: Option<Vec<String>>,
    /// <p>A filter string of the form "<i>AttributeName</i> <i>Filter-Type</i> "<i>AttributeValue</i>"". Quotation marks within the filter string must be escaped using the backslash (\) character. For example, "<code>family_name</code> = \"Reddy\"".</p> <ul> <li> <p> <i>AttributeName</i>: The name of the attribute to search for. You can only search for one attribute at a time.</p> </li> <li> <p> <i>Filter-Type</i>: For an exact match, use =, for example, "<code>given_name</code> = \"Jon\"". For a prefix ("starts with") match, use ^=, for example, "<code>given_name</code> ^= \"Jon\"". </p> </li> <li> <p> <i>AttributeValue</i>: The attribute value that must be matched for each user.</p> </li> </ul> <p>If the filter string is empty, <code>ListUsers</code> returns all users in the user pool.</p> <p>You can only search for the following standard attributes:</p> <ul> <li> <p> <code>username</code> (case-sensitive)</p> </li> <li> <p> <code>email</code> </p> </li> <li> <p> <code>phone_number</code> </p> </li> <li> <p> <code>name</code> </p> </li> <li> <p> <code>given_name</code> </p> </li> <li> <p> <code>family_name</code> </p> </li> <li> <p> <code>preferred_username</code> </p> </li> <li> <p> <code>cognito:user_status</code> (called <b>Status</b> in the Console) (case-insensitive)</p> </li> <li> <p> <code>status (called <b>Enabled</b> in the Console) (case-sensitive)</code> </p> </li> <li> <p> <code>sub</code> </p> </li> </ul> <p>Custom attributes are not searchable.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/cognito/latest/developerguide/how-to-manage-user-accounts.html#cognito-user-pools-searching-for-users-using-listusers-api">Searching for Users Using the ListUsers API</a> and <a href="http://docs.aws.amazon.com/cognito/latest/developerguide/how-to-manage-user-accounts.html#cognito-user-pools-searching-for-users-listusers-api-examples">Examples of Using the ListUsers API</a> in the <i>Amazon Cognito Developer Guide</i>.</p>
    #[serde(rename = "Filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    /// <p>Maximum number of users to be returned.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "PaginationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
    /// <p>The user pool ID for the user pool on which the search should be performed.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

/// <p>The response from the request to list users.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListUsersResponse {
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "PaginationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
    /// <p>The users returned in the request to list users.</p>
    #[serde(rename = "Users")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<UserType>>,
}

/// <p>Specifies the different settings for multi-factor authentication (MFA).</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MFAOptionType {
    /// <p>The attribute name of the MFA option type.</p>
    #[serde(rename = "AttributeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    /// <p>The delivery medium (email message or SMS message) to send the MFA code.</p>
    #[serde(rename = "DeliveryMedium")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_medium: Option<String>,
}

/// <p>The message template structure.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageTemplateType {
    /// <p>The message template for email messages.</p>
    #[serde(rename = "EmailMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_message: Option<String>,
    /// <p>The subject line for email messages.</p>
    #[serde(rename = "EmailSubject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_subject: Option<String>,
    /// <p>The message template for SMS messages.</p>
    #[serde(rename = "SMSMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_message: Option<String>,
}

/// <p>The new device metadata type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct NewDeviceMetadataType {
    /// <p>The device group key.</p>
    #[serde(rename = "DeviceGroupKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_group_key: Option<String>,
    /// <p>The device key.</p>
    #[serde(rename = "DeviceKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_key: Option<String>,
}

/// <p>The notify configuration type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotifyConfigurationType {
    /// <p>Email template used when a detected risk event is blocked.</p>
    #[serde(rename = "BlockEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_email: Option<NotifyEmailType>,
    /// <p>The email address that is sending the email. It must be either individually verified with Amazon SES, or from a domain that has been verified with Amazon SES.</p>
    #[serde(rename = "From")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// <p>The MFA email template used when MFA is challenged as part of a detected risk.</p>
    #[serde(rename = "MfaEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_email: Option<NotifyEmailType>,
    /// <p>The email template used when a detected risk event is allowed.</p>
    #[serde(rename = "NoActionEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_action_email: Option<NotifyEmailType>,
    /// <p>The destination to which the receiver of an email should reply to.</p>
    #[serde(rename = "ReplyTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the identity that is associated with the sending authorization policy. It permits Amazon Cognito to send for the email address specified in the <code>From</code> parameter.</p>
    #[serde(rename = "SourceArn")]
    pub source_arn: String,
}

/// <p>The notify email type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotifyEmailType {
    /// <p>The HTML body.</p>
    #[serde(rename = "HtmlBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_body: Option<String>,
    /// <p>The subject.</p>
    #[serde(rename = "Subject")]
    pub subject: String,
    /// <p>The text body.</p>
    #[serde(rename = "TextBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_body: Option<String>,
}

/// <p>The minimum and maximum value of an attribute that is of the number data type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NumberAttributeConstraintsType {
    /// <p>The maximum value of an attribute that is of the number data type.</p>
    #[serde(rename = "MaxValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<String>,
    /// <p>The minimum value of an attribute that is of the number data type.</p>
    #[serde(rename = "MinValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<String>,
}

/// <p>The password policy type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PasswordPolicyType {
    /// <p>The minimum length of the password policy that you have set. Cannot be less than 6.</p>
    #[serde(rename = "MinimumLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_length: Option<i64>,
    /// <p>In the password policy that you have set, refers to whether you have required users to use at least one lowercase letter in their password.</p>
    #[serde(rename = "RequireLowercase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_lowercase: Option<bool>,
    /// <p>In the password policy that you have set, refers to whether you have required users to use at least one number in their password.</p>
    #[serde(rename = "RequireNumbers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_numbers: Option<bool>,
    /// <p>In the password policy that you have set, refers to whether you have required users to use at least one symbol in their password.</p>
    #[serde(rename = "RequireSymbols")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_symbols: Option<bool>,
    /// <p>In the password policy that you have set, refers to whether you have required users to use at least one uppercase letter in their password.</p>
    #[serde(rename = "RequireUppercase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_uppercase: Option<bool>,
}

/// <p>A container for identity provider details.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ProviderDescription {
    /// <p>The date the provider was added to the user pool.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>The date the provider was last modified.</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    /// <p>The identity provider name.</p>
    #[serde(rename = "ProviderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    /// <p>The identity provider type.</p>
    #[serde(rename = "ProviderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_type: Option<String>,
}

/// <p>A container for information about an identity provider for a user pool.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ProviderUserIdentifierType {
    /// <p>The name of the provider attribute to link to, for example, <code>NameID</code>.</p>
    #[serde(rename = "ProviderAttributeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_attribute_name: Option<String>,
    /// <p>The value of the provider attribute to link to, for example, <code>xxxxx_account</code>.</p>
    #[serde(rename = "ProviderAttributeValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_attribute_value: Option<String>,
    /// <p>The name of the provider, for example, Facebook, Google, or Login with Amazon.</p>
    #[serde(rename = "ProviderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
}

/// <p>Represents the request to resend the confirmation code.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ResendConfirmationCodeRequest {
    /// <p>The Amazon Pinpoint analytics metadata for collecting metrics for <code>ResendConfirmationCode</code> calls.</p>
    #[serde(rename = "AnalyticsMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_metadata: Option<AnalyticsMetadataType>,
    /// <p>The ID of the client associated with the user pool.</p>
    #[serde(rename = "ClientId")]
    pub client_id: String,
    /// <p>A keyed-hash message authentication code (HMAC) calculated using the secret key of a user pool client and username plus the client ID in the message.</p>
    #[serde(rename = "SecretHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_hash: Option<String>,
    /// <p>Contextual data such as the user's device fingerprint, IP address, or location used for evaluating the risk of an unexpected event by Amazon Cognito advanced security.</p>
    #[serde(rename = "UserContextData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_context_data: Option<UserContextDataType>,
    /// <p>The user name of the user to whom you wish to resend a confirmation code.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

/// <p>The response from the server when the Amazon Cognito Your User Pools service makes the request to resend a confirmation code.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ResendConfirmationCodeResponse {
    /// <p>The code delivery details returned by the server in response to the request to resend the confirmation code.</p>
    #[serde(rename = "CodeDeliveryDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_delivery_details: Option<CodeDeliveryDetailsType>,
}

/// <p>A resource server scope.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceServerScopeType {
    /// <p>A description of the scope.</p>
    #[serde(rename = "ScopeDescription")]
    pub scope_description: String,
    /// <p>The name of the scope.</p>
    #[serde(rename = "ScopeName")]
    pub scope_name: String,
}

/// <p>A container for information about a resource server for a user pool.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ResourceServerType {
    /// <p>The identifier for the resource server.</p>
    #[serde(rename = "Identifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// <p>The name of the resource server.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A list of scopes that are defined for the resource server.</p>
    #[serde(rename = "Scopes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<ResourceServerScopeType>>,
    /// <p>The user pool ID for the user pool that hosts the resource server.</p>
    #[serde(rename = "UserPoolId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_id: Option<String>,
}

/// <p>The request to respond to an authentication challenge.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RespondToAuthChallengeRequest {
    /// <p>The Amazon Pinpoint analytics metadata for collecting metrics for <code>RespondToAuthChallenge</code> calls.</p>
    #[serde(rename = "AnalyticsMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_metadata: Option<AnalyticsMetadataType>,
    /// <p>The challenge name. For more information, see .</p> <p> <code>ADMIN_NO_SRP_AUTH</code> is not a valid value.</p>
    #[serde(rename = "ChallengeName")]
    pub challenge_name: String,
    /// <p><p>The challenge responses. These are inputs corresponding to the value of <code>ChallengeName</code>, for example:</p> <ul> <li> <p> <code>SMS<em>MFA</code>: <code>SMS</em>MFA<em>CODE</code>, <code>USERNAME</code>, <code>SECRET</em>HASH</code> (if app client is configured with client secret).</p> </li> <li> <p> <code>PASSWORD<em>VERIFIER</code>: <code>PASSWORD</em>CLAIM<em>SIGNATURE</code>, <code>PASSWORD</em>CLAIM<em>SECRET</em>BLOCK</code>, <code>TIMESTAMP</code>, <code>USERNAME</code>, <code>SECRET<em>HASH</code> (if app client is configured with client secret).</p> </li> <li> <p> <code>NEW</em>PASSWORD<em>REQUIRED</code>: <code>NEW</em>PASSWORD</code>, any other required attributes, <code>USERNAME</code>, <code>SECRET_HASH</code> (if app client is configured with client secret). </p> </li> </ul></p>
    #[serde(rename = "ChallengeResponses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_responses: Option<::std::collections::HashMap<String, String>>,
    /// <p>The app client ID.</p>
    #[serde(rename = "ClientId")]
    pub client_id: String,
    /// <p>The session which should be passed both ways in challenge-response calls to the service. If <code>InitiateAuth</code> or <code>RespondToAuthChallenge</code> API call determines that the caller needs to go through another challenge, they return a session with other challenge parameters. This session should be passed as it is to the next <code>RespondToAuthChallenge</code> API call.</p>
    #[serde(rename = "Session")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
    /// <p>Contextual data such as the user's device fingerprint, IP address, or location used for evaluating the risk of an unexpected event by Amazon Cognito advanced security.</p>
    #[serde(rename = "UserContextData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_context_data: Option<UserContextDataType>,
}

/// <p>The response to respond to the authentication challenge.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RespondToAuthChallengeResponse {
    /// <p>The result returned by the server in response to the request to respond to the authentication challenge.</p>
    #[serde(rename = "AuthenticationResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_result: Option<AuthenticationResultType>,
    /// <p>The challenge name. For more information, see .</p>
    #[serde(rename = "ChallengeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_name: Option<String>,
    /// <p>The challenge parameters. For more information, see .</p>
    #[serde(rename = "ChallengeParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The session which should be passed both ways in challenge-response calls to the service. If the or API call determines that the caller needs to go through another challenge, they return a session with other challenge parameters. This session should be passed as it is to the next <code>RespondToAuthChallenge</code> API call.</p>
    #[serde(rename = "Session")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
}

/// <p>The risk configuration type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RiskConfigurationType {
    /// <p>The account takeover risk configuration object including the <code>NotifyConfiguration</code> object and <code>Actions</code> to take in the case of an account takeover.</p>
    #[serde(rename = "AccountTakeoverRiskConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_takeover_risk_configuration: Option<AccountTakeoverRiskConfigurationType>,
    /// <p>The app client ID.</p>
    #[serde(rename = "ClientId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// <p>The compromised credentials risk configuration object including the <code>EventFilter</code> and the <code>EventAction</code> </p>
    #[serde(rename = "CompromisedCredentialsRiskConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compromised_credentials_risk_configuration:
        Option<CompromisedCredentialsRiskConfigurationType>,
    /// <p>The last modified date.</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    /// <p>The configuration to override the risk decision.</p>
    #[serde(rename = "RiskExceptionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_exception_configuration: Option<RiskExceptionConfigurationType>,
    /// <p>The user pool ID.</p>
    #[serde(rename = "UserPoolId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_id: Option<String>,
}

/// <p>The type of the configuration to override the risk decision.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RiskExceptionConfigurationType {
    /// <p>Overrides the risk decision to always block the pre-authentication requests. The IP range is in CIDR notation: a compact representation of an IP address and its associated routing prefix.</p>
    #[serde(rename = "BlockedIPRangeList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_ip_range_list: Option<Vec<String>>,
    /// <p>Risk detection is not performed on the IP addresses in the range list. The IP range is in CIDR notation.</p>
    #[serde(rename = "SkippedIPRangeList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipped_ip_range_list: Option<Vec<String>>,
}

/// <p>The SMS multi-factor authentication (MFA) settings type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SMSMfaSettingsType {
    /// <p>Specifies whether SMS text message MFA is enabled.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The preferred MFA method.</p>
    #[serde(rename = "PreferredMfa")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_mfa: Option<bool>,
}

/// <p>Contains information about the schema attribute.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchemaAttributeType {
    /// <p>The attribute data type.</p>
    #[serde(rename = "AttributeDataType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_data_type: Option<String>,
    /// <p>Specifies whether the attribute type is developer only.</p>
    #[serde(rename = "DeveloperOnlyAttribute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub developer_only_attribute: Option<bool>,
    /// <p>Specifies whether the value of the attribute can be changed.</p> <p>For any user pool attribute that's mapped to an identity provider attribute, you must set this parameter to <code>true</code>. Amazon Cognito updates mapped attributes when users sign in to your application through an identity provider. If an attribute is immutable, Amazon Cognito throws an error when it attempts to update the attribute. For more information, see <a href="http://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-specifying-attribute-mapping.html">Specifying Identity Provider Attribute Mappings for Your User Pool</a>.</p>
    #[serde(rename = "Mutable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutable: Option<bool>,
    /// <p>A schema attribute of the name type.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Specifies the constraints for an attribute of the number type.</p>
    #[serde(rename = "NumberAttributeConstraints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_attribute_constraints: Option<NumberAttributeConstraintsType>,
    /// <p>Specifies whether a user pool attribute is required. If the attribute is required and the user does not provide a value, registration or sign-in will fail.</p>
    #[serde(rename = "Required")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// <p>Specifies the constraints for an attribute of the string type.</p>
    #[serde(rename = "StringAttributeConstraints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_attribute_constraints: Option<StringAttributeConstraintsType>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SetRiskConfigurationRequest {
    /// <p>The account takeover risk configuration.</p>
    #[serde(rename = "AccountTakeoverRiskConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_takeover_risk_configuration: Option<AccountTakeoverRiskConfigurationType>,
    /// <p>The app client ID. If <code>ClientId</code> is null, then the risk configuration is mapped to <code>userPoolId</code>. When the client ID is null, the same risk configuration is applied to all the clients in the userPool.</p> <p>Otherwise, <code>ClientId</code> is mapped to the client. When the client ID is not null, the user pool configuration is overridden and the risk configuration for the client is used instead.</p>
    #[serde(rename = "ClientId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// <p>The compromised credentials risk configuration.</p>
    #[serde(rename = "CompromisedCredentialsRiskConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compromised_credentials_risk_configuration:
        Option<CompromisedCredentialsRiskConfigurationType>,
    /// <p>The configuration to override the risk decision.</p>
    #[serde(rename = "RiskExceptionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_exception_configuration: Option<RiskExceptionConfigurationType>,
    /// <p>The user pool ID. </p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SetRiskConfigurationResponse {
    /// <p>The risk configuration.</p>
    #[serde(rename = "RiskConfiguration")]
    pub risk_configuration: RiskConfigurationType,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SetUICustomizationRequest {
    /// <p>The CSS values in the UI customization.</p>
    #[serde(rename = "CSS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub css: Option<String>,
    /// <p>The client ID for the client app.</p>
    #[serde(rename = "ClientId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// <p>The uploaded logo image for the UI customization.</p>
    #[serde(rename = "ImageFile")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_file: Option<bytes::Bytes>,
    /// <p>The user pool ID for the user pool.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SetUICustomizationResponse {
    /// <p>The UI customization information.</p>
    #[serde(rename = "UICustomization")]
    pub ui_customization: UICustomizationType,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SetUserMFAPreferenceRequest {
    /// <p>The access token.</p>
    #[serde(rename = "AccessToken")]
    pub access_token: String,
    /// <p>The SMS text message multi-factor authentication (MFA) settings.</p>
    #[serde(rename = "SMSMfaSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_mfa_settings: Option<SMSMfaSettingsType>,
    /// <p>The time-based one-time password software token MFA settings.</p>
    #[serde(rename = "SoftwareTokenMfaSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub software_token_mfa_settings: Option<SoftwareTokenMfaSettingsType>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SetUserMFAPreferenceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SetUserPoolMfaConfigRequest {
    /// <p>The MFA configuration.</p>
    #[serde(rename = "MfaConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_configuration: Option<String>,
    /// <p>The SMS text message MFA configuration.</p>
    #[serde(rename = "SmsMfaConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_mfa_configuration: Option<SmsMfaConfigType>,
    /// <p>The software token MFA configuration.</p>
    #[serde(rename = "SoftwareTokenMfaConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub software_token_mfa_configuration: Option<SoftwareTokenMfaConfigType>,
    /// <p>The user pool ID.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SetUserPoolMfaConfigResponse {
    /// <p>The MFA configuration.</p>
    #[serde(rename = "MfaConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_configuration: Option<String>,
    /// <p>The SMS text message MFA configuration.</p>
    #[serde(rename = "SmsMfaConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_mfa_configuration: Option<SmsMfaConfigType>,
    /// <p>The software token MFA configuration.</p>
    #[serde(rename = "SoftwareTokenMfaConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub software_token_mfa_configuration: Option<SoftwareTokenMfaConfigType>,
}

/// <p>Represents the request to set user settings.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SetUserSettingsRequest {
    /// <p>The access token for the set user settings request.</p>
    #[serde(rename = "AccessToken")]
    pub access_token: String,
    /// <p>Specifies the options for MFA (e.g., email or phone number).</p>
    #[serde(rename = "MFAOptions")]
    pub mfa_options: Vec<MFAOptionType>,
}

/// <p>The response from the server for a set user settings request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SetUserSettingsResponse {}

/// <p>Represents the request to register a user.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SignUpRequest {
    /// <p>The Amazon Pinpoint analytics metadata for collecting metrics for <code>SignUp</code> calls.</p>
    #[serde(rename = "AnalyticsMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_metadata: Option<AnalyticsMetadataType>,
    /// <p>The ID of the client associated with the user pool.</p>
    #[serde(rename = "ClientId")]
    pub client_id: String,
    /// <p>The password of the user you wish to register.</p>
    #[serde(rename = "Password")]
    pub password: String,
    /// <p>A keyed-hash message authentication code (HMAC) calculated using the secret key of a user pool client and username plus the client ID in the message.</p>
    #[serde(rename = "SecretHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_hash: Option<String>,
    /// <p>An array of name-value pairs representing user attributes.</p> <p>For custom attributes, you must prepend the <code>custom:</code> prefix to the attribute name.</p>
    #[serde(rename = "UserAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_attributes: Option<Vec<AttributeType>>,
    /// <p>Contextual data such as the user's device fingerprint, IP address, or location used for evaluating the risk of an unexpected event by Amazon Cognito advanced security.</p>
    #[serde(rename = "UserContextData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_context_data: Option<UserContextDataType>,
    /// <p>The user name of the user you wish to register.</p>
    #[serde(rename = "Username")]
    pub username: String,
    /// <p>The validation data in the request to register a user.</p>
    #[serde(rename = "ValidationData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_data: Option<Vec<AttributeType>>,
}

/// <p>The response from the server for a registration request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SignUpResponse {
    /// <p>The code delivery details returned by the server response to the user registration request.</p>
    #[serde(rename = "CodeDeliveryDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_delivery_details: Option<CodeDeliveryDetailsType>,
    /// <p>A response from the server indicating that a user registration has been confirmed.</p>
    #[serde(rename = "UserConfirmed")]
    pub user_confirmed: bool,
    /// <p>The UUID of the authenticated user. This is not the same as <code>username</code>.</p>
    #[serde(rename = "UserSub")]
    pub user_sub: String,
}

/// <p>The SMS configuration type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SmsConfigurationType {
    /// <p>The external ID.</p>
    #[serde(rename = "ExternalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon Simple Notification Service (SNS) caller.</p>
    #[serde(rename = "SnsCallerArn")]
    pub sns_caller_arn: String,
}

/// <p>The SMS text message multi-factor authentication (MFA) configuration type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SmsMfaConfigType {
    /// <p>The SMS authentication message.</p>
    #[serde(rename = "SmsAuthenticationMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_authentication_message: Option<String>,
    /// <p>The SMS configuration.</p>
    #[serde(rename = "SmsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_configuration: Option<SmsConfigurationType>,
}

/// <p>The type used for enabling software token MFA at the user pool level.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SoftwareTokenMfaConfigType {
    /// <p>Specifies whether software token MFA is enabled.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// <p>The type used for enabling software token MFA at the user level.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SoftwareTokenMfaSettingsType {
    /// <p>Specifies whether software token MFA is enabled.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The preferred MFA method.</p>
    #[serde(rename = "PreferredMfa")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_mfa: Option<bool>,
}

/// <p>Represents the request to start the user import job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartUserImportJobRequest {
    /// <p>The job ID for the user import job.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
    /// <p>The user pool ID for the user pool that the users are being imported into.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

/// <p>Represents the response from the server to the request to start the user import job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartUserImportJobResponse {
    /// <p>The job object that represents the user import job.</p>
    #[serde(rename = "UserImportJob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_import_job: Option<UserImportJobType>,
}

/// <p>Represents the request to stop the user import job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopUserImportJobRequest {
    /// <p>The job ID for the user import job.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
    /// <p>The user pool ID for the user pool that the users are being imported into.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

/// <p>Represents the response from the server to the request to stop the user import job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StopUserImportJobResponse {
    /// <p>The job object that represents the user import job.</p>
    #[serde(rename = "UserImportJob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_import_job: Option<UserImportJobType>,
}

/// <p>The constraints associated with a string attribute.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StringAttributeConstraintsType {
    /// <p>The maximum length.</p>
    #[serde(rename = "MaxLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<String>,
    /// <p>The minimum length.</p>
    #[serde(rename = "MinLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<String>,
}

/// <p>A container for the UI customization information for a user pool's built-in app UI.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UICustomizationType {
    /// <p>The CSS values in the UI customization.</p>
    #[serde(rename = "CSS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub css: Option<String>,
    /// <p>The CSS version number.</p>
    #[serde(rename = "CSSVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub css_version: Option<String>,
    /// <p>The client ID for the client app.</p>
    #[serde(rename = "ClientId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// <p>The creation date for the UI customization.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>The logo image for the UI customization.</p>
    #[serde(rename = "ImageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// <p>The last-modified date for the UI customization.</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    /// <p>The user pool ID for the user pool.</p>
    #[serde(rename = "UserPoolId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateAuthEventFeedbackRequest {
    /// <p>The event ID.</p>
    #[serde(rename = "EventId")]
    pub event_id: String,
    /// <p>The feedback token.</p>
    #[serde(rename = "FeedbackToken")]
    pub feedback_token: String,
    /// <p>The authentication event feedback value.</p>
    #[serde(rename = "FeedbackValue")]
    pub feedback_value: String,
    /// <p>The user pool ID.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
    /// <p>The user pool username.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateAuthEventFeedbackResponse {}

/// <p>Represents the request to update the device status.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateDeviceStatusRequest {
    /// <p>The access token.</p>
    #[serde(rename = "AccessToken")]
    pub access_token: String,
    /// <p>The device key.</p>
    #[serde(rename = "DeviceKey")]
    pub device_key: String,
    /// <p>The status of whether a device is remembered.</p>
    #[serde(rename = "DeviceRememberedStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_remembered_status: Option<String>,
}

/// <p>The response to the request to update the device status.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateDeviceStatusResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateGroupRequest {
    /// <p>A string containing the new description of the group.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the group.</p>
    #[serde(rename = "GroupName")]
    pub group_name: String,
    /// <p>The new precedence value for the group. For more information about this parameter, see .</p>
    #[serde(rename = "Precedence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precedence: Option<i64>,
    /// <p>The new role ARN for the group. This is used for setting the <code>cognito:roles</code> and <code>cognito:preferred_role</code> claims in the token.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The user pool ID for the user pool.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateGroupResponse {
    /// <p>The group object for the group.</p>
    #[serde(rename = "Group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<GroupType>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateIdentityProviderRequest {
    /// <p>The identity provider attribute mapping to be changed.</p>
    #[serde(rename = "AttributeMapping")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_mapping: Option<::std::collections::HashMap<String, String>>,
    /// <p>A list of identity provider identifiers.</p>
    #[serde(rename = "IdpIdentifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idp_identifiers: Option<Vec<String>>,
    /// <p>The identity provider details to be updated, such as <code>MetadataURL</code> and <code>MetadataFile</code>.</p>
    #[serde(rename = "ProviderDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_details: Option<::std::collections::HashMap<String, String>>,
    /// <p>The identity provider name.</p>
    #[serde(rename = "ProviderName")]
    pub provider_name: String,
    /// <p>The user pool ID.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateIdentityProviderResponse {
    /// <p>The identity provider object.</p>
    #[serde(rename = "IdentityProvider")]
    pub identity_provider: IdentityProviderType,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateResourceServerRequest {
    /// <p>The identifier for the resource server.</p>
    #[serde(rename = "Identifier")]
    pub identifier: String,
    /// <p>The name of the resource server.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The scope values to be set for the resource server.</p>
    #[serde(rename = "Scopes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<ResourceServerScopeType>>,
    /// <p>The user pool ID for the user pool.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateResourceServerResponse {
    /// <p>The resource server.</p>
    #[serde(rename = "ResourceServer")]
    pub resource_server: ResourceServerType,
}

/// <p>Represents the request to update user attributes.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateUserAttributesRequest {
    /// <p>The access token for the request to update user attributes.</p>
    #[serde(rename = "AccessToken")]
    pub access_token: String,
    /// <p>An array of name-value pairs representing user attributes.</p> <p>For custom attributes, you must prepend the <code>custom:</code> prefix to the attribute name.</p>
    #[serde(rename = "UserAttributes")]
    pub user_attributes: Vec<AttributeType>,
}

/// <p>Represents the response from the server for the request to update user attributes.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateUserAttributesResponse {
    /// <p>The code delivery details list from the server for the request to update user attributes.</p>
    #[serde(rename = "CodeDeliveryDetailsList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_delivery_details_list: Option<Vec<CodeDeliveryDetailsType>>,
}

/// <p>Represents the request to update the user pool client.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateUserPoolClientRequest {
    /// <p>Set to <code>code</code> to initiate a code grant flow, which provides an authorization code as the response. This code can be exchanged for access tokens with the token endpoint.</p> <p>Set to <code>token</code> to specify that the client should get the access token (and, optionally, ID token, based on scopes) directly.</p>
    #[serde(rename = "AllowedOAuthFlows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_o_auth_flows: Option<Vec<String>>,
    /// <p>Set to TRUE if the client is allowed to follow the OAuth protocol when interacting with Cognito user pools.</p>
    #[serde(rename = "AllowedOAuthFlowsUserPoolClient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_o_auth_flows_user_pool_client: Option<bool>,
    /// <p>A list of allowed <code>OAuth</code> scopes. Currently supported values are <code>"phone"</code>, <code>"email"</code>, <code>"openid"</code>, and <code>"Cognito"</code>.</p>
    #[serde(rename = "AllowedOAuthScopes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_o_auth_scopes: Option<Vec<String>>,
    /// <p>The Amazon Pinpoint analytics configuration for collecting metrics for this user pool.</p>
    #[serde(rename = "AnalyticsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_configuration: Option<AnalyticsConfigurationType>,
    /// <p>A list of allowed redirect (callback) URLs for the identity providers.</p> <p>A redirect URI must:</p> <ul> <li> <p>Be an absolute URI.</p> </li> <li> <p>Be registered with the authorization server.</p> </li> <li> <p>Not include a fragment component.</p> </li> </ul> <p>See <a href="https://tools.ietf.org/html/rfc6749#section-3.1.2">OAuth 2.0 - Redirection Endpoint</a>.</p> <p>Amazon Cognito requires HTTPS over HTTP except for http://localhost for testing purposes only.</p> <p>App callback URLs such as myapp://example are also supported.</p>
    #[serde(rename = "CallbackURLs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_ur_ls: Option<Vec<String>>,
    /// <p>The ID of the client associated with the user pool.</p>
    #[serde(rename = "ClientId")]
    pub client_id: String,
    /// <p>The client name from the update user pool client request.</p>
    #[serde(rename = "ClientName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_name: Option<String>,
    /// <p>The default redirect URI. Must be in the <code>CallbackURLs</code> list.</p> <p>A redirect URI must:</p> <ul> <li> <p>Be an absolute URI.</p> </li> <li> <p>Be registered with the authorization server.</p> </li> <li> <p>Not include a fragment component.</p> </li> </ul> <p>See <a href="https://tools.ietf.org/html/rfc6749#section-3.1.2">OAuth 2.0 - Redirection Endpoint</a>.</p> <p>Amazon Cognito requires HTTPS over HTTP except for http://localhost for testing purposes only.</p> <p>App callback URLs such as myapp://example are also supported.</p>
    #[serde(rename = "DefaultRedirectURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_redirect_uri: Option<String>,
    /// <p>Explicit authentication flows.</p>
    #[serde(rename = "ExplicitAuthFlows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_auth_flows: Option<Vec<String>>,
    /// <p>A list of allowed logout URLs for the identity providers.</p>
    #[serde(rename = "LogoutURLs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logout_ur_ls: Option<Vec<String>>,
    /// <p>The read-only attributes of the user pool.</p>
    #[serde(rename = "ReadAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_attributes: Option<Vec<String>>,
    /// <p>The time limit, in days, after which the refresh token is no longer valid and cannot be used.</p>
    #[serde(rename = "RefreshTokenValidity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token_validity: Option<i64>,
    /// <p>A list of provider names for the identity providers that are supported on this client.</p>
    #[serde(rename = "SupportedIdentityProviders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_identity_providers: Option<Vec<String>>,
    /// <p>The user pool ID for the user pool where you want to update the user pool client.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
    /// <p>The writeable attributes of the user pool.</p>
    #[serde(rename = "WriteAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_attributes: Option<Vec<String>>,
}

/// <p>Represents the response from the server to the request to update the user pool client.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateUserPoolClientResponse {
    /// <p>The user pool client value from the response from the server when an update user pool client request is made.</p>
    #[serde(rename = "UserPoolClient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_client: Option<UserPoolClientType>,
}

/// <p>The UpdateUserPoolDomain request input.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateUserPoolDomainRequest {
    /// <p>The configuration for a custom domain that hosts the sign-up and sign-in pages for your application. Use this object to specify an SSL certificate that is managed by ACM.</p>
    #[serde(rename = "CustomDomainConfig")]
    pub custom_domain_config: CustomDomainConfigType,
    /// <p>The domain name for the custom domain that hosts the sign-up and sign-in pages for your application. For example: <code>auth.example.com</code>. </p> <p>This string can include only lowercase letters, numbers, and hyphens. Do not use a hyphen for the first or last character. Use periods to separate subdomain names.</p>
    #[serde(rename = "Domain")]
    pub domain: String,
    /// <p>The ID of the user pool that is associated with the custom domain that you are updating the certificate for.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

/// <p>The UpdateUserPoolDomain response output.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateUserPoolDomainResponse {
    /// <p>The Amazon CloudFront endpoint that Amazon Cognito set up when you added the custom domain to your user pool.</p>
    #[serde(rename = "CloudFrontDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_front_domain: Option<String>,
}

/// <p>Represents the request to update the user pool.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateUserPoolRequest {
    /// <p>The configuration for <code>AdminCreateUser</code> requests.</p>
    #[serde(rename = "AdminCreateUserConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_create_user_config: Option<AdminCreateUserConfigType>,
    /// <p>The attributes that are automatically verified when the Amazon Cognito service makes a request to update user pools.</p>
    #[serde(rename = "AutoVerifiedAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_verified_attributes: Option<Vec<String>>,
    /// <p>Device configuration.</p>
    #[serde(rename = "DeviceConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_configuration: Option<DeviceConfigurationType>,
    /// <p>Email configuration.</p>
    #[serde(rename = "EmailConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_configuration: Option<EmailConfigurationType>,
    /// <p>The contents of the email verification message.</p>
    #[serde(rename = "EmailVerificationMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verification_message: Option<String>,
    /// <p>The subject of the email verification message.</p>
    #[serde(rename = "EmailVerificationSubject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verification_subject: Option<String>,
    /// <p>The AWS Lambda configuration information from the request to update the user pool.</p>
    #[serde(rename = "LambdaConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_config: Option<LambdaConfigType>,
    /// <p><p>Can be one of the following values:</p> <ul> <li> <p> <code>OFF</code> - MFA tokens are not required and cannot be specified during user registration.</p> </li> <li> <p> <code>ON</code> - MFA tokens are required for all user registrations. You can only specify required when you are initially creating a user pool.</p> </li> <li> <p> <code>OPTIONAL</code> - Users have the option when registering to create an MFA token.</p> </li> </ul></p>
    #[serde(rename = "MfaConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_configuration: Option<String>,
    /// <p>A container with the policies you wish to update in a user pool.</p>
    #[serde(rename = "Policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<UserPoolPolicyType>,
    /// <p>The contents of the SMS authentication message.</p>
    #[serde(rename = "SmsAuthenticationMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_authentication_message: Option<String>,
    /// <p>SMS configuration.</p>
    #[serde(rename = "SmsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_configuration: Option<SmsConfigurationType>,
    /// <p>A container with information about the SMS verification message.</p>
    #[serde(rename = "SmsVerificationMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_verification_message: Option<String>,
    /// <p>Used to enable advanced security risk detection. Set the key <code>AdvancedSecurityMode</code> to the value "AUDIT".</p>
    #[serde(rename = "UserPoolAddOns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_add_ons: Option<UserPoolAddOnsType>,
    /// <p>The user pool ID for the user pool you want to update.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
    /// <p>The cost allocation tags for the user pool. For more information, see <a href="http://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-cost-allocation-tagging.html">Adding Cost Allocation Tags to Your User Pool</a> </p>
    #[serde(rename = "UserPoolTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The template for verification messages.</p>
    #[serde(rename = "VerificationMessageTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_message_template: Option<VerificationMessageTemplateType>,
}

/// <p>Represents the response from the server when you make a request to update the user pool.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateUserPoolResponse {}

/// <p>Contextual data such as the user's device fingerprint, IP address, or location used for evaluating the risk of an unexpected event by Amazon Cognito advanced security.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UserContextDataType {
    /// <p>Contextual data such as the user's device fingerprint, IP address, or location used for evaluating the risk of an unexpected event by Amazon Cognito advanced security.</p>
    #[serde(rename = "EncodedData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoded_data: Option<String>,
}

/// <p>The user import job type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UserImportJobType {
    /// <p>The role ARN for the Amazon CloudWatch Logging role for the user import job. For more information, see "Creating the CloudWatch Logs IAM Role" in the Amazon Cognito Developer Guide.</p>
    #[serde(rename = "CloudWatchLogsRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_role_arn: Option<String>,
    /// <p>The date when the user import job was completed.</p>
    #[serde(rename = "CompletionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<f64>,
    /// <p>The message returned when the user import job is completed.</p>
    #[serde(rename = "CompletionMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_message: Option<String>,
    /// <p>The date the user import job was created.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>The number of users that could not be imported.</p>
    #[serde(rename = "FailedUsers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_users: Option<i64>,
    /// <p>The number of users that were successfully imported.</p>
    #[serde(rename = "ImportedUsers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported_users: Option<i64>,
    /// <p>The job ID for the user import job.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p>The job name for the user import job.</p>
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>The pre-signed URL to be used to upload the <code>.csv</code> file.</p>
    #[serde(rename = "PreSignedUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_signed_url: Option<String>,
    /// <p>The number of users that were skipped.</p>
    #[serde(rename = "SkippedUsers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipped_users: Option<i64>,
    /// <p>The date when the user import job was started.</p>
    #[serde(rename = "StartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<f64>,
    /// <p><p>The status of the user import job. One of the following:</p> <ul> <li> <p> <code>Created</code> - The job was created but not started.</p> </li> <li> <p> <code>Pending</code> - A transition state. You have started the job, but it has not begun importing users yet.</p> </li> <li> <p> <code>InProgress</code> - The job has started, and users are being imported.</p> </li> <li> <p> <code>Stopping</code> - You have stopped the job, but the job has not stopped importing users yet.</p> </li> <li> <p> <code>Stopped</code> - You have stopped the job, and the job has stopped importing users.</p> </li> <li> <p> <code>Succeeded</code> - The job has completed successfully.</p> </li> <li> <p> <code>Failed</code> - The job has stopped due to an error.</p> </li> <li> <p> <code>Expired</code> - You created a job, but did not start the job within 24-48 hours. All data associated with the job was deleted, and the job cannot be started.</p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The user pool ID for the user pool that the users are being imported into.</p>
    #[serde(rename = "UserPoolId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_id: Option<String>,
}

/// <p>The user pool add-ons type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserPoolAddOnsType {
    /// <p>The advanced security mode.</p>
    #[serde(rename = "AdvancedSecurityMode")]
    pub advanced_security_mode: String,
}

/// <p>The description of the user pool client.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UserPoolClientDescription {
    /// <p>The ID of the client associated with the user pool.</p>
    #[serde(rename = "ClientId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// <p>The client name from the user pool client description.</p>
    #[serde(rename = "ClientName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_name: Option<String>,
    /// <p>The user pool ID for the user pool where you want to describe the user pool client.</p>
    #[serde(rename = "UserPoolId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_id: Option<String>,
}

/// <p>Contains information about a user pool client.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UserPoolClientType {
    /// <p>Set to <code>code</code> to initiate a code grant flow, which provides an authorization code as the response. This code can be exchanged for access tokens with the token endpoint.</p> <p>Set to <code>token</code> to specify that the client should get the access token (and, optionally, ID token, based on scopes) directly.</p>
    #[serde(rename = "AllowedOAuthFlows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_o_auth_flows: Option<Vec<String>>,
    /// <p>Set to TRUE if the client is allowed to follow the OAuth protocol when interacting with Cognito user pools.</p>
    #[serde(rename = "AllowedOAuthFlowsUserPoolClient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_o_auth_flows_user_pool_client: Option<bool>,
    /// <p>A list of allowed <code>OAuth</code> scopes. Currently supported values are <code>"phone"</code>, <code>"email"</code>, <code>"openid"</code>, and <code>"Cognito"</code>.</p>
    #[serde(rename = "AllowedOAuthScopes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_o_auth_scopes: Option<Vec<String>>,
    /// <p>The Amazon Pinpoint analytics configuration for the user pool client.</p>
    #[serde(rename = "AnalyticsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_configuration: Option<AnalyticsConfigurationType>,
    /// <p>A list of allowed redirect (callback) URLs for the identity providers.</p> <p>A redirect URI must:</p> <ul> <li> <p>Be an absolute URI.</p> </li> <li> <p>Be registered with the authorization server.</p> </li> <li> <p>Not include a fragment component.</p> </li> </ul> <p>See <a href="https://tools.ietf.org/html/rfc6749#section-3.1.2">OAuth 2.0 - Redirection Endpoint</a>.</p> <p>Amazon Cognito requires HTTPS over HTTP except for http://localhost for testing purposes only.</p> <p>App callback URLs such as myapp://example are also supported.</p>
    #[serde(rename = "CallbackURLs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_ur_ls: Option<Vec<String>>,
    /// <p>The ID of the client associated with the user pool.</p>
    #[serde(rename = "ClientId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// <p>The client name from the user pool request of the client type.</p>
    #[serde(rename = "ClientName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_name: Option<String>,
    /// <p>The client secret from the user pool request of the client type.</p>
    #[serde(rename = "ClientSecret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    /// <p>The date the user pool client was created.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>The default redirect URI. Must be in the <code>CallbackURLs</code> list.</p> <p>A redirect URI must:</p> <ul> <li> <p>Be an absolute URI.</p> </li> <li> <p>Be registered with the authorization server.</p> </li> <li> <p>Not include a fragment component.</p> </li> </ul> <p>See <a href="https://tools.ietf.org/html/rfc6749#section-3.1.2">OAuth 2.0 - Redirection Endpoint</a>.</p> <p>Amazon Cognito requires HTTPS over HTTP except for http://localhost for testing purposes only.</p> <p>App callback URLs such as myapp://example are also supported.</p>
    #[serde(rename = "DefaultRedirectURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_redirect_uri: Option<String>,
    /// <p>The explicit authentication flows.</p>
    #[serde(rename = "ExplicitAuthFlows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_auth_flows: Option<Vec<String>>,
    /// <p>The date the user pool client was last modified.</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    /// <p>A list of allowed logout URLs for the identity providers.</p>
    #[serde(rename = "LogoutURLs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logout_ur_ls: Option<Vec<String>>,
    /// <p>The Read-only attributes.</p>
    #[serde(rename = "ReadAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_attributes: Option<Vec<String>>,
    /// <p>The time limit, in days, after which the refresh token is no longer valid and cannot be used.</p>
    #[serde(rename = "RefreshTokenValidity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token_validity: Option<i64>,
    /// <p>A list of provider names for the identity providers that are supported on this client.</p>
    #[serde(rename = "SupportedIdentityProviders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_identity_providers: Option<Vec<String>>,
    /// <p>The user pool ID for the user pool client.</p>
    #[serde(rename = "UserPoolId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_id: Option<String>,
    /// <p>The writeable attributes.</p>
    #[serde(rename = "WriteAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_attributes: Option<Vec<String>>,
}

/// <p>A user pool description.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UserPoolDescriptionType {
    /// <p>The date the user pool description was created.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>The ID in a user pool description.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The AWS Lambda configuration information in a user pool description.</p>
    #[serde(rename = "LambdaConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_config: Option<LambdaConfigType>,
    /// <p>The date the user pool description was last modified.</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    /// <p>The name in a user pool description.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The user pool status in a user pool description.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>The policy associated with a user pool.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserPoolPolicyType {
    /// <p>The password policy.</p>
    #[serde(rename = "PasswordPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_policy: Option<PasswordPolicyType>,
}

/// <p>A container for information about the user pool.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UserPoolType {
    /// <p>The configuration for <code>AdminCreateUser</code> requests.</p>
    #[serde(rename = "AdminCreateUserConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_create_user_config: Option<AdminCreateUserConfigType>,
    /// <p>Specifies the attributes that are aliased in a user pool.</p>
    #[serde(rename = "AliasAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_attributes: Option<Vec<String>>,
    /// <p>The Amazon Resource Name (ARN) for the user pool.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Specifies the attributes that are auto-verified in a user pool.</p>
    #[serde(rename = "AutoVerifiedAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_verified_attributes: Option<Vec<String>>,
    /// <p>The date the user pool was created.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "CustomDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_domain: Option<String>,
    /// <p>The device configuration.</p>
    #[serde(rename = "DeviceConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_configuration: Option<DeviceConfigurationType>,
    /// <p>Holds the domain prefix if the user pool has a domain associated with it.</p>
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// <p>The email configuration.</p>
    #[serde(rename = "EmailConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_configuration: Option<EmailConfigurationType>,
    /// <p>The reason why the email configuration cannot send the messages to your users.</p>
    #[serde(rename = "EmailConfigurationFailure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_configuration_failure: Option<String>,
    /// <p>The contents of the email verification message.</p>
    #[serde(rename = "EmailVerificationMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verification_message: Option<String>,
    /// <p>The subject of the email verification message.</p>
    #[serde(rename = "EmailVerificationSubject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verification_subject: Option<String>,
    /// <p>A number estimating the size of the user pool.</p>
    #[serde(rename = "EstimatedNumberOfUsers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_number_of_users: Option<i64>,
    /// <p>The ID of the user pool.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The AWS Lambda triggers associated with the user pool.</p>
    #[serde(rename = "LambdaConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_config: Option<LambdaConfigType>,
    /// <p>The date the user pool was last modified.</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    /// <p><p>Can be one of the following values:</p> <ul> <li> <p> <code>OFF</code> - MFA tokens are not required and cannot be specified during user registration.</p> </li> <li> <p> <code>ON</code> - MFA tokens are required for all user registrations. You can only specify required when you are initially creating a user pool.</p> </li> <li> <p> <code>OPTIONAL</code> - Users have the option when registering to create an MFA token.</p> </li> </ul></p>
    #[serde(rename = "MfaConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_configuration: Option<String>,
    /// <p>The name of the user pool.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The policies associated with the user pool.</p>
    #[serde(rename = "Policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<UserPoolPolicyType>,
    /// <p>A container with the schema attributes of a user pool.</p>
    #[serde(rename = "SchemaAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_attributes: Option<Vec<SchemaAttributeType>>,
    /// <p>The contents of the SMS authentication message.</p>
    #[serde(rename = "SmsAuthenticationMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_authentication_message: Option<String>,
    /// <p>The SMS configuration.</p>
    #[serde(rename = "SmsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_configuration: Option<SmsConfigurationType>,
    /// <p>The reason why the SMS configuration cannot send the messages to your users.</p>
    #[serde(rename = "SmsConfigurationFailure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_configuration_failure: Option<String>,
    /// <p>The contents of the SMS verification message.</p>
    #[serde(rename = "SmsVerificationMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_verification_message: Option<String>,
    /// <p>The status of a user pool.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The user pool add-ons.</p>
    #[serde(rename = "UserPoolAddOns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_add_ons: Option<UserPoolAddOnsType>,
    /// <p>The cost allocation tags for the user pool. For more information, see <a href="http://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-cost-allocation-tagging.html">Adding Cost Allocation Tags to Your User Pool</a> </p>
    #[serde(rename = "UserPoolTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>Specifies whether email addresses or phone numbers can be specified as usernames when a user signs up.</p>
    #[serde(rename = "UsernameAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username_attributes: Option<Vec<String>>,
    /// <p>The template for verification messages.</p>
    #[serde(rename = "VerificationMessageTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_message_template: Option<VerificationMessageTemplateType>,
}

/// <p>The user type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UserType {
    /// <p>A container with information about the user type attributes.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<AttributeType>>,
    /// <p>Specifies whether the user is enabled.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The MFA options for the user.</p>
    #[serde(rename = "MFAOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_options: Option<Vec<MFAOptionType>>,
    /// <p>The creation date of the user.</p>
    #[serde(rename = "UserCreateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_create_date: Option<f64>,
    /// <p>The last modified date of the user.</p>
    #[serde(rename = "UserLastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_last_modified_date: Option<f64>,
    /// <p><p>The user status. Can be one of the following:</p> <ul> <li> <p>UNCONFIRMED - User has been created but not confirmed.</p> </li> <li> <p>CONFIRMED - User has been confirmed.</p> </li> <li> <p>ARCHIVED - User is no longer active.</p> </li> <li> <p>COMPROMISED - User is disabled due to a potential security threat.</p> </li> <li> <p>UNKNOWN - User status is not known.</p> </li> </ul></p>
    #[serde(rename = "UserStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_status: Option<String>,
    /// <p>The user name of the user you wish to describe.</p>
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// <p>The template for verification messages.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VerificationMessageTemplateType {
    /// <p>The default email option.</p>
    #[serde(rename = "DefaultEmailOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_email_option: Option<String>,
    /// <p>The email message template.</p>
    #[serde(rename = "EmailMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_message: Option<String>,
    /// <p>The email message template for sending a confirmation link to the user.</p>
    #[serde(rename = "EmailMessageByLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_message_by_link: Option<String>,
    /// <p>The subject line for the email message template.</p>
    #[serde(rename = "EmailSubject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_subject: Option<String>,
    /// <p>The subject line for the email message template for sending a confirmation link to the user.</p>
    #[serde(rename = "EmailSubjectByLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_subject_by_link: Option<String>,
    /// <p>The SMS message template.</p>
    #[serde(rename = "SmsMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_message: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct VerifySoftwareTokenRequest {
    /// <p>The access token.</p>
    #[serde(rename = "AccessToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    /// <p>The friendly device name.</p>
    #[serde(rename = "FriendlyDeviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friendly_device_name: Option<String>,
    /// <p>The session which should be passed both ways in challenge-response calls to the service.</p>
    #[serde(rename = "Session")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
    /// <p>The one time password computed using the secret code returned by </p>
    #[serde(rename = "UserCode")]
    pub user_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct VerifySoftwareTokenResponse {
    /// <p>The session which should be passed both ways in challenge-response calls to the service.</p>
    #[serde(rename = "Session")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
    /// <p>The status of the verify software token.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Represents the request to verify user attributes.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct VerifyUserAttributeRequest {
    /// <p>Represents the access token of the request to verify user attributes.</p>
    #[serde(rename = "AccessToken")]
    pub access_token: String,
    /// <p>The attribute name in the request to verify user attributes.</p>
    #[serde(rename = "AttributeName")]
    pub attribute_name: String,
    /// <p>The verification code in the request to verify user attributes.</p>
    #[serde(rename = "Code")]
    pub code: String,
}

/// <p>A container representing the response from the server from the request to verify user attributes.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct VerifyUserAttributeResponse {}

/// Errors returned by AddCustomAttributes
#[derive(Debug, PartialEq)]
pub enum AddCustomAttributesError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when you are trying to modify a user pool while a user import job is in progress for that pool.</p>
    UserImportInProgress(String),
}

impl AddCustomAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddCustomAttributesError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(AddCustomAttributesError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AddCustomAttributesError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AddCustomAttributesError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AddCustomAttributesError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AddCustomAttributesError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "UserImportInProgressException" => {
                    return RusotoError::Service(AddCustomAttributesError::UserImportInProgress(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by AdminAddUserToGroup
#[derive(Debug, PartialEq)]
pub enum AdminAddUserToGroupError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
}

impl AdminAddUserToGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AdminAddUserToGroupError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(AdminAddUserToGroupError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AdminAddUserToGroupError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminAddUserToGroupError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AdminAddUserToGroupError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AdminAddUserToGroupError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminAddUserToGroupError::UserNotFound(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by AdminConfirmSignUp
#[derive(Debug, PartialEq)]
pub enum AdminConfirmSignUpError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid AWS Lambda response.</p>
    InvalidLambdaResponse(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user exceeds the limit for a requested AWS resource.</p>
    LimitExceeded(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many failed attempts for a given action (e.g., sign in).</p>
    TooManyFailedAttempts(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an unexpected exception with the AWS Lambda service.</p>
    UnexpectedLambda(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters a user validation exception with the AWS Lambda service.</p>
    UserLambdaValidation(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
}

impl AdminConfirmSignUpError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AdminConfirmSignUpError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(AdminConfirmSignUpError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidLambdaResponseException" => {
                    return RusotoError::Service(AdminConfirmSignUpError::InvalidLambdaResponse(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AdminConfirmSignUpError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(AdminConfirmSignUpError::LimitExceeded(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminConfirmSignUpError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AdminConfirmSignUpError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyFailedAttemptsException" => {
                    return RusotoError::Service(AdminConfirmSignUpError::TooManyFailedAttempts(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AdminConfirmSignUpError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "UnexpectedLambdaException" => {
                    return RusotoError::Service(AdminConfirmSignUpError::UnexpectedLambda(
                        String::from(error_message),
                    ))
                }
                "UserLambdaValidationException" => {
                    return RusotoError::Service(AdminConfirmSignUpError::UserLambdaValidation(
                        String::from(error_message),
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminConfirmSignUpError::UserNotFound(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by AdminCreateUser
#[derive(Debug, PartialEq)]
pub enum AdminCreateUserError {
    /// <p>This exception is thrown when a verification code fails to deliver successfully.</p>
    CodeDeliveryFailure(String),
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid AWS Lambda response.</p>
    InvalidLambdaResponse(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid password.</p>
    InvalidPassword(String),
    /// <p>This exception is returned when the role provided for SMS configuration does not have permission to publish using Amazon SNS.</p>
    InvalidSmsRoleAccessPolicy(String),
    /// <p>This exception is thrown when the trust relationship is invalid for the role provided for SMS configuration. This can happen if you do not trust <b>cognito-idp.amazonaws.com</b> or the external ID provided in the role does not match what is provided in the SMS configuration for the user pool.</p>
    InvalidSmsRoleTrustRelationship(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when a precondition is not met.</p>
    PreconditionNotMet(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an unexpected exception with the AWS Lambda service.</p>
    UnexpectedLambda(String),
    /// <p>The request failed because the user is in an unsupported state.</p>
    UnsupportedUserState(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters a user validation exception with the AWS Lambda service.</p>
    UserLambdaValidation(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
    /// <p>This exception is thrown when Amazon Cognito encounters a user name that already exists in the user pool.</p>
    UsernameExists(String),
}

impl AdminCreateUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AdminCreateUserError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "CodeDeliveryFailureException" => {
                    return RusotoError::Service(AdminCreateUserError::CodeDeliveryFailure(
                        String::from(error_message),
                    ))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(AdminCreateUserError::InternalError(String::from(
                        error_message,
                    )))
                }
                "InvalidLambdaResponseException" => {
                    return RusotoError::Service(AdminCreateUserError::InvalidLambdaResponse(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AdminCreateUserError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "InvalidPasswordException" => {
                    return RusotoError::Service(AdminCreateUserError::InvalidPassword(
                        String::from(error_message),
                    ))
                }
                "InvalidSmsRoleAccessPolicyException" => {
                    return RusotoError::Service(AdminCreateUserError::InvalidSmsRoleAccessPolicy(
                        String::from(error_message),
                    ))
                }
                "InvalidSmsRoleTrustRelationshipException" => {
                    return RusotoError::Service(
                        AdminCreateUserError::InvalidSmsRoleTrustRelationship(String::from(
                            error_message,
                        )),
                    )
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminCreateUserError::NotAuthorized(String::from(
                        error_message,
                    )))
                }
                "PreconditionNotMetException" => {
                    return RusotoError::Service(AdminCreateUserError::PreconditionNotMet(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AdminCreateUserError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AdminCreateUserError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "UnexpectedLambdaException" => {
                    return RusotoError::Service(AdminCreateUserError::UnexpectedLambda(
                        String::from(error_message),
                    ))
                }
                "UnsupportedUserStateException" => {
                    return RusotoError::Service(AdminCreateUserError::UnsupportedUserState(
                        String::from(error_message),
                    ))
                }
                "UserLambdaValidationException" => {
                    return RusotoError::Service(AdminCreateUserError::UserLambdaValidation(
                        String::from(error_message),
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminCreateUserError::UserNotFound(String::from(
                        error_message,
                    )))
                }
                "UsernameExistsException" => {
                    return RusotoError::Service(AdminCreateUserError::UsernameExists(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by AdminDeleteUser
#[derive(Debug, PartialEq)]
pub enum AdminDeleteUserError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
}

impl AdminDeleteUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AdminDeleteUserError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(AdminDeleteUserError::InternalError(String::from(
                        error_message,
                    )))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AdminDeleteUserError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminDeleteUserError::NotAuthorized(String::from(
                        error_message,
                    )))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AdminDeleteUserError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AdminDeleteUserError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminDeleteUserError::UserNotFound(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by AdminDeleteUserAttributes
#[derive(Debug, PartialEq)]
pub enum AdminDeleteUserAttributesError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
}

impl AdminDeleteUserAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AdminDeleteUserAttributesError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(AdminDeleteUserAttributesError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AdminDeleteUserAttributesError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminDeleteUserAttributesError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AdminDeleteUserAttributesError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AdminDeleteUserAttributesError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminDeleteUserAttributesError::UserNotFound(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by AdminDisableProviderForUser
#[derive(Debug, PartialEq)]
pub enum AdminDisableProviderForUserError {
    /// <p>This exception is thrown when a user tries to confirm the account with an email or phone number that has already been supplied as an alias from a different account. This exception tells user that an account with this email or phone already exists.</p>
    AliasExists(String),
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
}

impl AdminDisableProviderForUserError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AdminDisableProviderForUserError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AliasExistsException" => {
                    return RusotoError::Service(AdminDisableProviderForUserError::AliasExists(
                        String::from(error_message),
                    ))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(AdminDisableProviderForUserError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        AdminDisableProviderForUserError::InvalidParameter(String::from(
                            error_message,
                        )),
                    )
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminDisableProviderForUserError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        AdminDisableProviderForUserError::ResourceNotFound(String::from(
                            error_message,
                        )),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AdminDisableProviderForUserError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminDisableProviderForUserError::UserNotFound(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AdminDisableProviderForUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AdminDisableProviderForUserError {
    fn description(&self) -> &str {
        match *self {
            AdminDisableProviderForUserError::AliasExists(ref cause) => cause,
            AdminDisableProviderForUserError::InternalError(ref cause) => cause,
            AdminDisableProviderForUserError::InvalidParameter(ref cause) => cause,
            AdminDisableProviderForUserError::NotAuthorized(ref cause) => cause,
            AdminDisableProviderForUserError::ResourceNotFound(ref cause) => cause,
            AdminDisableProviderForUserError::TooManyRequests(ref cause) => cause,
            AdminDisableProviderForUserError::UserNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by AdminDisableUser
#[derive(Debug, PartialEq)]
pub enum AdminDisableUserError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
}

impl AdminDisableUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AdminDisableUserError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(AdminDisableUserError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AdminDisableUserError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminDisableUserError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AdminDisableUserError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AdminDisableUserError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminDisableUserError::UserNotFound(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by AdminEnableUser
#[derive(Debug, PartialEq)]
pub enum AdminEnableUserError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
}

impl AdminEnableUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AdminEnableUserError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(AdminEnableUserError::InternalError(String::from(
                        error_message,
                    )))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AdminEnableUserError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminEnableUserError::NotAuthorized(String::from(
                        error_message,
                    )))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AdminEnableUserError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AdminEnableUserError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminEnableUserError::UserNotFound(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by AdminForgetDevice
#[derive(Debug, PartialEq)]
pub enum AdminForgetDeviceError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when the user pool configuration is invalid.</p>
    InvalidUserPoolConfiguration(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
}

impl AdminForgetDeviceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AdminForgetDeviceError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(AdminForgetDeviceError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AdminForgetDeviceError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "InvalidUserPoolConfigurationException" => {
                    return RusotoError::Service(
                        AdminForgetDeviceError::InvalidUserPoolConfiguration(String::from(
                            error_message,
                        )),
                    )
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminForgetDeviceError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AdminForgetDeviceError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AdminForgetDeviceError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminForgetDeviceError::UserNotFound(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by AdminGetDevice
#[derive(Debug, PartialEq)]
pub enum AdminGetDeviceError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when the user pool configuration is invalid.</p>
    InvalidUserPoolConfiguration(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
}

impl AdminGetDeviceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AdminGetDeviceError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(AdminGetDeviceError::InternalError(String::from(
                        error_message,
                    )))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AdminGetDeviceError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "InvalidUserPoolConfigurationException" => {
                    return RusotoError::Service(AdminGetDeviceError::InvalidUserPoolConfiguration(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminGetDeviceError::NotAuthorized(String::from(
                        error_message,
                    )))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AdminGetDeviceError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AdminGetDeviceError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by AdminGetUser
#[derive(Debug, PartialEq)]
pub enum AdminGetUserError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
}

impl AdminGetUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AdminGetUserError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(AdminGetUserError::InternalError(String::from(
                        error_message,
                    )))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AdminGetUserError::InvalidParameter(String::from(
                        error_message,
                    )))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminGetUserError::NotAuthorized(String::from(
                        error_message,
                    )))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AdminGetUserError::ResourceNotFound(String::from(
                        error_message,
                    )))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AdminGetUserError::TooManyRequests(String::from(
                        error_message,
                    )))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminGetUserError::UserNotFound(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by AdminInitiateAuth
#[derive(Debug, PartialEq)]
pub enum AdminInitiateAuthError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid AWS Lambda response.</p>
    InvalidLambdaResponse(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is returned when the role provided for SMS configuration does not have permission to publish using Amazon SNS.</p>
    InvalidSmsRoleAccessPolicy(String),
    /// <p>This exception is thrown when the trust relationship is invalid for the role provided for SMS configuration. This can happen if you do not trust <b>cognito-idp.amazonaws.com</b> or the external ID provided in the role does not match what is provided in the SMS configuration for the user pool.</p>
    InvalidSmsRoleTrustRelationship(String),
    /// <p>This exception is thrown when the user pool configuration is invalid.</p>
    InvalidUserPoolConfiguration(String),
    /// <p>This exception is thrown when Amazon Cognito cannot find a multi-factor authentication (MFA) method.</p>
    MFAMethodNotFound(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when a password reset is required.</p>
    PasswordResetRequired(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an unexpected exception with the AWS Lambda service.</p>
    UnexpectedLambda(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters a user validation exception with the AWS Lambda service.</p>
    UserLambdaValidation(String),
    /// <p>This exception is thrown when a user is not confirmed successfully.</p>
    UserNotConfirmed(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
}

impl AdminInitiateAuthError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AdminInitiateAuthError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(AdminInitiateAuthError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidLambdaResponseException" => {
                    return RusotoError::Service(AdminInitiateAuthError::InvalidLambdaResponse(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AdminInitiateAuthError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "InvalidSmsRoleAccessPolicyException" => {
                    return RusotoError::Service(
                        AdminInitiateAuthError::InvalidSmsRoleAccessPolicy(String::from(
                            error_message,
                        )),
                    )
                }
                "InvalidSmsRoleTrustRelationshipException" => {
                    return RusotoError::Service(
                        AdminInitiateAuthError::InvalidSmsRoleTrustRelationship(String::from(
                            error_message,
                        )),
                    )
                }
                "InvalidUserPoolConfigurationException" => {
                    return RusotoError::Service(
                        AdminInitiateAuthError::InvalidUserPoolConfiguration(String::from(
                            error_message,
                        )),
                    )
                }
                "MFAMethodNotFoundException" => {
                    return RusotoError::Service(AdminInitiateAuthError::MFAMethodNotFound(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminInitiateAuthError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "PasswordResetRequiredException" => {
                    return RusotoError::Service(AdminInitiateAuthError::PasswordResetRequired(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AdminInitiateAuthError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AdminInitiateAuthError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "UnexpectedLambdaException" => {
                    return RusotoError::Service(AdminInitiateAuthError::UnexpectedLambda(
                        String::from(error_message),
                    ))
                }
                "UserLambdaValidationException" => {
                    return RusotoError::Service(AdminInitiateAuthError::UserLambdaValidation(
                        String::from(error_message),
                    ))
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(AdminInitiateAuthError::UserNotConfirmed(
                        String::from(error_message),
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminInitiateAuthError::UserNotFound(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by AdminLinkProviderForUser
#[derive(Debug, PartialEq)]
pub enum AdminLinkProviderForUserError {
    /// <p>This exception is thrown when a user tries to confirm the account with an email or phone number that has already been supplied as an alias from a different account. This exception tells user that an account with this email or phone already exists.</p>
    AliasExists(String),
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
}

impl AdminLinkProviderForUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AdminLinkProviderForUserError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AliasExistsException" => {
                    return RusotoError::Service(AdminLinkProviderForUserError::AliasExists(
                        String::from(error_message),
                    ))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(AdminLinkProviderForUserError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AdminLinkProviderForUserError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminLinkProviderForUserError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AdminLinkProviderForUserError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AdminLinkProviderForUserError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminLinkProviderForUserError::UserNotFound(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AdminLinkProviderForUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AdminLinkProviderForUserError {
    fn description(&self) -> &str {
        match *self {
            AdminLinkProviderForUserError::AliasExists(ref cause) => cause,
            AdminLinkProviderForUserError::InternalError(ref cause) => cause,
            AdminLinkProviderForUserError::InvalidParameter(ref cause) => cause,
            AdminLinkProviderForUserError::NotAuthorized(ref cause) => cause,
            AdminLinkProviderForUserError::ResourceNotFound(ref cause) => cause,
            AdminLinkProviderForUserError::TooManyRequests(ref cause) => cause,
            AdminLinkProviderForUserError::UserNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by AdminListDevices
#[derive(Debug, PartialEq)]
pub enum AdminListDevicesError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when the user pool configuration is invalid.</p>
    InvalidUserPoolConfiguration(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
}

impl AdminListDevicesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AdminListDevicesError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(AdminListDevicesError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AdminListDevicesError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "InvalidUserPoolConfigurationException" => {
                    return RusotoError::Service(
                        AdminListDevicesError::InvalidUserPoolConfiguration(String::from(
                            error_message,
                        )),
                    )
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminListDevicesError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AdminListDevicesError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AdminListDevicesError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by AdminListGroupsForUser
#[derive(Debug, PartialEq)]
pub enum AdminListGroupsForUserError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
}

impl AdminListGroupsForUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AdminListGroupsForUserError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(AdminListGroupsForUserError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AdminListGroupsForUserError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminListGroupsForUserError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AdminListGroupsForUserError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AdminListGroupsForUserError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminListGroupsForUserError::UserNotFound(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by AdminListUserAuthEvents
#[derive(Debug, PartialEq)]
pub enum AdminListUserAuthEventsError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
    /// <p>This exception is thrown when user pool add-ons are not enabled.</p>
    UserPoolAddOnNotEnabled(String),
}

impl AdminListUserAuthEventsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AdminListUserAuthEventsError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(AdminListUserAuthEventsError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AdminListUserAuthEventsError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminListUserAuthEventsError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AdminListUserAuthEventsError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AdminListUserAuthEventsError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminListUserAuthEventsError::UserNotFound(
                        String::from(error_message),
                    ))
                }
                "UserPoolAddOnNotEnabledException" => {
                    return RusotoError::Service(
                        AdminListUserAuthEventsError::UserPoolAddOnNotEnabled(String::from(
                            error_message,
                        )),
                    )
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AdminListUserAuthEventsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AdminListUserAuthEventsError {
    fn description(&self) -> &str {
        match *self {
            AdminListUserAuthEventsError::InternalError(ref cause) => cause,
            AdminListUserAuthEventsError::InvalidParameter(ref cause) => cause,
            AdminListUserAuthEventsError::NotAuthorized(ref cause) => cause,
            AdminListUserAuthEventsError::ResourceNotFound(ref cause) => cause,
            AdminListUserAuthEventsError::TooManyRequests(ref cause) => cause,
            AdminListUserAuthEventsError::UserNotFound(ref cause) => cause,
            AdminListUserAuthEventsError::UserPoolAddOnNotEnabled(ref cause) => cause,
        }
    }
}
/// Errors returned by AdminRemoveUserFromGroup
#[derive(Debug, PartialEq)]
pub enum AdminRemoveUserFromGroupError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
}

impl AdminRemoveUserFromGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AdminRemoveUserFromGroupError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(AdminRemoveUserFromGroupError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AdminRemoveUserFromGroupError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminRemoveUserFromGroupError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AdminRemoveUserFromGroupError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AdminRemoveUserFromGroupError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminRemoveUserFromGroupError::UserNotFound(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by AdminResetUserPassword
#[derive(Debug, PartialEq)]
pub enum AdminResetUserPasswordError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when Amazon Cognito is not allowed to use your email identity. HTTP status code: 400.</p>
    InvalidEmailRoleAccessPolicy(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid AWS Lambda response.</p>
    InvalidLambdaResponse(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is returned when the role provided for SMS configuration does not have permission to publish using Amazon SNS.</p>
    InvalidSmsRoleAccessPolicy(String),
    /// <p>This exception is thrown when the trust relationship is invalid for the role provided for SMS configuration. This can happen if you do not trust <b>cognito-idp.amazonaws.com</b> or the external ID provided in the role does not match what is provided in the SMS configuration for the user pool.</p>
    InvalidSmsRoleTrustRelationship(String),
    /// <p>This exception is thrown when a user exceeds the limit for a requested AWS resource.</p>
    LimitExceeded(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an unexpected exception with the AWS Lambda service.</p>
    UnexpectedLambda(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters a user validation exception with the AWS Lambda service.</p>
    UserLambdaValidation(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
}

impl AdminResetUserPasswordError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AdminResetUserPasswordError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(AdminResetUserPasswordError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidEmailRoleAccessPolicyException" => {
                    return RusotoError::Service(
                        AdminResetUserPasswordError::InvalidEmailRoleAccessPolicy(String::from(
                            error_message,
                        )),
                    )
                }
                "InvalidLambdaResponseException" => {
                    return RusotoError::Service(
                        AdminResetUserPasswordError::InvalidLambdaResponse(String::from(
                            error_message,
                        )),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AdminResetUserPasswordError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "InvalidSmsRoleAccessPolicyException" => {
                    return RusotoError::Service(
                        AdminResetUserPasswordError::InvalidSmsRoleAccessPolicy(String::from(
                            error_message,
                        )),
                    )
                }
                "InvalidSmsRoleTrustRelationshipException" => {
                    return RusotoError::Service(
                        AdminResetUserPasswordError::InvalidSmsRoleTrustRelationship(String::from(
                            error_message,
                        )),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(AdminResetUserPasswordError::LimitExceeded(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminResetUserPasswordError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AdminResetUserPasswordError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AdminResetUserPasswordError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "UnexpectedLambdaException" => {
                    return RusotoError::Service(AdminResetUserPasswordError::UnexpectedLambda(
                        String::from(error_message),
                    ))
                }
                "UserLambdaValidationException" => {
                    return RusotoError::Service(AdminResetUserPasswordError::UserLambdaValidation(
                        String::from(error_message),
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminResetUserPasswordError::UserNotFound(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            AdminResetUserPasswordError::InvalidEmailRoleAccessPolicy(ref cause) => cause,
            AdminResetUserPasswordError::InvalidLambdaResponse(ref cause) => cause,
            AdminResetUserPasswordError::InvalidParameter(ref cause) => cause,
            AdminResetUserPasswordError::InvalidSmsRoleAccessPolicy(ref cause) => cause,
            AdminResetUserPasswordError::InvalidSmsRoleTrustRelationship(ref cause) => cause,
            AdminResetUserPasswordError::LimitExceeded(ref cause) => cause,
            AdminResetUserPasswordError::NotAuthorized(ref cause) => cause,
            AdminResetUserPasswordError::ResourceNotFound(ref cause) => cause,
            AdminResetUserPasswordError::TooManyRequests(ref cause) => cause,
            AdminResetUserPasswordError::UnexpectedLambda(ref cause) => cause,
            AdminResetUserPasswordError::UserLambdaValidation(ref cause) => cause,
            AdminResetUserPasswordError::UserNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by AdminRespondToAuthChallenge
#[derive(Debug, PartialEq)]
pub enum AdminRespondToAuthChallengeError {
    /// <p>This exception is thrown when a user tries to confirm the account with an email or phone number that has already been supplied as an alias from a different account. This exception tells user that an account with this email or phone already exists.</p>
    AliasExists(String),
    /// <p>This exception is thrown if the provided code does not match what the server was expecting.</p>
    CodeMismatch(String),
    /// <p>This exception is thrown if a code has expired.</p>
    ExpiredCode(String),
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid AWS Lambda response.</p>
    InvalidLambdaResponse(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid password.</p>
    InvalidPassword(String),
    /// <p>This exception is returned when the role provided for SMS configuration does not have permission to publish using Amazon SNS.</p>
    InvalidSmsRoleAccessPolicy(String),
    /// <p>This exception is thrown when the trust relationship is invalid for the role provided for SMS configuration. This can happen if you do not trust <b>cognito-idp.amazonaws.com</b> or the external ID provided in the role does not match what is provided in the SMS configuration for the user pool.</p>
    InvalidSmsRoleTrustRelationship(String),
    /// <p>This exception is thrown when the user pool configuration is invalid.</p>
    InvalidUserPoolConfiguration(String),
    /// <p>This exception is thrown when Amazon Cognito cannot find a multi-factor authentication (MFA) method.</p>
    MFAMethodNotFound(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when a password reset is required.</p>
    PasswordResetRequired(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the software token TOTP multi-factor authentication (MFA) is not enabled for the user pool.</p>
    SoftwareTokenMFANotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an unexpected exception with the AWS Lambda service.</p>
    UnexpectedLambda(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters a user validation exception with the AWS Lambda service.</p>
    UserLambdaValidation(String),
    /// <p>This exception is thrown when a user is not confirmed successfully.</p>
    UserNotConfirmed(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
}

impl AdminRespondToAuthChallengeError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AdminRespondToAuthChallengeError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AliasExistsException" => {
                    return RusotoError::Service(AdminRespondToAuthChallengeError::AliasExists(
                        String::from(error_message),
                    ))
                }
                "CodeMismatchException" => {
                    return RusotoError::Service(AdminRespondToAuthChallengeError::CodeMismatch(
                        String::from(error_message),
                    ))
                }
                "ExpiredCodeException" => {
                    return RusotoError::Service(AdminRespondToAuthChallengeError::ExpiredCode(
                        String::from(error_message),
                    ))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(AdminRespondToAuthChallengeError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidLambdaResponseException" => {
                    return RusotoError::Service(
                        AdminRespondToAuthChallengeError::InvalidLambdaResponse(String::from(
                            error_message,
                        )),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        AdminRespondToAuthChallengeError::InvalidParameter(String::from(
                            error_message,
                        )),
                    )
                }
                "InvalidPasswordException" => {
                    return RusotoError::Service(AdminRespondToAuthChallengeError::InvalidPassword(
                        String::from(error_message),
                    ))
                }
                "InvalidSmsRoleAccessPolicyException" => {
                    return RusotoError::Service(
                        AdminRespondToAuthChallengeError::InvalidSmsRoleAccessPolicy(String::from(
                            error_message,
                        )),
                    )
                }
                "InvalidSmsRoleTrustRelationshipException" => {
                    return RusotoError::Service(
                        AdminRespondToAuthChallengeError::InvalidSmsRoleTrustRelationship(
                            String::from(error_message),
                        ),
                    )
                }
                "InvalidUserPoolConfigurationException" => {
                    return RusotoError::Service(
                        AdminRespondToAuthChallengeError::InvalidUserPoolConfiguration(
                            String::from(error_message),
                        ),
                    )
                }
                "MFAMethodNotFoundException" => {
                    return RusotoError::Service(
                        AdminRespondToAuthChallengeError::MFAMethodNotFound(String::from(
                            error_message,
                        )),
                    )
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminRespondToAuthChallengeError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "PasswordResetRequiredException" => {
                    return RusotoError::Service(
                        AdminRespondToAuthChallengeError::PasswordResetRequired(String::from(
                            error_message,
                        )),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        AdminRespondToAuthChallengeError::ResourceNotFound(String::from(
                            error_message,
                        )),
                    )
                }
                "SoftwareTokenMFANotFoundException" => {
                    return RusotoError::Service(
                        AdminRespondToAuthChallengeError::SoftwareTokenMFANotFound(String::from(
                            error_message,
                        )),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AdminRespondToAuthChallengeError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "UnexpectedLambdaException" => {
                    return RusotoError::Service(
                        AdminRespondToAuthChallengeError::UnexpectedLambda(String::from(
                            error_message,
                        )),
                    )
                }
                "UserLambdaValidationException" => {
                    return RusotoError::Service(
                        AdminRespondToAuthChallengeError::UserLambdaValidation(String::from(
                            error_message,
                        )),
                    )
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(
                        AdminRespondToAuthChallengeError::UserNotConfirmed(String::from(
                            error_message,
                        )),
                    )
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminRespondToAuthChallengeError::UserNotFound(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            AdminRespondToAuthChallengeError::SoftwareTokenMFANotFound(ref cause) => cause,
            AdminRespondToAuthChallengeError::TooManyRequests(ref cause) => cause,
            AdminRespondToAuthChallengeError::UnexpectedLambda(ref cause) => cause,
            AdminRespondToAuthChallengeError::UserLambdaValidation(ref cause) => cause,
            AdminRespondToAuthChallengeError::UserNotConfirmed(ref cause) => cause,
            AdminRespondToAuthChallengeError::UserNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by AdminSetUserMFAPreference
#[derive(Debug, PartialEq)]
pub enum AdminSetUserMFAPreferenceError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when a password reset is required.</p>
    PasswordResetRequired(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when a user is not confirmed successfully.</p>
    UserNotConfirmed(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
}

impl AdminSetUserMFAPreferenceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AdminSetUserMFAPreferenceError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(AdminSetUserMFAPreferenceError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AdminSetUserMFAPreferenceError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminSetUserMFAPreferenceError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "PasswordResetRequiredException" => {
                    return RusotoError::Service(
                        AdminSetUserMFAPreferenceError::PasswordResetRequired(String::from(
                            error_message,
                        )),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AdminSetUserMFAPreferenceError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(AdminSetUserMFAPreferenceError::UserNotConfirmed(
                        String::from(error_message),
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminSetUserMFAPreferenceError::UserNotFound(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AdminSetUserMFAPreferenceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AdminSetUserMFAPreferenceError {
    fn description(&self) -> &str {
        match *self {
            AdminSetUserMFAPreferenceError::InternalError(ref cause) => cause,
            AdminSetUserMFAPreferenceError::InvalidParameter(ref cause) => cause,
            AdminSetUserMFAPreferenceError::NotAuthorized(ref cause) => cause,
            AdminSetUserMFAPreferenceError::PasswordResetRequired(ref cause) => cause,
            AdminSetUserMFAPreferenceError::ResourceNotFound(ref cause) => cause,
            AdminSetUserMFAPreferenceError::UserNotConfirmed(ref cause) => cause,
            AdminSetUserMFAPreferenceError::UserNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by AdminSetUserSettings
#[derive(Debug, PartialEq)]
pub enum AdminSetUserSettingsError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
}

impl AdminSetUserSettingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AdminSetUserSettingsError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(AdminSetUserSettingsError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AdminSetUserSettingsError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminSetUserSettingsError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AdminSetUserSettingsError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminSetUserSettingsError::UserNotFound(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by AdminUpdateAuthEventFeedback
#[derive(Debug, PartialEq)]
pub enum AdminUpdateAuthEventFeedbackError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
    /// <p>This exception is thrown when user pool add-ons are not enabled.</p>
    UserPoolAddOnNotEnabled(String),
}

impl AdminUpdateAuthEventFeedbackError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AdminUpdateAuthEventFeedbackError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(AdminUpdateAuthEventFeedbackError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        AdminUpdateAuthEventFeedbackError::InvalidParameter(String::from(
                            error_message,
                        )),
                    )
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminUpdateAuthEventFeedbackError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        AdminUpdateAuthEventFeedbackError::ResourceNotFound(String::from(
                            error_message,
                        )),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        AdminUpdateAuthEventFeedbackError::TooManyRequests(String::from(
                            error_message,
                        )),
                    )
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminUpdateAuthEventFeedbackError::UserNotFound(
                        String::from(error_message),
                    ))
                }
                "UserPoolAddOnNotEnabledException" => {
                    return RusotoError::Service(
                        AdminUpdateAuthEventFeedbackError::UserPoolAddOnNotEnabled(String::from(
                            error_message,
                        )),
                    )
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AdminUpdateAuthEventFeedbackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AdminUpdateAuthEventFeedbackError {
    fn description(&self) -> &str {
        match *self {
            AdminUpdateAuthEventFeedbackError::InternalError(ref cause) => cause,
            AdminUpdateAuthEventFeedbackError::InvalidParameter(ref cause) => cause,
            AdminUpdateAuthEventFeedbackError::NotAuthorized(ref cause) => cause,
            AdminUpdateAuthEventFeedbackError::ResourceNotFound(ref cause) => cause,
            AdminUpdateAuthEventFeedbackError::TooManyRequests(ref cause) => cause,
            AdminUpdateAuthEventFeedbackError::UserNotFound(ref cause) => cause,
            AdminUpdateAuthEventFeedbackError::UserPoolAddOnNotEnabled(ref cause) => cause,
        }
    }
}
/// Errors returned by AdminUpdateDeviceStatus
#[derive(Debug, PartialEq)]
pub enum AdminUpdateDeviceStatusError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when the user pool configuration is invalid.</p>
    InvalidUserPoolConfiguration(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
}

impl AdminUpdateDeviceStatusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AdminUpdateDeviceStatusError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(AdminUpdateDeviceStatusError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AdminUpdateDeviceStatusError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "InvalidUserPoolConfigurationException" => {
                    return RusotoError::Service(
                        AdminUpdateDeviceStatusError::InvalidUserPoolConfiguration(String::from(
                            error_message,
                        )),
                    )
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminUpdateDeviceStatusError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AdminUpdateDeviceStatusError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AdminUpdateDeviceStatusError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminUpdateDeviceStatusError::UserNotFound(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by AdminUpdateUserAttributes
#[derive(Debug, PartialEq)]
pub enum AdminUpdateUserAttributesError {
    /// <p>This exception is thrown when a user tries to confirm the account with an email or phone number that has already been supplied as an alias from a different account. This exception tells user that an account with this email or phone already exists.</p>
    AliasExists(String),
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid AWS Lambda response.</p>
    InvalidLambdaResponse(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an unexpected exception with the AWS Lambda service.</p>
    UnexpectedLambda(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters a user validation exception with the AWS Lambda service.</p>
    UserLambdaValidation(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
}

impl AdminUpdateUserAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AdminUpdateUserAttributesError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AliasExistsException" => {
                    return RusotoError::Service(AdminUpdateUserAttributesError::AliasExists(
                        String::from(error_message),
                    ))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(AdminUpdateUserAttributesError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidLambdaResponseException" => {
                    return RusotoError::Service(
                        AdminUpdateUserAttributesError::InvalidLambdaResponse(String::from(
                            error_message,
                        )),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AdminUpdateUserAttributesError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminUpdateUserAttributesError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AdminUpdateUserAttributesError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AdminUpdateUserAttributesError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "UnexpectedLambdaException" => {
                    return RusotoError::Service(AdminUpdateUserAttributesError::UnexpectedLambda(
                        String::from(error_message),
                    ))
                }
                "UserLambdaValidationException" => {
                    return RusotoError::Service(
                        AdminUpdateUserAttributesError::UserLambdaValidation(String::from(
                            error_message,
                        )),
                    )
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminUpdateUserAttributesError::UserNotFound(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by AdminUserGlobalSignOut
#[derive(Debug, PartialEq)]
pub enum AdminUserGlobalSignOutError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
}

impl AdminUserGlobalSignOutError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AdminUserGlobalSignOutError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(AdminUserGlobalSignOutError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AdminUserGlobalSignOutError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminUserGlobalSignOutError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AdminUserGlobalSignOutError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AdminUserGlobalSignOutError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminUserGlobalSignOutError::UserNotFound(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by AssociateSoftwareToken
#[derive(Debug, PartialEq)]
pub enum AssociateSoftwareTokenError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the software token TOTP multi-factor authentication (MFA) is not enabled for the user pool.</p>
    SoftwareTokenMFANotFound(String),
}

impl AssociateSoftwareTokenError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssociateSoftwareTokenError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(AssociateSoftwareTokenError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AssociateSoftwareTokenError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AssociateSoftwareTokenError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AssociateSoftwareTokenError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "SoftwareTokenMFANotFoundException" => {
                    return RusotoError::Service(
                        AssociateSoftwareTokenError::SoftwareTokenMFANotFound(String::from(
                            error_message,
                        )),
                    )
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AssociateSoftwareTokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateSoftwareTokenError {
    fn description(&self) -> &str {
        match *self {
            AssociateSoftwareTokenError::InternalError(ref cause) => cause,
            AssociateSoftwareTokenError::InvalidParameter(ref cause) => cause,
            AssociateSoftwareTokenError::NotAuthorized(ref cause) => cause,
            AssociateSoftwareTokenError::ResourceNotFound(ref cause) => cause,
            AssociateSoftwareTokenError::SoftwareTokenMFANotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by ChangePassword
#[derive(Debug, PartialEq)]
pub enum ChangePasswordError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid password.</p>
    InvalidPassword(String),
    /// <p>This exception is thrown when a user exceeds the limit for a requested AWS resource.</p>
    LimitExceeded(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when a password reset is required.</p>
    PasswordResetRequired(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when a user is not confirmed successfully.</p>
    UserNotConfirmed(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
}

impl ChangePasswordError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ChangePasswordError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(ChangePasswordError::InternalError(String::from(
                        error_message,
                    )))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ChangePasswordError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "InvalidPasswordException" => {
                    return RusotoError::Service(ChangePasswordError::InvalidPassword(
                        String::from(error_message),
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ChangePasswordError::LimitExceeded(String::from(
                        error_message,
                    )))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(ChangePasswordError::NotAuthorized(String::from(
                        error_message,
                    )))
                }
                "PasswordResetRequiredException" => {
                    return RusotoError::Service(ChangePasswordError::PasswordResetRequired(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ChangePasswordError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ChangePasswordError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(ChangePasswordError::UserNotConfirmed(
                        String::from(error_message),
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(ChangePasswordError::UserNotFound(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by ConfirmDevice
#[derive(Debug, PartialEq)]
pub enum ConfirmDeviceError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid AWS Lambda response.</p>
    InvalidLambdaResponse(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid password.</p>
    InvalidPassword(String),
    /// <p>This exception is thrown when the user pool configuration is invalid.</p>
    InvalidUserPoolConfiguration(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when a password reset is required.</p>
    PasswordResetRequired(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when a user is not confirmed successfully.</p>
    UserNotConfirmed(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
    /// <p>This exception is thrown when Amazon Cognito encounters a user name that already exists in the user pool.</p>
    UsernameExists(String),
}

impl ConfirmDeviceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ConfirmDeviceError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(ConfirmDeviceError::InternalError(String::from(
                        error_message,
                    )))
                }
                "InvalidLambdaResponseException" => {
                    return RusotoError::Service(ConfirmDeviceError::InvalidLambdaResponse(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ConfirmDeviceError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "InvalidPasswordException" => {
                    return RusotoError::Service(ConfirmDeviceError::InvalidPassword(String::from(
                        error_message,
                    )))
                }
                "InvalidUserPoolConfigurationException" => {
                    return RusotoError::Service(ConfirmDeviceError::InvalidUserPoolConfiguration(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(ConfirmDeviceError::NotAuthorized(String::from(
                        error_message,
                    )))
                }
                "PasswordResetRequiredException" => {
                    return RusotoError::Service(ConfirmDeviceError::PasswordResetRequired(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ConfirmDeviceError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ConfirmDeviceError::TooManyRequests(String::from(
                        error_message,
                    )))
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(ConfirmDeviceError::UserNotConfirmed(
                        String::from(error_message),
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(ConfirmDeviceError::UserNotFound(String::from(
                        error_message,
                    )))
                }
                "UsernameExistsException" => {
                    return RusotoError::Service(ConfirmDeviceError::UsernameExists(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by ConfirmForgotPassword
#[derive(Debug, PartialEq)]
pub enum ConfirmForgotPasswordError {
    /// <p>This exception is thrown if the provided code does not match what the server was expecting.</p>
    CodeMismatch(String),
    /// <p>This exception is thrown if a code has expired.</p>
    ExpiredCode(String),
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid AWS Lambda response.</p>
    InvalidLambdaResponse(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid password.</p>
    InvalidPassword(String),
    /// <p>This exception is thrown when a user exceeds the limit for a requested AWS resource.</p>
    LimitExceeded(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many failed attempts for a given action (e.g., sign in).</p>
    TooManyFailedAttempts(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an unexpected exception with the AWS Lambda service.</p>
    UnexpectedLambda(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters a user validation exception with the AWS Lambda service.</p>
    UserLambdaValidation(String),
    /// <p>This exception is thrown when a user is not confirmed successfully.</p>
    UserNotConfirmed(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
}

impl ConfirmForgotPasswordError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ConfirmForgotPasswordError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "CodeMismatchException" => {
                    return RusotoError::Service(ConfirmForgotPasswordError::CodeMismatch(
                        String::from(error_message),
                    ))
                }
                "ExpiredCodeException" => {
                    return RusotoError::Service(ConfirmForgotPasswordError::ExpiredCode(
                        String::from(error_message),
                    ))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(ConfirmForgotPasswordError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidLambdaResponseException" => {
                    return RusotoError::Service(ConfirmForgotPasswordError::InvalidLambdaResponse(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ConfirmForgotPasswordError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "InvalidPasswordException" => {
                    return RusotoError::Service(ConfirmForgotPasswordError::InvalidPassword(
                        String::from(error_message),
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ConfirmForgotPasswordError::LimitExceeded(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(ConfirmForgotPasswordError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ConfirmForgotPasswordError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyFailedAttemptsException" => {
                    return RusotoError::Service(ConfirmForgotPasswordError::TooManyFailedAttempts(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ConfirmForgotPasswordError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "UnexpectedLambdaException" => {
                    return RusotoError::Service(ConfirmForgotPasswordError::UnexpectedLambda(
                        String::from(error_message),
                    ))
                }
                "UserLambdaValidationException" => {
                    return RusotoError::Service(ConfirmForgotPasswordError::UserLambdaValidation(
                        String::from(error_message),
                    ))
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(ConfirmForgotPasswordError::UserNotConfirmed(
                        String::from(error_message),
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(ConfirmForgotPasswordError::UserNotFound(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by ConfirmSignUp
#[derive(Debug, PartialEq)]
pub enum ConfirmSignUpError {
    /// <p>This exception is thrown when a user tries to confirm the account with an email or phone number that has already been supplied as an alias from a different account. This exception tells user that an account with this email or phone already exists.</p>
    AliasExists(String),
    /// <p>This exception is thrown if the provided code does not match what the server was expecting.</p>
    CodeMismatch(String),
    /// <p>This exception is thrown if a code has expired.</p>
    ExpiredCode(String),
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid AWS Lambda response.</p>
    InvalidLambdaResponse(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user exceeds the limit for a requested AWS resource.</p>
    LimitExceeded(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many failed attempts for a given action (e.g., sign in).</p>
    TooManyFailedAttempts(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an unexpected exception with the AWS Lambda service.</p>
    UnexpectedLambda(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters a user validation exception with the AWS Lambda service.</p>
    UserLambdaValidation(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
}

impl ConfirmSignUpError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ConfirmSignUpError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AliasExistsException" => {
                    return RusotoError::Service(ConfirmSignUpError::AliasExists(String::from(
                        error_message,
                    )))
                }
                "CodeMismatchException" => {
                    return RusotoError::Service(ConfirmSignUpError::CodeMismatch(String::from(
                        error_message,
                    )))
                }
                "ExpiredCodeException" => {
                    return RusotoError::Service(ConfirmSignUpError::ExpiredCode(String::from(
                        error_message,
                    )))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(ConfirmSignUpError::InternalError(String::from(
                        error_message,
                    )))
                }
                "InvalidLambdaResponseException" => {
                    return RusotoError::Service(ConfirmSignUpError::InvalidLambdaResponse(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ConfirmSignUpError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ConfirmSignUpError::LimitExceeded(String::from(
                        error_message,
                    )))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(ConfirmSignUpError::NotAuthorized(String::from(
                        error_message,
                    )))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ConfirmSignUpError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyFailedAttemptsException" => {
                    return RusotoError::Service(ConfirmSignUpError::TooManyFailedAttempts(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ConfirmSignUpError::TooManyRequests(String::from(
                        error_message,
                    )))
                }
                "UnexpectedLambdaException" => {
                    return RusotoError::Service(ConfirmSignUpError::UnexpectedLambda(
                        String::from(error_message),
                    ))
                }
                "UserLambdaValidationException" => {
                    return RusotoError::Service(ConfirmSignUpError::UserLambdaValidation(
                        String::from(error_message),
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(ConfirmSignUpError::UserNotFound(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by CreateGroup
#[derive(Debug, PartialEq)]
pub enum CreateGroupError {
    /// <p>This exception is thrown when Amazon Cognito encounters a group that already exists in the user pool.</p>
    GroupExists(String),
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user exceeds the limit for a requested AWS resource.</p>
    LimitExceeded(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
}

impl CreateGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateGroupError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "GroupExistsException" => {
                    return RusotoError::Service(CreateGroupError::GroupExists(String::from(
                        error_message,
                    )))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(CreateGroupError::InternalError(String::from(
                        error_message,
                    )))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateGroupError::InvalidParameter(String::from(
                        error_message,
                    )))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateGroupError::LimitExceeded(String::from(
                        error_message,
                    )))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(CreateGroupError::NotAuthorized(String::from(
                        error_message,
                    )))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateGroupError::ResourceNotFound(String::from(
                        error_message,
                    )))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateGroupError::TooManyRequests(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by CreateIdentityProvider
#[derive(Debug, PartialEq)]
pub enum CreateIdentityProviderError {
    /// <p>This exception is thrown when the provider is already supported by the user pool.</p>
    DuplicateProvider(String),
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user exceeds the limit for a requested AWS resource.</p>
    LimitExceeded(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
}

impl CreateIdentityProviderError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateIdentityProviderError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DuplicateProviderException" => {
                    return RusotoError::Service(CreateIdentityProviderError::DuplicateProvider(
                        String::from(error_message),
                    ))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(CreateIdentityProviderError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateIdentityProviderError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateIdentityProviderError::LimitExceeded(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(CreateIdentityProviderError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateIdentityProviderError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateIdentityProviderError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by CreateResourceServer
#[derive(Debug, PartialEq)]
pub enum CreateResourceServerError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user exceeds the limit for a requested AWS resource.</p>
    LimitExceeded(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
}

impl CreateResourceServerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateResourceServerError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(CreateResourceServerError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateResourceServerError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateResourceServerError::LimitExceeded(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(CreateResourceServerError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateResourceServerError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateResourceServerError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateResourceServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateResourceServerError {
    fn description(&self) -> &str {
        match *self {
            CreateResourceServerError::InternalError(ref cause) => cause,
            CreateResourceServerError::InvalidParameter(ref cause) => cause,
            CreateResourceServerError::LimitExceeded(ref cause) => cause,
            CreateResourceServerError::NotAuthorized(ref cause) => cause,
            CreateResourceServerError::ResourceNotFound(ref cause) => cause,
            CreateResourceServerError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateUserImportJob
#[derive(Debug, PartialEq)]
pub enum CreateUserImportJobError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user exceeds the limit for a requested AWS resource.</p>
    LimitExceeded(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when a precondition is not met.</p>
    PreconditionNotMet(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
}

impl CreateUserImportJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateUserImportJobError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(CreateUserImportJobError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateUserImportJobError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateUserImportJobError::LimitExceeded(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(CreateUserImportJobError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "PreconditionNotMetException" => {
                    return RusotoError::Service(CreateUserImportJobError::PreconditionNotMet(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateUserImportJobError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateUserImportJobError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by CreateUserPool
#[derive(Debug, PartialEq)]
pub enum CreateUserPoolError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when Amazon Cognito is not allowed to use your email identity. HTTP status code: 400.</p>
    InvalidEmailRoleAccessPolicy(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is returned when the role provided for SMS configuration does not have permission to publish using Amazon SNS.</p>
    InvalidSmsRoleAccessPolicy(String),
    /// <p>This exception is thrown when the trust relationship is invalid for the role provided for SMS configuration. This can happen if you do not trust <b>cognito-idp.amazonaws.com</b> or the external ID provided in the role does not match what is provided in the SMS configuration for the user pool.</p>
    InvalidSmsRoleTrustRelationship(String),
    /// <p>This exception is thrown when a user exceeds the limit for a requested AWS resource.</p>
    LimitExceeded(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when a user pool tag cannot be set or updated.</p>
    UserPoolTagging(String),
}

impl CreateUserPoolError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateUserPoolError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(CreateUserPoolError::InternalError(String::from(
                        error_message,
                    )))
                }
                "InvalidEmailRoleAccessPolicyException" => {
                    return RusotoError::Service(CreateUserPoolError::InvalidEmailRoleAccessPolicy(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateUserPoolError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "InvalidSmsRoleAccessPolicyException" => {
                    return RusotoError::Service(CreateUserPoolError::InvalidSmsRoleAccessPolicy(
                        String::from(error_message),
                    ))
                }
                "InvalidSmsRoleTrustRelationshipException" => {
                    return RusotoError::Service(
                        CreateUserPoolError::InvalidSmsRoleTrustRelationship(String::from(
                            error_message,
                        )),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateUserPoolError::LimitExceeded(String::from(
                        error_message,
                    )))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(CreateUserPoolError::NotAuthorized(String::from(
                        error_message,
                    )))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateUserPoolError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "UserPoolTaggingException" => {
                    return RusotoError::Service(CreateUserPoolError::UserPoolTagging(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by CreateUserPoolClient
#[derive(Debug, PartialEq)]
pub enum CreateUserPoolClientError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the specified OAuth flow is invalid.</p>
    InvalidOAuthFlow(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user exceeds the limit for a requested AWS resource.</p>
    LimitExceeded(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the specified scope does not exist.</p>
    ScopeDoesNotExist(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
}

impl CreateUserPoolClientError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateUserPoolClientError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(CreateUserPoolClientError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidOAuthFlowException" => {
                    return RusotoError::Service(CreateUserPoolClientError::InvalidOAuthFlow(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateUserPoolClientError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateUserPoolClientError::LimitExceeded(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(CreateUserPoolClientError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateUserPoolClientError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "ScopeDoesNotExistException" => {
                    return RusotoError::Service(CreateUserPoolClientError::ScopeDoesNotExist(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateUserPoolClientError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by CreateUserPoolDomain
#[derive(Debug, PartialEq)]
pub enum CreateUserPoolDomainError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user exceeds the limit for a requested AWS resource.</p>
    LimitExceeded(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
}

impl CreateUserPoolDomainError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateUserPoolDomainError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(CreateUserPoolDomainError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateUserPoolDomainError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateUserPoolDomainError::LimitExceeded(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(CreateUserPoolDomainError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateUserPoolDomainError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            CreateUserPoolDomainError::LimitExceeded(ref cause) => cause,
            CreateUserPoolDomainError::NotAuthorized(ref cause) => cause,
            CreateUserPoolDomainError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteGroup
#[derive(Debug, PartialEq)]
pub enum DeleteGroupError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
}

impl DeleteGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteGroupError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(DeleteGroupError::InternalError(String::from(
                        error_message,
                    )))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteGroupError::InvalidParameter(String::from(
                        error_message,
                    )))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(DeleteGroupError::NotAuthorized(String::from(
                        error_message,
                    )))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteGroupError::ResourceNotFound(String::from(
                        error_message,
                    )))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteGroupError::TooManyRequests(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by DeleteIdentityProvider
#[derive(Debug, PartialEq)]
pub enum DeleteIdentityProviderError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when the specified identifier is not supported.</p>
    UnsupportedIdentityProvider(String),
}

impl DeleteIdentityProviderError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteIdentityProviderError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(DeleteIdentityProviderError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteIdentityProviderError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(DeleteIdentityProviderError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteIdentityProviderError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteIdentityProviderError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "UnsupportedIdentityProviderException" => {
                    return RusotoError::Service(
                        DeleteIdentityProviderError::UnsupportedIdentityProvider(String::from(
                            error_message,
                        )),
                    )
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by DeleteResourceServer
#[derive(Debug, PartialEq)]
pub enum DeleteResourceServerError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
}

impl DeleteResourceServerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteResourceServerError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(DeleteResourceServerError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteResourceServerError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(DeleteResourceServerError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteResourceServerError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteResourceServerError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteResourceServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteResourceServerError {
    fn description(&self) -> &str {
        match *self {
            DeleteResourceServerError::InternalError(ref cause) => cause,
            DeleteResourceServerError::InvalidParameter(ref cause) => cause,
            DeleteResourceServerError::NotAuthorized(ref cause) => cause,
            DeleteResourceServerError::ResourceNotFound(ref cause) => cause,
            DeleteResourceServerError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteUser
#[derive(Debug, PartialEq)]
pub enum DeleteUserError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when a password reset is required.</p>
    PasswordResetRequired(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when a user is not confirmed successfully.</p>
    UserNotConfirmed(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
}

impl DeleteUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteUserError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(DeleteUserError::InternalError(String::from(
                        error_message,
                    )))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteUserError::InvalidParameter(String::from(
                        error_message,
                    )))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(DeleteUserError::NotAuthorized(String::from(
                        error_message,
                    )))
                }
                "PasswordResetRequiredException" => {
                    return RusotoError::Service(DeleteUserError::PasswordResetRequired(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteUserError::ResourceNotFound(String::from(
                        error_message,
                    )))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteUserError::TooManyRequests(String::from(
                        error_message,
                    )))
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(DeleteUserError::UserNotConfirmed(String::from(
                        error_message,
                    )))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(DeleteUserError::UserNotFound(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by DeleteUserAttributes
#[derive(Debug, PartialEq)]
pub enum DeleteUserAttributesError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when a password reset is required.</p>
    PasswordResetRequired(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when a user is not confirmed successfully.</p>
    UserNotConfirmed(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
}

impl DeleteUserAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteUserAttributesError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(DeleteUserAttributesError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteUserAttributesError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(DeleteUserAttributesError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "PasswordResetRequiredException" => {
                    return RusotoError::Service(DeleteUserAttributesError::PasswordResetRequired(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteUserAttributesError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteUserAttributesError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(DeleteUserAttributesError::UserNotConfirmed(
                        String::from(error_message),
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(DeleteUserAttributesError::UserNotFound(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by DeleteUserPool
#[derive(Debug, PartialEq)]
pub enum DeleteUserPoolError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when you are trying to modify a user pool while a user import job is in progress for that pool.</p>
    UserImportInProgress(String),
}

impl DeleteUserPoolError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteUserPoolError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(DeleteUserPoolError::InternalError(String::from(
                        error_message,
                    )))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteUserPoolError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(DeleteUserPoolError::NotAuthorized(String::from(
                        error_message,
                    )))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteUserPoolError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteUserPoolError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "UserImportInProgressException" => {
                    return RusotoError::Service(DeleteUserPoolError::UserImportInProgress(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by DeleteUserPoolClient
#[derive(Debug, PartialEq)]
pub enum DeleteUserPoolClientError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
}

impl DeleteUserPoolClientError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteUserPoolClientError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(DeleteUserPoolClientError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteUserPoolClientError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(DeleteUserPoolClientError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteUserPoolClientError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteUserPoolClientError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by DeleteUserPoolDomain
#[derive(Debug, PartialEq)]
pub enum DeleteUserPoolDomainError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
}

impl DeleteUserPoolDomainError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteUserPoolDomainError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(DeleteUserPoolDomainError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteUserPoolDomainError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(DeleteUserPoolDomainError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteUserPoolDomainError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by DescribeIdentityProvider
#[derive(Debug, PartialEq)]
pub enum DescribeIdentityProviderError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
}

impl DescribeIdentityProviderError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeIdentityProviderError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(DescribeIdentityProviderError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeIdentityProviderError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(DescribeIdentityProviderError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeIdentityProviderError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeIdentityProviderError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by DescribeResourceServer
#[derive(Debug, PartialEq)]
pub enum DescribeResourceServerError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
}

impl DescribeResourceServerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeResourceServerError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(DescribeResourceServerError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeResourceServerError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(DescribeResourceServerError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeResourceServerError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeResourceServerError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeResourceServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeResourceServerError {
    fn description(&self) -> &str {
        match *self {
            DescribeResourceServerError::InternalError(ref cause) => cause,
            DescribeResourceServerError::InvalidParameter(ref cause) => cause,
            DescribeResourceServerError::NotAuthorized(ref cause) => cause,
            DescribeResourceServerError::ResourceNotFound(ref cause) => cause,
            DescribeResourceServerError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeRiskConfiguration
#[derive(Debug, PartialEq)]
pub enum DescribeRiskConfigurationError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when user pool add-ons are not enabled.</p>
    UserPoolAddOnNotEnabled(String),
}

impl DescribeRiskConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeRiskConfigurationError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(DescribeRiskConfigurationError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeRiskConfigurationError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(DescribeRiskConfigurationError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeRiskConfigurationError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeRiskConfigurationError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "UserPoolAddOnNotEnabledException" => {
                    return RusotoError::Service(
                        DescribeRiskConfigurationError::UserPoolAddOnNotEnabled(String::from(
                            error_message,
                        )),
                    )
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeRiskConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeRiskConfigurationError {
    fn description(&self) -> &str {
        match *self {
            DescribeRiskConfigurationError::InternalError(ref cause) => cause,
            DescribeRiskConfigurationError::InvalidParameter(ref cause) => cause,
            DescribeRiskConfigurationError::NotAuthorized(ref cause) => cause,
            DescribeRiskConfigurationError::ResourceNotFound(ref cause) => cause,
            DescribeRiskConfigurationError::TooManyRequests(ref cause) => cause,
            DescribeRiskConfigurationError::UserPoolAddOnNotEnabled(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeUserImportJob
#[derive(Debug, PartialEq)]
pub enum DescribeUserImportJobError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
}

impl DescribeUserImportJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeUserImportJobError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(DescribeUserImportJobError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeUserImportJobError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(DescribeUserImportJobError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeUserImportJobError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeUserImportJobError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by DescribeUserPool
#[derive(Debug, PartialEq)]
pub enum DescribeUserPoolError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when a user pool tag cannot be set or updated.</p>
    UserPoolTagging(String),
}

impl DescribeUserPoolError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeUserPoolError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(DescribeUserPoolError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeUserPoolError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(DescribeUserPoolError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeUserPoolError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeUserPoolError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "UserPoolTaggingException" => {
                    return RusotoError::Service(DescribeUserPoolError::UserPoolTagging(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by DescribeUserPoolClient
#[derive(Debug, PartialEq)]
pub enum DescribeUserPoolClientError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
}

impl DescribeUserPoolClientError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeUserPoolClientError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(DescribeUserPoolClientError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeUserPoolClientError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(DescribeUserPoolClientError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeUserPoolClientError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeUserPoolClientError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by DescribeUserPoolDomain
#[derive(Debug, PartialEq)]
pub enum DescribeUserPoolDomainError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
}

impl DescribeUserPoolDomainError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeUserPoolDomainError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(DescribeUserPoolDomainError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeUserPoolDomainError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(DescribeUserPoolDomainError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeUserPoolDomainError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by ForgetDevice
#[derive(Debug, PartialEq)]
pub enum ForgetDeviceError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when the user pool configuration is invalid.</p>
    InvalidUserPoolConfiguration(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when a password reset is required.</p>
    PasswordResetRequired(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when a user is not confirmed successfully.</p>
    UserNotConfirmed(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
}

impl ForgetDeviceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ForgetDeviceError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(ForgetDeviceError::InternalError(String::from(
                        error_message,
                    )))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ForgetDeviceError::InvalidParameter(String::from(
                        error_message,
                    )))
                }
                "InvalidUserPoolConfigurationException" => {
                    return RusotoError::Service(ForgetDeviceError::InvalidUserPoolConfiguration(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(ForgetDeviceError::NotAuthorized(String::from(
                        error_message,
                    )))
                }
                "PasswordResetRequiredException" => {
                    return RusotoError::Service(ForgetDeviceError::PasswordResetRequired(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ForgetDeviceError::ResourceNotFound(String::from(
                        error_message,
                    )))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ForgetDeviceError::TooManyRequests(String::from(
                        error_message,
                    )))
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(ForgetDeviceError::UserNotConfirmed(String::from(
                        error_message,
                    )))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(ForgetDeviceError::UserNotFound(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by ForgotPassword
#[derive(Debug, PartialEq)]
pub enum ForgotPasswordError {
    /// <p>This exception is thrown when a verification code fails to deliver successfully.</p>
    CodeDeliveryFailure(String),
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when Amazon Cognito is not allowed to use your email identity. HTTP status code: 400.</p>
    InvalidEmailRoleAccessPolicy(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid AWS Lambda response.</p>
    InvalidLambdaResponse(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is returned when the role provided for SMS configuration does not have permission to publish using Amazon SNS.</p>
    InvalidSmsRoleAccessPolicy(String),
    /// <p>This exception is thrown when the trust relationship is invalid for the role provided for SMS configuration. This can happen if you do not trust <b>cognito-idp.amazonaws.com</b> or the external ID provided in the role does not match what is provided in the SMS configuration for the user pool.</p>
    InvalidSmsRoleTrustRelationship(String),
    /// <p>This exception is thrown when a user exceeds the limit for a requested AWS resource.</p>
    LimitExceeded(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an unexpected exception with the AWS Lambda service.</p>
    UnexpectedLambda(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters a user validation exception with the AWS Lambda service.</p>
    UserLambdaValidation(String),
    /// <p>This exception is thrown when a user is not confirmed successfully.</p>
    UserNotConfirmed(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
}

impl ForgotPasswordError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ForgotPasswordError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "CodeDeliveryFailureException" => {
                    return RusotoError::Service(ForgotPasswordError::CodeDeliveryFailure(
                        String::from(error_message),
                    ))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(ForgotPasswordError::InternalError(String::from(
                        error_message,
                    )))
                }
                "InvalidEmailRoleAccessPolicyException" => {
                    return RusotoError::Service(ForgotPasswordError::InvalidEmailRoleAccessPolicy(
                        String::from(error_message),
                    ))
                }
                "InvalidLambdaResponseException" => {
                    return RusotoError::Service(ForgotPasswordError::InvalidLambdaResponse(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ForgotPasswordError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "InvalidSmsRoleAccessPolicyException" => {
                    return RusotoError::Service(ForgotPasswordError::InvalidSmsRoleAccessPolicy(
                        String::from(error_message),
                    ))
                }
                "InvalidSmsRoleTrustRelationshipException" => {
                    return RusotoError::Service(
                        ForgotPasswordError::InvalidSmsRoleTrustRelationship(String::from(
                            error_message,
                        )),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ForgotPasswordError::LimitExceeded(String::from(
                        error_message,
                    )))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(ForgotPasswordError::NotAuthorized(String::from(
                        error_message,
                    )))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ForgotPasswordError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ForgotPasswordError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "UnexpectedLambdaException" => {
                    return RusotoError::Service(ForgotPasswordError::UnexpectedLambda(
                        String::from(error_message),
                    ))
                }
                "UserLambdaValidationException" => {
                    return RusotoError::Service(ForgotPasswordError::UserLambdaValidation(
                        String::from(error_message),
                    ))
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(ForgotPasswordError::UserNotConfirmed(
                        String::from(error_message),
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(ForgotPasswordError::UserNotFound(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by GetCSVHeader
#[derive(Debug, PartialEq)]
pub enum GetCSVHeaderError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
}

impl GetCSVHeaderError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetCSVHeaderError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(GetCSVHeaderError::InternalError(String::from(
                        error_message,
                    )))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetCSVHeaderError::InvalidParameter(String::from(
                        error_message,
                    )))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(GetCSVHeaderError::NotAuthorized(String::from(
                        error_message,
                    )))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetCSVHeaderError::ResourceNotFound(String::from(
                        error_message,
                    )))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetCSVHeaderError::TooManyRequests(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by GetDevice
#[derive(Debug, PartialEq)]
pub enum GetDeviceError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when the user pool configuration is invalid.</p>
    InvalidUserPoolConfiguration(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when a password reset is required.</p>
    PasswordResetRequired(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when a user is not confirmed successfully.</p>
    UserNotConfirmed(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
}

impl GetDeviceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDeviceError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(GetDeviceError::InternalError(String::from(
                        error_message,
                    )))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetDeviceError::InvalidParameter(String::from(
                        error_message,
                    )))
                }
                "InvalidUserPoolConfigurationException" => {
                    return RusotoError::Service(GetDeviceError::InvalidUserPoolConfiguration(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(GetDeviceError::NotAuthorized(String::from(
                        error_message,
                    )))
                }
                "PasswordResetRequiredException" => {
                    return RusotoError::Service(GetDeviceError::PasswordResetRequired(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetDeviceError::ResourceNotFound(String::from(
                        error_message,
                    )))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetDeviceError::TooManyRequests(String::from(
                        error_message,
                    )))
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(GetDeviceError::UserNotConfirmed(String::from(
                        error_message,
                    )))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(GetDeviceError::UserNotFound(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by GetGroup
#[derive(Debug, PartialEq)]
pub enum GetGroupError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
}

impl GetGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetGroupError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(GetGroupError::InternalError(String::from(
                        error_message,
                    )))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetGroupError::InvalidParameter(String::from(
                        error_message,
                    )))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(GetGroupError::NotAuthorized(String::from(
                        error_message,
                    )))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetGroupError::ResourceNotFound(String::from(
                        error_message,
                    )))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetGroupError::TooManyRequests(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by GetIdentityProviderByIdentifier
#[derive(Debug, PartialEq)]
pub enum GetIdentityProviderByIdentifierError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
}

impl GetIdentityProviderByIdentifierError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetIdentityProviderByIdentifierError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(
                        GetIdentityProviderByIdentifierError::InternalError(String::from(
                            error_message,
                        )),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        GetIdentityProviderByIdentifierError::InvalidParameter(String::from(
                            error_message,
                        )),
                    )
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(
                        GetIdentityProviderByIdentifierError::NotAuthorized(String::from(
                            error_message,
                        )),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        GetIdentityProviderByIdentifierError::ResourceNotFound(String::from(
                            error_message,
                        )),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        GetIdentityProviderByIdentifierError::TooManyRequests(String::from(
                            error_message,
                        )),
                    )
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by GetSigningCertificate
#[derive(Debug, PartialEq)]
pub enum GetSigningCertificateError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
}

impl GetSigningCertificateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSigningCertificateError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(GetSigningCertificateError::InternalError(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetSigningCertificateError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetSigningCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSigningCertificateError {
    fn description(&self) -> &str {
        match *self {
            GetSigningCertificateError::InternalError(ref cause) => cause,
            GetSigningCertificateError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by GetUICustomization
#[derive(Debug, PartialEq)]
pub enum GetUICustomizationError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
}

impl GetUICustomizationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetUICustomizationError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(GetUICustomizationError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetUICustomizationError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(GetUICustomizationError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetUICustomizationError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetUICustomizationError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetUICustomizationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetUICustomizationError {
    fn description(&self) -> &str {
        match *self {
            GetUICustomizationError::InternalError(ref cause) => cause,
            GetUICustomizationError::InvalidParameter(ref cause) => cause,
            GetUICustomizationError::NotAuthorized(ref cause) => cause,
            GetUICustomizationError::ResourceNotFound(ref cause) => cause,
            GetUICustomizationError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetUser
#[derive(Debug, PartialEq)]
pub enum GetUserError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when a password reset is required.</p>
    PasswordResetRequired(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when a user is not confirmed successfully.</p>
    UserNotConfirmed(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
}

impl GetUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetUserError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(GetUserError::InternalError(String::from(
                        error_message,
                    )))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetUserError::InvalidParameter(String::from(
                        error_message,
                    )))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(GetUserError::NotAuthorized(String::from(
                        error_message,
                    )))
                }
                "PasswordResetRequiredException" => {
                    return RusotoError::Service(GetUserError::PasswordResetRequired(String::from(
                        error_message,
                    )))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetUserError::ResourceNotFound(String::from(
                        error_message,
                    )))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetUserError::TooManyRequests(String::from(
                        error_message,
                    )))
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(GetUserError::UserNotConfirmed(String::from(
                        error_message,
                    )))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(GetUserError::UserNotFound(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
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
            GetUserError::InternalError(ref cause) => cause,
            GetUserError::InvalidParameter(ref cause) => cause,
            GetUserError::NotAuthorized(ref cause) => cause,
            GetUserError::PasswordResetRequired(ref cause) => cause,
            GetUserError::ResourceNotFound(ref cause) => cause,
            GetUserError::TooManyRequests(ref cause) => cause,
            GetUserError::UserNotConfirmed(ref cause) => cause,
            GetUserError::UserNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by GetUserAttributeVerificationCode
#[derive(Debug, PartialEq)]
pub enum GetUserAttributeVerificationCodeError {
    /// <p>This exception is thrown when a verification code fails to deliver successfully.</p>
    CodeDeliveryFailure(String),
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when Amazon Cognito is not allowed to use your email identity. HTTP status code: 400.</p>
    InvalidEmailRoleAccessPolicy(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid AWS Lambda response.</p>
    InvalidLambdaResponse(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is returned when the role provided for SMS configuration does not have permission to publish using Amazon SNS.</p>
    InvalidSmsRoleAccessPolicy(String),
    /// <p>This exception is thrown when the trust relationship is invalid for the role provided for SMS configuration. This can happen if you do not trust <b>cognito-idp.amazonaws.com</b> or the external ID provided in the role does not match what is provided in the SMS configuration for the user pool.</p>
    InvalidSmsRoleTrustRelationship(String),
    /// <p>This exception is thrown when a user exceeds the limit for a requested AWS resource.</p>
    LimitExceeded(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when a password reset is required.</p>
    PasswordResetRequired(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an unexpected exception with the AWS Lambda service.</p>
    UnexpectedLambda(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters a user validation exception with the AWS Lambda service.</p>
    UserLambdaValidation(String),
    /// <p>This exception is thrown when a user is not confirmed successfully.</p>
    UserNotConfirmed(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
}

impl GetUserAttributeVerificationCodeError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetUserAttributeVerificationCodeError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "CodeDeliveryFailureException" => {
                    return RusotoError::Service(
                        GetUserAttributeVerificationCodeError::CodeDeliveryFailure(String::from(
                            error_message,
                        )),
                    )
                }
                "InternalErrorException" => {
                    return RusotoError::Service(
                        GetUserAttributeVerificationCodeError::InternalError(String::from(
                            error_message,
                        )),
                    )
                }
                "InvalidEmailRoleAccessPolicyException" => {
                    return RusotoError::Service(
                        GetUserAttributeVerificationCodeError::InvalidEmailRoleAccessPolicy(
                            String::from(error_message),
                        ),
                    )
                }
                "InvalidLambdaResponseException" => {
                    return RusotoError::Service(
                        GetUserAttributeVerificationCodeError::InvalidLambdaResponse(String::from(
                            error_message,
                        )),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        GetUserAttributeVerificationCodeError::InvalidParameter(String::from(
                            error_message,
                        )),
                    )
                }
                "InvalidSmsRoleAccessPolicyException" => {
                    return RusotoError::Service(
                        GetUserAttributeVerificationCodeError::InvalidSmsRoleAccessPolicy(
                            String::from(error_message),
                        ),
                    )
                }
                "InvalidSmsRoleTrustRelationshipException" => {
                    return RusotoError::Service(
                        GetUserAttributeVerificationCodeError::InvalidSmsRoleTrustRelationship(
                            String::from(error_message),
                        ),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(
                        GetUserAttributeVerificationCodeError::LimitExceeded(String::from(
                            error_message,
                        )),
                    )
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(
                        GetUserAttributeVerificationCodeError::NotAuthorized(String::from(
                            error_message,
                        )),
                    )
                }
                "PasswordResetRequiredException" => {
                    return RusotoError::Service(
                        GetUserAttributeVerificationCodeError::PasswordResetRequired(String::from(
                            error_message,
                        )),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        GetUserAttributeVerificationCodeError::ResourceNotFound(String::from(
                            error_message,
                        )),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        GetUserAttributeVerificationCodeError::TooManyRequests(String::from(
                            error_message,
                        )),
                    )
                }
                "UnexpectedLambdaException" => {
                    return RusotoError::Service(
                        GetUserAttributeVerificationCodeError::UnexpectedLambda(String::from(
                            error_message,
                        )),
                    )
                }
                "UserLambdaValidationException" => {
                    return RusotoError::Service(
                        GetUserAttributeVerificationCodeError::UserLambdaValidation(String::from(
                            error_message,
                        )),
                    )
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(
                        GetUserAttributeVerificationCodeError::UserNotConfirmed(String::from(
                            error_message,
                        )),
                    )
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(
                        GetUserAttributeVerificationCodeError::UserNotFound(String::from(
                            error_message,
                        )),
                    )
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by GetUserPoolMfaConfig
#[derive(Debug, PartialEq)]
pub enum GetUserPoolMfaConfigError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
}

impl GetUserPoolMfaConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetUserPoolMfaConfigError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(GetUserPoolMfaConfigError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetUserPoolMfaConfigError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(GetUserPoolMfaConfigError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetUserPoolMfaConfigError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetUserPoolMfaConfigError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetUserPoolMfaConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetUserPoolMfaConfigError {
    fn description(&self) -> &str {
        match *self {
            GetUserPoolMfaConfigError::InternalError(ref cause) => cause,
            GetUserPoolMfaConfigError::InvalidParameter(ref cause) => cause,
            GetUserPoolMfaConfigError::NotAuthorized(ref cause) => cause,
            GetUserPoolMfaConfigError::ResourceNotFound(ref cause) => cause,
            GetUserPoolMfaConfigError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GlobalSignOut
#[derive(Debug, PartialEq)]
pub enum GlobalSignOutError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when a password reset is required.</p>
    PasswordResetRequired(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when a user is not confirmed successfully.</p>
    UserNotConfirmed(String),
}

impl GlobalSignOutError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GlobalSignOutError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(GlobalSignOutError::InternalError(String::from(
                        error_message,
                    )))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GlobalSignOutError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(GlobalSignOutError::NotAuthorized(String::from(
                        error_message,
                    )))
                }
                "PasswordResetRequiredException" => {
                    return RusotoError::Service(GlobalSignOutError::PasswordResetRequired(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GlobalSignOutError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GlobalSignOutError::TooManyRequests(String::from(
                        error_message,
                    )))
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(GlobalSignOutError::UserNotConfirmed(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by InitiateAuth
#[derive(Debug, PartialEq)]
pub enum InitiateAuthError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid AWS Lambda response.</p>
    InvalidLambdaResponse(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when the user pool configuration is invalid.</p>
    InvalidUserPoolConfiguration(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when a password reset is required.</p>
    PasswordResetRequired(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an unexpected exception with the AWS Lambda service.</p>
    UnexpectedLambda(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters a user validation exception with the AWS Lambda service.</p>
    UserLambdaValidation(String),
    /// <p>This exception is thrown when a user is not confirmed successfully.</p>
    UserNotConfirmed(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
}

impl InitiateAuthError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<InitiateAuthError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(InitiateAuthError::InternalError(String::from(
                        error_message,
                    )))
                }
                "InvalidLambdaResponseException" => {
                    return RusotoError::Service(InitiateAuthError::InvalidLambdaResponse(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(InitiateAuthError::InvalidParameter(String::from(
                        error_message,
                    )))
                }
                "InvalidUserPoolConfigurationException" => {
                    return RusotoError::Service(InitiateAuthError::InvalidUserPoolConfiguration(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(InitiateAuthError::NotAuthorized(String::from(
                        error_message,
                    )))
                }
                "PasswordResetRequiredException" => {
                    return RusotoError::Service(InitiateAuthError::PasswordResetRequired(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(InitiateAuthError::ResourceNotFound(String::from(
                        error_message,
                    )))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(InitiateAuthError::TooManyRequests(String::from(
                        error_message,
                    )))
                }
                "UnexpectedLambdaException" => {
                    return RusotoError::Service(InitiateAuthError::UnexpectedLambda(String::from(
                        error_message,
                    )))
                }
                "UserLambdaValidationException" => {
                    return RusotoError::Service(InitiateAuthError::UserLambdaValidation(
                        String::from(error_message),
                    ))
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(InitiateAuthError::UserNotConfirmed(String::from(
                        error_message,
                    )))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(InitiateAuthError::UserNotFound(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by ListDevices
#[derive(Debug, PartialEq)]
pub enum ListDevicesError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when the user pool configuration is invalid.</p>
    InvalidUserPoolConfiguration(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when a password reset is required.</p>
    PasswordResetRequired(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when a user is not confirmed successfully.</p>
    UserNotConfirmed(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
}

impl ListDevicesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDevicesError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(ListDevicesError::InternalError(String::from(
                        error_message,
                    )))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListDevicesError::InvalidParameter(String::from(
                        error_message,
                    )))
                }
                "InvalidUserPoolConfigurationException" => {
                    return RusotoError::Service(ListDevicesError::InvalidUserPoolConfiguration(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(ListDevicesError::NotAuthorized(String::from(
                        error_message,
                    )))
                }
                "PasswordResetRequiredException" => {
                    return RusotoError::Service(ListDevicesError::PasswordResetRequired(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListDevicesError::ResourceNotFound(String::from(
                        error_message,
                    )))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListDevicesError::TooManyRequests(String::from(
                        error_message,
                    )))
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(ListDevicesError::UserNotConfirmed(String::from(
                        error_message,
                    )))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(ListDevicesError::UserNotFound(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by ListGroups
#[derive(Debug, PartialEq)]
pub enum ListGroupsError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
}

impl ListGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListGroupsError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(ListGroupsError::InternalError(String::from(
                        error_message,
                    )))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListGroupsError::InvalidParameter(String::from(
                        error_message,
                    )))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(ListGroupsError::NotAuthorized(String::from(
                        error_message,
                    )))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListGroupsError::ResourceNotFound(String::from(
                        error_message,
                    )))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListGroupsError::TooManyRequests(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by ListIdentityProviders
#[derive(Debug, PartialEq)]
pub enum ListIdentityProvidersError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
}

impl ListIdentityProvidersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListIdentityProvidersError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(ListIdentityProvidersError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListIdentityProvidersError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(ListIdentityProvidersError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListIdentityProvidersError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListIdentityProvidersError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by ListResourceServers
#[derive(Debug, PartialEq)]
pub enum ListResourceServersError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
}

impl ListResourceServersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListResourceServersError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(ListResourceServersError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListResourceServersError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(ListResourceServersError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListResourceServersError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListResourceServersError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListResourceServersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListResourceServersError {
    fn description(&self) -> &str {
        match *self {
            ListResourceServersError::InternalError(ref cause) => cause,
            ListResourceServersError::InvalidParameter(ref cause) => cause,
            ListResourceServersError::NotAuthorized(ref cause) => cause,
            ListResourceServersError::ResourceNotFound(ref cause) => cause,
            ListResourceServersError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by ListUserImportJobs
#[derive(Debug, PartialEq)]
pub enum ListUserImportJobsError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
}

impl ListUserImportJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListUserImportJobsError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(ListUserImportJobsError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListUserImportJobsError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(ListUserImportJobsError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListUserImportJobsError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListUserImportJobsError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by ListUserPoolClients
#[derive(Debug, PartialEq)]
pub enum ListUserPoolClientsError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
}

impl ListUserPoolClientsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListUserPoolClientsError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(ListUserPoolClientsError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListUserPoolClientsError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(ListUserPoolClientsError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListUserPoolClientsError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListUserPoolClientsError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by ListUserPools
#[derive(Debug, PartialEq)]
pub enum ListUserPoolsError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
}

impl ListUserPoolsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListUserPoolsError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(ListUserPoolsError::InternalError(String::from(
                        error_message,
                    )))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListUserPoolsError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(ListUserPoolsError::NotAuthorized(String::from(
                        error_message,
                    )))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListUserPoolsError::TooManyRequests(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by ListUsers
#[derive(Debug, PartialEq)]
pub enum ListUsersError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
}

impl ListUsersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListUsersError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(ListUsersError::InternalError(String::from(
                        error_message,
                    )))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListUsersError::InvalidParameter(String::from(
                        error_message,
                    )))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(ListUsersError::NotAuthorized(String::from(
                        error_message,
                    )))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListUsersError::ResourceNotFound(String::from(
                        error_message,
                    )))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListUsersError::TooManyRequests(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
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
            ListUsersError::InternalError(ref cause) => cause,
            ListUsersError::InvalidParameter(ref cause) => cause,
            ListUsersError::NotAuthorized(ref cause) => cause,
            ListUsersError::ResourceNotFound(ref cause) => cause,
            ListUsersError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by ListUsersInGroup
#[derive(Debug, PartialEq)]
pub enum ListUsersInGroupError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
}

impl ListUsersInGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListUsersInGroupError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(ListUsersInGroupError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListUsersInGroupError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(ListUsersInGroupError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListUsersInGroupError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListUsersInGroupError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by ResendConfirmationCode
#[derive(Debug, PartialEq)]
pub enum ResendConfirmationCodeError {
    /// <p>This exception is thrown when a verification code fails to deliver successfully.</p>
    CodeDeliveryFailure(String),
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when Amazon Cognito is not allowed to use your email identity. HTTP status code: 400.</p>
    InvalidEmailRoleAccessPolicy(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid AWS Lambda response.</p>
    InvalidLambdaResponse(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is returned when the role provided for SMS configuration does not have permission to publish using Amazon SNS.</p>
    InvalidSmsRoleAccessPolicy(String),
    /// <p>This exception is thrown when the trust relationship is invalid for the role provided for SMS configuration. This can happen if you do not trust <b>cognito-idp.amazonaws.com</b> or the external ID provided in the role does not match what is provided in the SMS configuration for the user pool.</p>
    InvalidSmsRoleTrustRelationship(String),
    /// <p>This exception is thrown when a user exceeds the limit for a requested AWS resource.</p>
    LimitExceeded(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an unexpected exception with the AWS Lambda service.</p>
    UnexpectedLambda(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters a user validation exception with the AWS Lambda service.</p>
    UserLambdaValidation(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
}

impl ResendConfirmationCodeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ResendConfirmationCodeError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "CodeDeliveryFailureException" => {
                    return RusotoError::Service(ResendConfirmationCodeError::CodeDeliveryFailure(
                        String::from(error_message),
                    ))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(ResendConfirmationCodeError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidEmailRoleAccessPolicyException" => {
                    return RusotoError::Service(
                        ResendConfirmationCodeError::InvalidEmailRoleAccessPolicy(String::from(
                            error_message,
                        )),
                    )
                }
                "InvalidLambdaResponseException" => {
                    return RusotoError::Service(
                        ResendConfirmationCodeError::InvalidLambdaResponse(String::from(
                            error_message,
                        )),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ResendConfirmationCodeError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "InvalidSmsRoleAccessPolicyException" => {
                    return RusotoError::Service(
                        ResendConfirmationCodeError::InvalidSmsRoleAccessPolicy(String::from(
                            error_message,
                        )),
                    )
                }
                "InvalidSmsRoleTrustRelationshipException" => {
                    return RusotoError::Service(
                        ResendConfirmationCodeError::InvalidSmsRoleTrustRelationship(String::from(
                            error_message,
                        )),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ResendConfirmationCodeError::LimitExceeded(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(ResendConfirmationCodeError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ResendConfirmationCodeError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ResendConfirmationCodeError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "UnexpectedLambdaException" => {
                    return RusotoError::Service(ResendConfirmationCodeError::UnexpectedLambda(
                        String::from(error_message),
                    ))
                }
                "UserLambdaValidationException" => {
                    return RusotoError::Service(ResendConfirmationCodeError::UserLambdaValidation(
                        String::from(error_message),
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(ResendConfirmationCodeError::UserNotFound(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by RespondToAuthChallenge
#[derive(Debug, PartialEq)]
pub enum RespondToAuthChallengeError {
    /// <p>This exception is thrown when a user tries to confirm the account with an email or phone number that has already been supplied as an alias from a different account. This exception tells user that an account with this email or phone already exists.</p>
    AliasExists(String),
    /// <p>This exception is thrown if the provided code does not match what the server was expecting.</p>
    CodeMismatch(String),
    /// <p>This exception is thrown if a code has expired.</p>
    ExpiredCode(String),
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid AWS Lambda response.</p>
    InvalidLambdaResponse(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid password.</p>
    InvalidPassword(String),
    /// <p>This exception is returned when the role provided for SMS configuration does not have permission to publish using Amazon SNS.</p>
    InvalidSmsRoleAccessPolicy(String),
    /// <p>This exception is thrown when the trust relationship is invalid for the role provided for SMS configuration. This can happen if you do not trust <b>cognito-idp.amazonaws.com</b> or the external ID provided in the role does not match what is provided in the SMS configuration for the user pool.</p>
    InvalidSmsRoleTrustRelationship(String),
    /// <p>This exception is thrown when the user pool configuration is invalid.</p>
    InvalidUserPoolConfiguration(String),
    /// <p>This exception is thrown when Amazon Cognito cannot find a multi-factor authentication (MFA) method.</p>
    MFAMethodNotFound(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when a password reset is required.</p>
    PasswordResetRequired(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the software token TOTP multi-factor authentication (MFA) is not enabled for the user pool.</p>
    SoftwareTokenMFANotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an unexpected exception with the AWS Lambda service.</p>
    UnexpectedLambda(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters a user validation exception with the AWS Lambda service.</p>
    UserLambdaValidation(String),
    /// <p>This exception is thrown when a user is not confirmed successfully.</p>
    UserNotConfirmed(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
}

impl RespondToAuthChallengeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RespondToAuthChallengeError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AliasExistsException" => {
                    return RusotoError::Service(RespondToAuthChallengeError::AliasExists(
                        String::from(error_message),
                    ))
                }
                "CodeMismatchException" => {
                    return RusotoError::Service(RespondToAuthChallengeError::CodeMismatch(
                        String::from(error_message),
                    ))
                }
                "ExpiredCodeException" => {
                    return RusotoError::Service(RespondToAuthChallengeError::ExpiredCode(
                        String::from(error_message),
                    ))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(RespondToAuthChallengeError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidLambdaResponseException" => {
                    return RusotoError::Service(
                        RespondToAuthChallengeError::InvalidLambdaResponse(String::from(
                            error_message,
                        )),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(RespondToAuthChallengeError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "InvalidPasswordException" => {
                    return RusotoError::Service(RespondToAuthChallengeError::InvalidPassword(
                        String::from(error_message),
                    ))
                }
                "InvalidSmsRoleAccessPolicyException" => {
                    return RusotoError::Service(
                        RespondToAuthChallengeError::InvalidSmsRoleAccessPolicy(String::from(
                            error_message,
                        )),
                    )
                }
                "InvalidSmsRoleTrustRelationshipException" => {
                    return RusotoError::Service(
                        RespondToAuthChallengeError::InvalidSmsRoleTrustRelationship(String::from(
                            error_message,
                        )),
                    )
                }
                "InvalidUserPoolConfigurationException" => {
                    return RusotoError::Service(
                        RespondToAuthChallengeError::InvalidUserPoolConfiguration(String::from(
                            error_message,
                        )),
                    )
                }
                "MFAMethodNotFoundException" => {
                    return RusotoError::Service(RespondToAuthChallengeError::MFAMethodNotFound(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(RespondToAuthChallengeError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "PasswordResetRequiredException" => {
                    return RusotoError::Service(
                        RespondToAuthChallengeError::PasswordResetRequired(String::from(
                            error_message,
                        )),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(RespondToAuthChallengeError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "SoftwareTokenMFANotFoundException" => {
                    return RusotoError::Service(
                        RespondToAuthChallengeError::SoftwareTokenMFANotFound(String::from(
                            error_message,
                        )),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(RespondToAuthChallengeError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "UnexpectedLambdaException" => {
                    return RusotoError::Service(RespondToAuthChallengeError::UnexpectedLambda(
                        String::from(error_message),
                    ))
                }
                "UserLambdaValidationException" => {
                    return RusotoError::Service(RespondToAuthChallengeError::UserLambdaValidation(
                        String::from(error_message),
                    ))
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(RespondToAuthChallengeError::UserNotConfirmed(
                        String::from(error_message),
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(RespondToAuthChallengeError::UserNotFound(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            RespondToAuthChallengeError::SoftwareTokenMFANotFound(ref cause) => cause,
            RespondToAuthChallengeError::TooManyRequests(ref cause) => cause,
            RespondToAuthChallengeError::UnexpectedLambda(ref cause) => cause,
            RespondToAuthChallengeError::UserLambdaValidation(ref cause) => cause,
            RespondToAuthChallengeError::UserNotConfirmed(ref cause) => cause,
            RespondToAuthChallengeError::UserNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by SetRiskConfiguration
#[derive(Debug, PartialEq)]
pub enum SetRiskConfigurationError {
    /// <p>This exception is thrown when a verification code fails to deliver successfully.</p>
    CodeDeliveryFailure(String),
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when Amazon Cognito is not allowed to use your email identity. HTTP status code: 400.</p>
    InvalidEmailRoleAccessPolicy(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when user pool add-ons are not enabled.</p>
    UserPoolAddOnNotEnabled(String),
}

impl SetRiskConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SetRiskConfigurationError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "CodeDeliveryFailureException" => {
                    return RusotoError::Service(SetRiskConfigurationError::CodeDeliveryFailure(
                        String::from(error_message),
                    ))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(SetRiskConfigurationError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidEmailRoleAccessPolicyException" => {
                    return RusotoError::Service(
                        SetRiskConfigurationError::InvalidEmailRoleAccessPolicy(String::from(
                            error_message,
                        )),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(SetRiskConfigurationError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(SetRiskConfigurationError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(SetRiskConfigurationError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(SetRiskConfigurationError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "UserPoolAddOnNotEnabledException" => {
                    return RusotoError::Service(
                        SetRiskConfigurationError::UserPoolAddOnNotEnabled(String::from(
                            error_message,
                        )),
                    )
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for SetRiskConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetRiskConfigurationError {
    fn description(&self) -> &str {
        match *self {
            SetRiskConfigurationError::CodeDeliveryFailure(ref cause) => cause,
            SetRiskConfigurationError::InternalError(ref cause) => cause,
            SetRiskConfigurationError::InvalidEmailRoleAccessPolicy(ref cause) => cause,
            SetRiskConfigurationError::InvalidParameter(ref cause) => cause,
            SetRiskConfigurationError::NotAuthorized(ref cause) => cause,
            SetRiskConfigurationError::ResourceNotFound(ref cause) => cause,
            SetRiskConfigurationError::TooManyRequests(ref cause) => cause,
            SetRiskConfigurationError::UserPoolAddOnNotEnabled(ref cause) => cause,
        }
    }
}
/// Errors returned by SetUICustomization
#[derive(Debug, PartialEq)]
pub enum SetUICustomizationError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
}

impl SetUICustomizationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SetUICustomizationError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(SetUICustomizationError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(SetUICustomizationError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(SetUICustomizationError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(SetUICustomizationError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(SetUICustomizationError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for SetUICustomizationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetUICustomizationError {
    fn description(&self) -> &str {
        match *self {
            SetUICustomizationError::InternalError(ref cause) => cause,
            SetUICustomizationError::InvalidParameter(ref cause) => cause,
            SetUICustomizationError::NotAuthorized(ref cause) => cause,
            SetUICustomizationError::ResourceNotFound(ref cause) => cause,
            SetUICustomizationError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by SetUserMFAPreference
#[derive(Debug, PartialEq)]
pub enum SetUserMFAPreferenceError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when a password reset is required.</p>
    PasswordResetRequired(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when a user is not confirmed successfully.</p>
    UserNotConfirmed(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
}

impl SetUserMFAPreferenceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SetUserMFAPreferenceError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(SetUserMFAPreferenceError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(SetUserMFAPreferenceError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(SetUserMFAPreferenceError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "PasswordResetRequiredException" => {
                    return RusotoError::Service(SetUserMFAPreferenceError::PasswordResetRequired(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(SetUserMFAPreferenceError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(SetUserMFAPreferenceError::UserNotConfirmed(
                        String::from(error_message),
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(SetUserMFAPreferenceError::UserNotFound(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for SetUserMFAPreferenceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetUserMFAPreferenceError {
    fn description(&self) -> &str {
        match *self {
            SetUserMFAPreferenceError::InternalError(ref cause) => cause,
            SetUserMFAPreferenceError::InvalidParameter(ref cause) => cause,
            SetUserMFAPreferenceError::NotAuthorized(ref cause) => cause,
            SetUserMFAPreferenceError::PasswordResetRequired(ref cause) => cause,
            SetUserMFAPreferenceError::ResourceNotFound(ref cause) => cause,
            SetUserMFAPreferenceError::UserNotConfirmed(ref cause) => cause,
            SetUserMFAPreferenceError::UserNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by SetUserPoolMfaConfig
#[derive(Debug, PartialEq)]
pub enum SetUserPoolMfaConfigError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is returned when the role provided for SMS configuration does not have permission to publish using Amazon SNS.</p>
    InvalidSmsRoleAccessPolicy(String),
    /// <p>This exception is thrown when the trust relationship is invalid for the role provided for SMS configuration. This can happen if you do not trust <b>cognito-idp.amazonaws.com</b> or the external ID provided in the role does not match what is provided in the SMS configuration for the user pool.</p>
    InvalidSmsRoleTrustRelationship(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
}

impl SetUserPoolMfaConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SetUserPoolMfaConfigError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(SetUserPoolMfaConfigError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(SetUserPoolMfaConfigError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "InvalidSmsRoleAccessPolicyException" => {
                    return RusotoError::Service(
                        SetUserPoolMfaConfigError::InvalidSmsRoleAccessPolicy(String::from(
                            error_message,
                        )),
                    )
                }
                "InvalidSmsRoleTrustRelationshipException" => {
                    return RusotoError::Service(
                        SetUserPoolMfaConfigError::InvalidSmsRoleTrustRelationship(String::from(
                            error_message,
                        )),
                    )
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(SetUserPoolMfaConfigError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(SetUserPoolMfaConfigError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(SetUserPoolMfaConfigError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for SetUserPoolMfaConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetUserPoolMfaConfigError {
    fn description(&self) -> &str {
        match *self {
            SetUserPoolMfaConfigError::InternalError(ref cause) => cause,
            SetUserPoolMfaConfigError::InvalidParameter(ref cause) => cause,
            SetUserPoolMfaConfigError::InvalidSmsRoleAccessPolicy(ref cause) => cause,
            SetUserPoolMfaConfigError::InvalidSmsRoleTrustRelationship(ref cause) => cause,
            SetUserPoolMfaConfigError::NotAuthorized(ref cause) => cause,
            SetUserPoolMfaConfigError::ResourceNotFound(ref cause) => cause,
            SetUserPoolMfaConfigError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by SetUserSettings
#[derive(Debug, PartialEq)]
pub enum SetUserSettingsError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when a password reset is required.</p>
    PasswordResetRequired(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when a user is not confirmed successfully.</p>
    UserNotConfirmed(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
}

impl SetUserSettingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SetUserSettingsError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(SetUserSettingsError::InternalError(String::from(
                        error_message,
                    )))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(SetUserSettingsError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(SetUserSettingsError::NotAuthorized(String::from(
                        error_message,
                    )))
                }
                "PasswordResetRequiredException" => {
                    return RusotoError::Service(SetUserSettingsError::PasswordResetRequired(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(SetUserSettingsError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(SetUserSettingsError::UserNotConfirmed(
                        String::from(error_message),
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(SetUserSettingsError::UserNotFound(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by SignUp
#[derive(Debug, PartialEq)]
pub enum SignUpError {
    /// <p>This exception is thrown when a verification code fails to deliver successfully.</p>
    CodeDeliveryFailure(String),
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when Amazon Cognito is not allowed to use your email identity. HTTP status code: 400.</p>
    InvalidEmailRoleAccessPolicy(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid AWS Lambda response.</p>
    InvalidLambdaResponse(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid password.</p>
    InvalidPassword(String),
    /// <p>This exception is returned when the role provided for SMS configuration does not have permission to publish using Amazon SNS.</p>
    InvalidSmsRoleAccessPolicy(String),
    /// <p>This exception is thrown when the trust relationship is invalid for the role provided for SMS configuration. This can happen if you do not trust <b>cognito-idp.amazonaws.com</b> or the external ID provided in the role does not match what is provided in the SMS configuration for the user pool.</p>
    InvalidSmsRoleTrustRelationship(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an unexpected exception with the AWS Lambda service.</p>
    UnexpectedLambda(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters a user validation exception with the AWS Lambda service.</p>
    UserLambdaValidation(String),
    /// <p>This exception is thrown when Amazon Cognito encounters a user name that already exists in the user pool.</p>
    UsernameExists(String),
}

impl SignUpError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SignUpError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "CodeDeliveryFailureException" => {
                    return RusotoError::Service(SignUpError::CodeDeliveryFailure(String::from(
                        error_message,
                    )))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(SignUpError::InternalError(String::from(
                        error_message,
                    )))
                }
                "InvalidEmailRoleAccessPolicyException" => {
                    return RusotoError::Service(SignUpError::InvalidEmailRoleAccessPolicy(
                        String::from(error_message),
                    ))
                }
                "InvalidLambdaResponseException" => {
                    return RusotoError::Service(SignUpError::InvalidLambdaResponse(String::from(
                        error_message,
                    )))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(SignUpError::InvalidParameter(String::from(
                        error_message,
                    )))
                }
                "InvalidPasswordException" => {
                    return RusotoError::Service(SignUpError::InvalidPassword(String::from(
                        error_message,
                    )))
                }
                "InvalidSmsRoleAccessPolicyException" => {
                    return RusotoError::Service(SignUpError::InvalidSmsRoleAccessPolicy(
                        String::from(error_message),
                    ))
                }
                "InvalidSmsRoleTrustRelationshipException" => {
                    return RusotoError::Service(SignUpError::InvalidSmsRoleTrustRelationship(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(SignUpError::NotAuthorized(String::from(
                        error_message,
                    )))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(SignUpError::ResourceNotFound(String::from(
                        error_message,
                    )))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(SignUpError::TooManyRequests(String::from(
                        error_message,
                    )))
                }
                "UnexpectedLambdaException" => {
                    return RusotoError::Service(SignUpError::UnexpectedLambda(String::from(
                        error_message,
                    )))
                }
                "UserLambdaValidationException" => {
                    return RusotoError::Service(SignUpError::UserLambdaValidation(String::from(
                        error_message,
                    )))
                }
                "UsernameExistsException" => {
                    return RusotoError::Service(SignUpError::UsernameExists(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by StartUserImportJob
#[derive(Debug, PartialEq)]
pub enum StartUserImportJobError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when a precondition is not met.</p>
    PreconditionNotMet(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
}

impl StartUserImportJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartUserImportJobError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(StartUserImportJobError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StartUserImportJobError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(StartUserImportJobError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "PreconditionNotMetException" => {
                    return RusotoError::Service(StartUserImportJobError::PreconditionNotMet(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StartUserImportJobError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(StartUserImportJobError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by StopUserImportJob
#[derive(Debug, PartialEq)]
pub enum StopUserImportJobError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when a precondition is not met.</p>
    PreconditionNotMet(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
}

impl StopUserImportJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopUserImportJobError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(StopUserImportJobError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StopUserImportJobError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(StopUserImportJobError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "PreconditionNotMetException" => {
                    return RusotoError::Service(StopUserImportJobError::PreconditionNotMet(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StopUserImportJobError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(StopUserImportJobError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by UpdateAuthEventFeedback
#[derive(Debug, PartialEq)]
pub enum UpdateAuthEventFeedbackError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
    /// <p>This exception is thrown when user pool add-ons are not enabled.</p>
    UserPoolAddOnNotEnabled(String),
}

impl UpdateAuthEventFeedbackError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateAuthEventFeedbackError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(UpdateAuthEventFeedbackError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateAuthEventFeedbackError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(UpdateAuthEventFeedbackError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateAuthEventFeedbackError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateAuthEventFeedbackError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(UpdateAuthEventFeedbackError::UserNotFound(
                        String::from(error_message),
                    ))
                }
                "UserPoolAddOnNotEnabledException" => {
                    return RusotoError::Service(
                        UpdateAuthEventFeedbackError::UserPoolAddOnNotEnabled(String::from(
                            error_message,
                        )),
                    )
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateAuthEventFeedbackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateAuthEventFeedbackError {
    fn description(&self) -> &str {
        match *self {
            UpdateAuthEventFeedbackError::InternalError(ref cause) => cause,
            UpdateAuthEventFeedbackError::InvalidParameter(ref cause) => cause,
            UpdateAuthEventFeedbackError::NotAuthorized(ref cause) => cause,
            UpdateAuthEventFeedbackError::ResourceNotFound(ref cause) => cause,
            UpdateAuthEventFeedbackError::TooManyRequests(ref cause) => cause,
            UpdateAuthEventFeedbackError::UserNotFound(ref cause) => cause,
            UpdateAuthEventFeedbackError::UserPoolAddOnNotEnabled(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateDeviceStatus
#[derive(Debug, PartialEq)]
pub enum UpdateDeviceStatusError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when the user pool configuration is invalid.</p>
    InvalidUserPoolConfiguration(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when a password reset is required.</p>
    PasswordResetRequired(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when a user is not confirmed successfully.</p>
    UserNotConfirmed(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
}

impl UpdateDeviceStatusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDeviceStatusError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(UpdateDeviceStatusError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateDeviceStatusError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "InvalidUserPoolConfigurationException" => {
                    return RusotoError::Service(
                        UpdateDeviceStatusError::InvalidUserPoolConfiguration(String::from(
                            error_message,
                        )),
                    )
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(UpdateDeviceStatusError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "PasswordResetRequiredException" => {
                    return RusotoError::Service(UpdateDeviceStatusError::PasswordResetRequired(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateDeviceStatusError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateDeviceStatusError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(UpdateDeviceStatusError::UserNotConfirmed(
                        String::from(error_message),
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(UpdateDeviceStatusError::UserNotFound(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by UpdateGroup
#[derive(Debug, PartialEq)]
pub enum UpdateGroupError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
}

impl UpdateGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateGroupError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(UpdateGroupError::InternalError(String::from(
                        error_message,
                    )))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateGroupError::InvalidParameter(String::from(
                        error_message,
                    )))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(UpdateGroupError::NotAuthorized(String::from(
                        error_message,
                    )))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateGroupError::ResourceNotFound(String::from(
                        error_message,
                    )))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateGroupError::TooManyRequests(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by UpdateIdentityProvider
#[derive(Debug, PartialEq)]
pub enum UpdateIdentityProviderError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when the specified identifier is not supported.</p>
    UnsupportedIdentityProvider(String),
}

impl UpdateIdentityProviderError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateIdentityProviderError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(UpdateIdentityProviderError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateIdentityProviderError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(UpdateIdentityProviderError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateIdentityProviderError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateIdentityProviderError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "UnsupportedIdentityProviderException" => {
                    return RusotoError::Service(
                        UpdateIdentityProviderError::UnsupportedIdentityProvider(String::from(
                            error_message,
                        )),
                    )
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by UpdateResourceServer
#[derive(Debug, PartialEq)]
pub enum UpdateResourceServerError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
}

impl UpdateResourceServerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateResourceServerError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(UpdateResourceServerError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateResourceServerError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(UpdateResourceServerError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateResourceServerError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateResourceServerError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateResourceServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateResourceServerError {
    fn description(&self) -> &str {
        match *self {
            UpdateResourceServerError::InternalError(ref cause) => cause,
            UpdateResourceServerError::InvalidParameter(ref cause) => cause,
            UpdateResourceServerError::NotAuthorized(ref cause) => cause,
            UpdateResourceServerError::ResourceNotFound(ref cause) => cause,
            UpdateResourceServerError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateUserAttributes
#[derive(Debug, PartialEq)]
pub enum UpdateUserAttributesError {
    /// <p>This exception is thrown when a user tries to confirm the account with an email or phone number that has already been supplied as an alias from a different account. This exception tells user that an account with this email or phone already exists.</p>
    AliasExists(String),
    /// <p>This exception is thrown when a verification code fails to deliver successfully.</p>
    CodeDeliveryFailure(String),
    /// <p>This exception is thrown if the provided code does not match what the server was expecting.</p>
    CodeMismatch(String),
    /// <p>This exception is thrown if a code has expired.</p>
    ExpiredCode(String),
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when Amazon Cognito is not allowed to use your email identity. HTTP status code: 400.</p>
    InvalidEmailRoleAccessPolicy(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid AWS Lambda response.</p>
    InvalidLambdaResponse(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is returned when the role provided for SMS configuration does not have permission to publish using Amazon SNS.</p>
    InvalidSmsRoleAccessPolicy(String),
    /// <p>This exception is thrown when the trust relationship is invalid for the role provided for SMS configuration. This can happen if you do not trust <b>cognito-idp.amazonaws.com</b> or the external ID provided in the role does not match what is provided in the SMS configuration for the user pool.</p>
    InvalidSmsRoleTrustRelationship(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when a password reset is required.</p>
    PasswordResetRequired(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an unexpected exception with the AWS Lambda service.</p>
    UnexpectedLambda(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters a user validation exception with the AWS Lambda service.</p>
    UserLambdaValidation(String),
    /// <p>This exception is thrown when a user is not confirmed successfully.</p>
    UserNotConfirmed(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
}

impl UpdateUserAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateUserAttributesError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AliasExistsException" => {
                    return RusotoError::Service(UpdateUserAttributesError::AliasExists(
                        String::from(error_message),
                    ))
                }
                "CodeDeliveryFailureException" => {
                    return RusotoError::Service(UpdateUserAttributesError::CodeDeliveryFailure(
                        String::from(error_message),
                    ))
                }
                "CodeMismatchException" => {
                    return RusotoError::Service(UpdateUserAttributesError::CodeMismatch(
                        String::from(error_message),
                    ))
                }
                "ExpiredCodeException" => {
                    return RusotoError::Service(UpdateUserAttributesError::ExpiredCode(
                        String::from(error_message),
                    ))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(UpdateUserAttributesError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidEmailRoleAccessPolicyException" => {
                    return RusotoError::Service(
                        UpdateUserAttributesError::InvalidEmailRoleAccessPolicy(String::from(
                            error_message,
                        )),
                    )
                }
                "InvalidLambdaResponseException" => {
                    return RusotoError::Service(UpdateUserAttributesError::InvalidLambdaResponse(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateUserAttributesError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "InvalidSmsRoleAccessPolicyException" => {
                    return RusotoError::Service(
                        UpdateUserAttributesError::InvalidSmsRoleAccessPolicy(String::from(
                            error_message,
                        )),
                    )
                }
                "InvalidSmsRoleTrustRelationshipException" => {
                    return RusotoError::Service(
                        UpdateUserAttributesError::InvalidSmsRoleTrustRelationship(String::from(
                            error_message,
                        )),
                    )
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(UpdateUserAttributesError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "PasswordResetRequiredException" => {
                    return RusotoError::Service(UpdateUserAttributesError::PasswordResetRequired(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateUserAttributesError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateUserAttributesError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "UnexpectedLambdaException" => {
                    return RusotoError::Service(UpdateUserAttributesError::UnexpectedLambda(
                        String::from(error_message),
                    ))
                }
                "UserLambdaValidationException" => {
                    return RusotoError::Service(UpdateUserAttributesError::UserLambdaValidation(
                        String::from(error_message),
                    ))
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(UpdateUserAttributesError::UserNotConfirmed(
                        String::from(error_message),
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(UpdateUserAttributesError::UserNotFound(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by UpdateUserPool
#[derive(Debug, PartialEq)]
pub enum UpdateUserPoolError {
    /// <p>This exception is thrown if two or more modifications are happening concurrently.</p>
    ConcurrentModification(String),
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when Amazon Cognito is not allowed to use your email identity. HTTP status code: 400.</p>
    InvalidEmailRoleAccessPolicy(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is returned when the role provided for SMS configuration does not have permission to publish using Amazon SNS.</p>
    InvalidSmsRoleAccessPolicy(String),
    /// <p>This exception is thrown when the trust relationship is invalid for the role provided for SMS configuration. This can happen if you do not trust <b>cognito-idp.amazonaws.com</b> or the external ID provided in the role does not match what is provided in the SMS configuration for the user pool.</p>
    InvalidSmsRoleTrustRelationship(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when you are trying to modify a user pool while a user import job is in progress for that pool.</p>
    UserImportInProgress(String),
    /// <p>This exception is thrown when a user pool tag cannot be set or updated.</p>
    UserPoolTagging(String),
}

impl UpdateUserPoolError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateUserPoolError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UpdateUserPoolError::ConcurrentModification(
                        String::from(error_message),
                    ))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(UpdateUserPoolError::InternalError(String::from(
                        error_message,
                    )))
                }
                "InvalidEmailRoleAccessPolicyException" => {
                    return RusotoError::Service(UpdateUserPoolError::InvalidEmailRoleAccessPolicy(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateUserPoolError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "InvalidSmsRoleAccessPolicyException" => {
                    return RusotoError::Service(UpdateUserPoolError::InvalidSmsRoleAccessPolicy(
                        String::from(error_message),
                    ))
                }
                "InvalidSmsRoleTrustRelationshipException" => {
                    return RusotoError::Service(
                        UpdateUserPoolError::InvalidSmsRoleTrustRelationship(String::from(
                            error_message,
                        )),
                    )
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(UpdateUserPoolError::NotAuthorized(String::from(
                        error_message,
                    )))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateUserPoolError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateUserPoolError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "UserImportInProgressException" => {
                    return RusotoError::Service(UpdateUserPoolError::UserImportInProgress(
                        String::from(error_message),
                    ))
                }
                "UserPoolTaggingException" => {
                    return RusotoError::Service(UpdateUserPoolError::UserPoolTagging(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by UpdateUserPoolClient
#[derive(Debug, PartialEq)]
pub enum UpdateUserPoolClientError {
    /// <p>This exception is thrown if two or more modifications are happening concurrently.</p>
    ConcurrentModification(String),
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the specified OAuth flow is invalid.</p>
    InvalidOAuthFlow(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the specified scope does not exist.</p>
    ScopeDoesNotExist(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
}

impl UpdateUserPoolClientError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateUserPoolClientError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UpdateUserPoolClientError::ConcurrentModification(
                        String::from(error_message),
                    ))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(UpdateUserPoolClientError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidOAuthFlowException" => {
                    return RusotoError::Service(UpdateUserPoolClientError::InvalidOAuthFlow(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateUserPoolClientError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(UpdateUserPoolClientError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateUserPoolClientError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "ScopeDoesNotExistException" => {
                    return RusotoError::Service(UpdateUserPoolClientError::ScopeDoesNotExist(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateUserPoolClientError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            UpdateUserPoolClientError::ConcurrentModification(ref cause) => cause,
            UpdateUserPoolClientError::InternalError(ref cause) => cause,
            UpdateUserPoolClientError::InvalidOAuthFlow(ref cause) => cause,
            UpdateUserPoolClientError::InvalidParameter(ref cause) => cause,
            UpdateUserPoolClientError::NotAuthorized(ref cause) => cause,
            UpdateUserPoolClientError::ResourceNotFound(ref cause) => cause,
            UpdateUserPoolClientError::ScopeDoesNotExist(ref cause) => cause,
            UpdateUserPoolClientError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateUserPoolDomain
#[derive(Debug, PartialEq)]
pub enum UpdateUserPoolDomainError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
}

impl UpdateUserPoolDomainError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateUserPoolDomainError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(UpdateUserPoolDomainError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateUserPoolDomainError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(UpdateUserPoolDomainError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateUserPoolDomainError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateUserPoolDomainError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateUserPoolDomainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateUserPoolDomainError {
    fn description(&self) -> &str {
        match *self {
            UpdateUserPoolDomainError::InternalError(ref cause) => cause,
            UpdateUserPoolDomainError::InvalidParameter(ref cause) => cause,
            UpdateUserPoolDomainError::NotAuthorized(ref cause) => cause,
            UpdateUserPoolDomainError::ResourceNotFound(ref cause) => cause,
            UpdateUserPoolDomainError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by VerifySoftwareToken
#[derive(Debug, PartialEq)]
pub enum VerifySoftwareTokenError {
    /// <p>This exception is thrown if the provided code does not match what the server was expecting.</p>
    CodeMismatch(String),
    /// <p>This exception is thrown when there is a code mismatch and the service fails to configure the software token TOTP multi-factor authentication (MFA).</p>
    EnableSoftwareTokenMFA(String),
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when the user pool configuration is invalid.</p>
    InvalidUserPoolConfiguration(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when a password reset is required.</p>
    PasswordResetRequired(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the software token TOTP multi-factor authentication (MFA) is not enabled for the user pool.</p>
    SoftwareTokenMFANotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when a user is not confirmed successfully.</p>
    UserNotConfirmed(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
}

impl VerifySoftwareTokenError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<VerifySoftwareTokenError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "CodeMismatchException" => {
                    return RusotoError::Service(VerifySoftwareTokenError::CodeMismatch(
                        String::from(error_message),
                    ))
                }
                "EnableSoftwareTokenMFAException" => {
                    return RusotoError::Service(VerifySoftwareTokenError::EnableSoftwareTokenMFA(
                        String::from(error_message),
                    ))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(VerifySoftwareTokenError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(VerifySoftwareTokenError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "InvalidUserPoolConfigurationException" => {
                    return RusotoError::Service(
                        VerifySoftwareTokenError::InvalidUserPoolConfiguration(String::from(
                            error_message,
                        )),
                    )
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(VerifySoftwareTokenError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "PasswordResetRequiredException" => {
                    return RusotoError::Service(VerifySoftwareTokenError::PasswordResetRequired(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(VerifySoftwareTokenError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "SoftwareTokenMFANotFoundException" => {
                    return RusotoError::Service(
                        VerifySoftwareTokenError::SoftwareTokenMFANotFound(String::from(
                            error_message,
                        )),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(VerifySoftwareTokenError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(VerifySoftwareTokenError::UserNotConfirmed(
                        String::from(error_message),
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(VerifySoftwareTokenError::UserNotFound(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for VerifySoftwareTokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for VerifySoftwareTokenError {
    fn description(&self) -> &str {
        match *self {
            VerifySoftwareTokenError::CodeMismatch(ref cause) => cause,
            VerifySoftwareTokenError::EnableSoftwareTokenMFA(ref cause) => cause,
            VerifySoftwareTokenError::InternalError(ref cause) => cause,
            VerifySoftwareTokenError::InvalidParameter(ref cause) => cause,
            VerifySoftwareTokenError::InvalidUserPoolConfiguration(ref cause) => cause,
            VerifySoftwareTokenError::NotAuthorized(ref cause) => cause,
            VerifySoftwareTokenError::PasswordResetRequired(ref cause) => cause,
            VerifySoftwareTokenError::ResourceNotFound(ref cause) => cause,
            VerifySoftwareTokenError::SoftwareTokenMFANotFound(ref cause) => cause,
            VerifySoftwareTokenError::TooManyRequests(ref cause) => cause,
            VerifySoftwareTokenError::UserNotConfirmed(ref cause) => cause,
            VerifySoftwareTokenError::UserNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by VerifyUserAttribute
#[derive(Debug, PartialEq)]
pub enum VerifyUserAttributeError {
    /// <p>This exception is thrown if the provided code does not match what the server was expecting.</p>
    CodeMismatch(String),
    /// <p>This exception is thrown if a code has expired.</p>
    ExpiredCode(String),
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when a user exceeds the limit for a requested AWS resource.</p>
    LimitExceeded(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when a password reset is required.</p>
    PasswordResetRequired(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when a user is not confirmed successfully.</p>
    UserNotConfirmed(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
}

impl VerifyUserAttributeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<VerifyUserAttributeError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "CodeMismatchException" => {
                    return RusotoError::Service(VerifyUserAttributeError::CodeMismatch(
                        String::from(error_message),
                    ))
                }
                "ExpiredCodeException" => {
                    return RusotoError::Service(VerifyUserAttributeError::ExpiredCode(
                        String::from(error_message),
                    ))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(VerifyUserAttributeError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(VerifyUserAttributeError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(VerifyUserAttributeError::LimitExceeded(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(VerifyUserAttributeError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "PasswordResetRequiredException" => {
                    return RusotoError::Service(VerifyUserAttributeError::PasswordResetRequired(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(VerifyUserAttributeError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(VerifyUserAttributeError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(VerifyUserAttributeError::UserNotConfirmed(
                        String::from(error_message),
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(VerifyUserAttributeError::UserNotFound(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Trait representing the capabilities of the Amazon Cognito Identity Provider API. Amazon Cognito Identity Provider clients implement this trait.
pub trait CognitoIdentityProvider {
    /// <p>Adds additional user attributes to the user pool schema.</p>
    fn add_custom_attributes(
        &self,
        input: AddCustomAttributesRequest,
    ) -> RusotoFuture<AddCustomAttributesResponse, AddCustomAttributesError>;

    /// <p>Adds the specified user to the specified group.</p> <p>Requires developer credentials.</p>
    fn admin_add_user_to_group(
        &self,
        input: AdminAddUserToGroupRequest,
    ) -> RusotoFuture<(), AdminAddUserToGroupError>;

    /// <p>Confirms user registration as an admin without using a confirmation code. Works on any user.</p> <p>Requires developer credentials.</p>
    fn admin_confirm_sign_up(
        &self,
        input: AdminConfirmSignUpRequest,
    ) -> RusotoFuture<AdminConfirmSignUpResponse, AdminConfirmSignUpError>;

    /// <p>Creates a new user in the specified user pool.</p> <p>If <code>MessageAction</code> is not set, the default is to send a welcome message via email or phone (SMS).</p> <note> <p>This message is based on a template that you configured in your call to or . This template includes your custom sign-up instructions and placeholders for user name and temporary password.</p> </note> <p>Alternatively, you can call AdminCreateUser with “SUPPRESS” for the <code>MessageAction</code> parameter, and Amazon Cognito will not send any email. </p> <p>In either case, the user will be in the <code>FORCE_CHANGE_PASSWORD</code> state until they sign in and change their password.</p> <p>AdminCreateUser requires developer credentials.</p>
    fn admin_create_user(
        &self,
        input: AdminCreateUserRequest,
    ) -> RusotoFuture<AdminCreateUserResponse, AdminCreateUserError>;

    /// <p>Deletes a user as an administrator. Works on any user.</p> <p>Requires developer credentials.</p>
    fn admin_delete_user(
        &self,
        input: AdminDeleteUserRequest,
    ) -> RusotoFuture<(), AdminDeleteUserError>;

    /// <p>Deletes the user attributes in a user pool as an administrator. Works on any user.</p> <p>Requires developer credentials.</p>
    fn admin_delete_user_attributes(
        &self,
        input: AdminDeleteUserAttributesRequest,
    ) -> RusotoFuture<AdminDeleteUserAttributesResponse, AdminDeleteUserAttributesError>;

    /// <p>Disables the user from signing in with the specified external (SAML or social) identity provider. If the user to disable is a Cognito User Pools native username + password user, they are not permitted to use their password to sign-in. If the user to disable is a linked external IdP user, any link between that user and an existing user is removed. The next time the external user (no longer attached to the previously linked <code>DestinationUser</code>) signs in, they must create a new user account. See .</p> <p>This action is enabled only for admin access and requires developer credentials.</p> <p>The <code>ProviderName</code> must match the value specified when creating an IdP for the pool. </p> <p>To disable a native username + password user, the <code>ProviderName</code> value must be <code>Cognito</code> and the <code>ProviderAttributeName</code> must be <code>Cognito_Subject</code>, with the <code>ProviderAttributeValue</code> being the name that is used in the user pool for the user.</p> <p>The <code>ProviderAttributeName</code> must always be <code>Cognito_Subject</code> for social identity providers. The <code>ProviderAttributeValue</code> must always be the exact subject that was used when the user was originally linked as a source user.</p> <p>For de-linking a SAML identity, there are two scenarios. If the linked identity has not yet been used to sign-in, the <code>ProviderAttributeName</code> and <code>ProviderAttributeValue</code> must be the same values that were used for the <code>SourceUser</code> when the identities were originally linked in the call. (If the linking was done with <code>ProviderAttributeName</code> set to <code>Cognito_Subject</code>, the same applies here). However, if the user has already signed in, the <code>ProviderAttributeName</code> must be <code>Cognito_Subject</code> and <code>ProviderAttributeValue</code> must be the subject of the SAML assertion.</p>
    fn admin_disable_provider_for_user(
        &self,
        input: AdminDisableProviderForUserRequest,
    ) -> RusotoFuture<AdminDisableProviderForUserResponse, AdminDisableProviderForUserError>;

    /// <p>Disables the specified user as an administrator. Works on any user.</p> <p>Requires developer credentials.</p>
    fn admin_disable_user(
        &self,
        input: AdminDisableUserRequest,
    ) -> RusotoFuture<AdminDisableUserResponse, AdminDisableUserError>;

    /// <p>Enables the specified user as an administrator. Works on any user.</p> <p>Requires developer credentials.</p>
    fn admin_enable_user(
        &self,
        input: AdminEnableUserRequest,
    ) -> RusotoFuture<AdminEnableUserResponse, AdminEnableUserError>;

    /// <p>Forgets the device, as an administrator.</p> <p>Requires developer credentials.</p>
    fn admin_forget_device(
        &self,
        input: AdminForgetDeviceRequest,
    ) -> RusotoFuture<(), AdminForgetDeviceError>;

    /// <p>Gets the device, as an administrator.</p> <p>Requires developer credentials.</p>
    fn admin_get_device(
        &self,
        input: AdminGetDeviceRequest,
    ) -> RusotoFuture<AdminGetDeviceResponse, AdminGetDeviceError>;

    /// <p>Gets the specified user by user name in a user pool as an administrator. Works on any user.</p> <p>Requires developer credentials.</p>
    fn admin_get_user(
        &self,
        input: AdminGetUserRequest,
    ) -> RusotoFuture<AdminGetUserResponse, AdminGetUserError>;

    /// <p>Initiates the authentication flow, as an administrator.</p> <p>Requires developer credentials.</p>
    fn admin_initiate_auth(
        &self,
        input: AdminInitiateAuthRequest,
    ) -> RusotoFuture<AdminInitiateAuthResponse, AdminInitiateAuthError>;

    /// <p>Links an existing user account in a user pool (<code>DestinationUser</code>) to an identity from an external identity provider (<code>SourceUser</code>) based on a specified attribute name and value from the external identity provider. This allows you to create a link from the existing user account to an external federated user identity that has not yet been used to sign in, so that the federated user identity can be used to sign in as the existing user account. </p> <p> For example, if there is an existing user with a username and password, this API links that user to a federated user identity, so that when the federated user identity is used, the user signs in as the existing user account. </p> <important> <p>Because this API allows a user with an external federated identity to sign in as an existing user in the user pool, it is critical that it only be used with external identity providers and provider attributes that have been trusted by the application owner.</p> </important> <p>See also .</p> <p>This action is enabled only for admin access and requires developer credentials.</p>
    fn admin_link_provider_for_user(
        &self,
        input: AdminLinkProviderForUserRequest,
    ) -> RusotoFuture<AdminLinkProviderForUserResponse, AdminLinkProviderForUserError>;

    /// <p>Lists devices, as an administrator.</p> <p>Requires developer credentials.</p>
    fn admin_list_devices(
        &self,
        input: AdminListDevicesRequest,
    ) -> RusotoFuture<AdminListDevicesResponse, AdminListDevicesError>;

    /// <p>Lists the groups that the user belongs to.</p> <p>Requires developer credentials.</p>
    fn admin_list_groups_for_user(
        &self,
        input: AdminListGroupsForUserRequest,
    ) -> RusotoFuture<AdminListGroupsForUserResponse, AdminListGroupsForUserError>;

    /// <p>Lists a history of user activity and any risks detected as part of Amazon Cognito advanced security.</p>
    fn admin_list_user_auth_events(
        &self,
        input: AdminListUserAuthEventsRequest,
    ) -> RusotoFuture<AdminListUserAuthEventsResponse, AdminListUserAuthEventsError>;

    /// <p>Removes the specified user from the specified group.</p> <p>Requires developer credentials.</p>
    fn admin_remove_user_from_group(
        &self,
        input: AdminRemoveUserFromGroupRequest,
    ) -> RusotoFuture<(), AdminRemoveUserFromGroupError>;

    /// <p>Resets the specified user's password in a user pool as an administrator. Works on any user.</p> <p>When a developer calls this API, the current password is invalidated, so it must be changed. If a user tries to sign in after the API is called, the app will get a PasswordResetRequiredException exception back and should direct the user down the flow to reset the password, which is the same as the forgot password flow. In addition, if the user pool has phone verification selected and a verified phone number exists for the user, or if email verification is selected and a verified email exists for the user, calling this API will also result in sending a message to the end user with the code to change their password.</p> <p>Requires developer credentials.</p>
    fn admin_reset_user_password(
        &self,
        input: AdminResetUserPasswordRequest,
    ) -> RusotoFuture<AdminResetUserPasswordResponse, AdminResetUserPasswordError>;

    /// <p>Responds to an authentication challenge, as an administrator.</p> <p>Requires developer credentials.</p>
    fn admin_respond_to_auth_challenge(
        &self,
        input: AdminRespondToAuthChallengeRequest,
    ) -> RusotoFuture<AdminRespondToAuthChallengeResponse, AdminRespondToAuthChallengeError>;

    /// <p>Sets the user's multi-factor authentication (MFA) preference.</p>
    fn admin_set_user_mfa_preference(
        &self,
        input: AdminSetUserMFAPreferenceRequest,
    ) -> RusotoFuture<AdminSetUserMFAPreferenceResponse, AdminSetUserMFAPreferenceError>;

    /// <p>Sets all the user settings for a specified user name. Works on any user.</p> <p>Requires developer credentials.</p>
    fn admin_set_user_settings(
        &self,
        input: AdminSetUserSettingsRequest,
    ) -> RusotoFuture<AdminSetUserSettingsResponse, AdminSetUserSettingsError>;

    /// <p>Provides feedback for an authentication event as to whether it was from a valid user. This feedback is used for improving the risk evaluation decision for the user pool as part of Amazon Cognito advanced security.</p>
    fn admin_update_auth_event_feedback(
        &self,
        input: AdminUpdateAuthEventFeedbackRequest,
    ) -> RusotoFuture<AdminUpdateAuthEventFeedbackResponse, AdminUpdateAuthEventFeedbackError>;

    /// <p>Updates the device status as an administrator.</p> <p>Requires developer credentials.</p>
    fn admin_update_device_status(
        &self,
        input: AdminUpdateDeviceStatusRequest,
    ) -> RusotoFuture<AdminUpdateDeviceStatusResponse, AdminUpdateDeviceStatusError>;

    /// <p>Updates the specified user's attributes, including developer attributes, as an administrator. Works on any user.</p> <p>For custom attributes, you must prepend the <code>custom:</code> prefix to the attribute name.</p> <p>In addition to updating user attributes, this API can also be used to mark phone and email as verified.</p> <p>Requires developer credentials.</p>
    fn admin_update_user_attributes(
        &self,
        input: AdminUpdateUserAttributesRequest,
    ) -> RusotoFuture<AdminUpdateUserAttributesResponse, AdminUpdateUserAttributesError>;

    /// <p>Signs out users from all devices, as an administrator.</p> <p>Requires developer credentials.</p>
    fn admin_user_global_sign_out(
        &self,
        input: AdminUserGlobalSignOutRequest,
    ) -> RusotoFuture<AdminUserGlobalSignOutResponse, AdminUserGlobalSignOutError>;

    /// <p>Returns a unique generated shared secret key code for the user account. The request takes an access token or a session string, but not both.</p>
    fn associate_software_token(
        &self,
        input: AssociateSoftwareTokenRequest,
    ) -> RusotoFuture<AssociateSoftwareTokenResponse, AssociateSoftwareTokenError>;

    /// <p>Changes the password for a specified user in a user pool.</p>
    fn change_password(
        &self,
        input: ChangePasswordRequest,
    ) -> RusotoFuture<ChangePasswordResponse, ChangePasswordError>;

    /// <p>Confirms tracking of the device. This API call is the call that begins device tracking.</p>
    fn confirm_device(
        &self,
        input: ConfirmDeviceRequest,
    ) -> RusotoFuture<ConfirmDeviceResponse, ConfirmDeviceError>;

    /// <p>Allows a user to enter a confirmation code to reset a forgotten password.</p>
    fn confirm_forgot_password(
        &self,
        input: ConfirmForgotPasswordRequest,
    ) -> RusotoFuture<ConfirmForgotPasswordResponse, ConfirmForgotPasswordError>;

    /// <p>Confirms registration of a user and handles the existing alias from a previous user.</p>
    fn confirm_sign_up(
        &self,
        input: ConfirmSignUpRequest,
    ) -> RusotoFuture<ConfirmSignUpResponse, ConfirmSignUpError>;

    /// <p>Creates a new group in the specified user pool.</p> <p>Requires developer credentials.</p>
    fn create_group(
        &self,
        input: CreateGroupRequest,
    ) -> RusotoFuture<CreateGroupResponse, CreateGroupError>;

    /// <p>Creates an identity provider for a user pool.</p>
    fn create_identity_provider(
        &self,
        input: CreateIdentityProviderRequest,
    ) -> RusotoFuture<CreateIdentityProviderResponse, CreateIdentityProviderError>;

    /// <p>Creates a new OAuth2.0 resource server and defines custom scopes in it.</p>
    fn create_resource_server(
        &self,
        input: CreateResourceServerRequest,
    ) -> RusotoFuture<CreateResourceServerResponse, CreateResourceServerError>;

    /// <p>Creates the user import job.</p>
    fn create_user_import_job(
        &self,
        input: CreateUserImportJobRequest,
    ) -> RusotoFuture<CreateUserImportJobResponse, CreateUserImportJobError>;

    /// <p>Creates a new Amazon Cognito user pool and sets the password policy for the pool.</p>
    fn create_user_pool(
        &self,
        input: CreateUserPoolRequest,
    ) -> RusotoFuture<CreateUserPoolResponse, CreateUserPoolError>;

    /// <p>Creates the user pool client.</p>
    fn create_user_pool_client(
        &self,
        input: CreateUserPoolClientRequest,
    ) -> RusotoFuture<CreateUserPoolClientResponse, CreateUserPoolClientError>;

    /// <p>Creates a new domain for a user pool.</p>
    fn create_user_pool_domain(
        &self,
        input: CreateUserPoolDomainRequest,
    ) -> RusotoFuture<CreateUserPoolDomainResponse, CreateUserPoolDomainError>;

    /// <p>Deletes a group. Currently only groups with no members can be deleted.</p> <p>Requires developer credentials.</p>
    fn delete_group(&self, input: DeleteGroupRequest) -> RusotoFuture<(), DeleteGroupError>;

    /// <p>Deletes an identity provider for a user pool.</p>
    fn delete_identity_provider(
        &self,
        input: DeleteIdentityProviderRequest,
    ) -> RusotoFuture<(), DeleteIdentityProviderError>;

    /// <p>Deletes a resource server.</p>
    fn delete_resource_server(
        &self,
        input: DeleteResourceServerRequest,
    ) -> RusotoFuture<(), DeleteResourceServerError>;

    /// <p>Allows a user to delete himself or herself.</p>
    fn delete_user(&self, input: DeleteUserRequest) -> RusotoFuture<(), DeleteUserError>;

    /// <p>Deletes the attributes for a user.</p>
    fn delete_user_attributes(
        &self,
        input: DeleteUserAttributesRequest,
    ) -> RusotoFuture<DeleteUserAttributesResponse, DeleteUserAttributesError>;

    /// <p>Deletes the specified Amazon Cognito user pool.</p>
    fn delete_user_pool(
        &self,
        input: DeleteUserPoolRequest,
    ) -> RusotoFuture<(), DeleteUserPoolError>;

    /// <p>Allows the developer to delete the user pool client.</p>
    fn delete_user_pool_client(
        &self,
        input: DeleteUserPoolClientRequest,
    ) -> RusotoFuture<(), DeleteUserPoolClientError>;

    /// <p>Deletes a domain for a user pool.</p>
    fn delete_user_pool_domain(
        &self,
        input: DeleteUserPoolDomainRequest,
    ) -> RusotoFuture<DeleteUserPoolDomainResponse, DeleteUserPoolDomainError>;

    /// <p>Gets information about a specific identity provider.</p>
    fn describe_identity_provider(
        &self,
        input: DescribeIdentityProviderRequest,
    ) -> RusotoFuture<DescribeIdentityProviderResponse, DescribeIdentityProviderError>;

    /// <p>Describes a resource server.</p>
    fn describe_resource_server(
        &self,
        input: DescribeResourceServerRequest,
    ) -> RusotoFuture<DescribeResourceServerResponse, DescribeResourceServerError>;

    /// <p>Describes the risk configuration.</p>
    fn describe_risk_configuration(
        &self,
        input: DescribeRiskConfigurationRequest,
    ) -> RusotoFuture<DescribeRiskConfigurationResponse, DescribeRiskConfigurationError>;

    /// <p>Describes the user import job.</p>
    fn describe_user_import_job(
        &self,
        input: DescribeUserImportJobRequest,
    ) -> RusotoFuture<DescribeUserImportJobResponse, DescribeUserImportJobError>;

    /// <p>Returns the configuration information and metadata of the specified user pool.</p>
    fn describe_user_pool(
        &self,
        input: DescribeUserPoolRequest,
    ) -> RusotoFuture<DescribeUserPoolResponse, DescribeUserPoolError>;

    /// <p>Client method for returning the configuration information and metadata of the specified user pool app client.</p>
    fn describe_user_pool_client(
        &self,
        input: DescribeUserPoolClientRequest,
    ) -> RusotoFuture<DescribeUserPoolClientResponse, DescribeUserPoolClientError>;

    /// <p>Gets information about a domain.</p>
    fn describe_user_pool_domain(
        &self,
        input: DescribeUserPoolDomainRequest,
    ) -> RusotoFuture<DescribeUserPoolDomainResponse, DescribeUserPoolDomainError>;

    /// <p>Forgets the specified device.</p>
    fn forget_device(&self, input: ForgetDeviceRequest) -> RusotoFuture<(), ForgetDeviceError>;

    /// <p>Calling this API causes a message to be sent to the end user with a confirmation code that is required to change the user's password. For the <code>Username</code> parameter, you can use the username or user alias. If a verified phone number exists for the user, the confirmation code is sent to the phone number. Otherwise, if a verified email exists, the confirmation code is sent to the email. If neither a verified phone number nor a verified email exists, <code>InvalidParameterException</code> is thrown. To use the confirmation code for resetting the password, call .</p>
    fn forgot_password(
        &self,
        input: ForgotPasswordRequest,
    ) -> RusotoFuture<ForgotPasswordResponse, ForgotPasswordError>;

    /// <p>Gets the header information for the .csv file to be used as input for the user import job.</p>
    fn get_csv_header(
        &self,
        input: GetCSVHeaderRequest,
    ) -> RusotoFuture<GetCSVHeaderResponse, GetCSVHeaderError>;

    /// <p>Gets the device.</p>
    fn get_device(
        &self,
        input: GetDeviceRequest,
    ) -> RusotoFuture<GetDeviceResponse, GetDeviceError>;

    /// <p>Gets a group.</p> <p>Requires developer credentials.</p>
    fn get_group(&self, input: GetGroupRequest) -> RusotoFuture<GetGroupResponse, GetGroupError>;

    /// <p>Gets the specified identity provider.</p>
    fn get_identity_provider_by_identifier(
        &self,
        input: GetIdentityProviderByIdentifierRequest,
    ) -> RusotoFuture<GetIdentityProviderByIdentifierResponse, GetIdentityProviderByIdentifierError>;

    /// <p>This method takes a user pool ID, and returns the signing certificate.</p>
    fn get_signing_certificate(
        &self,
        input: GetSigningCertificateRequest,
    ) -> RusotoFuture<GetSigningCertificateResponse, GetSigningCertificateError>;

    /// <p>Gets the UI Customization information for a particular app client's app UI, if there is something set. If nothing is set for the particular client, but there is an existing pool level customization (app <code>clientId</code> will be <code>ALL</code>), then that is returned. If nothing is present, then an empty shape is returned.</p>
    fn get_ui_customization(
        &self,
        input: GetUICustomizationRequest,
    ) -> RusotoFuture<GetUICustomizationResponse, GetUICustomizationError>;

    /// <p>Gets the user attributes and metadata for a user.</p>
    fn get_user(&self, input: GetUserRequest) -> RusotoFuture<GetUserResponse, GetUserError>;

    /// <p>Gets the user attribute verification code for the specified attribute name.</p>
    fn get_user_attribute_verification_code(
        &self,
        input: GetUserAttributeVerificationCodeRequest,
    ) -> RusotoFuture<GetUserAttributeVerificationCodeResponse, GetUserAttributeVerificationCodeError>;

    /// <p>Gets the user pool multi-factor authentication (MFA) configuration.</p>
    fn get_user_pool_mfa_config(
        &self,
        input: GetUserPoolMfaConfigRequest,
    ) -> RusotoFuture<GetUserPoolMfaConfigResponse, GetUserPoolMfaConfigError>;

    /// <p>Signs out users from all devices.</p>
    fn global_sign_out(
        &self,
        input: GlobalSignOutRequest,
    ) -> RusotoFuture<GlobalSignOutResponse, GlobalSignOutError>;

    /// <p>Initiates the authentication flow.</p>
    fn initiate_auth(
        &self,
        input: InitiateAuthRequest,
    ) -> RusotoFuture<InitiateAuthResponse, InitiateAuthError>;

    /// <p>Lists the devices.</p>
    fn list_devices(
        &self,
        input: ListDevicesRequest,
    ) -> RusotoFuture<ListDevicesResponse, ListDevicesError>;

    /// <p>Lists the groups associated with a user pool.</p> <p>Requires developer credentials.</p>
    fn list_groups(
        &self,
        input: ListGroupsRequest,
    ) -> RusotoFuture<ListGroupsResponse, ListGroupsError>;

    /// <p>Lists information about all identity providers for a user pool.</p>
    fn list_identity_providers(
        &self,
        input: ListIdentityProvidersRequest,
    ) -> RusotoFuture<ListIdentityProvidersResponse, ListIdentityProvidersError>;

    /// <p>Lists the resource servers for a user pool.</p>
    fn list_resource_servers(
        &self,
        input: ListResourceServersRequest,
    ) -> RusotoFuture<ListResourceServersResponse, ListResourceServersError>;

    /// <p>Lists the user import jobs.</p>
    fn list_user_import_jobs(
        &self,
        input: ListUserImportJobsRequest,
    ) -> RusotoFuture<ListUserImportJobsResponse, ListUserImportJobsError>;

    /// <p>Lists the clients that have been created for the specified user pool.</p>
    fn list_user_pool_clients(
        &self,
        input: ListUserPoolClientsRequest,
    ) -> RusotoFuture<ListUserPoolClientsResponse, ListUserPoolClientsError>;

    /// <p>Lists the user pools associated with an AWS account.</p>
    fn list_user_pools(
        &self,
        input: ListUserPoolsRequest,
    ) -> RusotoFuture<ListUserPoolsResponse, ListUserPoolsError>;

    /// <p>Lists the users in the Amazon Cognito user pool.</p>
    fn list_users(
        &self,
        input: ListUsersRequest,
    ) -> RusotoFuture<ListUsersResponse, ListUsersError>;

    /// <p>Lists the users in the specified group.</p> <p>Requires developer credentials.</p>
    fn list_users_in_group(
        &self,
        input: ListUsersInGroupRequest,
    ) -> RusotoFuture<ListUsersInGroupResponse, ListUsersInGroupError>;

    /// <p>Resends the confirmation (for confirmation of registration) to a specific user in the user pool.</p>
    fn resend_confirmation_code(
        &self,
        input: ResendConfirmationCodeRequest,
    ) -> RusotoFuture<ResendConfirmationCodeResponse, ResendConfirmationCodeError>;

    /// <p>Responds to the authentication challenge.</p>
    fn respond_to_auth_challenge(
        &self,
        input: RespondToAuthChallengeRequest,
    ) -> RusotoFuture<RespondToAuthChallengeResponse, RespondToAuthChallengeError>;

    /// <p>Configures actions on detected risks. To delete the risk configuration for <code>UserPoolId</code> or <code>ClientId</code>, pass null values for all four configuration types.</p> <p>To enable Amazon Cognito advanced security features, update the user pool to include the <code>UserPoolAddOns</code> key<code>AdvancedSecurityMode</code>.</p> <p>See .</p>
    fn set_risk_configuration(
        &self,
        input: SetRiskConfigurationRequest,
    ) -> RusotoFuture<SetRiskConfigurationResponse, SetRiskConfigurationError>;

    /// <p><p>Sets the UI customization information for a user pool&#39;s built-in app UI.</p> <p>You can specify app UI customization settings for a single client (with a specific <code>clientId</code>) or for all clients (by setting the <code>clientId</code> to <code>ALL</code>). If you specify <code>ALL</code>, the default configuration will be used for every client that has no UI customization set previously. If you specify UI customization settings for a particular client, it will no longer fall back to the <code>ALL</code> configuration. </p> <note> <p>To use this API, your user pool must have a domain associated with it. Otherwise, there is no place to host the app&#39;s pages, and the service will throw an error.</p> </note></p>
    fn set_ui_customization(
        &self,
        input: SetUICustomizationRequest,
    ) -> RusotoFuture<SetUICustomizationResponse, SetUICustomizationError>;

    /// <p>Set the user's multi-factor authentication (MFA) method preference.</p>
    fn set_user_mfa_preference(
        &self,
        input: SetUserMFAPreferenceRequest,
    ) -> RusotoFuture<SetUserMFAPreferenceResponse, SetUserMFAPreferenceError>;

    /// <p>Set the user pool MFA configuration.</p>
    fn set_user_pool_mfa_config(
        &self,
        input: SetUserPoolMfaConfigRequest,
    ) -> RusotoFuture<SetUserPoolMfaConfigResponse, SetUserPoolMfaConfigError>;

    /// <p>Sets the user settings like multi-factor authentication (MFA). If MFA is to be removed for a particular attribute pass the attribute with code delivery as null. If null list is passed, all MFA options are removed.</p>
    fn set_user_settings(
        &self,
        input: SetUserSettingsRequest,
    ) -> RusotoFuture<SetUserSettingsResponse, SetUserSettingsError>;

    /// <p>Registers the user in the specified user pool and creates a user name, password, and user attributes.</p>
    fn sign_up(&self, input: SignUpRequest) -> RusotoFuture<SignUpResponse, SignUpError>;

    /// <p>Starts the user import.</p>
    fn start_user_import_job(
        &self,
        input: StartUserImportJobRequest,
    ) -> RusotoFuture<StartUserImportJobResponse, StartUserImportJobError>;

    /// <p>Stops the user import job.</p>
    fn stop_user_import_job(
        &self,
        input: StopUserImportJobRequest,
    ) -> RusotoFuture<StopUserImportJobResponse, StopUserImportJobError>;

    /// <p>Provides the feedback for an authentication event whether it was from a valid user or not. This feedback is used for improving the risk evaluation decision for the user pool as part of Amazon Cognito advanced security.</p>
    fn update_auth_event_feedback(
        &self,
        input: UpdateAuthEventFeedbackRequest,
    ) -> RusotoFuture<UpdateAuthEventFeedbackResponse, UpdateAuthEventFeedbackError>;

    /// <p>Updates the device status.</p>
    fn update_device_status(
        &self,
        input: UpdateDeviceStatusRequest,
    ) -> RusotoFuture<UpdateDeviceStatusResponse, UpdateDeviceStatusError>;

    /// <p>Updates the specified group with the specified attributes.</p> <p>Requires developer credentials.</p>
    fn update_group(
        &self,
        input: UpdateGroupRequest,
    ) -> RusotoFuture<UpdateGroupResponse, UpdateGroupError>;

    /// <p>Updates identity provider information for a user pool.</p>
    fn update_identity_provider(
        &self,
        input: UpdateIdentityProviderRequest,
    ) -> RusotoFuture<UpdateIdentityProviderResponse, UpdateIdentityProviderError>;

    /// <p>Updates the name and scopes of resource server. All other fields are read-only.</p>
    fn update_resource_server(
        &self,
        input: UpdateResourceServerRequest,
    ) -> RusotoFuture<UpdateResourceServerResponse, UpdateResourceServerError>;

    /// <p>Allows a user to update a specific attribute (one at a time).</p>
    fn update_user_attributes(
        &self,
        input: UpdateUserAttributesRequest,
    ) -> RusotoFuture<UpdateUserAttributesResponse, UpdateUserAttributesError>;

    /// <p>Updates the specified user pool with the specified attributes. If you don't provide a value for an attribute, it will be set to the default value. You can get a list of the current user pool settings with .</p>
    fn update_user_pool(
        &self,
        input: UpdateUserPoolRequest,
    ) -> RusotoFuture<UpdateUserPoolResponse, UpdateUserPoolError>;

    /// <p>Updates the specified user pool app client with the specified attributes. If you don't provide a value for an attribute, it will be set to the default value. You can get a list of the current user pool app client settings with .</p>
    fn update_user_pool_client(
        &self,
        input: UpdateUserPoolClientRequest,
    ) -> RusotoFuture<UpdateUserPoolClientResponse, UpdateUserPoolClientError>;

    /// <p>Updates the Secure Sockets Layer (SSL) certificate for the custom domain for your user pool.</p> <p>You can use this operation to provide the Amazon Resource Name (ARN) of a new certificate to Amazon Cognito. You cannot use it to change the domain for a user pool.</p> <p>A custom domain is used to host the Amazon Cognito hosted UI, which provides sign-up and sign-in pages for your application. When you set up a custom domain, you provide a certificate that you manage with AWS Certificate Manager (ACM). When necessary, you can use this operation to change the certificate that you applied to your custom domain.</p> <p>Usually, this is unnecessary following routine certificate renewal with ACM. When you renew your existing certificate in ACM, the ARN for your certificate remains the same, and your custom domain uses the new certificate automatically.</p> <p>However, if you replace your existing certificate with a new one, ACM gives the new certificate a new ARN. To apply the new certificate to your custom domain, you must provide this ARN to Amazon Cognito.</p> <p>When you add your new certificate in ACM, you must choose US East (N. Virginia) as the AWS Region.</p> <p>After you submit your request, Amazon Cognito requires up to 1 hour to distribute your new certificate to your custom domain.</p> <p>For more information about adding a custom domain to your user pool, see <a href="http://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-add-custom-domain.html">Using Your Own Domain for the Hosted UI</a>.</p>
    fn update_user_pool_domain(
        &self,
        input: UpdateUserPoolDomainRequest,
    ) -> RusotoFuture<UpdateUserPoolDomainResponse, UpdateUserPoolDomainError>;

    /// <p>Use this API to register a user's entered TOTP code and mark the user's software token MFA status as "verified" if successful. The request takes an access token or a session string, but not both.</p>
    fn verify_software_token(
        &self,
        input: VerifySoftwareTokenRequest,
    ) -> RusotoFuture<VerifySoftwareTokenResponse, VerifySoftwareTokenError>;

    /// <p>Verifies the specified user attributes in the user pool.</p>
    fn verify_user_attribute(
        &self,
        input: VerifyUserAttributeRequest,
    ) -> RusotoFuture<VerifyUserAttributeResponse, VerifyUserAttributeError>;
}
/// A client for the Amazon Cognito Identity Provider API.
#[derive(Clone)]
pub struct CognitoIdentityProviderClient {
    client: Client,
    region: region::Region,
}

impl CognitoIdentityProviderClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> CognitoIdentityProviderClient {
        CognitoIdentityProviderClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> CognitoIdentityProviderClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        CognitoIdentityProviderClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl CognitoIdentityProvider for CognitoIdentityProviderClient {
    /// <p>Adds additional user attributes to the user pool schema.</p>
    fn add_custom_attributes(
        &self,
        input: AddCustomAttributesRequest,
    ) -> RusotoFuture<AddCustomAttributesResponse, AddCustomAttributesError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AddCustomAttributes",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<AddCustomAttributesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(AddCustomAttributesError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Adds the specified user to the specified group.</p> <p>Requires developer credentials.</p>
    fn admin_add_user_to_group(
        &self,
        input: AdminAddUserToGroupRequest,
    ) -> RusotoFuture<(), AdminAddUserToGroupError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminAddUserToGroup",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(AdminAddUserToGroupError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Confirms user registration as an admin without using a confirmation code. Works on any user.</p> <p>Requires developer credentials.</p>
    fn admin_confirm_sign_up(
        &self,
        input: AdminConfirmSignUpRequest,
    ) -> RusotoFuture<AdminConfirmSignUpResponse, AdminConfirmSignUpError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminConfirmSignUp",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<AdminConfirmSignUpResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(AdminConfirmSignUpError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a new user in the specified user pool.</p> <p>If <code>MessageAction</code> is not set, the default is to send a welcome message via email or phone (SMS).</p> <note> <p>This message is based on a template that you configured in your call to or . This template includes your custom sign-up instructions and placeholders for user name and temporary password.</p> </note> <p>Alternatively, you can call AdminCreateUser with “SUPPRESS” for the <code>MessageAction</code> parameter, and Amazon Cognito will not send any email. </p> <p>In either case, the user will be in the <code>FORCE_CHANGE_PASSWORD</code> state until they sign in and change their password.</p> <p>AdminCreateUser requires developer credentials.</p>
    fn admin_create_user(
        &self,
        input: AdminCreateUserRequest,
    ) -> RusotoFuture<AdminCreateUserResponse, AdminCreateUserError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminCreateUser",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<AdminCreateUserResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(AdminCreateUserError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a user as an administrator. Works on any user.</p> <p>Requires developer credentials.</p>
    fn admin_delete_user(
        &self,
        input: AdminDeleteUserRequest,
    ) -> RusotoFuture<(), AdminDeleteUserError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminDeleteUser",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(AdminDeleteUserError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the user attributes in a user pool as an administrator. Works on any user.</p> <p>Requires developer credentials.</p>
    fn admin_delete_user_attributes(
        &self,
        input: AdminDeleteUserAttributesRequest,
    ) -> RusotoFuture<AdminDeleteUserAttributesResponse, AdminDeleteUserAttributesError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminDeleteUserAttributes",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<AdminDeleteUserAttributesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AdminDeleteUserAttributesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Disables the user from signing in with the specified external (SAML or social) identity provider. If the user to disable is a Cognito User Pools native username + password user, they are not permitted to use their password to sign-in. If the user to disable is a linked external IdP user, any link between that user and an existing user is removed. The next time the external user (no longer attached to the previously linked <code>DestinationUser</code>) signs in, they must create a new user account. See .</p> <p>This action is enabled only for admin access and requires developer credentials.</p> <p>The <code>ProviderName</code> must match the value specified when creating an IdP for the pool. </p> <p>To disable a native username + password user, the <code>ProviderName</code> value must be <code>Cognito</code> and the <code>ProviderAttributeName</code> must be <code>Cognito_Subject</code>, with the <code>ProviderAttributeValue</code> being the name that is used in the user pool for the user.</p> <p>The <code>ProviderAttributeName</code> must always be <code>Cognito_Subject</code> for social identity providers. The <code>ProviderAttributeValue</code> must always be the exact subject that was used when the user was originally linked as a source user.</p> <p>For de-linking a SAML identity, there are two scenarios. If the linked identity has not yet been used to sign-in, the <code>ProviderAttributeName</code> and <code>ProviderAttributeValue</code> must be the same values that were used for the <code>SourceUser</code> when the identities were originally linked in the call. (If the linking was done with <code>ProviderAttributeName</code> set to <code>Cognito_Subject</code>, the same applies here). However, if the user has already signed in, the <code>ProviderAttributeName</code> must be <code>Cognito_Subject</code> and <code>ProviderAttributeValue</code> must be the subject of the SAML assertion.</p>
    fn admin_disable_provider_for_user(
        &self,
        input: AdminDisableProviderForUserRequest,
    ) -> RusotoFuture<AdminDisableProviderForUserResponse, AdminDisableProviderForUserError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminDisableProviderForUser",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<AdminDisableProviderForUserResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AdminDisableProviderForUserError::from_response(response))
                }))
            }
        })
    }

    /// <p>Disables the specified user as an administrator. Works on any user.</p> <p>Requires developer credentials.</p>
    fn admin_disable_user(
        &self,
        input: AdminDisableUserRequest,
    ) -> RusotoFuture<AdminDisableUserResponse, AdminDisableUserError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminDisableUser",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<AdminDisableUserResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(AdminDisableUserError::from_response(response))),
                )
            }
        })
    }

    /// <p>Enables the specified user as an administrator. Works on any user.</p> <p>Requires developer credentials.</p>
    fn admin_enable_user(
        &self,
        input: AdminEnableUserRequest,
    ) -> RusotoFuture<AdminEnableUserResponse, AdminEnableUserError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminEnableUser",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<AdminEnableUserResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(AdminEnableUserError::from_response(response))),
                )
            }
        })
    }

    /// <p>Forgets the device, as an administrator.</p> <p>Requires developer credentials.</p>
    fn admin_forget_device(
        &self,
        input: AdminForgetDeviceRequest,
    ) -> RusotoFuture<(), AdminForgetDeviceError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminForgetDevice",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(AdminForgetDeviceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets the device, as an administrator.</p> <p>Requires developer credentials.</p>
    fn admin_get_device(
        &self,
        input: AdminGetDeviceRequest,
    ) -> RusotoFuture<AdminGetDeviceResponse, AdminGetDeviceError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminGetDevice",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<AdminGetDeviceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(AdminGetDeviceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets the specified user by user name in a user pool as an administrator. Works on any user.</p> <p>Requires developer credentials.</p>
    fn admin_get_user(
        &self,
        input: AdminGetUserRequest,
    ) -> RusotoFuture<AdminGetUserResponse, AdminGetUserError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminGetUser",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<AdminGetUserResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(AdminGetUserError::from_response(response))),
                )
            }
        })
    }

    /// <p>Initiates the authentication flow, as an administrator.</p> <p>Requires developer credentials.</p>
    fn admin_initiate_auth(
        &self,
        input: AdminInitiateAuthRequest,
    ) -> RusotoFuture<AdminInitiateAuthResponse, AdminInitiateAuthError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminInitiateAuth",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<AdminInitiateAuthResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(AdminInitiateAuthError::from_response(response))),
                )
            }
        })
    }

    /// <p>Links an existing user account in a user pool (<code>DestinationUser</code>) to an identity from an external identity provider (<code>SourceUser</code>) based on a specified attribute name and value from the external identity provider. This allows you to create a link from the existing user account to an external federated user identity that has not yet been used to sign in, so that the federated user identity can be used to sign in as the existing user account. </p> <p> For example, if there is an existing user with a username and password, this API links that user to a federated user identity, so that when the federated user identity is used, the user signs in as the existing user account. </p> <important> <p>Because this API allows a user with an external federated identity to sign in as an existing user in the user pool, it is critical that it only be used with external identity providers and provider attributes that have been trusted by the application owner.</p> </important> <p>See also .</p> <p>This action is enabled only for admin access and requires developer credentials.</p>
    fn admin_link_provider_for_user(
        &self,
        input: AdminLinkProviderForUserRequest,
    ) -> RusotoFuture<AdminLinkProviderForUserResponse, AdminLinkProviderForUserError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminLinkProviderForUser",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<AdminLinkProviderForUserResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AdminLinkProviderForUserError::from_response(response))
                }))
            }
        })
    }

    /// <p>Lists devices, as an administrator.</p> <p>Requires developer credentials.</p>
    fn admin_list_devices(
        &self,
        input: AdminListDevicesRequest,
    ) -> RusotoFuture<AdminListDevicesResponse, AdminListDevicesError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminListDevices",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<AdminListDevicesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(AdminListDevicesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists the groups that the user belongs to.</p> <p>Requires developer credentials.</p>
    fn admin_list_groups_for_user(
        &self,
        input: AdminListGroupsForUserRequest,
    ) -> RusotoFuture<AdminListGroupsForUserResponse, AdminListGroupsForUserError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminListGroupsForUser",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<AdminListGroupsForUserResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(AdminListGroupsForUserError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Lists a history of user activity and any risks detected as part of Amazon Cognito advanced security.</p>
    fn admin_list_user_auth_events(
        &self,
        input: AdminListUserAuthEventsRequest,
    ) -> RusotoFuture<AdminListUserAuthEventsResponse, AdminListUserAuthEventsError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminListUserAuthEvents",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<AdminListUserAuthEventsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AdminListUserAuthEventsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Removes the specified user from the specified group.</p> <p>Requires developer credentials.</p>
    fn admin_remove_user_from_group(
        &self,
        input: AdminRemoveUserFromGroupRequest,
    ) -> RusotoFuture<(), AdminRemoveUserFromGroupError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminRemoveUserFromGroup",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AdminRemoveUserFromGroupError::from_response(response))
                }))
            }
        })
    }

    /// <p>Resets the specified user's password in a user pool as an administrator. Works on any user.</p> <p>When a developer calls this API, the current password is invalidated, so it must be changed. If a user tries to sign in after the API is called, the app will get a PasswordResetRequiredException exception back and should direct the user down the flow to reset the password, which is the same as the forgot password flow. In addition, if the user pool has phone verification selected and a verified phone number exists for the user, or if email verification is selected and a verified email exists for the user, calling this API will also result in sending a message to the end user with the code to change their password.</p> <p>Requires developer credentials.</p>
    fn admin_reset_user_password(
        &self,
        input: AdminResetUserPasswordRequest,
    ) -> RusotoFuture<AdminResetUserPasswordResponse, AdminResetUserPasswordError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminResetUserPassword",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<AdminResetUserPasswordResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(AdminResetUserPasswordError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Responds to an authentication challenge, as an administrator.</p> <p>Requires developer credentials.</p>
    fn admin_respond_to_auth_challenge(
        &self,
        input: AdminRespondToAuthChallengeRequest,
    ) -> RusotoFuture<AdminRespondToAuthChallengeResponse, AdminRespondToAuthChallengeError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminRespondToAuthChallenge",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<AdminRespondToAuthChallengeResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AdminRespondToAuthChallengeError::from_response(response))
                }))
            }
        })
    }

    /// <p>Sets the user's multi-factor authentication (MFA) preference.</p>
    fn admin_set_user_mfa_preference(
        &self,
        input: AdminSetUserMFAPreferenceRequest,
    ) -> RusotoFuture<AdminSetUserMFAPreferenceResponse, AdminSetUserMFAPreferenceError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminSetUserMFAPreference",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<AdminSetUserMFAPreferenceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AdminSetUserMFAPreferenceError::from_response(response))
                }))
            }
        })
    }

    /// <p>Sets all the user settings for a specified user name. Works on any user.</p> <p>Requires developer credentials.</p>
    fn admin_set_user_settings(
        &self,
        input: AdminSetUserSettingsRequest,
    ) -> RusotoFuture<AdminSetUserSettingsResponse, AdminSetUserSettingsError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminSetUserSettings",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<AdminSetUserSettingsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(AdminSetUserSettingsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Provides feedback for an authentication event as to whether it was from a valid user. This feedback is used for improving the risk evaluation decision for the user pool as part of Amazon Cognito advanced security.</p>
    fn admin_update_auth_event_feedback(
        &self,
        input: AdminUpdateAuthEventFeedbackRequest,
    ) -> RusotoFuture<AdminUpdateAuthEventFeedbackResponse, AdminUpdateAuthEventFeedbackError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminUpdateAuthEventFeedback",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<AdminUpdateAuthEventFeedbackResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AdminUpdateAuthEventFeedbackError::from_response(response))
                }))
            }
        })
    }

    /// <p>Updates the device status as an administrator.</p> <p>Requires developer credentials.</p>
    fn admin_update_device_status(
        &self,
        input: AdminUpdateDeviceStatusRequest,
    ) -> RusotoFuture<AdminUpdateDeviceStatusResponse, AdminUpdateDeviceStatusError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminUpdateDeviceStatus",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<AdminUpdateDeviceStatusResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AdminUpdateDeviceStatusError::from_response(response))
                }))
            }
        })
    }

    /// <p>Updates the specified user's attributes, including developer attributes, as an administrator. Works on any user.</p> <p>For custom attributes, you must prepend the <code>custom:</code> prefix to the attribute name.</p> <p>In addition to updating user attributes, this API can also be used to mark phone and email as verified.</p> <p>Requires developer credentials.</p>
    fn admin_update_user_attributes(
        &self,
        input: AdminUpdateUserAttributesRequest,
    ) -> RusotoFuture<AdminUpdateUserAttributesResponse, AdminUpdateUserAttributesError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminUpdateUserAttributes",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<AdminUpdateUserAttributesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AdminUpdateUserAttributesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Signs out users from all devices, as an administrator.</p> <p>Requires developer credentials.</p>
    fn admin_user_global_sign_out(
        &self,
        input: AdminUserGlobalSignOutRequest,
    ) -> RusotoFuture<AdminUserGlobalSignOutResponse, AdminUserGlobalSignOutError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminUserGlobalSignOut",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<AdminUserGlobalSignOutResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(AdminUserGlobalSignOutError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns a unique generated shared secret key code for the user account. The request takes an access token or a session string, but not both.</p>
    fn associate_software_token(
        &self,
        input: AssociateSoftwareTokenRequest,
    ) -> RusotoFuture<AssociateSoftwareTokenResponse, AssociateSoftwareTokenError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AssociateSoftwareToken",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<AssociateSoftwareTokenResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(AssociateSoftwareTokenError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Changes the password for a specified user in a user pool.</p>
    fn change_password(
        &self,
        input: ChangePasswordRequest,
    ) -> RusotoFuture<ChangePasswordResponse, ChangePasswordError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.ChangePassword",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<ChangePasswordResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ChangePasswordError::from_response(response))),
                )
            }
        })
    }

    /// <p>Confirms tracking of the device. This API call is the call that begins device tracking.</p>
    fn confirm_device(
        &self,
        input: ConfirmDeviceRequest,
    ) -> RusotoFuture<ConfirmDeviceResponse, ConfirmDeviceError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.ConfirmDevice",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<ConfirmDeviceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ConfirmDeviceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Allows a user to enter a confirmation code to reset a forgotten password.</p>
    fn confirm_forgot_password(
        &self,
        input: ConfirmForgotPasswordRequest,
    ) -> RusotoFuture<ConfirmForgotPasswordResponse, ConfirmForgotPasswordError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.ConfirmForgotPassword",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<ConfirmForgotPasswordResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ConfirmForgotPasswordError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Confirms registration of a user and handles the existing alias from a previous user.</p>
    fn confirm_sign_up(
        &self,
        input: ConfirmSignUpRequest,
    ) -> RusotoFuture<ConfirmSignUpResponse, ConfirmSignUpError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.ConfirmSignUp",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<ConfirmSignUpResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ConfirmSignUpError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a new group in the specified user pool.</p> <p>Requires developer credentials.</p>
    fn create_group(
        &self,
        input: CreateGroupRequest,
    ) -> RusotoFuture<CreateGroupResponse, CreateGroupError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.CreateGroup",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<CreateGroupResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateGroupError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates an identity provider for a user pool.</p>
    fn create_identity_provider(
        &self,
        input: CreateIdentityProviderRequest,
    ) -> RusotoFuture<CreateIdentityProviderResponse, CreateIdentityProviderError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.CreateIdentityProvider",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<CreateIdentityProviderResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateIdentityProviderError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Creates a new OAuth2.0 resource server and defines custom scopes in it.</p>
    fn create_resource_server(
        &self,
        input: CreateResourceServerRequest,
    ) -> RusotoFuture<CreateResourceServerResponse, CreateResourceServerError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.CreateResourceServer",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<CreateResourceServerResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateResourceServerError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Creates the user import job.</p>
    fn create_user_import_job(
        &self,
        input: CreateUserImportJobRequest,
    ) -> RusotoFuture<CreateUserImportJobResponse, CreateUserImportJobError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.CreateUserImportJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<CreateUserImportJobResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateUserImportJobError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Creates a new Amazon Cognito user pool and sets the password policy for the pool.</p>
    fn create_user_pool(
        &self,
        input: CreateUserPoolRequest,
    ) -> RusotoFuture<CreateUserPoolResponse, CreateUserPoolError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.CreateUserPool",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<CreateUserPoolResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateUserPoolError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates the user pool client.</p>
    fn create_user_pool_client(
        &self,
        input: CreateUserPoolClientRequest,
    ) -> RusotoFuture<CreateUserPoolClientResponse, CreateUserPoolClientError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.CreateUserPoolClient",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<CreateUserPoolClientResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateUserPoolClientError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Creates a new domain for a user pool.</p>
    fn create_user_pool_domain(
        &self,
        input: CreateUserPoolDomainRequest,
    ) -> RusotoFuture<CreateUserPoolDomainResponse, CreateUserPoolDomainError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.CreateUserPoolDomain",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<CreateUserPoolDomainResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateUserPoolDomainError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes a group. Currently only groups with no members can be deleted.</p> <p>Requires developer credentials.</p>
    fn delete_group(&self, input: DeleteGroupRequest) -> RusotoFuture<(), DeleteGroupError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.DeleteGroup",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteGroupError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes an identity provider for a user pool.</p>
    fn delete_identity_provider(
        &self,
        input: DeleteIdentityProviderRequest,
    ) -> RusotoFuture<(), DeleteIdentityProviderError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.DeleteIdentityProvider",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteIdentityProviderError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes a resource server.</p>
    fn delete_resource_server(
        &self,
        input: DeleteResourceServerRequest,
    ) -> RusotoFuture<(), DeleteResourceServerError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.DeleteResourceServer",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteResourceServerError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Allows a user to delete himself or herself.</p>
    fn delete_user(&self, input: DeleteUserRequest) -> RusotoFuture<(), DeleteUserError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.DeleteUser",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteUserError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the attributes for a user.</p>
    fn delete_user_attributes(
        &self,
        input: DeleteUserAttributesRequest,
    ) -> RusotoFuture<DeleteUserAttributesResponse, DeleteUserAttributesError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.DeleteUserAttributes",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<DeleteUserAttributesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteUserAttributesError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes the specified Amazon Cognito user pool.</p>
    fn delete_user_pool(
        &self,
        input: DeleteUserPoolRequest,
    ) -> RusotoFuture<(), DeleteUserPoolError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.DeleteUserPool",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteUserPoolError::from_response(response))),
                )
            }
        })
    }

    /// <p>Allows the developer to delete the user pool client.</p>
    fn delete_user_pool_client(
        &self,
        input: DeleteUserPoolClientRequest,
    ) -> RusotoFuture<(), DeleteUserPoolClientError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.DeleteUserPoolClient",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteUserPoolClientError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes a domain for a user pool.</p>
    fn delete_user_pool_domain(
        &self,
        input: DeleteUserPoolDomainRequest,
    ) -> RusotoFuture<DeleteUserPoolDomainResponse, DeleteUserPoolDomainError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.DeleteUserPoolDomain",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<DeleteUserPoolDomainResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteUserPoolDomainError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Gets information about a specific identity provider.</p>
    fn describe_identity_provider(
        &self,
        input: DescribeIdentityProviderRequest,
    ) -> RusotoFuture<DescribeIdentityProviderResponse, DescribeIdentityProviderError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.DescribeIdentityProvider",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<DescribeIdentityProviderResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeIdentityProviderError::from_response(response))
                }))
            }
        })
    }

    /// <p>Describes a resource server.</p>
    fn describe_resource_server(
        &self,
        input: DescribeResourceServerRequest,
    ) -> RusotoFuture<DescribeResourceServerResponse, DescribeResourceServerError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.DescribeResourceServer",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<DescribeResourceServerResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeResourceServerError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Describes the risk configuration.</p>
    fn describe_risk_configuration(
        &self,
        input: DescribeRiskConfigurationRequest,
    ) -> RusotoFuture<DescribeRiskConfigurationResponse, DescribeRiskConfigurationError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.DescribeRiskConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<DescribeRiskConfigurationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeRiskConfigurationError::from_response(response))
                }))
            }
        })
    }

    /// <p>Describes the user import job.</p>
    fn describe_user_import_job(
        &self,
        input: DescribeUserImportJobRequest,
    ) -> RusotoFuture<DescribeUserImportJobResponse, DescribeUserImportJobError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.DescribeUserImportJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<DescribeUserImportJobResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeUserImportJobError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns the configuration information and metadata of the specified user pool.</p>
    fn describe_user_pool(
        &self,
        input: DescribeUserPoolRequest,
    ) -> RusotoFuture<DescribeUserPoolResponse, DescribeUserPoolError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.DescribeUserPool",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<DescribeUserPoolResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeUserPoolError::from_response(response))),
                )
            }
        })
    }

    /// <p>Client method for returning the configuration information and metadata of the specified user pool app client.</p>
    fn describe_user_pool_client(
        &self,
        input: DescribeUserPoolClientRequest,
    ) -> RusotoFuture<DescribeUserPoolClientResponse, DescribeUserPoolClientError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.DescribeUserPoolClient",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<DescribeUserPoolClientResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeUserPoolClientError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Gets information about a domain.</p>
    fn describe_user_pool_domain(
        &self,
        input: DescribeUserPoolDomainRequest,
    ) -> RusotoFuture<DescribeUserPoolDomainResponse, DescribeUserPoolDomainError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.DescribeUserPoolDomain",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<DescribeUserPoolDomainResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeUserPoolDomainError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Forgets the specified device.</p>
    fn forget_device(&self, input: ForgetDeviceRequest) -> RusotoFuture<(), ForgetDeviceError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.ForgetDevice",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ForgetDeviceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Calling this API causes a message to be sent to the end user with a confirmation code that is required to change the user's password. For the <code>Username</code> parameter, you can use the username or user alias. If a verified phone number exists for the user, the confirmation code is sent to the phone number. Otherwise, if a verified email exists, the confirmation code is sent to the email. If neither a verified phone number nor a verified email exists, <code>InvalidParameterException</code> is thrown. To use the confirmation code for resetting the password, call .</p>
    fn forgot_password(
        &self,
        input: ForgotPasswordRequest,
    ) -> RusotoFuture<ForgotPasswordResponse, ForgotPasswordError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.ForgotPassword",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<ForgotPasswordResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ForgotPasswordError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets the header information for the .csv file to be used as input for the user import job.</p>
    fn get_csv_header(
        &self,
        input: GetCSVHeaderRequest,
    ) -> RusotoFuture<GetCSVHeaderResponse, GetCSVHeaderError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.GetCSVHeader",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<GetCSVHeaderResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetCSVHeaderError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets the device.</p>
    fn get_device(
        &self,
        input: GetDeviceRequest,
    ) -> RusotoFuture<GetDeviceResponse, GetDeviceError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.GetDevice",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<GetDeviceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetDeviceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets a group.</p> <p>Requires developer credentials.</p>
    fn get_group(&self, input: GetGroupRequest) -> RusotoFuture<GetGroupResponse, GetGroupError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSCognitoIdentityProviderService.GetGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<GetGroupResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetGroupError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets the specified identity provider.</p>
    fn get_identity_provider_by_identifier(
        &self,
        input: GetIdentityProviderByIdentifierRequest,
    ) -> RusotoFuture<GetIdentityProviderByIdentifierResponse, GetIdentityProviderByIdentifierError>
    {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.GetIdentityProviderByIdentifier",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<GetIdentityProviderByIdentifierResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetIdentityProviderByIdentifierError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>This method takes a user pool ID, and returns the signing certificate.</p>
    fn get_signing_certificate(
        &self,
        input: GetSigningCertificateRequest,
    ) -> RusotoFuture<GetSigningCertificateResponse, GetSigningCertificateError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.GetSigningCertificate",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<GetSigningCertificateResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetSigningCertificateError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Gets the UI Customization information for a particular app client's app UI, if there is something set. If nothing is set for the particular client, but there is an existing pool level customization (app <code>clientId</code> will be <code>ALL</code>), then that is returned. If nothing is present, then an empty shape is returned.</p>
    fn get_ui_customization(
        &self,
        input: GetUICustomizationRequest,
    ) -> RusotoFuture<GetUICustomizationResponse, GetUICustomizationError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.GetUICustomization",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<GetUICustomizationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetUICustomizationError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets the user attributes and metadata for a user.</p>
    fn get_user(&self, input: GetUserRequest) -> RusotoFuture<GetUserResponse, GetUserError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSCognitoIdentityProviderService.GetUser");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<GetUserResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
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

    /// <p>Gets the user attribute verification code for the specified attribute name.</p>
    fn get_user_attribute_verification_code(
        &self,
        input: GetUserAttributeVerificationCodeRequest,
    ) -> RusotoFuture<GetUserAttributeVerificationCodeResponse, GetUserAttributeVerificationCodeError>
    {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.GetUserAttributeVerificationCode",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<GetUserAttributeVerificationCodeResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetUserAttributeVerificationCodeError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Gets the user pool multi-factor authentication (MFA) configuration.</p>
    fn get_user_pool_mfa_config(
        &self,
        input: GetUserPoolMfaConfigRequest,
    ) -> RusotoFuture<GetUserPoolMfaConfigResponse, GetUserPoolMfaConfigError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.GetUserPoolMfaConfig",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<GetUserPoolMfaConfigResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetUserPoolMfaConfigError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Signs out users from all devices.</p>
    fn global_sign_out(
        &self,
        input: GlobalSignOutRequest,
    ) -> RusotoFuture<GlobalSignOutResponse, GlobalSignOutError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.GlobalSignOut",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<GlobalSignOutResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GlobalSignOutError::from_response(response))),
                )
            }
        })
    }

    /// <p>Initiates the authentication flow.</p>
    fn initiate_auth(
        &self,
        input: InitiateAuthRequest,
    ) -> RusotoFuture<InitiateAuthResponse, InitiateAuthError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.InitiateAuth",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<InitiateAuthResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(InitiateAuthError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists the devices.</p>
    fn list_devices(
        &self,
        input: ListDevicesRequest,
    ) -> RusotoFuture<ListDevicesResponse, ListDevicesError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.ListDevices",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<ListDevicesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListDevicesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists the groups associated with a user pool.</p> <p>Requires developer credentials.</p>
    fn list_groups(
        &self,
        input: ListGroupsRequest,
    ) -> RusotoFuture<ListGroupsResponse, ListGroupsError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.ListGroups",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<ListGroupsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListGroupsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists information about all identity providers for a user pool.</p>
    fn list_identity_providers(
        &self,
        input: ListIdentityProvidersRequest,
    ) -> RusotoFuture<ListIdentityProvidersResponse, ListIdentityProvidersError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.ListIdentityProviders",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<ListIdentityProvidersResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListIdentityProvidersError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Lists the resource servers for a user pool.</p>
    fn list_resource_servers(
        &self,
        input: ListResourceServersRequest,
    ) -> RusotoFuture<ListResourceServersResponse, ListResourceServersError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.ListResourceServers",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<ListResourceServersResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListResourceServersError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Lists the user import jobs.</p>
    fn list_user_import_jobs(
        &self,
        input: ListUserImportJobsRequest,
    ) -> RusotoFuture<ListUserImportJobsResponse, ListUserImportJobsError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.ListUserImportJobs",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<ListUserImportJobsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListUserImportJobsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists the clients that have been created for the specified user pool.</p>
    fn list_user_pool_clients(
        &self,
        input: ListUserPoolClientsRequest,
    ) -> RusotoFuture<ListUserPoolClientsResponse, ListUserPoolClientsError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.ListUserPoolClients",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<ListUserPoolClientsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListUserPoolClientsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Lists the user pools associated with an AWS account.</p>
    fn list_user_pools(
        &self,
        input: ListUserPoolsRequest,
    ) -> RusotoFuture<ListUserPoolsResponse, ListUserPoolsError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.ListUserPools",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<ListUserPoolsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListUserPoolsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists the users in the Amazon Cognito user pool.</p>
    fn list_users(
        &self,
        input: ListUsersRequest,
    ) -> RusotoFuture<ListUsersResponse, ListUsersError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.ListUsers",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<ListUsersResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
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

    /// <p>Lists the users in the specified group.</p> <p>Requires developer credentials.</p>
    fn list_users_in_group(
        &self,
        input: ListUsersInGroupRequest,
    ) -> RusotoFuture<ListUsersInGroupResponse, ListUsersInGroupError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.ListUsersInGroup",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<ListUsersInGroupResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListUsersInGroupError::from_response(response))),
                )
            }
        })
    }

    /// <p>Resends the confirmation (for confirmation of registration) to a specific user in the user pool.</p>
    fn resend_confirmation_code(
        &self,
        input: ResendConfirmationCodeRequest,
    ) -> RusotoFuture<ResendConfirmationCodeResponse, ResendConfirmationCodeError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.ResendConfirmationCode",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<ResendConfirmationCodeResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ResendConfirmationCodeError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Responds to the authentication challenge.</p>
    fn respond_to_auth_challenge(
        &self,
        input: RespondToAuthChallengeRequest,
    ) -> RusotoFuture<RespondToAuthChallengeResponse, RespondToAuthChallengeError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.RespondToAuthChallenge",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<RespondToAuthChallengeResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(RespondToAuthChallengeError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Configures actions on detected risks. To delete the risk configuration for <code>UserPoolId</code> or <code>ClientId</code>, pass null values for all four configuration types.</p> <p>To enable Amazon Cognito advanced security features, update the user pool to include the <code>UserPoolAddOns</code> key<code>AdvancedSecurityMode</code>.</p> <p>See .</p>
    fn set_risk_configuration(
        &self,
        input: SetRiskConfigurationRequest,
    ) -> RusotoFuture<SetRiskConfigurationResponse, SetRiskConfigurationError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.SetRiskConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<SetRiskConfigurationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(SetRiskConfigurationError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p><p>Sets the UI customization information for a user pool&#39;s built-in app UI.</p> <p>You can specify app UI customization settings for a single client (with a specific <code>clientId</code>) or for all clients (by setting the <code>clientId</code> to <code>ALL</code>). If you specify <code>ALL</code>, the default configuration will be used for every client that has no UI customization set previously. If you specify UI customization settings for a particular client, it will no longer fall back to the <code>ALL</code> configuration. </p> <note> <p>To use this API, your user pool must have a domain associated with it. Otherwise, there is no place to host the app&#39;s pages, and the service will throw an error.</p> </note></p>
    fn set_ui_customization(
        &self,
        input: SetUICustomizationRequest,
    ) -> RusotoFuture<SetUICustomizationResponse, SetUICustomizationError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.SetUICustomization",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<SetUICustomizationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(SetUICustomizationError::from_response(response))),
                )
            }
        })
    }

    /// <p>Set the user's multi-factor authentication (MFA) method preference.</p>
    fn set_user_mfa_preference(
        &self,
        input: SetUserMFAPreferenceRequest,
    ) -> RusotoFuture<SetUserMFAPreferenceResponse, SetUserMFAPreferenceError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.SetUserMFAPreference",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<SetUserMFAPreferenceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(SetUserMFAPreferenceError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Set the user pool MFA configuration.</p>
    fn set_user_pool_mfa_config(
        &self,
        input: SetUserPoolMfaConfigRequest,
    ) -> RusotoFuture<SetUserPoolMfaConfigResponse, SetUserPoolMfaConfigError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.SetUserPoolMfaConfig",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<SetUserPoolMfaConfigResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(SetUserPoolMfaConfigError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Sets the user settings like multi-factor authentication (MFA). If MFA is to be removed for a particular attribute pass the attribute with code delivery as null. If null list is passed, all MFA options are removed.</p>
    fn set_user_settings(
        &self,
        input: SetUserSettingsRequest,
    ) -> RusotoFuture<SetUserSettingsResponse, SetUserSettingsError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.SetUserSettings",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<SetUserSettingsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(SetUserSettingsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Registers the user in the specified user pool and creates a user name, password, and user attributes.</p>
    fn sign_up(&self, input: SignUpRequest) -> RusotoFuture<SignUpResponse, SignUpError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSCognitoIdentityProviderService.SignUp");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<SignUpResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(SignUpError::from_response(response))),
                )
            }
        })
    }

    /// <p>Starts the user import.</p>
    fn start_user_import_job(
        &self,
        input: StartUserImportJobRequest,
    ) -> RusotoFuture<StartUserImportJobResponse, StartUserImportJobError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.StartUserImportJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<StartUserImportJobResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(StartUserImportJobError::from_response(response))),
                )
            }
        })
    }

    /// <p>Stops the user import job.</p>
    fn stop_user_import_job(
        &self,
        input: StopUserImportJobRequest,
    ) -> RusotoFuture<StopUserImportJobResponse, StopUserImportJobError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.StopUserImportJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<StopUserImportJobResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(StopUserImportJobError::from_response(response))),
                )
            }
        })
    }

    /// <p>Provides the feedback for an authentication event whether it was from a valid user or not. This feedback is used for improving the risk evaluation decision for the user pool as part of Amazon Cognito advanced security.</p>
    fn update_auth_event_feedback(
        &self,
        input: UpdateAuthEventFeedbackRequest,
    ) -> RusotoFuture<UpdateAuthEventFeedbackResponse, UpdateAuthEventFeedbackError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.UpdateAuthEventFeedback",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<UpdateAuthEventFeedbackResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateAuthEventFeedbackError::from_response(response))
                }))
            }
        })
    }

    /// <p>Updates the device status.</p>
    fn update_device_status(
        &self,
        input: UpdateDeviceStatusRequest,
    ) -> RusotoFuture<UpdateDeviceStatusResponse, UpdateDeviceStatusError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.UpdateDeviceStatus",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<UpdateDeviceStatusResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateDeviceStatusError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates the specified group with the specified attributes.</p> <p>Requires developer credentials.</p>
    fn update_group(
        &self,
        input: UpdateGroupRequest,
    ) -> RusotoFuture<UpdateGroupResponse, UpdateGroupError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.UpdateGroup",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<UpdateGroupResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateGroupError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates identity provider information for a user pool.</p>
    fn update_identity_provider(
        &self,
        input: UpdateIdentityProviderRequest,
    ) -> RusotoFuture<UpdateIdentityProviderResponse, UpdateIdentityProviderError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.UpdateIdentityProvider",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<UpdateIdentityProviderResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateIdentityProviderError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Updates the name and scopes of resource server. All other fields are read-only.</p>
    fn update_resource_server(
        &self,
        input: UpdateResourceServerRequest,
    ) -> RusotoFuture<UpdateResourceServerResponse, UpdateResourceServerError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.UpdateResourceServer",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<UpdateResourceServerResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateResourceServerError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Allows a user to update a specific attribute (one at a time).</p>
    fn update_user_attributes(
        &self,
        input: UpdateUserAttributesRequest,
    ) -> RusotoFuture<UpdateUserAttributesResponse, UpdateUserAttributesError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.UpdateUserAttributes",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<UpdateUserAttributesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateUserAttributesError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Updates the specified user pool with the specified attributes. If you don't provide a value for an attribute, it will be set to the default value. You can get a list of the current user pool settings with .</p>
    fn update_user_pool(
        &self,
        input: UpdateUserPoolRequest,
    ) -> RusotoFuture<UpdateUserPoolResponse, UpdateUserPoolError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.UpdateUserPool",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<UpdateUserPoolResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateUserPoolError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates the specified user pool app client with the specified attributes. If you don't provide a value for an attribute, it will be set to the default value. You can get a list of the current user pool app client settings with .</p>
    fn update_user_pool_client(
        &self,
        input: UpdateUserPoolClientRequest,
    ) -> RusotoFuture<UpdateUserPoolClientResponse, UpdateUserPoolClientError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.UpdateUserPoolClient",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<UpdateUserPoolClientResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateUserPoolClientError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Updates the Secure Sockets Layer (SSL) certificate for the custom domain for your user pool.</p> <p>You can use this operation to provide the Amazon Resource Name (ARN) of a new certificate to Amazon Cognito. You cannot use it to change the domain for a user pool.</p> <p>A custom domain is used to host the Amazon Cognito hosted UI, which provides sign-up and sign-in pages for your application. When you set up a custom domain, you provide a certificate that you manage with AWS Certificate Manager (ACM). When necessary, you can use this operation to change the certificate that you applied to your custom domain.</p> <p>Usually, this is unnecessary following routine certificate renewal with ACM. When you renew your existing certificate in ACM, the ARN for your certificate remains the same, and your custom domain uses the new certificate automatically.</p> <p>However, if you replace your existing certificate with a new one, ACM gives the new certificate a new ARN. To apply the new certificate to your custom domain, you must provide this ARN to Amazon Cognito.</p> <p>When you add your new certificate in ACM, you must choose US East (N. Virginia) as the AWS Region.</p> <p>After you submit your request, Amazon Cognito requires up to 1 hour to distribute your new certificate to your custom domain.</p> <p>For more information about adding a custom domain to your user pool, see <a href="http://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-add-custom-domain.html">Using Your Own Domain for the Hosted UI</a>.</p>
    fn update_user_pool_domain(
        &self,
        input: UpdateUserPoolDomainRequest,
    ) -> RusotoFuture<UpdateUserPoolDomainResponse, UpdateUserPoolDomainError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.UpdateUserPoolDomain",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<UpdateUserPoolDomainResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateUserPoolDomainError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Use this API to register a user's entered TOTP code and mark the user's software token MFA status as "verified" if successful. The request takes an access token or a session string, but not both.</p>
    fn verify_software_token(
        &self,
        input: VerifySoftwareTokenRequest,
    ) -> RusotoFuture<VerifySoftwareTokenResponse, VerifySoftwareTokenError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.VerifySoftwareToken",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<VerifySoftwareTokenResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(VerifySoftwareTokenError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Verifies the specified user attributes in the user pool.</p>
    fn verify_user_attribute(
        &self,
        input: VerifyUserAttributeRequest,
    ) -> RusotoFuture<VerifyUserAttributeResponse, VerifyUserAttributeError> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.VerifyUserAttribute",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<VerifyUserAttributeResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(VerifyUserAttributeError::from_response(response))
                    }),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
