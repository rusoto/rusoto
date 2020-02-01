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
/// <p>The data type for <code>AccountRecoverySetting</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccountRecoverySettingType {
    /// <p>The list of <code>RecoveryOptionTypes</code>.</p>
    #[serde(rename = "RecoveryMechanisms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_mechanisms: Option<Vec<RecoveryOptionType>>,
}

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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AddCustomAttributesResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AdminConfirmSignUpRequest {
    /// <p><p>A map of custom key-value pairs that you can provide as input for any custom workflows that this action triggers. </p> <p>If your user pool configuration includes triggers, the AdminConfirmSignUp API action invokes the AWS Lambda function that is specified for the <i>post confirmation</i> trigger. When Amazon Cognito invokes this function, it passes a JSON payload, which the function receives as input. In this payload, the <code>clientMetadata</code> attribute provides the data that you assigned to the ClientMetadata parameter in your AdminConfirmSignUp request. In your function code in AWS Lambda, you can process the ClientMetadata value to enhance your workflow for your specific needs.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools-working-with-aws-lambda-triggers.html">Customizing User Pool Workflows with Lambda Triggers</a> in the <i>Amazon Cognito Developer Guide</i>.</p> <note> <p>Take the following limitations into consideration when you use the ClientMetadata parameter:</p> <ul> <li> <p>Amazon Cognito does not store the ClientMetadata value. This data is available only to AWS Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration does not include triggers, the ClientMetadata parameter serves no purpose.</p> </li> <li> <p>Amazon Cognito does not validate the ClientMetadata value.</p> </li> <li> <p>Amazon Cognito does not encrypt the the ClientMetadata value, so don&#39;t use it to provide sensitive information.</p> </li> </ul> </note></p>
    #[serde(rename = "ClientMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_metadata: Option<::std::collections::HashMap<String, String>>,
    /// <p>The user pool ID for which you want to confirm user registration.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
    /// <p>The user name for which you want to confirm user registration.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

/// <p>Represents the response from the server for the request to confirm registration.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AdminConfirmSignUpResponse {}

/// <p>The configuration for creating a new user profile.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdminCreateUserConfigType {
    /// <p>Set to <code>True</code> if only the administrator is allowed to create user profiles. Set to <code>False</code> if users can sign themselves up via an app.</p>
    #[serde(rename = "AllowAdminCreateUserOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_admin_create_user_only: Option<bool>,
    /// <p>The message template to be used for the welcome message to new users.</p> <p>See also <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pool-settings-message-customizations.html#cognito-user-pool-settings-user-invitation-message-customization">Customizing User Invitation Messages</a>.</p>
    #[serde(rename = "InviteMessageTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_message_template: Option<MessageTemplateType>,
    /// <p><p>The user account expiration limit, in days, after which the account is no longer usable. To reset the account after that time limit, you must call <code>AdminCreateUser</code> again, specifying <code>&quot;RESEND&quot;</code> for the <code>MessageAction</code> parameter. The default value for this parameter is 7. </p> <note> <p>If you set a value for <code>TemporaryPasswordValidityDays</code> in <code>PasswordPolicy</code>, that value will be used and <code>UnusedAccountValidityDays</code> will be deprecated for that user pool. </p> </note></p>
    #[serde(rename = "UnusedAccountValidityDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unused_account_validity_days: Option<i64>,
}

/// <p>Represents the request to create a user in the specified user pool.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AdminCreateUserRequest {
    /// <p><p>A map of custom key-value pairs that you can provide as input for any custom workflows that this action triggers. </p> <p>You create custom workflows by assigning AWS Lambda functions to user pool triggers. When you use the AdminCreateUser API action, Amazon Cognito invokes the function that is assigned to the <i>pre sign-up</i> trigger. When Amazon Cognito invokes this function, it passes a JSON payload, which the function receives as input. This payload contains a <code>clientMetadata</code> attribute, which provides the data that you assigned to the ClientMetadata parameter in your AdminCreateUser request. In your function code in AWS Lambda, you can process the <code>clientMetadata</code> value to enhance your workflow for your specific needs.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools-working-with-aws-lambda-triggers.html">Customizing User Pool Workflows with Lambda Triggers</a> in the <i>Amazon Cognito Developer Guide</i>.</p> <note> <p>Take the following limitations into consideration when you use the ClientMetadata parameter:</p> <ul> <li> <p>Amazon Cognito does not store the ClientMetadata value. This data is available only to AWS Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration does not include triggers, the ClientMetadata parameter serves no purpose.</p> </li> <li> <p>Amazon Cognito does not validate the ClientMetadata value.</p> </li> <li> <p>Amazon Cognito does not encrypt the the ClientMetadata value, so don&#39;t use it to provide sensitive information.</p> </li> </ul> </note></p>
    #[serde(rename = "ClientMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_metadata: Option<::std::collections::HashMap<String, String>>,
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AdminCreateUserResponse {
    /// <p>The newly created user.</p>
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserType>,
}

/// <p>Represents the request to delete user attributes as an administrator.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AdminDeleteUserAttributesResponse {}

/// <p>Represents the request to delete a user as an administrator.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AdminDeleteUserRequest {
    /// <p>The user pool ID for the user pool where you want to delete the user.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
    /// <p>The user name of the user you wish to delete.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AdminDisableProviderForUserRequest {
    /// <p>The user to be disabled.</p>
    #[serde(rename = "User")]
    pub user: ProviderUserIdentifierType,
    /// <p>The user pool ID for the user pool.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AdminDisableProviderForUserResponse {}

/// <p>Represents the request to disable any user as an administrator.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AdminDisableUserResponse {}

/// <p>Represents the request that enables the user as an administrator.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AdminEnableUserResponse {}

/// <p>Sends the forgot device request, as an administrator.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AdminGetDeviceResponse {
    /// <p>The device.</p>
    #[serde(rename = "Device")]
    pub device: DeviceType,
}

/// <p>Represents the request to get the specified user as an administrator.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AdminGetUserResponse {
    /// <p>Indicates that the status is enabled.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p> <i>This response parameter is no longer supported.</i> It provides information only about SMS MFA configurations. It doesn't provide information about TOTP software token MFA configurations. To look up information about either type of MFA configuration, use the <a>AdminGetUserResponse$UserMFASettingList</a> response instead.</p>
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
    /// <p>The MFA options that are enabled for the user. The possible values in this list are <code>SMS_MFA</code> and <code>SOFTWARE_TOKEN_MFA</code>.</p>
    #[serde(rename = "UserMFASettingList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_mfa_setting_list: Option<Vec<String>>,
    /// <p><p>The user status. Can be one of the following:</p> <ul> <li> <p>UNCONFIRMED - User has been created but not confirmed.</p> </li> <li> <p>CONFIRMED - User has been confirmed.</p> </li> <li> <p>ARCHIVED - User is no longer active.</p> </li> <li> <p>COMPROMISED - User is disabled due to a potential security threat.</p> </li> <li> <p>UNKNOWN - User status is not known.</p> </li> <li> <p>RESET<em>REQUIRED - User is confirmed, but the user must request a code and reset his or her password before he or she can sign in.</p> </li> <li> <p>FORCE</em>CHANGE_PASSWORD - The user is confirmed and the user can sign in using a temporary password, but on first sign-in, the user must change his or her password to a new value before doing anything else. </p> </li> </ul></p>
    #[serde(rename = "UserStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_status: Option<String>,
    /// <p>The user name of the user about whom you are receiving information.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

/// <p>Initiates the authorization request, as an administrator.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AdminInitiateAuthRequest {
    /// <p>The analytics metadata for collecting Amazon Pinpoint metrics for <code>AdminInitiateAuth</code> calls.</p>
    #[serde(rename = "AnalyticsMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_metadata: Option<AnalyticsMetadataType>,
    /// <p><p>The authentication flow for this call to execute. The API action will depend on this value. For example:</p> <ul> <li> <p> <code>REFRESH<em>TOKEN</em>AUTH</code> will take in a valid refresh token and return new tokens.</p> </li> <li> <p> <code>USER<em>SRP</em>AUTH</code> will take in <code>USERNAME</code> and <code>SRP<em>A</code> and return the SRP variables to be used for next challenge execution.</p> </li> <li> <p> <code>USER</em>PASSWORD<em>AUTH</code> will take in <code>USERNAME</code> and <code>PASSWORD</code> and return the next challenge or tokens.</p> </li> </ul> <p>Valid values include:</p> <ul> <li> <p> <code>USER</em>SRP<em>AUTH</code>: Authentication flow for the Secure Remote Password (SRP) protocol.</p> </li> <li> <p> <code>REFRESH</em>TOKEN<em>AUTH</code>/<code>REFRESH</em>TOKEN</code>: Authentication flow for refreshing the access token and ID token by supplying a valid refresh token.</p> </li> <li> <p> <code>CUSTOM<em>AUTH</code>: Custom authentication flow.</p> </li> <li> <p> <code>ADMIN</em>NO<em>SRP</em>AUTH</code>: Non-SRP authentication flow; you can pass in the USERNAME and PASSWORD directly if the flow is enabled for calling the app client.</p> </li> <li> <p> <code>USER<em>PASSWORD</em>AUTH</code>: Non-SRP authentication flow; USERNAME and PASSWORD are passed directly. If a user migration Lambda trigger is set, this flow will invoke the user migration Lambda if the USERNAME is not found in the user pool. </p> </li> <li> <p> <code>ADMIN<em>USER</em>PASSWORD<em>AUTH</code>: Admin-based user password authentication. This replaces the <code>ADMIN</em>NO<em>SRP</em>AUTH</code> authentication flow. In this flow, Cognito receives the password in the request instead of using the SRP process to verify passwords.</p> </li> </ul></p>
    #[serde(rename = "AuthFlow")]
    pub auth_flow: String,
    /// <p><p>The authentication parameters. These are inputs corresponding to the <code>AuthFlow</code> that you are invoking. The required values depend on the value of <code>AuthFlow</code>:</p> <ul> <li> <p>For <code>USER<em>SRP</em>AUTH</code>: <code>USERNAME</code> (required), <code>SRP<em>A</code> (required), <code>SECRET</em>HASH</code> (required if the app client is configured with a client secret), <code>DEVICE<em>KEY</code> </p> </li> <li> <p>For <code>REFRESH</em>TOKEN<em>AUTH/REFRESH</em>TOKEN</code>: <code>REFRESH<em>TOKEN</code> (required), <code>SECRET</em>HASH</code> (required if the app client is configured with a client secret), <code>DEVICE<em>KEY</code> </p> </li> <li> <p>For <code>ADMIN</em>NO<em>SRP</em>AUTH</code>: <code>USERNAME</code> (required), <code>SECRET<em>HASH</code> (if app client is configured with client secret), <code>PASSWORD</code> (required), <code>DEVICE</em>KEY</code> </p> </li> <li> <p>For <code>CUSTOM<em>AUTH</code>: <code>USERNAME</code> (required), <code>SECRET</em>HASH</code> (if app client is configured with client secret), <code>DEVICE_KEY</code> </p> </li> </ul></p>
    #[serde(rename = "AuthParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The app client ID.</p>
    #[serde(rename = "ClientId")]
    pub client_id: String,
    /// <p><p>A map of custom key-value pairs that you can provide as input for certain custom workflows that this action triggers.</p> <p>You create custom workflows by assigning AWS Lambda functions to user pool triggers. When you use the AdminInitiateAuth API action, Amazon Cognito invokes the AWS Lambda functions that are specified for various triggers. The ClientMetadata value is passed as input to the functions for only the following triggers:</p> <ul> <li> <p>Pre signup</p> </li> <li> <p>Pre authentication</p> </li> <li> <p>User migration</p> </li> </ul> <p>When Amazon Cognito invokes the functions for these triggers, it passes a JSON payload, which the function receives as input. This payload contains a <code>validationData</code> attribute, which provides the data that you assigned to the ClientMetadata parameter in your AdminInitiateAuth request. In your function code in AWS Lambda, you can process the <code>validationData</code> value to enhance your workflow for your specific needs.</p> <p>When you use the AdminInitiateAuth API action, Amazon Cognito also invokes the functions for the following triggers, but it does not provide the ClientMetadata value as input:</p> <ul> <li> <p>Post authentication</p> </li> <li> <p>Custom message</p> </li> <li> <p>Pre token generation</p> </li> <li> <p>Create auth challenge</p> </li> <li> <p>Define auth challenge</p> </li> <li> <p>Verify auth challenge</p> </li> </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools-working-with-aws-lambda-triggers.html">Customizing User Pool Workflows with Lambda Triggers</a> in the <i>Amazon Cognito Developer Guide</i>.</p> <note> <p>Take the following limitations into consideration when you use the ClientMetadata parameter:</p> <ul> <li> <p>Amazon Cognito does not store the ClientMetadata value. This data is available only to AWS Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration does not include triggers, the ClientMetadata parameter serves no purpose.</p> </li> <li> <p>Amazon Cognito does not validate the ClientMetadata value.</p> </li> <li> <p>Amazon Cognito does not encrypt the the ClientMetadata value, so don&#39;t use it to provide sensitive information.</p> </li> </ul> </note></p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AdminLinkProviderForUserResponse {}

/// <p>Represents the request to list devices, as an administrator.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AdminResetUserPasswordRequest {
    /// <p><p>A map of custom key-value pairs that you can provide as input for any custom workflows that this action triggers. </p> <p>You create custom workflows by assigning AWS Lambda functions to user pool triggers. When you use the AdminResetUserPassword API action, Amazon Cognito invokes the function that is assigned to the <i>custom message</i> trigger. When Amazon Cognito invokes this function, it passes a JSON payload, which the function receives as input. This payload contains a <code>clientMetadata</code> attribute, which provides the data that you assigned to the ClientMetadata parameter in your AdminResetUserPassword request. In your function code in AWS Lambda, you can process the <code>clientMetadata</code> value to enhance your workflow for your specific needs.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools-working-with-aws-lambda-triggers.html">Customizing User Pool Workflows with Lambda Triggers</a> in the <i>Amazon Cognito Developer Guide</i>.</p> <note> <p>Take the following limitations into consideration when you use the ClientMetadata parameter:</p> <ul> <li> <p>Amazon Cognito does not store the ClientMetadata value. This data is available only to AWS Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration does not include triggers, the ClientMetadata parameter serves no purpose.</p> </li> <li> <p>Amazon Cognito does not validate the ClientMetadata value.</p> </li> <li> <p>Amazon Cognito does not encrypt the the ClientMetadata value, so don&#39;t use it to provide sensitive information.</p> </li> </ul> </note></p>
    #[serde(rename = "ClientMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_metadata: Option<::std::collections::HashMap<String, String>>,
    /// <p>The user pool ID for the user pool where you want to reset the user's password.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
    /// <p>The user name of the user whose password you wish to reset.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

/// <p>Represents the response from the server to reset a user password as an administrator.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AdminResetUserPasswordResponse {}

/// <p>The request to respond to the authentication challenge, as an administrator.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p><p>A map of custom key-value pairs that you can provide as input for any custom workflows that this action triggers. </p> <p>You create custom workflows by assigning AWS Lambda functions to user pool triggers. When you use the AdminRespondToAuthChallenge API action, Amazon Cognito invokes any functions that are assigned to the following triggers: <i>pre sign-up</i>, <i>custom message</i>, <i>post authentication</i>, <i>user migration</i>, <i>pre token generation</i>, <i>define auth challenge</i>, <i>create auth challenge</i>, and <i>verify auth challenge response</i>. When Amazon Cognito invokes any of these functions, it passes a JSON payload, which the function receives as input. This payload contains a <code>clientMetadata</code> attribute, which provides the data that you assigned to the ClientMetadata parameter in your AdminRespondToAuthChallenge request. In your function code in AWS Lambda, you can process the <code>clientMetadata</code> value to enhance your workflow for your specific needs.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools-working-with-aws-lambda-triggers.html">Customizing User Pool Workflows with Lambda Triggers</a> in the <i>Amazon Cognito Developer Guide</i>.</p> <note> <p>Take the following limitations into consideration when you use the ClientMetadata parameter:</p> <ul> <li> <p>Amazon Cognito does not store the ClientMetadata value. This data is available only to AWS Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration does not include triggers, the ClientMetadata parameter serves no purpose.</p> </li> <li> <p>Amazon Cognito does not validate the ClientMetadata value.</p> </li> <li> <p>Amazon Cognito does not encrypt the the ClientMetadata value, so don&#39;t use it to provide sensitive information.</p> </li> </ul> </note></p>
    #[serde(rename = "ClientMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_metadata: Option<::std::collections::HashMap<String, String>>,
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AdminSetUserMFAPreferenceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AdminSetUserPasswordRequest {
    /// <p>The password for the user.</p>
    #[serde(rename = "Password")]
    pub password: String,
    /// <p> <code>True</code> if the password is permanent, <code>False</code> if it is temporary.</p>
    #[serde(rename = "Permanent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permanent: Option<bool>,
    /// <p>The user pool ID for the user pool where you want to set the user's password.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
    /// <p>The user name of the user whose password you wish to set.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AdminSetUserPasswordResponse {}

/// <p>You can use this parameter to set an MFA configuration that uses the SMS delivery medium.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AdminSetUserSettingsRequest {
    /// <p>You can use this parameter only to set an SMS configuration that uses SMS for delivery.</p>
    #[serde(rename = "MFAOptions")]
    pub mfa_options: Vec<MFAOptionType>,
    /// <p>The ID of the user pool that contains the user that you are setting options for.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
    /// <p>The user name of the user that you are setting options for.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

/// <p>Represents the response from the server to set user settings as an administrator.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AdminSetUserSettingsResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AdminUpdateAuthEventFeedbackResponse {}

/// <p>The request to update the device status, as an administrator.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AdminUpdateDeviceStatusResponse {}

/// <p>Represents the request to update the user's attributes as an administrator.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AdminUpdateUserAttributesRequest {
    /// <p><p>A map of custom key-value pairs that you can provide as input for any custom workflows that this action triggers. </p> <p>You create custom workflows by assigning AWS Lambda functions to user pool triggers. When you use the AdminUpdateUserAttributes API action, Amazon Cognito invokes the function that is assigned to the <i>custom message</i> trigger. When Amazon Cognito invokes this function, it passes a JSON payload, which the function receives as input. This payload contains a <code>clientMetadata</code> attribute, which provides the data that you assigned to the ClientMetadata parameter in your AdminUpdateUserAttributes request. In your function code in AWS Lambda, you can process the <code>clientMetadata</code> value to enhance your workflow for your specific needs.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools-working-with-aws-lambda-triggers.html">Customizing User Pool Workflows with Lambda Triggers</a> in the <i>Amazon Cognito Developer Guide</i>.</p> <note> <p>Take the following limitations into consideration when you use the ClientMetadata parameter:</p> <ul> <li> <p>Amazon Cognito does not store the ClientMetadata value. This data is available only to AWS Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration does not include triggers, the ClientMetadata parameter serves no purpose.</p> </li> <li> <p>Amazon Cognito does not validate the ClientMetadata value.</p> </li> <li> <p>Amazon Cognito does not encrypt the the ClientMetadata value, so don&#39;t use it to provide sensitive information.</p> </li> </ul> </note></p>
    #[serde(rename = "ClientMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_metadata: Option<::std::collections::HashMap<String, String>>,
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AdminUpdateUserAttributesResponse {}

/// <p>The request to sign out of all devices, as an administrator.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AnalyticsMetadataType {
    /// <p>The endpoint ID.</p>
    #[serde(rename = "AnalyticsEndpointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_endpoint_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ChangePasswordResponse {}

/// <p>The code delivery details being returned from the server.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConfirmDeviceResponse {
    /// <p>Indicates whether the user confirmation is necessary to confirm the device response.</p>
    #[serde(rename = "UserConfirmationNecessary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_confirmation_necessary: Option<bool>,
}

/// <p>The request representing the confirmation for a password reset.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ConfirmForgotPasswordRequest {
    /// <p>The Amazon Pinpoint analytics metadata for collecting metrics for <code>ConfirmForgotPassword</code> calls.</p>
    #[serde(rename = "AnalyticsMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_metadata: Option<AnalyticsMetadataType>,
    /// <p>The app client ID of the app associated with the user pool.</p>
    #[serde(rename = "ClientId")]
    pub client_id: String,
    /// <p><p>A map of custom key-value pairs that you can provide as input for any custom workflows that this action triggers. </p> <p>You create custom workflows by assigning AWS Lambda functions to user pool triggers. When you use the ConfirmForgotPassword API action, Amazon Cognito invokes the function that is assigned to the <i>post confirmation</i> trigger. When Amazon Cognito invokes this function, it passes a JSON payload, which the function receives as input. This payload contains a <code>clientMetadata</code> attribute, which provides the data that you assigned to the ClientMetadata parameter in your ConfirmForgotPassword request. In your function code in AWS Lambda, you can process the <code>clientMetadata</code> value to enhance your workflow for your specific needs.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools-working-with-aws-lambda-triggers.html">Customizing User Pool Workflows with Lambda Triggers</a> in the <i>Amazon Cognito Developer Guide</i>.</p> <note> <p>Take the following limitations into consideration when you use the ClientMetadata parameter:</p> <ul> <li> <p>Amazon Cognito does not store the ClientMetadata value. This data is available only to AWS Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration does not include triggers, the ClientMetadata parameter serves no purpose.</p> </li> <li> <p>Amazon Cognito does not validate the ClientMetadata value.</p> </li> <li> <p>Amazon Cognito does not encrypt the the ClientMetadata value, so don&#39;t use it to provide sensitive information.</p> </li> </ul> </note></p>
    #[serde(rename = "ClientMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_metadata: Option<::std::collections::HashMap<String, String>>,
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConfirmForgotPasswordResponse {}

/// <p>Represents the request to confirm registration of a user.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ConfirmSignUpRequest {
    /// <p>The Amazon Pinpoint analytics metadata for collecting metrics for <code>ConfirmSignUp</code> calls.</p>
    #[serde(rename = "AnalyticsMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_metadata: Option<AnalyticsMetadataType>,
    /// <p>The ID of the app client associated with the user pool.</p>
    #[serde(rename = "ClientId")]
    pub client_id: String,
    /// <p><p>A map of custom key-value pairs that you can provide as input for any custom workflows that this action triggers. </p> <p>You create custom workflows by assigning AWS Lambda functions to user pool triggers. When you use the ConfirmSignUp API action, Amazon Cognito invokes the function that is assigned to the <i>post confirmation</i> trigger. When Amazon Cognito invokes this function, it passes a JSON payload, which the function receives as input. This payload contains a <code>clientMetadata</code> attribute, which provides the data that you assigned to the ClientMetadata parameter in your ConfirmSignUp request. In your function code in AWS Lambda, you can process the <code>clientMetadata</code> value to enhance your workflow for your specific needs.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools-working-with-aws-lambda-triggers.html">Customizing User Pool Workflows with Lambda Triggers</a> in the <i>Amazon Cognito Developer Guide</i>.</p> <note> <p>Take the following limitations into consideration when you use the ClientMetadata parameter:</p> <ul> <li> <p>Amazon Cognito does not store the ClientMetadata value. This data is available only to AWS Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration does not include triggers, the ClientMetadata parameter serves no purpose.</p> </li> <li> <p>Amazon Cognito does not validate the ClientMetadata value.</p> </li> <li> <p>Amazon Cognito does not encrypt the the ClientMetadata value, so don&#39;t use it to provide sensitive information.</p> </li> </ul> </note></p>
    #[serde(rename = "ClientMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_metadata: Option<::std::collections::HashMap<String, String>>,
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConfirmSignUpResponse {}

/// <p>Contextual user data type used for evaluating the risk of an unexpected event by Amazon Cognito advanced security.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateGroupResponse {
    /// <p>The group object for the group.</p>
    #[serde(rename = "Group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<GroupType>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateIdentityProviderResponse {
    /// <p>The newly created identity provider object.</p>
    #[serde(rename = "IdentityProvider")]
    pub identity_provider: IdentityProviderType,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateResourceServerResponse {
    /// <p>The newly created resource server.</p>
    #[serde(rename = "ResourceServer")]
    pub resource_server: ResourceServerType,
}

/// <p>Represents the request to create the user import job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateUserImportJobResponse {
    /// <p>The job object that represents the user import job.</p>
    #[serde(rename = "UserImportJob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_import_job: Option<UserImportJobType>,
}

/// <p>Represents the request to create a user pool client.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateUserPoolClientRequest {
    /// <p>Set to <code>code</code> to initiate a code grant flow, which provides an authorization code as the response. This code can be exchanged for access tokens with the token endpoint.</p> <p>Set to <code>token</code> to specify that the client should get the access token (and, optionally, ID token, based on scopes) directly.</p>
    #[serde(rename = "AllowedOAuthFlows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_o_auth_flows: Option<Vec<String>>,
    /// <p>Set to <code>True</code> if the client is allowed to follow the OAuth protocol when interacting with Cognito user pools.</p>
    #[serde(rename = "AllowedOAuthFlowsUserPoolClient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_o_auth_flows_user_pool_client: Option<bool>,
    /// <p>A list of allowed <code>OAuth</code> scopes. Currently supported values are <code>"phone"</code>, <code>"email"</code>, <code>"openid"</code>, and <code>"Cognito"</code>. In addition to these values, custom scopes created in Resource Servers are also supported.</p>
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
    /// <p><p>The authentication flows that are supported by the user pool clients. Flow names without the <code>ALLOW<em></code> prefix are deprecated in favor of new names with the <code>ALLOW</em></code> prefix. Note that values with <code>ALLOW<em></code> prefix cannot be used along with values without <code>ALLOW</em></code> prefix.</p> <p>Valid values include:</p> <ul> <li> <p> <code>ALLOW<em>ADMIN</em>USER<em>PASSWORD</em>AUTH</code>: Enable admin based user password authentication flow <code>ADMIN<em>USER</em>PASSWORD<em>AUTH</code>. This setting replaces the <code>ADMIN</em>NO<em>SRP</em>AUTH</code> setting. With this authentication flow, Cognito receives the password in the request instead of using the SRP (Secure Remote Password protocol) protocol to verify passwords.</p> </li> <li> <p> <code>ALLOW<em>CUSTOM</em>AUTH</code>: Enable Lambda trigger based authentication.</p> </li> <li> <p> <code>ALLOW<em>USER</em>PASSWORD<em>AUTH</code>: Enable user password-based authentication. In this flow, Cognito receives the password in the request instead of using the SRP protocol to verify passwords.</p> </li> <li> <p> <code>ALLOW</em>USER<em>SRP</em>AUTH</code>: Enable SRP based authentication.</p> </li> <li> <p> <code>ALLOW<em>REFRESH</em>TOKEN_AUTH</code>: Enable authflow to refresh tokens.</p> </li> </ul></p>
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
    /// <p><p>Use this setting to choose which errors and responses are returned by Cognito APIs during authentication, account confirmation, and password recovery when the user does not exist in the user pool. When set to <code>ENABLED</code> and the user does not exist, authentication returns an error indicating either the username or password was incorrect, and account confirmation and password recovery return a response indicating a code was sent to a simulated destination. When set to <code>LEGACY</code>, those APIs will return a <code>UserNotFoundException</code> exception if the user does not exist in the user pool.</p> <p>Valid values include:</p> <ul> <li> <p> <code>ENABLED</code> - This prevents user existence-related errors.</p> </li> <li> <p> <code>LEGACY</code> - This represents the old behavior of Cognito where user existence related errors are not prevented.</p> </li> </ul> <p>This setting affects the behavior of following APIs:</p> <ul> <li> <p> <a>AdminInitiateAuth</a> </p> </li> <li> <p> <a>AdminRespondToAuthChallenge</a> </p> </li> <li> <p> <a>InitiateAuth</a> </p> </li> <li> <p> <a>RespondToAuthChallenge</a> </p> </li> <li> <p> <a>ForgotPassword</a> </p> </li> <li> <p> <a>ConfirmForgotPassword</a> </p> </li> <li> <p> <a>ConfirmSignUp</a> </p> </li> <li> <p> <a>ResendConfirmationCode</a> </p> </li> </ul> <note> <p>After January 1st 2020, the value of <code>PreventUserExistenceErrors</code> will default to <code>ENABLED</code> for newly created user pool clients if no value is provided.</p> </note></p>
    #[serde(rename = "PreventUserExistenceErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prevent_user_existence_errors: Option<String>,
    /// <p>The read attributes.</p>
    #[serde(rename = "ReadAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_attributes: Option<Vec<String>>,
    /// <p>The time limit, in days, after which the refresh token is no longer valid and cannot be used.</p>
    #[serde(rename = "RefreshTokenValidity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token_validity: Option<i64>,
    /// <p>A list of provider names for the identity providers that are supported on this client. The following are supported: <code>COGNITO</code>, <code>Facebook</code>, <code>Google</code> and <code>LoginWithAmazon</code>.</p>
    #[serde(rename = "SupportedIdentityProviders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_identity_providers: Option<Vec<String>>,
    /// <p>The user pool ID for the user pool where you want to create a user pool client.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
    /// <p>The user pool attributes that the app client can write to.</p> <p>If your app client allows users to sign in through an identity provider, this array must include all attributes that are mapped to identity provider attributes. Amazon Cognito updates mapped attributes when users sign in to your application through an identity provider. If your app client lacks write access to a mapped attribute, Amazon Cognito throws an error when it attempts to update the attribute. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-specifying-attribute-mapping.html">Specifying Identity Provider Attribute Mappings for Your User Pool</a>.</p>
    #[serde(rename = "WriteAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_attributes: Option<Vec<String>>,
}

/// <p>Represents the response from the server to create a user pool client.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateUserPoolClientResponse {
    /// <p>The user pool client that was just created.</p>
    #[serde(rename = "UserPoolClient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_client: Option<UserPoolClientType>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateUserPoolDomainRequest {
    /// <p>The configuration for a custom domain that hosts the sign-up and sign-in webpages for your application.</p> <p>Provide this parameter only if you want to use a custom domain for your user pool. Otherwise, you can exclude this parameter and use the Amazon Cognito hosted domain instead.</p> <p>For more information about the hosted domain and custom domains, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-assign-domain.html">Configuring a User Pool Domain</a>.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateUserPoolDomainResponse {
    /// <p>The Amazon CloudFront endpoint that you use as the target of the alias that you set up with your Domain Name Service (DNS) provider.</p>
    #[serde(rename = "CloudFrontDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_front_domain: Option<String>,
}

/// <p>Represents the request to create a user pool.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateUserPoolRequest {
    /// <p><p>Use this setting to define which verified available method a user can use to recover their password when they call <code>ForgotPassword</code>. It allows you to define a preferred method when a user has more than one method available. With this setting, SMS does not qualify for a valid password recovery mechanism if the user also has SMS MFA enabled. In the absence of this setting, Cognito uses the legacy behavior to determine the recovery method where SMS is preferred over email.</p> <note> <p>Starting February 1, 2020, the value of <code>AccountRecoverySetting</code> will default to <code>verified<em>email</code> first and <code>verified</em>phone_number</code> as the second option for newly created user pools if no value is provided.</p> </note></p>
    #[serde(rename = "AccountRecoverySetting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_recovery_setting: Option<AccountRecoverySettingType>,
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
    /// <p>The tag keys and values to assign to the user pool. A tag is a label that you can use to categorize and manage user pools in different ways, such as by purpose, owner, environment, or other criteria.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteGroupRequest {
    /// <p>The name of the group.</p>
    #[serde(rename = "GroupName")]
    pub group_name: String,
    /// <p>The user pool ID for the user pool.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteIdentityProviderRequest {
    /// <p>The identity provider name.</p>
    #[serde(rename = "ProviderName")]
    pub provider_name: String,
    /// <p>The user pool ID.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteUserAttributesResponse {}

/// <p>Represents the request to delete a user pool client.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteUserPoolClientRequest {
    /// <p>The app client ID of the app associated with the user pool.</p>
    #[serde(rename = "ClientId")]
    pub client_id: String,
    /// <p>The user pool ID for the user pool where you want to delete the client.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteUserPoolDomainRequest {
    /// <p>The domain string.</p>
    #[serde(rename = "Domain")]
    pub domain: String,
    /// <p>The user pool ID.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteUserPoolDomainResponse {}

/// <p>Represents the request to delete a user pool.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteUserPoolRequest {
    /// <p>The user pool ID for the user pool you want to delete.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

