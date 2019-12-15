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
#![allow(warnings)]

use futures::future;
use futures::Future;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};
use std::error::Error;
use std::fmt;

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociatePhoneNumberWithUserResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct CreateMeetingRequest {
    /// <p>The unique identifier for the client request. Use a different token for different meetings.</p>
    #[serde(rename = "ClientRequestToken")]
    pub client_request_token: String,
    /// <p>The Region in which to create the meeting. Available values: <code>us-east-1</code>, <code>us-west-2</code>.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteAccountResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteAttendeeRequest {
    /// <p>The Amazon Chime SDK attendee ID.</p>
    #[serde(rename = "AttendeeId")]
    pub attendee_id: String,
    /// <p>The Amazon Chime SDK meeting ID.</p>
    #[serde(rename = "MeetingId")]
    pub meeting_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteEventsConfigurationRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The bot ID.</p>
    #[serde(rename = "BotId")]
    pub bot_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteMeetingRequest {
    /// <p>The Amazon Chime SDK meeting ID.</p>
    #[serde(rename = "MeetingId")]
    pub meeting_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeletePhoneNumberRequest {
    /// <p>The phone number ID.</p>
    #[serde(rename = "PhoneNumberId")]
    pub phone_number_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct DeleteRoomRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The chat room ID.</p>
    #[serde(rename = "RoomId")]
    pub room_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteVoiceConnectorGroupRequest {
    /// <p>The Amazon Chime Voice Connector group ID.</p>
    #[serde(rename = "VoiceConnectorGroupId")]
    pub voice_connector_group_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteVoiceConnectorOriginationRequest {
    /// <p>The Amazon Chime Voice Connector ID.</p>
    #[serde(rename = "VoiceConnectorId")]
    pub voice_connector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteVoiceConnectorRequest {
    /// <p>The Amazon Chime Voice Connector ID.</p>
    #[serde(rename = "VoiceConnectorId")]
    pub voice_connector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteVoiceConnectorStreamingConfigurationRequest {
    /// <p>The Amazon Chime Voice Connector ID.</p>
    #[serde(rename = "VoiceConnectorId")]
    pub voice_connector_id: String,
}

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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteVoiceConnectorTerminationRequest {
    /// <p>The Amazon Chime Voice Connector ID.</p>
    #[serde(rename = "VoiceConnectorId")]
    pub voice_connector_id: String,
}

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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociatePhoneNumberFromUserResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct InviteUsersRequest {
    /// <p>The Amazon Chime account ID.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The user email addresses to which to send the email invitation.</p>
    #[serde(rename = "UserEmailList")]
    pub user_email_list: Vec<String>,
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
    /// <p>The Region in which to create the meeting. Available values: <code>us-east-1</code>, <code>us-west-2</code>.</p>
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
pub struct UpdateGlobalSettingsRequest {
    /// <p>The Amazon Chime Business Calling settings.</p>
    #[serde(rename = "BusinessCalling")]
    pub business_calling: BusinessCallingSettings,
    /// <p>The Amazon Chime Voice Connector settings.</p>
    #[serde(rename = "VoiceConnector")]
    pub voice_connector: VoiceConnectorSettings,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct UpdatePhoneNumberSettingsRequest {
    /// <p>The default outbound calling name for the account.</p>
    #[serde(rename = "CallingName")]
    pub calling_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
            AssociatePhoneNumberWithUserError::AccessDenied(ref cause) => cause,
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
            AssociatePhoneNumbersWithVoiceConnectorError::AccessDenied(ref cause) => cause,
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AssociatePhoneNumbersWithVoiceConnectorGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociatePhoneNumbersWithVoiceConnectorGroupError {
    fn description(&self) -> &str {
        match *self {
            AssociatePhoneNumbersWithVoiceConnectorGroupError::AccessDenied(ref cause) => cause,
            AssociatePhoneNumbersWithVoiceConnectorGroupError::BadRequest(ref cause) => cause,
            AssociatePhoneNumbersWithVoiceConnectorGroupError::Forbidden(ref cause) => cause,
            AssociatePhoneNumbersWithVoiceConnectorGroupError::NotFound(ref cause) => cause,
            AssociatePhoneNumbersWithVoiceConnectorGroupError::ServiceFailure(ref cause) => cause,
            AssociatePhoneNumbersWithVoiceConnectorGroupError::ServiceUnavailable(ref cause) => {
                cause
            }
            AssociatePhoneNumbersWithVoiceConnectorGroupError::ThrottledClient(ref cause) => cause,
            AssociatePhoneNumbersWithVoiceConnectorGroupError::UnauthorizedClient(ref cause) => {
                cause
            }
        }
    }
}
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for BatchCreateAttendeeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchCreateAttendeeError {
    fn description(&self) -> &str {
        match *self {
            BatchCreateAttendeeError::BadRequest(ref cause) => cause,
            BatchCreateAttendeeError::Forbidden(ref cause) => cause,
            BatchCreateAttendeeError::NotFound(ref cause) => cause,
            BatchCreateAttendeeError::ResourceLimitExceeded(ref cause) => cause,
            BatchCreateAttendeeError::ServiceFailure(ref cause) => cause,
            BatchCreateAttendeeError::ServiceUnavailable(ref cause) => cause,
            BatchCreateAttendeeError::ThrottledClient(ref cause) => cause,
            BatchCreateAttendeeError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
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
                "UnauthorizedClientException" => {
                    return RusotoError::Service(
                        BatchCreateRoomMembershipError::UnauthorizedClient(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for BatchCreateRoomMembershipError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchCreateRoomMembershipError {
    fn description(&self) -> &str {
        match *self {
            BatchCreateRoomMembershipError::BadRequest(ref cause) => cause,
            BatchCreateRoomMembershipError::Forbidden(ref cause) => cause,
            BatchCreateRoomMembershipError::NotFound(ref cause) => cause,
            BatchCreateRoomMembershipError::ServiceFailure(ref cause) => cause,
            BatchCreateRoomMembershipError::ServiceUnavailable(ref cause) => cause,
            BatchCreateRoomMembershipError::UnauthorizedClient(ref cause) => cause,
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateAttendeeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateAttendeeError {
    fn description(&self) -> &str {
        match *self {
            CreateAttendeeError::BadRequest(ref cause) => cause,
            CreateAttendeeError::Forbidden(ref cause) => cause,
            CreateAttendeeError::NotFound(ref cause) => cause,
            CreateAttendeeError::ResourceLimitExceeded(ref cause) => cause,
            CreateAttendeeError::ServiceFailure(ref cause) => cause,
            CreateAttendeeError::ServiceUnavailable(ref cause) => cause,
            CreateAttendeeError::ThrottledClient(ref cause) => cause,
            CreateAttendeeError::UnauthorizedClient(ref cause) => cause,
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateMeetingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateMeetingError {
    fn description(&self) -> &str {
        match *self {
            CreateMeetingError::BadRequest(ref cause) => cause,
            CreateMeetingError::Forbidden(ref cause) => cause,
            CreateMeetingError::ResourceLimitExceeded(ref cause) => cause,
            CreateMeetingError::ServiceFailure(ref cause) => cause,
            CreateMeetingError::ServiceUnavailable(ref cause) => cause,
            CreateMeetingError::ThrottledClient(ref cause) => cause,
            CreateMeetingError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
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
            CreatePhoneNumberOrderError::AccessDenied(ref cause) => cause,
            CreatePhoneNumberOrderError::BadRequest(ref cause) => cause,
            CreatePhoneNumberOrderError::Forbidden(ref cause) => cause,
            CreatePhoneNumberOrderError::ResourceLimitExceeded(ref cause) => cause,
            CreatePhoneNumberOrderError::ServiceFailure(ref cause) => cause,
            CreatePhoneNumberOrderError::ServiceUnavailable(ref cause) => cause,
            CreatePhoneNumberOrderError::ThrottledClient(ref cause) => cause,
            CreatePhoneNumberOrderError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
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
                "UnauthorizedClientException" => {
                    return RusotoError::Service(CreateRoomError::UnauthorizedClient(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateRoomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateRoomError {
    fn description(&self) -> &str {
        match *self {
            CreateRoomError::BadRequest(ref cause) => cause,
            CreateRoomError::Forbidden(ref cause) => cause,
            CreateRoomError::NotFound(ref cause) => cause,
            CreateRoomError::ResourceLimitExceeded(ref cause) => cause,
            CreateRoomError::ServiceFailure(ref cause) => cause,
            CreateRoomError::ServiceUnavailable(ref cause) => cause,
            CreateRoomError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
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
                "UnauthorizedClientException" => {
                    return RusotoError::Service(CreateRoomMembershipError::UnauthorizedClient(
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
impl fmt::Display for CreateRoomMembershipError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateRoomMembershipError {
    fn description(&self) -> &str {
        match *self {
            CreateRoomMembershipError::BadRequest(ref cause) => cause,
            CreateRoomMembershipError::Conflict(ref cause) => cause,
            CreateRoomMembershipError::Forbidden(ref cause) => cause,
            CreateRoomMembershipError::NotFound(ref cause) => cause,
            CreateRoomMembershipError::ResourceLimitExceeded(ref cause) => cause,
            CreateRoomMembershipError::ServiceFailure(ref cause) => cause,
            CreateRoomMembershipError::ServiceUnavailable(ref cause) => cause,
            CreateRoomMembershipError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
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
            CreateVoiceConnectorError::AccessDenied(ref cause) => cause,
            CreateVoiceConnectorError::BadRequest(ref cause) => cause,
            CreateVoiceConnectorError::Forbidden(ref cause) => cause,
            CreateVoiceConnectorError::ResourceLimitExceeded(ref cause) => cause,
            CreateVoiceConnectorError::ServiceFailure(ref cause) => cause,
            CreateVoiceConnectorError::ServiceUnavailable(ref cause) => cause,
            CreateVoiceConnectorError::ThrottledClient(ref cause) => cause,
            CreateVoiceConnectorError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateVoiceConnectorGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateVoiceConnectorGroupError {
    fn description(&self) -> &str {
        match *self {
            CreateVoiceConnectorGroupError::AccessDenied(ref cause) => cause,
            CreateVoiceConnectorGroupError::BadRequest(ref cause) => cause,
            CreateVoiceConnectorGroupError::Forbidden(ref cause) => cause,
            CreateVoiceConnectorGroupError::ResourceLimitExceeded(ref cause) => cause,
            CreateVoiceConnectorGroupError::ServiceFailure(ref cause) => cause,
            CreateVoiceConnectorGroupError::ServiceUnavailable(ref cause) => cause,
            CreateVoiceConnectorGroupError::ThrottledClient(ref cause) => cause,
            CreateVoiceConnectorGroupError::UnauthorizedClient(ref cause) => cause,
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteAttendeeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteAttendeeError {
    fn description(&self) -> &str {
        match *self {
            DeleteAttendeeError::BadRequest(ref cause) => cause,
            DeleteAttendeeError::Forbidden(ref cause) => cause,
            DeleteAttendeeError::NotFound(ref cause) => cause,
            DeleteAttendeeError::ServiceFailure(ref cause) => cause,
            DeleteAttendeeError::ServiceUnavailable(ref cause) => cause,
            DeleteAttendeeError::ThrottledClient(ref cause) => cause,
            DeleteAttendeeError::UnauthorizedClient(ref cause) => cause,
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteMeetingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteMeetingError {
    fn description(&self) -> &str {
        match *self {
            DeleteMeetingError::BadRequest(ref cause) => cause,
            DeleteMeetingError::Forbidden(ref cause) => cause,
            DeleteMeetingError::NotFound(ref cause) => cause,
            DeleteMeetingError::ServiceFailure(ref cause) => cause,
            DeleteMeetingError::ServiceUnavailable(ref cause) => cause,
            DeleteMeetingError::ThrottledClient(ref cause) => cause,
            DeleteMeetingError::UnauthorizedClient(ref cause) => cause,
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
                "UnauthorizedClientException" => {
                    return RusotoError::Service(DeleteRoomError::UnauthorizedClient(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteRoomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteRoomError {
    fn description(&self) -> &str {
        match *self {
            DeleteRoomError::BadRequest(ref cause) => cause,
            DeleteRoomError::Forbidden(ref cause) => cause,
            DeleteRoomError::NotFound(ref cause) => cause,
            DeleteRoomError::ServiceFailure(ref cause) => cause,
            DeleteRoomError::ServiceUnavailable(ref cause) => cause,
            DeleteRoomError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
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
                "UnauthorizedClientException" => {
                    return RusotoError::Service(DeleteRoomMembershipError::UnauthorizedClient(
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
impl fmt::Display for DeleteRoomMembershipError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteRoomMembershipError {
    fn description(&self) -> &str {
        match *self {
            DeleteRoomMembershipError::BadRequest(ref cause) => cause,
            DeleteRoomMembershipError::Forbidden(ref cause) => cause,
            DeleteRoomMembershipError::NotFound(ref cause) => cause,
            DeleteRoomMembershipError::ServiceFailure(ref cause) => cause,
            DeleteRoomMembershipError::ServiceUnavailable(ref cause) => cause,
            DeleteRoomMembershipError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
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
            DeleteVoiceConnectorError::Conflict(ref cause) => cause,
            DeleteVoiceConnectorError::Forbidden(ref cause) => cause,
            DeleteVoiceConnectorError::NotFound(ref cause) => cause,
            DeleteVoiceConnectorError::ServiceFailure(ref cause) => cause,
            DeleteVoiceConnectorError::ServiceUnavailable(ref cause) => cause,
            DeleteVoiceConnectorError::ThrottledClient(ref cause) => cause,
            DeleteVoiceConnectorError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteVoiceConnectorGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteVoiceConnectorGroupError {
    fn description(&self) -> &str {
        match *self {
            DeleteVoiceConnectorGroupError::BadRequest(ref cause) => cause,
            DeleteVoiceConnectorGroupError::Conflict(ref cause) => cause,
            DeleteVoiceConnectorGroupError::Forbidden(ref cause) => cause,
            DeleteVoiceConnectorGroupError::NotFound(ref cause) => cause,
            DeleteVoiceConnectorGroupError::ServiceFailure(ref cause) => cause,
            DeleteVoiceConnectorGroupError::ServiceUnavailable(ref cause) => cause,
            DeleteVoiceConnectorGroupError::ThrottledClient(ref cause) => cause,
            DeleteVoiceConnectorGroupError::UnauthorizedClient(ref cause) => cause,
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteVoiceConnectorStreamingConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteVoiceConnectorStreamingConfigurationError {
    fn description(&self) -> &str {
        match *self {
            DeleteVoiceConnectorStreamingConfigurationError::BadRequest(ref cause) => cause,
            DeleteVoiceConnectorStreamingConfigurationError::Forbidden(ref cause) => cause,
            DeleteVoiceConnectorStreamingConfigurationError::NotFound(ref cause) => cause,
            DeleteVoiceConnectorStreamingConfigurationError::ServiceFailure(ref cause) => cause,
            DeleteVoiceConnectorStreamingConfigurationError::ServiceUnavailable(ref cause) => cause,
            DeleteVoiceConnectorStreamingConfigurationError::ThrottledClient(ref cause) => cause,
            DeleteVoiceConnectorStreamingConfigurationError::UnauthorizedClient(ref cause) => cause,
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DisassociatePhoneNumbersFromVoiceConnectorGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociatePhoneNumbersFromVoiceConnectorGroupError {
    fn description(&self) -> &str {
        match *self {
            DisassociatePhoneNumbersFromVoiceConnectorGroupError::BadRequest(ref cause) => cause,
            DisassociatePhoneNumbersFromVoiceConnectorGroupError::Forbidden(ref cause) => cause,
            DisassociatePhoneNumbersFromVoiceConnectorGroupError::NotFound(ref cause) => cause,
            DisassociatePhoneNumbersFromVoiceConnectorGroupError::ServiceFailure(ref cause) => {
                cause
            }
            DisassociatePhoneNumbersFromVoiceConnectorGroupError::ServiceUnavailable(ref cause) => {
                cause
            }
            DisassociatePhoneNumbersFromVoiceConnectorGroupError::ThrottledClient(ref cause) => {
                cause
            }
            DisassociatePhoneNumbersFromVoiceConnectorGroupError::UnauthorizedClient(ref cause) => {
                cause
            }
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetAttendeeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAttendeeError {
    fn description(&self) -> &str {
        match *self {
            GetAttendeeError::BadRequest(ref cause) => cause,
            GetAttendeeError::Forbidden(ref cause) => cause,
            GetAttendeeError::NotFound(ref cause) => cause,
            GetAttendeeError::ServiceFailure(ref cause) => cause,
            GetAttendeeError::ServiceUnavailable(ref cause) => cause,
            GetAttendeeError::ThrottledClient(ref cause) => cause,
            GetAttendeeError::UnauthorizedClient(ref cause) => cause,
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetMeetingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetMeetingError {
    fn description(&self) -> &str {
        match *self {
            GetMeetingError::BadRequest(ref cause) => cause,
            GetMeetingError::Forbidden(ref cause) => cause,
            GetMeetingError::NotFound(ref cause) => cause,
            GetMeetingError::ServiceFailure(ref cause) => cause,
            GetMeetingError::ServiceUnavailable(ref cause) => cause,
            GetMeetingError::ThrottledClient(ref cause) => cause,
            GetMeetingError::UnauthorizedClient(ref cause) => cause,
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetPhoneNumberSettingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetPhoneNumberSettingsError {
    fn description(&self) -> &str {
        match *self {
            GetPhoneNumberSettingsError::BadRequest(ref cause) => cause,
            GetPhoneNumberSettingsError::Forbidden(ref cause) => cause,
            GetPhoneNumberSettingsError::ServiceFailure(ref cause) => cause,
            GetPhoneNumberSettingsError::ServiceUnavailable(ref cause) => cause,
            GetPhoneNumberSettingsError::ThrottledClient(ref cause) => cause,
            GetPhoneNumberSettingsError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
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
                "UnauthorizedClientException" => {
                    return RusotoError::Service(GetRoomError::UnauthorizedClient(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetRoomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetRoomError {
    fn description(&self) -> &str {
        match *self {
            GetRoomError::BadRequest(ref cause) => cause,
            GetRoomError::Forbidden(ref cause) => cause,
            GetRoomError::NotFound(ref cause) => cause,
            GetRoomError::ServiceFailure(ref cause) => cause,
            GetRoomError::ServiceUnavailable(ref cause) => cause,
            GetRoomError::UnauthorizedClient(ref cause) => cause,
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetVoiceConnectorGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetVoiceConnectorGroupError {
    fn description(&self) -> &str {
        match *self {
            GetVoiceConnectorGroupError::BadRequest(ref cause) => cause,
            GetVoiceConnectorGroupError::Forbidden(ref cause) => cause,
            GetVoiceConnectorGroupError::NotFound(ref cause) => cause,
            GetVoiceConnectorGroupError::ServiceFailure(ref cause) => cause,
            GetVoiceConnectorGroupError::ServiceUnavailable(ref cause) => cause,
            GetVoiceConnectorGroupError::ThrottledClient(ref cause) => cause,
            GetVoiceConnectorGroupError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetVoiceConnectorLoggingConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetVoiceConnectorLoggingConfigurationError {
    fn description(&self) -> &str {
        match *self {
            GetVoiceConnectorLoggingConfigurationError::BadRequest(ref cause) => cause,
            GetVoiceConnectorLoggingConfigurationError::Forbidden(ref cause) => cause,
            GetVoiceConnectorLoggingConfigurationError::NotFound(ref cause) => cause,
            GetVoiceConnectorLoggingConfigurationError::ServiceFailure(ref cause) => cause,
            GetVoiceConnectorLoggingConfigurationError::ServiceUnavailable(ref cause) => cause,
            GetVoiceConnectorLoggingConfigurationError::ThrottledClient(ref cause) => cause,
            GetVoiceConnectorLoggingConfigurationError::UnauthorizedClient(ref cause) => cause,
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetVoiceConnectorStreamingConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetVoiceConnectorStreamingConfigurationError {
    fn description(&self) -> &str {
        match *self {
            GetVoiceConnectorStreamingConfigurationError::BadRequest(ref cause) => cause,
            GetVoiceConnectorStreamingConfigurationError::Forbidden(ref cause) => cause,
            GetVoiceConnectorStreamingConfigurationError::NotFound(ref cause) => cause,
            GetVoiceConnectorStreamingConfigurationError::ServiceFailure(ref cause) => cause,
            GetVoiceConnectorStreamingConfigurationError::ServiceUnavailable(ref cause) => cause,
            GetVoiceConnectorStreamingConfigurationError::ThrottledClient(ref cause) => cause,
            GetVoiceConnectorStreamingConfigurationError::UnauthorizedClient(ref cause) => cause,
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListAttendeesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAttendeesError {
    fn description(&self) -> &str {
        match *self {
            ListAttendeesError::BadRequest(ref cause) => cause,
            ListAttendeesError::Forbidden(ref cause) => cause,
            ListAttendeesError::NotFound(ref cause) => cause,
            ListAttendeesError::ServiceFailure(ref cause) => cause,
            ListAttendeesError::ServiceUnavailable(ref cause) => cause,
            ListAttendeesError::ThrottledClient(ref cause) => cause,
            ListAttendeesError::UnauthorizedClient(ref cause) => cause,
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListMeetingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListMeetingsError {
    fn description(&self) -> &str {
        match *self {
            ListMeetingsError::BadRequest(ref cause) => cause,
            ListMeetingsError::Forbidden(ref cause) => cause,
            ListMeetingsError::ServiceFailure(ref cause) => cause,
            ListMeetingsError::ServiceUnavailable(ref cause) => cause,
            ListMeetingsError::ThrottledClient(ref cause) => cause,
            ListMeetingsError::UnauthorizedClient(ref cause) => cause,
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
                "UnauthorizedClientException" => {
                    return RusotoError::Service(ListRoomMembershipsError::UnauthorizedClient(
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
impl fmt::Display for ListRoomMembershipsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListRoomMembershipsError {
    fn description(&self) -> &str {
        match *self {
            ListRoomMembershipsError::BadRequest(ref cause) => cause,
            ListRoomMembershipsError::Forbidden(ref cause) => cause,
            ListRoomMembershipsError::NotFound(ref cause) => cause,
            ListRoomMembershipsError::ServiceFailure(ref cause) => cause,
            ListRoomMembershipsError::ServiceUnavailable(ref cause) => cause,
            ListRoomMembershipsError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
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
                "UnauthorizedClientException" => {
                    return RusotoError::Service(ListRoomsError::UnauthorizedClient(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListRoomsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListRoomsError {
    fn description(&self) -> &str {
        match *self {
            ListRoomsError::BadRequest(ref cause) => cause,
            ListRoomsError::Forbidden(ref cause) => cause,
            ListRoomsError::NotFound(ref cause) => cause,
            ListRoomsError::ServiceFailure(ref cause) => cause,
            ListRoomsError::ServiceUnavailable(ref cause) => cause,
            ListRoomsError::UnauthorizedClient(ref cause) => cause,
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListVoiceConnectorGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListVoiceConnectorGroupsError {
    fn description(&self) -> &str {
        match *self {
            ListVoiceConnectorGroupsError::BadRequest(ref cause) => cause,
            ListVoiceConnectorGroupsError::Forbidden(ref cause) => cause,
            ListVoiceConnectorGroupsError::ServiceFailure(ref cause) => cause,
            ListVoiceConnectorGroupsError::ServiceUnavailable(ref cause) => cause,
            ListVoiceConnectorGroupsError::ThrottledClient(ref cause) => cause,
            ListVoiceConnectorGroupsError::UnauthorizedClient(ref cause) => cause,
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutVoiceConnectorLoggingConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutVoiceConnectorLoggingConfigurationError {
    fn description(&self) -> &str {
        match *self {
            PutVoiceConnectorLoggingConfigurationError::BadRequest(ref cause) => cause,
            PutVoiceConnectorLoggingConfigurationError::Forbidden(ref cause) => cause,
            PutVoiceConnectorLoggingConfigurationError::NotFound(ref cause) => cause,
            PutVoiceConnectorLoggingConfigurationError::ServiceFailure(ref cause) => cause,
            PutVoiceConnectorLoggingConfigurationError::ServiceUnavailable(ref cause) => cause,
            PutVoiceConnectorLoggingConfigurationError::ThrottledClient(ref cause) => cause,
            PutVoiceConnectorLoggingConfigurationError::UnauthorizedClient(ref cause) => cause,
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutVoiceConnectorStreamingConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutVoiceConnectorStreamingConfigurationError {
    fn description(&self) -> &str {
        match *self {
            PutVoiceConnectorStreamingConfigurationError::BadRequest(ref cause) => cause,
            PutVoiceConnectorStreamingConfigurationError::Forbidden(ref cause) => cause,
            PutVoiceConnectorStreamingConfigurationError::NotFound(ref cause) => cause,
            PutVoiceConnectorStreamingConfigurationError::ServiceFailure(ref cause) => cause,
            PutVoiceConnectorStreamingConfigurationError::ServiceUnavailable(ref cause) => cause,
            PutVoiceConnectorStreamingConfigurationError::ThrottledClient(ref cause) => cause,
            PutVoiceConnectorStreamingConfigurationError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
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
            PutVoiceConnectorTerminationError::AccessDenied(ref cause) => cause,
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
            RestorePhoneNumberError::ResourceLimitExceeded(ref cause) => cause,
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
            SearchAvailablePhoneNumbersError::AccessDenied(ref cause) => cause,
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdatePhoneNumberSettingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdatePhoneNumberSettingsError {
    fn description(&self) -> &str {
        match *self {
            UpdatePhoneNumberSettingsError::BadRequest(ref cause) => cause,
            UpdatePhoneNumberSettingsError::Forbidden(ref cause) => cause,
            UpdatePhoneNumberSettingsError::ServiceFailure(ref cause) => cause,
            UpdatePhoneNumberSettingsError::ServiceUnavailable(ref cause) => cause,
            UpdatePhoneNumberSettingsError::ThrottledClient(ref cause) => cause,
            UpdatePhoneNumberSettingsError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
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
                "UnauthorizedClientException" => {
                    return RusotoError::Service(UpdateRoomError::UnauthorizedClient(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateRoomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateRoomError {
    fn description(&self) -> &str {
        match *self {
            UpdateRoomError::BadRequest(ref cause) => cause,
            UpdateRoomError::Forbidden(ref cause) => cause,
            UpdateRoomError::NotFound(ref cause) => cause,
            UpdateRoomError::ServiceFailure(ref cause) => cause,
            UpdateRoomError::ServiceUnavailable(ref cause) => cause,
            UpdateRoomError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
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
                "UnauthorizedClientException" => {
                    return RusotoError::Service(UpdateRoomMembershipError::UnauthorizedClient(
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
impl fmt::Display for UpdateRoomMembershipError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateRoomMembershipError {
    fn description(&self) -> &str {
        match *self {
            UpdateRoomMembershipError::BadRequest(ref cause) => cause,
            UpdateRoomMembershipError::Forbidden(ref cause) => cause,
            UpdateRoomMembershipError::NotFound(ref cause) => cause,
            UpdateRoomMembershipError::ServiceFailure(ref cause) => cause,
            UpdateRoomMembershipError::ServiceUnavailable(ref cause) => cause,
            UpdateRoomMembershipError::UnauthorizedClient(ref cause) => cause,
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateVoiceConnectorGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateVoiceConnectorGroupError {
    fn description(&self) -> &str {
        match *self {
            UpdateVoiceConnectorGroupError::BadRequest(ref cause) => cause,
            UpdateVoiceConnectorGroupError::Conflict(ref cause) => cause,
            UpdateVoiceConnectorGroupError::Forbidden(ref cause) => cause,
            UpdateVoiceConnectorGroupError::NotFound(ref cause) => cause,
            UpdateVoiceConnectorGroupError::ServiceFailure(ref cause) => cause,
            UpdateVoiceConnectorGroupError::ServiceUnavailable(ref cause) => cause,
            UpdateVoiceConnectorGroupError::ThrottledClient(ref cause) => cause,
            UpdateVoiceConnectorGroupError::UnauthorizedClient(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon Chime API. Amazon Chime clients implement this trait.
pub trait Chime: region::GetRegion {
    /// <p>Associates a phone number with the specified Amazon Chime user.</p>
    fn associate_phone_number_with_user(
        &self,
        input: AssociatePhoneNumberWithUserRequest,
    ) -> RusotoFuture<AssociatePhoneNumberWithUserResponse, AssociatePhoneNumberWithUserError>;

    /// <p>Associates phone numbers with the specified Amazon Chime Voice Connector.</p>
    fn associate_phone_numbers_with_voice_connector(
        &self,
        input: AssociatePhoneNumbersWithVoiceConnectorRequest,
    ) -> RusotoFuture<
        AssociatePhoneNumbersWithVoiceConnectorResponse,
        AssociatePhoneNumbersWithVoiceConnectorError,
    >;

    /// <p>Associates phone numbers with the specified Amazon Chime Voice Connector group.</p>
    fn associate_phone_numbers_with_voice_connector_group(
        &self,
        input: AssociatePhoneNumbersWithVoiceConnectorGroupRequest,
    ) -> RusotoFuture<
        AssociatePhoneNumbersWithVoiceConnectorGroupResponse,
        AssociatePhoneNumbersWithVoiceConnectorGroupError,
    >;

    /// <p>Creates up to 100 new attendees for an active Amazon Chime SDK meeting. For more information about the Amazon Chime SDK, see <a href="https://docs.aws.amazon.com/chime/latest/dg/meetings-sdk.html">Using the Amazon Chime SDK</a> in the <i>Amazon Chime Developer Guide</i>. </p>
    fn batch_create_attendee(
        &self,
        input: BatchCreateAttendeeRequest,
    ) -> RusotoFuture<BatchCreateAttendeeResponse, BatchCreateAttendeeError>;

    /// <p>Adds up to 50 members to a chat room. Members can be either users or bots. The member role designates whether the member is a chat room administrator or a general chat room member.</p>
    fn batch_create_room_membership(
        &self,
        input: BatchCreateRoomMembershipRequest,
    ) -> RusotoFuture<BatchCreateRoomMembershipResponse, BatchCreateRoomMembershipError>;

    /// <p>Moves phone numbers into the <b>Deletion queue</b>. Phone numbers must be disassociated from any users or Amazon Chime Voice Connectors before they can be deleted.</p> <p>Phone numbers remain in the <b>Deletion queue</b> for 7 days before they are deleted permanently.</p>
    fn batch_delete_phone_number(
        &self,
        input: BatchDeletePhoneNumberRequest,
    ) -> RusotoFuture<BatchDeletePhoneNumberResponse, BatchDeletePhoneNumberError>;

    /// <p>Suspends up to 50 users from a <code>Team</code> or <code>EnterpriseLWA</code> Amazon Chime account. For more information about different account types, see <a href="https://docs.aws.amazon.com/chime/latest/ag/manage-chime-account.html">Managing Your Amazon Chime Accounts</a> in the <i>Amazon Chime Administration Guide</i>.</p> <p>Users suspended from a <code>Team</code> account are dissasociated from the account, but they can continue to use Amazon Chime as free users. To remove the suspension from suspended <code>Team</code> account users, invite them to the <code>Team</code> account again. You can use the <a>InviteUsers</a> action to do so.</p> <p>Users suspended from an <code>EnterpriseLWA</code> account are immediately signed out of Amazon Chime and can no longer sign in. To remove the suspension from suspended <code>EnterpriseLWA</code> account users, use the <a>BatchUnsuspendUser</a> action. </p> <p>To sign out users without suspending them, use the <a>LogoutUser</a> action.</p>
    fn batch_suspend_user(
        &self,
        input: BatchSuspendUserRequest,
    ) -> RusotoFuture<BatchSuspendUserResponse, BatchSuspendUserError>;

    /// <p>Removes the suspension from up to 50 previously suspended users for the specified Amazon Chime <code>EnterpriseLWA</code> account. Only users on <code>EnterpriseLWA</code> accounts can be unsuspended using this action. For more information about different account types, see <a href="https://docs.aws.amazon.com/chime/latest/ag/manage-chime-account.html">Managing Your Amazon Chime Accounts</a> in the <i>Amazon Chime Administration Guide</i>.</p> <p>Previously suspended users who are unsuspended using this action are returned to <code>Registered</code> status. Users who are not previously suspended are ignored.</p>
    fn batch_unsuspend_user(
        &self,
        input: BatchUnsuspendUserRequest,
    ) -> RusotoFuture<BatchUnsuspendUserResponse, BatchUnsuspendUserError>;

    /// <p>Updates phone number product types or calling names. You can update one attribute at a time for each <code>UpdatePhoneNumberRequestItem</code>. For example, you can update either the product type or the calling name.</p> <p>For product types, choose from Amazon Chime Business Calling and Amazon Chime Voice Connector. For toll-free numbers, you must use the Amazon Chime Voice Connector product type.</p> <p>Updates to outbound calling names can take up to 72 hours to complete. Pending updates to outbound calling names must be complete before you can request another update.</p>
    fn batch_update_phone_number(
        &self,
        input: BatchUpdatePhoneNumberRequest,
    ) -> RusotoFuture<BatchUpdatePhoneNumberResponse, BatchUpdatePhoneNumberError>;

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

    /// <p>Creates a new attendee for an active Amazon Chime SDK meeting. For more information about the Amazon Chime SDK, see <a href="https://docs.aws.amazon.com/chime/latest/dg/meetings-sdk.html">Using the Amazon Chime SDK</a> in the <i>Amazon Chime Developer Guide</i>.</p>
    fn create_attendee(
        &self,
        input: CreateAttendeeRequest,
    ) -> RusotoFuture<CreateAttendeeResponse, CreateAttendeeError>;

    /// <p>Creates a bot for an Amazon Chime Enterprise account.</p>
    fn create_bot(
        &self,
        input: CreateBotRequest,
    ) -> RusotoFuture<CreateBotResponse, CreateBotError>;

    /// <p>Creates a new Amazon Chime SDK meeting in the specified media Region with no initial attendees. For more information about the Amazon Chime SDK, see <a href="https://docs.aws.amazon.com/chime/latest/dg/meetings-sdk.html">Using the Amazon Chime SDK</a> in the <i>Amazon Chime Developer Guide</i>.</p>
    fn create_meeting(
        &self,
        input: CreateMeetingRequest,
    ) -> RusotoFuture<CreateMeetingResponse, CreateMeetingError>;

    /// <p>Creates an order for phone numbers to be provisioned. Choose from Amazon Chime Business Calling and Amazon Chime Voice Connector product types. For toll-free numbers, you must use the Amazon Chime Voice Connector product type.</p>
    fn create_phone_number_order(
        &self,
        input: CreatePhoneNumberOrderRequest,
    ) -> RusotoFuture<CreatePhoneNumberOrderResponse, CreatePhoneNumberOrderError>;

    /// <p>Creates a chat room for the specified Amazon Chime account.</p>
    fn create_room(
        &self,
        input: CreateRoomRequest,
    ) -> RusotoFuture<CreateRoomResponse, CreateRoomError>;

    /// <p>Adds a member to a chat room. A member can be either a user or a bot. The member role designates whether the member is a chat room administrator or a general chat room member.</p>
    fn create_room_membership(
        &self,
        input: CreateRoomMembershipRequest,
    ) -> RusotoFuture<CreateRoomMembershipResponse, CreateRoomMembershipError>;

    /// <p>Creates an Amazon Chime Voice Connector under the administrator's AWS account. You can choose to create an Amazon Chime Voice Connector in a specific AWS Region.</p> <p>Enabling <a>CreateVoiceConnectorRequest$RequireEncryption</a> configures your Amazon Chime Voice Connector to use TLS transport for SIP signaling and Secure RTP (SRTP) for media. Inbound calls use TLS transport, and unencrypted outbound calls are blocked.</p>
    fn create_voice_connector(
        &self,
        input: CreateVoiceConnectorRequest,
    ) -> RusotoFuture<CreateVoiceConnectorResponse, CreateVoiceConnectorError>;

    /// <p>Creates an Amazon Chime Voice Connector group under the administrator's AWS account. You can associate up to three existing Amazon Chime Voice Connectors with the Amazon Chime Voice Connector group by including <code>VoiceConnectorItems</code> in the request.</p> <p>You can include Amazon Chime Voice Connectors from different AWS Regions in your group. This creates a fault tolerant mechanism for fallback in case of availability events.</p>
    fn create_voice_connector_group(
        &self,
        input: CreateVoiceConnectorGroupRequest,
    ) -> RusotoFuture<CreateVoiceConnectorGroupResponse, CreateVoiceConnectorGroupError>;

    /// <p>Deletes the specified Amazon Chime account. You must suspend all users before deleting a <code>Team</code> account. You can use the <a>BatchSuspendUser</a> action to do so.</p> <p>For <code>EnterpriseLWA</code> and <code>EnterpriseAD</code> accounts, you must release the claimed domains for your Amazon Chime account before deletion. As soon as you release the domain, all users under that account are suspended.</p> <p>Deleted accounts appear in your <code>Disabled</code> accounts list for 90 days. To restore a deleted account from your <code>Disabled</code> accounts list, you must contact AWS Support.</p> <p>After 90 days, deleted accounts are permanently removed from your <code>Disabled</code> accounts list.</p>
    fn delete_account(
        &self,
        input: DeleteAccountRequest,
    ) -> RusotoFuture<DeleteAccountResponse, DeleteAccountError>;

    /// <p>Deletes an attendee from the specified Amazon Chime SDK meeting and deletes their <code>JoinToken</code>. Attendees are automatically deleted when a Amazon Chime SDK meeting is deleted. For more information about the Amazon Chime SDK, see <a href="https://docs.aws.amazon.com/chime/latest/dg/meetings-sdk.html">Using the Amazon Chime SDK</a> in the <i>Amazon Chime Developer Guide</i>.</p>
    fn delete_attendee(
        &self,
        input: DeleteAttendeeRequest,
    ) -> RusotoFuture<(), DeleteAttendeeError>;

    /// <p>Deletes the events configuration that allows a bot to receive outgoing events.</p>
    fn delete_events_configuration(
        &self,
        input: DeleteEventsConfigurationRequest,
    ) -> RusotoFuture<(), DeleteEventsConfigurationError>;

    /// <p>Deletes the specified Amazon Chime SDK meeting. When a meeting is deleted, its attendees are also deleted and clients can no longer join it. For more information about the Amazon Chime SDK, see <a href="https://docs.aws.amazon.com/chime/latest/dg/meetings-sdk.html">Using the Amazon Chime SDK</a> in the <i>Amazon Chime Developer Guide</i>.</p>
    fn delete_meeting(&self, input: DeleteMeetingRequest) -> RusotoFuture<(), DeleteMeetingError>;

    /// <p>Moves the specified phone number into the <b>Deletion queue</b>. A phone number must be disassociated from any users or Amazon Chime Voice Connectors before it can be deleted.</p> <p>Deleted phone numbers remain in the <b>Deletion queue</b> for 7 days before they are deleted permanently.</p>
    fn delete_phone_number(
        &self,
        input: DeletePhoneNumberRequest,
    ) -> RusotoFuture<(), DeletePhoneNumberError>;

    /// <p>Deletes a chat room.</p>
    fn delete_room(&self, input: DeleteRoomRequest) -> RusotoFuture<(), DeleteRoomError>;

    /// <p>Removes a member from a chat room.</p>
    fn delete_room_membership(
        &self,
        input: DeleteRoomMembershipRequest,
    ) -> RusotoFuture<(), DeleteRoomMembershipError>;

    /// <p>Deletes the specified Amazon Chime Voice Connector. Any phone numbers associated with the Amazon Chime Voice Connector must be disassociated from it before it can be deleted.</p>
    fn delete_voice_connector(
        &self,
        input: DeleteVoiceConnectorRequest,
    ) -> RusotoFuture<(), DeleteVoiceConnectorError>;

    /// <p>Deletes the specified Amazon Chime Voice Connector group. Any <code>VoiceConnectorItems</code> and phone numbers associated with the group must be removed before it can be deleted.</p>
    fn delete_voice_connector_group(
        &self,
        input: DeleteVoiceConnectorGroupRequest,
    ) -> RusotoFuture<(), DeleteVoiceConnectorGroupError>;

    /// <p>Deletes the origination settings for the specified Amazon Chime Voice Connector.</p>
    fn delete_voice_connector_origination(
        &self,
        input: DeleteVoiceConnectorOriginationRequest,
    ) -> RusotoFuture<(), DeleteVoiceConnectorOriginationError>;

    /// <p>Deletes the streaming configuration for the specified Amazon Chime Voice Connector.</p>
    fn delete_voice_connector_streaming_configuration(
        &self,
        input: DeleteVoiceConnectorStreamingConfigurationRequest,
    ) -> RusotoFuture<(), DeleteVoiceConnectorStreamingConfigurationError>;

    /// <p>Deletes the termination settings for the specified Amazon Chime Voice Connector.</p>
    fn delete_voice_connector_termination(
        &self,
        input: DeleteVoiceConnectorTerminationRequest,
    ) -> RusotoFuture<(), DeleteVoiceConnectorTerminationError>;

    /// <p>Deletes the specified SIP credentials used by your equipment to authenticate during call termination.</p>
    fn delete_voice_connector_termination_credentials(
        &self,
        input: DeleteVoiceConnectorTerminationCredentialsRequest,
    ) -> RusotoFuture<(), DeleteVoiceConnectorTerminationCredentialsError>;

    /// <p>Disassociates the primary provisioned phone number from the specified Amazon Chime user.</p>
    fn disassociate_phone_number_from_user(
        &self,
        input: DisassociatePhoneNumberFromUserRequest,
    ) -> RusotoFuture<DisassociatePhoneNumberFromUserResponse, DisassociatePhoneNumberFromUserError>;

    /// <p>Disassociates the specified phone numbers from the specified Amazon Chime Voice Connector.</p>
    fn disassociate_phone_numbers_from_voice_connector(
        &self,
        input: DisassociatePhoneNumbersFromVoiceConnectorRequest,
    ) -> RusotoFuture<
        DisassociatePhoneNumbersFromVoiceConnectorResponse,
        DisassociatePhoneNumbersFromVoiceConnectorError,
    >;

    /// <p>Disassociates the specified phone numbers from the specified Amazon Chime Voice Connector group.</p>
    fn disassociate_phone_numbers_from_voice_connector_group(
        &self,
        input: DisassociatePhoneNumbersFromVoiceConnectorGroupRequest,
    ) -> RusotoFuture<
        DisassociatePhoneNumbersFromVoiceConnectorGroupResponse,
        DisassociatePhoneNumbersFromVoiceConnectorGroupError,
    >;

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

    /// <p>Gets the Amazon Chime SDK attendee details for a specified meeting ID and attendee ID. For more information about the Amazon Chime SDK, see <a href="https://docs.aws.amazon.com/chime/latest/dg/meetings-sdk.html">Using the Amazon Chime SDK</a> in the <i>Amazon Chime Developer Guide</i>.</p>
    fn get_attendee(
        &self,
        input: GetAttendeeRequest,
    ) -> RusotoFuture<GetAttendeeResponse, GetAttendeeError>;

    /// <p>Retrieves details for the specified bot, such as bot email address, bot type, status, and display name.</p>
    fn get_bot(&self, input: GetBotRequest) -> RusotoFuture<GetBotResponse, GetBotError>;

    /// <p>Gets details for an events configuration that allows a bot to receive outgoing events, such as an HTTPS endpoint or Lambda function ARN. </p>
    fn get_events_configuration(
        &self,
        input: GetEventsConfigurationRequest,
    ) -> RusotoFuture<GetEventsConfigurationResponse, GetEventsConfigurationError>;

    /// <p>Retrieves global settings for the administrator's AWS account, such as Amazon Chime Business Calling and Amazon Chime Voice Connector settings.</p>
    fn get_global_settings(
        &self,
    ) -> RusotoFuture<GetGlobalSettingsResponse, GetGlobalSettingsError>;

    /// <p>Gets the Amazon Chime SDK meeting details for the specified meeting ID. For more information about the Amazon Chime SDK, see <a href="https://docs.aws.amazon.com/chime/latest/dg/meetings-sdk.html">Using the Amazon Chime SDK</a> in the <i>Amazon Chime Developer Guide</i>.</p>
    fn get_meeting(
        &self,
        input: GetMeetingRequest,
    ) -> RusotoFuture<GetMeetingResponse, GetMeetingError>;

    /// <p>Retrieves details for the specified phone number ID, such as associations, capabilities, and product type.</p>
    fn get_phone_number(
        &self,
        input: GetPhoneNumberRequest,
    ) -> RusotoFuture<GetPhoneNumberResponse, GetPhoneNumberError>;

    /// <p>Retrieves details for the specified phone number order, such as order creation timestamp, phone numbers in E.164 format, product type, and order status.</p>
    fn get_phone_number_order(
        &self,
        input: GetPhoneNumberOrderRequest,
    ) -> RusotoFuture<GetPhoneNumberOrderResponse, GetPhoneNumberOrderError>;

    /// <p>Retrieves the phone number settings for the administrator's AWS account, such as the default outbound calling name.</p>
    fn get_phone_number_settings(
        &self,
    ) -> RusotoFuture<GetPhoneNumberSettingsResponse, GetPhoneNumberSettingsError>;

    /// <p>Retrieves room details, such as name.</p>
    fn get_room(&self, input: GetRoomRequest) -> RusotoFuture<GetRoomResponse, GetRoomError>;

    /// <p>Retrieves details for the specified user ID, such as primary email address, license type, and personal meeting PIN.</p> <p>To retrieve user details with an email address instead of a user ID, use the <a>ListUsers</a> action, and then filter by email address.</p>
    fn get_user(&self, input: GetUserRequest) -> RusotoFuture<GetUserResponse, GetUserError>;

    /// <p>Retrieves settings for the specified user ID, such as any associated phone number settings.</p>
    fn get_user_settings(
        &self,
        input: GetUserSettingsRequest,
    ) -> RusotoFuture<GetUserSettingsResponse, GetUserSettingsError>;

    /// <p>Retrieves details for the specified Amazon Chime Voice Connector, such as timestamps, name, outbound host, and encryption requirements.</p>
    fn get_voice_connector(
        &self,
        input: GetVoiceConnectorRequest,
    ) -> RusotoFuture<GetVoiceConnectorResponse, GetVoiceConnectorError>;

    /// <p>Retrieves details for the specified Amazon Chime Voice Connector group, such as timestamps, name, and associated <code>VoiceConnectorItems</code>.</p>
    fn get_voice_connector_group(
        &self,
        input: GetVoiceConnectorGroupRequest,
    ) -> RusotoFuture<GetVoiceConnectorGroupResponse, GetVoiceConnectorGroupError>;

    /// <p>Retrieves the logging configuration details for the specified Amazon Chime Voice Connector. Shows whether SIP message logs are enabled for sending to Amazon CloudWatch Logs.</p>
    fn get_voice_connector_logging_configuration(
        &self,
        input: GetVoiceConnectorLoggingConfigurationRequest,
    ) -> RusotoFuture<
        GetVoiceConnectorLoggingConfigurationResponse,
        GetVoiceConnectorLoggingConfigurationError,
    >;

    /// <p>Retrieves origination setting details for the specified Amazon Chime Voice Connector.</p>
    fn get_voice_connector_origination(
        &self,
        input: GetVoiceConnectorOriginationRequest,
    ) -> RusotoFuture<GetVoiceConnectorOriginationResponse, GetVoiceConnectorOriginationError>;

    /// <p>Retrieves the streaming configuration details for the specified Amazon Chime Voice Connector. Shows whether media streaming is enabled for sending to Amazon Kinesis. It also shows the retention period, in hours, for the Amazon Kinesis data.</p>
    fn get_voice_connector_streaming_configuration(
        &self,
        input: GetVoiceConnectorStreamingConfigurationRequest,
    ) -> RusotoFuture<
        GetVoiceConnectorStreamingConfigurationResponse,
        GetVoiceConnectorStreamingConfigurationError,
    >;

    /// <p>Retrieves termination setting details for the specified Amazon Chime Voice Connector.</p>
    fn get_voice_connector_termination(
        &self,
        input: GetVoiceConnectorTerminationRequest,
    ) -> RusotoFuture<GetVoiceConnectorTerminationResponse, GetVoiceConnectorTerminationError>;

    /// <p>Retrieves information about the last time a SIP <code>OPTIONS</code> ping was received from your SIP infrastructure for the specified Amazon Chime Voice Connector.</p>
    fn get_voice_connector_termination_health(
        &self,
        input: GetVoiceConnectorTerminationHealthRequest,
    ) -> RusotoFuture<
        GetVoiceConnectorTerminationHealthResponse,
        GetVoiceConnectorTerminationHealthError,
    >;

    /// <p>Sends email to a maximum of 50 users, inviting them to the specified Amazon Chime <code>Team</code> account. Only <code>Team</code> account types are currently supported for this action. </p>
    fn invite_users(
        &self,
        input: InviteUsersRequest,
    ) -> RusotoFuture<InviteUsersResponse, InviteUsersError>;

    /// <p>Lists the Amazon Chime accounts under the administrator's AWS account. You can filter accounts by account name prefix. To find out which Amazon Chime account a user belongs to, you can filter by the user's email address, which returns one account result.</p>
    fn list_accounts(
        &self,
        input: ListAccountsRequest,
    ) -> RusotoFuture<ListAccountsResponse, ListAccountsError>;

    /// <p>Lists the attendees for the specified Amazon Chime SDK meeting. For more information about the Amazon Chime SDK, see <a href="https://docs.aws.amazon.com/chime/latest/dg/meetings-sdk.html">Using the Amazon Chime SDK</a> in the <i>Amazon Chime Developer Guide</i>.</p>
    fn list_attendees(
        &self,
        input: ListAttendeesRequest,
    ) -> RusotoFuture<ListAttendeesResponse, ListAttendeesError>;

    /// <p>Lists the bots associated with the administrator's Amazon Chime Enterprise account ID.</p>
    fn list_bots(&self, input: ListBotsRequest) -> RusotoFuture<ListBotsResponse, ListBotsError>;

    /// <p>Lists up to 100 active Amazon Chime SDK meetings. For more information about the Amazon Chime SDK, see <a href="https://docs.aws.amazon.com/chime/latest/dg/meetings-sdk.html">Using the Amazon Chime SDK</a> in the <i>Amazon Chime Developer Guide</i>.</p>
    fn list_meetings(
        &self,
        input: ListMeetingsRequest,
    ) -> RusotoFuture<ListMeetingsResponse, ListMeetingsError>;

    /// <p>Lists the phone number orders for the administrator's Amazon Chime account.</p>
    fn list_phone_number_orders(
        &self,
        input: ListPhoneNumberOrdersRequest,
    ) -> RusotoFuture<ListPhoneNumberOrdersResponse, ListPhoneNumberOrdersError>;

    /// <p>Lists the phone numbers for the specified Amazon Chime account, Amazon Chime user, Amazon Chime Voice Connector, or Amazon Chime Voice Connector group.</p>
    fn list_phone_numbers(
        &self,
        input: ListPhoneNumbersRequest,
    ) -> RusotoFuture<ListPhoneNumbersResponse, ListPhoneNumbersError>;

    /// <p>Lists the membership details for the specified room, such as member IDs, member email addresses, and member names.</p>
    fn list_room_memberships(
        &self,
        input: ListRoomMembershipsRequest,
    ) -> RusotoFuture<ListRoomMembershipsResponse, ListRoomMembershipsError>;

    /// <p>Lists the room details for the specified Amazon Chime account. Optionally, filter the results by a member ID (user ID or bot ID) to see a list of rooms that the member belongs to.</p>
    fn list_rooms(
        &self,
        input: ListRoomsRequest,
    ) -> RusotoFuture<ListRoomsResponse, ListRoomsError>;

    /// <p>Lists the users that belong to the specified Amazon Chime account. You can specify an email address to list only the user that the email address belongs to.</p>
    fn list_users(
        &self,
        input: ListUsersRequest,
    ) -> RusotoFuture<ListUsersResponse, ListUsersError>;

    /// <p>Lists the Amazon Chime Voice Connector groups for the administrator's AWS account.</p>
    fn list_voice_connector_groups(
        &self,
        input: ListVoiceConnectorGroupsRequest,
    ) -> RusotoFuture<ListVoiceConnectorGroupsResponse, ListVoiceConnectorGroupsError>;

    /// <p>Lists the SIP credentials for the specified Amazon Chime Voice Connector.</p>
    fn list_voice_connector_termination_credentials(
        &self,
        input: ListVoiceConnectorTerminationCredentialsRequest,
    ) -> RusotoFuture<
        ListVoiceConnectorTerminationCredentialsResponse,
        ListVoiceConnectorTerminationCredentialsError,
    >;

    /// <p>Lists the Amazon Chime Voice Connectors for the administrator's AWS account.</p>
    fn list_voice_connectors(
        &self,
        input: ListVoiceConnectorsRequest,
    ) -> RusotoFuture<ListVoiceConnectorsResponse, ListVoiceConnectorsError>;

    /// <p>Logs out the specified user from all of the devices they are currently logged into.</p>
    fn logout_user(
        &self,
        input: LogoutUserRequest,
    ) -> RusotoFuture<LogoutUserResponse, LogoutUserError>;

    /// <p>Creates an events configuration that allows a bot to receive outgoing events sent by Amazon Chime. Choose either an HTTPS endpoint or a Lambda function ARN. For more information, see <a>Bot</a>.</p>
    fn put_events_configuration(
        &self,
        input: PutEventsConfigurationRequest,
    ) -> RusotoFuture<PutEventsConfigurationResponse, PutEventsConfigurationError>;

    /// <p>Adds a logging configuration for the specified Amazon Chime Voice Connector. The logging configuration specifies whether SIP message logs are enabled for sending to Amazon CloudWatch Logs.</p>
    fn put_voice_connector_logging_configuration(
        &self,
        input: PutVoiceConnectorLoggingConfigurationRequest,
    ) -> RusotoFuture<
        PutVoiceConnectorLoggingConfigurationResponse,
        PutVoiceConnectorLoggingConfigurationError,
    >;

    /// <p>Adds origination settings for the specified Amazon Chime Voice Connector.</p>
    fn put_voice_connector_origination(
        &self,
        input: PutVoiceConnectorOriginationRequest,
    ) -> RusotoFuture<PutVoiceConnectorOriginationResponse, PutVoiceConnectorOriginationError>;

    /// <p>Adds a streaming configuration for the specified Amazon Chime Voice Connector. The streaming configuration specifies whether media streaming is enabled for sending to Amazon Kinesis. It also sets the retention period, in hours, for the Amazon Kinesis data.</p>
    fn put_voice_connector_streaming_configuration(
        &self,
        input: PutVoiceConnectorStreamingConfigurationRequest,
    ) -> RusotoFuture<
        PutVoiceConnectorStreamingConfigurationResponse,
        PutVoiceConnectorStreamingConfigurationError,
    >;

    /// <p>Adds termination settings for the specified Amazon Chime Voice Connector.</p>
    fn put_voice_connector_termination(
        &self,
        input: PutVoiceConnectorTerminationRequest,
    ) -> RusotoFuture<PutVoiceConnectorTerminationResponse, PutVoiceConnectorTerminationError>;

    /// <p>Adds termination SIP credentials for the specified Amazon Chime Voice Connector.</p>
    fn put_voice_connector_termination_credentials(
        &self,
        input: PutVoiceConnectorTerminationCredentialsRequest,
    ) -> RusotoFuture<(), PutVoiceConnectorTerminationCredentialsError>;

    /// <p>Regenerates the security token for a bot.</p>
    fn regenerate_security_token(
        &self,
        input: RegenerateSecurityTokenRequest,
    ) -> RusotoFuture<RegenerateSecurityTokenResponse, RegenerateSecurityTokenError>;

    /// <p>Resets the personal meeting PIN for the specified user on an Amazon Chime account. Returns the <a>User</a> object with the updated personal meeting PIN.</p>
    fn reset_personal_pin(
        &self,
        input: ResetPersonalPINRequest,
    ) -> RusotoFuture<ResetPersonalPINResponse, ResetPersonalPINError>;

    /// <p>Moves a phone number from the <b>Deletion queue</b> back into the phone number <b>Inventory</b>.</p>
    fn restore_phone_number(
        &self,
        input: RestorePhoneNumberRequest,
    ) -> RusotoFuture<RestorePhoneNumberResponse, RestorePhoneNumberError>;

    /// <p>Searches phone numbers that can be ordered.</p>
    fn search_available_phone_numbers(
        &self,
        input: SearchAvailablePhoneNumbersRequest,
    ) -> RusotoFuture<SearchAvailablePhoneNumbersResponse, SearchAvailablePhoneNumbersError>;

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

    /// <p>Updates the status of the specified bot, such as starting or stopping the bot from running in your Amazon Chime Enterprise account.</p>
    fn update_bot(
        &self,
        input: UpdateBotRequest,
    ) -> RusotoFuture<UpdateBotResponse, UpdateBotError>;

    /// <p>Updates global settings for the administrator's AWS account, such as Amazon Chime Business Calling and Amazon Chime Voice Connector settings.</p>
    fn update_global_settings(
        &self,
        input: UpdateGlobalSettingsRequest,
    ) -> RusotoFuture<(), UpdateGlobalSettingsError>;

    /// <p>Updates phone number details, such as product type or calling name, for the specified phone number ID. You can update one phone number detail at a time. For example, you can update either the product type or the calling name in one action.</p> <p>For toll-free numbers, you must use the Amazon Chime Voice Connector product type.</p> <p>Updates to outbound calling names can take up to 72 hours to complete. Pending updates to outbound calling names must be complete before you can request another update.</p>
    fn update_phone_number(
        &self,
        input: UpdatePhoneNumberRequest,
    ) -> RusotoFuture<UpdatePhoneNumberResponse, UpdatePhoneNumberError>;

    /// <p>Updates the phone number settings for the administrator's AWS account, such as the default outbound calling name. You can update the default outbound calling name once every seven days. Outbound calling names can take up to 72 hours to update.</p>
    fn update_phone_number_settings(
        &self,
        input: UpdatePhoneNumberSettingsRequest,
    ) -> RusotoFuture<(), UpdatePhoneNumberSettingsError>;

    /// <p>Updates room details, such as the room name.</p>
    fn update_room(
        &self,
        input: UpdateRoomRequest,
    ) -> RusotoFuture<UpdateRoomResponse, UpdateRoomError>;

    /// <p>Updates room membership details, such as member role. The member role designates whether the member is a chat room administrator or a general chat room member. Member role can only be updated for user IDs.</p>
    fn update_room_membership(
        &self,
        input: UpdateRoomMembershipRequest,
    ) -> RusotoFuture<UpdateRoomMembershipResponse, UpdateRoomMembershipError>;

    /// <p>Updates user details for a specified user ID. Currently, only <code>LicenseType</code> updates are supported for this action.</p>
    fn update_user(
        &self,
        input: UpdateUserRequest,
    ) -> RusotoFuture<UpdateUserResponse, UpdateUserError>;

    /// <p>Updates the settings for the specified user, such as phone number settings.</p>
    fn update_user_settings(
        &self,
        input: UpdateUserSettingsRequest,
    ) -> RusotoFuture<(), UpdateUserSettingsError>;

    /// <p>Updates details for the specified Amazon Chime Voice Connector.</p>
    fn update_voice_connector(
        &self,
        input: UpdateVoiceConnectorRequest,
    ) -> RusotoFuture<UpdateVoiceConnectorResponse, UpdateVoiceConnectorError>;

    /// <p>Updates details for the specified Amazon Chime Voice Connector group, such as the name and Amazon Chime Voice Connector priority ranking.</p>
    fn update_voice_connector_group(
        &self,
        input: UpdateVoiceConnectorGroupRequest,
    ) -> RusotoFuture<UpdateVoiceConnectorGroupResponse, UpdateVoiceConnectorGroupError>;
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
        Self::new_with_client(Client::shared(), region)
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
        Self::new_with_client(
            Client::new_with(credentials_provider, request_dispatcher),
            region,
        )
    }

    pub fn new_with_client(client: Client, region: region::Region) -> ChimeClient {
        ChimeClient { client, region }
    }
}

impl fmt::Debug for ChimeClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ChimeClient")
            .field("region", &self.region)
            .finish()
    }
}

impl region::GetRegion for ChimeClient {
    fn region(&self) -> &region::Region {
        &self.region
    }
}

impl Chime for ChimeClient {
    /// <p>Associates a phone number with the specified Amazon Chime user.</p>
    fn associate_phone_number_with_user(
        &self,
        input: AssociatePhoneNumberWithUserRequest,
    ) -> RusotoFuture<AssociatePhoneNumberWithUserResponse, AssociatePhoneNumberWithUserError> {
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

        self.client.sign_and_dispatch(request, |response| {
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

    /// <p>Associates phone numbers with the specified Amazon Chime Voice Connector.</p>
    fn associate_phone_numbers_with_voice_connector(
        &self,
        input: AssociatePhoneNumbersWithVoiceConnectorRequest,
    ) -> RusotoFuture<
        AssociatePhoneNumbersWithVoiceConnectorResponse,
        AssociatePhoneNumbersWithVoiceConnectorError,
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

        self.client.sign_and_dispatch(request, |response| {
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

    /// <p>Associates phone numbers with the specified Amazon Chime Voice Connector group.</p>
    fn associate_phone_numbers_with_voice_connector_group(
        &self,
        input: AssociatePhoneNumbersWithVoiceConnectorGroupRequest,
    ) -> RusotoFuture<
        AssociatePhoneNumbersWithVoiceConnectorGroupResponse,
        AssociatePhoneNumbersWithVoiceConnectorGroupError,
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<AssociatePhoneNumbersWithVoiceConnectorGroupResponse, _>(
                    )?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AssociatePhoneNumbersWithVoiceConnectorGroupError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates up to 100 new attendees for an active Amazon Chime SDK meeting. For more information about the Amazon Chime SDK, see <a href="https://docs.aws.amazon.com/chime/latest/dg/meetings-sdk.html">Using the Amazon Chime SDK</a> in the <i>Amazon Chime Developer Guide</i>. </p>
    fn batch_create_attendee(
        &self,
        input: BatchCreateAttendeeRequest,
    ) -> RusotoFuture<BatchCreateAttendeeResponse, BatchCreateAttendeeError> {
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<BatchCreateAttendeeResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(BatchCreateAttendeeError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Adds up to 50 members to a chat room. Members can be either users or bots. The member role designates whether the member is a chat room administrator or a general chat room member.</p>
    fn batch_create_room_membership(
        &self,
        input: BatchCreateRoomMembershipRequest,
    ) -> RusotoFuture<BatchCreateRoomMembershipResponse, BatchCreateRoomMembershipError> {
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<BatchCreateRoomMembershipResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(BatchCreateRoomMembershipError::from_response(response))
                }))
            }
        })
    }

    /// <p>Moves phone numbers into the <b>Deletion queue</b>. Phone numbers must be disassociated from any users or Amazon Chime Voice Connectors before they can be deleted.</p> <p>Phone numbers remain in the <b>Deletion queue</b> for 7 days before they are deleted permanently.</p>
    fn batch_delete_phone_number(
        &self,
        input: BatchDeletePhoneNumberRequest,
    ) -> RusotoFuture<BatchDeletePhoneNumberResponse, BatchDeletePhoneNumberError> {
        let request_uri = "/phone-numbers";

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        params.put("operation", "batch-delete");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
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

    /// <p>Suspends up to 50 users from a <code>Team</code> or <code>EnterpriseLWA</code> Amazon Chime account. For more information about different account types, see <a href="https://docs.aws.amazon.com/chime/latest/ag/manage-chime-account.html">Managing Your Amazon Chime Accounts</a> in the <i>Amazon Chime Administration Guide</i>.</p> <p>Users suspended from a <code>Team</code> account are dissasociated from the account, but they can continue to use Amazon Chime as free users. To remove the suspension from suspended <code>Team</code> account users, invite them to the <code>Team</code> account again. You can use the <a>InviteUsers</a> action to do so.</p> <p>Users suspended from an <code>EnterpriseLWA</code> account are immediately signed out of Amazon Chime and can no longer sign in. To remove the suspension from suspended <code>EnterpriseLWA</code> account users, use the <a>BatchUnsuspendUser</a> action. </p> <p>To sign out users without suspending them, use the <a>LogoutUser</a> action.</p>
    fn batch_suspend_user(
        &self,
        input: BatchSuspendUserRequest,
    ) -> RusotoFuture<BatchSuspendUserResponse, BatchSuspendUserError> {
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

    /// <p>Updates phone number product types or calling names. You can update one attribute at a time for each <code>UpdatePhoneNumberRequestItem</code>. For example, you can update either the product type or the calling name.</p> <p>For product types, choose from Amazon Chime Business Calling and Amazon Chime Voice Connector. For toll-free numbers, you must use the Amazon Chime Voice Connector product type.</p> <p>Updates to outbound calling names can take up to 72 hours to complete. Pending updates to outbound calling names must be complete before you can request another update.</p>
    fn batch_update_phone_number(
        &self,
        input: BatchUpdatePhoneNumberRequest,
    ) -> RusotoFuture<BatchUpdatePhoneNumberResponse, BatchUpdatePhoneNumberError> {
        let request_uri = "/phone-numbers";

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        params.put("operation", "batch-update");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
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

    /// <p>Updates user details within the <a>UpdateUserRequestItem</a> object for up to 20 users for the specified Amazon Chime account. Currently, only <code>LicenseType</code> updates are supported for this action.</p>
    fn batch_update_user(
        &self,
        input: BatchUpdateUserRequest,
    ) -> RusotoFuture<BatchUpdateUserResponse, BatchUpdateUserError> {
        let request_uri = format!(
            "/accounts/{account_id}/users",
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
        let request_uri = "/accounts";

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

    /// <p>Creates a new attendee for an active Amazon Chime SDK meeting. For more information about the Amazon Chime SDK, see <a href="https://docs.aws.amazon.com/chime/latest/dg/meetings-sdk.html">Using the Amazon Chime SDK</a> in the <i>Amazon Chime Developer Guide</i>.</p>
    fn create_attendee(
        &self,
        input: CreateAttendeeRequest,
    ) -> RusotoFuture<CreateAttendeeResponse, CreateAttendeeError> {
        let request_uri = format!(
            "/meetings/{meeting_id}/attendees",
            meeting_id = input.meeting_id
        );

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateAttendeeResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateAttendeeError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a bot for an Amazon Chime Enterprise account.</p>
    fn create_bot(
        &self,
        input: CreateBotRequest,
    ) -> RusotoFuture<CreateBotResponse, CreateBotError> {
        let request_uri = format!("/accounts/{account_id}/bots", account_id = input.account_id);

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
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

    /// <p>Creates a new Amazon Chime SDK meeting in the specified media Region with no initial attendees. For more information about the Amazon Chime SDK, see <a href="https://docs.aws.amazon.com/chime/latest/dg/meetings-sdk.html">Using the Amazon Chime SDK</a> in the <i>Amazon Chime Developer Guide</i>.</p>
    fn create_meeting(
        &self,
        input: CreateMeetingRequest,
    ) -> RusotoFuture<CreateMeetingResponse, CreateMeetingError> {
        let request_uri = "/meetings";

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateMeetingResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateMeetingError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates an order for phone numbers to be provisioned. Choose from Amazon Chime Business Calling and Amazon Chime Voice Connector product types. For toll-free numbers, you must use the Amazon Chime Voice Connector product type.</p>
    fn create_phone_number_order(
        &self,
        input: CreatePhoneNumberOrderRequest,
    ) -> RusotoFuture<CreatePhoneNumberOrderResponse, CreatePhoneNumberOrderError> {
        let request_uri = "/phone-number-orders";

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
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

    /// <p>Creates a chat room for the specified Amazon Chime account.</p>
    fn create_room(
        &self,
        input: CreateRoomRequest,
    ) -> RusotoFuture<CreateRoomResponse, CreateRoomError> {
        let request_uri = format!(
            "/accounts/{account_id}/rooms",
            account_id = input.account_id
        );

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateRoomResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateRoomError::from_response(response))),
                )
            }
        })
    }

    /// <p>Adds a member to a chat room. A member can be either a user or a bot. The member role designates whether the member is a chat room administrator or a general chat room member.</p>
    fn create_room_membership(
        &self,
        input: CreateRoomMembershipRequest,
    ) -> RusotoFuture<CreateRoomMembershipResponse, CreateRoomMembershipError> {
        let request_uri = format!(
            "/accounts/{account_id}/rooms/{room_id}/memberships",
            account_id = input.account_id,
            room_id = input.room_id
        );

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateRoomMembershipResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateRoomMembershipError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Creates an Amazon Chime Voice Connector under the administrator's AWS account. You can choose to create an Amazon Chime Voice Connector in a specific AWS Region.</p> <p>Enabling <a>CreateVoiceConnectorRequest$RequireEncryption</a> configures your Amazon Chime Voice Connector to use TLS transport for SIP signaling and Secure RTP (SRTP) for media. Inbound calls use TLS transport, and unencrypted outbound calls are blocked.</p>
    fn create_voice_connector(
        &self,
        input: CreateVoiceConnectorRequest,
    ) -> RusotoFuture<CreateVoiceConnectorResponse, CreateVoiceConnectorError> {
        let request_uri = "/voice-connectors";

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
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

    /// <p>Creates an Amazon Chime Voice Connector group under the administrator's AWS account. You can associate up to three existing Amazon Chime Voice Connectors with the Amazon Chime Voice Connector group by including <code>VoiceConnectorItems</code> in the request.</p> <p>You can include Amazon Chime Voice Connectors from different AWS Regions in your group. This creates a fault tolerant mechanism for fallback in case of availability events.</p>
    fn create_voice_connector_group(
        &self,
        input: CreateVoiceConnectorGroupRequest,
    ) -> RusotoFuture<CreateVoiceConnectorGroupResponse, CreateVoiceConnectorGroupError> {
        let request_uri = "/voice-connector-groups";

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateVoiceConnectorGroupResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateVoiceConnectorGroupError::from_response(response))
                }))
            }
        })
    }

    /// <p>Deletes the specified Amazon Chime account. You must suspend all users before deleting a <code>Team</code> account. You can use the <a>BatchSuspendUser</a> action to do so.</p> <p>For <code>EnterpriseLWA</code> and <code>EnterpriseAD</code> accounts, you must release the claimed domains for your Amazon Chime account before deletion. As soon as you release the domain, all users under that account are suspended.</p> <p>Deleted accounts appear in your <code>Disabled</code> accounts list for 90 days. To restore a deleted account from your <code>Disabled</code> accounts list, you must contact AWS Support.</p> <p>After 90 days, deleted accounts are permanently removed from your <code>Disabled</code> accounts list.</p>
    fn delete_account(
        &self,
        input: DeleteAccountRequest,
    ) -> RusotoFuture<DeleteAccountResponse, DeleteAccountError> {
        let request_uri = format!("/accounts/{account_id}", account_id = input.account_id);

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

    /// <p>Deletes an attendee from the specified Amazon Chime SDK meeting and deletes their <code>JoinToken</code>. Attendees are automatically deleted when a Amazon Chime SDK meeting is deleted. For more information about the Amazon Chime SDK, see <a href="https://docs.aws.amazon.com/chime/latest/dg/meetings-sdk.html">Using the Amazon Chime SDK</a> in the <i>Amazon Chime Developer Guide</i>.</p>
    fn delete_attendee(
        &self,
        input: DeleteAttendeeRequest,
    ) -> RusotoFuture<(), DeleteAttendeeError> {
        let request_uri = format!(
            "/meetings/{meeting_id}/attendees/{attendee_id}",
            attendee_id = input.attendee_id,
            meeting_id = input.meeting_id
        );

        let mut request = SignedRequest::new("DELETE", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteAttendeeError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the events configuration that allows a bot to receive outgoing events.</p>
    fn delete_events_configuration(
        &self,
        input: DeleteEventsConfigurationRequest,
    ) -> RusotoFuture<(), DeleteEventsConfigurationError> {
        let request_uri = format!(
            "/accounts/{account_id}/bots/{bot_id}/events-configuration",
            account_id = input.account_id,
            bot_id = input.bot_id
        );

        let mut request = SignedRequest::new("DELETE", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteEventsConfigurationError::from_response(response))
                }))
            }
        })
    }

    /// <p>Deletes the specified Amazon Chime SDK meeting. When a meeting is deleted, its attendees are also deleted and clients can no longer join it. For more information about the Amazon Chime SDK, see <a href="https://docs.aws.amazon.com/chime/latest/dg/meetings-sdk.html">Using the Amazon Chime SDK</a> in the <i>Amazon Chime Developer Guide</i>.</p>
    fn delete_meeting(&self, input: DeleteMeetingRequest) -> RusotoFuture<(), DeleteMeetingError> {
        let request_uri = format!("/meetings/{meeting_id}", meeting_id = input.meeting_id);

        let mut request = SignedRequest::new("DELETE", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteMeetingError::from_response(response))),
                )
            }
        })
    }

    /// <p>Moves the specified phone number into the <b>Deletion queue</b>. A phone number must be disassociated from any users or Amazon Chime Voice Connectors before it can be deleted.</p> <p>Deleted phone numbers remain in the <b>Deletion queue</b> for 7 days before they are deleted permanently.</p>
    fn delete_phone_number(
        &self,
        input: DeletePhoneNumberRequest,
    ) -> RusotoFuture<(), DeletePhoneNumberError> {
        let request_uri = format!(
            "/phone-numbers/{phone_number_id}",
            phone_number_id = input.phone_number_id
        );

        let mut request = SignedRequest::new("DELETE", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

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

    /// <p>Deletes a chat room.</p>
    fn delete_room(&self, input: DeleteRoomRequest) -> RusotoFuture<(), DeleteRoomError> {
        let request_uri = format!(
            "/accounts/{account_id}/rooms/{room_id}",
            account_id = input.account_id,
            room_id = input.room_id
        );

        let mut request = SignedRequest::new("DELETE", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteRoomError::from_response(response))),
                )
            }
        })
    }

    /// <p>Removes a member from a chat room.</p>
    fn delete_room_membership(
        &self,
        input: DeleteRoomMembershipRequest,
    ) -> RusotoFuture<(), DeleteRoomMembershipError> {
        let request_uri = format!(
            "/accounts/{account_id}/rooms/{room_id}/memberships/{member_id}",
            account_id = input.account_id,
            member_id = input.member_id,
            room_id = input.room_id
        );

        let mut request = SignedRequest::new("DELETE", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteRoomMembershipError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes the specified Amazon Chime Voice Connector. Any phone numbers associated with the Amazon Chime Voice Connector must be disassociated from it before it can be deleted.</p>
    fn delete_voice_connector(
        &self,
        input: DeleteVoiceConnectorRequest,
    ) -> RusotoFuture<(), DeleteVoiceConnectorError> {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}",
            voice_connector_id = input.voice_connector_id
        );

        let mut request = SignedRequest::new("DELETE", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

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

    /// <p>Deletes the specified Amazon Chime Voice Connector group. Any <code>VoiceConnectorItems</code> and phone numbers associated with the group must be removed before it can be deleted.</p>
    fn delete_voice_connector_group(
        &self,
        input: DeleteVoiceConnectorGroupRequest,
    ) -> RusotoFuture<(), DeleteVoiceConnectorGroupError> {
        let request_uri = format!(
            "/voice-connector-groups/{voice_connector_group_id}",
            voice_connector_group_id = input.voice_connector_group_id
        );

        let mut request = SignedRequest::new("DELETE", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteVoiceConnectorGroupError::from_response(response))
                }))
            }
        })
    }

    /// <p>Deletes the origination settings for the specified Amazon Chime Voice Connector.</p>
    fn delete_voice_connector_origination(
        &self,
        input: DeleteVoiceConnectorOriginationRequest,
    ) -> RusotoFuture<(), DeleteVoiceConnectorOriginationError> {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}/origination",
            voice_connector_id = input.voice_connector_id
        );

        let mut request = SignedRequest::new("DELETE", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

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

    /// <p>Deletes the streaming configuration for the specified Amazon Chime Voice Connector.</p>
    fn delete_voice_connector_streaming_configuration(
        &self,
        input: DeleteVoiceConnectorStreamingConfigurationRequest,
    ) -> RusotoFuture<(), DeleteVoiceConnectorStreamingConfigurationError> {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}/streaming-configuration",
            voice_connector_id = input.voice_connector_id
        );

        let mut request = SignedRequest::new("DELETE", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteVoiceConnectorStreamingConfigurationError::from_response(response))
                }))
            }
        })
    }

    /// <p>Deletes the termination settings for the specified Amazon Chime Voice Connector.</p>
    fn delete_voice_connector_termination(
        &self,
        input: DeleteVoiceConnectorTerminationRequest,
    ) -> RusotoFuture<(), DeleteVoiceConnectorTerminationError> {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}/termination",
            voice_connector_id = input.voice_connector_id
        );

        let mut request = SignedRequest::new("DELETE", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

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

    /// <p>Deletes the specified SIP credentials used by your equipment to authenticate during call termination.</p>
    fn delete_voice_connector_termination_credentials(
        &self,
        input: DeleteVoiceConnectorTerminationCredentialsRequest,
    ) -> RusotoFuture<(), DeleteVoiceConnectorTerminationCredentialsError> {
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteVoiceConnectorTerminationCredentialsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Disassociates the primary provisioned phone number from the specified Amazon Chime user.</p>
    fn disassociate_phone_number_from_user(
        &self,
        input: DisassociatePhoneNumberFromUserRequest,
    ) -> RusotoFuture<DisassociatePhoneNumberFromUserResponse, DisassociatePhoneNumberFromUserError>
    {
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

        self.client.sign_and_dispatch(request, |response| {
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

    /// <p>Disassociates the specified phone numbers from the specified Amazon Chime Voice Connector.</p>
    fn disassociate_phone_numbers_from_voice_connector(
        &self,
        input: DisassociatePhoneNumbersFromVoiceConnectorRequest,
    ) -> RusotoFuture<
        DisassociatePhoneNumbersFromVoiceConnectorResponse,
        DisassociatePhoneNumbersFromVoiceConnectorError,
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

        self.client.sign_and_dispatch(request, |response| {
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

    /// <p>Disassociates the specified phone numbers from the specified Amazon Chime Voice Connector group.</p>
    fn disassociate_phone_numbers_from_voice_connector_group(
        &self,
        input: DisassociatePhoneNumbersFromVoiceConnectorGroupRequest,
    ) -> RusotoFuture<
        DisassociatePhoneNumbersFromVoiceConnectorGroupResponse,
        DisassociatePhoneNumbersFromVoiceConnectorGroupError,
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DisassociatePhoneNumbersFromVoiceConnectorGroupResponse, _>(
                    )?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(
                        DisassociatePhoneNumbersFromVoiceConnectorGroupError::from_response(
                            response,
                        ),
                    )
                }))
            }
        })
    }

    /// <p>Retrieves details for the specified Amazon Chime account, such as account type and supported licenses.</p>
    fn get_account(
        &self,
        input: GetAccountRequest,
    ) -> RusotoFuture<GetAccountResponse, GetAccountError> {
        let request_uri = format!("/accounts/{account_id}", account_id = input.account_id);

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
            "/accounts/{account_id}/settings",
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

    /// <p>Gets the Amazon Chime SDK attendee details for a specified meeting ID and attendee ID. For more information about the Amazon Chime SDK, see <a href="https://docs.aws.amazon.com/chime/latest/dg/meetings-sdk.html">Using the Amazon Chime SDK</a> in the <i>Amazon Chime Developer Guide</i>.</p>
    fn get_attendee(
        &self,
        input: GetAttendeeRequest,
    ) -> RusotoFuture<GetAttendeeResponse, GetAttendeeError> {
        let request_uri = format!(
            "/meetings/{meeting_id}/attendees/{attendee_id}",
            attendee_id = input.attendee_id,
            meeting_id = input.meeting_id
        );

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetAttendeeResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetAttendeeError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves details for the specified bot, such as bot email address, bot type, status, and display name.</p>
    fn get_bot(&self, input: GetBotRequest) -> RusotoFuture<GetBotResponse, GetBotError> {
        let request_uri = format!(
            "/accounts/{account_id}/bots/{bot_id}",
            account_id = input.account_id,
            bot_id = input.bot_id
        );

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
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

    /// <p>Gets details for an events configuration that allows a bot to receive outgoing events, such as an HTTPS endpoint or Lambda function ARN. </p>
    fn get_events_configuration(
        &self,
        input: GetEventsConfigurationRequest,
    ) -> RusotoFuture<GetEventsConfigurationResponse, GetEventsConfigurationError> {
        let request_uri = format!(
            "/accounts/{account_id}/bots/{bot_id}/events-configuration",
            account_id = input.account_id,
            bot_id = input.bot_id
        );

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
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

    /// <p>Retrieves global settings for the administrator's AWS account, such as Amazon Chime Business Calling and Amazon Chime Voice Connector settings.</p>
    fn get_global_settings(
        &self,
    ) -> RusotoFuture<GetGlobalSettingsResponse, GetGlobalSettingsError> {
        let request_uri = "/settings";

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
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

    /// <p>Gets the Amazon Chime SDK meeting details for the specified meeting ID. For more information about the Amazon Chime SDK, see <a href="https://docs.aws.amazon.com/chime/latest/dg/meetings-sdk.html">Using the Amazon Chime SDK</a> in the <i>Amazon Chime Developer Guide</i>.</p>
    fn get_meeting(
        &self,
        input: GetMeetingRequest,
    ) -> RusotoFuture<GetMeetingResponse, GetMeetingError> {
        let request_uri = format!("/meetings/{meeting_id}", meeting_id = input.meeting_id);

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetMeetingResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetMeetingError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves details for the specified phone number ID, such as associations, capabilities, and product type.</p>
    fn get_phone_number(
        &self,
        input: GetPhoneNumberRequest,
    ) -> RusotoFuture<GetPhoneNumberResponse, GetPhoneNumberError> {
        let request_uri = format!(
            "/phone-numbers/{phone_number_id}",
            phone_number_id = input.phone_number_id
        );

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
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

    /// <p>Retrieves details for the specified phone number order, such as order creation timestamp, phone numbers in E.164 format, product type, and order status.</p>
    fn get_phone_number_order(
        &self,
        input: GetPhoneNumberOrderRequest,
    ) -> RusotoFuture<GetPhoneNumberOrderResponse, GetPhoneNumberOrderError> {
        let request_uri = format!(
            "/phone-number-orders/{phone_number_order_id}",
            phone_number_order_id = input.phone_number_order_id
        );

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
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

    /// <p>Retrieves the phone number settings for the administrator's AWS account, such as the default outbound calling name.</p>
    fn get_phone_number_settings(
        &self,
    ) -> RusotoFuture<GetPhoneNumberSettingsResponse, GetPhoneNumberSettingsError> {
        let request_uri = "/settings/phone-number";

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetPhoneNumberSettingsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetPhoneNumberSettingsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Retrieves room details, such as name.</p>
    fn get_room(&self, input: GetRoomRequest) -> RusotoFuture<GetRoomResponse, GetRoomError> {
        let request_uri = format!(
            "/accounts/{account_id}/rooms/{room_id}",
            account_id = input.account_id,
            room_id = input.room_id
        );

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetRoomResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetRoomError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves details for the specified user ID, such as primary email address, license type, and personal meeting PIN.</p> <p>To retrieve user details with an email address instead of a user ID, use the <a>ListUsers</a> action, and then filter by email address.</p>
    fn get_user(&self, input: GetUserRequest) -> RusotoFuture<GetUserResponse, GetUserError> {
        let request_uri = format!(
            "/accounts/{account_id}/users/{user_id}",
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

    /// <p>Retrieves settings for the specified user ID, such as any associated phone number settings.</p>
    fn get_user_settings(
        &self,
        input: GetUserSettingsRequest,
    ) -> RusotoFuture<GetUserSettingsResponse, GetUserSettingsError> {
        let request_uri = format!(
            "/accounts/{account_id}/users/{user_id}/settings",
            account_id = input.account_id,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
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

    /// <p>Retrieves details for the specified Amazon Chime Voice Connector, such as timestamps, name, outbound host, and encryption requirements.</p>
    fn get_voice_connector(
        &self,
        input: GetVoiceConnectorRequest,
    ) -> RusotoFuture<GetVoiceConnectorResponse, GetVoiceConnectorError> {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}",
            voice_connector_id = input.voice_connector_id
        );

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
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

    /// <p>Retrieves details for the specified Amazon Chime Voice Connector group, such as timestamps, name, and associated <code>VoiceConnectorItems</code>.</p>
    fn get_voice_connector_group(
        &self,
        input: GetVoiceConnectorGroupRequest,
    ) -> RusotoFuture<GetVoiceConnectorGroupResponse, GetVoiceConnectorGroupError> {
        let request_uri = format!(
            "/voice-connector-groups/{voice_connector_group_id}",
            voice_connector_group_id = input.voice_connector_group_id
        );

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetVoiceConnectorGroupResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetVoiceConnectorGroupError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Retrieves the logging configuration details for the specified Amazon Chime Voice Connector. Shows whether SIP message logs are enabled for sending to Amazon CloudWatch Logs.</p>
    fn get_voice_connector_logging_configuration(
        &self,
        input: GetVoiceConnectorLoggingConfigurationRequest,
    ) -> RusotoFuture<
        GetVoiceConnectorLoggingConfigurationResponse,
        GetVoiceConnectorLoggingConfigurationError,
    > {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}/logging-configuration",
            voice_connector_id = input.voice_connector_id
        );

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetVoiceConnectorLoggingConfigurationResponse, _>(
                    )?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetVoiceConnectorLoggingConfigurationError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Retrieves origination setting details for the specified Amazon Chime Voice Connector.</p>
    fn get_voice_connector_origination(
        &self,
        input: GetVoiceConnectorOriginationRequest,
    ) -> RusotoFuture<GetVoiceConnectorOriginationResponse, GetVoiceConnectorOriginationError> {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}/origination",
            voice_connector_id = input.voice_connector_id
        );

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
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

    /// <p>Retrieves the streaming configuration details for the specified Amazon Chime Voice Connector. Shows whether media streaming is enabled for sending to Amazon Kinesis. It also shows the retention period, in hours, for the Amazon Kinesis data.</p>
    fn get_voice_connector_streaming_configuration(
        &self,
        input: GetVoiceConnectorStreamingConfigurationRequest,
    ) -> RusotoFuture<
        GetVoiceConnectorStreamingConfigurationResponse,
        GetVoiceConnectorStreamingConfigurationError,
    > {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}/streaming-configuration",
            voice_connector_id = input.voice_connector_id
        );

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetVoiceConnectorStreamingConfigurationResponse, _>(
                    )?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetVoiceConnectorStreamingConfigurationError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Retrieves termination setting details for the specified Amazon Chime Voice Connector.</p>
    fn get_voice_connector_termination(
        &self,
        input: GetVoiceConnectorTerminationRequest,
    ) -> RusotoFuture<GetVoiceConnectorTerminationResponse, GetVoiceConnectorTerminationError> {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}/termination",
            voice_connector_id = input.voice_connector_id
        );

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
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

    /// <p>Retrieves information about the last time a SIP <code>OPTIONS</code> ping was received from your SIP infrastructure for the specified Amazon Chime Voice Connector.</p>
    fn get_voice_connector_termination_health(
        &self,
        input: GetVoiceConnectorTerminationHealthRequest,
    ) -> RusotoFuture<
        GetVoiceConnectorTerminationHealthResponse,
        GetVoiceConnectorTerminationHealthError,
    > {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}/termination/health",
            voice_connector_id = input.voice_connector_id
        );

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
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

    /// <p>Sends email to a maximum of 50 users, inviting them to the specified Amazon Chime <code>Team</code> account. Only <code>Team</code> account types are currently supported for this action. </p>
    fn invite_users(
        &self,
        input: InviteUsersRequest,
    ) -> RusotoFuture<InviteUsersResponse, InviteUsersError> {
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

    /// <p>Lists the attendees for the specified Amazon Chime SDK meeting. For more information about the Amazon Chime SDK, see <a href="https://docs.aws.amazon.com/chime/latest/dg/meetings-sdk.html">Using the Amazon Chime SDK</a> in the <i>Amazon Chime Developer Guide</i>.</p>
    fn list_attendees(
        &self,
        input: ListAttendeesRequest,
    ) -> RusotoFuture<ListAttendeesResponse, ListAttendeesError> {
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListAttendeesResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListAttendeesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists the bots associated with the administrator's Amazon Chime Enterprise account ID.</p>
    fn list_bots(&self, input: ListBotsRequest) -> RusotoFuture<ListBotsResponse, ListBotsError> {
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

        self.client.sign_and_dispatch(request, |response| {
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

    /// <p>Lists up to 100 active Amazon Chime SDK meetings. For more information about the Amazon Chime SDK, see <a href="https://docs.aws.amazon.com/chime/latest/dg/meetings-sdk.html">Using the Amazon Chime SDK</a> in the <i>Amazon Chime Developer Guide</i>.</p>
    fn list_meetings(
        &self,
        input: ListMeetingsRequest,
    ) -> RusotoFuture<ListMeetingsResponse, ListMeetingsError> {
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListMeetingsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListMeetingsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists the phone number orders for the administrator's Amazon Chime account.</p>
    fn list_phone_number_orders(
        &self,
        input: ListPhoneNumberOrdersRequest,
    ) -> RusotoFuture<ListPhoneNumberOrdersResponse, ListPhoneNumberOrdersError> {
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

        self.client.sign_and_dispatch(request, |response| {
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

    /// <p>Lists the phone numbers for the specified Amazon Chime account, Amazon Chime user, Amazon Chime Voice Connector, or Amazon Chime Voice Connector group.</p>
    fn list_phone_numbers(
        &self,
        input: ListPhoneNumbersRequest,
    ) -> RusotoFuture<ListPhoneNumbersResponse, ListPhoneNumbersError> {
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

        self.client.sign_and_dispatch(request, |response| {
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

    /// <p>Lists the membership details for the specified room, such as member IDs, member email addresses, and member names.</p>
    fn list_room_memberships(
        &self,
        input: ListRoomMembershipsRequest,
    ) -> RusotoFuture<ListRoomMembershipsResponse, ListRoomMembershipsError> {
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListRoomMembershipsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListRoomMembershipsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Lists the room details for the specified Amazon Chime account. Optionally, filter the results by a member ID (user ID or bot ID) to see a list of rooms that the member belongs to.</p>
    fn list_rooms(
        &self,
        input: ListRoomsRequest,
    ) -> RusotoFuture<ListRoomsResponse, ListRoomsError> {
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListRoomsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListRoomsError::from_response(response))),
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

    /// <p>Lists the Amazon Chime Voice Connector groups for the administrator's AWS account.</p>
    fn list_voice_connector_groups(
        &self,
        input: ListVoiceConnectorGroupsRequest,
    ) -> RusotoFuture<ListVoiceConnectorGroupsResponse, ListVoiceConnectorGroupsError> {
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListVoiceConnectorGroupsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListVoiceConnectorGroupsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Lists the SIP credentials for the specified Amazon Chime Voice Connector.</p>
    fn list_voice_connector_termination_credentials(
        &self,
        input: ListVoiceConnectorTerminationCredentialsRequest,
    ) -> RusotoFuture<
        ListVoiceConnectorTerminationCredentialsResponse,
        ListVoiceConnectorTerminationCredentialsError,
    > {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}/termination/credentials",
            voice_connector_id = input.voice_connector_id
        );

        let mut request = SignedRequest::new("GET", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
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

    /// <p>Lists the Amazon Chime Voice Connectors for the administrator's AWS account.</p>
    fn list_voice_connectors(
        &self,
        input: ListVoiceConnectorsRequest,
    ) -> RusotoFuture<ListVoiceConnectorsResponse, ListVoiceConnectorsError> {
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

        self.client.sign_and_dispatch(request, |response| {
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

    /// <p>Logs out the specified user from all of the devices they are currently logged into.</p>
    fn logout_user(
        &self,
        input: LogoutUserRequest,
    ) -> RusotoFuture<LogoutUserResponse, LogoutUserError> {
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

    /// <p>Creates an events configuration that allows a bot to receive outgoing events sent by Amazon Chime. Choose either an HTTPS endpoint or a Lambda function ARN. For more information, see <a>Bot</a>.</p>
    fn put_events_configuration(
        &self,
        input: PutEventsConfigurationRequest,
    ) -> RusotoFuture<PutEventsConfigurationResponse, PutEventsConfigurationError> {
        let request_uri = format!(
            "/accounts/{account_id}/bots/{bot_id}/events-configuration",
            account_id = input.account_id,
            bot_id = input.bot_id
        );

        let mut request = SignedRequest::new("PUT", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
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

    /// <p>Adds a logging configuration for the specified Amazon Chime Voice Connector. The logging configuration specifies whether SIP message logs are enabled for sending to Amazon CloudWatch Logs.</p>
    fn put_voice_connector_logging_configuration(
        &self,
        input: PutVoiceConnectorLoggingConfigurationRequest,
    ) -> RusotoFuture<
        PutVoiceConnectorLoggingConfigurationResponse,
        PutVoiceConnectorLoggingConfigurationError,
    > {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}/logging-configuration",
            voice_connector_id = input.voice_connector_id
        );

        let mut request = SignedRequest::new("PUT", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<PutVoiceConnectorLoggingConfigurationResponse, _>(
                    )?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutVoiceConnectorLoggingConfigurationError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Adds origination settings for the specified Amazon Chime Voice Connector.</p>
    fn put_voice_connector_origination(
        &self,
        input: PutVoiceConnectorOriginationRequest,
    ) -> RusotoFuture<PutVoiceConnectorOriginationResponse, PutVoiceConnectorOriginationError> {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}/origination",
            voice_connector_id = input.voice_connector_id
        );

        let mut request = SignedRequest::new("PUT", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
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

    /// <p>Adds a streaming configuration for the specified Amazon Chime Voice Connector. The streaming configuration specifies whether media streaming is enabled for sending to Amazon Kinesis. It also sets the retention period, in hours, for the Amazon Kinesis data.</p>
    fn put_voice_connector_streaming_configuration(
        &self,
        input: PutVoiceConnectorStreamingConfigurationRequest,
    ) -> RusotoFuture<
        PutVoiceConnectorStreamingConfigurationResponse,
        PutVoiceConnectorStreamingConfigurationError,
    > {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}/streaming-configuration",
            voice_connector_id = input.voice_connector_id
        );

        let mut request = SignedRequest::new("PUT", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<PutVoiceConnectorStreamingConfigurationResponse, _>(
                    )?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutVoiceConnectorStreamingConfigurationError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Adds termination settings for the specified Amazon Chime Voice Connector.</p>
    fn put_voice_connector_termination(
        &self,
        input: PutVoiceConnectorTerminationRequest,
    ) -> RusotoFuture<PutVoiceConnectorTerminationResponse, PutVoiceConnectorTerminationError> {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}/termination",
            voice_connector_id = input.voice_connector_id
        );

        let mut request = SignedRequest::new("PUT", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
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

    /// <p>Adds termination SIP credentials for the specified Amazon Chime Voice Connector.</p>
    fn put_voice_connector_termination_credentials(
        &self,
        input: PutVoiceConnectorTerminationCredentialsRequest,
    ) -> RusotoFuture<(), PutVoiceConnectorTerminationCredentialsError> {
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

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

    /// <p>Regenerates the security token for a bot.</p>
    fn regenerate_security_token(
        &self,
        input: RegenerateSecurityTokenRequest,
    ) -> RusotoFuture<RegenerateSecurityTokenResponse, RegenerateSecurityTokenError> {
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

        self.client.sign_and_dispatch(request, |response| {
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

    /// <p>Resets the personal meeting PIN for the specified user on an Amazon Chime account. Returns the <a>User</a> object with the updated personal meeting PIN.</p>
    fn reset_personal_pin(
        &self,
        input: ResetPersonalPINRequest,
    ) -> RusotoFuture<ResetPersonalPINResponse, ResetPersonalPINError> {
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

    /// <p>Moves a phone number from the <b>Deletion queue</b> back into the phone number <b>Inventory</b>.</p>
    fn restore_phone_number(
        &self,
        input: RestorePhoneNumberRequest,
    ) -> RusotoFuture<RestorePhoneNumberResponse, RestorePhoneNumberError> {
        let request_uri = format!(
            "/phone-numbers/{phone_number_id}",
            phone_number_id = input.phone_number_id
        );

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("operation", "restore");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
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

    /// <p>Searches phone numbers that can be ordered.</p>
    fn search_available_phone_numbers(
        &self,
        input: SearchAvailablePhoneNumbersRequest,
    ) -> RusotoFuture<SearchAvailablePhoneNumbersResponse, SearchAvailablePhoneNumbersError> {
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

        self.client.sign_and_dispatch(request, |response| {
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

    /// <p>Updates account details for the specified Amazon Chime account. Currently, only account name updates are supported for this action.</p>
    fn update_account(
        &self,
        input: UpdateAccountRequest,
    ) -> RusotoFuture<UpdateAccountResponse, UpdateAccountError> {
        let request_uri = format!("/accounts/{account_id}", account_id = input.account_id);

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
            "/accounts/{account_id}/settings",
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

    /// <p>Updates the status of the specified bot, such as starting or stopping the bot from running in your Amazon Chime Enterprise account.</p>
    fn update_bot(
        &self,
        input: UpdateBotRequest,
    ) -> RusotoFuture<UpdateBotResponse, UpdateBotError> {
        let request_uri = format!(
            "/accounts/{account_id}/bots/{bot_id}",
            account_id = input.account_id,
            bot_id = input.bot_id
        );

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
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

    /// <p>Updates global settings for the administrator's AWS account, such as Amazon Chime Business Calling and Amazon Chime Voice Connector settings.</p>
    fn update_global_settings(
        &self,
        input: UpdateGlobalSettingsRequest,
    ) -> RusotoFuture<(), UpdateGlobalSettingsError> {
        let request_uri = "/settings";

        let mut request = SignedRequest::new("PUT", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

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

    /// <p>Updates phone number details, such as product type or calling name, for the specified phone number ID. You can update one phone number detail at a time. For example, you can update either the product type or the calling name in one action.</p> <p>For toll-free numbers, you must use the Amazon Chime Voice Connector product type.</p> <p>Updates to outbound calling names can take up to 72 hours to complete. Pending updates to outbound calling names must be complete before you can request another update.</p>
    fn update_phone_number(
        &self,
        input: UpdatePhoneNumberRequest,
    ) -> RusotoFuture<UpdatePhoneNumberResponse, UpdatePhoneNumberError> {
        let request_uri = format!(
            "/phone-numbers/{phone_number_id}",
            phone_number_id = input.phone_number_id
        );

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
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

    /// <p>Updates the phone number settings for the administrator's AWS account, such as the default outbound calling name. You can update the default outbound calling name once every seven days. Outbound calling names can take up to 72 hours to update.</p>
    fn update_phone_number_settings(
        &self,
        input: UpdatePhoneNumberSettingsRequest,
    ) -> RusotoFuture<(), UpdatePhoneNumberSettingsError> {
        let request_uri = "/settings/phone-number";

        let mut request = SignedRequest::new("PUT", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdatePhoneNumberSettingsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Updates room details, such as the room name.</p>
    fn update_room(
        &self,
        input: UpdateRoomRequest,
    ) -> RusotoFuture<UpdateRoomResponse, UpdateRoomError> {
        let request_uri = format!(
            "/accounts/{account_id}/rooms/{room_id}",
            account_id = input.account_id,
            room_id = input.room_id
        );

        let mut request = SignedRequest::new("POST", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateRoomResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateRoomError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates room membership details, such as member role. The member role designates whether the member is a chat room administrator or a general chat room member. Member role can only be updated for user IDs.</p>
    fn update_room_membership(
        &self,
        input: UpdateRoomMembershipRequest,
    ) -> RusotoFuture<UpdateRoomMembershipResponse, UpdateRoomMembershipError> {
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateRoomMembershipResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateRoomMembershipError::from_response(response))
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
            "/accounts/{account_id}/users/{user_id}",
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

    /// <p>Updates the settings for the specified user, such as phone number settings.</p>
    fn update_user_settings(
        &self,
        input: UpdateUserSettingsRequest,
    ) -> RusotoFuture<(), UpdateUserSettingsError> {
        let request_uri = format!(
            "/accounts/{account_id}/users/{user_id}/settings",
            account_id = input.account_id,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("PUT", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

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

    /// <p>Updates details for the specified Amazon Chime Voice Connector.</p>
    fn update_voice_connector(
        &self,
        input: UpdateVoiceConnectorRequest,
    ) -> RusotoFuture<UpdateVoiceConnectorResponse, UpdateVoiceConnectorError> {
        let request_uri = format!(
            "/voice-connectors/{voice_connector_id}",
            voice_connector_id = input.voice_connector_id
        );

        let mut request = SignedRequest::new("PUT", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
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

    /// <p>Updates details for the specified Amazon Chime Voice Connector group, such as the name and Amazon Chime Voice Connector priority ranking.</p>
    fn update_voice_connector_group(
        &self,
        input: UpdateVoiceConnectorGroupRequest,
    ) -> RusotoFuture<UpdateVoiceConnectorGroupResponse, UpdateVoiceConnectorGroupError> {
        let request_uri = format!(
            "/voice-connector-groups/{voice_connector_group_id}",
            voice_connector_group_id = input.voice_connector_group_id
        );

        let mut request = SignedRequest::new("PUT", "chime", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateVoiceConnectorGroupResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateVoiceConnectorGroupError::from_response(response))
                }))
            }
        })
    }
}
