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
use std::io;

#[allow(warnings)]
use futures::future;
use futures::Future;
use rusoto_core::reactor::{CredentialsProvider, RequestDispatcher};
use rusoto_core::region;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::{ClientInner, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use hyper::StatusCode;
use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_str;
use serde_json::Value as SerdeJsonValue;
/// <p>An address book with attributes.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AddressBook {
    /// <p>The ARN of the address book.</p>
    #[serde(rename = "AddressBookArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_book_arn: Option<String>,
    /// <p>The description of the address book.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the address book.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Information related to an address book.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AddressBookData {
    /// <p>The ARN of the address book.</p>
    #[serde(rename = "AddressBookArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_book_arn: Option<String>,
    /// <p>The description of the address book.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the address book.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssociateContactWithAddressBookRequest {
    /// <p>The ARN of the address book with which to associate the contact.</p>
    #[serde(rename = "AddressBookArn")]
    pub address_book_arn: String,
    /// <p>The ARN of the contact to associate with an address book.</p>
    #[serde(rename = "ContactArn")]
    pub contact_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AssociateContactWithAddressBookResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssociateDeviceWithRoomRequest {
    /// <p>The ARN of the device to associate to a room. Required.</p>
    #[serde(rename = "DeviceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_arn: Option<String>,
    /// <p>The ARN of the room with which to associate the device. Required.</p>
    #[serde(rename = "RoomArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AssociateDeviceWithRoomResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssociateSkillGroupWithRoomRequest {
    /// <p>The ARN of the room with which to associate the skill group. Required.</p>
    #[serde(rename = "RoomArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_arn: Option<String>,
    /// <p>The ARN of the skill group to associate with a room. Required.</p>
    #[serde(rename = "SkillGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_group_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AssociateSkillGroupWithRoomResponse {}

/// <p>A contact with attributes.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Contact {
    /// <p>The ARN of the contact.</p>
    #[serde(rename = "ContactArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_arn: Option<String>,
    /// <p>The name of the contact to display on the AWS management console.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The first name of the contact that is used to call the contact on the device.</p>
    #[serde(rename = "FirstName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// <p>The last name of the contact that is used to call the contact on the device.</p>
    #[serde(rename = "LastName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// <p>The phone number of the contact.</p>
    #[serde(rename = "PhoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}

/// <p>Information related to a contact.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ContactData {
    /// <p>The ARN of the contact.</p>
    #[serde(rename = "ContactArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_arn: Option<String>,
    /// <p>The name of the contact to display on the AWS management console.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The first name of the contact that is used to call the contact on the device.</p>
    #[serde(rename = "FirstName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// <p>The last name of the contact that is used to call the contact on the device.</p>
    #[serde(rename = "LastName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// <p>The phone number of the contact.</p>
    #[serde(rename = "PhoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateAddressBookRequest {
    /// <p>A unique, user-specified identifier for the request that ensures idempotency.</p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The description of the address book.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the address book.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateAddressBookResponse {
    /// <p>The ARN of the newly created address book.</p>
    #[serde(rename = "AddressBookArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_book_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateContactRequest {
    /// <p>A unique, user-specified identifier for this request that ensures idempotency.</p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The name of the contact to display on the AWS management console.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The first name of the contact that is used to call the contact on the device.</p>
    #[serde(rename = "FirstName")]
    pub first_name: String,
    /// <p>The last name of the contact that is used to call the contact on the device.</p>
    #[serde(rename = "LastName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// <p>The phone number of the contact in E164 format.</p>
    #[serde(rename = "PhoneNumber")]
    pub phone_number: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateContactResponse {
    /// <p>The ARN of the newly created address book.</p>
    #[serde(rename = "ContactArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateProfileRequest {
    /// <p>The valid address for the room.</p>
    #[serde(rename = "Address")]
    pub address: String,
    /// <p>The user-specified token that is used during the creation of a profile.</p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The distance unit to be used by devices in the profile.</p>
    #[serde(rename = "DistanceUnit")]
    pub distance_unit: String,
    /// <p>The maximum volume limit for a room profile.</p>
    #[serde(rename = "MaxVolumeLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_volume_limit: Option<i64>,
    /// <p>Whether PSTN calling is enabled.</p>
    #[serde(rename = "PSTNEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pstn_enabled: Option<bool>,
    /// <p>The name of a room profile.</p>
    #[serde(rename = "ProfileName")]
    pub profile_name: String,
    /// <p>Whether room profile setup is enabled.</p>
    #[serde(rename = "SetupModeDisabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_mode_disabled: Option<bool>,
    /// <p>The temperature unit to be used by devices in the profile.</p>
    #[serde(rename = "TemperatureUnit")]
    pub temperature_unit: String,
    /// <p>The time zone used by a room profile.</p>
    #[serde(rename = "Timezone")]
    pub timezone: String,
    /// <p>A wake word for Alexa, Echo, Amazon, or a computer.</p>
    #[serde(rename = "WakeWord")]
    pub wake_word: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateProfileResponse {
    /// <p>The ARN of the newly created room profile in the response.</p>
    #[serde(rename = "ProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateRoomRequest {
    /// <p>A unique, user-specified identifier for this request that ensures idempotency. </p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The description for the room.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The profile ARN for the room.</p>
    #[serde(rename = "ProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_arn: Option<String>,
    /// <p>The calendar ARN for the room.</p>
    #[serde(rename = "ProviderCalendarId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_calendar_id: Option<String>,
    /// <p>The name for the room.</p>
    #[serde(rename = "RoomName")]
    pub room_name: String,
    /// <p>The tags for the room.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateRoomResponse {
    /// <p>The ARN of the newly created room in the response.</p>
    #[serde(rename = "RoomArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateSkillGroupRequest {
    /// <p>A unique, user-specified identifier for this request that ensures idempotency. </p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The description for the skill group.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name for the skill group.</p>
    #[serde(rename = "SkillGroupName")]
    pub skill_group_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateSkillGroupResponse {
    /// <p>The ARN of the newly created skill group in the response.</p>
    #[serde(rename = "SkillGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_group_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateUserRequest {
    /// <p>A unique, user-specified identifier for this request that ensures idempotency. </p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The email address for the user.</p>
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// <p>The first name for the user.</p>
    #[serde(rename = "FirstName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// <p>The last name for the user.</p>
    #[serde(rename = "LastName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// <p>The tags for the user.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The ARN for the user.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateUserResponse {
    /// <p>The ARN of the newly created user in the response.</p>
    #[serde(rename = "UserArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteAddressBookRequest {
    /// <p>The ARN of the address book to delete.</p>
    #[serde(rename = "AddressBookArn")]
    pub address_book_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteAddressBookResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteContactRequest {
    /// <p>The ARN of the contact to delete.</p>
    #[serde(rename = "ContactArn")]
    pub contact_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteContactResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteProfileRequest {
    /// <p>The ARN of the room profile to delete. Required.</p>
    #[serde(rename = "ProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteProfileResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteRoomRequest {
    /// <p>The ARN of the room to delete. Required.</p>
    #[serde(rename = "RoomArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteRoomResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteRoomSkillParameterRequest {
    /// <p>The room skill parameter key for which to remove details.</p>
    #[serde(rename = "ParameterKey")]
    pub parameter_key: String,
    /// <p>The ARN of the room from which to remove the room skill parameter details.</p>
    #[serde(rename = "RoomArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_arn: Option<String>,
    /// <p>The ID of the skill from which to remove the room skill parameter details.</p>
    #[serde(rename = "SkillId")]
    pub skill_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteRoomSkillParameterResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteSkillGroupRequest {
    /// <p>The ARN of the skill group to delete. Required.</p>
    #[serde(rename = "SkillGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_group_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteSkillGroupResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteUserRequest {
    /// <p>The ARN of the user's enrollment in the organization. Required.</p>
    #[serde(rename = "EnrollmentId")]
    pub enrollment_id: String,
    /// <p>The ARN of the user to delete in the organization. Required.</p>
    #[serde(rename = "UserArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteUserResponse {}

/// <p>A device with attributes.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Device {
    /// <p>The ARN of a device.</p>
    #[serde(rename = "DeviceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_arn: Option<String>,
    /// <p>The name of a device.</p>
    #[serde(rename = "DeviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    /// <p>The serial number of a device.</p>
    #[serde(rename = "DeviceSerialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_serial_number: Option<String>,
    /// <p>The status of a device. If the status is not READY, check the DeviceStatusInfo for details.</p>
    #[serde(rename = "DeviceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_status: Option<String>,
    /// <p>Detailed information about a device's status.</p>
    #[serde(rename = "DeviceStatusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_status_info: Option<DeviceStatusInfo>,
    /// <p>The type of a device.</p>
    #[serde(rename = "DeviceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    /// <p>The MAC address of a device.</p>
    #[serde(rename = "MacAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    /// <p>The room ARN of a device.</p>
    #[serde(rename = "RoomArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_arn: Option<String>,
    /// <p>The software version of a device.</p>
    #[serde(rename = "SoftwareVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub software_version: Option<String>,
}

/// <p>Device attributes.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeviceData {
    /// <p>The ARN of a device.</p>
    #[serde(rename = "DeviceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_arn: Option<String>,
    /// <p>The name of a device.</p>
    #[serde(rename = "DeviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    /// <p>The serial number of a device.</p>
    #[serde(rename = "DeviceSerialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_serial_number: Option<String>,
    /// <p>The status of a device.</p>
    #[serde(rename = "DeviceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_status: Option<String>,
    /// <p>Detailed information about a device's status.</p>
    #[serde(rename = "DeviceStatusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_status_info: Option<DeviceStatusInfo>,
    /// <p>The type of a device.</p>
    #[serde(rename = "DeviceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    /// <p>The MAC address of a device.</p>
    #[serde(rename = "MacAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    /// <p>The room ARN associated with a device.</p>
    #[serde(rename = "RoomArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_arn: Option<String>,
    /// <p>The name of the room associated with a device.</p>
    #[serde(rename = "RoomName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_name: Option<String>,
    /// <p>The software version of a device.</p>
    #[serde(rename = "SoftwareVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub software_version: Option<String>,
}

/// <p>Details of a device’s status.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeviceStatusDetail {
    /// <p>The device status detail code.</p>
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

/// <p>Detailed information about a device's status.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeviceStatusInfo {
    /// <p>One or more device status detail descriptions.</p>
    #[serde(rename = "DeviceStatusDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_status_details: Option<Vec<DeviceStatusDetail>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisassociateContactFromAddressBookRequest {
    /// <p>The ARN of the address from which to disassociate the contact.</p>
    #[serde(rename = "AddressBookArn")]
    pub address_book_arn: String,
    /// <p>The ARN of the contact to disassociate from an address book.</p>
    #[serde(rename = "ContactArn")]
    pub contact_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DisassociateContactFromAddressBookResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisassociateDeviceFromRoomRequest {
    /// <p>The ARN of the device to disassociate from a room. Required.</p>
    #[serde(rename = "DeviceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DisassociateDeviceFromRoomResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisassociateSkillGroupFromRoomRequest {
    /// <p>The ARN of the room from which the skill group is to be disassociated. Required.</p>
    #[serde(rename = "RoomArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_arn: Option<String>,
    /// <p>The ARN of the skill group to disassociate from a room. Required.</p>
    #[serde(rename = "SkillGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_group_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DisassociateSkillGroupFromRoomResponse {}

/// <p>A filter name and value pair that is used to return a more specific list of results. Filters can be used to match a set of resources by various criteria.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Filter {
    /// <p>The key of a filter.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The values of a filter.</p>
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAddressBookRequest {
    /// <p>The ARN of the address book for which to request details.</p>
    #[serde(rename = "AddressBookArn")]
    pub address_book_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetAddressBookResponse {
    /// <p>The details of the requested address book.</p>
    #[serde(rename = "AddressBook")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_book: Option<AddressBook>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetContactRequest {
    /// <p>The ARN of the contact for which to request details.</p>
    #[serde(rename = "ContactArn")]
    pub contact_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetContactResponse {
    /// <p>The details of the requested contact.</p>
    #[serde(rename = "Contact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDeviceRequest {
    /// <p>The ARN of the device for which to request details. Required.</p>
    #[serde(rename = "DeviceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetDeviceResponse {
    /// <p>The details of the device requested. Required.</p>
    #[serde(rename = "Device")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Device>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetProfileRequest {
    /// <p>The ARN of the room profile for which to request details. Required.</p>
    #[serde(rename = "ProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetProfileResponse {
    /// <p>The details of the room profile requested. Required.</p>
    #[serde(rename = "Profile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<Profile>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetRoomRequest {
    /// <p>The ARN of the room for which to request details. Required.</p>
    #[serde(rename = "RoomArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetRoomResponse {
    /// <p>The details of the room requested.</p>
    #[serde(rename = "Room")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room: Option<Room>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetRoomSkillParameterRequest {
    /// <p>The room skill parameter key for which to get details. Required.</p>
    #[serde(rename = "ParameterKey")]
    pub parameter_key: String,
    /// <p>The ARN of the room from which to get the room skill parameter details. </p>
    #[serde(rename = "RoomArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_arn: Option<String>,
    /// <p>The ARN of the skill from which to get the room skill parameter details. Required.</p>
    #[serde(rename = "SkillId")]
    pub skill_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetRoomSkillParameterResponse {
    /// <p>The details of the room skill parameter requested. Required.</p>
    #[serde(rename = "RoomSkillParameter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_skill_parameter: Option<RoomSkillParameter>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSkillGroupRequest {
    /// <p>The ARN of the skill group for which to get details. Required.</p>
    #[serde(rename = "SkillGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_group_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetSkillGroupResponse {
    /// <p>The details of the skill group requested. Required.</p>
    #[serde(rename = "SkillGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_group: Option<SkillGroup>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListSkillsRequest {
    /// <p>The maximum number of results to include in the response. If more results exist than the specified <code>MaxResults</code> value, a token is included in the response so that the remaining results can be retrieved. Required.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An optional token returned from a prior request. Use this token for pagination of results from this action. If this parameter is specified, the response includes only results beyond the token, up to the value specified by <code>MaxResults</code>. Required.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ARN of the skill group for which to list enabled skills. Required.</p>
    #[serde(rename = "SkillGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_group_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListSkillsResponse {
    /// <p>The token returned to indicate that there is more data available.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of enabled skills requested. Required.</p>
    #[serde(rename = "SkillSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_summaries: Option<Vec<SkillSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTagsRequest {
    /// <p>The ARN of the specific resource for which to list tags. Required.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>The maximum number of results to include in the response. If more results exist than the specified <code>MaxResults</code> value, a token is included in the response so that the remaining results can be retrieved.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An optional token returned from a prior request. Use this token for pagination of results from this action. If this parameter is specified, the response includes only results beyond the token, up to the value specified by <code>MaxResults</code>. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListTagsResponse {
    /// <p>The token returned to indicate that there is more data available.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of tags requested for the specific resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>A room profile with attributes.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Profile {
    /// <p>The address of a room profile.</p>
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// <p>The distance unit of a room profile.</p>
    #[serde(rename = "DistanceUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_unit: Option<String>,
    /// <p>The max volume limit of a room profile.</p>
    #[serde(rename = "MaxVolumeLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_volume_limit: Option<i64>,
    /// <p>The PSTN setting of a room profile.</p>
    #[serde(rename = "PSTNEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pstn_enabled: Option<bool>,
    /// <p>The ARN of a room profile.</p>
    #[serde(rename = "ProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_arn: Option<String>,
    /// <p>The name of a room profile.</p>
    #[serde(rename = "ProfileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_name: Option<String>,
    /// <p>The setup mode of a room profile.</p>
    #[serde(rename = "SetupModeDisabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_mode_disabled: Option<bool>,
    /// <p>The temperature unit of a room profile.</p>
    #[serde(rename = "TemperatureUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature_unit: Option<String>,
    /// <p>The time zone of a room profile.</p>
    #[serde(rename = "Timezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// <p>The wake word of a room profile.</p>
    #[serde(rename = "WakeWord")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wake_word: Option<String>,
}

/// <p>The data of a room profile.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ProfileData {
    /// <p>The address of a room profile.</p>
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// <p>The distance unit of a room profile.</p>
    #[serde(rename = "DistanceUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_unit: Option<String>,
    /// <p>The ARN of a room profile.</p>
    #[serde(rename = "ProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_arn: Option<String>,
    /// <p>The name of a room profile.</p>
    #[serde(rename = "ProfileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_name: Option<String>,
    /// <p>The temperature unit of a room profile.</p>
    #[serde(rename = "TemperatureUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature_unit: Option<String>,
    /// <p>The timezone of a room profile.</p>
    #[serde(rename = "Timezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// <p>The wake word of a room profile.</p>
    #[serde(rename = "WakeWord")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wake_word: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutRoomSkillParameterRequest {
    /// <p>The ARN of the room associated with the room skill parameter. Required.</p>
    #[serde(rename = "RoomArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_arn: Option<String>,
    /// <p>The updated room skill parameter. Required.</p>
    #[serde(rename = "RoomSkillParameter")]
    pub room_skill_parameter: RoomSkillParameter,
    /// <p>The ARN of the skill associated with the room skill parameter. Required.</p>
    #[serde(rename = "SkillId")]
    pub skill_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PutRoomSkillParameterResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ResolveRoomRequest {
    /// <p>The ARN of the skill that was requested. Required.</p>
    #[serde(rename = "SkillId")]
    pub skill_id: String,
    /// <p>The ARN of the user. Required.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ResolveRoomResponse {
    /// <p>The ARN of the room from which the skill request was invoked.</p>
    #[serde(rename = "RoomArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_arn: Option<String>,
    /// <p>The name of the room from which the skill request was invoked.</p>
    #[serde(rename = "RoomName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_name: Option<String>,
    /// <p>Response to get the room profile request. Required.</p>
    #[serde(rename = "RoomSkillParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_skill_parameters: Option<Vec<RoomSkillParameter>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RevokeInvitationRequest {
    /// <p>The ARN of the enrollment invitation to revoke. Required.</p>
    #[serde(rename = "EnrollmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enrollment_id: Option<String>,
    /// <p>The ARN of the user for whom to revoke an enrollment invitation. Required.</p>
    #[serde(rename = "UserArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RevokeInvitationResponse {}

/// <p>A room with attributes.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Room {
    /// <p>The description of a room.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The profile ARN of a room.</p>
    #[serde(rename = "ProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_arn: Option<String>,
    /// <p>The provider calendar ARN of a room.</p>
    #[serde(rename = "ProviderCalendarId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_calendar_id: Option<String>,
    /// <p>The ARN of a room.</p>
    #[serde(rename = "RoomArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_arn: Option<String>,
    /// <p>The name of a room.</p>
    #[serde(rename = "RoomName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_name: Option<String>,
}

/// <p>The data of a room.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RoomData {
    /// <p>The description of a room.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The profile ARN of a room.</p>
    #[serde(rename = "ProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_arn: Option<String>,
    /// <p>The profile name of a room.</p>
    #[serde(rename = "ProfileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_name: Option<String>,
    /// <p>The provider calendar ARN of a room.</p>
    #[serde(rename = "ProviderCalendarId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_calendar_id: Option<String>,
    /// <p>The ARN of a room.</p>
    #[serde(rename = "RoomArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_arn: Option<String>,
    /// <p>The name of a room.</p>
    #[serde(rename = "RoomName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_name: Option<String>,
}

/// <p>A skill parameter associated with a room.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoomSkillParameter {
    /// <p>The parameter key of a room skill parameter. ParameterKey is an enumerated type that only takes “DEFAULT” or “SCOPE” as valid values.</p>
    #[serde(rename = "ParameterKey")]
    pub parameter_key: String,
    /// <p>The parameter value of a room skill parameter.</p>
    #[serde(rename = "ParameterValue")]
    pub parameter_value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SearchAddressBooksRequest {
    /// <p>The filters to use to list a specified set of address books. The supported filter key is AddressBookName.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>The maximum number of results to include in the response. If more results exist than the specified MaxResults value, a token is included in the response so that the remaining results can be retrieved.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An optional token returned from a prior request. Use this token for pagination of results from this action. If this parameter is specified, the response only includes results beyond the token, up to the value specified by MaxResults.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The sort order to use in listing the specified set of address books. The supported sort key is AddressBookName.</p>
    #[serde(rename = "SortCriteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_criteria: Option<Vec<Sort>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SearchAddressBooksResponse {
    /// <p>The address books that meet the specified set of filter criteria, in sort order.</p>
    #[serde(rename = "AddressBooks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_books: Option<Vec<AddressBookData>>,
    /// <p>The token returned to indicate that there is more data available.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The total number of address books returned.</p>
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SearchContactsRequest {
    /// <p>The filters to use to list a specified set of address books. The supported filter keys are DisplayName, FirstName, LastName, and AddressBookArns.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>The maximum number of results to include in the response. If more results exist than the specified MaxResults value, a token is included in the response so that the remaining results can be retrieved.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An optional token returned from a prior request. Use this token for pagination of results from this action. If this parameter is specified, the response only includes results beyond the token, up to the value specified by MaxResults.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The sort order to use in listing the specified set of contacts. The supported sort keys are DisplayName, FirstName, and LastName.</p>
    #[serde(rename = "SortCriteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_criteria: Option<Vec<Sort>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SearchContactsResponse {
    /// <p>The contacts that meet the specified set of filter criteria, in sort order.</p>
    #[serde(rename = "Contacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<Vec<ContactData>>,
    /// <p>The token returned to indicate that there is more data available.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The total number of contacts returned.</p>
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SearchDevicesRequest {
    /// <p>The filters to use to list a specified set of devices. Supported filter keys are DeviceName, DeviceStatus, DeviceStatusDetailCode, RoomName, DeviceType, DeviceSerialNumber, and UnassociatedOnly.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>The maximum number of results to include in the response. If more results exist than the specified <code>MaxResults</code> value, a token is included in the response so that the remaining results can be retrieved.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An optional token returned from a prior request. Use this token for pagination of results from this action. If this parameter is specified, the response includes only results beyond the token, up to the value specified by <code>MaxResults</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The sort order to use in listing the specified set of devices. Supported sort keys are DeviceName, DeviceStatus, RoomName, DeviceType, and DeviceSerialNumber.</p>
    #[serde(rename = "SortCriteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_criteria: Option<Vec<Sort>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SearchDevicesResponse {
    /// <p>The devices that meet the specified set of filter criteria, in sort order.</p>
    #[serde(rename = "Devices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<DeviceData>>,
    /// <p>The token returned to indicate that there is more data available.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The total number of devices returned.</p>
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SearchProfilesRequest {
    /// <p>The filters to use to list a specified set of room profiles. Supported filter keys are ProfileName and Address. Required. </p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>The maximum number of results to include in the response. If more results exist than the specified <code>MaxResults</code> value, a token is included in the response so that the remaining results can be retrieved.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An optional token returned from a prior request. Use this token for pagination of results from this action. If this parameter is specified, the response includes only results beyond the token, up to the value specified by <code>MaxResults</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The sort order to use in listing the specified set of room profiles. Supported sort keys are ProfileName and Address.</p>
    #[serde(rename = "SortCriteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_criteria: Option<Vec<Sort>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SearchProfilesResponse {
    /// <p>The token returned to indicate that there is more data available.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The profiles that meet the specified set of filter criteria, in sort order.</p>
    #[serde(rename = "Profiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Vec<ProfileData>>,
    /// <p>The total number of room profiles returned.</p>
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SearchRoomsRequest {
    /// <p>The filters to use to list a specified set of rooms. The supported filter keys are RoomName and ProfileName.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>The maximum number of results to include in the response. If more results exist than the specified <code>MaxResults</code> value, a token is included in the response so that the remaining results can be retrieved. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An optional token returned from a prior request. Use this token for pagination of results from this action. If this parameter is specified, the response includes only results beyond the token, up to the value specified by <code>MaxResults</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The sort order to use in listing the specified set of rooms. The supported sort keys are RoomName and ProfileName.</p>
    #[serde(rename = "SortCriteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_criteria: Option<Vec<Sort>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SearchRoomsResponse {
    /// <p>The token returned to indicate that there is more data available.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The rooms that meet the specified set of filter criteria, in sort order.</p>
    #[serde(rename = "Rooms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rooms: Option<Vec<RoomData>>,
    /// <p>The total number of rooms returned.</p>
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SearchSkillGroupsRequest {
    /// <p>The filters to use to list a specified set of skill groups. The supported filter key is SkillGroupName. </p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>The maximum number of results to include in the response. If more results exist than the specified <code>MaxResults</code> value, a token is included in the response so that the remaining results can be retrieved. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An optional token returned from a prior request. Use this token for pagination of results from this action. If this parameter is specified, the response includes only results beyond the token, up to the value specified by <code>MaxResults</code>. Required.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The sort order to use in listing the specified set of skill groups. The supported sort key is SkillGroupName. </p>
    #[serde(rename = "SortCriteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_criteria: Option<Vec<Sort>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SearchSkillGroupsResponse {
    /// <p>The token returned to indicate that there is more data available.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The skill groups that meet the filter criteria, in sort order.</p>
    #[serde(rename = "SkillGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_groups: Option<Vec<SkillGroupData>>,
    /// <p>The total number of skill groups returned.</p>
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SearchUsersRequest {
    /// <p>The filters to use for listing a specific set of users. Required. Supported filter keys are UserId, FirstName, LastName, Email, and EnrollmentStatus.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>The maximum number of results to include in the response. If more results exist than the specified <code>MaxResults</code> value, a token is included in the response so that the remaining results can be retrieved. Required.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An optional token returned from a prior request. Use this token for pagination of results from this action. If this parameter is specified, the response includes only results beyond the token, up to the value specified by <code>MaxResults</code>. Required.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The sort order to use in listing the filtered set of users. Required. Supported sort keys are UserId, FirstName, LastName, Email, and EnrollmentStatus.</p>
    #[serde(rename = "SortCriteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_criteria: Option<Vec<Sort>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SearchUsersResponse {
    /// <p>The token returned to indicate that there is more data available.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The total number of users returned.</p>
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// <p>The users that meet the specified set of filter criteria, in sort order.</p>
    #[serde(rename = "Users")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<UserData>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SendInvitationRequest {
    /// <p>The ARN of the user to whom to send an invitation. Required.</p>
    #[serde(rename = "UserArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SendInvitationResponse {}

/// <p>A skill group with attributes.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SkillGroup {
    /// <p>The description of a skill group.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ARN of a skill group.</p>
    #[serde(rename = "SkillGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_group_arn: Option<String>,
    /// <p>The name of a skill group.</p>
    #[serde(rename = "SkillGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_group_name: Option<String>,
}

/// <p>The attributes of a skill group.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SkillGroupData {
    /// <p>The description of a skill group.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The skill group ARN of a skill group.</p>
    #[serde(rename = "SkillGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_group_arn: Option<String>,
    /// <p>The skill group name of a skill group.</p>
    #[serde(rename = "SkillGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_group_name: Option<String>,
}

/// <p>The summary of skills.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SkillSummary {
    /// <p>The ARN of the skill summary.</p>
    #[serde(rename = "SkillId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_id: Option<String>,
    /// <p>The name of the skill.</p>
    #[serde(rename = "SkillName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_name: Option<String>,
    /// <p>Linking support for a skill.</p>
    #[serde(rename = "SupportsLinking")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_linking: Option<bool>,
}

/// <p>An object representing a sort criteria. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Sort {
    /// <p>The sort key of a sort object.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The sort value of a sort object.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartDeviceSyncRequest {
    /// <p>The ARN of the device to sync. Required.</p>
    #[serde(rename = "DeviceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_arn: Option<String>,
    /// <p>Request structure to start the device sync. Required.</p>
    #[serde(rename = "Features")]
    pub features: Vec<String>,
    /// <p>The ARN of the room with which the device to sync is associated. Required.</p>
    #[serde(rename = "RoomArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StartDeviceSyncResponse {}

/// <p>A key-value pair that can be associated with a resource. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>The key of a tag. Tag keys are case-sensitive. </p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The value of a tag. Tag values are case-sensitive and can be null.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagResourceRequest {
    /// <p>The ARN of the resource to which to add metadata tags. Required. </p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>The tags to be added to the specified resource. Do not provide system tags. Required. </p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UntagResourceRequest {
    /// <p>The ARN of the resource from which to remove metadata tags. Required. </p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>The tags to be removed from the specified resource. Do not provide system tags. Required. </p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UntagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateAddressBookRequest {
    /// <p>The ARN of the room to update.</p>
    #[serde(rename = "AddressBookArn")]
    pub address_book_arn: String,
    /// <p>The updated description of the room.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The updated name of the room.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateAddressBookResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateContactRequest {
    /// <p>The ARN of the contact to update.</p>
    #[serde(rename = "ContactArn")]
    pub contact_arn: String,
    /// <p>The updated display name of the contact.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The updated first name of the contact.</p>
    #[serde(rename = "FirstName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// <p>The updated last name of the contact.</p>
    #[serde(rename = "LastName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// <p>The updated phone number of the contact.</p>
    #[serde(rename = "PhoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateContactResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateDeviceRequest {
    /// <p>The ARN of the device to update. Required.</p>
    #[serde(rename = "DeviceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_arn: Option<String>,
    /// <p>The updated device name. Required.</p>
    #[serde(rename = "DeviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateDeviceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateProfileRequest {
    /// <p>The updated address for the room profile.</p>
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// <p>The updated distance unit for the room profile.</p>
    #[serde(rename = "DistanceUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_unit: Option<String>,
    /// <p>The updated maximum volume limit for the room profile.</p>
    #[serde(rename = "MaxVolumeLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_volume_limit: Option<i64>,
    /// <p>Whether the PSTN setting of the room profile is enabled.</p>
    #[serde(rename = "PSTNEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pstn_enabled: Option<bool>,
    /// <p>The ARN of the room profile to update. Required.</p>
    #[serde(rename = "ProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_arn: Option<String>,
    /// <p>The updated name for the room profile.</p>
    #[serde(rename = "ProfileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_name: Option<String>,
    /// <p>Whether the setup mode of the profile is enabled.</p>
    #[serde(rename = "SetupModeDisabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_mode_disabled: Option<bool>,
    /// <p>The updated temperature unit for the room profile.</p>
    #[serde(rename = "TemperatureUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature_unit: Option<String>,
    /// <p>The updated timezone for the room profile.</p>
    #[serde(rename = "Timezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// <p>The updated wake word for the room profile.</p>
    #[serde(rename = "WakeWord")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wake_word: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateProfileResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateRoomRequest {
    /// <p>The updated description for the room.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The updated profile ARN for the room.</p>
    #[serde(rename = "ProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_arn: Option<String>,
    /// <p>The updated provider calendar ARN for the room.</p>
    #[serde(rename = "ProviderCalendarId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_calendar_id: Option<String>,
    /// <p>The ARN of the room to update. </p>
    #[serde(rename = "RoomArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_arn: Option<String>,
    /// <p>The updated name for the room.</p>
    #[serde(rename = "RoomName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateRoomResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateSkillGroupRequest {
    /// <p>The updated description for the skill group.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ARN of the skill group to update. </p>
    #[serde(rename = "SkillGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_group_arn: Option<String>,
    /// <p>The updated name for the skill group.</p>
    #[serde(rename = "SkillGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_group_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateSkillGroupResponse {}

/// <p>Information related to a user.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UserData {
    /// <p>The email of a user.</p>
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// <p>The enrollment ARN of a user.</p>
    #[serde(rename = "EnrollmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enrollment_id: Option<String>,
    /// <p>The enrollment status of a user.</p>
    #[serde(rename = "EnrollmentStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enrollment_status: Option<String>,
    /// <p>The first name of a user.</p>
    #[serde(rename = "FirstName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// <p>The last name of a user.</p>
    #[serde(rename = "LastName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// <p>The ARN of a user.</p>
    #[serde(rename = "UserArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_arn: Option<String>,
}

/// Errors returned by AssociateContactWithAddressBook
#[derive(Debug, PartialEq)]
pub enum AssociateContactWithAddressBookError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AssociateContactWithAddressBookError {
    pub fn from_body(body: &str) -> AssociateContactWithAddressBookError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ValidationException" => {
                        AssociateContactWithAddressBookError::Validation(error_message.to_string())
                    }
                    _ => AssociateContactWithAddressBookError::Unknown(String::from(body)),
                }
            }
            Err(_) => AssociateContactWithAddressBookError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AssociateContactWithAddressBookError {
    fn from(err: serde_json::error::Error) -> AssociateContactWithAddressBookError {
        AssociateContactWithAddressBookError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AssociateContactWithAddressBookError {
    fn from(err: CredentialsError) -> AssociateContactWithAddressBookError {
        AssociateContactWithAddressBookError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AssociateContactWithAddressBookError {
    fn from(err: HttpDispatchError) -> AssociateContactWithAddressBookError {
        AssociateContactWithAddressBookError::HttpDispatch(err)
    }
}
impl From<io::Error> for AssociateContactWithAddressBookError {
    fn from(err: io::Error) -> AssociateContactWithAddressBookError {
        AssociateContactWithAddressBookError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AssociateContactWithAddressBookError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateContactWithAddressBookError {
    fn description(&self) -> &str {
        match *self {
            AssociateContactWithAddressBookError::Validation(ref cause) => cause,
            AssociateContactWithAddressBookError::Credentials(ref err) => err.description(),
            AssociateContactWithAddressBookError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AssociateContactWithAddressBookError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AssociateDeviceWithRoom
#[derive(Debug, PartialEq)]
pub enum AssociateDeviceWithRoomError {
    /// <p>You are performing an action that would put you beyond your account's limits. HTTP Status Code: 400</p>
    LimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AssociateDeviceWithRoomError {
    pub fn from_body(body: &str) -> AssociateDeviceWithRoomError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "LimitExceededException" => {
                        AssociateDeviceWithRoomError::LimitExceeded(String::from(error_message))
                    }
                    "ValidationException" => {
                        AssociateDeviceWithRoomError::Validation(error_message.to_string())
                    }
                    _ => AssociateDeviceWithRoomError::Unknown(String::from(body)),
                }
            }
            Err(_) => AssociateDeviceWithRoomError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AssociateDeviceWithRoomError {
    fn from(err: serde_json::error::Error) -> AssociateDeviceWithRoomError {
        AssociateDeviceWithRoomError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AssociateDeviceWithRoomError {
    fn from(err: CredentialsError) -> AssociateDeviceWithRoomError {
        AssociateDeviceWithRoomError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AssociateDeviceWithRoomError {
    fn from(err: HttpDispatchError) -> AssociateDeviceWithRoomError {
        AssociateDeviceWithRoomError::HttpDispatch(err)
    }
}
impl From<io::Error> for AssociateDeviceWithRoomError {
    fn from(err: io::Error) -> AssociateDeviceWithRoomError {
        AssociateDeviceWithRoomError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AssociateDeviceWithRoomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateDeviceWithRoomError {
    fn description(&self) -> &str {
        match *self {
            AssociateDeviceWithRoomError::LimitExceeded(ref cause) => cause,
            AssociateDeviceWithRoomError::Validation(ref cause) => cause,
            AssociateDeviceWithRoomError::Credentials(ref err) => err.description(),
            AssociateDeviceWithRoomError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AssociateDeviceWithRoomError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AssociateSkillGroupWithRoom
#[derive(Debug, PartialEq)]
pub enum AssociateSkillGroupWithRoomError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AssociateSkillGroupWithRoomError {
    pub fn from_body(body: &str) -> AssociateSkillGroupWithRoomError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ValidationException" => {
                        AssociateSkillGroupWithRoomError::Validation(error_message.to_string())
                    }
                    _ => AssociateSkillGroupWithRoomError::Unknown(String::from(body)),
                }
            }
            Err(_) => AssociateSkillGroupWithRoomError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AssociateSkillGroupWithRoomError {
    fn from(err: serde_json::error::Error) -> AssociateSkillGroupWithRoomError {
        AssociateSkillGroupWithRoomError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AssociateSkillGroupWithRoomError {
    fn from(err: CredentialsError) -> AssociateSkillGroupWithRoomError {
        AssociateSkillGroupWithRoomError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AssociateSkillGroupWithRoomError {
    fn from(err: HttpDispatchError) -> AssociateSkillGroupWithRoomError {
        AssociateSkillGroupWithRoomError::HttpDispatch(err)
    }
}
impl From<io::Error> for AssociateSkillGroupWithRoomError {
    fn from(err: io::Error) -> AssociateSkillGroupWithRoomError {
        AssociateSkillGroupWithRoomError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AssociateSkillGroupWithRoomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateSkillGroupWithRoomError {
    fn description(&self) -> &str {
        match *self {
            AssociateSkillGroupWithRoomError::Validation(ref cause) => cause,
            AssociateSkillGroupWithRoomError::Credentials(ref err) => err.description(),
            AssociateSkillGroupWithRoomError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AssociateSkillGroupWithRoomError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateAddressBook
#[derive(Debug, PartialEq)]
pub enum CreateAddressBookError {
    /// <p>The resource being created already exists. HTTP Status Code: 400</p>
    AlreadyExists(String),
    /// <p>You are performing an action that would put you beyond your account's limits. HTTP Status Code: 400</p>
    LimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateAddressBookError {
    pub fn from_body(body: &str) -> CreateAddressBookError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AlreadyExistsException" => {
                        CreateAddressBookError::AlreadyExists(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateAddressBookError::LimitExceeded(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateAddressBookError::Validation(error_message.to_string())
                    }
                    _ => CreateAddressBookError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateAddressBookError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateAddressBookError {
    fn from(err: serde_json::error::Error) -> CreateAddressBookError {
        CreateAddressBookError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateAddressBookError {
    fn from(err: CredentialsError) -> CreateAddressBookError {
        CreateAddressBookError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateAddressBookError {
    fn from(err: HttpDispatchError) -> CreateAddressBookError {
        CreateAddressBookError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateAddressBookError {
    fn from(err: io::Error) -> CreateAddressBookError {
        CreateAddressBookError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateAddressBookError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateAddressBookError {
    fn description(&self) -> &str {
        match *self {
            CreateAddressBookError::AlreadyExists(ref cause) => cause,
            CreateAddressBookError::LimitExceeded(ref cause) => cause,
            CreateAddressBookError::Validation(ref cause) => cause,
            CreateAddressBookError::Credentials(ref err) => err.description(),
            CreateAddressBookError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateAddressBookError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateContact
#[derive(Debug, PartialEq)]
pub enum CreateContactError {
    /// <p>The resource being created already exists. HTTP Status Code: 400</p>
    AlreadyExists(String),
    /// <p>You are performing an action that would put you beyond your account's limits. HTTP Status Code: 400</p>
    LimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateContactError {
    pub fn from_body(body: &str) -> CreateContactError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AlreadyExistsException" => {
                        CreateContactError::AlreadyExists(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateContactError::LimitExceeded(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateContactError::Validation(error_message.to_string())
                    }
                    _ => CreateContactError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateContactError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateContactError {
    fn from(err: serde_json::error::Error) -> CreateContactError {
        CreateContactError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateContactError {
    fn from(err: CredentialsError) -> CreateContactError {
        CreateContactError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateContactError {
    fn from(err: HttpDispatchError) -> CreateContactError {
        CreateContactError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateContactError {
    fn from(err: io::Error) -> CreateContactError {
        CreateContactError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateContactError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateContactError {
    fn description(&self) -> &str {
        match *self {
            CreateContactError::AlreadyExists(ref cause) => cause,
            CreateContactError::LimitExceeded(ref cause) => cause,
            CreateContactError::Validation(ref cause) => cause,
            CreateContactError::Credentials(ref err) => err.description(),
            CreateContactError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateContactError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateProfile
#[derive(Debug, PartialEq)]
pub enum CreateProfileError {
    /// <p>The resource being created already exists. HTTP Status Code: 400</p>
    AlreadyExists(String),
    /// <p>You are performing an action that would put you beyond your account's limits. HTTP Status Code: 400</p>
    LimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateProfileError {
    pub fn from_body(body: &str) -> CreateProfileError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AlreadyExistsException" => {
                        CreateProfileError::AlreadyExists(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateProfileError::LimitExceeded(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateProfileError::Validation(error_message.to_string())
                    }
                    _ => CreateProfileError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateProfileError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateProfileError {
    fn from(err: serde_json::error::Error) -> CreateProfileError {
        CreateProfileError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateProfileError {
    fn from(err: CredentialsError) -> CreateProfileError {
        CreateProfileError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateProfileError {
    fn from(err: HttpDispatchError) -> CreateProfileError {
        CreateProfileError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateProfileError {
    fn from(err: io::Error) -> CreateProfileError {
        CreateProfileError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateProfileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateProfileError {
    fn description(&self) -> &str {
        match *self {
            CreateProfileError::AlreadyExists(ref cause) => cause,
            CreateProfileError::LimitExceeded(ref cause) => cause,
            CreateProfileError::Validation(ref cause) => cause,
            CreateProfileError::Credentials(ref err) => err.description(),
            CreateProfileError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateProfileError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateRoom
#[derive(Debug, PartialEq)]
pub enum CreateRoomError {
    /// <p>The resource being created already exists. HTTP Status Code: 400</p>
    AlreadyExists(String),
    /// <p>You are performing an action that would put you beyond your account's limits. HTTP Status Code: 400</p>
    LimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateRoomError {
    pub fn from_body(body: &str) -> CreateRoomError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AlreadyExistsException" => {
                        CreateRoomError::AlreadyExists(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateRoomError::LimitExceeded(String::from(error_message))
                    }
                    "ValidationException" => CreateRoomError::Validation(error_message.to_string()),
                    _ => CreateRoomError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateRoomError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateRoomError {
    fn from(err: serde_json::error::Error) -> CreateRoomError {
        CreateRoomError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateRoomError {
    fn from(err: CredentialsError) -> CreateRoomError {
        CreateRoomError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateRoomError {
    fn from(err: HttpDispatchError) -> CreateRoomError {
        CreateRoomError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateRoomError {
    fn from(err: io::Error) -> CreateRoomError {
        CreateRoomError::HttpDispatch(HttpDispatchError::from(err))
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
            CreateRoomError::AlreadyExists(ref cause) => cause,
            CreateRoomError::LimitExceeded(ref cause) => cause,
            CreateRoomError::Validation(ref cause) => cause,
            CreateRoomError::Credentials(ref err) => err.description(),
            CreateRoomError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateRoomError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateSkillGroup
#[derive(Debug, PartialEq)]
pub enum CreateSkillGroupError {
    /// <p>The resource being created already exists. HTTP Status Code: 400</p>
    AlreadyExists(String),
    /// <p>You are performing an action that would put you beyond your account's limits. HTTP Status Code: 400</p>
    LimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateSkillGroupError {
    pub fn from_body(body: &str) -> CreateSkillGroupError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AlreadyExistsException" => {
                        CreateSkillGroupError::AlreadyExists(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateSkillGroupError::LimitExceeded(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateSkillGroupError::Validation(error_message.to_string())
                    }
                    _ => CreateSkillGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateSkillGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateSkillGroupError {
    fn from(err: serde_json::error::Error) -> CreateSkillGroupError {
        CreateSkillGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateSkillGroupError {
    fn from(err: CredentialsError) -> CreateSkillGroupError {
        CreateSkillGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateSkillGroupError {
    fn from(err: HttpDispatchError) -> CreateSkillGroupError {
        CreateSkillGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateSkillGroupError {
    fn from(err: io::Error) -> CreateSkillGroupError {
        CreateSkillGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateSkillGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateSkillGroupError {
    fn description(&self) -> &str {
        match *self {
            CreateSkillGroupError::AlreadyExists(ref cause) => cause,
            CreateSkillGroupError::LimitExceeded(ref cause) => cause,
            CreateSkillGroupError::Validation(ref cause) => cause,
            CreateSkillGroupError::Credentials(ref err) => err.description(),
            CreateSkillGroupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateSkillGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateUser
#[derive(Debug, PartialEq)]
pub enum CreateUserError {
    /// <p>You are performing an action that would put you beyond your account's limits. HTTP Status Code: 400</p>
    LimitExceeded(String),
    /// <p>The resource in the request is already in use. HTTP Status Code: 400</p>
    ResourceInUse(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateUserError {
    pub fn from_body(body: &str) -> CreateUserError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "LimitExceededException" => {
                        CreateUserError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        CreateUserError::ResourceInUse(String::from(error_message))
                    }
                    "ValidationException" => CreateUserError::Validation(error_message.to_string()),
                    _ => CreateUserError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateUserError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateUserError {
    fn from(err: serde_json::error::Error) -> CreateUserError {
        CreateUserError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateUserError {
    fn from(err: CredentialsError) -> CreateUserError {
        CreateUserError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateUserError {
    fn from(err: HttpDispatchError) -> CreateUserError {
        CreateUserError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateUserError {
    fn from(err: io::Error) -> CreateUserError {
        CreateUserError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateUserError {
    fn description(&self) -> &str {
        match *self {
            CreateUserError::LimitExceeded(ref cause) => cause,
            CreateUserError::ResourceInUse(ref cause) => cause,
            CreateUserError::Validation(ref cause) => cause,
            CreateUserError::Credentials(ref err) => err.description(),
            CreateUserError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateUserError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteAddressBook
#[derive(Debug, PartialEq)]
pub enum DeleteAddressBookError {
    /// <p>The resource is not found. HTTP Status Code: 400</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteAddressBookError {
    pub fn from_body(body: &str) -> DeleteAddressBookError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "NotFoundException" => {
                        DeleteAddressBookError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteAddressBookError::Validation(error_message.to_string())
                    }
                    _ => DeleteAddressBookError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteAddressBookError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteAddressBookError {
    fn from(err: serde_json::error::Error) -> DeleteAddressBookError {
        DeleteAddressBookError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteAddressBookError {
    fn from(err: CredentialsError) -> DeleteAddressBookError {
        DeleteAddressBookError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteAddressBookError {
    fn from(err: HttpDispatchError) -> DeleteAddressBookError {
        DeleteAddressBookError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteAddressBookError {
    fn from(err: io::Error) -> DeleteAddressBookError {
        DeleteAddressBookError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteAddressBookError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteAddressBookError {
    fn description(&self) -> &str {
        match *self {
            DeleteAddressBookError::NotFound(ref cause) => cause,
            DeleteAddressBookError::Validation(ref cause) => cause,
            DeleteAddressBookError::Credentials(ref err) => err.description(),
            DeleteAddressBookError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteAddressBookError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteContact
#[derive(Debug, PartialEq)]
pub enum DeleteContactError {
    /// <p>The resource is not found. HTTP Status Code: 400</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteContactError {
    pub fn from_body(body: &str) -> DeleteContactError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "NotFoundException" => {
                        DeleteContactError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteContactError::Validation(error_message.to_string())
                    }
                    _ => DeleteContactError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteContactError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteContactError {
    fn from(err: serde_json::error::Error) -> DeleteContactError {
        DeleteContactError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteContactError {
    fn from(err: CredentialsError) -> DeleteContactError {
        DeleteContactError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteContactError {
    fn from(err: HttpDispatchError) -> DeleteContactError {
        DeleteContactError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteContactError {
    fn from(err: io::Error) -> DeleteContactError {
        DeleteContactError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteContactError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteContactError {
    fn description(&self) -> &str {
        match *self {
            DeleteContactError::NotFound(ref cause) => cause,
            DeleteContactError::Validation(ref cause) => cause,
            DeleteContactError::Credentials(ref err) => err.description(),
            DeleteContactError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteContactError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteProfile
#[derive(Debug, PartialEq)]
pub enum DeleteProfileError {
    /// <p>The resource is not found. HTTP Status Code: 400</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteProfileError {
    pub fn from_body(body: &str) -> DeleteProfileError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "NotFoundException" => {
                        DeleteProfileError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteProfileError::Validation(error_message.to_string())
                    }
                    _ => DeleteProfileError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteProfileError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteProfileError {
    fn from(err: serde_json::error::Error) -> DeleteProfileError {
        DeleteProfileError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteProfileError {
    fn from(err: CredentialsError) -> DeleteProfileError {
        DeleteProfileError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteProfileError {
    fn from(err: HttpDispatchError) -> DeleteProfileError {
        DeleteProfileError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteProfileError {
    fn from(err: io::Error) -> DeleteProfileError {
        DeleteProfileError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteProfileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteProfileError {
    fn description(&self) -> &str {
        match *self {
            DeleteProfileError::NotFound(ref cause) => cause,
            DeleteProfileError::Validation(ref cause) => cause,
            DeleteProfileError::Credentials(ref err) => err.description(),
            DeleteProfileError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteProfileError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteRoom
#[derive(Debug, PartialEq)]
pub enum DeleteRoomError {
    /// <p>The resource is not found. HTTP Status Code: 400</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteRoomError {
    pub fn from_body(body: &str) -> DeleteRoomError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "NotFoundException" => DeleteRoomError::NotFound(String::from(error_message)),
                    "ValidationException" => DeleteRoomError::Validation(error_message.to_string()),
                    _ => DeleteRoomError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteRoomError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteRoomError {
    fn from(err: serde_json::error::Error) -> DeleteRoomError {
        DeleteRoomError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteRoomError {
    fn from(err: CredentialsError) -> DeleteRoomError {
        DeleteRoomError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteRoomError {
    fn from(err: HttpDispatchError) -> DeleteRoomError {
        DeleteRoomError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteRoomError {
    fn from(err: io::Error) -> DeleteRoomError {
        DeleteRoomError::HttpDispatch(HttpDispatchError::from(err))
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
            DeleteRoomError::NotFound(ref cause) => cause,
            DeleteRoomError::Validation(ref cause) => cause,
            DeleteRoomError::Credentials(ref err) => err.description(),
            DeleteRoomError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteRoomError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteRoomSkillParameter
#[derive(Debug, PartialEq)]
pub enum DeleteRoomSkillParameterError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteRoomSkillParameterError {
    pub fn from_body(body: &str) -> DeleteRoomSkillParameterError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ValidationException" => {
                        DeleteRoomSkillParameterError::Validation(error_message.to_string())
                    }
                    _ => DeleteRoomSkillParameterError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteRoomSkillParameterError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteRoomSkillParameterError {
    fn from(err: serde_json::error::Error) -> DeleteRoomSkillParameterError {
        DeleteRoomSkillParameterError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteRoomSkillParameterError {
    fn from(err: CredentialsError) -> DeleteRoomSkillParameterError {
        DeleteRoomSkillParameterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteRoomSkillParameterError {
    fn from(err: HttpDispatchError) -> DeleteRoomSkillParameterError {
        DeleteRoomSkillParameterError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteRoomSkillParameterError {
    fn from(err: io::Error) -> DeleteRoomSkillParameterError {
        DeleteRoomSkillParameterError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteRoomSkillParameterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteRoomSkillParameterError {
    fn description(&self) -> &str {
        match *self {
            DeleteRoomSkillParameterError::Validation(ref cause) => cause,
            DeleteRoomSkillParameterError::Credentials(ref err) => err.description(),
            DeleteRoomSkillParameterError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteRoomSkillParameterError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteSkillGroup
#[derive(Debug, PartialEq)]
pub enum DeleteSkillGroupError {
    /// <p>The resource is not found. HTTP Status Code: 400</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteSkillGroupError {
    pub fn from_body(body: &str) -> DeleteSkillGroupError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "NotFoundException" => {
                        DeleteSkillGroupError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteSkillGroupError::Validation(error_message.to_string())
                    }
                    _ => DeleteSkillGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteSkillGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteSkillGroupError {
    fn from(err: serde_json::error::Error) -> DeleteSkillGroupError {
        DeleteSkillGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteSkillGroupError {
    fn from(err: CredentialsError) -> DeleteSkillGroupError {
        DeleteSkillGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteSkillGroupError {
    fn from(err: HttpDispatchError) -> DeleteSkillGroupError {
        DeleteSkillGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteSkillGroupError {
    fn from(err: io::Error) -> DeleteSkillGroupError {
        DeleteSkillGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteSkillGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteSkillGroupError {
    fn description(&self) -> &str {
        match *self {
            DeleteSkillGroupError::NotFound(ref cause) => cause,
            DeleteSkillGroupError::Validation(ref cause) => cause,
            DeleteSkillGroupError::Credentials(ref err) => err.description(),
            DeleteSkillGroupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteSkillGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteUser
#[derive(Debug, PartialEq)]
pub enum DeleteUserError {
    /// <p>The resource is not found. HTTP Status Code: 400</p>
    NotFound(String),
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
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "NotFoundException" => DeleteUserError::NotFound(String::from(error_message)),
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
impl From<io::Error> for DeleteUserError {
    fn from(err: io::Error) -> DeleteUserError {
        DeleteUserError::HttpDispatch(HttpDispatchError::from(err))
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
            DeleteUserError::NotFound(ref cause) => cause,
            DeleteUserError::Validation(ref cause) => cause,
            DeleteUserError::Credentials(ref err) => err.description(),
            DeleteUserError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteUserError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DisassociateContactFromAddressBook
#[derive(Debug, PartialEq)]
pub enum DisassociateContactFromAddressBookError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DisassociateContactFromAddressBookError {
    pub fn from_body(body: &str) -> DisassociateContactFromAddressBookError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ValidationException" => DisassociateContactFromAddressBookError::Validation(
                        error_message.to_string(),
                    ),
                    _ => DisassociateContactFromAddressBookError::Unknown(String::from(body)),
                }
            }
            Err(_) => DisassociateContactFromAddressBookError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DisassociateContactFromAddressBookError {
    fn from(err: serde_json::error::Error) -> DisassociateContactFromAddressBookError {
        DisassociateContactFromAddressBookError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DisassociateContactFromAddressBookError {
    fn from(err: CredentialsError) -> DisassociateContactFromAddressBookError {
        DisassociateContactFromAddressBookError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisassociateContactFromAddressBookError {
    fn from(err: HttpDispatchError) -> DisassociateContactFromAddressBookError {
        DisassociateContactFromAddressBookError::HttpDispatch(err)
    }
}
impl From<io::Error> for DisassociateContactFromAddressBookError {
    fn from(err: io::Error) -> DisassociateContactFromAddressBookError {
        DisassociateContactFromAddressBookError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DisassociateContactFromAddressBookError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateContactFromAddressBookError {
    fn description(&self) -> &str {
        match *self {
            DisassociateContactFromAddressBookError::Validation(ref cause) => cause,
            DisassociateContactFromAddressBookError::Credentials(ref err) => err.description(),
            DisassociateContactFromAddressBookError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DisassociateContactFromAddressBookError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DisassociateDeviceFromRoom
#[derive(Debug, PartialEq)]
pub enum DisassociateDeviceFromRoomError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DisassociateDeviceFromRoomError {
    pub fn from_body(body: &str) -> DisassociateDeviceFromRoomError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ValidationException" => {
                        DisassociateDeviceFromRoomError::Validation(error_message.to_string())
                    }
                    _ => DisassociateDeviceFromRoomError::Unknown(String::from(body)),
                }
            }
            Err(_) => DisassociateDeviceFromRoomError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DisassociateDeviceFromRoomError {
    fn from(err: serde_json::error::Error) -> DisassociateDeviceFromRoomError {
        DisassociateDeviceFromRoomError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DisassociateDeviceFromRoomError {
    fn from(err: CredentialsError) -> DisassociateDeviceFromRoomError {
        DisassociateDeviceFromRoomError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisassociateDeviceFromRoomError {
    fn from(err: HttpDispatchError) -> DisassociateDeviceFromRoomError {
        DisassociateDeviceFromRoomError::HttpDispatch(err)
    }
}
impl From<io::Error> for DisassociateDeviceFromRoomError {
    fn from(err: io::Error) -> DisassociateDeviceFromRoomError {
        DisassociateDeviceFromRoomError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DisassociateDeviceFromRoomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateDeviceFromRoomError {
    fn description(&self) -> &str {
        match *self {
            DisassociateDeviceFromRoomError::Validation(ref cause) => cause,
            DisassociateDeviceFromRoomError::Credentials(ref err) => err.description(),
            DisassociateDeviceFromRoomError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DisassociateDeviceFromRoomError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DisassociateSkillGroupFromRoom
#[derive(Debug, PartialEq)]
pub enum DisassociateSkillGroupFromRoomError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DisassociateSkillGroupFromRoomError {
    pub fn from_body(body: &str) -> DisassociateSkillGroupFromRoomError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ValidationException" => {
                        DisassociateSkillGroupFromRoomError::Validation(error_message.to_string())
                    }
                    _ => DisassociateSkillGroupFromRoomError::Unknown(String::from(body)),
                }
            }
            Err(_) => DisassociateSkillGroupFromRoomError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DisassociateSkillGroupFromRoomError {
    fn from(err: serde_json::error::Error) -> DisassociateSkillGroupFromRoomError {
        DisassociateSkillGroupFromRoomError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DisassociateSkillGroupFromRoomError {
    fn from(err: CredentialsError) -> DisassociateSkillGroupFromRoomError {
        DisassociateSkillGroupFromRoomError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisassociateSkillGroupFromRoomError {
    fn from(err: HttpDispatchError) -> DisassociateSkillGroupFromRoomError {
        DisassociateSkillGroupFromRoomError::HttpDispatch(err)
    }
}
impl From<io::Error> for DisassociateSkillGroupFromRoomError {
    fn from(err: io::Error) -> DisassociateSkillGroupFromRoomError {
        DisassociateSkillGroupFromRoomError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DisassociateSkillGroupFromRoomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateSkillGroupFromRoomError {
    fn description(&self) -> &str {
        match *self {
            DisassociateSkillGroupFromRoomError::Validation(ref cause) => cause,
            DisassociateSkillGroupFromRoomError::Credentials(ref err) => err.description(),
            DisassociateSkillGroupFromRoomError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DisassociateSkillGroupFromRoomError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetAddressBook
#[derive(Debug, PartialEq)]
pub enum GetAddressBookError {
    /// <p>The resource is not found. HTTP Status Code: 400</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetAddressBookError {
    pub fn from_body(body: &str) -> GetAddressBookError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "NotFoundException" => {
                        GetAddressBookError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetAddressBookError::Validation(error_message.to_string())
                    }
                    _ => GetAddressBookError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetAddressBookError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetAddressBookError {
    fn from(err: serde_json::error::Error) -> GetAddressBookError {
        GetAddressBookError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetAddressBookError {
    fn from(err: CredentialsError) -> GetAddressBookError {
        GetAddressBookError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetAddressBookError {
    fn from(err: HttpDispatchError) -> GetAddressBookError {
        GetAddressBookError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetAddressBookError {
    fn from(err: io::Error) -> GetAddressBookError {
        GetAddressBookError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetAddressBookError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAddressBookError {
    fn description(&self) -> &str {
        match *self {
            GetAddressBookError::NotFound(ref cause) => cause,
            GetAddressBookError::Validation(ref cause) => cause,
            GetAddressBookError::Credentials(ref err) => err.description(),
            GetAddressBookError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetAddressBookError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetContact
#[derive(Debug, PartialEq)]
pub enum GetContactError {
    /// <p>The resource is not found. HTTP Status Code: 400</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetContactError {
    pub fn from_body(body: &str) -> GetContactError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "NotFoundException" => GetContactError::NotFound(String::from(error_message)),
                    "ValidationException" => GetContactError::Validation(error_message.to_string()),
                    _ => GetContactError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetContactError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetContactError {
    fn from(err: serde_json::error::Error) -> GetContactError {
        GetContactError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetContactError {
    fn from(err: CredentialsError) -> GetContactError {
        GetContactError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetContactError {
    fn from(err: HttpDispatchError) -> GetContactError {
        GetContactError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetContactError {
    fn from(err: io::Error) -> GetContactError {
        GetContactError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetContactError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetContactError {
    fn description(&self) -> &str {
        match *self {
            GetContactError::NotFound(ref cause) => cause,
            GetContactError::Validation(ref cause) => cause,
            GetContactError::Credentials(ref err) => err.description(),
            GetContactError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetContactError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDevice
#[derive(Debug, PartialEq)]
pub enum GetDeviceError {
    /// <p>The resource is not found. HTTP Status Code: 400</p>
    NotFound(String),
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
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "NotFoundException" => GetDeviceError::NotFound(String::from(error_message)),
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
impl From<io::Error> for GetDeviceError {
    fn from(err: io::Error) -> GetDeviceError {
        GetDeviceError::HttpDispatch(HttpDispatchError::from(err))
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
            GetDeviceError::NotFound(ref cause) => cause,
            GetDeviceError::Validation(ref cause) => cause,
            GetDeviceError::Credentials(ref err) => err.description(),
            GetDeviceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetDeviceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetProfile
#[derive(Debug, PartialEq)]
pub enum GetProfileError {
    /// <p>The resource is not found. HTTP Status Code: 400</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetProfileError {
    pub fn from_body(body: &str) -> GetProfileError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "NotFoundException" => GetProfileError::NotFound(String::from(error_message)),
                    "ValidationException" => GetProfileError::Validation(error_message.to_string()),
                    _ => GetProfileError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetProfileError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetProfileError {
    fn from(err: serde_json::error::Error) -> GetProfileError {
        GetProfileError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetProfileError {
    fn from(err: CredentialsError) -> GetProfileError {
        GetProfileError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetProfileError {
    fn from(err: HttpDispatchError) -> GetProfileError {
        GetProfileError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetProfileError {
    fn from(err: io::Error) -> GetProfileError {
        GetProfileError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetProfileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetProfileError {
    fn description(&self) -> &str {
        match *self {
            GetProfileError::NotFound(ref cause) => cause,
            GetProfileError::Validation(ref cause) => cause,
            GetProfileError::Credentials(ref err) => err.description(),
            GetProfileError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetProfileError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetRoom
#[derive(Debug, PartialEq)]
pub enum GetRoomError {
    /// <p>The resource is not found. HTTP Status Code: 400</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetRoomError {
    pub fn from_body(body: &str) -> GetRoomError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "NotFoundException" => GetRoomError::NotFound(String::from(error_message)),
                    "ValidationException" => GetRoomError::Validation(error_message.to_string()),
                    _ => GetRoomError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetRoomError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetRoomError {
    fn from(err: serde_json::error::Error) -> GetRoomError {
        GetRoomError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetRoomError {
    fn from(err: CredentialsError) -> GetRoomError {
        GetRoomError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetRoomError {
    fn from(err: HttpDispatchError) -> GetRoomError {
        GetRoomError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetRoomError {
    fn from(err: io::Error) -> GetRoomError {
        GetRoomError::HttpDispatch(HttpDispatchError::from(err))
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
            GetRoomError::NotFound(ref cause) => cause,
            GetRoomError::Validation(ref cause) => cause,
            GetRoomError::Credentials(ref err) => err.description(),
            GetRoomError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetRoomError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetRoomSkillParameter
#[derive(Debug, PartialEq)]
pub enum GetRoomSkillParameterError {
    /// <p>The resource is not found. HTTP Status Code: 400</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetRoomSkillParameterError {
    pub fn from_body(body: &str) -> GetRoomSkillParameterError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "NotFoundException" => {
                        GetRoomSkillParameterError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetRoomSkillParameterError::Validation(error_message.to_string())
                    }
                    _ => GetRoomSkillParameterError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetRoomSkillParameterError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetRoomSkillParameterError {
    fn from(err: serde_json::error::Error) -> GetRoomSkillParameterError {
        GetRoomSkillParameterError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetRoomSkillParameterError {
    fn from(err: CredentialsError) -> GetRoomSkillParameterError {
        GetRoomSkillParameterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetRoomSkillParameterError {
    fn from(err: HttpDispatchError) -> GetRoomSkillParameterError {
        GetRoomSkillParameterError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetRoomSkillParameterError {
    fn from(err: io::Error) -> GetRoomSkillParameterError {
        GetRoomSkillParameterError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetRoomSkillParameterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetRoomSkillParameterError {
    fn description(&self) -> &str {
        match *self {
            GetRoomSkillParameterError::NotFound(ref cause) => cause,
            GetRoomSkillParameterError::Validation(ref cause) => cause,
            GetRoomSkillParameterError::Credentials(ref err) => err.description(),
            GetRoomSkillParameterError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetRoomSkillParameterError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSkillGroup
#[derive(Debug, PartialEq)]
pub enum GetSkillGroupError {
    /// <p>The resource is not found. HTTP Status Code: 400</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetSkillGroupError {
    pub fn from_body(body: &str) -> GetSkillGroupError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "NotFoundException" => {
                        GetSkillGroupError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetSkillGroupError::Validation(error_message.to_string())
                    }
                    _ => GetSkillGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetSkillGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetSkillGroupError {
    fn from(err: serde_json::error::Error) -> GetSkillGroupError {
        GetSkillGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetSkillGroupError {
    fn from(err: CredentialsError) -> GetSkillGroupError {
        GetSkillGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetSkillGroupError {
    fn from(err: HttpDispatchError) -> GetSkillGroupError {
        GetSkillGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetSkillGroupError {
    fn from(err: io::Error) -> GetSkillGroupError {
        GetSkillGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetSkillGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSkillGroupError {
    fn description(&self) -> &str {
        match *self {
            GetSkillGroupError::NotFound(ref cause) => cause,
            GetSkillGroupError::Validation(ref cause) => cause,
            GetSkillGroupError::Credentials(ref err) => err.description(),
            GetSkillGroupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetSkillGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListSkills
#[derive(Debug, PartialEq)]
pub enum ListSkillsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListSkillsError {
    pub fn from_body(body: &str) -> ListSkillsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ValidationException" => ListSkillsError::Validation(error_message.to_string()),
                    _ => ListSkillsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListSkillsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListSkillsError {
    fn from(err: serde_json::error::Error) -> ListSkillsError {
        ListSkillsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListSkillsError {
    fn from(err: CredentialsError) -> ListSkillsError {
        ListSkillsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListSkillsError {
    fn from(err: HttpDispatchError) -> ListSkillsError {
        ListSkillsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListSkillsError {
    fn from(err: io::Error) -> ListSkillsError {
        ListSkillsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListSkillsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListSkillsError {
    fn description(&self) -> &str {
        match *self {
            ListSkillsError::Validation(ref cause) => cause,
            ListSkillsError::Credentials(ref err) => err.description(),
            ListSkillsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListSkillsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTags
#[derive(Debug, PartialEq)]
pub enum ListTagsError {
    /// <p>The resource is not found. HTTP Status Code: 400</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListTagsError {
    pub fn from_body(body: &str) -> ListTagsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "NotFoundException" => ListTagsError::NotFound(String::from(error_message)),
                    "ValidationException" => ListTagsError::Validation(error_message.to_string()),
                    _ => ListTagsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListTagsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListTagsError {
    fn from(err: serde_json::error::Error) -> ListTagsError {
        ListTagsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListTagsError {
    fn from(err: CredentialsError) -> ListTagsError {
        ListTagsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTagsError {
    fn from(err: HttpDispatchError) -> ListTagsError {
        ListTagsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTagsError {
    fn from(err: io::Error) -> ListTagsError {
        ListTagsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTagsError {
    fn description(&self) -> &str {
        match *self {
            ListTagsError::NotFound(ref cause) => cause,
            ListTagsError::Validation(ref cause) => cause,
            ListTagsError::Credentials(ref err) => err.description(),
            ListTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListTagsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutRoomSkillParameter
#[derive(Debug, PartialEq)]
pub enum PutRoomSkillParameterError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutRoomSkillParameterError {
    pub fn from_body(body: &str) -> PutRoomSkillParameterError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ValidationException" => {
                        PutRoomSkillParameterError::Validation(error_message.to_string())
                    }
                    _ => PutRoomSkillParameterError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutRoomSkillParameterError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutRoomSkillParameterError {
    fn from(err: serde_json::error::Error) -> PutRoomSkillParameterError {
        PutRoomSkillParameterError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutRoomSkillParameterError {
    fn from(err: CredentialsError) -> PutRoomSkillParameterError {
        PutRoomSkillParameterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutRoomSkillParameterError {
    fn from(err: HttpDispatchError) -> PutRoomSkillParameterError {
        PutRoomSkillParameterError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutRoomSkillParameterError {
    fn from(err: io::Error) -> PutRoomSkillParameterError {
        PutRoomSkillParameterError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutRoomSkillParameterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutRoomSkillParameterError {
    fn description(&self) -> &str {
        match *self {
            PutRoomSkillParameterError::Validation(ref cause) => cause,
            PutRoomSkillParameterError::Credentials(ref err) => err.description(),
            PutRoomSkillParameterError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutRoomSkillParameterError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ResolveRoom
#[derive(Debug, PartialEq)]
pub enum ResolveRoomError {
    /// <p>The resource is not found. HTTP Status Code: 400</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ResolveRoomError {
    pub fn from_body(body: &str) -> ResolveRoomError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "NotFoundException" => ResolveRoomError::NotFound(String::from(error_message)),
                    "ValidationException" => {
                        ResolveRoomError::Validation(error_message.to_string())
                    }
                    _ => ResolveRoomError::Unknown(String::from(body)),
                }
            }
            Err(_) => ResolveRoomError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ResolveRoomError {
    fn from(err: serde_json::error::Error) -> ResolveRoomError {
        ResolveRoomError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ResolveRoomError {
    fn from(err: CredentialsError) -> ResolveRoomError {
        ResolveRoomError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ResolveRoomError {
    fn from(err: HttpDispatchError) -> ResolveRoomError {
        ResolveRoomError::HttpDispatch(err)
    }
}
impl From<io::Error> for ResolveRoomError {
    fn from(err: io::Error) -> ResolveRoomError {
        ResolveRoomError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ResolveRoomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ResolveRoomError {
    fn description(&self) -> &str {
        match *self {
            ResolveRoomError::NotFound(ref cause) => cause,
            ResolveRoomError::Validation(ref cause) => cause,
            ResolveRoomError::Credentials(ref err) => err.description(),
            ResolveRoomError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ResolveRoomError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RevokeInvitation
#[derive(Debug, PartialEq)]
pub enum RevokeInvitationError {
    /// <p>The resource is not found. HTTP Status Code: 400</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RevokeInvitationError {
    pub fn from_body(body: &str) -> RevokeInvitationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "NotFoundException" => {
                        RevokeInvitationError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        RevokeInvitationError::Validation(error_message.to_string())
                    }
                    _ => RevokeInvitationError::Unknown(String::from(body)),
                }
            }
            Err(_) => RevokeInvitationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RevokeInvitationError {
    fn from(err: serde_json::error::Error) -> RevokeInvitationError {
        RevokeInvitationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RevokeInvitationError {
    fn from(err: CredentialsError) -> RevokeInvitationError {
        RevokeInvitationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RevokeInvitationError {
    fn from(err: HttpDispatchError) -> RevokeInvitationError {
        RevokeInvitationError::HttpDispatch(err)
    }
}
impl From<io::Error> for RevokeInvitationError {
    fn from(err: io::Error) -> RevokeInvitationError {
        RevokeInvitationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RevokeInvitationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RevokeInvitationError {
    fn description(&self) -> &str {
        match *self {
            RevokeInvitationError::NotFound(ref cause) => cause,
            RevokeInvitationError::Validation(ref cause) => cause,
            RevokeInvitationError::Credentials(ref err) => err.description(),
            RevokeInvitationError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RevokeInvitationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SearchAddressBooks
#[derive(Debug, PartialEq)]
pub enum SearchAddressBooksError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SearchAddressBooksError {
    pub fn from_body(body: &str) -> SearchAddressBooksError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ValidationException" => {
                        SearchAddressBooksError::Validation(error_message.to_string())
                    }
                    _ => SearchAddressBooksError::Unknown(String::from(body)),
                }
            }
            Err(_) => SearchAddressBooksError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SearchAddressBooksError {
    fn from(err: serde_json::error::Error) -> SearchAddressBooksError {
        SearchAddressBooksError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SearchAddressBooksError {
    fn from(err: CredentialsError) -> SearchAddressBooksError {
        SearchAddressBooksError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SearchAddressBooksError {
    fn from(err: HttpDispatchError) -> SearchAddressBooksError {
        SearchAddressBooksError::HttpDispatch(err)
    }
}
impl From<io::Error> for SearchAddressBooksError {
    fn from(err: io::Error) -> SearchAddressBooksError {
        SearchAddressBooksError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SearchAddressBooksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SearchAddressBooksError {
    fn description(&self) -> &str {
        match *self {
            SearchAddressBooksError::Validation(ref cause) => cause,
            SearchAddressBooksError::Credentials(ref err) => err.description(),
            SearchAddressBooksError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SearchAddressBooksError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SearchContacts
#[derive(Debug, PartialEq)]
pub enum SearchContactsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SearchContactsError {
    pub fn from_body(body: &str) -> SearchContactsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ValidationException" => {
                        SearchContactsError::Validation(error_message.to_string())
                    }
                    _ => SearchContactsError::Unknown(String::from(body)),
                }
            }
            Err(_) => SearchContactsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SearchContactsError {
    fn from(err: serde_json::error::Error) -> SearchContactsError {
        SearchContactsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SearchContactsError {
    fn from(err: CredentialsError) -> SearchContactsError {
        SearchContactsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SearchContactsError {
    fn from(err: HttpDispatchError) -> SearchContactsError {
        SearchContactsError::HttpDispatch(err)
    }
}
impl From<io::Error> for SearchContactsError {
    fn from(err: io::Error) -> SearchContactsError {
        SearchContactsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SearchContactsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SearchContactsError {
    fn description(&self) -> &str {
        match *self {
            SearchContactsError::Validation(ref cause) => cause,
            SearchContactsError::Credentials(ref err) => err.description(),
            SearchContactsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            SearchContactsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SearchDevices
#[derive(Debug, PartialEq)]
pub enum SearchDevicesError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SearchDevicesError {
    pub fn from_body(body: &str) -> SearchDevicesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ValidationException" => {
                        SearchDevicesError::Validation(error_message.to_string())
                    }
                    _ => SearchDevicesError::Unknown(String::from(body)),
                }
            }
            Err(_) => SearchDevicesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SearchDevicesError {
    fn from(err: serde_json::error::Error) -> SearchDevicesError {
        SearchDevicesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SearchDevicesError {
    fn from(err: CredentialsError) -> SearchDevicesError {
        SearchDevicesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SearchDevicesError {
    fn from(err: HttpDispatchError) -> SearchDevicesError {
        SearchDevicesError::HttpDispatch(err)
    }
}
impl From<io::Error> for SearchDevicesError {
    fn from(err: io::Error) -> SearchDevicesError {
        SearchDevicesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SearchDevicesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SearchDevicesError {
    fn description(&self) -> &str {
        match *self {
            SearchDevicesError::Validation(ref cause) => cause,
            SearchDevicesError::Credentials(ref err) => err.description(),
            SearchDevicesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            SearchDevicesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SearchProfiles
#[derive(Debug, PartialEq)]
pub enum SearchProfilesError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SearchProfilesError {
    pub fn from_body(body: &str) -> SearchProfilesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ValidationException" => {
                        SearchProfilesError::Validation(error_message.to_string())
                    }
                    _ => SearchProfilesError::Unknown(String::from(body)),
                }
            }
            Err(_) => SearchProfilesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SearchProfilesError {
    fn from(err: serde_json::error::Error) -> SearchProfilesError {
        SearchProfilesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SearchProfilesError {
    fn from(err: CredentialsError) -> SearchProfilesError {
        SearchProfilesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SearchProfilesError {
    fn from(err: HttpDispatchError) -> SearchProfilesError {
        SearchProfilesError::HttpDispatch(err)
    }
}
impl From<io::Error> for SearchProfilesError {
    fn from(err: io::Error) -> SearchProfilesError {
        SearchProfilesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SearchProfilesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SearchProfilesError {
    fn description(&self) -> &str {
        match *self {
            SearchProfilesError::Validation(ref cause) => cause,
            SearchProfilesError::Credentials(ref err) => err.description(),
            SearchProfilesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            SearchProfilesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SearchRooms
#[derive(Debug, PartialEq)]
pub enum SearchRoomsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SearchRoomsError {
    pub fn from_body(body: &str) -> SearchRoomsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ValidationException" => {
                        SearchRoomsError::Validation(error_message.to_string())
                    }
                    _ => SearchRoomsError::Unknown(String::from(body)),
                }
            }
            Err(_) => SearchRoomsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SearchRoomsError {
    fn from(err: serde_json::error::Error) -> SearchRoomsError {
        SearchRoomsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SearchRoomsError {
    fn from(err: CredentialsError) -> SearchRoomsError {
        SearchRoomsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SearchRoomsError {
    fn from(err: HttpDispatchError) -> SearchRoomsError {
        SearchRoomsError::HttpDispatch(err)
    }
}
impl From<io::Error> for SearchRoomsError {
    fn from(err: io::Error) -> SearchRoomsError {
        SearchRoomsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SearchRoomsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SearchRoomsError {
    fn description(&self) -> &str {
        match *self {
            SearchRoomsError::Validation(ref cause) => cause,
            SearchRoomsError::Credentials(ref err) => err.description(),
            SearchRoomsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            SearchRoomsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SearchSkillGroups
#[derive(Debug, PartialEq)]
pub enum SearchSkillGroupsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SearchSkillGroupsError {
    pub fn from_body(body: &str) -> SearchSkillGroupsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ValidationException" => {
                        SearchSkillGroupsError::Validation(error_message.to_string())
                    }
                    _ => SearchSkillGroupsError::Unknown(String::from(body)),
                }
            }
            Err(_) => SearchSkillGroupsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SearchSkillGroupsError {
    fn from(err: serde_json::error::Error) -> SearchSkillGroupsError {
        SearchSkillGroupsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SearchSkillGroupsError {
    fn from(err: CredentialsError) -> SearchSkillGroupsError {
        SearchSkillGroupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SearchSkillGroupsError {
    fn from(err: HttpDispatchError) -> SearchSkillGroupsError {
        SearchSkillGroupsError::HttpDispatch(err)
    }
}
impl From<io::Error> for SearchSkillGroupsError {
    fn from(err: io::Error) -> SearchSkillGroupsError {
        SearchSkillGroupsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SearchSkillGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SearchSkillGroupsError {
    fn description(&self) -> &str {
        match *self {
            SearchSkillGroupsError::Validation(ref cause) => cause,
            SearchSkillGroupsError::Credentials(ref err) => err.description(),
            SearchSkillGroupsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SearchSkillGroupsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SearchUsers
#[derive(Debug, PartialEq)]
pub enum SearchUsersError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SearchUsersError {
    pub fn from_body(body: &str) -> SearchUsersError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ValidationException" => {
                        SearchUsersError::Validation(error_message.to_string())
                    }
                    _ => SearchUsersError::Unknown(String::from(body)),
                }
            }
            Err(_) => SearchUsersError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SearchUsersError {
    fn from(err: serde_json::error::Error) -> SearchUsersError {
        SearchUsersError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SearchUsersError {
    fn from(err: CredentialsError) -> SearchUsersError {
        SearchUsersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SearchUsersError {
    fn from(err: HttpDispatchError) -> SearchUsersError {
        SearchUsersError::HttpDispatch(err)
    }
}
impl From<io::Error> for SearchUsersError {
    fn from(err: io::Error) -> SearchUsersError {
        SearchUsersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SearchUsersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SearchUsersError {
    fn description(&self) -> &str {
        match *self {
            SearchUsersError::Validation(ref cause) => cause,
            SearchUsersError::Credentials(ref err) => err.description(),
            SearchUsersError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            SearchUsersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SendInvitation
#[derive(Debug, PartialEq)]
pub enum SendInvitationError {
    /// <p>The attempt to update a user is invalid due to the user's current status. HTTP Status Code: 400</p>
    InvalidUserStatus(String),
    /// <p>The resource is not found. HTTP Status Code: 400</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SendInvitationError {
    pub fn from_body(body: &str) -> SendInvitationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidUserStatusException" => {
                        SendInvitationError::InvalidUserStatus(String::from(error_message))
                    }
                    "NotFoundException" => {
                        SendInvitationError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        SendInvitationError::Validation(error_message.to_string())
                    }
                    _ => SendInvitationError::Unknown(String::from(body)),
                }
            }
            Err(_) => SendInvitationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SendInvitationError {
    fn from(err: serde_json::error::Error) -> SendInvitationError {
        SendInvitationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SendInvitationError {
    fn from(err: CredentialsError) -> SendInvitationError {
        SendInvitationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SendInvitationError {
    fn from(err: HttpDispatchError) -> SendInvitationError {
        SendInvitationError::HttpDispatch(err)
    }
}
impl From<io::Error> for SendInvitationError {
    fn from(err: io::Error) -> SendInvitationError {
        SendInvitationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SendInvitationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SendInvitationError {
    fn description(&self) -> &str {
        match *self {
            SendInvitationError::InvalidUserStatus(ref cause) => cause,
            SendInvitationError::NotFound(ref cause) => cause,
            SendInvitationError::Validation(ref cause) => cause,
            SendInvitationError::Credentials(ref err) => err.description(),
            SendInvitationError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            SendInvitationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StartDeviceSync
#[derive(Debug, PartialEq)]
pub enum StartDeviceSyncError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl StartDeviceSyncError {
    pub fn from_body(body: &str) -> StartDeviceSyncError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ValidationException" => {
                        StartDeviceSyncError::Validation(error_message.to_string())
                    }
                    _ => StartDeviceSyncError::Unknown(String::from(body)),
                }
            }
            Err(_) => StartDeviceSyncError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StartDeviceSyncError {
    fn from(err: serde_json::error::Error) -> StartDeviceSyncError {
        StartDeviceSyncError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StartDeviceSyncError {
    fn from(err: CredentialsError) -> StartDeviceSyncError {
        StartDeviceSyncError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartDeviceSyncError {
    fn from(err: HttpDispatchError) -> StartDeviceSyncError {
        StartDeviceSyncError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartDeviceSyncError {
    fn from(err: io::Error) -> StartDeviceSyncError {
        StartDeviceSyncError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartDeviceSyncError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartDeviceSyncError {
    fn description(&self) -> &str {
        match *self {
            StartDeviceSyncError::Validation(ref cause) => cause,
            StartDeviceSyncError::Credentials(ref err) => err.description(),
            StartDeviceSyncError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            StartDeviceSyncError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>The resource is not found. HTTP Status Code: 400</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl TagResourceError {
    pub fn from_body(body: &str) -> TagResourceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "NotFoundException" => TagResourceError::NotFound(String::from(error_message)),
                    "ValidationException" => {
                        TagResourceError::Validation(error_message.to_string())
                    }
                    _ => TagResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => TagResourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for TagResourceError {
    fn from(err: serde_json::error::Error) -> TagResourceError {
        TagResourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for TagResourceError {
    fn from(err: CredentialsError) -> TagResourceError {
        TagResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for TagResourceError {
    fn from(err: HttpDispatchError) -> TagResourceError {
        TagResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for TagResourceError {
    fn from(err: io::Error) -> TagResourceError {
        TagResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for TagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TagResourceError {
    fn description(&self) -> &str {
        match *self {
            TagResourceError::NotFound(ref cause) => cause,
            TagResourceError::Validation(ref cause) => cause,
            TagResourceError::Credentials(ref err) => err.description(),
            TagResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            TagResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>The resource is not found. HTTP Status Code: 400</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UntagResourceError {
    pub fn from_body(body: &str) -> UntagResourceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "NotFoundException" => {
                        UntagResourceError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UntagResourceError::Validation(error_message.to_string())
                    }
                    _ => UntagResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => UntagResourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UntagResourceError {
    fn from(err: serde_json::error::Error) -> UntagResourceError {
        UntagResourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UntagResourceError {
    fn from(err: CredentialsError) -> UntagResourceError {
        UntagResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UntagResourceError {
    fn from(err: HttpDispatchError) -> UntagResourceError {
        UntagResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for UntagResourceError {
    fn from(err: io::Error) -> UntagResourceError {
        UntagResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UntagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UntagResourceError {
    fn description(&self) -> &str {
        match *self {
            UntagResourceError::NotFound(ref cause) => cause,
            UntagResourceError::Validation(ref cause) => cause,
            UntagResourceError::Credentials(ref err) => err.description(),
            UntagResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UntagResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateAddressBook
#[derive(Debug, PartialEq)]
pub enum UpdateAddressBookError {
    /// <p>The name sent in the request is already in use. HTTP Status Code: 400</p>
    NameInUse(String),
    /// <p>The resource is not found. HTTP Status Code: 400</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateAddressBookError {
    pub fn from_body(body: &str) -> UpdateAddressBookError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "NameInUseException" => {
                        UpdateAddressBookError::NameInUse(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateAddressBookError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateAddressBookError::Validation(error_message.to_string())
                    }
                    _ => UpdateAddressBookError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateAddressBookError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateAddressBookError {
    fn from(err: serde_json::error::Error) -> UpdateAddressBookError {
        UpdateAddressBookError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateAddressBookError {
    fn from(err: CredentialsError) -> UpdateAddressBookError {
        UpdateAddressBookError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateAddressBookError {
    fn from(err: HttpDispatchError) -> UpdateAddressBookError {
        UpdateAddressBookError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateAddressBookError {
    fn from(err: io::Error) -> UpdateAddressBookError {
        UpdateAddressBookError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateAddressBookError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateAddressBookError {
    fn description(&self) -> &str {
        match *self {
            UpdateAddressBookError::NameInUse(ref cause) => cause,
            UpdateAddressBookError::NotFound(ref cause) => cause,
            UpdateAddressBookError::Validation(ref cause) => cause,
            UpdateAddressBookError::Credentials(ref err) => err.description(),
            UpdateAddressBookError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateAddressBookError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateContact
#[derive(Debug, PartialEq)]
pub enum UpdateContactError {
    /// <p>The resource is not found. HTTP Status Code: 400</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateContactError {
    pub fn from_body(body: &str) -> UpdateContactError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "NotFoundException" => {
                        UpdateContactError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateContactError::Validation(error_message.to_string())
                    }
                    _ => UpdateContactError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateContactError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateContactError {
    fn from(err: serde_json::error::Error) -> UpdateContactError {
        UpdateContactError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateContactError {
    fn from(err: CredentialsError) -> UpdateContactError {
        UpdateContactError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateContactError {
    fn from(err: HttpDispatchError) -> UpdateContactError {
        UpdateContactError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateContactError {
    fn from(err: io::Error) -> UpdateContactError {
        UpdateContactError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateContactError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateContactError {
    fn description(&self) -> &str {
        match *self {
            UpdateContactError::NotFound(ref cause) => cause,
            UpdateContactError::Validation(ref cause) => cause,
            UpdateContactError::Credentials(ref err) => err.description(),
            UpdateContactError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateContactError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateDevice
#[derive(Debug, PartialEq)]
pub enum UpdateDeviceError {
    /// <p>The resource is not found. HTTP Status Code: 400</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateDeviceError {
    pub fn from_body(body: &str) -> UpdateDeviceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "NotFoundException" => UpdateDeviceError::NotFound(String::from(error_message)),
                    "ValidationException" => {
                        UpdateDeviceError::Validation(error_message.to_string())
                    }
                    _ => UpdateDeviceError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateDeviceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateDeviceError {
    fn from(err: serde_json::error::Error) -> UpdateDeviceError {
        UpdateDeviceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateDeviceError {
    fn from(err: CredentialsError) -> UpdateDeviceError {
        UpdateDeviceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateDeviceError {
    fn from(err: HttpDispatchError) -> UpdateDeviceError {
        UpdateDeviceError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateDeviceError {
    fn from(err: io::Error) -> UpdateDeviceError {
        UpdateDeviceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateDeviceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDeviceError {
    fn description(&self) -> &str {
        match *self {
            UpdateDeviceError::NotFound(ref cause) => cause,
            UpdateDeviceError::Validation(ref cause) => cause,
            UpdateDeviceError::Credentials(ref err) => err.description(),
            UpdateDeviceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateDeviceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateProfile
#[derive(Debug, PartialEq)]
pub enum UpdateProfileError {
    /// <p>The name sent in the request is already in use. HTTP Status Code: 400</p>
    NameInUse(String),
    /// <p>The resource is not found. HTTP Status Code: 400</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateProfileError {
    pub fn from_body(body: &str) -> UpdateProfileError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "NameInUseException" => {
                        UpdateProfileError::NameInUse(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateProfileError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateProfileError::Validation(error_message.to_string())
                    }
                    _ => UpdateProfileError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateProfileError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateProfileError {
    fn from(err: serde_json::error::Error) -> UpdateProfileError {
        UpdateProfileError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateProfileError {
    fn from(err: CredentialsError) -> UpdateProfileError {
        UpdateProfileError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateProfileError {
    fn from(err: HttpDispatchError) -> UpdateProfileError {
        UpdateProfileError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateProfileError {
    fn from(err: io::Error) -> UpdateProfileError {
        UpdateProfileError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateProfileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateProfileError {
    fn description(&self) -> &str {
        match *self {
            UpdateProfileError::NameInUse(ref cause) => cause,
            UpdateProfileError::NotFound(ref cause) => cause,
            UpdateProfileError::Validation(ref cause) => cause,
            UpdateProfileError::Credentials(ref err) => err.description(),
            UpdateProfileError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateProfileError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateRoom
#[derive(Debug, PartialEq)]
pub enum UpdateRoomError {
    /// <p>The name sent in the request is already in use. HTTP Status Code: 400</p>
    NameInUse(String),
    /// <p>The resource is not found. HTTP Status Code: 400</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateRoomError {
    pub fn from_body(body: &str) -> UpdateRoomError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "NameInUseException" => UpdateRoomError::NameInUse(String::from(error_message)),
                    "NotFoundException" => UpdateRoomError::NotFound(String::from(error_message)),
                    "ValidationException" => UpdateRoomError::Validation(error_message.to_string()),
                    _ => UpdateRoomError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateRoomError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateRoomError {
    fn from(err: serde_json::error::Error) -> UpdateRoomError {
        UpdateRoomError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateRoomError {
    fn from(err: CredentialsError) -> UpdateRoomError {
        UpdateRoomError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateRoomError {
    fn from(err: HttpDispatchError) -> UpdateRoomError {
        UpdateRoomError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateRoomError {
    fn from(err: io::Error) -> UpdateRoomError {
        UpdateRoomError::HttpDispatch(HttpDispatchError::from(err))
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
            UpdateRoomError::NameInUse(ref cause) => cause,
            UpdateRoomError::NotFound(ref cause) => cause,
            UpdateRoomError::Validation(ref cause) => cause,
            UpdateRoomError::Credentials(ref err) => err.description(),
            UpdateRoomError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateRoomError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateSkillGroup
#[derive(Debug, PartialEq)]
pub enum UpdateSkillGroupError {
    /// <p>The name sent in the request is already in use. HTTP Status Code: 400</p>
    NameInUse(String),
    /// <p>The resource is not found. HTTP Status Code: 400</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateSkillGroupError {
    pub fn from_body(body: &str) -> UpdateSkillGroupError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "NameInUseException" => {
                        UpdateSkillGroupError::NameInUse(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateSkillGroupError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateSkillGroupError::Validation(error_message.to_string())
                    }
                    _ => UpdateSkillGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateSkillGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateSkillGroupError {
    fn from(err: serde_json::error::Error) -> UpdateSkillGroupError {
        UpdateSkillGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateSkillGroupError {
    fn from(err: CredentialsError) -> UpdateSkillGroupError {
        UpdateSkillGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateSkillGroupError {
    fn from(err: HttpDispatchError) -> UpdateSkillGroupError {
        UpdateSkillGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateSkillGroupError {
    fn from(err: io::Error) -> UpdateSkillGroupError {
        UpdateSkillGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateSkillGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateSkillGroupError {
    fn description(&self) -> &str {
        match *self {
            UpdateSkillGroupError::NameInUse(ref cause) => cause,
            UpdateSkillGroupError::NotFound(ref cause) => cause,
            UpdateSkillGroupError::Validation(ref cause) => cause,
            UpdateSkillGroupError::Credentials(ref err) => err.description(),
            UpdateSkillGroupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateSkillGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Alexa For Business API. Alexa For Business clients implement this trait.
pub trait AlexaForBusiness {
    /// <p>Associates a contact to a given address book.</p>
    fn associate_contact_with_address_book(
        &self,
        input: AssociateContactWithAddressBookRequest,
    ) -> RusotoFuture<AssociateContactWithAddressBookResponse, AssociateContactWithAddressBookError>;

    /// <p>Associates a device to a given room. This applies all the settings from the room profile to the device, and all the skills in any skill groups added to that room. This operation requires the device to be online, or a manual sync is required. </p>
    fn associate_device_with_room(
        &self,
        input: AssociateDeviceWithRoomRequest,
    ) -> RusotoFuture<AssociateDeviceWithRoomResponse, AssociateDeviceWithRoomError>;

    /// <p>Associates a skill group to a given room. This enables all skills in the associated skill group on all devices in the room.</p>
    fn associate_skill_group_with_room(
        &self,
        input: AssociateSkillGroupWithRoomRequest,
    ) -> RusotoFuture<AssociateSkillGroupWithRoomResponse, AssociateSkillGroupWithRoomError>;

    /// <p>Creates an address book with the specified details.</p>
    fn create_address_book(
        &self,
        input: CreateAddressBookRequest,
    ) -> RusotoFuture<CreateAddressBookResponse, CreateAddressBookError>;

    /// <p>Creates a contact with the specified details.</p>
    fn create_contact(
        &self,
        input: CreateContactRequest,
    ) -> RusotoFuture<CreateContactResponse, CreateContactError>;

    /// <p>Creates a new room profile with the specified details.</p>
    fn create_profile(
        &self,
        input: CreateProfileRequest,
    ) -> RusotoFuture<CreateProfileResponse, CreateProfileError>;

    /// <p>Creates a room with the specified details.</p>
    fn create_room(
        &self,
        input: CreateRoomRequest,
    ) -> RusotoFuture<CreateRoomResponse, CreateRoomError>;

    /// <p>Creates a skill group with a specified name and description.</p>
    fn create_skill_group(
        &self,
        input: CreateSkillGroupRequest,
    ) -> RusotoFuture<CreateSkillGroupResponse, CreateSkillGroupError>;

    /// <p>Creates a user.</p>
    fn create_user(
        &self,
        input: CreateUserRequest,
    ) -> RusotoFuture<CreateUserResponse, CreateUserError>;

    /// <p>Deletes an address book by the address book ARN.</p>
    fn delete_address_book(
        &self,
        input: DeleteAddressBookRequest,
    ) -> RusotoFuture<DeleteAddressBookResponse, DeleteAddressBookError>;

    /// <p>Deletes a contact by the contact ARN.</p>
    fn delete_contact(
        &self,
        input: DeleteContactRequest,
    ) -> RusotoFuture<DeleteContactResponse, DeleteContactError>;

    /// <p>Deletes a room profile by the profile ARN.</p>
    fn delete_profile(
        &self,
        input: DeleteProfileRequest,
    ) -> RusotoFuture<DeleteProfileResponse, DeleteProfileError>;

    /// <p>Deletes a room by the room ARN.</p>
    fn delete_room(
        &self,
        input: DeleteRoomRequest,
    ) -> RusotoFuture<DeleteRoomResponse, DeleteRoomError>;

    /// <p>Deletes room skill parameter details by room, skill, and parameter key ID.</p>
    fn delete_room_skill_parameter(
        &self,
        input: DeleteRoomSkillParameterRequest,
    ) -> RusotoFuture<DeleteRoomSkillParameterResponse, DeleteRoomSkillParameterError>;

    /// <p>Deletes a skill group by skill group ARN.</p>
    fn delete_skill_group(
        &self,
        input: DeleteSkillGroupRequest,
    ) -> RusotoFuture<DeleteSkillGroupResponse, DeleteSkillGroupError>;

    /// <p>Deletes a specified user by user ARN and enrollment ARN.</p>
    fn delete_user(
        &self,
        input: DeleteUserRequest,
    ) -> RusotoFuture<DeleteUserResponse, DeleteUserError>;

    /// <p>Disassociates a contact from a given address book.</p>
    fn disassociate_contact_from_address_book(
        &self,
        input: DisassociateContactFromAddressBookRequest,
    ) -> RusotoFuture<
        DisassociateContactFromAddressBookResponse,
        DisassociateContactFromAddressBookError,
    >;

    /// <p>Disassociates a device from its current room. The device continues to be connected to the Wi-Fi network and is still registered to the account. The device settings and skills are removed from the room.</p>
    fn disassociate_device_from_room(
        &self,
        input: DisassociateDeviceFromRoomRequest,
    ) -> RusotoFuture<DisassociateDeviceFromRoomResponse, DisassociateDeviceFromRoomError>;

    /// <p>Disassociates a skill group from a specified room. This disables all skills in the skill group on all devices in the room.</p>
    fn disassociate_skill_group_from_room(
        &self,
        input: DisassociateSkillGroupFromRoomRequest,
    ) -> RusotoFuture<DisassociateSkillGroupFromRoomResponse, DisassociateSkillGroupFromRoomError>;

    /// <p>Gets address the book details by the address book ARN.</p>
    fn get_address_book(
        &self,
        input: GetAddressBookRequest,
    ) -> RusotoFuture<GetAddressBookResponse, GetAddressBookError>;

    /// <p>Gets the contact details by the contact ARN.</p>
    fn get_contact(
        &self,
        input: GetContactRequest,
    ) -> RusotoFuture<GetContactResponse, GetContactError>;

    /// <p>Gets the details of a device by device ARN.</p>
    fn get_device(
        &self,
        input: GetDeviceRequest,
    ) -> RusotoFuture<GetDeviceResponse, GetDeviceError>;

    /// <p>Gets the details of a room profile by profile ARN.</p>
    fn get_profile(
        &self,
        input: GetProfileRequest,
    ) -> RusotoFuture<GetProfileResponse, GetProfileError>;

    /// <p>Gets room details by room ARN.</p>
    fn get_room(&self, input: GetRoomRequest) -> RusotoFuture<GetRoomResponse, GetRoomError>;

    /// <p>Gets room skill parameter details by room, skill, and parameter key ARN.</p>
    fn get_room_skill_parameter(
        &self,
        input: GetRoomSkillParameterRequest,
    ) -> RusotoFuture<GetRoomSkillParameterResponse, GetRoomSkillParameterError>;

    /// <p>Gets skill group details by skill group ARN.</p>
    fn get_skill_group(
        &self,
        input: GetSkillGroupRequest,
    ) -> RusotoFuture<GetSkillGroupResponse, GetSkillGroupError>;

    /// <p>Lists all enabled skills in a specific skill group.</p>
    fn list_skills(
        &self,
        input: ListSkillsRequest,
    ) -> RusotoFuture<ListSkillsResponse, ListSkillsError>;

    /// <p>Lists all tags for a specific resource.</p>
    fn list_tags(&self, input: ListTagsRequest) -> RusotoFuture<ListTagsResponse, ListTagsError>;

    /// <p>Updates room skill parameter details by room, skill, and parameter key ID. Not all skills have a room skill parameter.</p>
    fn put_room_skill_parameter(
        &self,
        input: PutRoomSkillParameterRequest,
    ) -> RusotoFuture<PutRoomSkillParameterResponse, PutRoomSkillParameterError>;

    /// <p>Determines the details for the room from which a skill request was invoked. This operation is used by skill developers.</p>
    fn resolve_room(
        &self,
        input: ResolveRoomRequest,
    ) -> RusotoFuture<ResolveRoomResponse, ResolveRoomError>;

    /// <p>Revokes an invitation and invalidates the enrollment URL.</p>
    fn revoke_invitation(
        &self,
        input: RevokeInvitationRequest,
    ) -> RusotoFuture<RevokeInvitationResponse, RevokeInvitationError>;

    /// <p>Searches address books and lists the ones that meet a set of filter and sort criteria.</p>
    fn search_address_books(
        &self,
        input: SearchAddressBooksRequest,
    ) -> RusotoFuture<SearchAddressBooksResponse, SearchAddressBooksError>;

    /// <p>Searches contacts and lists the ones that meet a set of filter and sort criteria.</p>
    fn search_contacts(
        &self,
        input: SearchContactsRequest,
    ) -> RusotoFuture<SearchContactsResponse, SearchContactsError>;

    /// <p>Searches devices and lists the ones that meet a set of filter criteria.</p>
    fn search_devices(
        &self,
        input: SearchDevicesRequest,
    ) -> RusotoFuture<SearchDevicesResponse, SearchDevicesError>;

    /// <p>Searches room profiles and lists the ones that meet a set of filter criteria.</p>
    fn search_profiles(
        &self,
        input: SearchProfilesRequest,
    ) -> RusotoFuture<SearchProfilesResponse, SearchProfilesError>;

    /// <p>Searches rooms and lists the ones that meet a set of filter and sort criteria.</p>
    fn search_rooms(
        &self,
        input: SearchRoomsRequest,
    ) -> RusotoFuture<SearchRoomsResponse, SearchRoomsError>;

    /// <p>Searches skill groups and lists the ones that meet a set of filter and sort criteria.</p>
    fn search_skill_groups(
        &self,
        input: SearchSkillGroupsRequest,
    ) -> RusotoFuture<SearchSkillGroupsResponse, SearchSkillGroupsError>;

    /// <p>Searches users and lists the ones that meet a set of filter and sort criteria.</p>
    fn search_users(
        &self,
        input: SearchUsersRequest,
    ) -> RusotoFuture<SearchUsersResponse, SearchUsersError>;

    /// <p>Sends an enrollment invitation email with a URL to a user. The URL is valid for 72 hours or until you call this operation again, whichever comes first. </p>
    fn send_invitation(
        &self,
        input: SendInvitationRequest,
    ) -> RusotoFuture<SendInvitationResponse, SendInvitationError>;

    /// <p>Resets a device and its account to the known default settings by clearing all information and settings set by previous users.</p>
    fn start_device_sync(
        &self,
        input: StartDeviceSyncRequest,
    ) -> RusotoFuture<StartDeviceSyncResponse, StartDeviceSyncError>;

    /// <p>Adds metadata tags to a specified resource.</p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> RusotoFuture<TagResourceResponse, TagResourceError>;

    /// <p>Removes metadata tags from a specified resource.</p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> RusotoFuture<UntagResourceResponse, UntagResourceError>;

    /// <p>Updates address book details by the address book ARN.</p>
    fn update_address_book(
        &self,
        input: UpdateAddressBookRequest,
    ) -> RusotoFuture<UpdateAddressBookResponse, UpdateAddressBookError>;

    /// <p>Updates the contact details by the contact ARN.</p>
    fn update_contact(
        &self,
        input: UpdateContactRequest,
    ) -> RusotoFuture<UpdateContactResponse, UpdateContactError>;

    /// <p>Updates the device name by device ARN.</p>
    fn update_device(
        &self,
        input: UpdateDeviceRequest,
    ) -> RusotoFuture<UpdateDeviceResponse, UpdateDeviceError>;

    /// <p>Updates an existing room profile by room profile ARN.</p>
    fn update_profile(
        &self,
        input: UpdateProfileRequest,
    ) -> RusotoFuture<UpdateProfileResponse, UpdateProfileError>;

    /// <p>Updates room details by room ARN.</p>
    fn update_room(
        &self,
        input: UpdateRoomRequest,
    ) -> RusotoFuture<UpdateRoomResponse, UpdateRoomError>;

    /// <p>Updates skill group details by skill group ARN.</p>
    fn update_skill_group(
        &self,
        input: UpdateSkillGroupRequest,
    ) -> RusotoFuture<UpdateSkillGroupResponse, UpdateSkillGroupError>;
}
/// A client for the Alexa For Business API.
pub struct AlexaForBusinessClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl AlexaForBusinessClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> AlexaForBusinessClient {
        AlexaForBusinessClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> AlexaForBusinessClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        AlexaForBusinessClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> AlexaForBusiness for AlexaForBusinessClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p>Associates a contact to a given address book.</p>
    fn associate_contact_with_address_book(
        &self,
        input: AssociateContactWithAddressBookRequest,
    ) -> RusotoFuture<AssociateContactWithAddressBookResponse, AssociateContactWithAddressBookError>
    {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.AssociateContactWithAddressBook",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<AssociateContactWithAddressBookResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AssociateContactWithAddressBookError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Associates a device to a given room. This applies all the settings from the room profile to the device, and all the skills in any skill groups added to that room. This operation requires the device to be online, or a manual sync is required. </p>
    fn associate_device_with_room(
        &self,
        input: AssociateDeviceWithRoomRequest,
    ) -> RusotoFuture<AssociateDeviceWithRoomResponse, AssociateDeviceWithRoomError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.AssociateDeviceWithRoom");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<AssociateDeviceWithRoomResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AssociateDeviceWithRoomError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Associates a skill group to a given room. This enables all skills in the associated skill group on all devices in the room.</p>
    fn associate_skill_group_with_room(
        &self,
        input: AssociateSkillGroupWithRoomRequest,
    ) -> RusotoFuture<AssociateSkillGroupWithRoomResponse, AssociateSkillGroupWithRoomError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.AssociateSkillGroupWithRoom",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<AssociateSkillGroupWithRoomResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AssociateSkillGroupWithRoomError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates an address book with the specified details.</p>
    fn create_address_book(
        &self,
        input: CreateAddressBookRequest,
    ) -> RusotoFuture<CreateAddressBookResponse, CreateAddressBookError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.CreateAddressBook");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateAddressBookResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateAddressBookError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a contact with the specified details.</p>
    fn create_contact(
        &self,
        input: CreateContactRequest,
    ) -> RusotoFuture<CreateContactResponse, CreateContactError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.CreateContact");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateContactResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateContactError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a new room profile with the specified details.</p>
    fn create_profile(
        &self,
        input: CreateProfileRequest,
    ) -> RusotoFuture<CreateProfileResponse, CreateProfileError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.CreateProfile");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateProfileResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateProfileError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a room with the specified details.</p>
    fn create_room(
        &self,
        input: CreateRoomRequest,
    ) -> RusotoFuture<CreateRoomResponse, CreateRoomError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.CreateRoom");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateRoomResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateRoomError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a skill group with a specified name and description.</p>
    fn create_skill_group(
        &self,
        input: CreateSkillGroupRequest,
    ) -> RusotoFuture<CreateSkillGroupResponse, CreateSkillGroupError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.CreateSkillGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateSkillGroupResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateSkillGroupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a user.</p>
    fn create_user(
        &self,
        input: CreateUserRequest,
    ) -> RusotoFuture<CreateUserResponse, CreateUserError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.CreateUser");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateUserResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateUserError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes an address book by the address book ARN.</p>
    fn delete_address_book(
        &self,
        input: DeleteAddressBookRequest,
    ) -> RusotoFuture<DeleteAddressBookResponse, DeleteAddressBookError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.DeleteAddressBook");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteAddressBookResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteAddressBookError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes a contact by the contact ARN.</p>
    fn delete_contact(
        &self,
        input: DeleteContactRequest,
    ) -> RusotoFuture<DeleteContactResponse, DeleteContactError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.DeleteContact");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteContactResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteContactError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes a room profile by the profile ARN.</p>
    fn delete_profile(
        &self,
        input: DeleteProfileRequest,
    ) -> RusotoFuture<DeleteProfileResponse, DeleteProfileError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.DeleteProfile");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteProfileResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteProfileError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes a room by the room ARN.</p>
    fn delete_room(
        &self,
        input: DeleteRoomRequest,
    ) -> RusotoFuture<DeleteRoomResponse, DeleteRoomError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.DeleteRoom");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteRoomResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteRoomError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes room skill parameter details by room, skill, and parameter key ID.</p>
    fn delete_room_skill_parameter(
        &self,
        input: DeleteRoomSkillParameterRequest,
    ) -> RusotoFuture<DeleteRoomSkillParameterResponse, DeleteRoomSkillParameterError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.DeleteRoomSkillParameter");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteRoomSkillParameterResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteRoomSkillParameterError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes a skill group by skill group ARN.</p>
    fn delete_skill_group(
        &self,
        input: DeleteSkillGroupRequest,
    ) -> RusotoFuture<DeleteSkillGroupResponse, DeleteSkillGroupError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.DeleteSkillGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteSkillGroupResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteSkillGroupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes a specified user by user ARN and enrollment ARN.</p>
    fn delete_user(
        &self,
        input: DeleteUserRequest,
    ) -> RusotoFuture<DeleteUserResponse, DeleteUserError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.DeleteUser");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteUserResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteUserError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Disassociates a contact from a given address book.</p>
    fn disassociate_contact_from_address_book(
        &self,
        input: DisassociateContactFromAddressBookRequest,
    ) -> RusotoFuture<
        DisassociateContactFromAddressBookResponse,
        DisassociateContactFromAddressBookError,
    > {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.DisassociateContactFromAddressBook",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DisassociateContactFromAddressBookResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DisassociateContactFromAddressBookError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Disassociates a device from its current room. The device continues to be connected to the Wi-Fi network and is still registered to the account. The device settings and skills are removed from the room.</p>
    fn disassociate_device_from_room(
        &self,
        input: DisassociateDeviceFromRoomRequest,
    ) -> RusotoFuture<DisassociateDeviceFromRoomResponse, DisassociateDeviceFromRoomError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.DisassociateDeviceFromRoom",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DisassociateDeviceFromRoomResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DisassociateDeviceFromRoomError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Disassociates a skill group from a specified room. This disables all skills in the skill group on all devices in the room.</p>
    fn disassociate_skill_group_from_room(
        &self,
        input: DisassociateSkillGroupFromRoomRequest,
    ) -> RusotoFuture<DisassociateSkillGroupFromRoomResponse, DisassociateSkillGroupFromRoomError>
    {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AlexaForBusiness.DisassociateSkillGroupFromRoom",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DisassociateSkillGroupFromRoomResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DisassociateSkillGroupFromRoomError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets address the book details by the address book ARN.</p>
    fn get_address_book(
        &self,
        input: GetAddressBookRequest,
    ) -> RusotoFuture<GetAddressBookResponse, GetAddressBookError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.GetAddressBook");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetAddressBookResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetAddressBookError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets the contact details by the contact ARN.</p>
    fn get_contact(
        &self,
        input: GetContactRequest,
    ) -> RusotoFuture<GetContactResponse, GetContactError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.GetContact");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetContactResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetContactError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets the details of a device by device ARN.</p>
    fn get_device(
        &self,
        input: GetDeviceRequest,
    ) -> RusotoFuture<GetDeviceResponse, GetDeviceError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.GetDevice");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetDeviceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetDeviceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets the details of a room profile by profile ARN.</p>
    fn get_profile(
        &self,
        input: GetProfileRequest,
    ) -> RusotoFuture<GetProfileResponse, GetProfileError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.GetProfile");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetProfileResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetProfileError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets room details by room ARN.</p>
    fn get_room(&self, input: GetRoomRequest) -> RusotoFuture<GetRoomResponse, GetRoomError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.GetRoom");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetRoomResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetRoomError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets room skill parameter details by room, skill, and parameter key ARN.</p>
    fn get_room_skill_parameter(
        &self,
        input: GetRoomSkillParameterRequest,
    ) -> RusotoFuture<GetRoomSkillParameterResponse, GetRoomSkillParameterError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.GetRoomSkillParameter");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetRoomSkillParameterResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetRoomSkillParameterError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets skill group details by skill group ARN.</p>
    fn get_skill_group(
        &self,
        input: GetSkillGroupRequest,
    ) -> RusotoFuture<GetSkillGroupResponse, GetSkillGroupError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.GetSkillGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetSkillGroupResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetSkillGroupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists all enabled skills in a specific skill group.</p>
    fn list_skills(
        &self,
        input: ListSkillsRequest,
    ) -> RusotoFuture<ListSkillsResponse, ListSkillsError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.ListSkills");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListSkillsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListSkillsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists all tags for a specific resource.</p>
    fn list_tags(&self, input: ListTagsRequest) -> RusotoFuture<ListTagsResponse, ListTagsError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.ListTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListTagsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListTagsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates room skill parameter details by room, skill, and parameter key ID. Not all skills have a room skill parameter.</p>
    fn put_room_skill_parameter(
        &self,
        input: PutRoomSkillParameterRequest,
    ) -> RusotoFuture<PutRoomSkillParameterResponse, PutRoomSkillParameterError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.PutRoomSkillParameter");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PutRoomSkillParameterResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PutRoomSkillParameterError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Determines the details for the room from which a skill request was invoked. This operation is used by skill developers.</p>
    fn resolve_room(
        &self,
        input: ResolveRoomRequest,
    ) -> RusotoFuture<ResolveRoomResponse, ResolveRoomError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.ResolveRoom");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ResolveRoomResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ResolveRoomError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Revokes an invitation and invalidates the enrollment URL.</p>
    fn revoke_invitation(
        &self,
        input: RevokeInvitationRequest,
    ) -> RusotoFuture<RevokeInvitationResponse, RevokeInvitationError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.RevokeInvitation");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<RevokeInvitationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RevokeInvitationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Searches address books and lists the ones that meet a set of filter and sort criteria.</p>
    fn search_address_books(
        &self,
        input: SearchAddressBooksRequest,
    ) -> RusotoFuture<SearchAddressBooksResponse, SearchAddressBooksError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.SearchAddressBooks");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<SearchAddressBooksResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SearchAddressBooksError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Searches contacts and lists the ones that meet a set of filter and sort criteria.</p>
    fn search_contacts(
        &self,
        input: SearchContactsRequest,
    ) -> RusotoFuture<SearchContactsResponse, SearchContactsError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.SearchContacts");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<SearchContactsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SearchContactsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Searches devices and lists the ones that meet a set of filter criteria.</p>
    fn search_devices(
        &self,
        input: SearchDevicesRequest,
    ) -> RusotoFuture<SearchDevicesResponse, SearchDevicesError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.SearchDevices");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<SearchDevicesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SearchDevicesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Searches room profiles and lists the ones that meet a set of filter criteria.</p>
    fn search_profiles(
        &self,
        input: SearchProfilesRequest,
    ) -> RusotoFuture<SearchProfilesResponse, SearchProfilesError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.SearchProfiles");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<SearchProfilesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SearchProfilesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Searches rooms and lists the ones that meet a set of filter and sort criteria.</p>
    fn search_rooms(
        &self,
        input: SearchRoomsRequest,
    ) -> RusotoFuture<SearchRoomsResponse, SearchRoomsError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.SearchRooms");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<SearchRoomsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SearchRoomsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Searches skill groups and lists the ones that meet a set of filter and sort criteria.</p>
    fn search_skill_groups(
        &self,
        input: SearchSkillGroupsRequest,
    ) -> RusotoFuture<SearchSkillGroupsResponse, SearchSkillGroupsError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.SearchSkillGroups");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<SearchSkillGroupsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SearchSkillGroupsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Searches users and lists the ones that meet a set of filter and sort criteria.</p>
    fn search_users(
        &self,
        input: SearchUsersRequest,
    ) -> RusotoFuture<SearchUsersResponse, SearchUsersError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.SearchUsers");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<SearchUsersResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SearchUsersError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Sends an enrollment invitation email with a URL to a user. The URL is valid for 72 hours or until you call this operation again, whichever comes first. </p>
    fn send_invitation(
        &self,
        input: SendInvitationRequest,
    ) -> RusotoFuture<SendInvitationResponse, SendInvitationError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.SendInvitation");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<SendInvitationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SendInvitationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Resets a device and its account to the known default settings by clearing all information and settings set by previous users.</p>
    fn start_device_sync(
        &self,
        input: StartDeviceSyncRequest,
    ) -> RusotoFuture<StartDeviceSyncResponse, StartDeviceSyncError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.StartDeviceSync");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StartDeviceSyncResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(StartDeviceSyncError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Adds metadata tags to a specified resource.</p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> RusotoFuture<TagResourceResponse, TagResourceError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<TagResourceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(TagResourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Removes metadata tags from a specified resource.</p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> RusotoFuture<UntagResourceResponse, UntagResourceError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UntagResourceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UntagResourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates address book details by the address book ARN.</p>
    fn update_address_book(
        &self,
        input: UpdateAddressBookRequest,
    ) -> RusotoFuture<UpdateAddressBookResponse, UpdateAddressBookError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.UpdateAddressBook");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateAddressBookResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateAddressBookError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates the contact details by the contact ARN.</p>
    fn update_contact(
        &self,
        input: UpdateContactRequest,
    ) -> RusotoFuture<UpdateContactResponse, UpdateContactError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.UpdateContact");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateContactResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateContactError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates the device name by device ARN.</p>
    fn update_device(
        &self,
        input: UpdateDeviceRequest,
    ) -> RusotoFuture<UpdateDeviceResponse, UpdateDeviceError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.UpdateDevice");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateDeviceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateDeviceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates an existing room profile by room profile ARN.</p>
    fn update_profile(
        &self,
        input: UpdateProfileRequest,
    ) -> RusotoFuture<UpdateProfileResponse, UpdateProfileError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.UpdateProfile");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateProfileResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateProfileError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates room details by room ARN.</p>
    fn update_room(
        &self,
        input: UpdateRoomRequest,
    ) -> RusotoFuture<UpdateRoomResponse, UpdateRoomError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.UpdateRoom");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateRoomResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateRoomError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates skill group details by skill group ARN.</p>
    fn update_skill_group(
        &self,
        input: UpdateSkillGroupRequest,
    ) -> RusotoFuture<UpdateSkillGroupResponse, UpdateSkillGroupError> {
        let mut request = SignedRequest::new("POST", "a4b", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AlexaForBusiness.UpdateSkillGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateSkillGroupResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateSkillGroupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }
}

#[cfg(test)]
mod protocol_tests {}