/// <p>Represents the request to delete a user.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteUserRequest {
    /// <p>The access token from a request to delete a user.</p>
    #[serde(rename = "AccessToken")]
    pub access_token: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeIdentityProviderRequest {
    /// <p>The identity provider name.</p>
    #[serde(rename = "ProviderName")]
    pub provider_name: String,
    /// <p>The user pool ID.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeIdentityProviderResponse {
    /// <p>The identity provider that was deleted.</p>
    #[serde(rename = "IdentityProvider")]
    pub identity_provider: IdentityProviderType,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeResourceServerRequest {
    /// <p>The identifier for the resource server</p>
    #[serde(rename = "Identifier")]
    pub identifier: String,
    /// <p>The user pool ID for the user pool that hosts the resource server.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeResourceServerResponse {
    /// <p>The resource server.</p>
    #[serde(rename = "ResourceServer")]
    pub resource_server: ResourceServerType,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeRiskConfigurationResponse {
    /// <p>The risk configuration.</p>
    #[serde(rename = "RiskConfiguration")]
    pub risk_configuration: RiskConfigurationType,
}

/// <p>Represents the request to describe the user import job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeUserImportJobResponse {
    /// <p>The job object that represents the user import job.</p>
    #[serde(rename = "UserImportJob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_import_job: Option<UserImportJobType>,
}

/// <p>Represents the request to describe a user pool client.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeUserPoolClientResponse {
    /// <p>The user pool client from a server response to describe the user pool client.</p>
    #[serde(rename = "UserPoolClient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_client: Option<UserPoolClientType>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeUserPoolDomainRequest {
    /// <p>The domain string.</p>
    #[serde(rename = "Domain")]
    pub domain: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeUserPoolDomainResponse {
    /// <p>A domain description object containing information about the domain.</p>
    #[serde(rename = "DomainDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_description: Option<DomainDescriptionType>,
}

/// <p>Represents the request to describe the user pool.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeUserPoolRequest {
    /// <p>The user pool ID for the user pool you want to describe.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

/// <p>Represents the response to describe the user pool.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DomainDescriptionType {
    /// <p>The AWS account ID for the user pool owner.</p>
    #[serde(rename = "AWSAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,
    /// <p>The ARN of the CloudFront distribution.</p>
    #[serde(rename = "CloudFrontDistribution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_front_distribution: Option<String>,
    /// <p>The configuration for a custom domain that hosts the sign-up and sign-in webpages for your application.</p>
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
    /// <p><p>The set of configuration rules that can be applied to emails sent using Amazon SES. A configuration set is applied to an email by including a reference to the configuration set in the headers of the email. Once applied, all of the rules in that configuration set are applied to the email. Configuration sets can be used to apply the following types of rules to emails: </p> <ul> <li> <p>Event publishing – Amazon SES can track the number of send, delivery, open, click, bounce, and complaint events for each email sent. Use event publishing to send information about these events to other AWS services such as SNS and CloudWatch.</p> </li> <li> <p>IP pool management – When leasing dedicated IP addresses with Amazon SES, you can create groups of IP addresses, called dedicated IP pools. You can then associate the dedicated IP pools with configuration sets.</p> </li> </ul></p>
    #[serde(rename = "ConfigurationSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_set: Option<String>,
    /// <p><p>Specifies whether Amazon Cognito emails your users by using its built-in email functionality or your Amazon SES email configuration. Specify one of the following values:</p> <dl> <dt>COGNITO_DEFAULT</dt> <dd> <p>When Amazon Cognito emails your users, it uses its built-in email functionality. When you use the default option, Amazon Cognito allows only a limited number of emails each day for your user pool. For typical production environments, the default email limit is below the required delivery volume. To achieve a higher delivery volume, specify DEVELOPER to use your Amazon SES email configuration.</p> <p>To look up the email delivery limit for the default option, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/limits.html">Limits in Amazon Cognito</a> in the <i>Amazon Cognito Developer Guide</i>.</p> <p>The default FROM address is no-reply@verificationemail.com. To customize the FROM address, provide the ARN of an Amazon SES verified email address for the <code>SourceArn</code> parameter.</p> </dd> <dt>DEVELOPER</dt> <dd> <p>When Amazon Cognito emails your users, it uses your Amazon SES configuration. Amazon Cognito calls Amazon SES on your behalf to send email from your verified email address. When you use this option, the email delivery limits are the same limits that apply to your Amazon SES verified email address in your AWS account.</p> <p>If you use this option, you must provide the ARN of an Amazon SES verified email address for the <code>SourceArn</code> parameter.</p> <p>Before Amazon Cognito can email your users, it requires additional permissions to call Amazon SES on your behalf. When you update your user pool with this option, Amazon Cognito creates a <i>service-linked role</i>, which is a type of IAM role, in your AWS account. This role contains the permissions that allow Amazon Cognito to access Amazon SES and send email messages with your address. For more information about the service-linked role that Amazon Cognito creates, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/using-service-linked-roles.html">Using Service-Linked Roles for Amazon Cognito</a> in the <i>Amazon Cognito Developer Guide</i>.</p> </dd> </dl></p>
    #[serde(rename = "EmailSendingAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_sending_account: Option<String>,
    /// <p>Identifies either the sender’s email address or the sender’s name with their email address. For example, <code>testuser@example.com</code> or <code>Test User &lt;testuser@example.com&gt;</code>. This address will appear before the body of the email.</p>
    #[serde(rename = "From")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// <p>The destination to which the receiver of the email should reply to.</p>
    #[serde(rename = "ReplyToEmailAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_email_address: Option<String>,
    /// <p><p>The Amazon Resource Name (ARN) of a verified email address in Amazon SES. This email address is used in one of the following ways, depending on the value that you specify for the <code>EmailSendingAccount</code> parameter:</p> <ul> <li> <p>If you specify <code>COGNITO_DEFAULT</code>, Amazon Cognito uses this address as the custom FROM address when it emails your users by using its built-in email account.</p> </li> <li> <p>If you specify <code>DEVELOPER</code>, Amazon Cognito emails your users with this address by calling Amazon SES on your behalf.</p> </li> </ul></p>
    #[serde(rename = "SourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
}

/// <p>Specifies the user context data captured at the time of an event request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ForgotPasswordRequest {
    /// <p>The Amazon Pinpoint analytics metadata for collecting metrics for <code>ForgotPassword</code> calls.</p>
    #[serde(rename = "AnalyticsMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_metadata: Option<AnalyticsMetadataType>,
    /// <p>The ID of the client associated with the user pool.</p>
    #[serde(rename = "ClientId")]
    pub client_id: String,
    /// <p><p>A map of custom key-value pairs that you can provide as input for any custom workflows that this action triggers. </p> <p>You create custom workflows by assigning AWS Lambda functions to user pool triggers. When you use the ForgotPassword API action, Amazon Cognito invokes any functions that are assigned to the following triggers: <i>pre sign-up</i>, <i>custom message</i>, and <i>user migration</i>. When Amazon Cognito invokes any of these functions, it passes a JSON payload, which the function receives as input. This payload contains a <code>clientMetadata</code> attribute, which provides the data that you assigned to the ClientMetadata parameter in your ForgotPassword request. In your function code in AWS Lambda, you can process the <code>clientMetadata</code> value to enhance your workflow for your specific needs.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools-working-with-aws-lambda-triggers.html">Customizing User Pool Workflows with Lambda Triggers</a> in the <i>Amazon Cognito Developer Guide</i>.</p> <note> <p>Take the following limitations into consideration when you use the ClientMetadata parameter:</p> <ul> <li> <p>Amazon Cognito does not store the ClientMetadata value. This data is available only to AWS Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration does not include triggers, the ClientMetadata parameter serves no purpose.</p> </li> <li> <p>Amazon Cognito does not validate the ClientMetadata value.</p> </li> <li> <p>Amazon Cognito does not encrypt the the ClientMetadata value, so don&#39;t use it to provide sensitive information.</p> </li> </ul> </note></p>
    #[serde(rename = "ClientMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_metadata: Option<::std::collections::HashMap<String, String>>,
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ForgotPasswordResponse {
    /// <p>The code delivery details returned by the server in response to the request to reset a password.</p>
    #[serde(rename = "CodeDeliveryDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_delivery_details: Option<CodeDeliveryDetailsType>,
}

/// <p>Represents the request to get the header information for the .csv file for the user import job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetCSVHeaderRequest {
    /// <p>The user pool ID for the user pool that the users are to be imported into.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

/// <p>Represents the response from the server to the request to get the header information for the .csv file for the user import job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDeviceResponse {
    /// <p>The device.</p>
    #[serde(rename = "Device")]
    pub device: DeviceType,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetGroupRequest {
    /// <p>The name of the group.</p>
    #[serde(rename = "GroupName")]
    pub group_name: String,
    /// <p>The user pool ID for the user pool.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetGroupResponse {
    /// <p>The group object for the group.</p>
    #[serde(rename = "Group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<GroupType>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetIdentityProviderByIdentifierRequest {
    /// <p>The identity provider ID.</p>
    #[serde(rename = "IdpIdentifier")]
    pub idp_identifier: String,
    /// <p>The user pool ID.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetIdentityProviderByIdentifierResponse {
    /// <p>The identity provider object.</p>
    #[serde(rename = "IdentityProvider")]
    pub identity_provider: IdentityProviderType,
}

/// <p>Request to get a signing certificate from Cognito.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetSigningCertificateRequest {
    /// <p>The user pool ID.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

/// <p>Response from Cognito for a signing certificate request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetSigningCertificateResponse {
    /// <p>The signing certificate.</p>
    #[serde(rename = "Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetUICustomizationResponse {
    /// <p>The UI customization information.</p>
    #[serde(rename = "UICustomization")]
    pub ui_customization: UICustomizationType,
}

/// <p>Represents the request to get user attribute verification.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetUserAttributeVerificationCodeRequest {
    /// <p>The access token returned by the server response to get the user attribute verification code.</p>
    #[serde(rename = "AccessToken")]
    pub access_token: String,
    /// <p>The attribute name returned by the server response to get the user attribute verification code.</p>
    #[serde(rename = "AttributeName")]
    pub attribute_name: String,
    /// <p><p>A map of custom key-value pairs that you can provide as input for any custom workflows that this action triggers. </p> <p>You create custom workflows by assigning AWS Lambda functions to user pool triggers. When you use the GetUserAttributeVerificationCode API action, Amazon Cognito invokes the function that is assigned to the <i>custom message</i> trigger. When Amazon Cognito invokes this function, it passes a JSON payload, which the function receives as input. This payload contains a <code>clientMetadata</code> attribute, which provides the data that you assigned to the ClientMetadata parameter in your GetUserAttributeVerificationCode request. In your function code in AWS Lambda, you can process the <code>clientMetadata</code> value to enhance your workflow for your specific needs.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools-working-with-aws-lambda-triggers.html">Customizing User Pool Workflows with Lambda Triggers</a> in the <i>Amazon Cognito Developer Guide</i>.</p> <note> <p>Take the following limitations into consideration when you use the ClientMetadata parameter:</p> <ul> <li> <p>Amazon Cognito does not store the ClientMetadata value. This data is available only to AWS Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration does not include triggers, the ClientMetadata parameter serves no purpose.</p> </li> <li> <p>Amazon Cognito does not validate the ClientMetadata value.</p> </li> <li> <p>Amazon Cognito does not encrypt the the ClientMetadata value, so don&#39;t use it to provide sensitive information.</p> </li> </ul> </note></p>
    #[serde(rename = "ClientMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_metadata: Option<::std::collections::HashMap<String, String>>,
}

/// <p>The verification code response returned by the server response to get the user attribute verification code.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetUserAttributeVerificationCodeResponse {
    /// <p>The code delivery details returned by the server in response to the request to get the user attribute verification code.</p>
    #[serde(rename = "CodeDeliveryDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_delivery_details: Option<CodeDeliveryDetailsType>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetUserPoolMfaConfigRequest {
    /// <p>The user pool ID.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetUserPoolMfaConfigResponse {
    /// <p><p>The multi-factor (MFA) configuration. Valid values include:</p> <ul> <li> <p> <code>OFF</code> MFA will not be used for any users.</p> </li> <li> <p> <code>ON</code> MFA is required for all users to sign in.</p> </li> <li> <p> <code>OPTIONAL</code> MFA will be required only for individual users who have an MFA factor enabled.</p> </li> </ul></p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetUserRequest {
    /// <p>The access token returned by the server response to get information about the user.</p>
    #[serde(rename = "AccessToken")]
    pub access_token: String,
}

/// <p>Represents the response from the server from the request to get information about the user.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetUserResponse {
    /// <p> <i>This response parameter is no longer supported.</i> It provides information only about SMS MFA configurations. It doesn't provide information about TOTP software token MFA configurations. To look up information about either type of MFA configuration, use the use the <a>GetUserResponse$UserMFASettingList</a> response instead.</p>
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
    /// <p>The MFA options that are enabled for the user. The possible values in this list are <code>SMS_MFA</code> and <code>SOFTWARE_TOKEN_MFA</code>.</p>
    #[serde(rename = "UserMFASettingList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_mfa_setting_list: Option<Vec<String>>,
    /// <p>The user name of the user you wish to retrieve from the get user request.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

/// <p>Represents the request to sign out all devices.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GlobalSignOutRequest {
    /// <p>The access token.</p>
    #[serde(rename = "AccessToken")]
    pub access_token: String,
}

/// <p>The response to the request to sign out all devices.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GlobalSignOutResponse {}

/// <p>The group type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InitiateAuthRequest {
    /// <p>The Amazon Pinpoint analytics metadata for collecting metrics for <code>InitiateAuth</code> calls.</p>
    #[serde(rename = "AnalyticsMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_metadata: Option<AnalyticsMetadataType>,
    /// <p>The authentication flow for this call to execute. The API action will depend on this value. For example: </p> <ul> <li> <p> <code>REFRESH_TOKEN_AUTH</code> will take in a valid refresh token and return new tokens.</p> </li> <li> <p> <code>USER_SRP_AUTH</code> will take in <code>USERNAME</code> and <code>SRP_A</code> and return the SRP variables to be used for next challenge execution.</p> </li> <li> <p> <code>USER_PASSWORD_AUTH</code> will take in <code>USERNAME</code> and <code>PASSWORD</code> and return the next challenge or tokens.</p> </li> </ul> <p>Valid values include:</p> <ul> <li> <p> <code>USER_SRP_AUTH</code>: Authentication flow for the Secure Remote Password (SRP) protocol.</p> </li> <li> <p> <code>REFRESH_TOKEN_AUTH</code>/<code>REFRESH_TOKEN</code>: Authentication flow for refreshing the access token and ID token by supplying a valid refresh token.</p> </li> <li> <p> <code>CUSTOM_AUTH</code>: Custom authentication flow.</p> </li> <li> <p> <code>USER_PASSWORD_AUTH</code>: Non-SRP authentication flow; USERNAME and PASSWORD are passed directly. If a user migration Lambda trigger is set, this flow will invoke the user migration Lambda if the USERNAME is not found in the user pool. </p> </li> <li> <p> <code>ADMIN_USER_PASSWORD_AUTH</code>: Admin-based user password authentication. This replaces the <code>ADMIN_NO_SRP_AUTH</code> authentication flow. In this flow, Cognito receives the password in the request instead of using the SRP process to verify passwords.</p> </li> </ul> <p> <code>ADMIN_NO_SRP_AUTH</code> is not a valid value.</p>
    #[serde(rename = "AuthFlow")]
    pub auth_flow: String,
    /// <p><p>The authentication parameters. These are inputs corresponding to the <code>AuthFlow</code> that you are invoking. The required values depend on the value of <code>AuthFlow</code>:</p> <ul> <li> <p>For <code>USER<em>SRP</em>AUTH</code>: <code>USERNAME</code> (required), <code>SRP<em>A</code> (required), <code>SECRET</em>HASH</code> (required if the app client is configured with a client secret), <code>DEVICE<em>KEY</code> </p> </li> <li> <p>For <code>REFRESH</em>TOKEN<em>AUTH/REFRESH</em>TOKEN</code>: <code>REFRESH<em>TOKEN</code> (required), <code>SECRET</em>HASH</code> (required if the app client is configured with a client secret), <code>DEVICE<em>KEY</code> </p> </li> <li> <p>For <code>CUSTOM</em>AUTH</code>: <code>USERNAME</code> (required), <code>SECRET<em>HASH</code> (if app client is configured with client secret), <code>DEVICE</em>KEY</code> </p> </li> </ul></p>
    #[serde(rename = "AuthParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The app client ID.</p>
    #[serde(rename = "ClientId")]
    pub client_id: String,
    /// <p><p>A map of custom key-value pairs that you can provide as input for certain custom workflows that this action triggers.</p> <p>You create custom workflows by assigning AWS Lambda functions to user pool triggers. When you use the InitiateAuth API action, Amazon Cognito invokes the AWS Lambda functions that are specified for various triggers. The ClientMetadata value is passed as input to the functions for only the following triggers:</p> <ul> <li> <p>Pre signup</p> </li> <li> <p>Pre authentication</p> </li> <li> <p>User migration</p> </li> </ul> <p>When Amazon Cognito invokes the functions for these triggers, it passes a JSON payload, which the function receives as input. This payload contains a <code>validationData</code> attribute, which provides the data that you assigned to the ClientMetadata parameter in your InitiateAuth request. In your function code in AWS Lambda, you can process the <code>validationData</code> value to enhance your workflow for your specific needs.</p> <p>When you use the InitiateAuth API action, Amazon Cognito also invokes the functions for the following triggers, but it does not provide the ClientMetadata value as input:</p> <ul> <li> <p>Post authentication</p> </li> <li> <p>Custom message</p> </li> <li> <p>Pre token generation</p> </li> <li> <p>Create auth challenge</p> </li> <li> <p>Define auth challenge</p> </li> <li> <p>Verify auth challenge</p> </li> </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools-working-with-aws-lambda-triggers.html">Customizing User Pool Workflows with Lambda Triggers</a> in the <i>Amazon Cognito Developer Guide</i>.</p> <note> <p>Take the following limitations into consideration when you use the ClientMetadata parameter:</p> <ul> <li> <p>Amazon Cognito does not store the ClientMetadata value. This data is available only to AWS Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration does not include triggers, the ClientMetadata parameter serves no purpose.</p> </li> <li> <p>Amazon Cognito does not validate the ClientMetadata value.</p> </li> <li> <p>Amazon Cognito does not encrypt the the ClientMetadata value, so don&#39;t use it to provide sensitive information.</p> </li> </ul> </note></p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListResourceServersResponse {
    /// <p>A pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The resource servers.</p>
    #[serde(rename = "ResourceServers")]
    pub resource_servers: Vec<ResourceServerType>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the user pool that the tags are assigned to.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>The tags that are assigned to the user pool.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Represents the request to list the user import jobs.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListUsersRequest {
    /// <p>An array of strings, where each string is the name of a user attribute to be returned for each user in the search results. If the array is null, all attributes are returned.</p>
    #[serde(rename = "AttributesToGet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_to_get: Option<Vec<String>>,
    /// <p>A filter string of the form "<i>AttributeName</i> <i>Filter-Type</i> "<i>AttributeValue</i>"". Quotation marks within the filter string must be escaped using the backslash (\) character. For example, "<code>family_name</code> = \"Reddy\"".</p> <ul> <li> <p> <i>AttributeName</i>: The name of the attribute to search for. You can only search for one attribute at a time.</p> </li> <li> <p> <i>Filter-Type</i>: For an exact match, use =, for example, "<code>given_name</code> = \"Jon\"". For a prefix ("starts with") match, use ^=, for example, "<code>given_name</code> ^= \"Jon\"". </p> </li> <li> <p> <i>AttributeValue</i>: The attribute value that must be matched for each user.</p> </li> </ul> <p>If the filter string is empty, <code>ListUsers</code> returns all users in the user pool.</p> <p>You can only search for the following standard attributes:</p> <ul> <li> <p> <code>username</code> (case-sensitive)</p> </li> <li> <p> <code>email</code> </p> </li> <li> <p> <code>phone_number</code> </p> </li> <li> <p> <code>name</code> </p> </li> <li> <p> <code>given_name</code> </p> </li> <li> <p> <code>family_name</code> </p> </li> <li> <p> <code>preferred_username</code> </p> </li> <li> <p> <code>cognito:user_status</code> (called <b>Status</b> in the Console) (case-insensitive)</p> </li> <li> <p> <code>status (called <b>Enabled</b> in the Console) (case-sensitive)</code> </p> </li> <li> <p> <code>sub</code> </p> </li> </ul> <p>Custom attributes are not searchable.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/how-to-manage-user-accounts.html#cognito-user-pools-searching-for-users-using-listusers-api">Searching for Users Using the ListUsers API</a> and <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/how-to-manage-user-accounts.html#cognito-user-pools-searching-for-users-listusers-api-examples">Examples of Using the ListUsers API</a> in the <i>Amazon Cognito Developer Guide</i>.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p> <i>This data type is no longer supported.</i> You can use it only for SMS MFA configurations. You can't use it for TOTP software token MFA configurations.</p> <p>To set either type of MFA configuration, use the <a>AdminSetUserMFAPreference</a> or <a>SetUserMFAPreference</a> actions.</p> <p>To look up information about either type of MFA configuration, use the <a>AdminGetUserResponse$UserMFASettingList</a> or <a>GetUserResponse$UserMFASettingList</a> responses.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MFAOptionType {
    /// <p>The attribute name of the MFA option type. The only valid value is <code>phone_number</code>.</p>
    #[serde(rename = "AttributeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    /// <p>The delivery medium to send the MFA code. You can use this parameter to set only the <code>SMS</code> delivery medium value.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p><p>In the password policy you have set, refers to the number of days a temporary password is valid. If the user does not sign-in during this time, their password will need to be reset by an administrator.</p> <note> <p>When you set <code>TemporaryPasswordValidityDays</code> for a user pool, you will no longer be able to set the deprecated <code>UnusedAccountValidityDays</code> value for that user pool.</p> </note></p>
    #[serde(rename = "TemporaryPasswordValidityDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporary_password_validity_days: Option<i64>,
}

/// <p>A container for identity provider details.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

/// <p>A map containing a priority as a key, and recovery method name as a value.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecoveryOptionType {
    /// <p>Specifies the recovery method for a user.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>A positive integer specifying priority of a method with 1 being the highest priority.</p>
    #[serde(rename = "Priority")]
    pub priority: i64,
}

/// <p>Represents the request to resend the confirmation code.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ResendConfirmationCodeRequest {
    /// <p>The Amazon Pinpoint analytics metadata for collecting metrics for <code>ResendConfirmationCode</code> calls.</p>
    #[serde(rename = "AnalyticsMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_metadata: Option<AnalyticsMetadataType>,
    /// <p>The ID of the client associated with the user pool.</p>
    #[serde(rename = "ClientId")]
    pub client_id: String,
    /// <p><p>A map of custom key-value pairs that you can provide as input for any custom workflows that this action triggers. </p> <p>You create custom workflows by assigning AWS Lambda functions to user pool triggers. When you use the ResendConfirmationCode API action, Amazon Cognito invokes the function that is assigned to the <i>custom message</i> trigger. When Amazon Cognito invokes this function, it passes a JSON payload, which the function receives as input. This payload contains a <code>clientMetadata</code> attribute, which provides the data that you assigned to the ClientMetadata parameter in your ResendConfirmationCode request. In your function code in AWS Lambda, you can process the <code>clientMetadata</code> value to enhance your workflow for your specific needs.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools-working-with-aws-lambda-triggers.html">Customizing User Pool Workflows with Lambda Triggers</a> in the <i>Amazon Cognito Developer Guide</i>.</p> <note> <p>Take the following limitations into consideration when you use the ClientMetadata parameter:</p> <ul> <li> <p>Amazon Cognito does not store the ClientMetadata value. This data is available only to AWS Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration does not include triggers, the ClientMetadata parameter serves no purpose.</p> </li> <li> <p>Amazon Cognito does not validate the ClientMetadata value.</p> </li> <li> <p>Amazon Cognito does not encrypt the the ClientMetadata value, so don&#39;t use it to provide sensitive information.</p> </li> </ul> </note></p>
    #[serde(rename = "ClientMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_metadata: Option<::std::collections::HashMap<String, String>>,
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RespondToAuthChallengeRequest {
    /// <p>The Amazon Pinpoint analytics metadata for collecting metrics for <code>RespondToAuthChallenge</code> calls.</p>
    #[serde(rename = "AnalyticsMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_metadata: Option<AnalyticsMetadataType>,
    /// <p>The challenge name. For more information, see .</p> <p> <code>ADMIN_NO_SRP_AUTH</code> is not a valid value.</p>
    #[serde(rename = "ChallengeName")]
    pub challenge_name: String,
    /// <p><p>The challenge responses. These are inputs corresponding to the value of <code>ChallengeName</code>, for example:</p> <note> <p> <code>SECRET<em>HASH</code> (if app client is configured with client secret) applies to all inputs below (including <code>SOFTWARE</em>TOKEN<em>MFA</code>).</p> </note> <ul> <li> <p> <code>SMS</em>MFA</code>: <code>SMS<em>MFA</em>CODE</code>, <code>USERNAME</code>.</p> </li> <li> <p> <code>PASSWORD<em>VERIFIER</code>: <code>PASSWORD</em>CLAIM<em>SIGNATURE</code>, <code>PASSWORD</em>CLAIM<em>SECRET</em>BLOCK</code>, <code>TIMESTAMP</code>, <code>USERNAME</code>.</p> </li> <li> <p> <code>NEW<em>PASSWORD</em>REQUIRED</code>: <code>NEW<em>PASSWORD</code>, any other required attributes, <code>USERNAME</code>. </p> </li> <li> <p> <code>SOFTWARE</em>TOKEN<em>MFA</code>: <code>USERNAME</code> and <code>SOFTWARE</em>TOKEN<em>MFA</em>CODE</code> are required attributes.</p> </li> <li> <p> <code>DEVICE<em>SRP</em>AUTH</code> requires <code>USERNAME</code>, <code>DEVICE<em>KEY</code>, <code>SRP</em>A</code> (and <code>SECRET<em>HASH</code>).</p> </li> <li> <p> <code>DEVICE</em>PASSWORD<em>VERIFIER</code> requires everything that <code>PASSWORD</em>VERIFIER</code> requires plus <code>DEVICE_KEY</code>.</p> </li> </ul></p>
    #[serde(rename = "ChallengeResponses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_responses: Option<::std::collections::HashMap<String, String>>,
    /// <p>The app client ID.</p>
    #[serde(rename = "ClientId")]
    pub client_id: String,
    /// <p><p>A map of custom key-value pairs that you can provide as input for any custom workflows that this action triggers. </p> <p>You create custom workflows by assigning AWS Lambda functions to user pool triggers. When you use the RespondToAuthChallenge API action, Amazon Cognito invokes any functions that are assigned to the following triggers: <i>post authentication</i>, <i>pre token generation</i>, <i>define auth challenge</i>, <i>create auth challenge</i>, and <i>verify auth challenge</i>. When Amazon Cognito invokes any of these functions, it passes a JSON payload, which the function receives as input. This payload contains a <code>clientMetadata</code> attribute, which provides the data that you assigned to the ClientMetadata parameter in your RespondToAuthChallenge request. In your function code in AWS Lambda, you can process the <code>clientMetadata</code> value to enhance your workflow for your specific needs.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools-working-with-aws-lambda-triggers.html">Customizing User Pool Workflows with Lambda Triggers</a> in the <i>Amazon Cognito Developer Guide</i>.</p> <note> <p>Take the following limitations into consideration when you use the ClientMetadata parameter:</p> <ul> <li> <p>Amazon Cognito does not store the ClientMetadata value. This data is available only to AWS Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration does not include triggers, the ClientMetadata parameter serves no purpose.</p> </li> <li> <p>Amazon Cognito does not validate the ClientMetadata value.</p> </li> <li> <p>Amazon Cognito does not encrypt the the ClientMetadata value, so don&#39;t use it to provide sensitive information.</p> </li> </ul> </note></p>
    #[serde(rename = "ClientMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_metadata: Option<::std::collections::HashMap<String, String>>,
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>The type used for enabling SMS MFA at the user level.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SMSMfaSettingsType {
    /// <p>Specifies whether SMS text message MFA is enabled.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>Specifies whether SMS is the preferred MFA method.</p>
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
    /// <p>Specifies whether the value of the attribute can be changed.</p> <p>For any user pool attribute that's mapped to an identity provider attribute, you must set this parameter to <code>true</code>. Amazon Cognito updates mapped attributes when users sign in to your application through an identity provider. If an attribute is immutable, Amazon Cognito throws an error when it attempts to update the attribute. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-specifying-attribute-mapping.html">Specifying Identity Provider Attribute Mappings for Your User Pool</a>.</p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SetRiskConfigurationResponse {
    /// <p>The risk configuration.</p>
    #[serde(rename = "RiskConfiguration")]
    pub risk_configuration: RiskConfigurationType,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SetUICustomizationResponse {
    /// <p>The UI customization information.</p>
    #[serde(rename = "UICustomization")]
    pub ui_customization: UICustomizationType,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SetUserMFAPreferenceRequest {
    /// <p>The access token for the user.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SetUserMFAPreferenceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SetUserPoolMfaConfigRequest {
    /// <p><p>The MFA configuration. Valid values include:</p> <ul> <li> <p> <code>OFF</code> MFA will not be used for any users.</p> </li> <li> <p> <code>ON</code> MFA is required for all users to sign in.</p> </li> <li> <p> <code>OPTIONAL</code> MFA will be required only for individual users who have an MFA factor enabled.</p> </li> </ul></p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SetUserPoolMfaConfigResponse {
    /// <p><p>The MFA configuration. Valid values include:</p> <ul> <li> <p> <code>OFF</code> MFA will not be used for any users.</p> </li> <li> <p> <code>ON</code> MFA is required for all users to sign in.</p> </li> <li> <p> <code>OPTIONAL</code> MFA will be required only for individual users who have an MFA factor enabled.</p> </li> </ul></p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SetUserSettingsRequest {
    /// <p>The access token for the set user settings request.</p>
    #[serde(rename = "AccessToken")]
    pub access_token: String,
    /// <p>You can use this parameter only to set an SMS configuration that uses SMS for delivery.</p>
    #[serde(rename = "MFAOptions")]
    pub mfa_options: Vec<MFAOptionType>,
}

/// <p>The response from the server for a set user settings request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SetUserSettingsResponse {}

/// <p>Represents the request to register a user.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SignUpRequest {
    /// <p>The Amazon Pinpoint analytics metadata for collecting metrics for <code>SignUp</code> calls.</p>
    #[serde(rename = "AnalyticsMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_metadata: Option<AnalyticsMetadataType>,
    /// <p>The ID of the client associated with the user pool.</p>
    #[serde(rename = "ClientId")]
    pub client_id: String,
    /// <p><p>A map of custom key-value pairs that you can provide as input for any custom workflows that this action triggers. </p> <p>You create custom workflows by assigning AWS Lambda functions to user pool triggers. When you use the SignUp API action, Amazon Cognito invokes any functions that are assigned to the following triggers: <i>pre sign-up</i>, <i>custom message</i>, and <i>post confirmation</i>. When Amazon Cognito invokes any of these functions, it passes a JSON payload, which the function receives as input. This payload contains a <code>clientMetadata</code> attribute, which provides the data that you assigned to the ClientMetadata parameter in your SignUp request. In your function code in AWS Lambda, you can process the <code>clientMetadata</code> value to enhance your workflow for your specific needs.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools-working-with-aws-lambda-triggers.html">Customizing User Pool Workflows with Lambda Triggers</a> in the <i>Amazon Cognito Developer Guide</i>.</p> <note> <p>Take the following limitations into consideration when you use the ClientMetadata parameter:</p> <ul> <li> <p>Amazon Cognito does not store the ClientMetadata value. This data is available only to AWS Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration does not include triggers, the ClientMetadata parameter serves no purpose.</p> </li> <li> <p>Amazon Cognito does not validate the ClientMetadata value.</p> </li> <li> <p>Amazon Cognito does not encrypt the the ClientMetadata value, so don&#39;t use it to provide sensitive information.</p> </li> </ul> </note></p>
    #[serde(rename = "ClientMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_metadata: Option<::std::collections::HashMap<String, String>>,
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>The SMS configuration type that includes the settings the Cognito User Pool needs to call for the Amazon SNS service to send an SMS message from your AWS account. The Cognito User Pool makes the request to the Amazon SNS Service by using an AWS IAM role that you provide for your AWS account.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SmsConfigurationType {
    /// <p>The external ID is a value that we recommend you use to add security to your IAM role which is used to call Amazon SNS to send SMS messages for your user pool. If you provide an <code>ExternalId</code>, the Cognito User Pool will include it when attempting to assume your IAM role, so that you can set your roles trust policy to require the <code>ExternalID</code>. If you use the Cognito Management Console to create a role for SMS MFA, Cognito will create a role with the required permissions and a trust policy that demonstrates use of the <code>ExternalId</code>.</p>
    #[serde(rename = "ExternalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon Simple Notification Service (SNS) caller. This is the ARN of the IAM role in your AWS account which Cognito will use to send SMS messages.</p>
    #[serde(rename = "SnsCallerArn")]
    pub sns_caller_arn: String,
}

/// <p>The SMS text message multi-factor authentication (MFA) configuration type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SmsMfaConfigType {
    /// <p>The SMS authentication message that will be sent to users with the code they need to sign in. The message must contain the ‘{####}’ placeholder, which will be replaced with the code. If the message is not included, and default message will be used.</p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SoftwareTokenMfaSettingsType {
    /// <p>Specifies whether software token MFA is enabled.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>Specifies whether software token MFA is the preferred MFA method.</p>
    #[serde(rename = "PreferredMfa")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_mfa: Option<bool>,
}

/// <p>Represents the request to start the user import job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartUserImportJobResponse {
    /// <p>The job object that represents the user import job.</p>
    #[serde(rename = "UserImportJob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_import_job: Option<UserImportJobType>,
}

/// <p>Represents the request to stop the user import job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the user pool to assign the tags to.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The tags to assign to the user pool.</p>
    #[serde(rename = "Tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

/// <p>A container for the UI customization information for a user pool's built-in app UI.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the user pool that the tags are assigned to.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The keys of the tags to remove from the user pool.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateAuthEventFeedbackResponse {}

/// <p>Represents the request to update the device status.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDeviceStatusResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateGroupResponse {
    /// <p>The group object for the group.</p>
    #[serde(rename = "Group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<GroupType>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateIdentityProviderResponse {
    /// <p>The identity provider object.</p>
    #[serde(rename = "IdentityProvider")]
    pub identity_provider: IdentityProviderType,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateResourceServerResponse {
    /// <p>The resource server.</p>
    #[serde(rename = "ResourceServer")]
    pub resource_server: ResourceServerType,
}

/// <p>Represents the request to update user attributes.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateUserAttributesRequest {
    /// <p>The access token for the request to update user attributes.</p>
    #[serde(rename = "AccessToken")]
    pub access_token: String,
    /// <p><p>A map of custom key-value pairs that you can provide as input for any custom workflows that this action triggers. </p> <p>You create custom workflows by assigning AWS Lambda functions to user pool triggers. When you use the UpdateUserAttributes API action, Amazon Cognito invokes the function that is assigned to the <i>custom message</i> trigger. When Amazon Cognito invokes this function, it passes a JSON payload, which the function receives as input. This payload contains a <code>clientMetadata</code> attribute, which provides the data that you assigned to the ClientMetadata parameter in your UpdateUserAttributes request. In your function code in AWS Lambda, you can process the <code>clientMetadata</code> value to enhance your workflow for your specific needs.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools-working-with-aws-lambda-triggers.html">Customizing User Pool Workflows with Lambda Triggers</a> in the <i>Amazon Cognito Developer Guide</i>.</p> <note> <p>Take the following limitations into consideration when you use the ClientMetadata parameter:</p> <ul> <li> <p>Amazon Cognito does not store the ClientMetadata value. This data is available only to AWS Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration does not include triggers, the ClientMetadata parameter serves no purpose.</p> </li> <li> <p>Amazon Cognito does not validate the ClientMetadata value.</p> </li> <li> <p>Amazon Cognito does not encrypt the the ClientMetadata value, so don&#39;t use it to provide sensitive information.</p> </li> </ul> </note></p>
    #[serde(rename = "ClientMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_metadata: Option<::std::collections::HashMap<String, String>>,
    /// <p>An array of name-value pairs representing user attributes.</p> <p>For custom attributes, you must prepend the <code>custom:</code> prefix to the attribute name.</p>
    #[serde(rename = "UserAttributes")]
    pub user_attributes: Vec<AttributeType>,
}

