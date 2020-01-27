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
#[allow(warnings)]
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
/// <p>The Amazon Chime account details. An AWS account can have multiple Amazon Chime accounts.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The sign-in delegate groups associated with the account.</p>
    #[serde(rename = "SigninDelegateGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signin_delegate_groups: Option<Vec<SigninDelegateGroup>>,
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

/// <p>The Alexa for Business metadata associated with an Amazon Chime user, used to integrate Alexa for Business with a device.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlexaForBusinessMetadata {
    /// <p>The ARN of the room resource.</p>
    #[serde(rename = "AlexaForBusinessRoomArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alexa_for_business_room_arn: Option<String>,
    /// <p>Starts or stops Alexa for Business.</p>
    #[serde(rename = "IsAlexaForBusinessEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_alexa_for_business_enabled: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociatePhoneNumberWithUserResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociatePhoneNumbersWithVoiceConnectorGroupRequest {
    /// <p>List of phone numbers, in E.164 format.</p>
    #[serde(rename = "E164PhoneNumbers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e164_phone_numbers: Option<Vec<String>>,
    /// <p>If true, associates the provided phone numbers with the provided Amazon Chime Voice Connector Group and removes any previously existing associations. If false, does not associate any phone numbers that have previously existing associations.</p>
    #[serde(rename = "ForceAssociate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_associate: Option<bool>,
    /// <p>The Amazon Chime Voice Connector group ID.</p>
    #[serde(rename = "VoiceConnectorGroupId")]
    pub voice_connector_group_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociatePhoneNumbersWithVoiceConnectorGroupResponse {
    /// <p>If the action fails for one or more of the phone numbers in the request, a list of the phone numbers is returned, along with error codes and error messages.</p>
    #[serde(rename = "PhoneNumberErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_errors: Option<Vec<PhoneNumberError>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociatePhoneNumbersWithVoiceConnectorRequest {
    /// <p>List of phone numbers, in E.164 format.</p>
    #[serde(rename = "E164PhoneNumbers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e164_phone_numbers: Option<Vec<String>>,
    /// <p>If true, associates the provided phone numbers with the provided Amazon Chime Voice Connector and removes any previously existing associations. If false, does not associate any phone numbers that have previously existing associations.</p>
    #[serde(rename = "ForceAssociate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_associate: Option<bool>,
    /// <p>The Amazon Chime Voice Connector ID.</p>
    #[serde(rename = "VoiceConnectorId")]
    pub voice_connector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociatePhoneNumbersWithVoiceConnectorResponse {
    /// <p>If the action fails for one or more of the phone numbers in the request, a list of the phone numbers is returned, along with error codes and error messages.</p>
    #[serde(rename = "PhoneNumberErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_errors: Option<Vec<PhoneNumberError>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateSigninDelegateGroupsWithAccountRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The sign-in delegate groups.</p>
    #[serde(rename = "SigninDelegateGroups")]
    pub signin_delegate_groups: Vec<SigninDelegateGroup>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateSigninDelegateGroupsWithAccountResponse {}

/// <p>An Amazon Chime SDK meeting attendee. Includes a unique <code>AttendeeId</code> and <code>JoinToken</code>. The <code>JoinToken</code> allows a client to authenticate and join as the specified attendee. The <code>JoinToken</code> expires when the meeting ends or when <a>DeleteAttendee</a> is called. After that, the attendee is unable to join the meeting.</p> <p>We recommend securely transferring each <code>JoinToken</code> from your server application to the client so that no other client has access to the token except for the one authorized to represent the attendee.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Attendee {
    /// <p>The Amazon Chime SDK attendee ID.</p>
    #[serde(rename = "AttendeeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attendee_id: Option<String>,
    /// <p>The Amazon Chime SDK external user ID. Links the attendee to an identity managed by a builder application.</p>
    #[serde(rename = "ExternalUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_user_id: Option<String>,
    /// <p>The join token used by the Amazon Chime SDK attendee.</p>
    #[serde(rename = "JoinToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchCreateAttendeeRequest {
    /// <p>The request containing the attendees to create.</p>
    #[serde(rename = "Attendees")]
    pub attendees: Vec<CreateAttendeeRequestItem>,
    /// <p>The Amazon Chime SDK meeting ID.</p>
    #[serde(rename = "MeetingId")]
    pub meeting_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchCreateAttendeeResponse {
    /// <p>The attendee information, including attendees IDs and join tokens.</p>
    #[serde(rename = "Attendees")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attendees: Option<Vec<Attendee>>,
    /// <p>If the action fails for one or more of the attendees in the request, a list of the attendees is returned, along with error codes and error messages.</p>
    #[serde(rename = "Errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ChimeCreateAttendeeError>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchCreateRoomMembershipRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The list of membership items.</p>
    #[serde(rename = "MembershipItemList")]
    pub membership_item_list: Vec<MembershipItem>,
    /// <p>The room ID.</p>
    #[serde(rename = "RoomId")]
    pub room_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchCreateRoomMembershipResponse {
    /// <p>If the action fails for one or more of the member IDs in the request, a list of the member IDs is returned, along with error codes and error messages.</p>
    #[serde(rename = "Errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<MemberError>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchDeletePhoneNumberRequest {
    /// <p>List of phone number IDs.</p>
    #[serde(rename = "PhoneNumberIds")]
    pub phone_number_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchDeletePhoneNumberResponse {
    /// <p>If the action fails for one or more of the phone numbers in the request, a list of the phone numbers is returned, along with error codes and error messages.</p>
    #[serde(rename = "PhoneNumberErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_errors: Option<Vec<PhoneNumberError>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchSuspendUserRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The request containing the user IDs to suspend.</p>
    #[serde(rename = "UserIdList")]
    pub user_id_list: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchSuspendUserResponse {
    /// <p>If the <a>BatchSuspendUser</a> action fails for one or more of the user IDs in the request, a list of the user IDs is returned, along with error codes and error messages.</p>
    #[serde(rename = "UserErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_errors: Option<Vec<UserError>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchUnsuspendUserRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The request containing the user IDs to unsuspend.</p>
    #[serde(rename = "UserIdList")]
    pub user_id_list: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchUnsuspendUserResponse {
    /// <p>If the <a>BatchUnsuspendUser</a> action fails for one or more of the user IDs in the request, a list of the user IDs is returned, along with error codes and error messages.</p>
    #[serde(rename = "UserErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_errors: Option<Vec<UserError>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchUpdatePhoneNumberRequest {
    /// <p>The request containing the phone number IDs and product types or calling names to update.</p>
    #[serde(rename = "UpdatePhoneNumberRequestItems")]
    pub update_phone_number_request_items: Vec<UpdatePhoneNumberRequestItem>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchUpdatePhoneNumberResponse {
    /// <p>If the action fails for one or more of the phone numbers in the request, a list of the phone numbers is returned, along with error codes and error messages.</p>
    #[serde(rename = "PhoneNumberErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_errors: Option<Vec<PhoneNumberError>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchUpdateUserRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The request containing the user IDs and details to update.</p>
    #[serde(rename = "UpdateUserRequestItems")]
    pub update_user_request_items: Vec<UpdateUserRequestItem>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchUpdateUserResponse {
    /// <p>If the <a>BatchUpdateUser</a> action fails for one or more of the user IDs in the request, a list of the user IDs is returned, along with error codes and error messages.</p>
    #[serde(rename = "UserErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_errors: Option<Vec<UserError>>,
}

/// <p>A resource that allows Enterprise account administrators to configure an interface to receive events from Amazon Chime.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateAccountRequest {
    /// <p>The name of the Amazon Chime account.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateAccountResponse {
    /// <p>The Amazon Chime account details.</p>
    #[serde(rename = "Account")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<Account>,
}

/// <p>The list of errors returned when errors are encountered during the BatchCreateAttendee and CreateAttendee actions. This includes external user IDs, error codes, and error messages.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ChimeCreateAttendeeError {
    /// <p>The error code.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The error message.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The Amazon Chime SDK external user ID. Links the attendee to an identity managed by a builder application.</p>
    #[serde(rename = "ExternalUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_user_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateAttendeeRequest {
    /// <p>The Amazon Chime SDK external user ID. Links the attendee to an identity managed by a builder application.</p>
    #[serde(rename = "ExternalUserId")]
    pub external_user_id: String,
    /// <p>The Amazon Chime SDK meeting ID.</p>
    #[serde(rename = "MeetingId")]
    pub meeting_id: String,
}

/// <p>The Amazon Chime SDK attendee fields to create, used with the BatchCreateAttendee action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateAttendeeRequestItem {
    /// <p>The Amazon Chime SDK external user ID. Links the attendee to an identity managed by a builder application.</p>
    #[serde(rename = "ExternalUserId")]
    pub external_user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateAttendeeResponse {
    /// <p>The attendee information, including attendee ID and join token.</p>
    #[serde(rename = "Attendee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attendee: Option<Attendee>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateBotResponse {
    /// <p>The bot details.</p>
    #[serde(rename = "Bot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot: Option<Bot>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateMeetingRequest {
    /// <p>The unique identifier for the client request. Use a different token for different meetings.</p>
    #[serde(rename = "ClientRequestToken")]
    pub client_request_token: String,
    /// <p>The Region in which to create the meeting. Available values: <code>ap-northeast-1</code>, <code>ap-southeast-1</code>, <code>ap-southeast-2</code>, <code>ca-central-1</code>, <code>eu-central-1</code>, <code>eu-north-1</code>, <code>eu-west-1</code>, <code>eu-west-2</code>, <code>eu-west-3</code>, <code>sa-east-1</code>, <code>us-east-1</code>, <code>us-east-2</code>, <code>us-west-1</code>, <code>us-west-2</code>.</p>
    #[serde(rename = "MediaRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_region: Option<String>,
    /// <p>Reserved.</p>
    #[serde(rename = "MeetingHostId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_host_id: Option<String>,
    /// <p>The configuration for resource targets to receive notifications when meeting and attendee events occur.</p>
    #[serde(rename = "NotificationsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications_configuration: Option<MeetingNotificationConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateMeetingResponse {
    /// <p>The meeting information, including the meeting ID and <code>MediaPlacement</code>.</p>
    #[serde(rename = "Meeting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting: Option<Meeting>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreatePhoneNumberOrderRequest {
    /// <p>List of phone numbers, in E.164 format.</p>
    #[serde(rename = "E164PhoneNumbers")]
    pub e164_phone_numbers: Vec<String>,
    /// <p>The phone number product type.</p>
    #[serde(rename = "ProductType")]
    pub product_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreatePhoneNumberOrderResponse {
    /// <p>The phone number order details.</p>
    #[serde(rename = "PhoneNumberOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_order: Option<PhoneNumberOrder>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateRoomMembershipRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The Amazon Chime member ID (user ID or bot ID).</p>
    #[serde(rename = "MemberId")]
    pub member_id: String,
    /// <p>The role of the member.</p>
    #[serde(rename = "Role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// <p>The room ID.</p>
    #[serde(rename = "RoomId")]
    pub room_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateRoomMembershipResponse {
    /// <p>The room membership details.</p>
    #[serde(rename = "RoomMembership")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_membership: Option<RoomMembership>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateRoomRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The idempotency token for the request.</p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The room name.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateRoomResponse {
    /// <p>The room details.</p>
    #[serde(rename = "Room")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room: Option<Room>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateUserRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The user's email address.</p>
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// <p>The user type.</p>
    #[serde(rename = "UserType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<String>,
    /// <p>The user name.</p>
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateUserResponse {
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateVoiceConnectorGroupRequest {
    /// <p>The name of the Amazon Chime Voice Connector group.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The Amazon Chime Voice Connectors to route inbound calls to.</p>
    #[serde(rename = "VoiceConnectorItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_connector_items: Option<Vec<VoiceConnectorItem>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateVoiceConnectorGroupResponse {
    /// <p>The Amazon Chime Voice Connector group details.</p>
    #[serde(rename = "VoiceConnectorGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_connector_group: Option<VoiceConnectorGroup>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateVoiceConnectorRequest {
    /// <p>The AWS Region in which the Amazon Chime Voice Connector is created. Default value: <code>us-east-1</code>.</p>
    #[serde(rename = "AwsRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
    /// <p>The name of the Amazon Chime Voice Connector.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>When enabled, requires encryption for the Amazon Chime Voice Connector.</p>
    #[serde(rename = "RequireEncryption")]
    pub require_encryption: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateVoiceConnectorResponse {
    /// <p>The Amazon Chime Voice Connector details.</p>
    #[serde(rename = "VoiceConnector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_connector: Option<VoiceConnector>,
}

/// <p>The SIP credentials used to authenticate requests to your Amazon Chime Voice Connector.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteAccountRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteAccountResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteAttendeeRequest {
    /// <p>The Amazon Chime SDK attendee ID.</p>
    #[serde(rename = "AttendeeId")]
    pub attendee_id: String,
    /// <p>The Amazon Chime SDK meeting ID.</p>
    #[serde(rename = "MeetingId")]
    pub meeting_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteEventsConfigurationRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The bot ID.</p>
    #[serde(rename = "BotId")]
    pub bot_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteMeetingRequest {
    /// <p>The Amazon Chime SDK meeting ID.</p>
    #[serde(rename = "MeetingId")]
    pub meeting_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeletePhoneNumberRequest {
    /// <p>The phone number ID.</p>
    #[serde(rename = "PhoneNumberId")]
    pub phone_number_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteRoomMembershipRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The member ID (user ID or bot ID).</p>
    #[serde(rename = "MemberId")]
    pub member_id: String,
    /// <p>The room ID.</p>
    #[serde(rename = "RoomId")]
    pub room_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteRoomRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The chat room ID.</p>
    #[serde(rename = "RoomId")]
    pub room_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteVoiceConnectorGroupRequest {
    /// <p>The Amazon Chime Voice Connector group ID.</p>
    #[serde(rename = "VoiceConnectorGroupId")]
    pub voice_connector_group_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteVoiceConnectorOriginationRequest {
    /// <p>The Amazon Chime Voice Connector ID.</p>
    #[serde(rename = "VoiceConnectorId")]
    pub voice_connector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteVoiceConnectorRequest {
    /// <p>The Amazon Chime Voice Connector ID.</p>
    #[serde(rename = "VoiceConnectorId")]
    pub voice_connector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteVoiceConnectorStreamingConfigurationRequest {
    /// <p>The Amazon Chime Voice Connector ID.</p>
    #[serde(rename = "VoiceConnectorId")]
    pub voice_connector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteVoiceConnectorTerminationCredentialsRequest {
    /// <p>The RFC2617 compliant username associated with the SIP credentials, in US-ASCII format.</p>
    #[serde(rename = "Usernames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usernames: Option<Vec<String>>,
    /// <p>The Amazon Chime Voice Connector ID.</p>
    #[serde(rename = "VoiceConnectorId")]
    pub voice_connector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteVoiceConnectorTerminationRequest {
    /// <p>The Amazon Chime Voice Connector ID.</p>
    #[serde(rename = "VoiceConnectorId")]
    pub voice_connector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociatePhoneNumberFromUserRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The user ID.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociatePhoneNumberFromUserResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociatePhoneNumbersFromVoiceConnectorGroupRequest {
    /// <p>List of phone numbers, in E.164 format.</p>
    #[serde(rename = "E164PhoneNumbers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e164_phone_numbers: Option<Vec<String>>,
    /// <p>The Amazon Chime Voice Connector group ID.</p>
    #[serde(rename = "VoiceConnectorGroupId")]
    pub voice_connector_group_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociatePhoneNumbersFromVoiceConnectorGroupResponse {
    /// <p>If the action fails for one or more of the phone numbers in the request, a list of the phone numbers is returned, along with error codes and error messages.</p>
    #[serde(rename = "PhoneNumberErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_errors: Option<Vec<PhoneNumberError>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociatePhoneNumbersFromVoiceConnectorResponse {
    /// <p>If the action fails for one or more of the phone numbers in the request, a list of the phone numbers is returned, along with error codes and error messages.</p>
    #[serde(rename = "PhoneNumberErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_errors: Option<Vec<PhoneNumberError>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateSigninDelegateGroupsFromAccountRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The sign-in delegate group names.</p>
    #[serde(rename = "GroupNames")]
    pub group_names: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateSigninDelegateGroupsFromAccountResponse {}

/// <p>The configuration that allows a bot to receive outgoing events. Can be either an HTTPS endpoint or a Lambda function ARN.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAccountRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAccountResponse {
    /// <p>The Amazon Chime account details.</p>
    #[serde(rename = "Account")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<Account>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAccountSettingsRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAccountSettingsResponse {
    /// <p>The Amazon Chime account settings.</p>
    #[serde(rename = "AccountSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_settings: Option<AccountSettings>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAttendeeRequest {
    /// <p>The Amazon Chime SDK attendee ID.</p>
    #[serde(rename = "AttendeeId")]
    pub attendee_id: String,
    /// <p>The Amazon Chime SDK meeting ID.</p>
    #[serde(rename = "MeetingId")]
    pub meeting_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAttendeeResponse {
    /// <p>The Amazon Chime SDK attendee information.</p>
    #[serde(rename = "Attendee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attendee: Option<Attendee>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetBotRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The bot ID.</p>
    #[serde(rename = "BotId")]
    pub bot_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetBotResponse {
    /// <p>The chat bot details.</p>
    #[serde(rename = "Bot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot: Option<Bot>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetEventsConfigurationRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The bot ID.</p>
    #[serde(rename = "BotId")]
    pub bot_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetEventsConfigurationResponse {
    /// <p>The events configuration details.</p>
    #[serde(rename = "EventsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events_configuration: Option<EventsConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetMeetingRequest {
    /// <p>The Amazon Chime SDK meeting ID.</p>
    #[serde(rename = "MeetingId")]
    pub meeting_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetMeetingResponse {
    /// <p>The Amazon Chime SDK meeting information.</p>
    #[serde(rename = "Meeting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting: Option<Meeting>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetPhoneNumberOrderRequest {
    /// <p>The ID for the phone number order.</p>
    #[serde(rename = "PhoneNumberOrderId")]
    pub phone_number_order_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetPhoneNumberOrderResponse {
    /// <p>The phone number order details.</p>
    #[serde(rename = "PhoneNumberOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_order: Option<PhoneNumberOrder>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetPhoneNumberRequest {
    /// <p>The phone number ID.</p>
    #[serde(rename = "PhoneNumberId")]
    pub phone_number_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetPhoneNumberResponse {
    /// <p>The phone number details.</p>
    #[serde(rename = "PhoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<PhoneNumber>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetPhoneNumberSettingsResponse {
    /// <p>The default outbound calling name for the account.</p>
    #[serde(rename = "CallingName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calling_name: Option<String>,
    /// <p>The updated outbound calling name timestamp, in ISO 8601 format.</p>
    #[serde(rename = "CallingNameUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calling_name_updated_timestamp: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRoomRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The room ID.</p>
    #[serde(rename = "RoomId")]
    pub room_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRoomResponse {
    /// <p>The room details.</p>
    #[serde(rename = "Room")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room: Option<Room>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetUserRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The user ID.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetUserResponse {
    /// <p>The user details.</p>
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetUserSettingsRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The user ID.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetUserSettingsResponse {
    /// <p>The user settings.</p>
    #[serde(rename = "UserSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_settings: Option<UserSettings>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetVoiceConnectorGroupRequest {
    /// <p>The Amazon Chime Voice Connector group ID.</p>
    #[serde(rename = "VoiceConnectorGroupId")]
    pub voice_connector_group_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetVoiceConnectorGroupResponse {
    /// <p>The Amazon Chime Voice Connector group details.</p>
    #[serde(rename = "VoiceConnectorGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_connector_group: Option<VoiceConnectorGroup>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetVoiceConnectorLoggingConfigurationRequest {
    /// <p>The Amazon Chime Voice Connector ID.</p>
    #[serde(rename = "VoiceConnectorId")]
    pub voice_connector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetVoiceConnectorLoggingConfigurationResponse {
    /// <p>The logging configuration details.</p>
    #[serde(rename = "LoggingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_configuration: Option<LoggingConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetVoiceConnectorOriginationRequest {
    /// <p>The Amazon Chime Voice Connector ID.</p>
    #[serde(rename = "VoiceConnectorId")]
    pub voice_connector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetVoiceConnectorOriginationResponse {
    /// <p>The origination setting details.</p>
    #[serde(rename = "Origination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origination: Option<Origination>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetVoiceConnectorRequest {
    /// <p>The Amazon Chime Voice Connector ID.</p>
    #[serde(rename = "VoiceConnectorId")]
    pub voice_connector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetVoiceConnectorResponse {
    /// <p>The Amazon Chime Voice Connector details.</p>
    #[serde(rename = "VoiceConnector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_connector: Option<VoiceConnector>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetVoiceConnectorStreamingConfigurationRequest {
    /// <p>The Amazon Chime Voice Connector ID.</p>
    #[serde(rename = "VoiceConnectorId")]
    pub voice_connector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetVoiceConnectorStreamingConfigurationResponse {
    /// <p>The streaming configuration details.</p>
    #[serde(rename = "StreamingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streaming_configuration: Option<StreamingConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetVoiceConnectorTerminationHealthRequest {
    /// <p>The Amazon Chime Voice Connector ID.</p>
    #[serde(rename = "VoiceConnectorId")]
    pub voice_connector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetVoiceConnectorTerminationHealthResponse {
    /// <p>The termination health details.</p>
    #[serde(rename = "TerminationHealth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_health: Option<TerminationHealth>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetVoiceConnectorTerminationRequest {
    /// <p>The Amazon Chime Voice Connector ID.</p>
    #[serde(rename = "VoiceConnectorId")]
    pub voice_connector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetVoiceConnectorTerminationResponse {
    /// <p>The termination setting details.</p>
    #[serde(rename = "Termination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination: Option<Termination>,
}

/// <p>Invitation object returned after emailing users to invite them to join the Amazon Chime <code>Team</code> account.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InviteUsersRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The user email addresses to which to send the email invitation.</p>
    #[serde(rename = "UserEmailList")]
    pub user_email_list: Vec<String>,
    /// <p>The user type.</p>
    #[serde(rename = "UserType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InviteUsersResponse {
    /// <p>The email invitation details.</p>
    #[serde(rename = "Invites")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invites: Option<Vec<Invite>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListAttendeesRequest {
    /// <p>The maximum number of results to return in a single call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The Amazon Chime SDK meeting ID.</p>
    #[serde(rename = "MeetingId")]
    pub meeting_id: String,
    /// <p>The token to use to retrieve the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListAttendeesResponse {
    /// <p>The Amazon Chime SDK attendee information.</p>
    #[serde(rename = "Attendees")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attendees: Option<Vec<Attendee>>,
    /// <p>The token to use to retrieve the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListBotsRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The maximum number of results to return in a single call. The default is 10.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to use to retrieve the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListMeetingsRequest {
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListMeetingsResponse {
    /// <p>The Amazon Chime SDK meeting information.</p>
    #[serde(rename = "Meetings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meetings: Option<Vec<Meeting>>,
    /// <p>The token to use to retrieve the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListRoomMembershipsRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The maximum number of results to return in a single call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to use to retrieve the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The room ID.</p>
    #[serde(rename = "RoomId")]
    pub room_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListRoomMembershipsResponse {
    /// <p>The token to use to retrieve the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The room membership details.</p>
    #[serde(rename = "RoomMemberships")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_memberships: Option<Vec<RoomMembership>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListRoomsRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The maximum number of results to return in a single call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The member ID (user ID or bot ID).</p>
    #[serde(rename = "MemberId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// <p>The token to use to retrieve the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListRoomsResponse {
    /// <p>The token to use to retrieve the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The room details.</p>
    #[serde(rename = "Rooms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rooms: Option<Vec<Room>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>The user type.</p>
    #[serde(rename = "UserType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListVoiceConnectorGroupsRequest {
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListVoiceConnectorGroupsResponse {
    /// <p>The token to use to retrieve the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The details of the Amazon Chime Voice Connector groups.</p>
    #[serde(rename = "VoiceConnectorGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_connector_groups: Option<Vec<VoiceConnectorGroup>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListVoiceConnectorTerminationCredentialsRequest {
    /// <p>The Amazon Chime Voice Connector ID.</p>
    #[serde(rename = "VoiceConnectorId")]
    pub voice_connector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListVoiceConnectorTerminationCredentialsResponse {
    /// <p>A list of user names.</p>
    #[serde(rename = "Usernames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usernames: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>The logging configuration associated with an Amazon Chime Voice Connector. Specifies whether SIP message logs are enabled for sending to Amazon CloudWatch Logs.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoggingConfiguration {
    /// <p>When true, enables SIP message logs for sending to Amazon CloudWatch Logs.</p>
    #[serde(rename = "EnableSIPLogs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_sip_logs: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct LogoutUserRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The user ID.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LogoutUserResponse {}

/// <p>A set of endpoints used by clients to connect to the media service group for a Amazon Chime SDK meeting.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MediaPlacement {
    /// <p>The audio host URL.</p>
    #[serde(rename = "AudioHostUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_host_url: Option<String>,
    /// <p>The screen data URL.</p>
    #[serde(rename = "ScreenDataUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_data_url: Option<String>,
    /// <p>The screen sharing URL.</p>
    #[serde(rename = "ScreenSharingUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_sharing_url: Option<String>,
    /// <p>The screen viewing URL.</p>
    #[serde(rename = "ScreenViewingUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_viewing_url: Option<String>,
    /// <p>The signaling URL.</p>
    #[serde(rename = "SignalingUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signaling_url: Option<String>,
    /// <p>The turn control URL.</p>
    #[serde(rename = "TurnControlUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_control_url: Option<String>,
}

/// <p>A meeting created using the Amazon Chime SDK.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Meeting {
    /// <p>The media placement for the meeting.</p>
    #[serde(rename = "MediaPlacement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_placement: Option<MediaPlacement>,
    /// <p>The Region in which to create the meeting. Available values: <code>ap-northeast-1</code>, <code>ap-southeast-1</code>, <code>ap-southeast-2</code>, <code>ca-central-1</code>, <code>eu-central-1</code>, <code>eu-north-1</code>, <code>eu-west-1</code>, <code>eu-west-2</code>, <code>eu-west-3</code>, <code>sa-east-1</code>, <code>us-east-1</code>, <code>us-east-2</code>, <code>us-west-1</code>, <code>us-west-2</code>.</p>
    #[serde(rename = "MediaRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_region: Option<String>,
    /// <p>The Amazon Chime SDK meeting ID.</p>
    #[serde(rename = "MeetingId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_id: Option<String>,
}

/// <p>The configuration for resource targets to receive notifications when Amazon Chime SDK meeting and attendee events occur.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct MeetingNotificationConfiguration {
    /// <p>The SNS topic ARN.</p>
    #[serde(rename = "SnsTopicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
    /// <p>The SQS queue ARN.</p>
    #[serde(rename = "SqsQueueArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sqs_queue_arn: Option<String>,
}

/// <p>The member details, such as email address, name, member ID, and member type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Member {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The member email address.</p>
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// <p>The member name.</p>
    #[serde(rename = "FullName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    /// <p>The member ID (user ID or bot ID).</p>
    #[serde(rename = "MemberId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// <p>The member type.</p>
    #[serde(rename = "MemberType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
}

/// <p>The list of errors returned when a member action results in an error.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MemberError {
    /// <p>The error code.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The error message.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The member ID.</p>
    #[serde(rename = "MemberId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
}

/// <p>Membership details, such as member ID and member role.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct MembershipItem {
    /// <p>The member ID.</p>
    #[serde(rename = "MemberId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// <p>The member role.</p>
    #[serde(rename = "Role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

/// <p>A phone number for which an order has been placed.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>Origination routes define call distribution properties for your SIP hosts to receive inbound calls using your Amazon Chime Voice Connector. Limit: Ten origination routes for each Amazon Chime Voice Connector.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OriginationRoute {
    /// <p>The FQDN or IP address to contact for origination traffic.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PhoneNumber {
    /// <p>The phone number associations.</p>
    #[serde(rename = "Associations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associations: Option<Vec<PhoneNumberAssociation>>,
    /// <p>The outbound calling name associated with the phone number.</p>
    #[serde(rename = "CallingName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calling_name: Option<String>,
    /// <p>The outbound calling name status.</p>
    #[serde(rename = "CallingNameStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calling_name_status: Option<String>,
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
    /// <p>The phone number type.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The updated phone number timestamp, in ISO 8601 format.</p>
    #[serde(rename = "UpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<f64>,
}

/// <p>The phone number associations, such as Amazon Chime account ID, Amazon Chime user ID, Amazon Chime Voice Connector ID, or Amazon Chime Voice Connector group ID.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PhoneNumberAssociation {
    /// <p>The timestamp of the phone number association, in ISO 8601 format.</p>
    #[serde(rename = "AssociatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_timestamp: Option<f64>,
    /// <p>Defines the association with an Amazon Chime account ID, user ID, Amazon Chime Voice Connector ID, or Amazon Chime Voice Connector group ID.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Contains the ID for the entity specified in Name.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>The phone number capabilities for Amazon Chime Business Calling phone numbers, such as enabled inbound and outbound calling and text messaging.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutEventsConfigurationResponse {
    #[serde(rename = "EventsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events_configuration: Option<EventsConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutVoiceConnectorLoggingConfigurationRequest {
    /// <p>The logging configuration details to add.</p>
    #[serde(rename = "LoggingConfiguration")]
    pub logging_configuration: LoggingConfiguration,
    /// <p>The Amazon Chime Voice Connector ID.</p>
    #[serde(rename = "VoiceConnectorId")]
    pub voice_connector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutVoiceConnectorLoggingConfigurationResponse {
    /// <p>The updated logging configuration details.</p>
    #[serde(rename = "LoggingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_configuration: Option<LoggingConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutVoiceConnectorOriginationRequest {
    /// <p>The origination setting details to add.</p>
    #[serde(rename = "Origination")]
    pub origination: Origination,
    /// <p>The Amazon Chime Voice Connector ID.</p>
    #[serde(rename = "VoiceConnectorId")]
    pub voice_connector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutVoiceConnectorOriginationResponse {
    /// <p>The updated origination setting details.</p>
    #[serde(rename = "Origination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origination: Option<Origination>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutVoiceConnectorStreamingConfigurationRequest {
    /// <p>The streaming configuration details to add.</p>
    #[serde(rename = "StreamingConfiguration")]
    pub streaming_configuration: StreamingConfiguration,
    /// <p>The Amazon Chime Voice Connector ID.</p>
    #[serde(rename = "VoiceConnectorId")]
    pub voice_connector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutVoiceConnectorStreamingConfigurationResponse {
    /// <p>The updated streaming configuration details.</p>
    #[serde(rename = "StreamingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streaming_configuration: Option<StreamingConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutVoiceConnectorTerminationCredentialsRequest {
    /// <p>The termination SIP credentials.</p>
    #[serde(rename = "Credentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Vec<Credential>>,
    /// <p>The Amazon Chime Voice Connector ID.</p>
    #[serde(rename = "VoiceConnectorId")]
    pub voice_connector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutVoiceConnectorTerminationRequest {
    /// <p>The termination setting details to add.</p>
    #[serde(rename = "Termination")]
    pub termination: Termination,
    /// <p>The Amazon Chime Voice Connector ID.</p>
    #[serde(rename = "VoiceConnectorId")]
    pub voice_connector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutVoiceConnectorTerminationResponse {
    /// <p>The updated termination setting details.</p>
    #[serde(rename = "Termination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination: Option<Termination>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RegenerateSecurityTokenRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The bot ID.</p>
    #[serde(rename = "BotId")]
    pub bot_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RegenerateSecurityTokenResponse {
    #[serde(rename = "Bot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot: Option<Bot>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ResetPersonalPINRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The user ID.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResetPersonalPINResponse {
    /// <p>The user details and new personal meeting PIN.</p>
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RestorePhoneNumberRequest {
    /// <p>The phone number.</p>
    #[serde(rename = "PhoneNumberId")]
    pub phone_number_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RestorePhoneNumberResponse {
    /// <p>The phone number details.</p>
    #[serde(rename = "PhoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<PhoneNumber>,
}

/// <p>The Amazon Chime chat room details.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Room {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The identifier of the room creator.</p>
    #[serde(rename = "CreatedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// <p>The room creation timestamp, in ISO 8601 format.</p>
    #[serde(rename = "CreatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    /// <p>The room name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The room ID.</p>
    #[serde(rename = "RoomId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_id: Option<String>,
    /// <p>The room update timestamp, in ISO 8601 format.</p>
    #[serde(rename = "UpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<f64>,
}

/// <p>The room membership details.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RoomMembership {
    /// <p>The identifier of the user that invited the room member.</p>
    #[serde(rename = "InvitedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_by: Option<String>,
    #[serde(rename = "Member")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<Member>,
    /// <p>The membership role.</p>
    #[serde(rename = "Role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// <p>The room ID.</p>
    #[serde(rename = "RoomId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_id: Option<String>,
    /// <p>The room membership update timestamp, in ISO 8601 format.</p>
    #[serde(rename = "UpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>The toll-free prefix that you use to filter results.</p>
    #[serde(rename = "TollFreePrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toll_free_prefix: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SearchAvailablePhoneNumbersResponse {
    /// <p>List of phone numbers, in E.164 format.</p>
    #[serde(rename = "E164PhoneNumbers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e164_phone_numbers: Option<Vec<String>>,
}

/// <p>An Active Directory (AD) group whose members are granted permission to act as delegates.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SigninDelegateGroup {
    /// <p>The group name.</p>
    #[serde(rename = "GroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}

/// <p>The streaming configuration associated with an Amazon Chime Voice Connector. Specifies whether media streaming is enabled for sending to Amazon Kinesis, and shows the retention period for the Amazon Kinesis data, in hours.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamingConfiguration {
    /// <p>The retention period, in hours, for the Amazon Kinesis data.</p>
    #[serde(rename = "DataRetentionInHours")]
    pub data_retention_in_hours: i64,
    /// <p>When true, media streaming to Amazon Kinesis is turned off.</p>
    #[serde(rename = "Disabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
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
    /// <p>The countries to which calls are allowed, in ISO 3166-1 alpha-2 format. Required.</p>
    #[serde(rename = "CallingRegions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calling_regions: Option<Vec<String>>,
    /// <p>The IP addresses allowed to make calls, in CIDR format. Required.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateAccountResponse {
    /// <p>The updated Amazon Chime account details.</p>
    #[serde(rename = "Account")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<Account>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateAccountSettingsRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The Amazon Chime account settings to update.</p>
    #[serde(rename = "AccountSettings")]
    pub account_settings: AccountSettings,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateAccountSettingsResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateBotResponse {
    /// <p>The updated bot details.</p>
    #[serde(rename = "Bot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot: Option<Bot>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateGlobalSettingsRequest {
    /// <p>The Amazon Chime Business Calling settings.</p>
    #[serde(rename = "BusinessCalling")]
    pub business_calling: BusinessCallingSettings,
    /// <p>The Amazon Chime Voice Connector settings.</p>
    #[serde(rename = "VoiceConnector")]
    pub voice_connector: VoiceConnectorSettings,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdatePhoneNumberRequest {
    /// <p>The outbound calling name associated with the phone number.</p>
    #[serde(rename = "CallingName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calling_name: Option<String>,
    /// <p>The phone number ID.</p>
    #[serde(rename = "PhoneNumberId")]
    pub phone_number_id: String,
    /// <p>The product type.</p>
    #[serde(rename = "ProductType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<String>,
}

/// <p>The phone number ID, product type, or calling name fields to update, used with the <a>BatchUpdatePhoneNumber</a> and <a>UpdatePhoneNumber</a> actions.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdatePhoneNumberRequestItem {
    /// <p>The outbound calling name to update.</p>
    #[serde(rename = "CallingName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calling_name: Option<String>,
    /// <p>The phone number ID to update.</p>
    #[serde(rename = "PhoneNumberId")]
    pub phone_number_id: String,
    /// <p>The product type to update.</p>
    #[serde(rename = "ProductType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdatePhoneNumberResponse {
    /// <p>The updated phone number details.</p>
    #[serde(rename = "PhoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<PhoneNumber>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdatePhoneNumberSettingsRequest {
    /// <p>The default outbound calling name for the account.</p>
    #[serde(rename = "CallingName")]
    pub calling_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateRoomMembershipRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The member ID.</p>
    #[serde(rename = "MemberId")]
    pub member_id: String,
    /// <p>The role of the member.</p>
    #[serde(rename = "Role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// <p>The room ID.</p>
    #[serde(rename = "RoomId")]
    pub room_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateRoomMembershipResponse {
    /// <p>The room membership details.</p>
    #[serde(rename = "RoomMembership")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_membership: Option<RoomMembership>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateRoomRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The room name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The room ID.</p>
    #[serde(rename = "RoomId")]
    pub room_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateRoomResponse {
    /// <p>The room details.</p>
    #[serde(rename = "Room")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room: Option<Room>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateUserRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The Alexa for Business metadata.</p>
    #[serde(rename = "AlexaForBusinessMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alexa_for_business_metadata: Option<AlexaForBusinessMetadata>,
    /// <p>The user license type to update. This must be a supported license type for the Amazon Chime account that the user belongs to.</p>
    #[serde(rename = "LicenseType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_type: Option<String>,
    /// <p>The user ID.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
    /// <p>The user type.</p>
    #[serde(rename = "UserType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<String>,
}

/// <p>The user ID and user fields to update, used with the <a>BatchUpdateUser</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateUserRequestItem {
    /// <p>The Alexa for Business metadata.</p>
    #[serde(rename = "AlexaForBusinessMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alexa_for_business_metadata: Option<AlexaForBusinessMetadata>,
    /// <p>The user license type.</p>
    #[serde(rename = "LicenseType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_type: Option<String>,
    /// <p>The user ID.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
    /// <p>The user type.</p>
    #[serde(rename = "UserType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateUserResponse {
    /// <p>The updated user details.</p>
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateVoiceConnectorGroupRequest {
    /// <p>The name of the Amazon Chime Voice Connector group.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The Amazon Chime Voice Connector group ID.</p>
    #[serde(rename = "VoiceConnectorGroupId")]
    pub voice_connector_group_id: String,
    /// <p>The <code>VoiceConnectorItems</code> to associate with the group.</p>
    #[serde(rename = "VoiceConnectorItems")]
    pub voice_connector_items: Vec<VoiceConnectorItem>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateVoiceConnectorGroupResponse {
    /// <p>The updated Amazon Chime Voice Connector group details.</p>
    #[serde(rename = "VoiceConnectorGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_connector_group: Option<VoiceConnectorGroup>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateVoiceConnectorResponse {
    /// <p>The updated Amazon Chime Voice Connector details.</p>
    #[serde(rename = "VoiceConnector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_connector: Option<VoiceConnector>,
}

/// <p>The user on the Amazon Chime account.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct User {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The Alexa for Business metadata.</p>
    #[serde(rename = "AlexaForBusinessMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alexa_for_business_metadata: Option<AlexaForBusinessMetadata>,
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
    /// <p>The user type.</p>
    #[serde(rename = "UserType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<String>,
}

/// <p>The list of errors returned when errors are encountered during the <a>BatchSuspendUser</a>, <a>BatchUnsuspendUser</a>, or <a>BatchUpdateUser</a> actions. This includes user IDs, error codes, and error messages.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VoiceConnector {
    /// <p>The AWS Region in which the Amazon Chime Voice Connector is created. Default: <code>us-east-1</code>.</p>
    #[serde(rename = "AwsRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
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

/// <p>The Amazon Chime Voice Connector group configuration, including associated Amazon Chime Voice Connectors. You can include Amazon Chime Voice Connectors from different AWS Regions in your group. This creates a fault tolerant mechanism for fallback in case of availability events.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VoiceConnectorGroup {
    /// <p>The Amazon Chime Voice Connector group creation timestamp, in ISO 8601 format.</p>
    #[serde(rename = "CreatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    /// <p>The name of the Amazon Chime Voice Connector group.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The updated Amazon Chime Voice Connector group timestamp, in ISO 8601 format.</p>
    #[serde(rename = "UpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<f64>,
    /// <p>The Amazon Chime Voice Connector group ID.</p>
    #[serde(rename = "VoiceConnectorGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_connector_group_id: Option<String>,
    /// <p>The Amazon Chime Voice Connectors to which to route inbound calls.</p>
    #[serde(rename = "VoiceConnectorItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_connector_items: Option<Vec<VoiceConnectorItem>>,
}

/// <p>For Amazon Chime Voice Connector groups, the Amazon Chime Voice Connectors to which to route inbound calls. Includes priority configuration settings. Limit: 3 <code>VoiceConnectorItems</code> per Amazon Chime Voice Connector group.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceConnectorItem {
    /// <p>The priority associated with the Amazon Chime Voice Connector, with 1 being the highest priority. Higher priority Amazon Chime Voice Connectors are attempted first. </p>
    #[serde(rename = "Priority")]
    pub priority: i64,
    /// <p>The Amazon Chime Voice Connector ID.</p>
    #[serde(rename = "VoiceConnectorId")]
    pub voice_connector_id: String,
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
    /// <p>You don't have permissions to perform the requested operation.</p>
    AccessDenied(String),
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
                "AccessDeniedException" => {
                    return RusotoError::Service(AssociatePhoneNumberWithUserError::AccessDenied(
                        err.msg,
                    ))
                }
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AssociatePhoneNumberWithUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociatePhoneNumberWithUserError::AccessDenied(ref cause) => write!(f, "{}", cause),
            AssociatePhoneNumberWithUserError::BadRequest(ref cause) => write!(f, "{}", cause),
            AssociatePhoneNumberWithUserError::Forbidden(ref cause) => write!(f, "{}", cause),
            AssociatePhoneNumberWithUserError::NotFound(ref cause) => write!(f, "{}", cause),
            AssociatePhoneNumberWithUserError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            AssociatePhoneNumberWithUserError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociatePhoneNumberWithUserError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            AssociatePhoneNumberWithUserError::UnauthorizedClient(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AssociatePhoneNumberWithUserError {}
/// Errors returned by AssociatePhoneNumbersWithVoiceConnector
#[derive(Debug, PartialEq)]
pub enum AssociatePhoneNumbersWithVoiceConnectorError {
    /// <p>You don't have permissions to perform the requested operation.</p>
    AccessDenied(String),
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
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        AssociatePhoneNumbersWithVoiceConnectorError::AccessDenied(err.msg),
                    )
                }
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AssociatePhoneNumbersWithVoiceConnectorError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociatePhoneNumbersWithVoiceConnectorError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociatePhoneNumbersWithVoiceConnectorError::BadRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociatePhoneNumbersWithVoiceConnectorError::Forbidden(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociatePhoneNumbersWithVoiceConnectorError::NotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociatePhoneNumbersWithVoiceConnectorError::ServiceFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociatePhoneNumbersWithVoiceConnectorError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociatePhoneNumbersWithVoiceConnectorError::ThrottledClient(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociatePhoneNumbersWithVoiceConnectorError::UnauthorizedClient(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AssociatePhoneNumbersWithVoiceConnectorError {}
/// Errors returned by AssociatePhoneNumbersWithVoiceConnectorGroup
#[derive(Debug, PartialEq)]
pub enum AssociatePhoneNumbersWithVoiceConnectorGroupError {
    /// <p>You don't have permissions to perform the requested operation.</p>
    AccessDenied(String),
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

impl AssociatePhoneNumbersWithVoiceConnectorGroupError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AssociatePhoneNumbersWithVoiceConnectorGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        AssociatePhoneNumbersWithVoiceConnectorGroupError::AccessDenied(err.msg),
                    )
                }
                "BadRequestException" => {
                    return RusotoError::Service(
                        AssociatePhoneNumbersWithVoiceConnectorGroupError::BadRequest(err.msg),
                    )
                }
                "ForbiddenException" => {
                    return RusotoError::Service(
                        AssociatePhoneNumbersWithVoiceConnectorGroupError::Forbidden(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(
                        AssociatePhoneNumbersWithVoiceConnectorGroupError::NotFound(err.msg),
                    )
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(
                        AssociatePhoneNumbersWithVoiceConnectorGroupError::ServiceFailure(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        AssociatePhoneNumbersWithVoiceConnectorGroupError::ServiceUnavailable(
                            err.msg,
                        ),
                    )
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(
                        AssociatePhoneNumbersWithVoiceConnectorGroupError::ThrottledClient(err.msg),
                    )
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(
                        AssociatePhoneNumbersWithVoiceConnectorGroupError::UnauthorizedClient(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AssociatePhoneNumbersWithVoiceConnectorGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociatePhoneNumbersWithVoiceConnectorGroupError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociatePhoneNumbersWithVoiceConnectorGroupError::BadRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociatePhoneNumbersWithVoiceConnectorGroupError::Forbidden(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociatePhoneNumbersWithVoiceConnectorGroupError::NotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociatePhoneNumbersWithVoiceConnectorGroupError::ServiceFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociatePhoneNumbersWithVoiceConnectorGroupError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociatePhoneNumbersWithVoiceConnectorGroupError::ThrottledClient(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociatePhoneNumbersWithVoiceConnectorGroupError::UnauthorizedClient(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AssociatePhoneNumbersWithVoiceConnectorGroupError {}
/// Errors returned by AssociateSigninDelegateGroupsWithAccount
#[derive(Debug, PartialEq)]
pub enum AssociateSigninDelegateGroupsWithAccountError {
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

impl AssociateSigninDelegateGroupsWithAccountError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AssociateSigninDelegateGroupsWithAccountError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        AssociateSigninDelegateGroupsWithAccountError::BadRequest(err.msg),
                    )
                }
                "ForbiddenException" => {
                    return RusotoError::Service(
                        AssociateSigninDelegateGroupsWithAccountError::Forbidden(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(
                        AssociateSigninDelegateGroupsWithAccountError::NotFound(err.msg),
                    )
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(
                        AssociateSigninDelegateGroupsWithAccountError::ServiceFailure(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        AssociateSigninDelegateGroupsWithAccountError::ServiceUnavailable(err.msg),
                    )
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(
                        AssociateSigninDelegateGroupsWithAccountError::ThrottledClient(err.msg),
                    )
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(
                        AssociateSigninDelegateGroupsWithAccountError::UnauthorizedClient(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AssociateSigninDelegateGroupsWithAccountError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateSigninDelegateGroupsWithAccountError::BadRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateSigninDelegateGroupsWithAccountError::Forbidden(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateSigninDelegateGroupsWithAccountError::NotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateSigninDelegateGroupsWithAccountError::ServiceFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateSigninDelegateGroupsWithAccountError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateSigninDelegateGroupsWithAccountError::ThrottledClient(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateSigninDelegateGroupsWithAccountError::UnauthorizedClient(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AssociateSigninDelegateGroupsWithAccountError {}
/// Errors returned by BatchCreateAttendee
#[derive(Debug, PartialEq)]
pub enum BatchCreateAttendeeError {
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
    /// <p>The client exceeded its request rate limit.</p>
    ThrottledClient(String),
    /// <p>The client is not currently authorized to make the request.</p>
    UnauthorizedClient(String),
}

impl BatchCreateAttendeeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchCreateAttendeeError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(BatchCreateAttendeeError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(BatchCreateAttendeeError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(BatchCreateAttendeeError::NotFound(err.msg))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(BatchCreateAttendeeError::ResourceLimitExceeded(
                        err.msg,
                    ))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(BatchCreateAttendeeError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(BatchCreateAttendeeError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(BatchCreateAttendeeError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(BatchCreateAttendeeError::UnauthorizedClient(
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
impl fmt::Display for BatchCreateAttendeeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchCreateAttendeeError::BadRequest(ref cause) => write!(f, "{}", cause),
            BatchCreateAttendeeError::Forbidden(ref cause) => write!(f, "{}", cause),
            BatchCreateAttendeeError::NotFound(ref cause) => write!(f, "{}", cause),
            BatchCreateAttendeeError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            BatchCreateAttendeeError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            BatchCreateAttendeeError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            BatchCreateAttendeeError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            BatchCreateAttendeeError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchCreateAttendeeError {}
/// Errors returned by BatchCreateRoomMembership
#[derive(Debug, PartialEq)]
pub enum BatchCreateRoomMembershipError {
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

impl BatchCreateRoomMembershipError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchCreateRoomMembershipError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(BatchCreateRoomMembershipError::BadRequest(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(BatchCreateRoomMembershipError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(BatchCreateRoomMembershipError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(BatchCreateRoomMembershipError::ServiceFailure(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        BatchCreateRoomMembershipError::ServiceUnavailable(err.msg),
                    )
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(BatchCreateRoomMembershipError::ThrottledClient(
                        err.msg,
                    ))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(
                        BatchCreateRoomMembershipError::UnauthorizedClient(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchCreateRoomMembershipError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchCreateRoomMembershipError::BadRequest(ref cause) => write!(f, "{}", cause),
            BatchCreateRoomMembershipError::Forbidden(ref cause) => write!(f, "{}", cause),
            BatchCreateRoomMembershipError::NotFound(ref cause) => write!(f, "{}", cause),
            BatchCreateRoomMembershipError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            BatchCreateRoomMembershipError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            BatchCreateRoomMembershipError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            BatchCreateRoomMembershipError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchCreateRoomMembershipError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchDeletePhoneNumberError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchDeletePhoneNumberError::BadRequest(ref cause) => write!(f, "{}", cause),
            BatchDeletePhoneNumberError::Forbidden(ref cause) => write!(f, "{}", cause),
            BatchDeletePhoneNumberError::NotFound(ref cause) => write!(f, "{}", cause),
            BatchDeletePhoneNumberError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            BatchDeletePhoneNumberError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            BatchDeletePhoneNumberError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            BatchDeletePhoneNumberError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchDeletePhoneNumberError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchSuspendUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchSuspendUserError::BadRequest(ref cause) => write!(f, "{}", cause),
            BatchSuspendUserError::Forbidden(ref cause) => write!(f, "{}", cause),
            BatchSuspendUserError::NotFound(ref cause) => write!(f, "{}", cause),
            BatchSuspendUserError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            BatchSuspendUserError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            BatchSuspendUserError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            BatchSuspendUserError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchSuspendUserError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchUnsuspendUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchUnsuspendUserError::BadRequest(ref cause) => write!(f, "{}", cause),
            BatchUnsuspendUserError::Forbidden(ref cause) => write!(f, "{}", cause),
            BatchUnsuspendUserError::NotFound(ref cause) => write!(f, "{}", cause),
            BatchUnsuspendUserError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            BatchUnsuspendUserError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            BatchUnsuspendUserError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            BatchUnsuspendUserError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchUnsuspendUserError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchUpdatePhoneNumberError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchUpdatePhoneNumberError::BadRequest(ref cause) => write!(f, "{}", cause),
            BatchUpdatePhoneNumberError::Forbidden(ref cause) => write!(f, "{}", cause),
            BatchUpdatePhoneNumberError::NotFound(ref cause) => write!(f, "{}", cause),
            BatchUpdatePhoneNumberError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            BatchUpdatePhoneNumberError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            BatchUpdatePhoneNumberError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            BatchUpdatePhoneNumberError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchUpdatePhoneNumberError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchUpdateUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchUpdateUserError::BadRequest(ref cause) => write!(f, "{}", cause),
            BatchUpdateUserError::Forbidden(ref cause) => write!(f, "{}", cause),
            BatchUpdateUserError::NotFound(ref cause) => write!(f, "{}", cause),
            BatchUpdateUserError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            BatchUpdateUserError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            BatchUpdateUserError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            BatchUpdateUserError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchUpdateUserError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateAccountError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateAccountError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateAccountError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateAccountError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateAccountError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            CreateAccountError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateAccountError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            CreateAccountError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateAccountError {}
/// Errors returned by CreateAttendee
#[derive(Debug, PartialEq)]
pub enum CreateAttendeeError {
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
    /// <p>The client exceeded its request rate limit.</p>
    ThrottledClient(String),
    /// <p>The client is not currently authorized to make the request.</p>
    UnauthorizedClient(String),
}

impl CreateAttendeeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateAttendeeError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateAttendeeError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateAttendeeError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateAttendeeError::NotFound(err.msg))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(CreateAttendeeError::ResourceLimitExceeded(
                        err.msg,
                    ))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(CreateAttendeeError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateAttendeeError::ServiceUnavailable(err.msg))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(CreateAttendeeError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(CreateAttendeeError::UnauthorizedClient(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateAttendeeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateAttendeeError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateAttendeeError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateAttendeeError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateAttendeeError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateAttendeeError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            CreateAttendeeError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateAttendeeError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            CreateAttendeeError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateAttendeeError {}
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
    /// <p>The client exceeded its request rate limit.</p>
    ThrottledClient(String),
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
                "ThrottledClientException" => {
                    return RusotoError::Service(CreateBotError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(CreateBotError::UnauthorizedClient(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateBotError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateBotError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateBotError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateBotError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateBotError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateBotError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            CreateBotError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateBotError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            CreateBotError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateBotError {}
/// Errors returned by CreateMeeting
#[derive(Debug, PartialEq)]
pub enum CreateMeetingError {
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
    /// <p>The client exceeded its request rate limit.</p>
    ThrottledClient(String),
    /// <p>The client is not currently authorized to make the request.</p>
    UnauthorizedClient(String),
}

impl CreateMeetingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateMeetingError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateMeetingError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateMeetingError::Forbidden(err.msg))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(CreateMeetingError::ResourceLimitExceeded(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(CreateMeetingError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateMeetingError::ServiceUnavailable(err.msg))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(CreateMeetingError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(CreateMeetingError::UnauthorizedClient(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateMeetingError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateMeetingError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateMeetingError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateMeetingError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateMeetingError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            CreateMeetingError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateMeetingError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            CreateMeetingError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateMeetingError {}
/// Errors returned by CreatePhoneNumberOrder
#[derive(Debug, PartialEq)]
pub enum CreatePhoneNumberOrderError {
    /// <p>You don't have permissions to perform the requested operation.</p>
    AccessDenied(String),
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
    /// <p>The client exceeded its request rate limit.</p>
    ThrottledClient(String),
    /// <p>The client is not currently authorized to make the request.</p>
    UnauthorizedClient(String),
}

impl CreatePhoneNumberOrderError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreatePhoneNumberOrderError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreatePhoneNumberOrderError::AccessDenied(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(CreatePhoneNumberOrderError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreatePhoneNumberOrderError::Forbidden(err.msg))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(
                        CreatePhoneNumberOrderError::ResourceLimitExceeded(err.msg),
                    )
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreatePhoneNumberOrderError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreatePhoneNumberOrderError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreatePhoneNumberOrderError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreatePhoneNumberOrderError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreatePhoneNumberOrderError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreatePhoneNumberOrderError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            CreatePhoneNumberOrderError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            CreatePhoneNumberOrderError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            CreatePhoneNumberOrderError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreatePhoneNumberOrderError {}
/// Errors returned by CreateRoom
#[derive(Debug, PartialEq)]
pub enum CreateRoomError {
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
    /// <p>The client exceeded its request rate limit.</p>
    ThrottledClient(String),
    /// <p>The client is not currently authorized to make the request.</p>
    UnauthorizedClient(String),
}

impl CreateRoomError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateRoomError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateRoomError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateRoomError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateRoomError::NotFound(err.msg))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(CreateRoomError::ResourceLimitExceeded(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(CreateRoomError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateRoomError::ServiceUnavailable(err.msg))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(CreateRoomError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(CreateRoomError::UnauthorizedClient(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateRoomError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateRoomError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateRoomError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateRoomError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateRoomError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateRoomError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            CreateRoomError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateRoomError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            CreateRoomError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateRoomError {}
/// Errors returned by CreateRoomMembership
#[derive(Debug, PartialEq)]
pub enum CreateRoomMembershipError {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequest(String),
    /// <p>The request could not be processed because of conflict in the current state of the resource.</p>
    Conflict(String),
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
    /// <p>The client exceeded its request rate limit.</p>
    ThrottledClient(String),
    /// <p>The client is not currently authorized to make the request.</p>
    UnauthorizedClient(String),
}

impl CreateRoomMembershipError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateRoomMembershipError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateRoomMembershipError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateRoomMembershipError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateRoomMembershipError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateRoomMembershipError::NotFound(err.msg))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(CreateRoomMembershipError::ResourceLimitExceeded(
                        err.msg,
                    ))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(CreateRoomMembershipError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateRoomMembershipError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(CreateRoomMembershipError::ThrottledClient(
                        err.msg,
                    ))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(CreateRoomMembershipError::UnauthorizedClient(
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
impl fmt::Display for CreateRoomMembershipError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateRoomMembershipError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateRoomMembershipError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateRoomMembershipError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateRoomMembershipError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateRoomMembershipError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateRoomMembershipError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            CreateRoomMembershipError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateRoomMembershipError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            CreateRoomMembershipError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateRoomMembershipError {}
/// Errors returned by CreateUser
#[derive(Debug, PartialEq)]
pub enum CreateUserError {
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

impl CreateUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateUserError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateUserError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateUserError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateUserError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateUserError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(CreateUserError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateUserError::ServiceUnavailable(err.msg))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(CreateUserError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(CreateUserError::UnauthorizedClient(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateUserError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateUserError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateUserError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateUserError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateUserError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            CreateUserError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateUserError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            CreateUserError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateUserError {}
/// Errors returned by CreateVoiceConnector
#[derive(Debug, PartialEq)]
pub enum CreateVoiceConnectorError {
    /// <p>You don't have permissions to perform the requested operation.</p>
    AccessDenied(String),
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
    /// <p>The client exceeded its request rate limit.</p>
    ThrottledClient(String),
    /// <p>The client is not currently authorized to make the request.</p>
    UnauthorizedClient(String),
}

impl CreateVoiceConnectorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateVoiceConnectorError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateVoiceConnectorError::AccessDenied(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(CreateVoiceConnectorError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateVoiceConnectorError::Forbidden(err.msg))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(CreateVoiceConnectorError::ResourceLimitExceeded(
                        err.msg,
                    ))
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateVoiceConnectorError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateVoiceConnectorError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateVoiceConnectorError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateVoiceConnectorError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateVoiceConnectorError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateVoiceConnectorError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            CreateVoiceConnectorError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateVoiceConnectorError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            CreateVoiceConnectorError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateVoiceConnectorError {}
/// Errors returned by CreateVoiceConnectorGroup
#[derive(Debug, PartialEq)]
pub enum CreateVoiceConnectorGroupError {
    /// <p>You don't have permissions to perform the requested operation.</p>
    AccessDenied(String),
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
    /// <p>The client exceeded its request rate limit.</p>
    ThrottledClient(String),
    /// <p>The client is not currently authorized to make the request.</p>
    UnauthorizedClient(String),
}

impl CreateVoiceConnectorGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateVoiceConnectorGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateVoiceConnectorGroupError::AccessDenied(
                        err.msg,
                    ))
                }
                "BadRequestException" => {
                    return RusotoError::Service(CreateVoiceConnectorGroupError::BadRequest(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateVoiceConnectorGroupError::Forbidden(err.msg))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(
                        CreateVoiceConnectorGroupError::ResourceLimitExceeded(err.msg),
                    )
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(CreateVoiceConnectorGroupError::ServiceFailure(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        CreateVoiceConnectorGroupError::ServiceUnavailable(err.msg),
                    )
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(CreateVoiceConnectorGroupError::ThrottledClient(
                        err.msg,
                    ))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(
                        CreateVoiceConnectorGroupError::UnauthorizedClient(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateVoiceConnectorGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateVoiceConnectorGroupError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateVoiceConnectorGroupError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateVoiceConnectorGroupError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateVoiceConnectorGroupError::ResourceLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateVoiceConnectorGroupError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            CreateVoiceConnectorGroupError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateVoiceConnectorGroupError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            CreateVoiceConnectorGroupError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateVoiceConnectorGroupError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteAccountError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteAccountError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteAccountError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteAccountError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteAccountError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            DeleteAccountError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteAccountError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            DeleteAccountError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
            DeleteAccountError::UnprocessableEntity(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteAccountError {}
/// Errors returned by DeleteAttendee
#[derive(Debug, PartialEq)]
pub enum DeleteAttendeeError {
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

impl DeleteAttendeeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAttendeeError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteAttendeeError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteAttendeeError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteAttendeeError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(DeleteAttendeeError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteAttendeeError::ServiceUnavailable(err.msg))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(DeleteAttendeeError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(DeleteAttendeeError::UnauthorizedClient(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteAttendeeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteAttendeeError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteAttendeeError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteAttendeeError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteAttendeeError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            DeleteAttendeeError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteAttendeeError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            DeleteAttendeeError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteAttendeeError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteEventsConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteEventsConfigurationError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteEventsConfigurationError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteEventsConfigurationError::ResourceLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteEventsConfigurationError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            DeleteEventsConfigurationError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteEventsConfigurationError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteEventsConfigurationError {}
/// Errors returned by DeleteMeeting
#[derive(Debug, PartialEq)]
pub enum DeleteMeetingError {
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

impl DeleteMeetingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteMeetingError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteMeetingError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteMeetingError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteMeetingError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(DeleteMeetingError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteMeetingError::ServiceUnavailable(err.msg))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(DeleteMeetingError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(DeleteMeetingError::UnauthorizedClient(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteMeetingError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteMeetingError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteMeetingError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteMeetingError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteMeetingError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            DeleteMeetingError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteMeetingError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            DeleteMeetingError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteMeetingError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeletePhoneNumberError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeletePhoneNumberError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeletePhoneNumberError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeletePhoneNumberError::NotFound(ref cause) => write!(f, "{}", cause),
            DeletePhoneNumberError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            DeletePhoneNumberError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DeletePhoneNumberError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            DeletePhoneNumberError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeletePhoneNumberError {}
/// Errors returned by DeleteRoom
#[derive(Debug, PartialEq)]
pub enum DeleteRoomError {
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

impl DeleteRoomError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteRoomError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteRoomError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteRoomError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteRoomError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(DeleteRoomError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteRoomError::ServiceUnavailable(err.msg))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(DeleteRoomError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(DeleteRoomError::UnauthorizedClient(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteRoomError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteRoomError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteRoomError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteRoomError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteRoomError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            DeleteRoomError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteRoomError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            DeleteRoomError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteRoomError {}
/// Errors returned by DeleteRoomMembership
#[derive(Debug, PartialEq)]
pub enum DeleteRoomMembershipError {
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

impl DeleteRoomMembershipError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteRoomMembershipError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteRoomMembershipError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteRoomMembershipError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteRoomMembershipError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(DeleteRoomMembershipError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteRoomMembershipError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(DeleteRoomMembershipError::ThrottledClient(
                        err.msg,
                    ))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(DeleteRoomMembershipError::UnauthorizedClient(
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
impl fmt::Display for DeleteRoomMembershipError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteRoomMembershipError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteRoomMembershipError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteRoomMembershipError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteRoomMembershipError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            DeleteRoomMembershipError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteRoomMembershipError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            DeleteRoomMembershipError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteRoomMembershipError {}
/// Errors returned by DeleteVoiceConnector
#[derive(Debug, PartialEq)]
pub enum DeleteVoiceConnectorError {
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

impl DeleteVoiceConnectorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteVoiceConnectorError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteVoiceConnectorError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteVoiceConnectorError::Conflict(err.msg))
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteVoiceConnectorError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteVoiceConnectorError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteVoiceConnectorError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteVoiceConnectorError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteVoiceConnectorError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteVoiceConnectorError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            DeleteVoiceConnectorError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteVoiceConnectorError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            DeleteVoiceConnectorError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteVoiceConnectorError {}
/// Errors returned by DeleteVoiceConnectorGroup
#[derive(Debug, PartialEq)]
pub enum DeleteVoiceConnectorGroupError {
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

impl DeleteVoiceConnectorGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteVoiceConnectorGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteVoiceConnectorGroupError::BadRequest(
                        err.msg,
                    ))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteVoiceConnectorGroupError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteVoiceConnectorGroupError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteVoiceConnectorGroupError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(DeleteVoiceConnectorGroupError::ServiceFailure(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        DeleteVoiceConnectorGroupError::ServiceUnavailable(err.msg),
                    )
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(DeleteVoiceConnectorGroupError::ThrottledClient(
                        err.msg,
                    ))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(
                        DeleteVoiceConnectorGroupError::UnauthorizedClient(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteVoiceConnectorGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteVoiceConnectorGroupError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteVoiceConnectorGroupError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteVoiceConnectorGroupError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteVoiceConnectorGroupError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteVoiceConnectorGroupError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            DeleteVoiceConnectorGroupError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteVoiceConnectorGroupError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            DeleteVoiceConnectorGroupError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteVoiceConnectorGroupError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteVoiceConnectorOriginationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteVoiceConnectorOriginationError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteVoiceConnectorOriginationError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteVoiceConnectorOriginationError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteVoiceConnectorOriginationError::ServiceFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteVoiceConnectorOriginationError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteVoiceConnectorOriginationError::ThrottledClient(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteVoiceConnectorOriginationError::UnauthorizedClient(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteVoiceConnectorOriginationError {}
/// Errors returned by DeleteVoiceConnectorStreamingConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteVoiceConnectorStreamingConfigurationError {
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

impl DeleteVoiceConnectorStreamingConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteVoiceConnectorStreamingConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        DeleteVoiceConnectorStreamingConfigurationError::BadRequest(err.msg),
                    )
                }
                "ForbiddenException" => {
                    return RusotoError::Service(
                        DeleteVoiceConnectorStreamingConfigurationError::Forbidden(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(
                        DeleteVoiceConnectorStreamingConfigurationError::NotFound(err.msg),
                    )
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(
                        DeleteVoiceConnectorStreamingConfigurationError::ServiceFailure(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        DeleteVoiceConnectorStreamingConfigurationError::ServiceUnavailable(
                            err.msg,
                        ),
                    )
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(
                        DeleteVoiceConnectorStreamingConfigurationError::ThrottledClient(err.msg),
                    )
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(
                        DeleteVoiceConnectorStreamingConfigurationError::UnauthorizedClient(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteVoiceConnectorStreamingConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteVoiceConnectorStreamingConfigurationError::BadRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteVoiceConnectorStreamingConfigurationError::Forbidden(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteVoiceConnectorStreamingConfigurationError::NotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteVoiceConnectorStreamingConfigurationError::ServiceFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteVoiceConnectorStreamingConfigurationError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteVoiceConnectorStreamingConfigurationError::ThrottledClient(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteVoiceConnectorStreamingConfigurationError::UnauthorizedClient(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteVoiceConnectorStreamingConfigurationError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteVoiceConnectorTerminationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteVoiceConnectorTerminationError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteVoiceConnectorTerminationError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteVoiceConnectorTerminationError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteVoiceConnectorTerminationError::ServiceFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteVoiceConnectorTerminationError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteVoiceConnectorTerminationError::ThrottledClient(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteVoiceConnectorTerminationError::UnauthorizedClient(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteVoiceConnectorTerminationError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteVoiceConnectorTerminationCredentialsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteVoiceConnectorTerminationCredentialsError::BadRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteVoiceConnectorTerminationCredentialsError::Forbidden(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteVoiceConnectorTerminationCredentialsError::NotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteVoiceConnectorTerminationCredentialsError::ServiceFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteVoiceConnectorTerminationCredentialsError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteVoiceConnectorTerminationCredentialsError::ThrottledClient(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteVoiceConnectorTerminationCredentialsError::UnauthorizedClient(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteVoiceConnectorTerminationCredentialsError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisassociatePhoneNumberFromUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociatePhoneNumberFromUserError::BadRequest(ref cause) => write!(f, "{}", cause),
            DisassociatePhoneNumberFromUserError::Forbidden(ref cause) => write!(f, "{}", cause),
            DisassociatePhoneNumberFromUserError::NotFound(ref cause) => write!(f, "{}", cause),
            DisassociatePhoneNumberFromUserError::ServiceFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociatePhoneNumberFromUserError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociatePhoneNumberFromUserError::ThrottledClient(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociatePhoneNumberFromUserError::UnauthorizedClient(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DisassociatePhoneNumberFromUserError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisassociatePhoneNumbersFromVoiceConnectorError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociatePhoneNumbersFromVoiceConnectorError::BadRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociatePhoneNumbersFromVoiceConnectorError::Forbidden(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociatePhoneNumbersFromVoiceConnectorError::NotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociatePhoneNumbersFromVoiceConnectorError::ServiceFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociatePhoneNumbersFromVoiceConnectorError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociatePhoneNumbersFromVoiceConnectorError::ThrottledClient(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociatePhoneNumbersFromVoiceConnectorError::UnauthorizedClient(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DisassociatePhoneNumbersFromVoiceConnectorError {}
/// Errors returned by DisassociatePhoneNumbersFromVoiceConnectorGroup
#[derive(Debug, PartialEq)]
pub enum DisassociatePhoneNumbersFromVoiceConnectorGroupError {
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

impl DisassociatePhoneNumbersFromVoiceConnectorGroupError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociatePhoneNumbersFromVoiceConnectorGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        DisassociatePhoneNumbersFromVoiceConnectorGroupError::BadRequest(err.msg),
                    )
                }
                "ForbiddenException" => {
                    return RusotoError::Service(
                        DisassociatePhoneNumbersFromVoiceConnectorGroupError::Forbidden(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(
                        DisassociatePhoneNumbersFromVoiceConnectorGroupError::NotFound(err.msg),
                    )
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(
                        DisassociatePhoneNumbersFromVoiceConnectorGroupError::ServiceFailure(
                            err.msg,
                        ),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        DisassociatePhoneNumbersFromVoiceConnectorGroupError::ServiceUnavailable(
                            err.msg,
                        ),
                    )
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(
                        DisassociatePhoneNumbersFromVoiceConnectorGroupError::ThrottledClient(
                            err.msg,
                        ),
                    )
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(
                        DisassociatePhoneNumbersFromVoiceConnectorGroupError::UnauthorizedClient(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisassociatePhoneNumbersFromVoiceConnectorGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociatePhoneNumbersFromVoiceConnectorGroupError::BadRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociatePhoneNumbersFromVoiceConnectorGroupError::Forbidden(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociatePhoneNumbersFromVoiceConnectorGroupError::NotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociatePhoneNumbersFromVoiceConnectorGroupError::ServiceFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociatePhoneNumbersFromVoiceConnectorGroupError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociatePhoneNumbersFromVoiceConnectorGroupError::ThrottledClient(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociatePhoneNumbersFromVoiceConnectorGroupError::UnauthorizedClient(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DisassociatePhoneNumbersFromVoiceConnectorGroupError {}
/// Errors returned by DisassociateSigninDelegateGroupsFromAccount
#[derive(Debug, PartialEq)]
pub enum DisassociateSigninDelegateGroupsFromAccountError {
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

impl DisassociateSigninDelegateGroupsFromAccountError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateSigninDelegateGroupsFromAccountError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        DisassociateSigninDelegateGroupsFromAccountError::BadRequest(err.msg),
                    )
                }
                "ForbiddenException" => {
                    return RusotoError::Service(
                        DisassociateSigninDelegateGroupsFromAccountError::Forbidden(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(
                        DisassociateSigninDelegateGroupsFromAccountError::NotFound(err.msg),
                    )
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(
                        DisassociateSigninDelegateGroupsFromAccountError::ServiceFailure(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        DisassociateSigninDelegateGroupsFromAccountError::ServiceUnavailable(
                            err.msg,
                        ),
                    )
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(
                        DisassociateSigninDelegateGroupsFromAccountError::ThrottledClient(err.msg),
                    )
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(
                        DisassociateSigninDelegateGroupsFromAccountError::UnauthorizedClient(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisassociateSigninDelegateGroupsFromAccountError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateSigninDelegateGroupsFromAccountError::BadRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateSigninDelegateGroupsFromAccountError::Forbidden(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateSigninDelegateGroupsFromAccountError::NotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateSigninDelegateGroupsFromAccountError::ServiceFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateSigninDelegateGroupsFromAccountError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateSigninDelegateGroupsFromAccountError::ThrottledClient(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateSigninDelegateGroupsFromAccountError::UnauthorizedClient(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DisassociateSigninDelegateGroupsFromAccountError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetAccountError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAccountError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetAccountError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetAccountError::NotFound(ref cause) => write!(f, "{}", cause),
            GetAccountError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            GetAccountError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            GetAccountError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            GetAccountError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetAccountError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetAccountSettingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAccountSettingsError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetAccountSettingsError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetAccountSettingsError::NotFound(ref cause) => write!(f, "{}", cause),
            GetAccountSettingsError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            GetAccountSettingsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            GetAccountSettingsError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            GetAccountSettingsError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetAccountSettingsError {}
/// Errors returned by GetAttendee
#[derive(Debug, PartialEq)]
pub enum GetAttendeeError {
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

impl GetAttendeeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAttendeeError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetAttendeeError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetAttendeeError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetAttendeeError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(GetAttendeeError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetAttendeeError::ServiceUnavailable(err.msg))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(GetAttendeeError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(GetAttendeeError::UnauthorizedClient(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetAttendeeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAttendeeError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetAttendeeError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetAttendeeError::NotFound(ref cause) => write!(f, "{}", cause),
            GetAttendeeError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            GetAttendeeError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            GetAttendeeError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            GetAttendeeError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetAttendeeError {}
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
    /// <p>The client exceeded its request rate limit.</p>
    ThrottledClient(String),
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
                "ThrottledClientException" => {
                    return RusotoError::Service(GetBotError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(GetBotError::UnauthorizedClient(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetBotError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetBotError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetBotError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetBotError::NotFound(ref cause) => write!(f, "{}", cause),
            GetBotError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            GetBotError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            GetBotError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            GetBotError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetBotError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetEventsConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetEventsConfigurationError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetEventsConfigurationError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetEventsConfigurationError::NotFound(ref cause) => write!(f, "{}", cause),
            GetEventsConfigurationError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            GetEventsConfigurationError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            GetEventsConfigurationError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            GetEventsConfigurationError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetEventsConfigurationError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetGlobalSettingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetGlobalSettingsError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetGlobalSettingsError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetGlobalSettingsError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            GetGlobalSettingsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            GetGlobalSettingsError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            GetGlobalSettingsError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetGlobalSettingsError {}
/// Errors returned by GetMeeting
#[derive(Debug, PartialEq)]
pub enum GetMeetingError {
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

impl GetMeetingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetMeetingError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetMeetingError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetMeetingError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetMeetingError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(GetMeetingError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetMeetingError::ServiceUnavailable(err.msg))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(GetMeetingError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(GetMeetingError::UnauthorizedClient(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetMeetingError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetMeetingError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetMeetingError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetMeetingError::NotFound(ref cause) => write!(f, "{}", cause),
            GetMeetingError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            GetMeetingError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            GetMeetingError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            GetMeetingError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetMeetingError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetPhoneNumberError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetPhoneNumberError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetPhoneNumberError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetPhoneNumberError::NotFound(ref cause) => write!(f, "{}", cause),
            GetPhoneNumberError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            GetPhoneNumberError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            GetPhoneNumberError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            GetPhoneNumberError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetPhoneNumberError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetPhoneNumberOrderError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetPhoneNumberOrderError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetPhoneNumberOrderError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetPhoneNumberOrderError::NotFound(ref cause) => write!(f, "{}", cause),
            GetPhoneNumberOrderError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            GetPhoneNumberOrderError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            GetPhoneNumberOrderError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            GetPhoneNumberOrderError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetPhoneNumberOrderError {}
/// Errors returned by GetPhoneNumberSettings
#[derive(Debug, PartialEq)]
pub enum GetPhoneNumberSettingsError {
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

impl GetPhoneNumberSettingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetPhoneNumberSettingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetPhoneNumberSettingsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetPhoneNumberSettingsError::Forbidden(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(GetPhoneNumberSettingsError::ServiceFailure(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetPhoneNumberSettingsError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(GetPhoneNumberSettingsError::ThrottledClient(
                        err.msg,
                    ))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(GetPhoneNumberSettingsError::UnauthorizedClient(
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
impl fmt::Display for GetPhoneNumberSettingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetPhoneNumberSettingsError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetPhoneNumberSettingsError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetPhoneNumberSettingsError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            GetPhoneNumberSettingsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            GetPhoneNumberSettingsError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            GetPhoneNumberSettingsError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetPhoneNumberSettingsError {}
/// Errors returned by GetRoom
#[derive(Debug, PartialEq)]
pub enum GetRoomError {
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

impl GetRoomError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetRoomError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetRoomError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetRoomError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetRoomError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(GetRoomError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetRoomError::ServiceUnavailable(err.msg))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(GetRoomError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(GetRoomError::UnauthorizedClient(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetRoomError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRoomError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetRoomError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetRoomError::NotFound(ref cause) => write!(f, "{}", cause),
            GetRoomError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            GetRoomError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            GetRoomError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            GetRoomError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetRoomError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetUserError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetUserError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetUserError::NotFound(ref cause) => write!(f, "{}", cause),
            GetUserError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            GetUserError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            GetUserError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            GetUserError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetUserError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetUserSettingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetUserSettingsError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetUserSettingsError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetUserSettingsError::NotFound(ref cause) => write!(f, "{}", cause),
            GetUserSettingsError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            GetUserSettingsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            GetUserSettingsError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            GetUserSettingsError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetUserSettingsError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetVoiceConnectorError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetVoiceConnectorError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetVoiceConnectorError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetVoiceConnectorError::NotFound(ref cause) => write!(f, "{}", cause),
            GetVoiceConnectorError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            GetVoiceConnectorError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            GetVoiceConnectorError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            GetVoiceConnectorError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetVoiceConnectorError {}
/// Errors returned by GetVoiceConnectorGroup
#[derive(Debug, PartialEq)]
pub enum GetVoiceConnectorGroupError {
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

impl GetVoiceConnectorGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetVoiceConnectorGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetVoiceConnectorGroupError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetVoiceConnectorGroupError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetVoiceConnectorGroupError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(GetVoiceConnectorGroupError::ServiceFailure(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetVoiceConnectorGroupError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(GetVoiceConnectorGroupError::ThrottledClient(
                        err.msg,
                    ))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(GetVoiceConnectorGroupError::UnauthorizedClient(
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
impl fmt::Display for GetVoiceConnectorGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetVoiceConnectorGroupError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetVoiceConnectorGroupError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetVoiceConnectorGroupError::NotFound(ref cause) => write!(f, "{}", cause),
            GetVoiceConnectorGroupError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            GetVoiceConnectorGroupError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            GetVoiceConnectorGroupError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            GetVoiceConnectorGroupError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetVoiceConnectorGroupError {}
/// Errors returned by GetVoiceConnectorLoggingConfiguration
#[derive(Debug, PartialEq)]
pub enum GetVoiceConnectorLoggingConfigurationError {
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

impl GetVoiceConnectorLoggingConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetVoiceConnectorLoggingConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        GetVoiceConnectorLoggingConfigurationError::BadRequest(err.msg),
                    )
                }
                "ForbiddenException" => {
                    return RusotoError::Service(
                        GetVoiceConnectorLoggingConfigurationError::Forbidden(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(
                        GetVoiceConnectorLoggingConfigurationError::NotFound(err.msg),
                    )
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(
                        GetVoiceConnectorLoggingConfigurationError::ServiceFailure(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        GetVoiceConnectorLoggingConfigurationError::ServiceUnavailable(err.msg),
                    )
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(
                        GetVoiceConnectorLoggingConfigurationError::ThrottledClient(err.msg),
                    )
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(
                        GetVoiceConnectorLoggingConfigurationError::UnauthorizedClient(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetVoiceConnectorLoggingConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetVoiceConnectorLoggingConfigurationError::BadRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            GetVoiceConnectorLoggingConfigurationError::Forbidden(ref cause) => {
                write!(f, "{}", cause)
            }
            GetVoiceConnectorLoggingConfigurationError::NotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            GetVoiceConnectorLoggingConfigurationError::ServiceFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            GetVoiceConnectorLoggingConfigurationError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            GetVoiceConnectorLoggingConfigurationError::ThrottledClient(ref cause) => {
                write!(f, "{}", cause)
            }
            GetVoiceConnectorLoggingConfigurationError::UnauthorizedClient(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetVoiceConnectorLoggingConfigurationError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetVoiceConnectorOriginationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetVoiceConnectorOriginationError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetVoiceConnectorOriginationError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetVoiceConnectorOriginationError::NotFound(ref cause) => write!(f, "{}", cause),
            GetVoiceConnectorOriginationError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            GetVoiceConnectorOriginationError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            GetVoiceConnectorOriginationError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            GetVoiceConnectorOriginationError::UnauthorizedClient(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetVoiceConnectorOriginationError {}
/// Errors returned by GetVoiceConnectorStreamingConfiguration
#[derive(Debug, PartialEq)]
pub enum GetVoiceConnectorStreamingConfigurationError {
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

impl GetVoiceConnectorStreamingConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetVoiceConnectorStreamingConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        GetVoiceConnectorStreamingConfigurationError::BadRequest(err.msg),
                    )
                }
                "ForbiddenException" => {
                    return RusotoError::Service(
                        GetVoiceConnectorStreamingConfigurationError::Forbidden(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(
                        GetVoiceConnectorStreamingConfigurationError::NotFound(err.msg),
                    )
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(
                        GetVoiceConnectorStreamingConfigurationError::ServiceFailure(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        GetVoiceConnectorStreamingConfigurationError::ServiceUnavailable(err.msg),
                    )
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(
                        GetVoiceConnectorStreamingConfigurationError::ThrottledClient(err.msg),
                    )
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(
                        GetVoiceConnectorStreamingConfigurationError::UnauthorizedClient(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetVoiceConnectorStreamingConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetVoiceConnectorStreamingConfigurationError::BadRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            GetVoiceConnectorStreamingConfigurationError::Forbidden(ref cause) => {
                write!(f, "{}", cause)
            }
            GetVoiceConnectorStreamingConfigurationError::NotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            GetVoiceConnectorStreamingConfigurationError::ServiceFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            GetVoiceConnectorStreamingConfigurationError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            GetVoiceConnectorStreamingConfigurationError::ThrottledClient(ref cause) => {
                write!(f, "{}", cause)
            }
            GetVoiceConnectorStreamingConfigurationError::UnauthorizedClient(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetVoiceConnectorStreamingConfigurationError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetVoiceConnectorTerminationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetVoiceConnectorTerminationError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetVoiceConnectorTerminationError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetVoiceConnectorTerminationError::NotFound(ref cause) => write!(f, "{}", cause),
            GetVoiceConnectorTerminationError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            GetVoiceConnectorTerminationError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            GetVoiceConnectorTerminationError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            GetVoiceConnectorTerminationError::UnauthorizedClient(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetVoiceConnectorTerminationError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetVoiceConnectorTerminationHealthError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetVoiceConnectorTerminationHealthError::BadRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            GetVoiceConnectorTerminationHealthError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetVoiceConnectorTerminationHealthError::NotFound(ref cause) => write!(f, "{}", cause),
            GetVoiceConnectorTerminationHealthError::ServiceFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            GetVoiceConnectorTerminationHealthError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            GetVoiceConnectorTerminationHealthError::ThrottledClient(ref cause) => {
                write!(f, "{}", cause)
            }
            GetVoiceConnectorTerminationHealthError::UnauthorizedClient(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetVoiceConnectorTerminationHealthError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for InviteUsersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InviteUsersError::BadRequest(ref cause) => write!(f, "{}", cause),
            InviteUsersError::Forbidden(ref cause) => write!(f, "{}", cause),
            InviteUsersError::NotFound(ref cause) => write!(f, "{}", cause),
            InviteUsersError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            InviteUsersError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            InviteUsersError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            InviteUsersError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for InviteUsersError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListAccountsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListAccountsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListAccountsError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListAccountsError::NotFound(ref cause) => write!(f, "{}", cause),
            ListAccountsError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            ListAccountsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListAccountsError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            ListAccountsError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListAccountsError {}
/// Errors returned by ListAttendees
#[derive(Debug, PartialEq)]
pub enum ListAttendeesError {
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

impl ListAttendeesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAttendeesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListAttendeesError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListAttendeesError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListAttendeesError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(ListAttendeesError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListAttendeesError::ServiceUnavailable(err.msg))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(ListAttendeesError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(ListAttendeesError::UnauthorizedClient(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListAttendeesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListAttendeesError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListAttendeesError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListAttendeesError::NotFound(ref cause) => write!(f, "{}", cause),
            ListAttendeesError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            ListAttendeesError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListAttendeesError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            ListAttendeesError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListAttendeesError {}
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
    /// <p>The client exceeded its request rate limit.</p>
    ThrottledClient(String),
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
                "ThrottledClientException" => {
                    return RusotoError::Service(ListBotsError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(ListBotsError::UnauthorizedClient(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListBotsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListBotsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListBotsError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListBotsError::NotFound(ref cause) => write!(f, "{}", cause),
            ListBotsError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            ListBotsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListBotsError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            ListBotsError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListBotsError {}
/// Errors returned by ListMeetings
#[derive(Debug, PartialEq)]
pub enum ListMeetingsError {
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

impl ListMeetingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListMeetingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListMeetingsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListMeetingsError::Forbidden(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(ListMeetingsError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListMeetingsError::ServiceUnavailable(err.msg))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(ListMeetingsError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(ListMeetingsError::UnauthorizedClient(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListMeetingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListMeetingsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListMeetingsError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListMeetingsError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            ListMeetingsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListMeetingsError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            ListMeetingsError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListMeetingsError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListPhoneNumberOrdersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListPhoneNumberOrdersError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListPhoneNumberOrdersError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListPhoneNumberOrdersError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            ListPhoneNumberOrdersError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListPhoneNumberOrdersError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            ListPhoneNumberOrdersError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListPhoneNumberOrdersError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListPhoneNumbersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListPhoneNumbersError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListPhoneNumbersError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListPhoneNumbersError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            ListPhoneNumbersError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListPhoneNumbersError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            ListPhoneNumbersError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListPhoneNumbersError {}
/// Errors returned by ListRoomMemberships
#[derive(Debug, PartialEq)]
pub enum ListRoomMembershipsError {
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

impl ListRoomMembershipsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRoomMembershipsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListRoomMembershipsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListRoomMembershipsError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListRoomMembershipsError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(ListRoomMembershipsError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListRoomMembershipsError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(ListRoomMembershipsError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(ListRoomMembershipsError::UnauthorizedClient(
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
impl fmt::Display for ListRoomMembershipsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListRoomMembershipsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListRoomMembershipsError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListRoomMembershipsError::NotFound(ref cause) => write!(f, "{}", cause),
            ListRoomMembershipsError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            ListRoomMembershipsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListRoomMembershipsError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            ListRoomMembershipsError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListRoomMembershipsError {}
/// Errors returned by ListRooms
#[derive(Debug, PartialEq)]
pub enum ListRoomsError {
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

impl ListRoomsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRoomsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListRoomsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListRoomsError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListRoomsError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(ListRoomsError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListRoomsError::ServiceUnavailable(err.msg))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(ListRoomsError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(ListRoomsError::UnauthorizedClient(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListRoomsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListRoomsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListRoomsError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListRoomsError::NotFound(ref cause) => write!(f, "{}", cause),
            ListRoomsError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            ListRoomsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListRoomsError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            ListRoomsError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListRoomsError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListUsersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListUsersError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListUsersError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListUsersError::NotFound(ref cause) => write!(f, "{}", cause),
            ListUsersError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            ListUsersError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListUsersError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            ListUsersError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListUsersError {}
/// Errors returned by ListVoiceConnectorGroups
#[derive(Debug, PartialEq)]
pub enum ListVoiceConnectorGroupsError {
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

impl ListVoiceConnectorGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListVoiceConnectorGroupsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListVoiceConnectorGroupsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListVoiceConnectorGroupsError::Forbidden(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(ListVoiceConnectorGroupsError::ServiceFailure(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListVoiceConnectorGroupsError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(ListVoiceConnectorGroupsError::ThrottledClient(
                        err.msg,
                    ))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(ListVoiceConnectorGroupsError::UnauthorizedClient(
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
impl fmt::Display for ListVoiceConnectorGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListVoiceConnectorGroupsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListVoiceConnectorGroupsError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListVoiceConnectorGroupsError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            ListVoiceConnectorGroupsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListVoiceConnectorGroupsError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            ListVoiceConnectorGroupsError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListVoiceConnectorGroupsError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListVoiceConnectorTerminationCredentialsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListVoiceConnectorTerminationCredentialsError::BadRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            ListVoiceConnectorTerminationCredentialsError::Forbidden(ref cause) => {
                write!(f, "{}", cause)
            }
            ListVoiceConnectorTerminationCredentialsError::NotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            ListVoiceConnectorTerminationCredentialsError::ServiceFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            ListVoiceConnectorTerminationCredentialsError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            ListVoiceConnectorTerminationCredentialsError::ThrottledClient(ref cause) => {
                write!(f, "{}", cause)
            }
            ListVoiceConnectorTerminationCredentialsError::UnauthorizedClient(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListVoiceConnectorTerminationCredentialsError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListVoiceConnectorsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListVoiceConnectorsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListVoiceConnectorsError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListVoiceConnectorsError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            ListVoiceConnectorsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListVoiceConnectorsError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            ListVoiceConnectorsError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListVoiceConnectorsError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for LogoutUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LogoutUserError::BadRequest(ref cause) => write!(f, "{}", cause),
            LogoutUserError::Forbidden(ref cause) => write!(f, "{}", cause),
            LogoutUserError::NotFound(ref cause) => write!(f, "{}", cause),
            LogoutUserError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            LogoutUserError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            LogoutUserError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            LogoutUserError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for LogoutUserError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutEventsConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutEventsConfigurationError::BadRequest(ref cause) => write!(f, "{}", cause),
            PutEventsConfigurationError::Forbidden(ref cause) => write!(f, "{}", cause),
            PutEventsConfigurationError::NotFound(ref cause) => write!(f, "{}", cause),
            PutEventsConfigurationError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            PutEventsConfigurationError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            PutEventsConfigurationError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            PutEventsConfigurationError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutEventsConfigurationError {}
/// Errors returned by PutVoiceConnectorLoggingConfiguration
#[derive(Debug, PartialEq)]
pub enum PutVoiceConnectorLoggingConfigurationError {
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

impl PutVoiceConnectorLoggingConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutVoiceConnectorLoggingConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        PutVoiceConnectorLoggingConfigurationError::BadRequest(err.msg),
                    )
                }
                "ForbiddenException" => {
                    return RusotoError::Service(
                        PutVoiceConnectorLoggingConfigurationError::Forbidden(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(
                        PutVoiceConnectorLoggingConfigurationError::NotFound(err.msg),
                    )
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(
                        PutVoiceConnectorLoggingConfigurationError::ServiceFailure(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        PutVoiceConnectorLoggingConfigurationError::ServiceUnavailable(err.msg),
                    )
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(
                        PutVoiceConnectorLoggingConfigurationError::ThrottledClient(err.msg),
                    )
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(
                        PutVoiceConnectorLoggingConfigurationError::UnauthorizedClient(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutVoiceConnectorLoggingConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutVoiceConnectorLoggingConfigurationError::BadRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            PutVoiceConnectorLoggingConfigurationError::Forbidden(ref cause) => {
                write!(f, "{}", cause)
            }
            PutVoiceConnectorLoggingConfigurationError::NotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            PutVoiceConnectorLoggingConfigurationError::ServiceFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            PutVoiceConnectorLoggingConfigurationError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            PutVoiceConnectorLoggingConfigurationError::ThrottledClient(ref cause) => {
                write!(f, "{}", cause)
            }
            PutVoiceConnectorLoggingConfigurationError::UnauthorizedClient(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutVoiceConnectorLoggingConfigurationError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutVoiceConnectorOriginationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutVoiceConnectorOriginationError::BadRequest(ref cause) => write!(f, "{}", cause),
            PutVoiceConnectorOriginationError::Forbidden(ref cause) => write!(f, "{}", cause),
            PutVoiceConnectorOriginationError::NotFound(ref cause) => write!(f, "{}", cause),
            PutVoiceConnectorOriginationError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            PutVoiceConnectorOriginationError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            PutVoiceConnectorOriginationError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            PutVoiceConnectorOriginationError::UnauthorizedClient(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutVoiceConnectorOriginationError {}
/// Errors returned by PutVoiceConnectorStreamingConfiguration
#[derive(Debug, PartialEq)]
pub enum PutVoiceConnectorStreamingConfigurationError {
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

impl PutVoiceConnectorStreamingConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutVoiceConnectorStreamingConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        PutVoiceConnectorStreamingConfigurationError::BadRequest(err.msg),
                    )
                }
                "ForbiddenException" => {
                    return RusotoError::Service(
                        PutVoiceConnectorStreamingConfigurationError::Forbidden(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(
                        PutVoiceConnectorStreamingConfigurationError::NotFound(err.msg),
                    )
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(
                        PutVoiceConnectorStreamingConfigurationError::ServiceFailure(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        PutVoiceConnectorStreamingConfigurationError::ServiceUnavailable(err.msg),
                    )
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(
                        PutVoiceConnectorStreamingConfigurationError::ThrottledClient(err.msg),
                    )
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(
                        PutVoiceConnectorStreamingConfigurationError::UnauthorizedClient(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutVoiceConnectorStreamingConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutVoiceConnectorStreamingConfigurationError::BadRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            PutVoiceConnectorStreamingConfigurationError::Forbidden(ref cause) => {
                write!(f, "{}", cause)
            }
            PutVoiceConnectorStreamingConfigurationError::NotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            PutVoiceConnectorStreamingConfigurationError::ServiceFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            PutVoiceConnectorStreamingConfigurationError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            PutVoiceConnectorStreamingConfigurationError::ThrottledClient(ref cause) => {
                write!(f, "{}", cause)
            }
            PutVoiceConnectorStreamingConfigurationError::UnauthorizedClient(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutVoiceConnectorStreamingConfigurationError {}
/// Errors returned by PutVoiceConnectorTermination
#[derive(Debug, PartialEq)]
pub enum PutVoiceConnectorTerminationError {
    /// <p>You don't have permissions to perform the requested operation.</p>
    AccessDenied(String),
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
                "AccessDeniedException" => {
                    return RusotoError::Service(PutVoiceConnectorTerminationError::AccessDenied(
                        err.msg,
                    ))
                }
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutVoiceConnectorTerminationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutVoiceConnectorTerminationError::AccessDenied(ref cause) => write!(f, "{}", cause),
            PutVoiceConnectorTerminationError::BadRequest(ref cause) => write!(f, "{}", cause),
            PutVoiceConnectorTerminationError::Forbidden(ref cause) => write!(f, "{}", cause),
            PutVoiceConnectorTerminationError::NotFound(ref cause) => write!(f, "{}", cause),
            PutVoiceConnectorTerminationError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            PutVoiceConnectorTerminationError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            PutVoiceConnectorTerminationError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            PutVoiceConnectorTerminationError::UnauthorizedClient(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutVoiceConnectorTerminationError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutVoiceConnectorTerminationCredentialsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutVoiceConnectorTerminationCredentialsError::BadRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            PutVoiceConnectorTerminationCredentialsError::Forbidden(ref cause) => {
                write!(f, "{}", cause)
            }
            PutVoiceConnectorTerminationCredentialsError::NotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            PutVoiceConnectorTerminationCredentialsError::ServiceFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            PutVoiceConnectorTerminationCredentialsError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            PutVoiceConnectorTerminationCredentialsError::ThrottledClient(ref cause) => {
                write!(f, "{}", cause)
            }
            PutVoiceConnectorTerminationCredentialsError::UnauthorizedClient(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutVoiceConnectorTerminationCredentialsError {}
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
    /// <p>The client exceeded its request rate limit.</p>
    ThrottledClient(String),
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
                "ThrottledClientException" => {
                    return RusotoError::Service(RegenerateSecurityTokenError::ThrottledClient(
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RegenerateSecurityTokenError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RegenerateSecurityTokenError::BadRequest(ref cause) => write!(f, "{}", cause),
            RegenerateSecurityTokenError::Forbidden(ref cause) => write!(f, "{}", cause),
            RegenerateSecurityTokenError::NotFound(ref cause) => write!(f, "{}", cause),
            RegenerateSecurityTokenError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            RegenerateSecurityTokenError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            RegenerateSecurityTokenError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            RegenerateSecurityTokenError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RegenerateSecurityTokenError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ResetPersonalPINError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ResetPersonalPINError::BadRequest(ref cause) => write!(f, "{}", cause),
            ResetPersonalPINError::Forbidden(ref cause) => write!(f, "{}", cause),
            ResetPersonalPINError::NotFound(ref cause) => write!(f, "{}", cause),
            ResetPersonalPINError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            ResetPersonalPINError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ResetPersonalPINError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            ResetPersonalPINError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ResetPersonalPINError {}
/// Errors returned by RestorePhoneNumber
#[derive(Debug, PartialEq)]
pub enum RestorePhoneNumberError {
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
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(RestorePhoneNumberError::ResourceLimitExceeded(
                        err.msg,
                    ))
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RestorePhoneNumberError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RestorePhoneNumberError::BadRequest(ref cause) => write!(f, "{}", cause),
            RestorePhoneNumberError::Forbidden(ref cause) => write!(f, "{}", cause),
            RestorePhoneNumberError::NotFound(ref cause) => write!(f, "{}", cause),
            RestorePhoneNumberError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            RestorePhoneNumberError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            RestorePhoneNumberError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            RestorePhoneNumberError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            RestorePhoneNumberError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RestorePhoneNumberError {}
/// Errors returned by SearchAvailablePhoneNumbers
#[derive(Debug, PartialEq)]
pub enum SearchAvailablePhoneNumbersError {
    /// <p>You don't have permissions to perform the requested operation.</p>
    AccessDenied(String),
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
                "AccessDeniedException" => {
                    return RusotoError::Service(SearchAvailablePhoneNumbersError::AccessDenied(
                        err.msg,
                    ))
                }
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SearchAvailablePhoneNumbersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SearchAvailablePhoneNumbersError::AccessDenied(ref cause) => write!(f, "{}", cause),
            SearchAvailablePhoneNumbersError::BadRequest(ref cause) => write!(f, "{}", cause),
            SearchAvailablePhoneNumbersError::Forbidden(ref cause) => write!(f, "{}", cause),
            SearchAvailablePhoneNumbersError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            SearchAvailablePhoneNumbersError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            SearchAvailablePhoneNumbersError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            SearchAvailablePhoneNumbersError::UnauthorizedClient(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for SearchAvailablePhoneNumbersError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateAccountError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateAccountError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateAccountError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateAccountError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateAccountError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            UpdateAccountError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateAccountError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            UpdateAccountError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateAccountError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateAccountSettingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateAccountSettingsError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateAccountSettingsError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateAccountSettingsError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateAccountSettingsError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateAccountSettingsError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            UpdateAccountSettingsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateAccountSettingsError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            UpdateAccountSettingsError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateAccountSettingsError {}
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
    /// <p>The client exceeded its request rate limit.</p>
    ThrottledClient(String),
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
                "ThrottledClientException" => {
                    return RusotoError::Service(UpdateBotError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(UpdateBotError::UnauthorizedClient(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateBotError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateBotError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateBotError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateBotError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateBotError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            UpdateBotError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateBotError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            UpdateBotError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateBotError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateGlobalSettingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateGlobalSettingsError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateGlobalSettingsError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateGlobalSettingsError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            UpdateGlobalSettingsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateGlobalSettingsError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            UpdateGlobalSettingsError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateGlobalSettingsError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdatePhoneNumberError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdatePhoneNumberError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdatePhoneNumberError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdatePhoneNumberError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdatePhoneNumberError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            UpdatePhoneNumberError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdatePhoneNumberError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            UpdatePhoneNumberError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdatePhoneNumberError {}
/// Errors returned by UpdatePhoneNumberSettings
#[derive(Debug, PartialEq)]
pub enum UpdatePhoneNumberSettingsError {
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

impl UpdatePhoneNumberSettingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdatePhoneNumberSettingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdatePhoneNumberSettingsError::BadRequest(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdatePhoneNumberSettingsError::Forbidden(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(UpdatePhoneNumberSettingsError::ServiceFailure(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        UpdatePhoneNumberSettingsError::ServiceUnavailable(err.msg),
                    )
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(UpdatePhoneNumberSettingsError::ThrottledClient(
                        err.msg,
                    ))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(
                        UpdatePhoneNumberSettingsError::UnauthorizedClient(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdatePhoneNumberSettingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdatePhoneNumberSettingsError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdatePhoneNumberSettingsError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdatePhoneNumberSettingsError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            UpdatePhoneNumberSettingsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdatePhoneNumberSettingsError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            UpdatePhoneNumberSettingsError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdatePhoneNumberSettingsError {}
/// Errors returned by UpdateRoom
#[derive(Debug, PartialEq)]
pub enum UpdateRoomError {
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

impl UpdateRoomError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateRoomError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateRoomError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateRoomError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateRoomError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(UpdateRoomError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateRoomError::ServiceUnavailable(err.msg))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(UpdateRoomError::ThrottledClient(err.msg))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(UpdateRoomError::UnauthorizedClient(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateRoomError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateRoomError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateRoomError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateRoomError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateRoomError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            UpdateRoomError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateRoomError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            UpdateRoomError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateRoomError {}
/// Errors returned by UpdateRoomMembership
#[derive(Debug, PartialEq)]
pub enum UpdateRoomMembershipError {
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

impl UpdateRoomMembershipError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateRoomMembershipError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateRoomMembershipError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateRoomMembershipError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateRoomMembershipError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(UpdateRoomMembershipError::ServiceFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateRoomMembershipError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(UpdateRoomMembershipError::ThrottledClient(
                        err.msg,
                    ))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(UpdateRoomMembershipError::UnauthorizedClient(
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
impl fmt::Display for UpdateRoomMembershipError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateRoomMembershipError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateRoomMembershipError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateRoomMembershipError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateRoomMembershipError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            UpdateRoomMembershipError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateRoomMembershipError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            UpdateRoomMembershipError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateRoomMembershipError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateUserError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateUserError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateUserError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateUserError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            UpdateUserError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateUserError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            UpdateUserError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateUserError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateUserSettingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateUserSettingsError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateUserSettingsError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateUserSettingsError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateUserSettingsError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            UpdateUserSettingsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateUserSettingsError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            UpdateUserSettingsError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateUserSettingsError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateVoiceConnectorError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateVoiceConnectorError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateVoiceConnectorError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateVoiceConnectorError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateVoiceConnectorError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            UpdateVoiceConnectorError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateVoiceConnectorError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            UpdateVoiceConnectorError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateVoiceConnectorError {}
/// Errors returned by UpdateVoiceConnectorGroup
#[derive(Debug, PartialEq)]
pub enum UpdateVoiceConnectorGroupError {
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

impl UpdateVoiceConnectorGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateVoiceConnectorGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateVoiceConnectorGroupError::BadRequest(
                        err.msg,
                    ))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateVoiceConnectorGroupError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateVoiceConnectorGroupError::Forbidden(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateVoiceConnectorGroupError::NotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(UpdateVoiceConnectorGroupError::ServiceFailure(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        UpdateVoiceConnectorGroupError::ServiceUnavailable(err.msg),
                    )
                }
                "ThrottledClientException" => {
                    return RusotoError::Service(UpdateVoiceConnectorGroupError::ThrottledClient(
                        err.msg,
                    ))
                }
                "UnauthorizedClientException" => {
                    return RusotoError::Service(
                        UpdateVoiceConnectorGroupError::UnauthorizedClient(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateVoiceConnectorGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateVoiceConnectorGroupError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateVoiceConnectorGroupError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateVoiceConnectorGroupError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateVoiceConnectorGroupError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateVoiceConnectorGroupError::ServiceFailure(ref cause) => write!(f, "{}", cause),
            UpdateVoiceConnectorGroupError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateVoiceConnectorGroupError::ThrottledClient(ref cause) => write!(f, "{}", cause),
            UpdateVoiceConnectorGroupError::UnauthorizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateVoiceConnectorGroupError {}
/// Trait representing the capabilities of the Amazon Chime API. Amazon Chime clients implement this trait.
#[async_trait]
pub trait Chime {
    /// <p>Associates a phone number with the specified Amazon Chime user.</p>
    async fn associate_phone_number_with_user(
        &self,
        input: AssociatePhoneNumberWithUserRequest,
    ) -> Result<AssociatePhoneNumberWithUserResponse, RusotoError<AssociatePhoneNumberWithUserError>>;

    /// <p>Associates phone numbers with the specified Amazon Chime Voice Connector.</p>
    async fn associate_phone_numbers_with_voice_connector(
        &self,
        input: AssociatePhoneNumbersWithVoiceConnectorRequest,
    ) -> Result<
        AssociatePhoneNumbersWithVoiceConnectorResponse,
        RusotoError<AssociatePhoneNumbersWithVoiceConnectorError>,
    >;

    /// <p>Associates phone numbers with the specified Amazon Chime Voice Connector group.</p>
    async fn associate_phone_numbers_with_voice_connector_group(
        &self,
        input: AssociatePhoneNumbersWithVoiceConnectorGroupRequest,
    ) -> Result<
        AssociatePhoneNumbersWithVoiceConnectorGroupResponse,
        RusotoError<AssociatePhoneNumbersWithVoiceConnectorGroupError>,
    >;

    /// <p>Associates the specified sign-in delegate groups with the specified Amazon Chime account.</p>
    async fn associate_signin_delegate_groups_with_account(
        &self,
        input: AssociateSigninDelegateGroupsWithAccountRequest,
    ) -> Result<
        AssociateSigninDelegateGroupsWithAccountResponse,
        RusotoError<AssociateSigninDelegateGroupsWithAccountError>,
    >;

    /// <p>Creates up to 100 new attendees for an active Amazon Chime SDK meeting. For more information about the Amazon Chime SDK, see <a href="https://docs.aws.amazon.com/chime/latest/dg/meetings-sdk.html">Using the Amazon Chime SDK</a> in the <i>Amazon Chime Developer Guide</i>. </p>
    async fn batch_create_attendee(
        &self,
        input: BatchCreateAttendeeRequest,
    ) -> Result<BatchCreateAttendeeResponse, RusotoError<BatchCreateAttendeeError>>;

    /// <p>Adds up to 50 members to a chat room. Members can be either users or bots. The member role designates whether the member is a chat room administrator or a general chat room member.</p>
    async fn batch_create_room_membership(
        &self,
        input: BatchCreateRoomMembershipRequest,
    ) -> Result<BatchCreateRoomMembershipResponse, RusotoError<BatchCreateRoomMembershipError>>;

    /// <p>Moves phone numbers into the <b>Deletion queue</b>. Phone numbers must be disassociated from any users or Amazon Chime Voice Connectors before they can be deleted.</p> <p>Phone numbers remain in the <b>Deletion queue</b> for 7 days before they are deleted permanently.</p>
    async fn batch_delete_phone_number(
        &self,
        input: BatchDeletePhoneNumberRequest,
    ) -> Result<BatchDeletePhoneNumberResponse, RusotoError<BatchDeletePhoneNumberError>>;

    /// <p>Suspends up to 50 users from a <code>Team</code> or <code>EnterpriseLWA</code> Amazon Chime account. For more information about different account types, see <a href="https://docs.aws.amazon.com/chime/latest/ag/manage-chime-account.html">Managing Your Amazon Chime Accounts</a> in the <i>Amazon Chime Administration Guide</i>.</p> <p>Users suspended from a <code>Team</code> account are disassociated from the account, but they can continue to use Amazon Chime as free users. To remove the suspension from suspended <code>Team</code> account users, invite them to the <code>Team</code> account again. You can use the <a>InviteUsers</a> action to do so.</p> <p>Users suspended from an <code>EnterpriseLWA</code> account are immediately signed out of Amazon Chime and can no longer sign in. To remove the suspension from suspended <code>EnterpriseLWA</code> account users, use the <a>BatchUnsuspendUser</a> action. </p> <p>To sign out users without suspending them, use the <a>LogoutUser</a> action.</p>
    async fn batch_suspend_user(
        &self,
        input: BatchSuspendUserRequest,
    ) -> Result<BatchSuspendUserResponse, RusotoError<BatchSuspendUserError>>;

    /// <p>Removes the suspension from up to 50 previously suspended users for the specified Amazon Chime <code>EnterpriseLWA</code> account. Only users on <code>EnterpriseLWA</code> accounts can be unsuspended using this action. For more information about different account types, see <a href="https://docs.aws.amazon.com/chime/latest/ag/manage-chime-account.html">Managing Your Amazon Chime Accounts</a> in the <i>Amazon Chime Administration Guide</i>.</p> <p>Previously suspended users who are unsuspended using this action are returned to <code>Registered</code> status. Users who are not previously suspended are ignored.</p>
    async fn batch_unsuspend_user(
        &self,
        input: BatchUnsuspendUserRequest,
    ) -> Result<BatchUnsuspendUserResponse, RusotoError<BatchUnsuspendUserError>>;

    /// <p>Updates phone number product types or calling names. You can update one attribute at a time for each <code>UpdatePhoneNumberRequestItem</code>. For example, you can update either the product type or the calling name.</p> <p>For product types, choose from Amazon Chime Business Calling and Amazon Chime Voice Connector. For toll-free numbers, you must use the Amazon Chime Voice Connector product type.</p> <p>Updates to outbound calling names can take up to 72 hours to complete. Pending updates to outbound calling names must be complete before you can request another update.</p>
    async fn batch_update_phone_number(
        &self,
        input: BatchUpdatePhoneNumberRequest,
    ) -> Result<BatchUpdatePhoneNumberResponse, RusotoError<BatchUpdatePhoneNumberError>>;

    /// <p>Updates user details within the <a>UpdateUserRequestItem</a> object for up to 20 users for the specified Amazon Chime account. Currently, only <code>LicenseType</code> updates are supported for this action.</p>
    async fn batch_update_user(
        &self,
        input: BatchUpdateUserRequest,
    ) -> Result<BatchUpdateUserResponse, RusotoError<BatchUpdateUserError>>;

    /// <p>Creates an Amazon Chime account under the administrator's AWS account. Only <code>Team</code> account types are currently supported for this action. For more information about different account types, see <a href="https://docs.aws.amazon.com/chime/latest/ag/manage-chime-account.html">Managing Your Amazon Chime Accounts</a> in the <i>Amazon Chime Administration Guide</i>.</p>
    async fn create_account(
        &self,
        input: CreateAccountRequest,
    ) -> Result<CreateAccountResponse, RusotoError<CreateAccountError>>;

    /// <p>Creates a new attendee for an active Amazon Chime SDK meeting. For more information about the Amazon Chime SDK, see <a href="https://docs.aws.amazon.com/chime/latest/dg/meetings-sdk.html">Using the Amazon Chime SDK</a> in the <i>Amazon Chime Developer Guide</i>.</p>
    async fn create_attendee(
        &self,
        input: CreateAttendeeRequest,
    ) -> Result<CreateAttendeeResponse, RusotoError<CreateAttendeeError>>;

    /// <p>Creates a bot for an Amazon Chime Enterprise account.</p>
    async fn create_bot(
        &self,
        input: CreateBotRequest,
    ) -> Result<CreateBotResponse, RusotoError<CreateBotError>>;

    /// <p>Creates a new Amazon Chime SDK meeting in the specified media Region with no initial attendees. For more information about the Amazon Chime SDK, see <a href="https://docs.aws.amazon.com/chime/latest/dg/meetings-sdk.html">Using the Amazon Chime SDK</a> in the <i>Amazon Chime Developer Guide</i>.</p>
    async fn create_meeting(
        &self,
        input: CreateMeetingRequest,
    ) -> Result<CreateMeetingResponse, RusotoError<CreateMeetingError>>;

    /// <p>Creates an order for phone numbers to be provisioned. Choose from Amazon Chime Business Calling and Amazon Chime Voice Connector product types. For toll-free numbers, you must use the Amazon Chime Voice Connector product type.</p>
    async fn create_phone_number_order(
        &self,
        input: CreatePhoneNumberOrderRequest,
    ) -> Result<CreatePhoneNumberOrderResponse, RusotoError<CreatePhoneNumberOrderError>>;

    /// <p>Creates a chat room for the specified Amazon Chime account.</p>
    async fn create_room(
        &self,
        input: CreateRoomRequest,
    ) -> Result<CreateRoomResponse, RusotoError<CreateRoomError>>;

    /// <p>Adds a member to a chat room. A member can be either a user or a bot. The member role designates whether the member is a chat room administrator or a general chat room member.</p>
    async fn create_room_membership(
        &self,
        input: CreateRoomMembershipRequest,
    ) -> Result<CreateRoomMembershipResponse, RusotoError<CreateRoomMembershipError>>;

    /// <p>Creates a user under the specified Amazon Chime account.</p>
    async fn create_user(
        &self,
        input: CreateUserRequest,
    ) -> Result<CreateUserResponse, RusotoError<CreateUserError>>;

    /// <p>Creates an Amazon Chime Voice Connector under the administrator's AWS account. You can choose to create an Amazon Chime Voice Connector in a specific AWS Region.</p> <p>Enabling <a>CreateVoiceConnectorRequest$RequireEncryption</a> configures your Amazon Chime Voice Connector to use TLS transport for SIP signaling and Secure RTP (SRTP) for media. Inbound calls use TLS transport, and unencrypted outbound calls are blocked.</p>
    async fn create_voice_connector(
        &self,
        input: CreateVoiceConnectorRequest,
    ) -> Result<CreateVoiceConnectorResponse, RusotoError<CreateVoiceConnectorError>>;

    /// <p>Creates an Amazon Chime Voice Connector group under the administrator's AWS account. You can associate up to three existing Amazon Chime Voice Connectors with the Amazon Chime Voice Connector group by including <code>VoiceConnectorItems</code> in the request.</p> <p>You can include Amazon Chime Voice Connectors from different AWS Regions in your group. This creates a fault tolerant mechanism for fallback in case of availability events.</p>
    async fn create_voice_connector_group(
        &self,
        input: CreateVoiceConnectorGroupRequest,
    ) -> Result<CreateVoiceConnectorGroupResponse, RusotoError<CreateVoiceConnectorGroupError>>;

    /// <p>Deletes the specified Amazon Chime account. You must suspend all users before deleting a <code>Team</code> account. You can use the <a>BatchSuspendUser</a> action to do so.</p> <p>For <code>EnterpriseLWA</code> and <code>EnterpriseAD</code> accounts, you must release the claimed domains for your Amazon Chime account before deletion. As soon as you release the domain, all users under that account are suspended.</p> <p>Deleted accounts appear in your <code>Disabled</code> accounts list for 90 days. To restore a deleted account from your <code>Disabled</code> accounts list, you must contact AWS Support.</p> <p>After 90 days, deleted accounts are permanently removed from your <code>Disabled</code> accounts list.</p>
    async fn delete_account(
        &self,
        input: DeleteAccountRequest,
    ) -> Result<DeleteAccountResponse, RusotoError<DeleteAccountError>>;

    /// <p>Deletes an attendee from the specified Amazon Chime SDK meeting and deletes their <code>JoinToken</code>. Attendees are automatically deleted when a Amazon Chime SDK meeting is deleted. For more information about the Amazon Chime SDK, see <a href="https://docs.aws.amazon.com/chime/latest/dg/meetings-sdk.html">Using the Amazon Chime SDK</a> in the <i>Amazon Chime Developer Guide</i>.</p>
    async fn delete_attendee(
        &self,
        input: DeleteAttendeeRequest,
    ) -> Result<(), RusotoError<DeleteAttendeeError>>;

    /// <p>Deletes the events configuration that allows a bot to receive outgoing events.</p>
    async fn delete_events_configuration(
        &self,
        input: DeleteEventsConfigurationRequest,
    ) -> Result<(), RusotoError<DeleteEventsConfigurationError>>;

    /// <p>Deletes the specified Amazon Chime SDK meeting. When a meeting is deleted, its attendees are also deleted and clients can no longer join it. For more information about the Amazon Chime SDK, see <a href="https://docs.aws.amazon.com/chime/latest/dg/meetings-sdk.html">Using the Amazon Chime SDK</a> in the <i>Amazon Chime Developer Guide</i>.</p>
    async fn delete_meeting(
        &self,
        input: DeleteMeetingRequest,
    ) -> Result<(), RusotoError<DeleteMeetingError>>;

    /// <p>Moves the specified phone number into the <b>Deletion queue</b>. A phone number must be disassociated from any users or Amazon Chime Voice Connectors before it can be deleted.</p> <p>Deleted phone numbers remain in the <b>Deletion queue</b> for 7 days before they are deleted permanently.</p>
    async fn delete_phone_number(
        &self,
        input: DeletePhoneNumberRequest,
    ) -> Result<(), RusotoError<DeletePhoneNumberError>>;

    /// <p>Deletes a chat room.</p>
    async fn delete_room(
        &self,
        input: DeleteRoomRequest,
    ) -> Result<(), RusotoError<DeleteRoomError>>;

    /// <p>Removes a member from a chat room.</p>
    async fn delete_room_membership(
        &self,
        input: DeleteRoomMembershipRequest,
    ) -> Result<(), RusotoError<DeleteRoomMembershipError>>;

    /// <p>Deletes the specified Amazon Chime Voice Connector. Any phone numbers associated with the Amazon Chime Voice Connector must be disassociated from it before it can be deleted.</p>
    async fn delete_voice_connector(
        &self,
        input: DeleteVoiceConnectorRequest,
    ) -> Result<(), RusotoError<DeleteVoiceConnectorError>>;

    /// <p>Deletes the specified Amazon Chime Voice Connector group. Any <code>VoiceConnectorItems</code> and phone numbers associated with the group must be removed before it can be deleted.</p>
    async fn delete_voice_connector_group(
        &self,
        input: DeleteVoiceConnectorGroupRequest,
    ) -> Result<(), RusotoError<DeleteVoiceConnectorGroupError>>;

    /// <p>Deletes the origination settings for the specified Amazon Chime Voice Connector.</p>
    async fn delete_voice_connector_origination(
        &self,
        input: DeleteVoiceConnectorOriginationRequest,
    ) -> Result<(), RusotoError<DeleteVoiceConnectorOriginationError>>;

    /// <p>Deletes the streaming configuration for the specified Amazon Chime Voice Connector.</p>
    async fn delete_voice_connector_streaming_configuration(
        &self,
        input: DeleteVoiceConnectorStreamingConfigurationRequest,
    ) -> Result<(), RusotoError<DeleteVoiceConnectorStreamingConfigurationError>>;

    /// <p>Deletes the termination settings for the specified Amazon Chime Voice Connector.</p>
    async fn delete_voice_connector_termination(
        &self,
        input: DeleteVoiceConnectorTerminationRequest,
    ) -> Result<(), RusotoError<DeleteVoiceConnectorTerminationError>>;

    /// <p>Deletes the specified SIP credentials used by your equipment to authenticate during call termination.</p>
    async fn delete_voice_connector_termination_credentials(
        &self,
        input: DeleteVoiceConnectorTerminationCredentialsRequest,
    ) -> Result<(), RusotoError<DeleteVoiceConnectorTerminationCredentialsError>>;

    /// <p>Disassociates the primary provisioned phone number from the specified Amazon Chime user.</p>
    async fn disassociate_phone_number_from_user(
        &self,
        input: DisassociatePhoneNumberFromUserRequest,
    ) -> Result<
        DisassociatePhoneNumberFromUserResponse,
        RusotoError<DisassociatePhoneNumberFromUserError>,
    >;

    /// <p>Disassociates the specified phone numbers from the specified Amazon Chime Voice Connector.</p>
    async fn disassociate_phone_numbers_from_voice_connector(
        &self,
        input: DisassociatePhoneNumbersFromVoiceConnectorRequest,
    ) -> Result<
        DisassociatePhoneNumbersFromVoiceConnectorResponse,
        RusotoError<DisassociatePhoneNumbersFromVoiceConnectorError>,
    >;

    /// <p>Disassociates the specified phone numbers from the specified Amazon Chime Voice Connector group.</p>
    async fn disassociate_phone_numbers_from_voice_connector_group(
        &self,
        input: DisassociatePhoneNumbersFromVoiceConnectorGroupRequest,
    ) -> Result<
        DisassociatePhoneNumbersFromVoiceConnectorGroupResponse,
        RusotoError<DisassociatePhoneNumbersFromVoiceConnectorGroupError>,
    >;

    /// <p>Disassociates the specified sign-in delegate groups from the specified Amazon Chime account.</p>
    async fn disassociate_signin_delegate_groups_from_account(
        &self,
        input: DisassociateSigninDelegateGroupsFromAccountRequest,
    ) -> Result<
        DisassociateSigninDelegateGroupsFromAccountResponse,
        RusotoError<DisassociateSigninDelegateGroupsFromAccountError>,
    >;

    /// <p>Retrieves details for the specified Amazon Chime account, such as account type and supported licenses.</p>
    async fn get_account(
        &self,
        input: GetAccountRequest,
    ) -> Result<GetAccountResponse, RusotoError<GetAccountError>>;

    /// <p>Retrieves account settings for the specified Amazon Chime account ID, such as remote control and dial out settings. For more information about these settings, see <a href="https://docs.aws.amazon.com/chime/latest/ag/policies.html">Use the Policies Page</a> in the <i>Amazon Chime Administration Guide</i>.</p>
    async fn get_account_settings(
        &self,
        input: GetAccountSettingsRequest,
    ) -> Result<GetAccountSettingsResponse, RusotoError<GetAccountSettingsError>>;

    /// <p>Gets the Amazon Chime SDK attendee details for a specified meeting ID and attendee ID. For more information about the Amazon Chime SDK, see <a href="https://docs.aws.amazon.com/chime/latest/dg/meetings-sdk.html">Using the Amazon Chime SDK</a> in the <i>Amazon Chime Developer Guide</i>.</p>
    async fn get_attendee(
        &self,
        input: GetAttendeeRequest,
    ) -> Result<GetAttendeeResponse, RusotoError<GetAttendeeError>>;

    /// <p>Retrieves details for the specified bot, such as bot email address, bot type, status, and display name.</p>
    async fn get_bot(
        &self,
        input: GetBotRequest,
    ) -> Result<GetBotResponse, RusotoError<GetBotError>>;

    /// <p>Gets details for an events configuration that allows a bot to receive outgoing events, such as an HTTPS endpoint or Lambda function ARN. </p>
    async fn get_events_configuration(
        &self,
        input: GetEventsConfigurationRequest,
    ) -> Result<GetEventsConfigurationResponse, RusotoError<GetEventsConfigurationError>>;

    /// <p>Retrieves global settings for the administrator's AWS account, such as Amazon Chime Business Calling and Amazon Chime Voice Connector settings.</p>
    async fn get_global_settings(
        &self,
    ) -> Result<GetGlobalSettingsResponse, RusotoError<GetGlobalSettingsError>>;

    /// <p>Gets the Amazon Chime SDK meeting details for the specified meeting ID. For more information about the Amazon Chime SDK, see <a href="https://docs.aws.amazon.com/chime/latest/dg/meetings-sdk.html">Using the Amazon Chime SDK</a> in the <i>Amazon Chime Developer Guide</i>.</p>
    async fn get_meeting(
        &self,
        input: GetMeetingRequest,
    ) -> Result<GetMeetingResponse, RusotoError<GetMeetingError>>;

    /// <p>Retrieves details for the specified phone number ID, such as associations, capabilities, and product type.</p>
    async fn get_phone_number(
        &self,
        input: GetPhoneNumberRequest,
    ) -> Result<GetPhoneNumberResponse, RusotoError<GetPhoneNumberError>>;

    /// <p>Retrieves details for the specified phone number order, such as order creation timestamp, phone numbers in E.164 format, product type, and order status.</p>
    async fn get_phone_number_order(
        &self,
        input: GetPhoneNumberOrderRequest,
    ) -> Result<GetPhoneNumberOrderResponse, RusotoError<GetPhoneNumberOrderError>>;

    /// <p>Retrieves the phone number settings for the administrator's AWS account, such as the default outbound calling name.</p>
    async fn get_phone_number_settings(
        &self,
    ) -> Result<GetPhoneNumberSettingsResponse, RusotoError<GetPhoneNumberSettingsError>>;

    /// <p>Retrieves room details, such as the room name.</p>
    async fn get_room(
        &self,
        input: GetRoomRequest,
    ) -> Result<GetRoomResponse, RusotoError<GetRoomError>>;

    /// <p>Retrieves details for the specified user ID, such as primary email address, license type, and personal meeting PIN.</p> <p>To retrieve user details with an email address instead of a user ID, use the <a>ListUsers</a> action, and then filter by email address.</p>
    async fn get_user(
        &self,
        input: GetUserRequest,
    ) -> Result<GetUserResponse, RusotoError<GetUserError>>;

    /// <p>Retrieves settings for the specified user ID, such as any associated phone number settings.</p>
    async fn get_user_settings(
        &self,
        input: GetUserSettingsRequest,
    ) -> Result<GetUserSettingsResponse, RusotoError<GetUserSettingsError>>;

    /// <p>Retrieves details for the specified Amazon Chime Voice Connector, such as timestamps, name, outbound host, and encryption requirements.</p>
    async fn get_voice_connector(
        &self,
        input: GetVoiceConnectorRequest,
    ) -> Result<GetVoiceConnectorResponse, RusotoError<GetVoiceConnectorError>>;

    /// <p>Retrieves details for the specified Amazon Chime Voice Connector group, such as timestamps, name, and associated <code>VoiceConnectorItems</code>.</p>
    async fn get_voice_connector_group(
        &self,
        input: GetVoiceConnectorGroupRequest,
    ) -> Result<GetVoiceConnectorGroupResponse, RusotoError<GetVoiceConnectorGroupError>>;

    /// <p>Retrieves the logging configuration details for the specified Amazon Chime Voice Connector. Shows whether SIP message logs are enabled for sending to Amazon CloudWatch Logs.</p>
    async fn get_voice_connector_logging_configuration(
        &self,
        input: GetVoiceConnectorLoggingConfigurationRequest,
    ) -> Result<
        GetVoiceConnectorLoggingConfigurationResponse,
        RusotoError<GetVoiceConnectorLoggingConfigurationError>,
    >;

    /// <p>Retrieves origination setting details for the specified Amazon Chime Voice Connector.</p>
    async fn get_voice_connector_origination(
        &self,
        input: GetVoiceConnectorOriginationRequest,
    ) -> Result<GetVoiceConnectorOriginationResponse, RusotoError<GetVoiceConnectorOriginationError>>;

    /// <p>Retrieves the streaming configuration details for the specified Amazon Chime Voice Connector. Shows whether media streaming is enabled for sending to Amazon Kinesis. It also shows the retention period, in hours, for the Amazon Kinesis data.</p>
    async fn get_voice_connector_streaming_configuration(
        &self,
        input: GetVoiceConnectorStreamingConfigurationRequest,
    ) -> Result<
        GetVoiceConnectorStreamingConfigurationResponse,
        RusotoError<GetVoiceConnectorStreamingConfigurationError>,
    >;

    /// <p>Retrieves termination setting details for the specified Amazon Chime Voice Connector.</p>
    async fn get_voice_connector_termination(
        &self,
        input: GetVoiceConnectorTerminationRequest,
    ) -> Result<GetVoiceConnectorTerminationResponse, RusotoError<GetVoiceConnectorTerminationError>>;

    /// <p>Retrieves information about the last time a SIP <code>OPTIONS</code> ping was received from your SIP infrastructure for the specified Amazon Chime Voice Connector.</p>
    async fn get_voice_connector_termination_health(
        &self,
        input: GetVoiceConnectorTerminationHealthRequest,
    ) -> Result<
        GetVoiceConnectorTerminationHealthResponse,
        RusotoError<GetVoiceConnectorTerminationHealthError>,
    >;

    /// <p>Sends email to a maximum of 50 users, inviting them to the specified Amazon Chime <code>Team</code> account. Only <code>Team</code> account types are currently supported for this action. </p>
    async fn invite_users(
        &self,
        input: InviteUsersRequest,
    ) -> Result<InviteUsersResponse, RusotoError<InviteUsersError>>;

    /// <p>Lists the Amazon Chime accounts under the administrator's AWS account. You can filter accounts by account name prefix. To find out which Amazon Chime account a user belongs to, you can filter by the user's email address, which returns one account result.</p>
    async fn list_accounts(
        &self,
        input: ListAccountsRequest,
    ) -> Result<ListAccountsResponse, RusotoError<ListAccountsError>>;

    /// <p>Lists the attendees for the specified Amazon Chime SDK meeting. For more information about the Amazon Chime SDK, see <a href="https://docs.aws.amazon.com/chime/latest/dg/meetings-sdk.html">Using the Amazon Chime SDK</a> in the <i>Amazon Chime Developer Guide</i>.</p>
    async fn list_attendees(
        &self,
        input: ListAttendeesRequest,
    ) -> Result<ListAttendeesResponse, RusotoError<ListAttendeesError>>;

    /// <p>Lists the bots associated with the administrator's Amazon Chime Enterprise account ID.</p>
    async fn list_bots(
        &self,
        input: ListBotsRequest,
    ) -> Result<ListBotsResponse, RusotoError<ListBotsError>>;

    /// <p>Lists up to 100 active Amazon Chime SDK meetings. For more information about the Amazon Chime SDK, see <a href="https://docs.aws.amazon.com/chime/latest/dg/meetings-sdk.html">Using the Amazon Chime SDK</a> in the <i>Amazon Chime Developer Guide</i>.</p>
    async fn list_meetings(
        &self,
        input: ListMeetingsRequest,
    ) -> Result<ListMeetingsResponse, RusotoError<ListMeetingsError>>;

    /// <p>Lists the phone number orders for the administrator's Amazon Chime account.</p>
    async fn list_phone_number_orders(
        &self,
        input: ListPhoneNumberOrdersRequest,
    ) -> Result<ListPhoneNumberOrdersResponse, RusotoError<ListPhoneNumberOrdersError>>;

    /// <p>Lists the phone numbers for the specified Amazon Chime account, Amazon Chime user, Amazon Chime Voice Connector, or Amazon Chime Voice Connector group.</p>
    async fn list_phone_numbers(
        &self,
        input: ListPhoneNumbersRequest,
    ) -> Result<ListPhoneNumbersResponse, RusotoError<ListPhoneNumbersError>>;

    /// <p>Lists the membership details for the specified room, such as the members' IDs, email addresses, and names.</p>
    async fn list_room_memberships(
        &self,
        input: ListRoomMembershipsRequest,
    ) -> Result<ListRoomMembershipsResponse, RusotoError<ListRoomMembershipsError>>;

    /// <p>Lists the room details for the specified Amazon Chime account. Optionally, filter the results by a member ID (user ID or bot ID) to see a list of rooms that the member belongs to.</p>
    async fn list_rooms(
        &self,
        input: ListRoomsRequest,
    ) -> Result<ListRoomsResponse, RusotoError<ListRoomsError>>;

    /// <p>Lists the users that belong to the specified Amazon Chime account. You can specify an email address to list only the user that the email address belongs to.</p>
    async fn list_users(
        &self,
        input: ListUsersRequest,
    ) -> Result<ListUsersResponse, RusotoError<ListUsersError>>;

    /// <p>Lists the Amazon Chime Voice Connector groups for the administrator's AWS account.</p>
    async fn list_voice_connector_groups(
        &self,
        input: ListVoiceConnectorGroupsRequest,
    ) -> Result<ListVoiceConnectorGroupsResponse, RusotoError<ListVoiceConnectorGroupsError>>;

    /// <p>Lists the SIP credentials for the specified Amazon Chime Voice Connector.</p>
    async fn list_voice_connector_termination_credentials(
        &self,
        input: ListVoiceConnectorTerminationCredentialsRequest,
    ) -> Result<
        ListVoiceConnectorTerminationCredentialsResponse,
        RusotoError<ListVoiceConnectorTerminationCredentialsError>,
    >;

    /// <p>Lists the Amazon Chime Voice Connectors for the administrator's AWS account.</p>
    async fn list_voice_connectors(
        &self,
        input: ListVoiceConnectorsRequest,
    ) -> Result<ListVoiceConnectorsResponse, RusotoError<ListVoiceConnectorsError>>;

    /// <p>Logs out the specified user from all of the devices they are currently logged into.</p>
    async fn logout_user(
        &self,
        input: LogoutUserRequest,
    ) -> Result<LogoutUserResponse, RusotoError<LogoutUserError>>;

    /// <p>Creates an events configuration that allows a bot to receive outgoing events sent by Amazon Chime. Choose either an HTTPS endpoint or a Lambda function ARN. For more information, see <a>Bot</a>.</p>
    async fn put_events_configuration(
        &self,
        input: PutEventsConfigurationRequest,
    ) -> Result<PutEventsConfigurationResponse, RusotoError<PutEventsConfigurationError>>;

    /// <p>Adds a logging configuration for the specified Amazon Chime Voice Connector. The logging configuration specifies whether SIP message logs are enabled for sending to Amazon CloudWatch Logs.</p>
    async fn put_voice_connector_logging_configuration(
        &self,
        input: PutVoiceConnectorLoggingConfigurationRequest,
    ) -> Result<
        PutVoiceConnectorLoggingConfigurationResponse,
        RusotoError<PutVoiceConnectorLoggingConfigurationError>,
    >;

    /// <p>Adds origination settings for the specified Amazon Chime Voice Connector.</p>
    async fn put_voice_connector_origination(
        &self,
        input: PutVoiceConnectorOriginationRequest,
    ) -> Result<PutVoiceConnectorOriginationResponse, RusotoError<PutVoiceConnectorOriginationError>>;

    /// <p>Adds a streaming configuration for the specified Amazon Chime Voice Connector. The streaming configuration specifies whether media streaming is enabled for sending to Amazon Kinesis. It also sets the retention period, in hours, for the Amazon Kinesis data.</p>
    async fn put_voice_connector_streaming_configuration(
        &self,
        input: PutVoiceConnectorStreamingConfigurationRequest,
    ) -> Result<
        PutVoiceConnectorStreamingConfigurationResponse,
        RusotoError<PutVoiceConnectorStreamingConfigurationError>,
    >;

    /// <p>Adds termination settings for the specified Amazon Chime Voice Connector.</p>
    async fn put_voice_connector_termination(
        &self,
        input: PutVoiceConnectorTerminationRequest,
    ) -> Result<PutVoiceConnectorTerminationResponse, RusotoError<PutVoiceConnectorTerminationError>>;

    /// <p>Adds termination SIP credentials for the specified Amazon Chime Voice Connector.</p>
    async fn put_voice_connector_termination_credentials(
        &self,
        input: PutVoiceConnectorTerminationCredentialsRequest,
    ) -> Result<(), RusotoError<PutVoiceConnectorTerminationCredentialsError>>;

    /// <p>Regenerates the security token for a bot.</p>
    async fn regenerate_security_token(
        &self,
        input: RegenerateSecurityTokenRequest,
    ) -> Result<RegenerateSecurityTokenResponse, RusotoError<RegenerateSecurityTokenError>>;

    /// <p>Resets the personal meeting PIN for the specified user on an Amazon Chime account. Returns the <a>User</a> object with the updated personal meeting PIN.</p>
    async fn reset_personal_pin(
        &self,
        input: ResetPersonalPINRequest,
    ) -> Result<ResetPersonalPINResponse, RusotoError<ResetPersonalPINError>>;

    /// <p>Moves a phone number from the <b>Deletion queue</b> back into the phone number <b>Inventory</b>.</p>
    async fn restore_phone_number(
        &self,
        input: RestorePhoneNumberRequest,
    ) -> Result<RestorePhoneNumberResponse, RusotoError<RestorePhoneNumberError>>;

    /// <p>Searches phone numbers that can be ordered.</p>
    async fn search_available_phone_numbers(
        &self,
        input: SearchAvailablePhoneNumbersRequest,
    ) -> Result<SearchAvailablePhoneNumbersResponse, RusotoError<SearchAvailablePhoneNumbersError>>;

    /// <p>Updates account details for the specified Amazon Chime account. Currently, only account name updates are supported for this action.</p>
    async fn update_account(
        &self,
        input: UpdateAccountRequest,
    ) -> Result<UpdateAccountResponse, RusotoError<UpdateAccountError>>;

    /// <p>Updates the settings for the specified Amazon Chime account. You can update settings for remote control of shared screens, or for the dial-out option. For more information about these settings, see <a href="https://docs.aws.amazon.com/chime/latest/ag/policies.html">Use the Policies Page</a> in the <i>Amazon Chime Administration Guide</i>.</p>
    async fn update_account_settings(
        &self,
        input: UpdateAccountSettingsRequest,
    ) -> Result<UpdateAccountSettingsResponse, RusotoError<UpdateAccountSettingsError>>;

    /// <p>Updates the status of the specified bot, such as starting or stopping the bot from running in your Amazon Chime Enterprise account.</p>
    async fn update_bot(
        &self,
        input: UpdateBotRequest,
    ) -> Result<UpdateBotResponse, RusotoError<UpdateBotError>>;

    /// <p>Updates global settings for the administrator's AWS account, such as Amazon Chime Business Calling and Amazon Chime Voice Connector settings.</p>
    async fn update_global_settings(
        &self,
        input: UpdateGlobalSettingsRequest,
    ) -> Result<(), RusotoError<UpdateGlobalSettingsError>>;

    /// <p>Updates phone number details, such as product type or calling name, for the specified phone number ID. You can update one phone number detail at a time. For example, you can update either the product type or the calling name in one action.</p> <p>For toll-free numbers, you must use the Amazon Chime Voice Connector product type.</p> <p>Updates to outbound calling names can take up to 72 hours to complete. Pending updates to outbound calling names must be complete before you can request another update.</p>
    async fn update_phone_number(
        &self,
        input: UpdatePhoneNumberRequest,
    ) -> Result<UpdatePhoneNumberResponse, RusotoError<UpdatePhoneNumberError>>;

    /// <p>Updates the phone number settings for the administrator's AWS account, such as the default outbound calling name. You can update the default outbound calling name once every seven days. Outbound calling names can take up to 72 hours to update.</p>
    async fn update_phone_number_settings(
        &self,
        input: UpdatePhoneNumberSettingsRequest,
    ) -> Result<(), RusotoError<UpdatePhoneNumberSettingsError>>;

    /// <p>Updates room details, such as the room name.</p>
    async fn update_room(
        &self,
        input: UpdateRoomRequest,
    ) -> Result<UpdateRoomResponse, RusotoError<UpdateRoomError>>;

    /// <p>Updates room membership details, such as the member role. The member role designates whether the member is a chat room administrator or a general chat room member. The member role can be updated only for user IDs.</p>
    async fn update_room_membership(
        &self,
        input: UpdateRoomMembershipRequest,
    ) -> Result<UpdateRoomMembershipResponse, RusotoError<UpdateRoomMembershipError>>;

    /// <p>Updates user details for a specified user ID. Currently, only <code>LicenseType</code> updates are supported for this action.</p>
    async fn update_user(
        &self,
        input: UpdateUserRequest,
    ) -> Result<UpdateUserResponse, RusotoError<UpdateUserError>>;

    /// <p>Updates the settings for the specified user, such as phone number settings.</p>
    async fn update_user_settings(
        &self,
        input: UpdateUserSettingsRequest,
    ) -> Result<(), RusotoError<UpdateUserSettingsError>>;

    /// <p>Updates details for the specified Amazon Chime Voice Connector.</p>
    async fn update_voice_connector(
        &self,
        input: UpdateVoiceConnectorRequest,
    ) -> Result<UpdateVoiceConnectorResponse, RusotoError<UpdateVoiceConnectorError>>;

    /// <p>Updates details for the specified Amazon Chime Voice Connector group, such as the name and Amazon Chime Voice Connector priority ranking.</p>
    async fn update_voice_connector_group(
        &self,
        input: UpdateVoiceConnectorGroupRequest,
    ) -> Result<UpdateVoiceConnectorGroupResponse, RusotoError<UpdateVoiceConnectorGroupError>>;
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
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        ChimeClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> ChimeClient {
        ChimeClient { client, region }
    }
}

#[async_trait]
impl Chime for ChimeClient {
    /// <p>Associates a phone number with the specified Amazon Chime user.</p>
    async fn associate_phone_number_with_user(
        &self,
        input: AssociatePhoneNumberWithUserRequest,
    ) -> Result<AssociatePhoneNumberWithUserResponse, RusotoError<AssociatePhoneNumberWithUserError>>
    {
        let request_uri = format!(
            "/accounts/{account_id}/users/{user_id}",
            account_id = input.account_id,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        params.put("operation", "associate-phone-number");
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<AssociatePhoneNumberWithUserResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AssociatePhoneNumberWithUserError::from_response(response))
        }
    }

    /// <p>Associates phone numbers with the specified Amazon Chime Voice Connector.</p>
    async fn associate_phone_numbers_with_voice_connector(
        &self,
        input: AssociatePhoneNumbersWithVoiceConnectorRequest,
    ) -> Result<
        AssociatePhoneNumbersWithVoiceConnectorResponse,
        RusotoError<AssociatePhoneNumbersWithVoiceConnectorError>,
    > {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}",
            voice_connector_id = input.voice_connector_id
        );

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        params.put("operation", "associate-phone-numbers");
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<AssociatePhoneNumbersWithVoiceConnectorResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AssociatePhoneNumbersWithVoiceConnectorError::from_response(
                response,
            ))
        }
    }

    /// <p>Associates phone numbers with the specified Amazon Chime Voice Connector group.</p>
    async fn associate_phone_numbers_with_voice_connector_group(
        &self,
        input: AssociatePhoneNumbersWithVoiceConnectorGroupRequest,
    ) -> Result<
        AssociatePhoneNumbersWithVoiceConnectorGroupResponse,
        RusotoError<AssociatePhoneNumbersWithVoiceConnectorGroupError>,
    > {
        let request_uri = format!(
            "/voice-connector-groups/{voice_connector_group_id}",
            voice_connector_group_id = input.voice_connector_group_id
        );

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        params.put("operation", "associate-phone-numbers");
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<AssociatePhoneNumbersWithVoiceConnectorGroupResponse, _>(
            )?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AssociatePhoneNumbersWithVoiceConnectorGroupError::from_response(response))
        }
    }

    /// <p>Associates the specified sign-in delegate groups with the specified Amazon Chime account.</p>
    async fn associate_signin_delegate_groups_with_account(
        &self,
        input: AssociateSigninDelegateGroupsWithAccountRequest,
    ) -> Result<
        AssociateSigninDelegateGroupsWithAccountResponse,
        RusotoError<AssociateSigninDelegateGroupsWithAccountError>,
    > {
        let request_uri = format!("/accounts/{account_id}", account_id = input.account_id);

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        params.put("operation", "associate-signin-delegate-groups");
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<AssociateSigninDelegateGroupsWithAccountResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AssociateSigninDelegateGroupsWithAccountError::from_response(response))
        }
    }

    /// <p>Creates up to 100 new attendees for an active Amazon Chime SDK meeting. For more information about the Amazon Chime SDK, see <a href="https://docs.aws.amazon.com/chime/latest/dg/meetings-sdk.html">Using the Amazon Chime SDK</a> in the <i>Amazon Chime Developer Guide</i>. </p>
    async fn batch_create_attendee(
        &self,
        input: BatchCreateAttendeeRequest,
    ) -> Result<BatchCreateAttendeeResponse, RusotoError<BatchCreateAttendeeError>> {
        let request_uri = format!(
            "/meetings/{meeting_id}/attendees",
            meeting_id = input.meeting_id
        );

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        params.put("operation", "batch-create");
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<BatchCreateAttendeeResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(BatchCreateAttendeeError::from_response(response))
        }
    }

    /// <p>Adds up to 50 members to a chat room. Members can be either users or bots. The member role designates whether the member is a chat room administrator or a general chat room member.</p>
    async fn batch_create_room_membership(
        &self,
        input: BatchCreateRoomMembershipRequest,
    ) -> Result<BatchCreateRoomMembershipResponse, RusotoError<BatchCreateRoomMembershipError>>
    {
        let request_uri = format!(
            "/accounts/{account_id}/rooms/{room_id}/memberships",
            account_id = input.account_id,
            room_id = input.room_id
        );

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        params.put("operation", "batch-create");
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<BatchCreateRoomMembershipResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(BatchCreateRoomMembershipError::from_response(response))
        }
    }

    /// <p>Moves phone numbers into the <b>Deletion queue</b>. Phone numbers must be disassociated from any users or Amazon Chime Voice Connectors before they can be deleted.</p> <p>Phone numbers remain in the <b>Deletion queue</b> for 7 days before they are deleted permanently.</p>
    async fn batch_delete_phone_number(
        &self,
        input: BatchDeletePhoneNumberRequest,
    ) -> Result<BatchDeletePhoneNumberResponse, RusotoError<BatchDeletePhoneNumberError>> {
        let request_uri = "/phone-numbers";

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        params.put("operation", "batch-delete");
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<BatchDeletePhoneNumberResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(BatchDeletePhoneNumberError::from_response(response))
        }
    }

    /// <p>Suspends up to 50 users from a <code>Team</code> or <code>EnterpriseLWA</code> Amazon Chime account. For more information about different account types, see <a href="https://docs.aws.amazon.com/chime/latest/ag/manage-chime-account.html">Managing Your Amazon Chime Accounts</a> in the <i>Amazon Chime Administration Guide</i>.</p> <p>Users suspended from a <code>Team</code> account are disassociated from the account, but they can continue to use Amazon Chime as free users. To remove the suspension from suspended <code>Team</code> account users, invite them to the <code>Team</code> account again. You can use the <a>InviteUsers</a> action to do so.</p> <p>Users suspended from an <code>EnterpriseLWA</code> account are immediately signed out of Amazon Chime and can no longer sign in. To remove the suspension from suspended <code>EnterpriseLWA</code> account users, use the <a>BatchUnsuspendUser</a> action. </p> <p>To sign out users without suspending them, use the <a>LogoutUser</a> action.</p>
    async fn batch_suspend_user(
        &self,
        input: BatchSuspendUserRequest,
    ) -> Result<BatchSuspendUserResponse, RusotoError<BatchSuspendUserError>> {
        let request_uri = format!(
            "/accounts/{account_id}/users",
            account_id = input.account_id
        );

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        params.put("operation", "suspend");
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<BatchSuspendUserResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(BatchSuspendUserError::from_response(response))
        }
    }

    /// <p>Removes the suspension from up to 50 previously suspended users for the specified Amazon Chime <code>EnterpriseLWA</code> account. Only users on <code>EnterpriseLWA</code> accounts can be unsuspended using this action. For more information about different account types, see <a href="https://docs.aws.amazon.com/chime/latest/ag/manage-chime-account.html">Managing Your Amazon Chime Accounts</a> in the <i>Amazon Chime Administration Guide</i>.</p> <p>Previously suspended users who are unsuspended using this action are returned to <code>Registered</code> status. Users who are not previously suspended are ignored.</p>
    async fn batch_unsuspend_user(
        &self,
        input: BatchUnsuspendUserRequest,
    ) -> Result<BatchUnsuspendUserResponse, RusotoError<BatchUnsuspendUserError>> {
        let request_uri = format!(
            "/accounts/{account_id}/users",
            account_id = input.account_id
        );

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        params.put("operation", "unsuspend");
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<BatchUnsuspendUserResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(BatchUnsuspendUserError::from_response(response))
        }
    }

    /// <p>Updates phone number product types or calling names. You can update one attribute at a time for each <code>UpdatePhoneNumberRequestItem</code>. For example, you can update either the product type or the calling name.</p> <p>For product types, choose from Amazon Chime Business Calling and Amazon Chime Voice Connector. For toll-free numbers, you must use the Amazon Chime Voice Connector product type.</p> <p>Updates to outbound calling names can take up to 72 hours to complete. Pending updates to outbound calling names must be complete before you can request another update.</p>
    async fn batch_update_phone_number(
        &self,
        input: BatchUpdatePhoneNumberRequest,
    ) -> Result<BatchUpdatePhoneNumberResponse, RusotoError<BatchUpdatePhoneNumberError>> {
        let request_uri = "/phone-numbers";

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        params.put("operation", "batch-update");
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<BatchUpdatePhoneNumberResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(BatchUpdatePhoneNumberError::from_response(response))
        }
    }

    /// <p>Updates user details within the <a>UpdateUserRequestItem</a> object for up to 20 users for the specified Amazon Chime account. Currently, only <code>LicenseType</code> updates are supported for this action.</p>
    async fn batch_update_user(
        &self,
        input: BatchUpdateUserRequest,
    ) -> Result<BatchUpdateUserResponse, RusotoError<BatchUpdateUserError>> {
        let request_uri = format!(
            "/accounts/{account_id}/users",
            account_id = input.account_id
        );

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<BatchUpdateUserResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(BatchUpdateUserError::from_response(response))
        }
    }

    /// <p>Creates an Amazon Chime account under the administrator's AWS account. Only <code>Team</code> account types are currently supported for this action. For more information about different account types, see <a href="https://docs.aws.amazon.com/chime/latest/ag/manage-chime-account.html">Managing Your Amazon Chime Accounts</a> in the <i>Amazon Chime Administration Guide</i>.</p>
    async fn create_account(
        &self,
        input: CreateAccountRequest,
    ) -> Result<CreateAccountResponse, RusotoError<CreateAccountError>> {
        let request_uri = "/accounts";

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateAccountResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateAccountError::from_response(response))
        }
    }

    /// <p>Creates a new attendee for an active Amazon Chime SDK meeting. For more information about the Amazon Chime SDK, see <a href="https://docs.aws.amazon.com/chime/latest/dg/meetings-sdk.html">Using the Amazon Chime SDK</a> in the <i>Amazon Chime Developer Guide</i>.</p>
    async fn create_attendee(
        &self,
        input: CreateAttendeeRequest,
    ) -> Result<CreateAttendeeResponse, RusotoError<CreateAttendeeError>> {
        let request_uri = format!(
            "/meetings/{meeting_id}/attendees",
            meeting_id = input.meeting_id
        );

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateAttendeeResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateAttendeeError::from_response(response))
        }
    }

    /// <p>Creates a bot for an Amazon Chime Enterprise account.</p>
    async fn create_bot(
        &self,
        input: CreateBotRequest,
    ) -> Result<CreateBotResponse, RusotoError<CreateBotError>> {
        let request_uri = format!("/accounts/{account_id}/bots", account_id = input.account_id);

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateBotResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateBotError::from_response(response))
        }
    }

    /// <p>Creates a new Amazon Chime SDK meeting in the specified media Region with no initial attendees. For more information about the Amazon Chime SDK, see <a href="https://docs.aws.amazon.com/chime/latest/dg/meetings-sdk.html">Using the Amazon Chime SDK</a> in the <i>Amazon Chime Developer Guide</i>.</p>
    async fn create_meeting(
        &self,
        input: CreateMeetingRequest,
    ) -> Result<CreateMeetingResponse, RusotoError<CreateMeetingError>> {
        let request_uri = "/meetings";

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateMeetingResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateMeetingError::from_response(response))
        }
    }

    /// <p>Creates an order for phone numbers to be provisioned. Choose from Amazon Chime Business Calling and Amazon Chime Voice Connector product types. For toll-free numbers, you must use the Amazon Chime Voice Connector product type.</p>
    async fn create_phone_number_order(
        &self,
        input: CreatePhoneNumberOrderRequest,
    ) -> Result<CreatePhoneNumberOrderResponse, RusotoError<CreatePhoneNumberOrderError>> {
        let request_uri = "/phone-number-orders";

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreatePhoneNumberOrderResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreatePhoneNumberOrderError::from_response(response))
        }
    }

    /// <p>Creates a chat room for the specified Amazon Chime account.</p>
    async fn create_room(
        &self,
        input: CreateRoomRequest,
    ) -> Result<CreateRoomResponse, RusotoError<CreateRoomError>> {
        let request_uri = format!(
            "/accounts/{account_id}/rooms",
            account_id = input.account_id
        );

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateRoomResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateRoomError::from_response(response))
        }
    }

    /// <p>Adds a member to a chat room. A member can be either a user or a bot. The member role designates whether the member is a chat room administrator or a general chat room member.</p>
    async fn create_room_membership(
        &self,
        input: CreateRoomMembershipRequest,
    ) -> Result<CreateRoomMembershipResponse, RusotoError<CreateRoomMembershipError>> {
        let request_uri = format!(
            "/accounts/{account_id}/rooms/{room_id}/memberships",
            account_id = input.account_id,
            room_id = input.room_id
        );

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateRoomMembershipResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateRoomMembershipError::from_response(response))
        }
    }

    /// <p>Creates a user under the specified Amazon Chime account.</p>
    async fn create_user(
        &self,
        input: CreateUserRequest,
    ) -> Result<CreateUserResponse, RusotoError<CreateUserError>> {
        let request_uri = format!(
            "/accounts/{account_id}/users",
            account_id = input.account_id
        );

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        params.put("operation", "create");
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateUserResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateUserError::from_response(response))
        }
    }

    /// <p>Creates an Amazon Chime Voice Connector under the administrator's AWS account. You can choose to create an Amazon Chime Voice Connector in a specific AWS Region.</p> <p>Enabling <a>CreateVoiceConnectorRequest$RequireEncryption</a> configures your Amazon Chime Voice Connector to use TLS transport for SIP signaling and Secure RTP (SRTP) for media. Inbound calls use TLS transport, and unencrypted outbound calls are blocked.</p>
    async fn create_voice_connector(
        &self,
        input: CreateVoiceConnectorRequest,
    ) -> Result<CreateVoiceConnectorResponse, RusotoError<CreateVoiceConnectorError>> {
        let request_uri = "/voice-connectors";

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateVoiceConnectorResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateVoiceConnectorError::from_response(response))
        }
    }

    /// <p>Creates an Amazon Chime Voice Connector group under the administrator's AWS account. You can associate up to three existing Amazon Chime Voice Connectors with the Amazon Chime Voice Connector group by including <code>VoiceConnectorItems</code> in the request.</p> <p>You can include Amazon Chime Voice Connectors from different AWS Regions in your group. This creates a fault tolerant mechanism for fallback in case of availability events.</p>
    async fn create_voice_connector_group(
        &self,
        input: CreateVoiceConnectorGroupRequest,
    ) -> Result<CreateVoiceConnectorGroupResponse, RusotoError<CreateVoiceConnectorGroupError>>
    {
        let request_uri = "/voice-connector-groups";

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateVoiceConnectorGroupResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateVoiceConnectorGroupError::from_response(response))
        }
    }

    /// <p>Deletes the specified Amazon Chime account. You must suspend all users before deleting a <code>Team</code> account. You can use the <a>BatchSuspendUser</a> action to do so.</p> <p>For <code>EnterpriseLWA</code> and <code>EnterpriseAD</code> accounts, you must release the claimed domains for your Amazon Chime account before deletion. As soon as you release the domain, all users under that account are suspended.</p> <p>Deleted accounts appear in your <code>Disabled</code> accounts list for 90 days. To restore a deleted account from your <code>Disabled</code> accounts list, you must contact AWS Support.</p> <p>After 90 days, deleted accounts are permanently removed from your <code>Disabled</code> accounts list.</p>
    async fn delete_account(
        &self,
        input: DeleteAccountRequest,
    ) -> Result<DeleteAccountResponse, RusotoError<DeleteAccountError>> {
        let request_uri = format!("/accounts/{account_id}", account_id = input.account_id);

        let mut request = SignedRequest::new("DELETE", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteAccountResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteAccountError::from_response(response))
        }
    }

    /// <p>Deletes an attendee from the specified Amazon Chime SDK meeting and deletes their <code>JoinToken</code>. Attendees are automatically deleted when a Amazon Chime SDK meeting is deleted. For more information about the Amazon Chime SDK, see <a href="https://docs.aws.amazon.com/chime/latest/dg/meetings-sdk.html">Using the Amazon Chime SDK</a> in the <i>Amazon Chime Developer Guide</i>.</p>
    async fn delete_attendee(
        &self,
        input: DeleteAttendeeRequest,
    ) -> Result<(), RusotoError<DeleteAttendeeError>> {
        let request_uri = format!(
            "/meetings/{meeting_id}/attendees/{attendee_id}",
            attendee_id = input.attendee_id,
            meeting_id = input.meeting_id
        );

        let mut request = SignedRequest::new("DELETE", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteAttendeeError::from_response(response))
        }
    }

    /// <p>Deletes the events configuration that allows a bot to receive outgoing events.</p>
    async fn delete_events_configuration(
        &self,
        input: DeleteEventsConfigurationRequest,
    ) -> Result<(), RusotoError<DeleteEventsConfigurationError>> {
        let request_uri = format!(
            "/accounts/{account_id}/bots/{bot_id}/events-configuration",
            account_id = input.account_id,
            bot_id = input.bot_id
        );

        let mut request = SignedRequest::new("DELETE", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteEventsConfigurationError::from_response(response))
        }
    }

    /// <p>Deletes the specified Amazon Chime SDK meeting. When a meeting is deleted, its attendees are also deleted and clients can no longer join it. For more information about the Amazon Chime SDK, see <a href="https://docs.aws.amazon.com/chime/latest/dg/meetings-sdk.html">Using the Amazon Chime SDK</a> in the <i>Amazon Chime Developer Guide</i>.</p>
    async fn delete_meeting(
        &self,
        input: DeleteMeetingRequest,
    ) -> Result<(), RusotoError<DeleteMeetingError>> {
        let request_uri = format!("/meetings/{meeting_id}", meeting_id = input.meeting_id);

        let mut request = SignedRequest::new("DELETE", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteMeetingError::from_response(response))
        }
    }

    /// <p>Moves the specified phone number into the <b>Deletion queue</b>. A phone number must be disassociated from any users or Amazon Chime Voice Connectors before it can be deleted.</p> <p>Deleted phone numbers remain in the <b>Deletion queue</b> for 7 days before they are deleted permanently.</p>
    async fn delete_phone_number(
        &self,
        input: DeletePhoneNumberRequest,
    ) -> Result<(), RusotoError<DeletePhoneNumberError>> {
        let request_uri = format!(
            "/phone-numbers/{phone_number_id}",
            phone_number_id = input.phone_number_id
        );

        let mut request = SignedRequest::new("DELETE", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeletePhoneNumberError::from_response(response))
        }
    }

    /// <p>Deletes a chat room.</p>
    async fn delete_room(
        &self,
        input: DeleteRoomRequest,
    ) -> Result<(), RusotoError<DeleteRoomError>> {
        let request_uri = format!(
            "/accounts/{account_id}/rooms/{room_id}",
            account_id = input.account_id,
            room_id = input.room_id
        );

        let mut request = SignedRequest::new("DELETE", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteRoomError::from_response(response))
        }
    }

    /// <p>Removes a member from a chat room.</p>
    async fn delete_room_membership(
        &self,
        input: DeleteRoomMembershipRequest,
    ) -> Result<(), RusotoError<DeleteRoomMembershipError>> {
        let request_uri = format!(
            "/accounts/{account_id}/rooms/{room_id}/memberships/{member_id}",
            account_id = input.account_id,
            member_id = input.member_id,
            room_id = input.room_id
        );

        let mut request = SignedRequest::new("DELETE", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteRoomMembershipError::from_response(response))
        }
    }

    /// <p>Deletes the specified Amazon Chime Voice Connector. Any phone numbers associated with the Amazon Chime Voice Connector must be disassociated from it before it can be deleted.</p>
    async fn delete_voice_connector(
        &self,
        input: DeleteVoiceConnectorRequest,
    ) -> Result<(), RusotoError<DeleteVoiceConnectorError>> {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}",
            voice_connector_id = input.voice_connector_id
        );

        let mut request = SignedRequest::new("DELETE", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteVoiceConnectorError::from_response(response))
        }
    }

    /// <p>Deletes the specified Amazon Chime Voice Connector group. Any <code>VoiceConnectorItems</code> and phone numbers associated with the group must be removed before it can be deleted.</p>
    async fn delete_voice_connector_group(
        &self,
        input: DeleteVoiceConnectorGroupRequest,
    ) -> Result<(), RusotoError<DeleteVoiceConnectorGroupError>> {
        let request_uri = format!(
            "/voice-connector-groups/{voice_connector_group_id}",
            voice_connector_group_id = input.voice_connector_group_id
        );

        let mut request = SignedRequest::new("DELETE", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteVoiceConnectorGroupError::from_response(response))
        }
    }

    /// <p>Deletes the origination settings for the specified Amazon Chime Voice Connector.</p>
    async fn delete_voice_connector_origination(
        &self,
        input: DeleteVoiceConnectorOriginationRequest,
    ) -> Result<(), RusotoError<DeleteVoiceConnectorOriginationError>> {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}/origination",
            voice_connector_id = input.voice_connector_id
        );

        let mut request = SignedRequest::new("DELETE", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteVoiceConnectorOriginationError::from_response(
                response,
            ))
        }
    }

    /// <p>Deletes the streaming configuration for the specified Amazon Chime Voice Connector.</p>
    async fn delete_voice_connector_streaming_configuration(
        &self,
        input: DeleteVoiceConnectorStreamingConfigurationRequest,
    ) -> Result<(), RusotoError<DeleteVoiceConnectorStreamingConfigurationError>> {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}/streaming-configuration",
            voice_connector_id = input.voice_connector_id
        );

        let mut request = SignedRequest::new("DELETE", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteVoiceConnectorStreamingConfigurationError::from_response(response))
        }
    }

    /// <p>Deletes the termination settings for the specified Amazon Chime Voice Connector.</p>
    async fn delete_voice_connector_termination(
        &self,
        input: DeleteVoiceConnectorTerminationRequest,
    ) -> Result<(), RusotoError<DeleteVoiceConnectorTerminationError>> {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}/termination",
            voice_connector_id = input.voice_connector_id
        );

        let mut request = SignedRequest::new("DELETE", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteVoiceConnectorTerminationError::from_response(
                response,
            ))
        }
    }

    /// <p>Deletes the specified SIP credentials used by your equipment to authenticate during call termination.</p>
    async fn delete_voice_connector_termination_credentials(
        &self,
        input: DeleteVoiceConnectorTerminationCredentialsRequest,
    ) -> Result<(), RusotoError<DeleteVoiceConnectorTerminationCredentialsError>> {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}/termination/credentials",
            voice_connector_id = input.voice_connector_id
        );

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        params.put("operation", "delete");
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteVoiceConnectorTerminationCredentialsError::from_response(response))
        }
    }

    /// <p>Disassociates the primary provisioned phone number from the specified Amazon Chime user.</p>
    async fn disassociate_phone_number_from_user(
        &self,
        input: DisassociatePhoneNumberFromUserRequest,
    ) -> Result<
        DisassociatePhoneNumberFromUserResponse,
        RusotoError<DisassociatePhoneNumberFromUserError>,
    > {
        let request_uri = format!(
            "/accounts/{account_id}/users/{user_id}",
            account_id = input.account_id,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("operation", "disassociate-phone-number");
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DisassociatePhoneNumberFromUserResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociatePhoneNumberFromUserError::from_response(
                response,
            ))
        }
    }

    /// <p>Disassociates the specified phone numbers from the specified Amazon Chime Voice Connector.</p>
    async fn disassociate_phone_numbers_from_voice_connector(
        &self,
        input: DisassociatePhoneNumbersFromVoiceConnectorRequest,
    ) -> Result<
        DisassociatePhoneNumbersFromVoiceConnectorResponse,
        RusotoError<DisassociatePhoneNumbersFromVoiceConnectorError>,
    > {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}",
            voice_connector_id = input.voice_connector_id
        );

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        params.put("operation", "disassociate-phone-numbers");
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DisassociatePhoneNumbersFromVoiceConnectorResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociatePhoneNumbersFromVoiceConnectorError::from_response(response))
        }
    }

    /// <p>Disassociates the specified phone numbers from the specified Amazon Chime Voice Connector group.</p>
    async fn disassociate_phone_numbers_from_voice_connector_group(
        &self,
        input: DisassociatePhoneNumbersFromVoiceConnectorGroupRequest,
    ) -> Result<
        DisassociatePhoneNumbersFromVoiceConnectorGroupResponse,
        RusotoError<DisassociatePhoneNumbersFromVoiceConnectorGroupError>,
    > {
        let request_uri = format!(
            "/voice-connector-groups/{voice_connector_group_id}",
            voice_connector_group_id = input.voice_connector_group_id
        );

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        params.put("operation", "disassociate-phone-numbers");
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DisassociatePhoneNumbersFromVoiceConnectorGroupResponse, _>(
            )?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociatePhoneNumbersFromVoiceConnectorGroupError::from_response(response))
        }
    }

    /// <p>Disassociates the specified sign-in delegate groups from the specified Amazon Chime account.</p>
    async fn disassociate_signin_delegate_groups_from_account(
        &self,
        input: DisassociateSigninDelegateGroupsFromAccountRequest,
    ) -> Result<
        DisassociateSigninDelegateGroupsFromAccountResponse,
        RusotoError<DisassociateSigninDelegateGroupsFromAccountError>,
    > {
        let request_uri = format!("/accounts/{account_id}", account_id = input.account_id);

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        params.put("operation", "disassociate-signin-delegate-groups");
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DisassociateSigninDelegateGroupsFromAccountResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateSigninDelegateGroupsFromAccountError::from_response(response))
        }
    }

    /// <p>Retrieves details for the specified Amazon Chime account, such as account type and supported licenses.</p>
    async fn get_account(
        &self,
        input: GetAccountRequest,
    ) -> Result<GetAccountResponse, RusotoError<GetAccountError>> {
        let request_uri = format!("/accounts/{account_id}", account_id = input.account_id);

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetAccountResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetAccountError::from_response(response))
        }
    }

    /// <p>Retrieves account settings for the specified Amazon Chime account ID, such as remote control and dial out settings. For more information about these settings, see <a href="https://docs.aws.amazon.com/chime/latest/ag/policies.html">Use the Policies Page</a> in the <i>Amazon Chime Administration Guide</i>.</p>
    async fn get_account_settings(
        &self,
        input: GetAccountSettingsRequest,
    ) -> Result<GetAccountSettingsResponse, RusotoError<GetAccountSettingsError>> {
        let request_uri = format!(
            "/accounts/{account_id}/settings",
            account_id = input.account_id
        );

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetAccountSettingsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetAccountSettingsError::from_response(response))
        }
    }

    /// <p>Gets the Amazon Chime SDK attendee details for a specified meeting ID and attendee ID. For more information about the Amazon Chime SDK, see <a href="https://docs.aws.amazon.com/chime/latest/dg/meetings-sdk.html">Using the Amazon Chime SDK</a> in the <i>Amazon Chime Developer Guide</i>.</p>
    async fn get_attendee(
        &self,
        input: GetAttendeeRequest,
    ) -> Result<GetAttendeeResponse, RusotoError<GetAttendeeError>> {
        let request_uri = format!(
            "/meetings/{meeting_id}/attendees/{attendee_id}",
            attendee_id = input.attendee_id,
            meeting_id = input.meeting_id
        );

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetAttendeeResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetAttendeeError::from_response(response))
        }
    }

    /// <p>Retrieves details for the specified bot, such as bot email address, bot type, status, and display name.</p>
    async fn get_bot(
        &self,
        input: GetBotRequest,
    ) -> Result<GetBotResponse, RusotoError<GetBotError>> {
        let request_uri = format!(
            "/accounts/{account_id}/bots/{bot_id}",
            account_id = input.account_id,
            bot_id = input.bot_id
        );

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<GetBotResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetBotError::from_response(response))
        }
    }

    /// <p>Gets details for an events configuration that allows a bot to receive outgoing events, such as an HTTPS endpoint or Lambda function ARN. </p>
    async fn get_events_configuration(
        &self,
        input: GetEventsConfigurationRequest,
    ) -> Result<GetEventsConfigurationResponse, RusotoError<GetEventsConfigurationError>> {
        let request_uri = format!(
            "/accounts/{account_id}/bots/{bot_id}/events-configuration",
            account_id = input.account_id,
            bot_id = input.bot_id
        );

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetEventsConfigurationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetEventsConfigurationError::from_response(response))
        }
    }

    /// <p>Retrieves global settings for the administrator's AWS account, such as Amazon Chime Business Calling and Amazon Chime Voice Connector settings.</p>
    async fn get_global_settings(
        &self,
    ) -> Result<GetGlobalSettingsResponse, RusotoError<GetGlobalSettingsError>> {
        let request_uri = "/settings";

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetGlobalSettingsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetGlobalSettingsError::from_response(response))
        }
    }

    /// <p>Gets the Amazon Chime SDK meeting details for the specified meeting ID. For more information about the Amazon Chime SDK, see <a href="https://docs.aws.amazon.com/chime/latest/dg/meetings-sdk.html">Using the Amazon Chime SDK</a> in the <i>Amazon Chime Developer Guide</i>.</p>
    async fn get_meeting(
        &self,
        input: GetMeetingRequest,
    ) -> Result<GetMeetingResponse, RusotoError<GetMeetingError>> {
        let request_uri = format!("/meetings/{meeting_id}", meeting_id = input.meeting_id);

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetMeetingResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetMeetingError::from_response(response))
        }
    }

    /// <p>Retrieves details for the specified phone number ID, such as associations, capabilities, and product type.</p>
    async fn get_phone_number(
        &self,
        input: GetPhoneNumberRequest,
    ) -> Result<GetPhoneNumberResponse, RusotoError<GetPhoneNumberError>> {
        let request_uri = format!(
            "/phone-numbers/{phone_number_id}",
            phone_number_id = input.phone_number_id
        );

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetPhoneNumberResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetPhoneNumberError::from_response(response))
        }
    }

    /// <p>Retrieves details for the specified phone number order, such as order creation timestamp, phone numbers in E.164 format, product type, and order status.</p>
    async fn get_phone_number_order(
        &self,
        input: GetPhoneNumberOrderRequest,
    ) -> Result<GetPhoneNumberOrderResponse, RusotoError<GetPhoneNumberOrderError>> {
        let request_uri = format!(
            "/phone-number-orders/{phone_number_order_id}",
            phone_number_order_id = input.phone_number_order_id
        );

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetPhoneNumberOrderResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetPhoneNumberOrderError::from_response(response))
        }
    }

    /// <p>Retrieves the phone number settings for the administrator's AWS account, such as the default outbound calling name.</p>
    async fn get_phone_number_settings(
        &self,
    ) -> Result<GetPhoneNumberSettingsResponse, RusotoError<GetPhoneNumberSettingsError>> {
        let request_uri = "/settings/phone-number";

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetPhoneNumberSettingsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetPhoneNumberSettingsError::from_response(response))
        }
    }

    /// <p>Retrieves room details, such as the room name.</p>
    async fn get_room(
        &self,
        input: GetRoomRequest,
    ) -> Result<GetRoomResponse, RusotoError<GetRoomError>> {
        let request_uri = format!(
            "/accounts/{account_id}/rooms/{room_id}",
            account_id = input.account_id,
            room_id = input.room_id
        );

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<GetRoomResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetRoomError::from_response(response))
        }
    }

    /// <p>Retrieves details for the specified user ID, such as primary email address, license type, and personal meeting PIN.</p> <p>To retrieve user details with an email address instead of a user ID, use the <a>ListUsers</a> action, and then filter by email address.</p>
    async fn get_user(
        &self,
        input: GetUserRequest,
    ) -> Result<GetUserResponse, RusotoError<GetUserError>> {
        let request_uri = format!(
            "/accounts/{account_id}/users/{user_id}",
            account_id = input.account_id,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<GetUserResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetUserError::from_response(response))
        }
    }

    /// <p>Retrieves settings for the specified user ID, such as any associated phone number settings.</p>
    async fn get_user_settings(
        &self,
        input: GetUserSettingsRequest,
    ) -> Result<GetUserSettingsResponse, RusotoError<GetUserSettingsError>> {
        let request_uri = format!(
            "/accounts/{account_id}/users/{user_id}/settings",
            account_id = input.account_id,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetUserSettingsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetUserSettingsError::from_response(response))
        }
    }

    /// <p>Retrieves details for the specified Amazon Chime Voice Connector, such as timestamps, name, outbound host, and encryption requirements.</p>
    async fn get_voice_connector(
        &self,
        input: GetVoiceConnectorRequest,
    ) -> Result<GetVoiceConnectorResponse, RusotoError<GetVoiceConnectorError>> {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}",
            voice_connector_id = input.voice_connector_id
        );

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetVoiceConnectorResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetVoiceConnectorError::from_response(response))
        }
    }

    /// <p>Retrieves details for the specified Amazon Chime Voice Connector group, such as timestamps, name, and associated <code>VoiceConnectorItems</code>.</p>
    async fn get_voice_connector_group(
        &self,
        input: GetVoiceConnectorGroupRequest,
    ) -> Result<GetVoiceConnectorGroupResponse, RusotoError<GetVoiceConnectorGroupError>> {
        let request_uri = format!(
            "/voice-connector-groups/{voice_connector_group_id}",
            voice_connector_group_id = input.voice_connector_group_id
        );

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetVoiceConnectorGroupResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetVoiceConnectorGroupError::from_response(response))
        }
    }

    /// <p>Retrieves the logging configuration details for the specified Amazon Chime Voice Connector. Shows whether SIP message logs are enabled for sending to Amazon CloudWatch Logs.</p>
    async fn get_voice_connector_logging_configuration(
        &self,
        input: GetVoiceConnectorLoggingConfigurationRequest,
    ) -> Result<
        GetVoiceConnectorLoggingConfigurationResponse,
        RusotoError<GetVoiceConnectorLoggingConfigurationError>,
    > {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}/logging-configuration",
            voice_connector_id = input.voice_connector_id
        );

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetVoiceConnectorLoggingConfigurationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetVoiceConnectorLoggingConfigurationError::from_response(
                response,
            ))
        }
    }

    /// <p>Retrieves origination setting details for the specified Amazon Chime Voice Connector.</p>
    async fn get_voice_connector_origination(
        &self,
        input: GetVoiceConnectorOriginationRequest,
    ) -> Result<GetVoiceConnectorOriginationResponse, RusotoError<GetVoiceConnectorOriginationError>>
    {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}/origination",
            voice_connector_id = input.voice_connector_id
        );

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetVoiceConnectorOriginationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetVoiceConnectorOriginationError::from_response(response))
        }
    }

    /// <p>Retrieves the streaming configuration details for the specified Amazon Chime Voice Connector. Shows whether media streaming is enabled for sending to Amazon Kinesis. It also shows the retention period, in hours, for the Amazon Kinesis data.</p>
    async fn get_voice_connector_streaming_configuration(
        &self,
        input: GetVoiceConnectorStreamingConfigurationRequest,
    ) -> Result<
        GetVoiceConnectorStreamingConfigurationResponse,
        RusotoError<GetVoiceConnectorStreamingConfigurationError>,
    > {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}/streaming-configuration",
            voice_connector_id = input.voice_connector_id
        );

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetVoiceConnectorStreamingConfigurationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetVoiceConnectorStreamingConfigurationError::from_response(
                response,
            ))
        }
    }

    /// <p>Retrieves termination setting details for the specified Amazon Chime Voice Connector.</p>
    async fn get_voice_connector_termination(
        &self,
        input: GetVoiceConnectorTerminationRequest,
    ) -> Result<GetVoiceConnectorTerminationResponse, RusotoError<GetVoiceConnectorTerminationError>>
    {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}/termination",
            voice_connector_id = input.voice_connector_id
        );

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetVoiceConnectorTerminationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetVoiceConnectorTerminationError::from_response(response))
        }
    }

    /// <p>Retrieves information about the last time a SIP <code>OPTIONS</code> ping was received from your SIP infrastructure for the specified Amazon Chime Voice Connector.</p>
    async fn get_voice_connector_termination_health(
        &self,
        input: GetVoiceConnectorTerminationHealthRequest,
    ) -> Result<
        GetVoiceConnectorTerminationHealthResponse,
        RusotoError<GetVoiceConnectorTerminationHealthError>,
    > {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}/termination/health",
            voice_connector_id = input.voice_connector_id
        );

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetVoiceConnectorTerminationHealthResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetVoiceConnectorTerminationHealthError::from_response(
                response,
            ))
        }
    }

    /// <p>Sends email to a maximum of 50 users, inviting them to the specified Amazon Chime <code>Team</code> account. Only <code>Team</code> account types are currently supported for this action. </p>
    async fn invite_users(
        &self,
        input: InviteUsersRequest,
    ) -> Result<InviteUsersResponse, RusotoError<InviteUsersError>> {
        let request_uri = format!(
            "/accounts/{account_id}/users",
            account_id = input.account_id
        );

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        params.put("operation", "add");
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<InviteUsersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(InviteUsersError::from_response(response))
        }
    }

    /// <p>Lists the Amazon Chime accounts under the administrator's AWS account. You can filter accounts by account name prefix. To find out which Amazon Chime account a user belongs to, you can filter by the user's email address, which returns one account result.</p>
    async fn list_accounts(
        &self,
        input: ListAccountsRequest,
    ) -> Result<ListAccountsResponse, RusotoError<ListAccountsError>> {
        let request_uri = "/accounts";

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

    /// <p>Lists the attendees for the specified Amazon Chime SDK meeting. For more information about the Amazon Chime SDK, see <a href="https://docs.aws.amazon.com/chime/latest/dg/meetings-sdk.html">Using the Amazon Chime SDK</a> in the <i>Amazon Chime Developer Guide</i>.</p>
    async fn list_attendees(
        &self,
        input: ListAttendeesRequest,
    ) -> Result<ListAttendeesResponse, RusotoError<ListAttendeesError>> {
        let request_uri = format!(
            "/meetings/{meeting_id}/attendees",
            meeting_id = input.meeting_id
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
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListAttendeesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListAttendeesError::from_response(response))
        }
    }

    /// <p>Lists the bots associated with the administrator's Amazon Chime Enterprise account ID.</p>
    async fn list_bots(
        &self,
        input: ListBotsRequest,
    ) -> Result<ListBotsResponse, RusotoError<ListBotsError>> {
        let request_uri = format!("/accounts/{account_id}/bots", account_id = input.account_id);

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("max-results", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("next-token", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListBotsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListBotsError::from_response(response))
        }
    }

    /// <p>Lists up to 100 active Amazon Chime SDK meetings. For more information about the Amazon Chime SDK, see <a href="https://docs.aws.amazon.com/chime/latest/dg/meetings-sdk.html">Using the Amazon Chime SDK</a> in the <i>Amazon Chime Developer Guide</i>.</p>
    async fn list_meetings(
        &self,
        input: ListMeetingsRequest,
    ) -> Result<ListMeetingsResponse, RusotoError<ListMeetingsError>> {
        let request_uri = "/meetings";

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("max-results", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("next-token", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListMeetingsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListMeetingsError::from_response(response))
        }
    }

    /// <p>Lists the phone number orders for the administrator's Amazon Chime account.</p>
    async fn list_phone_number_orders(
        &self,
        input: ListPhoneNumberOrdersRequest,
    ) -> Result<ListPhoneNumberOrdersResponse, RusotoError<ListPhoneNumberOrdersError>> {
        let request_uri = "/phone-number-orders";

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("max-results", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("next-token", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListPhoneNumberOrdersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListPhoneNumberOrdersError::from_response(response))
        }
    }

    /// <p>Lists the phone numbers for the specified Amazon Chime account, Amazon Chime user, Amazon Chime Voice Connector, or Amazon Chime Voice Connector group.</p>
    async fn list_phone_numbers(
        &self,
        input: ListPhoneNumbersRequest,
    ) -> Result<ListPhoneNumbersResponse, RusotoError<ListPhoneNumbersError>> {
        let request_uri = "/phone-numbers";

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.filter_name {
            params.put("filter-name", x);
        }
        if let Some(ref x) = input.filter_value {
            params.put("filter-value", x);
        }
        if let Some(ref x) = input.max_results {
            params.put("max-results", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("next-token", x);
        }
        if let Some(ref x) = input.product_type {
            params.put("product-type", x);
        }
        if let Some(ref x) = input.status {
            params.put("status", x);
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
                .deserialize::<ListPhoneNumbersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListPhoneNumbersError::from_response(response))
        }
    }

    /// <p>Lists the membership details for the specified room, such as the members' IDs, email addresses, and names.</p>
    async fn list_room_memberships(
        &self,
        input: ListRoomMembershipsRequest,
    ) -> Result<ListRoomMembershipsResponse, RusotoError<ListRoomMembershipsError>> {
        let request_uri = format!(
            "/accounts/{account_id}/rooms/{room_id}/memberships",
            account_id = input.account_id,
            room_id = input.room_id
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
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListRoomMembershipsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListRoomMembershipsError::from_response(response))
        }
    }

    /// <p>Lists the room details for the specified Amazon Chime account. Optionally, filter the results by a member ID (user ID or bot ID) to see a list of rooms that the member belongs to.</p>
    async fn list_rooms(
        &self,
        input: ListRoomsRequest,
    ) -> Result<ListRoomsResponse, RusotoError<ListRoomsError>> {
        let request_uri = format!(
            "/accounts/{account_id}/rooms",
            account_id = input.account_id
        );

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("max-results", x);
        }
        if let Some(ref x) = input.member_id {
            params.put("member-id", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("next-token", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListRoomsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListRoomsError::from_response(response))
        }
    }

    /// <p>Lists the users that belong to the specified Amazon Chime account. You can specify an email address to list only the user that the email address belongs to.</p>
    async fn list_users(
        &self,
        input: ListUsersRequest,
    ) -> Result<ListUsersResponse, RusotoError<ListUsersError>> {
        let request_uri = format!(
            "/accounts/{account_id}/users",
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
        if let Some(ref x) = input.user_type {
            params.put("user-type", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListUsersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListUsersError::from_response(response))
        }
    }

    /// <p>Lists the Amazon Chime Voice Connector groups for the administrator's AWS account.</p>
    async fn list_voice_connector_groups(
        &self,
        input: ListVoiceConnectorGroupsRequest,
    ) -> Result<ListVoiceConnectorGroupsResponse, RusotoError<ListVoiceConnectorGroupsError>> {
        let request_uri = "/voice-connector-groups";

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("max-results", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("next-token", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListVoiceConnectorGroupsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListVoiceConnectorGroupsError::from_response(response))
        }
    }

    /// <p>Lists the SIP credentials for the specified Amazon Chime Voice Connector.</p>
    async fn list_voice_connector_termination_credentials(
        &self,
        input: ListVoiceConnectorTerminationCredentialsRequest,
    ) -> Result<
        ListVoiceConnectorTerminationCredentialsResponse,
        RusotoError<ListVoiceConnectorTerminationCredentialsError>,
    > {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}/termination/credentials",
            voice_connector_id = input.voice_connector_id
        );

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListVoiceConnectorTerminationCredentialsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListVoiceConnectorTerminationCredentialsError::from_response(response))
        }
    }

    /// <p>Lists the Amazon Chime Voice Connectors for the administrator's AWS account.</p>
    async fn list_voice_connectors(
        &self,
        input: ListVoiceConnectorsRequest,
    ) -> Result<ListVoiceConnectorsResponse, RusotoError<ListVoiceConnectorsError>> {
        let request_uri = "/voice-connectors";

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("max-results", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("next-token", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListVoiceConnectorsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListVoiceConnectorsError::from_response(response))
        }
    }

    /// <p>Logs out the specified user from all of the devices they are currently logged into.</p>
    async fn logout_user(
        &self,
        input: LogoutUserRequest,
    ) -> Result<LogoutUserResponse, RusotoError<LogoutUserError>> {
        let request_uri = format!(
            "/accounts/{account_id}/users/{user_id}",
            account_id = input.account_id,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("operation", "logout");
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<LogoutUserResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(LogoutUserError::from_response(response))
        }
    }

    /// <p>Creates an events configuration that allows a bot to receive outgoing events sent by Amazon Chime. Choose either an HTTPS endpoint or a Lambda function ARN. For more information, see <a>Bot</a>.</p>
    async fn put_events_configuration(
        &self,
        input: PutEventsConfigurationRequest,
    ) -> Result<PutEventsConfigurationResponse, RusotoError<PutEventsConfigurationError>> {
        let request_uri = format!(
            "/accounts/{account_id}/bots/{bot_id}/events-configuration",
            account_id = input.account_id,
            bot_id = input.bot_id
        );

        let mut request = SignedRequest::new("PUT", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutEventsConfigurationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutEventsConfigurationError::from_response(response))
        }
    }

    /// <p>Adds a logging configuration for the specified Amazon Chime Voice Connector. The logging configuration specifies whether SIP message logs are enabled for sending to Amazon CloudWatch Logs.</p>
    async fn put_voice_connector_logging_configuration(
        &self,
        input: PutVoiceConnectorLoggingConfigurationRequest,
    ) -> Result<
        PutVoiceConnectorLoggingConfigurationResponse,
        RusotoError<PutVoiceConnectorLoggingConfigurationError>,
    > {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}/logging-configuration",
            voice_connector_id = input.voice_connector_id
        );

        let mut request = SignedRequest::new("PUT", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutVoiceConnectorLoggingConfigurationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutVoiceConnectorLoggingConfigurationError::from_response(
                response,
            ))
        }
    }

    /// <p>Adds origination settings for the specified Amazon Chime Voice Connector.</p>
    async fn put_voice_connector_origination(
        &self,
        input: PutVoiceConnectorOriginationRequest,
    ) -> Result<PutVoiceConnectorOriginationResponse, RusotoError<PutVoiceConnectorOriginationError>>
    {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}/origination",
            voice_connector_id = input.voice_connector_id
        );

        let mut request = SignedRequest::new("PUT", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutVoiceConnectorOriginationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutVoiceConnectorOriginationError::from_response(response))
        }
    }

    /// <p>Adds a streaming configuration for the specified Amazon Chime Voice Connector. The streaming configuration specifies whether media streaming is enabled for sending to Amazon Kinesis. It also sets the retention period, in hours, for the Amazon Kinesis data.</p>
    async fn put_voice_connector_streaming_configuration(
        &self,
        input: PutVoiceConnectorStreamingConfigurationRequest,
    ) -> Result<
        PutVoiceConnectorStreamingConfigurationResponse,
        RusotoError<PutVoiceConnectorStreamingConfigurationError>,
    > {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}/streaming-configuration",
            voice_connector_id = input.voice_connector_id
        );

        let mut request = SignedRequest::new("PUT", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutVoiceConnectorStreamingConfigurationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutVoiceConnectorStreamingConfigurationError::from_response(
                response,
            ))
        }
    }

    /// <p>Adds termination settings for the specified Amazon Chime Voice Connector.</p>
    async fn put_voice_connector_termination(
        &self,
        input: PutVoiceConnectorTerminationRequest,
    ) -> Result<PutVoiceConnectorTerminationResponse, RusotoError<PutVoiceConnectorTerminationError>>
    {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}/termination",
            voice_connector_id = input.voice_connector_id
        );

        let mut request = SignedRequest::new("PUT", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutVoiceConnectorTerminationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutVoiceConnectorTerminationError::from_response(response))
        }
    }

    /// <p>Adds termination SIP credentials for the specified Amazon Chime Voice Connector.</p>
    async fn put_voice_connector_termination_credentials(
        &self,
        input: PutVoiceConnectorTerminationCredentialsRequest,
    ) -> Result<(), RusotoError<PutVoiceConnectorTerminationCredentialsError>> {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}/termination/credentials",
            voice_connector_id = input.voice_connector_id
        );

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        params.put("operation", "put");
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutVoiceConnectorTerminationCredentialsError::from_response(
                response,
            ))
        }
    }

    /// <p>Regenerates the security token for a bot.</p>
    async fn regenerate_security_token(
        &self,
        input: RegenerateSecurityTokenRequest,
    ) -> Result<RegenerateSecurityTokenResponse, RusotoError<RegenerateSecurityTokenError>> {
        let request_uri = format!(
            "/accounts/{account_id}/bots/{bot_id}",
            account_id = input.account_id,
            bot_id = input.bot_id
        );

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("operation", "regenerate-security-token");
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<RegenerateSecurityTokenResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(RegenerateSecurityTokenError::from_response(response))
        }
    }

    /// <p>Resets the personal meeting PIN for the specified user on an Amazon Chime account. Returns the <a>User</a> object with the updated personal meeting PIN.</p>
    async fn reset_personal_pin(
        &self,
        input: ResetPersonalPINRequest,
    ) -> Result<ResetPersonalPINResponse, RusotoError<ResetPersonalPINError>> {
        let request_uri = format!(
            "/accounts/{account_id}/users/{user_id}",
            account_id = input.account_id,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("operation", "reset-personal-pin");
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ResetPersonalPINResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ResetPersonalPINError::from_response(response))
        }
    }

    /// <p>Moves a phone number from the <b>Deletion queue</b> back into the phone number <b>Inventory</b>.</p>
    async fn restore_phone_number(
        &self,
        input: RestorePhoneNumberRequest,
    ) -> Result<RestorePhoneNumberResponse, RusotoError<RestorePhoneNumberError>> {
        let request_uri = format!(
            "/phone-numbers/{phone_number_id}",
            phone_number_id = input.phone_number_id
        );

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("operation", "restore");
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<RestorePhoneNumberResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(RestorePhoneNumberError::from_response(response))
        }
    }

    /// <p>Searches phone numbers that can be ordered.</p>
    async fn search_available_phone_numbers(
        &self,
        input: SearchAvailablePhoneNumbersRequest,
    ) -> Result<SearchAvailablePhoneNumbersResponse, RusotoError<SearchAvailablePhoneNumbersError>>
    {
        let request_uri = "/search";

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.area_code {
            params.put("area-code", x);
        }
        if let Some(ref x) = input.city {
            params.put("city", x);
        }
        if let Some(ref x) = input.country {
            params.put("country", x);
        }
        if let Some(ref x) = input.max_results {
            params.put("max-results", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("next-token", x);
        }
        if let Some(ref x) = input.state {
            params.put("state", x);
        }
        if let Some(ref x) = input.toll_free_prefix {
            params.put("toll-free-prefix", x);
        }
        params.put("type", "phone-numbers");
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<SearchAvailablePhoneNumbersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(SearchAvailablePhoneNumbersError::from_response(response))
        }
    }

    /// <p>Updates account details for the specified Amazon Chime account. Currently, only account name updates are supported for this action.</p>
    async fn update_account(
        &self,
        input: UpdateAccountRequest,
    ) -> Result<UpdateAccountResponse, RusotoError<UpdateAccountError>> {
        let request_uri = format!("/accounts/{account_id}", account_id = input.account_id);

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateAccountResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateAccountError::from_response(response))
        }
    }

    /// <p>Updates the settings for the specified Amazon Chime account. You can update settings for remote control of shared screens, or for the dial-out option. For more information about these settings, see <a href="https://docs.aws.amazon.com/chime/latest/ag/policies.html">Use the Policies Page</a> in the <i>Amazon Chime Administration Guide</i>.</p>
    async fn update_account_settings(
        &self,
        input: UpdateAccountSettingsRequest,
    ) -> Result<UpdateAccountSettingsResponse, RusotoError<UpdateAccountSettingsError>> {
        let request_uri = format!(
            "/accounts/{account_id}/settings",
            account_id = input.account_id
        );

        let mut request = SignedRequest::new("PUT", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateAccountSettingsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateAccountSettingsError::from_response(response))
        }
    }

    /// <p>Updates the status of the specified bot, such as starting or stopping the bot from running in your Amazon Chime Enterprise account.</p>
    async fn update_bot(
        &self,
        input: UpdateBotRequest,
    ) -> Result<UpdateBotResponse, RusotoError<UpdateBotError>> {
        let request_uri = format!(
            "/accounts/{account_id}/bots/{bot_id}",
            account_id = input.account_id,
            bot_id = input.bot_id
        );

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateBotResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateBotError::from_response(response))
        }
    }

    /// <p>Updates global settings for the administrator's AWS account, such as Amazon Chime Business Calling and Amazon Chime Voice Connector settings.</p>
    async fn update_global_settings(
        &self,
        input: UpdateGlobalSettingsRequest,
    ) -> Result<(), RusotoError<UpdateGlobalSettingsError>> {
        let request_uri = "/settings";

        let mut request = SignedRequest::new("PUT", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateGlobalSettingsError::from_response(response))
        }
    }

    /// <p>Updates phone number details, such as product type or calling name, for the specified phone number ID. You can update one phone number detail at a time. For example, you can update either the product type or the calling name in one action.</p> <p>For toll-free numbers, you must use the Amazon Chime Voice Connector product type.</p> <p>Updates to outbound calling names can take up to 72 hours to complete. Pending updates to outbound calling names must be complete before you can request another update.</p>
    async fn update_phone_number(
        &self,
        input: UpdatePhoneNumberRequest,
    ) -> Result<UpdatePhoneNumberResponse, RusotoError<UpdatePhoneNumberError>> {
        let request_uri = format!(
            "/phone-numbers/{phone_number_id}",
            phone_number_id = input.phone_number_id
        );

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdatePhoneNumberResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdatePhoneNumberError::from_response(response))
        }
    }

    /// <p>Updates the phone number settings for the administrator's AWS account, such as the default outbound calling name. You can update the default outbound calling name once every seven days. Outbound calling names can take up to 72 hours to update.</p>
    async fn update_phone_number_settings(
        &self,
        input: UpdatePhoneNumberSettingsRequest,
    ) -> Result<(), RusotoError<UpdatePhoneNumberSettingsError>> {
        let request_uri = "/settings/phone-number";

        let mut request = SignedRequest::new("PUT", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdatePhoneNumberSettingsError::from_response(response))
        }
    }

    /// <p>Updates room details, such as the room name.</p>
    async fn update_room(
        &self,
        input: UpdateRoomRequest,
    ) -> Result<UpdateRoomResponse, RusotoError<UpdateRoomError>> {
        let request_uri = format!(
            "/accounts/{account_id}/rooms/{room_id}",
            account_id = input.account_id,
            room_id = input.room_id
        );

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateRoomResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateRoomError::from_response(response))
        }
    }

    /// <p>Updates room membership details, such as the member role. The member role designates whether the member is a chat room administrator or a general chat room member. The member role can be updated only for user IDs.</p>
    async fn update_room_membership(
        &self,
        input: UpdateRoomMembershipRequest,
    ) -> Result<UpdateRoomMembershipResponse, RusotoError<UpdateRoomMembershipError>> {
        let request_uri = format!(
            "/accounts/{account_id}/rooms/{room_id}/memberships/{member_id}",
            account_id = input.account_id,
            member_id = input.member_id,
            room_id = input.room_id
        );

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateRoomMembershipResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateRoomMembershipError::from_response(response))
        }
    }

    /// <p>Updates user details for a specified user ID. Currently, only <code>LicenseType</code> updates are supported for this action.</p>
    async fn update_user(
        &self,
        input: UpdateUserRequest,
    ) -> Result<UpdateUserResponse, RusotoError<UpdateUserError>> {
        let request_uri = format!(
            "/accounts/{account_id}/users/{user_id}",
            account_id = input.account_id,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateUserResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateUserError::from_response(response))
        }
    }

    /// <p>Updates the settings for the specified user, such as phone number settings.</p>
    async fn update_user_settings(
        &self,
        input: UpdateUserSettingsRequest,
    ) -> Result<(), RusotoError<UpdateUserSettingsError>> {
        let request_uri = format!(
            "/accounts/{account_id}/users/{user_id}/settings",
            account_id = input.account_id,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("PUT", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateUserSettingsError::from_response(response))
        }
    }

    /// <p>Updates details for the specified Amazon Chime Voice Connector.</p>
    async fn update_voice_connector(
        &self,
        input: UpdateVoiceConnectorRequest,
    ) -> Result<UpdateVoiceConnectorResponse, RusotoError<UpdateVoiceConnectorError>> {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}",
            voice_connector_id = input.voice_connector_id
        );

        let mut request = SignedRequest::new("PUT", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateVoiceConnectorResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateVoiceConnectorError::from_response(response))
        }
    }

    /// <p>Updates details for the specified Amazon Chime Voice Connector group, such as the name and Amazon Chime Voice Connector priority ranking.</p>
    async fn update_voice_connector_group(
        &self,
        input: UpdateVoiceConnectorGroupRequest,
    ) -> Result<UpdateVoiceConnectorGroupResponse, RusotoError<UpdateVoiceConnectorGroupError>>
    {
        let request_uri = format!(
            "/voice-connector-groups/{voice_connector_group_id}",
            voice_connector_group_id = input.voice_connector_group_id
        );

        let mut request = SignedRequest::new("PUT", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateVoiceConnectorGroupResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateVoiceConnectorGroupError::from_response(response))
        }
    }
}
