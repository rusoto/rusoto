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
use rusoto_core::v2::{Dispatcher, Request, ServiceRequest};
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
pub struct AssociatePhoneNumberWithUserRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The phone number, in E.164 format.</p>
    #[serde(rename = "E164PhoneNumber")]
    pub e164_phone_number: String,
    /// <p>The user ID.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AssociatePhoneNumberWithUserResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssociatePhoneNumbersWithVoiceConnectorRequest {
    /// <p>List of phone numbers, in E.164 format.</p>
    #[serde(rename = "E164PhoneNumbers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e164_phone_numbers: Option<Vec<String>>,
    /// <p>The Amazon Chime Voice Connector ID.</p>
    #[serde(rename = "VoiceConnectorId")]
    pub voice_connector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AssociatePhoneNumbersWithVoiceConnectorResponse {
    /// <p>If the action fails for one or more of the phone numbers in the request, a list of the phone numbers is returned, along with error codes and error messages.</p>
    #[serde(rename = "PhoneNumberErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_errors: Option<Vec<PhoneNumberError>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchDeletePhoneNumberRequest {
    /// <p>List of phone number IDs.</p>
    #[serde(rename = "PhoneNumberIds")]
    pub phone_number_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct BatchDeletePhoneNumberResponse {
    /// <p>If the action fails for one or more of the phone numbers in the request, a list of the phone numbers is returned, along with error codes and error messages.</p>
    #[serde(rename = "PhoneNumberErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_errors: Option<Vec<PhoneNumberError>>,
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
pub struct BatchUpdatePhoneNumberRequest {
    /// <p>The request containing the phone number IDs and product types to update.</p>
    #[serde(rename = "UpdatePhoneNumberRequestItems")]
    pub update_phone_number_request_items: Vec<UpdatePhoneNumberRequestItem>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct BatchUpdatePhoneNumberResponse {
    /// <p>If the action fails for one or more of the phone numbers in the request, a list of the phone numbers is returned, along with error codes and error messages.</p>
    #[serde(rename = "PhoneNumberErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_errors: Option<Vec<PhoneNumberError>>,
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

/// <p>A resource that allows Enterprise account administrators to configure an interface to receive events from Amazon Chime.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Bot {
    /// <p>The bot email address.</p>
    #[serde(rename = "BotEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_email: Option<String>,
    /// <p>The bot ID.</p>
    #[serde(rename = "BotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    /// <p>The bot type.</p>
    #[serde(rename = "BotType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_type: Option<String>,
    /// <p>The bot creation timestamp, in ISO 8601 format.</p>
    #[serde(rename = "CreatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    /// <p>When true, the bot is stopped from running in your account.</p>
    #[serde(rename = "Disabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    /// <p>The bot display name.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The security token used to authenticate Amazon Chime with the outgoing event endpoint.</p>
    #[serde(rename = "SecurityToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_token: Option<String>,
    /// <p>The updated bot timestamp, in ISO 8601 format.</p>
    #[serde(rename = "UpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<f64>,
    /// <p>The unique ID for the bot user.</p>
    #[serde(rename = "UserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// <p>The Amazon Chime Business Calling settings for the administrator's AWS account. Includes any Amazon S3 buckets designated for storing call detail records.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BusinessCallingSettings {
    /// <p>The Amazon S3 bucket designated for call detail record storage.</p>
    #[serde(rename = "CdrBucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdr_bucket: Option<String>,
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
pub struct CreateBotRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The bot display name.</p>
    #[serde(rename = "DisplayName")]
    pub display_name: String,
    /// <p>The domain of the Amazon Chime Enterprise account.</p>
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateBotResponse {
    /// <p>The bot details.</p>
    #[serde(rename = "Bot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot: Option<Bot>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreatePhoneNumberOrderRequest {
    /// <p>List of phone numbers, in E.164 format.</p>
    #[serde(rename = "E164PhoneNumbers")]
    pub e164_phone_numbers: Vec<String>,
    /// <p>The phone number product type.</p>
    #[serde(rename = "ProductType")]
    pub product_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreatePhoneNumberOrderResponse {
    /// <p>The phone number order details.</p>
    #[serde(rename = "PhoneNumberOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_order: Option<PhoneNumberOrder>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateVoiceConnectorRequest {
    /// <p>The name of the Amazon Chime Voice Connector.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>When enabled, requires encryption for the Amazon Chime Voice Connector.</p>
    #[serde(rename = "RequireEncryption")]
    pub require_encryption: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateVoiceConnectorResponse {
    /// <p>The Amazon Chime Voice Connector details.</p>
    #[serde(rename = "VoiceConnector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_connector: Option<VoiceConnector>,
}

/// <p>The SIP credentials used to authenticate requests to your Amazon Chime Voice Connector.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Credential {
    /// <p>The RFC2617 compliant password associated with the SIP credentials, in US-ASCII format.</p>
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// <p>The RFC2617 compliant user name associated with the SIP credentials, in US-ASCII format.</p>
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
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
pub struct DeleteEventsConfigurationRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The bot ID.</p>
    #[serde(rename = "BotId")]
    pub bot_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteEventsConfigurationResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeletePhoneNumberRequest {
    /// <p>The phone number ID.</p>
    #[serde(rename = "PhoneNumberId")]
    pub phone_number_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeletePhoneNumberResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteVoiceConnectorOriginationRequest {
    /// <p>The Amazon Chime Voice Connector ID.</p>
    #[serde(rename = "VoiceConnectorId")]
    pub voice_connector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteVoiceConnectorOriginationResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteVoiceConnectorRequest {
    /// <p>The Amazon Chime Voice Connector ID.</p>
    #[serde(rename = "VoiceConnectorId")]
    pub voice_connector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteVoiceConnectorResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteVoiceConnectorTerminationCredentialsRequest {
    /// <p>The RFC2617 compliant username associated with the SIP credentials, in US-ASCII format.</p>
    #[serde(rename = "Usernames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usernames: Option<Vec<String>>,
    /// <p>The Amazon Chime Voice Connector ID.</p>
    #[serde(rename = "VoiceConnectorId")]
    pub voice_connector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteVoiceConnectorTerminationCredentialsResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteVoiceConnectorTerminationRequest {
    /// <p>The Amazon Chime Voice Connector ID.</p>
    #[serde(rename = "VoiceConnectorId")]
    pub voice_connector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteVoiceConnectorTerminationResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisassociatePhoneNumberFromUserRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The user ID.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DisassociatePhoneNumberFromUserResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisassociatePhoneNumbersFromVoiceConnectorRequest {
    /// <p>List of phone numbers, in E.164 format.</p>
    #[serde(rename = "E164PhoneNumbers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e164_phone_numbers: Option<Vec<String>>,
    /// <p>The Amazon Chime Voice Connector ID.</p>
    #[serde(rename = "VoiceConnectorId")]
    pub voice_connector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DisassociatePhoneNumbersFromVoiceConnectorResponse {
    /// <p>If the action fails for one or more of the phone numbers in the request, a list of the phone numbers is returned, along with error codes and error messages.</p>
    #[serde(rename = "PhoneNumberErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_errors: Option<Vec<PhoneNumberError>>,
}

/// <p>The configuration that allows a bot to receive outgoing events. Can be either an HTTPS endpoint or a Lambda function ARN.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct EventsConfiguration {
    /// <p>The bot ID.</p>
    #[serde(rename = "BotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    /// <p>Lambda function ARN that allows a bot to receive outgoing events.</p>
    #[serde(rename = "LambdaFunctionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_arn: Option<String>,
    /// <p>HTTPS endpoint that allows a bot to receive outgoing events.</p>
    #[serde(rename = "OutboundEventsHTTPSEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_events_https_endpoint: Option<String>,
}

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
pub struct GetBotRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The bot ID.</p>
    #[serde(rename = "BotId")]
    pub bot_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetBotResponse {
    /// <p>The chat bot details.</p>
    #[serde(rename = "Bot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot: Option<Bot>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetEventsConfigurationRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The bot ID.</p>
    #[serde(rename = "BotId")]
    pub bot_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetEventsConfigurationResponse {
    /// <p>The events configuration details.</p>
    #[serde(rename = "EventsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events_configuration: Option<EventsConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetGlobalSettingsRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetGlobalSettingsResponse {
    /// <p>The Amazon Chime Business Calling settings.</p>
    #[serde(rename = "BusinessCalling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_calling: Option<BusinessCallingSettings>,
    /// <p>The Amazon Chime Voice Connector settings.</p>
    #[serde(rename = "VoiceConnector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_connector: Option<VoiceConnectorSettings>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetPhoneNumberOrderRequest {
    /// <p>The ID for the phone number order.</p>
    #[serde(rename = "PhoneNumberOrderId")]
    pub phone_number_order_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetPhoneNumberOrderResponse {
    /// <p>The phone number order details.</p>
    #[serde(rename = "PhoneNumberOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_order: Option<PhoneNumberOrder>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetPhoneNumberRequest {
    /// <p>The phone number ID.</p>
    #[serde(rename = "PhoneNumberId")]
    pub phone_number_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetPhoneNumberResponse {
    /// <p>The phone number details.</p>
    #[serde(rename = "PhoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<PhoneNumber>,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetUserSettingsRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The user ID.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetUserSettingsResponse {
    /// <p>The user settings.</p>
    #[serde(rename = "UserSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_settings: Option<UserSettings>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetVoiceConnectorOriginationRequest {
    /// <p>The Amazon Chime Voice Connector ID.</p>
    #[serde(rename = "VoiceConnectorId")]
    pub voice_connector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetVoiceConnectorOriginationResponse {
    /// <p>The origination setting details.</p>
    #[serde(rename = "Origination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origination: Option<Origination>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetVoiceConnectorRequest {
    /// <p>The Amazon Chime Voice Connector ID.</p>
    #[serde(rename = "VoiceConnectorId")]
    pub voice_connector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetVoiceConnectorResponse {
    /// <p>The Amazon Chime Voice Connector details.</p>
    #[serde(rename = "VoiceConnector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_connector: Option<VoiceConnector>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetVoiceConnectorTerminationHealthRequest {
    /// <p>The Amazon Chime Voice Connector ID.</p>
    #[serde(rename = "VoiceConnectorId")]
    pub voice_connector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetVoiceConnectorTerminationHealthResponse {
    /// <p>The termination health details.</p>
    #[serde(rename = "TerminationHealth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_health: Option<TerminationHealth>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetVoiceConnectorTerminationRequest {
    /// <p>The Amazon Chime Voice Connector ID.</p>
    #[serde(rename = "VoiceConnectorId")]
    pub voice_connector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetVoiceConnectorTerminationResponse {
    /// <p>The termination setting details.</p>
    #[serde(rename = "Termination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination: Option<Termination>,
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
pub struct ListBotsRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The maximum number of results to return in a single call. Default is 10.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to use to retrieve the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListBotsResponse {
    /// <p>List of bots and bot details.</p>
    #[serde(rename = "Bots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bots: Option<Vec<Bot>>,
    /// <p>The token to use to retrieve the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListPhoneNumberOrdersRequest {
    /// <p>The maximum number of results to return in a single call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to use to retrieve the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListPhoneNumberOrdersResponse {
    /// <p>The token to use to retrieve the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The phone number order details.</p>
    #[serde(rename = "PhoneNumberOrders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_orders: Option<Vec<PhoneNumberOrder>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListPhoneNumbersRequest {
    /// <p>The filter to use to limit the number of results.</p>
    #[serde(rename = "FilterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_name: Option<String>,
    /// <p>The value to use for the filter.</p>
    #[serde(rename = "FilterValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_value: Option<String>,
    /// <p>The maximum number of results to return in a single call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to use to retrieve the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The phone number product type.</p>
    #[serde(rename = "ProductType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<String>,
    /// <p>The phone number status.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListPhoneNumbersResponse {
    /// <p>The token to use to retrieve the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The phone number details.</p>
    #[serde(rename = "PhoneNumbers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_numbers: Option<Vec<PhoneNumber>>,
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
pub struct ListVoiceConnectorTerminationCredentialsRequest {
    /// <p>The Amazon Chime Voice Connector ID.</p>
    #[serde(rename = "VoiceConnectorId")]
    pub voice_connector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListVoiceConnectorTerminationCredentialsResponse {
    /// <p>A list of user names.</p>
    #[serde(rename = "Usernames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usernames: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListVoiceConnectorsRequest {
    /// <p>The maximum number of results to return in a single call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to use to retrieve the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListVoiceConnectorsResponse {
    /// <p>The token to use to retrieve the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The details of the Amazon Chime Voice Connectors.</p>
    #[serde(rename = "VoiceConnectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_connectors: Option<Vec<VoiceConnector>>,
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

/// <p>A phone number for which an order has been placed.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct OrderedPhoneNumber {
    /// <p>The phone number, in E.164 format.</p>
    #[serde(rename = "E164PhoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e164_phone_number: Option<String>,
    /// <p>The phone number status.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Origination settings enable your SIP hosts to receive inbound calls using your Amazon Chime Voice Connector.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Origination {
    /// <p>When origination settings are disabled, inbound calls are not enabled for your Amazon Chime Voice Connector.</p>
    #[serde(rename = "Disabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    /// <p>The call distribution properties defined for your SIP hosts. Valid range: Minimum value of 1. Maximum value of 20.</p>
    #[serde(rename = "Routes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routes: Option<Vec<OriginationRoute>>,
}

/// <p>Origination routes define call distribution properties for your SIP hosts to receive inbound calls using your Amazon Chime Voice Connector. Limit: 10 origination routes per Amazon Chime Voice Connector.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OriginationRoute {
    /// <p>The FODN or IP address to contact for origination traffic.</p>
    #[serde(rename = "Host")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// <p>The designated origination route port. Defaults to 5060.</p>
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// <p>The priority associated with the host, with 1 being the highest priority. Higher priority hosts are attempted first.</p>
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// <p>The protocol to use for the origination route. Encryption-enabled Amazon Chime Voice Connectors use TCP protocol by default.</p>
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// <p>The weight associated with the host. If hosts are equal in priority, calls are distributed among them based on their relative weight.</p>
    #[serde(rename = "Weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
}

/// <p>A phone number used for Amazon Chime Business Calling or an Amazon Chime Voice Connector.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PhoneNumber {
    /// <p>The phone number associations.</p>
    #[serde(rename = "Associations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associations: Option<Vec<PhoneNumberAssociation>>,
    /// <p>The phone number capabilities.</p>
    #[serde(rename = "Capabilities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<PhoneNumberCapabilities>,
    /// <p>The phone number creation timestamp, in ISO 8601 format.</p>
    #[serde(rename = "CreatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    /// <p>The deleted phone number timestamp, in ISO 8601 format.</p>
    #[serde(rename = "DeletionTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_timestamp: Option<f64>,
    /// <p>The phone number, in E.164 format.</p>
    #[serde(rename = "E164PhoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e164_phone_number: Option<String>,
    /// <p>The phone number ID.</p>
    #[serde(rename = "PhoneNumberId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_id: Option<String>,
    /// <p>The phone number product type.</p>
    #[serde(rename = "ProductType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<String>,
    /// <p>The phone number status.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The updated phone number timestamp, in ISO 8601 format.</p>
    #[serde(rename = "UpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<f64>,
}

/// <p>The phone number associations, such as Amazon Chime account ID, Amazon Chime user ID, or Amazon Chime Voice Connector ID.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PhoneNumberAssociation {
    /// <p>The timestamp of the phone number association, in ISO 8601 format.</p>
    #[serde(rename = "AssociatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_timestamp: Option<f64>,
    /// <p>Defines the association with an Amazon Chime account ID, user ID, or Amazon Chime Voice Connector ID.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Contains the ID for the entity specified in Name.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>The phone number capabilities, such as enabled inbound and outbound calling and text messaging.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PhoneNumberCapabilities {
    /// <p>Allows or denies inbound calling for the specified phone number.</p>
    #[serde(rename = "InboundCall")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_call: Option<bool>,
    /// <p>Allows or denies inbound MMS messaging for the specified phone number.</p>
    #[serde(rename = "InboundMMS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_mms: Option<bool>,
    /// <p>Allows or denies inbound SMS messaging for the specified phone number.</p>
    #[serde(rename = "InboundSMS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_sms: Option<bool>,
    /// <p>Allows or denies outbound calling for the specified phone number.</p>
    #[serde(rename = "OutboundCall")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_call: Option<bool>,
    /// <p>Allows or denies outbound MMS messaging for the specified phone number.</p>
    #[serde(rename = "OutboundMMS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_mms: Option<bool>,
    /// <p>Allows or denies outbound SMS messaging for the specified phone number.</p>
    #[serde(rename = "OutboundSMS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_sms: Option<bool>,
}

/// <p>If the phone number action fails for one or more of the phone numbers in the request, a list of the phone numbers is returned, along with error codes and error messages.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PhoneNumberError {
    /// <p>The error code.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The error message.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The phone number ID for which the action failed.</p>
    #[serde(rename = "PhoneNumberId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_id: Option<String>,
}

/// <p>The details of a phone number order created for Amazon Chime.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PhoneNumberOrder {
    /// <p>The phone number order creation timestamp, in ISO 8601 format.</p>
    #[serde(rename = "CreatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    /// <p>The ordered phone number details, such as the phone number in E.164 format and the phone number status.</p>
    #[serde(rename = "OrderedPhoneNumbers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordered_phone_numbers: Option<Vec<OrderedPhoneNumber>>,
    /// <p>The phone number order ID.</p>
    #[serde(rename = "PhoneNumberOrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_order_id: Option<String>,
    /// <p>The phone number order product type.</p>
    #[serde(rename = "ProductType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<String>,
    /// <p>The status of the phone number order.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The updated phone number order timestamp, in ISO 8601 format.</p>
    #[serde(rename = "UpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutEventsConfigurationRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The bot ID.</p>
    #[serde(rename = "BotId")]
    pub bot_id: String,
    /// <p>Lambda function ARN that allows the bot to receive outgoing events.</p>
    #[serde(rename = "LambdaFunctionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_arn: Option<String>,
    /// <p>HTTPS endpoint that allows the bot to receive outgoing events.</p>
    #[serde(rename = "OutboundEventsHTTPSEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_events_https_endpoint: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PutEventsConfigurationResponse {
    #[serde(rename = "EventsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events_configuration: Option<EventsConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutVoiceConnectorOriginationRequest {
    /// <p>The origination setting details to add.</p>
    #[serde(rename = "Origination")]
    pub origination: Origination,
    /// <p>The Amazon Chime Voice Connector ID.</p>
    #[serde(rename = "VoiceConnectorId")]
    pub voice_connector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PutVoiceConnectorOriginationResponse {
    /// <p>The updated origination setting details.</p>
    #[serde(rename = "Origination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origination: Option<Origination>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutVoiceConnectorTerminationCredentialsRequest {
    /// <p>The termination SIP credentials.</p>
    #[serde(rename = "Credentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Vec<Credential>>,
    /// <p>The Amazon Chime Voice Connector ID.</p>
    #[serde(rename = "VoiceConnectorId")]
    pub voice_connector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PutVoiceConnectorTerminationCredentialsResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutVoiceConnectorTerminationRequest {
    /// <p>The termination setting details to add.</p>
    #[serde(rename = "Termination")]
    pub termination: Termination,
    /// <p>The Amazon Chime Voice Connector ID.</p>
    #[serde(rename = "VoiceConnectorId")]
    pub voice_connector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PutVoiceConnectorTerminationResponse {
    /// <p>The updated termination setting details.</p>
    #[serde(rename = "Termination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination: Option<Termination>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RegenerateSecurityTokenRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The bot ID.</p>
    #[serde(rename = "BotId")]
    pub bot_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RegenerateSecurityTokenResponse {
    #[serde(rename = "Bot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot: Option<Bot>,
}

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
pub struct RestorePhoneNumberRequest {
    /// <p>The phone number.</p>
    #[serde(rename = "PhoneNumberId")]
    pub phone_number_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RestorePhoneNumberResponse {
    /// <p>The phone number details.</p>
    #[serde(rename = "PhoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<PhoneNumber>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SearchAvailablePhoneNumbersRequest {
    /// <p>The area code used to filter results.</p>
    #[serde(rename = "AreaCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area_code: Option<String>,
    /// <p>The city used to filter results.</p>
    #[serde(rename = "City")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// <p>The country used to filter results.</p>
    #[serde(rename = "Country")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// <p>The maximum number of results to return in a single call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to use to retrieve the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The state used to filter results.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SearchAvailablePhoneNumbersResponse {
    /// <p>List of phone numbers, in E.164 format.</p>
    #[serde(rename = "E164PhoneNumbers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e164_phone_numbers: Option<Vec<String>>,
}

/// <p>Settings that allow management of telephony permissions for an Amazon Chime user, such as inbound and outbound calling and text messaging.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TelephonySettings {
    /// <p>Allows or denies inbound calling.</p>
    #[serde(rename = "InboundCalling")]
    pub inbound_calling: bool,
    /// <p>Allows or denies outbound calling.</p>
    #[serde(rename = "OutboundCalling")]
    pub outbound_calling: bool,
    /// <p>Allows or denies SMS messaging.</p>
    #[serde(rename = "SMS")]
    pub sms: bool,
}

/// <p>Termination settings enable your SIP hosts to make outbound calls using your Amazon Chime Voice Connector.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Termination {
    /// <p>The countries to which calls are allowed.</p>
    #[serde(rename = "CallingRegions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calling_regions: Option<Vec<String>>,
    /// <p>The IP addresses allowed to make calls, in CIDR format.</p>
    #[serde(rename = "CidrAllowedList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_allowed_list: Option<Vec<String>>,
    /// <p>The limit on calls per second. Max value based on account service limit. Default value of 1.</p>
    #[serde(rename = "CpsLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cps_limit: Option<i64>,
    /// <p>The default caller ID phone number.</p>
    #[serde(rename = "DefaultPhoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_phone_number: Option<String>,
    /// <p>When termination settings are disabled, outbound calls can not be made.</p>
    #[serde(rename = "Disabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}

/// <p>The termination health details, including the source IP address and timestamp of the last successful SIP <code>OPTIONS</code> message from your SIP infrastructure.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TerminationHealth {
    /// <p>The source IP address.</p>
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// <p>The timestamp, in ISO 8601 format.</p>
    #[serde(rename = "Timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
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
pub struct UpdateBotRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The bot ID.</p>
    #[serde(rename = "BotId")]
    pub bot_id: String,
    /// <p>When true, stops the specified bot from running in your account.</p>
    #[serde(rename = "Disabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateBotResponse {
    /// <p>The updated bot details.</p>
    #[serde(rename = "Bot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot: Option<Bot>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateGlobalSettingsRequest {
    /// <p>The Amazon Chime Business Calling settings.</p>
    #[serde(rename = "BusinessCalling")]
    pub business_calling: BusinessCallingSettings,
    /// <p>The Amazon Chime Voice Connector settings.</p>
    #[serde(rename = "VoiceConnector")]
    pub voice_connector: VoiceConnectorSettings,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateGlobalSettingsResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdatePhoneNumberRequest {
    /// <p>The phone number ID.</p>
    #[serde(rename = "PhoneNumberId")]
    pub phone_number_id: String,
    /// <p>The product type.</p>
    #[serde(rename = "ProductType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<String>,
}

/// <p>The phone number ID and product type fields to update, used with the <a>BatchUpdatePhoneNumber</a> and <a>UpdatePhoneNumber</a> actions.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdatePhoneNumberRequestItem {
    /// <p>The phone number ID to update.</p>
    #[serde(rename = "PhoneNumberId")]
    pub phone_number_id: String,
    /// <p>The product type to update.</p>
    #[serde(rename = "ProductType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdatePhoneNumberResponse {
    /// <p>The updated phone number details.</p>
    #[serde(rename = "PhoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<PhoneNumber>,
}

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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateUserSettingsRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The user ID.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
    /// <p>The user settings to update.</p>
    #[serde(rename = "UserSettings")]
    pub user_settings: UserSettings,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateUserSettingsResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateVoiceConnectorRequest {
    /// <p>The name of the Amazon Chime Voice Connector.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>When enabled, requires encryption for the Amazon Chime Voice Connector.</p>
    #[serde(rename = "RequireEncryption")]
    pub require_encryption: bool,
    /// <p>The Amazon Chime Voice Connector ID.</p>
    #[serde(rename = "VoiceConnectorId")]
    pub voice_connector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateVoiceConnectorResponse {
    /// <p>The Amazon Chime Voice Connector details.</p>
    #[serde(rename = "VoiceConnector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_connector: Option<VoiceConnector>,
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
    /// <p>The primary phone number associated with the user.</p>
    #[serde(rename = "PrimaryProvisionedNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_provisioned_number: Option<String>,
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

/// <p>Settings associated with an Amazon Chime user, including inbound and outbound calling and text messaging.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserSettings {
    /// <p>The telephony settings associated with the user.</p>
    #[serde(rename = "Telephony")]
    pub telephony: TelephonySettings,
}

/// <p>The Amazon Chime Voice Connector configuration, including outbound host name and encryption settings.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct VoiceConnector {
    /// <p>The Amazon Chime Voice Connector creation timestamp, in ISO 8601 format.</p>
    #[serde(rename = "CreatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    /// <p>The name of the Amazon Chime Voice Connector.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The outbound host name for the Amazon Chime Voice Connector.</p>
    #[serde(rename = "OutboundHostName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_host_name: Option<String>,
    /// <p>Designates whether encryption is required for the Amazon Chime Voice Connector.</p>
    #[serde(rename = "RequireEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_encryption: Option<bool>,
    /// <p>The updated Amazon Chime Voice Connector timestamp, in ISO 8601 format.</p>
    #[serde(rename = "UpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<f64>,
    /// <p>The Amazon Chime Voice Connector ID.</p>
    #[serde(rename = "VoiceConnectorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_connector_id: Option<String>,
}

/// <p>The Amazon Chime Voice Connector settings. Includes any Amazon S3 buckets designated for storing call detail records.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceConnectorSettings {
    /// <p>The Amazon S3 bucket designated for call detail record storage.</p>
    #[serde(rename = "CdrBucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdr_bucket: Option<String>,
}

/// Errors returned by AssociatePhoneNumberWithUser
#[derive(Debug, PartialEq)]
pub enum AssociatePhoneNumberWithUserError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
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

impl AssociatePhoneNumberWithUserError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AssociatePhoneNumberWithUserError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(AssociatePhoneNumberWithUserError::BadRequest(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(AssociatePhoneNumberWithUserError::Forbidden(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(AssociatePhoneNumberWithUserError::NotFound(
                        err.msg,
                    ))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(AssociatePhoneNumberWithUserError::ServiceFailure(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        AssociatePhoneNumberWithUserError::ServiceUnavailable(err.msg),
                    )
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(
                        AssociatePhoneNumberWithUserError::ThrottledClient(err.msg),
                    )
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(
                        AssociatePhoneNumberWithUserError::UnauthorizedClient(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AssociatePhoneNumberWithUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociatePhoneNumberWithUserError {
    fn description(&self) -> &str {
        match *self {
            AssociatePhoneNumberWithUserError::BadRequest(ref cause) => cause,
            AssociatePhoneNumberWithUserError::Forbidden(ref cause) => cause,
            AssociatePhoneNumberWithUserError::NotFound(ref cause) => cause,
            AssociatePhoneNumberWithUserError::ServiceFailure(ref cause) => cause,
            AssociatePhoneNumberWithUserError::ServiceUnavailable(ref cause) => cause,
            AssociatePhoneNumberWithUserError::ThrottledClient(ref cause) => cause,
            AssociatePhoneNumberWithUserError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by AssociatePhoneNumbersWithVoiceConnector
#[derive(Debug, PartialEq)]
pub enum AssociatePhoneNumbersWithVoiceConnectorError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
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

impl AssociatePhoneNumbersWithVoiceConnectorError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AssociatePhoneNumbersWithVoiceConnectorError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        AssociatePhoneNumbersWithVoiceConnectorError::BadRequest(err.msg),
                    )
                }
                "ForbiddenException" => {
                    return RusotoError::Service(
                        AssociatePhoneNumbersWithVoiceConnectorError::Forbidden(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(
                        AssociatePhoneNumbersWithVoiceConnectorError::NotFound(err.msg),
                    )
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(
                        AssociatePhoneNumbersWithVoiceConnectorError::ServiceFailure(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        AssociatePhoneNumbersWithVoiceConnectorError::ServiceUnavailable(err.msg),
                    )
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(
                        AssociatePhoneNumbersWithVoiceConnectorError::ThrottledClient(err.msg),
                    )
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(
                        AssociatePhoneNumbersWithVoiceConnectorError::UnauthorizedClient(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AssociatePhoneNumbersWithVoiceConnectorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociatePhoneNumbersWithVoiceConnectorError {
    fn description(&self) -> &str {
        match *self {
            AssociatePhoneNumbersWithVoiceConnectorError::BadRequest(ref cause) => cause,
            AssociatePhoneNumbersWithVoiceConnectorError::Forbidden(ref cause) => cause,
            AssociatePhoneNumbersWithVoiceConnectorError::NotFound(ref cause) => cause,
            AssociatePhoneNumbersWithVoiceConnectorError::ServiceFailure(ref cause) => cause,
            AssociatePhoneNumbersWithVoiceConnectorError::ServiceUnavailable(ref cause) => cause,
            AssociatePhoneNumbersWithVoiceConnectorError::ThrottledClient(ref cause) => cause,
            AssociatePhoneNumbersWithVoiceConnectorError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by BatchDeletePhoneNumber
#[derive(Debug, PartialEq)]
pub enum BatchDeletePhoneNumberError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
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

impl BatchDeletePhoneNumberError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchDeletePhoneNumberError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(BatchDeletePhoneNumberError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(BatchDeletePhoneNumberError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(BatchDeletePhoneNumberError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(BatchDeletePhoneNumberError::ServiceFailure(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(BatchDeletePhoneNumberError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(BatchDeletePhoneNumberError::ThrottledClient(
                        err.msg,
                    ))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(BatchDeletePhoneNumberError::UnauthorizedClient(
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
impl fmt::Display for BatchDeletePhoneNumberError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchDeletePhoneNumberError {
    fn description(&self) -> &str {
        match *self {
            BatchDeletePhoneNumberError::BadRequest(ref cause) => cause,
            BatchDeletePhoneNumberError::Forbidden(ref cause) => cause,
            BatchDeletePhoneNumberError::NotFound(ref cause) => cause,
            BatchDeletePhoneNumberError::ServiceFailure(ref cause) => cause,
            BatchDeletePhoneNumberError::ServiceUnavailable(ref cause) => cause,
            BatchDeletePhoneNumberError::ThrottledClient(ref cause) => cause,
            BatchDeletePhoneNumberError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by BatchSuspendUser
#[derive(Debug, PartialEq)]
pub enum BatchSuspendUserError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
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
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
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
/// Errors returned by BatchUpdatePhoneNumber
#[derive(Debug, PartialEq)]
pub enum BatchUpdatePhoneNumberError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
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

impl BatchUpdatePhoneNumberError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchUpdatePhoneNumberError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(BatchUpdatePhoneNumberError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(BatchUpdatePhoneNumberError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(BatchUpdatePhoneNumberError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(BatchUpdatePhoneNumberError::ServiceFailure(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(BatchUpdatePhoneNumberError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(BatchUpdatePhoneNumberError::ThrottledClient(
                        err.msg,
                    ))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(BatchUpdatePhoneNumberError::UnauthorizedClient(
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
impl fmt::Display for BatchUpdatePhoneNumberError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchUpdatePhoneNumberError {
    fn description(&self) -> &str {
        match *self {
            BatchUpdatePhoneNumberError::BadRequest(ref cause) => cause,
            BatchUpdatePhoneNumberError::Forbidden(ref cause) => cause,
            BatchUpdatePhoneNumberError::NotFound(ref cause) => cause,
            BatchUpdatePhoneNumberError::ServiceFailure(ref cause) => cause,
            BatchUpdatePhoneNumberError::ServiceUnavailable(ref cause) => cause,
            BatchUpdatePhoneNumberError::ThrottledClient(ref cause) => cause,
            BatchUpdatePhoneNumberError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by BatchUpdateUser
#[derive(Debug, PartialEq)]
pub enum BatchUpdateUserError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
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
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
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
/// Errors returned by CreateBot
#[derive(Debug, PartialEq)]
pub enum CreateBotError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
    Forbidden(String),
    /// <p>One or more of the resources in the request does not exist in the system.</p>
    NotFound(String),
    /// <p>The request exceeds the resource limit.</p>
    ResourceLimitExceeded(String),
    /// <p>The service encountered an unexpected error.</p>
    ServiceFailure(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The client is not currently authorized to make the request.</p>
    UnauthorizedClient(String),
}

impl CreateBotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateBotError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateBotError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateBotError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateBotError::NotFound(err.msg))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(CreateBotError::ResourceLimitExceeded(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(CreateBotError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateBotError::ServiceUnavailable(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(CreateBotError::UnauthorizedClient(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateBotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateBotError {
    fn description(&self) -> &str {
        match *self {
            CreateBotError::BadRequest(ref cause) => cause,
            CreateBotError::Forbidden(ref cause) => cause,
            CreateBotError::NotFound(ref cause) => cause,
            CreateBotError::ResourceLimitExceeded(ref cause) => cause,
            CreateBotError::ServiceFailure(ref cause) => cause,
            CreateBotError::ServiceUnavailable(ref cause) => cause,
            CreateBotError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by CreatePhoneNumberOrder
#[derive(Debug, PartialEq)]
pub enum CreatePhoneNumberOrderError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected error.</p>
    ServiceFailure(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The client exceeded its request rate limit.</p>
    ThrottledClient(String),
    /// <p>The client is not currently authorized to make the request.</p>
    UnauthorizedClient(String),
}

impl CreatePhoneNumberOrderError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreatePhoneNumberOrderError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreatePhoneNumberOrderError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreatePhoneNumberOrderError::Forbidden(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(CreatePhoneNumberOrderError::ServiceFailure(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreatePhoneNumberOrderError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(CreatePhoneNumberOrderError::ThrottledClient(
                        err.msg,
                    ))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(CreatePhoneNumberOrderError::UnauthorizedClient(
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
impl fmt::Display for CreatePhoneNumberOrderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreatePhoneNumberOrderError {
    fn description(&self) -> &str {
        match *self {
            CreatePhoneNumberOrderError::BadRequest(ref cause) => cause,
            CreatePhoneNumberOrderError::Forbidden(ref cause) => cause,
            CreatePhoneNumberOrderError::ServiceFailure(ref cause) => cause,
            CreatePhoneNumberOrderError::ServiceUnavailable(ref cause) => cause,
            CreatePhoneNumberOrderError::ThrottledClient(ref cause) => cause,
            CreatePhoneNumberOrderError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateVoiceConnector
#[derive(Debug, PartialEq)]
pub enum CreateVoiceConnectorError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected error.</p>
    ServiceFailure(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The client exceeded its request rate limit.</p>
    ThrottledClient(String),
    /// <p>The client is not currently authorized to make the request.</p>
    UnauthorizedClient(String),
}

impl CreateVoiceConnectorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateVoiceConnectorError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateVoiceConnectorError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateVoiceConnectorError::Forbidden(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(CreateVoiceConnectorError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateVoiceConnectorError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(CreateVoiceConnectorError::ThrottledClient(
                        err.msg,
                    ))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(CreateVoiceConnectorError::UnauthorizedClient(
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
impl fmt::Display for CreateVoiceConnectorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateVoiceConnectorError {
    fn description(&self) -> &str {
        match *self {
            CreateVoiceConnectorError::BadRequest(ref cause) => cause,
            CreateVoiceConnectorError::Forbidden(ref cause) => cause,
            CreateVoiceConnectorError::ServiceFailure(ref cause) => cause,
            CreateVoiceConnectorError::ServiceUnavailable(ref cause) => cause,
            CreateVoiceConnectorError::ThrottledClient(ref cause) => cause,
            CreateVoiceConnectorError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteAccount
#[derive(Debug, PartialEq)]
pub enum DeleteAccountError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
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
/// Errors returned by DeleteEventsConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteEventsConfigurationError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
    Forbidden(String),
    /// <p>The request exceeds the resource limit.</p>
    ResourceLimitExceeded(String),
    /// <p>The service encountered an unexpected error.</p>
    ServiceFailure(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The client is not currently authorized to make the request.</p>
    UnauthorizedClient(String),
}

impl DeleteEventsConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteEventsConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteEventsConfigurationError::BadRequest(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteEventsConfigurationError::Forbidden(err.msg))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(
                        DeleteEventsConfigurationError::ResourceLimitExceeded(err.msg),
                    )
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(DeleteEventsConfigurationError::ServiceFailure(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        DeleteEventsConfigurationError::ServiceUnavailable(err.msg),
                    )
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(
                        DeleteEventsConfigurationError::UnauthorizedClient(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteEventsConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteEventsConfigurationError {
    fn description(&self) -> &str {
        match *self {
            DeleteEventsConfigurationError::BadRequest(ref cause) => cause,
            DeleteEventsConfigurationError::Forbidden(ref cause) => cause,
            DeleteEventsConfigurationError::ResourceLimitExceeded(ref cause) => cause,
            DeleteEventsConfigurationError::ServiceFailure(ref cause) => cause,
            DeleteEventsConfigurationError::ServiceUnavailable(ref cause) => cause,
            DeleteEventsConfigurationError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by DeletePhoneNumber
#[derive(Debug, PartialEq)]
pub enum DeletePhoneNumberError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
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

impl DeletePhoneNumberError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeletePhoneNumberError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeletePhoneNumberError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeletePhoneNumberError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeletePhoneNumberError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(DeletePhoneNumberError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeletePhoneNumberError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(DeletePhoneNumberError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(DeletePhoneNumberError::UnauthorizedClient(
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
impl fmt::Display for DeletePhoneNumberError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeletePhoneNumberError {
    fn description(&self) -> &str {
        match *self {
            DeletePhoneNumberError::BadRequest(ref cause) => cause,
            DeletePhoneNumberError::Forbidden(ref cause) => cause,
            DeletePhoneNumberError::NotFound(ref cause) => cause,
            DeletePhoneNumberError::ServiceFailure(ref cause) => cause,
            DeletePhoneNumberError::ServiceUnavailable(ref cause) => cause,
            DeletePhoneNumberError::ThrottledClient(ref cause) => cause,
            DeletePhoneNumberError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteVoiceConnector
#[derive(Debug, PartialEq)]
pub enum DeleteVoiceConnectorError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
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

impl DeleteVoiceConnectorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteVoiceConnectorError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteVoiceConnectorError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteVoiceConnectorError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteVoiceConnectorError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(DeleteVoiceConnectorError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteVoiceConnectorError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(DeleteVoiceConnectorError::ThrottledClient(
                        err.msg,
                    ))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(DeleteVoiceConnectorError::UnauthorizedClient(
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
impl fmt::Display for DeleteVoiceConnectorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteVoiceConnectorError {
    fn description(&self) -> &str {
        match *self {
            DeleteVoiceConnectorError::BadRequest(ref cause) => cause,
            DeleteVoiceConnectorError::Forbidden(ref cause) => cause,
            DeleteVoiceConnectorError::NotFound(ref cause) => cause,
            DeleteVoiceConnectorError::ServiceFailure(ref cause) => cause,
            DeleteVoiceConnectorError::ServiceUnavailable(ref cause) => cause,
            DeleteVoiceConnectorError::ThrottledClient(ref cause) => cause,
            DeleteVoiceConnectorError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteVoiceConnectorOrigination
#[derive(Debug, PartialEq)]
pub enum DeleteVoiceConnectorOriginationError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
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

impl DeleteVoiceConnectorOriginationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteVoiceConnectorOriginationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteVoiceConnectorOriginationError::BadRequest(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteVoiceConnectorOriginationError::Forbidden(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteVoiceConnectorOriginationError::NotFound(
                        err.msg,
                    ))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(
                        DeleteVoiceConnectorOriginationError::ServiceFailure(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        DeleteVoiceConnectorOriginationError::ServiceUnavailable(err.msg),
                    )
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(
                        DeleteVoiceConnectorOriginationError::ThrottledClient(err.msg),
                    )
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(
                        DeleteVoiceConnectorOriginationError::UnauthorizedClient(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteVoiceConnectorOriginationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteVoiceConnectorOriginationError {
    fn description(&self) -> &str {
        match *self {
            DeleteVoiceConnectorOriginationError::BadRequest(ref cause) => cause,
            DeleteVoiceConnectorOriginationError::Forbidden(ref cause) => cause,
            DeleteVoiceConnectorOriginationError::NotFound(ref cause) => cause,
            DeleteVoiceConnectorOriginationError::ServiceFailure(ref cause) => cause,
            DeleteVoiceConnectorOriginationError::ServiceUnavailable(ref cause) => cause,
            DeleteVoiceConnectorOriginationError::ThrottledClient(ref cause) => cause,
            DeleteVoiceConnectorOriginationError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteVoiceConnectorTermination
#[derive(Debug, PartialEq)]
pub enum DeleteVoiceConnectorTerminationError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
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

impl DeleteVoiceConnectorTerminationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteVoiceConnectorTerminationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteVoiceConnectorTerminationError::BadRequest(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteVoiceConnectorTerminationError::Forbidden(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteVoiceConnectorTerminationError::NotFound(
                        err.msg,
                    ))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(
                        DeleteVoiceConnectorTerminationError::ServiceFailure(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        DeleteVoiceConnectorTerminationError::ServiceUnavailable(err.msg),
                    )
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(
                        DeleteVoiceConnectorTerminationError::ThrottledClient(err.msg),
                    )
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(
                        DeleteVoiceConnectorTerminationError::UnauthorizedClient(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteVoiceConnectorTerminationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteVoiceConnectorTerminationError {
    fn description(&self) -> &str {
        match *self {
            DeleteVoiceConnectorTerminationError::BadRequest(ref cause) => cause,
            DeleteVoiceConnectorTerminationError::Forbidden(ref cause) => cause,
            DeleteVoiceConnectorTerminationError::NotFound(ref cause) => cause,
            DeleteVoiceConnectorTerminationError::ServiceFailure(ref cause) => cause,
            DeleteVoiceConnectorTerminationError::ServiceUnavailable(ref cause) => cause,
            DeleteVoiceConnectorTerminationError::ThrottledClient(ref cause) => cause,
            DeleteVoiceConnectorTerminationError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteVoiceConnectorTerminationCredentials
#[derive(Debug, PartialEq)]
pub enum DeleteVoiceConnectorTerminationCredentialsError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
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

impl DeleteVoiceConnectorTerminationCredentialsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteVoiceConnectorTerminationCredentialsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        DeleteVoiceConnectorTerminationCredentialsError::BadRequest(err.msg),
                    )
                }
                "ForbiddenException" => {
                    return RusotoError::Service(
                        DeleteVoiceConnectorTerminationCredentialsError::Forbidden(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(
                        DeleteVoiceConnectorTerminationCredentialsError::NotFound(err.msg),
                    )
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(
                        DeleteVoiceConnectorTerminationCredentialsError::ServiceFailure(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        DeleteVoiceConnectorTerminationCredentialsError::ServiceUnavailable(
                            err.msg,
                        ),
                    )
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(
                        DeleteVoiceConnectorTerminationCredentialsError::ThrottledClient(err.msg),
                    )
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(
                        DeleteVoiceConnectorTerminationCredentialsError::UnauthorizedClient(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteVoiceConnectorTerminationCredentialsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteVoiceConnectorTerminationCredentialsError {
    fn description(&self) -> &str {
        match *self {
            DeleteVoiceConnectorTerminationCredentialsError::BadRequest(ref cause) => cause,
            DeleteVoiceConnectorTerminationCredentialsError::Forbidden(ref cause) => cause,
            DeleteVoiceConnectorTerminationCredentialsError::NotFound(ref cause) => cause,
            DeleteVoiceConnectorTerminationCredentialsError::ServiceFailure(ref cause) => cause,
            DeleteVoiceConnectorTerminationCredentialsError::ServiceUnavailable(ref cause) => cause,
            DeleteVoiceConnectorTerminationCredentialsError::ThrottledClient(ref cause) => cause,
            DeleteVoiceConnectorTerminationCredentialsError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by DisassociatePhoneNumberFromUser
#[derive(Debug, PartialEq)]
pub enum DisassociatePhoneNumberFromUserError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
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

impl DisassociatePhoneNumberFromUserError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociatePhoneNumberFromUserError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DisassociatePhoneNumberFromUserError::BadRequest(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DisassociatePhoneNumberFromUserError::Forbidden(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DisassociatePhoneNumberFromUserError::NotFound(
                        err.msg,
                    ))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(
                        DisassociatePhoneNumberFromUserError::ServiceFailure(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        DisassociatePhoneNumberFromUserError::ServiceUnavailable(err.msg),
                    )
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(
                        DisassociatePhoneNumberFromUserError::ThrottledClient(err.msg),
                    )
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(
                        DisassociatePhoneNumberFromUserError::UnauthorizedClient(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DisassociatePhoneNumberFromUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociatePhoneNumberFromUserError {
    fn description(&self) -> &str {
        match *self {
            DisassociatePhoneNumberFromUserError::BadRequest(ref cause) => cause,
            DisassociatePhoneNumberFromUserError::Forbidden(ref cause) => cause,
            DisassociatePhoneNumberFromUserError::NotFound(ref cause) => cause,
            DisassociatePhoneNumberFromUserError::ServiceFailure(ref cause) => cause,
            DisassociatePhoneNumberFromUserError::ServiceUnavailable(ref cause) => cause,
            DisassociatePhoneNumberFromUserError::ThrottledClient(ref cause) => cause,
            DisassociatePhoneNumberFromUserError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by DisassociatePhoneNumbersFromVoiceConnector
#[derive(Debug, PartialEq)]
pub enum DisassociatePhoneNumbersFromVoiceConnectorError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
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

impl DisassociatePhoneNumbersFromVoiceConnectorError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociatePhoneNumbersFromVoiceConnectorError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        DisassociatePhoneNumbersFromVoiceConnectorError::BadRequest(err.msg),
                    )
                }
                "ForbiddenException" => {
                    return RusotoError::Service(
                        DisassociatePhoneNumbersFromVoiceConnectorError::Forbidden(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(
                        DisassociatePhoneNumbersFromVoiceConnectorError::NotFound(err.msg),
                    )
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(
                        DisassociatePhoneNumbersFromVoiceConnectorError::ServiceFailure(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        DisassociatePhoneNumbersFromVoiceConnectorError::ServiceUnavailable(
                            err.msg,
                        ),
                    )
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(
                        DisassociatePhoneNumbersFromVoiceConnectorError::ThrottledClient(err.msg),
                    )
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(
                        DisassociatePhoneNumbersFromVoiceConnectorError::UnauthorizedClient(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DisassociatePhoneNumbersFromVoiceConnectorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociatePhoneNumbersFromVoiceConnectorError {
    fn description(&self) -> &str {
        match *self {
            DisassociatePhoneNumbersFromVoiceConnectorError::BadRequest(ref cause) => cause,
            DisassociatePhoneNumbersFromVoiceConnectorError::Forbidden(ref cause) => cause,
            DisassociatePhoneNumbersFromVoiceConnectorError::NotFound(ref cause) => cause,
            DisassociatePhoneNumbersFromVoiceConnectorError::ServiceFailure(ref cause) => cause,
            DisassociatePhoneNumbersFromVoiceConnectorError::ServiceUnavailable(ref cause) => cause,
            DisassociatePhoneNumbersFromVoiceConnectorError::ThrottledClient(ref cause) => cause,
            DisassociatePhoneNumbersFromVoiceConnectorError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by GetAccount
#[derive(Debug, PartialEq)]
pub enum GetAccountError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
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
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
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
/// Errors returned by GetBot
#[derive(Debug, PartialEq)]
pub enum GetBotError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
    Forbidden(String),
    /// <p>One or more of the resources in the request does not exist in the system.</p>
    NotFound(String),
    /// <p>The service encountered an unexpected error.</p>
    ServiceFailure(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The client is not currently authorized to make the request.</p>
    UnauthorizedClient(String),
}

impl GetBotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBotError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetBotError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetBotError::Forbidden(err.msg))
                }
                "NotFoundException" => return RusotoError::Service(GetBotError::NotFound(err.msg)),
                "ServiceFailureException" => {
                    return RusotoError::Service(GetBotError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetBotError::ServiceUnavailable(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(GetBotError::UnauthorizedClient(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetBotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBotError {
    fn description(&self) -> &str {
        match *self {
            GetBotError::BadRequest(ref cause) => cause,
            GetBotError::Forbidden(ref cause) => cause,
            GetBotError::NotFound(ref cause) => cause,
            GetBotError::ServiceFailure(ref cause) => cause,
            GetBotError::ServiceUnavailable(ref cause) => cause,
            GetBotError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by GetEventsConfiguration
#[derive(Debug, PartialEq)]
pub enum GetEventsConfigurationError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
    Forbidden(String),
    /// <p>One or more of the resources in the request does not exist in the system.</p>
    NotFound(String),
    /// <p>The request exceeds the resource limit.</p>
    ResourceLimitExceeded(String),
    /// <p>The service encountered an unexpected error.</p>
    ServiceFailure(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The client is not currently authorized to make the request.</p>
    UnauthorizedClient(String),
}

impl GetEventsConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetEventsConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetEventsConfigurationError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetEventsConfigurationError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetEventsConfigurationError::NotFound(err.msg))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(
                        GetEventsConfigurationError::ResourceLimitExceeded(err.msg),
                    )
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(GetEventsConfigurationError::ServiceFailure(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetEventsConfigurationError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(GetEventsConfigurationError::UnauthorizedClient(
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
impl fmt::Display for GetEventsConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetEventsConfigurationError {
    fn description(&self) -> &str {
        match *self {
            GetEventsConfigurationError::BadRequest(ref cause) => cause,
            GetEventsConfigurationError::Forbidden(ref cause) => cause,
            GetEventsConfigurationError::NotFound(ref cause) => cause,
            GetEventsConfigurationError::ResourceLimitExceeded(ref cause) => cause,
            GetEventsConfigurationError::ServiceFailure(ref cause) => cause,
            GetEventsConfigurationError::ServiceUnavailable(ref cause) => cause,
            GetEventsConfigurationError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by GetGlobalSettings
#[derive(Debug, PartialEq)]
pub enum GetGlobalSettingsError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected error.</p>
    ServiceFailure(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The client exceeded its request rate limit.</p>
    ThrottledClient(String),
    /// <p>The client is not currently authorized to make the request.</p>
    UnauthorizedClient(String),
}

impl GetGlobalSettingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetGlobalSettingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetGlobalSettingsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetGlobalSettingsError::Forbidden(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(GetGlobalSettingsError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetGlobalSettingsError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(GetGlobalSettingsError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(GetGlobalSettingsError::UnauthorizedClient(
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
impl fmt::Display for GetGlobalSettingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetGlobalSettingsError {
    fn description(&self) -> &str {
        match *self {
            GetGlobalSettingsError::BadRequest(ref cause) => cause,
            GetGlobalSettingsError::Forbidden(ref cause) => cause,
            GetGlobalSettingsError::ServiceFailure(ref cause) => cause,
            GetGlobalSettingsError::ServiceUnavailable(ref cause) => cause,
            GetGlobalSettingsError::ThrottledClient(ref cause) => cause,
            GetGlobalSettingsError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by GetPhoneNumber
#[derive(Debug, PartialEq)]
pub enum GetPhoneNumberError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
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

impl GetPhoneNumberError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetPhoneNumberError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetPhoneNumberError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetPhoneNumberError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetPhoneNumberError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(GetPhoneNumberError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetPhoneNumberError::ServiceUnavailable(err.msg))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(GetPhoneNumberError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(GetPhoneNumberError::UnauthorizedClient(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetPhoneNumberError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetPhoneNumberError {
    fn description(&self) -> &str {
        match *self {
            GetPhoneNumberError::BadRequest(ref cause) => cause,
            GetPhoneNumberError::Forbidden(ref cause) => cause,
            GetPhoneNumberError::NotFound(ref cause) => cause,
            GetPhoneNumberError::ServiceFailure(ref cause) => cause,
            GetPhoneNumberError::ServiceUnavailable(ref cause) => cause,
            GetPhoneNumberError::ThrottledClient(ref cause) => cause,
            GetPhoneNumberError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by GetPhoneNumberOrder
#[derive(Debug, PartialEq)]
pub enum GetPhoneNumberOrderError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
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

impl GetPhoneNumberOrderError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetPhoneNumberOrderError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetPhoneNumberOrderError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetPhoneNumberOrderError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetPhoneNumberOrderError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(GetPhoneNumberOrderError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetPhoneNumberOrderError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(GetPhoneNumberOrderError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(GetPhoneNumberOrderError::UnauthorizedClient(
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
impl fmt::Display for GetPhoneNumberOrderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetPhoneNumberOrderError {
    fn description(&self) -> &str {
        match *self {
            GetPhoneNumberOrderError::BadRequest(ref cause) => cause,
            GetPhoneNumberOrderError::Forbidden(ref cause) => cause,
            GetPhoneNumberOrderError::NotFound(ref cause) => cause,
            GetPhoneNumberOrderError::ServiceFailure(ref cause) => cause,
            GetPhoneNumberOrderError::ServiceUnavailable(ref cause) => cause,
            GetPhoneNumberOrderError::ThrottledClient(ref cause) => cause,
            GetPhoneNumberOrderError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by GetUser
#[derive(Debug, PartialEq)]
pub enum GetUserError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
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
/// Errors returned by GetUserSettings
#[derive(Debug, PartialEq)]
pub enum GetUserSettingsError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
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

impl GetUserSettingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetUserSettingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetUserSettingsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetUserSettingsError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetUserSettingsError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(GetUserSettingsError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetUserSettingsError::ServiceUnavailable(err.msg))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(GetUserSettingsError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(GetUserSettingsError::UnauthorizedClient(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetUserSettingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetUserSettingsError {
    fn description(&self) -> &str {
        match *self {
            GetUserSettingsError::BadRequest(ref cause) => cause,
            GetUserSettingsError::Forbidden(ref cause) => cause,
            GetUserSettingsError::NotFound(ref cause) => cause,
            GetUserSettingsError::ServiceFailure(ref cause) => cause,
            GetUserSettingsError::ServiceUnavailable(ref cause) => cause,
            GetUserSettingsError::ThrottledClient(ref cause) => cause,
            GetUserSettingsError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by GetVoiceConnector
#[derive(Debug, PartialEq)]
pub enum GetVoiceConnectorError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
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

impl GetVoiceConnectorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetVoiceConnectorError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetVoiceConnectorError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetVoiceConnectorError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetVoiceConnectorError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(GetVoiceConnectorError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetVoiceConnectorError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(GetVoiceConnectorError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(GetVoiceConnectorError::UnauthorizedClient(
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
impl fmt::Display for GetVoiceConnectorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetVoiceConnectorError {
    fn description(&self) -> &str {
        match *self {
            GetVoiceConnectorError::BadRequest(ref cause) => cause,
            GetVoiceConnectorError::Forbidden(ref cause) => cause,
            GetVoiceConnectorError::NotFound(ref cause) => cause,
            GetVoiceConnectorError::ServiceFailure(ref cause) => cause,
            GetVoiceConnectorError::ServiceUnavailable(ref cause) => cause,
            GetVoiceConnectorError::ThrottledClient(ref cause) => cause,
            GetVoiceConnectorError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by GetVoiceConnectorOrigination
#[derive(Debug, PartialEq)]
pub enum GetVoiceConnectorOriginationError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
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

impl GetVoiceConnectorOriginationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetVoiceConnectorOriginationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetVoiceConnectorOriginationError::BadRequest(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetVoiceConnectorOriginationError::Forbidden(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetVoiceConnectorOriginationError::NotFound(
                        err.msg,
                    ))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(GetVoiceConnectorOriginationError::ServiceFailure(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        GetVoiceConnectorOriginationError::ServiceUnavailable(err.msg),
                    )
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(
                        GetVoiceConnectorOriginationError::ThrottledClient(err.msg),
                    )
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(
                        GetVoiceConnectorOriginationError::UnauthorizedClient(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetVoiceConnectorOriginationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetVoiceConnectorOriginationError {
    fn description(&self) -> &str {
        match *self {
            GetVoiceConnectorOriginationError::BadRequest(ref cause) => cause,
            GetVoiceConnectorOriginationError::Forbidden(ref cause) => cause,
            GetVoiceConnectorOriginationError::NotFound(ref cause) => cause,
            GetVoiceConnectorOriginationError::ServiceFailure(ref cause) => cause,
            GetVoiceConnectorOriginationError::ServiceUnavailable(ref cause) => cause,
            GetVoiceConnectorOriginationError::ThrottledClient(ref cause) => cause,
            GetVoiceConnectorOriginationError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by GetVoiceConnectorTermination
#[derive(Debug, PartialEq)]
pub enum GetVoiceConnectorTerminationError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
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

impl GetVoiceConnectorTerminationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetVoiceConnectorTerminationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetVoiceConnectorTerminationError::BadRequest(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetVoiceConnectorTerminationError::Forbidden(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetVoiceConnectorTerminationError::NotFound(
                        err.msg,
                    ))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(GetVoiceConnectorTerminationError::ServiceFailure(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        GetVoiceConnectorTerminationError::ServiceUnavailable(err.msg),
                    )
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(
                        GetVoiceConnectorTerminationError::ThrottledClient(err.msg),
                    )
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(
                        GetVoiceConnectorTerminationError::UnauthorizedClient(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetVoiceConnectorTerminationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetVoiceConnectorTerminationError {
    fn description(&self) -> &str {
        match *self {
            GetVoiceConnectorTerminationError::BadRequest(ref cause) => cause,
            GetVoiceConnectorTerminationError::Forbidden(ref cause) => cause,
            GetVoiceConnectorTerminationError::NotFound(ref cause) => cause,
            GetVoiceConnectorTerminationError::ServiceFailure(ref cause) => cause,
            GetVoiceConnectorTerminationError::ServiceUnavailable(ref cause) => cause,
            GetVoiceConnectorTerminationError::ThrottledClient(ref cause) => cause,
            GetVoiceConnectorTerminationError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by GetVoiceConnectorTerminationHealth
#[derive(Debug, PartialEq)]
pub enum GetVoiceConnectorTerminationHealthError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
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

impl GetVoiceConnectorTerminationHealthError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetVoiceConnectorTerminationHealthError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        GetVoiceConnectorTerminationHealthError::BadRequest(err.msg),
                    )
                }
                "ForbiddenException" => {
                    return RusotoError::Service(
                        GetVoiceConnectorTerminationHealthError::Forbidden(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetVoiceConnectorTerminationHealthError::NotFound(
                        err.msg,
                    ))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(
                        GetVoiceConnectorTerminationHealthError::ServiceFailure(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        GetVoiceConnectorTerminationHealthError::ServiceUnavailable(err.msg),
                    )
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(
                        GetVoiceConnectorTerminationHealthError::ThrottledClient(err.msg),
                    )
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(
                        GetVoiceConnectorTerminationHealthError::UnauthorizedClient(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetVoiceConnectorTerminationHealthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetVoiceConnectorTerminationHealthError {
    fn description(&self) -> &str {
        match *self {
            GetVoiceConnectorTerminationHealthError::BadRequest(ref cause) => cause,
            GetVoiceConnectorTerminationHealthError::Forbidden(ref cause) => cause,
            GetVoiceConnectorTerminationHealthError::NotFound(ref cause) => cause,
            GetVoiceConnectorTerminationHealthError::ServiceFailure(ref cause) => cause,
            GetVoiceConnectorTerminationHealthError::ServiceUnavailable(ref cause) => cause,
            GetVoiceConnectorTerminationHealthError::ThrottledClient(ref cause) => cause,
            GetVoiceConnectorTerminationHealthError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by InviteUsers
#[derive(Debug, PartialEq)]
pub enum InviteUsersError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
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
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
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
/// Errors returned by ListBots
#[derive(Debug, PartialEq)]
pub enum ListBotsError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
    Forbidden(String),
    /// <p>One or more of the resources in the request does not exist in the system.</p>
    NotFound(String),
    /// <p>The service encountered an unexpected error.</p>
    ServiceFailure(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The client is not currently authorized to make the request.</p>
    UnauthorizedClient(String),
}

impl ListBotsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListBotsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListBotsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListBotsError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListBotsError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(ListBotsError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListBotsError::ServiceUnavailable(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(ListBotsError::UnauthorizedClient(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListBotsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListBotsError {
    fn description(&self) -> &str {
        match *self {
            ListBotsError::BadRequest(ref cause) => cause,
            ListBotsError::Forbidden(ref cause) => cause,
            ListBotsError::NotFound(ref cause) => cause,
            ListBotsError::ServiceFailure(ref cause) => cause,
            ListBotsError::ServiceUnavailable(ref cause) => cause,
            ListBotsError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by ListPhoneNumberOrders
#[derive(Debug, PartialEq)]
pub enum ListPhoneNumberOrdersError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected error.</p>
    ServiceFailure(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The client exceeded its request rate limit.</p>
    ThrottledClient(String),
    /// <p>The client is not currently authorized to make the request.</p>
    UnauthorizedClient(String),
}

impl ListPhoneNumberOrdersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListPhoneNumberOrdersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListPhoneNumberOrdersError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListPhoneNumberOrdersError::Forbidden(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(ListPhoneNumberOrdersError::ServiceFailure(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListPhoneNumberOrdersError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(ListPhoneNumberOrdersError::ThrottledClient(
                        err.msg,
                    ))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(ListPhoneNumberOrdersError::UnauthorizedClient(
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
impl fmt::Display for ListPhoneNumberOrdersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListPhoneNumberOrdersError {
    fn description(&self) -> &str {
        match *self {
            ListPhoneNumberOrdersError::BadRequest(ref cause) => cause,
            ListPhoneNumberOrdersError::Forbidden(ref cause) => cause,
            ListPhoneNumberOrdersError::ServiceFailure(ref cause) => cause,
            ListPhoneNumberOrdersError::ServiceUnavailable(ref cause) => cause,
            ListPhoneNumberOrdersError::ThrottledClient(ref cause) => cause,
            ListPhoneNumberOrdersError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by ListPhoneNumbers
#[derive(Debug, PartialEq)]
pub enum ListPhoneNumbersError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected error.</p>
    ServiceFailure(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The client exceeded its request rate limit.</p>
    ThrottledClient(String),
    /// <p>The client is not currently authorized to make the request.</p>
    UnauthorizedClient(String),
}

impl ListPhoneNumbersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListPhoneNumbersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListPhoneNumbersError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListPhoneNumbersError::Forbidden(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(ListPhoneNumbersError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListPhoneNumbersError::ServiceUnavailable(err.msg))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(ListPhoneNumbersError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(ListPhoneNumbersError::UnauthorizedClient(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListPhoneNumbersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListPhoneNumbersError {
    fn description(&self) -> &str {
        match *self {
            ListPhoneNumbersError::BadRequest(ref cause) => cause,
            ListPhoneNumbersError::Forbidden(ref cause) => cause,
            ListPhoneNumbersError::ServiceFailure(ref cause) => cause,
            ListPhoneNumbersError::ServiceUnavailable(ref cause) => cause,
            ListPhoneNumbersError::ThrottledClient(ref cause) => cause,
            ListPhoneNumbersError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by ListUsers
#[derive(Debug, PartialEq)]
pub enum ListUsersError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
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
/// Errors returned by ListVoiceConnectorTerminationCredentials
#[derive(Debug, PartialEq)]
pub enum ListVoiceConnectorTerminationCredentialsError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
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

impl ListVoiceConnectorTerminationCredentialsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListVoiceConnectorTerminationCredentialsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        ListVoiceConnectorTerminationCredentialsError::BadRequest(err.msg),
                    )
                }
                "ForbiddenException" => {
                    return RusotoError::Service(
                        ListVoiceConnectorTerminationCredentialsError::Forbidden(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(
                        ListVoiceConnectorTerminationCredentialsError::NotFound(err.msg),
                    )
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(
                        ListVoiceConnectorTerminationCredentialsError::ServiceFailure(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        ListVoiceConnectorTerminationCredentialsError::ServiceUnavailable(err.msg),
                    )
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(
                        ListVoiceConnectorTerminationCredentialsError::ThrottledClient(err.msg),
                    )
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(
                        ListVoiceConnectorTerminationCredentialsError::UnauthorizedClient(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListVoiceConnectorTerminationCredentialsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListVoiceConnectorTerminationCredentialsError {
    fn description(&self) -> &str {
        match *self {
            ListVoiceConnectorTerminationCredentialsError::BadRequest(ref cause) => cause,
            ListVoiceConnectorTerminationCredentialsError::Forbidden(ref cause) => cause,
            ListVoiceConnectorTerminationCredentialsError::NotFound(ref cause) => cause,
            ListVoiceConnectorTerminationCredentialsError::ServiceFailure(ref cause) => cause,
            ListVoiceConnectorTerminationCredentialsError::ServiceUnavailable(ref cause) => cause,
            ListVoiceConnectorTerminationCredentialsError::ThrottledClient(ref cause) => cause,
            ListVoiceConnectorTerminationCredentialsError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by ListVoiceConnectors
#[derive(Debug, PartialEq)]
pub enum ListVoiceConnectorsError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected error.</p>
    ServiceFailure(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The client exceeded its request rate limit.</p>
    ThrottledClient(String),
    /// <p>The client is not currently authorized to make the request.</p>
    UnauthorizedClient(String),
}

impl ListVoiceConnectorsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListVoiceConnectorsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListVoiceConnectorsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListVoiceConnectorsError::Forbidden(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(ListVoiceConnectorsError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListVoiceConnectorsError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(ListVoiceConnectorsError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(ListVoiceConnectorsError::UnauthorizedClient(
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
impl fmt::Display for ListVoiceConnectorsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListVoiceConnectorsError {
    fn description(&self) -> &str {
        match *self {
            ListVoiceConnectorsError::BadRequest(ref cause) => cause,
            ListVoiceConnectorsError::Forbidden(ref cause) => cause,
            ListVoiceConnectorsError::ServiceFailure(ref cause) => cause,
            ListVoiceConnectorsError::ServiceUnavailable(ref cause) => cause,
            ListVoiceConnectorsError::ThrottledClient(ref cause) => cause,
            ListVoiceConnectorsError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by LogoutUser
#[derive(Debug, PartialEq)]
pub enum LogoutUserError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
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
/// Errors returned by PutEventsConfiguration
#[derive(Debug, PartialEq)]
pub enum PutEventsConfigurationError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
    Forbidden(String),
    /// <p>One or more of the resources in the request does not exist in the system.</p>
    NotFound(String),
    /// <p>The request exceeds the resource limit.</p>
    ResourceLimitExceeded(String),
    /// <p>The service encountered an unexpected error.</p>
    ServiceFailure(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The client is not currently authorized to make the request.</p>
    UnauthorizedClient(String),
}

impl PutEventsConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutEventsConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(PutEventsConfigurationError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(PutEventsConfigurationError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(PutEventsConfigurationError::NotFound(err.msg))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(
                        PutEventsConfigurationError::ResourceLimitExceeded(err.msg),
                    )
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(PutEventsConfigurationError::ServiceFailure(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(PutEventsConfigurationError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(PutEventsConfigurationError::UnauthorizedClient(
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
impl fmt::Display for PutEventsConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutEventsConfigurationError {
    fn description(&self) -> &str {
        match *self {
            PutEventsConfigurationError::BadRequest(ref cause) => cause,
            PutEventsConfigurationError::Forbidden(ref cause) => cause,
            PutEventsConfigurationError::NotFound(ref cause) => cause,
            PutEventsConfigurationError::ResourceLimitExceeded(ref cause) => cause,
            PutEventsConfigurationError::ServiceFailure(ref cause) => cause,
            PutEventsConfigurationError::ServiceUnavailable(ref cause) => cause,
            PutEventsConfigurationError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by PutVoiceConnectorOrigination
#[derive(Debug, PartialEq)]
pub enum PutVoiceConnectorOriginationError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
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

impl PutVoiceConnectorOriginationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutVoiceConnectorOriginationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(PutVoiceConnectorOriginationError::BadRequest(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(PutVoiceConnectorOriginationError::Forbidden(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(PutVoiceConnectorOriginationError::NotFound(
                        err.msg,
                    ))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(PutVoiceConnectorOriginationError::ServiceFailure(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        PutVoiceConnectorOriginationError::ServiceUnavailable(err.msg),
                    )
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(
                        PutVoiceConnectorOriginationError::ThrottledClient(err.msg),
                    )
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(
                        PutVoiceConnectorOriginationError::UnauthorizedClient(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutVoiceConnectorOriginationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutVoiceConnectorOriginationError {
    fn description(&self) -> &str {
        match *self {
            PutVoiceConnectorOriginationError::BadRequest(ref cause) => cause,
            PutVoiceConnectorOriginationError::Forbidden(ref cause) => cause,
            PutVoiceConnectorOriginationError::NotFound(ref cause) => cause,
            PutVoiceConnectorOriginationError::ServiceFailure(ref cause) => cause,
            PutVoiceConnectorOriginationError::ServiceUnavailable(ref cause) => cause,
            PutVoiceConnectorOriginationError::ThrottledClient(ref cause) => cause,
            PutVoiceConnectorOriginationError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by PutVoiceConnectorTermination
#[derive(Debug, PartialEq)]
pub enum PutVoiceConnectorTerminationError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
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

impl PutVoiceConnectorTerminationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutVoiceConnectorTerminationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(PutVoiceConnectorTerminationError::BadRequest(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(PutVoiceConnectorTerminationError::Forbidden(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(PutVoiceConnectorTerminationError::NotFound(
                        err.msg,
                    ))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(PutVoiceConnectorTerminationError::ServiceFailure(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        PutVoiceConnectorTerminationError::ServiceUnavailable(err.msg),
                    )
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(
                        PutVoiceConnectorTerminationError::ThrottledClient(err.msg),
                    )
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(
                        PutVoiceConnectorTerminationError::UnauthorizedClient(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutVoiceConnectorTerminationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutVoiceConnectorTerminationError {
    fn description(&self) -> &str {
        match *self {
            PutVoiceConnectorTerminationError::BadRequest(ref cause) => cause,
            PutVoiceConnectorTerminationError::Forbidden(ref cause) => cause,
            PutVoiceConnectorTerminationError::NotFound(ref cause) => cause,
            PutVoiceConnectorTerminationError::ServiceFailure(ref cause) => cause,
            PutVoiceConnectorTerminationError::ServiceUnavailable(ref cause) => cause,
            PutVoiceConnectorTerminationError::ThrottledClient(ref cause) => cause,
            PutVoiceConnectorTerminationError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by PutVoiceConnectorTerminationCredentials
#[derive(Debug, PartialEq)]
pub enum PutVoiceConnectorTerminationCredentialsError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
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

impl PutVoiceConnectorTerminationCredentialsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutVoiceConnectorTerminationCredentialsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        PutVoiceConnectorTerminationCredentialsError::BadRequest(err.msg),
                    )
                }
                "ForbiddenException" => {
                    return RusotoError::Service(
                        PutVoiceConnectorTerminationCredentialsError::Forbidden(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(
                        PutVoiceConnectorTerminationCredentialsError::NotFound(err.msg),
                    )
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(
                        PutVoiceConnectorTerminationCredentialsError::ServiceFailure(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        PutVoiceConnectorTerminationCredentialsError::ServiceUnavailable(err.msg),
                    )
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(
                        PutVoiceConnectorTerminationCredentialsError::ThrottledClient(err.msg),
                    )
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(
                        PutVoiceConnectorTerminationCredentialsError::UnauthorizedClient(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutVoiceConnectorTerminationCredentialsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutVoiceConnectorTerminationCredentialsError {
    fn description(&self) -> &str {
        match *self {
            PutVoiceConnectorTerminationCredentialsError::BadRequest(ref cause) => cause,
            PutVoiceConnectorTerminationCredentialsError::Forbidden(ref cause) => cause,
            PutVoiceConnectorTerminationCredentialsError::NotFound(ref cause) => cause,
            PutVoiceConnectorTerminationCredentialsError::ServiceFailure(ref cause) => cause,
            PutVoiceConnectorTerminationCredentialsError::ServiceUnavailable(ref cause) => cause,
            PutVoiceConnectorTerminationCredentialsError::ThrottledClient(ref cause) => cause,
            PutVoiceConnectorTerminationCredentialsError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by RegenerateSecurityToken
#[derive(Debug, PartialEq)]
pub enum RegenerateSecurityTokenError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
    Forbidden(String),
    /// <p>One or more of the resources in the request does not exist in the system.</p>
    NotFound(String),
    /// <p>The service encountered an unexpected error.</p>
    ServiceFailure(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The client is not currently authorized to make the request.</p>
    UnauthorizedClient(String),
}

impl RegenerateSecurityTokenError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RegenerateSecurityTokenError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(RegenerateSecurityTokenError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(RegenerateSecurityTokenError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(RegenerateSecurityTokenError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(RegenerateSecurityTokenError::ServiceFailure(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(RegenerateSecurityTokenError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(RegenerateSecurityTokenError::UnauthorizedClient(
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
impl fmt::Display for RegenerateSecurityTokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RegenerateSecurityTokenError {
    fn description(&self) -> &str {
        match *self {
            RegenerateSecurityTokenError::BadRequest(ref cause) => cause,
            RegenerateSecurityTokenError::Forbidden(ref cause) => cause,
            RegenerateSecurityTokenError::NotFound(ref cause) => cause,
            RegenerateSecurityTokenError::ServiceFailure(ref cause) => cause,
            RegenerateSecurityTokenError::ServiceUnavailable(ref cause) => cause,
            RegenerateSecurityTokenError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by ResetPersonalPIN
#[derive(Debug, PartialEq)]
pub enum ResetPersonalPINError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
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
/// Errors returned by RestorePhoneNumber
#[derive(Debug, PartialEq)]
pub enum RestorePhoneNumberError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
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

impl RestorePhoneNumberError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RestorePhoneNumberError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(RestorePhoneNumberError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(RestorePhoneNumberError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(RestorePhoneNumberError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(RestorePhoneNumberError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(RestorePhoneNumberError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(RestorePhoneNumberError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(RestorePhoneNumberError::UnauthorizedClient(
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
impl fmt::Display for RestorePhoneNumberError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RestorePhoneNumberError {
    fn description(&self) -> &str {
        match *self {
            RestorePhoneNumberError::BadRequest(ref cause) => cause,
            RestorePhoneNumberError::Forbidden(ref cause) => cause,
            RestorePhoneNumberError::NotFound(ref cause) => cause,
            RestorePhoneNumberError::ServiceFailure(ref cause) => cause,
            RestorePhoneNumberError::ServiceUnavailable(ref cause) => cause,
            RestorePhoneNumberError::ThrottledClient(ref cause) => cause,
            RestorePhoneNumberError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by SearchAvailablePhoneNumbers
#[derive(Debug, PartialEq)]
pub enum SearchAvailablePhoneNumbersError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected error.</p>
    ServiceFailure(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The client exceeded its request rate limit.</p>
    ThrottledClient(String),
    /// <p>The client is not currently authorized to make the request.</p>
    UnauthorizedClient(String),
}

impl SearchAvailablePhoneNumbersError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<SearchAvailablePhoneNumbersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(SearchAvailablePhoneNumbersError::BadRequest(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(SearchAvailablePhoneNumbersError::Forbidden(
                        err.msg,
                    ))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(SearchAvailablePhoneNumbersError::ServiceFailure(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        SearchAvailablePhoneNumbersError::ServiceUnavailable(err.msg),
                    )
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(SearchAvailablePhoneNumbersError::ThrottledClient(
                        err.msg,
                    ))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(
                        SearchAvailablePhoneNumbersError::UnauthorizedClient(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for SearchAvailablePhoneNumbersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SearchAvailablePhoneNumbersError {
    fn description(&self) -> &str {
        match *self {
            SearchAvailablePhoneNumbersError::BadRequest(ref cause) => cause,
            SearchAvailablePhoneNumbersError::Forbidden(ref cause) => cause,
            SearchAvailablePhoneNumbersError::ServiceFailure(ref cause) => cause,
            SearchAvailablePhoneNumbersError::ServiceUnavailable(ref cause) => cause,
            SearchAvailablePhoneNumbersError::ThrottledClient(ref cause) => cause,
            SearchAvailablePhoneNumbersError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateAccount
#[derive(Debug, PartialEq)]
pub enum UpdateAccountError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
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
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
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
/// Errors returned by UpdateBot
#[derive(Debug, PartialEq)]
pub enum UpdateBotError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
    Forbidden(String),
    /// <p>One or more of the resources in the request does not exist in the system.</p>
    NotFound(String),
    /// <p>The service encountered an unexpected error.</p>
    ServiceFailure(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The client is not currently authorized to make the request.</p>
    UnauthorizedClient(String),
}

impl UpdateBotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateBotError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateBotError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateBotError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateBotError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(UpdateBotError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateBotError::ServiceUnavailable(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(UpdateBotError::UnauthorizedClient(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateBotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateBotError {
    fn description(&self) -> &str {
        match *self {
            UpdateBotError::BadRequest(ref cause) => cause,
            UpdateBotError::Forbidden(ref cause) => cause,
            UpdateBotError::NotFound(ref cause) => cause,
            UpdateBotError::ServiceFailure(ref cause) => cause,
            UpdateBotError::ServiceUnavailable(ref cause) => cause,
            UpdateBotError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateGlobalSettings
#[derive(Debug, PartialEq)]
pub enum UpdateGlobalSettingsError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected error.</p>
    ServiceFailure(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The client exceeded its request rate limit.</p>
    ThrottledClient(String),
    /// <p>The client is not currently authorized to make the request.</p>
    UnauthorizedClient(String),
}

impl UpdateGlobalSettingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateGlobalSettingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateGlobalSettingsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateGlobalSettingsError::Forbidden(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(UpdateGlobalSettingsError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateGlobalSettingsError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(UpdateGlobalSettingsError::ThrottledClient(
                        err.msg,
                    ))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(UpdateGlobalSettingsError::UnauthorizedClient(
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
impl fmt::Display for UpdateGlobalSettingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateGlobalSettingsError {
    fn description(&self) -> &str {
        match *self {
            UpdateGlobalSettingsError::BadRequest(ref cause) => cause,
            UpdateGlobalSettingsError::Forbidden(ref cause) => cause,
            UpdateGlobalSettingsError::ServiceFailure(ref cause) => cause,
            UpdateGlobalSettingsError::ServiceUnavailable(ref cause) => cause,
            UpdateGlobalSettingsError::ThrottledClient(ref cause) => cause,
            UpdateGlobalSettingsError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdatePhoneNumber
#[derive(Debug, PartialEq)]
pub enum UpdatePhoneNumberError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
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

impl UpdatePhoneNumberError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdatePhoneNumberError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdatePhoneNumberError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdatePhoneNumberError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdatePhoneNumberError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(UpdatePhoneNumberError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdatePhoneNumberError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(UpdatePhoneNumberError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(UpdatePhoneNumberError::UnauthorizedClient(
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
impl fmt::Display for UpdatePhoneNumberError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdatePhoneNumberError {
    fn description(&self) -> &str {
        match *self {
            UpdatePhoneNumberError::BadRequest(ref cause) => cause,
            UpdatePhoneNumberError::Forbidden(ref cause) => cause,
            UpdatePhoneNumberError::NotFound(ref cause) => cause,
            UpdatePhoneNumberError::ServiceFailure(ref cause) => cause,
            UpdatePhoneNumberError::ServiceUnavailable(ref cause) => cause,
            UpdatePhoneNumberError::ThrottledClient(ref cause) => cause,
            UpdatePhoneNumberError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateUser
#[derive(Debug, PartialEq)]
pub enum UpdateUserError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
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
/// Errors returned by UpdateUserSettings
#[derive(Debug, PartialEq)]
pub enum UpdateUserSettingsError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
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

impl UpdateUserSettingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateUserSettingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateUserSettingsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateUserSettingsError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateUserSettingsError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(UpdateUserSettingsError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateUserSettingsError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(UpdateUserSettingsError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(UpdateUserSettingsError::UnauthorizedClient(
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
impl fmt::Display for UpdateUserSettingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateUserSettingsError {
    fn description(&self) -> &str {
        match *self {
            UpdateUserSettingsError::BadRequest(ref cause) => cause,
            UpdateUserSettingsError::Forbidden(ref cause) => cause,
            UpdateUserSettingsError::NotFound(ref cause) => cause,
            UpdateUserSettingsError::ServiceFailure(ref cause) => cause,
            UpdateUserSettingsError::ServiceUnavailable(ref cause) => cause,
            UpdateUserSettingsError::ThrottledClient(ref cause) => cause,
            UpdateUserSettingsError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateVoiceConnector
#[derive(Debug, PartialEq)]
pub enum UpdateVoiceConnectorError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The client is permanently forbidden from making the request. For example, when a user tries to create an account from an unsupported Region.</p>
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

impl UpdateVoiceConnectorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateVoiceConnectorError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateVoiceConnectorError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateVoiceConnectorError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateVoiceConnectorError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(UpdateVoiceConnectorError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateVoiceConnectorError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(UpdateVoiceConnectorError::ThrottledClient(
                        err.msg,
                    ))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(UpdateVoiceConnectorError::UnauthorizedClient(
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
impl fmt::Display for UpdateVoiceConnectorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateVoiceConnectorError {
    fn description(&self) -> &str {
        match *self {
            UpdateVoiceConnectorError::BadRequest(ref cause) => cause,
            UpdateVoiceConnectorError::Forbidden(ref cause) => cause,
            UpdateVoiceConnectorError::NotFound(ref cause) => cause,
            UpdateVoiceConnectorError::ServiceFailure(ref cause) => cause,
            UpdateVoiceConnectorError::ServiceUnavailable(ref cause) => cause,
            UpdateVoiceConnectorError::ThrottledClient(ref cause) => cause,
            UpdateVoiceConnectorError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon Chime API. Amazon Chime clients implement this trait.
pub trait Chime {
    /// <p>Associates a phone number with the specified Amazon Chime user.</p>
    fn associate_phone_number_with_user(
        &self,
        input: AssociatePhoneNumberWithUserRequest,
    ) -> Request<AssociatePhoneNumberWithUserRequest>;

    /// <p>Associates a phone number with the specified Amazon Chime Voice Connector.</p>
    fn associate_phone_numbers_with_voice_connector(
        &self,
        input: AssociatePhoneNumbersWithVoiceConnectorRequest,
    ) -> Request<AssociatePhoneNumbersWithVoiceConnectorRequest>;

    /// <p>Moves phone numbers into the <b>Deletion queue</b>. Phone numbers must be disassociated from any users or Amazon Chime Voice Connectors before they can be deleted.</p> <p>Phone numbers remain in the <b>Deletion queue</b> for 7 days before they are deleted permanently.</p>
    fn batch_delete_phone_number(
        &self,
        input: BatchDeletePhoneNumberRequest,
    ) -> Request<BatchDeletePhoneNumberRequest>;

    /// <p>Suspends up to 50 users from a <code>Team</code> or <code>EnterpriseLWA</code> Amazon Chime account. For more information about different account types, see <a href="https://docs.aws.amazon.com/chime/latest/ag/manage-chime-account.html">Managing Your Amazon Chime Accounts</a> in the <i>Amazon Chime Administration Guide</i>.</p> <p>Users suspended from a <code>Team</code> account are dissasociated from the account, but they can continue to use Amazon Chime as free users. To remove the suspension from suspended <code>Team</code> account users, invite them to the <code>Team</code> account again. You can use the <a>InviteUsers</a> action to do so.</p> <p>Users suspended from an <code>EnterpriseLWA</code> account are immediately signed out of Amazon Chime and can no longer sign in. To remove the suspension from suspended <code>EnterpriseLWA</code> account users, use the <a>BatchUnsuspendUser</a> action. </p> <p>To sign out users without suspending them, use the <a>LogoutUser</a> action.</p>
    fn batch_suspend_user(
        &self,
        input: BatchSuspendUserRequest,
    ) -> Request<BatchSuspendUserRequest>;

    /// <p>Removes the suspension from up to 50 previously suspended users for the specified Amazon Chime <code>EnterpriseLWA</code> account. Only users on <code>EnterpriseLWA</code> accounts can be unsuspended using this action. For more information about different account types, see <a href="https://docs.aws.amazon.com/chime/latest/ag/manage-chime-account.html">Managing Your Amazon Chime Accounts</a> in the <i>Amazon Chime Administration Guide</i>.</p> <p>Previously suspended users who are unsuspended using this action are returned to <code>Registered</code> status. Users who are not previously suspended are ignored.</p>
    fn batch_unsuspend_user(
        &self,
        input: BatchUnsuspendUserRequest,
    ) -> Request<BatchUnsuspendUserRequest>;

    /// <p>Updates phone number product types. Choose from Amazon Chime Business Calling and Amazon Chime Voice Connector product types.</p>
    fn batch_update_phone_number(
        &self,
        input: BatchUpdatePhoneNumberRequest,
    ) -> Request<BatchUpdatePhoneNumberRequest>;

    /// <p>Updates user details within the <a>UpdateUserRequestItem</a> object for up to 20 users for the specified Amazon Chime account. Currently, only <code>LicenseType</code> updates are supported for this action.</p>
    fn batch_update_user(&self, input: BatchUpdateUserRequest) -> Request<BatchUpdateUserRequest>;

    /// <p>Creates an Amazon Chime account under the administrator's AWS account. Only <code>Team</code> account types are currently supported for this action. For more information about different account types, see <a href="https://docs.aws.amazon.com/chime/latest/ag/manage-chime-account.html">Managing Your Amazon Chime Accounts</a> in the <i>Amazon Chime Administration Guide</i>.</p>
    fn create_account(&self, input: CreateAccountRequest) -> Request<CreateAccountRequest>;

    /// <p>Creates a bot for an Amazon Chime Enterprise account.</p>
    fn create_bot(&self, input: CreateBotRequest) -> Request<CreateBotRequest>;

    /// <p>Creates an order for phone numbers to be provisioned. Choose from Amazon Chime Business Calling and Amazon Chime Voice Connector product types.</p>
    fn create_phone_number_order(
        &self,
        input: CreatePhoneNumberOrderRequest,
    ) -> Request<CreatePhoneNumberOrderRequest>;

    /// <p>Creates an Amazon Chime Voice Connector under the administrator's AWS account. Enabling <a>CreateVoiceConnectorRequest$RequireEncryption</a> configures your Amazon Chime Voice Connector to use TLS transport for SIP signaling and Secure RTP (SRTP) for media. Inbound calls use TLS transport, and unencrypted outbound calls are blocked.</p>
    fn create_voice_connector(
        &self,
        input: CreateVoiceConnectorRequest,
    ) -> Request<CreateVoiceConnectorRequest>;

    /// <p>Deletes the specified Amazon Chime account. You must suspend all users before deleting a <code>Team</code> account. You can use the <a>BatchSuspendUser</a> action to do so.</p> <p>For <code>EnterpriseLWA</code> and <code>EnterpriseAD</code> accounts, you must release the claimed domains for your Amazon Chime account before deletion. As soon as you release the domain, all users under that account are suspended.</p> <p>Deleted accounts appear in your <code>Disabled</code> accounts list for 90 days. To restore a deleted account from your <code>Disabled</code> accounts list, you must contact AWS Support.</p> <p>After 90 days, deleted accounts are permanently removed from your <code>Disabled</code> accounts list.</p>
    fn delete_account(&self, input: DeleteAccountRequest) -> Request<DeleteAccountRequest>;

    /// <p>Deletes the events configuration that allows a bot to receive outgoing events.</p>
    fn delete_events_configuration(
        &self,
        input: DeleteEventsConfigurationRequest,
    ) -> Request<DeleteEventsConfigurationRequest>;

    /// <p>Moves the specified phone number into the <b>Deletion queue</b>. A phone number must be disassociated from any users or Amazon Chime Voice Connectors before it can be deleted.</p> <p>Deleted phone numbers remain in the <b>Deletion queue</b> for 7 days before they are deleted permanently.</p>
    fn delete_phone_number(
        &self,
        input: DeletePhoneNumberRequest,
    ) -> Request<DeletePhoneNumberRequest>;

    /// <p>Deletes the specified Amazon Chime Voice Connector. Any phone numbers assigned to the Amazon Chime Voice Connector must be unassigned from it before it can be deleted.</p>
    fn delete_voice_connector(
        &self,
        input: DeleteVoiceConnectorRequest,
    ) -> Request<DeleteVoiceConnectorRequest>;

    /// <p>Deletes the origination settings for the specified Amazon Chime Voice Connector.</p>
    fn delete_voice_connector_origination(
        &self,
        input: DeleteVoiceConnectorOriginationRequest,
    ) -> Request<DeleteVoiceConnectorOriginationRequest>;

    /// <p>Deletes the termination settings for the specified Amazon Chime Voice Connector.</p>
    fn delete_voice_connector_termination(
        &self,
        input: DeleteVoiceConnectorTerminationRequest,
    ) -> Request<DeleteVoiceConnectorTerminationRequest>;

    /// <p>Deletes the specified SIP credentials used by your equipment to authenticate during call termination.</p>
    fn delete_voice_connector_termination_credentials(
        &self,
        input: DeleteVoiceConnectorTerminationCredentialsRequest,
    ) -> Request<DeleteVoiceConnectorTerminationCredentialsRequest>;

    /// <p>Disassociates the primary provisioned phone number from the specified Amazon Chime user.</p>
    fn disassociate_phone_number_from_user(
        &self,
        input: DisassociatePhoneNumberFromUserRequest,
    ) -> Request<DisassociatePhoneNumberFromUserRequest>;

    /// <p>Disassociates the specified phone number from the specified Amazon Chime Voice Connector.</p>
    fn disassociate_phone_numbers_from_voice_connector(
        &self,
        input: DisassociatePhoneNumbersFromVoiceConnectorRequest,
    ) -> Request<DisassociatePhoneNumbersFromVoiceConnectorRequest>;

    /// <p>Retrieves details for the specified Amazon Chime account, such as account type and supported licenses.</p>
    fn get_account(&self, input: GetAccountRequest) -> Request<GetAccountRequest>;

    /// <p>Retrieves account settings for the specified Amazon Chime account ID, such as remote control and dial out settings. For more information about these settings, see <a href="https://docs.aws.amazon.com/chime/latest/ag/policies.html">Use the Policies Page</a> in the <i>Amazon Chime Administration Guide</i>.</p>
    fn get_account_settings(
        &self,
        input: GetAccountSettingsRequest,
    ) -> Request<GetAccountSettingsRequest>;

    /// <p>Retrieves details for the specified bot, such as bot email address, bot type, status, and display name.</p>
    fn get_bot(&self, input: GetBotRequest) -> Request<GetBotRequest>;

    /// <p>Gets details for an events configuration that allows a bot to receive outgoing events, such as an HTTPS endpoint or Lambda function ARN. </p>
    fn get_events_configuration(
        &self,
        input: GetEventsConfigurationRequest,
    ) -> Request<GetEventsConfigurationRequest>;

    /// <p>Retrieves global settings for the administrator's AWS account, such as Amazon Chime Business Calling and Amazon Chime Voice Connector settings.</p>
    fn get_global_settings(&self) -> Request<GetGlobalSettingsRequest>;

    /// <p>Retrieves details for the specified phone number ID, such as associations, capabilities, and product type.</p>
    fn get_phone_number(&self, input: GetPhoneNumberRequest) -> Request<GetPhoneNumberRequest>;

    /// <p>Retrieves details for the specified phone number order, such as order creation timestamp, phone numbers in E.164 format, product type, and order status.</p>
    fn get_phone_number_order(
        &self,
        input: GetPhoneNumberOrderRequest,
    ) -> Request<GetPhoneNumberOrderRequest>;

    /// <p>Retrieves details for the specified user ID, such as primary email address, license type, and personal meeting PIN.</p> <p>To retrieve user details with an email address instead of a user ID, use the <a>ListUsers</a> action, and then filter by email address.</p>
    fn get_user(&self, input: GetUserRequest) -> Request<GetUserRequest>;

    /// <p>Retrieves settings for the specified user ID, such as any associated phone number settings.</p>
    fn get_user_settings(&self, input: GetUserSettingsRequest) -> Request<GetUserSettingsRequest>;

    /// <p>Retrieves details for the specified Amazon Chime Voice Connector, such as timestamps, name, outbound host, and encryption requirements.</p>
    fn get_voice_connector(
        &self,
        input: GetVoiceConnectorRequest,
    ) -> Request<GetVoiceConnectorRequest>;

    /// <p>Retrieves origination setting details for the specified Amazon Chime Voice Connector.</p>
    fn get_voice_connector_origination(
        &self,
        input: GetVoiceConnectorOriginationRequest,
    ) -> Request<GetVoiceConnectorOriginationRequest>;

    /// <p>Retrieves termination setting details for the specified Amazon Chime Voice Connector.</p>
    fn get_voice_connector_termination(
        &self,
        input: GetVoiceConnectorTerminationRequest,
    ) -> Request<GetVoiceConnectorTerminationRequest>;

    /// <p>Retrieves information about the last time a SIP <code>OPTIONS</code> ping was received from your SIP infrastructure for the specified Amazon Chime Voice Connector.</p>
    fn get_voice_connector_termination_health(
        &self,
        input: GetVoiceConnectorTerminationHealthRequest,
    ) -> Request<GetVoiceConnectorTerminationHealthRequest>;

    /// <p>Sends email invites to as many as 50 users, inviting them to the specified Amazon Chime <code>Team</code> account. Only <code>Team</code> account types are currently supported for this action. </p>
    fn invite_users(&self, input: InviteUsersRequest) -> Request<InviteUsersRequest>;

    /// <p>Lists the Amazon Chime accounts under the administrator's AWS account. You can filter accounts by account name prefix. To find out which Amazon Chime account a user belongs to, you can filter by the user's email address, which returns one account result.</p>
    fn list_accounts(&self, input: ListAccountsRequest) -> Request<ListAccountsRequest>;

    /// <p>Lists the bots associated with the administrator's Amazon Chime Enterprise account ID.</p>
    fn list_bots(&self, input: ListBotsRequest) -> Request<ListBotsRequest>;

    /// <p>Lists the phone number orders for the administrator's Amazon Chime account.</p>
    fn list_phone_number_orders(
        &self,
        input: ListPhoneNumberOrdersRequest,
    ) -> Request<ListPhoneNumberOrdersRequest>;

    /// <p>Lists the phone numbers for the specified Amazon Chime account, Amazon Chime user, or Amazon Chime Voice Connector.</p>
    fn list_phone_numbers(
        &self,
        input: ListPhoneNumbersRequest,
    ) -> Request<ListPhoneNumbersRequest>;

    /// <p>Lists the users that belong to the specified Amazon Chime account. You can specify an email address to list only the user that the email address belongs to.</p>
    fn list_users(&self, input: ListUsersRequest) -> Request<ListUsersRequest>;

    /// <p>Lists the SIP credentials for the specified Amazon Chime Voice Connector.</p>
    fn list_voice_connector_termination_credentials(
        &self,
        input: ListVoiceConnectorTerminationCredentialsRequest,
    ) -> Request<ListVoiceConnectorTerminationCredentialsRequest>;

    /// <p>Lists the Amazon Chime Voice Connectors for the administrator's AWS account.</p>
    fn list_voice_connectors(
        &self,
        input: ListVoiceConnectorsRequest,
    ) -> Request<ListVoiceConnectorsRequest>;

    /// <p>Logs out the specified user from all of the devices they are currently logged into.</p>
    fn logout_user(&self, input: LogoutUserRequest) -> Request<LogoutUserRequest>;

    /// <p>Creates an events configuration that allows a bot to receive outgoing events sent by Amazon Chime. Choose either an HTTPS endpoint or a Lambda function ARN. For more information, see <a>Bot</a>.</p>
    fn put_events_configuration(
        &self,
        input: PutEventsConfigurationRequest,
    ) -> Request<PutEventsConfigurationRequest>;

    /// <p>Adds origination settings for the specified Amazon Chime Voice Connector.</p>
    fn put_voice_connector_origination(
        &self,
        input: PutVoiceConnectorOriginationRequest,
    ) -> Request<PutVoiceConnectorOriginationRequest>;

    /// <p>Adds termination settings for the specified Amazon Chime Voice Connector.</p>
    fn put_voice_connector_termination(
        &self,
        input: PutVoiceConnectorTerminationRequest,
    ) -> Request<PutVoiceConnectorTerminationRequest>;

    /// <p>Adds termination SIP credentials for the specified Amazon Chime Voice Connector.</p>
    fn put_voice_connector_termination_credentials(
        &self,
        input: PutVoiceConnectorTerminationCredentialsRequest,
    ) -> Request<PutVoiceConnectorTerminationCredentialsRequest>;

    /// <p>Regenerates the security token for a bot.</p>
    fn regenerate_security_token(
        &self,
        input: RegenerateSecurityTokenRequest,
    ) -> Request<RegenerateSecurityTokenRequest>;

    /// <p>Resets the personal meeting PIN for the specified user on an Amazon Chime account. Returns the <a>User</a> object with the updated personal meeting PIN.</p>
    fn reset_personal_pin(
        &self,
        input: ResetPersonalPINRequest,
    ) -> Request<ResetPersonalPINRequest>;

    /// <p>Moves a phone number from the <b>Deletion queue</b> back into the phone number <b>Inventory</b>.</p>
    fn restore_phone_number(
        &self,
        input: RestorePhoneNumberRequest,
    ) -> Request<RestorePhoneNumberRequest>;

    /// <p>Searches phone numbers that can be ordered.</p>
    fn search_available_phone_numbers(
        &self,
        input: SearchAvailablePhoneNumbersRequest,
    ) -> Request<SearchAvailablePhoneNumbersRequest>;

    /// <p>Updates account details for the specified Amazon Chime account. Currently, only account name updates are supported for this action.</p>
    fn update_account(&self, input: UpdateAccountRequest) -> Request<UpdateAccountRequest>;

    /// <p>Updates the settings for the specified Amazon Chime account. You can update settings for remote control of shared screens, or for the dial-out option. For more information about these settings, see <a href="https://docs.aws.amazon.com/chime/latest/ag/policies.html">Use the Policies Page</a> in the <i>Amazon Chime Administration Guide</i>.</p>
    fn update_account_settings(
        &self,
        input: UpdateAccountSettingsRequest,
    ) -> Request<UpdateAccountSettingsRequest>;

    /// <p>Updates the status of the specified bot, such as starting or stopping the bot from running in your Amazon Chime Enterprise account.</p>
    fn update_bot(&self, input: UpdateBotRequest) -> Request<UpdateBotRequest>;

    /// <p>Updates global settings for the administrator's AWS account, such as Amazon Chime Business Calling and Amazon Chime Voice Connector settings.</p>
    fn update_global_settings(
        &self,
        input: UpdateGlobalSettingsRequest,
    ) -> Request<UpdateGlobalSettingsRequest>;

    /// <p>Updates phone number details, such as product type, for the specified phone number ID.</p>
    fn update_phone_number(
        &self,
        input: UpdatePhoneNumberRequest,
    ) -> Request<UpdatePhoneNumberRequest>;

    /// <p>Updates user details for a specified user ID. Currently, only <code>LicenseType</code> updates are supported for this action.</p>
    fn update_user(&self, input: UpdateUserRequest) -> Request<UpdateUserRequest>;

    /// <p>Updates the settings for the specified user, such as phone number settings.</p>
    fn update_user_settings(
        &self,
        input: UpdateUserSettingsRequest,
    ) -> Request<UpdateUserSettingsRequest>;

    /// <p>Updates details for the specified Amazon Chime Voice Connector.</p>
    fn update_voice_connector(
        &self,
        input: UpdateVoiceConnectorRequest,
    ) -> Request<UpdateVoiceConnectorRequest>;
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
            region,
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
            region,
        }
    }
}

impl Chime for ChimeClient {
    /// <p>Associates a phone number with the specified Amazon Chime user.</p>
    fn associate_phone_number_with_user(
        &self,
        input: AssociatePhoneNumberWithUserRequest,
    ) -> Request<AssociatePhoneNumberWithUserRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Associates a phone number with the specified Amazon Chime Voice Connector.</p>
    fn associate_phone_numbers_with_voice_connector(
        &self,
        input: AssociatePhoneNumbersWithVoiceConnectorRequest,
    ) -> Request<AssociatePhoneNumbersWithVoiceConnectorRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Moves phone numbers into the <b>Deletion queue</b>. Phone numbers must be disassociated from any users or Amazon Chime Voice Connectors before they can be deleted.</p> <p>Phone numbers remain in the <b>Deletion queue</b> for 7 days before they are deleted permanently.</p>
    fn batch_delete_phone_number(
        &self,
        input: BatchDeletePhoneNumberRequest,
    ) -> Request<BatchDeletePhoneNumberRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Suspends up to 50 users from a <code>Team</code> or <code>EnterpriseLWA</code> Amazon Chime account. For more information about different account types, see <a href="https://docs.aws.amazon.com/chime/latest/ag/manage-chime-account.html">Managing Your Amazon Chime Accounts</a> in the <i>Amazon Chime Administration Guide</i>.</p> <p>Users suspended from a <code>Team</code> account are dissasociated from the account, but they can continue to use Amazon Chime as free users. To remove the suspension from suspended <code>Team</code> account users, invite them to the <code>Team</code> account again. You can use the <a>InviteUsers</a> action to do so.</p> <p>Users suspended from an <code>EnterpriseLWA</code> account are immediately signed out of Amazon Chime and can no longer sign in. To remove the suspension from suspended <code>EnterpriseLWA</code> account users, use the <a>BatchUnsuspendUser</a> action. </p> <p>To sign out users without suspending them, use the <a>LogoutUser</a> action.</p>
    fn batch_suspend_user(
        &self,
        input: BatchSuspendUserRequest,
    ) -> Request<BatchSuspendUserRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Removes the suspension from up to 50 previously suspended users for the specified Amazon Chime <code>EnterpriseLWA</code> account. Only users on <code>EnterpriseLWA</code> accounts can be unsuspended using this action. For more information about different account types, see <a href="https://docs.aws.amazon.com/chime/latest/ag/manage-chime-account.html">Managing Your Amazon Chime Accounts</a> in the <i>Amazon Chime Administration Guide</i>.</p> <p>Previously suspended users who are unsuspended using this action are returned to <code>Registered</code> status. Users who are not previously suspended are ignored.</p>
    fn batch_unsuspend_user(
        &self,
        input: BatchUnsuspendUserRequest,
    ) -> Request<BatchUnsuspendUserRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Updates phone number product types. Choose from Amazon Chime Business Calling and Amazon Chime Voice Connector product types.</p>
    fn batch_update_phone_number(
        &self,
        input: BatchUpdatePhoneNumberRequest,
    ) -> Request<BatchUpdatePhoneNumberRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Updates user details within the <a>UpdateUserRequestItem</a> object for up to 20 users for the specified Amazon Chime account. Currently, only <code>LicenseType</code> updates are supported for this action.</p>
    fn batch_update_user(&self, input: BatchUpdateUserRequest) -> Request<BatchUpdateUserRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Creates an Amazon Chime account under the administrator's AWS account. Only <code>Team</code> account types are currently supported for this action. For more information about different account types, see <a href="https://docs.aws.amazon.com/chime/latest/ag/manage-chime-account.html">Managing Your Amazon Chime Accounts</a> in the <i>Amazon Chime Administration Guide</i>.</p>
    fn create_account(&self, input: CreateAccountRequest) -> Request<CreateAccountRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Creates a bot for an Amazon Chime Enterprise account.</p>
    fn create_bot(&self, input: CreateBotRequest) -> Request<CreateBotRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Creates an order for phone numbers to be provisioned. Choose from Amazon Chime Business Calling and Amazon Chime Voice Connector product types.</p>
    fn create_phone_number_order(
        &self,
        input: CreatePhoneNumberOrderRequest,
    ) -> Request<CreatePhoneNumberOrderRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Creates an Amazon Chime Voice Connector under the administrator's AWS account. Enabling <a>CreateVoiceConnectorRequest$RequireEncryption</a> configures your Amazon Chime Voice Connector to use TLS transport for SIP signaling and Secure RTP (SRTP) for media. Inbound calls use TLS transport, and unencrypted outbound calls are blocked.</p>
    fn create_voice_connector(
        &self,
        input: CreateVoiceConnectorRequest,
    ) -> Request<CreateVoiceConnectorRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Deletes the specified Amazon Chime account. You must suspend all users before deleting a <code>Team</code> account. You can use the <a>BatchSuspendUser</a> action to do so.</p> <p>For <code>EnterpriseLWA</code> and <code>EnterpriseAD</code> accounts, you must release the claimed domains for your Amazon Chime account before deletion. As soon as you release the domain, all users under that account are suspended.</p> <p>Deleted accounts appear in your <code>Disabled</code> accounts list for 90 days. To restore a deleted account from your <code>Disabled</code> accounts list, you must contact AWS Support.</p> <p>After 90 days, deleted accounts are permanently removed from your <code>Disabled</code> accounts list.</p>
    fn delete_account(&self, input: DeleteAccountRequest) -> Request<DeleteAccountRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Deletes the events configuration that allows a bot to receive outgoing events.</p>
    fn delete_events_configuration(
        &self,
        input: DeleteEventsConfigurationRequest,
    ) -> Request<DeleteEventsConfigurationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Moves the specified phone number into the <b>Deletion queue</b>. A phone number must be disassociated from any users or Amazon Chime Voice Connectors before it can be deleted.</p> <p>Deleted phone numbers remain in the <b>Deletion queue</b> for 7 days before they are deleted permanently.</p>
    fn delete_phone_number(
        &self,
        input: DeletePhoneNumberRequest,
    ) -> Request<DeletePhoneNumberRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Deletes the specified Amazon Chime Voice Connector. Any phone numbers assigned to the Amazon Chime Voice Connector must be unassigned from it before it can be deleted.</p>
    fn delete_voice_connector(
        &self,
        input: DeleteVoiceConnectorRequest,
    ) -> Request<DeleteVoiceConnectorRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Deletes the origination settings for the specified Amazon Chime Voice Connector.</p>
    fn delete_voice_connector_origination(
        &self,
        input: DeleteVoiceConnectorOriginationRequest,
    ) -> Request<DeleteVoiceConnectorOriginationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Deletes the termination settings for the specified Amazon Chime Voice Connector.</p>
    fn delete_voice_connector_termination(
        &self,
        input: DeleteVoiceConnectorTerminationRequest,
    ) -> Request<DeleteVoiceConnectorTerminationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Deletes the specified SIP credentials used by your equipment to authenticate during call termination.</p>
    fn delete_voice_connector_termination_credentials(
        &self,
        input: DeleteVoiceConnectorTerminationCredentialsRequest,
    ) -> Request<DeleteVoiceConnectorTerminationCredentialsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Disassociates the primary provisioned phone number from the specified Amazon Chime user.</p>
    fn disassociate_phone_number_from_user(
        &self,
        input: DisassociatePhoneNumberFromUserRequest,
    ) -> Request<DisassociatePhoneNumberFromUserRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Disassociates the specified phone number from the specified Amazon Chime Voice Connector.</p>
    fn disassociate_phone_numbers_from_voice_connector(
        &self,
        input: DisassociatePhoneNumbersFromVoiceConnectorRequest,
    ) -> Request<DisassociatePhoneNumbersFromVoiceConnectorRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Retrieves details for the specified Amazon Chime account, such as account type and supported licenses.</p>
    fn get_account(&self, input: GetAccountRequest) -> Request<GetAccountRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Retrieves account settings for the specified Amazon Chime account ID, such as remote control and dial out settings. For more information about these settings, see <a href="https://docs.aws.amazon.com/chime/latest/ag/policies.html">Use the Policies Page</a> in the <i>Amazon Chime Administration Guide</i>.</p>
    fn get_account_settings(
        &self,
        input: GetAccountSettingsRequest,
    ) -> Request<GetAccountSettingsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Retrieves details for the specified bot, such as bot email address, bot type, status, and display name.</p>
    fn get_bot(&self, input: GetBotRequest) -> Request<GetBotRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Gets details for an events configuration that allows a bot to receive outgoing events, such as an HTTPS endpoint or Lambda function ARN. </p>
    fn get_events_configuration(
        &self,
        input: GetEventsConfigurationRequest,
    ) -> Request<GetEventsConfigurationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Retrieves global settings for the administrator's AWS account, such as Amazon Chime Business Calling and Amazon Chime Voice Connector settings.</p>
    fn get_global_settings(&self) -> Request<GetGlobalSettingsRequest> {
        Request::new(
            GetGlobalSettingsRequest {},
            self.region.clone(),
            self.client.clone(),
        )
    }

    /// <p>Retrieves details for the specified phone number ID, such as associations, capabilities, and product type.</p>
    fn get_phone_number(&self, input: GetPhoneNumberRequest) -> Request<GetPhoneNumberRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Retrieves details for the specified phone number order, such as order creation timestamp, phone numbers in E.164 format, product type, and order status.</p>
    fn get_phone_number_order(
        &self,
        input: GetPhoneNumberOrderRequest,
    ) -> Request<GetPhoneNumberOrderRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Retrieves details for the specified user ID, such as primary email address, license type, and personal meeting PIN.</p> <p>To retrieve user details with an email address instead of a user ID, use the <a>ListUsers</a> action, and then filter by email address.</p>
    fn get_user(&self, input: GetUserRequest) -> Request<GetUserRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Retrieves settings for the specified user ID, such as any associated phone number settings.</p>
    fn get_user_settings(&self, input: GetUserSettingsRequest) -> Request<GetUserSettingsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Retrieves details for the specified Amazon Chime Voice Connector, such as timestamps, name, outbound host, and encryption requirements.</p>
    fn get_voice_connector(
        &self,
        input: GetVoiceConnectorRequest,
    ) -> Request<GetVoiceConnectorRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Retrieves origination setting details for the specified Amazon Chime Voice Connector.</p>
    fn get_voice_connector_origination(
        &self,
        input: GetVoiceConnectorOriginationRequest,
    ) -> Request<GetVoiceConnectorOriginationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Retrieves termination setting details for the specified Amazon Chime Voice Connector.</p>
    fn get_voice_connector_termination(
        &self,
        input: GetVoiceConnectorTerminationRequest,
    ) -> Request<GetVoiceConnectorTerminationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Retrieves information about the last time a SIP <code>OPTIONS</code> ping was received from your SIP infrastructure for the specified Amazon Chime Voice Connector.</p>
    fn get_voice_connector_termination_health(
        &self,
        input: GetVoiceConnectorTerminationHealthRequest,
    ) -> Request<GetVoiceConnectorTerminationHealthRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Sends email invites to as many as 50 users, inviting them to the specified Amazon Chime <code>Team</code> account. Only <code>Team</code> account types are currently supported for this action. </p>
    fn invite_users(&self, input: InviteUsersRequest) -> Request<InviteUsersRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Lists the Amazon Chime accounts under the administrator's AWS account. You can filter accounts by account name prefix. To find out which Amazon Chime account a user belongs to, you can filter by the user's email address, which returns one account result.</p>
    fn list_accounts(&self, input: ListAccountsRequest) -> Request<ListAccountsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Lists the bots associated with the administrator's Amazon Chime Enterprise account ID.</p>
    fn list_bots(&self, input: ListBotsRequest) -> Request<ListBotsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Lists the phone number orders for the administrator's Amazon Chime account.</p>
    fn list_phone_number_orders(
        &self,
        input: ListPhoneNumberOrdersRequest,
    ) -> Request<ListPhoneNumberOrdersRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Lists the phone numbers for the specified Amazon Chime account, Amazon Chime user, or Amazon Chime Voice Connector.</p>
    fn list_phone_numbers(
        &self,
        input: ListPhoneNumbersRequest,
    ) -> Request<ListPhoneNumbersRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Lists the users that belong to the specified Amazon Chime account. You can specify an email address to list only the user that the email address belongs to.</p>
    fn list_users(&self, input: ListUsersRequest) -> Request<ListUsersRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Lists the SIP credentials for the specified Amazon Chime Voice Connector.</p>
    fn list_voice_connector_termination_credentials(
        &self,
        input: ListVoiceConnectorTerminationCredentialsRequest,
    ) -> Request<ListVoiceConnectorTerminationCredentialsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Lists the Amazon Chime Voice Connectors for the administrator's AWS account.</p>
    fn list_voice_connectors(
        &self,
        input: ListVoiceConnectorsRequest,
    ) -> Request<ListVoiceConnectorsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Logs out the specified user from all of the devices they are currently logged into.</p>
    fn logout_user(&self, input: LogoutUserRequest) -> Request<LogoutUserRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Creates an events configuration that allows a bot to receive outgoing events sent by Amazon Chime. Choose either an HTTPS endpoint or a Lambda function ARN. For more information, see <a>Bot</a>.</p>
    fn put_events_configuration(
        &self,
        input: PutEventsConfigurationRequest,
    ) -> Request<PutEventsConfigurationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Adds origination settings for the specified Amazon Chime Voice Connector.</p>
    fn put_voice_connector_origination(
        &self,
        input: PutVoiceConnectorOriginationRequest,
    ) -> Request<PutVoiceConnectorOriginationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Adds termination settings for the specified Amazon Chime Voice Connector.</p>
    fn put_voice_connector_termination(
        &self,
        input: PutVoiceConnectorTerminationRequest,
    ) -> Request<PutVoiceConnectorTerminationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Adds termination SIP credentials for the specified Amazon Chime Voice Connector.</p>
    fn put_voice_connector_termination_credentials(
        &self,
        input: PutVoiceConnectorTerminationCredentialsRequest,
    ) -> Request<PutVoiceConnectorTerminationCredentialsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Regenerates the security token for a bot.</p>
    fn regenerate_security_token(
        &self,
        input: RegenerateSecurityTokenRequest,
    ) -> Request<RegenerateSecurityTokenRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Resets the personal meeting PIN for the specified user on an Amazon Chime account. Returns the <a>User</a> object with the updated personal meeting PIN.</p>
    fn reset_personal_pin(
        &self,
        input: ResetPersonalPINRequest,
    ) -> Request<ResetPersonalPINRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Moves a phone number from the <b>Deletion queue</b> back into the phone number <b>Inventory</b>.</p>
    fn restore_phone_number(
        &self,
        input: RestorePhoneNumberRequest,
    ) -> Request<RestorePhoneNumberRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Searches phone numbers that can be ordered.</p>
    fn search_available_phone_numbers(
        &self,
        input: SearchAvailablePhoneNumbersRequest,
    ) -> Request<SearchAvailablePhoneNumbersRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Updates account details for the specified Amazon Chime account. Currently, only account name updates are supported for this action.</p>
    fn update_account(&self, input: UpdateAccountRequest) -> Request<UpdateAccountRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Updates the settings for the specified Amazon Chime account. You can update settings for remote control of shared screens, or for the dial-out option. For more information about these settings, see <a href="https://docs.aws.amazon.com/chime/latest/ag/policies.html">Use the Policies Page</a> in the <i>Amazon Chime Administration Guide</i>.</p>
    fn update_account_settings(
        &self,
        input: UpdateAccountSettingsRequest,
    ) -> Request<UpdateAccountSettingsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Updates the status of the specified bot, such as starting or stopping the bot from running in your Amazon Chime Enterprise account.</p>
    fn update_bot(&self, input: UpdateBotRequest) -> Request<UpdateBotRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Updates global settings for the administrator's AWS account, such as Amazon Chime Business Calling and Amazon Chime Voice Connector settings.</p>
    fn update_global_settings(
        &self,
        input: UpdateGlobalSettingsRequest,
    ) -> Request<UpdateGlobalSettingsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Updates phone number details, such as product type, for the specified phone number ID.</p>
    fn update_phone_number(
        &self,
        input: UpdatePhoneNumberRequest,
    ) -> Request<UpdatePhoneNumberRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Updates user details for a specified user ID. Currently, only <code>LicenseType</code> updates are supported for this action.</p>
    fn update_user(&self, input: UpdateUserRequest) -> Request<UpdateUserRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Updates the settings for the specified user, such as phone number settings.</p>
    fn update_user_settings(
        &self,
        input: UpdateUserSettingsRequest,
    ) -> Request<UpdateUserSettingsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Updates details for the specified Amazon Chime Voice Connector.</p>
    fn update_voice_connector(
        &self,
        input: UpdateVoiceConnectorRequest,
    ) -> Request<UpdateVoiceConnectorRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }
}

impl ServiceRequest for AssociatePhoneNumberWithUserRequest {
    type Output = AssociatePhoneNumberWithUserResponse;
    type Error = AssociatePhoneNumberWithUserError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/accounts/{account_id}/users/{user_id}",
            account_id = self.account_id,
            user_id = self.user_id
        );

        let mut request = SignedRequest::new("POST", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        params.put("operation", "associate-phone-number");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<AssociatePhoneNumberWithUserResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AssociatePhoneNumberWithUserError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for AssociatePhoneNumbersWithVoiceConnectorRequest {
    type Output = AssociatePhoneNumbersWithVoiceConnectorResponse;
    type Error = AssociatePhoneNumbersWithVoiceConnectorError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}",
            voice_connector_id = self.voice_connector_id
        );

        let mut request = SignedRequest::new("POST", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        params.put("operation", "associate-phone-numbers");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<AssociatePhoneNumbersWithVoiceConnectorResponse, _>(
                    )?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AssociatePhoneNumbersWithVoiceConnectorError::from_response(
                        response,
                    ))
                }))
            }
        })
    }
}

impl ServiceRequest for BatchDeletePhoneNumberRequest {
    type Output = BatchDeletePhoneNumberResponse;
    type Error = BatchDeletePhoneNumberError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = "/phone-numbers";

        let mut request = SignedRequest::new("POST", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        params.put("operation", "batch-delete");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<BatchDeletePhoneNumberResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(BatchDeletePhoneNumberError::from_response(response))
                    }),
                )
            }
        })
    }
}

impl ServiceRequest for BatchSuspendUserRequest {
    type Output = BatchSuspendUserResponse;
    type Error = BatchSuspendUserError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/accounts/{account_id}/users", account_id = self.account_id);

        let mut request = SignedRequest::new("POST", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        params.put("operation", "suspend");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
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
}

impl ServiceRequest for BatchUnsuspendUserRequest {
    type Output = BatchUnsuspendUserResponse;
    type Error = BatchUnsuspendUserError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/accounts/{account_id}/users", account_id = self.account_id);

        let mut request = SignedRequest::new("POST", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        params.put("operation", "unsuspend");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
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
}

impl ServiceRequest for BatchUpdatePhoneNumberRequest {
    type Output = BatchUpdatePhoneNumberResponse;
    type Error = BatchUpdatePhoneNumberError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = "/phone-numbers";

        let mut request = SignedRequest::new("POST", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        params.put("operation", "batch-update");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<BatchUpdatePhoneNumberResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(BatchUpdatePhoneNumberError::from_response(response))
                    }),
                )
            }
        })
    }
}

impl ServiceRequest for BatchUpdateUserRequest {
    type Output = BatchUpdateUserResponse;
    type Error = BatchUpdateUserError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/accounts/{account_id}/users", account_id = self.account_id);

        let mut request = SignedRequest::new("POST", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
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
}

impl ServiceRequest for CreateAccountRequest {
    type Output = CreateAccountResponse;
    type Error = CreateAccountError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = "/accounts";

        let mut request = SignedRequest::new("POST", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
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
}

impl ServiceRequest for CreateBotRequest {
    type Output = CreateBotResponse;
    type Error = CreateBotError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/accounts/{account_id}/bots", account_id = self.account_id);

        let mut request = SignedRequest::new("POST", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateBotResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateBotError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for CreatePhoneNumberOrderRequest {
    type Output = CreatePhoneNumberOrderResponse;
    type Error = CreatePhoneNumberOrderError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = "/phone-number-orders";

        let mut request = SignedRequest::new("POST", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreatePhoneNumberOrderResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreatePhoneNumberOrderError::from_response(response))
                    }),
                )
            }
        })
    }
}

impl ServiceRequest for CreateVoiceConnectorRequest {
    type Output = CreateVoiceConnectorResponse;
    type Error = CreateVoiceConnectorError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = "/voice-connectors";

        let mut request = SignedRequest::new("POST", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateVoiceConnectorResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateVoiceConnectorError::from_response(response))
                    }),
                )
            }
        })
    }
}

impl ServiceRequest for DeleteAccountRequest {
    type Output = DeleteAccountResponse;
    type Error = DeleteAccountError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/accounts/{account_id}", account_id = self.account_id);

        let mut request = SignedRequest::new("DELETE", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        dispatcher.dispatch(request, |response| {
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
}

impl ServiceRequest for DeleteEventsConfigurationRequest {
    type Output = DeleteEventsConfigurationResponse;
    type Error = DeleteEventsConfigurationError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/accounts/{account_id}/bots/{bot_id}/events-configuration",
            account_id = self.account_id,
            bot_id = self.bot_id
        );

        let mut request = SignedRequest::new("DELETE", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = DeleteEventsConfigurationResponse {};

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteEventsConfigurationError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for DeletePhoneNumberRequest {
    type Output = DeletePhoneNumberResponse;
    type Error = DeletePhoneNumberError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/phone-numbers/{phone_number_id}",
            phone_number_id = self.phone_number_id
        );

        let mut request = SignedRequest::new("DELETE", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = DeletePhoneNumberResponse {};

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeletePhoneNumberError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for DeleteVoiceConnectorRequest {
    type Output = DeleteVoiceConnectorResponse;
    type Error = DeleteVoiceConnectorError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}",
            voice_connector_id = self.voice_connector_id
        );

        let mut request = SignedRequest::new("DELETE", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = DeleteVoiceConnectorResponse {};

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteVoiceConnectorError::from_response(response))
                    }),
                )
            }
        })
    }
}

impl ServiceRequest for DeleteVoiceConnectorOriginationRequest {
    type Output = DeleteVoiceConnectorOriginationResponse;
    type Error = DeleteVoiceConnectorOriginationError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}/origination",
            voice_connector_id = self.voice_connector_id
        );

        let mut request = SignedRequest::new("DELETE", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = DeleteVoiceConnectorOriginationResponse {};

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteVoiceConnectorOriginationError::from_response(
                        response,
                    ))
                }))
            }
        })
    }
}

impl ServiceRequest for DeleteVoiceConnectorTerminationRequest {
    type Output = DeleteVoiceConnectorTerminationResponse;
    type Error = DeleteVoiceConnectorTerminationError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}/termination",
            voice_connector_id = self.voice_connector_id
        );

        let mut request = SignedRequest::new("DELETE", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = DeleteVoiceConnectorTerminationResponse {};

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteVoiceConnectorTerminationError::from_response(
                        response,
                    ))
                }))
            }
        })
    }
}

impl ServiceRequest for DeleteVoiceConnectorTerminationCredentialsRequest {
    type Output = DeleteVoiceConnectorTerminationCredentialsResponse;
    type Error = DeleteVoiceConnectorTerminationCredentialsError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}/termination/credentials",
            voice_connector_id = self.voice_connector_id
        );

        let mut request = SignedRequest::new("POST", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        params.put("operation", "delete");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = DeleteVoiceConnectorTerminationCredentialsResponse {};

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteVoiceConnectorTerminationCredentialsError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for DisassociatePhoneNumberFromUserRequest {
    type Output = DisassociatePhoneNumberFromUserResponse;
    type Error = DisassociatePhoneNumberFromUserError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/accounts/{account_id}/users/{user_id}",
            account_id = self.account_id,
            user_id = self.user_id
        );

        let mut request = SignedRequest::new("POST", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("operation", "disassociate-phone-number");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DisassociatePhoneNumberFromUserResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DisassociatePhoneNumberFromUserError::from_response(
                        response,
                    ))
                }))
            }
        })
    }
}

impl ServiceRequest for DisassociatePhoneNumbersFromVoiceConnectorRequest {
    type Output = DisassociatePhoneNumbersFromVoiceConnectorResponse;
    type Error = DisassociatePhoneNumbersFromVoiceConnectorError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}",
            voice_connector_id = self.voice_connector_id
        );

        let mut request = SignedRequest::new("POST", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        params.put("operation", "disassociate-phone-numbers");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DisassociatePhoneNumbersFromVoiceConnectorResponse, _>(
                    )?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DisassociatePhoneNumbersFromVoiceConnectorError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for GetAccountRequest {
    type Output = GetAccountResponse;
    type Error = GetAccountError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/accounts/{account_id}", account_id = self.account_id);

        let mut request = SignedRequest::new("GET", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        dispatcher.dispatch(request, |response| {
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
}

impl ServiceRequest for GetAccountSettingsRequest {
    type Output = GetAccountSettingsResponse;
    type Error = GetAccountSettingsError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/accounts/{account_id}/settings",
            account_id = self.account_id
        );

        let mut request = SignedRequest::new("GET", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        dispatcher.dispatch(request, |response| {
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
}

impl ServiceRequest for GetBotRequest {
    type Output = GetBotResponse;
    type Error = GetBotError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/accounts/{account_id}/bots/{bot_id}",
            account_id = self.account_id,
            bot_id = self.bot_id
        );

        let mut request = SignedRequest::new("GET", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetBotResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetBotError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for GetEventsConfigurationRequest {
    type Output = GetEventsConfigurationResponse;
    type Error = GetEventsConfigurationError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/accounts/{account_id}/bots/{bot_id}/events-configuration",
            account_id = self.account_id,
            bot_id = self.bot_id
        );

        let mut request = SignedRequest::new("GET", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetEventsConfigurationResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetEventsConfigurationError::from_response(response))
                    }),
                )
            }
        })
    }
}

impl ServiceRequest for GetGlobalSettingsRequest {
    type Output = GetGlobalSettingsResponse;
    type Error = GetGlobalSettingsError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = "/settings";

        let mut request = SignedRequest::new("GET", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetGlobalSettingsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetGlobalSettingsError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for GetPhoneNumberRequest {
    type Output = GetPhoneNumberResponse;
    type Error = GetPhoneNumberError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/phone-numbers/{phone_number_id}",
            phone_number_id = self.phone_number_id
        );

        let mut request = SignedRequest::new("GET", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetPhoneNumberResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetPhoneNumberError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for GetPhoneNumberOrderRequest {
    type Output = GetPhoneNumberOrderResponse;
    type Error = GetPhoneNumberOrderError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/phone-number-orders/{phone_number_order_id}",
            phone_number_order_id = self.phone_number_order_id
        );

        let mut request = SignedRequest::new("GET", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetPhoneNumberOrderResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetPhoneNumberOrderError::from_response(response))
                    }),
                )
            }
        })
    }
}

impl ServiceRequest for GetUserRequest {
    type Output = GetUserResponse;
    type Error = GetUserError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/accounts/{account_id}/users/{user_id}",
            account_id = self.account_id,
            user_id = self.user_id
        );

        let mut request = SignedRequest::new("GET", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        dispatcher.dispatch(request, |response| {
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
}

impl ServiceRequest for GetUserSettingsRequest {
    type Output = GetUserSettingsResponse;
    type Error = GetUserSettingsError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/accounts/{account_id}/users/{user_id}/settings",
            account_id = self.account_id,
            user_id = self.user_id
        );

        let mut request = SignedRequest::new("GET", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetUserSettingsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetUserSettingsError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for GetVoiceConnectorRequest {
    type Output = GetVoiceConnectorResponse;
    type Error = GetVoiceConnectorError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}",
            voice_connector_id = self.voice_connector_id
        );

        let mut request = SignedRequest::new("GET", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetVoiceConnectorResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetVoiceConnectorError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for GetVoiceConnectorOriginationRequest {
    type Output = GetVoiceConnectorOriginationResponse;
    type Error = GetVoiceConnectorOriginationError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}/origination",
            voice_connector_id = self.voice_connector_id
        );

        let mut request = SignedRequest::new("GET", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetVoiceConnectorOriginationResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetVoiceConnectorOriginationError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for GetVoiceConnectorTerminationRequest {
    type Output = GetVoiceConnectorTerminationResponse;
    type Error = GetVoiceConnectorTerminationError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}/termination",
            voice_connector_id = self.voice_connector_id
        );

        let mut request = SignedRequest::new("GET", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetVoiceConnectorTerminationResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetVoiceConnectorTerminationError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for GetVoiceConnectorTerminationHealthRequest {
    type Output = GetVoiceConnectorTerminationHealthResponse;
    type Error = GetVoiceConnectorTerminationHealthError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}/termination/health",
            voice_connector_id = self.voice_connector_id
        );

        let mut request = SignedRequest::new("GET", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetVoiceConnectorTerminationHealthResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetVoiceConnectorTerminationHealthError::from_response(
                        response,
                    ))
                }))
            }
        })
    }
}

impl ServiceRequest for InviteUsersRequest {
    type Output = InviteUsersResponse;
    type Error = InviteUsersError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/accounts/{account_id}/users", account_id = self.account_id);

        let mut request = SignedRequest::new("POST", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        params.put("operation", "add");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
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
}

impl ServiceRequest for ListAccountsRequest {
    type Output = ListAccountsResponse;
    type Error = ListAccountsError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = "/accounts";

        let mut request = SignedRequest::new("GET", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = self.max_results {
            params.put("max-results", x);
        }
        if let Some(ref x) = self.name {
            params.put("name", x);
        }
        if let Some(ref x) = self.next_token {
            params.put("next-token", x);
        }
        if let Some(ref x) = self.user_email {
            params.put("user-email", x);
        }
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
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
}

impl ServiceRequest for ListBotsRequest {
    type Output = ListBotsResponse;
    type Error = ListBotsError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/accounts/{account_id}/bots", account_id = self.account_id);

        let mut request = SignedRequest::new("GET", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = self.max_results {
            params.put("max-results", x);
        }
        if let Some(ref x) = self.next_token {
            params.put("next-token", x);
        }
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListBotsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListBotsError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for ListPhoneNumberOrdersRequest {
    type Output = ListPhoneNumberOrdersResponse;
    type Error = ListPhoneNumberOrdersError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = "/phone-number-orders";

        let mut request = SignedRequest::new("GET", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = self.max_results {
            params.put("max-results", x);
        }
        if let Some(ref x) = self.next_token {
            params.put("next-token", x);
        }
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListPhoneNumberOrdersResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListPhoneNumberOrdersError::from_response(response))
                    }),
                )
            }
        })
    }
}

impl ServiceRequest for ListPhoneNumbersRequest {
    type Output = ListPhoneNumbersResponse;
    type Error = ListPhoneNumbersError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = "/phone-numbers";

        let mut request = SignedRequest::new("GET", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = self.filter_name {
            params.put("filter-name", x);
        }
        if let Some(ref x) = self.filter_value {
            params.put("filter-value", x);
        }
        if let Some(ref x) = self.max_results {
            params.put("max-results", x);
        }
        if let Some(ref x) = self.next_token {
            params.put("next-token", x);
        }
        if let Some(ref x) = self.product_type {
            params.put("product-type", x);
        }
        if let Some(ref x) = self.status {
            params.put("status", x);
        }
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListPhoneNumbersResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListPhoneNumbersError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for ListUsersRequest {
    type Output = ListUsersResponse;
    type Error = ListUsersError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/accounts/{account_id}/users", account_id = self.account_id);

        let mut request = SignedRequest::new("GET", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = self.max_results {
            params.put("max-results", x);
        }
        if let Some(ref x) = self.next_token {
            params.put("next-token", x);
        }
        if let Some(ref x) = self.user_email {
            params.put("user-email", x);
        }
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
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
}

impl ServiceRequest for ListVoiceConnectorTerminationCredentialsRequest {
    type Output = ListVoiceConnectorTerminationCredentialsResponse;
    type Error = ListVoiceConnectorTerminationCredentialsError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}/termination/credentials",
            voice_connector_id = self.voice_connector_id
        );

        let mut request = SignedRequest::new("GET", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListVoiceConnectorTerminationCredentialsResponse, _>(
                    )?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListVoiceConnectorTerminationCredentialsError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for ListVoiceConnectorsRequest {
    type Output = ListVoiceConnectorsResponse;
    type Error = ListVoiceConnectorsError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = "/voice-connectors";

        let mut request = SignedRequest::new("GET", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = self.max_results {
            params.put("max-results", x);
        }
        if let Some(ref x) = self.next_token {
            params.put("next-token", x);
        }
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListVoiceConnectorsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListVoiceConnectorsError::from_response(response))
                    }),
                )
            }
        })
    }
}

impl ServiceRequest for LogoutUserRequest {
    type Output = LogoutUserResponse;
    type Error = LogoutUserError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/accounts/{account_id}/users/{user_id}",
            account_id = self.account_id,
            user_id = self.user_id
        );

        let mut request = SignedRequest::new("POST", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("operation", "logout");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
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
}

impl ServiceRequest for PutEventsConfigurationRequest {
    type Output = PutEventsConfigurationResponse;
    type Error = PutEventsConfigurationError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/accounts/{account_id}/bots/{bot_id}/events-configuration",
            account_id = self.account_id,
            bot_id = self.bot_id
        );

        let mut request = SignedRequest::new("PUT", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<PutEventsConfigurationResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(PutEventsConfigurationError::from_response(response))
                    }),
                )
            }
        })
    }
}

impl ServiceRequest for PutVoiceConnectorOriginationRequest {
    type Output = PutVoiceConnectorOriginationResponse;
    type Error = PutVoiceConnectorOriginationError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}/origination",
            voice_connector_id = self.voice_connector_id
        );

        let mut request = SignedRequest::new("PUT", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<PutVoiceConnectorOriginationResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutVoiceConnectorOriginationError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for PutVoiceConnectorTerminationRequest {
    type Output = PutVoiceConnectorTerminationResponse;
    type Error = PutVoiceConnectorTerminationError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}/termination",
            voice_connector_id = self.voice_connector_id
        );

        let mut request = SignedRequest::new("PUT", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<PutVoiceConnectorTerminationResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutVoiceConnectorTerminationError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for PutVoiceConnectorTerminationCredentialsRequest {
    type Output = PutVoiceConnectorTerminationCredentialsResponse;
    type Error = PutVoiceConnectorTerminationCredentialsError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}/termination/credentials",
            voice_connector_id = self.voice_connector_id
        );

        let mut request = SignedRequest::new("POST", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        params.put("operation", "put");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = PutVoiceConnectorTerminationCredentialsResponse {};

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutVoiceConnectorTerminationCredentialsError::from_response(
                        response,
                    ))
                }))
            }
        })
    }
}

impl ServiceRequest for RegenerateSecurityTokenRequest {
    type Output = RegenerateSecurityTokenResponse;
    type Error = RegenerateSecurityTokenError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/accounts/{account_id}/bots/{bot_id}",
            account_id = self.account_id,
            bot_id = self.bot_id
        );

        let mut request = SignedRequest::new("POST", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("operation", "regenerate-security-token");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<RegenerateSecurityTokenResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(RegenerateSecurityTokenError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for ResetPersonalPINRequest {
    type Output = ResetPersonalPINResponse;
    type Error = ResetPersonalPINError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/accounts/{account_id}/users/{user_id}",
            account_id = self.account_id,
            user_id = self.user_id
        );

        let mut request = SignedRequest::new("POST", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("operation", "reset-personal-pin");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
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
}

impl ServiceRequest for RestorePhoneNumberRequest {
    type Output = RestorePhoneNumberResponse;
    type Error = RestorePhoneNumberError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/phone-numbers/{phone_number_id}",
            phone_number_id = self.phone_number_id
        );

        let mut request = SignedRequest::new("POST", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("operation", "restore");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<RestorePhoneNumberResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(RestorePhoneNumberError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for SearchAvailablePhoneNumbersRequest {
    type Output = SearchAvailablePhoneNumbersResponse;
    type Error = SearchAvailablePhoneNumbersError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = "/search";

        let mut request = SignedRequest::new("GET", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = self.area_code {
            params.put("area-code", x);
        }
        if let Some(ref x) = self.city {
            params.put("city", x);
        }
        if let Some(ref x) = self.country {
            params.put("country", x);
        }
        if let Some(ref x) = self.max_results {
            params.put("max-results", x);
        }
        if let Some(ref x) = self.next_token {
            params.put("next-token", x);
        }
        if let Some(ref x) = self.state {
            params.put("state", x);
        }
        params.put("type", "phone-numbers");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<SearchAvailablePhoneNumbersResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(SearchAvailablePhoneNumbersError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for UpdateAccountRequest {
    type Output = UpdateAccountResponse;
    type Error = UpdateAccountError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/accounts/{account_id}", account_id = self.account_id);

        let mut request = SignedRequest::new("POST", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
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
}

impl ServiceRequest for UpdateAccountSettingsRequest {
    type Output = UpdateAccountSettingsResponse;
    type Error = UpdateAccountSettingsError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/accounts/{account_id}/settings",
            account_id = self.account_id
        );

        let mut request = SignedRequest::new("PUT", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
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
}

impl ServiceRequest for UpdateBotRequest {
    type Output = UpdateBotResponse;
    type Error = UpdateBotError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/accounts/{account_id}/bots/{bot_id}",
            account_id = self.account_id,
            bot_id = self.bot_id
        );

        let mut request = SignedRequest::new("POST", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateBotResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateBotError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for UpdateGlobalSettingsRequest {
    type Output = UpdateGlobalSettingsResponse;
    type Error = UpdateGlobalSettingsError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = "/settings";

        let mut request = SignedRequest::new("PUT", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = UpdateGlobalSettingsResponse {};

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateGlobalSettingsError::from_response(response))
                    }),
                )
            }
        })
    }
}

impl ServiceRequest for UpdatePhoneNumberRequest {
    type Output = UpdatePhoneNumberResponse;
    type Error = UpdatePhoneNumberError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/phone-numbers/{phone_number_id}",
            phone_number_id = self.phone_number_id
        );

        let mut request = SignedRequest::new("POST", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdatePhoneNumberResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdatePhoneNumberError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for UpdateUserRequest {
    type Output = UpdateUserResponse;
    type Error = UpdateUserError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/accounts/{account_id}/users/{user_id}",
            account_id = self.account_id,
            user_id = self.user_id
        );

        let mut request = SignedRequest::new("POST", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
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

impl ServiceRequest for UpdateUserSettingsRequest {
    type Output = UpdateUserSettingsResponse;
    type Error = UpdateUserSettingsError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/accounts/{account_id}/users/{user_id}/settings",
            account_id = self.account_id,
            user_id = self.user_id
        );

        let mut request = SignedRequest::new("PUT", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = UpdateUserSettingsResponse {};

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateUserSettingsError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for UpdateVoiceConnectorRequest {
    type Output = UpdateVoiceConnectorResponse;
    type Error = UpdateVoiceConnectorError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}",
            voice_connector_id = self.voice_connector_id
        );

        let mut request = SignedRequest::new("PUT", "chime", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateVoiceConnectorResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateVoiceConnectorError::from_response(response))
                    }),
                )
            }
        })
    }
}