/// <p>Represents the response from the server for the request to update user attributes.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateUserAttributesResponse {
    /// <p>The code delivery details list from the server for the request to update user attributes.</p>
    #[serde(rename = "CodeDeliveryDetailsList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_delivery_details_list: Option<Vec<CodeDeliveryDetailsType>>,
}

/// <p>Represents the request to update the user pool client.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateUserPoolClientRequest {
    /// <p>Set to <code>code</code> to initiate a code grant flow, which provides an authorization code as the response. This code can be exchanged for access tokens with the token endpoint.</p>
    #[serde(rename = "AllowedOAuthFlows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_o_auth_flows: Option<Vec<String>>,
    /// <p>Set to TRUE if the client is allowed to follow the OAuth protocol when interacting with Cognito user pools.</p>
    #[serde(rename = "AllowedOAuthFlowsUserPoolClient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_o_auth_flows_user_pool_client: Option<bool>,
    /// <p>A list of allowed <code>OAuth</code> scopes. Currently supported values are <code>"phone"</code>, <code>"email"</code>, <code>"openid"</code>, and <code>"Cognito"</code>. In addition to these values, custom scopes created in Resource Servers are also supported.</p>
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
    /// <p><p>The authentication flows that are supported by the user pool clients. Flow names without the <code>ALLOW<em></code> prefix are deprecated in favor of new names with the <code>ALLOW</em></code> prefix. Note that values with <code>ALLOW<em></code> prefix cannot be used along with values without <code>ALLOW</em></code> prefix.</p> <p>Valid values include:</p> <ul> <li> <p> <code>ALLOW<em>ADMIN</em>USER<em>PASSWORD</em>AUTH</code>: Enable admin based user password authentication flow <code>ADMIN<em>USER</em>PASSWORD<em>AUTH</code>. This setting replaces the <code>ADMIN</em>NO<em>SRP</em>AUTH</code> setting. With this authentication flow, Cognito receives the password in the request instead of using the SRP (Secure Remote Password protocol) protocol to verify passwords.</p> </li> <li> <p> <code>ALLOW<em>CUSTOM</em>AUTH</code>: Enable Lambda trigger based authentication.</p> </li> <li> <p> <code>ALLOW<em>USER</em>PASSWORD<em>AUTH</code>: Enable user password-based authentication. In this flow, Cognito receives the password in the request instead of using the SRP protocol to verify passwords.</p> </li> <li> <p> <code>ALLOW</em>USER<em>SRP</em>AUTH</code>: Enable SRP based authentication.</p> </li> <li> <p> <code>ALLOW<em>REFRESH</em>TOKEN_AUTH</code>: Enable authflow to refresh tokens.</p> </li> </ul></p>
    #[serde(rename = "ExplicitAuthFlows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_auth_flows: Option<Vec<String>>,
    /// <p>A list of allowed logout URLs for the identity providers.</p>
    #[serde(rename = "LogoutURLs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logout_ur_ls: Option<Vec<String>>,
    /// <p><p>Use this setting to choose which errors and responses are returned by Cognito APIs during authentication, account confirmation, and password recovery when the user does not exist in the user pool. When set to <code>ENABLED</code> and the user does not exist, authentication returns an error indicating either the username or password was incorrect, and account confirmation and password recovery return a response indicating a code was sent to a simulated destination. When set to <code>LEGACY</code>, those APIs will return a <code>UserNotFoundException</code> exception if the user does not exist in the user pool.</p> <p>Valid values include:</p> <ul> <li> <p> <code>ENABLED</code> - This prevents user existence-related errors.</p> </li> <li> <p> <code>LEGACY</code> - This represents the old behavior of Cognito where user existence related errors are not prevented.</p> </li> </ul> <p>This setting affects the behavior of following APIs:</p> <ul> <li> <p> <a>AdminInitiateAuth</a> </p> </li> <li> <p> <a>AdminRespondToAuthChallenge</a> </p> </li> <li> <p> <a>InitiateAuth</a> </p> </li> <li> <p> <a>RespondToAuthChallenge</a> </p> </li> <li> <p> <a>ForgotPassword</a> </p> </li> <li> <p> <a>ConfirmForgotPassword</a> </p> </li> <li> <p> <a>ConfirmSignUp</a> </p> </li> <li> <p> <a>ResendConfirmationCode</a> </p> </li> </ul> <note> <p>After January 1st 2020, the value of <code>PreventUserExistenceErrors</code> will default to <code>ENABLED</code> for newly created user pool clients if no value is provided.</p> </note></p>
    #[serde(rename = "PreventUserExistenceErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prevent_user_existence_errors: Option<String>,
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateUserPoolClientResponse {
    /// <p>The user pool client value from the response from the server when an update user pool client request is made.</p>
    #[serde(rename = "UserPoolClient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_client: Option<UserPoolClientType>,
}

/// <p>The UpdateUserPoolDomain request input.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateUserPoolDomainResponse {
    /// <p>The Amazon CloudFront endpoint that Amazon Cognito set up when you added the custom domain to your user pool.</p>
    #[serde(rename = "CloudFrontDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_front_domain: Option<String>,
}

/// <p>Represents the request to update the user pool.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateUserPoolRequest {
    /// <p>Use this setting to define which verified available method a user can use to recover their password when they call <code>ForgotPassword</code>. It allows you to define a preferred method when a user has more than one method available. With this setting, SMS does not qualify for a valid password recovery mechanism if the user also has SMS MFA enabled. In the absence of this setting, Cognito uses the legacy behavior to determine the recovery method where SMS is preferred over email.</p>
    #[serde(rename = "AccountRecoverySetting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_recovery_setting: Option<AccountRecoverySettingType>,
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
    /// <p>The tag keys and values to assign to the user pool. A tag is a label that you can use to categorize and manage user pools in different ways, such as by purpose, owner, environment, or other criteria.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateUserPoolResponse {}

/// <p>Contextual data such as the user's device fingerprint, IP address, or location used for evaluating the risk of an unexpected event by Amazon Cognito advanced security.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UserContextDataType {
    /// <p>Contextual data such as the user's device fingerprint, IP address, or location used for evaluating the risk of an unexpected event by Amazon Cognito advanced security.</p>
    #[serde(rename = "EncodedData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoded_data: Option<String>,
}

/// <p>The user import job type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UserPoolClientType {
    /// <p>Set to <code>code</code> to initiate a code grant flow, which provides an authorization code as the response. This code can be exchanged for access tokens with the token endpoint.</p> <p>Set to <code>token</code> to specify that the client should get the access token (and, optionally, ID token, based on scopes) directly.</p>
    #[serde(rename = "AllowedOAuthFlows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_o_auth_flows: Option<Vec<String>>,
    /// <p>Set to TRUE if the client is allowed to follow the OAuth protocol when interacting with Cognito user pools.</p>
    #[serde(rename = "AllowedOAuthFlowsUserPoolClient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_o_auth_flows_user_pool_client: Option<bool>,
    /// <p>A list of allowed <code>OAuth</code> scopes. Currently supported values are <code>"phone"</code>, <code>"email"</code>, <code>"openid"</code>, and <code>"Cognito"</code>. In addition to these values, custom scopes created in Resource Servers are also supported.</p>
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
    /// <p><p>The authentication flows that are supported by the user pool clients. Flow names without the <code>ALLOW<em></code> prefix are deprecated in favor of new names with the <code>ALLOW</em></code> prefix. Note that values with <code>ALLOW<em></code> prefix cannot be used along with values without <code>ALLOW</em></code> prefix.</p> <p>Valid values include:</p> <ul> <li> <p> <code>ALLOW<em>ADMIN</em>USER<em>PASSWORD</em>AUTH</code>: Enable admin based user password authentication flow <code>ADMIN<em>USER</em>PASSWORD<em>AUTH</code>. This setting replaces the <code>ADMIN</em>NO<em>SRP</em>AUTH</code> setting. With this authentication flow, Cognito receives the password in the request instead of using the SRP (Secure Remote Password protocol) protocol to verify passwords.</p> </li> <li> <p> <code>ALLOW<em>CUSTOM</em>AUTH</code>: Enable Lambda trigger based authentication.</p> </li> <li> <p> <code>ALLOW<em>USER</em>PASSWORD<em>AUTH</code>: Enable user password-based authentication. In this flow, Cognito receives the password in the request instead of using the SRP protocol to verify passwords.</p> </li> <li> <p> <code>ALLOW</em>USER<em>SRP</em>AUTH</code>: Enable SRP based authentication.</p> </li> <li> <p> <code>ALLOW<em>REFRESH</em>TOKEN_AUTH</code>: Enable authflow to refresh tokens.</p> </li> </ul></p>
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
    /// <p><p>Use this setting to choose which errors and responses are returned by Cognito APIs during authentication, account confirmation, and password recovery when the user does not exist in the user pool. When set to <code>ENABLED</code> and the user does not exist, authentication returns an error indicating either the username or password was incorrect, and account confirmation and password recovery return a response indicating a code was sent to a simulated destination. When set to <code>LEGACY</code>, those APIs will return a <code>UserNotFoundException</code> exception if the user does not exist in the user pool.</p> <p>Valid values include:</p> <ul> <li> <p> <code>ENABLED</code> - This prevents user existence-related errors.</p> </li> <li> <p> <code>LEGACY</code> - This represents the old behavior of Cognito where user existence related errors are not prevented.</p> </li> </ul> <p>This setting affects the behavior of following APIs:</p> <ul> <li> <p> <a>AdminInitiateAuth</a> </p> </li> <li> <p> <a>AdminRespondToAuthChallenge</a> </p> </li> <li> <p> <a>InitiateAuth</a> </p> </li> <li> <p> <a>RespondToAuthChallenge</a> </p> </li> <li> <p> <a>ForgotPassword</a> </p> </li> <li> <p> <a>ConfirmForgotPassword</a> </p> </li> <li> <p> <a>ConfirmSignUp</a> </p> </li> <li> <p> <a>ResendConfirmationCode</a> </p> </li> </ul> <note> <p>After January 1st 2020, the value of <code>PreventUserExistenceErrors</code> will default to <code>ENABLED</code> for newly created user pool clients if no value is provided.</p> </note></p>
    #[serde(rename = "PreventUserExistenceErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prevent_user_existence_errors: Option<String>,
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UserPoolType {
    /// <p>Use this setting to define which verified available method a user can use to recover their password when they call <code>ForgotPassword</code>. It allows you to define a preferred method when a user has more than one method available. With this setting, SMS does not qualify for a valid password recovery mechanism if the user also has SMS MFA enabled. In the absence of this setting, Cognito uses the legacy behavior to determine the recovery method where SMS is preferred over email.</p>
    #[serde(rename = "AccountRecoverySetting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_recovery_setting: Option<AccountRecoverySettingType>,
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
    /// <p>A custom domain name that you provide to Amazon Cognito. This parameter applies only if you use a custom domain to host the sign-up and sign-in pages for your application. For example: <code>auth.example.com</code>.</p> <p>For more information about adding a custom domain to your user pool, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-add-custom-domain.html">Using Your Own Domain for the Hosted UI</a>.</p>
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
    /// <p>The tags that are assigned to the user pool. A tag is a label that you can apply to user pools to categorize and manage them in different ways, such as by purpose, owner, environment, or other criteria.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p><p>The user status. Can be one of the following:</p> <ul> <li> <p>UNCONFIRMED - User has been created but not confirmed.</p> </li> <li> <p>CONFIRMED - User has been confirmed.</p> </li> <li> <p>ARCHIVED - User is no longer active.</p> </li> <li> <p>COMPROMISED - User is disabled due to a potential security threat.</p> </li> <li> <p>UNKNOWN - User status is not known.</p> </li> <li> <p>RESET<em>REQUIRED - User is confirmed, but the user must request a code and reset his or her password before he or she can sign in.</p> </li> <li> <p>FORCE</em>CHANGE_PASSWORD - The user is confirmed and the user can sign in using a temporary password, but on first sign-in, the user must change his or her password to a new value before doing anything else. </p> </li> </ul></p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(AddCustomAttributesError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AddCustomAttributesError::InvalidParameter(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AddCustomAttributesError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AddCustomAttributesError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AddCustomAttributesError::TooManyRequests(err.msg))
                }
                "UserImportInProgressException" => {
                    return RusotoError::Service(AddCustomAttributesError::UserImportInProgress(
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
impl fmt::Display for AddCustomAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddCustomAttributesError::InternalError(ref cause) => write!(f, "{}", cause),
            AddCustomAttributesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AddCustomAttributesError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            AddCustomAttributesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AddCustomAttributesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            AddCustomAttributesError::UserImportInProgress(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AddCustomAttributesError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(AdminAddUserToGroupError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AdminAddUserToGroupError::InvalidParameter(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminAddUserToGroupError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AdminAddUserToGroupError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AdminAddUserToGroupError::TooManyRequests(err.msg))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminAddUserToGroupError::UserNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AdminAddUserToGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AdminAddUserToGroupError::InternalError(ref cause) => write!(f, "{}", cause),
            AdminAddUserToGroupError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AdminAddUserToGroupError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            AdminAddUserToGroupError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AdminAddUserToGroupError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            AdminAddUserToGroupError::UserNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AdminAddUserToGroupError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(AdminConfirmSignUpError::InternalError(err.msg))
                }
                "InvalidLambdaResponseException" => {
                    return RusotoError::Service(AdminConfirmSignUpError::InvalidLambdaResponse(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AdminConfirmSignUpError::InvalidParameter(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(AdminConfirmSignUpError::LimitExceeded(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminConfirmSignUpError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AdminConfirmSignUpError::ResourceNotFound(err.msg))
                }
                "TooManyFailedAttemptsException" => {
                    return RusotoError::Service(AdminConfirmSignUpError::TooManyFailedAttempts(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AdminConfirmSignUpError::TooManyRequests(err.msg))
                }
                "UnexpectedLambdaException" => {
                    return RusotoError::Service(AdminConfirmSignUpError::UnexpectedLambda(err.msg))
                }
                "UserLambdaValidationException" => {
                    return RusotoError::Service(AdminConfirmSignUpError::UserLambdaValidation(
                        err.msg,
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminConfirmSignUpError::UserNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AdminConfirmSignUpError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AdminConfirmSignUpError::InternalError(ref cause) => write!(f, "{}", cause),
            AdminConfirmSignUpError::InvalidLambdaResponse(ref cause) => write!(f, "{}", cause),
            AdminConfirmSignUpError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AdminConfirmSignUpError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            AdminConfirmSignUpError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            AdminConfirmSignUpError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AdminConfirmSignUpError::TooManyFailedAttempts(ref cause) => write!(f, "{}", cause),
            AdminConfirmSignUpError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            AdminConfirmSignUpError::UnexpectedLambda(ref cause) => write!(f, "{}", cause),
            AdminConfirmSignUpError::UserLambdaValidation(ref cause) => write!(f, "{}", cause),
            AdminConfirmSignUpError::UserNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AdminConfirmSignUpError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CodeDeliveryFailureException" => {
                    return RusotoError::Service(AdminCreateUserError::CodeDeliveryFailure(err.msg))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(AdminCreateUserError::InternalError(err.msg))
                }
                "InvalidLambdaResponseException" => {
                    return RusotoError::Service(AdminCreateUserError::InvalidLambdaResponse(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AdminCreateUserError::InvalidParameter(err.msg))
                }
                "InvalidPasswordException" => {
                    return RusotoError::Service(AdminCreateUserError::InvalidPassword(err.msg))
                }
                "InvalidSmsRoleAccessPolicyException" => {
                    return RusotoError::Service(AdminCreateUserError::InvalidSmsRoleAccessPolicy(
                        err.msg,
                    ))
                }
                "InvalidSmsRoleTrustRelationshipException" => {
                    return RusotoError::Service(
                        AdminCreateUserError::InvalidSmsRoleTrustRelationship(err.msg),
                    )
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminCreateUserError::NotAuthorized(err.msg))
                }
                "PreconditionNotMetException" => {
                    return RusotoError::Service(AdminCreateUserError::PreconditionNotMet(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AdminCreateUserError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AdminCreateUserError::TooManyRequests(err.msg))
                }
                "UnexpectedLambdaException" => {
                    return RusotoError::Service(AdminCreateUserError::UnexpectedLambda(err.msg))
                }
                "UnsupportedUserStateException" => {
                    return RusotoError::Service(AdminCreateUserError::UnsupportedUserState(
                        err.msg,
                    ))
                }
                "UserLambdaValidationException" => {
                    return RusotoError::Service(AdminCreateUserError::UserLambdaValidation(
                        err.msg,
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminCreateUserError::UserNotFound(err.msg))
                }
                "UsernameExistsException" => {
                    return RusotoError::Service(AdminCreateUserError::UsernameExists(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AdminCreateUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AdminCreateUserError::CodeDeliveryFailure(ref cause) => write!(f, "{}", cause),
            AdminCreateUserError::InternalError(ref cause) => write!(f, "{}", cause),
            AdminCreateUserError::InvalidLambdaResponse(ref cause) => write!(f, "{}", cause),
            AdminCreateUserError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AdminCreateUserError::InvalidPassword(ref cause) => write!(f, "{}", cause),
            AdminCreateUserError::InvalidSmsRoleAccessPolicy(ref cause) => write!(f, "{}", cause),
            AdminCreateUserError::InvalidSmsRoleTrustRelationship(ref cause) => {
                write!(f, "{}", cause)
            }
            AdminCreateUserError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            AdminCreateUserError::PreconditionNotMet(ref cause) => write!(f, "{}", cause),
            AdminCreateUserError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AdminCreateUserError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            AdminCreateUserError::UnexpectedLambda(ref cause) => write!(f, "{}", cause),
            AdminCreateUserError::UnsupportedUserState(ref cause) => write!(f, "{}", cause),
            AdminCreateUserError::UserLambdaValidation(ref cause) => write!(f, "{}", cause),
            AdminCreateUserError::UserNotFound(ref cause) => write!(f, "{}", cause),
            AdminCreateUserError::UsernameExists(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AdminCreateUserError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(AdminDeleteUserError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AdminDeleteUserError::InvalidParameter(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminDeleteUserError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AdminDeleteUserError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AdminDeleteUserError::TooManyRequests(err.msg))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminDeleteUserError::UserNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AdminDeleteUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AdminDeleteUserError::InternalError(ref cause) => write!(f, "{}", cause),
            AdminDeleteUserError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AdminDeleteUserError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            AdminDeleteUserError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AdminDeleteUserError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            AdminDeleteUserError::UserNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AdminDeleteUserError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(AdminDeleteUserAttributesError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AdminDeleteUserAttributesError::InvalidParameter(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminDeleteUserAttributesError::NotAuthorized(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AdminDeleteUserAttributesError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AdminDeleteUserAttributesError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminDeleteUserAttributesError::UserNotFound(
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
impl fmt::Display for AdminDeleteUserAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AdminDeleteUserAttributesError::InternalError(ref cause) => write!(f, "{}", cause),
            AdminDeleteUserAttributesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AdminDeleteUserAttributesError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            AdminDeleteUserAttributesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AdminDeleteUserAttributesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            AdminDeleteUserAttributesError::UserNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AdminDeleteUserAttributesError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AliasExistsException" => {
                    return RusotoError::Service(AdminDisableProviderForUserError::AliasExists(
                        err.msg,
                    ))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(AdminDisableProviderForUserError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        AdminDisableProviderForUserError::InvalidParameter(err.msg),
                    )
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminDisableProviderForUserError::NotAuthorized(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        AdminDisableProviderForUserError::ResourceNotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AdminDisableProviderForUserError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminDisableProviderForUserError::UserNotFound(
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
impl fmt::Display for AdminDisableProviderForUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AdminDisableProviderForUserError::AliasExists(ref cause) => write!(f, "{}", cause),
            AdminDisableProviderForUserError::InternalError(ref cause) => write!(f, "{}", cause),
            AdminDisableProviderForUserError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AdminDisableProviderForUserError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            AdminDisableProviderForUserError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AdminDisableProviderForUserError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            AdminDisableProviderForUserError::UserNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AdminDisableProviderForUserError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(AdminDisableUserError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AdminDisableUserError::InvalidParameter(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminDisableUserError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AdminDisableUserError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AdminDisableUserError::TooManyRequests(err.msg))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminDisableUserError::UserNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AdminDisableUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AdminDisableUserError::InternalError(ref cause) => write!(f, "{}", cause),
            AdminDisableUserError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AdminDisableUserError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            AdminDisableUserError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AdminDisableUserError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            AdminDisableUserError::UserNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AdminDisableUserError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(AdminEnableUserError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AdminEnableUserError::InvalidParameter(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminEnableUserError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AdminEnableUserError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AdminEnableUserError::TooManyRequests(err.msg))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminEnableUserError::UserNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AdminEnableUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AdminEnableUserError::InternalError(ref cause) => write!(f, "{}", cause),
            AdminEnableUserError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AdminEnableUserError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            AdminEnableUserError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AdminEnableUserError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            AdminEnableUserError::UserNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AdminEnableUserError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(AdminForgetDeviceError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AdminForgetDeviceError::InvalidParameter(err.msg))
                }
                "InvalidUserPoolConfigurationException" => {
                    return RusotoError::Service(
                        AdminForgetDeviceError::InvalidUserPoolConfiguration(err.msg),
                    )
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminForgetDeviceError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AdminForgetDeviceError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AdminForgetDeviceError::TooManyRequests(err.msg))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminForgetDeviceError::UserNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AdminForgetDeviceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AdminForgetDeviceError::InternalError(ref cause) => write!(f, "{}", cause),
            AdminForgetDeviceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AdminForgetDeviceError::InvalidUserPoolConfiguration(ref cause) => {
                write!(f, "{}", cause)
            }
            AdminForgetDeviceError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            AdminForgetDeviceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AdminForgetDeviceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            AdminForgetDeviceError::UserNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AdminForgetDeviceError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(AdminGetDeviceError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AdminGetDeviceError::InvalidParameter(err.msg))
                }
                "InvalidUserPoolConfigurationException" => {
                    return RusotoError::Service(AdminGetDeviceError::InvalidUserPoolConfiguration(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminGetDeviceError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AdminGetDeviceError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AdminGetDeviceError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AdminGetDeviceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AdminGetDeviceError::InternalError(ref cause) => write!(f, "{}", cause),
            AdminGetDeviceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AdminGetDeviceError::InvalidUserPoolConfiguration(ref cause) => write!(f, "{}", cause),
            AdminGetDeviceError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            AdminGetDeviceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AdminGetDeviceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AdminGetDeviceError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(AdminGetUserError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AdminGetUserError::InvalidParameter(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminGetUserError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AdminGetUserError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AdminGetUserError::TooManyRequests(err.msg))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminGetUserError::UserNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AdminGetUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AdminGetUserError::InternalError(ref cause) => write!(f, "{}", cause),
            AdminGetUserError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AdminGetUserError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            AdminGetUserError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AdminGetUserError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            AdminGetUserError::UserNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AdminGetUserError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(AdminInitiateAuthError::InternalError(err.msg))
                }
                "InvalidLambdaResponseException" => {
                    return RusotoError::Service(AdminInitiateAuthError::InvalidLambdaResponse(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AdminInitiateAuthError::InvalidParameter(err.msg))
                }
                "InvalidSmsRoleAccessPolicyException" => {
                    return RusotoError::Service(
                        AdminInitiateAuthError::InvalidSmsRoleAccessPolicy(err.msg),
                    )
                }
                "InvalidSmsRoleTrustRelationshipException" => {
                    return RusotoError::Service(
                        AdminInitiateAuthError::InvalidSmsRoleTrustRelationship(err.msg),
                    )
                }
                "InvalidUserPoolConfigurationException" => {
                    return RusotoError::Service(
                        AdminInitiateAuthError::InvalidUserPoolConfiguration(err.msg),
                    )
                }
                "MFAMethodNotFoundException" => {
                    return RusotoError::Service(AdminInitiateAuthError::MFAMethodNotFound(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminInitiateAuthError::NotAuthorized(err.msg))
                }
                "PasswordResetRequiredException" => {
                    return RusotoError::Service(AdminInitiateAuthError::PasswordResetRequired(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AdminInitiateAuthError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AdminInitiateAuthError::TooManyRequests(err.msg))
                }
                "UnexpectedLambdaException" => {
                    return RusotoError::Service(AdminInitiateAuthError::UnexpectedLambda(err.msg))
                }
                "UserLambdaValidationException" => {
                    return RusotoError::Service(AdminInitiateAuthError::UserLambdaValidation(
                        err.msg,
                    ))
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(AdminInitiateAuthError::UserNotConfirmed(err.msg))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminInitiateAuthError::UserNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AdminInitiateAuthError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AdminInitiateAuthError::InternalError(ref cause) => write!(f, "{}", cause),
            AdminInitiateAuthError::InvalidLambdaResponse(ref cause) => write!(f, "{}", cause),
            AdminInitiateAuthError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AdminInitiateAuthError::InvalidSmsRoleAccessPolicy(ref cause) => write!(f, "{}", cause),
            AdminInitiateAuthError::InvalidSmsRoleTrustRelationship(ref cause) => {
                write!(f, "{}", cause)
            }
            AdminInitiateAuthError::InvalidUserPoolConfiguration(ref cause) => {
                write!(f, "{}", cause)
            }
            AdminInitiateAuthError::MFAMethodNotFound(ref cause) => write!(f, "{}", cause),
            AdminInitiateAuthError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            AdminInitiateAuthError::PasswordResetRequired(ref cause) => write!(f, "{}", cause),
            AdminInitiateAuthError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AdminInitiateAuthError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            AdminInitiateAuthError::UnexpectedLambda(ref cause) => write!(f, "{}", cause),
            AdminInitiateAuthError::UserLambdaValidation(ref cause) => write!(f, "{}", cause),
            AdminInitiateAuthError::UserNotConfirmed(ref cause) => write!(f, "{}", cause),
            AdminInitiateAuthError::UserNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AdminInitiateAuthError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AliasExistsException" => {
                    return RusotoError::Service(AdminLinkProviderForUserError::AliasExists(
                        err.msg,
                    ))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(AdminLinkProviderForUserError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AdminLinkProviderForUserError::InvalidParameter(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminLinkProviderForUserError::NotAuthorized(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AdminLinkProviderForUserError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AdminLinkProviderForUserError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminLinkProviderForUserError::UserNotFound(
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
impl fmt::Display for AdminLinkProviderForUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AdminLinkProviderForUserError::AliasExists(ref cause) => write!(f, "{}", cause),
            AdminLinkProviderForUserError::InternalError(ref cause) => write!(f, "{}", cause),
            AdminLinkProviderForUserError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AdminLinkProviderForUserError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            AdminLinkProviderForUserError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AdminLinkProviderForUserError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            AdminLinkProviderForUserError::UserNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AdminLinkProviderForUserError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(AdminListDevicesError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AdminListDevicesError::InvalidParameter(err.msg))
                }
                "InvalidUserPoolConfigurationException" => {
                    return RusotoError::Service(
                        AdminListDevicesError::InvalidUserPoolConfiguration(err.msg),
                    )
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminListDevicesError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AdminListDevicesError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AdminListDevicesError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AdminListDevicesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AdminListDevicesError::InternalError(ref cause) => write!(f, "{}", cause),
            AdminListDevicesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AdminListDevicesError::InvalidUserPoolConfiguration(ref cause) => {
                write!(f, "{}", cause)
            }
            AdminListDevicesError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            AdminListDevicesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AdminListDevicesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AdminListDevicesError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(AdminListGroupsForUserError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AdminListGroupsForUserError::InvalidParameter(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminListGroupsForUserError::NotAuthorized(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AdminListGroupsForUserError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AdminListGroupsForUserError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminListGroupsForUserError::UserNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AdminListGroupsForUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AdminListGroupsForUserError::InternalError(ref cause) => write!(f, "{}", cause),
            AdminListGroupsForUserError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AdminListGroupsForUserError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            AdminListGroupsForUserError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AdminListGroupsForUserError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            AdminListGroupsForUserError::UserNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AdminListGroupsForUserError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(AdminListUserAuthEventsError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AdminListUserAuthEventsError::InvalidParameter(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminListUserAuthEventsError::NotAuthorized(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AdminListUserAuthEventsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AdminListUserAuthEventsError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminListUserAuthEventsError::UserNotFound(
                        err.msg,
                    ))
                }
                "UserPoolAddOnNotEnabledException" => {
                    return RusotoError::Service(
                        AdminListUserAuthEventsError::UserPoolAddOnNotEnabled(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AdminListUserAuthEventsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AdminListUserAuthEventsError::InternalError(ref cause) => write!(f, "{}", cause),
            AdminListUserAuthEventsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AdminListUserAuthEventsError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            AdminListUserAuthEventsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AdminListUserAuthEventsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            AdminListUserAuthEventsError::UserNotFound(ref cause) => write!(f, "{}", cause),
            AdminListUserAuthEventsError::UserPoolAddOnNotEnabled(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AdminListUserAuthEventsError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(AdminRemoveUserFromGroupError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AdminRemoveUserFromGroupError::InvalidParameter(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminRemoveUserFromGroupError::NotAuthorized(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AdminRemoveUserFromGroupError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AdminRemoveUserFromGroupError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminRemoveUserFromGroupError::UserNotFound(
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
impl fmt::Display for AdminRemoveUserFromGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AdminRemoveUserFromGroupError::InternalError(ref cause) => write!(f, "{}", cause),
            AdminRemoveUserFromGroupError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AdminRemoveUserFromGroupError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            AdminRemoveUserFromGroupError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AdminRemoveUserFromGroupError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            AdminRemoveUserFromGroupError::UserNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AdminRemoveUserFromGroupError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(AdminResetUserPasswordError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidEmailRoleAccessPolicyException" => {
                    return RusotoError::Service(
                        AdminResetUserPasswordError::InvalidEmailRoleAccessPolicy(err.msg),
                    )
                }
                "InvalidLambdaResponseException" => {
                    return RusotoError::Service(
                        AdminResetUserPasswordError::InvalidLambdaResponse(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AdminResetUserPasswordError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidSmsRoleAccessPolicyException" => {
                    return RusotoError::Service(
                        AdminResetUserPasswordError::InvalidSmsRoleAccessPolicy(err.msg),
                    )
                }
                "InvalidSmsRoleTrustRelationshipException" => {
                    return RusotoError::Service(
                        AdminResetUserPasswordError::InvalidSmsRoleTrustRelationship(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(AdminResetUserPasswordError::LimitExceeded(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminResetUserPasswordError::NotAuthorized(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AdminResetUserPasswordError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AdminResetUserPasswordError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnexpectedLambdaException" => {
                    return RusotoError::Service(AdminResetUserPasswordError::UnexpectedLambda(
                        err.msg,
                    ))
                }
                "UserLambdaValidationException" => {
                    return RusotoError::Service(AdminResetUserPasswordError::UserLambdaValidation(
                        err.msg,
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminResetUserPasswordError::UserNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AdminResetUserPasswordError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AdminResetUserPasswordError::InternalError(ref cause) => write!(f, "{}", cause),
            AdminResetUserPasswordError::InvalidEmailRoleAccessPolicy(ref cause) => {
                write!(f, "{}", cause)
            }
            AdminResetUserPasswordError::InvalidLambdaResponse(ref cause) => write!(f, "{}", cause),
            AdminResetUserPasswordError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AdminResetUserPasswordError::InvalidSmsRoleAccessPolicy(ref cause) => {
                write!(f, "{}", cause)
            }
            AdminResetUserPasswordError::InvalidSmsRoleTrustRelationship(ref cause) => {
                write!(f, "{}", cause)
            }
            AdminResetUserPasswordError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            AdminResetUserPasswordError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            AdminResetUserPasswordError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AdminResetUserPasswordError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            AdminResetUserPasswordError::UnexpectedLambda(ref cause) => write!(f, "{}", cause),
            AdminResetUserPasswordError::UserLambdaValidation(ref cause) => write!(f, "{}", cause),
            AdminResetUserPasswordError::UserNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AdminResetUserPasswordError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AliasExistsException" => {
                    return RusotoError::Service(AdminRespondToAuthChallengeError::AliasExists(
                        err.msg,
                    ))
                }
                "CodeMismatchException" => {
                    return RusotoError::Service(AdminRespondToAuthChallengeError::CodeMismatch(
                        err.msg,
                    ))
                }
                "ExpiredCodeException" => {
                    return RusotoError::Service(AdminRespondToAuthChallengeError::ExpiredCode(
                        err.msg,
                    ))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(AdminRespondToAuthChallengeError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidLambdaResponseException" => {
                    return RusotoError::Service(
                        AdminRespondToAuthChallengeError::InvalidLambdaResponse(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        AdminRespondToAuthChallengeError::InvalidParameter(err.msg),
                    )
                }
                "InvalidPasswordException" => {
                    return RusotoError::Service(AdminRespondToAuthChallengeError::InvalidPassword(
                        err.msg,
                    ))
                }
                "InvalidSmsRoleAccessPolicyException" => {
                    return RusotoError::Service(
                        AdminRespondToAuthChallengeError::InvalidSmsRoleAccessPolicy(err.msg),
                    )
                }
                "InvalidSmsRoleTrustRelationshipException" => {
                    return RusotoError::Service(
                        AdminRespondToAuthChallengeError::InvalidSmsRoleTrustRelationship(err.msg),
                    )
                }
                "InvalidUserPoolConfigurationException" => {
                    return RusotoError::Service(
                        AdminRespondToAuthChallengeError::InvalidUserPoolConfiguration(err.msg),
                    )
                }
                "MFAMethodNotFoundException" => {
                    return RusotoError::Service(
                        AdminRespondToAuthChallengeError::MFAMethodNotFound(err.msg),
                    )
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminRespondToAuthChallengeError::NotAuthorized(
                        err.msg,
                    ))
                }
                "PasswordResetRequiredException" => {
                    return RusotoError::Service(
                        AdminRespondToAuthChallengeError::PasswordResetRequired(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        AdminRespondToAuthChallengeError::ResourceNotFound(err.msg),
                    )
                }
                "SoftwareTokenMFANotFoundException" => {
                    return RusotoError::Service(
                        AdminRespondToAuthChallengeError::SoftwareTokenMFANotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AdminRespondToAuthChallengeError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnexpectedLambdaException" => {
                    return RusotoError::Service(
                        AdminRespondToAuthChallengeError::UnexpectedLambda(err.msg),
                    )
                }
                "UserLambdaValidationException" => {
                    return RusotoError::Service(
                        AdminRespondToAuthChallengeError::UserLambdaValidation(err.msg),
                    )
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(
                        AdminRespondToAuthChallengeError::UserNotConfirmed(err.msg),
                    )
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminRespondToAuthChallengeError::UserNotFound(
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
impl fmt::Display for AdminRespondToAuthChallengeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AdminRespondToAuthChallengeError::AliasExists(ref cause) => write!(f, "{}", cause),
            AdminRespondToAuthChallengeError::CodeMismatch(ref cause) => write!(f, "{}", cause),
            AdminRespondToAuthChallengeError::ExpiredCode(ref cause) => write!(f, "{}", cause),
            AdminRespondToAuthChallengeError::InternalError(ref cause) => write!(f, "{}", cause),
            AdminRespondToAuthChallengeError::InvalidLambdaResponse(ref cause) => {
                write!(f, "{}", cause)
            }
            AdminRespondToAuthChallengeError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AdminRespondToAuthChallengeError::InvalidPassword(ref cause) => write!(f, "{}", cause),
            AdminRespondToAuthChallengeError::InvalidSmsRoleAccessPolicy(ref cause) => {
                write!(f, "{}", cause)
            }
            AdminRespondToAuthChallengeError::InvalidSmsRoleTrustRelationship(ref cause) => {
                write!(f, "{}", cause)
            }
            AdminRespondToAuthChallengeError::InvalidUserPoolConfiguration(ref cause) => {
                write!(f, "{}", cause)
            }
            AdminRespondToAuthChallengeError::MFAMethodNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            AdminRespondToAuthChallengeError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            AdminRespondToAuthChallengeError::PasswordResetRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            AdminRespondToAuthChallengeError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AdminRespondToAuthChallengeError::SoftwareTokenMFANotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            AdminRespondToAuthChallengeError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            AdminRespondToAuthChallengeError::UnexpectedLambda(ref cause) => write!(f, "{}", cause),
            AdminRespondToAuthChallengeError::UserLambdaValidation(ref cause) => {
                write!(f, "{}", cause)
            }
            AdminRespondToAuthChallengeError::UserNotConfirmed(ref cause) => write!(f, "{}", cause),
            AdminRespondToAuthChallengeError::UserNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AdminRespondToAuthChallengeError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(AdminSetUserMFAPreferenceError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AdminSetUserMFAPreferenceError::InvalidParameter(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminSetUserMFAPreferenceError::NotAuthorized(
                        err.msg,
                    ))
                }
                "PasswordResetRequiredException" => {
                    return RusotoError::Service(
                        AdminSetUserMFAPreferenceError::PasswordResetRequired(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AdminSetUserMFAPreferenceError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(AdminSetUserMFAPreferenceError::UserNotConfirmed(
                        err.msg,
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminSetUserMFAPreferenceError::UserNotFound(
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
impl fmt::Display for AdminSetUserMFAPreferenceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AdminSetUserMFAPreferenceError::InternalError(ref cause) => write!(f, "{}", cause),
            AdminSetUserMFAPreferenceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AdminSetUserMFAPreferenceError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            AdminSetUserMFAPreferenceError::PasswordResetRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            AdminSetUserMFAPreferenceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AdminSetUserMFAPreferenceError::UserNotConfirmed(ref cause) => write!(f, "{}", cause),
            AdminSetUserMFAPreferenceError::UserNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AdminSetUserMFAPreferenceError {}
/// Errors returned by AdminSetUserPassword
#[derive(Debug, PartialEq)]
pub enum AdminSetUserPasswordError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid password.</p>
    InvalidPassword(String),
    /// <p>This exception is thrown when a user is not authorized.</p>
    NotAuthorized(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequests(String),
    /// <p>This exception is thrown when a user is not found.</p>
    UserNotFound(String),
}

impl AdminSetUserPasswordError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AdminSetUserPasswordError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(AdminSetUserPasswordError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AdminSetUserPasswordError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidPasswordException" => {
                    return RusotoError::Service(AdminSetUserPasswordError::InvalidPassword(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminSetUserPasswordError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AdminSetUserPasswordError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AdminSetUserPasswordError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminSetUserPasswordError::UserNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AdminSetUserPasswordError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AdminSetUserPasswordError::InternalError(ref cause) => write!(f, "{}", cause),
            AdminSetUserPasswordError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AdminSetUserPasswordError::InvalidPassword(ref cause) => write!(f, "{}", cause),
            AdminSetUserPasswordError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            AdminSetUserPasswordError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AdminSetUserPasswordError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            AdminSetUserPasswordError::UserNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AdminSetUserPasswordError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(AdminSetUserSettingsError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AdminSetUserSettingsError::InvalidParameter(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminSetUserSettingsError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AdminSetUserSettingsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminSetUserSettingsError::UserNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AdminSetUserSettingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AdminSetUserSettingsError::InternalError(ref cause) => write!(f, "{}", cause),
            AdminSetUserSettingsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AdminSetUserSettingsError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            AdminSetUserSettingsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AdminSetUserSettingsError::UserNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AdminSetUserSettingsError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(AdminUpdateAuthEventFeedbackError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        AdminUpdateAuthEventFeedbackError::InvalidParameter(err.msg),
                    )
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminUpdateAuthEventFeedbackError::NotAuthorized(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        AdminUpdateAuthEventFeedbackError::ResourceNotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        AdminUpdateAuthEventFeedbackError::TooManyRequests(err.msg),
                    )
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminUpdateAuthEventFeedbackError::UserNotFound(
                        err.msg,
                    ))
                }
                "UserPoolAddOnNotEnabledException" => {
                    return RusotoError::Service(
                        AdminUpdateAuthEventFeedbackError::UserPoolAddOnNotEnabled(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AdminUpdateAuthEventFeedbackError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AdminUpdateAuthEventFeedbackError::InternalError(ref cause) => write!(f, "{}", cause),
            AdminUpdateAuthEventFeedbackError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            AdminUpdateAuthEventFeedbackError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            AdminUpdateAuthEventFeedbackError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            AdminUpdateAuthEventFeedbackError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            AdminUpdateAuthEventFeedbackError::UserNotFound(ref cause) => write!(f, "{}", cause),
            AdminUpdateAuthEventFeedbackError::UserPoolAddOnNotEnabled(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AdminUpdateAuthEventFeedbackError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(AdminUpdateDeviceStatusError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AdminUpdateDeviceStatusError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidUserPoolConfigurationException" => {
                    return RusotoError::Service(
                        AdminUpdateDeviceStatusError::InvalidUserPoolConfiguration(err.msg),
                    )
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminUpdateDeviceStatusError::NotAuthorized(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AdminUpdateDeviceStatusError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AdminUpdateDeviceStatusError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminUpdateDeviceStatusError::UserNotFound(
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
impl fmt::Display for AdminUpdateDeviceStatusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AdminUpdateDeviceStatusError::InternalError(ref cause) => write!(f, "{}", cause),
            AdminUpdateDeviceStatusError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AdminUpdateDeviceStatusError::InvalidUserPoolConfiguration(ref cause) => {
                write!(f, "{}", cause)
            }
            AdminUpdateDeviceStatusError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            AdminUpdateDeviceStatusError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AdminUpdateDeviceStatusError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            AdminUpdateDeviceStatusError::UserNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AdminUpdateDeviceStatusError {}
/// Errors returned by AdminUpdateUserAttributes
#[derive(Debug, PartialEq)]
pub enum AdminUpdateUserAttributesError {
    /// <p>This exception is thrown when a user tries to confirm the account with an email or phone number that has already been supplied as an alias from a different account. This exception tells user that an account with this email or phone already exists.</p>
    AliasExists(String),
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AliasExistsException" => {
                    return RusotoError::Service(AdminUpdateUserAttributesError::AliasExists(
                        err.msg,
                    ))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(AdminUpdateUserAttributesError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidEmailRoleAccessPolicyException" => {
                    return RusotoError::Service(
                        AdminUpdateUserAttributesError::InvalidEmailRoleAccessPolicy(err.msg),
                    )
                }
                "InvalidLambdaResponseException" => {
                    return RusotoError::Service(
                        AdminUpdateUserAttributesError::InvalidLambdaResponse(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AdminUpdateUserAttributesError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidSmsRoleAccessPolicyException" => {
                    return RusotoError::Service(
                        AdminUpdateUserAttributesError::InvalidSmsRoleAccessPolicy(err.msg),
                    )
                }
                "InvalidSmsRoleTrustRelationshipException" => {
                    return RusotoError::Service(
                        AdminUpdateUserAttributesError::InvalidSmsRoleTrustRelationship(err.msg),
                    )
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminUpdateUserAttributesError::NotAuthorized(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AdminUpdateUserAttributesError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AdminUpdateUserAttributesError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnexpectedLambdaException" => {
                    return RusotoError::Service(AdminUpdateUserAttributesError::UnexpectedLambda(
                        err.msg,
                    ))
                }
                "UserLambdaValidationException" => {
                    return RusotoError::Service(
                        AdminUpdateUserAttributesError::UserLambdaValidation(err.msg),
                    )
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminUpdateUserAttributesError::UserNotFound(
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
impl fmt::Display for AdminUpdateUserAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AdminUpdateUserAttributesError::AliasExists(ref cause) => write!(f, "{}", cause),
            AdminUpdateUserAttributesError::InternalError(ref cause) => write!(f, "{}", cause),
            AdminUpdateUserAttributesError::InvalidEmailRoleAccessPolicy(ref cause) => {
                write!(f, "{}", cause)
            }
            AdminUpdateUserAttributesError::InvalidLambdaResponse(ref cause) => {
                write!(f, "{}", cause)
            }
            AdminUpdateUserAttributesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AdminUpdateUserAttributesError::InvalidSmsRoleAccessPolicy(ref cause) => {
                write!(f, "{}", cause)
            }
            AdminUpdateUserAttributesError::InvalidSmsRoleTrustRelationship(ref cause) => {
                write!(f, "{}", cause)
            }
            AdminUpdateUserAttributesError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            AdminUpdateUserAttributesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AdminUpdateUserAttributesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            AdminUpdateUserAttributesError::UnexpectedLambda(ref cause) => write!(f, "{}", cause),
            AdminUpdateUserAttributesError::UserLambdaValidation(ref cause) => {
                write!(f, "{}", cause)
            }
            AdminUpdateUserAttributesError::UserNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AdminUpdateUserAttributesError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(AdminUserGlobalSignOutError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AdminUserGlobalSignOutError::InvalidParameter(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AdminUserGlobalSignOutError::NotAuthorized(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AdminUserGlobalSignOutError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AdminUserGlobalSignOutError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(AdminUserGlobalSignOutError::UserNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AdminUserGlobalSignOutError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AdminUserGlobalSignOutError::InternalError(ref cause) => write!(f, "{}", cause),
            AdminUserGlobalSignOutError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AdminUserGlobalSignOutError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            AdminUserGlobalSignOutError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AdminUserGlobalSignOutError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            AdminUserGlobalSignOutError::UserNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AdminUserGlobalSignOutError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(AssociateSoftwareTokenError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AssociateSoftwareTokenError::InvalidParameter(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(AssociateSoftwareTokenError::NotAuthorized(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AssociateSoftwareTokenError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "SoftwareTokenMFANotFoundException" => {
                    return RusotoError::Service(
                        AssociateSoftwareTokenError::SoftwareTokenMFANotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AssociateSoftwareTokenError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateSoftwareTokenError::InternalError(ref cause) => write!(f, "{}", cause),
            AssociateSoftwareTokenError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AssociateSoftwareTokenError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            AssociateSoftwareTokenError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AssociateSoftwareTokenError::SoftwareTokenMFANotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AssociateSoftwareTokenError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(ChangePasswordError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ChangePasswordError::InvalidParameter(err.msg))
                }
                "InvalidPasswordException" => {
                    return RusotoError::Service(ChangePasswordError::InvalidPassword(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ChangePasswordError::LimitExceeded(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(ChangePasswordError::NotAuthorized(err.msg))
                }
                "PasswordResetRequiredException" => {
                    return RusotoError::Service(ChangePasswordError::PasswordResetRequired(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ChangePasswordError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ChangePasswordError::TooManyRequests(err.msg))
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(ChangePasswordError::UserNotConfirmed(err.msg))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(ChangePasswordError::UserNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ChangePasswordError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ChangePasswordError::InternalError(ref cause) => write!(f, "{}", cause),
            ChangePasswordError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ChangePasswordError::InvalidPassword(ref cause) => write!(f, "{}", cause),
            ChangePasswordError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ChangePasswordError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            ChangePasswordError::PasswordResetRequired(ref cause) => write!(f, "{}", cause),
            ChangePasswordError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ChangePasswordError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            ChangePasswordError::UserNotConfirmed(ref cause) => write!(f, "{}", cause),
            ChangePasswordError::UserNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ChangePasswordError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(ConfirmDeviceError::InternalError(err.msg))
                }
                "InvalidLambdaResponseException" => {
                    return RusotoError::Service(ConfirmDeviceError::InvalidLambdaResponse(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ConfirmDeviceError::InvalidParameter(err.msg))
                }
                "InvalidPasswordException" => {
                    return RusotoError::Service(ConfirmDeviceError::InvalidPassword(err.msg))
                }
                "InvalidUserPoolConfigurationException" => {
                    return RusotoError::Service(ConfirmDeviceError::InvalidUserPoolConfiguration(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(ConfirmDeviceError::NotAuthorized(err.msg))
                }
                "PasswordResetRequiredException" => {
                    return RusotoError::Service(ConfirmDeviceError::PasswordResetRequired(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ConfirmDeviceError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ConfirmDeviceError::TooManyRequests(err.msg))
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(ConfirmDeviceError::UserNotConfirmed(err.msg))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(ConfirmDeviceError::UserNotFound(err.msg))
                }
                "UsernameExistsException" => {
                    return RusotoError::Service(ConfirmDeviceError::UsernameExists(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ConfirmDeviceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfirmDeviceError::InternalError(ref cause) => write!(f, "{}", cause),
            ConfirmDeviceError::InvalidLambdaResponse(ref cause) => write!(f, "{}", cause),
            ConfirmDeviceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ConfirmDeviceError::InvalidPassword(ref cause) => write!(f, "{}", cause),
            ConfirmDeviceError::InvalidUserPoolConfiguration(ref cause) => write!(f, "{}", cause),
            ConfirmDeviceError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            ConfirmDeviceError::PasswordResetRequired(ref cause) => write!(f, "{}", cause),
            ConfirmDeviceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ConfirmDeviceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            ConfirmDeviceError::UserNotConfirmed(ref cause) => write!(f, "{}", cause),
            ConfirmDeviceError::UserNotFound(ref cause) => write!(f, "{}", cause),
            ConfirmDeviceError::UsernameExists(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ConfirmDeviceError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CodeMismatchException" => {
                    return RusotoError::Service(ConfirmForgotPasswordError::CodeMismatch(err.msg))
                }
                "ExpiredCodeException" => {
                    return RusotoError::Service(ConfirmForgotPasswordError::ExpiredCode(err.msg))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(ConfirmForgotPasswordError::InternalError(err.msg))
                }
                "InvalidLambdaResponseException" => {
                    return RusotoError::Service(ConfirmForgotPasswordError::InvalidLambdaResponse(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ConfirmForgotPasswordError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidPasswordException" => {
                    return RusotoError::Service(ConfirmForgotPasswordError::InvalidPassword(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ConfirmForgotPasswordError::LimitExceeded(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(ConfirmForgotPasswordError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ConfirmForgotPasswordError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyFailedAttemptsException" => {
                    return RusotoError::Service(ConfirmForgotPasswordError::TooManyFailedAttempts(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ConfirmForgotPasswordError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnexpectedLambdaException" => {
                    return RusotoError::Service(ConfirmForgotPasswordError::UnexpectedLambda(
                        err.msg,
                    ))
                }
                "UserLambdaValidationException" => {
                    return RusotoError::Service(ConfirmForgotPasswordError::UserLambdaValidation(
                        err.msg,
                    ))
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(ConfirmForgotPasswordError::UserNotConfirmed(
                        err.msg,
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(ConfirmForgotPasswordError::UserNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ConfirmForgotPasswordError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfirmForgotPasswordError::CodeMismatch(ref cause) => write!(f, "{}", cause),
            ConfirmForgotPasswordError::ExpiredCode(ref cause) => write!(f, "{}", cause),
            ConfirmForgotPasswordError::InternalError(ref cause) => write!(f, "{}", cause),
            ConfirmForgotPasswordError::InvalidLambdaResponse(ref cause) => write!(f, "{}", cause),
            ConfirmForgotPasswordError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ConfirmForgotPasswordError::InvalidPassword(ref cause) => write!(f, "{}", cause),
            ConfirmForgotPasswordError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ConfirmForgotPasswordError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            ConfirmForgotPasswordError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ConfirmForgotPasswordError::TooManyFailedAttempts(ref cause) => write!(f, "{}", cause),
            ConfirmForgotPasswordError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            ConfirmForgotPasswordError::UnexpectedLambda(ref cause) => write!(f, "{}", cause),
            ConfirmForgotPasswordError::UserLambdaValidation(ref cause) => write!(f, "{}", cause),
            ConfirmForgotPasswordError::UserNotConfirmed(ref cause) => write!(f, "{}", cause),
            ConfirmForgotPasswordError::UserNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ConfirmForgotPasswordError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AliasExistsException" => {
                    return RusotoError::Service(ConfirmSignUpError::AliasExists(err.msg))
                }
                "CodeMismatchException" => {
                    return RusotoError::Service(ConfirmSignUpError::CodeMismatch(err.msg))
                }
                "ExpiredCodeException" => {
                    return RusotoError::Service(ConfirmSignUpError::ExpiredCode(err.msg))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(ConfirmSignUpError::InternalError(err.msg))
                }
                "InvalidLambdaResponseException" => {
                    return RusotoError::Service(ConfirmSignUpError::InvalidLambdaResponse(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ConfirmSignUpError::InvalidParameter(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ConfirmSignUpError::LimitExceeded(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(ConfirmSignUpError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ConfirmSignUpError::ResourceNotFound(err.msg))
                }
                "TooManyFailedAttemptsException" => {
                    return RusotoError::Service(ConfirmSignUpError::TooManyFailedAttempts(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ConfirmSignUpError::TooManyRequests(err.msg))
                }
                "UnexpectedLambdaException" => {
                    return RusotoError::Service(ConfirmSignUpError::UnexpectedLambda(err.msg))
                }
                "UserLambdaValidationException" => {
                    return RusotoError::Service(ConfirmSignUpError::UserLambdaValidation(err.msg))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(ConfirmSignUpError::UserNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ConfirmSignUpError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfirmSignUpError::AliasExists(ref cause) => write!(f, "{}", cause),
            ConfirmSignUpError::CodeMismatch(ref cause) => write!(f, "{}", cause),
            ConfirmSignUpError::ExpiredCode(ref cause) => write!(f, "{}", cause),
            ConfirmSignUpError::InternalError(ref cause) => write!(f, "{}", cause),
            ConfirmSignUpError::InvalidLambdaResponse(ref cause) => write!(f, "{}", cause),
            ConfirmSignUpError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ConfirmSignUpError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ConfirmSignUpError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            ConfirmSignUpError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ConfirmSignUpError::TooManyFailedAttempts(ref cause) => write!(f, "{}", cause),
            ConfirmSignUpError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            ConfirmSignUpError::UnexpectedLambda(ref cause) => write!(f, "{}", cause),
            ConfirmSignUpError::UserLambdaValidation(ref cause) => write!(f, "{}", cause),
            ConfirmSignUpError::UserNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ConfirmSignUpError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "GroupExistsException" => {
                    return RusotoError::Service(CreateGroupError::GroupExists(err.msg))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(CreateGroupError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateGroupError::InvalidParameter(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateGroupError::LimitExceeded(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(CreateGroupError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateGroupError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateGroupError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateGroupError::GroupExists(ref cause) => write!(f, "{}", cause),
            CreateGroupError::InternalError(ref cause) => write!(f, "{}", cause),
            CreateGroupError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateGroupError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateGroupError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            CreateGroupError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateGroupError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateGroupError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DuplicateProviderException" => {
                    return RusotoError::Service(CreateIdentityProviderError::DuplicateProvider(
                        err.msg,
                    ))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(CreateIdentityProviderError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateIdentityProviderError::InvalidParameter(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateIdentityProviderError::LimitExceeded(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(CreateIdentityProviderError::NotAuthorized(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateIdentityProviderError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateIdentityProviderError::TooManyRequests(
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
impl fmt::Display for CreateIdentityProviderError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateIdentityProviderError::DuplicateProvider(ref cause) => write!(f, "{}", cause),
            CreateIdentityProviderError::InternalError(ref cause) => write!(f, "{}", cause),
            CreateIdentityProviderError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateIdentityProviderError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateIdentityProviderError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            CreateIdentityProviderError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateIdentityProviderError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateIdentityProviderError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(CreateResourceServerError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateResourceServerError::InvalidParameter(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateResourceServerError::LimitExceeded(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(CreateResourceServerError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateResourceServerError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateResourceServerError::TooManyRequests(
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
impl fmt::Display for CreateResourceServerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateResourceServerError::InternalError(ref cause) => write!(f, "{}", cause),
            CreateResourceServerError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateResourceServerError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateResourceServerError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            CreateResourceServerError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateResourceServerError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateResourceServerError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(CreateUserImportJobError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateUserImportJobError::InvalidParameter(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateUserImportJobError::LimitExceeded(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(CreateUserImportJobError::NotAuthorized(err.msg))
                }
                "PreconditionNotMetException" => {
                    return RusotoError::Service(CreateUserImportJobError::PreconditionNotMet(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateUserImportJobError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateUserImportJobError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateUserImportJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateUserImportJobError::InternalError(ref cause) => write!(f, "{}", cause),
            CreateUserImportJobError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateUserImportJobError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateUserImportJobError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            CreateUserImportJobError::PreconditionNotMet(ref cause) => write!(f, "{}", cause),
            CreateUserImportJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateUserImportJobError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateUserImportJobError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(CreateUserPoolError::InternalError(err.msg))
                }
                "InvalidEmailRoleAccessPolicyException" => {
                    return RusotoError::Service(CreateUserPoolError::InvalidEmailRoleAccessPolicy(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateUserPoolError::InvalidParameter(err.msg))
                }
                "InvalidSmsRoleAccessPolicyException" => {
                    return RusotoError::Service(CreateUserPoolError::InvalidSmsRoleAccessPolicy(
                        err.msg,
                    ))
                }
                "InvalidSmsRoleTrustRelationshipException" => {
                    return RusotoError::Service(
                        CreateUserPoolError::InvalidSmsRoleTrustRelationship(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateUserPoolError::LimitExceeded(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(CreateUserPoolError::NotAuthorized(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateUserPoolError::TooManyRequests(err.msg))
                }
                "UserPoolTaggingException" => {
                    return RusotoError::Service(CreateUserPoolError::UserPoolTagging(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateUserPoolError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateUserPoolError::InternalError(ref cause) => write!(f, "{}", cause),
            CreateUserPoolError::InvalidEmailRoleAccessPolicy(ref cause) => write!(f, "{}", cause),
            CreateUserPoolError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateUserPoolError::InvalidSmsRoleAccessPolicy(ref cause) => write!(f, "{}", cause),
            CreateUserPoolError::InvalidSmsRoleTrustRelationship(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateUserPoolError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateUserPoolError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            CreateUserPoolError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            CreateUserPoolError::UserPoolTagging(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateUserPoolError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(CreateUserPoolClientError::InternalError(err.msg))
                }
                "InvalidOAuthFlowException" => {
                    return RusotoError::Service(CreateUserPoolClientError::InvalidOAuthFlow(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateUserPoolClientError::InvalidParameter(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateUserPoolClientError::LimitExceeded(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(CreateUserPoolClientError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateUserPoolClientError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ScopeDoesNotExistException" => {
                    return RusotoError::Service(CreateUserPoolClientError::ScopeDoesNotExist(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateUserPoolClientError::TooManyRequests(
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
impl fmt::Display for CreateUserPoolClientError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateUserPoolClientError::InternalError(ref cause) => write!(f, "{}", cause),
            CreateUserPoolClientError::InvalidOAuthFlow(ref cause) => write!(f, "{}", cause),
            CreateUserPoolClientError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateUserPoolClientError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateUserPoolClientError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            CreateUserPoolClientError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateUserPoolClientError::ScopeDoesNotExist(ref cause) => write!(f, "{}", cause),
            CreateUserPoolClientError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateUserPoolClientError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(CreateUserPoolDomainError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateUserPoolDomainError::InvalidParameter(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateUserPoolDomainError::LimitExceeded(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(CreateUserPoolDomainError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateUserPoolDomainError::ResourceNotFound(
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
impl fmt::Display for CreateUserPoolDomainError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateUserPoolDomainError::InternalError(ref cause) => write!(f, "{}", cause),
            CreateUserPoolDomainError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateUserPoolDomainError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateUserPoolDomainError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            CreateUserPoolDomainError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateUserPoolDomainError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(DeleteGroupError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteGroupError::InvalidParameter(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(DeleteGroupError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteGroupError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteGroupError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteGroupError::InternalError(ref cause) => write!(f, "{}", cause),
            DeleteGroupError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteGroupError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            DeleteGroupError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteGroupError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteGroupError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(DeleteIdentityProviderError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteIdentityProviderError::InvalidParameter(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(DeleteIdentityProviderError::NotAuthorized(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteIdentityProviderError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteIdentityProviderError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnsupportedIdentityProviderException" => {
                    return RusotoError::Service(
                        DeleteIdentityProviderError::UnsupportedIdentityProvider(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteIdentityProviderError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteIdentityProviderError::InternalError(ref cause) => write!(f, "{}", cause),
            DeleteIdentityProviderError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteIdentityProviderError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            DeleteIdentityProviderError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteIdentityProviderError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            DeleteIdentityProviderError::UnsupportedIdentityProvider(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteIdentityProviderError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(DeleteResourceServerError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteResourceServerError::InvalidParameter(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(DeleteResourceServerError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteResourceServerError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteResourceServerError::TooManyRequests(
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
impl fmt::Display for DeleteResourceServerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteResourceServerError::InternalError(ref cause) => write!(f, "{}", cause),
            DeleteResourceServerError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteResourceServerError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            DeleteResourceServerError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteResourceServerError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteResourceServerError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(DeleteUserError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteUserError::InvalidParameter(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(DeleteUserError::NotAuthorized(err.msg))
                }
                "PasswordResetRequiredException" => {
                    return RusotoError::Service(DeleteUserError::PasswordResetRequired(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteUserError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteUserError::TooManyRequests(err.msg))
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(DeleteUserError::UserNotConfirmed(err.msg))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(DeleteUserError::UserNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteUserError::InternalError(ref cause) => write!(f, "{}", cause),
            DeleteUserError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteUserError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            DeleteUserError::PasswordResetRequired(ref cause) => write!(f, "{}", cause),
            DeleteUserError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteUserError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            DeleteUserError::UserNotConfirmed(ref cause) => write!(f, "{}", cause),
            DeleteUserError::UserNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteUserError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(DeleteUserAttributesError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteUserAttributesError::InvalidParameter(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(DeleteUserAttributesError::NotAuthorized(err.msg))
                }
                "PasswordResetRequiredException" => {
                    return RusotoError::Service(DeleteUserAttributesError::PasswordResetRequired(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteUserAttributesError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteUserAttributesError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(DeleteUserAttributesError::UserNotConfirmed(
                        err.msg,
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(DeleteUserAttributesError::UserNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteUserAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteUserAttributesError::InternalError(ref cause) => write!(f, "{}", cause),
            DeleteUserAttributesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteUserAttributesError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            DeleteUserAttributesError::PasswordResetRequired(ref cause) => write!(f, "{}", cause),
            DeleteUserAttributesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteUserAttributesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            DeleteUserAttributesError::UserNotConfirmed(ref cause) => write!(f, "{}", cause),
            DeleteUserAttributesError::UserNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteUserAttributesError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(DeleteUserPoolError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteUserPoolError::InvalidParameter(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(DeleteUserPoolError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteUserPoolError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteUserPoolError::TooManyRequests(err.msg))
                }
                "UserImportInProgressException" => {
                    return RusotoError::Service(DeleteUserPoolError::UserImportInProgress(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteUserPoolError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteUserPoolError::InternalError(ref cause) => write!(f, "{}", cause),
            DeleteUserPoolError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteUserPoolError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            DeleteUserPoolError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteUserPoolError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            DeleteUserPoolError::UserImportInProgress(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteUserPoolError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(DeleteUserPoolClientError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteUserPoolClientError::InvalidParameter(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(DeleteUserPoolClientError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteUserPoolClientError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteUserPoolClientError::TooManyRequests(
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
impl fmt::Display for DeleteUserPoolClientError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteUserPoolClientError::InternalError(ref cause) => write!(f, "{}", cause),
            DeleteUserPoolClientError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteUserPoolClientError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            DeleteUserPoolClientError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteUserPoolClientError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteUserPoolClientError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(DeleteUserPoolDomainError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteUserPoolDomainError::InvalidParameter(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(DeleteUserPoolDomainError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteUserPoolDomainError::ResourceNotFound(
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
impl fmt::Display for DeleteUserPoolDomainError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteUserPoolDomainError::InternalError(ref cause) => write!(f, "{}", cause),
            DeleteUserPoolDomainError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteUserPoolDomainError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            DeleteUserPoolDomainError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteUserPoolDomainError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(DescribeIdentityProviderError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeIdentityProviderError::InvalidParameter(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(DescribeIdentityProviderError::NotAuthorized(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeIdentityProviderError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeIdentityProviderError::TooManyRequests(
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
impl fmt::Display for DescribeIdentityProviderError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeIdentityProviderError::InternalError(ref cause) => write!(f, "{}", cause),
            DescribeIdentityProviderError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeIdentityProviderError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            DescribeIdentityProviderError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeIdentityProviderError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeIdentityProviderError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(DescribeResourceServerError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeResourceServerError::InvalidParameter(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(DescribeResourceServerError::NotAuthorized(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeResourceServerError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeResourceServerError::TooManyRequests(
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
impl fmt::Display for DescribeResourceServerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeResourceServerError::InternalError(ref cause) => write!(f, "{}", cause),
            DescribeResourceServerError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeResourceServerError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            DescribeResourceServerError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeResourceServerError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeResourceServerError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(DescribeRiskConfigurationError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeRiskConfigurationError::InvalidParameter(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(DescribeRiskConfigurationError::NotAuthorized(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeRiskConfigurationError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeRiskConfigurationError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UserPoolAddOnNotEnabledException" => {
                    return RusotoError::Service(
                        DescribeRiskConfigurationError::UserPoolAddOnNotEnabled(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeRiskConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeRiskConfigurationError::InternalError(ref cause) => write!(f, "{}", cause),
            DescribeRiskConfigurationError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeRiskConfigurationError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            DescribeRiskConfigurationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeRiskConfigurationError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            DescribeRiskConfigurationError::UserPoolAddOnNotEnabled(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeRiskConfigurationError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(DescribeUserImportJobError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeUserImportJobError::InvalidParameter(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(DescribeUserImportJobError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeUserImportJobError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeUserImportJobError::TooManyRequests(
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
impl fmt::Display for DescribeUserImportJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeUserImportJobError::InternalError(ref cause) => write!(f, "{}", cause),
            DescribeUserImportJobError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeUserImportJobError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            DescribeUserImportJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeUserImportJobError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeUserImportJobError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(DescribeUserPoolError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeUserPoolError::InvalidParameter(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(DescribeUserPoolError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeUserPoolError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeUserPoolError::TooManyRequests(err.msg))
                }
                "UserPoolTaggingException" => {
                    return RusotoError::Service(DescribeUserPoolError::UserPoolTagging(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeUserPoolError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeUserPoolError::InternalError(ref cause) => write!(f, "{}", cause),
            DescribeUserPoolError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeUserPoolError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            DescribeUserPoolError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeUserPoolError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            DescribeUserPoolError::UserPoolTagging(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeUserPoolError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(DescribeUserPoolClientError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeUserPoolClientError::InvalidParameter(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(DescribeUserPoolClientError::NotAuthorized(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeUserPoolClientError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeUserPoolClientError::TooManyRequests(
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
impl fmt::Display for DescribeUserPoolClientError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeUserPoolClientError::InternalError(ref cause) => write!(f, "{}", cause),
            DescribeUserPoolClientError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeUserPoolClientError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            DescribeUserPoolClientError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeUserPoolClientError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeUserPoolClientError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(DescribeUserPoolDomainError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeUserPoolDomainError::InvalidParameter(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(DescribeUserPoolDomainError::NotAuthorized(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeUserPoolDomainError::ResourceNotFound(
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
impl fmt::Display for DescribeUserPoolDomainError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeUserPoolDomainError::InternalError(ref cause) => write!(f, "{}", cause),
            DescribeUserPoolDomainError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeUserPoolDomainError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            DescribeUserPoolDomainError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeUserPoolDomainError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(ForgetDeviceError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ForgetDeviceError::InvalidParameter(err.msg))
                }
                "InvalidUserPoolConfigurationException" => {
                    return RusotoError::Service(ForgetDeviceError::InvalidUserPoolConfiguration(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(ForgetDeviceError::NotAuthorized(err.msg))
                }
                "PasswordResetRequiredException" => {
                    return RusotoError::Service(ForgetDeviceError::PasswordResetRequired(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ForgetDeviceError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ForgetDeviceError::TooManyRequests(err.msg))
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(ForgetDeviceError::UserNotConfirmed(err.msg))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(ForgetDeviceError::UserNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ForgetDeviceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ForgetDeviceError::InternalError(ref cause) => write!(f, "{}", cause),
            ForgetDeviceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ForgetDeviceError::InvalidUserPoolConfiguration(ref cause) => write!(f, "{}", cause),
            ForgetDeviceError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            ForgetDeviceError::PasswordResetRequired(ref cause) => write!(f, "{}", cause),
            ForgetDeviceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ForgetDeviceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            ForgetDeviceError::UserNotConfirmed(ref cause) => write!(f, "{}", cause),
            ForgetDeviceError::UserNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ForgetDeviceError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CodeDeliveryFailureException" => {
                    return RusotoError::Service(ForgotPasswordError::CodeDeliveryFailure(err.msg))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(ForgotPasswordError::InternalError(err.msg))
                }
                "InvalidEmailRoleAccessPolicyException" => {
                    return RusotoError::Service(ForgotPasswordError::InvalidEmailRoleAccessPolicy(
                        err.msg,
                    ))
                }
                "InvalidLambdaResponseException" => {
                    return RusotoError::Service(ForgotPasswordError::InvalidLambdaResponse(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ForgotPasswordError::InvalidParameter(err.msg))
                }
                "InvalidSmsRoleAccessPolicyException" => {
                    return RusotoError::Service(ForgotPasswordError::InvalidSmsRoleAccessPolicy(
                        err.msg,
                    ))
                }
                "InvalidSmsRoleTrustRelationshipException" => {
                    return RusotoError::Service(
                        ForgotPasswordError::InvalidSmsRoleTrustRelationship(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ForgotPasswordError::LimitExceeded(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(ForgotPasswordError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ForgotPasswordError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ForgotPasswordError::TooManyRequests(err.msg))
                }
                "UnexpectedLambdaException" => {
                    return RusotoError::Service(ForgotPasswordError::UnexpectedLambda(err.msg))
                }
                "UserLambdaValidationException" => {
                    return RusotoError::Service(ForgotPasswordError::UserLambdaValidation(err.msg))
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(ForgotPasswordError::UserNotConfirmed(err.msg))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(ForgotPasswordError::UserNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ForgotPasswordError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ForgotPasswordError::CodeDeliveryFailure(ref cause) => write!(f, "{}", cause),
            ForgotPasswordError::InternalError(ref cause) => write!(f, "{}", cause),
            ForgotPasswordError::InvalidEmailRoleAccessPolicy(ref cause) => write!(f, "{}", cause),
            ForgotPasswordError::InvalidLambdaResponse(ref cause) => write!(f, "{}", cause),
            ForgotPasswordError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ForgotPasswordError::InvalidSmsRoleAccessPolicy(ref cause) => write!(f, "{}", cause),
            ForgotPasswordError::InvalidSmsRoleTrustRelationship(ref cause) => {
                write!(f, "{}", cause)
            }
            ForgotPasswordError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ForgotPasswordError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            ForgotPasswordError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ForgotPasswordError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            ForgotPasswordError::UnexpectedLambda(ref cause) => write!(f, "{}", cause),
            ForgotPasswordError::UserLambdaValidation(ref cause) => write!(f, "{}", cause),
            ForgotPasswordError::UserNotConfirmed(ref cause) => write!(f, "{}", cause),
            ForgotPasswordError::UserNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ForgotPasswordError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(GetCSVHeaderError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetCSVHeaderError::InvalidParameter(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(GetCSVHeaderError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetCSVHeaderError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetCSVHeaderError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetCSVHeaderError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetCSVHeaderError::InternalError(ref cause) => write!(f, "{}", cause),
            GetCSVHeaderError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetCSVHeaderError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            GetCSVHeaderError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetCSVHeaderError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetCSVHeaderError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(GetDeviceError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetDeviceError::InvalidParameter(err.msg))
                }
                "InvalidUserPoolConfigurationException" => {
                    return RusotoError::Service(GetDeviceError::InvalidUserPoolConfiguration(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(GetDeviceError::NotAuthorized(err.msg))
                }
                "PasswordResetRequiredException" => {
                    return RusotoError::Service(GetDeviceError::PasswordResetRequired(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetDeviceError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetDeviceError::TooManyRequests(err.msg))
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(GetDeviceError::UserNotConfirmed(err.msg))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(GetDeviceError::UserNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDeviceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDeviceError::InternalError(ref cause) => write!(f, "{}", cause),
            GetDeviceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetDeviceError::InvalidUserPoolConfiguration(ref cause) => write!(f, "{}", cause),
            GetDeviceError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            GetDeviceError::PasswordResetRequired(ref cause) => write!(f, "{}", cause),
            GetDeviceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetDeviceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            GetDeviceError::UserNotConfirmed(ref cause) => write!(f, "{}", cause),
            GetDeviceError::UserNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDeviceError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(GetGroupError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetGroupError::InvalidParameter(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(GetGroupError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetGroupError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetGroupError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetGroupError::InternalError(ref cause) => write!(f, "{}", cause),
            GetGroupError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetGroupError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            GetGroupError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetGroupError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetGroupError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(
                        GetIdentityProviderByIdentifierError::InternalError(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        GetIdentityProviderByIdentifierError::InvalidParameter(err.msg),
                    )
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(
                        GetIdentityProviderByIdentifierError::NotAuthorized(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        GetIdentityProviderByIdentifierError::ResourceNotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        GetIdentityProviderByIdentifierError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetIdentityProviderByIdentifierError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetIdentityProviderByIdentifierError::InternalError(ref cause) => {
                write!(f, "{}", cause)
            }
            GetIdentityProviderByIdentifierError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            GetIdentityProviderByIdentifierError::NotAuthorized(ref cause) => {
                write!(f, "{}", cause)
            }
            GetIdentityProviderByIdentifierError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            GetIdentityProviderByIdentifierError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetIdentityProviderByIdentifierError {}
/// Errors returned by GetSigningCertificate
#[derive(Debug, PartialEq)]
pub enum GetSigningCertificateError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalError(String),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameter(String),
    /// <p>This exception is thrown when the Amazon Cognito service cannot find the requested resource.</p>
    ResourceNotFound(String),
}

impl GetSigningCertificateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSigningCertificateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(GetSigningCertificateError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetSigningCertificateError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetSigningCertificateError::ResourceNotFound(
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
impl fmt::Display for GetSigningCertificateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetSigningCertificateError::InternalError(ref cause) => write!(f, "{}", cause),
            GetSigningCertificateError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetSigningCertificateError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetSigningCertificateError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(GetUICustomizationError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetUICustomizationError::InvalidParameter(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(GetUICustomizationError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetUICustomizationError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetUICustomizationError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetUICustomizationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetUICustomizationError::InternalError(ref cause) => write!(f, "{}", cause),
            GetUICustomizationError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetUICustomizationError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            GetUICustomizationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetUICustomizationError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetUICustomizationError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(GetUserError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetUserError::InvalidParameter(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(GetUserError::NotAuthorized(err.msg))
                }
                "PasswordResetRequiredException" => {
                    return RusotoError::Service(GetUserError::PasswordResetRequired(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetUserError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetUserError::TooManyRequests(err.msg))
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(GetUserError::UserNotConfirmed(err.msg))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(GetUserError::UserNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetUserError::InternalError(ref cause) => write!(f, "{}", cause),
            GetUserError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetUserError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            GetUserError::PasswordResetRequired(ref cause) => write!(f, "{}", cause),
            GetUserError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetUserError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            GetUserError::UserNotConfirmed(ref cause) => write!(f, "{}", cause),
            GetUserError::UserNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetUserError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CodeDeliveryFailureException" => {
                    return RusotoError::Service(
                        GetUserAttributeVerificationCodeError::CodeDeliveryFailure(err.msg),
                    )
                }
                "InternalErrorException" => {
                    return RusotoError::Service(
                        GetUserAttributeVerificationCodeError::InternalError(err.msg),
                    )
                }
                "InvalidEmailRoleAccessPolicyException" => {
                    return RusotoError::Service(
                        GetUserAttributeVerificationCodeError::InvalidEmailRoleAccessPolicy(
                            err.msg,
                        ),
                    )
                }
                "InvalidLambdaResponseException" => {
                    return RusotoError::Service(
                        GetUserAttributeVerificationCodeError::InvalidLambdaResponse(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        GetUserAttributeVerificationCodeError::InvalidParameter(err.msg),
                    )
                }
                "InvalidSmsRoleAccessPolicyException" => {
                    return RusotoError::Service(
                        GetUserAttributeVerificationCodeError::InvalidSmsRoleAccessPolicy(err.msg),
                    )
                }
                "InvalidSmsRoleTrustRelationshipException" => {
                    return RusotoError::Service(
                        GetUserAttributeVerificationCodeError::InvalidSmsRoleTrustRelationship(
                            err.msg,
                        ),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(
                        GetUserAttributeVerificationCodeError::LimitExceeded(err.msg),
                    )
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(
                        GetUserAttributeVerificationCodeError::NotAuthorized(err.msg),
                    )
                }
                "PasswordResetRequiredException" => {
                    return RusotoError::Service(
                        GetUserAttributeVerificationCodeError::PasswordResetRequired(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        GetUserAttributeVerificationCodeError::ResourceNotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        GetUserAttributeVerificationCodeError::TooManyRequests(err.msg),
                    )
                }
                "UnexpectedLambdaException" => {
                    return RusotoError::Service(
                        GetUserAttributeVerificationCodeError::UnexpectedLambda(err.msg),
                    )
                }
                "UserLambdaValidationException" => {
                    return RusotoError::Service(
                        GetUserAttributeVerificationCodeError::UserLambdaValidation(err.msg),
                    )
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(
                        GetUserAttributeVerificationCodeError::UserNotConfirmed(err.msg),
                    )
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(
                        GetUserAttributeVerificationCodeError::UserNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetUserAttributeVerificationCodeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetUserAttributeVerificationCodeError::CodeDeliveryFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            GetUserAttributeVerificationCodeError::InternalError(ref cause) => {
                write!(f, "{}", cause)
            }
            GetUserAttributeVerificationCodeError::InvalidEmailRoleAccessPolicy(ref cause) => {
                write!(f, "{}", cause)
            }
            GetUserAttributeVerificationCodeError::InvalidLambdaResponse(ref cause) => {
                write!(f, "{}", cause)
            }
            GetUserAttributeVerificationCodeError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            GetUserAttributeVerificationCodeError::InvalidSmsRoleAccessPolicy(ref cause) => {
                write!(f, "{}", cause)
            }
            GetUserAttributeVerificationCodeError::InvalidSmsRoleTrustRelationship(ref cause) => {
                write!(f, "{}", cause)
            }
            GetUserAttributeVerificationCodeError::LimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            GetUserAttributeVerificationCodeError::NotAuthorized(ref cause) => {
                write!(f, "{}", cause)
            }
            GetUserAttributeVerificationCodeError::PasswordResetRequired(ref cause) => {
                write!(f, "{}", cause)
            }
            GetUserAttributeVerificationCodeError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            GetUserAttributeVerificationCodeError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
            GetUserAttributeVerificationCodeError::UnexpectedLambda(ref cause) => {
                write!(f, "{}", cause)
            }
            GetUserAttributeVerificationCodeError::UserLambdaValidation(ref cause) => {
                write!(f, "{}", cause)
            }
            GetUserAttributeVerificationCodeError::UserNotConfirmed(ref cause) => {
                write!(f, "{}", cause)
            }
            GetUserAttributeVerificationCodeError::UserNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetUserAttributeVerificationCodeError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(GetUserPoolMfaConfigError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetUserPoolMfaConfigError::InvalidParameter(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(GetUserPoolMfaConfigError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetUserPoolMfaConfigError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetUserPoolMfaConfigError::TooManyRequests(
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
impl fmt::Display for GetUserPoolMfaConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetUserPoolMfaConfigError::InternalError(ref cause) => write!(f, "{}", cause),
            GetUserPoolMfaConfigError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetUserPoolMfaConfigError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            GetUserPoolMfaConfigError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetUserPoolMfaConfigError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetUserPoolMfaConfigError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(GlobalSignOutError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GlobalSignOutError::InvalidParameter(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(GlobalSignOutError::NotAuthorized(err.msg))
                }
                "PasswordResetRequiredException" => {
                    return RusotoError::Service(GlobalSignOutError::PasswordResetRequired(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GlobalSignOutError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GlobalSignOutError::TooManyRequests(err.msg))
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(GlobalSignOutError::UserNotConfirmed(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GlobalSignOutError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GlobalSignOutError::InternalError(ref cause) => write!(f, "{}", cause),
            GlobalSignOutError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GlobalSignOutError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            GlobalSignOutError::PasswordResetRequired(ref cause) => write!(f, "{}", cause),
            GlobalSignOutError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GlobalSignOutError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            GlobalSignOutError::UserNotConfirmed(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GlobalSignOutError {}
/// Errors returned by InitiateAuth
#[derive(Debug, PartialEq)]
pub enum InitiateAuthError {
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(InitiateAuthError::InternalError(err.msg))
                }
                "InvalidLambdaResponseException" => {
                    return RusotoError::Service(InitiateAuthError::InvalidLambdaResponse(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(InitiateAuthError::InvalidParameter(err.msg))
                }
                "InvalidSmsRoleAccessPolicyException" => {
                    return RusotoError::Service(InitiateAuthError::InvalidSmsRoleAccessPolicy(
                        err.msg,
                    ))
                }
                "InvalidSmsRoleTrustRelationshipException" => {
                    return RusotoError::Service(
                        InitiateAuthError::InvalidSmsRoleTrustRelationship(err.msg),
                    )
                }
                "InvalidUserPoolConfigurationException" => {
                    return RusotoError::Service(InitiateAuthError::InvalidUserPoolConfiguration(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(InitiateAuthError::NotAuthorized(err.msg))
                }
                "PasswordResetRequiredException" => {
                    return RusotoError::Service(InitiateAuthError::PasswordResetRequired(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(InitiateAuthError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(InitiateAuthError::TooManyRequests(err.msg))
                }
                "UnexpectedLambdaException" => {
                    return RusotoError::Service(InitiateAuthError::UnexpectedLambda(err.msg))
                }
                "UserLambdaValidationException" => {
                    return RusotoError::Service(InitiateAuthError::UserLambdaValidation(err.msg))
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(InitiateAuthError::UserNotConfirmed(err.msg))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(InitiateAuthError::UserNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for InitiateAuthError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InitiateAuthError::InternalError(ref cause) => write!(f, "{}", cause),
            InitiateAuthError::InvalidLambdaResponse(ref cause) => write!(f, "{}", cause),
            InitiateAuthError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            InitiateAuthError::InvalidSmsRoleAccessPolicy(ref cause) => write!(f, "{}", cause),
            InitiateAuthError::InvalidSmsRoleTrustRelationship(ref cause) => write!(f, "{}", cause),
            InitiateAuthError::InvalidUserPoolConfiguration(ref cause) => write!(f, "{}", cause),
            InitiateAuthError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            InitiateAuthError::PasswordResetRequired(ref cause) => write!(f, "{}", cause),
            InitiateAuthError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            InitiateAuthError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            InitiateAuthError::UnexpectedLambda(ref cause) => write!(f, "{}", cause),
            InitiateAuthError::UserLambdaValidation(ref cause) => write!(f, "{}", cause),
            InitiateAuthError::UserNotConfirmed(ref cause) => write!(f, "{}", cause),
            InitiateAuthError::UserNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for InitiateAuthError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(ListDevicesError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListDevicesError::InvalidParameter(err.msg))
                }
                "InvalidUserPoolConfigurationException" => {
                    return RusotoError::Service(ListDevicesError::InvalidUserPoolConfiguration(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(ListDevicesError::NotAuthorized(err.msg))
                }
                "PasswordResetRequiredException" => {
                    return RusotoError::Service(ListDevicesError::PasswordResetRequired(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListDevicesError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListDevicesError::TooManyRequests(err.msg))
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(ListDevicesError::UserNotConfirmed(err.msg))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(ListDevicesError::UserNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListDevicesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDevicesError::InternalError(ref cause) => write!(f, "{}", cause),
            ListDevicesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListDevicesError::InvalidUserPoolConfiguration(ref cause) => write!(f, "{}", cause),
            ListDevicesError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            ListDevicesError::PasswordResetRequired(ref cause) => write!(f, "{}", cause),
            ListDevicesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListDevicesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            ListDevicesError::UserNotConfirmed(ref cause) => write!(f, "{}", cause),
            ListDevicesError::UserNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDevicesError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(ListGroupsError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListGroupsError::InvalidParameter(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(ListGroupsError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListGroupsError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListGroupsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListGroupsError::InternalError(ref cause) => write!(f, "{}", cause),
            ListGroupsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListGroupsError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            ListGroupsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListGroupsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListGroupsError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(ListIdentityProvidersError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListIdentityProvidersError::InvalidParameter(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(ListIdentityProvidersError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListIdentityProvidersError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListIdentityProvidersError::TooManyRequests(
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
impl fmt::Display for ListIdentityProvidersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListIdentityProvidersError::InternalError(ref cause) => write!(f, "{}", cause),
            ListIdentityProvidersError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListIdentityProvidersError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            ListIdentityProvidersError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListIdentityProvidersError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListIdentityProvidersError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(ListResourceServersError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListResourceServersError::InvalidParameter(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(ListResourceServersError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListResourceServersError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListResourceServersError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListResourceServersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListResourceServersError::InternalError(ref cause) => write!(f, "{}", cause),
            ListResourceServersError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListResourceServersError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            ListResourceServersError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListResourceServersError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListResourceServersError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
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

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(ListTagsForResourceError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidParameter(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(ListTagsForResourceError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListTagsForResourceError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTagsForResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsForResourceError::InternalError(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(ListUserImportJobsError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListUserImportJobsError::InvalidParameter(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(ListUserImportJobsError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListUserImportJobsError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListUserImportJobsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListUserImportJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListUserImportJobsError::InternalError(ref cause) => write!(f, "{}", cause),
            ListUserImportJobsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListUserImportJobsError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            ListUserImportJobsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListUserImportJobsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListUserImportJobsError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(ListUserPoolClientsError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListUserPoolClientsError::InvalidParameter(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(ListUserPoolClientsError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListUserPoolClientsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListUserPoolClientsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListUserPoolClientsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListUserPoolClientsError::InternalError(ref cause) => write!(f, "{}", cause),
            ListUserPoolClientsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListUserPoolClientsError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            ListUserPoolClientsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListUserPoolClientsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListUserPoolClientsError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(ListUserPoolsError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListUserPoolsError::InvalidParameter(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(ListUserPoolsError::NotAuthorized(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListUserPoolsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListUserPoolsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListUserPoolsError::InternalError(ref cause) => write!(f, "{}", cause),
            ListUserPoolsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListUserPoolsError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            ListUserPoolsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListUserPoolsError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(ListUsersError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListUsersError::InvalidParameter(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(ListUsersError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListUsersError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListUsersError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListUsersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListUsersError::InternalError(ref cause) => write!(f, "{}", cause),
            ListUsersError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListUsersError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            ListUsersError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListUsersError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListUsersError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(ListUsersInGroupError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListUsersInGroupError::InvalidParameter(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(ListUsersInGroupError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListUsersInGroupError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListUsersInGroupError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListUsersInGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListUsersInGroupError::InternalError(ref cause) => write!(f, "{}", cause),
            ListUsersInGroupError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListUsersInGroupError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            ListUsersInGroupError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListUsersInGroupError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListUsersInGroupError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CodeDeliveryFailureException" => {
                    return RusotoError::Service(ResendConfirmationCodeError::CodeDeliveryFailure(
                        err.msg,
                    ))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(ResendConfirmationCodeError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidEmailRoleAccessPolicyException" => {
                    return RusotoError::Service(
                        ResendConfirmationCodeError::InvalidEmailRoleAccessPolicy(err.msg),
                    )
                }
                "InvalidLambdaResponseException" => {
                    return RusotoError::Service(
                        ResendConfirmationCodeError::InvalidLambdaResponse(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ResendConfirmationCodeError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidSmsRoleAccessPolicyException" => {
                    return RusotoError::Service(
                        ResendConfirmationCodeError::InvalidSmsRoleAccessPolicy(err.msg),
                    )
                }
                "InvalidSmsRoleTrustRelationshipException" => {
                    return RusotoError::Service(
                        ResendConfirmationCodeError::InvalidSmsRoleTrustRelationship(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ResendConfirmationCodeError::LimitExceeded(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(ResendConfirmationCodeError::NotAuthorized(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ResendConfirmationCodeError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ResendConfirmationCodeError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnexpectedLambdaException" => {
                    return RusotoError::Service(ResendConfirmationCodeError::UnexpectedLambda(
                        err.msg,
                    ))
                }
                "UserLambdaValidationException" => {
                    return RusotoError::Service(ResendConfirmationCodeError::UserLambdaValidation(
                        err.msg,
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(ResendConfirmationCodeError::UserNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ResendConfirmationCodeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ResendConfirmationCodeError::CodeDeliveryFailure(ref cause) => write!(f, "{}", cause),
            ResendConfirmationCodeError::InternalError(ref cause) => write!(f, "{}", cause),
            ResendConfirmationCodeError::InvalidEmailRoleAccessPolicy(ref cause) => {
                write!(f, "{}", cause)
            }
            ResendConfirmationCodeError::InvalidLambdaResponse(ref cause) => write!(f, "{}", cause),
            ResendConfirmationCodeError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ResendConfirmationCodeError::InvalidSmsRoleAccessPolicy(ref cause) => {
                write!(f, "{}", cause)
            }
            ResendConfirmationCodeError::InvalidSmsRoleTrustRelationship(ref cause) => {
                write!(f, "{}", cause)
            }
            ResendConfirmationCodeError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ResendConfirmationCodeError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            ResendConfirmationCodeError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ResendConfirmationCodeError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            ResendConfirmationCodeError::UnexpectedLambda(ref cause) => write!(f, "{}", cause),
            ResendConfirmationCodeError::UserLambdaValidation(ref cause) => write!(f, "{}", cause),
            ResendConfirmationCodeError::UserNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ResendConfirmationCodeError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AliasExistsException" => {
                    return RusotoError::Service(RespondToAuthChallengeError::AliasExists(err.msg))
                }
                "CodeMismatchException" => {
                    return RusotoError::Service(RespondToAuthChallengeError::CodeMismatch(err.msg))
                }
                "ExpiredCodeException" => {
                    return RusotoError::Service(RespondToAuthChallengeError::ExpiredCode(err.msg))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(RespondToAuthChallengeError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidLambdaResponseException" => {
                    return RusotoError::Service(
                        RespondToAuthChallengeError::InvalidLambdaResponse(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(RespondToAuthChallengeError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidPasswordException" => {
                    return RusotoError::Service(RespondToAuthChallengeError::InvalidPassword(
                        err.msg,
                    ))
                }
                "InvalidSmsRoleAccessPolicyException" => {
                    return RusotoError::Service(
                        RespondToAuthChallengeError::InvalidSmsRoleAccessPolicy(err.msg),
                    )
                }
                "InvalidSmsRoleTrustRelationshipException" => {
                    return RusotoError::Service(
                        RespondToAuthChallengeError::InvalidSmsRoleTrustRelationship(err.msg),
                    )
                }
                "InvalidUserPoolConfigurationException" => {
                    return RusotoError::Service(
                        RespondToAuthChallengeError::InvalidUserPoolConfiguration(err.msg),
                    )
                }
                "MFAMethodNotFoundException" => {
                    return RusotoError::Service(RespondToAuthChallengeError::MFAMethodNotFound(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(RespondToAuthChallengeError::NotAuthorized(
                        err.msg,
                    ))
                }
                "PasswordResetRequiredException" => {
                    return RusotoError::Service(
                        RespondToAuthChallengeError::PasswordResetRequired(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(RespondToAuthChallengeError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "SoftwareTokenMFANotFoundException" => {
                    return RusotoError::Service(
                        RespondToAuthChallengeError::SoftwareTokenMFANotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(RespondToAuthChallengeError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnexpectedLambdaException" => {
                    return RusotoError::Service(RespondToAuthChallengeError::UnexpectedLambda(
                        err.msg,
                    ))
                }
                "UserLambdaValidationException" => {
                    return RusotoError::Service(RespondToAuthChallengeError::UserLambdaValidation(
                        err.msg,
                    ))
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(RespondToAuthChallengeError::UserNotConfirmed(
                        err.msg,
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(RespondToAuthChallengeError::UserNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RespondToAuthChallengeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RespondToAuthChallengeError::AliasExists(ref cause) => write!(f, "{}", cause),
            RespondToAuthChallengeError::CodeMismatch(ref cause) => write!(f, "{}", cause),
            RespondToAuthChallengeError::ExpiredCode(ref cause) => write!(f, "{}", cause),
            RespondToAuthChallengeError::InternalError(ref cause) => write!(f, "{}", cause),
            RespondToAuthChallengeError::InvalidLambdaResponse(ref cause) => write!(f, "{}", cause),
            RespondToAuthChallengeError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            RespondToAuthChallengeError::InvalidPassword(ref cause) => write!(f, "{}", cause),
            RespondToAuthChallengeError::InvalidSmsRoleAccessPolicy(ref cause) => {
                write!(f, "{}", cause)
            }
            RespondToAuthChallengeError::InvalidSmsRoleTrustRelationship(ref cause) => {
                write!(f, "{}", cause)
            }
            RespondToAuthChallengeError::InvalidUserPoolConfiguration(ref cause) => {
                write!(f, "{}", cause)
            }
            RespondToAuthChallengeError::MFAMethodNotFound(ref cause) => write!(f, "{}", cause),
            RespondToAuthChallengeError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            RespondToAuthChallengeError::PasswordResetRequired(ref cause) => write!(f, "{}", cause),
            RespondToAuthChallengeError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            RespondToAuthChallengeError::SoftwareTokenMFANotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            RespondToAuthChallengeError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            RespondToAuthChallengeError::UnexpectedLambda(ref cause) => write!(f, "{}", cause),
            RespondToAuthChallengeError::UserLambdaValidation(ref cause) => write!(f, "{}", cause),
            RespondToAuthChallengeError::UserNotConfirmed(ref cause) => write!(f, "{}", cause),
            RespondToAuthChallengeError::UserNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RespondToAuthChallengeError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CodeDeliveryFailureException" => {
                    return RusotoError::Service(SetRiskConfigurationError::CodeDeliveryFailure(
                        err.msg,
                    ))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(SetRiskConfigurationError::InternalError(err.msg))
                }
                "InvalidEmailRoleAccessPolicyException" => {
                    return RusotoError::Service(
                        SetRiskConfigurationError::InvalidEmailRoleAccessPolicy(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(SetRiskConfigurationError::InvalidParameter(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(SetRiskConfigurationError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(SetRiskConfigurationError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(SetRiskConfigurationError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UserPoolAddOnNotEnabledException" => {
                    return RusotoError::Service(
                        SetRiskConfigurationError::UserPoolAddOnNotEnabled(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SetRiskConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SetRiskConfigurationError::CodeDeliveryFailure(ref cause) => write!(f, "{}", cause),
            SetRiskConfigurationError::InternalError(ref cause) => write!(f, "{}", cause),
            SetRiskConfigurationError::InvalidEmailRoleAccessPolicy(ref cause) => {
                write!(f, "{}", cause)
            }
            SetRiskConfigurationError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            SetRiskConfigurationError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            SetRiskConfigurationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            SetRiskConfigurationError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            SetRiskConfigurationError::UserPoolAddOnNotEnabled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SetRiskConfigurationError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(SetUICustomizationError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(SetUICustomizationError::InvalidParameter(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(SetUICustomizationError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(SetUICustomizationError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(SetUICustomizationError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SetUICustomizationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SetUICustomizationError::InternalError(ref cause) => write!(f, "{}", cause),
            SetUICustomizationError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            SetUICustomizationError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            SetUICustomizationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            SetUICustomizationError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SetUICustomizationError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(SetUserMFAPreferenceError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(SetUserMFAPreferenceError::InvalidParameter(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(SetUserMFAPreferenceError::NotAuthorized(err.msg))
                }
                "PasswordResetRequiredException" => {
                    return RusotoError::Service(SetUserMFAPreferenceError::PasswordResetRequired(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(SetUserMFAPreferenceError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(SetUserMFAPreferenceError::UserNotConfirmed(
                        err.msg,
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(SetUserMFAPreferenceError::UserNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SetUserMFAPreferenceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SetUserMFAPreferenceError::InternalError(ref cause) => write!(f, "{}", cause),
            SetUserMFAPreferenceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            SetUserMFAPreferenceError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            SetUserMFAPreferenceError::PasswordResetRequired(ref cause) => write!(f, "{}", cause),
            SetUserMFAPreferenceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            SetUserMFAPreferenceError::UserNotConfirmed(ref cause) => write!(f, "{}", cause),
            SetUserMFAPreferenceError::UserNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SetUserMFAPreferenceError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(SetUserPoolMfaConfigError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(SetUserPoolMfaConfigError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidSmsRoleAccessPolicyException" => {
                    return RusotoError::Service(
                        SetUserPoolMfaConfigError::InvalidSmsRoleAccessPolicy(err.msg),
                    )
                }
                "InvalidSmsRoleTrustRelationshipException" => {
                    return RusotoError::Service(
                        SetUserPoolMfaConfigError::InvalidSmsRoleTrustRelationship(err.msg),
                    )
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(SetUserPoolMfaConfigError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(SetUserPoolMfaConfigError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(SetUserPoolMfaConfigError::TooManyRequests(
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
impl fmt::Display for SetUserPoolMfaConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SetUserPoolMfaConfigError::InternalError(ref cause) => write!(f, "{}", cause),
            SetUserPoolMfaConfigError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            SetUserPoolMfaConfigError::InvalidSmsRoleAccessPolicy(ref cause) => {
                write!(f, "{}", cause)
            }
            SetUserPoolMfaConfigError::InvalidSmsRoleTrustRelationship(ref cause) => {
                write!(f, "{}", cause)
            }
            SetUserPoolMfaConfigError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            SetUserPoolMfaConfigError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            SetUserPoolMfaConfigError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SetUserPoolMfaConfigError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(SetUserSettingsError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(SetUserSettingsError::InvalidParameter(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(SetUserSettingsError::NotAuthorized(err.msg))
                }
                "PasswordResetRequiredException" => {
                    return RusotoError::Service(SetUserSettingsError::PasswordResetRequired(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(SetUserSettingsError::ResourceNotFound(err.msg))
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(SetUserSettingsError::UserNotConfirmed(err.msg))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(SetUserSettingsError::UserNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SetUserSettingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SetUserSettingsError::InternalError(ref cause) => write!(f, "{}", cause),
            SetUserSettingsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            SetUserSettingsError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            SetUserSettingsError::PasswordResetRequired(ref cause) => write!(f, "{}", cause),
            SetUserSettingsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            SetUserSettingsError::UserNotConfirmed(ref cause) => write!(f, "{}", cause),
            SetUserSettingsError::UserNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SetUserSettingsError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CodeDeliveryFailureException" => {
                    return RusotoError::Service(SignUpError::CodeDeliveryFailure(err.msg))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(SignUpError::InternalError(err.msg))
                }
                "InvalidEmailRoleAccessPolicyException" => {
                    return RusotoError::Service(SignUpError::InvalidEmailRoleAccessPolicy(err.msg))
                }
                "InvalidLambdaResponseException" => {
                    return RusotoError::Service(SignUpError::InvalidLambdaResponse(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(SignUpError::InvalidParameter(err.msg))
                }
                "InvalidPasswordException" => {
                    return RusotoError::Service(SignUpError::InvalidPassword(err.msg))
                }
                "InvalidSmsRoleAccessPolicyException" => {
                    return RusotoError::Service(SignUpError::InvalidSmsRoleAccessPolicy(err.msg))
                }
                "InvalidSmsRoleTrustRelationshipException" => {
                    return RusotoError::Service(SignUpError::InvalidSmsRoleTrustRelationship(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(SignUpError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(SignUpError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(SignUpError::TooManyRequests(err.msg))
                }
                "UnexpectedLambdaException" => {
                    return RusotoError::Service(SignUpError::UnexpectedLambda(err.msg))
                }
                "UserLambdaValidationException" => {
                    return RusotoError::Service(SignUpError::UserLambdaValidation(err.msg))
                }
                "UsernameExistsException" => {
                    return RusotoError::Service(SignUpError::UsernameExists(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SignUpError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SignUpError::CodeDeliveryFailure(ref cause) => write!(f, "{}", cause),
            SignUpError::InternalError(ref cause) => write!(f, "{}", cause),
            SignUpError::InvalidEmailRoleAccessPolicy(ref cause) => write!(f, "{}", cause),
            SignUpError::InvalidLambdaResponse(ref cause) => write!(f, "{}", cause),
            SignUpError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            SignUpError::InvalidPassword(ref cause) => write!(f, "{}", cause),
            SignUpError::InvalidSmsRoleAccessPolicy(ref cause) => write!(f, "{}", cause),
            SignUpError::InvalidSmsRoleTrustRelationship(ref cause) => write!(f, "{}", cause),
            SignUpError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            SignUpError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            SignUpError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            SignUpError::UnexpectedLambda(ref cause) => write!(f, "{}", cause),
            SignUpError::UserLambdaValidation(ref cause) => write!(f, "{}", cause),
            SignUpError::UsernameExists(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SignUpError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(StartUserImportJobError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StartUserImportJobError::InvalidParameter(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(StartUserImportJobError::NotAuthorized(err.msg))
                }
                "PreconditionNotMetException" => {
                    return RusotoError::Service(StartUserImportJobError::PreconditionNotMet(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StartUserImportJobError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(StartUserImportJobError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartUserImportJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartUserImportJobError::InternalError(ref cause) => write!(f, "{}", cause),
            StartUserImportJobError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            StartUserImportJobError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            StartUserImportJobError::PreconditionNotMet(ref cause) => write!(f, "{}", cause),
            StartUserImportJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            StartUserImportJobError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartUserImportJobError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(StopUserImportJobError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StopUserImportJobError::InvalidParameter(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(StopUserImportJobError::NotAuthorized(err.msg))
                }
                "PreconditionNotMetException" => {
                    return RusotoError::Service(StopUserImportJobError::PreconditionNotMet(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StopUserImportJobError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(StopUserImportJobError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StopUserImportJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopUserImportJobError::InternalError(ref cause) => write!(f, "{}", cause),
            StopUserImportJobError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            StopUserImportJobError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            StopUserImportJobError::PreconditionNotMet(ref cause) => write!(f, "{}", cause),
            StopUserImportJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            StopUserImportJobError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopUserImportJobError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
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

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(TagResourceError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(TagResourceError::InvalidParameter(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(TagResourceError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(TagResourceError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for TagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TagResourceError::InternalError(ref cause) => write!(f, "{}", cause),
            TagResourceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            TagResourceError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
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

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(UntagResourceError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UntagResourceError::InvalidParameter(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(UntagResourceError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UntagResourceError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UntagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UntagResourceError::InternalError(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UntagResourceError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UntagResourceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(UpdateAuthEventFeedbackError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateAuthEventFeedbackError::InvalidParameter(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(UpdateAuthEventFeedbackError::NotAuthorized(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateAuthEventFeedbackError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateAuthEventFeedbackError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(UpdateAuthEventFeedbackError::UserNotFound(
                        err.msg,
                    ))
                }
                "UserPoolAddOnNotEnabledException" => {
                    return RusotoError::Service(
                        UpdateAuthEventFeedbackError::UserPoolAddOnNotEnabled(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateAuthEventFeedbackError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateAuthEventFeedbackError::InternalError(ref cause) => write!(f, "{}", cause),
            UpdateAuthEventFeedbackError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateAuthEventFeedbackError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            UpdateAuthEventFeedbackError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateAuthEventFeedbackError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            UpdateAuthEventFeedbackError::UserNotFound(ref cause) => write!(f, "{}", cause),
            UpdateAuthEventFeedbackError::UserPoolAddOnNotEnabled(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateAuthEventFeedbackError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(UpdateDeviceStatusError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateDeviceStatusError::InvalidParameter(err.msg))
                }
                "InvalidUserPoolConfigurationException" => {
                    return RusotoError::Service(
                        UpdateDeviceStatusError::InvalidUserPoolConfiguration(err.msg),
                    )
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(UpdateDeviceStatusError::NotAuthorized(err.msg))
                }
                "PasswordResetRequiredException" => {
                    return RusotoError::Service(UpdateDeviceStatusError::PasswordResetRequired(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateDeviceStatusError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateDeviceStatusError::TooManyRequests(err.msg))
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(UpdateDeviceStatusError::UserNotConfirmed(err.msg))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(UpdateDeviceStatusError::UserNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateDeviceStatusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateDeviceStatusError::InternalError(ref cause) => write!(f, "{}", cause),
            UpdateDeviceStatusError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateDeviceStatusError::InvalidUserPoolConfiguration(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateDeviceStatusError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            UpdateDeviceStatusError::PasswordResetRequired(ref cause) => write!(f, "{}", cause),
            UpdateDeviceStatusError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateDeviceStatusError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            UpdateDeviceStatusError::UserNotConfirmed(ref cause) => write!(f, "{}", cause),
            UpdateDeviceStatusError::UserNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateDeviceStatusError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(UpdateGroupError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateGroupError::InvalidParameter(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(UpdateGroupError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateGroupError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateGroupError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateGroupError::InternalError(ref cause) => write!(f, "{}", cause),
            UpdateGroupError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateGroupError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            UpdateGroupError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateGroupError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateGroupError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(UpdateIdentityProviderError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateIdentityProviderError::InvalidParameter(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(UpdateIdentityProviderError::NotAuthorized(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateIdentityProviderError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateIdentityProviderError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnsupportedIdentityProviderException" => {
                    return RusotoError::Service(
                        UpdateIdentityProviderError::UnsupportedIdentityProvider(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateIdentityProviderError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateIdentityProviderError::InternalError(ref cause) => write!(f, "{}", cause),
            UpdateIdentityProviderError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateIdentityProviderError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            UpdateIdentityProviderError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateIdentityProviderError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            UpdateIdentityProviderError::UnsupportedIdentityProvider(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateIdentityProviderError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(UpdateResourceServerError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateResourceServerError::InvalidParameter(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(UpdateResourceServerError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateResourceServerError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateResourceServerError::TooManyRequests(
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
impl fmt::Display for UpdateResourceServerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateResourceServerError::InternalError(ref cause) => write!(f, "{}", cause),
            UpdateResourceServerError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateResourceServerError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            UpdateResourceServerError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateResourceServerError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateResourceServerError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AliasExistsException" => {
                    return RusotoError::Service(UpdateUserAttributesError::AliasExists(err.msg))
                }
                "CodeDeliveryFailureException" => {
                    return RusotoError::Service(UpdateUserAttributesError::CodeDeliveryFailure(
                        err.msg,
                    ))
                }
                "CodeMismatchException" => {
                    return RusotoError::Service(UpdateUserAttributesError::CodeMismatch(err.msg))
                }
                "ExpiredCodeException" => {
                    return RusotoError::Service(UpdateUserAttributesError::ExpiredCode(err.msg))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(UpdateUserAttributesError::InternalError(err.msg))
                }
                "InvalidEmailRoleAccessPolicyException" => {
                    return RusotoError::Service(
                        UpdateUserAttributesError::InvalidEmailRoleAccessPolicy(err.msg),
                    )
                }
                "InvalidLambdaResponseException" => {
                    return RusotoError::Service(UpdateUserAttributesError::InvalidLambdaResponse(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateUserAttributesError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidSmsRoleAccessPolicyException" => {
                    return RusotoError::Service(
                        UpdateUserAttributesError::InvalidSmsRoleAccessPolicy(err.msg),
                    )
                }
                "InvalidSmsRoleTrustRelationshipException" => {
                    return RusotoError::Service(
                        UpdateUserAttributesError::InvalidSmsRoleTrustRelationship(err.msg),
                    )
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(UpdateUserAttributesError::NotAuthorized(err.msg))
                }
                "PasswordResetRequiredException" => {
                    return RusotoError::Service(UpdateUserAttributesError::PasswordResetRequired(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateUserAttributesError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateUserAttributesError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnexpectedLambdaException" => {
                    return RusotoError::Service(UpdateUserAttributesError::UnexpectedLambda(
                        err.msg,
                    ))
                }
                "UserLambdaValidationException" => {
                    return RusotoError::Service(UpdateUserAttributesError::UserLambdaValidation(
                        err.msg,
                    ))
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(UpdateUserAttributesError::UserNotConfirmed(
                        err.msg,
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(UpdateUserAttributesError::UserNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateUserAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateUserAttributesError::AliasExists(ref cause) => write!(f, "{}", cause),
            UpdateUserAttributesError::CodeDeliveryFailure(ref cause) => write!(f, "{}", cause),
            UpdateUserAttributesError::CodeMismatch(ref cause) => write!(f, "{}", cause),
            UpdateUserAttributesError::ExpiredCode(ref cause) => write!(f, "{}", cause),
            UpdateUserAttributesError::InternalError(ref cause) => write!(f, "{}", cause),
            UpdateUserAttributesError::InvalidEmailRoleAccessPolicy(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateUserAttributesError::InvalidLambdaResponse(ref cause) => write!(f, "{}", cause),
            UpdateUserAttributesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateUserAttributesError::InvalidSmsRoleAccessPolicy(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateUserAttributesError::InvalidSmsRoleTrustRelationship(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateUserAttributesError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            UpdateUserAttributesError::PasswordResetRequired(ref cause) => write!(f, "{}", cause),
            UpdateUserAttributesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateUserAttributesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            UpdateUserAttributesError::UnexpectedLambda(ref cause) => write!(f, "{}", cause),
            UpdateUserAttributesError::UserLambdaValidation(ref cause) => write!(f, "{}", cause),
            UpdateUserAttributesError::UserNotConfirmed(ref cause) => write!(f, "{}", cause),
            UpdateUserAttributesError::UserNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateUserAttributesError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UpdateUserPoolError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(UpdateUserPoolError::InternalError(err.msg))
                }
                "InvalidEmailRoleAccessPolicyException" => {
                    return RusotoError::Service(UpdateUserPoolError::InvalidEmailRoleAccessPolicy(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateUserPoolError::InvalidParameter(err.msg))
                }
                "InvalidSmsRoleAccessPolicyException" => {
                    return RusotoError::Service(UpdateUserPoolError::InvalidSmsRoleAccessPolicy(
                        err.msg,
                    ))
                }
                "InvalidSmsRoleTrustRelationshipException" => {
                    return RusotoError::Service(
                        UpdateUserPoolError::InvalidSmsRoleTrustRelationship(err.msg),
                    )
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(UpdateUserPoolError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateUserPoolError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateUserPoolError::TooManyRequests(err.msg))
                }
                "UserImportInProgressException" => {
                    return RusotoError::Service(UpdateUserPoolError::UserImportInProgress(err.msg))
                }
                "UserPoolTaggingException" => {
                    return RusotoError::Service(UpdateUserPoolError::UserPoolTagging(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateUserPoolError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateUserPoolError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            UpdateUserPoolError::InternalError(ref cause) => write!(f, "{}", cause),
            UpdateUserPoolError::InvalidEmailRoleAccessPolicy(ref cause) => write!(f, "{}", cause),
            UpdateUserPoolError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateUserPoolError::InvalidSmsRoleAccessPolicy(ref cause) => write!(f, "{}", cause),
            UpdateUserPoolError::InvalidSmsRoleTrustRelationship(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateUserPoolError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            UpdateUserPoolError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateUserPoolError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            UpdateUserPoolError::UserImportInProgress(ref cause) => write!(f, "{}", cause),
            UpdateUserPoolError::UserPoolTagging(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateUserPoolError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UpdateUserPoolClientError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(UpdateUserPoolClientError::InternalError(err.msg))
                }
                "InvalidOAuthFlowException" => {
                    return RusotoError::Service(UpdateUserPoolClientError::InvalidOAuthFlow(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateUserPoolClientError::InvalidParameter(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(UpdateUserPoolClientError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateUserPoolClientError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ScopeDoesNotExistException" => {
                    return RusotoError::Service(UpdateUserPoolClientError::ScopeDoesNotExist(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateUserPoolClientError::TooManyRequests(
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
impl fmt::Display for UpdateUserPoolClientError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateUserPoolClientError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            UpdateUserPoolClientError::InternalError(ref cause) => write!(f, "{}", cause),
            UpdateUserPoolClientError::InvalidOAuthFlow(ref cause) => write!(f, "{}", cause),
            UpdateUserPoolClientError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateUserPoolClientError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            UpdateUserPoolClientError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateUserPoolClientError::ScopeDoesNotExist(ref cause) => write!(f, "{}", cause),
            UpdateUserPoolClientError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateUserPoolClientError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(UpdateUserPoolDomainError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateUserPoolDomainError::InvalidParameter(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(UpdateUserPoolDomainError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateUserPoolDomainError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateUserPoolDomainError::TooManyRequests(
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
impl fmt::Display for UpdateUserPoolDomainError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateUserPoolDomainError::InternalError(ref cause) => write!(f, "{}", cause),
            UpdateUserPoolDomainError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateUserPoolDomainError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            UpdateUserPoolDomainError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateUserPoolDomainError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateUserPoolDomainError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CodeMismatchException" => {
                    return RusotoError::Service(VerifySoftwareTokenError::CodeMismatch(err.msg))
                }
                "EnableSoftwareTokenMFAException" => {
                    return RusotoError::Service(VerifySoftwareTokenError::EnableSoftwareTokenMFA(
                        err.msg,
                    ))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(VerifySoftwareTokenError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(VerifySoftwareTokenError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidUserPoolConfigurationException" => {
                    return RusotoError::Service(
                        VerifySoftwareTokenError::InvalidUserPoolConfiguration(err.msg),
                    )
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(VerifySoftwareTokenError::NotAuthorized(err.msg))
                }
                "PasswordResetRequiredException" => {
                    return RusotoError::Service(VerifySoftwareTokenError::PasswordResetRequired(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(VerifySoftwareTokenError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "SoftwareTokenMFANotFoundException" => {
                    return RusotoError::Service(
                        VerifySoftwareTokenError::SoftwareTokenMFANotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(VerifySoftwareTokenError::TooManyRequests(err.msg))
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(VerifySoftwareTokenError::UserNotConfirmed(
                        err.msg,
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(VerifySoftwareTokenError::UserNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for VerifySoftwareTokenError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VerifySoftwareTokenError::CodeMismatch(ref cause) => write!(f, "{}", cause),
            VerifySoftwareTokenError::EnableSoftwareTokenMFA(ref cause) => write!(f, "{}", cause),
            VerifySoftwareTokenError::InternalError(ref cause) => write!(f, "{}", cause),
            VerifySoftwareTokenError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            VerifySoftwareTokenError::InvalidUserPoolConfiguration(ref cause) => {
                write!(f, "{}", cause)
            }
            VerifySoftwareTokenError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            VerifySoftwareTokenError::PasswordResetRequired(ref cause) => write!(f, "{}", cause),
            VerifySoftwareTokenError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            VerifySoftwareTokenError::SoftwareTokenMFANotFound(ref cause) => write!(f, "{}", cause),
            VerifySoftwareTokenError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            VerifySoftwareTokenError::UserNotConfirmed(ref cause) => write!(f, "{}", cause),
            VerifySoftwareTokenError::UserNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for VerifySoftwareTokenError {}
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CodeMismatchException" => {
                    return RusotoError::Service(VerifyUserAttributeError::CodeMismatch(err.msg))
                }
                "ExpiredCodeException" => {
                    return RusotoError::Service(VerifyUserAttributeError::ExpiredCode(err.msg))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(VerifyUserAttributeError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(VerifyUserAttributeError::InvalidParameter(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(VerifyUserAttributeError::LimitExceeded(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(VerifyUserAttributeError::NotAuthorized(err.msg))
                }
                "PasswordResetRequiredException" => {
                    return RusotoError::Service(VerifyUserAttributeError::PasswordResetRequired(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(VerifyUserAttributeError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(VerifyUserAttributeError::TooManyRequests(err.msg))
                }
                "UserNotConfirmedException" => {
                    return RusotoError::Service(VerifyUserAttributeError::UserNotConfirmed(
                        err.msg,
                    ))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(VerifyUserAttributeError::UserNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for VerifyUserAttributeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VerifyUserAttributeError::CodeMismatch(ref cause) => write!(f, "{}", cause),
            VerifyUserAttributeError::ExpiredCode(ref cause) => write!(f, "{}", cause),
            VerifyUserAttributeError::InternalError(ref cause) => write!(f, "{}", cause),
            VerifyUserAttributeError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            VerifyUserAttributeError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            VerifyUserAttributeError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            VerifyUserAttributeError::PasswordResetRequired(ref cause) => write!(f, "{}", cause),
            VerifyUserAttributeError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            VerifyUserAttributeError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            VerifyUserAttributeError::UserNotConfirmed(ref cause) => write!(f, "{}", cause),
            VerifyUserAttributeError::UserNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for VerifyUserAttributeError {}
/// Trait representing the capabilities of the Amazon Cognito Identity Provider API. Amazon Cognito Identity Provider clients implement this trait.
#[async_trait]
pub trait CognitoIdentityProvider {
    /// <p>Adds additional user attributes to the user pool schema.</p>
    async fn add_custom_attributes(
        &self,
        input: AddCustomAttributesRequest,
    ) -> Result<AddCustomAttributesResponse, RusotoError<AddCustomAttributesError>>;

    /// <p>Adds the specified user to the specified group.</p> <p>Calling this action requires developer credentials.</p>
    async fn admin_add_user_to_group(
        &self,
        input: AdminAddUserToGroupRequest,
    ) -> Result<(), RusotoError<AdminAddUserToGroupError>>;

    /// <p>Confirms user registration as an admin without using a confirmation code. Works on any user.</p> <p>Calling this action requires developer credentials.</p>
    async fn admin_confirm_sign_up(
        &self,
        input: AdminConfirmSignUpRequest,
    ) -> Result<AdminConfirmSignUpResponse, RusotoError<AdminConfirmSignUpError>>;

    /// <p>Creates a new user in the specified user pool.</p> <p>If <code>MessageAction</code> is not set, the default is to send a welcome message via email or phone (SMS).</p> <note> <p>This message is based on a template that you configured in your call to or . This template includes your custom sign-up instructions and placeholders for user name and temporary password.</p> </note> <p>Alternatively, you can call AdminCreateUser with “SUPPRESS” for the <code>MessageAction</code> parameter, and Amazon Cognito will not send any email. </p> <p>In either case, the user will be in the <code>FORCE_CHANGE_PASSWORD</code> state until they sign in and change their password.</p> <p>AdminCreateUser requires developer credentials.</p>
    async fn admin_create_user(
        &self,
        input: AdminCreateUserRequest,
    ) -> Result<AdminCreateUserResponse, RusotoError<AdminCreateUserError>>;

    /// <p>Deletes a user as an administrator. Works on any user.</p> <p>Calling this action requires developer credentials.</p>
    async fn admin_delete_user(
        &self,
        input: AdminDeleteUserRequest,
    ) -> Result<(), RusotoError<AdminDeleteUserError>>;

    /// <p>Deletes the user attributes in a user pool as an administrator. Works on any user.</p> <p>Calling this action requires developer credentials.</p>
    async fn admin_delete_user_attributes(
        &self,
        input: AdminDeleteUserAttributesRequest,
    ) -> Result<AdminDeleteUserAttributesResponse, RusotoError<AdminDeleteUserAttributesError>>;

    /// <p>Disables the user from signing in with the specified external (SAML or social) identity provider. If the user to disable is a Cognito User Pools native username + password user, they are not permitted to use their password to sign-in. If the user to disable is a linked external IdP user, any link between that user and an existing user is removed. The next time the external user (no longer attached to the previously linked <code>DestinationUser</code>) signs in, they must create a new user account. See .</p> <p>This action is enabled only for admin access and requires developer credentials.</p> <p>The <code>ProviderName</code> must match the value specified when creating an IdP for the pool. </p> <p>To disable a native username + password user, the <code>ProviderName</code> value must be <code>Cognito</code> and the <code>ProviderAttributeName</code> must be <code>Cognito_Subject</code>, with the <code>ProviderAttributeValue</code> being the name that is used in the user pool for the user.</p> <p>The <code>ProviderAttributeName</code> must always be <code>Cognito_Subject</code> for social identity providers. The <code>ProviderAttributeValue</code> must always be the exact subject that was used when the user was originally linked as a source user.</p> <p>For de-linking a SAML identity, there are two scenarios. If the linked identity has not yet been used to sign-in, the <code>ProviderAttributeName</code> and <code>ProviderAttributeValue</code> must be the same values that were used for the <code>SourceUser</code> when the identities were originally linked in the call. (If the linking was done with <code>ProviderAttributeName</code> set to <code>Cognito_Subject</code>, the same applies here). However, if the user has already signed in, the <code>ProviderAttributeName</code> must be <code>Cognito_Subject</code> and <code>ProviderAttributeValue</code> must be the subject of the SAML assertion.</p>
    async fn admin_disable_provider_for_user(
        &self,
        input: AdminDisableProviderForUserRequest,
    ) -> Result<AdminDisableProviderForUserResponse, RusotoError<AdminDisableProviderForUserError>>;

    /// <p>Disables the specified user.</p> <p>Calling this action requires developer credentials.</p>
    async fn admin_disable_user(
        &self,
        input: AdminDisableUserRequest,
    ) -> Result<AdminDisableUserResponse, RusotoError<AdminDisableUserError>>;

    /// <p>Enables the specified user as an administrator. Works on any user.</p> <p>Calling this action requires developer credentials.</p>
    async fn admin_enable_user(
        &self,
        input: AdminEnableUserRequest,
    ) -> Result<AdminEnableUserResponse, RusotoError<AdminEnableUserError>>;

    /// <p>Forgets the device, as an administrator.</p> <p>Calling this action requires developer credentials.</p>
    async fn admin_forget_device(
        &self,
        input: AdminForgetDeviceRequest,
    ) -> Result<(), RusotoError<AdminForgetDeviceError>>;

    /// <p>Gets the device, as an administrator.</p> <p>Calling this action requires developer credentials.</p>
    async fn admin_get_device(
        &self,
        input: AdminGetDeviceRequest,
    ) -> Result<AdminGetDeviceResponse, RusotoError<AdminGetDeviceError>>;

    /// <p>Gets the specified user by user name in a user pool as an administrator. Works on any user.</p> <p>Calling this action requires developer credentials.</p>
    async fn admin_get_user(
        &self,
        input: AdminGetUserRequest,
    ) -> Result<AdminGetUserResponse, RusotoError<AdminGetUserError>>;

    /// <p>Initiates the authentication flow, as an administrator.</p> <p>Calling this action requires developer credentials.</p>
    async fn admin_initiate_auth(
        &self,
        input: AdminInitiateAuthRequest,
    ) -> Result<AdminInitiateAuthResponse, RusotoError<AdminInitiateAuthError>>;

    /// <p>Links an existing user account in a user pool (<code>DestinationUser</code>) to an identity from an external identity provider (<code>SourceUser</code>) based on a specified attribute name and value from the external identity provider. This allows you to create a link from the existing user account to an external federated user identity that has not yet been used to sign in, so that the federated user identity can be used to sign in as the existing user account. </p> <p> For example, if there is an existing user with a username and password, this API links that user to a federated user identity, so that when the federated user identity is used, the user signs in as the existing user account. </p> <important> <p>Because this API allows a user with an external federated identity to sign in as an existing user in the user pool, it is critical that it only be used with external identity providers and provider attributes that have been trusted by the application owner.</p> </important> <p>See also .</p> <p>This action is enabled only for admin access and requires developer credentials.</p>
    async fn admin_link_provider_for_user(
        &self,
        input: AdminLinkProviderForUserRequest,
    ) -> Result<AdminLinkProviderForUserResponse, RusotoError<AdminLinkProviderForUserError>>;

    /// <p>Lists devices, as an administrator.</p> <p>Calling this action requires developer credentials.</p>
    async fn admin_list_devices(
        &self,
        input: AdminListDevicesRequest,
    ) -> Result<AdminListDevicesResponse, RusotoError<AdminListDevicesError>>;

    /// <p>Lists the groups that the user belongs to.</p> <p>Calling this action requires developer credentials.</p>
    async fn admin_list_groups_for_user(
        &self,
        input: AdminListGroupsForUserRequest,
    ) -> Result<AdminListGroupsForUserResponse, RusotoError<AdminListGroupsForUserError>>;

    /// <p>Lists a history of user activity and any risks detected as part of Amazon Cognito advanced security.</p>
    async fn admin_list_user_auth_events(
        &self,
        input: AdminListUserAuthEventsRequest,
    ) -> Result<AdminListUserAuthEventsResponse, RusotoError<AdminListUserAuthEventsError>>;

    /// <p>Removes the specified user from the specified group.</p> <p>Calling this action requires developer credentials.</p>
    async fn admin_remove_user_from_group(
        &self,
        input: AdminRemoveUserFromGroupRequest,
    ) -> Result<(), RusotoError<AdminRemoveUserFromGroupError>>;

    /// <p>Resets the specified user's password in a user pool as an administrator. Works on any user.</p> <p>When a developer calls this API, the current password is invalidated, so it must be changed. If a user tries to sign in after the API is called, the app will get a PasswordResetRequiredException exception back and should direct the user down the flow to reset the password, which is the same as the forgot password flow. In addition, if the user pool has phone verification selected and a verified phone number exists for the user, or if email verification is selected and a verified email exists for the user, calling this API will also result in sending a message to the end user with the code to change their password.</p> <p>Calling this action requires developer credentials.</p>
    async fn admin_reset_user_password(
        &self,
        input: AdminResetUserPasswordRequest,
    ) -> Result<AdminResetUserPasswordResponse, RusotoError<AdminResetUserPasswordError>>;

    /// <p>Responds to an authentication challenge, as an administrator.</p> <p>Calling this action requires developer credentials.</p>
    async fn admin_respond_to_auth_challenge(
        &self,
        input: AdminRespondToAuthChallengeRequest,
    ) -> Result<AdminRespondToAuthChallengeResponse, RusotoError<AdminRespondToAuthChallengeError>>;

    /// <p>Sets the user's multi-factor authentication (MFA) preference, including which MFA options are enabled and if any are preferred. Only one factor can be set as preferred. The preferred MFA factor will be used to authenticate a user if multiple factors are enabled. If multiple options are enabled and no preference is set, a challenge to choose an MFA option will be returned during sign in.</p>
    async fn admin_set_user_mfa_preference(
        &self,
        input: AdminSetUserMFAPreferenceRequest,
    ) -> Result<AdminSetUserMFAPreferenceResponse, RusotoError<AdminSetUserMFAPreferenceError>>;

    /// <p>Sets the specified user's password in a user pool as an administrator. Works on any user. </p> <p>The password can be temporary or permanent. If it is temporary, the user status will be placed into the <code>FORCE_CHANGE_PASSWORD</code> state. When the user next tries to sign in, the InitiateAuth/AdminInitiateAuth response will contain the <code>NEW_PASSWORD_REQUIRED</code> challenge. If the user does not sign in before it expires, the user will not be able to sign in and their password will need to be reset by an administrator. </p> <p>Once the user has set a new password, or the password is permanent, the user status will be set to <code>Confirmed</code>.</p>
    async fn admin_set_user_password(
        &self,
        input: AdminSetUserPasswordRequest,
    ) -> Result<AdminSetUserPasswordResponse, RusotoError<AdminSetUserPasswordError>>;

    /// <p> <i>This action is no longer supported.</i> You can use it to configure only SMS MFA. You can't use it to configure TOTP software token MFA. To configure either type of MFA, use the <a>AdminSetUserMFAPreference</a> action instead.</p>
    async fn admin_set_user_settings(
        &self,
        input: AdminSetUserSettingsRequest,
    ) -> Result<AdminSetUserSettingsResponse, RusotoError<AdminSetUserSettingsError>>;

    /// <p>Provides feedback for an authentication event as to whether it was from a valid user. This feedback is used for improving the risk evaluation decision for the user pool as part of Amazon Cognito advanced security.</p>
    async fn admin_update_auth_event_feedback(
        &self,
        input: AdminUpdateAuthEventFeedbackRequest,
    ) -> Result<AdminUpdateAuthEventFeedbackResponse, RusotoError<AdminUpdateAuthEventFeedbackError>>;

    /// <p>Updates the device status as an administrator.</p> <p>Calling this action requires developer credentials.</p>
    async fn admin_update_device_status(
        &self,
        input: AdminUpdateDeviceStatusRequest,
    ) -> Result<AdminUpdateDeviceStatusResponse, RusotoError<AdminUpdateDeviceStatusError>>;

    /// <p>Updates the specified user's attributes, including developer attributes, as an administrator. Works on any user.</p> <p>For custom attributes, you must prepend the <code>custom:</code> prefix to the attribute name.</p> <p>In addition to updating user attributes, this API can also be used to mark phone and email as verified.</p> <p>Calling this action requires developer credentials.</p>
    async fn admin_update_user_attributes(
        &self,
        input: AdminUpdateUserAttributesRequest,
    ) -> Result<AdminUpdateUserAttributesResponse, RusotoError<AdminUpdateUserAttributesError>>;

    /// <p>Signs out users from all devices, as an administrator. It also invalidates all refresh tokens issued to a user. The user's current access and Id tokens remain valid until their expiry. Access and Id tokens expire one hour after they are issued.</p> <p>Calling this action requires developer credentials.</p>
    async fn admin_user_global_sign_out(
        &self,
        input: AdminUserGlobalSignOutRequest,
    ) -> Result<AdminUserGlobalSignOutResponse, RusotoError<AdminUserGlobalSignOutError>>;

    /// <p>Returns a unique generated shared secret key code for the user account. The request takes an access token or a session string, but not both.</p>
    async fn associate_software_token(
        &self,
        input: AssociateSoftwareTokenRequest,
    ) -> Result<AssociateSoftwareTokenResponse, RusotoError<AssociateSoftwareTokenError>>;

    /// <p>Changes the password for a specified user in a user pool.</p>
    async fn change_password(
        &self,
        input: ChangePasswordRequest,
    ) -> Result<ChangePasswordResponse, RusotoError<ChangePasswordError>>;

    /// <p>Confirms tracking of the device. This API call is the call that begins device tracking.</p>
    async fn confirm_device(
        &self,
        input: ConfirmDeviceRequest,
    ) -> Result<ConfirmDeviceResponse, RusotoError<ConfirmDeviceError>>;

    /// <p>Allows a user to enter a confirmation code to reset a forgotten password.</p>
    async fn confirm_forgot_password(
        &self,
        input: ConfirmForgotPasswordRequest,
    ) -> Result<ConfirmForgotPasswordResponse, RusotoError<ConfirmForgotPasswordError>>;

    /// <p>Confirms registration of a user and handles the existing alias from a previous user.</p>
    async fn confirm_sign_up(
        &self,
        input: ConfirmSignUpRequest,
    ) -> Result<ConfirmSignUpResponse, RusotoError<ConfirmSignUpError>>;

    /// <p>Creates a new group in the specified user pool.</p> <p>Calling this action requires developer credentials.</p>
    async fn create_group(
        &self,
        input: CreateGroupRequest,
    ) -> Result<CreateGroupResponse, RusotoError<CreateGroupError>>;

    /// <p>Creates an identity provider for a user pool.</p>
    async fn create_identity_provider(
        &self,
        input: CreateIdentityProviderRequest,
    ) -> Result<CreateIdentityProviderResponse, RusotoError<CreateIdentityProviderError>>;

    /// <p>Creates a new OAuth2.0 resource server and defines custom scopes in it.</p>
    async fn create_resource_server(
        &self,
        input: CreateResourceServerRequest,
    ) -> Result<CreateResourceServerResponse, RusotoError<CreateResourceServerError>>;

    /// <p>Creates the user import job.</p>
    async fn create_user_import_job(
        &self,
        input: CreateUserImportJobRequest,
    ) -> Result<CreateUserImportJobResponse, RusotoError<CreateUserImportJobError>>;

    /// <p>Creates a new Amazon Cognito user pool and sets the password policy for the pool.</p>
    async fn create_user_pool(
        &self,
        input: CreateUserPoolRequest,
    ) -> Result<CreateUserPoolResponse, RusotoError<CreateUserPoolError>>;

    /// <p>Creates the user pool client.</p>
    async fn create_user_pool_client(
        &self,
        input: CreateUserPoolClientRequest,
    ) -> Result<CreateUserPoolClientResponse, RusotoError<CreateUserPoolClientError>>;

    /// <p>Creates a new domain for a user pool.</p>
    async fn create_user_pool_domain(
        &self,
        input: CreateUserPoolDomainRequest,
    ) -> Result<CreateUserPoolDomainResponse, RusotoError<CreateUserPoolDomainError>>;

    /// <p>Deletes a group. Currently only groups with no members can be deleted.</p> <p>Calling this action requires developer credentials.</p>
    async fn delete_group(
        &self,
        input: DeleteGroupRequest,
    ) -> Result<(), RusotoError<DeleteGroupError>>;

    /// <p>Deletes an identity provider for a user pool.</p>
    async fn delete_identity_provider(
        &self,
        input: DeleteIdentityProviderRequest,
    ) -> Result<(), RusotoError<DeleteIdentityProviderError>>;

    /// <p>Deletes a resource server.</p>
    async fn delete_resource_server(
        &self,
        input: DeleteResourceServerRequest,
    ) -> Result<(), RusotoError<DeleteResourceServerError>>;

    /// <p>Allows a user to delete himself or herself.</p>
    async fn delete_user(
        &self,
        input: DeleteUserRequest,
    ) -> Result<(), RusotoError<DeleteUserError>>;

    /// <p>Deletes the attributes for a user.</p>
    async fn delete_user_attributes(
        &self,
        input: DeleteUserAttributesRequest,
    ) -> Result<DeleteUserAttributesResponse, RusotoError<DeleteUserAttributesError>>;

    /// <p>Deletes the specified Amazon Cognito user pool.</p>
    async fn delete_user_pool(
        &self,
        input: DeleteUserPoolRequest,
    ) -> Result<(), RusotoError<DeleteUserPoolError>>;

    /// <p>Allows the developer to delete the user pool client.</p>
    async fn delete_user_pool_client(
        &self,
        input: DeleteUserPoolClientRequest,
    ) -> Result<(), RusotoError<DeleteUserPoolClientError>>;

    /// <p>Deletes a domain for a user pool.</p>
    async fn delete_user_pool_domain(
        &self,
        input: DeleteUserPoolDomainRequest,
    ) -> Result<DeleteUserPoolDomainResponse, RusotoError<DeleteUserPoolDomainError>>;

    /// <p>Gets information about a specific identity provider.</p>
    async fn describe_identity_provider(
        &self,
        input: DescribeIdentityProviderRequest,
    ) -> Result<DescribeIdentityProviderResponse, RusotoError<DescribeIdentityProviderError>>;

    /// <p>Describes a resource server.</p>
    async fn describe_resource_server(
        &self,
        input: DescribeResourceServerRequest,
    ) -> Result<DescribeResourceServerResponse, RusotoError<DescribeResourceServerError>>;

    /// <p>Describes the risk configuration.</p>
    async fn describe_risk_configuration(
        &self,
        input: DescribeRiskConfigurationRequest,
    ) -> Result<DescribeRiskConfigurationResponse, RusotoError<DescribeRiskConfigurationError>>;

    /// <p>Describes the user import job.</p>
    async fn describe_user_import_job(
        &self,
        input: DescribeUserImportJobRequest,
    ) -> Result<DescribeUserImportJobResponse, RusotoError<DescribeUserImportJobError>>;

    /// <p>Returns the configuration information and metadata of the specified user pool.</p>
    async fn describe_user_pool(
        &self,
        input: DescribeUserPoolRequest,
    ) -> Result<DescribeUserPoolResponse, RusotoError<DescribeUserPoolError>>;

    /// <p>Client method for returning the configuration information and metadata of the specified user pool app client.</p>
    async fn describe_user_pool_client(
        &self,
        input: DescribeUserPoolClientRequest,
    ) -> Result<DescribeUserPoolClientResponse, RusotoError<DescribeUserPoolClientError>>;

    /// <p>Gets information about a domain.</p>
    async fn describe_user_pool_domain(
        &self,
        input: DescribeUserPoolDomainRequest,
    ) -> Result<DescribeUserPoolDomainResponse, RusotoError<DescribeUserPoolDomainError>>;

    /// <p>Forgets the specified device.</p>
    async fn forget_device(
        &self,
        input: ForgetDeviceRequest,
    ) -> Result<(), RusotoError<ForgetDeviceError>>;

    /// <p>Calling this API causes a message to be sent to the end user with a confirmation code that is required to change the user's password. For the <code>Username</code> parameter, you can use the username or user alias. If a verified phone number exists for the user, the confirmation code is sent to the phone number. Otherwise, if a verified email exists, the confirmation code is sent to the email. If neither a verified phone number nor a verified email exists, <code>InvalidParameterException</code> is thrown. To use the confirmation code for resetting the password, call .</p>
    async fn forgot_password(
        &self,
        input: ForgotPasswordRequest,
    ) -> Result<ForgotPasswordResponse, RusotoError<ForgotPasswordError>>;

    /// <p>Gets the header information for the .csv file to be used as input for the user import job.</p>
    async fn get_csv_header(
        &self,
        input: GetCSVHeaderRequest,
    ) -> Result<GetCSVHeaderResponse, RusotoError<GetCSVHeaderError>>;

    /// <p>Gets the device.</p>
    async fn get_device(
        &self,
        input: GetDeviceRequest,
    ) -> Result<GetDeviceResponse, RusotoError<GetDeviceError>>;

    /// <p>Gets a group.</p> <p>Calling this action requires developer credentials.</p>
    async fn get_group(
        &self,
        input: GetGroupRequest,
    ) -> Result<GetGroupResponse, RusotoError<GetGroupError>>;

    /// <p>Gets the specified identity provider.</p>
    async fn get_identity_provider_by_identifier(
        &self,
        input: GetIdentityProviderByIdentifierRequest,
    ) -> Result<
        GetIdentityProviderByIdentifierResponse,
        RusotoError<GetIdentityProviderByIdentifierError>,
    >;

    /// <p>This method takes a user pool ID, and returns the signing certificate.</p>
    async fn get_signing_certificate(
        &self,
        input: GetSigningCertificateRequest,
    ) -> Result<GetSigningCertificateResponse, RusotoError<GetSigningCertificateError>>;

    /// <p>Gets the UI Customization information for a particular app client's app UI, if there is something set. If nothing is set for the particular client, but there is an existing pool level customization (app <code>clientId</code> will be <code>ALL</code>), then that is returned. If nothing is present, then an empty shape is returned.</p>
    async fn get_ui_customization(
        &self,
        input: GetUICustomizationRequest,
    ) -> Result<GetUICustomizationResponse, RusotoError<GetUICustomizationError>>;

    /// <p>Gets the user attributes and metadata for a user.</p>
    async fn get_user(
        &self,
        input: GetUserRequest,
    ) -> Result<GetUserResponse, RusotoError<GetUserError>>;

    /// <p>Gets the user attribute verification code for the specified attribute name.</p>
    async fn get_user_attribute_verification_code(
        &self,
        input: GetUserAttributeVerificationCodeRequest,
    ) -> Result<
        GetUserAttributeVerificationCodeResponse,
        RusotoError<GetUserAttributeVerificationCodeError>,
    >;

    /// <p>Gets the user pool multi-factor authentication (MFA) configuration.</p>
    async fn get_user_pool_mfa_config(
        &self,
        input: GetUserPoolMfaConfigRequest,
    ) -> Result<GetUserPoolMfaConfigResponse, RusotoError<GetUserPoolMfaConfigError>>;

    /// <p>Signs out users from all devices. It also invalidates all refresh tokens issued to a user. The user's current access and Id tokens remain valid until their expiry. Access and Id tokens expire one hour after they are issued.</p>
    async fn global_sign_out(
        &self,
        input: GlobalSignOutRequest,
    ) -> Result<GlobalSignOutResponse, RusotoError<GlobalSignOutError>>;

    /// <p>Initiates the authentication flow.</p>
    async fn initiate_auth(
        &self,
        input: InitiateAuthRequest,
    ) -> Result<InitiateAuthResponse, RusotoError<InitiateAuthError>>;

    /// <p>Lists the devices.</p>
    async fn list_devices(
        &self,
        input: ListDevicesRequest,
    ) -> Result<ListDevicesResponse, RusotoError<ListDevicesError>>;

    /// <p>Lists the groups associated with a user pool.</p> <p>Calling this action requires developer credentials.</p>
    async fn list_groups(
        &self,
        input: ListGroupsRequest,
    ) -> Result<ListGroupsResponse, RusotoError<ListGroupsError>>;

    /// <p>Lists information about all identity providers for a user pool.</p>
    async fn list_identity_providers(
        &self,
        input: ListIdentityProvidersRequest,
    ) -> Result<ListIdentityProvidersResponse, RusotoError<ListIdentityProvidersError>>;

    /// <p>Lists the resource servers for a user pool.</p>
    async fn list_resource_servers(
        &self,
        input: ListResourceServersRequest,
    ) -> Result<ListResourceServersResponse, RusotoError<ListResourceServersError>>;

    /// <p>Lists the tags that are assigned to an Amazon Cognito user pool.</p> <p>A tag is a label that you can apply to user pools to categorize and manage them in different ways, such as by purpose, owner, environment, or other criteria.</p> <p>You can use this action up to 10 times per second, per account.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Lists the user import jobs.</p>
    async fn list_user_import_jobs(
        &self,
        input: ListUserImportJobsRequest,
    ) -> Result<ListUserImportJobsResponse, RusotoError<ListUserImportJobsError>>;

    /// <p>Lists the clients that have been created for the specified user pool.</p>
    async fn list_user_pool_clients(
        &self,
        input: ListUserPoolClientsRequest,
    ) -> Result<ListUserPoolClientsResponse, RusotoError<ListUserPoolClientsError>>;

    /// <p>Lists the user pools associated with an AWS account.</p>
    async fn list_user_pools(
        &self,
        input: ListUserPoolsRequest,
    ) -> Result<ListUserPoolsResponse, RusotoError<ListUserPoolsError>>;

    /// <p>Lists the users in the Amazon Cognito user pool.</p>
    async fn list_users(
        &self,
        input: ListUsersRequest,
    ) -> Result<ListUsersResponse, RusotoError<ListUsersError>>;

    /// <p>Lists the users in the specified group.</p> <p>Calling this action requires developer credentials.</p>
    async fn list_users_in_group(
        &self,
        input: ListUsersInGroupRequest,
    ) -> Result<ListUsersInGroupResponse, RusotoError<ListUsersInGroupError>>;

    /// <p>Resends the confirmation (for confirmation of registration) to a specific user in the user pool.</p>
    async fn resend_confirmation_code(
        &self,
        input: ResendConfirmationCodeRequest,
    ) -> Result<ResendConfirmationCodeResponse, RusotoError<ResendConfirmationCodeError>>;

    /// <p>Responds to the authentication challenge.</p>
    async fn respond_to_auth_challenge(
        &self,
        input: RespondToAuthChallengeRequest,
    ) -> Result<RespondToAuthChallengeResponse, RusotoError<RespondToAuthChallengeError>>;

    /// <p>Configures actions on detected risks. To delete the risk configuration for <code>UserPoolId</code> or <code>ClientId</code>, pass null values for all four configuration types.</p> <p>To enable Amazon Cognito advanced security features, update the user pool to include the <code>UserPoolAddOns</code> key<code>AdvancedSecurityMode</code>.</p> <p>See .</p>
    async fn set_risk_configuration(
        &self,
        input: SetRiskConfigurationRequest,
    ) -> Result<SetRiskConfigurationResponse, RusotoError<SetRiskConfigurationError>>;

    /// <p><p>Sets the UI customization information for a user pool&#39;s built-in app UI.</p> <p>You can specify app UI customization settings for a single client (with a specific <code>clientId</code>) or for all clients (by setting the <code>clientId</code> to <code>ALL</code>). If you specify <code>ALL</code>, the default configuration will be used for every client that has no UI customization set previously. If you specify UI customization settings for a particular client, it will no longer fall back to the <code>ALL</code> configuration. </p> <note> <p>To use this API, your user pool must have a domain associated with it. Otherwise, there is no place to host the app&#39;s pages, and the service will throw an error.</p> </note></p>
    async fn set_ui_customization(
        &self,
        input: SetUICustomizationRequest,
    ) -> Result<SetUICustomizationResponse, RusotoError<SetUICustomizationError>>;

    /// <p>Set the user's multi-factor authentication (MFA) method preference, including which MFA factors are enabled and if any are preferred. Only one factor can be set as preferred. The preferred MFA factor will be used to authenticate a user if multiple factors are enabled. If multiple options are enabled and no preference is set, a challenge to choose an MFA option will be returned during sign in.</p>
    async fn set_user_mfa_preference(
        &self,
        input: SetUserMFAPreferenceRequest,
    ) -> Result<SetUserMFAPreferenceResponse, RusotoError<SetUserMFAPreferenceError>>;

    /// <p>Set the user pool multi-factor authentication (MFA) configuration.</p>
    async fn set_user_pool_mfa_config(
        &self,
        input: SetUserPoolMfaConfigRequest,
    ) -> Result<SetUserPoolMfaConfigResponse, RusotoError<SetUserPoolMfaConfigError>>;

    /// <p> <i>This action is no longer supported.</i> You can use it to configure only SMS MFA. You can't use it to configure TOTP software token MFA. To configure either type of MFA, use the <a>SetUserMFAPreference</a> action instead.</p>
    async fn set_user_settings(
        &self,
        input: SetUserSettingsRequest,
    ) -> Result<SetUserSettingsResponse, RusotoError<SetUserSettingsError>>;

    /// <p>Registers the user in the specified user pool and creates a user name, password, and user attributes.</p>
    async fn sign_up(
        &self,
        input: SignUpRequest,
    ) -> Result<SignUpResponse, RusotoError<SignUpError>>;

    /// <p>Starts the user import.</p>
    async fn start_user_import_job(
        &self,
        input: StartUserImportJobRequest,
    ) -> Result<StartUserImportJobResponse, RusotoError<StartUserImportJobError>>;

    /// <p>Stops the user import job.</p>
    async fn stop_user_import_job(
        &self,
        input: StopUserImportJobRequest,
    ) -> Result<StopUserImportJobResponse, RusotoError<StopUserImportJobError>>;

    /// <p>Assigns a set of tags to an Amazon Cognito user pool. A tag is a label that you can use to categorize and manage user pools in different ways, such as by purpose, owner, environment, or other criteria.</p> <p>Each tag consists of a key and value, both of which you define. A key is a general category for more specific values. For example, if you have two versions of a user pool, one for testing and another for production, you might assign an <code>Environment</code> tag key to both user pools. The value of this key might be <code>Test</code> for one user pool and <code>Production</code> for the other.</p> <p>Tags are useful for cost tracking and access control. You can activate your tags so that they appear on the Billing and Cost Management console, where you can track the costs associated with your user pools. In an IAM policy, you can constrain permissions for user pools based on specific tags or tag values.</p> <p>You can use this action up to 5 times per second, per account. A user pool can have as many as 50 tags.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Removes the specified tags from an Amazon Cognito user pool. You can use this action up to 5 times per second, per account</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p>Provides the feedback for an authentication event whether it was from a valid user or not. This feedback is used for improving the risk evaluation decision for the user pool as part of Amazon Cognito advanced security.</p>
    async fn update_auth_event_feedback(
        &self,
        input: UpdateAuthEventFeedbackRequest,
    ) -> Result<UpdateAuthEventFeedbackResponse, RusotoError<UpdateAuthEventFeedbackError>>;

    /// <p>Updates the device status.</p>
    async fn update_device_status(
        &self,
        input: UpdateDeviceStatusRequest,
    ) -> Result<UpdateDeviceStatusResponse, RusotoError<UpdateDeviceStatusError>>;

    /// <p><p>Updates the specified group with the specified attributes.</p> <p>Calling this action requires developer credentials.</p> <important> <p>If you don&#39;t provide a value for an attribute, it will be set to the default value.</p> </important></p>
    async fn update_group(
        &self,
        input: UpdateGroupRequest,
    ) -> Result<UpdateGroupResponse, RusotoError<UpdateGroupError>>;

    /// <p>Updates identity provider information for a user pool.</p>
    async fn update_identity_provider(
        &self,
        input: UpdateIdentityProviderRequest,
    ) -> Result<UpdateIdentityProviderResponse, RusotoError<UpdateIdentityProviderError>>;

    /// <p><p>Updates the name and scopes of resource server. All other fields are read-only.</p> <important> <p>If you don&#39;t provide a value for an attribute, it will be set to the default value.</p> </important></p>
    async fn update_resource_server(
        &self,
        input: UpdateResourceServerRequest,
    ) -> Result<UpdateResourceServerResponse, RusotoError<UpdateResourceServerError>>;

    /// <p>Allows a user to update a specific attribute (one at a time).</p>
    async fn update_user_attributes(
        &self,
        input: UpdateUserAttributesRequest,
    ) -> Result<UpdateUserAttributesResponse, RusotoError<UpdateUserAttributesError>>;

    /// <p><p>Updates the specified user pool with the specified attributes. You can get a list of the current user pool settings with .</p> <important> <p>If you don&#39;t provide a value for an attribute, it will be set to the default value.</p> </important></p>
    async fn update_user_pool(
        &self,
        input: UpdateUserPoolRequest,
    ) -> Result<UpdateUserPoolResponse, RusotoError<UpdateUserPoolError>>;

    /// <p><p>Updates the specified user pool app client with the specified attributes. You can get a list of the current user pool app client settings with .</p> <important> <p>If you don&#39;t provide a value for an attribute, it will be set to the default value.</p> </important></p>
    async fn update_user_pool_client(
        &self,
        input: UpdateUserPoolClientRequest,
    ) -> Result<UpdateUserPoolClientResponse, RusotoError<UpdateUserPoolClientError>>;

    /// <p>Updates the Secure Sockets Layer (SSL) certificate for the custom domain for your user pool.</p> <p>You can use this operation to provide the Amazon Resource Name (ARN) of a new certificate to Amazon Cognito. You cannot use it to change the domain for a user pool.</p> <p>A custom domain is used to host the Amazon Cognito hosted UI, which provides sign-up and sign-in pages for your application. When you set up a custom domain, you provide a certificate that you manage with AWS Certificate Manager (ACM). When necessary, you can use this operation to change the certificate that you applied to your custom domain.</p> <p>Usually, this is unnecessary following routine certificate renewal with ACM. When you renew your existing certificate in ACM, the ARN for your certificate remains the same, and your custom domain uses the new certificate automatically.</p> <p>However, if you replace your existing certificate with a new one, ACM gives the new certificate a new ARN. To apply the new certificate to your custom domain, you must provide this ARN to Amazon Cognito.</p> <p>When you add your new certificate in ACM, you must choose US East (N. Virginia) as the AWS Region.</p> <p>After you submit your request, Amazon Cognito requires up to 1 hour to distribute your new certificate to your custom domain.</p> <p>For more information about adding a custom domain to your user pool, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-add-custom-domain.html">Using Your Own Domain for the Hosted UI</a>.</p>
    async fn update_user_pool_domain(
        &self,
        input: UpdateUserPoolDomainRequest,
    ) -> Result<UpdateUserPoolDomainResponse, RusotoError<UpdateUserPoolDomainError>>;

    /// <p>Use this API to register a user's entered TOTP code and mark the user's software token MFA status as "verified" if successful. The request takes an access token or a session string, but not both.</p>
    async fn verify_software_token(
        &self,
        input: VerifySoftwareTokenRequest,
    ) -> Result<VerifySoftwareTokenResponse, RusotoError<VerifySoftwareTokenError>>;

    /// <p>Verifies the specified user attributes in the user pool.</p>
    async fn verify_user_attribute(
        &self,
        input: VerifyUserAttributeRequest,
    ) -> Result<VerifyUserAttributeResponse, RusotoError<VerifyUserAttributeError>>;
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
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> CognitoIdentityProviderClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        CognitoIdentityProviderClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(
        client: Client,
        region: region::Region,
    ) -> CognitoIdentityProviderClient {
        CognitoIdentityProviderClient { client, region }
    }
}

#[async_trait]
impl CognitoIdentityProvider for CognitoIdentityProviderClient {
    /// <p>Adds additional user attributes to the user pool schema.</p>
    async fn add_custom_attributes(
        &self,
        input: AddCustomAttributesRequest,
    ) -> Result<AddCustomAttributesResponse, RusotoError<AddCustomAttributesError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AddCustomAttributes",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<AddCustomAttributesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AddCustomAttributesError::from_response(response))
        }
    }

    /// <p>Adds the specified user to the specified group.</p> <p>Calling this action requires developer credentials.</p>
    async fn admin_add_user_to_group(
        &self,
        input: AdminAddUserToGroupRequest,
    ) -> Result<(), RusotoError<AdminAddUserToGroupError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminAddUserToGroup",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AdminAddUserToGroupError::from_response(response))
        }
    }

    /// <p>Confirms user registration as an admin without using a confirmation code. Works on any user.</p> <p>Calling this action requires developer credentials.</p>
    async fn admin_confirm_sign_up(
        &self,
        input: AdminConfirmSignUpRequest,
    ) -> Result<AdminConfirmSignUpResponse, RusotoError<AdminConfirmSignUpError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminConfirmSignUp",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<AdminConfirmSignUpResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AdminConfirmSignUpError::from_response(response))
        }
    }

    /// <p>Creates a new user in the specified user pool.</p> <p>If <code>MessageAction</code> is not set, the default is to send a welcome message via email or phone (SMS).</p> <note> <p>This message is based on a template that you configured in your call to or . This template includes your custom sign-up instructions and placeholders for user name and temporary password.</p> </note> <p>Alternatively, you can call AdminCreateUser with “SUPPRESS” for the <code>MessageAction</code> parameter, and Amazon Cognito will not send any email. </p> <p>In either case, the user will be in the <code>FORCE_CHANGE_PASSWORD</code> state until they sign in and change their password.</p> <p>AdminCreateUser requires developer credentials.</p>
    async fn admin_create_user(
        &self,
        input: AdminCreateUserRequest,
    ) -> Result<AdminCreateUserResponse, RusotoError<AdminCreateUserError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminCreateUser",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<AdminCreateUserResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AdminCreateUserError::from_response(response))
        }
    }

    /// <p>Deletes a user as an administrator. Works on any user.</p> <p>Calling this action requires developer credentials.</p>
    async fn admin_delete_user(
        &self,
        input: AdminDeleteUserRequest,
    ) -> Result<(), RusotoError<AdminDeleteUserError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminDeleteUser",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AdminDeleteUserError::from_response(response))
        }
    }

    /// <p>Deletes the user attributes in a user pool as an administrator. Works on any user.</p> <p>Calling this action requires developer credentials.</p>
    async fn admin_delete_user_attributes(
        &self,
        input: AdminDeleteUserAttributesRequest,
    ) -> Result<AdminDeleteUserAttributesResponse, RusotoError<AdminDeleteUserAttributesError>>
    {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminDeleteUserAttributes",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<AdminDeleteUserAttributesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AdminDeleteUserAttributesError::from_response(response))
        }
    }

    /// <p>Disables the user from signing in with the specified external (SAML or social) identity provider. If the user to disable is a Cognito User Pools native username + password user, they are not permitted to use their password to sign-in. If the user to disable is a linked external IdP user, any link between that user and an existing user is removed. The next time the external user (no longer attached to the previously linked <code>DestinationUser</code>) signs in, they must create a new user account. See .</p> <p>This action is enabled only for admin access and requires developer credentials.</p> <p>The <code>ProviderName</code> must match the value specified when creating an IdP for the pool. </p> <p>To disable a native username + password user, the <code>ProviderName</code> value must be <code>Cognito</code> and the <code>ProviderAttributeName</code> must be <code>Cognito_Subject</code>, with the <code>ProviderAttributeValue</code> being the name that is used in the user pool for the user.</p> <p>The <code>ProviderAttributeName</code> must always be <code>Cognito_Subject</code> for social identity providers. The <code>ProviderAttributeValue</code> must always be the exact subject that was used when the user was originally linked as a source user.</p> <p>For de-linking a SAML identity, there are two scenarios. If the linked identity has not yet been used to sign-in, the <code>ProviderAttributeName</code> and <code>ProviderAttributeValue</code> must be the same values that were used for the <code>SourceUser</code> when the identities were originally linked in the call. (If the linking was done with <code>ProviderAttributeName</code> set to <code>Cognito_Subject</code>, the same applies here). However, if the user has already signed in, the <code>ProviderAttributeName</code> must be <code>Cognito_Subject</code> and <code>ProviderAttributeValue</code> must be the subject of the SAML assertion.</p>
    async fn admin_disable_provider_for_user(
        &self,
        input: AdminDisableProviderForUserRequest,
    ) -> Result<AdminDisableProviderForUserResponse, RusotoError<AdminDisableProviderForUserError>>
    {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminDisableProviderForUser",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<AdminDisableProviderForUserResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AdminDisableProviderForUserError::from_response(response))
        }
    }

    /// <p>Disables the specified user.</p> <p>Calling this action requires developer credentials.</p>
    async fn admin_disable_user(
        &self,
        input: AdminDisableUserRequest,
    ) -> Result<AdminDisableUserResponse, RusotoError<AdminDisableUserError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminDisableUser",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<AdminDisableUserResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AdminDisableUserError::from_response(response))
        }
    }

    /// <p>Enables the specified user as an administrator. Works on any user.</p> <p>Calling this action requires developer credentials.</p>
    async fn admin_enable_user(
        &self,
        input: AdminEnableUserRequest,
    ) -> Result<AdminEnableUserResponse, RusotoError<AdminEnableUserError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminEnableUser",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<AdminEnableUserResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AdminEnableUserError::from_response(response))
        }
    }

    /// <p>Forgets the device, as an administrator.</p> <p>Calling this action requires developer credentials.</p>
    async fn admin_forget_device(
        &self,
        input: AdminForgetDeviceRequest,
    ) -> Result<(), RusotoError<AdminForgetDeviceError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminForgetDevice",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AdminForgetDeviceError::from_response(response))
        }
    }

    /// <p>Gets the device, as an administrator.</p> <p>Calling this action requires developer credentials.</p>
    async fn admin_get_device(
        &self,
        input: AdminGetDeviceRequest,
    ) -> Result<AdminGetDeviceResponse, RusotoError<AdminGetDeviceError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminGetDevice",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<AdminGetDeviceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AdminGetDeviceError::from_response(response))
        }
    }

    /// <p>Gets the specified user by user name in a user pool as an administrator. Works on any user.</p> <p>Calling this action requires developer credentials.</p>
    async fn admin_get_user(
        &self,
        input: AdminGetUserRequest,
    ) -> Result<AdminGetUserResponse, RusotoError<AdminGetUserError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminGetUser",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<AdminGetUserResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AdminGetUserError::from_response(response))
        }
    }

    /// <p>Initiates the authentication flow, as an administrator.</p> <p>Calling this action requires developer credentials.</p>
    async fn admin_initiate_auth(
        &self,
        input: AdminInitiateAuthRequest,
    ) -> Result<AdminInitiateAuthResponse, RusotoError<AdminInitiateAuthError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminInitiateAuth",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<AdminInitiateAuthResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AdminInitiateAuthError::from_response(response))
        }
    }

    /// <p>Links an existing user account in a user pool (<code>DestinationUser</code>) to an identity from an external identity provider (<code>SourceUser</code>) based on a specified attribute name and value from the external identity provider. This allows you to create a link from the existing user account to an external federated user identity that has not yet been used to sign in, so that the federated user identity can be used to sign in as the existing user account. </p> <p> For example, if there is an existing user with a username and password, this API links that user to a federated user identity, so that when the federated user identity is used, the user signs in as the existing user account. </p> <important> <p>Because this API allows a user with an external federated identity to sign in as an existing user in the user pool, it is critical that it only be used with external identity providers and provider attributes that have been trusted by the application owner.</p> </important> <p>See also .</p> <p>This action is enabled only for admin access and requires developer credentials.</p>
    async fn admin_link_provider_for_user(
        &self,
        input: AdminLinkProviderForUserRequest,
    ) -> Result<AdminLinkProviderForUserResponse, RusotoError<AdminLinkProviderForUserError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminLinkProviderForUser",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<AdminLinkProviderForUserResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AdminLinkProviderForUserError::from_response(response))
        }
    }

    /// <p>Lists devices, as an administrator.</p> <p>Calling this action requires developer credentials.</p>
    async fn admin_list_devices(
        &self,
        input: AdminListDevicesRequest,
    ) -> Result<AdminListDevicesResponse, RusotoError<AdminListDevicesError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminListDevices",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<AdminListDevicesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AdminListDevicesError::from_response(response))
        }
    }

    /// <p>Lists the groups that the user belongs to.</p> <p>Calling this action requires developer credentials.</p>
    async fn admin_list_groups_for_user(
        &self,
        input: AdminListGroupsForUserRequest,
    ) -> Result<AdminListGroupsForUserResponse, RusotoError<AdminListGroupsForUserError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminListGroupsForUser",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<AdminListGroupsForUserResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AdminListGroupsForUserError::from_response(response))
        }
    }

    /// <p>Lists a history of user activity and any risks detected as part of Amazon Cognito advanced security.</p>
    async fn admin_list_user_auth_events(
        &self,
        input: AdminListUserAuthEventsRequest,
    ) -> Result<AdminListUserAuthEventsResponse, RusotoError<AdminListUserAuthEventsError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminListUserAuthEvents",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<AdminListUserAuthEventsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AdminListUserAuthEventsError::from_response(response))
        }
    }

    /// <p>Removes the specified user from the specified group.</p> <p>Calling this action requires developer credentials.</p>
    async fn admin_remove_user_from_group(
        &self,
        input: AdminRemoveUserFromGroupRequest,
    ) -> Result<(), RusotoError<AdminRemoveUserFromGroupError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminRemoveUserFromGroup",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AdminRemoveUserFromGroupError::from_response(response))
        }
    }

    /// <p>Resets the specified user's password in a user pool as an administrator. Works on any user.</p> <p>When a developer calls this API, the current password is invalidated, so it must be changed. If a user tries to sign in after the API is called, the app will get a PasswordResetRequiredException exception back and should direct the user down the flow to reset the password, which is the same as the forgot password flow. In addition, if the user pool has phone verification selected and a verified phone number exists for the user, or if email verification is selected and a verified email exists for the user, calling this API will also result in sending a message to the end user with the code to change their password.</p> <p>Calling this action requires developer credentials.</p>
    async fn admin_reset_user_password(
        &self,
        input: AdminResetUserPasswordRequest,
    ) -> Result<AdminResetUserPasswordResponse, RusotoError<AdminResetUserPasswordError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminResetUserPassword",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<AdminResetUserPasswordResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AdminResetUserPasswordError::from_response(response))
        }
    }

    /// <p>Responds to an authentication challenge, as an administrator.</p> <p>Calling this action requires developer credentials.</p>
    async fn admin_respond_to_auth_challenge(
        &self,
        input: AdminRespondToAuthChallengeRequest,
    ) -> Result<AdminRespondToAuthChallengeResponse, RusotoError<AdminRespondToAuthChallengeError>>
    {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminRespondToAuthChallenge",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<AdminRespondToAuthChallengeResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AdminRespondToAuthChallengeError::from_response(response))
        }
    }

    /// <p>Sets the user's multi-factor authentication (MFA) preference, including which MFA options are enabled and if any are preferred. Only one factor can be set as preferred. The preferred MFA factor will be used to authenticate a user if multiple factors are enabled. If multiple options are enabled and no preference is set, a challenge to choose an MFA option will be returned during sign in.</p>
    async fn admin_set_user_mfa_preference(
        &self,
        input: AdminSetUserMFAPreferenceRequest,
    ) -> Result<AdminSetUserMFAPreferenceResponse, RusotoError<AdminSetUserMFAPreferenceError>>
    {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminSetUserMFAPreference",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<AdminSetUserMFAPreferenceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AdminSetUserMFAPreferenceError::from_response(response))
        }
    }

    /// <p>Sets the specified user's password in a user pool as an administrator. Works on any user. </p> <p>The password can be temporary or permanent. If it is temporary, the user status will be placed into the <code>FORCE_CHANGE_PASSWORD</code> state. When the user next tries to sign in, the InitiateAuth/AdminInitiateAuth response will contain the <code>NEW_PASSWORD_REQUIRED</code> challenge. If the user does not sign in before it expires, the user will not be able to sign in and their password will need to be reset by an administrator. </p> <p>Once the user has set a new password, or the password is permanent, the user status will be set to <code>Confirmed</code>.</p>
    async fn admin_set_user_password(
        &self,
        input: AdminSetUserPasswordRequest,
    ) -> Result<AdminSetUserPasswordResponse, RusotoError<AdminSetUserPasswordError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminSetUserPassword",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<AdminSetUserPasswordResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AdminSetUserPasswordError::from_response(response))
        }
    }

    /// <p> <i>This action is no longer supported.</i> You can use it to configure only SMS MFA. You can't use it to configure TOTP software token MFA. To configure either type of MFA, use the <a>AdminSetUserMFAPreference</a> action instead.</p>
    async fn admin_set_user_settings(
        &self,
        input: AdminSetUserSettingsRequest,
    ) -> Result<AdminSetUserSettingsResponse, RusotoError<AdminSetUserSettingsError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminSetUserSettings",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<AdminSetUserSettingsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AdminSetUserSettingsError::from_response(response))
        }
    }

    /// <p>Provides feedback for an authentication event as to whether it was from a valid user. This feedback is used for improving the risk evaluation decision for the user pool as part of Amazon Cognito advanced security.</p>
    async fn admin_update_auth_event_feedback(
        &self,
        input: AdminUpdateAuthEventFeedbackRequest,
    ) -> Result<AdminUpdateAuthEventFeedbackResponse, RusotoError<AdminUpdateAuthEventFeedbackError>>
    {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminUpdateAuthEventFeedback",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<AdminUpdateAuthEventFeedbackResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AdminUpdateAuthEventFeedbackError::from_response(response))
        }
    }

    /// <p>Updates the device status as an administrator.</p> <p>Calling this action requires developer credentials.</p>
    async fn admin_update_device_status(
        &self,
        input: AdminUpdateDeviceStatusRequest,
    ) -> Result<AdminUpdateDeviceStatusResponse, RusotoError<AdminUpdateDeviceStatusError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminUpdateDeviceStatus",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<AdminUpdateDeviceStatusResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AdminUpdateDeviceStatusError::from_response(response))
        }
    }

    /// <p>Updates the specified user's attributes, including developer attributes, as an administrator. Works on any user.</p> <p>For custom attributes, you must prepend the <code>custom:</code> prefix to the attribute name.</p> <p>In addition to updating user attributes, this API can also be used to mark phone and email as verified.</p> <p>Calling this action requires developer credentials.</p>
    async fn admin_update_user_attributes(
        &self,
        input: AdminUpdateUserAttributesRequest,
    ) -> Result<AdminUpdateUserAttributesResponse, RusotoError<AdminUpdateUserAttributesError>>
    {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminUpdateUserAttributes",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<AdminUpdateUserAttributesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AdminUpdateUserAttributesError::from_response(response))
        }
    }

    /// <p>Signs out users from all devices, as an administrator. It also invalidates all refresh tokens issued to a user. The user's current access and Id tokens remain valid until their expiry. Access and Id tokens expire one hour after they are issued.</p> <p>Calling this action requires developer credentials.</p>
    async fn admin_user_global_sign_out(
        &self,
        input: AdminUserGlobalSignOutRequest,
    ) -> Result<AdminUserGlobalSignOutResponse, RusotoError<AdminUserGlobalSignOutError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AdminUserGlobalSignOut",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<AdminUserGlobalSignOutResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AdminUserGlobalSignOutError::from_response(response))
        }
    }

    /// <p>Returns a unique generated shared secret key code for the user account. The request takes an access token or a session string, but not both.</p>
    async fn associate_software_token(
        &self,
        input: AssociateSoftwareTokenRequest,
    ) -> Result<AssociateSoftwareTokenResponse, RusotoError<AssociateSoftwareTokenError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.AssociateSoftwareToken",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<AssociateSoftwareTokenResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AssociateSoftwareTokenError::from_response(response))
        }
    }

    /// <p>Changes the password for a specified user in a user pool.</p>
    async fn change_password(
        &self,
        input: ChangePasswordRequest,
    ) -> Result<ChangePasswordResponse, RusotoError<ChangePasswordError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.ChangePassword",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ChangePasswordResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ChangePasswordError::from_response(response))
        }
    }

    /// <p>Confirms tracking of the device. This API call is the call that begins device tracking.</p>
    async fn confirm_device(
        &self,
        input: ConfirmDeviceRequest,
    ) -> Result<ConfirmDeviceResponse, RusotoError<ConfirmDeviceError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.ConfirmDevice",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ConfirmDeviceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ConfirmDeviceError::from_response(response))
        }
    }

    /// <p>Allows a user to enter a confirmation code to reset a forgotten password.</p>
    async fn confirm_forgot_password(
        &self,
        input: ConfirmForgotPasswordRequest,
    ) -> Result<ConfirmForgotPasswordResponse, RusotoError<ConfirmForgotPasswordError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.ConfirmForgotPassword",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ConfirmForgotPasswordResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ConfirmForgotPasswordError::from_response(response))
        }
    }

    /// <p>Confirms registration of a user and handles the existing alias from a previous user.</p>
    async fn confirm_sign_up(
        &self,
        input: ConfirmSignUpRequest,
    ) -> Result<ConfirmSignUpResponse, RusotoError<ConfirmSignUpError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.ConfirmSignUp",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ConfirmSignUpResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ConfirmSignUpError::from_response(response))
        }
    }

    /// <p>Creates a new group in the specified user pool.</p> <p>Calling this action requires developer credentials.</p>
    async fn create_group(
        &self,
        input: CreateGroupRequest,
    ) -> Result<CreateGroupResponse, RusotoError<CreateGroupError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.CreateGroup",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateGroupResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateGroupError::from_response(response))
        }
    }

    /// <p>Creates an identity provider for a user pool.</p>
    async fn create_identity_provider(
        &self,
        input: CreateIdentityProviderRequest,
    ) -> Result<CreateIdentityProviderResponse, RusotoError<CreateIdentityProviderError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.CreateIdentityProvider",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateIdentityProviderResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateIdentityProviderError::from_response(response))
        }
    }

    /// <p>Creates a new OAuth2.0 resource server and defines custom scopes in it.</p>
    async fn create_resource_server(
        &self,
        input: CreateResourceServerRequest,
    ) -> Result<CreateResourceServerResponse, RusotoError<CreateResourceServerError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.CreateResourceServer",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateResourceServerResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateResourceServerError::from_response(response))
        }
    }

    /// <p>Creates the user import job.</p>
    async fn create_user_import_job(
        &self,
        input: CreateUserImportJobRequest,
    ) -> Result<CreateUserImportJobResponse, RusotoError<CreateUserImportJobError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.CreateUserImportJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateUserImportJobResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateUserImportJobError::from_response(response))
        }
    }

    /// <p>Creates a new Amazon Cognito user pool and sets the password policy for the pool.</p>
    async fn create_user_pool(
        &self,
        input: CreateUserPoolRequest,
    ) -> Result<CreateUserPoolResponse, RusotoError<CreateUserPoolError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.CreateUserPool",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateUserPoolResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateUserPoolError::from_response(response))
        }
    }

    /// <p>Creates the user pool client.</p>
    async fn create_user_pool_client(
        &self,
        input: CreateUserPoolClientRequest,
    ) -> Result<CreateUserPoolClientResponse, RusotoError<CreateUserPoolClientError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.CreateUserPoolClient",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateUserPoolClientResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateUserPoolClientError::from_response(response))
        }
    }

    /// <p>Creates a new domain for a user pool.</p>
    async fn create_user_pool_domain(
        &self,
        input: CreateUserPoolDomainRequest,
    ) -> Result<CreateUserPoolDomainResponse, RusotoError<CreateUserPoolDomainError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.CreateUserPoolDomain",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateUserPoolDomainResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateUserPoolDomainError::from_response(response))
        }
    }

    /// <p>Deletes a group. Currently only groups with no members can be deleted.</p> <p>Calling this action requires developer credentials.</p>
    async fn delete_group(
        &self,
        input: DeleteGroupRequest,
    ) -> Result<(), RusotoError<DeleteGroupError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.DeleteGroup",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteGroupError::from_response(response))
        }
    }

    /// <p>Deletes an identity provider for a user pool.</p>
    async fn delete_identity_provider(
        &self,
        input: DeleteIdentityProviderRequest,
    ) -> Result<(), RusotoError<DeleteIdentityProviderError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.DeleteIdentityProvider",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteIdentityProviderError::from_response(response))
        }
    }

    /// <p>Deletes a resource server.</p>
    async fn delete_resource_server(
        &self,
        input: DeleteResourceServerRequest,
    ) -> Result<(), RusotoError<DeleteResourceServerError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.DeleteResourceServer",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteResourceServerError::from_response(response))
        }
    }

    /// <p>Allows a user to delete himself or herself.</p>
    async fn delete_user(
        &self,
        input: DeleteUserRequest,
    ) -> Result<(), RusotoError<DeleteUserError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.DeleteUser",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteUserError::from_response(response))
        }
    }

    /// <p>Deletes the attributes for a user.</p>
    async fn delete_user_attributes(
        &self,
        input: DeleteUserAttributesRequest,
    ) -> Result<DeleteUserAttributesResponse, RusotoError<DeleteUserAttributesError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.DeleteUserAttributes",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteUserAttributesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteUserAttributesError::from_response(response))
        }
    }

    /// <p>Deletes the specified Amazon Cognito user pool.</p>
    async fn delete_user_pool(
        &self,
        input: DeleteUserPoolRequest,
    ) -> Result<(), RusotoError<DeleteUserPoolError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.DeleteUserPool",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteUserPoolError::from_response(response))
        }
    }

    /// <p>Allows the developer to delete the user pool client.</p>
    async fn delete_user_pool_client(
        &self,
        input: DeleteUserPoolClientRequest,
    ) -> Result<(), RusotoError<DeleteUserPoolClientError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.DeleteUserPoolClient",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteUserPoolClientError::from_response(response))
        }
    }

    /// <p>Deletes a domain for a user pool.</p>
    async fn delete_user_pool_domain(
        &self,
        input: DeleteUserPoolDomainRequest,
    ) -> Result<DeleteUserPoolDomainResponse, RusotoError<DeleteUserPoolDomainError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.DeleteUserPoolDomain",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteUserPoolDomainResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteUserPoolDomainError::from_response(response))
        }
    }

    /// <p>Gets information about a specific identity provider.</p>
    async fn describe_identity_provider(
        &self,
        input: DescribeIdentityProviderRequest,
    ) -> Result<DescribeIdentityProviderResponse, RusotoError<DescribeIdentityProviderError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.DescribeIdentityProvider",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeIdentityProviderResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeIdentityProviderError::from_response(response))
        }
    }

    /// <p>Describes a resource server.</p>
    async fn describe_resource_server(
        &self,
        input: DescribeResourceServerRequest,
    ) -> Result<DescribeResourceServerResponse, RusotoError<DescribeResourceServerError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.DescribeResourceServer",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeResourceServerResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeResourceServerError::from_response(response))
        }
    }

    /// <p>Describes the risk configuration.</p>
    async fn describe_risk_configuration(
        &self,
        input: DescribeRiskConfigurationRequest,
    ) -> Result<DescribeRiskConfigurationResponse, RusotoError<DescribeRiskConfigurationError>>
    {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.DescribeRiskConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeRiskConfigurationResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeRiskConfigurationError::from_response(response))
        }
    }

    /// <p>Describes the user import job.</p>
    async fn describe_user_import_job(
        &self,
        input: DescribeUserImportJobRequest,
    ) -> Result<DescribeUserImportJobResponse, RusotoError<DescribeUserImportJobError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.DescribeUserImportJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeUserImportJobResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeUserImportJobError::from_response(response))
        }
    }

    /// <p>Returns the configuration information and metadata of the specified user pool.</p>
    async fn describe_user_pool(
        &self,
        input: DescribeUserPoolRequest,
    ) -> Result<DescribeUserPoolResponse, RusotoError<DescribeUserPoolError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.DescribeUserPool",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeUserPoolResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeUserPoolError::from_response(response))
        }
    }

    /// <p>Client method for returning the configuration information and metadata of the specified user pool app client.</p>
    async fn describe_user_pool_client(
        &self,
        input: DescribeUserPoolClientRequest,
    ) -> Result<DescribeUserPoolClientResponse, RusotoError<DescribeUserPoolClientError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.DescribeUserPoolClient",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeUserPoolClientResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeUserPoolClientError::from_response(response))
        }
    }

    /// <p>Gets information about a domain.</p>
    async fn describe_user_pool_domain(
        &self,
        input: DescribeUserPoolDomainRequest,
    ) -> Result<DescribeUserPoolDomainResponse, RusotoError<DescribeUserPoolDomainError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.DescribeUserPoolDomain",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeUserPoolDomainResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeUserPoolDomainError::from_response(response))
        }
    }

    /// <p>Forgets the specified device.</p>
    async fn forget_device(
        &self,
        input: ForgetDeviceRequest,
    ) -> Result<(), RusotoError<ForgetDeviceError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.ForgetDevice",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ForgetDeviceError::from_response(response))
        }
    }

    /// <p>Calling this API causes a message to be sent to the end user with a confirmation code that is required to change the user's password. For the <code>Username</code> parameter, you can use the username or user alias. If a verified phone number exists for the user, the confirmation code is sent to the phone number. Otherwise, if a verified email exists, the confirmation code is sent to the email. If neither a verified phone number nor a verified email exists, <code>InvalidParameterException</code> is thrown. To use the confirmation code for resetting the password, call .</p>
    async fn forgot_password(
        &self,
        input: ForgotPasswordRequest,
    ) -> Result<ForgotPasswordResponse, RusotoError<ForgotPasswordError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.ForgotPassword",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ForgotPasswordResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ForgotPasswordError::from_response(response))
        }
    }

    /// <p>Gets the header information for the .csv file to be used as input for the user import job.</p>
    async fn get_csv_header(
        &self,
        input: GetCSVHeaderRequest,
    ) -> Result<GetCSVHeaderResponse, RusotoError<GetCSVHeaderError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.GetCSVHeader",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetCSVHeaderResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetCSVHeaderError::from_response(response))
        }
    }

    /// <p>Gets the device.</p>
    async fn get_device(
        &self,
        input: GetDeviceRequest,
    ) -> Result<GetDeviceResponse, RusotoError<GetDeviceError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.GetDevice",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetDeviceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetDeviceError::from_response(response))
        }
    }

    /// <p>Gets a group.</p> <p>Calling this action requires developer credentials.</p>
    async fn get_group(
        &self,
        input: GetGroupRequest,
    ) -> Result<GetGroupResponse, RusotoError<GetGroupError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSCognitoIdentityProviderService.GetGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetGroupResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetGroupError::from_response(response))
        }
    }

    /// <p>Gets the specified identity provider.</p>
    async fn get_identity_provider_by_identifier(
        &self,
        input: GetIdentityProviderByIdentifierRequest,
    ) -> Result<
        GetIdentityProviderByIdentifierResponse,
        RusotoError<GetIdentityProviderByIdentifierError>,
    > {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.GetIdentityProviderByIdentifier",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetIdentityProviderByIdentifierResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetIdentityProviderByIdentifierError::from_response(
                response,
            ))
        }
    }

    /// <p>This method takes a user pool ID, and returns the signing certificate.</p>
    async fn get_signing_certificate(
        &self,
        input: GetSigningCertificateRequest,
    ) -> Result<GetSigningCertificateResponse, RusotoError<GetSigningCertificateError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.GetSigningCertificate",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetSigningCertificateResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetSigningCertificateError::from_response(response))
        }
    }

    /// <p>Gets the UI Customization information for a particular app client's app UI, if there is something set. If nothing is set for the particular client, but there is an existing pool level customization (app <code>clientId</code> will be <code>ALL</code>), then that is returned. If nothing is present, then an empty shape is returned.</p>
    async fn get_ui_customization(
        &self,
        input: GetUICustomizationRequest,
    ) -> Result<GetUICustomizationResponse, RusotoError<GetUICustomizationError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.GetUICustomization",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetUICustomizationResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetUICustomizationError::from_response(response))
        }
    }

    /// <p>Gets the user attributes and metadata for a user.</p>
    async fn get_user(
        &self,
        input: GetUserRequest,
    ) -> Result<GetUserResponse, RusotoError<GetUserError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSCognitoIdentityProviderService.GetUser");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetUserResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetUserError::from_response(response))
        }
    }

    /// <p>Gets the user attribute verification code for the specified attribute name.</p>
    async fn get_user_attribute_verification_code(
        &self,
        input: GetUserAttributeVerificationCodeRequest,
    ) -> Result<
        GetUserAttributeVerificationCodeResponse,
        RusotoError<GetUserAttributeVerificationCodeError>,
    > {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.GetUserAttributeVerificationCode",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetUserAttributeVerificationCodeResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetUserAttributeVerificationCodeError::from_response(
                response,
            ))
        }
    }

    /// <p>Gets the user pool multi-factor authentication (MFA) configuration.</p>
    async fn get_user_pool_mfa_config(
        &self,
        input: GetUserPoolMfaConfigRequest,
    ) -> Result<GetUserPoolMfaConfigResponse, RusotoError<GetUserPoolMfaConfigError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.GetUserPoolMfaConfig",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetUserPoolMfaConfigResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetUserPoolMfaConfigError::from_response(response))
        }
    }

    /// <p>Signs out users from all devices. It also invalidates all refresh tokens issued to a user. The user's current access and Id tokens remain valid until their expiry. Access and Id tokens expire one hour after they are issued.</p>
    async fn global_sign_out(
        &self,
        input: GlobalSignOutRequest,
    ) -> Result<GlobalSignOutResponse, RusotoError<GlobalSignOutError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.GlobalSignOut",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GlobalSignOutResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GlobalSignOutError::from_response(response))
        }
    }

    /// <p>Initiates the authentication flow.</p>
    async fn initiate_auth(
        &self,
        input: InitiateAuthRequest,
    ) -> Result<InitiateAuthResponse, RusotoError<InitiateAuthError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.InitiateAuth",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<InitiateAuthResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(InitiateAuthError::from_response(response))
        }
    }

    /// <p>Lists the devices.</p>
    async fn list_devices(
        &self,
        input: ListDevicesRequest,
    ) -> Result<ListDevicesResponse, RusotoError<ListDevicesError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.ListDevices",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListDevicesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListDevicesError::from_response(response))
        }
    }

    /// <p>Lists the groups associated with a user pool.</p> <p>Calling this action requires developer credentials.</p>
    async fn list_groups(
        &self,
        input: ListGroupsRequest,
    ) -> Result<ListGroupsResponse, RusotoError<ListGroupsError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.ListGroups",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListGroupsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListGroupsError::from_response(response))
        }
    }

    /// <p>Lists information about all identity providers for a user pool.</p>
    async fn list_identity_providers(
        &self,
        input: ListIdentityProvidersRequest,
    ) -> Result<ListIdentityProvidersResponse, RusotoError<ListIdentityProvidersError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.ListIdentityProviders",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ListIdentityProvidersResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListIdentityProvidersError::from_response(response))
        }
    }

    /// <p>Lists the resource servers for a user pool.</p>
    async fn list_resource_servers(
        &self,
        input: ListResourceServersRequest,
    ) -> Result<ListResourceServersResponse, RusotoError<ListResourceServersError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.ListResourceServers",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ListResourceServersResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListResourceServersError::from_response(response))
        }
    }

    /// <p>Lists the tags that are assigned to an Amazon Cognito user pool.</p> <p>A tag is a label that you can apply to user pools to categorize and manage them in different ways, such as by purpose, owner, environment, or other criteria.</p> <p>You can use this action up to 10 times per second, per account.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.ListTagsForResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTagsForResourceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p>Lists the user import jobs.</p>
    async fn list_user_import_jobs(
        &self,
        input: ListUserImportJobsRequest,
    ) -> Result<ListUserImportJobsResponse, RusotoError<ListUserImportJobsError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.ListUserImportJobs",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ListUserImportJobsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListUserImportJobsError::from_response(response))
        }
    }

    /// <p>Lists the clients that have been created for the specified user pool.</p>
    async fn list_user_pool_clients(
        &self,
        input: ListUserPoolClientsRequest,
    ) -> Result<ListUserPoolClientsResponse, RusotoError<ListUserPoolClientsError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.ListUserPoolClients",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ListUserPoolClientsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListUserPoolClientsError::from_response(response))
        }
    }

    /// <p>Lists the user pools associated with an AWS account.</p>
    async fn list_user_pools(
        &self,
        input: ListUserPoolsRequest,
    ) -> Result<ListUserPoolsResponse, RusotoError<ListUserPoolsError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.ListUserPools",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListUserPoolsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListUserPoolsError::from_response(response))
        }
    }

    /// <p>Lists the users in the Amazon Cognito user pool.</p>
    async fn list_users(
        &self,
        input: ListUsersRequest,
    ) -> Result<ListUsersResponse, RusotoError<ListUsersError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.ListUsers",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListUsersResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListUsersError::from_response(response))
        }
    }

    /// <p>Lists the users in the specified group.</p> <p>Calling this action requires developer credentials.</p>
    async fn list_users_in_group(
        &self,
        input: ListUsersInGroupRequest,
    ) -> Result<ListUsersInGroupResponse, RusotoError<ListUsersInGroupError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.ListUsersInGroup",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ListUsersInGroupResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListUsersInGroupError::from_response(response))
        }
    }

    /// <p>Resends the confirmation (for confirmation of registration) to a specific user in the user pool.</p>
    async fn resend_confirmation_code(
        &self,
        input: ResendConfirmationCodeRequest,
    ) -> Result<ResendConfirmationCodeResponse, RusotoError<ResendConfirmationCodeError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.ResendConfirmationCode",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ResendConfirmationCodeResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ResendConfirmationCodeError::from_response(response))
        }
    }

    /// <p>Responds to the authentication challenge.</p>
    async fn respond_to_auth_challenge(
        &self,
        input: RespondToAuthChallengeRequest,
    ) -> Result<RespondToAuthChallengeResponse, RusotoError<RespondToAuthChallengeError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.RespondToAuthChallenge",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<RespondToAuthChallengeResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(RespondToAuthChallengeError::from_response(response))
        }
    }

    /// <p>Configures actions on detected risks. To delete the risk configuration for <code>UserPoolId</code> or <code>ClientId</code>, pass null values for all four configuration types.</p> <p>To enable Amazon Cognito advanced security features, update the user pool to include the <code>UserPoolAddOns</code> key<code>AdvancedSecurityMode</code>.</p> <p>See .</p>
    async fn set_risk_configuration(
        &self,
        input: SetRiskConfigurationRequest,
    ) -> Result<SetRiskConfigurationResponse, RusotoError<SetRiskConfigurationError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.SetRiskConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<SetRiskConfigurationResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(SetRiskConfigurationError::from_response(response))
        }
    }

    /// <p><p>Sets the UI customization information for a user pool&#39;s built-in app UI.</p> <p>You can specify app UI customization settings for a single client (with a specific <code>clientId</code>) or for all clients (by setting the <code>clientId</code> to <code>ALL</code>). If you specify <code>ALL</code>, the default configuration will be used for every client that has no UI customization set previously. If you specify UI customization settings for a particular client, it will no longer fall back to the <code>ALL</code> configuration. </p> <note> <p>To use this API, your user pool must have a domain associated with it. Otherwise, there is no place to host the app&#39;s pages, and the service will throw an error.</p> </note></p>
    async fn set_ui_customization(
        &self,
        input: SetUICustomizationRequest,
    ) -> Result<SetUICustomizationResponse, RusotoError<SetUICustomizationError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.SetUICustomization",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<SetUICustomizationResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(SetUICustomizationError::from_response(response))
        }
    }

    /// <p>Set the user's multi-factor authentication (MFA) method preference, including which MFA factors are enabled and if any are preferred. Only one factor can be set as preferred. The preferred MFA factor will be used to authenticate a user if multiple factors are enabled. If multiple options are enabled and no preference is set, a challenge to choose an MFA option will be returned during sign in.</p>
    async fn set_user_mfa_preference(
        &self,
        input: SetUserMFAPreferenceRequest,
    ) -> Result<SetUserMFAPreferenceResponse, RusotoError<SetUserMFAPreferenceError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.SetUserMFAPreference",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<SetUserMFAPreferenceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(SetUserMFAPreferenceError::from_response(response))
        }
    }

    /// <p>Set the user pool multi-factor authentication (MFA) configuration.</p>
    async fn set_user_pool_mfa_config(
        &self,
        input: SetUserPoolMfaConfigRequest,
    ) -> Result<SetUserPoolMfaConfigResponse, RusotoError<SetUserPoolMfaConfigError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.SetUserPoolMfaConfig",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<SetUserPoolMfaConfigResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(SetUserPoolMfaConfigError::from_response(response))
        }
    }

    /// <p> <i>This action is no longer supported.</i> You can use it to configure only SMS MFA. You can't use it to configure TOTP software token MFA. To configure either type of MFA, use the <a>SetUserMFAPreference</a> action instead.</p>
    async fn set_user_settings(
        &self,
        input: SetUserSettingsRequest,
    ) -> Result<SetUserSettingsResponse, RusotoError<SetUserSettingsError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.SetUserSettings",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<SetUserSettingsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(SetUserSettingsError::from_response(response))
        }
    }

    /// <p>Registers the user in the specified user pool and creates a user name, password, and user attributes.</p>
    async fn sign_up(
        &self,
        input: SignUpRequest,
    ) -> Result<SignUpResponse, RusotoError<SignUpError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSCognitoIdentityProviderService.SignUp");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<SignUpResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(SignUpError::from_response(response))
        }
    }

    /// <p>Starts the user import.</p>
    async fn start_user_import_job(
        &self,
        input: StartUserImportJobRequest,
    ) -> Result<StartUserImportJobResponse, RusotoError<StartUserImportJobError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.StartUserImportJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<StartUserImportJobResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StartUserImportJobError::from_response(response))
        }
    }

    /// <p>Stops the user import job.</p>
    async fn stop_user_import_job(
        &self,
        input: StopUserImportJobRequest,
    ) -> Result<StopUserImportJobResponse, RusotoError<StopUserImportJobError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.StopUserImportJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<StopUserImportJobResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StopUserImportJobError::from_response(response))
        }
    }

    /// <p>Assigns a set of tags to an Amazon Cognito user pool. A tag is a label that you can use to categorize and manage user pools in different ways, such as by purpose, owner, environment, or other criteria.</p> <p>Each tag consists of a key and value, both of which you define. A key is a general category for more specific values. For example, if you have two versions of a user pool, one for testing and another for production, you might assign an <code>Environment</code> tag key to both user pools. The value of this key might be <code>Test</code> for one user pool and <code>Production</code> for the other.</p> <p>Tags are useful for cost tracking and access control. You can activate your tags so that they appear on the Billing and Cost Management console, where you can track the costs associated with your user pools. In an IAM policy, you can constrain permissions for user pools based on specific tags or tag values.</p> <p>You can use this action up to 5 times per second, per account. A user pool can have as many as 50 tags.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.TagResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<TagResourceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Removes the specified tags from an Amazon Cognito user pool. You can use this action up to 5 times per second, per account</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.UntagResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UntagResourceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p>Provides the feedback for an authentication event whether it was from a valid user or not. This feedback is used for improving the risk evaluation decision for the user pool as part of Amazon Cognito advanced security.</p>
    async fn update_auth_event_feedback(
        &self,
        input: UpdateAuthEventFeedbackRequest,
    ) -> Result<UpdateAuthEventFeedbackResponse, RusotoError<UpdateAuthEventFeedbackError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.UpdateAuthEventFeedback",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateAuthEventFeedbackResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateAuthEventFeedbackError::from_response(response))
        }
    }

    /// <p>Updates the device status.</p>
    async fn update_device_status(
        &self,
        input: UpdateDeviceStatusRequest,
    ) -> Result<UpdateDeviceStatusResponse, RusotoError<UpdateDeviceStatusError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.UpdateDeviceStatus",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateDeviceStatusResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateDeviceStatusError::from_response(response))
        }
    }

    /// <p><p>Updates the specified group with the specified attributes.</p> <p>Calling this action requires developer credentials.</p> <important> <p>If you don&#39;t provide a value for an attribute, it will be set to the default value.</p> </important></p>
    async fn update_group(
        &self,
        input: UpdateGroupRequest,
    ) -> Result<UpdateGroupResponse, RusotoError<UpdateGroupError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.UpdateGroup",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UpdateGroupResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateGroupError::from_response(response))
        }
    }

    /// <p>Updates identity provider information for a user pool.</p>
    async fn update_identity_provider(
        &self,
        input: UpdateIdentityProviderRequest,
    ) -> Result<UpdateIdentityProviderResponse, RusotoError<UpdateIdentityProviderError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.UpdateIdentityProvider",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateIdentityProviderResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateIdentityProviderError::from_response(response))
        }
    }

    /// <p><p>Updates the name and scopes of resource server. All other fields are read-only.</p> <important> <p>If you don&#39;t provide a value for an attribute, it will be set to the default value.</p> </important></p>
    async fn update_resource_server(
        &self,
        input: UpdateResourceServerRequest,
    ) -> Result<UpdateResourceServerResponse, RusotoError<UpdateResourceServerError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.UpdateResourceServer",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateResourceServerResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateResourceServerError::from_response(response))
        }
    }

    /// <p>Allows a user to update a specific attribute (one at a time).</p>
    async fn update_user_attributes(
        &self,
        input: UpdateUserAttributesRequest,
    ) -> Result<UpdateUserAttributesResponse, RusotoError<UpdateUserAttributesError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.UpdateUserAttributes",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateUserAttributesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateUserAttributesError::from_response(response))
        }
    }

    /// <p><p>Updates the specified user pool with the specified attributes. You can get a list of the current user pool settings with .</p> <important> <p>If you don&#39;t provide a value for an attribute, it will be set to the default value.</p> </important></p>
    async fn update_user_pool(
        &self,
        input: UpdateUserPoolRequest,
    ) -> Result<UpdateUserPoolResponse, RusotoError<UpdateUserPoolError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.UpdateUserPool",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UpdateUserPoolResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateUserPoolError::from_response(response))
        }
    }

    /// <p><p>Updates the specified user pool app client with the specified attributes. You can get a list of the current user pool app client settings with .</p> <important> <p>If you don&#39;t provide a value for an attribute, it will be set to the default value.</p> </important></p>
    async fn update_user_pool_client(
        &self,
        input: UpdateUserPoolClientRequest,
    ) -> Result<UpdateUserPoolClientResponse, RusotoError<UpdateUserPoolClientError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.UpdateUserPoolClient",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateUserPoolClientResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateUserPoolClientError::from_response(response))
        }
    }

    /// <p>Updates the Secure Sockets Layer (SSL) certificate for the custom domain for your user pool.</p> <p>You can use this operation to provide the Amazon Resource Name (ARN) of a new certificate to Amazon Cognito. You cannot use it to change the domain for a user pool.</p> <p>A custom domain is used to host the Amazon Cognito hosted UI, which provides sign-up and sign-in pages for your application. When you set up a custom domain, you provide a certificate that you manage with AWS Certificate Manager (ACM). When necessary, you can use this operation to change the certificate that you applied to your custom domain.</p> <p>Usually, this is unnecessary following routine certificate renewal with ACM. When you renew your existing certificate in ACM, the ARN for your certificate remains the same, and your custom domain uses the new certificate automatically.</p> <p>However, if you replace your existing certificate with a new one, ACM gives the new certificate a new ARN. To apply the new certificate to your custom domain, you must provide this ARN to Amazon Cognito.</p> <p>When you add your new certificate in ACM, you must choose US East (N. Virginia) as the AWS Region.</p> <p>After you submit your request, Amazon Cognito requires up to 1 hour to distribute your new certificate to your custom domain.</p> <p>For more information about adding a custom domain to your user pool, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-add-custom-domain.html">Using Your Own Domain for the Hosted UI</a>.</p>
    async fn update_user_pool_domain(
        &self,
        input: UpdateUserPoolDomainRequest,
    ) -> Result<UpdateUserPoolDomainResponse, RusotoError<UpdateUserPoolDomainError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.UpdateUserPoolDomain",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateUserPoolDomainResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateUserPoolDomainError::from_response(response))
        }
    }

    /// <p>Use this API to register a user's entered TOTP code and mark the user's software token MFA status as "verified" if successful. The request takes an access token or a session string, but not both.</p>
    async fn verify_software_token(
        &self,
        input: VerifySoftwareTokenRequest,
    ) -> Result<VerifySoftwareTokenResponse, RusotoError<VerifySoftwareTokenError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.VerifySoftwareToken",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<VerifySoftwareTokenResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(VerifySoftwareTokenError::from_response(response))
        }
    }

    /// <p>Verifies the specified user attributes in the user pool.</p>
    async fn verify_user_attribute(
        &self,
        input: VerifyUserAttributeRequest,
    ) -> Result<VerifyUserAttributeResponse, RusotoError<VerifyUserAttributeError>> {
        let mut request = SignedRequest::new("POST", "cognito-idp", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.VerifyUserAttribute",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<VerifyUserAttributeResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(VerifyUserAttributeError::from_response(response))
        }
    }
}
